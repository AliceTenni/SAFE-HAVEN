use soroban_sdk::{contracttype, Address};

pub const MAX_DEPOSIT_AMOUNT: i128 = 1_000_000_000_000_000;
pub const MAX_LOCK_DURATION_SECS: u64 = 157_788_000;
pub const MIN_LOCK_DURATION_SECS: u64 = 60;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VaultKey {
    Deposit(Address, u32),
    DepositByLedger(Address, u32),
    DepositCounter(Address),
    /// Stores a `Vec<u32>` of active deposit IDs for a depositor (both timestamp- and
    /// ledger-based). Maintained alongside the counter so `get_deposit_ids` is O(1).
    ActiveDepositIds(Address),
    Admin,
    PendingAdmin,
    Initialized,
    DepositorList,
    /// Boolean existence flag per depositor — O(1) duplicate check in `add_depositor`.
    DepositorFlag(Address),
    FeeRecipient,
    MaxDeposit,
    MaxLockSecs,
    Paused,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultEntry {
    pub token: Address,
    pub amount: i128,
    pub unlock_time: u64,
    pub depositor: Address,
    pub penalty_bps: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerVaultEntry {
    pub token: Address,
    pub amount: i128,
    pub unlock_ledger: u32,
    pub depositor: Address,
    pub penalty_bps: u32,
}
