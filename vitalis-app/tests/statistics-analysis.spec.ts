import { test, expect } from '@playwright/test';

test.describe('Vitalis Studio - Statistics Analysis', () => {
  // Test sequences with known properties
  const highGcSequence = `>high_gc_content
GCGCGCGCGCGCGCGCGCGC`;

  const lowGcSequence = `>low_gc_content
ATATATATATATATATATATAT`;

  const mixedSequence = `>mixed_content
ATGCGTACGTCGATCGATCGATCGATCGATCGATCG`;

  const sequenceWithN = `>sequence_with_n
ATGCNNNNATGCNNNNATGCNNNNATGC`;

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await expect(page.getByText('Vitalis Studio')).toBeVisible();
  });

  test('should calculate high GC content correctly', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    await textarea.fill(highGcSequence);

    // Parse and analyze
    await page.getByRole('button', { name: /parse sequence/i }).click();
    await expect(page.getByText(/Sequence ID:/)).toBeVisible({ timeout: 10000 });

    await page.getByRole('button', { name: /get statistics/i }).click();
    await expect(page.getByText('Sequence Statistics')).toBeVisible({ timeout: 10000 });

    // Check GC content is 100% (all G and C)
    const gcText = await page.getByText(/GC Content: \d+\.\d+%/).textContent();
    expect(gcText).toContain('100.00%');

    // Check AT content is 0%
    const atText = await page.getByText(/AT Content: \d+\.\d+%/).textContent();
    expect(atText).toContain('0.00%');

    // Check length
    await expect(page.getByText('Length: 20 bp')).toBeVisible();
  });

  test('should calculate low GC content correctly', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    await textarea.fill(lowGcSequence);

    // Parse and analyze
    await page.getByRole('button', { name: /parse sequence/i }).click();
    await expect(page.getByText(/Sequence ID:/)).toBeVisible({ timeout: 10000 });

    await page.getByRole('button', { name: /get statistics/i }).click();
    await expect(page.getByText('Sequence Statistics')).toBeVisible({ timeout: 10000 });

    // Check GC content is 0% (all A and T)
    const gcText = await page.getByText(/GC Content: \d+\.\d+%/).textContent();
    expect(gcText).toContain('0.00%');

    // Check AT content is 100%
    const atText = await page.getByText(/AT Content: \d+\.\d+%/).textContent();
    expect(atText).toContain('100.00%');

    // Check length
    await expect(page.getByText('Length: 22 bp')).toBeVisible();
  });

  test('should handle sequences with N content', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    await textarea.fill(sequenceWithN);

    // Parse and analyze
    await page.getByRole('button', { name: /parse sequence/i }).click();
    await expect(page.getByText(/Sequence ID:/)).toBeVisible({ timeout: 10000 });

    await page.getByRole('button', { name: /get statistics/i }).click();
    await expect(page.getByText('Sequence Statistics')).toBeVisible({ timeout: 10000 });

    // Check that N content is properly calculated
    const nText = await page.getByText(/N Content: \d+\.\d+%/).textContent();
    const nMatch = nText?.match(/(\d+\.\d+)%/);
    if (nMatch) {
      const nPercent = parseFloat(nMatch[1]);
      expect(nPercent).toBeGreaterThan(0);
      // Sequence has 12 Ns out of 28 total = ~42.86%
      expect(nPercent).toBeCloseTo(42.86, 1);
    }

    // Check length includes N nucleotides
    await expect(page.getByText('Length: 28 bp')).toBeVisible();
  });

  test('should display statistics with proper formatting', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    await textarea.fill(mixedSequence);

    // Parse and analyze
    await page.getByRole('button', { name: /parse sequence/i }).click();
    await expect(page.getByText(/Sequence ID:/)).toBeVisible({ timeout: 10000 });

    await page.getByRole('button', { name: /get statistics/i }).click();
    await expect(page.getByText('Sequence Statistics')).toBeVisible({ timeout: 10000 });

    // Check formatting - all percentages should have 2 decimal places
    const gcText = await page.getByText(/GC Content: \d+\.\d{2}%/).textContent();
    const atText = await page.getByText(/AT Content: \d+\.\d{2}%/).textContent();
    const nText = await page.getByText(/N Content: \d+\.\d{2}%/).textContent();

    expect(gcText).toMatch(/GC Content: \d+\.\d{2}%/);
    expect(atText).toMatch(/AT Content: \d+\.\d{2}%/);
    expect(nText).toMatch(/N Content: \d+\.\d{2}%/);

    // Length should be displayed as integer with 'bp' unit
    await expect(page.getByText(/Length: \d+ bp/)).toBeVisible();
  });

  test('should maintain statistics consistency', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    await textarea.fill(mixedSequence);

    // Parse and analyze
    await page.getByRole('button', { name: /parse sequence/i }).click();
    await expect(page.getByText(/Sequence ID:/)).toBeVisible({ timeout: 10000 });

    await page.getByRole('button', { name: /get statistics/i }).click();
    await expect(page.getByText('Sequence Statistics')).toBeVisible({ timeout: 10000 });

    // Extract percentage values
    const gcText = await page.getByText(/GC Content: \d+\.\d+%/).textContent();
    const atText = await page.getByText(/AT Content: \d+\.\d+%/).textContent();
    const nText = await page.getByText(/N Content: \d+\.\d+%/).textContent();

    if (gcText && atText && nText) {
      const gcPercent = parseFloat(gcText.match(/(\d+\.\d+)%/)?.[1] || '0');
      const atPercent = parseFloat(atText.match(/(\d+\.\d+)%/)?.[1] || '0');
      const nPercent = parseFloat(nText.match(/(\d+\.\d+)%/)?.[1] || '0');

      // Total percentages should equal 100% (within rounding tolerance)
      const total = gcPercent + atPercent + nPercent;
      expect(total).toBeCloseTo(100, 1);
    }
  });

  test('should update statistics when analyzing different sequences', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');

    // First analysis - high GC
    await textarea.fill(highGcSequence);
    await page.getByRole('button', { name: /parse sequence/i }).click();
    await expect(page.getByText(/Sequence ID:/)).toBeVisible({ timeout: 10000 });

    await page.getByRole('button', { name: /get statistics/i }).click();
    await expect(page.getByText('Sequence Statistics')).toBeVisible({ timeout: 10000 });

    const firstGcText = await page.getByText(/GC Content: \d+\.\d+%/).textContent();

    // Second analysis - low GC
    await textarea.fill(lowGcSequence);
    await page.getByRole('button', { name: /parse sequence/i }).click();
    await expect(page.getByText(/Sequence ID:/)).toBeVisible({ timeout: 10000 });

    await page.getByRole('button', { name: /get statistics/i }).click();
    await expect(page.getByText('Sequence Statistics')).toBeVisible({ timeout: 10000 });

    const secondGcText = await page.getByText(/GC Content: \d+\.\d+%/).textContent();

    // GC content should be different between the two analyses
    expect(firstGcText).not.toBe(secondGcText);
    expect(firstGcText).toContain('100.00%');
    expect(secondGcText).toContain('0.00%');
  });

  test('should display all required statistics fields', async ({ page }) => {
    const textarea = page.locator('textarea[placeholder*="FASTA"]');
    await textarea.fill(mixedSequence);

    // Parse and analyze
    await page.getByRole('button', { name: /parse sequence/i }).click();
    await expect(page.getByText(/Sequence ID:/)).toBeVisible({ timeout: 10000 });

    await page.getByRole('button', { name: /get statistics/i }).click();
    await expect(page.getByText('Sequence Statistics')).toBeVisible({ timeout: 10000 });

    // Check all required fields are present
    await expect(page.getByText(/Length:/)).toBeVisible();
    await expect(page.getByText(/GC Content:/)).toBeVisible();
    await expect(page.getByText(/AT Content:/)).toBeVisible();
    await expect(page.getByText(/N Content:/)).toBeVisible();

    // Check the statistics section has proper heading
    await expect(page.getByText('Sequence Statistics')).toBeVisible();

    // Check values are in list format
    const statsList = page.locator('.stats ul');
    await expect(statsList).toBeVisible();

    const listItems = statsList.locator('li');
    await expect(listItems).toHaveCount(4);
  });
});