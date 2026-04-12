#!/usr/bin/env bash
# Source this file to set LLVM_PROFDATA and LLVM_COV for bazel coverage.
# Usage: source tools/coverage-env.sh && bazel coverage //...

_find_llvm_tool() {
  local tool="$1"
  local _candidate
  case "$(uname -s)" in
    Darwin)
      _candidate="$(xcrun --find "$tool" 2>/dev/null)" && [ -x "$_candidate" ] && echo "$_candidate" && return
      _candidate="$(brew --prefix llvm 2>/dev/null)/bin/$tool" && [ -x "$_candidate" ] && echo "$_candidate" && return
      ;;
    Linux)
      _candidate="$(command -v "$tool" 2>/dev/null)" && [ -x "$_candidate" ] && echo "$_candidate" && return
      for v in 20 19 18 17 16 15; do
        _candidate="$(command -v "${tool}-${v}" 2>/dev/null)" && [ -x "$_candidate" ] && echo "$_candidate" && return
      done
      ;;
  esac
  echo "warning: $tool not found. Install LLVM (e.g. brew install llvm / apt install llvm)." >&2
}

export LLVM_PROFDATA="${LLVM_PROFDATA:-$(_find_llvm_tool llvm-profdata)}"
export LLVM_COV="${LLVM_COV:-$(_find_llvm_tool llvm-cov)}"
