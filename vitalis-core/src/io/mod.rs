pub mod fasta;
pub mod fastq;

// Re-export main parsers
pub use fasta::parse_fasta;
pub use fastq::parse_fastq;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("Length mismatch: {0}")]
    LengthMismatch(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}