#!/bin/bash

# Build script for Razen Zed Extension

set -e

echo "Building Razen Zed Extension..."
echo ""

# Detect fastest package manager (prefer bun > pnpm > npm)
PKG_MANAGER=""
PKG_EXEC=""

if command -v bun &> /dev/null; then
    PKG_MANAGER="bun"
    PKG_EXEC="bunx"
    echo "✨ Using Bun (fastest package manager)"
elif command -v pnpm &> /dev/null; then
    PKG_MANAGER="pnpm"
    PKG_EXEC="pnpm exec"
    echo "⚡ Using pnpm (fast package manager)"
elif command -v npm &> /dev/null; then
    PKG_MANAGER="npm"
    PKG_EXEC="npx"
    echo "📦 Using npm (default package manager)"
else
    echo "❌ Error: No package manager found"
    echo "Please install bun, pnpm, or npm"
    exit 1
fi

echo "Package manager: $PKG_MANAGER"
echo ""

# Build Tree-sitter grammar
echo "Building Tree-sitter grammar..."
cd tree-sitter-razen

# Install dependencies
if [ ! -d "node_modules" ]; then
    echo "Installing dependencies with $PKG_MANAGER..."
    if [ "$PKG_MANAGER" = "bun" ]; then
        bun install
    elif [ "$PKG_MANAGER" = "pnpm" ]; then
        pnpm install
    else
        npm install
    fi
fi

# Generate parser
echo "Generating parser..."
$PKG_EXEC tree-sitter generate

# Run tests
echo "Running tests..."
$PKG_EXEC tree-sitter test || true

cd ..

echo ""
echo "✅ Build complete!"
echo ""
echo "To install in Zed:"
echo "1. Open Zed"
echo "2. Press Cmd+Shift+P (Mac) or Ctrl+Shift+P (Linux/Windows)"
echo "3. Type 'Install Dev Extension'"
echo "4. Select this directory"
echo ""
echo "To test the grammar:"
echo "  cd tree-sitter-razen"
echo "  $PKG_EXEC tree-sitter parse ../test.rzn"
echo ""
