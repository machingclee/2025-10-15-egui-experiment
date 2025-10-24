#!/bin/bash
# Script to remove macOS quarantine attribute from the app bundle
# This allows the app to run without Gatekeeper warnings during development

APP_BUNDLE="Shell Script Manager.app"

if [ ! -d "$APP_BUNDLE" ]; then
    echo "❌ Error: '$APP_BUNDLE' not found!"
    echo "   Please run ./bundle_macos.sh first to create the app bundle."
    exit 1
fi

echo "Removing quarantine attributes from '$APP_BUNDLE'..."
sudo xattr -rd com.apple.quarantine "$APP_BUNDLE"

if [ $? -eq 0 ]; then
    echo "✅ Quarantine removed successfully!"
    echo ""
    echo "You can now double-click '$APP_BUNDLE' to launch it."
    echo "Or run: open '$APP_BUNDLE'"
else
    echo "⚠️  Failed to remove quarantine. You may need to enter your password."
fi

