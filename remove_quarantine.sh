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

    # Ask if user wants to launch the app
    read -p "Launch the app now? (Y/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]] || [[ -z $REPLY ]]; then
        echo "Launching '$APP_BUNDLE'..."
        open "$APP_BUNDLE"
    else
        echo "You can launch it later by running: open '$APP_BUNDLE'"
    fi
else
    echo "⚠️  Failed to remove quarantine. You may need to enter your password."
fi

