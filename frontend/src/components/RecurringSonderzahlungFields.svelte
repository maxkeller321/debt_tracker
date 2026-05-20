<script lang="ts">
  import { t } from '../lib/i18n';

  export type RecurringEntry = { amount: string; month: string; day: string };

  export let entries: RecurringEntry[] = [{ amount: '', month: '12', day: '1' }];

  $: tr = $t;

  export function addEntry() {
    entries = [...entries, { amount: '', month: '12', day: '1' }];
  }

  export function removeEntry(index: number) {
    entries = entries.filter((_, i) => i !== index);
  }

  export function toApiPayload() {
    return entries
      .filter((e) => e.amount && parseFloat(e.amount) > 0)
      .map((e) => ({
        amount_minor: Math.round(parseFloat(e.amount) * 100),
        month: parseInt(e.month, 10),
        day: parseInt(e.day, 10),
      }));
  }
</script>

<fieldset class="recurring">
  <legend>{tr('recurring.legend')} <span class="muted">{tr('recurring.subtitle')}</span></legend>
  {#each entries as entry, i}
    <div class="row">
      <input type="number" step="0.01" placeholder={tr('recurring.amount')} bind:value={entry.amount} aria-label={tr('recurring.amount')} />
      <select bind:value={entry.month} aria-label={tr('recurring.month')}>
        {#each Array.from({ length: 12 }, (_, m) => m + 1) as m}
          <option value={String(m)}>{m}</option>
        {/each}
      </select>
      <input type="number" min="1" max="28" bind:value={entry.day} aria-label={tr('recurring.day')} />
      {#if entries.length > 1}
        <button type="button" class="secondary" on:click={() => removeEntry(i)}>{tr('recurring.remove')}</button>
      {/if}
    </div>
  {/each}
  <button type="button" class="secondary" on:click={addEntry}>{tr('recurring.add')}</button>
</fieldset>

<style>
  .recurring {
    border: 1px solid #e2e8f0;
    border-radius: 8px;
    padding: 0.75rem;
    margin-bottom: 1rem;
  }
  legend {
    font-size: 0.875rem;
    padding: 0 0.25rem;
  }
  .row {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
    flex-wrap: wrap;
  }
</style>
