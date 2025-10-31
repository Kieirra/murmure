# Contributing to Murmure

Contributions are welcome!

If you’d like to improve Murmure or just build the sources, you're in the right place.

## 🧭 Development Principles

We believe in building software that is **simple**, **secure**, and **respectful of users**.  
Our guiding principles:
- **Privacy first** : Never store user data, except for the last five transcriptions.  
- **Security** : No compromises, no open CORS, no unsafe shortcuts, no exceptions.  
- **Clean Code** : Code must be easy to read and maintain. Follow SRP and SOLID principles, and avoid duplication.  
- **Simplicity over complexity** : Prefer minimal, understandable solutions instead of over-engineered features.  
- **Small and focused PRs** : Keep pull requests small, focused, and easy to review.

> 🧩 *Simple, secure, and maintainable, that’s the spirit of Murmure.*

## Quick Start

### Prerequisites

- Download the Parakeet ONNX model [here](https://www.dropbox.com/scl/fi/ufc74ed80777f5oq407a7/parakeet-tdt-0.6b-v3-int8.tar.gz?rlkey=qfpfxjc0lkn0tczqhecvv4fup&st=072tatpp&dl=0)
- Place the extracted folder into `resources/parakeet-tdt-0.6b-v3-int8`
- Install all required dependencies for Tauri: [https://v2.tauri.app/fr/start/prerequisites/](https://v2.tauri.app/fr/start/prerequisites/)

### Start Murmure
```sh
pnpm install    # fetch dependencies
pnpm tauri dev  # Start a Vite dev server on http://127.0.0.1:1420/ + the Desktop app in Rust
```
## Understanding the Codebase

Murmure consists of two parts:

- A desktop app in Rust (using [Tauri](https://tauri.app/)) responsible for
  displaying the frontend, using audio primitives, and instantiating the
  Parakeet model — in the `src-tauri` directory
- A frontend in React + TypeScript as per Tauri convention — in the `src/` directory

The main flow of Murmure is the following:
1. User presses the push-to-talk shortcut
2. Murmure starts recording
3. User releases the keys
4. Murmure writes a .wav file
5. Audio is sent to Parakeet for transcription
6. Parakeet returns the transcription
7. Murmure saves the current clipboard content
8. Murmure sets the new transcription to the clipboard
9. Murmure simulates Ctrl+V to paste the transcription
10. Murmure restores the original clipboard content

For more, see [the **Tauri** documentation](https://v2.tauri.app/fr/start/), the framework Murmure is written with.

### Frontend

Murmure uses React + TypeScript + Tailwind CSS + shadcn/ui + lucide-react.
- `src/components/` : Atomic UI primitives and shadcn/ui components
- `src/features/`   : Feature-oriented pages and modules

For each feature, keep the main component at the feature root (e.g. `feature.tsx`), place subcomponents in a `components/` subfolder, and internal hooks in a `hooks/` subfolder when needed.

Components should be pure and keep markup simple; move logic to custom hooks or `*.helpers.ts` files.

### Backend 

 - `lib.rs` : Tauri app builder; initializes plugins, state, commands, shortcuts, overlay, tray, and HTTP API
 - `commands.rs` : Tauri commands exposed to the frontend (single communication layer)
 - `audio.rs` : Recording/transcription pipeline: capture audio, write WAV, run Parakeet, update history, clipboard, and paste into the active field
 - `history.rs` : Stores and manages the last 5 transcriptions (persistent) and emits updates
 - `dictionary.rs` : Post-processing using the Beider–Morse phonetic algorithm to apply custom words
 - `model.rs` : Resolves bundled Parakeet model path and checks availability
 - `overlay.rs` : Creates and manages the recording overlay (show/hide/position)
 - `settings.rs` : Loads and saves app settings (shortcuts, overlay, API) to JSON
 - `tray_icon.rs` : Creates the system tray and handles menu/click events
 - `clipboard.rs` : Clipboard integration and simulated paste; restores previous clipboard content
 - `shortcuts/` : Global keyboard shortcuts (push-to-talk, last transcript, suspend), with per-OS backends
 - `http_api/` : Local HTTP API: server lifecycle, routes, and shared state
 - `engine/` : CPU transcription engine and Parakeet runtime bindings (adapted from open source)

## Pull request

- Ensure your changes follow the Development Principles
- Test your code on Ubuntu and Windows


1. Fork the repository
2. Create a feature branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -m "Add new feature"`)
4. Push and open a pull request

## Releasing

When releasing a new version of Murmure:

1. Bump version number in <./src-tauri/Cargo.toml>
2. Rebuild to make sure version number gets updated in <./src-tauri/Cargo.lock>
3. Bump version number in <./package.json>
4. Bump version number in <./src-tauri/tauri.conf.json>
5. Use Tauri updater to bump version number and file signature in <./latest.json>
6. Add new version overview to the Changelog section in the <./README.md>
