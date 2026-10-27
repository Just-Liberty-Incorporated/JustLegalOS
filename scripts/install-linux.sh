#!/usr/bin/env bash
# ==============================================================================
# JustLegal OS — Linux CLI, Agent MCP Server & Prerequisite Installer
# Developed by Just Liberty Incorporated (Colorado 501(c)(3) Nonprofit)
# Canonical URL: https://github.com/Just-Liberty-Incorporated/JustLegalOS
# ==============================================================================
set -euo pipefail

BOLD='\033[1m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BOLD}${BLUE}=== JustLegal OS: Linux CLI & GrokBot Agent Installer ===${NC}"

AUTO_YES=false
INSTALL_DEPS=true
NO_SUDO=true
INIT_WORKSPACE=false

for arg in "$@"; do
    case "$arg" in
        -y|--yes)
            AUTO_YES=true
            ;;
        --sudo)
            NO_SUDO=false
            ;;
        --no-sudo)
            NO_SUDO=true
            ;;
        --no-deps)
            INSTALL_DEPS=false
            ;;
        --init-workspace)
            INIT_WORKSPACE=true
            ;;
    esac
done

if [ "${CI:-}" = "true" ] || [ "${DEBIAN_FRONTEND:-}" = "noninteractive" ]; then
    AUTO_YES=true
fi

# Detect OS
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
if [ "$OS" != "linux" ]; then
    echo -e "${RED}[ERROR] This installer is intended for Linux environments. Detected: $OS${NC}"
    echo -e "For Windows or macOS, download pre-packaged installers from https://justlegal.me"
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

# Determine install destination (consent-first, user-space preferred)
if [ "$NO_SUDO" = "true" ] || [ "$(id -u)" -ne 0 ]; then
    INSTALL_DIR="${HOME}/.local/bin"
    DATA_DIR="${HOME}/.local/share/justlegalos"
else
    INSTALL_DIR="/usr/local/bin"
    DATA_DIR="/usr/local/share/justlegalos"
fi

BIN_TARGET="${INSTALL_DIR}/justlegalos-cli"
LIB_TARGET="${INSTALL_DIR}/justlegalos_lib"

echo -e "\n${BOLD}Planned Installation Manifest:${NC}"
echo -e "  • Target binary wrapper:    ${BIN_TARGET}"
echo -e "  • Target library directory: ${LIB_TARGET}"
echo -e "  • Target data & fixtures:   ${DATA_DIR}"
echo -e "  • Engine Architecture:      Unified Python 3 + Native Rust Hybrid Engine"
echo -e "  • Core Data & Fixtures:     data/states/, data/authorities_registry.json, tests/fixtures (01-06)"
echo -e "  • Document Engine:          User-space static Typst (${INSTALL_DIR}/typst)"

# Interactive consent check: never crash when /dev/tty has no controlling terminal, and never auto-consent without -y
if [ "$AUTO_YES" = "false" ]; then
    CONFIRM="n"
    if [ -t 0 ]; then
        echo -ne "\n${YELLOW}Proceed with installation into ${INSTALL_DIR}? [y/N]: ${NC}"
        read -r CONFIRM || CONFIRM="n"
    elif ( : </dev/tty ) 2>/dev/null; then
        echo -ne "\n${YELLOW}Proceed with installation into ${INSTALL_DIR}? [y/N]: ${NC}" > /dev/tty
        read -r CONFIRM < /dev/tty || CONFIRM="n"
    else
        echo -e "\n${RED}[ERROR] No interactive terminal (TTY) available to prompt for user consent.${NC}" >&2
        echo -e "${YELLOW}To grant consent in non-interactive environments, pass '-y' or '--yes':${NC}" >&2
        echo -e "  curl -fsSL https://raw.githubusercontent.com/Just-Liberty-Incorporated/JustLegalOS/main/scripts/install-linux.sh | bash -s -- -y" >&2
        exit 1
    fi
    case "$CONFIRM" in
        [yY]|[yY][eE][sS])
            echo -e "${GREEN}[INFO] Consent granted. Proceeding...${NC}"
            ;;
        *)
            echo -e "${YELLOW}[INFO] Installation cancelled by user.${NC}"
            exit 0
            ;;
    esac
fi

mkdir -p "$INSTALL_DIR" "$DATA_DIR" "$LIB_TARGET"

# Set up user-space static Typst binary (uses Python lzma/tarfile so xz-utils CLI is NOT required)
if [ "$INSTALL_DEPS" = "true" ]; then
    if ! command -v typst >/dev/null 2>&1 && [ ! -f "${INSTALL_DIR}/typst" ]; then
        echo -e "${BLUE}[INFO] Installing static Typst binary to ${INSTALL_DIR}/typst...${NC}"
        TYPST_URL="https://github.com/typst/typst/releases/latest/download/typst-${TARGET_ARCH}-unknown-linux-musl.tar.xz"
        if command -v python3 >/dev/null 2>&1; then
            python3 - "$TYPST_URL" "${INSTALL_DIR}/typst" << 'EOF_PY_TYPST' || true
