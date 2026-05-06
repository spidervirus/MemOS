# Contributing to MemOS

First off, thank you for considering contributing to MemOS! It's people like you that make MemOS such a great tool. 

MemOS is a local-first, privacy-centric "second brain" that indexes and semantically searches everything you've ever read. 

## Getting Started

1. **Fork the repository** on GitHub.
2. **Clone your fork** locally: `git clone https://github.com/spidervirus/MemOS.git`
3. **Install dependencies**: 
   - Node.js & npm (for the frontend)
   - Rust & Cargo (for the backend)
   - Run `npm install` in the project root.
4. **Run the development environment**:
   - Run `npm run tauri dev` to start the app in development mode.

## Good First Issues

If you're looking for a way to jump in, we have curated a list of issues perfect for new contributors. Look for the `good first issue` label on our issue tracker. 

Here are a few examples of great ways to contribute right now:

* **Add file type `.rst` support:** Currently, we support `.md`, `.txt`, and `.pdf`. Adding support for reStructuredText (`.rst`) or other text formats is a straightforward addition to the indexing pipeline (`src-tauri/src/memory/indexer.rs`).
* **UI Themes:** Help us expand our visual design by contributing new Tailwind color palettes or a dedicated light mode.
* **Keyboard Shortcuts:** Implement global hotkeys (e.g., `Cmd+Shift+Space`) to instantly bring up the MemOS search bar from anywhere on the desktop.

## Submitting a Pull Request

1. Create a new branch for your feature or bugfix: `git checkout -b feature/my-awesome-feature`
2. Make your changes, ensuring code is clean and well-commented.
3. Commit your changes: `git commit -m "Add some awesome feature"`
4. Push to your fork: `git push origin feature/my-awesome-feature`
5. Open a Pull Request against the `main` branch of the upstream repository.

## Development Guidelines

- **Privacy First:** Never introduce external telemetry, tracking, or cloud fallbacks without explicit, opt-in user consent. MemOS is local-first by design.
- **Rust Backend:** Ensure your Rust code is safe and handles errors gracefully. We rely heavily on the `anyhow` crate.
- **React Frontend:** Keep components modular and use Tailwind CSS for styling.

Thank you for helping make MemOS better!
