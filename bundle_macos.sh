#!/bin/bash
# Script to bundle the Rust app into a macOS .app bundle

set -e

APP_NAME="Shell Script Manager"
BUNDLE_NAME="Shell Script Manager.app"
EXECUTABLE_NAME="shell_script_manager"
BUNDLE_ID="com.shellscriptmanager.app"
VERSION="0.1.0"

echo "Building release binary..."
cargo build --release

echo "Creating app bundle structure..."
rm -rf "$BUNDLE_NAME"
mkdir -p "$BUNDLE_NAME/Contents/MacOS"
mkdir -p "$BUNDLE_NAME/Contents/Resources"

echo "Copying executable..."
cp "target/release/$EXECUTABLE_NAME" "$BUNDLE_NAME/Contents/MacOS/$EXECUTABLE_NAME"

echo "Copying icon..."
cp "assets/icon-256.png" "$BUNDLE_NAME/Contents/Resources/icon.png"

# Convert PNG to ICNS (macOS icon format) if sips is available
if command -v sips &> /dev/null && command -v iconutil &> /dev/null; then
    echo "Converting icon to ICNS format..."
    mkdir -p icon.iconset
    sips -z 16 16     assets/icon-256.png --out icon.iconset/icon_16x16.png
    sips -z 32 32     assets/icon-256.png --out icon.iconset/icon_16x16@2x.png
    sips -z 32 32     assets/icon-256.png --out icon.iconset/icon_32x32.png
    sips -z 64 64     assets/icon-256.png --out icon.iconset/icon_32x32@2x.png
    sips -z 128 128   assets/icon-256.png --out icon.iconset/icon_128x128.png
    sips -z 256 256   assets/icon-256.png --out icon.iconset/icon_128x128@2x.png
    sips -z 256 256   assets/icon-256.png --out icon.iconset/icon_256x256.png
    sips -z 512 512   assets/icon-1024.png --out icon.iconset/icon_256x256@2x.png
    sips -z 512 512   assets/icon-1024.png --out icon.iconset/icon_512x512.png
    sips -z 1024 1024 assets/icon-1024.png --out icon.iconset/icon_512x512@2x.png
    iconutil -c icns icon.iconset -o "$BUNDLE_NAME/Contents/Resources/icon.icns"
    rm -rf icon.iconset
fi

echo "Creating Info.plist..."
cat > "$BUNDLE_NAME/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>$EXECUTABLE_NAME</string>
    <key>CFBundleIconFile</key>
    <string>icon.icns</string>
    <key>CFBundleIdentifier</key>
    <string>$BUNDLE_ID</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSPrincipalClass</key>
    <string>NSApplication</string>
</dict>
</plist>
EOF

echo "Setting executable permissions..."
chmod +x "$BUNDLE_NAME/Contents/MacOS/$EXECUTABLE_NAME"

echo ""
echo "✅ App bundle created successfully: $BUNDLE_NAME"
echo ""
echo "You can now:"
echo "  1. Double-click '$BUNDLE_NAME' to run the app"
echo "  2. Drag it to /Applications folder"
echo "  3. Run: open '$BUNDLE_NAME'"
echo ""

