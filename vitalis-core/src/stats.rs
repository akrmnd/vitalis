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

/// Calculate detailed statistics for a sequence
pub fn calculate_detailed_stats(sequence: &str) -> DetailedStats {
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
}
