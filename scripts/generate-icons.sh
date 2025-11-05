#!/bin/bash
# Icon-Generierungs-Script für rustic-gui
# Quelle: docs/images/rustic-gui.svg → PNG/ICO/ICNS für alle Plattformen

set -e

SOURCE_SVG="docs/images/rustic-gui.svg"

echo "🎨 Generiere Icons aus $SOURCE_SVG..."

# Prüfe ob Inkscape verfügbar ist
if ! command -v inkscape &> /dev/null; then
    echo "❌ Fehler: Inkscape nicht installiert. Bitte installieren: sudo pacman -S inkscape"
    exit 1
fi

# Prüfe ob ImageMagick verfügbar ist
if ! command -v magick &> /dev/null; then
    echo "❌ Fehler: ImageMagick nicht installiert. Bitte installieren: sudo pacman -S imagemagick"
    exit 1
fi

echo "📦 Generiere Tauri Icons..."
inkscape "$SOURCE_SVG" -w 32 -h 32 -o src-tauri/icons/32x32.png
inkscape "$SOURCE_SVG" -w 128 -h 128 -o src-tauri/icons/128x128.png
inkscape "$SOURCE_SVG" -w 256 -h 256 -o src-tauri/icons/128x128@2x.png
inkscape "$SOURCE_SVG" -w 512 -h 512 -o src-tauri/icons/icon.png
inkscape "$SOURCE_SVG" -w 768 -h 768 -o src-tauri/icons/icon768x768.png

echo "🪟 Generiere Windows ICO..."
magick src-tauri/icons/32x32.png src-tauri/icons/128x128.png \
       src-tauri/icons/128x128@2x.png src-tauri/icons/icon.png \
       src-tauri/icons/icon.ico

echo "🍎 Generiere macOS ICNS..."
magick src-tauri/icons/icon.png -define icon:auto-resize=256,128,64,48,32,16 \
       src-tauri/icons/icon.icns

echo "🌐 Generiere Web Icons..."
inkscape "$SOURCE_SVG" -w 32 -h 32 -o static/favicon.png
inkscape "$SOURCE_SVG" -w 512 -h 512 -o static/icon.png
cp "$SOURCE_SVG" static/icon.svg

echo "✅ Alle Icons erfolgreich generiert!"
echo ""
echo "Generierte Dateien:"
echo "  - src-tauri/icons/*.png (32x32, 128x128, 256x256, 512x512, 768x768)"
echo "  - src-tauri/icons/icon.ico (Windows)"
echo "  - src-tauri/icons/icon.icns (macOS)"
echo "  - static/favicon.png (32x32)"
echo "  - static/icon.png (512x512)"
echo "  - static/icon.svg (SVG Original)"