import sys, urllib.request, tarfile, io, os
url, dest = sys.argv[1], sys.argv[2]
req = urllib.request.Request(url, headers={"User-Agent": "JustLegalOS-Installer/0.8.0"})
with urllib.request.urlopen(req, timeout=30) as resp:
    raw = resp.read()
with tarfile.open(fileobj=io.BytesIO(raw), mode="r:xz") as tf:
    for member in tf.getmembers():
        if member.name.endswith("/typst") or member.name == "typst":
            f = tf.extractfile(member)
            if f:
                with open(dest, "wb") as out:
                    out.write(f.read())
                os.chmod(dest, 0o755)
                break
EOF_PY_TYPST
        fi
        if [ ! -f "${INSTALL_DIR}/typst" ] && command -v curl >/dev/null 2>&1 && command -v xz >/dev/null 2>&1; then
            curl -fsSL "$TYPST_URL" | tar -xJ --strip-components=1 -C "$INSTALL_DIR" "typst-${TARGET_ARCH}-unknown-linux-musl/typst" || true
            chmod +x "${INSTALL_DIR}/typst" 2>/dev/null || true
        fi
        if [ -f "${INSTALL_DIR}/typst" ]; then
            echo -e "${GREEN}[INFO] Installed user-space Typst binary:${NC} $("${INSTALL_DIR}/typst" --version 2>/dev/null || echo 'ready')"
        else
            echo -e "${YELLOW}[WARN] Could not install static Typst binary; built-in pure-Python PDF 1.4 writer will be used as fallback.${NC}"
        fi
    fi

    # If sudo is explicitly requested and available, install system helpers
    if [ "$NO_SUDO" = "false" ] && command -v apt-get >/dev/null 2>&1; then
        SUDO_CMD=""
        if [ "$(id -u)" -ne 0 ] && command -v sudo >/dev/null 2>&1; then
            SUDO_CMD="sudo"
        fi
        $SUDO_CMD apt-get update -y || true
        $SUDO_CMD apt-get install -y poppler-utils libheif-examples libimage-exiftool-perl fonts-noto-core fonts-noto-cjk || true
    fi
fi

# Install Python Engine, Canonical Data, & All 6 Golden Test Fixtures
echo -e "${GREEN}[INFO] Installing JustLegal OS engine modules, data, and test fixtures...${NC}"

SCRIPT_SRC_DIR=""
if [ -n "${BASH_SOURCE[0]:-}" ]; then
    SCRIPT_SRC_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" 2>/dev/null && pwd || echo "")"
fi
REPO_ROOT=""
if [ -n "$SCRIPT_SRC_DIR" ] && [ -f "${SCRIPT_SRC_DIR}/justlegal_fallback.py" ]; then
    REPO_ROOT="$(cd "${SCRIPT_SRC_DIR}/.." 2>/dev/null && pwd || echo "")"
fi

SCRIPTS_LIST="justlegal_fallback.py pdf_writer.py image_scrubber.py whip_prepare.py whip_gate5.py system_tools.py fixture_verifier.py lint_runner.py whip_commands.py records_commands.py accountability_commands.py case_commands.py mcp_server.py check_authorities.py"

if [ -n "$REPO_ROOT" ] && [ -f "${REPO_ROOT}/scripts/justlegal_fallback.py" ]; then
    for s in $SCRIPTS_LIST; do
        if [ -f "${REPO_ROOT}/scripts/${s}" ]; then
            cp -f "${REPO_ROOT}/scripts/${s}" "$LIB_TARGET/"
        fi
    done
    mkdir -p "${DATA_DIR}/data" "${DATA_DIR}/tests" "${INSTALL_DIR}/data" "${INSTALL_DIR}/tests" 2>/dev/null || true
    if [ -d "${REPO_ROOT}/data/states" ]; then
        mkdir -p "${LIB_TARGET}/states" "${DATA_DIR}/data/states" "${INSTALL_DIR}/data/states" 2>/dev/null || true
        cp -rf "${REPO_ROOT}/data/states/"* "${LIB_TARGET}/states/" 2>/dev/null || true
        cp -rf "${REPO_ROOT}/data/states/"* "${DATA_DIR}/data/states/" 2>/dev/null || true
        cp -rf "${REPO_ROOT}/data/states/"* "${INSTALL_DIR}/data/states/" 2>/dev/null || true
    fi
    if [ -f "${REPO_ROOT}/data/authorities_registry.json" ]; then
        cp -f "${REPO_ROOT}/data/authorities_registry.json" "${DATA_DIR}/data/authorities_registry.json"
        cp -f "${REPO_ROOT}/data/authorities_registry.json" "${INSTALL_DIR}/data/authorities_registry.json" 2>/dev/null || true
    fi
    if [ -d "${REPO_ROOT}/tests/fixtures" ]; then
        cp -rf "${REPO_ROOT}/tests/fixtures" "${DATA_DIR}/tests/"
        cp -rf "${REPO_ROOT}/tests/fixtures" "${INSTALL_DIR}/tests/" 2>/dev/null || true
    fi
    if [ -f "${REPO_ROOT}/src-tauri/target/release/justlegalos-cli" ]; then
        cp -f "${REPO_ROOT}/src-tauri/target/release/justlegalos-cli" "${LIB_TARGET}/justlegalos-cli-rust" 2>/dev/null || true
        chmod +x "${LIB_TARGET}/justlegalos-cli-rust" 2>/dev/null || true
    elif [ -f "${REPO_ROOT}/JustLegalOS-CLI.exe" ]; then
        cp -f "${REPO_ROOT}/JustLegalOS-CLI.exe" "${LIB_TARGET}/justlegalos-cli-rust.exe" 2>/dev/null || true
    fi
