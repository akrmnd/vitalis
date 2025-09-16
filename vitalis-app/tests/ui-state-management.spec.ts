import { test, expect } from '@playwright/test';

test.describe('Vitalis Studio - UI State Management', () => {
  const validSequence = `>test_sequence
ATGCGTACGTAGCTAGCTAGC`;

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await expect(page.getByText('Vitalis Studio')).toBeVisible();
  });

  test('should manage button states correctly throughout workflow', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    const parseButton = page.getByRole('button', { name: /parse sequence/i });

    // Initial state: Parse button disabled with empty input
    await expect(parseButton).toBeDisabled();
    await expect(parseButton).toHaveText('Parse Sequence');

    // State after entering text: Parse button enabled
    await textarea.fill(validSequence);
    await expect(parseButton).toBeEnabled();

    // State during parsing: Parse button disabled with loading text
    await parseButton.click();
    await expect(parseButton).toBeDisabled();
    await expect(parseButton).toHaveText('Parsing...');

    // Wait for parsing completion
    await expect(page.getByText(/Sequence ID:/)).toBeVisible({ timeout: 10000 });

    // State after parsing: Parse button re-enabled
    await expect(parseButton).toBeEnabled();
    await expect(parseButton).toHaveText('Parse Sequence');

    // Statistics button should now be available
    const statsButton = page.getByRole('button', { name: /get statistics/i });
    await expect(statsButton).toBeVisible();
    await expect(statsButton).toBeEnabled();
    await expect(statsButton).toHaveText('Get Statistics');

    // State during statistics calculation
    await statsButton.click();
    await expect(statsButton).toBeDisabled();
    await expect(statsButton).toHaveText('Calculating...');

    // Wait for statistics completion
    await expect(page.getByText('Sequence Statistics')).toBeVisible({ timeout: 10000 });

    // State after statistics: Statistics button re-enabled
    await expect(statsButton).toBeEnabled();
    await expect(statsButton).toHaveText('Get Statistics');
  });

  test('should show progressive UI sections correctly', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');

    // Initially, only input section should be visible
    await expect(page.getByText('Sequence Input')).toBeVisible();
    await expect(page.getByText('Analysis')).not.toBeVisible();
    await expect(page.getByText('Sequence Statistics')).not.toBeVisible();

    // After entering sequence, input section still visible, analysis not yet
    await textarea.fill(validSequence);
    await expect(page.getByText('Sequence Input')).toBeVisible();
    await expect(page.getByText('Analysis')).not.toBeVisible();

    // After parsing, analysis section should appear
    await page.getByRole('button', { name: /parse sequence/i }).click();
    await expect(page.getByText(/Sequence ID:/)).toBeVisible({ timeout: 10000 });

    await expect(page.getByText('Analysis')).toBeVisible();
    await expect(page.getByText('Sequence Statistics')).not.toBeVisible();

    // After getting statistics, statistics section should appear
    await page.getByRole('button', { name: /get statistics/i }).click();
    await expect(page.getByText('Sequence Statistics')).toBeVisible({ timeout: 10000 });

    // All sections should now be visible
    await expect(page.getByText('Sequence Input')).toBeVisible();
    await expect(page.getByText('Analysis')).toBeVisible();
    await expect(page.getByText('Sequence Statistics')).toBeVisible();
  });

  test('should handle textarea input states correctly', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    const parseButton = page.getByRole('button', { name: /parse sequence/i });

    // Test empty input
    await expect(textarea).toHaveValue('');
    await expect(parseButton).toBeDisabled();

    // Test single character
    await textarea.fill('A');
    await expect(parseButton).toBeEnabled();

    // Test clearing input
    await textarea.fill('');
    await expect(parseButton).toBeDisabled();

    // Test whitespace only
    await textarea.fill('   \n\t  ');
    await expect(parseButton).toBeDisabled();

    // Test valid input with whitespace
    await textarea.fill('  ' + validSequence + '  ');
    await expect(parseButton).toBeEnabled();

    // Test maintaining input during interaction
    const inputValue = await textarea.inputValue();
    expect(inputValue).toContain('test_sequence');
    expect(inputValue).toContain('ATGCGTACGTAGCTAGCTAGC');
  });

  test('should display sequence ID consistently', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    await textarea.fill(validSequence);

    // Parse sequence
    await page.getByRole('button', { name: /parse sequence/i }).click();
    await expect(page.getByText(/Sequence ID:/)).toBeVisible({ timeout: 10000 });

    // Sequence ID should have consistent format
    const sequenceIdElement = page.getByText(/Sequence ID:/);
    const sequenceIdText = await sequenceIdElement.textContent();

    expect(sequenceIdText).toMatch(/^Sequence ID: \w+$/);

    // The sequence ID should persist during statistics calculation
    await page.getByRole('button', { name: /get statistics/i }).click();

    // During calculation
    await expect(page.getByText('Calculating...')).toBeVisible();
    await expect(sequenceIdElement).toBeVisible();

    // After calculation
    await expect(page.getByText('Sequence Statistics')).toBeVisible({ timeout: 10000 });
    await expect(sequenceIdElement).toBeVisible();

    // Sequence ID text should remain the same
    const persistedSequenceIdText = await sequenceIdElement.textContent();
    expect(persistedSequenceIdText).toBe(sequenceIdText);
  });

  test('should handle rapid user interactions gracefully', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    const parseButton = page.getByRole('button', { name: /parse sequence/i });

    await textarea.fill(validSequence);

    // Rapid clicking on parse button should not cause issues
    await parseButton.click();

    // Button should immediately become disabled
    await expect(parseButton).toBeDisabled();

    // Additional clicks should not interfere
    await parseButton.click({ force: true }); // Force click even if disabled
    await parseButton.click({ force: true });

    // Should still complete normally
    await expect(page.getByText(/Sequence ID:/)).toBeVisible({ timeout: 10000 });
    await expect(parseButton).toBeEnabled();
  });

  test('should maintain consistent layout during state changes', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');

    // Check initial layout
    await expect(page.locator('header')).toBeVisible();
    await expect(page.locator('main')).toBeVisible();
    await expect(page.locator('footer')).toBeVisible();

    // Layout should remain consistent during input
    await textarea.fill(validSequence);
    await expect(page.locator('header')).toBeVisible();
    await expect(page.locator('main')).toBeVisible();
    await expect(page.locator('footer')).toBeVisible();

    // Layout should remain consistent during parsing
    await page.getByRole('button', { name: /parse sequence/i }).click();
    await expect(page.locator('header')).toBeVisible();
    await expect(page.locator('main')).toBeVisible();
    await expect(page.locator('footer')).toBeVisible();

    // Layout should remain consistent after parsing
    await expect(page.getByText(/Sequence ID:/)).toBeVisible({ timeout: 10000 });
    await expect(page.locator('header')).toBeVisible();
    await expect(page.locator('main')).toBeVisible();
    await expect(page.locator('footer')).toBeVisible();

    // Layout should remain consistent during statistics
    await page.getByRole('button', { name: /get statistics/i }).click();
    await expect(page.getByText('Sequence Statistics')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('header')).toBeVisible();
    await expect(page.locator('main')).toBeVisible();
    await expect(page.locator('footer')).toBeVisible();
  });

  test('should handle input validation correctly', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    const parseButton = page.getByRole('button', { name: /parse sequence/i });

    // Test various input scenarios
    const testInputs = [
      { input: '', shouldEnable: false, description: 'empty string' },
      { input: '   ', shouldEnable: false, description: 'whitespace only' },
      { input: '\n\t\r', shouldEnable: false, description: 'newlines and tabs' },
      { input: 'A', shouldEnable: true, description: 'single character' },
      { input: 'ATGC', shouldEnable: true, description: 'valid nucleotides' },
      { input: validSequence, shouldEnable: true, description: 'valid FASTA' },
    ];

    for (const { input, shouldEnable, description } of testInputs) {
      await textarea.fill(input);

      if (shouldEnable) {
        await expect(parseButton).toBeEnabled();
      } else {
        await expect(parseButton).toBeDisabled();
      }

      // Clear for next test
      await textarea.fill('');
    }
  });

  test('should preserve user input during operations', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    const originalSequence = validSequence;

    await textarea.fill(originalSequence);

    // Parse sequence
    await page.getByRole('button', { name: /parse sequence/i }).click();
    await expect(page.getByText(/Sequence ID:/)).toBeVisible({ timeout: 10000 });

    // Input should still contain the original sequence
    const inputValueAfterParsing = await textarea.inputValue();
    expect(inputValueAfterParsing).toBe(originalSequence);

    // Get statistics
    await page.getByRole('button', { name: /get statistics/i }).click();
    await expect(page.getByText('Sequence Statistics')).toBeVisible({ timeout: 10000 });

    // Input should still contain the original sequence
    const inputValueAfterStats = await textarea.inputValue();
    expect(inputValueAfterStats).toBe(originalSequence);
  });
});