// ============================================================
//  Hook: load contract-level info (admin, paused, constants)
// ============================================================

import { useCallback, useEffect, useState } from 'react'
import { getAdmin, isPaused, getConstants, getDepositorCount, getFeeRecipient } from '../lib/stellar'

interface ContractInfo {
  admin: string | null
  paused: boolean
  maxDeposit: bigint
  maxLockSecs: number
  depositorCount: number
  feeRecipient: string | null
  loading: boolean
  refresh: () => void
}

export function useContractInfo(): ContractInfo {
  const [admin,          setAdmin]          = useState<string | null>(null)
  const [paused,         setPaused]         = useState(false)
  const [maxDeposit,     setMaxDeposit]     = useState<bigint>(1_000_000_000_000_000n)
  const [maxLockSecs,    setMaxLockSecs]    = useState(157_788_000)
  const [depositorCount, setDepositorCount] = useState(0)
  const [feeRecipient,   setFeeRecipient]   = useState<string | null>(null)
  const [loading,        setLoading]        = useState(true)

  const refresh = useCallback(async () => {
    setLoading(true)
    try {
      const [adminVal, pausedVal, constants, count, fee] = await Promise.all([
        getAdmin(),
        isPaused(),
        getConstants(),
        getDepositorCount(),
        getFeeRecipient(),
      ])
      setAdmin(adminVal)
      setPaused(pausedVal)
      if (constants) {
        setMaxDeposit(constants.maxDeposit)
        setMaxLockSecs(constants.maxLockSecs)
      }
      setDepositorCount(count)
      setFeeRecipient(fee)
    } catch (e) {
      console.error('Failed to load contract info:', e)
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => { void refresh() }, [refresh])

  return {
    admin, paused, maxDeposit, maxLockSecs, depositorCount, feeRecipient, loading, refresh,
  }
}
