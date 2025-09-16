// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use vitalis_core::{
    parse_and_import, get_window, stats, detailed_stats, window_stats,
    import_from_file, export, get_meta, storage_info,
    ImportFromFileRequest, ExportResponse, ImportResponse
};

// Tauri command handlers - vitalis-coreのAPI関数をラップ
#[tauri::command]
async fn tauri_parse_and_import(content: String, format: String) -> Result<ImportResponse, String> {
    parse_and_import(content, format).map_err(|e| e.to_string())
}

#[tauri::command]
async fn tauri_import_from_file(request: ImportFromFileRequest) -> Result<ImportResponse, String> {
    import_from_file(request).map_err(|e| e.to_string())
}

#[tauri::command]
async fn tauri_get_window(seq_id: String, start: usize, end: usize) -> Result<vitalis_core::WindowResponse, String> {
    get_window(seq_id, start, end).map_err(|e| e.to_string())
}

#[tauri::command]
async fn tauri_stats(seq_id: String) -> Result<vitalis_core::SequenceStats, String> {
    stats(seq_id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn tauri_detailed_stats(seq_id: String) -> Result<vitalis_core::DetailedStatsResponse, String> {
    detailed_stats(seq_id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn tauri_window_stats(
    seq_id: String,
    window_size: usize,
    step: usize
) -> Result<vitalis_core::WindowStatsResponse, String> {
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

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            tauri_parse_and_import,
            tauri_import_from_file,
            tauri_get_window,
            tauri_stats,
            tauri_detailed_stats,
            tauri_window_stats,
            tauri_export,
            tauri_get_meta,
            tauri_storage_info
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