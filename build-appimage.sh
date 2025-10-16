#!/bin/bash
set -e

echo "Building with Tauri CLI..."
pnpm tauri build --bundles deb

echo "Verifying binary exists..."
if [ ! -f src-tauri/target/release/murmure ]; then
    echo "❌ Error: Binary not found"
    exit 1
fi
echo "✓ Binary found"

echo "Preparing AppImage directory..."
rm -rf murmure.AppDir/usr
mkdir -p murmure.AppDir/usr/bin
mkdir -p murmure.AppDir/usr/lib/murmure
mkdir -p murmure.AppDir/usr/share/applications
mkdir -p murmure.AppDir/usr/share/metainfo

echo "Copying binary..."
cp src-tauri/target/release/murmure murmure.AppDir/usr/bin/

echo "Copying resources (including AI model)..."
if [ -d "resources" ]; then
    cp -r resources murmure.AppDir/usr/lib/murmure/
    echo "✓ Resources copied ($(du -sh resources | cut -f1))"
else
    echo "⚠ Warning: resources folder not found at project root"
fi

echo "Creating AppRun..."
cat > murmure.AppDir/AppRun << 'EOF'
#!/bin/sh
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export PATH="${HERE}/usr/bin:${PATH}"
export LD_LIBRARY_PATH="${HERE}/usr/lib:${LD_LIBRARY_PATH}"
exec "${HERE}/usr/bin/murmure" "$@"
EOF
chmod +x murmure.AppDir/AppRun

echo "Creating desktop file..."
cat > murmure.AppDir/murmure.desktop << 'EOF'
[Desktop Entry]
Name=Murmure
Comment=Local voice transcription
Exec=murmure
Icon=murmure
Type=Application
Categories=AudioVideo;
Terminal=false
EOF
cp murmure.AppDir/murmure.desktop murmure.AppDir/usr/share/applications/

echo "Creating AppStream metadata..."
cat > murmure.AppDir/usr/share/metainfo/murmure.appdata.xml << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<component type="desktop-application">
    <id>io.github.kieirra.murmure</id>
    <name>Murmure</name>
    <summary>Local voice transcription</summary>
    <developer id="io.github.kieirra"><name>Murmure Team</name></developer>
    <metadata_license>CC0-1.0</metadata_license>
    <project_license>MIT</project_license>
    <description><p>Privacy-first speech-to-text running on your machine</p></description>
    <launchable type="desktop-id">murmure.desktop</launchable>
    <url type="homepage">https://github.com/Kieirra/murmure</url>
    <provides><binary>murmure</binary></provides>
    <categories><category>AudioVideo</category></categories>
</component>
EOF

echo "Copying icon..."
cp src-tauri/icons/icon.png murmure.AppDir/murmure.png 2>/dev/null || \
cp src-tauri/icons/128x128.png murmure.AppDir/murmure.png

if [ ! -f appimagetool-x86_64.AppImage ]; then
    wget https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage
    chmod +x appimagetool-x86_64.AppImage
fi

echo "Building AppImage..."
./appimagetool-x86_64.AppImage --no-appstream murmure.AppDir murmure-x86_64.AppImage

echo ""
echo "✓ AppImage created: murmure-x86_64.AppImage"
echo "  Size: $(du -h murmure-x86_64.AppImage | cut -f1)"
echo ""
echo "🧪 Test with: ./murmure-x86_64.AppImage"