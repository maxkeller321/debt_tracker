import type { Page } from '@playwright/test';

/** Playwright fill() does not always update Svelte bind:value on number inputs. */
export async function fillNumber(page: Page, selector: string, value: string) {
  const input = page.locator(selector);
  await input.click();
  await input.fill('');
  await input.pressSequentially(value);
}
