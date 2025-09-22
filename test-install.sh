#!/bin/bash

# Local installation test script
# This simulates the GitHub download process locally

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Check for existing installation and version comparison (local test version)
check_and_prompt_update() {
    if [ -d "$RAZEN_HOME" ] && [ -f "$RAZEN_HOME/version" ]; then
        echo -e "${BLUE}Checking for updates...${NC}"

        # Read local version
        local LOCAL_VERSION=$(cat "$RAZEN_HOME/version" 2>/dev/null)
        echo -e "${CYAN}Current version: $LOCAL_VERSION${NC}"

        # Read local production version
        if [ -f "production/version" ]; then
            local REMOTE_VERSION=$(cat "production/version" 2>/dev/null)

            if [ "$LOCAL_VERSION" = "$REMOTE_VERSION" ]; then
                echo -e "${YELLOW}Razen is already installed with the same version (${LOCAL_VERSION})${NC}"
                echo ""
                read -p "Do you want to reinstall anyway? (y/N): " -n 1 -r
                echo ""

                if [[ $REPLY =~ ^[Yy]$ ]]; then
                    echo -e "${BLUE}Reinstalling Razen...${NC}"
                    return 0
                else
                    echo -e "${YELLOW}Reinstallation cancelled. Razen is already at version $LOCAL_VERSION${NC}"
                    echo ""
                    echo -e "${BLUE}Usage Examples:${NC}"
                    echo "  razen run program.rzn          # Compile and run"
                    echo "  razen dev program.rzn          # Development mode"
                    echo "  razen compile program.rzn      # Compile to executable"
                    echo "  razen test program.rzn         # Run tests"
                    echo "  razen --help                   # Show help"
                    echo ""
                    exit 0
                fi
            else
                echo -e "${YELLOW}New version available:${NC}"
                echo -e "${CYAN}  Current: $LOCAL_VERSION${NC}"
                echo -e "${CYAN}  Local:   $REMOTE_VERSION${NC}"
                echo ""

                read -p "Do you want to update Razen? (y/N): " -n 1 -r
                echo ""

                if [[ $REPLY =~ ^[Yy]$ ]]; then
                    echo -e "${BLUE}Updating Razen...${NC}"
                    return 0
                else
                    echo -e "${YELLOW}Update cancelled. Razen is still at version $LOCAL_VERSION${NC}"
                    echo ""
                    echo -e "${BLUE}Usage Examples:${NC}"
                    echo "  razen run program.rzn          # Compile and run"
                    echo "  razen dev program.rzn          # Development mode"
                    echo "  razen compile program.rzn      # Compile to executable"
                    echo "  razen test program.rzn         # Run tests"
                    echo "  razen --help                   # Show help"
                    echo ""
                    exit 0
                fi
            fi
        else
            echo -e "${YELLOW}⚠ Could not check local production/version file${NC}"
            echo -e "${YELLOW}Proceeding with reinstallation...${NC}"
            return 0
        fi
    else
        echo -e "${GREEN}Fresh installation detected${NC}"
        return 0
    fi
}

echo -e "${BLUE}🧪 Testing Razen Installation Locally${NC}"
echo ""

# Set up local variables FIRST
RAZEN_HOME="$HOME/.razen"
BINARY_NAME="razen"

# Check for existing installation and version
check_and_prompt_update

# Remove existing installation
if [ -d "$RAZEN_HOME" ]; then
    echo -e "${YELLOW}Removing existing installation...${NC}"
    rm -rf "$RAZEN_HOME"
fi

# Create .razen directory
echo -e "${BLUE}Creating $RAZEN_HOME...${NC}"
mkdir -p "$RAZEN_HOME"

# Copy production contents to ~/.razen
echo -e "${BLUE}Copying production files...${NC}"
cp -r production/* "$RAZEN_HOME/"

# Make binary executable and create symlink
if [ -f "$RAZEN_HOME/bin/razen-lang" ]; then
    chmod +x "$RAZEN_HOME/bin/razen-lang"
    ln -sf "$RAZEN_HOME/bin/razen-lang" "$RAZEN_HOME/bin/$BINARY_NAME"
    chmod +x "$RAZEN_HOME/bin/$BINARY_NAME"
    echo -e "${GREEN}✓ Binary setup completed${NC}"
else
    echo -e "${RED}Error: Binary not found${NC}"
    exit 1
fi

# Setup PATH permanently
setup_path_permanent() {
    local razen_bin="$RAZEN_HOME/bin"
    local shell_profile=""

    echo -e "${BLUE}Setting up PATH permanently...${NC}"

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
        echo -e "${YELLOW}Please run 'source $shell_profile' or restart your terminal${NC}"
    fi
}

# Setup PATH for current session and permanently
setup_path_permanent

# Add to current session immediately
export PATH="$RAZEN_HOME/bin:$PATH"

# Test the installation
echo -e "${BLUE}Testing installation...${NC}"
if command -v razen >/dev/null 2>&1; then
    echo -e "${GREEN}✓ 'razen' command is available${NC}"
    razen --version
    echo ""
    echo -e "${BLUE}Testing with hello.rzn...${NC}"
    razen run hello.rzn
else
    echo -e "${RED}⚠ 'razen' not found in current session${NC}"
    echo -e "${YELLOW}Please run 'source ~/.bashrc' or restart your terminal${NC}"
fi

echo ""
echo -e "${GREEN}🎉 Local installation test completed!${NC}"
echo -e "${BLUE}Razen is now available globally!${NC}"
echo ""
echo -e "${YELLOW}Installation summary:${NC}"
echo "  • Binary: $RAZEN_HOME/bin/razen"
echo "  • PATH: Added to $shell_profile"
echo "  • Status: Ready to use from anywhere"
echo ""
echo -e "${GREEN}Try these commands from any directory:${NC}"
echo "  razen --help"
echo "  razen run hello.rzn"
echo "  razen dev hello.rzn"
