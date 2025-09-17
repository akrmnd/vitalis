use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedStats {
    pub length: usize,
    pub gc_percent: f64,
    pub at_percent: f64,
    pub n_percent: f64,
    pub base_counts: BaseCount,
    pub dinucleotide_counts: HashMap<String, usize>,
    pub gc_skew: f64,
    pub at_skew: f64,
    pub entropy: f64,
    pub complexity: f64,
    // New fields for enhanced statistics
    pub codon_usage: Option<CodonUsage>,
    pub quality_stats: Option<QualityStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseCount {
    pub a: usize,
    pub t: usize,
    pub g: usize,
    pub c: usize,
    pub n: usize,
    pub other: usize,
}

impl BaseCount {
    pub fn new() -> Self {
        Self {
            a: 0,
            t: 0,
            g: 0,
            c: 0,
            n: 0,
            other: 0,
        }
    }

    pub fn total(&self) -> usize {
        self.a + self.t + self.g + self.c + self.n + self.other
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowStats {
    pub position: usize,
    pub window_size: usize,
    pub gc_percent: f64,
    pub entropy: f64,
}

/// Codon usage statistics for coding sequences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodonUsage {
    pub codon_counts: HashMap<String, usize>,
    pub codon_frequencies: HashMap<String, f64>,
    pub amino_acid_counts: HashMap<char, usize>,
    pub start_codons: usize,
    pub stop_codons: usize,
    pub rare_codons: Vec<String>,
}

/// Quality statistics for FASTQ sequences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityStats {
    pub mean_quality: f64,
    pub median_quality: f64,
    pub min_quality: u8,
    pub max_quality: u8,
    pub q20_bases: usize, // Bases with quality >= 20
    pub q30_bases: usize, // Bases with quality >= 30
    pub quality_distribution: HashMap<u8, usize>,
}

/// Calculate detailed statistics for a sequence
pub fn calculate_detailed_stats(sequence: &str) -> DetailedStats {
    calculate_detailed_stats_with_options(sequence, None, None)
}

/// Calculate detailed statistics with optional quality scores and genetic code
pub fn calculate_detailed_stats_with_options(
    sequence: &str,
    quality_scores: Option<&[u8]>,
    genetic_code: Option<u8>,
) -> DetailedStats {
    let mut base_counts = BaseCount::new();
    let mut dinucleotides: HashMap<String, usize> = HashMap::new();

    let chars: Vec<char> = sequence.chars().collect();
    let length = chars.len();

    // Count bases
    for c in &chars {
        match c.to_ascii_uppercase() {
            'A' => base_counts.a += 1,
            'T' | 'U' => base_counts.t += 1,
            'G' => base_counts.g += 1,
            'C' => base_counts.c += 1,
            'N' => base_counts.n += 1,
            _ => base_counts.other += 1,
        }
    }

    // Count dinucleotides
    for window in chars.windows(2) {
        if window.len() == 2 {
            let dinuc = format!("{}{}", window[0], window[1]).to_uppercase();
            *dinucleotides.entry(dinuc).or_insert(0) += 1;
        }
    }

    // Calculate percentages
    let gc_percent = if length > 0 {
        ((base_counts.g + base_counts.c) as f64 / length as f64) * 100.0
    } else {
        0.0
    };

    let at_percent = if length > 0 {
        ((base_counts.a + base_counts.t) as f64 / length as f64) * 100.0
    } else {
        0.0
    };

    let n_percent = if length > 0 {
        (base_counts.n as f64 / length as f64) * 100.0
    } else {
        0.0
    };

    // Calculate GC skew: (G - C) / (G + C)
    let gc_skew = if base_counts.g + base_counts.c > 0 {
        (base_counts.g as f64 - base_counts.c as f64)
            / (base_counts.g as f64 + base_counts.c as f64)
    } else {
        0.0
    };

    // Calculate AT skew: (A - T) / (A + T)
    let at_skew = if base_counts.a + base_counts.t > 0 {
        (base_counts.a as f64 - base_counts.t as f64)
            / (base_counts.a as f64 + base_counts.t as f64)
    } else {
        0.0
    };

    // Calculate Shannon entropy
    let entropy = calculate_entropy(sequence);

    // Calculate sequence complexity
    let complexity = calculate_complexity(sequence);

    // Calculate codon usage if applicable
    let codon_usage = if length % 3 == 0 && length > 0 {
        calculate_codon_usage(sequence, genetic_code)
    } else {
        None
    };

    // Calculate quality statistics if provided
    let quality_stats = quality_scores.map(calculate_quality_stats);

    DetailedStats {
        length,
        gc_percent,
        at_percent,
        n_percent,
        base_counts,
        dinucleotide_counts: dinucleotides,
        gc_skew,
        at_skew,
        entropy,
        complexity,
        codon_usage,
        quality_stats,
    }
}

/// Calculate Shannon entropy of a sequence
fn calculate_entropy(sequence: &str) -> f64 {
    let mut freq_map: HashMap<char, usize> = HashMap::new();
    let length = sequence.len() as f64;

    if length == 0.0 {
        return 0.0;
    }

    // Count frequencies
    for c in sequence.chars() {
        *freq_map.entry(c.to_ascii_uppercase()).or_insert(0) += 1;
    }

    // Calculate entropy
    let mut entropy = 0.0;
    for count in freq_map.values() {
        let p = *count as f64 / length;
        if p > 0.0 {
            entropy -= p * p.log2();
        }
    }

    entropy
}

/// Calculate linguistic complexity (ratio of unique k-mers)
fn calculate_complexity(sequence: &str) -> f64 {
    if sequence.len() < 3 {
        return 0.0;
    }

    let mut unique_3mers = HashMap::new();
    let chars: Vec<char> = sequence.chars().collect();

    for window in chars.windows(3) {
        let kmer = window.iter().collect::<String>();
        *unique_3mers.entry(kmer).or_insert(0) += 1;
    }

    let max_possible = (sequence.len() - 2).min(64); // 4^3 = 64 possible 3-mers
    let unique_count = unique_3mers.len();

    unique_count as f64 / max_possible as f64
}

/// Calculate statistics for sliding windows
pub fn calculate_window_stats(sequence: &str, window_size: usize, step: usize) -> Vec<WindowStats> {
    let mut stats = Vec::new();
    let chars: Vec<char> = sequence.chars().collect();

    for pos in (0..chars.len()).step_by(step) {
        if pos + window_size > chars.len() {
            break;
        }

        let window_seq: String = chars[pos..pos + window_size].iter().collect();

        // Calculate GC% for window
        let gc_count = window_seq
            .chars()
            .filter(|&c| c == 'G' || c == 'C' || c == 'g' || c == 'c')
            .count();
        let gc_percent = (gc_count as f64 / window_size as f64) * 100.0;

        // Calculate entropy for window
        let entropy = calculate_entropy(&window_seq);

        stats.push(WindowStats {
            position: pos,
            window_size,
            gc_percent,
            entropy,
        });
    }

    stats
}

/// Calculate codon usage statistics for a coding sequence
pub fn calculate_codon_usage(sequence: &str, genetic_code: Option<u8>) -> Option<CodonUsage> {
    // Only process sequences with length divisible by 3
    if sequence.len() % 3 != 0 {
        return None;
    }

    let mut codon_counts: HashMap<String, usize> = HashMap::new();
    let mut amino_acid_counts: HashMap<char, usize> = HashMap::new();
    let mut start_codons = 0;
    let mut stop_codons = 0;

    // Standard genetic code (code 1)
    let genetic_code_table = get_genetic_code(genetic_code.unwrap_or(1));

    // Process sequence in triplets
    for chunk in sequence.chars().collect::<Vec<_>>().chunks(3) {
        if chunk.len() == 3 {
            let codon = chunk.iter().collect::<String>().to_uppercase();

            // Skip codons with ambiguous bases
            if codon.contains('N') {
                continue;
            }

            *codon_counts.entry(codon.clone()).or_insert(0) += 1;

            // Translate codon to amino acid
            if let Some(&aa) = genetic_code_table.get(codon.as_str()) {
                *amino_acid_counts.entry(aa).or_insert(0) += 1;

                // Count start and stop codons
                if codon == "ATG" {
                    start_codons += 1;
                }
                if codon == "TAA" || codon == "TAG" || codon == "TGA" {
                    stop_codons += 1;
                }
            }
        }
    }

    // Calculate frequencies
    let total_codons: usize = codon_counts.values().sum();
    let mut codon_frequencies = HashMap::new();
    for (codon, count) in &codon_counts {
        codon_frequencies.insert(codon.clone(), *count as f64 / total_codons as f64);
    }

    // Identify rare codons (frequency < 0.01)
    let rare_codons: Vec<String> = codon_frequencies
        .iter()
        .filter(|(_, freq)| **freq < 0.01)
        .map(|(codon, _)| codon.clone())
        .collect();

    Some(CodonUsage {
        codon_counts,
        codon_frequencies,
        amino_acid_counts,
        start_codons,
        stop_codons,
        rare_codons,
    })
}

/// Get genetic code table
fn get_genetic_code(_code: u8) -> HashMap<&'static str, char> {
    // Standard genetic code (NCBI code 1)
    let mut table = HashMap::new();

    // Phenylalanine
    table.insert("TTT", 'F');
    table.insert("TTC", 'F');
    // Leucine
    table.insert("TTA", 'L');
    table.insert("TTG", 'L');
    table.insert("CTT", 'L');
    table.insert("CTC", 'L');
    table.insert("CTA", 'L');
    table.insert("CTG", 'L');
    // Isoleucine
    table.insert("ATT", 'I');
    table.insert("ATC", 'I');
    table.insert("ATA", 'I');
    // Methionine
    table.insert("ATG", 'M');
    // Valine
    table.insert("GTT", 'V');
    table.insert("GTC", 'V');
    table.insert("GTA", 'V');
    table.insert("GTG", 'V');
    // Serine
    table.insert("TCT", 'S');
    table.insert("TCC", 'S');
    table.insert("TCA", 'S');
    table.insert("TCG", 'S');
    table.insert("AGT", 'S');
    table.insert("AGC", 'S');
    // Proline
    table.insert("CCT", 'P');
    table.insert("CCC", 'P');
    table.insert("CCA", 'P');
    table.insert("CCG", 'P');
    // Threonine
    table.insert("ACT", 'T');
    table.insert("ACC", 'T');
    table.insert("ACA", 'T');
    table.insert("ACG", 'T');
    // Alanine
    table.insert("GCT", 'A');
    table.insert("GCC", 'A');
    table.insert("GCA", 'A');
    table.insert("GCG", 'A');
    // Tyrosine
    table.insert("TAT", 'Y');
    table.insert("TAC", 'Y');
    // Stop codons
    table.insert("TAA", '*');
    table.insert("TAG", '*');
    table.insert("TGA", '*');
    // Histidine
    table.insert("CAT", 'H');
    table.insert("CAC", 'H');
    // Glutamine
    table.insert("CAA", 'Q');
    table.insert("CAG", 'Q');
    // Asparagine
    table.insert("AAT", 'N');
    table.insert("AAC", 'N');
    // Lysine
    table.insert("AAA", 'K');
    table.insert("AAG", 'K');
    // Aspartic acid
    table.insert("GAT", 'D');
    table.insert("GAC", 'D');
    // Glutamic acid
    table.insert("GAA", 'E');
    table.insert("GAG", 'E');
    // Cysteine
    table.insert("TGT", 'C');
    table.insert("TGC", 'C');
    // Tryptophan
    table.insert("TGG", 'W');
    // Arginine
    table.insert("CGT", 'R');
    table.insert("CGC", 'R');
    table.insert("CGA", 'R');
    table.insert("CGG", 'R');
    table.insert("AGA", 'R');
    table.insert("AGG", 'R');
    // Glycine
    table.insert("GGT", 'G');
    table.insert("GGC", 'G');
    table.insert("GGA", 'G');
    table.insert("GGG", 'G');

    table
}

