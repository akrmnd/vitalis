// Infrastructure layer: Storage implementation
use crate::domain::{Sequence, SequenceMetadata, SequenceRepository, Topology};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Sequence not found: {0}")]
    SequenceNotFound(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Invalid range: start={0}, end={1}")]
    InvalidRange(usize, usize),
}

/// ファイル内のバイト位置を記録
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ByteOffset {
    start: u64,
    length: usize,
}

/// 配列のソース（メモリまたはファイル）
#[derive(Debug, Clone)]
enum SequenceSource {
    Memory(String),
    File { path: PathBuf, offset: ByteOffset },
}

/// Infrastructure層でのRepositoryトレイト実装
pub struct FileSequenceRepository {
    sequences: HashMap<String, SequenceSource>,
    metadata: HashMap<String, SequenceMetadata>,
    next_id: usize,
}

impl FileSequenceRepository {
    pub fn new() -> Self {
        Self {
            sequences: HashMap::new(),
            metadata: HashMap::new(),
            next_id: 1,
        }
    }

    fn generate_id(&mut self) -> String {
        let id = format!("seq_{}", self.next_id);
        self.next_id += 1;
        id
    }

    fn parse_fasta(&self, content: &str) -> Result<Vec<Sequence>, StorageError> {
        let mut sequences = Vec::new();
        let mut current_id = String::new();
        let mut current_name = String::new();
        let mut current_sequence = String::new();

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('>') {
                // Save previous sequence if exists
                if !current_id.is_empty() {
                    sequences.push(Sequence {
                        id: current_id.clone(),
                        name: current_name.clone(),
                        sequence: current_sequence.clone(),
                        topology: Topology::Linear,
                    });
                }

                // Parse new header
                let header = &line[1..];
                let parts: Vec<&str> = header.split_whitespace().collect();
                current_id = parts.first().unwrap_or(&"unknown").to_string();
                current_name = parts.get(1..).map(|p| p.join(" ")).unwrap_or_default();
                current_sequence.clear();
            } else if !line.is_empty() {
                current_sequence.push_str(line);
            }
        }

        // Save last sequence
        if !current_id.is_empty() {
            sequences.push(Sequence {
                id: current_id,
                name: current_name,
                sequence: current_sequence,
                topology: Topology::Linear,
            });
        }

