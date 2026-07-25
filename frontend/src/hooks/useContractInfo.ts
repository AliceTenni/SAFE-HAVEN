// ============================================================
//  Hook: load contract-level info (admin, paused, constants)
// ============================================================

import { useCallback, useEffect, useRef, useState } from 'react'
import { getAdmin, isPaused, getConstants, getDepositorCount, getFeeRecipient } from '../lib/stellar'
import toast from 'react-hot-toast'

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
  const prevPausedRef = useRef<boolean>(false)

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
      
      // Check if pause state changed and emit notification
      if (pausedVal !== prevPausedRef.current) {
        prevPausedRef.current = pausedVal
        if (pausedVal) {
          toast.error('⚠️ Contract paused. New deposits are temporarily disabled.', {
            duration: 5000,
          })
        } else {
          toast.success('✓ Contract unpaused. Deposits are now enabled.', {
            duration: 4000,
          })
        }
      }
      
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

  // Initial fetch on mount
  useEffect(() => { void refresh() }, [refresh])

  // Set up 30-second polling interval for pause state
  useEffect(() => {
    const intervalId = setInterval(() => {
      void refresh()
    }, 30_000) // 30 seconds

    return () => clearInterval(intervalId)
  }, [refresh])

  return {
    admin, paused, maxDeposit, maxLockSecs, depositorCount, feeRecipient, loading, refresh,
  }
}
