#!/usr/bin/env bash
# Source this file to set LLVM_PROFDATA and LLVM_COV for bazel coverage.
# Usage: source tools/coverage-env.sh && bazel coverage //...

_find_llvm_tool() {
  local tool="$1"
  for candidate in \
    "$(xcrun --find "$tool" 2>/dev/null)" \
    "$(brew --prefix llvm 2>/dev/null)/bin/$tool" \
    "$(command -v "$tool" 2>/dev/null)"; do
    if [ -x "$candidate" ]; then
      echo "$candidate"
      return
    fi
  done
  echo "warning: $tool not found. Install LLVM (e.g. brew install llvm)." >&2
}

export LLVM_PROFDATA="${LLVM_PROFDATA:-$(_find_llvm_tool llvm-profdata)}"
export LLVM_COV="${LLVM_COV:-$(_find_llvm_tool llvm-cov)}"
