# Installation Guide

This guide provides detailed instructions for installing the Razen programming language on various platforms.

## System Requirements

### Minimum Requirements

- Operating System: Linux, macOS, or Windows with Git Bash
- Architecture: x86_64 (64-bit)
- Disk Space: 50 MB for installation
- RAM: 512 MB minimum
- Internet connection for initial download

### Supported Platforms

- Linux (Ubuntu, Debian, Fedora, Arch, and other distributions)
- macOS (10.12 Sierra and later)
- Windows 10/11 (via Git Bash)

## Installation Methods

### Method 1: One-Line Installation (Recommended)

The quickest way to install Razen:

```bash
curl -sSf https://raw.githubusercontent.com/BasaiCorp/Razen-New/main/install.sh | bash
```

This method:
- Downloads the latest stable release
- Installs to `~/.razen/`
- Configures PATH automatically
- Verifies installation integrity

### Method 2: Manual Installation

For users who prefer to review the installation script:

```bash
# Download the installer
curl -O https://raw.githubusercontent.com/BasaiCorp/Razen-New/main/install.sh

# Inspect the script (recommended)
less install.sh

# Make executable
chmod +x install.sh

# Run installer
./install.sh
```

### Method 3: Building from Source

For developers who want to build from source:

```bash
# Clone the repository
git clone https://github.com/BasaiCorp/Razen-New.git
cd Razen-New

# Build with Cargo
cargo build --release

# Install to ~/.razen/bin
mkdir -p ~/.razen/bin
cp target/release/razen-lang ~/.razen/bin/
ln -s ~/.razen/bin/razen-lang ~/.razen/bin/razen

# Add to PATH
echo 'export PATH="$HOME/.razen/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Platform-Specific Instructions

### Linux Installation

#### Ubuntu/Debian

```bash
# Install curl if not present
sudo apt-get update
sudo apt-get install curl

# Run installer
curl -sSf https://raw.githubusercontent.com/BasaiCorp/Razen-New/main/install.sh | bash

# Reload shell configuration
source ~/.bashrc
```

#### Fedora/RHEL/CentOS

```bash
# Install curl if not present
sudo dnf install curl

# Run installer
curl -sSf https://raw.githubusercontent.com/BasaiCorp/Razen-New/main/install.sh | bash

# Reload shell configuration
source ~/.bashrc
```

#### Arch Linux

```bash
# Install curl if not present
sudo pacman -S curl

# Run installer
curl -sSf https://raw.githubusercontent.com/BasaiCorp/Razen-New/main/install.sh | bash

# Reload shell configuration
source ~/.bashrc
```

### macOS Installation

```bash
# Ensure curl is available (pre-installed on macOS)
# Run installer
curl -sSf https://raw.githubusercontent.com/BasaiCorp/Razen-New/main/install.sh | bash

# Reload shell configuration
source ~/.zshrc  # or ~/.bash_profile for older macOS versions
```

### Windows Installation

Windows users must use Git Bash for installation:

1. **Install Git for Windows**
   - Download from https://git-scm.com/download/win
   - Run the installer with default options
   - This includes Git Bash

2. **Open Git Bash**
   - Right-click on desktop or folder
   - Select "Git Bash Here"

3. **Run Installation Command**
   ```bash
   curl -sSf https://raw.githubusercontent.com/BasaiCorp/Razen-New/main/install.sh | bash
   ```

4. **Restart Git Bash**
   - Close and reopen Git Bash for PATH changes to take effect

## Installation Directory Structure

After installation, Razen creates the following structure:

```
~/.razen/
├── bin/
│   ├── razen-lang    # Main compiler binary
│   └── razen         # Symlink for convenience
├── version           # Version tracking file
└── scripts/          # Utility scripts (if any)
```

## Configuring PATH

The installer automatically adds Razen to your PATH by modifying your shell configuration file.

### Bash

The installer adds this line to `~/.bashrc`:
```bash
export PATH="$HOME/.razen/bin:$PATH"
```

### Zsh

For Zsh users (default on macOS), add to `~/.zshrc`:
```bash
export PATH="$HOME/.razen/bin:$PATH"
```

### Fish

For Fish shell users, add to `~/.config/fish/config.fish`:
```fish
set -gx PATH $HOME/.razen/bin $PATH
```

### Manual PATH Configuration

If automatic PATH configuration fails, add manually:

```bash
# Open your shell configuration file
nano ~/.bashrc  # or ~/.zshrc, ~/.bash_profile

