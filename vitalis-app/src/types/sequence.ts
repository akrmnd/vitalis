export interface SequenceStats {
  length: number;
  gc_percent: number;
  at_percent: number;
  n_percent: number;
}

export interface ParseResult {
  sequence_id: string;
}

export interface SequenceInputData {
  content: string;
  format: 'fasta' | 'fastq';
}