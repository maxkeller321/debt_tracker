<script lang="ts">
  import type { DashboardResponse } from '../lib/api/client';
  import { formatMoney } from '../lib/format';
  import { t } from '../lib/i18n';

  export let data: DashboardResponse['household'];

  $: tr = $t;
</script>

<div class="card summary">
  <h2>{tr('household.title')}</h2>
  <div class="grid">
    <div>
      <span class="muted">{tr('household.totalRemaining')}</span>
      <strong>{formatMoney(data.total_balance.amount_minor, data.total_balance.currency)}</strong>
    </div>
    <div>
      <span class="muted">{tr('household.monthlyObligation')}</span>
      <strong>{formatMoney(data.total_monthly_obligation.amount_minor, data.total_monthly_obligation.currency)}</strong>
    </div>
  </div>
</div>

<style>
  .summary h2 {
    margin: 0 0 0.75rem;
    font-size: 1.1rem;
  }
  .grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }
  .grid div {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
</style>
