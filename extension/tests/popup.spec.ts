// @eslint-env node

import { test, expect } from './fixtures';

test('popup page', async ({ page, extensionId }) => {
    await page.goto(`chrome-extension://${extensionId}/popup/popup.html`);
    await expect(page.locator('#save')).toBeVisible();
});
