import { useState } from 'react'
import { useWallet } from '../context/WalletContext'
import { shortAddr } from '../lib/format'
import { FeedbackModal } from './FeedbackModal'

interface HeaderProps {
  isPaused: boolean
}

export function Header({ isPaused }: HeaderProps) {
  const { wallet, wallets, isConnecting, connect, disconnect, switchWallet } = useWallet()
  const [feedbackOpen, setFeedbackOpen] = useState(false)

  return (
    <header className="sticky top-0 z-30 border-b border-slate-800/80 bg-slate-950/80 backdrop-blur-md">
      <div className="max-w-6xl mx-auto px-4 h-16 flex items-center justify-between gap-4">
        {/* Logo */}
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 rounded-lg bg-stellar-600 flex items-center justify-center flex-shrink-0">
            <svg viewBox="0 0 24 24" fill="none" className="w-5 h-5 text-white" stroke="currentColor" strokeWidth={1.8}>
              <rect x="3" y="3" width="18" height="18" rx="3" />
              <circle cx="12" cy="12" r="3" />
              <path d="M12 7v1M12 16v1M7 12h1M16 12h1" strokeLinecap="round" />
            </svg>
          </div>
          <div>
            <p className="font-semibold text-sm leading-tight">SAFE-HAVEN</p>
            <p className="text-xs text-slate-500 leading-tight">Stellar · Soroban</p>
          </div>
        </div>

        <div className="flex items-center gap-3">
          {/* Contract paused badge */}
          {isPaused && (
            <span className="badge-red hidden sm:flex">
              <span className="w-1.5 h-1.5 rounded-full bg-red-400 animate-pulse" />
              Contract Paused
            </span>
          )}

          <button
            onClick={() => setFeedbackOpen(true)}
            className="btn-secondary text-xs px-3 py-2"
          >
            Feedback
          </button>

          {/* Wallet button */}
          {wallet ? (
            <div className="flex items-center gap-2">
              <div className="hidden sm:block text-right">
                <p className="text-xs text-slate-400">Active wallet</p>
                <p className="text-sm font-mono text-slate-200">{shortAddr(wallet.address)}</p>
              </div>
              {wallets.length > 1 && (
                <select
                  aria-label="Switch wallet"
                  value={wallet.address}
                  onChange={(event) => switchWallet(event.target.value)}
                  className="input max-w-32 text-xs py-2"
                >
                  {wallets.map((connectedWallet) => (
                    <option key={connectedWallet.address} value={connectedWallet.address}>
                      {connectedWallet.displayAddress}
                    </option>
                  ))}
                </select>
              )}
              <button
                onClick={connect}
                disabled={isConnecting}
                className="btn-secondary text-xs px-3 py-2"
                title="Add the currently selected wallet"
              >
                {isConnecting ? 'Connecting…' : 'Add wallet'}
              </button>
              <button
                onClick={disconnect}
                className="btn-secondary text-xs px-3 py-2"
                title="Disconnect wallet"
              >
                Disconnect
              </button>
            </div>
          ) : (
            <button
              onClick={connect}
              disabled={isConnecting}
              className="btn-primary"
            >
              {isConnecting ? (
                <>
                  <span className="w-3.5 h-3.5 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                  Connecting…
                </>
              ) : (
                <>
                  <svg viewBox="0 0 20 20" fill="currentColor" className="w-4 h-4">
                    <path d="M10 2a6 6 0 00-6 6v3.586l-.707.707A1 1 0 004 14h12a1 1 0 00.707-1.707L16 11.586V8a6 6 0 00-6-6z" />
                    <path d="M10 18a3 3 0 01-3-3h6a3 3 0 01-3 3z" />
                  </svg>
                  Connect Wallet
                </>
              )}
            </button>
          )}
        </div>
      </div>
      {feedbackOpen && <FeedbackModal onClose={() => setFeedbackOpen(false)} />}
    </header>
  )
}
