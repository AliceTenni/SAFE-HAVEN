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
    FaucetNotConfigured = 16,
    FaucetRateLimited = 17,
    FaucetInsufficientFunds = 18,
    FaucetAmountTooLarge = 19,
    UpgradeEvidenceRequired = 20,
    UpgradeNotFound = 21,
    UpgradeReviewRequired = 22,
    UpgradeNotVoting = 23,
    UpgradeAlreadyVoted = 24,
    UpgradeNotApproved = 25,
    UpgradeTimelocked = 26,
}
