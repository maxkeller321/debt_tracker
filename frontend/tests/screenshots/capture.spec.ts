import { test } from '@playwright/test';
import fs from 'fs';
import path from 'path';

// Regenerates the docs/README screenshots from example data.
// Run with: npm run screenshots
const OUT = path.join(process.cwd(), '..', 'docs', 'screenshots');
const shot = (name: string) => path.join(OUT, name);

test('capture screenshots', async ({ page, request }) => {
  fs.mkdirSync(OUT, { recursive: true });

  // ---- Seed example data via the API (separate cookie jar from the page) ----
  const login = await request.post('/api/v1/auth/login', {
    data: { username: 'demo', password: 'demo-password' },
  });
  if (!login.ok()) throw new Error(`login failed: ${login.status()} ${await login.text()}`);
  const cookie = (login.headers()['set-cookie'] ?? '').split(';')[0] ?? '';

  const post = async (p: string, body: unknown) => {
    const r = await request.post(`/api/v1/${p}`, { data: body, headers: { Cookie: cookie } });
    if (!r.ok()) throw new Error(`${p} -> ${r.status()} ${await r.text()}`);
    return r.json();
  };

  // Start from a clean slate so runs are deterministic.
  await post('import?confirm=true', { schema_version: 1, loans: [] });

  const mkLoan = (
    label: string,
    balance: number,
    apr: number,
    tilgungEuro: number,
    start: string,
    first: string,
  ) =>
    post('loans', {
      label,
      setup_mode: 'advanced',
      remaining_balance_minor: Math.round(balance * 100),
      payment_frequency: 'monthly',
      payment_type: 'tilgung_euro',
      apr_basis_points: Math.round(apr * 100),
      tilgung_euro_minor: Math.round(tilgungEuro * 100),
      loan_start_date: start,
      first_payment_date: first,
    });

  // Past start dates → real applied payments, interest paid, and progress.
  const mortgage = await mkLoan('Home Mortgage', 265000, 3.6, 1150, '2021-05-01', '2021-06-01');
  await mkLoan('Car Loan', 18500, 4.9, 340, '2023-09-01', '2023-10-01');
  await mkLoan('Renovation Loan', 12000, 5.2, 220, '2024-02-01', '2024-03-01');
  // A scheduled extra payment so the "Upcoming extra payments" panel is populated.
  await post(`loans/${mortgage.id}/sonderzahlungen/scheduled`, {
    amount_minor: 500000,
    due_date: '2026-12-01',
  });

  // ---- Deterministic UI: English + light theme ----
  await page.addInitScript(() => {
    localStorage.setItem('debt-tracker-locale', 'en');
    localStorage.setItem('debt-tracker-theme', 'light');
  });

  // ---- 01 · Login screen (page has no session cookie yet) ----
  await page.goto('/');
  await page.getByLabel('Username').waitFor({ state: 'visible' });
  await page.screenshot({ path: shot('01-login.png') });

  // ---- Sign in ----
  await page.getByLabel('Username').fill('demo');
  await page.getByLabel('Password').fill('demo-password');
  await page.locator('form button[type="submit"]').click();
  await page.getByRole('button', { name: 'Add loan' }).waitFor({ state: 'visible' });
  await page.getByText('Total remaining').first().waitFor({ state: 'visible' });
  await page.waitForTimeout(700); // let charts settle

  // ---- 02 · Dashboard (hero) ----
  await page.screenshot({ path: shot('02-dashboard.png') });

  // ---- 03 · Loan detail (expanded card) ----
  const card = page.locator('article').filter({ hasText: 'Home Mortgage' });
  await card.getByTestId('loan-expand').click();
  await card.locator('.loan-details').waitFor({ state: 'visible' });
  await card.evaluate((el) => el.scrollIntoView({ block: 'start' }));
  await page.waitForTimeout(400);
  await page.screenshot({ path: shot('03-loan-detail.png') });

  // ---- 04 · Amortization schedule ----
  await card.getByRole('button', { name: /amortization/i }).click();
  await page.waitForTimeout(500);
  await card.locator('table').first().evaluate((el) => el.scrollIntoView({ block: 'center' }));
  await page.waitForTimeout(300);
  await page.screenshot({ path: shot('04-loan-amortization.png') });
  await page.evaluate(() => window.scrollTo(0, 0));
  await card.getByTestId('loan-expand').click(); // collapse

  // ---- 05 · Add loan modal ----
  await page.getByRole('button', { name: 'Add loan' }).click();
  await page.locator('.modal').waitFor({ state: 'visible' });
  await page.waitForTimeout(300);
  await page.screenshot({ path: shot('05-add-loan.png') });
  await page.getByRole('button', { name: 'Cancel' }).click();

  // ---- 06 · Settings (backup / restore) ----
  await page.getByRole('button', { name: 'Settings' }).click();
  await page.waitForTimeout(400);
  await page.screenshot({ path: shot('06-settings.png') });
});
