import { test, expect } from '@playwright/test';

test.describe('Vitalis Studio - Sequence Analysis Workflow', () => {
  // Sample FASTA sequences for testing
  const validFastaSequence = `>test_sequence_1
ATGCGTACGTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGC
GCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAG
CTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAG`;

  const shortFastaSequence = `>short_seq
ATGCGT`;

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await expect(page.getByText('Vitalis Studio')).toBeVisible();
  });

  test('should complete full sequence analysis workflow', async ({ page }) => {
    // Step 1: Input FASTA sequence
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    await expect(textarea).toBeVisible();
    await textarea.fill(validFastaSequence);

    // Step 2: Parse sequence
    const parseButton = page.getByRole('button', { name: /parse sequence/i });
    await expect(parseButton).toBeEnabled();

    // Check loading state
    await parseButton.click();
    await expect(page.getByText('Parsing...')).toBeVisible();

    // Wait for parsing to complete and sequence ID to appear
    await expect(page.getByText(/Sequence ID:/)).toBeVisible({ timeout: 10000 });
    const sequenceIdText = await page.getByText(/Sequence ID:/).textContent();
    expect(sequenceIdText).toMatch(/Sequence ID: \w+/);

    // Step 3: Verify analysis section appears
    await expect(page.getByText('Analysis')).toBeVisible();
    const statsButton = page.getByRole('button', { name: /get statistics/i });
    await expect(statsButton).toBeVisible();
    await expect(statsButton).toBeEnabled();

    // Step 4: Get statistics
    await statsButton.click();
    await expect(page.getByText('Calculating...')).toBeVisible();

    // Step 5: Verify statistics display
    await expect(page.getByText('Sequence Statistics')).toBeVisible({ timeout: 10000 });

    // Check all statistics are displayed
    await expect(page.getByText(/Length: \d+ bp/)).toBeVisible();
    await expect(page.getByText(/GC Content: \d+\.\d+%/)).toBeVisible();
    await expect(page.getByText(/AT Content: \d+\.\d+%/)).toBeVisible();
    await expect(page.getByText(/N Content: \d+\.\d+%/)).toBeVisible();

    // Verify statistics values make sense (GC + AT should be close to 100% for normal DNA)
    const gcText = await page.getByText(/GC Content: \d+\.\d+%/).textContent();
    const atText = await page.getByText(/AT Content: \d+\.\d+%/).textContent();

    if (gcText && atText) {
      const gcValue = parseFloat(gcText.match(/(\d+\.\d+)%/)?.[1] || '0');
      const atValue = parseFloat(atText.match(/(\d+\.\d+)%/)?.[1] || '0');
      const total = gcValue + atValue;

      // Should be close to 100% (allowing small variance for N content)
      expect(total).toBeGreaterThan(90);
      expect(total).toBeLessThanOrEqual(100);
    }
  });

  test('should handle button states correctly', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    const parseButton = page.getByRole('button', { name: /parse sequence/i });

    // Initially, button should be disabled with empty input
    await expect(parseButton).toBeDisabled();

    // Button should enable when text is entered
    await textarea.fill('A');
    await expect(parseButton).toBeEnabled();

    // Button should disable again when text is cleared
    await textarea.fill('');
    await expect(parseButton).toBeDisabled();

    // Test with whitespace only
    await textarea.fill('   ');
    await expect(parseButton).toBeDisabled();
  });

  test('should display sequence length correctly', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    await textarea.fill(shortFastaSequence);

    const parseButton = page.getByRole('button', { name: /parse sequence/i });
    await parseButton.click();

    // Wait for sequence ID
    await expect(page.getByText(/Sequence ID:/)).toBeVisible({ timeout: 10000 });

    // Get statistics
    const statsButton = page.getByRole('button', { name: /get statistics/i });
    await statsButton.click();

    // Check length matches expected value (6 nucleotides)
    await expect(page.getByText('Length: 6 bp')).toBeVisible({ timeout: 10000 });
  });

  test('should maintain UI consistency during operations', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    await textarea.fill(validFastaSequence);

    // Parse sequence
    const parseButton = page.getByRole('button', { name: /parse sequence/i });
    await parseButton.click();

    // During parsing, button should be disabled and show loading text
    await expect(parseButton).toBeDisabled();
    await expect(parseButton).toHaveText('Parsing...');

    // Wait for completion
    await expect(page.getByText(/Sequence ID:/)).toBeVisible({ timeout: 10000 });

    // Button should return to normal state
    await expect(parseButton).toBeEnabled();
    await expect(parseButton).toHaveText('Parse Sequence');

    // Now test statistics button
    const statsButton = page.getByRole('button', { name: /get statistics/i });
    await statsButton.click();

    // During calculation, button should be disabled and show loading text
    await expect(statsButton).toBeDisabled();
    await expect(statsButton).toHaveText('Calculating...');

    // Wait for completion
    await expect(page.getByText('Sequence Statistics')).toBeVisible({ timeout: 10000 });

    // Button should return to normal state
    await expect(statsButton).toBeEnabled();
    await expect(statsButton).toHaveText('Get Statistics');
  });

  test('should handle multiple sequence parsing', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');

    // Parse first sequence
    await textarea.fill(validFastaSequence);
    await page.getByRole('button', { name: /parse sequence/i }).click();
    await expect(page.getByText(/Sequence ID:/)).toBeVisible({ timeout: 10000 });

    const firstSequenceId = await page.getByText(/Sequence ID:/).textContent();

    // Parse second sequence
    await textarea.fill(shortFastaSequence);
    await page.getByRole('button', { name: /parse sequence/i }).click();
    await expect(page.getByText(/Sequence ID:/)).toBeVisible({ timeout: 10000 });

    const secondSequenceId = await page.getByText(/Sequence ID:/).textContent();

    // Sequence IDs should be different
    expect(firstSequenceId).not.toBe(secondSequenceId);
  });

  test('should display phase information', async ({ page }) => {
    // Check footer displays current phase
    await expect(page.getByText('Phase 1 - Basic Sequence Analysis')).toBeVisible();
  });
});