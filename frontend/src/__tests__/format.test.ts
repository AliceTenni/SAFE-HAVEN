import { describe, it, expect } from 'vitest'
import {
  stroopsToXlm,
  xlmToStroops,
  formatCountdown,
  formatBps,
  shortAddr,
  isValidStellarAddress,
} from '../lib/format'

// ================================================================
//  stroopsToXlm
// ================================================================

describe('stroopsToXlm', () => {
  it('converts 0 stroops to "0.0000000"', () => {
    expect(stroopsToXlm(0n)).toBe('0.0000000')
  })

  it('converts 10_000_000 stroops (1 XLM) to "1.0000000"', () => {
    expect(stroopsToXlm(10_000_000n)).toBe('1.0000000')
  })

  it('converts 1 stroop to "0.0000001"', () => {
    expect(stroopsToXlm(1n)).toBe('0.0000001')
  })

  it('converts 12_345_678 stroops to "1.2345678"', () => {
    expect(stroopsToXlm(12_345_678n)).toBe('1.2345678')
  })

  it('handles large values (1 billion stroops)', () => {
    expect(stroopsToXlm(1_000_000_000n)).toBe('100.0000000')
  })

  it('pads fractional part correctly', () => {
    expect(stroopsToXlm(10_000_100n)).toBe('1.0000100')
  })

  it('handles values with no fractional part', () => {
    expect(stroopsToXlm(50_000_000n)).toBe('5.0000000')
  })
})

// ================================================================
//  xlmToStroops
// ================================================================

describe('xlmToStroops', () => {
  it('converts "1.0000000" to 10_000_000n', () => {
    expect(xlmToStroops('1.0000000')).toBe(10_000_000n)
  })

  it('converts "0.0000001" to 1n', () => {
    expect(xlmToStroops('0.0000001')).toBe(1n)
  })

  it('converts "0" (no fraction) to 0n', () => {
    expect(xlmToStroops('0')).toBe(0n)
  })

  it('converts "5" (no fraction) to 50_000_000n', () => {
    expect(xlmToStroops('5')).toBe(50_000_000n)
  })

  it('converts "0.1" (short fraction) to 1_000_000n', () => {
    expect(xlmToStroops('0.1')).toBe(1_000_000n)
  })

  it('converts "0.01" to 100_000n', () => {
    expect(xlmToStroops('0.01')).toBe(100_000n)
  })

  it('round-trip: xlmToStroops(stroopsToXlm(x)) === x', () => {
    const cases = [0n, 1n, 10_000_000n, 12_345_678n, 1_000_000_000n]
    for (const s of cases) {
      expect(xlmToStroops(stroopsToXlm(s))).toBe(s)
    }
  })

  it('handles fraction longer than 7 digits by truncating', () => {
    // "0.12345678" -> pads "12345678" to 7 -> "1234567" so 1_234_567 stroops
    expect(xlmToStroops('0.12345678')).toBe(1_234_567n)
  })

  it('handles very small amounts correctly', () => {
    expect(xlmToStroops('0.0000000')).toBe(0n)
  })
})

// ================================================================
//  formatCountdown
// ================================================================

describe('formatCountdown', () => {
  it('returns "Unlocked" for null', () => {
    expect(formatCountdown(null)).toBe('Unlocked')
  })

  it('returns "Unlocked" for 0', () => {
    expect(formatCountdown(0)).toBe('Unlocked')
  })

  it('returns "Unlocked" for negative values', () => {
    expect(formatCountdown(-5)).toBe('Unlocked')
  })

  it('formats seconds only (< 1 day)', () => {
    expect(formatCountdown(65)).toBe('1m 5s')
  })

  it('formats minutes and seconds', () => {
    expect(formatCountdown(125)).toBe('2m 5s')
  })

  it('formats hours, minutes, seconds', () => {
    expect(formatCountdown(3661)).toBe('1h 1m 1s')
  })

  it('formats days and hours (no seconds for >= 1 day)', () => {
    expect(formatCountdown(90000)).toBe('1d 1h')
  })

  it('formats multiple days', () => {
    // 2d exactly: h=0, m=0 are skipped; seconds skipped when d>0
    expect(formatCountdown(172800)).toBe('2d')
  })

  it('handles exactly one second', () => {
    expect(formatCountdown(1)).toBe('1s')
  })

  it('formats exactly one hour', () => {
    // d=0 so seconds shown; m=0 so skipped
    expect(formatCountdown(3600)).toBe('1h 0s')
  })
})

// ================================================================
//  formatBps
// ================================================================

describe('formatBps', () => {
  it('formats 0 bps as "0.00%"', () => {
    expect(formatBps(0)).toBe('0.00%')
  })

  it('formats 100 bps as "1.00%"', () => {
    expect(formatBps(100)).toBe('1.00%')
  })

  it('formats 10000 bps as "100.00%"', () => {
    expect(formatBps(10000)).toBe('100.00%')
  })

  it('formats 50 bps as "0.50%"', () => {
    expect(formatBps(50)).toBe('0.50%')
  })

  it('formats 1 bps as "0.01%"', () => {
    expect(formatBps(1)).toBe('0.01%')
  })

  it('formats 1234 bps as "12.34%"', () => {
    expect(formatBps(1234)).toBe('12.34%')
  })
})

// ================================================================
//  shortAddr
// ================================================================

describe('shortAddr', () => {
  it('shortens a long Stellar address', () => {
    const addr = 'GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H'
    const result = shortAddr(addr)
    // Default: first 6 chars + … + last 4 chars
    expect(result).toBe('GBRPYH…OX2H')
  })

  it('shortens with custom chars', () => {
    const addr = 'GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H'
    const result = shortAddr(addr, 4)
    expect(result).toBe('GBRP…OX2H')
  })

  it('does not shorten a short address', () => {
    const addr = 'GABC'
    expect(shortAddr(addr)).toBe('GABC')
  })
})

// ================================================================
//  isValidStellarAddress
// ================================================================

describe('isValidStellarAddress', () => {
  it('validates a correct G-address', () => {
    // Must be exactly 55 chars (G + 54 base32 chars) per the current regex
    // G-address pattern: ^G[A-Z2-7]{54}$
    // 55-character valid G-address
    expect(isValidStellarAddress('GABCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGHIJKLMNOPQRSTUV')).toBe(true)
  })

  it('validates a correct C-address', () => {
    expect(isValidStellarAddress('CABCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGHIJKLMNOPQRSTUV')).toBe(true)
  })

  it('rejects an empty string', () => {
    expect(isValidStellarAddress('')).toBe(false)
  })

  it('rejects a string that is too short', () => {
    expect(isValidStellarAddress('GABC')).toBe(false)
  })

  it('rejects a string that starts with invalid prefix', () => {
    // 55-char address with X prefix
    expect(isValidStellarAddress('XABCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGHIJKLMNOPQRSTUV')).toBe(false)
  })

  it('rejects a string with invalid characters', () => {
    // '0' is not in base32 alphabet (A-Z, 2-7)
    expect(isValidStellarAddress('G0BCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGHIJKLMNOPQRSTUV')).toBe(false)
  })
})
