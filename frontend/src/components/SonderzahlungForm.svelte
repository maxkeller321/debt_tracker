<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { todayIso } from '../lib/format';
  import { t } from '../lib/i18n';

  export let disabled = false;

  const dispatch = createEventDispatcher<{
    immediate: { amount: string; date: string };
    schedule: { amount: string; date: string };
  }>();

  let extraAmount = '';
  let extraDate = todayIso();
  let scheduleAmount = '';
  let scheduleDate = todayIso();

  $: tr = $t;
</script>

<section class="sonderzahlung">
  <h3>{tr('sonder.title')} <span class="muted">{tr('sonder.subtitle')}</span></h3>
  <div class="row">
    <input
      type="number"
      step="0.01"
      placeholder={tr('sonder.extraAmount')}
      aria-label={tr('sonder.extraAmount')}
      bind:value={extraAmount}
      disabled={disabled}
    />
    <input type="date" bind:value={extraDate} disabled={disabled} />
    <button
      type="button"
      disabled={disabled || !extraAmount}
      on:click={() => dispatch('immediate', { amount: extraAmount, date: extraDate })}
    >{tr('sonder.applyNow')}</button>
  </div>
  <div class="row">
    <input
      type="number"
      step="0.01"
      placeholder={tr('sonder.scheduleAmount')}
      bind:value={scheduleAmount}
      disabled={disabled}
    />
    <input type="date" bind:value={scheduleDate} disabled={disabled} />
    <button
      type="button"
      class="secondary"
      disabled={disabled || !scheduleAmount}
      on:click={() => dispatch('schedule', { amount: scheduleAmount, date: scheduleDate })}
    >{tr('sonder.schedule')}</button>
  </div>
</section>

<style>
  h3 {
    font-size: 0.95rem;
    margin: 1rem 0 0.5rem;
  }
  .row {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
    flex-wrap: wrap;
  }
</style>
