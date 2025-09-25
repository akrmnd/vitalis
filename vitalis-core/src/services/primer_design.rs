use crate::domain::primer::*;
use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

/// プライマー設計サービス実装
pub struct PrimerDesignServiceImpl;

impl PrimerDesignServiceImpl {
    pub fn new() -> Self {
        Self
    }

    /// DNA配列を逆相補配列に変換
    fn reverse_complement(&self, sequence: &str) -> String {
        sequence
            .chars()
            .rev()
            .map(|base| match base.to_ascii_uppercase() {
                'A' => 'T',
                'T' => 'A',
                'G' => 'C',
                'C' => 'G',
                _ => base,
            })
            .collect()
    }

    /// 配列から候補プライマーを生成
    fn generate_primer_candidates(
        &self,
        sequence: &str,
        start: usize,
        end: usize,
        params: &PrimerDesignParams,
        direction: PrimerDirection,
    ) -> Vec<Primer> {
        let mut primers = Vec::new();
        let _target_seq = &sequence[start..=end];

        for length in params.length_min..=params.length_max {
            // Forward primers: target regionの開始付近
            // Reverse primers: target regionの終了付近
            let positions = if direction == PrimerDirection::Forward {
                // Forward primer positions around start
                let range_start = start.saturating_sub(50);
                let range_end = (start + 50).min(sequence.len().saturating_sub(length));
                range_start..=range_end
            } else {
                // Reverse primer positions around end
                let range_start = end.saturating_sub(50);
                let range_end = (end + 50).min(sequence.len().saturating_sub(length));
                range_start..=range_end
            };

            for pos in positions {
                if pos + length > sequence.len() {
                    continue;
                }

                let primer_seq = if direction == PrimerDirection::Forward {
                    sequence[pos..pos + length].to_string()
                } else {
                    self.reverse_complement(&sequence[pos..pos + length])
                };

                let tm = self.calculate_tm(&primer_seq);
                let gc = self.calculate_gc_content(&primer_seq);

                // 基本フィルタリング
                if tm >= params.tm_min
                    && tm <= params.tm_max
                    && gc >= params.gc_min
                    && gc <= params.gc_max
                {
                    let self_dimer = self.calculate_self_dimer(&primer_seq);
                    let hairpin = self.calculate_hairpin(&primer_seq);
                    let three_prime = self.calculate_three_prime_stability(&primer_seq);

                    primers.push(Primer {
                        sequence: primer_seq,
                        position: pos,
                        length,
                        tm,
                        gc_content: gc,
                        self_dimer_score: self_dimer,
                        hairpin_score: hairpin,
                        three_prime_stability: three_prime,
                        direction: direction.clone(),
                    });
                }
            }
        }

        // Tm値の最適値に近い順にソート
        primers.sort_by(|a, b| {
            let a_diff = (a.tm - params.tm_optimal).abs();
            let b_diff = (b.tm - params.tm_optimal).abs();
            a_diff.partial_cmp(&b_diff).unwrap()
        });

        primers.truncate(50); // 上位50候補まで
        primers
    }

    /// 3'末端の安定性を計算
    fn calculate_three_prime_stability(&self, sequence: &str) -> f32 {
        if sequence.len() < 5 {
            return 0.0;
        }

        let three_prime = &sequence[sequence.len() - 5..];
        self.calculate_tm(three_prime)
    }

    /// プライマーペアの適合性をチェック
    fn is_compatible_pair(&self, forward: &Primer, reverse: &Primer, params: &PrimerDesignParams) -> bool {
        // Tm差が大きすぎる場合は不適合
        let tm_diff = (forward.tm - reverse.tm).abs();
        if tm_diff > 3.0 {
            return false;
        }

        // プライマー間の相互作用をチェック（より負の値=より強い結合=悪い）
        let hetero_dimer = self.calculate_hetero_dimer(&forward.sequence, &reverse.sequence);
        if hetero_dimer < -params.max_hetero_dimer.abs() {
            return false;
        }

        true
    }
}

impl PrimerDesignService for PrimerDesignServiceImpl {
    type Error = anyhow::Error;

