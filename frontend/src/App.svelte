<script lang="ts">
  import './app.css';
  import { onMount } from 'svelte';
  import { api, type DashboardResponse } from './lib/api/client';
  import { t } from './lib/i18n';
  import HouseholdSummary from './components/HouseholdSummary.svelte';
  import LoanCard from './components/LoanCard.svelte';
  import AddLoanModal from './components/AddLoanModal.svelte';
  import SettingsPanel from './components/SettingsPanel.svelte';
  import LanguageSwitcher from './components/LanguageSwitcher.svelte';

  let dashboard: DashboardResponse | null = null;
  let showArchived = false;
  let showAdd = false;
  let showSettings = false;
  let loading = true;
  let error = '';

  $: tr = $t;

  async function load() {
    loading = true;
    error = '';
    try {
      dashboard = await api.dashboard(showArchived);
    } catch (e) {
      error = e instanceof Error ? e.message : tr('errors.loadDashboard');
    } finally {
      loading = false;
    }
  }

  onMount(load);
</script>

<main class="page" id="main-content">
  <header class="top">
    <div>
      <h1>{tr('app.title')}</h1>
      <p class="muted">{tr('app.tagline')}</p>
    </div>
    <div class="actions">
      <LanguageSwitcher />
      <label class="toggle">
        <input type="checkbox" bind:checked={showArchived} on:change={load} />
        {tr('nav.showArchived')}
      </label>
      <button type="button" class="secondary" on:click={() => (showSettings = true)}>{tr('nav.settings')}</button>
      <button type="button" on:click={() => (showAdd = true)}>{tr('nav.addLoan')}</button>
    </div>
  </header>

  {#if loading}
    <p role="status">{tr('common.loading')}</p>
  {:else if error}
    <p class="error" role="alert">{error}</p>
  {:else if dashboard}
    {#if dashboard.loans.length === 0}
      <div class="card empty">
        <h2>{tr('empty.title')}</h2>
        <p>{tr('empty.body')}</p>
        <button type="button" on:click={() => (showAdd = true)}>{tr('empty.cta')}</button>
      </div>
    {:else}
      <HouseholdSummary data={dashboard.household} />
      <section class="loans" aria-label="Loans">
        {#each dashboard.loans as loan (loan.id)}
          <LoanCard {loan} on:refresh={load} />
        {/each}
      </section>
    {/if}
  {/if}
</main>

<AddLoanModal open={showAdd} on:close={() => (showAdd = false)} on:saved={load} />
<SettingsPanel open={showSettings} on:close={() => (showSettings = false)} on:imported={load} />

<style>
  .page { max-width: 900px; margin: 0 auto; padding: 1.5rem; }
  .top {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 1rem;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
  }
  h1 { margin: 0; font-size: 1.75rem; }
  .actions { display: flex; align-items: center; gap: 0.75rem; flex-wrap: wrap; }
  .toggle { display: flex; align-items: center; gap: 0.35rem; font-size: 0.875rem; }
  .loans { margin-top: 1rem; }
  .empty { text-align: center; padding: 2rem; }
</style>