else
    # Remote curl | bash mode: download tarball containing scripts/, data/, and tests/fixtures/
    TARBALL_URL="https://codeload.github.com/Just-Liberty-Incorporated/JustLegalOS/tar.gz/refs/tags/v0.8.0"
    python3 - "$TARBALL_URL" "$LIB_TARGET" "$DATA_DIR" "$INSTALL_DIR" << 'EOF_PY_BUNDLE'
import sys, urllib.request, tarfile, io, os, shutil

url, lib_target, data_dir, install_dir = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
urls = [url]
raw = None
for u in urls:
    try:
        req = urllib.request.Request(u, headers={"User-Agent": "JustLegalOS-Installer/0.8.0"})
        with urllib.request.urlopen(req, timeout=45) as resp:
            data = resp.read()
            if b"<!doctype" in data[:128].lower() or b"<html" in data[:128].lower():
                continue
            raw = data
            break
    except Exception:
        continue

if not raw:
    sys.exit("Error: Failed to retrieve JustLegal OS engine payload from GitHub or justlegal.me mirror (anti-HTML gate triggered or network error).")

os.makedirs(os.path.join(data_dir, "data"), exist_ok=True)
os.makedirs(os.path.join(data_dir, "tests", "fixtures"), exist_ok=True)
os.makedirs(os.path.join(install_dir, "data"), exist_ok=True)
os.makedirs(os.path.join(install_dir, "tests", "fixtures"), exist_ok=True)

with tarfile.open(fileobj=io.BytesIO(raw), mode="r:gz") as tf:
    for member in tf.getmembers():
        if not member.isfile():
            continue
        parts = member.name.split("/", 1)
        if len(parts) < 2:
            continue
        rel = parts[1]
        if rel.startswith("scripts/") and rel.endswith(".py"):
            f = tf.extractfile(member)
            if f:
                with open(os.path.join(lib_target, os.path.basename(rel)), "wb") as out:
                    out.write(f.read())
        elif rel.startswith("data/states/") or rel in ("data/states_50.json", "data/authorities_registry.json"):
            f = tf.extractfile(member)
            if f:
                b = f.read()
                fname = os.path.basename(rel)
                if rel.startswith("data/states/"):
                    for base_d in (data_dir, install_dir):
                        dp = os.path.join(base_d, "data", "states", fname)
                        os.makedirs(os.path.dirname(dp), exist_ok=True)
                        with open(dp, "wb") as out:
                            out.write(b)
                    lp = os.path.join(lib_target, "states", fname)
                    os.makedirs(os.path.dirname(lp), exist_ok=True)
                    with open(lp, "wb") as out:
                        out.write(b)
                else:
                    with open(os.path.join(data_dir, "data", fname), "wb") as out:
                        out.write(b)
                    with open(os.path.join(install_dir, "data", fname), "wb") as out:
                        out.write(b)
        elif rel.startswith("tests/fixtures/"):
            f = tf.extractfile(member)
            if f:
                b = f.read()
                for base_d in (data_dir, install_dir):
                    dest_p = os.path.join(base_d, rel)
                    os.makedirs(os.path.dirname(dest_p), exist_ok=True)
                    with open(dest_p, "wb") as out:
                        out.write(b)
