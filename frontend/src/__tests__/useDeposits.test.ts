import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { renderHook, waitFor, act } from '@testing-library/react'

// ================================================================
//  Mock the stellar module — must be before any imports that use it
// ================================================================

const mockGetDepositIds = vi.fn()
const mockGetDepositBatch = vi.fn()
const mockGetLedgerTime = vi.fn()
const mockGetVault = vi.fn()

vi.mock('../lib/stellar', () => ({
  getDepositIds: (...args: unknown[]) => mockGetDepositIds(...args),
  getDepositBatch: (...args: unknown[]) => mockGetDepositBatch(...args),
  getLedgerTime: (...args: unknown[]) => mockGetLedgerTime(...args),
  getVault: (...args: unknown[]) => mockGetVault(...args),
}))

// Use dynamic import so the mock is applied first
let useDeposits: typeof import('../hooks/useDeposits').useDeposits

beforeAll(async () => {
  const mod = await import('../hooks/useDeposits')
  useDeposits = mod.useDeposits
})

// Helper address
const ADDR = 'GABCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGHIJKLMNOPQRSTUV'
const TOKEN = 'CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC'

function makeEntry(amount: bigint, unlockTime: number, penaltyBps = 0) {
  return {
    token: TOKEN,
    amount,
    unlockTime,
    depositor: ADDR,
    penaltyBps,
  }
}

function makeBatchResult(id: number, entry: ReturnType<typeof makeEntry> | null) {
  return { id, entry }
}

describe('useDeposits', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('returns empty deposits when no address is provided', () => {
    const { result } = renderHook(() => useDeposits(null))
    expect(result.current.deposits).toEqual([])
    expect(result.current.loading).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('loads deposits for a given address', async () => {
    const now = 1_700_000_000

    mockGetDepositIds.mockResolvedValueOnce([0, 1])
    mockGetLedgerTime.mockResolvedValueOnce(now)
    mockGetDepositBatch.mockResolvedValueOnce([
      makeBatchResult(0, makeEntry(1000n, now + 10_000)),
      makeBatchResult(1, makeEntry(5000n, now + 20_000, 500)),
    ])

    const { result } = renderHook(() => useDeposits(ADDR))

    // Initially loading
    expect(result.current.loading).toBe(true)

    await waitFor(() => {
      expect(result.current.loading).toBe(false)
    }, { timeout: 5000 })

    expect(result.current.deposits).toHaveLength(2)
    expect(result.current.deposits[0].depositId).toBe(0)
    expect(result.current.deposits[0].amount).toBe(1000n)
    expect(result.current.deposits[0].timeRemaining).toBe(10_000)
    expect(result.current.deposits[1].depositId).toBe(1)
    expect(result.current.deposits[1].amount).toBe(5000n)
    expect(result.current.deposits[1].timeRemaining).toBe(20_000)
    expect(result.current.error).toBeNull()
  })

  it('handles empty deposit IDs gracefully', async () => {
    mockGetDepositIds.mockResolvedValueOnce([])

    const { result } = renderHook(() => useDeposits(ADDR))

    await waitFor(() => {
      expect(result.current.loading).toBe(false)
    }, { timeout: 5000 })

    expect(result.current.deposits).toEqual([])
    expect(result.current.error).toBeNull()
  })

  it('handles errors gracefully', async () => {
    mockGetDepositIds.mockRejectedValueOnce(new Error('Network error'))

    const { result } = renderHook(() => useDeposits(ADDR))

    await waitFor(() => {
      expect(result.current.loading).toBe(false)
    }, { timeout: 5000 })

    expect(result.current.error).toBe('Network error')
    expect(result.current.deposits).toEqual([])
  })

  it('filters out entries that are null from batch', async () => {
    const now = 1_700_000_000

    mockGetDepositIds.mockResolvedValueOnce([0, 1, 2])
    mockGetLedgerTime.mockResolvedValueOnce(now)
    mockGetDepositBatch.mockResolvedValueOnce([
      makeBatchResult(0, null),
      makeBatchResult(1, makeEntry(5000n, now + 5000)),
      makeBatchResult(2, null),
    ])

    const { result } = renderHook(() => useDeposits(ADDR))

    await waitFor(() => {
      expect(result.current.loading).toBe(false)
    }, { timeout: 5000 })

    expect(result.current.deposits).toHaveLength(1)
    expect(result.current.deposits[0].depositId).toBe(1)
  })

  it('handles already-unlocked deposits (timeRemaining = 0)', async () => {
    const now = 1_700_000_000

    mockGetDepositIds.mockResolvedValueOnce([0])
    mockGetLedgerTime.mockResolvedValueOnce(now)
    mockGetDepositBatch.mockResolvedValueOnce([
      makeBatchResult(0, makeEntry(1000n, now - 100)),
    ])

    const { result } = renderHook(() => useDeposits(ADDR))

    await waitFor(() => {
      expect(result.current.loading).toBe(false)
    }, { timeout: 5000 })

    expect(result.current.deposits[0].timeRemaining).toBe(0)
  })
})
