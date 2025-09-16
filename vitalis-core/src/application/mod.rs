// Application layer - Tauri commands and use cases
use crate::domain::{
    DetailedStats, SequenceAnalysisService, SequenceRepository, Topology, WindowStats,
};
use crate::infrastructure::FileSequenceRepository;
use crate::services::StatsServiceImpl;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;

// Response types for Tauri commands
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportResponse {
    pub seq_id: String,
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
pub struct ImportFromFileRequest {
    pub file_path: String,
    pub format: String,
}

// Global service instance (thread-safe)
type ServiceType = SequenceAnalysisService<FileSequenceRepository, StatsServiceImpl>;

lazy_static::lazy_static! {
    static ref SERVICE: Mutex<ServiceType> = Mutex::new(
        SequenceAnalysisService::new(
            FileSequenceRepository::new(),
            StatsServiceImpl::new()
        )
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

/// Calculate windowed statistics
pub fn window_stats(
    seq_id: String,
    window_size: usize,
    step: usize,
) -> Result<WindowStatsResponse, String> {
    let mut service = SERVICE.lock().map_err(|e| e.to_string())?;
    let windows = service
        .analyze_window(&seq_id, window_size, step)
        .map_err(|e| e.to_string())?;

    Ok(WindowStatsResponse { windows })
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
            "Layered architecture with clean separation"
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
        assert_eq!(windows.windows.len(), 4);
        assert_eq!(windows.windows[0].gc_percent, 100.0); // GGGG
        assert_eq!(windows.windows[1].gc_percent, 100.0); // CCCC
        assert_eq!(windows.windows[2].gc_percent, 0.0); // AAAA
        assert_eq!(windows.windows[3].gc_percent, 0.0); // TTTT
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
