// Layered Architecture modules
pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod services;

// Legacy modules for backward compatibility (will be phased out)
pub mod io;
pub mod stats;
pub mod storage;

// Re-export domain types for public API
pub use domain::{BaseCount, DetailedStats, Range, Topology, WindowStats};

// Re-export application layer commands for Tauri
pub use application::{
    detailed_stats, detailed_stats_enhanced, export, get_genbank_metadata, get_meta, get_window,
    import_from_file, import_sequence, parse_and_import, parse_preview, stats, storage_info,
    window_stats, design_primers, calculate_primer_tm, calculate_primer_gc, evaluate_primer_multiplex,
    DetailedStatsEnhancedResponse, DetailedStatsResponse, ExportResponse,
    GenBankFeatureInfo, GenBankMetadata, ImportFromFileRequest, ImportResponse,
    ParsePreviewResponse, SequenceInfo, SequenceMeta, SequenceStats, WindowResponse,
    WindowStatsItem, WindowStatsResponse,
};
