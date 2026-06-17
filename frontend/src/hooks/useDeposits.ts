// ============================================================
//  Hook: load all deposits for the connected wallet
// ============================================================

import { useCallback, useEffect, useRef, useState } from 'react'
import { getDepositIds, getVault, getTimeRemaining } from '../lib/stellar'
import type { Deposit } from '../types'

interface UseDepositsResult {
  deposits: Deposit[]
  loading: boolean
  error: string | null
  refresh: () => void
}

export function useDeposits(depositorAddress: string | null): UseDepositsResult {
  const [deposits, setDeposits] = useState<Deposit[]>([])
  const [loading,  setLoading]  = useState(false)
  const [error,    setError]    = useState<string | null>(null)
  const abortRef = useRef<AbortController | null>(null)

  const refresh = useCallback(async () => {
    if (!depositorAddress) {
      setDeposits([])
      return
    }

    // Cancel any in-flight fetch
    abortRef.current?.abort()
    const ctrl = new AbortController()
    abortRef.current = ctrl

    setLoading(true)
    setError(null)

    try {
      const ids = await getDepositIds(depositorAddress)
      if (ctrl.signal.aborted) return

      const results = await Promise.all(
        ids.map(async (id) => {
          const [entry, remaining] = await Promise.all([
            getVault(depositorAddress, id),
            getTimeRemaining(depositorAddress, id),
          ])
          if (!entry) return null
          return { ...entry, depositId: id, timeRemaining: remaining } as Deposit
        }),
      )

      if (ctrl.signal.aborted) return
      setDeposits(results.filter((d): d is Deposit => d !== null))
    } catch (e) {
      if (ctrl.signal.aborted) return
      setError(e instanceof Error ? e.message : 'Failed to load deposits')
    } finally {
      if (!ctrl.signal.aborted) setLoading(false)
    }
  }, [depositorAddress])

  // Auto-refresh on address change
  useEffect(() => {
    void refresh()
    return () => abortRef.current?.abort()
  }, [refresh])

  // Tick countdown every second
  useEffect(() => {
    const id = setInterval(() => {
      setDeposits((prev) =>
        prev.map((d) => ({
          ...d,
          timeRemaining: Math.max(0, d.timeRemaining - 1),
        })),
      )
    }, 1000)
    return () => clearInterval(id)
  }, [])

  return { deposits, loading, error, refresh }
}
