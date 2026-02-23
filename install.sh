#!/bin/sh
set -eu

# 1. Vérifier l'architecture
ARCH=$(uname -m)
if [ "$ARCH" != "x86_64" ]; then
    echo "Error: Murmure only supports x86_64 architecture. Detected: $ARCH" >&2
    exit 1
fi

# 2. Vérifier dpkg
if ! command -v dpkg >/dev/null 2>&1; then
    echo "Error: dpkg not found. This installer supports Debian/Ubuntu only." >&2
    echo "For other distributions, download the AppImage from:" >&2
    echo "  https://github.com/Kieirra/murmure/releases" >&2
    exit 1
fi

# 3. Récupérer la dernière version via l'API GitHub
echo "Fetching latest Murmure version..."
LATEST_URL=$(curl -fsSo /dev/null -w '%{redirect_url}' \
    "https://github.com/Kieirra/murmure/releases/latest")
VERSION=$(echo "$LATEST_URL" | sed 's|.*/||')

if [ -z "$VERSION" ]; then
    echo "Error: Could not determine latest version." >&2
    exit 1
fi

echo "Latest version: $VERSION"

# 4. Télécharger le .deb
DEB_URL="https://github.com/Kieirra/murmure/releases/download/${VERSION}/Murmure_amd64.deb"
TMP_DEB="/tmp/murmure_${VERSION}_amd64.deb"

echo "Downloading Murmure $VERSION..."
curl -fSL -o "$TMP_DEB" "$DEB_URL"

# 5. Installer
echo "Installing Murmure (requires sudo)..."
sudo dpkg -i "$TMP_DEB" || sudo apt-get install -f -y

# 6. Nettoyage
rm -f "$TMP_DEB"

echo ""
echo "Murmure $VERSION installed successfully!"
echo "Launch it from your application menu or run: murmure"
