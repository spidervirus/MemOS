use std::fs;
use std::path::Path;
use std::io::copy;

fn main() {
    let out_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let model_dir = Path::new(&out_dir).join("models");
    let model_path = model_dir.join("model.onnx");

    if !model_dir.exists() {
        fs::create_dir_all(&model_dir).unwrap();
    }

    if !model_path.exists() {
        println!("cargo:warning=Downloading quantized embedding model... (~22MB)");
        let url = "https://huggingface.co/Xenova/all-MiniLM-L6-v2/resolve/main/onnx/model_quantized.onnx";
        let mut response = reqwest::blocking::get(url).expect("Failed to download model");
        let mut dest = fs::File::create(&model_path).expect("Failed to create model file");
        copy(&mut response, &mut dest).expect("Failed to copy model content");
    }

    let tokenizer_path = model_dir.join("tokenizer.json");
    if !tokenizer_path.exists() {
        println!("cargo:warning=Downloading tokenizer... ");
        let url = "https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main/tokenizer.json";
        let mut response = reqwest::blocking::get(url).expect("Failed to download tokenizer");
        let mut dest = fs::File::create(tokenizer_path).expect("Failed to create tokenizer file");
        copy(&mut response, &mut dest).expect("Failed to copy tokenizer content");
    }


    tauri_build::build();
}
