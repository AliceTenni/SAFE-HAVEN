// ============================================================
//  Shared TypeScript types — mirrors the Soroban contract types
// ============================================================

/** Mirrors the Rust VaultEntry struct */
export interface VaultEntry {
  token: string       // Stellar address
  amount: bigint      // in token base units (stroops for XLM)
  unlockTime: number  // Unix timestamp (seconds)
  depositor: string   // Stellar address
  penaltyBps: number  // 0–10_000 (basis points)
}

/** A deposit with its ID attached */
export interface Deposit extends VaultEntry {
  depositId: number
  /** Seconds remaining until unlock (0 if unlocked, null during initial load) */
  timeRemaining: number | null
}

/** Result of wallet connection */
export interface WalletInfo {
  address: string
  displayAddress: string
}

/** Tab pages */
export type PageTab = 'dashboard' | 'deposit' | 'withdraw' | 'admin'

/** Loading states for async operations */
export type TxStatus = 'idle' | 'signing' | 'submitting' | 'confirming' | 'success' | 'error'

/** Contract call result wrapper */
export interface ContractResult<T> {
  success: boolean
  data?: T
  error?: string
  txHash?: string
}

export interface TokenMetadata {
  assetCode: string
  issuer: string
  name: string | null
  description: string | null
  image: string | null
  totalSupply: bigint
  holders: number
  issuerCreatedAt: string | null
  issuerTransactionCount: number | null
  stellarExpertUrl: string
}

export interface TokenVettingChecks {
  contractVerified: boolean
  tvlAboveThreshold: boolean
  noKnownVulnerabilities: boolean
  creatorReputable: boolean
  ageAboveThreshold: boolean
}

export interface TokenVettingResult {
  metadata: TokenMetadata
  checks: TokenVettingChecks
  passed: boolean
  reasons: string[]
}

export interface TokenVettingRecord {
  proposer: string
  proposedAt: number
  reviewed: boolean
  reviewPassed: boolean
  reviewer: string | null
  reviewedAt: number | null
  approved: boolean
}
