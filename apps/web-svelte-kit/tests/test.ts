import { expect, test } from '@playwright/test';

test('home page has expected input with type and name of "url"', async ({ page }) => {
	await page.goto('/');

	const inputElem = page.getByRole('textbox', { name: 'url' });
	await expect(inputElem).toBeVisible();
	await expect(inputElem.getAttribute('type')).resolves.toBe('url');
});