EOF_PY_BUNDLE

    # Download pre-compiled native Rust binary asset as hybrid companion
    RUST_BIN_URL="https://github.com/Just-Liberty-Incorporated/JustLegalOS/releases/download/v0.8.0/justlegalos-cli-linux-${TARGET_ARCH}"
    SUMS_URL="https://github.com/Just-Liberty-Incorporated/JustLegalOS/releases/download/v0.8.0/SHA256SUMS.txt"
    RUST_MIRROR_URL="https://justlegal.me/dl/justlegalos-cli-linux-${TARGET_ARCH}"
    for r_url in "$RUST_BIN_URL" "$RUST_MIRROR_URL"; do
        if curl -fsSL -A "JustLegalOS-Installer/0.8.0" "$r_url" -o "${LIB_TARGET}/justlegalos-cli-rust" 2>/dev/null; then
            if head -c 4 "${LIB_TARGET}/justlegalos-cli-rust" | grep -q $'\x7fELF'; then
                if [ "$r_url" = "$RUST_BIN_URL" ]; then
                    SUMS_FILE="${LIB_TARGET}/SHA256SUMS.txt"
                    ASSET_NAME="justlegalos-cli-linux-${TARGET_ARCH}"
                    if ! curl -fsSL -A "JustLegalOS-Installer/0.8.0" "$SUMS_URL" -o "$SUMS_FILE"; then
                        echo "[ERROR] Could not download SHA256SUMS.txt for v0.8.0. The binary was not installed." >&2
                        rm -f "${LIB_TARGET}/justlegalos-cli-rust"
                        continue
                    fi
                    EXPECTED="$(awk -v name="$ASSET_NAME" '$2==name {print $1}' "$SUMS_FILE" | head -n 1)"
                    ACTUAL="$(sha256sum "${LIB_TARGET}/justlegalos-cli-rust" | awk '{print $1}')"
                    if [ -z "$EXPECTED" ] || [ "$EXPECTED" != "$ACTUAL" ]; then
                        echo "[ERROR] Checksum mismatch for ${ASSET_NAME}. The binary was not installed." >&2
                        rm -f "${LIB_TARGET}/justlegalos-cli-rust"
                        continue
                    fi
                fi
                chmod +x "${LIB_TARGET}/justlegalos-cli-rust"
                break
            else
                rm -f "${LIB_TARGET}/justlegalos-cli-rust"
            fi
        fi
    done
fi

# Write canonical executable wrapper
cat << 'EOF_WRAPPER' > "$BIN_TARGET"
#!/usr/bin/env bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" 2>/dev/null && pwd || echo "")"
LIB_DIR="${SCRIPT_DIR}/justlegalos_lib"

# Dual-Engine Hybrid Dispatch (Native Rust prioritized, Python fallback):
if [ -x "${LIB_DIR}/justlegalos-cli-rust" ] && [ "${JUSTLEGAL_FORCE_PYTHON:-0}" != "1" ]; then
    exec "${LIB_DIR}/justlegalos-cli-rust" "$@"
elif command -v python3 >/dev/null 2>&1 && [ -f "${LIB_DIR}/justlegal_fallback.py" ]; then
    exec python3 "${LIB_DIR}/justlegal_fallback.py" "$@"
elif [ -x "${LIB_DIR}/justlegalos-cli-rust" ]; then
    exec "${LIB_DIR}/justlegalos-cli-rust" "$@"
elif [ -n "${JUSTLEGAL_REPO_ROOT:-}" ] && [ -f "${JUSTLEGAL_REPO_ROOT}/scripts/justlegal_fallback.py" ]; then
    exec python3 "${JUSTLEGAL_REPO_ROOT}/scripts/justlegal_fallback.py" "$@"
else
    echo "[ERROR] JustLegal OS engine scripts or binary not found at ${LIB_DIR}." >&2
    exit 1
fi
EOF_WRAPPER

chmod +x "$BIN_TARGET"
echo -e "${GREEN}[SUCCESS] Installed CLI executable to:${NC} $BIN_TARGET"

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

CASES_DIR="${HOME}/Documents/JustLegal/Cases"
PACKS_DIR="${HOME}/Documents/JustLegal/Packs"
mkdir -p "$CASES_DIR" "$PACKS_DIR"

if [ "$INIT_WORKSPACE" = "true" ]; then
    WORKSPACE_ROOT="${WORKSPACE:-/workspace}"
    echo -e "${BLUE}[INFO] Initializing shared workspace at ${WORKSPACE_ROOT}...${NC}"
    mkdir -p "${WORKSPACE_ROOT}/bin"
    cp -f "$BIN_TARGET" "${WORKSPACE_ROOT}/bin/justlegalos-cli" 2>/dev/null || ln -sf "$BIN_TARGET" "${WORKSPACE_ROOT}/bin/justlegalos-cli" 2>/dev/null || true
    chmod +x "${WORKSPACE_ROOT}/bin/justlegalos-cli" 2>/dev/null || true
fi

echo -e "\n${BOLD}${BLUE}=== System Verification (doctor) ===${NC}"
"$BIN_TARGET" --json doctor 2>/dev/null || true

echo -e "\n${BOLD}${GREEN}✔ JustLegal OS is ready for human and agent operation!${NC}\n"
