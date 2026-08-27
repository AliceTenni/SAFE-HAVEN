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
    TokenNotAllowed = 15,
    TokenVettingNotFound = 16,
    TokenAlreadyApproved = 17,
    TokenReviewRequired = 18,
    ProposalNotFound = 19,
    VotingEnded = 20,
    VotingStillOpen = 21,
    TimelockActive = 22,
    ProposalAlreadyExecuted = 23,
    AlreadyVoted = 24,
    NoVotingPower = 25,
    ProposalRejected = 26,
    InvalidGovernanceMode = 27,
}
