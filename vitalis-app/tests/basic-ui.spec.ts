import { test, expect } from '@playwright/test';

test.describe('Vitalis Studio - Basic UI Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should display main application elements', async ({ page }) => {
    // Title and header
    await expect(page.getByText('Vitalis Studio')).toBeVisible();
    await expect(page.getByText('DNA/RNA Sequence Analysis Tool')).toBeVisible();

    // Input section
    await expect(page.getByText('Sequence Input')).toBeVisible();
    await expect(page.locator('textarea[placeholder*="FASTA"]')).toBeVisible();
    await expect(page.getByRole('button', { name: /parse sequence/i })).toBeVisible();

    // Footer
    await expect(page.getByText('Phase 1 - Basic Sequence Analysis')).toBeVisible();
  });

  test('should validate input field behavior', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    const parseButton = page.getByRole('button', { name: /parse sequence/i });

    // Button should be disabled with empty input
    await expect(parseButton).toBeDisabled();

    // Button should be enabled with input
    await textarea.fill('>test\nATGC');
    await expect(parseButton).toBeEnabled();

    // Button should be disabled when cleared
    await textarea.fill('');
    await expect(parseButton).toBeDisabled();
  });

  test('should be responsive', async ({ page }) => {
    // Desktop view
    await expect(page.getByText('Vitalis Studio')).toBeVisible();
    const textarea = page.locator('textarea');
    await expect(textarea).toBeVisible();

    // Mobile view
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page.getByText('Vitalis Studio')).toBeVisible();
    await expect(textarea).toBeVisible();

    // Tablet view
    await page.setViewportSize({ width: 768, height: 1024 });
    await expect(page.getByText('Vitalis Studio')).toBeVisible();
    await expect(textarea).toBeVisible();
  });
});

// Note: Actual sequence parsing and analysis features require Tauri context
// and cannot be tested with Playwright. Use cargo test for backend logic.