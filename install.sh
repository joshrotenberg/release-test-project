#!/usr/bin/env bash

# Release Test Project Installation Script
# This script downloads and installs the latest release of release-test

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO="joshrotenberg/release-test-project"
BINARY_NAME="release-test"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Functions
print_error() {
    echo -e "${RED}Error: $1${NC}" >&2
}

print_success() {
    echo -e "${GREEN}$1${NC}"
}

print_info() {
    echo -e "${BLUE}$1${NC}"
}

print_warning() {
    echo -e "${YELLOW}$1${NC}"
}

detect_platform() {
    local os arch

    # Detect OS
    case "$(uname -s)" in
        Linux*)     os="linux";;
        Darwin*)    os="darwin";;
        CYGWIN*|MINGW*|MSYS*) os="windows";;
        *)          print_error "Unsupported OS: $(uname -s)"; exit 1;;
    esac

    # Detect architecture
    case "$(uname -m)" in
        x86_64|amd64)   arch="x86_64";;
        aarch64|arm64)  arch="aarch64";;
        *)              print_error "Unsupported architecture: $(uname -m)"; exit 1;;
    esac

    # Construct target triple
    case "${os}-${arch}" in
        linux-x86_64)   echo "x86_64-unknown-linux-gnu";;
        linux-aarch64)  echo "aarch64-unknown-linux-gnu";;
        darwin-x86_64)  echo "x86_64-apple-darwin";;
        darwin-aarch64) echo "aarch64-apple-darwin";;
        windows-x86_64) echo "x86_64-pc-windows-msvc";;
        *)              print_error "Unsupported platform: ${os}-${arch}"; exit 1;;
    esac
}

download_binary() {
    local target="$1"
    local version="$2"
    local download_url
    local archive_name
    local archive_ext

    # Determine archive extension
    if [[ "$target" == *"windows"* ]]; then
        archive_ext="zip"
    else
        archive_ext="tar.gz"
    fi

    archive_name="${BINARY_NAME}-${target}.${archive_ext}"
    
    # Construct download URL
    if [ "$version" = "latest" ]; then
        download_url="https://github.com/${REPO}/releases/latest/download/${archive_name}"
        print_info "Downloading latest version..."
    else
        download_url="https://github.com/${REPO}/releases/download/${version}/${archive_name}"
        print_info "Downloading version ${version}..."
    fi

    # Download the archive
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$download_url" -o "$archive_name"
    elif command -v wget >/dev/null 2>&1; then
        wget -q "$download_url" -O "$archive_name"
    else
        print_error "Neither curl nor wget found. Please install one of them."
        exit 1
    fi

    print_success "Download complete!"
    echo "$archive_name"
}

extract_binary() {
    local archive="$1"
    local target="$2"

    print_info "Extracting binary..."

    if [[ "$archive" == *.tar.gz ]]; then
        tar -xzf "$archive"
    elif [[ "$archive" == *.zip ]]; then
        unzip -q "$archive"
    else
        print_error "Unknown archive format: $archive"
        exit 1
    fi

    # Handle Windows .exe extension
    local binary_file="$BINARY_NAME"
    if [[ "$target" == *"windows"* ]]; then
        binary_file="${BINARY_NAME}.exe"
    fi

    if [ ! -f "$binary_file" ]; then
        print_error "Binary not found after extraction"
        exit 1
    fi

    print_success "Extraction complete!"
    echo "$binary_file"
}

install_binary() {
    local binary_file="$1"

    # Create install directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"

    # Move binary to install directory
    print_info "Installing to ${INSTALL_DIR}..."
    mv "$binary_file" "${INSTALL_DIR}/${BINARY_NAME}"
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

    print_success "Installation complete!"

    # Check if install directory is in PATH
    if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
        print_warning "Warning: ${INSTALL_DIR} is not in your PATH"
        print_info "Add the following to your shell configuration file:"
        echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
    else
        print_success "You can now run: ${BINARY_NAME} --help"
    fi
}

cleanup() {
    print_info "Cleaning up..."
    rm -f "${BINARY_NAME}-"*.tar.gz "${BINARY_NAME}-"*.zip "${BINARY_NAME}" "${BINARY_NAME}.exe" 2>/dev/null || true
}

main() {
    print_info "Release Test Project Installer"
    print_info "==============================="
    
    # Parse arguments
    local version="${1:-latest}"
    
    # Detect platform
    local target
    target=$(detect_platform)
    print_info "Detected platform: $target"

    # Create temp directory
    local temp_dir
    temp_dir=$(mktemp -d)
    cd "$temp_dir"

    # Set trap to cleanup on exit
    trap cleanup EXIT

    # Download binary
    local archive
    archive=$(download_binary "$target" "$version")

    # Extract binary
    local binary_file
    binary_file=$(extract_binary "$archive" "$target")

    # Install binary
    install_binary "$binary_file"

    # Cleanup is handled by trap
}

# Run main function
main "$@"