const API_BASE = import.meta.env.VITE_API_BASE ?? '/api/v1';

export interface Money {
  amount_minor: number;
  currency: string;
}

export interface LoanSummary {
  id: string;
  label: string;
  remaining_balance: Money;
  periodic_payment: Money;
  payment_frequency: string;
  last_payment_date: string | null;
  projected_payoff_date: string | null;
  progress_percent: number;
}

export interface DashboardResponse {
  household: {
    total_balance: Money;
    total_monthly_obligation: Money;
  };
  loans: LoanSummary[];
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${API_BASE}${path}`, {
    headers: { 'Content-Type': 'application/json', ...init?.headers },
    ...init,
  });
  if (!res.ok) {
    const err = await res.json().catch(() => ({ error: res.statusText }));
    throw new Error((err as { error?: string }).error ?? res.statusText);
  }
  if (res.status === 204) return undefined as T;
  return res.json() as Promise<T>;
}

export const api = {
  dashboard: (includeArchived = false) =>
    request<DashboardResponse>(`/dashboard?include_archived=${includeArchived}`),

  createLoan: (body: unknown) =>
    request<Record<string, unknown>>('/loans', { method: 'POST', body: JSON.stringify(body) }),

  loanDetail: (id: string) => request<Record<string, unknown>>(`/loans/${id}`),

  updateLoan: (id: string, body: unknown) =>
    request<Record<string, unknown>>(`/loans/${id}`, {
      method: 'PATCH',
      body: JSON.stringify(body),
    }),

  listPayments: (id: string) =>
    request<unknown[]>(`/loans/${id}/payments`),

  immediateSonderzahlung: (id: string, body: unknown) =>
    request<unknown>(`/loans/${id}/sonderzahlungen/immediate`, {
      method: 'POST',
      body: JSON.stringify(body),
    }),

  scheduleSonderzahlung: (id: string, body: unknown) =>
    request<unknown>(`/loans/${id}/sonderzahlungen/scheduled`, {
      method: 'POST',
      body: JSON.stringify(body),
    }),

  cancelScheduled: (loanId: string, scheduleId: string) =>
    request<void>(`/loans/${loanId}/sonderzahlungen/scheduled/${scheduleId}`, {
      method: 'DELETE',
    }),

  recordPayment: (id: string, body: unknown) =>
    request<unknown>(`/loans/${id}/payments`, {
      method: 'POST',
      body: JSON.stringify(body),
    }),

  exportData: () => request<unknown>('/export'),

  importData: (body: unknown) =>
    request<unknown>('/import?confirm=true', {
      method: 'POST',
      body: JSON.stringify(body),
    }),

  deleteLoan: (id: string) =>
    request<void>(`/loans/${id}?confirm=true`, { method: 'DELETE' }),

  archiveLoan: (id: string) =>
    request<unknown>(`/loans/${id}/archive`, { method: 'POST' }),
};
