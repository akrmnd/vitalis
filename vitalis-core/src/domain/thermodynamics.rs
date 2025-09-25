use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 熱力学パラメータ（ΔH, ΔS）
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ThermodynamicParams {
    /// エンタルピー変化 (kcal/mol)
    pub delta_h: f32,
    /// エントロピー変化 (cal/mol·K)
    pub delta_s: f32,
}

impl ThermodynamicParams {
    pub fn new(delta_h: f32, delta_s: f32) -> Self {
        Self { delta_h, delta_s }
    }

    /// ギブス自由エネルギー変化を計算 (kcal/mol)
    /// ΔG = ΔH - T·ΔS
    pub fn delta_g(&self, temperature_k: f32) -> f32 {
        self.delta_h - temperature_k * (self.delta_s / 1000.0) // ΔSをkcal/mol·Kに変換
    }
}

/// DNA二重鎖形成の熱力学パラメータセット
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNAThermodynamicsDatabase {
    /// Watson-Crick塩基対の最近接パラメータ
    /// キー: "AA/TT", "AT/AT", "TA/TA", "CA/TG", "GT/AC", "CT/AG", "GA/TC", "CG/CG", "GC/GC", "GG/CC"
    pub nearest_neighbor: HashMap<String, ThermodynamicParams>,

    /// 末端効果（Initiation parameters）
    /// キー: "A", "T", "G", "C"
    pub initiation: HashMap<String, ThermodynamicParams>,

    /// ミスマッチペアの熱力学パラメータ
    /// キー: "AA/TT", "AC/TG", "AG/TC", など
    pub mismatches: HashMap<String, ThermodynamicParams>,

    /// 対称的な内部ループの熱力学パラメータ
    /// キー: ループサイズ（文字列として）"1", "2", "3", など
    pub symmetric_internal_loops: HashMap<String, ThermodynamicParams>,

    /// 非対称的な内部ループの熱力学パラメータ
    /// キー: "1x2", "1x3", "2x3", など（小x大の形式）
    pub asymmetric_internal_loops: HashMap<String, ThermodynamicParams>,

    /// バルジループの熱力学パラメータ
    /// キー: ループサイズ（文字列として）"1", "2", "3", など
    pub bulge_loops: HashMap<String, ThermodynamicParams>,

    /// ヘアピンループの熱力学パラメータ
    /// キー: ループサイズ（文字列として）"3", "4", "5", など
    pub hairpin_loops: HashMap<String, ThermodynamicParams>,

    /// 特殊配列（TLOOP、CLOOP等）の熱力学パラメータ
    pub special_sequences: HashMap<String, ThermodynamicParams>,

    /// 塩濃度補正パラメータ
    pub salt_correction: SaltCorrectionParams,
}

/// 塩濃度補正のパラメータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaltCorrectionParams {
    /// Na+濃度 (M)
    pub sodium_concentration: f32,
    /// Mg2+濃度 (M)
    pub magnesium_concentration: f32,
    /// K+濃度 (M)
    pub potassium_concentration: f32,
    /// その他のモノ価カチオン濃度 (M)
    pub other_monovalent: f32,
}

impl Default for SaltCorrectionParams {
    fn default() -> Self {
        Self {
            sodium_concentration: 0.05,     // 50 mM
            magnesium_concentration: 0.002, // 2 mM
            potassium_concentration: 0.0,
            other_monovalent: 0.0,
        }
    }
}

impl DNAThermodynamicsDatabase {
    /// NNDB 2024パラメータで初期化されたデータベースを作成
    pub fn nndb_2024() -> Self {
        let mut db = Self {
            nearest_neighbor: HashMap::new(),
            initiation: HashMap::new(),
            mismatches: HashMap::new(),
            symmetric_internal_loops: HashMap::new(),
            asymmetric_internal_loops: HashMap::new(),
            bulge_loops: HashMap::new(),
            hairpin_loops: HashMap::new(),
            special_sequences: HashMap::new(),
            salt_correction: SaltCorrectionParams::default(),
        };

        // 基本の最近接パラメータを設定（NNDB 2024ベース）
        db.load_nearest_neighbor_params();
        db.load_initiation_params();
        db.load_mismatch_params();
        db.load_loop_params();
        db.load_special_sequences();

        db
    }

