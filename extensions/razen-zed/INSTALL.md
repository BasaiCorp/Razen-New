# Installation Guide - Razen Extension for Zed

## Quick Install (Dev Extension)

This is the recommended method for testing and development.

### Steps:

1. **Clone or download this repository**
   ```bash
   cd /path/to/razen-lang-new/extensions/razen-zed
   ```

2. **Build the Tree-sitter grammar** (first time only)
   ```bash
   cd tree-sitter-razen
   npm install
   npx tree-sitter generate
   cd ..
   ```

3. **Install in Zed**
   - Open Zed editor
   - Press `Cmd+Shift+P` (Mac) or `Ctrl+Shift+P` (Linux/Windows)
   - Type "Install Dev Extension"
   - Select the `razen-zed` directory
   - Restart Zed if prompted

4. **Test it**
   - Open any `.rzn` or `.razen` file
   - You should see syntax highlighting!

## Publishing to Zed Extensions Registry

To make the extension available to all Zed users:

### Prerequisites:
- GitHub account
- Git installed
- Node.js and pnpm installed

### Steps:

1. **Fork the Zed extensions repository**
   ```bash
   # Visit https://github.com/zed-industries/extensions
   # Click "Fork" button
   ```

2. **Clone your fork**
   ```bash
   git clone https://github.com/YOUR_USERNAME/extensions.git
   cd extensions
   ```

3. **Add Razen extension as submodule**
   ```bash
   # First, push your razen-zed to GitHub
   # Then add it as submodule:
   git submodule add https://github.com/razen-lang/razen-zed.git extensions/razen
   git add extensions/razen
   ```

4. **Update extensions.toml**
   ```bash
   # Add to extensions.toml:
   cat >> extensions.toml << 'EOF'

[razen]
submodule = "extensions/razen"
version = "0.1.0"
EOF
   ```

5. **Sort extensions**
   ```bash
   pnpm install
   pnpm sort-extensions
   ```

6. **Commit and push**
   ```bash
   git add .
   git commit -m "Add Razen language extension"
   git push origin main
   ```

7. **Create Pull Request**
   - Go to your fork on GitHub
   - Click "Pull Request"
   - Submit PR to zed-industries/extensions

8. **Wait for review**
   - Zed team will review your extension
   - They may request changes
   - Once approved, it will be published!

## Troubleshooting

### Extension not showing up

1. Check Zed logs:
   - Press `Cmd+Shift+P` / `Ctrl+Shift+P`
   - Type "Open Log"
   - Look for errors

2. Verify Tree-sitter grammar built correctly:
   ```bash
   cd tree-sitter-razen
   npx tree-sitter test
   ```

3. Check extension.toml syntax:
   ```bash
   cat extension.toml
   ```

### Syntax highlighting not working

1. Verify file extension is `.rzn` or `.razen`
2. Check if grammar is loaded:
   - Open Zed log
   - Look for "Loading grammar: razen"
3. Rebuild grammar:
   ```bash
   cd tree-sitter-razen
   npx tree-sitter generate
   ```

### Grammar build errors

1. Ensure Node.js is installed:
   ```bash
   node --version  # Should be v14 or higher
   ```

2. Install dependencies:
   ```bash
   cd tree-sitter-razen
   npm install
   ```

3. Check grammar.js syntax:
   ```bash
   npx tree-sitter generate
   ```

## Updating the Extension

### For Dev Extension:

1. Make your changes
2. If grammar changed:
   ```bash
   cd tree-sitter-razen
   npx tree-sitter generate
   ```
3. Reload Zed or reinstall dev extension

### For Published Extension:

1. Update version in `extension.toml`
2. Commit and push changes
3. Update version in extensions.toml PR
4. Zed team will publish new version

## Uninstalling

### Dev Extension:
1. Open Zed
2. Go to Extensions panel
3. Find "Razen"
4. Click "Uninstall"

### Published Extension:
Same as above - use Extensions panel in Zed

## Getting Help

- Check [DEVELOPMENT.md](DEVELOPMENT.md) for development guide
- Open issue on GitHub
- Join Zed Discord community
- Check Zed documentation: https://zed.dev/docs

---

**Need more help?** Open an issue on GitHub!
