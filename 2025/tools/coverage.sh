#!/usr/bin/env bash
# Run bazel coverage and generate an HTML report.
# Usage: tools/coverage.sh [bazel coverage args...]
set -euo pipefail

bazel coverage //... "$@"

REPORT="$(bazel info output_path 2>/dev/null)/_coverage/_coverage_report.dat"
if [ ! -s "$REPORT" ]; then
  echo "error: no coverage data produced" >&2
  exit 1
fi

OUTDIR="coverage-report"
if command -v genhtml >/dev/null 2>&1; then
  genhtml "$REPORT" --output-directory "$OUTDIR" --quiet --ignore-errors unsupported
else
  # Fall back to llvm-cov: convert LCOV back to profdata + use show --format=html
  # genhtml gives nicer output, so suggest installing lcov
  echo "tip: install lcov (brew install lcov / apt install lcov) for better HTML reports"
  echo ""
  # Just print the summary since llvm-cov can't consume LCOV directly
  cat "$REPORT" | awk '
    /^SF:/ { file=substr($0,4) }
    /^LF:/ { lf=substr($0,4) }
    /^LH:/ { lh=substr($0,4); if (lf+0 > 0) printf "%-60s %s/%s (%5.1f%%)\n", file, lh, lf, (lh/lf)*100 }
  '
  exit 0
fi

echo "Coverage report: ${OUTDIR}/index.html"
case "$(uname -s)" in
  Darwin) open "$OUTDIR/index.html" ;;
  Linux)  xdg-open "$OUTDIR/index.html" 2>/dev/null || echo "Open ${OUTDIR}/index.html in a browser" ;;
esac
