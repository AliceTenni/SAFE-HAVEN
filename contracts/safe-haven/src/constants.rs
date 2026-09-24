// ----------------------------------------------------------------
//  Protocol Constants
// ----------------------------------------------------------------

use crate::storage::LEDGER_SECONDS;

/// Maximum deposit amount (in stroops or token base units).
pub const MAX_DEPOSIT_AMOUNT: i128 = 1_000_000_000_000_000;

/// Maximum lock duration in seconds (~5 years).
pub const MAX_LOCK_DURATION_SECS: u64 = 157_788_000;

/// Minimum lock duration: prevent trivial, pointless vaults that waste storage.
pub const MIN_LOCK_DURATION_SECS: u64 = 60;

/// Minimum number of ledgers required for a ledger-based deposit.
pub const MIN_LOCK_LEDGERS: u32 = (MIN_LOCK_DURATION_SECS / LEDGER_SECONDS) as u32;

/// Maximum depositors per `batch_emergency_withdraw` call.
///
/// Soroban's per-transaction instruction budget is ~100M instructions.
/// Each iteration performs two persistent-storage removes, one token transfer,
/// and one event publish — roughly 1–2M instructions each.
/// 25 leaves comfortable headroom for the common migration use-case.
pub const MAX_BATCH_SIZE: u32 = 25;

// ----------------------------------------------------------------
//  Sustainability Constants
// ----------------------------------------------------------------

/// Carbon footprint baseline per token unit per second (in grams CO2e).
/// 1 unit locked for 1 second = CARBON_BASELINE_PER_UNIT_SECOND grams CO2e
/// Represents the environmental cost of holding crypto assets.
pub const CARBON_BASELINE_PER_UNIT_SECOND: i128 = 1; // adjustable based on network

/// Renewable energy baseline percentage (0-100).
/// Default assumption for energy mix if not specified.
pub const RENEWABLE_ENERGY_BASELINE: u32 = 50; // 50% renewable by default

/// Carbon offset rate in basis points (0-10000).
/// For every 1 basis point of offset configured, 1% of carbon footprint is offset.
pub const CARBON_OFFSET_RATE_BPS: u32 = 100; // 1% per basis point

/// Threshold for "high renewable energy" milestone (percentage).
pub const HIGH_RENEWABLE_THRESHOLD: u32 = 75;

/// Threshold for "carbon neutral" milestone (basis points of offset).
pub const CARBON_NEUTRAL_THRESHOLD_BPS: u32 = 10_000; // 100% offset

/// Threshold for "large offset" milestone (in grams CO2e).
pub const LARGE_OFFSET_THRESHOLD_GRAMS: i128 = 1_000_000_000; // 1000 kg CO2e
