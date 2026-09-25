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
    /// Too many tokens in a multi-token deposit — exceeds MAX_TOKENS_PER_DEPOSIT (issue #330).
    TooManyTokens = 16,
    /// A multi-token deposit must contain at least one token (issue #330).
    EmptyTokenList = 17,
    /// Recipient is not on the withdrawal whitelist (issue #331).
    RecipientNotWhitelisted = 18,
    /// Compound frequency must be >= 60 seconds if non-zero (issue #332).
    InvalidCompoundFrequency = 19,
    /// The ML-DSA public key or signature has an invalid encoding or length.
    InvalidQuantumSafeKey = 20,
    /// The ML-DSA signature does not authorize the exact deposit payload.
    InvalidQuantumSafeSignature = 21,
    /// The DID method is outside the supported W3C method set.
    UnsupportedDidMethod = 22,
    UpgradeEvidenceRequired = 23,
    UpgradeNotFound = 24,
    UpgradeReviewRequired = 25,
    UpgradeNotVoting = 26,
    UpgradeAlreadyVoted = 27,
    UpgradeNotApproved = 28,
    UpgradeTimelocked = 29,
    ProposalNotFound = 30,
    VotingEnded = 31,
    AlreadyVoted = 32,
    NoVotingPower = 33,
    VotingStillOpen = 34,
    TimelockActive = 35,
    ProposalRejected = 36,
    ProposalAlreadyExecuted = 37,
}
