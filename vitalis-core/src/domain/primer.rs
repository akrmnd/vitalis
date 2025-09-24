use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// プライマー設計パラメータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimerDesignParams {
    pub length_min: usize,
    pub length_max: usize,
    pub tm_min: f32,
    pub tm_max: f32,
    pub tm_optimal: f32,
    pub gc_min: f32,
    pub gc_max: f32,
    pub max_self_dimer: f32,
    pub max_hairpin: f32,
    pub max_hetero_dimer: f32,
}

impl Default for PrimerDesignParams {
    fn default() -> Self {
        Self {
            length_min: 18,
            length_max: 25,
            tm_min: 55.0,
            tm_max: 65.0,
            tm_optimal: 60.0,
            gc_min: 40.0,
            gc_max: 60.0,
            max_self_dimer: -5.0,
            max_hairpin: -3.0,
            max_hetero_dimer: -5.0,
        }
    }
}

/// 単一プライマー
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Primer {
    pub sequence: String,
    pub position: usize,
    pub length: usize,
    pub tm: f32,
    pub gc_content: f32,
    pub self_dimer_score: f32,
    pub hairpin_score: f32,
    pub three_prime_stability: f32,
    pub direction: PrimerDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PrimerDirection {
    Forward,
    Reverse,
}

/// プライマーペア
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimerPair {
    pub id: String,
    pub forward: Primer,
    pub reverse: Primer,
    pub amplicon_length: usize,
    pub amplicon_sequence: String,
    pub target_gene: Option<String>,
    pub target_transcript: Option<String>,
    pub compatibility_score: f32,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub validation_results: ValidationResults,
}

/// バリデーション結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResults {
    pub self_dimer_check: bool,
    pub hairpin_check: bool,
    pub hetero_dimer_check: Option<bool>,
    pub specificity: Option<f32>,
    pub warnings: Vec<String>,
}

impl ValidationResults {
    pub fn new() -> Self {
        Self {
            self_dimer_check: false,
            hairpin_check: false,
            hetero_dimer_check: None,
            specificity: None,
            warnings: Vec::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.self_dimer_check && self.hairpin_check && self.warnings.is_empty()
    }
}

/// マルチプレックス互換性結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiplexCompatibility {
    pub compatibility_matrix: HashMap<String, HashMap<String, f32>>,
    pub warnings: Vec<String>,
    pub overall_score: f32,
}

/// プライマー設計結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimerDesignResult {
    pub pairs: Vec<PrimerPair>,
    pub design_params: PrimerDesignParams,
    pub target_sequence: String,
    pub target_start: usize,
    pub target_end: usize,
    pub multiplex_compatibility: Option<MultiplexCompatibility>,
}

/// プライマー設計サービストレイト
pub trait PrimerDesignService {
    type Error: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static;

    /// 指定範囲でプライマーペアを設計
    fn design_primers(
        &self,
        sequence: &str,
        start: usize,
        end: usize,
        params: &PrimerDesignParams,
    ) -> Result<PrimerDesignResult, Self::Error>;

    /// Tm値計算（Nearest Neighbor法）
    fn calculate_tm(&self, sequence: &str) -> f32;

    /// GC含量計算
    fn calculate_gc_content(&self, sequence: &str) -> f32;

    /// セルフダイマー評価
    fn calculate_self_dimer(&self, sequence: &str) -> f32;

    /// ヘアピン構造評価
    fn calculate_hairpin(&self, sequence: &str) -> f32;

    /// プライマー間相互作用評価
    fn calculate_hetero_dimer(&self, primer1: &str, primer2: &str) -> f32;

    /// マルチプレックス互換性評価
    fn evaluate_multiplex(&self, primers: &[PrimerPair]) -> MultiplexCompatibility;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primer_design_params_default() {
        let params = PrimerDesignParams::default();
        assert_eq!(params.length_min, 18);
        assert_eq!(params.length_max, 25);
        assert_eq!(params.tm_optimal, 60.0);
    }

    #[test]
    fn test_validation_results() {
        let mut validation = ValidationResults::new();
        assert!(!validation.is_valid());

        validation.self_dimer_check = true;
        validation.hairpin_check = true;
        assert!(validation.is_valid());

        validation.warnings.push("Warning message".to_string());
        assert!(!validation.is_valid());
    }
}