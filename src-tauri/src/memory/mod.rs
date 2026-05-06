pub mod embeddings;
pub mod vector_db;
pub mod indexer;

use std::path::Path;
use std::sync::Mutex;
use anyhow::Result;
use self::embeddings::EmbeddingModel;
use self::vector_db::{VectorStore, SearchResult};
use self::indexer::Indexer;

pub struct MemoryManager {
    store: Mutex<VectorStore>,
    model: EmbeddingModel,
}

impl MemoryManager {
    pub fn new(data_dir: &Path, model_path: &Path, tokenizer_path: &Path) -> Result<Self> {
        let store = VectorStore::open(data_dir)?;
        let model = EmbeddingModel::new(model_path, tokenizer_path)?;

        Ok(Self {
            store: Mutex::new(store),
            model,
        })
    }

    pub fn index_file(&self, path: &Path) -> Result<()> {
        let text = Indexer::extract_text(path)?;
        let chunks = Indexer::chunk_text(&text, 200);
        
        let mut embeddings = Vec::new();
        for chunk in &chunks {
            let emb = self.model.generate_embedding(chunk)?;
            embeddings.push(emb);
        }

        let memory_chunks = Indexer::prepare_chunks(
            chunks,
            embeddings,
            path.to_str().unwrap_or("unknown"),
        );

        let mut store = self.store.lock().unwrap();
        store.insert(memory_chunks)?;
        Ok(())
    }

    pub fn index_directory(&self, dir_path: &Path) -> Result<()> {
        for entry in walkdir::WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            if matches!(ext, "txt" | "md" | "pdf") {
                println!("Indexing: {:?}", path);
                if let Err(e) = self.index_file(path) {
                    eprintln!("Failed to index {:?}: {}", path, e);
                }
            }
        }
        Ok(())
    }

    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let query_vector = self.model.generate_embedding(query)?;
        let store = self.store.lock().unwrap();
        Ok(store.search(&query_vector, limit))
    }

    pub fn get_count(&self) -> Result<usize> {
        let store = self.store.lock().unwrap();
        Ok(store.count())
    }
}
