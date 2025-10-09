#!/bin/bash

# Copy blog images to dist directory after build
echo "Copying blog images to dist..."

# Create the destination directory
mkdir -p dist/blog/Images

# Copy all images from blog/images to dist/blog/Images
cp -r blog/images/* dist/blog/Images/ 2>/dev/null || echo "No images to copy"

echo "Images copied successfully!"