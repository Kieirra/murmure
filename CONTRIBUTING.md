# Contributing to Murmure

Contributions are welcome!

If you’d like to improve Murmure or just build the sources, you're in the right place.

## 🧭 Development Principles

For specific coding conventions (naming, file structure, error handling...), please consult the [**GUIDELINES.md**](GUIDELINES.md) file.

We believe in building software that is **simple**, **secure**, and **respectful of users**.  
Our guiding principles:

- **Privacy first** : Never store user data, except for the last five transcriptions.
- **Security** : No compromises, no open CORS, no unsafe shortcuts, no exceptions.
- **Clean Code** : Code must be easy to read and maintain. Follow SRP and SOLID principles, and avoid duplication.
- **Simplicity over complexity** : Prefer minimal, understandable solutions instead of over-engineered features.
- **Small and focused PRs** : Keep pull requests small, focused, and easy to review.

> 🧩 _Simple, secure, and maintainable, that’s the spirit of Murmure._

## Quick Start

### Prerequisites

- Download the Parakeet ONNX model [here](https://github.com/Kieirra/murmure-model/releases/download/1.0.0/parakeet-tdt-0.6b-v3-int8.zip)
- Place the extracted folder into `resources/parakeet-tdt-0.6b-v3-int8`
- Install all required dependencies for Tauri: [https://v2.tauri.app/fr/start/prerequisites/](https://v2.tauri.app/fr/start/prerequisites/)

### Start Murmure

```sh
pnpm install    # fetch dependencies
pnpm tauri dev  # Start a Vite dev server on http://127.0.0.1:1420/ + the Desktop app in Rust
```

## Development Workflow

1. **Pick one small feature/issue**
2. **Create a fresh branch from main** (don't carry over other changes)
3. **Write the minimum code necessary** that works and respects all Murmure principles
4. **Test it manually** (does it work as expected? Does it look good?)
5. **Review your own code** (do you understand every line? Is there a simpler way?)
6. **Run clippy and fmt** to catch Rust issues
7. **Create a draft PR** and check for SonarQube issues (fix them before requesting review)
8. **Mark it ready for review** once everything is clean
9. **After merge**, delete your branch and start fresh from main for the next feature

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
- `src/features/` : Feature-oriented pages and modules

For each feature, keep the main component at the feature root (e.g. `feature.tsx`), place subcomponents in a `components/` subfolder, and internal hooks in a `hooks/` subfolder when needed.

Components should be pure and keep markup simple; move logic to custom hooks or `*.helpers.ts` files.

### Translations (i18n)

- Murmure uses i18next with English sentences as keys (no technical keys).  
  Punctuation matters; changing the sentence changes the key.
- Where to translate: `src/i18n/locales/{locale}.json`

Add or update a string:

1. Use the key directly in React:
    ```tsx
    import { useTranslation } from 'react-i18next';
    const { t } = useTranslation();
    // Example
    t('Check for updates');
    ```
2. Extract keys:
    ```sh
    pnpm i18n:extract
    ```
3. Open `src/i18n/locales/{locale}.json` and provide/adjust the {locale} value(s).

Rename a key:

- Update the English sentence in code, then run `pnpm i18n:extract`.  
  Remove any obsolete entries from `{locale}.json` if they remain.

Interpolation:

- Use `{{variable}}` in both the key (English sentence) and translation value.

Notes:

- The extractor is configured to keep keys flat (no key/namespace separators).  
  Do not introduce nested objects in locale files.

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


## Community Release

I currently manage the official releases of Murmure for Windows and Linux (X11 Debian AppImage builds).

For community builds (Arch Linux, macOS, etc.), you can fork the main Murmure repository, add a release section similar to the one in the official repo, and adapt the GitHub Actions workflow here: [Murmure Workflows](https://github.com/Kieirra/murmure/tree/main/.github/workflows).

Please define an `OS_PLATFORM` variable in `main.rs` so that I can identify your build as a community version.  
This will later allow the “Check for updates” feature to point to your own community repository releases instead of the official ones.
