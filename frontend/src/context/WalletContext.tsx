// ============================================================
//  Wallet Context — manages multi-wallet connection via stellar-wallets-kit
// ============================================================

import React, {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
} from 'react'
import toast from 'react-hot-toast'
import { StellarWalletsKit } from '@creit.tech/stellar-wallets-kit'
import {
  FREIGHTER_ID,
  WalletNetwork,
  FreighterModule,
  xBullModule,
  AlbedoModule,
  LobstrModule,
  HanaModule,
} from '@creit.tech/stellar-wallets-kit'
import type { WalletInfo } from '../types'
import { shortAddr } from '../lib/format'
import { CONFIG } from '../config'

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
  const walletKitRef = useRef<StellarWalletsKit | null>(null)

  // Determine network
  const isMainnet = CONFIG.NETWORK_PASSPHRASE === 'Public Global Stellar Network ; September 2015'
  const network = isMainnet ? WalletNetwork.PUBLIC : WalletNetwork.TESTNET

  // Initialize wallet kit on mount
  useEffect(() => {
    try {
      walletKitRef.current = new StellarWalletsKit({
        network,
        selectedWalletId: FREIGHTER_ID,
        modules: [
          new FreighterModule(),
          new xBullModule(),
          new AlbedoModule(),
          new LobstrModule(),
          new HanaModule(),
        ],
      })
    } catch (e) {
      console.error('Failed to initialize StellarWalletsKit:', e)
    }
  }, [network])

  // Restore session on mount — re-validate against the live wallet address
  // to guard against stale sessions after an account or network switch (#12).
  useEffect(() => {
    if (!walletKitRef.current) return

    const saved = localStorage.getItem('tlv_wallet_address')
    if (!saved) return

    const restore = async () => {
      try {
        // Try to get the current address from the wallet kit
        try {
          const result = await walletKitRef.current!.getAddress()
          if (!result || !result.address) {
            localStorage.removeItem('tlv_wallet_address')
            return
          }

          const { address } = result
          if (address !== saved) {
            // Active account changed — clear the stale session.
            localStorage.removeItem('tlv_wallet_address')
            toast('Wallet account changed — please reconnect.', { icon: '🔄' })
            return
          }

          // Address is still valid; restore the session.
          setWallet({ address: saved, displayAddress: shortAddr(saved) })
        } catch {
          // Not connected — clear the stale session.
          localStorage.removeItem('tlv_wallet_address')
        }
      } catch (e) {
        console.error('Session restore failed:', e)
        localStorage.removeItem('tlv_wallet_address')
      }
    }

    restore()
  }, [walletKitRef])

  const connect = useCallback(async () => {
    setConnecting(true)
    try {
      if (!walletKitRef.current) {
        toast.error('Wallet initialization failed. Please refresh the page.')
        return
      }

      // Get the address from the wallet kit (this will prompt the user)
      const result = await walletKitRef.current.getAddress()
      if (!result || !result.address) {
        toast.error('Could not get address from wallet')
        return
      }

      const { address } = result
      const info: WalletInfo = { address, displayAddress: shortAddr(address) }
      setWallet(info)
      localStorage.setItem('tlv_wallet_address', address)
      toast.success(`Connected: ${shortAddr(address)}`)
    } catch (e) {
      const msg = e instanceof Error ? e.message : 'Failed to connect wallet'
      // Suppress rejection/cancellation messages from user-initiated cancellations
      if (!msg.toLowerCase().includes('reject') && !msg.toLowerCase().includes('cancel')) {
        toast.error(msg, { duration: 8000 })
      }
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
      if (!walletKitRef.current) {
        toast.error('Wallet not initialized')
        return null
      }

      const result = await walletKitRef.current.signTransaction(txXdr, {
        networkPassphrase: CONFIG.NETWORK_PASSPHRASE,
      })

      if (!result || !result.signedTxXdr) {
        toast.error('Failed to sign transaction')
        return null
      }

      return result.signedTxXdr
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
