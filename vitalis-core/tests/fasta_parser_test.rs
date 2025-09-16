use vitalis_core::io::fasta::parse_fasta;
use std::path::PathBuf;

fn test_data_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(filename)
}

#[test]
fn test_parse_simple_fasta() {
    let path = test_data_path("simple.fasta");
    let content = std::fs::read_to_string(&path).unwrap();
    let records = parse_fasta(&content).unwrap();
    
    assert_eq!(records.len(), 6);
    
    // Test first record
    assert_eq!(records[0].id, "seq1");
    assert_eq!(records[0].description, Some("Simple DNA sequence".to_string()));
    assert_eq!(records[0].sequence, "ATCGATCGATCGATCGATCGATCGATCGATCG");
    
    // Test second record with mixed case (uppercase conversion)
    assert_eq!(records[1].id, "seq2");
    assert_eq!(records[1].description, Some("Another sequence with mixed case".to_string()));
    assert_eq!(records[1].sequence, "ATCGATCGATCGATCG"); // Mixed case -> uppercase
    
    // Test record with N bases
    assert_eq!(records[2].id, "seq3");
    assert_eq!(records[2].sequence, "ATCGATCGNNNNATCG");
    
    // Test empty description
    assert_eq!(records[3].id, "empty_description");
    assert_eq!(records[3].description, None);
    assert_eq!(records[3].sequence, "AAAAAAAA");
}

#[test]
fn test_parse_edge_cases() {
    let path = test_data_path("edge_cases.fasta");
    let content = std::fs::read_to_string(&path).unwrap();
    let records = parse_fasta(&content).unwrap();
    
    // Test empty sequence
    assert_eq!(records[0].id, "empty_sequence");
    assert_eq!(records[0].sequence, "");
    
    // Test only newlines
    assert_eq!(records[1].id, "only_newlines");
    assert_eq!(records[1].sequence, "");
    
    // Test single base
    assert_eq!(records[2].id, "single_base");
    assert_eq!(records[2].sequence, "A");
    
    // Test lowercase (should be converted to uppercase)
    assert_eq!(records[4].id, "lowercase_only");
    assert_eq!(records[4].sequence, "ATCGATCG");
    
    // Test mixed whitespace (should be removed)
    assert_eq!(records[5].id, "mixed_whitespace");
    assert_eq!(records[5].sequence, "ATCGATCGATCGATCG");
    
    // Test IUPAC ambiguity codes
    assert_eq!(records[8].id, "ambiguous_bases");
    assert_eq!(records[8].sequence, "ATCGRYSWKMBDHVN");
}

#[test]
fn test_parse_multiline_fasta() {
    let path = test_data_path("multiline.fasta");
    let content = std::fs::read_to_string(&path).unwrap();
    let records = parse_fasta(&content).unwrap();
    
    assert_eq!(records.len(), 3);
    
    // Test 60-character wrapped sequence (60*3 = 180 chars)
    assert_eq!(records[0].sequence.len(), 180);
    assert!(records[0].sequence.starts_with("ATCGATCG"));
    assert!(records[0].sequence.ends_with("ATCGATCG"));
    
    // Test 80-character wrapped sequence (80*2 = 160 chars)
    assert_eq!(records[1].sequence.len(), 160);
    
    // Test irregular wrapping
    assert_eq!(records[2].id, "irregular_wrapping");
    assert_eq!(records[2].sequence, "ATCGATCGATCGATCGATCGATCGATCGATCGATCGAT");
}

#[test]
fn test_invalid_fasta() {
    // Test missing '>' at start
    let invalid = "seq1\nATCG";
    let result = parse_fasta(invalid);
    assert!(result.is_err());
    
    // Test empty file
    let empty = "";
    let result = parse_fasta(empty);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
    
    // Test only headers, no sequences
    let headers_only = ">seq1\n>seq2\n>seq3";
    let result = parse_fasta(headers_only);
    assert!(result.is_ok());
    let records = result.unwrap();
    assert_eq!(records.len(), 3);
    assert_eq!(records[0].sequence, "");
}

#[test]
fn test_sequence_stats() {
    let path = test_data_path("simple.fasta");
    let content = std::fs::read_to_string(&path).unwrap();
    let records = parse_fasta(&content).unwrap();
    
    // Test GC content calculation
    let seq_with_n = &records[2]; // seq3 with N bases: ATCGATCGNNNNATCG
    let stats = seq_with_n.calculate_stats();
    
    assert_eq!(stats.length, 16);
    assert_eq!(stats.n_count, 4);
    // ATCGATCGNNNNATCG -> C: 3 times, G: 3 times = 6 total
    assert_eq!(stats.gc_count, 6);
    assert!((stats.gc_percent - 37.5).abs() < 0.01); // 6/16 = 37.5%
    assert!((stats.n_percent - 25.0).abs() < 0.01);  // 4/16 = 25%
}