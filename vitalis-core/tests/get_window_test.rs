use std::io::Write;
use tempfile::NamedTempFile;
use vitalis_core::application::{
    get_window, import_from_file, parse_and_import, ImportFromFileRequest,
};

#[test]
fn test_get_window_basic() {
    let fasta_content = ">test_seq\nATCGATCGATCGATCG".to_string();
    let result = parse_and_import(fasta_content, "fasta".to_string()).unwrap();

    let window = get_window(result.seq_id, 2, 6).unwrap();
    assert_eq!(window.bases, "CGAT");
}

#[test]
fn test_get_window_boundary_conditions() {
    let fasta_content = ">test_seq\nATCGATCG".to_string();
    let result = parse_and_import(fasta_content, "fasta".to_string()).unwrap();

    // Test start at 0
    let window = get_window(result.seq_id.clone(), 0, 4).unwrap();
    assert_eq!(window.bases, "ATCG");

    // Test end at sequence length
    let window = get_window(result.seq_id.clone(), 4, 8).unwrap();
    assert_eq!(window.bases, "ATCG");

    // Test full sequence
    let window = get_window(result.seq_id, 0, 8).unwrap();
    assert_eq!(window.bases, "ATCGATCG");
}

#[test]
fn test_get_window_invalid_ranges() {
    let fasta_content = ">test_seq\nATCGATCG".to_string();
    let result = parse_and_import(fasta_content, "fasta".to_string()).unwrap();

    // Test start >= sequence length
    let result_err = get_window(result.seq_id.clone(), 8, 10);
    assert!(result_err.is_err());
    let error_msg = result_err.unwrap_err();
    assert!(error_msg.contains("Invalid range"));

    // Test start >= end (should return empty)
    let window = get_window(result.seq_id.clone(), 5, 5).unwrap();
    assert_eq!(window.bases, "");

    // Test start > end (should return empty)
    let window = get_window(result.seq_id, 6, 4).unwrap();
    assert_eq!(window.bases, "");
}

#[test]
fn test_get_window_end_exceeds_length() {
    let fasta_content = ">test_seq\nATCGATCG".to_string();
    let result = parse_and_import(fasta_content, "fasta".to_string()).unwrap();

    // Test end > sequence length (should clamp to sequence length)
    let window = get_window(result.seq_id, 4, 20).unwrap();
    assert_eq!(window.bases, "ATCG");
}

#[test]
fn test_get_window_nonexistent_sequence() {
    let result = get_window("nonexistent_seq".to_string(), 0, 4);
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("not found") || error_msg.contains("Sequence not found"));
}

#[test]
fn test_get_window_large_sequence_memory() {
    // Create a large sequence (1000 bases) in memory
    let large_seq = "A".repeat(500) + &"T".repeat(500);
    let fasta_content = format!(">large_seq\n{}", large_seq);
    let result = parse_and_import(fasta_content, "fasta".to_string()).unwrap();

    // Test various windows
    let window = get_window(result.seq_id.clone(), 0, 100).unwrap();
    assert_eq!(window.bases, "A".repeat(100));

    let window = get_window(result.seq_id.clone(), 250, 350).unwrap();
    assert_eq!(window.bases, "A".repeat(100));

    let window = get_window(result.seq_id.clone(), 900, 1000).unwrap();
    assert_eq!(window.bases, "T".repeat(100));

    // Test overlapping windows
    let window1 = get_window(result.seq_id.clone(), 490, 510).unwrap();
    let window2 = get_window(result.seq_id.clone(), 500, 520).unwrap();
    assert_eq!(window1.bases, "A".repeat(10) + &"T".repeat(10));
    assert_eq!(window2.bases, "T".repeat(20));
}

#[test]
fn test_get_window_file_based_sequence() {
    // Create a temporary FASTA file with multiple lines
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, ">file_seq Test sequence from file").unwrap();
    writeln!(temp_file, "ATCGATCGATCGATCG").unwrap(); // 16 bases
    writeln!(temp_file, "GCTAGCTAGCTAGCTA").unwrap(); // 16 bases
    writeln!(temp_file, "TTAATTAATTAATTAA").unwrap(); // 16 bases
                                                      // Total: 48 bases

    let request = ImportFromFileRequest {
        file_path: temp_file.path().to_string_lossy().to_string(),
        format: "fasta".to_string(),
    };

    let result = import_from_file(request).unwrap();

    // Test various windows across line boundaries
    let window = get_window(result.seq_id.clone(), 0, 8).unwrap();
    assert_eq!(window.bases, "ATCGATCG");

    let window = get_window(result.seq_id.clone(), 14, 18).unwrap();
    assert_eq!(window.bases, "CGGC"); // Cross line boundary

    let window = get_window(result.seq_id.clone(), 16, 32).unwrap();
    assert_eq!(window.bases, "GCTAGCTAGCTAGCTA"); // Exactly second line

    let window = get_window(result.seq_id.clone(), 30, 40).unwrap();
    assert_eq!(window.bases, "TATTAATTAA"); // Cross to third line

    let window = get_window(result.seq_id, 40, 48).unwrap();
    assert_eq!(window.bases, "TTAATTAA"); // End of sequence
}