    fn design_primers(
        &self,
        sequence: &str,
        start: usize,
        end: usize,
        params: &PrimerDesignParams,
    ) -> Result<PrimerDesignResult, Self::Error> {
        if start >= end || end > sequence.len() {
            return Err(anyhow::anyhow!("Invalid target region"));
        }

        // Forward and reverse primer candidates generation
        let forward_candidates = self.generate_primer_candidates(
            sequence,
            start,
            end,
            params,
            PrimerDirection::Forward,
        );

        let reverse_candidates = self.generate_primer_candidates(
            sequence,
            start,
            end,
            params,
            PrimerDirection::Reverse,
        );

        let mut pairs = Vec::new();

        // Generate primer pairs
        for forward in &forward_candidates {
            for reverse in &reverse_candidates {
                if !self.is_compatible_pair(forward, reverse, params) {
                    continue;
                }

                let amplicon_start = forward.position.min(reverse.position);
                let amplicon_end = forward.position.max(reverse.position) + forward.length.max(reverse.length);
                let amplicon_length = amplicon_end - amplicon_start;

                // 適切な増幅産物サイズかチェック
                if amplicon_length < 100 || amplicon_length > 3000 {
                    continue;
                }

                let amplicon_sequence = sequence[amplicon_start..amplicon_end].to_string();

                let mut validation = ValidationResults::new();
                validation.self_dimer_check = forward.self_dimer_score >= params.max_self_dimer
                    && reverse.self_dimer_score >= params.max_self_dimer;
                validation.hairpin_check = forward.hairpin_score >= params.max_hairpin
                    && reverse.hairpin_score >= params.max_hairpin;

                let pair = PrimerPair {
                    id: Uuid::new_v4().to_string(),
                    forward: forward.clone(),
                    reverse: reverse.clone(),
                    amplicon_length,
                    amplicon_sequence,
                    target_gene: None,
                    target_transcript: None,
                    compatibility_score: 0.0, // 後で計算
                    created_by: "system".to_string(),
                    created_at: Utc::now(),
                    tags: Vec::new(),
                    validation_results: validation,
                };

                pairs.push(pair);
            }
        }

        // 最良の候補10組まで
        pairs.sort_by(|a, b| {
            // スコアリング: Tm最適値からの差、GC含量、二次構造スコア
            let score_a = self.calculate_pair_score(a, params);
            let score_b = self.calculate_pair_score(b, params);
            score_b.partial_cmp(&score_a).unwrap()
        });

        pairs.truncate(10);

        Ok(PrimerDesignResult {
            pairs,
            design_params: params.clone(),
            target_sequence: sequence[start..=end].to_string(),
            target_start: start,
            target_end: end,
            multiplex_compatibility: None,
        })
    }

    fn calculate_tm(&self, sequence: &str) -> f32 {
        // 簡単なTm計算（Wallace rule）
        // より正確にはNearest Neighbor法を使用すべき
        let seq_upper = sequence.to_uppercase();
        let at_count = seq_upper.chars().filter(|&c| c == 'A' || c == 'T').count();
        let gc_count = seq_upper.chars().filter(|&c| c == 'G' || c == 'C').count();

        if sequence.len() <= 13 {
            // Short primers: Wallace rule
            (at_count as f32) * 2.0 + (gc_count as f32) * 4.0
        } else {
            // Long primers: basic formula
            64.9 + 41.0 * (gc_count as f32 - 16.4) / sequence.len() as f32
        }
    }

    fn calculate_gc_content(&self, sequence: &str) -> f32 {
        let seq_upper = sequence.to_uppercase();
        let gc_count = seq_upper.chars().filter(|&c| c == 'G' || c == 'C').count();
        (gc_count as f32 / sequence.len() as f32) * 100.0
    }

    fn calculate_self_dimer(&self, sequence: &str) -> f32 {
        // 簡易的なセルフダイマー評価
        // より正確にはthermodynamic calculationが必要
        let rev_comp = sequence
            .chars()
            .rev()
            .map(|base| match base.to_ascii_uppercase() {
                'A' => 'T',
                'T' => 'A',
                'G' => 'C',
                'C' => 'G',
                _ => base,
            })
            .collect::<String>();

        self.calculate_alignment_score(sequence, &rev_comp)
    }

    fn calculate_hairpin(&self, sequence: &str) -> f32 {
        // 簡易的なヘアピン評価
        // palindromic subsequenceの検出
        let seq_chars: Vec<char> = sequence.chars().collect();
        let mut max_hairpin = 0.0;

        for i in 0..seq_chars.len() {
            for j in (i + 4)..seq_chars.len() {
                let sub1 = &seq_chars[i..j];
                let sub2_rev: Vec<char> = seq_chars[j..].iter().rev().cloned().collect();

                if sub1.len() == sub2_rev.len() {
                    let matches = sub1
                        .iter()
                        .zip(sub2_rev.iter())
                        .filter(|(a, b)| self.is_complement(**a, **b))
                        .count();

                    let score = -(matches as f32) * 1.0;
                    if score < max_hairpin {
                        max_hairpin = score;
                    }
                }
            }
        }

        max_hairpin
    }

