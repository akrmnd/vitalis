export interface BasicStats {
  length: number;
  gc_percent: number;
  at_percent: number;
  n_percent: number;
  gc_skew: number;
  at_skew: number;
  entropy: number;
  complexity: number;
}

export interface BaseCount {
  a: number;
  t: number;
  g: number;
  c: number;
  n: number;
  other: number;
}

export interface CodonUsage {
  codon_counts: Record<string, number>;
  codon_frequencies: Record<string, number>;
  amino_acid_counts: Record<string, number>;
  start_codons: number;
  stop_codons: number;
  rare_codons: string[];
}

export interface QualityStats {
  mean_quality: number;
  median_quality: number;
  min_quality: number;
  max_quality: number;
  q20_bases: number;
  q30_bases: number;
  quality_distribution: Record<number, number>;
}

export interface DetailedStats {
  basic: BasicStats;
  base_counts: BaseCount;
  dinucleotide_counts: Record<string, number>;
  codon_usage?: CodonUsage;
  quality_stats?: QualityStats;
}

export interface WindowStatsItem {
  position: number;
  window_size: number;
  gc_percent: number;
  entropy: number;
}

export interface ParseResult {
  seq_id: string;
}

export interface SequenceInputData {
  content: string;
  format: 'fasta' | 'fastq';
}

export interface WindowResponse {
  bases: string;
}