    /// SantaLucia 1998パラメータで初期化されたデータベースを作成（後方互換性）
    pub fn santalucia_1998() -> Self {
        let mut db = Self {
            nearest_neighbor: HashMap::new(),
            initiation: HashMap::new(),
            mismatches: HashMap::new(),
            symmetric_internal_loops: HashMap::new(),
            asymmetric_internal_loops: HashMap::new(),
            bulge_loops: HashMap::new(),
            hairpin_loops: HashMap::new(),
            special_sequences: HashMap::new(),
            salt_correction: SaltCorrectionParams::default(),
        };

        // SantaLucia 1998パラメータを設定
        db.load_santalucia_1998_params();

        db
    }

    /// 最近接パラメータを取得
    pub fn get_nearest_neighbor(&self, sequence: &str) -> Option<&ThermodynamicParams> {
        self.nearest_neighbor.get(sequence)
    }

    /// 末端パラメータを取得
    pub fn get_initiation(&self, base: &str) -> Option<&ThermodynamicParams> {
        self.initiation.get(base)
    }

    /// ミスマッチパラメータを取得
    pub fn get_mismatch(&self, sequence: &str) -> Option<&ThermodynamicParams> {
        self.mismatches.get(sequence)
    }

    /// 対称内部ループパラメータを取得
    pub fn get_symmetric_loop(&self, size: usize) -> Option<&ThermodynamicParams> {
        self.symmetric_internal_loops.get(&size.to_string())
    }

    /// 非対称内部ループパラメータを取得
    pub fn get_asymmetric_loop(&self, size1: usize, size2: usize) -> Option<&ThermodynamicParams> {
        let key = if size1 <= size2 {
            format!("{}x{}", size1, size2)
        } else {
            format!("{}x{}", size2, size1)
        };
        self.asymmetric_internal_loops.get(&key)
    }

    /// バルジループパラメータを取得
    pub fn get_bulge_loop(&self, size: usize) -> Option<&ThermodynamicParams> {
        self.bulge_loops.get(&size.to_string())
    }

    /// ヘアピンループパラメータを取得
    pub fn get_hairpin_loop(&self, size: usize) -> Option<&ThermodynamicParams> {
        self.hairpin_loops.get(&size.to_string())
    }

    /// 特殊配列パラメータを取得
    pub fn get_special_sequence(&self, sequence: &str) -> Option<&ThermodynamicParams> {
        self.special_sequences.get(sequence)
    }
}

impl DNAThermodynamicsDatabase {
    /// NNDB 2024最近接パラメータを読み込み
    fn load_nearest_neighbor_params(&mut self) {
        // Watson-Crick塩基対の最近接パラメータ
        // SantaLucia 1998 unified parameters (still widely used as standard)
        // Updated with latest NNDB 2024 structure for future expansion
        self.nearest_neighbor
            .insert("AA/TT".to_string(), ThermodynamicParams::new(-7.9, -22.2));
        self.nearest_neighbor
            .insert("AT/TA".to_string(), ThermodynamicParams::new(-7.2, -20.4));
        self.nearest_neighbor
            .insert("TA/AT".to_string(), ThermodynamicParams::new(-7.2, -21.3));
        self.nearest_neighbor
            .insert("CA/GT".to_string(), ThermodynamicParams::new(-8.5, -22.7));
        self.nearest_neighbor
            .insert("GT/CA".to_string(), ThermodynamicParams::new(-8.4, -22.4));
        self.nearest_neighbor
            .insert("CT/GA".to_string(), ThermodynamicParams::new(-7.8, -21.0));
        self.nearest_neighbor
            .insert("GA/CT".to_string(), ThermodynamicParams::new(-8.2, -22.2));
        self.nearest_neighbor
            .insert("CG/GC".to_string(), ThermodynamicParams::new(-10.6, -27.2));
        self.nearest_neighbor
            .insert("GC/CG".to_string(), ThermodynamicParams::new(-9.8, -24.4));
        self.nearest_neighbor
            .insert("GG/CC".to_string(), ThermodynamicParams::new(-8.0, -19.9));

        // Additional missing complementary pairs for complete coverage
        self.nearest_neighbor
            .insert("TG/AC".to_string(), ThermodynamicParams::new(-8.4, -22.4)); // Same as GT/CA
        self.nearest_neighbor
            .insert("AC/TG".to_string(), ThermodynamicParams::new(-8.5, -22.7)); // Same as CA/GT
        self.nearest_neighbor
            .insert("AG/TC".to_string(), ThermodynamicParams::new(-7.8, -21.0)); // Same as CT/GA
        self.nearest_neighbor
            .insert("TC/AG".to_string(), ThermodynamicParams::new(-8.2, -22.2));
        // Same as GA/CT
    }

