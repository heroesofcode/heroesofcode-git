#!/usr/bin/env bash
set -e

# Simple wrapper for the hoc CLI
# Usage:
#   ./hoc.sh c
#   ./hoc.sh r

if ! command -v hoc >/dev/null 2>&1; then
  echo "❌ hoc is not installed or not in PATH"
  echo "Install it with:"
  echo "  cargo install --git https://github.com/heroesofcode/heroesofcode-git"
  exit 1
fi

cmd="$1"

case "$cmd" in
  c|clone)
    hoc clone
    ;;
  r|repos|list)
    hoc repos
    ;;
  ""|-h|--help)
    echo "Usage:"
    echo "  ./hoc.sh c -> hoc clone"
    echo "  ./hoc.sh r -> hoc repos"
    ;;
  *)
    echo "Unknown command: $cmd"
    echo "Run './hoc.sh --help' for usage."
    exit 2
    ;;
esac