#[test]
fn test_get_window_multiline_sequences() {
    // Test sequences that span multiple lines within the file
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, ">multiline_seq").unwrap();
    writeln!(temp_file, "AAAAAAAAAA").unwrap(); // 10 A's
    writeln!(temp_file, "TTTTTTTTTT").unwrap(); // 10 T's
    writeln!(temp_file, "GGGGGGGGGG").unwrap(); // 10 G's
    writeln!(temp_file, "CCCCCCCCCC").unwrap(); // 10 C's
                                                // Total: 40 bases

    let request = ImportFromFileRequest {
        file_path: temp_file.path().to_string_lossy().to_string(),
        format: "fasta".to_string(),
    };

    let result = import_from_file(request).unwrap();

    // Test windows that span multiple lines
    let window = get_window(result.seq_id.clone(), 5, 15).unwrap();
    assert_eq!(window.bases, "AAAAATTTTT"); // A's to T's

    let window = get_window(result.seq_id.clone(), 18, 32).unwrap();
    assert_eq!(window.bases, "TTGGGGGGGGGGCC"); // T's to G's to C's
}

#[test]
fn test_get_window_performance_large_file() {
    // Create a larger file to test performance
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, ">perf_test").unwrap();

    // Write 100 lines of 80 characters each = 8000 bases
    for i in 0..100 {
        let base = match i % 4 {
            0 => 'A',
            1 => 'T',
            2 => 'G',
            _ => 'C',
        };
        writeln!(temp_file, "{}", base.to_string().repeat(80)).unwrap();
    }

    let request = ImportFromFileRequest {
        file_path: temp_file.path().to_string_lossy().to_string(),
        format: "fasta".to_string(),
    };

    let result = import_from_file(request).unwrap();

    // Test random access patterns
    let start = std::time::Instant::now();

    // Small windows
    let _window = get_window(result.seq_id.clone(), 100, 200).unwrap();
    let _window = get_window(result.seq_id.clone(), 1000, 1100).unwrap();
    let _window = get_window(result.seq_id.clone(), 5000, 5100).unwrap();

    // Large windows
    let _window = get_window(result.seq_id.clone(), 0, 1000).unwrap();
    let _window = get_window(result.seq_id.clone(), 2000, 4000).unwrap();

    let elapsed = start.elapsed();

    // Should complete within reasonable time (< 100ms for this size)
    assert!(
        elapsed.as_millis() < 100,
        "Window access too slow: {:?}",
        elapsed
    );
}

#[test]
fn test_get_window_special_characters() {
    let fasta_content = ">test_seq\nATCGNNNNatcgXYZ".to_string();
    let result = parse_and_import(fasta_content, "fasta".to_string()).unwrap();

    // Test that lowercase letters are handled (should be converted to uppercase)
    let window = get_window(result.seq_id.clone(), 8, 12).unwrap();
    assert_eq!(window.bases, "ATCG");

    // Test that N's and other characters are preserved
    let window = get_window(result.seq_id, 4, 8).unwrap();
    assert_eq!(window.bases, "NNNN");
}

#[test]
fn test_get_window_empty_sequence() {
    let fasta_content = ">empty_seq\n".to_string();
    let result = parse_and_import(fasta_content, "fasta".to_string()).unwrap();

    // Any window request on empty sequence should fail
    let result_err = get_window(result.seq_id, 0, 1);
    assert!(result_err.is_err());
}

#[test]
fn test_get_window_consistency_memory_vs_file() {
    let sequence = "ATCGATCGATCGATCGATCGATCGATCGATCG"; // 32 bases

    // Test with memory-based storage
    let fasta_memory = format!(">mem_seq\n{}", sequence);
    let mem_result = parse_and_import(fasta_memory, "fasta".to_string()).unwrap();

    // Test with file-based storage
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, ">file_seq").unwrap();
    writeln!(temp_file, "{}", sequence).unwrap();

    let request = ImportFromFileRequest {
        file_path: temp_file.path().to_string_lossy().to_string(),
        format: "fasta".to_string(),
    };
    let file_result = import_from_file(request).unwrap();

    // Test same windows on both
    let test_cases = vec![(0, 8), (8, 16), (16, 24), (24, 32), (5, 15), (10, 20)];

    for (start, end) in test_cases {
        let mem_window = get_window(mem_result.seq_id.clone(), start, end).unwrap();
        let file_window = get_window(file_result.seq_id.clone(), start, end).unwrap();

        assert_eq!(
            mem_window.bases, file_window.bases,
            "Mismatch at window [{}, {})",
            start, end
        );
    }
}
