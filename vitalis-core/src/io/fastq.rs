use super::ParseError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FastqRecord {
    pub id: String,
    pub description: Option<String>,
    pub sequence: String,
    pub quality: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FastqStats {
    pub length: usize,
    pub min_quality: u8,
    pub max_quality: u8,
    pub mean_quality: f64,
}

impl FastqRecord {
    pub fn new(id: String, description: Option<String>, sequence: String, quality: String) -> Result<Self, ParseError> {
        if sequence.len() != quality.len() {
            return Err(ParseError::LengthMismatch(
                format!("Sequence length ({}) != quality length ({})", sequence.len(), quality.len())
            ));
        }
        
        Ok(Self {
            id,
            description,
            sequence: sequence.to_uppercase().replace(|c: char| c.is_whitespace(), ""),
            quality: quality.replace(|c: char| c.is_whitespace(), ""),
        })
    }
    
    pub fn get_quality_scores(&self) -> Vec<u8> {
        self.quality.bytes()
            .map(|b| b.saturating_sub(33)) // Phred+33 encoding
            .collect()
    }
    
    pub fn calculate_stats(&self) -> FastqStats {
        let scores = self.get_quality_scores();
        let length = scores.len();
        
        let min_quality = *scores.iter().min().unwrap_or(&0);
        let max_quality = *scores.iter().max().unwrap_or(&0);
        let mean_quality = if length > 0 {
            scores.iter().map(|&q| q as f64).sum::<f64>() / length as f64
        } else {
            0.0
        };
        
        FastqStats {
            length,
            min_quality,
            max_quality,
            mean_quality,
        }
    }
    
    pub fn trim_by_quality(&mut self, min_quality: u8) {
        let scores = self.get_quality_scores();
        
        // Find first position with quality < min_quality from the start
        let mut start_pos = 0;
        for (i, &score) in scores.iter().enumerate() {
            if score < min_quality {
                break;
            }
            start_pos = i + 1;
        }
        
        // Find first position with quality < min_quality from the end
        let mut end_pos = scores.len();
        for (i, &score) in scores.iter().enumerate().rev() {
            if score < min_quality {
                end_pos = i;
            } else {
                break;
            }
        }
        
        if start_pos < end_pos {
            self.sequence = self.sequence[start_pos..end_pos].to_string();
            self.quality = self.quality[start_pos..end_pos].to_string();
        } else if start_pos == scores.len() {
            // All qualities are good, keep everything
            // Do nothing
        } else {
            self.sequence.clear();
            self.quality.clear();
        }
    }
    
    pub fn trim_to_length(&mut self, max_length: usize) {
        if self.sequence.len() > max_length {
            self.sequence.truncate(max_length);
            self.quality.truncate(max_length);
        }
    }
}

pub fn parse_fastq(content: &str) -> Result<Vec<FastqRecord>, ParseError> {
    let mut records = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    
    while i < lines.len() {
        // Skip empty lines
        if lines[i].trim().is_empty() {
            i += 1;
            continue;
        }
        
        // Parse header
        if !lines[i].starts_with('@') {
            return Err(ParseError::InvalidFormat(
                format!("Expected '@' at line {}, found '{}'", i + 1, lines[i])
            ));
        }
        
        let header = &lines[i][1..]; // Remove '@'
        let parts: Vec<&str> = header.splitn(2, |c: char| c.is_whitespace()).collect();
        let id = parts[0].to_string();
        let description = if parts.len() > 1 && !parts[1].is_empty() {
            Some(parts[1].to_string())
        } else {
            None
        };
        
        // Parse sequence
        i += 1;
        if i >= lines.len() {
            return Err(ParseError::MissingField("sequence".to_string()));
        }
        let sequence = lines[i].trim().to_string();
        
        // Parse '+' separator
        i += 1;
        if i >= lines.len() || !lines[i].starts_with('+') {
            return Err(ParseError::InvalidFormat(
                "Expected '+' separator".to_string()
            ));
        }
        
        // Parse quality
        i += 1;
        if i >= lines.len() {
            return Err(ParseError::MissingField("quality".to_string()));
        }
        let quality = lines[i].trim().to_string();
        
        // Create record
        let record = FastqRecord::new(id, description, sequence, quality)?;
        records.push(record);
        
        i += 1;
    }
    
    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_fastq_parsing() {
        let content = "@read1 desc\nATCG\n+\nIIII\n@read2\nGGCC\n+\nHHHH";
        let records = parse_fastq(content).unwrap();
        
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].id, "read1");
        assert_eq!(records[0].description, Some("desc".to_string()));
        assert_eq!(records[0].sequence, "ATCG");
        assert_eq!(records[0].quality, "IIII");
    }
    
    #[test]
    fn test_quality_scores() {
        let content = "@read1\nATCG\n+\n!III"; // ! = 33 (Q0), I = 73 (Q40)
        let records = parse_fastq(content).unwrap();
        let scores = records[0].get_quality_scores();
        
        assert_eq!(scores, vec![0, 40, 40, 40]);
    }
    
    #[test]
    fn test_length_mismatch() {
        let content = "@read1\nATCG\n+\nII"; // Mismatched lengths
        let result = parse_fastq(content);
        
        assert!(result.is_err());
    }
}