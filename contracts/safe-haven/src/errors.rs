use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VaultError {
    InvalidAmount = 1,
    UnlockTimeNotInFuture = 2,
    NoDepositFound = 3,
    FundsStillLocked = 4,
    DepositAlreadyExists = 5,
    LockDurationTooLong = 6,
    Unauthorized = 7,
    AmountTooLarge = 8,
    InvalidPenaltyBps = 9,
    InvalidAdmin = 10,
    LockDurationTooShort = 11,
    ContractPaused = 12,
    VaultAlreadyUnlocked = 13,
    MissingFeeRecipient = 14,
    /// `initialize` was called on an already-initialized contract.
    /// The `is_initialized` flag is the sole re-initialization guard (closes #46).
    AlreadyInitialized = 15,

    // ── Issue #333: Recurring deposit subscriptions ───────────────────────
    /// Subscription not found for the given (depositor, subscription_id).
    NoSubscriptionFound = 16,
    /// The subscription has already been cancelled.
    SubscriptionCancelled = 17,
    /// All scheduled executions for this subscription have already run.
    SubscriptionCompleted = 18,
    /// The interval between executions has not yet elapsed.
    SubscriptionNotDue = 19,
    /// interval_secs or total_count is zero, or lock_duration_secs is invalid.
    InvalidSubscriptionParams = 20,

    // ── Issue #334: Deposit insurance pool ───────────────────────────────
    /// Insurance claim not found for the given claim_id.
    NoClaimFound = 21,
    /// The claim has already been approved or denied.
    ClaimAlreadyResolved = 22,
    /// The insurance pool does not hold enough of the requested token.
    InsufficientInsurancePool = 23,
}
