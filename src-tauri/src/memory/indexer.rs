use std::path::Path;
use anyhow::{Result, anyhow};
use std::fs;
use uuid::Uuid;
use chrono::Utc;

use super::vector_db::MemoryChunk;

pub struct Indexer;

impl Indexer {
    pub fn chunk_text(text: &str, max_tokens: usize) -> Vec<String> {
        let paragraphs: Vec<&str> = text.split("\n\n").collect();
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        let mut current_count = 0;

        for p in paragraphs {
            let p_trimmed = p.trim();
            if p_trimmed.is_empty() { continue; }
            
            // Simple word count as token proxy (~1.5 words per token)
            let word_count = p_trimmed.split_whitespace().count();
            let token_est = (word_count as f32 * 1.5) as usize;

            if current_count + token_est > max_tokens && !current_chunk.is_empty() {
                chunks.push(current_chunk.clone());
                current_chunk = String::new();
                current_count = 0;
            }

            if !current_chunk.is_empty() {
                current_chunk.push_str("\n\n");
            }
            current_chunk.push_str(p_trimmed);
            current_count += token_est;

            // If a single paragraph is too large, just push it
            if current_count > max_tokens {
                chunks.push(current_chunk.clone());
                current_chunk = String::new();
                current_count = 0;
            }
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }

        chunks
    }

    pub fn extract_text(path: &Path) -> Result<String> {
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        match ext {
            "txt" | "md" => {
                fs::read_to_string(path).map_err(|e| anyhow!("Failed to read text file: {}", e))
            }
            "pdf" => {
                pdf_extract::extract_text(path).map_err(|e| anyhow!("Failed to extract PDF: {}", e))
            }
            _ => Err(anyhow!("Unsupported file extension: {}", ext)),
        }
    }

    /// Build MemoryChunk objects from text chunks and their embeddings.
    pub fn prepare_chunks(
        chunks: Vec<String>,
        embeddings: Vec<Vec<f32>>,
        source: &str,
    ) -> Vec<MemoryChunk> {
        let timestamp = Utc::now().to_rfc3339();
        chunks
            .into_iter()
            .zip(embeddings.into_iter())
            .enumerate()
            .map(|(i, (content, vector))| MemoryChunk {
                id: Uuid::new_v4().to_string(),
                content,
                source: source.to_string(),
                chunk_index: i as u32,
                timestamp: timestamp.clone(),
                vector,
            })
            .collect()
    }
}
