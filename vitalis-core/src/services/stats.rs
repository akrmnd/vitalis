// Service layer: Statistics service implementation
use crate::domain::{BaseCount, DetailedStats, StatsService, WindowStats};
use std::collections::HashMap;

/// Statistics service implementation
pub struct StatsServiceImpl;

impl StatsServiceImpl {
    pub fn new() -> Self {
        Self
    }

    /// Calculate Shannon entropy of a sequence
    fn calculate_entropy(&self, sequence: &str) -> f64 {
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
    fn calculate_complexity(&self, sequence: &str) -> f64 {
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
}

impl StatsService for StatsServiceImpl {
    fn calculate_detailed_stats(&self, sequence: &str) -> DetailedStats {
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
        let entropy = self.calculate_entropy(sequence);

        // Calculate sequence complexity
        let complexity = self.calculate_complexity(sequence);

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
            codon_usage: None,   // Will be calculated separately if needed
            quality_stats: None, // Will be added from FASTQ data if available
        }
    }

    fn calculate_window_stats(
        &self,
        sequence: &str,
        window_size: usize,
        step: usize,
    ) -> Vec<WindowStats> {
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
            let entropy = self.calculate_entropy(&window_seq);

            stats.push(WindowStats {
                position: pos,
                window_size,
                gc_percent,
                entropy,
            });
        }

        stats
    }
}
