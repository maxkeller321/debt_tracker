export function formatMoney(amountMinor: number, currency = 'EUR'): string {
  const value = amountMinor / 100;
  return new Intl.NumberFormat(undefined, { style: 'currency', currency }).format(value);
}

export function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—';
  try {
    return new Date(iso).toLocaleDateString();
  } catch {
    return iso;
  }
}

export function formatLastPayment(iso: string | null | undefined): string {
  if (!iso) return 'No payments recorded yet';
  return formatDate(iso);
}

export function todayIso(): string {
  return new Date().toISOString().slice(0, 10);
}
