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
export type PageTab = 'dashboard' | 'deposit' | 'withdraw' | 'settings' | 'admin'

export type RecoveryContactType = 'email' | 'wallet'

export interface RecoveryContact {
  id: string
  type: RecoveryContactType
  value: string
  addedAt: number
}

export interface RecoveryRequest {
  recoveryContactId: string
  newWallet: string
  verificationCode: string
  initiatedAt: number
  unlockAt: number
  verifiedAt: number | null
}

/** Loading states for async operations */
export type TxStatus = 'idle' | 'signing' | 'submitting' | 'confirming' | 'success' | 'error'

/** Contract call result wrapper */
export interface ContractResult<T> {
  success: boolean
  data?: T
  error?: string
  txHash?: string
}
