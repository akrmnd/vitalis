use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;
use vitalis_core::io::{parse_fasta, parse_fastq};
use vitalis_core::stats::calculate_detailed_stats;
use vitalis_core::storage::SequenceStorage;

fn generate_fasta(length: usize) -> String {
    let mut result = String::new();
    result.push_str(">test_sequence Generated test sequence\n");

    let bases = ['A', 'T', 'C', 'G'];
    let mut sequence = String::new();
    for i in 0..length {
        sequence.push(bases[i % 4]);
        if (i + 1) % 80 == 0 {
            sequence.push('\n');
        }
    }

    result.push_str(&sequence);
    result
}

fn generate_fastq(length: usize) -> String {
    let mut result = String::new();
    result.push_str("@test_read Generated test read\n");

    let bases = ['A', 'T', 'C', 'G'];
    let mut sequence = String::new();
    for i in 0..length {
        sequence.push(bases[i % 4]);
    }
    result.push_str(&sequence);
    result.push_str("\n+\n");

    // Generate quality scores
    let quality = "I".repeat(length);
    result.push_str(&quality);
    result.push('\n');

    result
}

fn bench_fasta_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("fasta_parsing");
    group.measurement_time(Duration::from_secs(10));

    for size in [1000, 10000, 100000].iter() {
        let fasta_content = generate_fasta(*size);

        group.bench_with_input(
            BenchmarkId::new("parse_fasta", size),
            &fasta_content,
            |b, content| {
                b.iter(|| {
                    let result = parse_fasta(black_box(content));
                    black_box(result)
                })
            },
        );
    }
    group.finish();
}

fn bench_fastq_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("fastq_parsing");
    group.measurement_time(Duration::from_secs(10));

    for size in [1000, 10000, 100000].iter() {
        let fastq_content = generate_fastq(*size);

        group.bench_with_input(
            BenchmarkId::new("parse_fastq", size),
            &fastq_content,
            |b, content| {
                b.iter(|| {
                    let result = parse_fastq(black_box(content));
                    black_box(result)
                })
            },
        );
    }
    group.finish();
}

fn bench_stats_calculation(c: &mut Criterion) {
    let mut group = c.benchmark_group("stats_calculation");
    group.measurement_time(Duration::from_secs(10));

    for size in [1000, 10000, 100000].iter() {
        let sequence = "ATCG".repeat(*size / 4);

        group.bench_with_input(
            BenchmarkId::new("detailed_stats", size),
            &sequence,
            |b, seq| {
                b.iter(|| {
                    let result = calculate_detailed_stats(black_box(seq));
                    black_box(result)
                })
            },
        );
    }
    group.finish();
}

fn bench_window_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("window_access");
    group.measurement_time(Duration::from_secs(10));

    // Set up storage with test sequence
    let mut storage = SequenceStorage::new();
    let test_sequence = "ATCG".repeat(25000); // 100KB sequence
    let fasta_content = format!(">test\n{}", test_sequence);
    let seq_id = storage.import_from_text(&fasta_content, "fasta").unwrap();

    group.bench_function("get_window_1kb", |b| {
        b.iter(|| {
            let result = storage.get_window(black_box(&seq_id), black_box(0), black_box(1000));
            black_box(result)
        })
    });

    group.bench_function("get_window_10kb", |b| {
        b.iter(|| {
            let result = storage.get_window(black_box(&seq_id), black_box(0), black_box(10000));
            black_box(result)
        })
    });

    group.finish();
}

fn bench_integration_test(c: &mut Criterion) {
    let mut group = c.benchmark_group("integration");
    group.measurement_time(Duration::from_secs(10));

    // This is our target: 100KB FASTA parsing + import < 400ms
    let large_fasta = generate_fasta(100000);

    group.bench_function("100kb_fasta_parse_and_import", |b| {
        b.iter(|| {
            let mut storage = SequenceStorage::new();
            let result = storage.import_from_text(black_box(&large_fasta), "fasta");
            black_box(result)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_fasta_parsing,
    bench_fastq_parsing,
    bench_stats_calculation,
    bench_window_access,
    bench_integration_test
);
criterion_main!(benches);
