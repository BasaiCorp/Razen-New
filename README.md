# The Razen Programming Language

Razen is a modern, efficient programming language designed for building reliable and performant software with clean, readable syntax.

**Current Version: v0.1-beta.4**

This is the main source code repository for Razen. It contains the compiler, standard library, and documentation.

## Why Razen?

- **Performance**: Fast compilation and efficient execution, suitable for system programming, web services, and general-purpose applications.

- **Simplicity**: Clean, intuitive syntax that reduces cognitive overhead and makes code easier to read and maintain.

- **Reliability**: Strong type system and compile-time checks help catch errors early and ensure program correctness.

- **Productivity**: Professional toolchain with comprehensive CLI, clear error messages, and seamless development workflow.

## Quick Start

### Installation

#### One-Line Installation (Recommended)
```bash
curl -sSf https://raw.githubusercontent.com/BasaiCorp/Razen-New/main/install.sh | bash
```

#### Manual Installation
```bash
# Download the installer
curl -O https://raw.githubusercontent.com/BasaiCorp/Razen-New/main/install.sh

# Make it executable and run
chmod +x install.sh
./install.sh
```

#### Windows Users
Windows users should use **Git Bash** to run the installation commands above. Git Bash provides the necessary Unix-like environment for the installation script.

### Your First Program

Create a file named `hello.rzn`:
```razen
fun main() {
    println("Hello, Razen!")
    println("Welcome to modern programming!")
}
```

Run your program:
```bash
razen run hello.rzn
```

## Usage

After installation, you can use Razen with the following commands:

```bash
# Compile and run immediately (like go run)
razen run program.rzn

# Development mode with detailed compiler output
razen dev program.rzn

# Compile to native executable
razen compile program.rzn -o myprogram

# Run test files
razen test program.rzn

# Show help
razen --help

# Show version information
razen --version
```

## Installation Details

The installer performs the following actions:
1. Downloads the latest Razen compiler from GitHub
2. Installs it to `~/.razen/`
3. Adds `~/.razen/bin` to your PATH
4. Creates a global `razen` command

### Installation Directory Structure
```
~/.razen/
├── bin/
│   ├── razen-lang    # Main compiler binary
│   └── razen         # Symlink for easy access
├── version           # Version tracking for updates
└── scripts/          # Additional utility scripts
```

### Updating Razen

The installer automatically checks for updates when run again:
```bash
./install.sh
```

This will check your current version against the latest release and prompt you to update if a newer version is available.

### Uninstalling

To remove Razen from your system:
```bash
# Remove installation directory
rm -rf ~/.razen

# Remove from PATH (edit your shell profile)
# Remove the line: export PATH="$HOME/.razen/bin:$PATH"
```

## Language Features

- **Modern Syntax**: Clean, readable code structure
- **Static Typing**: Compile-time type checking with type inference
- **Memory Safety**: Automatic memory management without garbage collection overhead
- **Cross-Platform**: Runs on Linux, macOS, and Windows
- **Fast Compilation**: Quick build times for rapid development cycles
- **Professional Tooling**: Comprehensive CLI with helpful diagnostics

## Getting Help

For questions, bug reports, and discussions:
- Check the [documentation](docs/)
- Browse [existing issues](https://github.com/BasaiCorp/Razen-New/issues)
- Join our community discussions

## Contributing

We welcome contributions to Razen! To get started:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes and add tests
4. Commit your changes (`git commit -m 'Add amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

Please read our [Contributing Guidelines](CONTRIBUTING.md) for detailed information about the development process, coding standards, and how to submit patches.

## License

Razen is distributed under the Apache License 2.0. See [LICENSE](LICENSE) for details.

## Trademark

The Razen name and logo are trademarks of BasaiCorp. Please see our trademark policy for usage guidelines.
