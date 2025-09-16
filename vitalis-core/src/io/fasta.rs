use super::ParseError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FastaRecord {
    pub id: String,
    pub description: Option<String>,
    pub sequence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceStats {
    pub length: usize,
    pub gc_count: usize,
    pub n_count: usize,
    pub gc_percent: f64,
    pub n_percent: f64,
}

impl FastaRecord {
    pub fn new(id: String, description: Option<String>, sequence: String) -> Self {
        Self {
            id,
            description,
            sequence: sequence
                .to_uppercase()
                .replace(|c: char| c.is_whitespace(), ""),
        }
    }

    pub fn calculate_stats(&self) -> SequenceStats {
        let length = self.sequence.len();
        let gc_count = self
            .sequence
            .chars()
            .filter(|&c| c == 'G' || c == 'C')
            .count();
        let n_count = self.sequence.chars().filter(|&c| c == 'N').count();

        let gc_percent = if length > 0 {
            (gc_count as f64 / length as f64) * 100.0
        } else {
            0.0
        };

        let n_percent = if length > 0 {
            (n_count as f64 / length as f64) * 100.0
        } else {
            0.0
        };

        SequenceStats {
            length,
            gc_count,
            n_count,
            gc_percent,
            n_percent,
        }
    }
}

pub fn parse_fasta(content: &str) -> Result<Vec<FastaRecord>, ParseError> {
    let mut records = Vec::new();
    let mut current_id = String::new();
    let mut current_desc = None;
    let mut current_seq = String::new();

    for line in content.lines() {
        if line.starts_with('>') {
            // Save previous record if exists
            if !current_id.is_empty() {
                records.push(FastaRecord::new(
                    current_id.clone(),
                    current_desc.clone(),
                    current_seq.clone(),
                ));
                current_seq.clear();
            }

            // Parse header
            let header = &line[1..].trim_start(); // Remove '>' and leading whitespace
            let parts: Vec<&str> = header.splitn(2, |c: char| c.is_whitespace()).collect();

            current_id = parts[0].to_string();
            current_desc = if parts.len() > 1 && !parts[1].is_empty() {
                Some(parts[1].to_string())
            } else {
                None
            };
        } else if !line.trim().is_empty() {
            // Accumulate sequence
            current_seq.push_str(line.trim());
        }
    }

    // Save last record if exists
    if !current_id.is_empty() {
        records.push(FastaRecord::new(current_id, current_desc, current_seq));
    }

    // Check for invalid format (no sequences starting with >)
    if !content.is_empty() && !content.trim().is_empty() && !content.trim().starts_with('>') {
        return Err(ParseError::InvalidFormat(
            "FASTA content must start with '>'".to_string(),
        ));
    }

    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_fasta_parsing() {
        let content = ">seq1 description\nATCG\n>seq2\nGGCC";
        let records = parse_fasta(content).unwrap();

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].id, "seq1");
        assert_eq!(records[0].description, Some("description".to_string()));
        assert_eq!(records[0].sequence, "ATCG");
        assert_eq!(records[1].id, "seq2");
        assert_eq!(records[1].description, None);
        assert_eq!(records[1].sequence, "GGCC");
    }

    #[test]
    fn test_multiline_sequence() {
        let content = ">seq1\nATCG\nGGCC\nTTAA";
        let records = parse_fasta(content).unwrap();

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].sequence, "ATCGGGCCTTAA");
    }

    #[test]
    fn test_lowercase_conversion() {
        let content = ">seq1\natcg";
        let records = parse_fasta(content).unwrap();

        assert_eq!(records[0].sequence, "ATCG");
    }
}