/// Calculate quality statistics for FASTQ sequences
pub fn calculate_quality_stats(quality_scores: &[u8]) -> QualityStats {
    if quality_scores.is_empty() {
        return QualityStats {
            mean_quality: 0.0,
            median_quality: 0.0,
            min_quality: 0,
            max_quality: 0,
            q20_bases: 0,
            q30_bases: 0,
            quality_distribution: HashMap::new(),
        };
    }

    let mut quality_distribution = HashMap::new();
    let mut q20_bases = 0;
    let mut q30_bases = 0;
    let mut sorted_scores = quality_scores.to_vec();

    // Calculate distribution and Q20/Q30 counts
    for &score in quality_scores {
        *quality_distribution.entry(score).or_insert(0) += 1;
        if score >= 20 {
            q20_bases += 1;
        }
        if score >= 30 {
            q30_bases += 1;
        }
    }

    // Calculate mean
    let sum: u32 = quality_scores.iter().map(|&s| s as u32).sum();
    let mean_quality = sum as f64 / quality_scores.len() as f64;

    // Calculate median
    sorted_scores.sort_unstable();
    let median_quality = if sorted_scores.len() % 2 == 0 {
        let mid = sorted_scores.len() / 2;
        (sorted_scores[mid - 1] as f64 + sorted_scores[mid] as f64) / 2.0
    } else {
        sorted_scores[sorted_scores.len() / 2] as f64
    };

    QualityStats {
        mean_quality,
        median_quality,
        min_quality: *sorted_scores.first().unwrap(),
        max_quality: *sorted_scores.last().unwrap(),
        q20_bases,
        q30_bases,
        quality_distribution,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detailed_stats() {
        let sequence = "ATCGATCGATCG";
        let stats = calculate_detailed_stats(sequence);

        assert_eq!(stats.length, 12);
        assert_eq!(stats.base_counts.a, 3);
        assert_eq!(stats.base_counts.t, 3);
        assert_eq!(stats.base_counts.g, 3);
        assert_eq!(stats.base_counts.c, 3);
        assert_eq!(stats.gc_percent, 50.0);
        assert_eq!(stats.at_percent, 50.0);
        assert_eq!(stats.gc_skew, 0.0); // Equal G and C
        assert_eq!(stats.at_skew, 0.0); // Equal A and T
    }

    #[test]
    fn test_entropy() {
        // Uniform distribution should have maximum entropy
        let uniform = "ATCGATCG";
        let uniform_entropy = calculate_entropy(uniform);

        // Biased sequence should have lower entropy
        let biased = "AAAAAAAA";
        let biased_entropy = calculate_entropy(biased);

        assert!(uniform_entropy > biased_entropy);
        assert_eq!(biased_entropy, 0.0); // Single character has 0 entropy
    }

    #[test]
    fn test_window_stats() {
        let sequence = "GGGGCCCCAAAATTTT";
        let windows = calculate_window_stats(sequence, 4, 4);

        assert_eq!(windows.len(), 4);
        assert_eq!(windows[0].gc_percent, 100.0); // GGGG
        assert_eq!(windows[1].gc_percent, 100.0); // CCCC
        assert_eq!(windows[2].gc_percent, 0.0); // AAAA
        assert_eq!(windows[3].gc_percent, 0.0); // TTTT
    }

    #[test]
    fn test_dinucleotides() {
        let sequence = "ATCG";
        let stats = calculate_detailed_stats(sequence);

        assert!(stats.dinucleotide_counts.contains_key("AT"));
        assert!(stats.dinucleotide_counts.contains_key("TC"));
        assert!(stats.dinucleotide_counts.contains_key("CG"));
        assert_eq!(stats.dinucleotide_counts.get("AT"), Some(&1));
    }

    #[test]
    fn test_complexity() {
        let repetitive = "AAAAAAAAAA";
        let complex = "ATCGATCGAT";

        let rep_complexity = calculate_complexity(repetitive);
        let com_complexity = calculate_complexity(complex);

        assert!(com_complexity > rep_complexity);
    }

    #[test]
    fn test_codon_usage() {
        let cds = "ATGGCACGTTAA"; // ATG-GCA-CGT-TAA (M-A-R-*)
        let usage = calculate_codon_usage(cds, None).unwrap();

        assert_eq!(usage.codon_counts.get("ATG"), Some(&1));
        assert_eq!(usage.codon_counts.get("GCA"), Some(&1));
        assert_eq!(usage.codon_counts.get("CGT"), Some(&1));
        assert_eq!(usage.codon_counts.get("TAA"), Some(&1));
        assert_eq!(usage.start_codons, 1);
        assert_eq!(usage.stop_codons, 1);
        assert_eq!(usage.amino_acid_counts.get(&'M'), Some(&1));
        assert_eq!(usage.amino_acid_counts.get(&'A'), Some(&1));
        assert_eq!(usage.amino_acid_counts.get(&'R'), Some(&1));
        assert_eq!(usage.amino_acid_counts.get(&'*'), Some(&1));
    }

    #[test]
    fn test_codon_usage_invalid_length() {
        let seq = "ATGG"; // Not divisible by 3
        let usage = calculate_codon_usage(seq, None);
        assert!(usage.is_none());
    }

    #[test]
    fn test_quality_stats() {
        let quality_scores = vec![20, 25, 30, 35, 40, 15, 20, 25, 30, 35];
        let stats = calculate_quality_stats(&quality_scores);

        assert_eq!(stats.min_quality, 15);
        assert_eq!(stats.max_quality, 40);
        assert_eq!(stats.q20_bases, 9); // All except 15 (which is less than 20)
        assert_eq!(stats.q30_bases, 5); // 30, 35, 40, 30, 35
        assert!((stats.mean_quality - 27.5).abs() < 0.01);
        assert!((stats.median_quality - 27.5).abs() < 0.01);
    }

    #[test]
    fn test_quality_stats_empty() {
        let quality_scores = vec![];
        let stats = calculate_quality_stats(&quality_scores);

        assert_eq!(stats.mean_quality, 0.0);
        assert_eq!(stats.median_quality, 0.0);
        assert_eq!(stats.min_quality, 0);
        assert_eq!(stats.max_quality, 0);
        assert_eq!(stats.q20_bases, 0);
        assert_eq!(stats.q30_bases, 0);
    }

    #[test]
    fn test_detailed_stats_with_options() {
        let seq = "ATGGCACGTTAA";
        let quality_scores = vec![30, 35, 40, 25, 30, 35, 30, 35, 40, 30, 35, 40];
        let stats = calculate_detailed_stats_with_options(seq, Some(&quality_scores), Some(1));

        // Check basic stats
        assert_eq!(stats.length, 12);

        // Check codon usage
        assert!(stats.codon_usage.is_some());
        let codon_usage = stats.codon_usage.unwrap();
        assert_eq!(codon_usage.start_codons, 1);
        assert_eq!(codon_usage.stop_codons, 1);

        // Check quality stats
        assert!(stats.quality_stats.is_some());
        let quality_stats = stats.quality_stats.unwrap();
        assert_eq!(quality_stats.q30_bases, 11); // All except one 25
    }
}
