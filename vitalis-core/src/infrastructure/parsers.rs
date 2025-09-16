// Infrastructure layer: Parser implementations
use crate::domain::{Sequence, SequenceParser, Topology};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Length mismatch: {0}")]
    LengthMismatch(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// FASTA parser implementation
pub struct FastaParser;

impl SequenceParser for FastaParser {
    type Error = ParserError;

    fn parse(&self, content: &str) -> Result<Vec<Sequence>, Self::Error> {
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

        if sequences.is_empty() {
            return Err(ParserError::InvalidFormat("No sequences found".to_string()));
        }

        Ok(sequences)
    }
}

/// FASTQ parser implementation
pub struct FastqParser;

impl SequenceParser for FastqParser {
    type Error = ParserError;

    fn parse(&self, content: &str) -> Result<Vec<Sequence>, Self::Error> {
        let mut sequences = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        let mut i = 0;
        while i + 3 < lines.len() {
            if !lines[i].starts_with('@') {
                return Err(ParserError::InvalidFormat(
                    "Invalid FASTQ format".to_string(),
                ));
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

        if sequences.is_empty() {
            return Err(ParserError::InvalidFormat("No sequences found".to_string()));
        }

        Ok(sequences)
    }
}
