use soroban_sdk::{Address, Env, Vec};

use crate::types::{MultiTokenVaultEntry, VaultEntry, VaultKey, LedgerVaultEntry, MAX_LOCK_DURATION_SECS};

// Number of seconds per ledger — Soroban ledgers are ~5 seconds apart.
pub const LEDGER_SECONDS: u64 = 5;

// How many ledgers to extend TTL to cover the maximum allowed lock duration.
// BUMP_THRESHOLD is derived from BUMP_TARGET so both stay in sync automatically
// when MAX_LOCK_DURATION_SECS changes — no silent inconsistency.
pub const BUMP_TARGET: u32 = ((MAX_LOCK_DURATION_SECS + LEDGER_SECONDS - 1) / LEDGER_SECONDS) as u32;
pub const BUMP_THRESHOLD: u32 = BUMP_TARGET / 2;

// ----------------------------------------------------------------
//  Deposit counter helpers
// ----------------------------------------------------------------

// `next_deposit_id` returns the next unused deposit ID for a depositor.
// The counter is monotonic and is never decremented when deposits are
// removed. If all deposits are withdrawn, the counter still advances
// from its historical maximum, so a new deposit after a full drain will
// receive the next unused ID and cannot collide with any prior deposit.
//
// The active deposit IDs are tracked separately in `ActiveDepositIds`, so
// `get_deposit_ids` does not rely on scanning `0..counter` or checking
// whether each historical deposit key exists. This keeps active ID
// enumeration bounded by the number of open deposits rather than by the
// historical counter value.
pub fn next_deposit_id(env: &Env, depositor: &Address) -> u32 {
    let key = VaultKey::DepositCounter(depositor.clone());
    let id: u32 = env.storage().persistent().get(&key).unwrap_or(0);
    env.storage().persistent().set(&key, &(id.saturating_add(1)));
    env.storage()
        .persistent()
        .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
    id
}

// ----------------------------------------------------------------
//  Active deposit ID list helpers
// ----------------------------------------------------------------

fn get_active_ids(env: &Env, depositor: &Address) -> Vec<u32> {
    let key = VaultKey::ActiveDepositIds(depositor.clone());
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or_else(|| Vec::new(env))
}

fn save_active_ids(env: &Env, depositor: &Address, ids: &Vec<u32>) {
    let key = VaultKey::ActiveDepositIds(depositor.clone());
    env.storage().persistent().set(&key, ids);
    env.storage()
        .persistent()
        .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
}

/// Append `deposit_id` to the active ID list for `depositor`. Called
/// immediately after a new deposit entry is written to storage.
pub fn add_active_deposit_id(env: &Env, depositor: &Address, deposit_id: u32) {
    let mut ids = get_active_ids(env, depositor);
    ids.push_back(deposit_id);
    save_active_ids(env, depositor, &ids);
}

/// Remove `deposit_id` from the active ID list for `depositor`. Called
/// immediately after a deposit entry is removed from storage.
pub fn remove_active_deposit_id(env: &Env, depositor: &Address, deposit_id: u32) {
    let ids = get_active_ids(env, depositor);
    let mut new_ids: Vec<u32> = Vec::new(env);
    for id in ids.iter() {
        if id != deposit_id {
            new_ids.push_back(id);
        }
    }
    save_active_ids(env, depositor, &new_ids);
}

/// O(1) single storage read — returns all active deposit IDs for
/// `depositor`, regardless of whether they are timestamp- or
/// ledger-based.
pub fn get_deposit_ids(env: &Env, depositor: &Address) -> Vec<u32> {
    get_active_ids(env, depositor)
}

// ----------------------------------------------------------------
//  Deposit helpers (single-token, timestamp-based)
// ----------------------------------------------------------------

pub fn set_deposit(env: &Env, depositor: &Address, deposit_id: u32, entry: &VaultEntry) {
    let key = VaultKey::Deposit(depositor.clone(), deposit_id);
    env.storage().persistent().set(&key, entry);
    env.storage()
        .persistent()
        .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
    add_active_deposit_id(env, depositor, deposit_id);
}

pub fn get_deposit(env: &Env, depositor: &Address, deposit_id: u32) -> Option<VaultEntry> {
    let key = VaultKey::Deposit(depositor.clone(), deposit_id);
    let entry: Option<VaultEntry> = env.storage().persistent().get(&key);
    if entry.is_some() {
        env.storage()
            .persistent()
            .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
    }
    entry
}

