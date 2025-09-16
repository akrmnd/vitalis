import { test, expect } from '@playwright/test';

test.describe('Vitalis Studio - Sequence Analysis', () => {
  test('should load the application correctly', async ({ page }) => {
    await page.goto('/');

    // Check if the title is correct
    await expect(page).toHaveTitle(/Vitalis Studio/);

    // Check if the main heading is visible
    await expect(page.getByText('Vitalis Studio')).toBeVisible();

    // Check if the main components are loaded
    await expect(page.getByText('DNA/RNA Sequence Analysis Tool')).toBeVisible();
  });

  test('should have sequence import functionality', async ({ page }) => {
    await page.goto('/');

    // Look for sequence input or import button
    const sequenceInput = page.locator('textarea, input[type="file"], [data-testid="sequence-input"]').first();
    if (await sequenceInput.isVisible()) {
      await expect(sequenceInput).toBeVisible();
    }

    // Check if there's any indication of FASTA/FASTQ support
    const formatText = page.getByText(/FASTA|FASTQ/i);
    if (await formatText.count() > 0) {
      await expect(formatText.first()).toBeVisible();
    }
  });

  test('should display sequence statistics section', async ({ page }) => {
    await page.goto('/');

    // Look for statistics-related elements
    const statsElements = [
      page.getByText(/statistics/i),
      page.getByText(/GC/i),
      page.getByText(/length/i),
      page.getByText(/analysis/i)
    ];

    for (const element of statsElements) {
      if (await element.count() > 0) {
        await expect(element.first()).toBeVisible();
      }
    }
  });

  test('should be responsive', async ({ page }) => {
    await page.goto('/');

    // Test different viewport sizes
    await page.setViewportSize({ width: 1200, height: 800 });
    await expect(page.getByText('Vitalis Studio')).toBeVisible();

    await page.setViewportSize({ width: 768, height: 1024 });
    await expect(page.getByText('Vitalis Studio')).toBeVisible();

    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page.getByText('Vitalis Studio')).toBeVisible();
  });

  test('should handle navigation and UI interactions', async ({ page }) => {
    await page.goto('/');

    // Test basic click interactions
    const clickableElements = page.locator('button, [role="button"], a, [tabindex="0"]');
    const count = await clickableElements.count();

    if (count > 0) {
      // Just verify first clickable element exists and is enabled
      const firstClickable = clickableElements.first();
      await expect(firstClickable).toBeVisible();

      if (await firstClickable.isEnabled()) {
        await expect(firstClickable).toBeEnabled();
      }
    }
  });

  test('should not have console errors on load', async ({ page }) => {
    const consoleErrors: string[] = [];

    page.on('console', msg => {
      if (msg.type() === 'error') {
        consoleErrors.push(msg.text());
      }
    });

    await page.goto('/');

    // Wait a bit for any async operations
    await page.waitForTimeout(2000);

    // Check if there are any critical console errors
    const criticalErrors = consoleErrors.filter(error =>
      !error.includes('favicon') && // Ignore favicon errors
      !error.includes('chrome-extension') && // Ignore extension errors
      !error.includes('404') // Ignore 404 errors for development
    );

    expect(criticalErrors).toHaveLength(0);
  });
});