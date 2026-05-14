# tests-e2e

End to end harness for Murmure. Covers a single scenario: the golden path
(record shortcut press, WAV injected, transcript contains "murmure").

## Layout

This directory is intentionally **isolated** from the pnpm workspace at the
repo root. It owns its own `node_modules` (installed via `npm`, not `pnpm`) so
the production build never pulls in WebDriverIO and friends.

## Prerequisites (local Linux)

- `cargo`, `node` (>= 18), `npm`
- System packages: `webkit2gtk-driver`, `xvfb` (apt: `sudo apt install webkit2gtk-driver xvfb`)
- `tauri-driver` available on PATH (`cargo install tauri-driver --locked`)
- A Parakeet ONNX model unzipped at `resources/parakeet-tdt-0.6b-v3-int8/`
  (same path as `ci.yaml` uses).
- To regenerate the fixture only: `piper-tts` and the `fr_FR-siwis-medium`
  voice (see `fixtures/generate.sh`).

## Run from the repo root

```bash
pnpm test:e2e
```

This builds `src-tauri` with `--features audio-injection`, installs the
isolated npm deps, and runs the wdio scenario under `xvfb-run`.

## Regenerate the WAV fixture

The fixture `fixtures/sample.wav` is committed for bit for bit reproducibility.
To regenerate it locally:

```bash
cd tests-e2e/fixtures
# One-time setup: download the voice model + config files alongside generate.sh
# (see the comment block at the top of generate.sh).
./generate.sh
```

## Scope

This harness covers exactly one scenario, the golden path. Other scenarios
(clipboard paste, LLM modes, wake word, cancel, multi language, etc.) are
tracked as separate user stories.
