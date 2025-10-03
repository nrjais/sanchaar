#!/usr/bin/env bash
set -e

# Script to generate Windows .ico file from PNG assets
# Requires ImageMagick to be installed

echo "Generating Windows icon file..."

# Check if ImageMagick is installed
if ! command -v convert &> /dev/null; then
    echo "Error: ImageMagick is not installed."
    echo "Please install it first:"
    echo "  macOS: brew install imagemagick"
    echo "  Ubuntu/Debian: sudo apt-get install imagemagick"
    echo "  Windows: Download from https://imagemagick.org"
    exit 1
fi

# Check if required PNG files exist
if [ ! -f "assets/16x16.png" ] || [ ! -f "assets/32x32.png" ] || [ ! -f "assets/48x48.png" ] || [ ! -f "assets/256x256.png" ]; then
    echo "Error: Required PNG files not found in assets/ directory"
    echo "Required files: 16x16.png, 32x32.png, 48x48.png, 256x256.png"
    exit 1
fi

# Generate .ico file
echo "Converting PNG files to .ico..."
convert assets/16x16.png assets/32x32.png assets/48x48.png assets/256x256.png assets/sanchaar.ico

echo "✓ Generated assets/sanchaar.ico successfully!"
echo ""
echo "You can now uncomment the icon lines in wix/main.wxs:"
echo "  Line 154: <Icon Id='ProductICO' SourceFile='assets\\sanchaar.ico'/>"
echo "  Line 155: <Property Id='ARPPRODUCTICON' Value='ProductICO' />"
