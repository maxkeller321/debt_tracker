<script lang="ts">
  import { formatDate, formatMoney } from '../lib/format';
  import { t } from '../lib/i18n';

  export let items: Array<{
    id: string;
    amount_minor: number;
    due_date: string;
    status: string;
  }> = [];

  export let onCancel: ((id: string) => void) | undefined = undefined;

  $: tr = $t;
</script>

{#if items.length > 0}
  <section class="upcoming">
    <h3>{tr('upcoming.title')}</h3>
    <ul>
      {#each items as item (item.id)}
        <li>
          <span>{formatMoney(item.amount_minor)} {tr('upcoming.on')} {formatDate(item.due_date)}</span>
          {#if onCancel && item.status === 'pending'}
            <button type="button" class="secondary small" on:click={() => onCancel(item.id)}>{tr('upcoming.cancel')}</button>
          {/if}
        </li>
      {/each}
    </ul>
  </section>
{/if}

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
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.35rem 0;
    border-bottom: 1px solid #f1f5f9;
  }
  .small {
    padding: 0.25rem 0.5rem;
    font-size: 0.8rem;
  }
</style>
