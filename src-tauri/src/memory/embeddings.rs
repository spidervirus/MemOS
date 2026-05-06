use ort::session::Session;
use ort::value::Tensor;
use tokenizers::Tokenizer;
use anyhow::{Result, anyhow};
use std::path::Path;
use std::sync::Mutex;

pub struct EmbeddingModel {
    session: Mutex<Session>,
    tokenizer: Tokenizer,
}

impl EmbeddingModel {
    pub fn new(model_path: &Path, tokenizer_path: &Path) -> Result<Self> {
        let session = Session::builder()
            .map_err(|e| anyhow!("Failed to create session builder: {}", e))?
            .with_intra_threads(4)
            .map_err(|e| anyhow!("Failed to set threads: {}", e))?
            .commit_from_file(model_path)
            .map_err(|e| anyhow!("Failed to load ONNX model: {}", e))?;

        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| anyhow!("Failed to load tokenizer: {}", e))?;

        Ok(Self {
            session: Mutex::new(session),
            tokenizer,
        })
    }

    pub fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        let encoding = self.tokenizer.encode(text, true)
            .map_err(|e| anyhow!("Tokenization error: {}", e))?;

        let input_ids: Vec<i64> = encoding.get_ids().iter().map(|&id| id as i64).collect();
        let attention_mask: Vec<i64> = encoding.get_attention_mask().iter().map(|&m| m as i64).collect();
        let token_type_ids: Vec<i64> = encoding.get_type_ids().iter().map(|&id| id as i64).collect();

        let seq_len = input_ids.len();

        let input_ids_tensor = Tensor::from_array(([1usize, seq_len], input_ids))
            .map_err(|e| anyhow!("Failed to create input_ids tensor: {}", e))?;
        let attention_mask_tensor = Tensor::from_array(([1usize, seq_len], attention_mask))
            .map_err(|e| anyhow!("Failed to create attention_mask tensor: {}", e))?;
        let token_type_ids_tensor = Tensor::from_array(([1usize, seq_len], token_type_ids))
            .map_err(|e| anyhow!("Failed to create token_type_ids tensor: {}", e))?;

        let mut session = self.session.lock().unwrap();
        let outputs = session.run(ort::inputs![
            "input_ids" => input_ids_tensor,
            "attention_mask" => attention_mask_tensor,
            "token_type_ids" => token_type_ids_tensor,
        ]).map_err(|e| anyhow!("ONNX inference failed: {}", e))?;

        let binding = &outputs["last_hidden_state"];
        let (shape, raw_data) = binding.try_extract_tensor::<f32>()
            .map_err(|e| anyhow!("Failed to extract tensor: {}", e))?;

        // shape dimensions: [batch_size=1, sequence_length, hidden_size=384]
        let dims: Vec<usize> = shape.iter().map(|&d| d as usize).collect();
        let sequence_length = dims[1];
        let hidden_size = dims[2];

        // Average pooling across the sequence dimension
        let mut embedding = vec![0.0f32; hidden_size];
        for i in 0..sequence_length {
            let offset = i * hidden_size;
            for j in 0..hidden_size {
                embedding[j] += raw_data[offset + j];
            }
        }
        for val in embedding.iter_mut() {
            *val /= sequence_length as f32;
        }

        // L2 normalization
        let norm = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for val in embedding.iter_mut() {
                *val /= norm;
            }
        }

        Ok(embedding)
    }
}
