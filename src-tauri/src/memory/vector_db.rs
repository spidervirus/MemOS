use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};

/// A single indexed chunk stored on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryChunk {
    pub id: String,
    pub content: String,
    pub source: String,
    pub chunk_index: u32,
    pub timestamp: String,
    pub vector: Vec<f32>,
}

/// Search result returned to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub content: String,
    pub source: String,
    pub score: f32,
}

/// A lightweight, file-backed vector store.
/// Stores all chunks in a single bincode file.
/// Uses brute-force cosine similarity — fast enough for <500k chunks.
pub struct VectorStore {
    path: PathBuf,
    chunks: Vec<MemoryChunk>,
}

impl VectorStore {
    /// Open or create the store at `dir/memories.bin`.
    pub fn open(dir: &Path) -> Result<Self> {
        let path = dir.join("memories.bin");
        let chunks = if path.exists() {
            let data = fs::read(&path).context("Failed to read vector store")?;
            bincode::deserialize(&data).unwrap_or_default()
        } else {
            Vec::new()
        };
        Ok(Self { path, chunks })
    }

    /// Persist the current state to disk.
    pub fn save(&self) -> Result<()> {
        let data = bincode::serialize(&self.chunks).context("Failed to serialize vector store")?;
        fs::write(&self.path, data).context("Failed to write vector store")?;
        Ok(())
    }

    /// Insert a batch of chunks and persist.
    pub fn insert(&mut self, new_chunks: Vec<MemoryChunk>) -> Result<()> {
        self.chunks.extend(new_chunks);
        self.save()
    }

    /// Brute-force cosine similarity search.
    pub fn search(&self, query_vector: &[f32], limit: usize) -> Vec<SearchResult> {
        if self.chunks.is_empty() {
            return Vec::new();
        }

        let mut scored: Vec<(&MemoryChunk, f32)> = self
            .chunks
            .iter()
            .map(|chunk| {
                let score = cosine_similarity(query_vector, &chunk.vector);
                (chunk, score)
            })
            .collect();

        // Sort descending by score
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        scored
            .into_iter()
            .take(limit)
            .map(|(chunk, score)| SearchResult {
                id: chunk.id.clone(),
                content: chunk.content.clone(),
                source: chunk.source.clone(),
                score,
            })
            .collect()
    }

    /// Total number of indexed chunks.
    pub fn count(&self) -> usize {
        self.chunks.len()
    }
}

/// Cosine similarity between two vectors.
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot / (norm_a * norm_b)
}
