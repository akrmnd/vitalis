pub mod io;
pub mod commands;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Topology {
    Linear,
    Circular,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    
    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

// Re-export commands for Tauri
pub use commands::{
    parse_and_import, get_meta, get_window, stats, export,
    ImportResponse, SequenceMeta, SequenceStats, WindowResponse, ExportResponse,
};