pub fn get_deposit_readonly(env: &Env, depositor: &Address, deposit_id: u32) -> Option<VaultEntry> {
    let key = VaultKey::Deposit(depositor.clone(), deposit_id);
    env.storage().persistent().get(&key)
}

pub fn remove_deposit(env: &Env, depositor: &Address, deposit_id: u32) {
    let key = VaultKey::Deposit(depositor.clone(), deposit_id);
    env.storage().persistent().remove(&key);
    remove_active_deposit_id(env, depositor, deposit_id);
}

// ----------------------------------------------------------------
//  Ledger-based deposit helpers
// ----------------------------------------------------------------

pub fn set_deposit_by_ledger(env: &Env, depositor: &Address, deposit_id: u32, entry: &LedgerVaultEntry) {
    let key = VaultKey::DepositByLedger(depositor.clone(), deposit_id);
    env.storage().persistent().set(&key, entry);
    env.storage()
        .persistent()
        .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
    add_active_deposit_id(env, depositor, deposit_id);
}

pub fn get_deposit_by_ledger_readonly(env: &Env, depositor: &Address, deposit_id: u32) -> Option<LedgerVaultEntry> {
    let key = VaultKey::DepositByLedger(depositor.clone(), deposit_id);
    env.storage().persistent().get(&key)
}

pub fn remove_deposit_by_ledger(env: &Env, depositor: &Address, deposit_id: u32) {
    let key = VaultKey::DepositByLedger(depositor.clone(), deposit_id);
    env.storage().persistent().remove(&key);
    remove_active_deposit_id(env, depositor, deposit_id);
}

// ----------------------------------------------------------------
//  Multi-token deposit helpers (issue #330)
// ----------------------------------------------------------------

/// Write a `MultiTokenVaultEntry` to persistent storage.
pub fn set_multi_deposit(
    env: &Env,
    depositor: &Address,
    deposit_id: u32,
    entry: &MultiTokenVaultEntry,
) {
    let key = VaultKey::MultiDeposit(depositor.clone(), deposit_id);
    env.storage().persistent().set(&key, entry);
    env.storage()
        .persistent()
        .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
    add_active_deposit_id(env, depositor, deposit_id);
}

/// Read a `MultiTokenVaultEntry` (mutable path — extends TTL).
pub fn get_multi_deposit(
    env: &Env,
    depositor: &Address,
    deposit_id: u32,
) -> Option<MultiTokenVaultEntry> {
    let key = VaultKey::MultiDeposit(depositor.clone(), deposit_id);
    let entry: Option<MultiTokenVaultEntry> = env.storage().persistent().get(&key);
    if entry.is_some() {
        env.storage()
            .persistent()
            .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
    }
    entry
}

/// Read a `MultiTokenVaultEntry` (read-only — does not extend TTL).
pub fn get_multi_deposit_readonly(
    env: &Env,
    depositor: &Address,
    deposit_id: u32,
) -> Option<MultiTokenVaultEntry> {
    let key = VaultKey::MultiDeposit(depositor.clone(), deposit_id);
    env.storage().persistent().get(&key)
}

/// Remove a `MultiTokenVaultEntry` from storage.
pub fn remove_multi_deposit(env: &Env, depositor: &Address, deposit_id: u32) {
    let key = VaultKey::MultiDeposit(depositor.clone(), deposit_id);
    env.storage().persistent().remove(&key);
    remove_active_deposit_id(env, depositor, deposit_id);
}

// ----------------------------------------------------------------
//  Withdrawal whitelist helpers (issue #331)
// ----------------------------------------------------------------

/// Persist a whitelist for a specific deposit. An empty Vec means "no restriction".
pub fn set_withdrawal_whitelist(
    env: &Env,
    depositor: &Address,
    deposit_id: u32,
    whitelist: &Vec<Address>,
) {
    let key = VaultKey::WithdrawalWhitelist(depositor.clone(), deposit_id);
    env.storage().persistent().set(&key, whitelist);
    env.storage()
        .persistent()
        .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
}

/// Read the whitelist for a deposit. Returns `None` if no whitelist has been configured.
pub fn get_withdrawal_whitelist(
    env: &Env,
    depositor: &Address,
    deposit_id: u32,
) -> Option<Vec<Address>> {
    let key = VaultKey::WithdrawalWhitelist(depositor.clone(), deposit_id);
    env.storage().persistent().get(&key)
}

