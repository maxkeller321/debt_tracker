<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { LoanSummary } from '../lib/api/client';
  import { api } from '../lib/api/client';
  import { formatDate, formatLastPayment, formatMoney } from '../lib/format';
  import { t } from '../lib/i18n';
  import SonderzahlungForm from './SonderzahlungForm.svelte';
  import UpcomingPayments from './UpcomingPayments.svelte';
  import InterestSummary from './InterestSummary.svelte';
  import PaymentHistory from './PaymentHistory.svelte';
  import EditLoanModal from './EditLoanModal.svelte';

  export let loan: LoanSummary;
  export let expanded = false;

  const dispatch = createEventDispatcher<{ refresh: void }>();

  let detail: Record<string, unknown> | null = null;
  let error = '';
  let showEdit = false;

  $: tr = $t;
  $: freqLabel =
    loan.payment_frequency === 'yearly'
      ? tr('loan.frequency.yearly')
      : tr('loan.frequency.monthly');

  async function loadDetail() {
    detail = await api.loanDetail(loan.id);
  }

  async function toggle() {
    expanded = !expanded;
    if (expanded) await loadDetail();
  }

  async function refresh() {
    await loadDetail();
    dispatch('refresh');
  }

  async function onImmediate(e: CustomEvent<{ amount: string; date: string }>) {
    error = '';
    try {
      await api.immediateSonderzahlung(loan.id, {
        amount_minor: Math.round(parseFloat(e.detail.amount) * 100),
        paid_at: e.detail.date,
        confirm_overpayment: true,
      });
      await refresh();
    } catch (err) {
      error = err instanceof Error ? err.message : tr('errors.generic');
    }
  }

  async function onSchedule(e: CustomEvent<{ amount: string; date: string }>) {
    try {
      await api.scheduleSonderzahlung(loan.id, {
        amount_minor: Math.round(parseFloat(e.detail.amount) * 100),
        due_date: e.detail.date,
      });
      await refresh();
    } catch (err) {
      error = err instanceof Error ? err.message : tr('errors.generic');
    }
  }

  async function cancelScheduled(id: string) {
    await api.cancelScheduled(loan.id, id);
    await refresh();
  }

  async function archive() {
    if (!confirm(tr('loan.confirmArchive'))) return;
    await api.archiveLoan(loan.id);
    dispatch('refresh');
  }

  async function remove() {
    if (!confirm(tr('loan.confirmDelete'))) return;
    await api.deleteLoan(loan.id);
    dispatch('refresh');
  }

  $: upcoming = (detail?.upcoming_scheduled as Array<{
    id: string;
    amount_minor: number;
    due_date: string;
    status: string;
  }>) ?? [];
  $: interestPaid = detail?.interest_paid_to_date as { amount_minor: number } | undefined;
  $: interestRemaining = detail?.interest_remaining_estimate as { amount_minor: number } | undefined;
  $: editApr = (detail?.apr_percent as number | null | undefined) ?? null;
  $: editFixed = detail?.payment_type === 'fixed'
    ? (loan.periodic_payment?.amount_minor ?? null)
    : null;
  $: editPaymentType = (detail?.payment_type as 'fixed' | 'apr') ?? 'fixed';
</script>

<article class="card loan">
  <button type="button" class="header" data-testid="loan-expand" on:click={toggle} aria-expanded={expanded}>
    <div>
      <strong>{loan.label}</strong>
      <span class="muted">{formatMoney(loan.remaining_balance.amount_minor)} · {freqLabel}</span>
    </div>
    <div class="meta">
      <span>{tr('loan.payoff')}: {formatDate(loan.projected_payoff_date)}</span>
      <span>{tr('loan.paidPercent', { percent: loan.progress_percent.toFixed(0) })}</span>
    </div>
  </button>

  {#if expanded}
    <div class="details">
      <p>
        {tr('loan.paymentLine', {
          payment: formatMoney(loan.periodic_payment.amount_minor),
          last: formatLastPayment(loan.last_payment_date),
        })}
      </p>

      {#if detail}
        <InterestSummary
          interestPaidMinor={interestPaid?.amount_minor ?? null}
          interestRemainingMinor={interestRemaining?.amount_minor ?? null}
          message={(detail.interest_message as string) ?? null}
          currency={loan.remaining_balance.currency}
        />
      {/if}

      <p class="muted auto-hint">{tr('loan.autoPaymentsHint')}</p>

      <SonderzahlungForm on:immediate={onImmediate} on:schedule={onSchedule} />
      <UpcomingPayments items={upcoming} onCancel={cancelScheduled} />
      <PaymentHistory loanId={loan.id} />

      <div class="loan-actions">
        <button type="button" class="secondary" on:click={() => (showEdit = true)}>{tr('loan.edit')}</button>
        <button type="button" class="secondary" on:click={archive}>{tr('loan.markPaidOff')}</button>
        <button type="button" class="danger" on:click={remove}>{tr('loan.delete')}</button>
      </div>
      {#if error}<p class="error">{error}</p>{/if}
    </div>
  {/if}
</article>

<EditLoanModal
  open={showEdit}
  loanId={loan.id}
  initialLabel={loan.label}
  initialAprPercent={editApr}
  initialFixedPayment={editFixed}
  initialPaymentType={editPaymentType}
  on:close={() => (showEdit = false)}
  on:saved={() => { showEdit = false; dispatch('refresh'); }}
/>

<style>
  .loan { margin-bottom: 0.75rem; }
  .header {
    width: 100%;
    text-align: left;
    background: transparent;
    color: inherit;
    padding: 0;
    display: flex;
    justify-content: space-between;
    gap: 1rem;
  }
  .meta {
    display: flex;
    flex-direction: column;
    font-size: 0.875rem;
    color: #64748b;
  }
  .details {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid #e2e8f0;
  }
  .loan-actions {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
    flex-wrap: wrap;
  }
  button.danger {
    background: #b91c1c;
  }
  .auto-hint {
    margin: 0.75rem 0;
    font-size: 0.875rem;
  }
</style>
