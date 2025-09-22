# Razen Programming Language

A modern, efficient programming language with clean syntax and powerful features.

## 🚀 Quick Installation

### One-Line Installation (Recommended)
```bash
curl -sSf https://raw.githubusercontent.com/YOUR_USERNAME/razen-lang-new/main/install.sh | bash
```

### Manual Installation
```bash
# Download the installer
curl -O https://raw.githubusercontent.com/YOUR_USERNAME/razen-lang-new/main/install.sh

# Make it executable
chmod +x install.sh

# Run the installer
./install.sh
```

## 📋 What the installer does:
1. Downloads the latest Razen compiler from GitHub
2. Installs it to `~/.razen/`
3. Adds `~/.razen/bin` to your PATH
4. Creates a global `razen` command

## 🛠️ Usage

After installation, you can use Razen globally:

```bash
# Compile and run (like go run)
razen run program.rzn

# Development mode with detailed output
razen dev program.rzn

# Compile to executable
razen compile program.rzn -o myprogram

# Run tests
razen test program.rzn

# Show help
razen --help

# Show version
razen --version
```

## 📁 Installation Directory Structure
```
~/.razen/
├── bin/
│   ├── razen-lang    # Main binary
│   └── razen         # Symlink for easy access
└── scripts/          # Additional scripts
```

## 🔧 Manual Uninstall
```bash
# Remove Razen installation
rm -rf ~/.razen

# Remove from PATH (edit your shell profile)
# Remove the line: export PATH="$HOME/.razen/bin:$PATH"
```

## 🌟 Example Program

Create `hello.rzn`:
```razen
fun main() {
    println("Hello, Razen!")
    println("Welcome to modern programming!")
}
```

Run it:
```bash
razen run hello.rzn
```

## 🎯 Features

- **Clean Syntax**: Modern, readable code
- **Fast Compilation**: Quick build times
- **Professional CLI**: Industry-standard command interface
- **Cross-Platform**: Works on Linux, macOS, Windows
- **Zero Dependencies**: Self-contained installation

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## 📄 License

This project is licensed under the MIT License.

---

**Happy coding with Razen! 🚀**
