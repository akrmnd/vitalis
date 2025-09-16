import { test, expect } from '@playwright/test';

test.describe('Vitalis Studio - Error Handling', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await expect(page.getByText('Vitalis Studio')).toBeVisible();
  });

  test('should handle invalid FASTA format gracefully', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    const parseButton = page.getByRole('button', { name: /parse sequence/i });

    // Test various invalid FASTA formats
    const invalidFormats = [
      'This is not FASTA format at all',
      'ATGCGTACGT', // No header
      '>header_only',
      '>invalid\nXYZ123', // Invalid nucleotides
      '>empty_sequence\n',
      '>', // Just a header marker
    ];

    for (const invalidFormat of invalidFormats) {
      await textarea.fill(invalidFormat);
      await parseButton.click();

      // Check that loading state appears
      await expect(parseButton).toHaveText('Parsing...');
      await expect(parseButton).toBeDisabled();

      // Wait for operation to complete
      await expect(parseButton).toHaveText('Parse Sequence', { timeout: 10000 });
      await expect(parseButton).toBeEnabled();

      // Sequence ID should not appear for invalid formats
      // (The app might handle this silently or show an error)

      // Clear for next test
      await textarea.fill('');
    }
  });

  test('should handle network/backend errors gracefully', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    const validSequence = `>test_sequence
ATGCGTACGTAGC`;

    await textarea.fill(validSequence);

    // Mock console to capture error messages
    const consoleErrors: string[] = [];
    page.on('console', msg => {
      if (msg.type() === 'error') {
        consoleErrors.push(msg.text());
      }
    });

    // Try to parse - this might fail if backend is not available
    await page.getByRole('button', { name: /parse sequence/i }).click();

    // Wait for operation to complete (either success or failure)
    await page.waitForTimeout(5000);

    // UI should return to stable state regardless of backend availability
    const parseButton = page.getByRole('button', { name: /parse sequence/i });
    await expect(parseButton).toBeEnabled();
    await expect(parseButton).toHaveText('Parse Sequence');

    // App should not crash
    await expect(page.getByText('Vitalis Studio')).toBeVisible();
  });

  test('should handle statistics calculation errors', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    const validSequence = `>test_sequence
ATGCGTACGTAGC`;

    await textarea.fill(validSequence);

    // Try to get statistics without parsing first (should fail gracefully)
    // This test assumes the UI might allow clicking stats button without sequence ID

    // First, let's try normal flow and then test error scenarios
    await page.getByRole('button', { name: /parse sequence/i }).click();

    // If parsing succeeds, test statistics error handling
    const sequenceIdVisible = await page.getByText(/Sequence ID:/).isVisible({ timeout: 5000 }).catch(() => false);

    if (sequenceIdVisible) {
      const statsButton = page.getByRole('button', { name: /get statistics/i });
      await statsButton.click();

      // Monitor for errors during statistics calculation
      const consoleErrors: string[] = [];
      page.on('console', msg => {
        if (msg.type() === 'error') {
          consoleErrors.push(msg.text());
        }
      });

      // Wait for operation to complete
      await page.waitForTimeout(5000);

      // UI should return to stable state
      await expect(statsButton).toBeEnabled();
      await expect(statsButton).toHaveText('Get Statistics');
    }

    // App should not crash
    await expect(page.getByText('Vitalis Studio')).toBeVisible();
  });

  test('should handle empty statistics response', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    const validSequence = `>test_sequence
ATGCGTACGTAGC`;

    await textarea.fill(validSequence);
    await page.getByRole('button', { name: /parse sequence/i }).click();

    const sequenceIdVisible = await page.getByText(/Sequence ID:/).isVisible({ timeout: 10000 }).catch(() => false);

    if (sequenceIdVisible) {
      await page.getByRole('button', { name: /get statistics/i }).click();

      // Wait for statistics operation
      await page.waitForTimeout(5000);

      // Check if statistics section appears
      const statsVisible = await page.getByText('Sequence Statistics').isVisible().catch(() => false);

      if (statsVisible) {
        // Verify that all expected fields are present
        await expect(page.getByText(/Length:/)).toBeVisible();
        await expect(page.getByText(/GC Content:/)).toBeVisible();
        await expect(page.getByText(/AT Content:/)).toBeVisible();
        await expect(page.getByText(/N Content:/)).toBeVisible();
      }
    }

    // App should remain functional
    await expect(page.getByText('Vitalis Studio')).toBeVisible();
  });

  test('should handle rapid successive operations', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    const sequences = [
      `>seq1\nATGCGT`,
      `>seq2\nCGTACG`,
      `>seq3\nGTACGT`,
    ];

    // Rapidly parse multiple sequences
    for (const sequence of sequences) {
      await textarea.fill(sequence);
      const parseButton = page.getByRole('button', { name: /parse sequence/i });
      await parseButton.click();

      // Don't wait for completion, immediately try next sequence
      await page.waitForTimeout(100);
    }

    // Wait for all operations to settle
    await page.waitForTimeout(5000);

    // UI should be in a stable state
    const parseButton = page.getByRole('button', { name: /parse sequence/i });
    await expect(parseButton).toBeEnabled();
    await expect(parseButton).toHaveText('Parse Sequence');

    // App should not crash
    await expect(page.getByText('Vitalis Studio')).toBeVisible();
  });

  test('should handle large sequence input', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');

    // Create a large sequence (10,000 nucleotides)
    const largeSequence = '>large_sequence\n' + 'ATGC'.repeat(2500);

    await textarea.fill(largeSequence);

    const parseButton = page.getByRole('button', { name: /parse sequence/i });
    await expect(parseButton).toBeEnabled();

    await parseButton.click();

    // Should handle large input gracefully
    await expect(parseButton).toHaveText('Parsing...');

    // Wait longer for large sequence processing
    await expect(parseButton).toHaveText('Parse Sequence', { timeout: 15000 });

    // App should remain responsive
    await expect(page.getByText('Vitalis Studio')).toBeVisible();
  });

  test('should handle special characters in sequence', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');

    // Test sequences with various special characters
    const specialCharSequences = [
      `>seq_with_numbers\nATGC123GTN`,
      `>seq_with_spaces\nATGC GTN`,
      `>seq_with_newlines\nATGC\n\nGTN`,
      `>seq_with_tabs\nATGC\tGTN`,
      `>unicode_header_序列\nATGCGTN`,
      `>seq_with_symbols\nATGC*GTN`,
    ];

    for (const sequence of specialCharSequences) {
      await textarea.fill(sequence);

      const parseButton = page.getByRole('button', { name: /parse sequence/i });
      await parseButton.click();

      // Wait for parsing to complete
      await expect(parseButton).toHaveText('Parse Sequence', { timeout: 10000 });

      // UI should remain stable regardless of input
      await expect(parseButton).toBeEnabled();
      await expect(page.getByText('Vitalis Studio')).toBeVisible();

      // Clear for next test
      await textarea.fill('');
    }
  });

  test('should maintain functionality after errors', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');

    // First, try an invalid sequence
    await textarea.fill('invalid_sequence_format');
    await page.getByRole('button', { name: /parse sequence/i }).click();
    await page.waitForTimeout(3000);

    // Then try a valid sequence
    const validSequence = `>recovery_test\nATGCGTACGT`;
    await textarea.fill(validSequence);
    await page.getByRole('button', { name: /parse sequence/i }).click();

    // Should be able to parse valid sequence after error
    const sequenceIdVisible = await page.getByText(/Sequence ID:/).isVisible({ timeout: 10000 }).catch(() => false);

    if (sequenceIdVisible) {
      // Should be able to get statistics after recovery
      const statsButton = page.getByRole('button', { name: /get statistics/i });
      await expect(statsButton).toBeVisible();
      await expect(statsButton).toBeEnabled();
    }

    // App should be fully functional
    await expect(page.getByText('Vitalis Studio')).toBeVisible();
  });

  test('should handle browser back/forward during operations', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    const validSequence = `>navigation_test\nATGCGT`;

    await textarea.fill(validSequence);
    await page.getByRole('button', { name: /parse sequence/i }).click();

    // Navigate away and back during operation
    await page.goto('about:blank');
    await page.goBack();

    // Should return to stable state
    await expect(page.getByText('Vitalis Studio')).toBeVisible();

    // UI elements should be present and functional
    await expect(page.locator('textarea[placeholder*="FASTA"]')).toBeVisible();
    await expect(page.getByRole('button', { name: /parse sequence/i })).toBeVisible();
  });
});