        Ok(sequences)
    }

    fn parse_fastq(&self, content: &str) -> Result<Vec<Sequence>, StorageError> {
        let mut sequences = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        let mut i = 0;
        while i + 3 < lines.len() {
            if !lines[i].starts_with('@') {
                return Err(StorageError::ParseError("Invalid FASTQ format".to_string()));
            }

            let header = &lines[i][1..];
            let parts: Vec<&str> = header.split_whitespace().collect();
            let id = parts.first().unwrap_or(&"unknown").to_string();
            let name = parts.get(1..).map(|p| p.join(" ")).unwrap_or_default();
            let sequence = lines[i + 1].to_string();

            sequences.push(Sequence {
                id,
                name,
                sequence,
                topology: Topology::Linear,
            });

            i += 4; // Skip quality lines
        }

        Ok(sequences)
    }

    pub fn import_from_text(
        &mut self,
        content: &str,
        format: &str,
    ) -> Result<String, StorageError> {
        let sequences = match format {
            "fasta" => self.parse_fasta(content)?,
            "fastq" => self.parse_fastq(content)?,
            _ => {
                return Err(StorageError::ParseError(format!(
                    "Unsupported format: {}",
                    format
                )))
            }
        };

        if sequences.is_empty() {
            return Err(StorageError::ParseError("No sequences found".to_string()));
        }

        // For simplicity, just use the first sequence
        let sequence = &sequences[0];
        let seq_id = self.generate_id();

        // Store in memory for text import
        self.sequences.insert(
            seq_id.clone(),
            SequenceSource::Memory(sequence.sequence.clone()),
        );
        self.metadata.insert(
            seq_id.clone(),
            SequenceMetadata {
                id: sequence.id.clone(),
                name: sequence.name.clone(),
                length: sequence.sequence.len(),
                topology: sequence.topology.clone(),
                file_path: None,
            },
        );

        Ok(seq_id)
    }

    pub fn import_from_file(
        &mut self,
        file_path: &Path,
        format: &str,
    ) -> Result<String, StorageError> {
        let mut file = File::open(file_path)?;
        let metadata = file.metadata()?;

        // For large files, use indexed access
        if metadata.len() > 1024 * 1024 {
            // 1MB threshold
            self.import_large_file(file_path, format)
        } else {
            // For small files, load into memory
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            let seq_id = self.import_from_text(&content, format)?;

            // Update metadata to include file path
            if let Some(meta) = self.metadata.get_mut(&seq_id) {
                meta.file_path = Some(file_path.to_path_buf());
            }

            Ok(seq_id)
        }
    }

    fn import_large_file(
        &mut self,
        file_path: &Path,
        format: &str,
    ) -> Result<String, StorageError> {
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();

        // Find the first sequence header and data
        let mut header_pos = 0u64;
        let mut data_start = 0u64;
        let mut sequence_length = 0usize;
        let mut id = String::new();
        let mut name = String::new();

        // Find header
        loop {
            line.clear();
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 {
                return Err(StorageError::ParseError("No sequence found".to_string()));
            }

            if (format == "fasta" && line.starts_with('>'))
                || (format == "fastq" && line.starts_with('@'))
            {
                let header = &line[1..].trim();
                let parts: Vec<&str> = header.split_whitespace().collect();
                id = parts.first().unwrap_or(&"unknown").to_string();
                name = parts.get(1..).map(|p| p.join(" ")).unwrap_or_default();
                data_start = reader.stream_position()?;
                break;
            }

            header_pos += bytes_read as u64;
        }

        // Count sequence length
        loop {
            line.clear();
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 {
                break;
            }

            let trimmed = line.trim();
            if trimmed.starts_with('>') || trimmed.starts_with('@') || trimmed.starts_with('+') {
                break;
            }

            if !trimmed.is_empty() {
                sequence_length += trimmed.len();
            }
        }

        let seq_id = self.generate_id();

        // Store file reference
        self.sequences.insert(
            seq_id.clone(),
            SequenceSource::File {
                path: file_path.to_path_buf(),
                offset: ByteOffset {
                    start: data_start,
                    length: sequence_length,
                },
            },
        );

        self.metadata.insert(
            seq_id.clone(),
            SequenceMetadata {
                id,
                name,
                length: sequence_length,
                topology: Topology::Linear,
                file_path: Some(file_path.to_path_buf()),
            },
        );

        Ok(seq_id)
    }

    fn read_file_window(
        &self,
        path: &Path,
        offset: &ByteOffset,
        start: usize,
        end: usize,
    ) -> Result<String, StorageError> {
        // Handle edge cases consistently with memory implementation
        if start >= offset.length {
            return Err(StorageError::InvalidRange(start, end));
        }
        
        // Allow start >= end, return empty string
        if start >= end {
            return Ok(String::new());
        }
        
        // Clamp end to sequence length
        let end = end.min(offset.length);

        let mut file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // Seek to the data start
        reader.seek(SeekFrom::Start(offset.start))?;

        let mut result = String::new();
        let mut current_pos = 0;
        let mut line = String::new();

        while current_pos < end {
            line.clear();
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 {
                break;
            }

            let trimmed = line.trim();
            // Skip header lines and empty lines
            if trimmed.starts_with('>')
                || trimmed.starts_with('@')
                || trimmed.starts_with('+')
                || trimmed.is_empty()
            {
                continue;
            }

            // Process each character in the line
            for ch in trimmed.chars() {
                if current_pos >= start && current_pos < end {
                    result.push(ch.to_ascii_uppercase());
                }
                current_pos += 1;
                if current_pos >= end {
                    break;
                }
            }
        }

        Ok(result)
    }
}

impl SequenceRepository for FileSequenceRepository {
    type Error = StorageError;

    fn store_sequence(&mut self, sequence: Sequence) -> Result<String, Self::Error> {
        let seq_id = self.generate_id();

        self.sequences.insert(
            seq_id.clone(),
            SequenceSource::Memory(sequence.sequence.clone()),
        );
        self.metadata.insert(
            seq_id.clone(),
            SequenceMetadata {
                id: sequence.id,
                name: sequence.name,
                length: sequence.sequence.len(),
                topology: sequence.topology,
                file_path: None,
            },
        );

        Ok(seq_id)
    }

    fn store_sequence_from_file(
        &mut self,
        file_path: &Path,
        format: &str,
    ) -> Result<String, Self::Error> {
        self.import_from_file(file_path, format)
    }

    fn get_metadata(&self, seq_id: &str) -> Option<SequenceMetadata> {
        self.metadata.get(seq_id).cloned()
    }

    fn get_sequence(&self, seq_id: &str) -> Result<String, Self::Error> {
        match self.sequences.get(seq_id) {
            Some(SequenceSource::Memory(seq)) => Ok(seq.clone()),
            Some(SequenceSource::File { path, offset }) => {
                self.read_file_window(path, offset, 0, offset.length)
            }
            None => Err(StorageError::SequenceNotFound(seq_id.to_string())),
        }
    }

    fn get_window(&self, seq_id: &str, start: usize, end: usize) -> Result<String, Self::Error> {
        match self.sequences.get(seq_id) {
            Some(SequenceSource::Memory(seq)) => {
                // Handle edge cases consistently
                if start >= seq.len() {
                    return Err(StorageError::InvalidRange(start, end));
                }
                
                // Allow start >= end, return empty string
                if start >= end {
                    return Ok(String::new());
                }
                
                // Clamp end to sequence length
                let end = end.min(seq.len());
                // Convert to uppercase for consistency
                Ok(seq[start..end].to_ascii_uppercase())
            }
            Some(SequenceSource::File { path, offset }) => {
                self.read_file_window(path, offset, start, end)
            }
            None => Err(StorageError::SequenceNotFound(seq_id.to_string())),
        }
    }
}
