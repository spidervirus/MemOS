pub mod memory;
pub mod commands;

use std::sync::Arc;
use tauri::Manager;
use crate::commands::{AppState, start_indexing, search_memories, get_indexed_count};
use crate::memory::MemoryManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let handle = app.handle().clone();

            let app_dir = handle.path().app_data_dir().expect("Failed to get app data dir");
            if !app_dir.exists() {
                std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");
            }

            // Path to models — check resources first, fallback to dev location
            let resource_dir = handle.path().resource_dir().expect("Failed to get resource dir");
            let model_path = resource_dir.join("models").join("model.onnx");
            let tokenizer_path = resource_dir.join("models").join("tokenizer.json");

            let (m_path, t_path) = if model_path.exists() {
                (model_path, tokenizer_path)
            } else {
                let dev_model = std::env::current_dir().unwrap().join("models").join("model.onnx");
                let dev_tok = std::env::current_dir().unwrap().join("models").join("tokenizer.json");
                (dev_model, dev_tok)
            };

            let manager = match MemoryManager::new(&app_dir, &m_path, &t_path) {
                Ok(m) => {
                    println!("✅ Memory Manager initialized successfully");
                    Some(m)
                }
                Err(e) => {
                    eprintln!("❌ Failed to initialize Memory Manager: {}", e);
                    None
                }
            };

            let app_state = AppState {
                manager: Arc::new(manager),
            };
            app.manage(app_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_indexing,
            search_memories,
            get_indexed_count
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
