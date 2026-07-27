// ============================================================
//  Wallet Context — manages Freighter / wallet-kit connection
// ============================================================

import React, {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
} from 'react'
import toast from 'react-hot-toast'
import type { WalletInfo, SigningResult } from '../types'
import { shortAddr } from '../lib/format'
import { CONFIG } from '../config'

interface WalletContextValue {
  wallet: WalletInfo | null
  isConnecting: boolean
  isRestoringSession: boolean
  networkMismatch: boolean
  connect: () => Promise<void>
  disconnect: () => void
  signTransaction: (xdr: string) => Promise<SigningResult>
}

const WalletContext = createContext<WalletContextValue | null>(null)

/**
 * Initialize wallet state synchronously from localStorage
 * Returns [wallet, isRestoringSession] where isRestoringSession is true if we found
 * a saved wallet that needs async validation
 */
function initializeWalletFromStorage(): [WalletInfo | null, boolean] {
  // Only run on client side
  if (typeof window === 'undefined' || typeof localStorage === 'undefined') {
    return [null, false]
  }

  const saved = localStorage.getItem('tlv_wallet_address')
  if (!saved) {
    return [null, false]
  }

  // We found a saved address — restore it immediately, but mark as restoring
  // so the UI knows it's pending async validation
  return [{ address: saved, displayAddress: shortAddr(saved) }, true]
}

