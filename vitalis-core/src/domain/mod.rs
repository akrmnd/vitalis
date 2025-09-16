// Domain layer - ビジネスロジックとエンティティ
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// ドメインエンティティ: 配列情報
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sequence {
    pub id: String,
    pub name: String,
    pub sequence: String,
    pub topology: Topology,
}

/// ドメインエンティティ: 配列メタデータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceMetadata {
    pub id: String,
    pub name: String,
    pub length: usize,
    pub topology: Topology,
    pub file_path: Option<PathBuf>,
}

/// 配列の形状
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Topology {
    Linear,
    Circular,
}

/// 範囲指定
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

/// 詳細統計情報（ドメインエンティティ）
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

/// 塩基カウント
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

/// ウィンドウ統計
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowStats {
    pub position: usize,
    pub window_size: usize,
    pub gc_percent: f64,
    pub entropy: f64,
}

// ドメインレイヤーでのRepositoryトレイト定義（依存性の逆転）
pub trait SequenceRepository {
    type Error: std::error::Error + Send + Sync + 'static;

    fn store_sequence(&mut self, sequence: Sequence) -> Result<String, Self::Error>;
    fn store_sequence_from_file(
        &mut self,
        file_path: &std::path::Path,
        format: &str,
    ) -> Result<String, Self::Error>;
    fn get_metadata(&self, seq_id: &str) -> Option<SequenceMetadata>;
    fn get_sequence(&self, seq_id: &str) -> Result<String, Self::Error>;
    fn get_window(&self, seq_id: &str, start: usize, end: usize) -> Result<String, Self::Error>;
}

// ドメインレイヤーでのParserトレイト定義
pub trait SequenceParser {
    type Error: std::error::Error + Send + Sync + 'static;

    fn parse(&self, content: &str) -> Result<Vec<Sequence>, Self::Error>;
}

// ドメインレイヤーでのStatsサービストレイト定義
pub trait StatsService {
    fn calculate_detailed_stats(&self, sequence: &str) -> DetailedStats;
    fn calculate_window_stats(
        &self,
        sequence: &str,
        window_size: usize,
        step: usize,
    ) -> Vec<WindowStats>;
}

// ドメインサービス: 配列解析
pub struct SequenceAnalysisService<R, S>
where
    R: SequenceRepository,
    S: StatsService,
{
    repository: R,
    stats_service: S,
}

impl<R, S> SequenceAnalysisService<R, S>
where
    R: SequenceRepository,
    S: StatsService,
{
    pub fn new(repository: R, stats_service: S) -> Self {
        Self {
            repository,
            stats_service,
        }
    }

    pub fn analyze_sequence(&mut self, seq_id: &str) -> Result<DetailedStats, R::Error> {
        let sequence = self.repository.get_sequence(seq_id)?;
        Ok(self.stats_service.calculate_detailed_stats(&sequence))
    }

    pub fn analyze_window(
        &mut self,
        seq_id: &str,
        window_size: usize,
        step: usize,
    ) -> Result<Vec<WindowStats>, R::Error> {
        let sequence = self.repository.get_sequence(seq_id)?;
        Ok(self
            .stats_service
            .calculate_window_stats(&sequence, window_size, step))
    }

    pub fn get_repository_mut(&mut self) -> &mut R {
        &mut self.repository
    }

    pub fn get_repository(&self) -> &R {
        &self.repository
    }
}
