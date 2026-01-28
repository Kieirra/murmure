# Ubuntu 22.04 Build

Docker-based build environment for creating Murmure AppImages compatible with Ubuntu 22.04's glibc (2.35).

## Why?

The official Murmure AppImage requires glibc 2.36+, which is newer than what Ubuntu 22.04 ships (glibc 2.35). Official glibc 2.35 builds were removed for security reasons.

This build uses Docker to compile Murmure inside Ubuntu 22.04, ensuring the resulting binary links against glibc 2.35 and runs on older systems.

## Usage

From the root of the repository:

```bash
./COMPILE_GUIDES/ubuntu_22.04/docker-compile.sh
```

The script will:
1. Download the ONNX model if not present
2. Build a Docker image with all dependencies
3. Compile Murmure inside the container
4. Create an AppImage (either via Tauri or using appimagetool on the host)

Output: `src-tauri/target/release/bundle/appimage/*.AppImage`

## Requirements

- Docker
- `curl`, `unzip` (for model download)
- `sudo` access (for Docker if not in docker group, and for file ownership fixes)

---

*Created with Claude Code.*
