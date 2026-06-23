#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

REPO_ROOT=""
if REPO_ROOT_GUESS="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null)"; then
  REPO_ROOT="$REPO_ROOT_GUESS"
else
  REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
fi

if [ ! -d "$REPO_ROOT/.git" ]; then
  echo "[ERROR] Could not resolve repository root from $SCRIPT_DIR"
  exit 1
fi

SILVERSCRIPT_DIR="$REPO_ROOT/external/silverscript"
FIXTURES_DIR="$REPO_ROOT/spikes/tn12-minimal-covenant/fixtures"
ARTIFACTS_DIR="$REPO_ROOT/spikes/tn12-minimal-covenant/artifacts"

if [ ! -d "$SILVERSCRIPT_DIR" ]; then
  echo "[ERROR] Missing external Silverscript clone at $SILVERSCRIPT_DIR"
  exit 1
fi

if command -v cargo >/dev/null 2>&1; then
  CARGO_CMD="$(command -v cargo)"
elif [ -x "/root/.cargo/bin/cargo" ]; then
  CARGO_CMD="/root/.cargo/bin/cargo"
else
  echo "[ERROR] cargo not found in PATH and /root/.cargo/bin/cargo is missing"
  exit 1
fi

mkdir -p "$ARTIFACTS_DIR"

check() {
  local name="$1"
  local script_path="$2"
  local test_path="$3"
  local log_file="$ARTIFACTS_DIR/${name}.log"

  echo "\n[INFO] Running $name"
  (cd "$SILVERSCRIPT_DIR" && "$CARGO_CMD" run -p cli-debugger -- "$script_path" --run-all --test-file "$test_path" | tee "$log_file")
}

check "simple-covenant-version2" \
  "$FIXTURES_DIR/simple_covenant.sil" \
  "$FIXTURES_DIR/simple_covenant.test.json"

check "transition-demo" \
  "$FIXTURES_DIR/tn12_demo_transition.sil" \
  "$FIXTURES_DIR/tn12_demo_transition.test.json"

check "simple-covenant-tx-structured" \
  "$FIXTURES_DIR/simple_covenant.sil" \
  "$FIXTURES_DIR/simple_covenant_tx_structured.test.json"

echo "\n[PASS] All local no-broadcast checks passed."
echo "[PASS] Logs are available in: $ARTIFACTS_DIR"
