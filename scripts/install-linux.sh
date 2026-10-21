#!/usr/bin/env bash
# ==============================================================================
# JustLegal OS — Headless Linux CLI & Agent MCP Server Installer
# Developed by Just Liberty Incorporated (Colorado 501(c)(3) Nonprofit)
# Canonical URL: https://github.com/Just-Liberty-Incorporated/JustLegalOS
# ==============================================================================
set -euo pipefail

BOLD='\033[1m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BOLD}${BLUE}=== JustLegal OS: Linux CLI & GrokBot Agent Installer ===${NC}"

# Detect OS
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
if [ "$OS" != "linux" ]; then
    echo -e "${RED}[ERROR] This installer is intended for Linux environments. Detected: $OS${NC}"
    echo -e "For Windows or macOS, download the pre-packaged installers from https://justlegal.me"
    exit 1
fi

# Detect Architecture
ARCH="$(uname -m)"
case "$ARCH" in
    x86_64|amd64)
        TARGET_ARCH="x86_64"
        ;;
    aarch64|arm64)
        TARGET_ARCH="aarch64"
        ;;
    *)
        echo -e "${RED}[ERROR] Unsupported architecture: $ARCH. Supported: x86_64, aarch64${NC}"
        exit 1
        ;;
esac

echo -e "${GREEN}[INFO] Detected architecture:${NC} $TARGET_ARCH"

# Determine install destination
if [ "$(id -u)" -eq 0 ]; then
    INSTALL_DIR="/usr/local/bin"
else
    INSTALL_DIR="${HOME}/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

BIN_TARGET="${INSTALL_DIR}/justlegalos-cli"

# Check if compiling locally from source (if repo present) or downloading prebuilt
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." 2>/dev/null && pwd || echo "")"

if [ -n "$REPO_ROOT" ] && [ -f "${REPO_ROOT}/src-tauri/Cargo.toml" ] && command -v cargo >/dev/null 2>&1; then
    echo -e "${BLUE}[INFO] Found local repository source. Building headless binary with Cargo...${NC}"
    cargo build --release --manifest-path "${REPO_ROOT}/src-tauri/Cargo.toml" --no-default-features --bin justlegalos-cli
    cp -f "${REPO_ROOT}/src-tauri/target/release/justlegalos-cli" "$BIN_TARGET"
else
    # Download latest release binary from GitHub Releases
    GITHUB_REPO="${JUSTLEGAL_GITHUB_REPO:-Just-Liberty-Incorporated/JustLegalOS}"
    DOWNLOAD_URL="https://github.com/${GITHUB_REPO}/releases/latest/download/justlegalos-cli-linux-${TARGET_ARCH}"
    CHECKSUM_URL="https://github.com/${GITHUB_REPO}/releases/latest/download/SHA256SUMS.txt"
    echo -e "${BLUE}[INFO] Downloading precompiled binary from:${NC} $DOWNLOAD_URL"

    TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/justlegalos-install.XXXXXX")"
    trap 'rm -rf "$TMP_DIR"' EXIT
    TMP_BIN="${TMP_DIR}/justlegalos-cli"
    TMP_SUMS="${TMP_DIR}/SHA256SUMS.txt"

    if command -v curl >/dev/null 2>&1; then
        if ! curl -fsSL "$DOWNLOAD_URL" -o "$TMP_BIN"; then
            echo -e "${RED}[ERROR] Failed to download binary from $DOWNLOAD_URL.${NC}"
            echo -e "Ensure a release has been published for ${GITHUB_REPO}."
            exit 1
        fi
        if ! curl -fsSL "$CHECKSUM_URL" -o "$TMP_SUMS"; then
            echo -e "${RED}[ERROR] Failed to download checksum manifest from $CHECKSUM_URL.${NC}"
            echo -e "Refusing to install unverified binary."
            exit 1
        fi
    elif command -v wget >/dev/null 2>&1; then
        if ! wget -qO "$TMP_BIN" "$DOWNLOAD_URL"; then
            echo -e "${RED}[ERROR] Failed to download binary from $DOWNLOAD_URL.${NC}"
            exit 1
        fi
        if ! wget -qO "$TMP_SUMS" "$CHECKSUM_URL"; then
            echo -e "${RED}[ERROR] Failed to download checksum manifest from $CHECKSUM_URL.${NC}"
            echo -e "Refusing to install unverified binary."
            exit 1
        fi
    else
        echo -e "${RED}[ERROR] Neither curl nor wget found. Please install curl or wget.${NC}"
        exit 1
    fi

    if [ ! -s "$TMP_BIN" ]; then
        echo -e "${RED}[ERROR] Downloaded binary is empty or corrupted.${NC}"
        exit 1
    fi

    echo -e "${BLUE}[INFO] Verifying SHA-256 checksum...${NC}"
    EXPECTED_HASH="$(grep "justlegalos-cli-linux-${TARGET_ARCH}" "$TMP_SUMS" 2>/dev/null | awk '{print $1}' || echo "")"
    if [ -z "$EXPECTED_HASH" ]; then
        echo -e "${RED}[ERROR] Checksum entry for justlegalos-cli-linux-${TARGET_ARCH} not found in SHA256SUMS.txt.${NC}"
        echo -e "Refusing to install unverified binary."
        exit 1
    fi

    if command -v sha256sum >/dev/null 2>&1; then
        ACTUAL_HASH="$(sha256sum "$TMP_BIN" | awk '{print $1}')"
    elif command -v shasum >/dev/null 2>&1; then
        ACTUAL_HASH="$(shasum -a 256 "$TMP_BIN" | awk '{print $1}')"
    else
        echo -e "${RED}[ERROR] Neither sha256sum nor shasum found to verify checksum.${NC}"
        exit 1
    fi

    if [ "$EXPECTED_HASH" != "$ACTUAL_HASH" ]; then
        echo -e "${RED}[ERROR] Checksum mismatch!${NC}"
        echo -e "Expected: $EXPECTED_HASH"
        echo -e "Actual:   $ACTUAL_HASH"
        echo -e "Refusing to install corrupted or tampered binary."
        exit 1
    fi
    echo -e "${GREEN}[INFO] SHA-256 checksum verified successfully: ${ACTUAL_HASH}${NC}"

    mv -f "$TMP_BIN" "$BIN_TARGET"
