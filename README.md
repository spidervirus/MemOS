# MemOS: Never forget a thought.

MemOS is a local-first, privacy-centric "second brain" that indexes and semantically searches everything you've ever read or written. 

No clouds. No subscriptions. Just your knowledge, instantly accessible.

![MemOS Search Demo](https://raw.githubusercontent.com/tauri-apps/tauri/dev/app-icon.png) *(Note: Add a real GIF demonstrating the semantic search and folder indexing here)*

## Features

- **Semantic Search:** Find documents by meaning, not just exact keywords. Powered by local ONNX models (`all-MiniLM-L6-v2`).
- **Privacy First:** All data is stored locally in a high-performance vector store. No data ever leaves your machine.
- **Multiple File Types:** Supports Markdown (`.md`), plain text (`.txt`), and PDFs (`.pdf`).
- **Blazing Fast:** Built with Rust and Tauri for native desktop performance.

## Installation

### Windows
1. Go to the [Releases](https://github.com/your-username/memos/releases) page.
2. Download the `.msi` or `.exe` installer for the latest version.
3. Run the installer.

### macOS (Apple Silicon & Intel)
1. Go to the [Releases](https://github.com/your-username/memos/releases) page.
2. Download the `.dmg` file for the latest version.
3. Open the `.dmg` and drag the MemOS app to your Applications folder.
*(Note: Releases are notarized by Apple)*

### Linux
1. Go to the [Releases](https://github.com/your-username/memos/releases) page.
2. Download the `.AppImage` or `.deb` file.
3. If using the AppImage, make it executable (`chmod +x MemOS-*.AppImage`) and run it.

## Quick Start

1. Open MemOS.
2. Click **Select Folder** and choose the `sample_docs` folder included in this repository, or any directory containing your notes and PDFs.
3. Wait for the indexing to complete. 
4. Start typing in the search bar. Try searching for "offline access" or "memory management". MemOS will find the relevant documents even if you don't use those exact words!

## Development

Want to build MemOS from source or contribute?

### Prerequisites
- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/)

### Setup
```bash
git clone https://github.com/your-username/memos.git
cd memos
npm install
npm run tauri dev
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for more details on how to get involved!

## License

MIT License. See `LICENSE` for more information.
