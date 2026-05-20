<script lang="ts">
  import { locale, locales, t } from '../lib/i18n';

  $: translate = $t;
</script>

<div class="lang-switcher" role="group" aria-label="Language">
  {#each locales as loc (loc.code)}
    <button
      type="button"
      class="lang-btn"
      class:active={$locale === loc.code}
      title={translate(loc.labelKey)}
      aria-label={translate(loc.labelKey)}
      aria-pressed={$locale === loc.code}
      on:click={() => locale.set(loc.code)}
    >
      <span class="flag" aria-hidden="true">{loc.flag}</span>
    </button>
  {/each}
</div>

<style>
  .lang-switcher {
    display: flex;
    gap: 0.25rem;
    padding: 0.2rem;
    background: #e2e8f0;
    border-radius: 999px;
  }
  .lang-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2.25rem;
    height: 2.25rem;
    padding: 0;
    border-radius: 999px;
    background: transparent;
    color: inherit;
    transition: background 0.15s, box-shadow 0.15s;
  }
  .lang-btn:hover {
    background: rgb(255 255 255 / 70%);
  }
  .lang-btn.active {
    background: #fff;
    box-shadow: 0 1px 4px rgb(0 0 0 / 12%);
  }
  .flag {
    font-size: 1.25rem;
    line-height: 1;
  }
</style>
