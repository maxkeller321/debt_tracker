import { test as base, expect } from '@playwright/test';

const EMPTY_BUNDLE = { schema_version: 1, loans: [] as unknown[] };

export const test = base.extend({
  page: async ({ page, request }, use) => {
    await page.addInitScript(() => {
      localStorage.setItem('dept-tracker-locale', 'en');
    });
    const res = await request.post('/api/v1/import?confirm=true', {
      data: EMPTY_BUNDLE,
    });
    if (!res.ok()) {
      throw new Error(`Failed to reset database: ${res.status()} ${await res.text()}`);
    }
    await use(page);
  },
});

export { expect };
