// Integration test for GenBank parser with real NCBI files
use std::fs;
use vitalis_core::infrastructure::GenBankParser;

#[test]
fn test_parse_real_ecoli_genbank() {
    let parser = GenBankParser::new();

    // Skip if file doesn't exist (for CI environments)
    let content = match fs::read_to_string("tests/data/sample_ecoli.gb") {
        Ok(content) => content,
        Err(_) => {
            println!("Warning: E. coli GenBank file not found, skipping test");
            return;
        }
    };

    let record = parser
        .parse(&content)
        .expect("Failed to parse E. coli GenBank");

    assert_eq!(record.accession, "NC_000913");
    assert!(record.definition.contains("Escherichia coli"));
    assert_eq!(record.length, 5000);
    assert!(record.sequence.len() > 0);
    assert!(record.features.len() > 0);

    // Convert to sequence format
    let sequence = parser.to_sequence(&record);
    assert_eq!(sequence.id, "NC_000913");
    assert_eq!(sequence.sequence.len(), 5000);
}

#[test]
fn test_parse_real_virus_genbank() {
    let parser = GenBankParser::new();

    // Skip if file doesn't exist (for CI environments)
    let content = match fs::read_to_string("tests/data/sample_virus.gb") {
        Ok(content) => content,
        Err(_) => {
            println!("Warning: Virus GenBank file not found, skipping test");
            return;
        }
    };

    let record = parser
        .parse(&content)
        .expect("Failed to parse virus GenBank");

    assert_eq!(record.accession, "NC_001802");
    assert!(record.definition.contains("Human immunodeficiency virus"));
    assert!(record.sequence.len() > 0);
    assert!(record.features.len() > 0);

    // Convert to sequence format
    let sequence = parser.to_sequence(&record);
    assert_eq!(sequence.id, "NC_001802");
    assert!(sequence.sequence.len() > 8000); // HIV genome is ~9kb
}
