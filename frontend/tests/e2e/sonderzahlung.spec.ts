import { test, expect } from './fixtures';
import { fillNumber } from './helpers';

test('record extra payment on expanded loan', async ({ page }) => {
  await page.goto('/');
  await page.getByRole('button', { name: 'Add loan' }).click();
  await page.getByLabel('Name').fill('Extra Pay Test');
  await fillNumber(page, '#balance', '10000');
  await fillNumber(page, '#apr', '5');
  await fillNumber(page, '#fixed', '200');
  const create = page.waitForResponse(
    (r) => r.url().includes('/api/v1/loans') && r.request().method() === 'POST',
  );
  await page.getByRole('button', { name: 'Save loan' }).click();
  expect((await create).ok()).toBeTruthy();
  const card = page.locator('article').filter({ hasText: 'Extra Pay Test' });
  await card.getByTestId('loan-expand').click();
  await expect(card.locator('.details')).toBeVisible();
  await fillNumber(page, 'input[aria-label="Extra payment amount"]', '500');
  const extra = page.waitForResponse((r) => r.url().includes('/sonderzahlungen/immediate') && r.request().method() === 'POST');
  await page.getByRole('button', { name: 'Apply now' }).click();
  await extra;
  await expect(page.getByRole('button', { name: /Extra Pay Test/ })).toBeVisible();
});