fi

if [ -f "$BIN_TARGET" ]; then
    chmod +x "$BIN_TARGET"
    echo -e "${GREEN}[SUCCESS] Installed binary to:${NC} $BIN_TARGET"
else
    echo -e "${RED}[ERROR] Installation failed: binary could not be generated or placed.${NC}"
    exit 1
fi

# Ensure user PATH includes INSTALL_DIR
if [ "$INSTALL_DIR" = "${HOME}/.local/bin" ]; then
    case ":${PATH}:" in
        *":${INSTALL_DIR}:"*) ;;
        *)
            echo -e "${YELLOW}[INFO] Adding ${INSTALL_DIR} to your shell PATH...${NC}"
            SHELL_RC=""
            if [ -n "${BASH_VERSION:-}" ]; then
                SHELL_RC="${HOME}/.bashrc"
            elif [ -n "${ZSH_VERSION:-}" ]; then
                SHELL_RC="${HOME}/.zshrc"
            elif [ -f "${HOME}/.profile" ]; then
                SHELL_RC="${HOME}/.profile"
            fi
            if [ -n "$SHELL_RC" ] && [ -f "$SHELL_RC" ]; then
                echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_RC"
                export PATH="$HOME/.local/bin:$PATH"
            fi
            ;;
    esac
fi

# Setup standard JustLegal folders
CASES_DIR="${HOME}/Documents/JustLegal/Cases"
PACKS_DIR="${HOME}/Documents/JustLegal/Packs"
mkdir -p "$CASES_DIR" "$PACKS_DIR"

# Initialize shared /workspace if running inside x.ai GrokBot or container sandbox
if [ -d "/workspace" ] || [ -n "${WORKSPACE:-}" ]; then
    WORKSPACE_ROOT="${WORKSPACE:-/workspace}"
    echo -e "${BLUE}[INFO] Detected agent environment at ${WORKSPACE_ROOT}. Initializing shared workspace...${NC}"
    mkdir -p "${WORKSPACE_ROOT}/bin"
    cp -f "$BIN_TARGET" "${WORKSPACE_ROOT}/bin/justlegalos-cli" 2>/dev/null || ln -sf "$BIN_TARGET" "${WORKSPACE_ROOT}/bin/justlegalos-cli" 2>/dev/null || true
    chmod +x "${WORKSPACE_ROOT}/bin/justlegalos-cli" 2>/dev/null || true
    "$BIN_TARGET" --json init-workspace --root "$WORKSPACE_ROOT" || true
    echo -e "${GREEN}[INFO] Shared agent skills & templates initialized at ${WORKSPACE_ROOT}.${NC}"
fi

echo -e "${GREEN}[INFO] Legal OS workspace initialized:${NC}"
echo -e "  • Cases directory: $CASES_DIR"
echo -e "  • Packs directory: $PACKS_DIR"

# Verify with doctor command
echo -e "\n${BOLD}${BLUE}=== Verifying System Health ===${NC}"
"$BIN_TARGET" --json doctor || true

echo -e "\n${BOLD}${GREEN}✔ JustLegal OS is ready for human and agent operation!${NC}\n"
echo -e "${BOLD}To launch the Bot-Safe MCP Server (for GrokBot, Claude Desktop, Cursor):${NC}"
echo -e "  ${YELLOW}$BIN_TARGET mcp --safe${NC}\n"
echo -e "${BOLD}Add to your agent's MCP configuration:${NC}"
cat << EOF
{
  "mcpServers": {
    "justlegalos": {
      "command": "$BIN_TARGET",
      "args": ["mcp", "--safe"]
    }
  }
}
EOF
echo ""
