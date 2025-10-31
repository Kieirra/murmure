# Contributing to Murmure

Contributions are welcome!

If you’d like to improve Murmure, you're in the right place.

## Pre-requisite

- Download onnx [here](https://www.dropbox.com/scl/fi/ufc74ed80777f5oq407a7/parakeet-tdt-0.6b-v3-int8.tar.gz?rlkey=qfpfxjc0lkn0tczqhecvv4fup&st=072tatpp&dl=0) 
- Place files into /ressources/parakeet-tdt-0.6b-v3-int8

## Overall process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -m "Add new feature"`)
4. Push and open a pull request

## On Windows

- Install Visual Studio Build Tools 2022 (Desktop C++ workload)

## On Linux

Murmure is composed of two components:

- A desktop app in Rust (using [Tauri](https://tauri.app/)) responsible for
  displaying the frontend, using audio primitives, and instantiating the
  Parakeet model — in the `src-tauri` directory
- A frontend in React+Typescript as per Tauri convention — in the `src/` directory

Here are the steps to run them after cloning the repository:

```sh
cd murmure/
pnpm install    # fetch dependencies
pnpm tauri dev  # Start a Vite dev server on http://127.0.0.1:1420/ + the Desktop app in Rust
```

For more, see [the **Tauri** documentation](https://v2.tauri.app/fr/start/), the framework Murmure is written with.
