# Getting Started with Razen

Razen is a modern, efficient programming language designed for building reliable and performant software with clean, readable syntax. This guide will help you get started with Razen development.

## Prerequisites

Before installing Razen, ensure you have:

- A Unix-like environment (Linux, macOS, or Git Bash on Windows)
- Internet connection for downloading the compiler
- Basic familiarity with command-line interfaces

## Installation

### Quick Installation

The fastest way to install Razen is using the one-line installer:

```bash
curl -sSf https://raw.githubusercontent.com/BasaiCorp/Razen-New/main/install.sh | bash
```

This command downloads and executes the installation script, which will:
1. Download the latest Razen compiler binary
2. Install it to `~/.razen/`
3. Add the Razen binary to your PATH
4. Verify the installation

### Manual Installation

If you prefer to review the installation script before running it:

```bash
# Download the installer
curl -O https://raw.githubusercontent.com/BasaiCorp/Razen-New/main/install.sh

# Review the script (optional but recommended)
cat install.sh

# Make it executable
chmod +x install.sh

# Run the installer
./install.sh
```

### Windows Installation

Windows users should use Git Bash to run the installation commands. Git Bash provides the necessary Unix-like environment for the installation script.

1. Install Git for Windows from https://git-scm.com/download/win
2. Open Git Bash
3. Run the installation command shown above

### Verifying Installation

After installation completes, verify that Razen is correctly installed:

```bash
razen --version
```

You should see output similar to:
```
Razen v0.1-beta.7
```

If the command is not found, you may need to restart your terminal or manually add `~/.razen/bin` to your PATH.

## Your First Program

Let's write a simple "Hello, World!" program to verify everything works.

### Creating the File

Create a new file named `hello.rzn`:

```razen
fun main() {
    println("Hello, Razen!")
}
```

### Running the Program

Execute your program using the `run` command:

```bash
razen run hello.rzn
```

You should see:
```
Hello, Razen!
```

Congratulations! You've successfully written and executed your first Razen program.

## Understanding the Basics

Let's break down the hello world program:

- `fun main()` - Defines the main function, the entry point of every Razen program
- `println()` - A built-in function that prints text to the console with a newline
- Strings are enclosed in double quotes (`"..."`)
- Function bodies are enclosed in curly braces (`{...}`)

## Development Workflow

Razen provides several commands for different development scenarios:

### Quick Execution

For rapid development and testing, use `run`:

```bash
razen run program.rzn
```

This compiles and executes your program immediately, similar to `go run`.

### Development Mode

For detailed compiler output and debugging:

```bash
razen dev program.rzn
```

This shows:
- Parsing progress
- Semantic analysis results
- Compilation status
- Execution output

### Building Executables

To create a standalone executable:

```bash
razen compile program.rzn -o myprogram
```

This produces a native executable that can be distributed and run independently.

## Project Structure

For larger projects, Razen supports project-based development:

### Creating a Project

```bash
razen create my-project
cd my-project
```

This creates a project directory with:
- `razen.toml` - Project configuration file
- `main.rzn` - Entry point source file
- `.gitignore` - Git ignore file

### Project Configuration

The `razen.toml` file contains project metadata:

```toml
[project]
name = "my-project"
version = "0.1.0"
description = "A Razen project"

[build]
main = "main.rzn"
src_dir = "src"
optimization = 2
debug = false
```

### Building Projects

Build your entire project with:

```bash
razen build
```

For optimized release builds:

```bash
razen build --release
```

## Next Steps

Now that you have Razen installed and running, you can:

1. Learn about [variables and data types](variables-datatypes.md)
2. Explore [control flow and loops](control-flow.md)
3. Understand [functions and modules](functions-modules.md)
4. Study [object-oriented programming](oop.md)

## Getting Help

If you encounter issues:

- Check the [documentation](README.md)
- Review [common issues](troubleshooting.md)
- Report bugs on [GitHub Issues](https://github.com/BasaiCorp/Razen-New/issues)

## Updating Razen

To update to the latest version, run the installer again:

```bash
curl -sSf https://raw.githubusercontent.com/BasaiCorp/Razen-New/main/install.sh | bash
```

The installer will detect your current version and prompt you to update if a newer version is available.

## Uninstalling

To remove Razen from your system:

```bash
# Remove installation directory
rm -rf ~/.razen

# Remove from PATH by editing your shell profile
# Remove the line: export PATH="$HOME/.razen/bin:$PATH"
```

After removing the PATH entry, restart your terminal for changes to take effect.
