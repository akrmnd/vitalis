use super::thermodynamics::{DNAThermodynamicsDatabase, ThermodynamicParams, SaltCorrectionParams};
use serde::{Deserialize, Serialize};

/// 改良された熱力学計算エンジン（NNDB 2024対応）
pub struct ThermodynamicCalculator {
    /// 熱力学パラメータデータベース
    database: DNAThermodynamicsDatabase,
    /// 計算条件設定
    conditions: CalculationConditions,
}

/// 熱力学計算の条件設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationConditions {
    /// 温度 (Kelvin)
    pub temperature_k: f32,
    /// プライマー濃度 (M)
    pub primer_concentration: f32,
    /// 対称性補正を適用するか
    pub apply_symmetry_correction: bool,
    /// 分子クラウディング効果を考慮するか（NNDB 2024新機能）
    pub molecular_crowding: bool,
    /// ミスマッチペナルティ重み
    pub mismatch_penalty_weight: f32,
}

impl Default for CalculationConditions {
    fn default() -> Self {
        Self {
            temperature_k: 310.15,  // 37°C
            primer_concentration: 1e-6,  // 1 μM
            apply_symmetry_correction: true,
            molecular_crowding: false,  // デフォルトはオフ
            mismatch_penalty_weight: 1.0,
        }
    }
}

/// 包括的な熱力学計算結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveThermodynamicResult {
    /// エンタルピー変化 (kcal/mol)
    pub delta_h: f32,
    /// エントロピー変化 (cal/mol·K)
    pub delta_s: f32,
    /// ギブス自由エネルギー変化 (kcal/mol)
    pub delta_g: f32,
    /// 融解温度 (°C)
    pub melting_temperature: f32,
    /// 二次構造形成確率 (0-1)
    pub formation_probability: f32,
    /// 適用された補正項目
    pub corrections_applied: Vec<String>,
    /// 詳細な寄与の内訳
    pub contribution_breakdown: ContributionBreakdown,
}

/// エネルギー寄与の内訳
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionBreakdown {
    /// 最近隣効果
    pub nearest_neighbor: f32,
    /// 末端効果
    pub terminal_effects: f32,
    /// ミスマッチペナルティ
    pub mismatch_penalty: f32,
    /// ループ構造
    pub loop_structures: f32,
    /// 塩濃度補正
    pub salt_correction: f32,
    /// 分子クラウディング
    pub molecular_crowding: f32,
}

impl ThermodynamicCalculator {
    /// NNDB 2024パラメータで初期化
    pub fn new_nndb_2024() -> Self {
        Self {
            database: DNAThermodynamicsDatabase::nndb_2024(),
            conditions: CalculationConditions::default(),
        }
    }

    /// SantaLucia 1998パラメータで初期化（後方互換性）
    pub fn new_santalucia_1998() -> Self {
        Self {
            database: DNAThermodynamicsDatabase::santalucia_1998(),
            conditions: CalculationConditions::default(),
        }
    }

    /// カスタムデータベースで初期化
    pub fn new_with_database(database: DNAThermodynamicsDatabase) -> Self {
        Self {
            database,
            conditions: CalculationConditions::default(),
        }
    }

    /// 計算条件を設定
    pub fn set_conditions(&mut self, conditions: CalculationConditions) {
        self.conditions = conditions;
    }

    /// 計算条件を取得
    pub fn get_conditions(&self) -> &CalculationConditions {
        &self.conditions
    }

