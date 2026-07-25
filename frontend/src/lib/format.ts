// ============================================================
//  Formatting helpers
// ============================================================

import { formatDistanceToNow, format, fromUnixTime, isPast } from 'date-fns'
import { CONFIG } from '../config'

/** Convert stroops → XLM string, e.g. 10_000_000n → "1.0000000" */
export function stroopsToXlm(stroops: bigint): string {
  const whole = stroops / BigInt(CONFIG.STROOPS_PER_XLM)
  const frac  = stroops % BigInt(CONFIG.STROOPS_PER_XLM)
  const fracStr = frac.toString().padStart(7, '0')
  return `${whole}.${fracStr}`
}

/** Convert XLM string → stroops bigint */
export function xlmToStroops(xlm: string): bigint {
  const [whole, frac = ''] = xlm.split('.')
  const fracPadded = frac.padEnd(7, '0').slice(0, 7)
  return BigInt(whole) * BigInt(CONFIG.STROOPS_PER_XLM) + BigInt(fracPadded)
}

/** Format a Unix timestamp as a human-readable date+time string */
export function formatUnlockDate(unixSecs: number): string {
  return format(fromUnixTime(unixSecs), 'MMM d, yyyy HH:mm')
}

/** Format a Unix timestamp as a relative string like "in 3 days" */
export function formatRelativeTime(unixSecs: number): string {
  const date = fromUnixTime(unixSecs)
  if (isPast(date)) return 'Unlocked'
  return formatDistanceToNow(date, { addSuffix: true })
}

/** Format seconds duration into "2d 4h 30m 10s" */
export function formatCountdown(seconds: number | null): string {
  if (seconds === null || seconds <= 0) return 'Unlocked'
  const d = Math.floor(seconds / 86400)
  const h = Math.floor((seconds % 86400) / 3600)
  const m = Math.floor((seconds % 3600) / 60)
  const s = seconds % 60
  const parts: string[] = []
  if (d > 0) parts.push(`${d}d`)
  if (h > 0) parts.push(`${h}h`)
  if (m > 0) parts.push(`${m}m`)
  if (d === 0) parts.push(`${s}s`) // show seconds only when < 1 day
  return parts.join(' ')
}

/** Format basis points as a percentage string */
export function formatBps(bps: number): string {
  return `${(bps / 100).toFixed(2)}%`
}

/** Shorten a Stellar address for display */
export function shortAddr(addr: string, chars = 6): string {
  if (addr.length <= chars * 2 + 1) return addr
  return `${addr.slice(0, chars)}…${addr.slice(-4)}`
}

/** Explorer URL for a transaction */
export function explorerTxUrl(txHash: string): string {
  return `${CONFIG.EXPLORER_URL}/tx/${txHash}`
}

/** Explorer URL for an address */
export function explorerAddrUrl(addr: string): string {
  return `${CONFIG.EXPLORER_URL}/account/${addr}`
}
