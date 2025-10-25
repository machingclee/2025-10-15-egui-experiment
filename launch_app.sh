#!/bin/bash
# Simple script to launch the Shell Script Manager app

APP_BUNDLE="Shell Script Manager.app"

if [ ! -d "$APP_BUNDLE" ]; then
    echo "❌ Error: '$APP_BUNDLE' not found!"
    echo "   Please run ./bundle_macos.sh first to create the app bundle."
    exit 1
fi

echo "🚀 Launching '$APP_BUNDLE'..."
open "$APP_BUNDLE"

if [ $? -eq 0 ]; then
    echo "✅ App launched successfully!"
else
    echo "⚠️  Failed to launch the app."
    echo ""
    echo "If you get a Gatekeeper warning, run:"
    echo "  ./remove_quarantine.sh"
fi

