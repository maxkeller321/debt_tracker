<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { api } from '../lib/api/client';
  import { todayIso } from '../lib/format';
  import { t } from '../lib/i18n';
  import RecurringSonderzahlungFields from './RecurringSonderzahlungFields.svelte';

  export let open = false;

  const dispatch = createEventDispatcher<{ saved: void; close: void }>();

  let mode: 'quick' | 'advanced' = 'quick';
  let label = '';
  let balance = '';
  let aprPercent = '';
  let paymentType: 'fixed' | 'apr' = 'fixed';
  let fixedPayment = '';
  let frequency: 'monthly' | 'yearly' = 'monthly';
  let startDate = todayIso();
  let error = '';
  let recurringFields: RecurringSonderzahlungFields;

  $: tr = $t;

  function fieldValue(id: string, fallback: string) {
    const el = document.getElementById(id) as HTMLInputElement | null;
    return (el?.value ?? fallback).trim();
  }

  async function save() {
    error = '';
    const aprVal = fieldValue('apr', aprPercent);
    const balanceVal = fieldValue('balance', balance);
    const fixedVal = fieldValue('fixed', fixedPayment);
    if (!aprVal) {
      error = tr('errors.aprRequired');
      return;
    }
    if (paymentType === 'fixed' && !fixedVal) {
      error = tr('errors.fixedRequired');
      return;
    }
    const body: Record<string, unknown> = {
      label,
      setup_mode: mode,
      remaining_balance_minor: Math.round(parseFloat(balanceVal) * 100),
      payment_frequency: frequency,
      payment_type: paymentType,
      apr_basis_points: Math.round(parseFloat(aprVal) * 100),
    };
    if (paymentType === 'fixed') {
      body.fixed_payment_minor = Math.round(parseFloat(fixedVal) * 100);
    }
    if (mode === 'advanced') body.loan_start_date = startDate;
    const recurring = recurringFields?.toApiPayload() ?? [];
    if (recurring.length) body.recurring_sonderzahlungen = recurring;
    try {
      await api.createLoan(body);
      dispatch('saved');
      dispatch('close');
    } catch (e) {
      error = e instanceof Error ? e.message : tr('errors.saveFailed');
    }
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="modal-backdrop"
    role="presentation"
    tabindex="-1"
    on:click={() => dispatch('close')}
    on:keydown={(e) => e.key === 'Escape' && dispatch('close')}
  >
    <div
      class="modal"
      role="dialog"
      aria-modal="true"
      aria-labelledby="add-loan-title"
      tabindex="0"
      on:click|stopPropagation={() => {}}
      on:keydown|stopPropagation={() => {}}
    >
      <h2 id="add-loan-title">{tr('addLoan.title')}</h2>
      <div class="tabs">
        <button type="button" class:secondary={mode !== 'quick'} on:click={() => (mode = 'quick')}>{tr('addLoan.quick')}</button>
        <button type="button" class:secondary={mode !== 'advanced'} on:click={() => (mode = 'advanced')}>{tr('addLoan.advanced')}</button>
      </div>
      <div class="field">
        <label for="label">{tr('addLoan.name')}</label>
        <input id="label" bind:value={label} required />
      </div>
      <div class="field">
        <label for="balance">{tr('addLoan.balance')}</label>
        <input id="balance" type="number" step="0.01" bind:value={balance} required />
      </div>
      <div class="field">
        <label for="apr">{tr('addLoan.interestRate')}</label>
        <input id="apr" type="number" step="0.01" min="0" bind:value={aprPercent} required />
        <p class="hint muted">{tr('addLoan.interestHint')}</p>
      </div>
      <div class="field">
        <label for="ptype">{tr('addLoan.paymentMethod')}</label>
        <select id="ptype" bind:value={paymentType}>
          <option value="fixed">{tr('addLoan.fixedOption')}</option>
          <option value="apr">{tr('addLoan.aprOption')}</option>
        </select>
      </div>
      {#if paymentType === 'fixed'}
        <div class="field">
          <label for="fixed">{tr('addLoan.periodicPayment')}</label>
          <input id="fixed" type="number" step="0.01" bind:value={fixedPayment} required />
        </div>
      {/if}
      <div class="field">
        <label for="freq">{tr('addLoan.frequency')}</label>
        <select id="freq" bind:value={frequency}>
          <option value="monthly">{tr('addLoan.monthly')}</option>
          <option value="yearly">{tr('addLoan.yearly')}</option>
        </select>
      </div>
      {#if mode === 'advanced'}
        <div class="field">
          <label for="start">{tr('addLoan.startDate')}</label>
          <input id="start" type="date" bind:value={startDate} />
        </div>
      {/if}
      <RecurringSonderzahlungFields bind:this={recurringFields} />
      {#if error}<p class="error">{error}</p>{/if}
      <div class="actions">
        <button type="button" class="secondary" on:click={() => dispatch('close')}>{tr('common.cancel')}</button>
        <button type="button" on:click={save}>{tr('common.saveLoan')}</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .tabs { display: flex; gap: 0.5rem; margin-bottom: 1rem; }
  .actions { display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 1rem; }
  .hint { margin: 0.35rem 0 0; font-size: 0.8rem; }
</style>
