#!/bin/bash

# Razen VSCode Extension Installation and Testing Script
echo "🚀 Razen VSCode Extension Installation and Testing"
echo "=================================================="

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js first."
    exit 1
fi

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    echo "❌ npm is not installed. Please install npm first."
    exit 1
fi

# Check if Windsurf or VSCode is installed
EDITOR_CMD=""
if command -v windsurf &> /dev/null; then
    EDITOR_CMD="windsurf"
    echo "✅ Found Windsurf editor"
elif command -v code &> /dev/null; then
    EDITOR_CMD="code"
    echo "✅ Found VSCode editor"
else
    echo "❌ Neither Windsurf nor VSCode is installed. Please install one of them first."
    exit 1
fi

echo "✅ Prerequisites check passed"
echo ""

# Install dependencies
echo "📦 Installing dependencies..."
npm install
if [ $? -ne 0 ]; then
    echo "❌ Failed to install dependencies"
    exit 1
fi
echo "✅ Dependencies installed successfully"
echo ""

# Compile TypeScript
echo "🔨 Compiling TypeScript..."
npm run compile
if [ $? -ne 0 ]; then
    echo "❌ TypeScript compilation failed"
    exit 1
fi
echo "✅ TypeScript compiled successfully"
echo ""

# Install vsce locally if not available globally
VSCE_CMD="vsce"
if ! command -v vsce &> /dev/null; then
    echo "📦 Installing vsce locally (VSCode Extension Manager)..."
    npm install vsce --save-dev
    if [ $? -ne 0 ]; then
        echo "❌ Failed to install vsce locally"
        exit 1
    fi
    VSCE_CMD="npx vsce"
    echo "✅ vsce installed locally successfully"
fi

# Package the extension
echo "📦 Packaging extension..."
$VSCE_CMD package
if [ $? -ne 0 ]; then
    echo "❌ Failed to package extension"
    exit 1
fi
echo "✅ Extension packaged successfully"
echo ""

# Find the generated .vsix file
VSIX_FILE=$(ls *.vsix 2>/dev/null | head -n 1)
if [ -z "$VSIX_FILE" ]; then
    echo "❌ No .vsix file found"
    exit 1
fi

echo "📁 Generated extension file: $VSIX_FILE"
echo ""

# Install the extension
echo "🔧 Installing extension in $EDITOR_CMD..."
$EDITOR_CMD --install-extension "$VSIX_FILE" --force
if [ $? -ne 0 ]; then
    echo "❌ Failed to install extension"
    exit 1
fi
echo "✅ Extension installed successfully"
echo ""

# Open test file
echo "📝 Opening test file..."
$EDITOR_CMD test-files/sample.rzn
echo "✅ Test file opened"
echo ""

echo "🎉 Installation completed successfully!"
echo ""
echo "📋 Next steps:"
echo "1. $EDITOR_CMD should now have syntax highlighting for .rzn files"
echo "2. Try typing 'main' + Tab to test snippets"
echo "3. Use Ctrl+Shift+P and search for 'Razen' commands"
echo "4. Test auto-completion by typing Razen keywords"
echo ""
echo "🔧 To test compilation (requires Razen compiler):"
echo "   Ctrl+Shift+P → 'Razen: Compile' or 'Razen: Run'"
echo ""
echo "Happy coding with Razen! 🚀"
