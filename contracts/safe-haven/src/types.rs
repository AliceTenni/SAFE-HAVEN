use soroban_sdk::{contracttype, Address, BytesN, String};

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
    FaucetToken(FaucetAsset),
    FaucetMaxAmount(FaucetAsset),
    FaucetLastRequest(Address),
    FaucetRequestCount(FaucetAsset),
    FaucetDistributed(FaucetAsset),
    NextUpgradeId,
    UpgradeProposal(u32),
    UpgradeVote(u32, Address),
    UpgradeVeto(u32, Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UpgradeStatus { Review, Voting, Approved, Vetoed, Executed }

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpgradeProposal {
    pub id: u32,
    pub proposer: Address,
    pub old_version: String,
    pub new_version: String,
    pub diff_url: String,
    pub audit_url: String,
    pub review_url: String,
    pub wasm_hash: BytesN<32>,
    pub status: UpgradeStatus,
    pub approval_votes: u32,
    pub rejection_votes: u32,
    pub veto_votes: u32,
    pub approved_at: Option<u64>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FaucetAsset {
    Usdc,
    Eth,
    Btc,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FaucetStatus {
    pub token: Option<Address>,
    pub balance: i128,
    pub max_amount: i128,
    pub request_count: u32,
    pub distributed: i128,
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
