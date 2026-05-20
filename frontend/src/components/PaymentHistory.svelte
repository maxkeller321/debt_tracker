<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api/client';
  import { formatDate, formatMoney } from '../lib/format';
  import { t } from '../lib/i18n';

  export let loanId: string;

  let events: Array<{
    id: string;
    event_type: string;
    amount_minor: number;
    paid_at: string;
  }> = [];
  let loading = true;

  $: tr = $t;

  onMount(async () => {
    try {
      events = (await api.listPayments(loanId)) as typeof events;
    } finally {
      loading = false;
    }
  });
</script>

<section class="history" aria-label={tr('history.regionLabel')}>
  <h3>{tr('history.title')}</h3>
  {#if loading}
    <p class="muted">{tr('history.loading')}</p>
  {:else if events.length === 0}
    <p class="muted">{tr('history.empty')}</p>
  {:else}
    <ul>
      {#each events as ev (ev.id)}
        <li>
          <span class="type">{ev.event_type === 'sonderzahlung' ? tr('history.extra') : tr('history.regular')}</span>
          <span>{formatMoney(ev.amount_minor)}</span>
          <span class="muted">{formatDate(ev.paid_at)}</span>
        </li>
      {/each}
    </ul>
  {/if}
</section>

<style>
  h3 {
    font-size: 0.95rem;
    margin: 1rem 0 0.5rem;
  }
  ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  li {
    display: grid;
    grid-template-columns: 1fr auto auto;
    gap: 0.5rem;
    padding: 0.35rem 0;
    border-bottom: 1px solid #f1f5f9;
    font-size: 0.875rem;
  }
  .type {
    text-transform: capitalize;
  }
</style>