    /// 包括的な熱力学計算（NNDB 2024の全機能を活用）
    pub fn calculate_comprehensive(&self, sequence: &str) -> Result<ComprehensiveThermodynamicResult, ThermodynamicError> {
        if sequence.len() < 2 {
            return Err(ThermodynamicError::SequenceTooShort);
        }

        let sequence = sequence.to_uppercase();
        let mut breakdown = ContributionBreakdown {
            nearest_neighbor: 0.0,
            terminal_effects: 0.0,
            mismatch_penalty: 0.0,
            loop_structures: 0.0,
            salt_correction: 0.0,
            molecular_crowding: 0.0,
        };

        let mut corrections = Vec::new();
        let mut total_delta_h = 0.0;
        let mut total_delta_s = 0.0;

        // 1. 最近隣効果の計算
        for i in 0..sequence.len().saturating_sub(1) {
            let dinucleotide = &sequence[i..i+2];
            let reverse_complement = self.reverse_complement_dinucleotide(dinucleotide)?;
            let key = format!("{}/{}", dinucleotide, reverse_complement);

            if let Some(params) = self.database.get_nearest_neighbor(&key) {
                total_delta_h += params.delta_h;
                total_delta_s += params.delta_s;
                breakdown.nearest_neighbor += params.delta_h;
            } else {
                // 代替キーパターンを試行
                let alt_key = format!("{}/{}", reverse_complement, dinucleotide);
                if let Some(params) = self.database.get_nearest_neighbor(&alt_key) {
                    total_delta_h += params.delta_h;
                    total_delta_s += params.delta_s;
                    breakdown.nearest_neighbor += params.delta_h;
                } else {
                    return Err(ThermodynamicError::UnknownDinucleotide(dinucleotide.to_string()));
                }
            }
        }

        // 2. 末端効果の計算
        if let Some(first_base) = sequence.chars().next() {
            if let Some(params) = self.database.get_initiation(&first_base.to_string()) {
                total_delta_h += params.delta_h;
                total_delta_s += params.delta_s;
                breakdown.terminal_effects += params.delta_h;
            }
        }

        if let Some(last_base) = sequence.chars().last() {
            if let Some(params) = self.database.get_initiation(&last_base.to_string()) {
                total_delta_h += params.delta_h;
                total_delta_s += params.delta_s;
                breakdown.terminal_effects += params.delta_h;
            }
        }

        corrections.push("terminal_correction".to_string());

        // 3. 対称性補正（回文配列）
        if self.conditions.apply_symmetry_correction && self.is_palindrome(&sequence) {
            total_delta_s -= 1.4; // エントロピー減少
            corrections.push("symmetry_correction".to_string());
        }

        // 4. 塩濃度補正の適用
        let salt_correction_entropy = self.apply_advanced_salt_correction(
            total_delta_s,
            sequence.len(),
            &self.database.salt_correction
        );
        breakdown.salt_correction = salt_correction_entropy - total_delta_s;
        total_delta_s = salt_correction_entropy;
        corrections.push("advanced_salt_correction".to_string());

        // 5. 分子クラウディング効果（NNDB 2024新機能）
        if self.conditions.molecular_crowding {
            let crowding_factor_h = 1.05; // エンタルピー増強
            let crowding_factor_s = 0.98; // エントロピー減少

            breakdown.molecular_crowding = total_delta_h * (crowding_factor_h - 1.0);
            total_delta_h *= crowding_factor_h;
            total_delta_s *= crowding_factor_s;
            corrections.push("molecular_crowding".to_string());
        }

        // 6. 最終計算
        let delta_g = self.calculate_delta_g_from_components(total_delta_h, total_delta_s);
        let melting_temperature = self.calculate_melting_temperature(total_delta_h, total_delta_s);
        let formation_probability = self.calculate_formation_probability_internal(delta_g);

        Ok(ComprehensiveThermodynamicResult {
            delta_h: total_delta_h,
            delta_s: total_delta_s,
            delta_g,
            melting_temperature,
            formation_probability,
            corrections_applied: corrections,
            contribution_breakdown: breakdown,
        })
    }

    /// 最近接法によるTm値計算（改良版）
    pub fn calculate_tm_nearest_neighbor(&self, sequence: &str) -> Result<f32, ThermodynamicError> {
        self.calculate_tm_with_conditions(sequence, &self.database.salt_correction, self.conditions.temperature_k)
    }

