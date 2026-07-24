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
import type { WalletInfo } from '../types'
import { shortAddr } from '../lib/format'

interface WalletContextValue {
  wallet: WalletInfo | null
  isConnecting: boolean
  connect: () => Promise<void>
  disconnect: () => void
  signTransaction: (xdr: string) => Promise<string | null>
}

const WalletContext = createContext<WalletContextValue | null>(null)

export function WalletProvider({ children }: { children: React.ReactNode }) {
  const [wallet, setWallet]           = useState<WalletInfo | null>(null)
  const [isConnecting, setConnecting] = useState(false)

  // Restore session on mount — re-validate against the live Freighter account
  // to guard against stale sessions after an account or network switch (#12).
  useEffect(() => {
    const saved = localStorage.getItem('tlv_wallet_address')
    if (!saved) return

    const freighter = (window as any).freighter // eslint-disable-line @typescript-eslint/no-explicit-any
    if (!freighter) {
      // Freighter not available — clear the stale session silently.
      localStorage.removeItem('tlv_wallet_address')
      return
    }

    freighter.isConnected().then(({ isConnected }: { isConnected: boolean }) => {
      if (!isConnected) {
        // Wallet is locked — clear the stale session and ask the user to reconnect.
        localStorage.removeItem('tlv_wallet_address')
        toast('Wallet locked — please reconnect.', { icon: '🔒' })
        return
      }

      freighter.getAddress().then(({ address }: { address: string }) => {
        if (!address || address !== saved) {
          // Active account changed — clear the stale session.
          localStorage.removeItem('tlv_wallet_address')
          if (address) {
            toast('Wallet account changed — please reconnect.', { icon: '🔄' })
          }
          return
        }

        // Address is still valid; restore the session.
        setWallet({ address: saved, displayAddress: shortAddr(saved) })
      }).catch(() => {
        localStorage.removeItem('tlv_wallet_address')
      })
    }).catch(() => {
      localStorage.removeItem('tlv_wallet_address')
    })
  }, [])

  const connect = useCallback(async () => {
    setConnecting(true)
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

      const info: WalletInfo = { address, displayAddress: shortAddr(address) }
      setWallet(info)
      localStorage.setItem('tlv_wallet_address', address)
      toast.success(`Connected: ${shortAddr(address)}`)
    } catch (e) {
      const msg = e instanceof Error ? e.message : 'Failed to connect wallet'
      toast.error(msg)
    } finally {
      setConnecting(false)
    }
  }, [])

  const disconnect = useCallback(() => {
    setWallet(null)
    localStorage.removeItem('tlv_wallet_address')
    toast.success('Wallet disconnected')
  }, [])

  const signTransaction = useCallback(async (txXdr: string): Promise<string | null> => {
    try {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const freighter = (window as any).freighter
      if (!freighter) {
        toast.error('Freighter not available')
        return null
      }
      const { signedTxXdr, error } = await freighter.signTransaction(txXdr, {
        networkPassphrase: import.meta.env.VITE_NETWORK_PASSPHRASE,
      })
      if (error) {
        toast.error(`Signing failed: ${error}`)
        return null
      }
      return signedTxXdr as string
    } catch (e) {
      const msg = e instanceof Error ? e.message : 'Signing rejected'
      if (!msg.toLowerCase().includes('reject') && !msg.toLowerCase().includes('cancel')) {
        toast.error(msg)
      }
      return null
    }
  }, [])

  return (
    <WalletContext.Provider value={{ wallet, isConnecting, connect, disconnect, signTransaction }}>
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
