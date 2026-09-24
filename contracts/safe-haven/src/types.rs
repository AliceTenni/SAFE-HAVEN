use soroban_sdk::{contracttype, Address, Vec};

pub const MAX_DEPOSIT_AMOUNT: i128 = 1_000_000_000_000_000;
pub const MAX_LOCK_DURATION_SECS: u64 = 157_788_000;
pub const MIN_LOCK_DURATION_SECS: u64 = 60;

/// Maximum number of tokens allowed in a single multi-token deposit (issue #330).
pub const MAX_TOKENS_PER_DEPOSIT: u32 = 5;

/// Current storage schema version. Bump this constant when the on-chain
/// layout of a `contracttype` struct changes so `migrate()` can detect
/// and upgrade stale entries.
pub const STORAGE_VERSION: u32 = 1;

/// Fraction of the penalty fee reserved for the insurance pool (5 = 5%).
pub const INSURANCE_POOL_BPS: u32 = 500; // 5% in basis points

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DepositType {
    TimeBased,
    LedgerBased,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepositRequest {
    pub token: Address,
    pub amount: i128,
    pub unlock_time: u64,
    pub penalty_bps: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DepositType {
    TimeBased,
    LedgerBased,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VaultKey {
    Deposit(Address, u32),
    DepositByLedger(Address, u32),
    /// Multi-token deposit entry (issue #330).
    MultiDeposit(Address, u32),
    /// Withdrawal whitelist for a deposit (issue #331).
    WithdrawalWhitelist(Address, u32),
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
    /// Boolean membership flag for the token allowlist.
    AllowedToken(Address),
    /// When true, deposits may use only tokens in the allowlist.
    StrictTokenAllowlist,
    /// Stores the token vetting workflow state.
    TokenVetting(Address),
    ProposalCounter,
    GovernanceProposal(u32),
    GovernanceVote(u32, Address),
    /// Persists the schema version written by the last `migrate()` call (or 1
    /// for contracts that were initialized before versioning was introduced).
    StorageVersion,
    /// Staker entry: maps staker address to their stake amount
    Staker(Address),
    /// List of all registered stakers
    StakerList,
    /// Flag to track if a staker is in the StakerList (prevents duplicates)
    StakerInList(Address),
    /// Total amount staked by all stakers
    TotalStaked,
    /// Rewards pool for stakers (accumulated from penalties)
    RewardsPool,
    /// Rewards claimed by a staker (track cumulative for auditing)
    StakerRewardsClaimed(Address),
    /// MEV commitment for a deposit: (depositor, deposit_id) -> MEVCommitment
    MEVCommitment(Address, u32),
    /// MEV detection record: (depositor, deposit_id, detection_id) -> MEVDetection
    MEVDetection(Address, u32, u32),
    /// MEV status for a deposit: (depositor, deposit_id) -> MEVStatus
    MEVStatus(Address, u32),
    /// Time-weighted average price history: (token, timestamp) -> Vec<PriceSample>
    MEVPriceHistory(Address, u64),
    /// Pool of recovered MEV ready for redistribution
    MEVPool,
    /// MEV recovered and claimed per depositor
    MEVClaimed(Address),
    /// Counter for MEV detections per deposit
    MEVDetectionCounter(Address, u32),
    /// Archived timestamp-based deposit (issue #XXX)
    ArchivedDeposit(Address, u32),
    /// Archived ledger-based deposit (issue #XXX)
    ArchivedDepositByLedger(Address, u32),
    /// Total carbon footprint for a depositor (sustainability tracking)
    TotalCarbonFootprint(Address),
    /// Total carbon offset for a depositor (sustainability tracking)
    TotalCarbonOffset(Address),
    /// Sustainability metrics for a single deposit
    SustainabilityMetrics(Address, u32),
    /// Bitmap of achieved sustainability milestones for a depositor
    MilestoneAchieved(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultEntry {
    pub token: Address,
    pub amount: i128,
    pub unlock_time: u64,
    pub depositor: Address,
    pub penalty_bps: u32,
    /// Compound interest accrual frequency in seconds (0 = no compounding). (issue #332)
    pub compound_frequency_secs: u64,
    /// Timestamp of last compound accrual (issue #332).
    pub last_accrual_timestamp: u64,
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

/// A single token+amount pair used in multi-token deposits (issue #330).
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenDeposit {
    pub token: Address,
    pub amount: i128,
}

/// Vault entry that holds multiple token deposits (issue #330).
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultiTokenVaultEntry {
    /// Each element is a (token, amount) pair. Length ≤ MAX_TOKENS_PER_DEPOSIT.
    pub tokens: Vec<TokenDeposit>,
    pub unlock_time: u64,
    pub depositor: Address,
    pub penalty_bps: u32,
    /// Compound interest accrual frequency in seconds (0 = no compounding). (issue #332)
    pub compound_frequency_secs: u64,
    /// Timestamp of last compound accrual (issue #332).
    pub last_accrual_timestamp: u64,
}

/// The deposit type discriminant returned by `get_deposit_type`.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DepositType {
    /// Timestamp-based (`VaultEntry`) single-token deposit.
    TimeBased,
    /// Ledger-sequence-based (`LedgerVaultEntry`) deposit.
    LedgerBased,
    /// Multi-token timestamp-based deposit (`MultiTokenVaultEntry`). (issue #330)
    MultiToken,
}

/// Paginated query result for depositor addresses.
/// (Soroban `#[contracttype]` does not support generics.)
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Page {
    /// The items in this page
    pub items: soroban_sdk::Vec<Address>,
    /// Total number of active items across all pages
    pub total_count: u32,
}

/// Staker entry: tracks stake amount and optionally last claim timestamp
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StakerEntry {
    pub staker: Address,
    pub stake_amount: i128,
}

/// MEV protection: stores the hash of a private order commit
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MEVCommitment {
    /// Keccak256 hash of (token, amount, price, nonce)
    pub commit_hash: soroban_sdk::BytesN<32>,
    /// Timestamp when the commit was submitted
    pub timestamp: u64,
    /// Depositor making the commitment
    pub depositor: Address,
    /// Whether the commitment has been revealed
    pub revealed: bool,
}

/// MEV detection record: tracks detected MEV attacks
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MEVDetection {
    /// Detection timestamp
    pub timestamp: u64,
    /// Affected depositor
    pub depositor: Address,
    /// Detected deposit ID
    pub deposit_id: u32,
    /// Detected price deviation (in basis points)
    pub price_deviation_bps: u32,
    /// MEV recovered (in smallest units of token)
    pub mev_recovered: i128,
    /// Whether the detection has been resolved
    pub resolved: bool,
}

/// MEV status for a deposit: commit-reveal state
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MEVStatus {
    /// No MEV protection requested
    Unprotected,
    /// Commit submitted, waiting for reveal
    Committed,
    /// Reveal submitted and validated
    Revealed,
    /// MEV attack detected
    AttackDetected,
}

/// Price sample for time-weighted average calculation
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PriceSample {
    /// Token address
    pub token: Address,
    /// Price (in smallest units)
    pub price: i128,
    /// Timestamp of price sample
    pub timestamp: u64,
}

/// Extended VaultKey enum with MEV keys
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MEVKey {
    /// MEV commitment for a deposit: (depositor, deposit_id) -> MEVCommitment
    Commitment(Address, u32),
    /// MEV detection record: (depositor, deposit_id, detection_id) -> MEVDetection
    Detection(Address, u32, u32),
    /// MEV status for a deposit: (depositor, deposit_id) -> MEVStatus
    Status(Address, u32),
    /// Time-weighted average price history: (token, timestamp) -> Vec<PriceSample>
    PriceHistory(Address, u64),
    /// Pool of recovered MEV ready for redistribution
    MEVPool,
    /// Accumulated MEV recovered per depositor for claims
    MEVClaimed(Address),
}

/// Archived timestamp-based deposit (issue #XXX)
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArchivedVaultEntry {
    pub token: Address,
    pub amount: i128,
    pub unlock_time: u64,
    pub depositor: Address,
    pub penalty_bps: u32,
    pub archive_timestamp: u64,
}

/// Archived ledger-based deposit (issue #XXX)
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArchivedLedgerVaultEntry {
    pub token: Address,
    pub amount: i128,
    pub unlock_ledger: u32,
    pub depositor: Address,
    pub penalty_bps: u32,
    pub archive_timestamp: u64,
}

/// Sustainability metrics for a single deposit
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SustainabilityMetrics {
    pub deposit_id: u32,
    pub depositor: Address,
    pub carbon_footprint: i128,
    pub renewable_energy_percent: u32,
    pub carbon_offset_grams: i128,
    pub timestamp: u64,
}
