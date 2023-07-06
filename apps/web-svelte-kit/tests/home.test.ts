import { expect, test } from '@playwright/test';

import type { ShortenResponse } from 'api';

test('home page has expected input with "type" and "name" of "url"', async ({ page }) => {
	await page.goto('/');

	const inputElem = page.getByRole('textbox', { name: 'url' });
	await expect(inputElem).toBeVisible();
	await expect(inputElem.getAttribute('type')).resolves.toBe('url');
});

test('ID of shortened url gets displayed', async ({ page, context }) => {
	context.route('**/api/url/shorten', (route) => {
		route.fulfill({
			status: 200,
			contentType: 'application/json',
			body: JSON.stringify({
				id: 'test_id'
			} satisfies ShortenResponse)
		});
	});

	await page.goto('/');

	await page.getByRole('textbox', { name: 'url' }).fill('https://www.google.com');

	await page.click('button[type=submit]');
	// await page.waitForResponse('**/api/url/shorten');

	const link = page.locator('a[href]');
	await expect(link).toBeVisible();
	await expect(link.getAttribute('href')).resolves.toBe('/test_id');
});
