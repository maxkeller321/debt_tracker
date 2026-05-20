import { describe, it, expect } from 'vitest';
import { formatMoney, formatDate } from './format';

describe('formatMoney', () => {
  it('formats EUR minor units', () => {
    const s = formatMoney(12345, 'EUR');
    expect(s).toContain('123');
  });
});

describe('formatDate', () => {
  it('returns placeholder for null', () => {
    expect(formatDate(null)).toBe('No payments recorded yet');
  });

  it('formats iso date', () => {
    expect(formatDate('2025-06-01')).not.toBe('No payments recorded yet');
  });
});
