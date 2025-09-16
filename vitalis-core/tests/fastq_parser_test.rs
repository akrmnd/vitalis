use std::path::PathBuf;
use vitalis_core::io::fastq::parse_fastq;

fn test_data_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(filename)
}

#[test]
fn test_parse_simple_fastq() {
    let path = test_data_path("simple.fastq");
    let content = std::fs::read_to_string(&path).unwrap();
    let records = parse_fastq(&content).unwrap();

    assert_eq!(records.len(), 4);

    // Test first record
    assert_eq!(records[0].id, "read1");
    assert_eq!(records[0].description, Some("Test read 1".to_string()));
    assert_eq!(records[0].sequence, "ATCGATCGATCG");
    assert_eq!(records[0].quality, "IIIIIIIIIIII");
    assert_eq!(records[0].sequence.len(), records[0].quality.len());

    // Test second record with lower quality
    assert_eq!(records[1].id, "read2");
    assert_eq!(records[1].sequence, "ATCGATCGATCGATCG");
    assert_eq!(records[1].quality, "BBBBBBBBBBBBBBBB");

    // Test record with N bases
    assert_eq!(records[2].id, "read3");
    assert_eq!(records[2].sequence, "ATCGNNNNATCG");
    assert_eq!(records[2].quality, "IIII!!!!IIII");

    // Test variable quality scores
    assert_eq!(records[3].id, "read4");
    assert_eq!(records[3].quality, "IIIHHHGGGFFF");
}

#[test]
fn test_quality_score_parsing() {
    let path = test_data_path("simple.fastq");
    let content = std::fs::read_to_string(&path).unwrap();
    let records = parse_fastq(&content).unwrap();

    // Test quality score conversion (Phred+33)
    let qual_scores = records[0].get_quality_scores();
    assert_eq!(qual_scores.len(), 12);
    assert!(qual_scores.iter().all(|&q| q == 40)); // 'I' = ASCII 73, 73-33 = 40

    let qual_scores = records[2].get_quality_scores();
    assert_eq!(qual_scores[4], 0); // '!' = ASCII 33, 33-33 = 0
}

#[test]
fn test_invalid_fastq() {
    // Test mismatched sequence and quality lengths
    let invalid = "@read1\nATCG\n+\nIII";
    let result = parse_fastq(invalid);
    assert!(result.is_err());

    // Test missing quality line
    let invalid = "@read1\nATCG\n+";
    let result = parse_fastq(invalid);
    assert!(result.is_err());

    // Test missing '+' separator
    let invalid = "@read1\nATCG\nIIII";
    let result = parse_fastq(invalid);
    assert!(result.is_err());

    // Test empty file
    let empty = "";
    let result = parse_fastq(empty);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_fastq_stats() {
    let path = test_data_path("simple.fastq");
    let content = std::fs::read_to_string(&path).unwrap();
    let records = parse_fastq(&content).unwrap();

    let stats = records[0].calculate_stats();
    assert_eq!(stats.length, 12);
    assert_eq!(stats.min_quality, 40);
    assert_eq!(stats.max_quality, 40);
    assert!((stats.mean_quality - 40.0).abs() < 0.01);

    // Test with variable quality
    let stats = records[3].calculate_stats();
    assert_eq!(stats.length, 12);
    assert!(stats.min_quality < stats.max_quality);
}

#[test]
fn test_fastq_trimming() {
    let path = test_data_path("simple.fastq");
    let content = std::fs::read_to_string(&path).unwrap();
    let mut records = parse_fastq(&content).unwrap();

    // Test quality-based trimming
    let original_len = records[2].sequence.len();
    records[2].trim_by_quality(10); // Trim bases with quality < 10
    assert!(records[2].sequence.len() < original_len);
    assert_eq!(records[2].sequence.len(), records[2].quality.len());

    // Test fixed-length trimming
    records[0].trim_to_length(8);
    assert_eq!(records[0].sequence.len(), 8);
    assert_eq!(records[0].quality.len(), 8);
    assert_eq!(records[0].sequence, "ATCGATCG");
}