export function WalletProvider({ children }: { children: React.ReactNode }) {
  const [wallet, setWallet]                   = useState<WalletInfo | null>(null)
  const [isRestoringSession, setIsRestoring]  = useState(false)
  const [isConnecting, setConnecting]         = useState(false)
  const [networkMismatch, setNetworkMismatch] = useState(false)

  // Initialize synchronously from localStorage on first render
  useEffect(() => {
    const [initialWallet, isRestoring] = initializeWalletFromStorage()
    setWallet(initialWallet)
    setIsRestoring(isRestoring)
  }, [])

  // Validate restored session against Freighter, or clear it if invalid
  useEffect(() => {
    if (!isRestoringSession || !wallet) return

    const validateSession = async () => {
      const freighter = (window as any).freighter // eslint-disable-line @typescript-eslint/no-explicit-any
      if (!freighter) {
        // Freighter not available — clear the stale session silently.
        localStorage.removeItem('tlv_wallet_address')
        setWallet(null)
        setNetworkMismatch(false)
        setIsRestoring(false)
        return
      }

      try {
        const { isConnected } = await freighter.isConnected()
        if (!isConnected) {
          // Wallet is locked — clear the stale session and ask the user to reconnect.
          localStorage.removeItem('tlv_wallet_address')
          setWallet(null)
          setNetworkMismatch(false)
          setIsRestoring(false)
          toast('Wallet locked — please reconnect.', { icon: '🔒' })
          return
        }

        const { address } = await freighter.getAddress()
        if (!address || address !== wallet.address) {
          // Active account changed — clear the stale session.
          localStorage.removeItem('tlv_wallet_address')
          setWallet(null)
          setNetworkMismatch(false)
          setIsRestoring(false)
          if (address) {
            toast('Wallet account changed — please reconnect.', { icon: '🔄' })
          }
          return
        }

        // Check network mismatch during restoration
        let walletNetworkPassphrase: string | undefined
        try {
          const networkDetails = await freighter.getNetworkDetails()
          walletNetworkPassphrase = networkDetails?.networkPassphrase
        } catch (e) {
          console.warn('Could not get network details from Freighter:', e)
        }

        const hasNetworkMismatch = walletNetworkPassphrase && walletNetworkPassphrase !== CONFIG.NETWORK_PASSPHRASE

        // Address is still valid; keep the wallet and mark restoration as complete
        setWallet({
          ...wallet,
          networkMismatch: !!hasNetworkMismatch,
          walletNetwork: walletNetworkPassphrase,
        })
        setNetworkMismatch(!!hasNetworkMismatch)
        setIsRestoring(false)
      } catch {
        // Error during validation — clear the stale session
        localStorage.removeItem('tlv_wallet_address')
        setWallet(null)
        setNetworkMismatch(false)
        setIsRestoring(false)
      }
    }

    validateSession()
  }, [isRestoringSession, wallet])

  const connect = useCallback(async () => {
    setConnecting(true)
    setNetworkMismatch(false)
    try {
      // Try Freighter first (most common Stellar wallet)
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const freighter = (window as any).freighter
      if (!freighter) {
        toast.error('Freighter wallet not found. Install it from freighter.app', { duration: 8000 })
        return
      }

      const { isConnected } = await freighter.isConnected()
      if (!isConnected) {
        toast.error('Please unlock your Freighter wallet first')
        return
      }

      const { address } = await freighter.getAddress()
      if (!address) {
        toast.error('Could not get address from Freighter')
        return
      }

      // Check network mismatch
      let walletNetworkPassphrase: string | undefined
      try {
        const networkDetails = await freighter.getNetworkDetails()
        walletNetworkPassphrase = networkDetails?.networkPassphrase
      } catch (e) {
        // If getNetworkDetails fails, we can't verify the network
        // but we shouldn't block the connection
        console.warn('Could not get network details from Freighter:', e)
      }

      const hasNetworkMismatch = walletNetworkPassphrase && walletNetworkPassphrase !== CONFIG.NETWORK_PASSPHRASE

      const info: WalletInfo = {
        address,
        displayAddress: shortAddr(address),
        networkMismatch: !!hasNetworkMismatch,
        walletNetwork: walletNetworkPassphrase,
      }
      setWallet(info)
      setNetworkMismatch(!!hasNetworkMismatch)
      localStorage.setItem('tlv_wallet_address', address)
      
      if (hasNetworkMismatch) {
        // Show a warning instead of success
        toast.error(
          `Network mismatch! Wallet: ${walletNetworkPassphrase}, App: ${CONFIG.NETWORK_PASSPHRASE}`,
          { duration: 0 }
        )
      } else {
        toast.success(`Connected: ${shortAddr(address)}`)
      }
    } catch (e) {
      const msg = e instanceof Error ? e.message : 'Failed to connect wallet'
      toast.error(msg)
    } finally {
      setConnecting(false)
    }
  }, [])

  const disconnect = useCallback(() => {
    setWallet(null)
    setNetworkMismatch(false)
    localStorage.removeItem('tlv_wallet_address')
    toast.success('Wallet disconnected')
  }, [])

  const signTransaction = useCallback(async (txXdr: string): Promise<SigningResult> => {
    // Check for network mismatch before attempting to sign
    if (networkMismatch) {
      const msg = `Network mismatch: Wallet is on ${wallet?.walletNetwork}, but app is on ${CONFIG.NETWORK_PASSPHRASE}`
      toast.error(msg, { duration: 0 })
      return { signed: false, rejected: false, error: msg }
    }

    try {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const freighter = (window as any).freighter
      if (!freighter) {
        toast.error('Freighter not available')
        return { signed: false, rejected: false, error: 'Freighter not available' }
      }
      const { signedTxXdr, error } = await freighter.signTransaction(txXdr, {
        networkPassphrase: import.meta.env.VITE_NETWORK_PASSPHRASE,
      })
      if (error) {
        toast.error(`Signing failed: ${error}`)
        return { signed: false, rejected: false, error }
      }
      return { signed: true, xdr: signedTxXdr as string }
    } catch (e) {
      const msg = e instanceof Error ? e.message : 'Signing rejected'
      const isUserReject = msg.toLowerCase().includes('reject') || msg.toLowerCase().includes('cancel')
      
      if (!isUserReject) {
        toast.error(`Signing error: ${msg}`)
        return { signed: false, rejected: false, error: msg }
      }
      
      // User rejection — silent, just return rejected flag
      return { signed: false, rejected: true }
    }
  }, [networkMismatch, wallet?.walletNetwork])

  return (
    <WalletContext.Provider value={{ wallet, isConnecting, isRestoringSession, networkMismatch, connect, disconnect, signTransaction }}>
      {children}
    </WalletContext.Provider>
  )
}

// eslint-disable-next-line react-refresh/only-export-components
export function useWallet(): WalletContextValue {
  const ctx = useContext(WalletContext)
  if (!ctx) throw new Error('useWallet must be used inside WalletProvider')
  return ctx
}
