// Infrastructure layer - 外部依存の具体実装
pub mod genbank_parser;
pub mod parsers;
pub mod storage;

pub use genbank_parser::{GenBankFeature, GenBankParser, GenBankRecord};
pub use parsers::{FastaParser, FastqParser};
pub use storage::FileSequenceRepository;