# Add this line at the end
export PATH="$HOME/.razen/bin:$PATH"

# Save and reload
source ~/.bashrc
```

## Verifying Installation

After installation, verify Razen is correctly installed:

```bash
# Check version
razen --version

# Expected output:
# Razen v0.1-beta.7

# Check help
razen --help

# Test with a simple program
echo 'fun main() { println("Hello!") }' > test.rzn
razen run test.rzn
rm test.rzn
```

## Troubleshooting Installation

### Command Not Found

If `razen` command is not found after installation:

1. **Restart your terminal** - PATH changes require a new shell session
2. **Verify PATH** - Check if `~/.razen/bin` is in your PATH:
   ```bash
   echo $PATH | grep razen
   ```
3. **Manually source configuration**:
   ```bash
   source ~/.bashrc  # or ~/.zshrc
   ```

### Permission Denied

If you encounter permission errors:

```bash
# Ensure the binary is executable
chmod +x ~/.razen/bin/razen-lang

# Verify ownership
ls -la ~/.razen/bin/
```

### Download Failures

If the installer fails to download:

1. **Check internet connection**
2. **Verify curl is installed**: `curl --version`
3. **Try manual download**:
   ```bash
   curl -L -o razen-lang https://github.com/BasaiCorp/Razen-New/releases/latest/download/razen-lang
   ```

### Existing Installation

If Razen is already installed:

```bash
# The installer will detect existing installation
# and prompt for update or reinstall
./install.sh
```

## Updating Razen

To update to the latest version:

```bash
# Run the installer again
curl -sSf https://raw.githubusercontent.com/BasaiCorp/Razen-New/main/install.sh | bash
```

The installer will:
- Detect your current version
- Check for newer releases
- Prompt for update if available
- Preserve your configuration

### Manual Update

```bash
# Download latest binary
curl -L -o ~/.razen/bin/razen-lang https://github.com/BasaiCorp/Razen-New/releases/latest/download/razen-lang

# Make executable
chmod +x ~/.razen/bin/razen-lang

# Verify update
razen --version
```

## Uninstalling Razen

To completely remove Razen from your system:

```bash
# Remove installation directory
rm -rf ~/.razen

# Remove PATH entry from shell configuration
# Edit ~/.bashrc, ~/.zshrc, or equivalent
# Remove the line: export PATH="$HOME/.razen/bin:$PATH"

# Reload shell configuration
source ~/.bashrc  # or ~/.zshrc
```

### Verification of Uninstallation

```bash
# Should return "command not found"
razen --version

# Verify directory is removed
ls ~/.razen
```

## Multiple Versions

To install multiple versions of Razen:

```bash
# Install to custom directory
export RAZEN_HOME=~/.razen-0.1.7
curl -sSf https://raw.githubusercontent.com/BasaiCorp/Razen-New/main/install.sh | bash

# Switch versions by changing PATH
export PATH="$HOME/.razen-0.1.7/bin:$PATH"
```

## Development Installation

For contributors and developers:

```bash
# Clone repository
git clone https://github.com/BasaiCorp/Razen-New.git
cd Razen-New

# Install Rust if not present
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build in debug mode
cargo build

# Run tests
cargo test

# Build release version
cargo build --release

# Install locally
cargo install --path .
```

## Next Steps

After successful installation:

1. Read the [Getting Started Guide](getting-started.md)
2. Learn about [Variables and Data Types](variables-datatypes.md)
3. Explore [Example Programs](../examples/)
4. Join the community and contribute

## Support

For installation issues:

- Check [Troubleshooting Guide](troubleshooting.md)
- Search [GitHub Issues](https://github.com/BasaiCorp/Razen-New/issues)
- Report new issues with:
  - Operating system and version
  - Installation method used
  - Error messages received
  - Output of `razen --version`