/// Remove the whitelist when a deposit is withdrawn / cancelled.
pub fn remove_withdrawal_whitelist(env: &Env, depositor: &Address, deposit_id: u32) {
    let key = VaultKey::WithdrawalWhitelist(depositor.clone(), deposit_id);
    // Only remove if present (avoid a panic on missing key).
    if env.storage().persistent().has(&key) {
        env.storage().persistent().remove(&key);
    }
}

// ----------------------------------------------------------------
//  Admin helpers
// ----------------------------------------------------------------

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().persistent().set(&VaultKey::Admin, admin);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::Admin, BUMP_THRESHOLD, BUMP_TARGET);
}

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().persistent().get(&VaultKey::Admin)
}

pub fn remove_admin(env: &Env) {
    env.storage().persistent().remove(&VaultKey::Admin);
}

pub fn set_pending_admin(env: &Env, pending: &Address) {
    env.storage()
        .persistent()
        .set(&VaultKey::PendingAdmin, pending);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::PendingAdmin, BUMP_THRESHOLD, BUMP_TARGET);
}

pub fn get_pending_admin(env: &Env) -> Option<Address> {
    env.storage().persistent().get(&VaultKey::PendingAdmin)
}

pub fn remove_pending_admin(env: &Env) {
    env.storage().persistent().remove(&VaultKey::PendingAdmin);
}

// ----------------------------------------------------------------
//  Initialized flag
// ----------------------------------------------------------------

pub fn set_initialized(env: &Env) {
    env.storage()
        .persistent()
        .set(&VaultKey::Initialized, &true);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::Initialized, BUMP_THRESHOLD, BUMP_TARGET);
}

pub fn is_initialized(env: &Env) -> bool {
    env.storage()
        .persistent()
        .get::<VaultKey, bool>(&VaultKey::Initialized)
        .unwrap_or(false)
}

// ----------------------------------------------------------------
//  Runtime limits helpers
// ----------------------------------------------------------------

pub fn set_max_deposit(env: &Env, v: i128) {
    env.storage().persistent().set(&VaultKey::MaxDeposit, &v);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::MaxDeposit, BUMP_THRESHOLD, BUMP_TARGET);
}

pub fn get_max_deposit(env: &Env) -> Option<i128> {
    env.storage().persistent().get(&VaultKey::MaxDeposit)
}

pub fn set_max_lock_secs(env: &Env, v: u64) {
    env.storage().persistent().set(&VaultKey::MaxLockSecs, &v);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::MaxLockSecs, BUMP_THRESHOLD, BUMP_TARGET);
}

/// Returns the runtime-configured max lock duration, or `None` to use the compile-time default.
pub fn get_max_lock_secs(env: &Env) -> Option<u64> {
    env.storage().persistent().get(&VaultKey::MaxLockSecs)
}

// ----------------------------------------------------------------
//  Fee recipient helpers
// ----------------------------------------------------------------

/// Persists the `fee_recipient` address and bumps TTL. Called once during `initialize`.
pub fn set_fee_recipient(env: &Env, recipient: &Address) {
    env.storage()
        .persistent()
        .set(&VaultKey::FeeRecipient, recipient);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::FeeRecipient, BUMP_THRESHOLD, BUMP_TARGET);
}

pub fn get_fee_recipient(env: &Env) -> Option<Address> {
    env.storage().persistent().get(&VaultKey::FeeRecipient)
}

// ----------------------------------------------------------------
//  Depositor list helpers
// ----------------------------------------------------------------

fn get_depositor_list(env: &Env) -> soroban_sdk::Vec<Address> {
    env.storage()
        .persistent()
        .get(&VaultKey::DepositorList)
        .unwrap_or_else(|| soroban_sdk::Vec::new(env))
}

fn save_depositor_list(env: &Env, list: &soroban_sdk::Vec<Address>) {
    env.storage()
        .persistent()
        .set(&VaultKey::DepositorList, list);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::DepositorList, BUMP_THRESHOLD, BUMP_TARGET);
}

pub fn add_depositor(env: &Env, depositor: &Address) {
    let flag_key = VaultKey::DepositorFlag(depositor.clone());
    let in_list_key = VaultKey::DepositorInList(depositor.clone());

    if env
        .storage()
        .persistent()
        .get::<VaultKey, bool>(&flag_key)
        .unwrap_or(false)
    {
        return;
    }

    env.storage().persistent().set(&flag_key, &true);
    env.storage()
        .persistent()
        .extend_ttl(&flag_key, BUMP_THRESHOLD, BUMP_TARGET);

    if !env
        .storage()
        .persistent()
        .get::<VaultKey, bool>(&in_list_key)
        .unwrap_or(false)
    {
        env.storage().persistent().set(&in_list_key, &true);
        env.storage()
            .persistent()
            .extend_ttl(&in_list_key, BUMP_THRESHOLD, BUMP_TARGET);

        let mut list = get_depositor_list(env);
        list.push_back(depositor.clone());
        save_depositor_list(env, &list);
    }
}

