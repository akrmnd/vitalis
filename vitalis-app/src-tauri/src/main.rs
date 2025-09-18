// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use vitalis_core::application::{get_genbank_metadata, GenBankMetadata};
use vitalis_core::{
    detailed_stats, detailed_stats_enhanced, export, get_meta, get_window, import_from_file,
    import_sequence, parse_and_import, parse_preview, stats, storage_info, window_stats,
    DetailedStatsEnhancedResponse, ExportResponse, ImportFromFileRequest, ImportResponse,
    ParsePreviewResponse, WindowStatsItem,
};

// Tauri command handlers - vitalis-coreのAPI関数をラップ
#[tauri::command]
async fn tauri_parse_and_import(content: String, format: String) -> Result<ImportResponse, String> {
    parse_and_import(content, format).map_err(|e| e.to_string())
}

#[tauri::command]
async fn tauri_parse_preview(
    content: String,
    format: String,
) -> Result<ParsePreviewResponse, String> {
    parse_preview(content, format).map_err(|e| e.to_string())
}

#[tauri::command]
async fn tauri_import_sequence(
    content: String,
    format: String,
    sequence_index: usize,
) -> Result<ImportResponse, String> {
    import_sequence(content, format, sequence_index).map_err(|e| e.to_string())
}

#[tauri::command]
async fn tauri_import_from_file(request: ImportFromFileRequest) -> Result<ImportResponse, String> {
    import_from_file(request).map_err(|e| e.to_string())
}

#[tauri::command]
async fn tauri_get_window(
    seq_id: String,
    start: usize,
    end: usize,
) -> Result<vitalis_core::WindowResponse, String> {
    get_window(seq_id, start, end).map_err(|e| e.to_string())
}

#[tauri::command]
async fn tauri_stats(seq_id: String) -> Result<vitalis_core::SequenceStats, String> {
    stats(seq_id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn tauri_detailed_stats(
    seq_id: String,
) -> Result<vitalis_core::DetailedStatsResponse, String> {
    detailed_stats(seq_id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn tauri_detailed_stats_enhanced(
    seq_id: String,
) -> Result<DetailedStatsEnhancedResponse, String> {
    detailed_stats_enhanced(seq_id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn tauri_window_stats(
    seq_id: String,
    window_size: usize,
    step: usize,
) -> Result<Vec<WindowStatsItem>, String> {
    window_stats(seq_id, window_size, step).map_err(|e| e.to_string())
}

#[tauri::command]
async fn tauri_export(seq_id: String, format: String) -> Result<ExportResponse, String> {
    export(seq_id, format).map_err(|e| e.to_string())
}

#[tauri::command]
async fn tauri_get_meta(seq_id: String) -> Result<vitalis_core::SequenceMeta, String> {
    get_meta(seq_id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn tauri_storage_info() -> Result<serde_json::Value, String> {
    storage_info().map_err(|e| e.to_string())
}

#[tauri::command]
async fn tauri_read_file(file_path: String) -> Result<String, String> {
    std::fs::read_to_string(&file_path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn tauri_get_genbank_metadata(content: String) -> Result<GenBankMetadata, String> {
    get_genbank_metadata(content).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            tauri_parse_and_import,
            tauri_parse_preview,
            tauri_import_sequence,
            tauri_import_from_file,
            tauri_get_window,
            tauri_stats,
            tauri_detailed_stats,
            tauri_detailed_stats_enhanced,
            tauri_window_stats,
            tauri_export,
            tauri_get_meta,
            tauri_storage_info,
            tauri_read_file,
            tauri_get_genbank_metadata
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
