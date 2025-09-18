use crate::domain::{Sequence, SequenceMetadata, Topology};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GenBankFeature {
    pub feature_type: String,
    pub location: String,
    pub qualifiers: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct GenBankRecord {
    pub locus: String,
    pub definition: String,
    pub accession: String,
    pub version: String,
    pub source: String,
    pub organism: String,
    pub length: usize,
    pub topology: Topology,
    pub molecule_type: String,
    pub division: String,
    pub date: String,
    pub features: Vec<GenBankFeature>,
    pub sequence: String,
}

pub struct GenBankParser;

impl GenBankParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, content: &str) -> Result<GenBankRecord, String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut record = GenBankRecord {
            locus: String::new(),
            definition: String::new(),
            accession: String::new(),
            version: String::new(),
            source: String::new(),
            organism: String::new(),
            length: 0,
            topology: Topology::Linear,
            molecule_type: String::new(),
            division: String::new(),
            date: String::new(),
            features: Vec::new(),
            sequence: String::new(),
        };

        let mut current_section = "";
        let mut sequence_section = false;
        let mut current_feature: Option<GenBankFeature> = None;

        for line in lines {
            if line.starts_with("//") {
                // End of record
                if let Some(feature) = current_feature {
                    record.features.push(feature);
                }
                break;
            }

            if sequence_section {
                // Parse sequence data
                let cleaned_line = line
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .collect::<String>()
                    .to_uppercase();
                record.sequence.push_str(&cleaned_line);
                continue;
            }

            if line.starts_with("LOCUS") {
                current_section = "LOCUS";
                self.parse_locus_line(line, &mut record)?;
            } else if line.starts_with("DEFINITION") {
                current_section = "DEFINITION";
                record.definition = self.extract_field_value(line, "DEFINITION");
            } else if line.starts_with("ACCESSION") {
                current_section = "ACCESSION";
                record.accession = self.extract_field_value(line, "ACCESSION");
            } else if line.starts_with("VERSION") {
                current_section = "VERSION";
                record.version = self.extract_field_value(line, "VERSION");
            } else if line.starts_with("SOURCE") {
                current_section = "SOURCE";
                record.source = self.extract_field_value(line, "SOURCE");
            } else if line.starts_with("  ORGANISM") {
                record.organism = self.extract_field_value(line, "ORGANISM");
            } else if line.starts_with("FEATURES") {
                current_section = "FEATURES";
            } else if line.starts_with("ORIGIN") {
                current_section = "ORIGIN";
                sequence_section = true;
                // Save any pending feature
                if let Some(feature) = current_feature.take() {
                    record.features.push(feature);
                }
            } else if current_section == "FEATURES" && !line.trim().is_empty() {
                // Parse features
                if line.starts_with("     ") && !line.starts_with("                     ") {
                    // New feature
                    if let Some(feature) = current_feature.take() {
                        record.features.push(feature);
                    }
                    current_feature = self.parse_feature_line(line)?;
                } else if line.starts_with("                     ") {
                    // Feature qualifier
                    if let Some(ref mut feature) = current_feature {
                        self.parse_feature_qualifier(line, feature)?;
                    }
                }
            } else if current_section == "DEFINITION" && line.starts_with("            ") {
                // Continuation of definition
                record.definition.push(' ');
                record.definition.push_str(line.trim());
            }
        }

        record.length = record.sequence.len();
        Ok(record)
    }

    fn parse_locus_line(&self, line: &str, record: &mut GenBankRecord) -> Result<(), String> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            record.locus = parts[1].to_string();

            // Parse length
            if let Ok(length) = parts[2].parse::<usize>() {
                record.length = length;
            }

            // Parse topology (circular/linear)
            for part in &parts[3..] {
                if part.to_lowercase() == "circular" {
                    record.topology = Topology::Circular;
                } else if part.to_lowercase() == "linear" {
                    record.topology = Topology::Linear;
                }

                // Parse molecule type
                if part.contains("DNA") || part.contains("RNA") {
                    record.molecule_type = part.to_string();
                }

                // Parse division (e.g., BCT, PLN, etc.)
                if part.len() == 3 && part.chars().all(|c| c.is_alphabetic()) {
                    record.division = part.to_string();
                }

                // Parse date
                if part.contains("-") && part.len() == 11 {
                    record.date = part.to_string();
                }
            }
        }
        Ok(())
    }

    fn extract_field_value(&self, line: &str, field_name: &str) -> String {
        if let Some(start) = line.find(field_name) {
            let value = line[start + field_name.len()..].trim();
            // For ACCESSION field, handle cases like "NC_000913 REGION: 1..5000"
            if field_name == "ACCESSION" {
                value.split_whitespace().next().unwrap_or(value).to_string()
            } else {
                value.to_string()
            }
        } else {
            String::new()
        }
    }

    fn parse_feature_line(&self, line: &str) -> Result<Option<GenBankFeature>, String> {
        let trimmed = line.trim();
        if let Some(space_pos) = trimmed.find(' ') {
            let feature_type = trimmed[..space_pos].to_string();
            let location = trimmed[space_pos + 1..].trim().to_string();

            Ok(Some(GenBankFeature {
                feature_type,
                location,
                qualifiers: HashMap::new(),
            }))
        } else {
            Ok(None)
        }
    }

    fn parse_feature_qualifier(
        &self,
        line: &str,
        feature: &mut GenBankFeature,
    ) -> Result<(), String> {
        let trimmed = line.trim();
        if trimmed.starts_with('/') {
            if let Some(eq_pos) = trimmed.find('=') {
                let key = trimmed[1..eq_pos].to_string();
                let mut value = trimmed[eq_pos + 1..].to_string();

                // Remove quotes if present
                if value.starts_with('"') && value.ends_with('"') {
                    value = value[1..value.len() - 1].to_string();
                }

                feature.qualifiers.insert(key, value);
            } else {
                // Boolean qualifier
                feature
                    .qualifiers
                    .insert(trimmed[1..].to_string(), "true".to_string());
            }
        }
        Ok(())
    }

    pub fn to_sequence(&self, record: &GenBankRecord) -> Sequence {
        Sequence {
            id: record.accession.clone(),
            name: record.definition.clone(),
            sequence: record.sequence.clone(),
            topology: record.topology.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_genbank() {
        let genbank_content = r#"LOCUS       TEST_SEQ                 100 bp    DNA     linear   BCT 01-JAN-2024
DEFINITION  Test sequence for GenBank parser.
ACCESSION   TEST001
VERSION     TEST001.1
SOURCE      Test organism
  ORGANISM  Test organism
            Bacteria; Test phylum; Test class.
FEATURES             Location/Qualifiers
     source          1..100
                     /organism="Test organism"
                     /mol_type="genomic DNA"
     gene            10..90
                     /gene="testA"
                     /product="test protein A"
ORIGIN
        1 atgcgtacgt cgtagctagt cgtagctagc tagctagcta gctagctagt cgtagctacg
       61 tagctagcta gctagctagt cgtagctagt cgtagctacg
//
"#;

        let parser = GenBankParser::new();
        let result = parser.parse(genbank_content);

        assert!(result.is_ok());
        let record = result.unwrap();

        assert_eq!(record.locus, "TEST_SEQ");
        assert_eq!(record.accession, "TEST001");
        assert_eq!(record.length, 100);
        assert_eq!(record.topology, Topology::Linear);
        assert!(record.sequence.len() > 0);
        assert!(!record.features.is_empty());
    }
}
