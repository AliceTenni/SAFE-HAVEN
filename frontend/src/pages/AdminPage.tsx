import { useState } from 'react'
import toast from 'react-hot-toast'
import { useWallet } from '../context/WalletContext'
import { TxStatusBadge } from '../components/TxStatusBadge'
import { TwoFAVerification } from '../components/TwoFAVerification'
import { buildPause, buildUnpause, buildEmergencyWithdraw, submitTx } from '../lib/stellar'
import { shortAddr, explorerAddrUrl } from '../lib/format'
import { use2FA } from '../hooks/use2FA'
import type { TxStatus } from '../types'
import type { ContractInfo } from '../App'

interface AdminPageProps {
  contractInfo: ContractInfo
  onContractInfoRefresh: () => void
}

export function AdminPage({ contractInfo, onContractInfoRefresh }: AdminPageProps) {
  const { wallet, signTransaction } = useWallet()
  const { twoFAState } = use2FA()

  // Pause/unpause
  const [pauseTxStatus, setPauseTxStatus] = useState<TxStatus>('idle')
  const [pauseTxHash,   setPauseTxHash]   = useState<string | undefined>()
  const [pauseTxError,  setPauseTxError]  = useState<string | undefined>()

  // Emergency withdraw
  const [emrgDepositor,  setEmrgDepositor]  = useState('')
  const [emrgDepositId,  setEmrgDepositId]  = useState('')
  const [emrgTxStatus,   setEmrgTxStatus]   = useState<TxStatus>('idle')
  const [emrgTxHash,     setEmrgTxHash]     = useState<string | undefined>()
  const [emrgTxError,    setEmrgTxError]    = useState<string | undefined>()

  // 2FA states
  const [show2FA, setShow2FA] = useState(false)
  const [pendingAction, setPendingAction] = useState<'pause' | 'unpause' | 'emergency' | null>(null)

  const isAdmin = wallet?.address === contractInfo.admin
  const pausePending = pauseTxStatus === 'signing' || pauseTxStatus === 'submitting' || pauseTxStatus === 'confirming'
  const emrgPending  = emrgTxStatus  === 'signing' || emrgTxStatus  === 'submitting' || emrgTxStatus  === 'confirming'

  async function handleTogglePause() {
    if (!wallet) return

    // Check if 2FA is required
    if (twoFAState.enabled) {
      setPendingAction(contractInfo.paused ? 'unpause' : 'pause')
      setShow2FA(true)
      return
    }

    // Proceed without 2FA
    await executePauseToggle()
  }

  async function executePauseToggle() {
    if (!wallet) return
    setPauseTxStatus('signing')
    setPauseTxError(undefined)
    setPauseTxHash(undefined)

    try {
      const xdr = contractInfo.paused
        ? await buildUnpause(wallet.address)
        : await buildPause(wallet.address)

      if (!xdr) throw new Error('Failed to build transaction')
      const signed = await signTransaction(xdr)
      if (!signed) { setPauseTxStatus('idle'); return }

      setPauseTxStatus('submitting')
      const result = await submitTx(signed)
      if (result.success) {
        setPauseTxStatus('success')
        setPauseTxHash(result.txHash)
        toast.success(contractInfo.paused ? 'Contract unpaused.' : 'Contract paused.')
        setTimeout(onContractInfoRefresh, 1500)
      } else {
        setPauseTxStatus('error')
        setPauseTxError(result.error)
        toast.error(result.error ?? 'Transaction failed')
      }
    } catch (e) {
      const msg = e instanceof Error ? e.message : 'Unexpected error'
      setPauseTxStatus('error')
      setPauseTxError(msg)
      toast.error(msg)
    }
  }

  async function handleEmergencyWithdraw(e: React.FormEvent) {
    e.preventDefault()
    if (!wallet || !emrgDepositor || !emrgDepositId) return

    // Check if 2FA is required
    if (twoFAState.enabled) {
      setPendingAction('emergency')
      setShow2FA(true)
      return
    }

    // Proceed without 2FA
    await executeEmergencyWithdraw()
  }

  async function executeEmergencyWithdraw() {
    if (!wallet || !emrgDepositor || !emrgDepositId) return

    setEmrgTxStatus('signing')
    setEmrgTxError(undefined)
    setEmrgTxHash(undefined)

    try {
      const xdr = await buildEmergencyWithdraw(wallet.address, emrgDepositor, parseInt(emrgDepositId, 10))
      if (!xdr) throw new Error('Failed to build transaction')

      const signed = await signTransaction(xdr)
      if (!signed) { setEmrgTxStatus('idle'); return }

      setEmrgTxStatus('submitting')
      const result = await submitTx(signed)
      if (result.success) {
        setEmrgTxStatus('success')
        setEmrgTxHash(result.txHash)
        toast.success('Emergency withdrawal successful. Funds returned to depositor.')
        setEmrgDepositor('')
        setEmrgDepositId('')
      } else {
        setEmrgTxStatus('error')
        setEmrgTxError(result.error)
        toast.error(result.error ?? 'Emergency withdrawal failed')
      }
    } catch (e) {
      const msg = e instanceof Error ? e.message : 'Unexpected error'
      setEmrgTxStatus('error')
      setEmrgTxError(msg)
      toast.error(msg)
    }
  }

  const handle2FAVerified = () => {
    setShow2FA(false)
    if (pendingAction === 'pause' || pendingAction === 'unpause') {
      void executePauseToggle()
    } else if (pendingAction === 'emergency') {
      void executeEmergencyWithdraw()
    }
    setPendingAction(null)
  }

  if (!wallet) {
    return (
      <div className="card p-10 text-center max-w-lg">
        <p className="text-slate-400">Connect your wallet to access admin controls.</p>
      </div>
    )
  }

  if (!isAdmin) {
    return (
      <div className="card p-10 text-center max-w-lg">
        <div className="w-12 h-12 rounded-xl bg-red-900/30 border border-red-700/40 flex items-center justify-center mx-auto mb-4">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5} className="w-6 h-6 text-red-400">
            <path strokeLinecap="round" strokeLinejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
          </svg>
        </div>
        <p className="font-medium text-red-400">Not authorized</p>
        <p className="text-sm text-slate-400 mt-1">
          Connected address is not the contract admin.
        </p>
        {contractInfo.admin && (
          <p className="text-xs text-slate-500 mt-3 font-mono">
            Admin:{' '}
            <a
              href={explorerAddrUrl(contractInfo.admin)}
              target="_blank"
              rel="noopener noreferrer"
              className="text-stellar-400 hover:text-stellar-300"
            >
              {shortAddr(contractInfo.admin)}
            </a>
          </p>
        )}
      </div>
    )
  }

  return (
    <div className="max-w-lg space-y-5">
      {/* Contract status card */}
      <div className="card p-6">
        <h2 className="font-semibold text-lg mb-4">Contract Status</h2>
        <div className="grid grid-cols-2 gap-y-3 text-sm mb-5">
          <span className="text-slate-400">Admin</span>
          <a
            href={explorerAddrUrl(contractInfo.admin!)}
            target="_blank"
            rel="noopener noreferrer"
            className="font-mono text-stellar-400 hover:text-stellar-300 truncate"
          >
            {shortAddr(contractInfo.admin!)}
          </a>

          <span className="text-slate-400">Status</span>
          <span>
            {contractInfo.paused
              ? <span className="badge-red">Paused</span>
              : <span className="badge-green">Active</span>
            }
          </span>

          <span className="text-slate-400">Depositors</span>
          <span className="text-slate-200">{contractInfo.depositorCount}</span>

          {contractInfo.feeRecipient && (
            <>
              <span className="text-slate-400">Fee recipient</span>
              <a
                href={explorerAddrUrl(contractInfo.feeRecipient)}
                target="_blank"
                rel="noopener noreferrer"
                className="font-mono text-stellar-400 hover:text-stellar-300 truncate"
              >
                {shortAddr(contractInfo.feeRecipient)}
              </a>
            </>
          )}
        </div>

        <TxStatusBadge status={pauseTxStatus} txHash={pauseTxHash} error={pauseTxError} />

        <div className="mt-4">
          <button
            className={contractInfo.paused ? 'btn-primary w-full' : 'btn-danger w-full'}
            onClick={handleTogglePause}
            disabled={pausePending}
          >
            {pausePending
              ? <span className="w-4 h-4 border-2 border-current/30 border-t-current rounded-full animate-spin" />
              : contractInfo.paused ? 'Unpause contract' : 'Pause contract'
            }
          </button>
        </div>
      </div>

      {/* Emergency withdrawal */}
      <div className="card p-6">
        <h2 className="font-semibold text-lg mb-1">Emergency Withdrawal</h2>
        <p className="text-sm text-slate-400 mb-5">
          Returns locked tokens directly to the depositor, bypassing the time lock. Funds always go to the depositor — never to admin.
        </p>

        <form onSubmit={handleEmergencyWithdraw} className="space-y-4">
          <div>
            <label className="label">Depositor address</label>
            <input
              className="input"
              type="text"
              value={emrgDepositor}
              onChange={(e) => setEmrgDepositor(e.target.value.trim())}
              placeholder="G... or C..."
              disabled={emrgPending}
            />
          </div>

          <div>
            <label className="label">Deposit ID</label>
            <input
              className="input"
              type="number"
              min="0"
              value={emrgDepositId}
              onChange={(e) => setEmrgDepositId(e.target.value)}
              placeholder="0"
              disabled={emrgPending}
            />
          </div>

          <TxStatusBadge status={emrgTxStatus} txHash={emrgTxHash} error={emrgTxError} />

          <button
            type="submit"
            className="btn-danger w-full"
            disabled={!emrgDepositor || !emrgDepositId || emrgPending}
          >
            {emrgPending
              ? <span className="w-4 h-4 border-2 border-current/30 border-t-current rounded-full animate-spin" />
              : 'Emergency withdraw'
            }
          </button>
        </form>
      </div>

      {/* 2FA verification modal */}
      {show2FA && (
        <TwoFAVerification
          onVerified={handle2FAVerified}
          onCancel={() => {
            setShow2FA(false)
            setPendingAction(null)
          }}
        />
      )}
    </div>
  )
}
