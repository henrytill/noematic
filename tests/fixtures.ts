/* eslint-env node */

import { test as base, chromium, type BrowserContext } from '@playwright/test';
import * as path from 'node:path';

export const test = base.extend<{
  context: BrowserContext;
  extensionId: string;
}>({
  // eslint-disable-next-line no-empty-pattern
  context: async ({}, use) => {
    const buildPrefix = process.env.CI
      ? [__dirname, '..', 'result']
      : [__dirname, '..', '_build', 'install', 'default'];
    const pathToExtension = path.join(...buildPrefix, 'share', 'noematic', 'chromium');
    const args = [
      `--disable-extensions-except=${pathToExtension}`,
      `--load-extension=${pathToExtension}`,
    ];
    if (process.env.CI) {
      args.push('--headless=new');
    }
    const context = await chromium.launchPersistentContext('', {
      headless: false,
      args,
    });
    await use(context);
    await context.close();
  },
  extensionId: async ({ context }, use) => {
    // for manifest v3:
    let [background] = context.serviceWorkers();
    if (!background) background = await context.waitForEvent('serviceworker');

    const extensionId = background.url().split('/')[2];
    await use(extensionId);
  },
});

export const expect = test.expect;
