import { defineConfig } from '@playwright/test';

// Dedicated config for regenerating the README/docs screenshots.
// Run via `npm run screenshots` (which builds the frontend first).
// It boots the real backend serving the built frontend, seeds example
// data, and writes PNGs to ../docs/screenshots/.
export default defineConfig({
  testDir: './tests/screenshots',
  workers: 1,
  timeout: 120_000,
  use: {
    baseURL: 'http://127.0.0.1:8080',
    viewport: { width: 1360, height: 1000 },
    deviceScaleFactor: 2,
    colorScheme: 'light',
  },
  webServer: {
    command:
      'cd ../backend && DATA_DIR=../data-screenshots STATIC_DIR=../frontend/build AUTH_USERNAME=demo AUTH_PASSWORD=demo-password cargo run -p api',
    url: 'http://127.0.0.1:8080/api/v1/health',
    reuseExistingServer: true,
    timeout: 300_000,
  },
});
