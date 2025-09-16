use crate::io::{parse_fasta, parse_fastq};
use crate::{Range, Topology};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

// In-memory sequence storage (simplified for Phase 0)
static mut SEQUENCE_STORAGE: Option<HashMap<String, StoredSequence>> = None;
static mut NEXT_ID: u32 = 1;

#[derive(Debug, Clone)]
struct StoredSequence {
    id: String,
    name: String,
    sequence: String,
    topology: Topology,
    format: String, // "fasta" or "fastq"
    quality: Option<String>, // For FASTQ
}

fn get_storage() -> &'static mut HashMap<String, StoredSequence> {
    unsafe {
        if SEQUENCE_STORAGE.is_none() {
            SEQUENCE_STORAGE = Some(HashMap::new());
        }
        SEQUENCE_STORAGE.as_mut().unwrap()
    }
}

fn next_id() -> String {
    unsafe {
        let id = format!("seq_{}", NEXT_ID);
        NEXT_ID += 1;
        id
    }
}

/// Parse and import sequences from text content
pub fn parse_and_import(text: String, fmt: String) -> Result<ImportResponse, String> {
    let storage = get_storage();
    
    match fmt.as_str() {
        "fasta" => {
            let records = parse_fasta(&text).map_err(|e| e.to_string())?;
            
            if records.is_empty() {
                return Err("No sequences found in FASTA content".to_string());
            }
            
            // For Phase 0, import only the first sequence
            let record = &records[0];
            let seq_id = next_id();
            
            storage.insert(seq_id.clone(), StoredSequence {
                id: record.id.clone(),
                name: record.description.clone().unwrap_or_else(|| record.id.clone()),
                sequence: record.sequence.clone(),
                topology: Topology::Linear, // Default to linear
                format: "fasta".to_string(),
                quality: None,
            });
            
            Ok(ImportResponse { seq_id })
        }
        "fastq" => {
            let records = parse_fastq(&text).map_err(|e| e.to_string())?;
            
            if records.is_empty() {
                return Err("No sequences found in FASTQ content".to_string());
            }
            
            // For Phase 0, import only the first sequence
            let record = &records[0];
            let seq_id = next_id();
            
            storage.insert(seq_id.clone(), StoredSequence {
                id: record.id.clone(),
                name: record.description.clone().unwrap_or_else(|| record.id.clone()),
                sequence: record.sequence.clone(),
                topology: Topology::Linear,
                format: "fastq".to_string(),
                quality: Some(record.quality.clone()),
            });
            
            Ok(ImportResponse { seq_id })
        }
        _ => Err(format!("Unsupported format: {}", fmt)),
    }
}

/// Get sequence metadata
pub fn get_meta(seq_id: String) -> Result<SequenceMeta, String> {
    let storage = get_storage();
    
    match storage.get(&seq_id) {
        Some(seq) => Ok(SequenceMeta {
            id: seq.id.clone(),
            name: seq.name.clone(),
            length: seq.sequence.len(),
            topology: seq.topology.clone(),
        }),
        None => Err(format!("Sequence not found: {}", seq_id)),
    }
}

/// Get sequence window
pub fn get_window(seq_id: String, start: usize, end: usize) -> Result<WindowResponse, String> {
    let storage = get_storage();
    
    match storage.get(&seq_id) {
        Some(seq) => {
            if start >= seq.sequence.len() {
                return Err("Start position exceeds sequence length".to_string());
            }
            
            let end = end.min(seq.sequence.len());
            let bases = seq.sequence[start..end].to_string();
            
            Ok(WindowResponse { bases })
        }
        None => Err(format!("Sequence not found: {}", seq_id)),
    }
}

/// Calculate sequence statistics
pub fn stats(seq_id: String) -> Result<SequenceStats, String> {
    let storage = get_storage();
    
    match storage.get(&seq_id) {
        Some(seq) => {
            let sequence = &seq.sequence;
            let length = sequence.len();
            
            if length == 0 {
                return Ok(SequenceStats {
                    gc_overall: 0.0,
                    n_rate: 0.0,
                    length: 0,
                });
            }
            
            let gc_count = sequence.chars()
                .filter(|&c| c == 'G' || c == 'C')
                .count();
            let n_count = sequence.chars()
                .filter(|&c| c == 'N')
                .count();
            
            let gc_overall = (gc_count as f64 / length as f64) * 100.0;
            let n_rate = (n_count as f64 / length as f64) * 100.0;
            
            Ok(SequenceStats {
                gc_overall,
                n_rate,
                length,
            })
        }
        None => Err(format!("Sequence not found: {}", seq_id)),
    }
}

/// Export sequence to text format
pub fn export(seq_id: String, fmt: String) -> Result<ExportResponse, String> {
    let storage = get_storage();
    
    match storage.get(&seq_id) {
        Some(seq) => {
            let text = match fmt.as_str() {
                "fasta" => {
                    format!(">{} {}\n{}\n", seq.id, seq.name, seq.sequence)
                }
                "fastq" => {
                    if let Some(quality) = &seq.quality {
                        format!("@{} {}\n{}\n+\n{}\n", seq.id, seq.name, seq.sequence, quality)
                    } else {
                        // Convert FASTA to FASTQ with dummy quality
                        let dummy_quality = "I".repeat(seq.sequence.len());
                        format!("@{} {}\n{}\n+\n{}\n", seq.id, seq.name, seq.sequence, dummy_quality)
                    }
                }
                _ => return Err(format!("Unsupported export format: {}", fmt)),
            };
            
            Ok(ExportResponse { text })
        }
        None => Err(format!("Sequence not found: {}", seq_id)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_and_import_fasta() {
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
    fn test_export() {
        let fasta_content = ">test_seq Test\nATCG".to_string();
        let result = parse_and_import(fasta_content, "fasta".to_string()).unwrap();
        
        let exported = export(result.seq_id, "fasta".to_string()).unwrap();
        assert!(exported.text.contains(">test_seq Test"));
        assert!(exported.text.contains("ATCG"));
    }
}