    /// 条件指定でのTm値計算
    pub fn calculate_tm_with_conditions(
        &self,
        sequence: &str,
        salt_conditions: &SaltCorrectionParams,
        temperature_k: f32,
    ) -> Result<f32, ThermodynamicError> {
        if sequence.len() < 2 {
            return Err(ThermodynamicError::SequenceTooShort);
        }

        let sequence = sequence.to_uppercase();
        let mut total_enthalpy = 0.0f32;
        let mut total_entropy = 0.0f32;

        // 末端効果を追加
        if let Some(first_base) = sequence.chars().next() {
            if let Some(params) = self.database.get_initiation(&first_base.to_string()) {
                total_enthalpy += params.delta_h;
                total_entropy += params.delta_s;
            }
        }

        if let Some(last_base) = sequence.chars().last() {
            if let Some(params) = self.database.get_initiation(&last_base.to_string()) {
                total_enthalpy += params.delta_h;
                total_entropy += params.delta_s;
            }
        }

        // 二核酸対の寄与を計算
        for i in 0..sequence.len() - 1 {
            let dinucleotide = &sequence[i..i + 2];

            // データベースに登録されている正確なキーを探す
            let params = if let Some(params) = self.find_dinucleotide_params(dinucleotide) {
                params
            } else {
                return Err(ThermodynamicError::UnknownDinucleotide(dinucleotide.to_string()));
            };

            total_enthalpy += params.delta_h;
            total_entropy += params.delta_s;
        }

        // 塩濃度補正
        let corrected_entropy = self.apply_salt_correction(total_entropy, sequence.len(), salt_conditions);

        // Tm計算: Tm = ΔH / ΔS (エントロピーはcal/mol·Kからkcal/mol·Kに変換)
        if corrected_entropy != 0.0 {
            let tm_k = (total_enthalpy * 1000.0) / corrected_entropy; // ΔSをcal/mol·KからJ/mol·Kに変換
            Ok(tm_k - 273.15) // Kelvinから摂氏に変換
        } else {
            Err(ThermodynamicError::ZeroEntropy)
        }
    }

    /// ギブス自由エネルギー計算
    pub fn calculate_delta_g(&self, sequence: &str, temperature_k: f32) -> Result<f32, ThermodynamicError> {
        if sequence.len() < 2 {
            return Err(ThermodynamicError::SequenceTooShort);
        }

        let sequence = sequence.to_uppercase();
        let mut total_delta_g = 0.0f32;

        // 末端効果
        if let Some(first_base) = sequence.chars().next() {
            if let Some(params) = self.database.get_initiation(&first_base.to_string()) {
                total_delta_g += params.delta_g(temperature_k);
            }
        }

        if let Some(last_base) = sequence.chars().last() {
            if let Some(params) = self.database.get_initiation(&last_base.to_string()) {
                total_delta_g += params.delta_g(temperature_k);
            }
        }

        // 二核酸対の寄与
        for i in 0..sequence.len() - 1 {
            let dinucleotide = &sequence[i..i + 2];
            let reverse_complement = self.reverse_complement_dinucleotide(dinucleotide)?;
            let key = format!("{}/{}", dinucleotide, reverse_complement);

            if let Some(params) = self.database.get_nearest_neighbor(&key) {
                total_delta_g += params.delta_g(temperature_k);
            } else {
                return Err(ThermodynamicError::UnknownDinucleotide(dinucleotide.to_string()));
            }
        }

        Ok(total_delta_g)
    }

    /// セルフダイマー評価（改良版）
    pub fn calculate_enhanced_self_dimer(&self, sequence: &str) -> Result<SelfDimerAnalysis, ThermodynamicError> {
        let sequence = sequence.to_uppercase();
        let mut max_score = 0.0f32;
        let mut best_alignment = None;
        let mut alignments = Vec::new();

        // 全ての可能なアライメントをチェック
        for offset in 1..sequence.len() {
            let (score, mismatches) = self.calculate_alignment_score(&sequence, &sequence, offset)?;

            alignments.push(AlignmentResult {
                offset,
                score,
                mismatches,
                length: sequence.len() - offset,
            });

            if score < max_score { // より負の値（安定）を探す
                max_score = score;
                best_alignment = Some(offset);
            }
        }

        // 逆相補も考慮
        let reverse_complement = self.reverse_complement(sequence.as_str())?;
        for offset in 1..sequence.len() {
            let (score, mismatches) = self.calculate_alignment_score(&sequence, &reverse_complement, offset)?;

            alignments.push(AlignmentResult {
                offset,
                score,
                mismatches,
                length: sequence.len() - offset,
            });

            if score < max_score {
                max_score = score;
                best_alignment = Some(offset);
            }
        }

        Ok(SelfDimerAnalysis {
            max_score,
            best_alignment_offset: best_alignment,
            all_alignments: alignments,
            is_problematic: max_score < -8.0, // 閾値: -8.0 kcal/mol未満で問題あり
        })
    }

