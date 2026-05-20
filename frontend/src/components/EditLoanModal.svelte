<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { api } from '../lib/api/client';
  import { t } from '../lib/i18n';

  export let open = false;
  export let loanId = '';
  export let initialLabel = '';
  export let initialAprPercent: number | null = null;
  export let initialFixedPayment: number | null = null;
  export let initialPaymentType: 'fixed' | 'apr' = 'fixed';

  const dispatch = createEventDispatcher<{ saved: void; close: void }>();

  let label = '';
  let fixedPayment = '';
  let aprPercent = '';
  let paymentType: 'fixed' | 'apr' = 'fixed';
  let error = '';

  $: tr = $t;

  $: if (open) {
    label = initialLabel;
    paymentType = initialPaymentType;
    aprPercent = initialAprPercent != null ? String(initialAprPercent) : '';
    fixedPayment =
      initialFixedPayment != null ? String(initialFixedPayment / 100) : '';
  }

  function fieldValue(id: string, fallback: string) {
    const el = document.getElementById(id) as HTMLInputElement | null;
    return (el?.value ?? fallback).trim();
  }

  async function save() {
    error = '';
    const aprVal = fieldValue('edit-apr', aprPercent);
    const fixedVal = fieldValue('edit-fixed', fixedPayment);
    if (!aprVal) {
      error = tr('errors.aprRequired');
      return;
    }
    const body: Record<string, unknown> = {
      label,
      payment_type: paymentType,
      apr_basis_points: Math.round(parseFloat(aprVal) * 100),
    };
    if (paymentType === 'fixed') {
      if (!fixedVal) {
        error = tr('errors.fixedRequired');
        return;
      }
      body.fixed_payment_minor = Math.round(parseFloat(fixedVal) * 100);
    }
    try {
      await api.updateLoan(loanId, body);
      dispatch('saved');
      dispatch('close');
    } catch (e) {
      error = e instanceof Error ? e.message : tr('errors.updateFailed');
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
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
      class="modal"
      role="dialog"
      aria-modal="true"
      aria-labelledby="edit-loan-title"
      tabindex="0"
      on:click|stopPropagation={() => {}}
      on:keydown|stopPropagation={() => {}}
    >
      <h2 id="edit-loan-title">{tr('editLoan.title')}</h2>
      <div class="field">
        <label for="edit-label">{tr('editLoan.name')}</label>
        <input id="edit-label" bind:value={label} />
      </div>
      <div class="field">
        <label for="edit-apr">{tr('editLoan.interestRate')}</label>
        <input id="edit-apr" type="number" step="0.01" min="0" bind:value={aprPercent} required />
      </div>
      <div class="field">
        <label for="edit-ptype">{tr('editLoan.paymentMethod')}</label>
        <select id="edit-ptype" bind:value={paymentType}>
          <option value="fixed">{tr('addLoan.fixedOption')}</option>
          <option value="apr">{tr('addLoan.aprOption')}</option>
        </select>
      </div>
      {#if paymentType === 'fixed'}
        <div class="field">
          <label for="edit-fixed">{tr('editLoan.periodicPayment')}</label>
          <input id="edit-fixed" type="number" step="0.01" bind:value={fixedPayment} required />
        </div>
      {/if}
      {#if error}<p class="error">{error}</p>{/if}
      <div class="actions">
        <button type="button" class="secondary" on:click={() => dispatch('close')}>{tr('common.cancel')}</button>
        <button type="button" on:click={save}>{tr('common.saveChanges')}</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    margin-top: 1rem;
  }
</style>