pub fn remove_depositor(env: &Env, depositor: &Address) {
    let flag_key = VaultKey::DepositorFlag(depositor.clone());
    env.storage().persistent().remove(&flag_key);
}

pub fn get_depositor_count(env: &Env) -> u32 {
    let list = get_depositor_list(env);
    let mut count: u32 = 0;
    for addr in list.iter() {
        let flag_key = VaultKey::DepositorFlag(addr.clone());
        if env
            .storage()
            .persistent()
            .get::<VaultKey, bool>(&flag_key)
            .unwrap_or(false)
        {
            count = count.saturating_add(1);
        }
    }
    count
}

// ----------------------------------------------------------------
//  Paused flag helpers
// ----------------------------------------------------------------

pub fn set_paused(env: &Env, paused: bool) {
    env.storage().persistent().set(&VaultKey::Paused, &paused);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::Paused, BUMP_THRESHOLD, BUMP_TARGET);
}

pub fn is_paused(env: &Env) -> bool {
    env.storage()
        .persistent()
        .get::<VaultKey, bool>(&VaultKey::Paused)
        .unwrap_or(false)
}

/// Returns the raw append-only depositor list (may contain stale entries after
/// O(1) removes).  Callers that need only *active* depositors should filter with
/// `depositor_is_active`.
pub fn get_all_depositors_raw(env: &Env) -> soroban_sdk::Vec<Address> {
    get_depositor_list(env)
}

/// Returns `true` if `depositor` currently has an active existence flag.
pub fn depositor_is_active(env: &Env, depositor: &Address) -> bool {
    let flag_key = VaultKey::DepositorFlag(depositor.clone());
    env.storage()
        .persistent()
        .get::<VaultKey, bool>(&flag_key)
        .unwrap_or(false)
}

pub fn get_depositors_page(env: &Env, offset: u32, limit: u32) -> (soroban_sdk::Vec<Address>, u32) {
    let list = get_depositor_list(env);
    let mut page: soroban_sdk::Vec<Address> = soroban_sdk::Vec::new(env);
    let mut active_seen: u32 = 0;
    let end_at = offset.saturating_add(limit);

    let mut total_count: u32 = 0;
    for addr in list.iter() {
        let flag_key = VaultKey::DepositorFlag(addr.clone());
        if env
            .storage()
            .persistent()
            .get::<VaultKey, bool>(&flag_key)
            .unwrap_or(false)
        {
            total_count = total_count.saturating_add(1);
        }
    }

    for addr in list.iter() {
        let flag_key = VaultKey::DepositorFlag(addr.clone());
        if !env
            .storage()
            .persistent()
            .get::<VaultKey, bool>(&flag_key)
            .unwrap_or(false)
        {
            continue;
        }
        if active_seen >= offset && active_seen < end_at {
            page.push_back(addr.clone());
        }
        active_seen = active_seen.saturating_add(1);
        if active_seen >= end_at {
            break;
        }
    }
    (page, total_count)
}

// ----------------------------------------------------------------
//  Admin authorization check helper
// ----------------------------------------------------------------

/// Returns `Ok(())` if `caller` matches the stored admin, else `Err(VaultError::Unauthorized)`.
pub fn require_admin(env: &Env, caller: &Address) -> Result<(), crate::errors::VaultError> {
    match get_admin(env) {
        Some(ref stored) if stored == caller => Ok(()),
        _ => Err(crate::errors::VaultError::Unauthorized),
    }
}

// ----------------------------------------------------------------
//  Storage version helpers
// ----------------------------------------------------------------

/// Write the current schema version into persistent storage.
pub fn set_storage_version(env: &Env, version: u32) {
    env.storage()
        .persistent()
        .set(&VaultKey::StorageVersion, &version);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::StorageVersion, BUMP_THRESHOLD, BUMP_TARGET);
}

/// Read the schema version that was last written by `migrate()`.
/// Returns `None` for contracts deployed before versioning was introduced.
pub fn get_storage_version(env: &Env) -> Option<u32> {
    env.storage()
        .persistent()
        .get(&VaultKey::StorageVersion)
}