    /// ヘアピン構造評価（改良版）
    pub fn calculate_enhanced_hairpin(&self, sequence: &str) -> Result<HairpinAnalysis, ThermodynamicError> {
        let sequence = sequence.to_uppercase();
        let mut hairpins = Vec::new();

        // 3bp以上のステムを持つヘアピンを探索
        for stem_length in 3..=(sequence.len() / 2) {
            let min_required_length = 2 * stem_length + 3; // 最小ループ3bp
            if sequence.len() < min_required_length {
                continue;
            }
            for start in 0..=(sequence.len() - min_required_length) {
                let stem5 = &sequence[start..start + stem_length];
                let loop_start = start + stem_length;

                // 可能なループサイズを試す（3-10bp）
                for loop_size in 3..=10.min(sequence.len() - start - 2 * stem_length) {
                    let stem3_start = loop_start + loop_size;
                    if stem3_start + stem_length <= sequence.len() {
                        let stem3 = &sequence[stem3_start..stem3_start + stem_length];
                        let stem3_rc = self.reverse_complement(stem3)?;

                        if stem5 == stem3_rc {
                            let loop_seq = &sequence[loop_start..loop_start + loop_size];
                            let score = self.calculate_hairpin_score(stem_length, loop_size, loop_seq)?;

                            hairpins.push(HairpinStructure {
                                start_pos: start,
                                stem_length,
                                loop_start: loop_start,
                                loop_size,
                                stem5: stem5.to_string(),
                                loop_sequence: loop_seq.to_string(),
                                stem3: stem3.to_string(),
                                score,
                            });
                        }
                    }
                }
            }
        }

        // 最も安定なヘアピンを選択
        let best_hairpin = hairpins.iter().min_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal));
        let min_score = best_hairpin.map(|h| h.score).unwrap_or(0.0);

        Ok(HairpinAnalysis {
            min_score,
            best_hairpin: best_hairpin.cloned(),
            all_hairpins: hairpins,
            is_problematic: min_score < -5.0, // 閾値: -5.0 kcal/mol未満で問題あり
        })
    }

    /// ヘテロダイマー評価（改良版）
    pub fn calculate_enhanced_hetero_dimer(&self, primer1: &str, primer2: &str) -> Result<HeteroDimerAnalysis, ThermodynamicError> {
        let seq1 = primer1.to_uppercase();
        let seq2 = primer2.to_uppercase();
        let mut max_score = 0.0f32;
        let mut best_alignment = None;
        let mut alignments = Vec::new();

        // primer1 vs primer2 (全方向)
        for offset in 0..seq1.len() {
            let (score, mismatches) = self.calculate_alignment_score(&seq1, &seq2, offset)?;
            alignments.push(AlignmentResult {
                offset,
                score,
                mismatches,
                length: (seq1.len() - offset).min(seq2.len()),
            });

            if score < max_score {
                max_score = score;
                best_alignment = Some(offset);
            }
        }

        // primer1 vs reverse_complement(primer2)
        let seq2_rc = self.reverse_complement(&seq2)?;
        for offset in 0..seq1.len() {
            let (score, mismatches) = self.calculate_alignment_score(&seq1, &seq2_rc, offset)?;
            alignments.push(AlignmentResult {
                offset,
                score,
                mismatches,
                length: (seq1.len() - offset).min(seq2_rc.len()),
            });

            if score < max_score {
                max_score = score;
                best_alignment = Some(offset);
            }
        }

        Ok(HeteroDimerAnalysis {
            max_score,
            best_alignment_offset: best_alignment,
            all_alignments: alignments,
            is_problematic: max_score < -8.0, // 閾値: -8.0 kcal/mol未満で問題あり
        })
    }

    // ヘルパー関数

    /// 二核酸に対応する熱力学パラメータを検索
    fn find_dinucleotide_params(&self, dinucleotide: &str) -> Option<&ThermodynamicParams> {
        if dinucleotide.len() != 2 {
            return None;
        }

        let reverse_complement = match self.reverse_complement_dinucleotide(dinucleotide) {
            Ok(rc) => rc,
            Err(_) => return None,
        };

        // 直接検索
        let key1 = format!("{}/{}", dinucleotide, reverse_complement);
        if let Some(params) = self.database.get_nearest_neighbor(&key1) {
            return Some(params);
        }

        // 逆向きで検索（例：AT/TAとTA/ATは相補的）
        let key2 = format!("{}/{}", reverse_complement, dinucleotide);
        if let Some(params) = self.database.get_nearest_neighbor(&key2) {
            return Some(params);
        }
        None
    }

    fn reverse_complement_dinucleotide(&self, dinucleotide: &str) -> Result<String, ThermodynamicError> {
        if dinucleotide.len() != 2 {
            return Err(ThermodynamicError::InvalidSequence(dinucleotide.to_string()));
        }

        let complement = |base: char| -> Result<char, ThermodynamicError> {
            match base {
                'A' => Ok('T'),
                'T' => Ok('A'),
                'G' => Ok('C'),
                'C' => Ok('G'),
                _ => Err(ThermodynamicError::UnknownBase(base)),
            }
        };

        let chars: Vec<char> = dinucleotide.chars().collect();
        let rc1 = complement(chars[1])?;
        let rc0 = complement(chars[0])?;
        Ok(format!("{}{}", rc1, rc0))
    }

    fn reverse_complement(&self, sequence: &str) -> Result<String, ThermodynamicError> {
        let complement = |base: char| -> Result<char, ThermodynamicError> {
            match base {
                'A' => Ok('T'),
                'T' => Ok('A'),
                'G' => Ok('C'),
                'C' => Ok('G'),
                _ => Err(ThermodynamicError::UnknownBase(base)),
            }
        };

        sequence
            .chars()
            .rev()
            .map(complement)
            .collect::<Result<String, _>>()
    }

    fn apply_salt_correction(&self, entropy: f32, sequence_length: usize, salt: &SaltCorrectionParams) -> f32 {
        // 簡易塩濃度補正（SantaLucia model）
        let n = sequence_length as f32;
        let na_molarity = salt.sodium_concentration + salt.potassium_concentration + salt.other_monovalent;

        if na_molarity > 0.0 {
            let salt_correction = 0.368 * n * na_molarity.ln();
            entropy + salt_correction
        } else {
            entropy
        }
    }

    /// 高度な塩濃度補正（NNDB 2024拡張）
    fn apply_advanced_salt_correction(&self, entropy: f32, sequence_length: usize, salt: &SaltCorrectionParams) -> f32 {
        let n = sequence_length as f32;
        let na_conc = salt.sodium_concentration + salt.potassium_concentration + salt.other_monovalent;
        let mg_conc = salt.magnesium_concentration;

        let mut corrected_entropy = entropy;

        // Na+/K+補正
        if na_conc > 0.0 {
            corrected_entropy += 0.368 * n * na_conc.ln();
        }

        // Mg2+補正（より強い効果）
        if mg_conc > 0.0 {
            corrected_entropy += 0.175 * n * mg_conc.ln();
        }

        // 混合塩効果（Na+とMg2+の相互作用）
        if na_conc > 0.0 && mg_conc > 0.0 {
            corrected_entropy += 0.1 * n * (na_conc * mg_conc).ln();
        }

        corrected_entropy
    }

    /// ギブス自由エネルギーを内部的に計算
    fn calculate_delta_g_from_components(&self, delta_h: f32, delta_s: f32) -> f32 {
        delta_h - self.conditions.temperature_k * (delta_s / 1000.0) // ΔSをkcal/mol·Kに変換
    }

    /// 融解温度を内部的に計算
    fn calculate_melting_temperature(&self, delta_h: f32, delta_s: f32) -> f32 {
        if delta_s == 0.0 { return 0.0; }

        // Tm = ΔH / (ΔS + R*ln(C/4)) - 273.15
        // C = primer concentration
        let r = 1.987; // cal/mol·K
        let concentration_term = r * (self.conditions.primer_concentration / 4.0).ln();

        (delta_h * 1000.0) / (delta_s + concentration_term) - 273.15
    }

    /// 回文配列（palindrome）判定の改良版
    fn is_palindrome(&self, sequence: &str) -> bool {
        let complement: String = sequence.chars()
            .rev()
            .map(|c| match c {
                'A' => 'T',
                'T' => 'A',
                'G' => 'C',
                'C' => 'G',
                _ => c,
            })
            .collect();
        sequence == complement
    }

    /// 二次構造形成確率を計算（ボルツマン分布）
    fn calculate_formation_probability_internal(&self, delta_g: f32) -> f32 {
        let rt = 0.001987 * self.conditions.temperature_k; // R = 1.987 cal/mol·K
        let exp_term = (-delta_g / rt).exp();
        exp_term / (1.0 + exp_term)
    }

    fn calculate_alignment_score(&self, seq1: &str, seq2: &str, offset: usize) -> Result<(f32, usize), ThermodynamicError> {
        let mut score = 0.0f32;
        let mut mismatches = 0usize;

        let start = offset;
        let end = (seq1.len()).min(seq2.len() + offset);

        for i in start..end {
            if i < seq1.len() && (i - offset) < seq2.len() {
                let base1 = seq1.chars().nth(i).unwrap();
                let base2 = seq2.chars().nth(i - offset).unwrap();

                if self.is_complementary(base1, base2) {
                    // Watson-Crick ペアのスコア
                    score -= 2.0; // 安定化
                } else {
                    // ミスマッチのペナルティ
                    score += 1.0; // 不安定化
                    mismatches += 1;
                }
            }
        }

        Ok((score, mismatches))
    }

    fn is_complementary(&self, base1: char, base2: char) -> bool {
        matches!((base1, base2), ('A', 'T') | ('T', 'A') | ('G', 'C') | ('C', 'G'))
    }

    fn calculate_hairpin_score(&self, stem_length: usize, loop_size: usize, loop_sequence: &str) -> Result<f32, ThermodynamicError> {
        // ヘアピンのエネルギー = ステムの安定化 + ループの不安定化
        let stem_stabilization = -2.0 * stem_length as f32; // 概算

        let loop_penalty = if let Some(params) = self.database.get_hairpin_loop(loop_size) {
            params.delta_g(310.15)
        } else {
            // デフォルトループペナルティ
            match loop_size {
                3 => 5.7,
                4 => 5.6,
                5 => 5.8,
                6 => 6.0,
                _ => 6.0 + 1.75 * ((loop_size as f32).ln()),
            }
        };

        Ok(stem_stabilization + loop_penalty)
    }
}

