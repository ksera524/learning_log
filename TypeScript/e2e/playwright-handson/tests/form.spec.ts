import { test, expect } from '@playwright/test';

test('フォーム画面への遷移', async ({ page }) => {
  await page.goto('http://localhost:3000/');
  await page.getByRole('link', { name: '入力フォーム' }).click();
  await expect(page.getByRole('heading', { name: '入力フォーム' })).toBeVisible();
  await expect(page).toHaveURL("http://localhost:3000/form")
});