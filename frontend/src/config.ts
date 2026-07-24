// ============================================================
//  App Configuration
//  Update CONTRACT_ID and NETWORK after deploying your contract
// ============================================================

// Validate critical env vars at startup so misconfigured deployments fail fast
// rather than silently targeting the wrong network (#13).
if (!import.meta.env.VITE_NETWORK_PASSPHRASE) {
  throw new Error(
    '[SAFE-HAVEN] VITE_NETWORK_PASSPHRASE is not set. ' +
    'Add it to your .env file (e.g. "Test SDF Network ; September 2015" for testnet ' +
    'or "Public Global Stellar Network ; September 2015" for mainnet). ' +
    'Refusing to start to prevent transactions from being signed for the wrong network.'
  )
}

export const CONFIG = {
  /** Deployed contract ID — set via VITE_CONTRACT_ID env var or paste here */
  CONTRACT_ID: import.meta.env.VITE_CONTRACT_ID as string ?? 'CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4',

  /** Stellar network passphrase */
  NETWORK_PASSPHRASE: import.meta.env.VITE_NETWORK_PASSPHRASE as string,

  /** Horizon / Soroban RPC endpoint */
  RPC_URL: (import.meta.env.VITE_RPC_URL as string) ??
    'https://soroban-testnet.stellar.org',

  /** Horizon endpoint (for account info) */
  HORIZON_URL: (import.meta.env.VITE_HORIZON_URL as string) ??
    'https://horizon-testnet.stellar.org',

  /** Explorer base URL for transactions */
  EXPLORER_URL: (import.meta.env.VITE_EXPLORER_URL as string) ??
    'https://stellar.expert/explorer/testnet',

  /** Native XLM token contract on testnet */
  NATIVE_TOKEN: 'CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC',

  /** Contract constants (mirror of Rust compile-time defaults) */
  MAX_DEPOSIT_AMOUNT: 1_000_000_000_000_000n,
  MAX_LOCK_DURATION_SECS: 157_788_000,
  MIN_LOCK_DURATION_SECS: 60,
  MAX_PENALTY_BPS: 10_000,

  /** Stroops per XLM */
  STROOPS_PER_XLM: 10_000_000,
} as const
