# Razen VSCode Extension Development Guide

## 🚀 Quick Start

### Prerequisites
- Node.js 16.x or higher
- npm 8.x or higher
- Visual Studio Code 1.74.0 or higher (or Windsurf - VSCode fork)
- Git

### Installation & Setup

1. **Clone and Navigate**
   ```bash
   cd extensions/razen-vscode
   ```

2. **Install Dependencies**
   ```bash
   npm install
   ```

3. **Compile TypeScript**
   ```bash
   npm run compile
   ```

4. **Run Extension in Development Mode**
   - Press `F5` in VSCode to launch Extension Development Host
   - Or use `Ctrl+Shift+P` → "Debug: Start Debugging"

## 📁 Project Structure

```
razen-vscode/
├── src/
│   └── extension.ts          # Main extension logic
├── syntaxes/
│   └── razen.tmGrammar.json  # TextMate grammar for syntax highlighting
├── snippets/
│   └── razen.json            # Code snippets
├── themes/
│   └── razen-dark.json       # Custom theme
├── images/
│   ├── razen-icon.png        # Extension icon
│   ├── razen-file-icon-dark.svg
│   └── razen-file-icon-light.svg
├── test-files/
│   └── sample.rzn            # Test file for validation
├── language-configuration.json  # Language configuration
├── package.json              # Extension manifest
├── tsconfig.json            # TypeScript configuration
└── README.md                # Documentation
```

## 🛠️ Development Workflow

### 1. Making Changes

**Syntax Highlighting:**
- Edit `syntaxes/razen.tmGrammar.json`
- Reload extension window (`Ctrl+R`)

**Code Snippets:**
- Edit `snippets/razen.json`
- Reload extension window

**Language Features:**
- Edit `src/extension.ts`
- Recompile: `npm run compile`
- Reload extension window

**Language Configuration:**
- Edit `language-configuration.json`
- Reload extension window

### 2. Testing Changes

**Manual Testing:**
1. Press `F5` to launch Extension Development Host
2. Open `test-files/sample.rzn`
3. Test syntax highlighting, snippets, and IntelliSense

**Automated Testing:**
```bash
npm test  # (when test suite is implemented)
```

### 3. Debugging

**Extension Host Debugging:**
- Set breakpoints in `src/extension.ts`
- Press `F5` to start debugging
- Use Debug Console to inspect variables

**Grammar Debugging:**
- Use `Ctrl+Shift+P` → "Developer: Inspect Editor Tokens and Scopes"
- Inspect token scopes and grammar matching

## 📦 Building & Packaging

### 1. Install VSCE (VSCode Extension Manager)
```bash
npm install -g vsce
```

### 2. Package Extension
```bash
vsce package
```
This creates a `.vsix` file that can be installed.

### 3. Install Locally
```bash
code --install-extension razen-language-support-1.0.0.vsix
```

### 4. Automated Installation
```bash
./install-and-test.sh
```

## 🔧 Extension Features

### Current Features
- ✅ Syntax highlighting for all Razen constructs
- ✅ 30+ code snippets
- ✅ IntelliSense and auto-completion
- ✅ Bracket matching and auto-closing
- ✅ Comment toggling
- ✅ Document symbols (outline)
- ✅ Basic code formatting
- ✅ Hover information
- ✅ Compile and run commands
- ✅ Custom dark theme

### Planned Features
- 🔄 Language Server Protocol integration
- 🔄 Advanced error diagnostics
- 🔄 Go-to-definition
- 🔄 Find references
- 🔄 Refactoring support
- 🔄 Debugging integration

## 🎨 Customization

### Adding New Snippets
1. Edit `snippets/razen.json`
2. Add new snippet object:
   ```json
   "Snippet Name": {
     "prefix": "trigger",
     "body": ["line1", "line2"],
     "description": "Description"
   }
   ```

### Extending Syntax Highlighting
1. Edit `syntaxes/razen.tmGrammar.json`
2. Add new patterns to appropriate repository sections
3. Use [TextMate grammar documentation](https://macromates.com/manual/en/language_grammars)

### Adding Language Features
1. Edit `src/extension.ts`
2. Register new providers:
   ```typescript
   const provider = vscode.languages.registerXxxProvider('razen', {
     // Implementation
   });
   context.subscriptions.push(provider);
   ```

## 🧪 Testing

### Manual Testing Checklist
- [ ] Syntax highlighting works for all language constructs
- [ ] Code snippets trigger correctly
- [ ] Auto-completion shows relevant suggestions
- [ ] Bracket matching and auto-closing works
- [ ] Comment toggling works (`Ctrl+/`)
- [ ] Document outline shows functions and structures
- [ ] Hover information displays correctly
- [ ] Compile and run commands work (with Razen compiler)

### Test Files
- `test-files/sample.rzn` - Comprehensive test file
- Create additional `.rzn` files for specific feature testing

## 🚀 Publishing

### Prerequisites
- Create [Visual Studio Marketplace](https://marketplace.visualstudio.com/) publisher account
- Update `package.json` with correct publisher name

### Publish to Marketplace
```bash
vsce publish
```

### Publish Specific Version
```bash
vsce publish 1.0.1
```

## 🐛 Troubleshooting

### Common Issues

**Extension not loading:**
- Check `package.json` syntax
- Verify activation events
- Check developer console for errors

**Syntax highlighting not working:**
- Validate `razen.tmGrammar.json` syntax
- Check file associations in `package.json`
- Verify scope names match theme

**TypeScript compilation errors:**
- Run `npm install` to ensure dependencies
- Check `tsconfig.json` configuration
- Verify VS Code API usage

**Snippets not triggering:**
- Check `snippets/razen.json` syntax
- Verify language ID matches
- Ensure prefix doesn't conflict

### Debug Commands
```bash
# Check extension status
code --list-extensions

# Uninstall extension
code --uninstall-extension razen-lang.razen-language-support

# View extension logs
# Check VS Code Developer Tools Console
```

## 📚 Resources

- [VS Code Extension API](https://code.visualstudio.com/api)
- [TextMate Grammar Guide](https://code.visualstudio.com/api/language-extensions/syntax-highlight-guide)
- [Language Configuration Guide](https://code.visualstudio.com/api/language-extensions/language-configuration-guide)
- [Extension Samples](https://github.com/microsoft/vscode-extension-samples)
- [Publishing Extensions](https://code.visualstudio.com/api/working-with-extensions/publishing-extension)

## 🤝 Contributing

1. Fork the repository
2. Create feature branch
3. Make changes following this guide
4. Test thoroughly
5. Submit pull request

---

**Happy Extension Development!** 🚀
