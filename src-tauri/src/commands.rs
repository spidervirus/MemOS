use tauri::State;
use crate::memory::MemoryManager;
use crate::memory::vector_db::SearchResult;
use std::sync::Arc;

pub struct AppState {
    pub manager: Arc<Option<MemoryManager>>,
}

#[tauri::command]
pub async fn start_indexing(state: State<'_, AppState>, path: String) -> Result<(), String> {
    // Verify the manager exists
    if state.manager.is_none() {
        return Err("Memory manager not initialized".to_string());
    }

    let mgr = state.manager.clone();
    let path_buf = std::path::PathBuf::from(path);

    // Run indexing on a blocking thread since it does CPU-heavy embedding work
    tokio::task::spawn_blocking(move || {
        if let Some(m) = mgr.as_ref() {
            if let Err(e) = m.index_directory(&path_buf) {
                eprintln!("Indexing error: {}", e);
            } else {
                println!("✅ Indexing complete!");
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn search_memories(state: State<'_, AppState>, query: String, top_k: usize) -> Result<Vec<SearchResult>, String> {
    let manager = state.manager.as_ref().as_ref()
        .ok_or("Memory manager not initialized")?;
    manager.search(&query, top_k).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_indexed_count(state: State<'_, AppState>) -> Result<usize, String> {
    let manager = state.manager.as_ref().as_ref()
        .ok_or("Memory manager not initialized")?;
    manager.get_count().map_err(|e| e.to_string())
}
