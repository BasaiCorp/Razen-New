#!/bin/bash

# Razen Programming Language - Universal Installation Script
# This script downloads and installs the Razen compiler globally
# Inspired by rustup.rs installation approach

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
GITHUB_REPO="https://github.com/YOUR_USERNAME/razen-lang-new"  # Replace with your actual repo
GITHUB_RAW="https://raw.githubusercontent.com/YOUR_USERNAME/razen-lang-new/main"  # Replace with your actual repo
RAZEN_HOME="$HOME/.razen"
BINARY_NAME="razen"
TEMP_DIR="/tmp/razen-install-$$"

# Banner
echo -e "${CYAN}"
echo "██████╗  █████╗ ███████╗███████╗███╗   ██╗"
echo "██╔══██╗██╔══██╗╚══███╔╝██╔════╝████╗  ██║"
echo "██████╔╝███████║  ███╔╝ █████╗  ██╔██╗ ██║"
echo "██╔══██╗██╔══██║ ███╔╝  ██╔══╝  ██║╚██╗██║"
echo "██║  ██║██║  ██║███████╗███████╗██║ ╚████║"
echo "╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═══╝"
echo -e "${NC}"
echo -e "${BLUE}Universal Installation Script${NC}"
echo -e "${YELLOW}Professional Programming Language Compiler${NC}"
echo ""

# Check dependencies
check_dependencies() {
    echo -e "${BLUE}Checking dependencies...${NC}"
    
    if ! command -v curl >/dev/null 2>&1; then
        echo -e "${RED}Error: curl is required but not installed${NC}"
        echo -e "${YELLOW}Please install curl and try again${NC}"
        exit 1
    fi
    
    if ! command -v tar >/dev/null 2>&1; then
        echo -e "${RED}Error: tar is required but not installed${NC}"
        echo -e "${YELLOW}Please install tar and try again${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✓ Dependencies satisfied${NC}"
}

# Detect OS and architecture
detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)
    
    case "$os" in
        linux*) OS="linux" ;;
        darwin*) OS="macos" ;;
        mingw*|msys*|cygwin*) OS="windows" ;;
        *) 
            echo -e "${RED}Error: Unsupported operating system: $os${NC}"
            exit 1
            ;;
    esac
    
    case "$arch" in
        x86_64|amd64) ARCH="x64" ;;
        i386|i686) ARCH="x86" ;;
        aarch64|arm64) ARCH="arm64" ;;
        *)
            echo -e "${RED}Error: Unsupported architecture: $arch${NC}"
            exit 1
            ;;
    esac
    
    echo -e "${GREEN}✓ Detected platform: $OS-$ARCH${NC}"
}

# Download production folder from GitHub
download_razen() {
    echo -e "${BLUE}Downloading Razen from GitHub...${NC}"
    
    # Create temporary directory
    mkdir -p "$TEMP_DIR"
    cd "$TEMP_DIR"
    
    # Download the production folder as zip
    echo -e "${CYAN}Downloading production files...${NC}"
    curl -L -o production.zip "$GITHUB_REPO/archive/refs/heads/main.zip" || {
        echo -e "${RED}Error: Failed to download from GitHub${NC}"
        echo -e "${YELLOW}Please check your internet connection and repository URL${NC}"
        exit 1
    }
    
    # Extract the zip file
    echo -e "${CYAN}Extracting files...${NC}"
    unzip -q production.zip || {
        echo -e "${RED}Error: Failed to extract files${NC}"
        exit 1
    }
    
    # Find the production directory
    PRODUCTION_DIR=$(find . -name "production" -type d | head -1)
    if [ -z "$PRODUCTION_DIR" ]; then
        echo -e "${RED}Error: Production directory not found in download${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✓ Downloaded successfully${NC}"
}

