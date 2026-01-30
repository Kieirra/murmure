#!/usr/bin/env bash
# docker-compile.sh - Build Murmure AppImage using Docker (Ubuntu 24.04)
# Based on .github/workflows/build-linux.yml

set -euo pipefail

# Script location and repo root (script is in COMPILE_GUIDES/ubuntu_24.04/)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

IMAGE_NAME="murmure-builder"
MODEL_DIR="$REPO_ROOT/resources/parakeet-tdt-0.6b-v3-int8"
MODEL_URL="https://github.com/Kieirra/murmure-model/releases/download/1.0.0/parakeet-tdt-0.6b-v3-int8.zip"
APPIMAGE_DIR="$REPO_ROOT/src-tauri/target/release/bundle/appimage"
APPIMAGETOOL_URL="https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage"
APPIMAGETOOL="/tmp/appimagetool-x86_64.AppImage"

# Detect whether we need sudo for docker
if docker info &>/dev/null; then
    DOCKER="docker"
else
    echo "Docker requires elevated privileges, using sudo..."
    DOCKER="sudo docker"
fi

echo "=== Murmure Docker Build ==="

# Download model if not present
if [ ! -d "$MODEL_DIR" ]; then
    echo "Downloading ONNX model..."
    TMP_ZIP="/tmp/parakeet-model.zip"
    curl -L -o "$TMP_ZIP" "$MODEL_URL"
    echo "Extracting model to resources/..."
    unzip -o "$TMP_ZIP" -d "$REPO_ROOT/resources"
    rm "$TMP_ZIP"
    echo "Model ready."
else
    echo "Model already present at $MODEL_DIR"
fi

echo ""
echo "Cleaning all caches for fresh build..."
$DOCKER volume rm -f murmure-cargo-cache murmure-pnpm-cache 2>/dev/null || true
sudo \rm -rf "$REPO_ROOT/src-tauri/target/release/bundle"
sudo \rm -rf "$REPO_ROOT/node_modules"

echo ""
echo "Building Docker image (Ubuntu 24.04)..."
$DOCKER build --no-cache -t "$IMAGE_NAME" -f "$SCRIPT_DIR/Dockerfile" "$REPO_ROOT"

echo ""
echo "Starting build inside Docker (compile only, no AppImage packaging)..."
echo "  - Source: $REPO_ROOT"
echo ""

# Build inside Docker - let Tauri create the AppDir but don't worry if AppImage packaging fails
$DOCKER run --rm \
    -v "$REPO_ROOT:/murmure" \
    -v murmure-cargo-cache:/root/.cargo/registry \
    -v murmure-pnpm-cache:/root/.local/share/pnpm \
    -e RUST_BACKTRACE=1 \
    "$IMAGE_NAME" \
    bash -c 'pnpm install && pnpm tauri build --bundles appimage -v || true'

echo ""
echo "=== Docker build phase complete ==="

# Check if AppDir was created
APPDIR="$APPIMAGE_DIR/murmure.AppDir"
if [ ! -d "$APPDIR" ]; then
    echo "ERROR: AppDir not found at $APPDIR"
    echo "Build failed before creating AppDir."
    exit 1
fi

echo "AppDir created successfully at $APPDIR"

# Check if Tauri already created the AppImage
if ls "$APPIMAGE_DIR"/*.AppImage 1>/dev/null 2>&1; then
    echo "=== Build complete (Tauri created AppImage) ==="
    ls -lh "$APPIMAGE_DIR"/*.AppImage
    exit 0
fi

# Tauri didn't create the AppImage, do it on the host
echo ""
echo "=== Packaging AppImage on host ==="

# Fix ownership (files created by Docker are owned by root)
echo "Fixing ownership..."
sudo chown -R "$(id -u):$(id -g)" "$APPIMAGE_DIR"
chmod -R u+w "$APPIMAGE_DIR"

# Fix broken symlinks created inside Docker (they point to /murmure/... which doesn't exist on host)
echo "Fixing symlinks in AppDir..."
cd "$APPDIR"
for link in murmure.desktop .DirIcon; do
    if [ -L "$link" ]; then
        target=$(readlink "$link")
        # Convert absolute /murmure path to relative
        relative_target=$(echo "$target" | sed 's|.*/murmure.AppDir/||')
        rm "$link"
        ln -s "$relative_target" "$link"
    fi
done
cd "$REPO_ROOT"

# Download appimagetool if not present
if [ ! -x "$APPIMAGETOOL" ]; then
    echo "Downloading appimagetool..."
    curl -fSL -o "$APPIMAGETOOL" "$APPIMAGETOOL_URL"
    chmod +x "$APPIMAGETOOL"
fi

echo "Creating AppImage..."
ARCH=x86_64 "$APPIMAGETOOL" "$APPDIR" "$APPIMAGE_DIR/murmure_$(date +%Y%m%d).AppImage"

echo ""
echo "=== Build complete ==="
ls -lh "$APPIMAGE_DIR"/*.AppImage
