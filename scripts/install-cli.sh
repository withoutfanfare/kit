#!/usr/bin/env bash
# install-cli.sh — build the Kit CLI in release mode and symlink into ~/bin.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN_SRC="$ROOT/src-tauri/target/release/kit"
INSTALL_DIR="$HOME/bin"
INSTALL_LINK="$INSTALL_DIR/kit"

echo "Building kit CLI (release)..."
(cd "$ROOT/src-tauri" && cargo build --bin kit --release)

if [[ ! -x "$BIN_SRC" ]]; then
  echo "Expected binary not found: $BIN_SRC" >&2
  exit 1
fi

mkdir -p "$INSTALL_DIR"

if [[ -L "$INSTALL_LINK" ]]; then
  rm -f "$INSTALL_LINK"
elif [[ -e "$INSTALL_LINK" ]]; then
  echo "Refusing to overwrite non-symlink at $INSTALL_LINK" >&2
  exit 1
fi

ln -s "$BIN_SRC" "$INSTALL_LINK"
echo "Installed: $INSTALL_LINK -> $BIN_SRC"
echo ""
echo "If '$INSTALL_DIR' is not in your PATH, add to ~/.zshrc:"
echo "  export PATH=\"\$HOME/bin:\$PATH\""
echo ""
"$INSTALL_LINK" --version
