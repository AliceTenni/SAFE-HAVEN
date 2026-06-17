import type { TxStatus } from '../types'
import { explorerTxUrl } from '../lib/format'

interface TxStatusBadgeProps {
  status: TxStatus
  txHash?: string
  error?: string
}

export function TxStatusBadge({ status, txHash, error }: TxStatusBadgeProps) {
  if (status === 'idle') return null

  const configs: Record<TxStatus, { label: string; className: string; spin?: boolean }> = {
    idle:        { label: '',                    className: '' },
    signing:     { label: 'Waiting for signature…', className: 'badge-blue', spin: true },
    submitting:  { label: 'Submitting…',         className: 'badge-blue', spin: true },
    confirming:  { label: 'Confirming…',          className: 'badge-yellow', spin: true },
    success:     { label: 'Transaction confirmed', className: 'badge-green' },
    error:       { label: error ?? 'Transaction failed', className: 'badge-red' },
  }

  const cfg = configs[status]
  if (!cfg.label) return null

  return (
    <div className={`${cfg.className} flex items-center gap-2 text-sm px-3 py-2 rounded-xl`}>
      {cfg.spin && (
        <span className="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin flex-shrink-0" />
      )}
      <span className="truncate">{cfg.label}</span>
      {status === 'success' && txHash && (
        <a
          href={explorerTxUrl(txHash)}
          target="_blank"
          rel="noopener noreferrer"
          className="underline hover:no-underline whitespace-nowrap"
        >
          View tx
        </a>
      )}
    </div>
  )
}
