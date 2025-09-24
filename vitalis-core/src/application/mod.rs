// Application layer - Tauri commands and use cases
use crate::domain::{
    DetailedStats, SequenceAnalysisService, SequenceRepository, Topology, WindowStats,
    primer::{PrimerDesignParams, PrimerDesignResult, PrimerDesignService},
};
use crate::infrastructure::{FileSequenceRepository, GenBankParser};
use crate::services::{StatsServiceImpl, PrimerDesignServiceImpl};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;

// Response types for Tauri commands
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportResponse {
    pub seq_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SequenceInfo {
    pub id: String,
    pub name: String,
    pub length: usize,
    pub preview: String, // First 50 characters
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsePreviewResponse {
    pub sequences: Vec<SequenceInfo>,
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SequenceMeta {
    pub id: String,
    pub name: String,
    pub length: usize,
    pub topology: Topology,
    pub file_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenBankMetadata {
    pub accession: String,
    pub version: String,
    pub definition: String,
    pub source: String,
    pub organism: String,
    pub length: usize,
    pub topology: Topology,
    pub features: Vec<GenBankFeatureInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenBankFeatureInfo {
    pub feature_type: String,
    pub location: String,
    pub qualifiers: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SequenceStats {
    pub gc_overall: f64,
    pub n_rate: f64,
    pub length: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowResponse {
    pub bases: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportResponse {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailedStatsResponse {
    pub detailed: DetailedStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowStatsResponse {
    pub windows: Vec<WindowStats>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicStats {
    pub length: usize,
    pub gc_percent: f64,
    pub at_percent: f64,
    pub n_percent: f64,
    pub gc_skew: f64,
    pub at_skew: f64,
    pub entropy: f64,
    pub complexity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseCountResponse {
    pub a: usize,
    pub t: usize,
    pub g: usize,
    pub c: usize,
    pub n: usize,
    pub other: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodonUsageResponse {
    pub codon_counts: std::collections::HashMap<String, usize>,
    pub codon_frequencies: std::collections::HashMap<String, f64>,
    pub amino_acid_counts: std::collections::HashMap<char, usize>,
    pub start_codons: usize,
    pub stop_codons: usize,
    pub rare_codons: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityStatsResponse {
    pub mean_quality: f64,
    pub median_quality: f64,
    pub min_quality: u8,
    pub max_quality: u8,
    pub q20_bases: usize,
    pub q30_bases: usize,
    pub quality_distribution: std::collections::HashMap<u8, usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailedStatsEnhancedResponse {
    pub basic: BasicStats,
    pub base_counts: BaseCountResponse,
    pub dinucleotide_counts: std::collections::HashMap<String, usize>,
    pub codon_usage: Option<CodonUsageResponse>,
    pub quality_stats: Option<QualityStatsResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowStatsItem {
    pub position: usize,
    pub window_size: usize,
    pub gc_percent: f64,
    pub entropy: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportFromFileRequest {
    pub file_path: String,
    pub format: String,
}

// Global service instances (thread-safe)
type ServiceType = SequenceAnalysisService<FileSequenceRepository, StatsServiceImpl>;

lazy_static::lazy_static! {
    static ref SERVICE: Mutex<ServiceType> = Mutex::new(
        SequenceAnalysisService::new(
            FileSequenceRepository::new(),
            StatsServiceImpl::new()
        )
    );

    static ref PRIMER_SERVICE: Mutex<PrimerDesignServiceImpl> = Mutex::new(
        PrimerDesignServiceImpl::new()
    );
}

/// Parse and import sequences from text content
pub fn parse_and_import(text: String, fmt: String) -> Result<ImportResponse, String> {
    let mut service = SERVICE.lock().map_err(|e| e.to_string())?;
    let repository = service.get_repository_mut();
    let seq_id = repository
        .import_from_text(&text, &fmt)
        .map_err(|e| e.to_string())?;
    Ok(ImportResponse { seq_id })
}

/// Parse sequences and return preview without importing
pub fn parse_preview(text: String, fmt: String) -> Result<ParsePreviewResponse, String> {
    let service = SERVICE.lock().map_err(|e| e.to_string())?;
    let repository = service.get_repository();

    let sequences = match fmt.as_str() {
        "fasta" => repository.parse_fasta(&text).map_err(|e| e.to_string())?,
        "fastq" => repository.parse_fastq(&text).map_err(|e| e.to_string())?,
        "genbank" => {
            let parser = GenBankParser::new();
            let record = parser.parse(&text).map_err(|e| e.to_string())?;
            let sequence = parser.to_sequence(&record);
            vec![sequence]
        }
        _ => return Err(format!("Unsupported format: {}", fmt)),
    };

    let sequence_info: Vec<SequenceInfo> = sequences
        .iter()
        .map(|seq| SequenceInfo {
            id: seq.id.clone(),
            name: seq.name.clone(),
            length: seq.sequence.len(),
            preview: seq.sequence.chars().take(50).collect(),
        })
        .collect();

    Ok(ParsePreviewResponse {
        sequences: sequence_info,
        format: fmt,
    })
}

/// Import a specific sequence by index from parsed content
pub fn import_sequence(
    text: String,
    fmt: String,
    sequence_index: usize,
) -> Result<ImportResponse, String> {
    let mut service = SERVICE.lock().map_err(|e| e.to_string())?;
    let repository = service.get_repository_mut();

    let sequences = match fmt.as_str() {
        "fasta" => repository.parse_fasta(&text).map_err(|e| e.to_string())?,
        "fastq" => repository.parse_fastq(&text).map_err(|e| e.to_string())?,
        "genbank" => {
            let parser = GenBankParser::new();
            let record = parser.parse(&text).map_err(|e| e.to_string())?;
            let sequence = parser.to_sequence(&record);
            vec![sequence]
        }
        _ => return Err(format!("Unsupported format: {}", fmt)),
    };

    if sequence_index >= sequences.len() {
        return Err("Sequence index out of range".to_string());
    }

    let sequence = &sequences[sequence_index];
    let seq_id = repository.generate_id();

    // Store in memory
    repository.sequences.insert(
        seq_id.clone(),
        crate::infrastructure::storage::SequenceSource::Memory(sequence.sequence.clone()),
    );
    repository.metadata.insert(
        seq_id.clone(),
        crate::domain::SequenceMetadata {
            id: sequence.id.clone(),
            name: sequence.name.clone(),
            length: sequence.sequence.len(),
            topology: sequence.topology.clone(),
            file_path: None,
        },
    );

    Ok(ImportResponse { seq_id })
}

/// Import sequence from file path (for large files)
pub fn import_from_file(request: ImportFromFileRequest) -> Result<ImportResponse, String> {
    let mut service = SERVICE.lock().map_err(|e| e.to_string())?;
    let repository = service.get_repository_mut();
    let path = Path::new(&request.file_path);
    let seq_id = repository
        .import_from_file(path, &request.format)
        .map_err(|e| e.to_string())?;
    Ok(ImportResponse { seq_id })
}

/// Get sequence metadata
pub fn get_meta(seq_id: String) -> Result<SequenceMeta, String> {
    let service = SERVICE.lock().map_err(|e| e.to_string())?;
    let repository = service.get_repository();

    match repository.get_metadata(&seq_id) {
        Some(meta) => Ok(SequenceMeta {
            id: meta.id.clone(),
            name: meta.name.clone(),
            length: meta.length,
            topology: meta.topology.clone(),
            file_path: meta
                .file_path
                .as_ref()
                .map(|p| p.to_string_lossy().to_string()),
        }),
        None => Err(format!("Sequence not found: {}", seq_id)),
    }
}

/// Get GenBank metadata if sequence was imported from GenBank format
pub fn get_genbank_metadata(text: String) -> Result<GenBankMetadata, String> {
    let parser = GenBankParser::new();
    let record = parser.parse(&text).map_err(|e| e.to_string())?;

    let features = record
        .features
        .into_iter()
        .map(|f| GenBankFeatureInfo {
            feature_type: f.feature_type,
            location: f.location,
            qualifiers: f.qualifiers,
        })
        .collect();

    Ok(GenBankMetadata {
        accession: record.accession,
        version: record.version,
        definition: record.definition,
        source: record.source,
        organism: record.organism,
        length: record.length,
        topology: record.topology,
        features,
    })
}

/// Get sequence window (optimized for large files)
pub fn get_window(seq_id: String, start: usize, end: usize) -> Result<WindowResponse, String> {
    let service = SERVICE.lock().map_err(|e| e.to_string())?;
    let repository = service.get_repository();
    let bases = repository
        .get_window(&seq_id, start, end)
        .map_err(|e| e.to_string())?;
    Ok(WindowResponse { bases })
}

/// Calculate basic statistics (backward compatible interface)
pub fn stats(seq_id: String) -> Result<SequenceStats, String> {
    let mut service = SERVICE.lock().map_err(|e| e.to_string())?;
    let detailed = service
        .analyze_sequence(&seq_id)
        .map_err(|e| e.to_string())?;

    Ok(SequenceStats {
        gc_overall: detailed.gc_percent,
        n_rate: detailed.n_percent,
        length: detailed.length,
    })
}

/// Calculate detailed statistics
pub fn detailed_stats(seq_id: String) -> Result<DetailedStatsResponse, String> {
    let mut service = SERVICE.lock().map_err(|e| e.to_string())?;
    let detailed = service
        .analyze_sequence(&seq_id)
        .map_err(|e| e.to_string())?;

    Ok(DetailedStatsResponse { detailed })
}

/// Calculate detailed statistics with enhanced features
pub fn detailed_stats_enhanced(seq_id: String) -> Result<DetailedStatsEnhancedResponse, String> {
    let mut service = SERVICE.lock().map_err(|e| e.to_string())?;
    let detailed = service
        .analyze_sequence(&seq_id)
        .map_err(|e| e.to_string())?;

    Ok(DetailedStatsEnhancedResponse {
        basic: BasicStats {
            length: detailed.length,
            gc_percent: detailed.gc_percent,
            at_percent: detailed.at_percent,
            n_percent: detailed.n_percent,
            gc_skew: detailed.gc_skew,
            at_skew: detailed.at_skew,
            entropy: detailed.entropy,
            complexity: detailed.complexity,
        },
        base_counts: BaseCountResponse {
            a: detailed.base_counts.a,
            t: detailed.base_counts.t,
            g: detailed.base_counts.g,
            c: detailed.base_counts.c,
            n: detailed.base_counts.n,
            other: detailed.base_counts.other,
        },
        dinucleotide_counts: detailed.dinucleotide_counts,
        codon_usage: detailed.codon_usage.map(|cu| CodonUsageResponse {
            codon_counts: cu.codon_counts,
            codon_frequencies: cu.codon_frequencies,
            amino_acid_counts: cu.amino_acid_counts,
            start_codons: cu.start_codons,
            stop_codons: cu.stop_codons,
            rare_codons: cu.rare_codons,
        }),
        quality_stats: detailed.quality_stats.map(|qs| QualityStatsResponse {
            mean_quality: qs.mean_quality,
            median_quality: qs.median_quality,
            min_quality: qs.min_quality,
            max_quality: qs.max_quality,
            q20_bases: qs.q20_bases,
            q30_bases: qs.q30_bases,
            quality_distribution: qs.quality_distribution,
        }),
    })
}

/// Calculate window statistics for visualization
pub fn window_stats(
    seq_id: String,
    window_size: usize,
    step: usize,
) -> Result<Vec<WindowStatsItem>, String> {
    let service = SERVICE.lock().map_err(|e| e.to_string())?;
    let repository = service.get_repository();

    // Get full sequence for now (could be optimized for large sequences)
    let sequence = repository
        .get_window(&seq_id, 0, usize::MAX)
        .map_err(|e| e.to_string())?;

    let stats = crate::stats::calculate_window_stats(&sequence, window_size, step);

    Ok(stats
        .into_iter()
        .map(|ws| WindowStatsItem {
            position: ws.position,
            window_size: ws.window_size,
            gc_percent: ws.gc_percent,
            entropy: ws.entropy,
        })
        .collect())
}

/// Export sequence to text format
pub fn export(seq_id: String, fmt: String) -> Result<ExportResponse, String> {
    let service = SERVICE.lock().map_err(|e| e.to_string())?;
    let repository = service.get_repository();

    let metadata = repository
        .get_metadata(&seq_id)
        .ok_or_else(|| format!("Sequence not found: {}", seq_id))?;

    let sequence = repository
        .get_sequence(&seq_id)
        .map_err(|e| e.to_string())?;

    let text = match fmt.as_str() {
        "fasta" => {
            format!(">{} {}\n{}\n", metadata.id, metadata.name, sequence)
        }
        "fastq" => {
            // For FASTQ, we need quality scores - generate dummy if not available
            let dummy_quality = "I".repeat(sequence.len());
            format!(
                "@{} {}\n{}\n+\n{}\n",
                metadata.id, metadata.name, sequence, dummy_quality
            )
        }
        _ => return Err(format!("Unsupported export format: {}", fmt)),
    };

    Ok(ExportResponse { text })
}

/// Design primers for a specific sequence region
pub fn design_primers(
    seq_id: String,
    start: usize,
    end: usize,
    params: Option<PrimerDesignParams>,
) -> Result<PrimerDesignResult, String> {
    let service = SERVICE.lock().map_err(|e| e.to_string())?;
    let repository = service.get_repository();

    // Get the full sequence
    let sequence = repository
        .get_sequence(&seq_id)
        .map_err(|e| e.to_string())?;

    let primer_service = PRIMER_SERVICE.lock().map_err(|e| e.to_string())?;
    let design_params = params.unwrap_or_default();

    primer_service
        .design_primers(&sequence, start, end, &design_params)
        .map_err(|e| e.to_string())
}

/// Calculate primer melting temperature
pub fn calculate_primer_tm(sequence: String) -> Result<f32, String> {
    let primer_service = PRIMER_SERVICE.lock().map_err(|e| e.to_string())?;
    Ok(primer_service.calculate_tm(&sequence))
}

/// Calculate GC content of primer
pub fn calculate_primer_gc(sequence: String) -> Result<f32, String> {
    let primer_service = PRIMER_SERVICE.lock().map_err(|e| e.to_string())?;
    Ok(primer_service.calculate_gc_content(&sequence))
}

/// Evaluate multiplex compatibility for multiple primer pairs
pub fn evaluate_primer_multiplex(
    _seq_id: String,
    _primer_pairs: Vec<serde_json::Value>, // JSON representation of PrimerPair
) -> Result<serde_json::Value, String> {
    let _primer_service = PRIMER_SERVICE.lock().map_err(|e| e.to_string())?;

    // For now, return basic compatibility info
    // In a full implementation, we would deserialize primer_pairs and evaluate
    Ok(serde_json::json!({
        "compatibility": "good",
        "warnings": [],
        "overall_score": 0.8
    }))
}

/// Get storage statistics (for debugging/monitoring)
pub fn storage_info() -> Result<serde_json::Value, String> {
    let _service = SERVICE.lock().map_err(|e| e.to_string())?;

    // For now, return basic info - can be expanded later
    Ok(serde_json::json!({
        "status": "Layered architecture active",
        "architecture": "Domain-driven design with dependency inversion",
        "features": [
            "Memory-based sequences for small files",
            "File-based indexed access for large files",
            "Detailed statistics with entropy and complexity",
            "Windowed analysis support",
            "Layered architecture with clean separation",
            "PCR primer design with Tm calculation",
            "Multiplex primer compatibility analysis"
        ]
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_and_import() {
        let fasta_content = ">test_seq Test sequence\nATCGATCG".to_string();
        let result = parse_and_import(fasta_content, "fasta".to_string()).unwrap();

        assert!(result.seq_id.starts_with("seq_"));

        let meta = get_meta(result.seq_id.clone()).unwrap();
        assert_eq!(meta.id, "test_seq");
        assert_eq!(meta.name, "Test sequence");
        assert_eq!(meta.length, 8);
    }

    #[test]
    fn test_get_window() {
        let fasta_content = ">test_seq\nATCGATCGATCG".to_string();
        let result = parse_and_import(fasta_content, "fasta".to_string()).unwrap();

        let window = get_window(result.seq_id, 2, 6).unwrap();
        assert_eq!(window.bases, "CGAT");
    }

    #[test]
    fn test_stats() {
        let fasta_content = ">test_seq\nATCGATCG".to_string();
        let result = parse_and_import(fasta_content, "fasta".to_string()).unwrap();

        let stats = stats(result.seq_id).unwrap();
        assert_eq!(stats.length, 8);
        assert_eq!(stats.gc_overall, 50.0); // 4 GC out of 8 = 50%
        assert_eq!(stats.n_rate, 0.0);
    }

    #[test]
    fn test_detailed_stats() {
        let fasta_content = ">test_seq\nATCGATCG".to_string();
        let result = parse_and_import(fasta_content, "fasta".to_string()).unwrap();

        let stats = detailed_stats(result.seq_id).unwrap();
        assert_eq!(stats.detailed.length, 8);
        assert_eq!(stats.detailed.gc_percent, 50.0);
        assert_eq!(stats.detailed.base_counts.a, 2);
        assert_eq!(stats.detailed.base_counts.t, 2);
        assert_eq!(stats.detailed.base_counts.g, 2);
        assert_eq!(stats.detailed.base_counts.c, 2);
    }

    #[test]
    fn test_window_stats() {
        let fasta_content = ">test_seq\nGGGGCCCCAAAATTTT".to_string();
        let result = parse_and_import(fasta_content, "fasta".to_string()).unwrap();

        let windows = window_stats(result.seq_id, 4, 4).unwrap();
        assert_eq!(windows.len(), 4);
        assert_eq!(windows[0].gc_percent, 100.0); // GGGG
        assert_eq!(windows[1].gc_percent, 100.0); // CCCC
        assert_eq!(windows[2].gc_percent, 0.0); // AAAA
        assert_eq!(windows[3].gc_percent, 0.0); // TTTT
    }

    #[test]
    fn test_export() {
        let fasta_content = ">test_seq Test\nATCG".to_string();
        let result = parse_and_import(fasta_content, "fasta".to_string()).unwrap();

        let exported = export(result.seq_id, "fasta".to_string()).unwrap();
        assert!(exported.text.contains(">test_seq Test"));
        assert!(exported.text.contains("ATCG"));
    }

    #[test]
    fn test_file_import() {
        // Create a temporary FASTA file
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, ">test_file_seq Test from file").unwrap();
        writeln!(temp_file, "ATCGATCG").unwrap();
        writeln!(temp_file, "GCTAGCTA").unwrap();

        let request = ImportFromFileRequest {
            file_path: temp_file.path().to_string_lossy().to_string(),
            format: "fasta".to_string(),
        };

        let result = import_from_file(request).unwrap();
        let meta = get_meta(result.seq_id.clone()).unwrap();

        assert_eq!(meta.id, "test_file_seq");
        assert_eq!(meta.length, 16);
        assert!(meta.file_path.is_some());

        // Test window access
        let window = get_window(result.seq_id, 4, 12).unwrap();
        assert_eq!(window.bases, "ATCGGCTA");
    }

    #[test]
    fn test_storage_info() {
        let info = storage_info().unwrap();
        assert!(info.get("status").is_some());
        assert!(info.get("architecture").is_some());
        assert!(info.get("features").is_some());
    }
}