    /// 末端効果パラメータを読み込み
    fn load_initiation_params(&mut self) {
        // 末端効果（NNDB 2024ベース）
        // TODO: 実際のNNDB 2024データに置き換え予定
        self.initiation
            .insert("A".to_string(), ThermodynamicParams::new(2.3, 4.1));
        self.initiation
            .insert("T".to_string(), ThermodynamicParams::new(2.3, 4.1));
        self.initiation
            .insert("G".to_string(), ThermodynamicParams::new(0.1, -2.8));
        self.initiation
            .insert("C".to_string(), ThermodynamicParams::new(0.1, -2.8));
    }

    /// ミスマッチパラメータを読み込み
    fn load_mismatch_params(&mut self) {
        // ミスマッチペアのパラメータ（Allawi & SantaLucia 1997-1998拡張）
        // G·T ミスマッチ（Allawi & SantaLucia 1997）
        self.mismatches
            .insert("GT/TG".to_string(), ThermodynamicParams::new(-4.1, -9.5));
        self.mismatches
            .insert("GG/TT".to_string(), ThermodynamicParams::new(-2.8, -5.3));
        self.mismatches
            .insert("TG/GT".to_string(), ThermodynamicParams::new(-4.1, -9.5));
        self.mismatches
            .insert("TT/GG".to_string(), ThermodynamicParams::new(-2.8, -5.3));

        // G·A ミスマッチ（Allawi & SantaLucia 1998）
        self.mismatches
            .insert("GA/AG".to_string(), ThermodynamicParams::new(-0.6, -1.3));
        self.mismatches
            .insert("GG/AA".to_string(), ThermodynamicParams::new(-0.6, -1.3));
        self.mismatches
            .insert("AG/GA".to_string(), ThermodynamicParams::new(-0.6, -1.3));
        self.mismatches
            .insert("AA/GG".to_string(), ThermodynamicParams::new(-0.6, -1.3));

        // A·A, C·C, G·G, T·T ミスマッチ（SantaLucia et al. 1999）
        self.mismatches
            .insert("AA/AA".to_string(), ThermodynamicParams::new(1.2, 1.7));
        self.mismatches
            .insert("CC/CC".to_string(), ThermodynamicParams::new(0.6, -0.6));
        self.mismatches
            .insert("GG/GG".to_string(), ThermodynamicParams::new(3.1, 5.8));
        self.mismatches
            .insert("TT/TT".to_string(), ThermodynamicParams::new(1.2, 1.7));

        // C·T ミスマッチ（基本的なもの）
        self.mismatches
            .insert("CT/TC".to_string(), ThermodynamicParams::new(-0.1, -1.5));
        self.mismatches
            .insert("CC/TT".to_string(), ThermodynamicParams::new(-0.1, -1.5));

        // 追加のミスマッチパラメータ（文献値に基づく）
        self.mismatches
            .insert("AC/CA".to_string(), ThermodynamicParams::new(2.3, 5.3));
        self.mismatches
            .insert("AT/TA".to_string(), ThermodynamicParams::new(1.2, 1.7));
    }

    /// ループパラメータを読み込み
    fn load_loop_params(&mut self) {
        // 対称内部ループ
        self.symmetric_internal_loops
            .insert("1".to_string(), ThermodynamicParams::new(0.0, -9.3));
        self.symmetric_internal_loops
            .insert("2".to_string(), ThermodynamicParams::new(0.0, -10.4));
        self.symmetric_internal_loops
            .insert("3".to_string(), ThermodynamicParams::new(0.0, -12.6));

        // 非対称内部ループ
        self.asymmetric_internal_loops
            .insert("1x2".to_string(), ThermodynamicParams::new(1.6, 0.0));
        self.asymmetric_internal_loops
            .insert("1x3".to_string(), ThermodynamicParams::new(1.9, 0.0));

        // バルジループ
        self.bulge_loops
            .insert("1".to_string(), ThermodynamicParams::new(3.8, 2.8));
        self.bulge_loops
            .insert("2".to_string(), ThermodynamicParams::new(2.8, -0.1));
        self.bulge_loops
            .insert("3".to_string(), ThermodynamicParams::new(3.2, 0.5));

        // ヘアピンループ
        self.hairpin_loops
            .insert("3".to_string(), ThermodynamicParams::new(5.7, 9.4));
        self.hairpin_loops
            .insert("4".to_string(), ThermodynamicParams::new(5.6, 9.5));
        self.hairpin_loops
            .insert("5".to_string(), ThermodynamicParams::new(5.8, 10.2));
    }

