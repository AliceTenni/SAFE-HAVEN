use soroban_sdk::{contracttype, Address};

pub const MAX_DEPOSIT_AMOUNT: i128 = 1_000_000_000_000_000;
pub const MAX_LOCK_DURATION_SECS: u64 = 157_788_000;
pub const MIN_LOCK_DURATION_SECS: u64 = 60;

/// Current storage schema version. Bump this constant when the on-chain
/// layout of a `contracttype` struct changes so `migrate()` can detect
/// and upgrade stale entries.
pub const STORAGE_VERSION: u32 = 1;

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
    /// Set-once flag recording that an address was appended to `DepositorList`.
    /// Never deleted, so re-deposits don't create duplicate list entries even
    /// after the corresponding `DepositorFlag` has been cleared by `remove_depositor`.
    DepositorInList(Address),
    FeeRecipient,
    MaxDeposit,
    MaxLockSecs,
    Paused,
    /// Persists the schema version written by the last `migrate()` call (or 1
    /// for contracts that were initialized before versioning was introduced).
    StorageVersion,
    /// Current contract version (semantic versioning string)
    ContractVersion,
    /// Tracks the pending upgrade proposal
    PendingUpgrade,
    /// Tracks the previous contract version for rollback support
    PreviousContractVersion,
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

/// Paginated query result containing a page of items and the total count.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Page<T> {
    /// The items in this page
    pub items: soroban_sdk::Vec<T>,
    /// Total number of active items across all pages
    pub total_count: u32,
}

/// Represents the status of an upgrade proposal
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UpgradeStatus {
    None = 0,
    Proposed = 1,
    Executed = 2,
    RolledBack = 3,
}

/// Information about a pending upgrade
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpgradeEntry {
    /// Address of the new contract code
    pub new_contract_id: Address,
    /// Timestamp when the upgrade can be executed
    pub execute_after: u64,
    /// Status of the upgrade
    pub status: UpgradeStatus,
    /// Optional migration step count for tracking
    pub migration_step_count: u32,
}