    fn calculate_hetero_dimer(&self, primer1: &str, primer2: &str) -> f32 {
        // 簡易的なヘテロダイマー評価
        self.calculate_alignment_score(primer1, primer2)
    }

    fn evaluate_multiplex(&self, primers: &[PrimerPair]) -> MultiplexCompatibility {
        let mut compatibility_matrix = HashMap::new();
        let mut warnings = Vec::new();

        for (i, pair1) in primers.iter().enumerate() {
            let mut row = HashMap::new();
            for (j, pair2) in primers.iter().enumerate() {
                if i != j {
                    // Cross-reactivity score calculation
                    let score = self.calculate_cross_reactivity(pair1, pair2);
                    row.insert(pair2.id.clone(), score);

                    if score < -3.0 {
                        warnings.push(format!(
                            "Potential cross-reactivity between {} and {}",
                            pair1.id, pair2.id
                        ));
                    }
                }
            }
            compatibility_matrix.insert(pair1.id.clone(), row);
        }

        let overall_score = if warnings.is_empty() { 1.0 } else { 0.5 };

        MultiplexCompatibility {
            compatibility_matrix,
            warnings,
            overall_score,
        }
    }
}

impl PrimerDesignServiceImpl {
    /// プライマーペアのスコア計算
    fn calculate_pair_score(&self, pair: &PrimerPair, params: &PrimerDesignParams) -> f32 {
        let tm_score = 1.0 - ((pair.forward.tm - params.tm_optimal).abs()
                           + (pair.reverse.tm - params.tm_optimal).abs()) / 10.0;
        let gc_score = (pair.forward.gc_content + pair.reverse.gc_content) / 200.0;
        let secondary_score = (pair.forward.self_dimer_score + pair.forward.hairpin_score
                             + pair.reverse.self_dimer_score + pair.reverse.hairpin_score) / 4.0;

        tm_score + gc_score - secondary_score.abs() / 10.0
    }

    /// 配列アライメントスコア計算（簡易版）
    fn calculate_alignment_score(&self, seq1: &str, seq2: &str) -> f32 {
        let s1: Vec<char> = seq1.chars().collect();
        let s2: Vec<char> = seq2.chars().collect();

        let mut max_score = 0.0;

        // Sliding window alignment
        for offset in 0..s1.len() {
            let mut score = 0.0;
            let mut matches = 0;

            for i in 0..(s1.len() - offset).min(s2.len()) {
                if self.is_complement(s1[i + offset], s2[i]) {
                    score -= 2.0; // Negative because binding is energetically favorable
                    matches += 1;
                } else if s1[i + offset] == s2[i] {
                    score -= 0.5;
                }
            }

            if matches >= 3 && score < max_score {
                max_score = score;
            }
        }

        max_score
    }

    /// 塩基の相補性チェック
    fn is_complement(&self, base1: char, base2: char) -> bool {
        let b1 = base1.to_ascii_uppercase();
        let b2 = base2.to_ascii_uppercase();

        matches!((b1, b2), ('A', 'T') | ('T', 'A') | ('G', 'C') | ('C', 'G'))
    }

    /// クロスリアクティビティスコア計算
    fn calculate_cross_reactivity(&self, pair1: &PrimerPair, pair2: &PrimerPair) -> f32 {
        let scores = vec![
            self.calculate_hetero_dimer(&pair1.forward.sequence, &pair2.forward.sequence),
            self.calculate_hetero_dimer(&pair1.forward.sequence, &pair2.reverse.sequence),
            self.calculate_hetero_dimer(&pair1.reverse.sequence, &pair2.forward.sequence),
            self.calculate_hetero_dimer(&pair1.reverse.sequence, &pair2.reverse.sequence),
        ];

        scores.into_iter().fold(0.0, f32::min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tm_calculation() {
        let service = PrimerDesignServiceImpl::new();
        let tm = service.calculate_tm("ATGCGCGCGCAT");
        assert!(tm > 40.0);
        assert!(tm < 80.0);
    }

    #[test]
    fn test_gc_content() {
        let service = PrimerDesignServiceImpl::new();
        let gc = service.calculate_gc_content("ATGCGCGCGCAT");
        assert_eq!(gc, 66.66667);
    }

    #[test]
    fn test_reverse_complement() {
        let service = PrimerDesignServiceImpl::new();
        let rc = service.reverse_complement("ATGC");
        assert_eq!(rc, "GCAT");
    }
}