use soroban_sdk::{contracttype, Address, String};

pub const MAX_DEPOSIT_AMOUNT: i128 = 1_000_000_000_000_000;
pub const MAX_LOCK_DURATION_SECS: u64 = 157_788_000;
pub const MIN_LOCK_DURATION_SECS: u64 = 60;

/// Current storage schema version. Bump this constant when the on-chain
/// layout of a `contracttype` struct changes so `migrate()` can detect
/// and upgrade stale entries.
pub const STORAGE_VERSION: u32 = 1;

/// Fraction of the penalty fee reserved for the insurance pool (5 = 5%).
pub const INSURANCE_POOL_BPS: u32 = 500; // 5% in basis points

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

    // ── Issue #333: Recurring deposit subscriptions ──────────────────────
    /// The recurring deposit entry for `(depositor, subscription_id)`.
    Subscription(Address, u32),
    /// Monotonic counter for subscription IDs per depositor.
    SubscriptionCounter(Address),
    /// `Vec<u32>` of active subscription IDs for a depositor.
    ActiveSubscriptionIds(Address),

    // ── Issue #334: Insurance pool ────────────────────────────────────────
    /// Total token-agnostic balance held in the insurance pool (i128).
    /// Stored per-token so multi-token pools are supported.
    InsurancePoolBalance(Address),
    /// An individual insurance claim keyed by global claim ID.
    InsuranceClaim(u32),
    /// Global monotonic claim ID counter.
    InsuranceClaimCounter,
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

/// Paginated query result for depositor addresses.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Page {
    /// The items in this page
    pub items: soroban_sdk::Vec<Address>,
    /// Total number of active items across all pages
    pub total_count: u32,
}

// ────────────────────────────────────────────────────────────────
//  Issue #333 — Recurring deposit subscriptions
// ────────────────────────────────────────────────────────────────

/// A recurring deposit subscription.  Once created, anyone can call
/// `execute_subscription` on behalf of the depositor as long as
/// `executed_count < total_count` and the current timestamp is at or
/// past `next_execution_time`.
///
/// Each execution creates a fresh `VaultEntry` via the normal deposit
/// path (same validation rules apply).  The subscription ID is
/// independent of deposit IDs — a single subscription may produce many
/// individual deposit entries.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecurringDeposit {
    /// The account that authorised the subscription and whose balance is
    /// debited on each execution.
    pub depositor: Address,
    /// SAC-compatible token to lock on every execution.
    pub token: Address,
    /// Amount to lock per execution (in token base units).
    pub amount: i128,
    /// Seconds between successive executions.
    pub interval_secs: u64,
    /// Total number of executions requested.  Use `u32::MAX` for
    /// open-ended subscriptions (not recommended for production).
    pub total_count: u32,
    /// Number of executions already performed.
    pub executed_count: u32,
    /// Lock duration per individual deposit, in seconds from execution
    /// time.  Must satisfy `MIN_LOCK_DURATION_SECS` and the contract's
    /// `max_lock_secs`.
    pub lock_duration_secs: u64,
    /// Early-exit penalty in basis points for each produced deposit.
    pub penalty_bps: u32,
    /// Ledger timestamp after which the *next* execution is permitted.
    pub next_execution_time: u64,
    /// Whether the subscription has been cancelled by the depositor.
    pub cancelled: bool,
}

// ────────────────────────────────────────────────────────────────
//  Issue #334 — Deposit insurance pool
// ────────────────────────────────────────────────────────────────

/// Status of an insurance claim.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClaimStatus {
    /// Submitted but not yet reviewed.
    Pending,
    /// Approved by admin; funds disbursed.
    Approved,
    /// Rejected by admin; no funds disbursed.
    Denied,
}

/// An insurance claim filed by a depositor.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InsuranceClaim {
    /// Unique monotonic claim identifier.
    pub claim_id: u32,
    /// Address that filed the claim.
    pub claimant: Address,
    /// The token the claim is denominated in.
    pub token: Address,
    /// Amount requested from the insurance pool.
    pub amount_requested: i128,
    /// Free-form evidence string (e.g. incident description or tx hash).
    pub incident_evidence: String,
    /// Current state of the claim.
    pub status: ClaimStatus,
}
