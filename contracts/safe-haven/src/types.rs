use soroban_sdk::{contracttype, Address};

pub const MAX_DEPOSIT_AMOUNT: i128 = 1_000_000_000_000_000;
pub const MAX_LOCK_DURATION_SECS: u64 = 157_788_000;
pub const MIN_LOCK_DURATION_SECS: u64 = 60;

/// Current storage schema version. Bump this constant when the on-chain
/// layout of a `contracttype` struct changes so `migrate()` can detect
/// and upgrade stale entries.
pub const STORAGE_VERSION: u32 = 1;

/// Sustainability metrics for a deposit.
/// All values are stored as integers to maintain precision in Soroban.
/// - carbon_footprint: grams of CO2 equivalent
/// - renewable_energy_percent: percentage (0-100)
/// - carbon_offset_grams: grams of CO2 offset
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SustainabilityMetrics {
    pub deposit_id: u32,
    pub depositor: Address,
    pub carbon_footprint: i128,        // grams CO2e
    pub renewable_energy_percent: u32, // 0-100
    pub carbon_offset_grams: i128,     // grams CO2 offset
    pub timestamp: u64,                // when recorded
}

/// Aggregated sustainability report for a depositor.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SustainabilityReport {
    pub depositor: Address,
    pub total_carbon_footprint: i128,        // total grams CO2e across all deposits
    pub average_renewable_energy_percent: u32, // average percentage
    pub total_carbon_offset: i128,            // total grams CO2 offset
    pub active_deposit_count: u32,            // number of active deposits
    pub report_timestamp: u64,
}

/// Milestone achievement for sustainability goals.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SustainabilityMilestoneType {
    CarbonNeutral,      // 100% of carbon offset
    HighRenewable,      // >= 75% renewable energy
    CarbonNegative,     // More carbon offset than footprint
    LargeOffset,        // Total offset >= 1000 kg CO2e
}

/// Deposit type indicator: either timestamp-based or ledger-sequence-based.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DepositType {
    TimeBased,   // Deposit with a unix timestamp unlock time
    LedgerBased, // Deposit with a ledger sequence number unlock condition
}

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
    // Sustainability metrics storage
    SustainabilityMetrics(Address, u32), // (depositor, deposit_id)
    TotalCarbonFootprint(Address),       // sum of all carbon footprints
    TotalCarbonOffset(Address),          // sum of all carbon offsets
    MilestoneAchieved(Address),          // bitmap of achieved milestones
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
