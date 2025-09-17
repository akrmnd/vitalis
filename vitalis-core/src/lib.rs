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
    detailed_stats, detailed_stats_enhanced, export, get_meta, get_window, import_from_file,
    parse_and_import, stats, storage_info, window_stats, DetailedStatsEnhancedResponse,
    DetailedStatsResponse, ExportResponse, ImportFromFileRequest, ImportResponse, SequenceMeta,
    SequenceStats, WindowResponse, WindowStatsItem, WindowStatsResponse,
};