# Install Razen to ~/.razen
install_razen() {
    echo -e "${BLUE}Installing Razen to $RAZEN_HOME...${NC}"
    
    # Remove existing installation
    if [ -d "$RAZEN_HOME" ]; then
        echo -e "${YELLOW}Removing existing installation...${NC}"
        rm -rf "$RAZEN_HOME"
    fi
    
    # Create .razen directory
    mkdir -p "$RAZEN_HOME"
    
    # Copy production contents to ~/.razen
    echo -e "${CYAN}Copying files...${NC}"
    cp -r "$PRODUCTION_DIR"/* "$RAZEN_HOME/"
    
    # Make binary executable
    if [ -f "$RAZEN_HOME/bin/razen-lang" ]; then
        chmod +x "$RAZEN_HOME/bin/razen-lang"
        
        # Create razen symlink
        ln -sf "$RAZEN_HOME/bin/razen-lang" "$RAZEN_HOME/bin/$BINARY_NAME"
        chmod +x "$RAZEN_HOME/bin/$BINARY_NAME"
    else
        echo -e "${RED}Error: Binary not found in production/bin/${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✓ Installation completed${NC}"
}

# Setup PATH environment
setup_path() {
    echo -e "${BLUE}Setting up PATH environment...${NC}"

    local razen_bin="$RAZEN_HOME/bin"
    local shell_profile=""

    # Detect shell and profile file
    if [ -n "$BASH_VERSION" ]; then
        shell_profile="$HOME/.bashrc"
    elif [ -n "$ZSH_VERSION" ]; then
        shell_profile="$HOME/.zshrc"
    else
        # Try to detect from $SHELL
        case "$SHELL" in
            */bash) shell_profile="$HOME/.bashrc" ;;
            */zsh) shell_profile="$HOME/.zshrc" ;;
            */fish) shell_profile="$HOME/.config/fish/config.fish" ;;
            *) shell_profile="$HOME/.profile" ;;
        esac
    fi

    # Check if already in PATH
    if [[ ":$PATH:" == *":$razen_bin:"* ]]; then
        echo -e "${GREEN}✓ Razen is already in PATH${NC}"
        return
    fi

    # Add to PATH
    echo -e "${CYAN}Adding $razen_bin to PATH in $shell_profile${NC}"

    if [ "$shell_profile" = "$HOME/.config/fish/config.fish" ]; then
        # Fish shell syntax
        mkdir -p "$(dirname "$shell_profile")"
        echo "set -gx PATH $razen_bin \$PATH" >> "$shell_profile"
        echo -e "${GREEN}✓ Added to Fish shell configuration${NC}"
    else
        # Bash/Zsh syntax
        echo "" >> "$shell_profile"
        echo "# Razen Programming Language" >> "$shell_profile"
        echo "export PATH=\"$razen_bin:\$PATH\"" >> "$shell_profile"
        echo -e "${GREEN}✓ Added to $shell_profile${NC}"
    fi

    # Also add to current session
    export PATH="$razen_bin:$PATH"

    echo -e "${GREEN}✓ PATH updated${NC}"
}

# Test installation
test_installation() {
    echo -e "${BLUE}Testing installation...${NC}"
    
    # Test if razen command works
    if command -v razen >/dev/null 2>&1; then
        echo -e "${GREEN}✓ 'razen' command is available${NC}"
        
        # Test version
        local version=$(razen --version 2>/dev/null || echo "unknown")
        echo -e "${CYAN}Version: $version${NC}"
        
        # Show usage
        echo ""
        echo -e "${BLUE}Usage Examples:${NC}"
        echo "  razen run program.rzn          # Compile and run"
        echo "  razen dev program.rzn          # Development mode"
        echo "  razen compile program.rzn      # Compile to executable"
        echo "  razen test program.rzn         # Run tests"
        echo "  razen --help                   # Show help"
        
    else
        echo -e "${YELLOW}⚠ 'razen' command not found in current session${NC}"
        echo -e "${YELLOW}Please restart your terminal or run: source ~/.bashrc${NC}"
    fi
}

# Cleanup
cleanup() {
    echo -e "${BLUE}Cleaning up...${NC}"
    rm -rf "$TEMP_DIR"
    echo -e "${GREEN}✓ Cleanup completed${NC}"
}

# Main installation process
main() {
    echo -e "${BLUE}Starting Razen installation...${NC}"
    echo ""
    
    check_dependencies
    detect_platform
    download_razen
    install_razen
    setup_path
    test_installation
    cleanup
    
    echo ""
    echo -e "${GREEN}🎉 Razen Programming Language installed successfully!${NC}"
    echo -e "${CYAN}Installation directory: $RAZEN_HOME${NC}"
    echo -e "${BLUE}Happy coding with Razen! 🚀${NC}"
    echo ""
    echo -e "${YELLOW}Next steps:${NC}"
    echo "1. Restart your terminal or run: source ~/.bashrc"
    echo "2. Try: razen --help"
    echo "3. Create your first Razen program!"
}

# Handle interruption
trap 'echo -e "\n${RED}Installation interrupted${NC}"; cleanup; exit 1' INT TERM

# Run main installation
main "$@"
