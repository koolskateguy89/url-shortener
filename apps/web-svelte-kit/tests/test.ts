import { expect, test } from '@playwright/test';

// trying to run playwright tests keeps giving an error idk why lmao

test('about page has expected h1', async ({ page }) => {
	await page.goto('/default/about');
	await expect(page.getByRole('heading', { name: 'About this app' })).toBeVisible();
});