// 結果構造体

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfDimerAnalysis {
    pub max_score: f32,
    pub best_alignment_offset: Option<usize>,
    pub all_alignments: Vec<AlignmentResult>,
    pub is_problematic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HairpinAnalysis {
    pub min_score: f32,
    pub best_hairpin: Option<HairpinStructure>,
    pub all_hairpins: Vec<HairpinStructure>,
    pub is_problematic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeteroDimerAnalysis {
    pub max_score: f32,
    pub best_alignment_offset: Option<usize>,
    pub all_alignments: Vec<AlignmentResult>,
    pub is_problematic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlignmentResult {
    pub offset: usize,
    pub score: f32,
    pub mismatches: usize,
    pub length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HairpinStructure {
    pub start_pos: usize,
    pub stem_length: usize,
    pub loop_start: usize,
    pub loop_size: usize,
    pub stem5: String,
    pub loop_sequence: String,
    pub stem3: String,
    pub score: f32,
}

// エラー型

#[derive(Debug, thiserror::Error)]
pub enum ThermodynamicError {
    #[error("Sequence is too short (minimum 2 bases required)")]
    SequenceTooShort,

    #[error("Unknown dinucleotide: {0}")]
    UnknownDinucleotide(String),

    #[error("Unknown base: {0}")]
    UnknownBase(char),

    #[error("Invalid sequence: {0}")]
    InvalidSequence(String),

    #[error("Zero entropy encountered")]
    ZeroEntropy,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tm_calculation() {
        let calculator = ThermodynamicCalculator::new_santalucia_1998();

        // 簡単な配列でテスト
        let result = calculator.calculate_tm_nearest_neighbor("ATCGATCG");
        assert!(result.is_ok());

        let tm = result.unwrap();
        assert!(tm > 0.0 && tm < 100.0); // 現実的な範囲
    }

    #[test]
    fn test_delta_g_calculation() {
        let calculator = ThermodynamicCalculator::new_santalucia_1998();

        let result = calculator.calculate_delta_g("ATCG", 310.15);
        assert!(result.is_ok());
    }

    #[test]
    fn test_comprehensive_calculation() {
        let calculator = ThermodynamicCalculator::new_nndb_2024();

        let result = calculator.calculate_comprehensive("ATGCATGC");
        assert!(result.is_ok());

        let comp_result = result.unwrap();
        assert!(comp_result.melting_temperature > 0.0);
        assert!(!comp_result.corrections_applied.is_empty());
        assert!(comp_result.formation_probability >= 0.0 && comp_result.formation_probability <= 1.0);
    }

    #[test]
    fn test_molecular_crowding_effect() {
        let mut calculator = ThermodynamicCalculator::new_nndb_2024();

        // 分子クラウディングなし
        let result1 = calculator.calculate_comprehensive("ATGCATGC").unwrap();

        // 分子クラウディングあり
        let mut conditions = CalculationConditions::default();
        conditions.molecular_crowding = true;
        calculator.set_conditions(conditions);

        let result2 = calculator.calculate_comprehensive("ATGCATGC").unwrap();

        // クラウディング効果により安定化される
        assert!(result2.delta_h < result1.delta_h);
        assert!(result2.corrections_applied.contains(&"molecular_crowding".to_string()));
    }

    #[test]
    fn test_enhanced_self_dimer_analysis() {
        let calculator = ThermodynamicCalculator::new_nndb_2024();

        // セルフダイマーを形成しやすい配列
        let result = calculator.calculate_enhanced_self_dimer("AAAAAAAA");
        assert!(result.is_ok());

        let analysis = result.unwrap();
        assert!(!analysis.all_alignments.is_empty());
    }

    #[test]
    fn test_enhanced_hairpin_analysis() {
        let calculator = ThermodynamicCalculator::new_nndb_2024();

        // ヘアピンを形成しやすい配列
        let result = calculator.calculate_enhanced_hairpin("GCATGCAAAGCATGC");
        assert!(result.is_ok());

        let analysis = result.unwrap();
        // ヘアピンが見つかることを期待
        assert!(analysis.best_hairpin.is_some());
    }

    #[test]
    fn test_palindrome_detection() {
        let calculator = ThermodynamicCalculator::new_nndb_2024();

        // 回文配列
        assert!(calculator.is_palindrome("ATCGAT"));
        assert!(calculator.is_palindrome("GAATTC"));

        // 非回文配列
        assert!(!calculator.is_palindrome("ATGCAT"));
        assert!(!calculator.is_palindrome("ACGTACGT"));
    }

    #[test]
    fn test_contribution_breakdown() {
        let calculator = ThermodynamicCalculator::new_nndb_2024();

        let result = calculator.calculate_comprehensive("ATGCATGC").unwrap();
        let breakdown = &result.contribution_breakdown;

        // 各寄与が適切に分離されている
        assert!(breakdown.nearest_neighbor != 0.0);
        assert!(breakdown.terminal_effects != 0.0);

        // 全体の寄与の合計は概ねdelta_hと一致する
        let total_contribution = breakdown.nearest_neighbor + breakdown.terminal_effects +
                                 breakdown.mismatch_penalty + breakdown.loop_structures;
        assert!((total_contribution - result.delta_h).abs() < 0.1); // 許容誤差内
    }

    #[test]
    fn test_reverse_complement() {
        let calculator = ThermodynamicCalculator::new_santalucia_1998();

        let result = calculator.reverse_complement("ATCG");
        assert_eq!(result.unwrap(), "CGAT");

        let result = calculator.reverse_complement("GCTA");
        assert_eq!(result.unwrap(), "TAGC");
    }

    #[test]
    fn test_error_handling() {
        let calculator = ThermodynamicCalculator::new_santalucia_1998();

        // 短すぎる配列
        let result = calculator.calculate_tm_nearest_neighbor("A");
        assert!(matches!(result, Err(ThermodynamicError::SequenceTooShort)));

        let result = calculator.calculate_comprehensive("A");
        assert!(matches!(result, Err(ThermodynamicError::SequenceTooShort)));

        // 不正な塩基
        let result = calculator.reverse_complement("ATCGX");
        assert!(matches!(result, Err(ThermodynamicError::UnknownBase('X'))));
    }

    #[test]
    fn test_nndb_2024_vs_santalucia_1998() {
        let calc_nndb = ThermodynamicCalculator::new_nndb_2024();
        let calc_santalucia = ThermodynamicCalculator::new_santalucia_1998();

        let sequence = "ATGCATGC";

        let tm_nndb = calc_nndb.calculate_tm_nearest_neighbor(sequence).unwrap();
        let tm_santalucia = calc_santalucia.calculate_tm_nearest_neighbor(sequence).unwrap();

        // NNDB 2024は高精度パラメータのため、わずかな違いがある
        assert!((tm_nndb - tm_santalucia).abs() < 5.0); // 5°C以内の差
    }
}