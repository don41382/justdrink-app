#!/bin/bash

# Check for platform argument
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <platform>"
    echo "Platform options: mac, windows"
    exit 1
fi

PLATFORM=$1
ZIP_NAME="$PLATFORM.zip"

rm -f "$ZIP_NAME"

# Determine the file pattern and handle platform-specific logic
if [ "$PLATFORM" == "mac" ]; then
    FILE_PATTERN="*.mov"
    echo "Zipping .mov files for mac..."
    zip -r "$ZIP_NAME" $(find . -type f -name "$FILE_PATTERN")
elif [ "$PLATFORM" == "windows" ]; then
    FILE_PATTERN="*.webm"
    echo "Zipping .webm files for windows..."
    zip -r "$ZIP_NAME" bg-h264.mov $(find . -type f -name "$FILE_PATTERN")
else
    echo "Invalid platform specified. Use 'mac' or 'windows'."
    exit 1
fi

echo "Files successfully zipped into $ZIP_NAME"