    /// 特殊配列パラメータを読み込み
    fn load_special_sequences(&mut self) {
        // TLOOP、CLOOP等の特殊配列
        self.special_sequences
            .insert("TLOOP".to_string(), ThermodynamicParams::new(-1.0, -4.0));
        self.special_sequences
            .insert("CLOOP".to_string(), ThermodynamicParams::new(-0.5, -2.0));
    }

    /// SantaLucia 1998パラメータを読み込み（後方互換性）
    fn load_santalucia_1998_params(&mut self) {
        // 既存の実装と同じパラメータ（Watson-Crick complement format）
        self.nearest_neighbor
            .insert("AA/TT".to_string(), ThermodynamicParams::new(-7.9, -22.2));
        self.nearest_neighbor
            .insert("AT/TA".to_string(), ThermodynamicParams::new(-7.2, -20.4));
        self.nearest_neighbor
            .insert("TA/AT".to_string(), ThermodynamicParams::new(-7.2, -21.3));
        self.nearest_neighbor
            .insert("CA/GT".to_string(), ThermodynamicParams::new(-8.5, -22.7));
        self.nearest_neighbor
            .insert("GT/CA".to_string(), ThermodynamicParams::new(-8.4, -22.4));
        self.nearest_neighbor
            .insert("CT/GA".to_string(), ThermodynamicParams::new(-7.8, -21.0));
        self.nearest_neighbor
            .insert("GA/CT".to_string(), ThermodynamicParams::new(-8.2, -22.2));
        self.nearest_neighbor
            .insert("CG/GC".to_string(), ThermodynamicParams::new(-10.6, -27.2));
        self.nearest_neighbor
            .insert("GC/CG".to_string(), ThermodynamicParams::new(-9.8, -24.4));
        self.nearest_neighbor
            .insert("GG/CC".to_string(), ThermodynamicParams::new(-8.0, -19.9));

        // Additional missing complementary pairs for complete coverage
        self.nearest_neighbor
            .insert("TG/AC".to_string(), ThermodynamicParams::new(-8.4, -22.4)); // Same as GT/CA
        self.nearest_neighbor
            .insert("AC/TG".to_string(), ThermodynamicParams::new(-8.5, -22.7)); // Same as CA/GT
        self.nearest_neighbor
            .insert("AG/TC".to_string(), ThermodynamicParams::new(-7.8, -21.0)); // Same as CT/GA
        self.nearest_neighbor
            .insert("TC/AG".to_string(), ThermodynamicParams::new(-8.2, -22.2)); // Same as GA/CT

        // 基本的な末端効果
        self.initiation
            .insert("A".to_string(), ThermodynamicParams::new(2.3, 4.1));
        self.initiation
            .insert("T".to_string(), ThermodynamicParams::new(2.3, 4.1));
        self.initiation
            .insert("G".to_string(), ThermodynamicParams::new(0.1, -2.8));
        self.initiation
            .insert("C".to_string(), ThermodynamicParams::new(0.1, -2.8));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermodynamic_params() {
        let params = ThermodynamicParams::new(-7.9, -22.2);
        assert_eq!(params.delta_h, -7.9);
        assert_eq!(params.delta_s, -22.2);

        // ギブス自由エネルギーのテスト（37°C = 310.15K）
        let delta_g = params.delta_g(310.15);
        assert!((delta_g - (-0.99)).abs() < 0.1); // 概算値
    }

    #[test]
    fn test_nndb_2024_database() {
        let db = DNAThermodynamicsDatabase::nndb_2024();

        // 基本的なパラメータが存在することを確認
        assert!(db.get_nearest_neighbor("AA/TT").is_some());
        assert!(db.get_initiation("A").is_some());
        assert!(db.get_hairpin_loop(4).is_some());
    }

    #[test]
    fn test_santalucia_1998_database() {
        let db = DNAThermodynamicsDatabase::santalucia_1998();

        // SantaLucia 1998パラメータが正しく設定されているか確認
        let aa_tt = db.get_nearest_neighbor("AA/TT").unwrap();
        assert_eq!(aa_tt.delta_h, -7.9);
        assert_eq!(aa_tt.delta_s, -22.2);
    }

    #[test]
    fn test_loop_parameter_access() {
        let db = DNAThermodynamicsDatabase::nndb_2024();

        // 対称ループアクセス
        assert!(db.get_symmetric_loop(2).is_some());

        // 非対称ループアクセス（順序に依存しない）
        let loop1 = db.get_asymmetric_loop(1, 2);
        let loop2 = db.get_asymmetric_loop(2, 1);
        assert!(loop1.is_some());
        assert_eq!(loop1, loop2); // 同じパラメータを返すべき
    }
}
