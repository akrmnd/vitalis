use crate::io::{parse_fasta, parse_fastq};
use crate::Topology;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceMetadata {
    pub id: String,
    pub name: String,
    pub length: usize,
    pub topology: Topology,
    pub format: String,
    pub file_path: Option<PathBuf>,
    pub byte_offsets: Vec<ByteOffset>, // For indexed access
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByteOffset {
    pub seq_position: usize, // Position in sequence
    pub byte_position: u64,  // Position in file
    pub line_length: usize,  // Length of this line
}

#[derive(Debug)]
pub enum SequenceSource {
    Memory(String),
    File(PathBuf, Vec<ByteOffset>),
}

#[derive(Debug)]
pub struct SequenceStorage {
    metadata: HashMap<String, SequenceMetadata>,
    sources: HashMap<String, SequenceSource>,
    next_id: u32,
}

impl SequenceStorage {
    pub fn new() -> Self {
        Self {
            metadata: HashMap::new(),
            sources: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn next_id(&mut self) -> String {
        let id = format!("seq_{}", self.next_id);
        self.next_id += 1;
        id
    }

    /// Import sequence from text (keeps in memory)
    pub fn import_from_text(&mut self, text: &str, format: &str) -> Result<String, String> {
        match format {
            "fasta" => {
                let records = parse_fasta(text).map_err(|e| e.to_string())?;
                if records.is_empty() {
                    return Err("No sequences found".to_string());
                }

                let record = &records[0];
                let seq_id = self.next_id();

                let metadata = SequenceMetadata {
                    id: record.id.clone(),
                    name: record
                        .description
                        .clone()
                        .unwrap_or_else(|| record.id.clone()),
                    length: record.sequence.len(),
                    topology: Topology::Linear,
                    format: format.to_string(),
                    file_path: None,
                    byte_offsets: Vec::new(),
                };

                self.metadata.insert(seq_id.clone(), metadata);
                self.sources.insert(
                    seq_id.clone(),
                    SequenceSource::Memory(record.sequence.clone()),
                );

                Ok(seq_id)
            }
            "fastq" => {
                let records = parse_fastq(text).map_err(|e| e.to_string())?;
                if records.is_empty() {
                    return Err("No sequences found".to_string());
                }

                let record = &records[0];
                let seq_id = self.next_id();

                let metadata = SequenceMetadata {
                    id: record.id.clone(),
                    name: record
                        .description
                        .clone()
                        .unwrap_or_else(|| record.id.clone()),
                    length: record.sequence.len(),
                    topology: Topology::Linear,
                    format: format.to_string(),
                    file_path: None,
                    byte_offsets: Vec::new(),
                };

                self.metadata.insert(seq_id.clone(), metadata);
                self.sources.insert(
                    seq_id.clone(),
                    SequenceSource::Memory(record.sequence.clone()),
                );

                Ok(seq_id)
            }
            _ => Err(format!("Unsupported format: {}", format)),
        }
    }

    /// Import sequence from file (builds index for streaming)
    pub fn import_from_file(&mut self, path: &Path, format: &str) -> Result<String, String> {
        if !path.exists() {
            return Err(format!("File not found: {:?}", path));
        }

        match format {
            "fasta" => {
                let file = File::open(path).map_err(|e| e.to_string())?;
                let reader = BufReader::new(file);
                let mut offsets = Vec::new();
                let mut current_pos = 0u64;
                let mut seq_started = false;
                let mut seq_position = 0usize;
                let mut total_length = 0usize;
                let mut id = String::new();
                let mut description = String::new();

                for line in reader.lines() {
                    let line = line.map_err(|e| e.to_string())?;
                    let line_bytes = line.len() as u64 + 1; // +1 for newline

                    if line.starts_with('>') {
                        if seq_started {
                            break; // Only handle first sequence for now
                        }
                        let header = &line[1..];
                        let parts: Vec<&str> =
                            header.splitn(2, |c: char| c.is_whitespace()).collect();
                        id = parts[0].to_string();
                        description = parts
                            .get(1)
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| id.clone());
                        seq_started = true;
                    } else if seq_started && !line.trim().is_empty() {
                        let clean_line = line.trim();
                        offsets.push(ByteOffset {
                            seq_position,
                            byte_position: current_pos,
                            line_length: clean_line.len(),
                        });
                        seq_position += clean_line.len();
                        total_length += clean_line.len();
                    }

                    current_pos += line_bytes;
                }

                if id.is_empty() {
                    return Err("No valid FASTA sequence found".to_string());
                }

                let seq_id = self.next_id();

                let metadata = SequenceMetadata {
                    id: id.clone(),
                    name: description,
                    length: total_length,
                    topology: Topology::Linear,
                    format: format.to_string(),
                    file_path: Some(path.to_path_buf()),
                    byte_offsets: offsets.clone(),
                };

                self.metadata.insert(seq_id.clone(), metadata);
                self.sources.insert(
                    seq_id.clone(),
                    SequenceSource::File(path.to_path_buf(), offsets),
                );

                Ok(seq_id)
            }
            _ => Err(format!(
                "File-based import not yet supported for format: {}",
                format
            )),
        }
    }

    // REMOVED: Old get_window implementation moved to new layered architecture
    // Use application::get_window instead

    // DELETED: read_window_from_file - moved to new layered architecture

    pub fn get_metadata(&self, seq_id: &str) -> Option<&SequenceMetadata> {
        self.metadata.get(seq_id)
    }

    pub fn get_full_sequence(&self, seq_id: &str) -> Result<String, String> {
        let metadata = self
            .metadata
            .get(seq_id)
            .ok_or_else(|| format!("Sequence not found: {}", seq_id))?;

        // Use new layered architecture
        crate::application::get_window(seq_id.to_string(), 0, metadata.length)
            .map(|response| response.bases)
            .map_err(|e| e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_memory_storage() {
        let fasta = ">test\nATCGATCG";
        
        // Use new layered architecture for import
        let result = crate::application::parse_and_import(fasta.to_string(), "fasta".to_string()).unwrap();
        let seq_id = result.seq_id;

        // Use new layered architecture for window access
        let response = crate::application::get_window(seq_id, 2, 6).unwrap();
        assert_eq!(response.bases, "CGAT");
    }

    #[test]
    fn test_file_storage() {
        // Create a temporary FASTA file
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, ">test_seq Test sequence").unwrap();
        writeln!(temp_file, "ATCGATCG").unwrap();
        writeln!(temp_file, "GCTAGCTA").unwrap();
        writeln!(temp_file, "TTAATTAA").unwrap();

        // Read the file content and import using new architecture
        let file_content = std::fs::read_to_string(temp_file.path()).unwrap();
        let result = crate::application::parse_and_import(file_content, "fasta".to_string()).unwrap();
        let seq_id = result.seq_id;

        // Test window access using new layered architecture
        let response = crate::application::get_window(seq_id.clone(), 0, 8).unwrap();
        assert_eq!(response.bases, "ATCGATCG");

        let response = crate::application::get_window(seq_id.clone(), 8, 16).unwrap();
        assert_eq!(response.bases, "GCTAGCTA");

        let response = crate::application::get_window(seq_id, 4, 12).unwrap();
        assert_eq!(response.bases, "ATCGGCTA");
    }
}
