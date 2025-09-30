#!/usr/bin/env sh
set -e
ROOT_DIR=$(git rev-parse --show-toplevel 2>/dev/null || echo ".")
HOOKS_DIR="$ROOT_DIR/.git/hooks"
GITHOOKS_DIR="$ROOT_DIR/.githooks"

if [ ! -d "$HOOKS_DIR" ]; then
  echo "No .git/hooks directory found. Are you in a git repo?"
  exit 1
fi

cp "$GITHOOKS_DIR/pre-commit" "$HOOKS_DIR/pre-commit"
chmod +x "$HOOKS_DIR/pre-commit"
echo "Installed pre-commit hook to $HOOKS_DIR/pre-commit"
