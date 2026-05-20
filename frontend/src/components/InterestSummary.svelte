<script lang="ts">
  import { formatMoney } from '../lib/format';
  import { interestMessage, t } from '../lib/i18n';

  export let interestPaidMinor: number | null = null;
  export let interestRemainingMinor: number | null = null;
  export let message: string | null = null;
  export let currency = 'EUR';

  $: tr = $t;
  $: displayMessage = interestMessage(message, tr);
</script>

<section class="interest" aria-label={tr('interest.regionLabel')}>
  <h3>{tr('interest.title')}</h3>
  {#if displayMessage}
    <p class="muted">{displayMessage}</p>
  {:else}
    <p>
      {tr('interest.paidToDate')}: <strong>{formatMoney(interestPaidMinor ?? 0, currency)}</strong>
    </p>
    <p>
      {tr('interest.remaining')}: <strong>{formatMoney(interestRemainingMinor ?? 0, currency)}</strong>
    </p>
  {/if}
</section>

<style>
  h3 {
    font-size: 0.95rem;
    margin: 1rem 0 0.5rem;
  }
  .interest p {
    margin: 0.25rem 0;
  }
</style>
