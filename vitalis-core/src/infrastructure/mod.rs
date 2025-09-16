// Infrastructure layer - 外部依存の具体実装
pub mod parsers;
pub mod storage;

pub use parsers::{FastaParser, FastqParser};
pub use storage::FileSequenceRepository;
