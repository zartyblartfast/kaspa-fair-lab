#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="/root/kaspa-fair-lab"
KASPA_CARGO_DIR="/root/.cargo/git/checkouts/rusty-kaspa-410e06d1fde91a92/42b734f"
KASPA_MANIFEST_PATH="$KASPA_CARGO_DIR/Cargo.toml"
ARTIFACT_DIR="$REPO_ROOT/spikes/tn12-minimal-covenant/artifacts"
LOG_PATH="$ARTIFACT_DIR/env-042-kaspad-30min-sync.log"
RUNNER_LOG="$ARTIFACT_DIR/env-042-runner.log"
LISTENER_CHECK="$ARTIFACT_DIR/env-042-post-stop-listeners.txt"
TARGET_URL="grpc://127.0.0.1:16210"
OBS_SECONDS="${OBS_SECONDS:-1800}"

mkdir -p "$ARTIFACT_DIR"
: > "$RUNNER_LOG"

log() {
  printf '%s %s\n' "$(date -u '+%Y-%m-%dT%H:%M:%SZ')" "$*" | tee -a "$RUNNER_LOG"
}

fail_workspace() {
  log "$1"
  log "Refusing to run cargo from repo root."
  exit 1
}

validate_kaspa_workspace() {
  if [[ ! -d "$KASPA_CARGO_DIR" ]]; then
    fail_workspace "ERROR: rusty-kaspa source directory not found: $KASPA_CARGO_DIR"
  fi

  if [[ ! -f "$KASPA_MANIFEST_PATH" ]]; then
    fail_workspace "ERROR: rusty-kaspa Cargo workspace not found. Expected Cargo.toml at: $KASPA_MANIFEST_PATH"
  fi

  if ! grep -Eq '^\s*"kaspad"\s*,?$|^\s*kaspad\s*=' "$KASPA_MANIFEST_PATH"; then
    fail_workspace "ERROR: $KASPA_MANIFEST_PATH does not look like the expected rusty-kaspa workspace (missing kaspad workspace/binary entry)."
  fi

  log "rusty_kaspa_workspace=$KASPA_CARGO_DIR"
  log "rusty_kaspa_manifest=$KASPA_MANIFEST_PATH"
}

cleanup() {
  if [[ -n "${KASPAD_SHELL_PID:-}" ]]; then
    if kill -0 "$KASPAD_SHELL_PID" 2>/dev/null; then
      log "cleanup: terminating process group for shell pid $KASPAD_SHELL_PID"
      kill -TERM -- "-$KASPAD_SHELL_PID" 2>/dev/null || kill -TERM "$KASPAD_SHELL_PID" 2>/dev/null || true
      wait "$KASPAD_SHELL_PID" 2>/dev/null || true
    fi
  fi
}
trap cleanup EXIT

validate_kaspa_workspace()

if ss -ltnp | grep -Eq '127\.0\.0\.1:(16210|16311|17210)'; then
  log "preflight failure: required localhost ports already in use"
  ss -ltnp | grep -E '127\.0\.0\.1:(16210|16311|17210)' | tee -a "$RUNNER_LOG"
  exit 1
fi

export LIBCLANG_PATH="/usr/lib/llvm-18/lib"
export BINDGEN_EXTRA_CLANG_ARGS="-isystem /usr/lib/gcc/x86_64-linux-gnu/13/include -isystem /usr/include"
export PROTOC="/usr/bin/protoc"

log "starting env-042 observation"
log "startup_command=cargo run --release --manifest-path $KASPA_MANIFEST_PATH --bin kaspad -- --testnet --netsuffix=12 --disable-upnp --listen=127.0.0.1:16311 --rpclisten=127.0.0.1:16210 --rpclisten-borsh=127.0.0.1:17210"
log "observation_seconds=$OBS_SECONDS"

(
  export LIBCLANG_PATH BINDGEN_EXTRA_CLANG_ARGS PROTOC
  cargo run --release --manifest-path "$KASPA_MANIFEST_PATH" --bin kaspad -- --testnet --netsuffix=12 --disable-upnp --listen=127.0.0.1:16311 --rpclisten=127.0.0.1:16210 --rpclisten-borsh=127.0.0.1:17210
) >"$LOG_PATH" 2>&1 &
KASPAD_SHELL_PID=$!
log "kaspad_shell_pid=$KASPAD_SHELL_PID"

READY=0
for _ in $(seq 1 240); do
  if grep -q 'GRPC Server starting on: 127.0.0.1:16210' "$LOG_PATH" \
    && grep -q 'P2P Server starting on: 127.0.0.1:16311' "$LOG_PATH" \
    && grep -q 'WRPC Server starting on: 127.0.0.1:17210' "$LOG_PATH"; then
    READY=1
    break
  fi
  if ! kill -0 "$KASPAD_SHELL_PID" 2>/dev/null; then
    log "kaspad exited before readiness"
    tail -n 200 "$LOG_PATH" | tee -a "$RUNNER_LOG"
    exit 1
  fi
  sleep 1
done

if [[ "$READY" != "1" ]]; then
  log "readiness timeout after 240 seconds"
  tail -n 200 "$LOG_PATH" | tee -a "$RUNNER_LOG"
  exit 1
fi

log "readiness confirmed"
ss -ltnp | grep -E '127\.0\.0\.1:(16210|16311|17210)' | tee -a "$RUNNER_LOG"

START_TS=$(date +%s)
END_TS=$((START_TS + OBS_SECONDS))
while (( $(date +%s) < END_TS )); do
  if ! kill -0 "$KASPAD_SHELL_PID" 2>/dev/null; then
    log "kaspad exited during observation"
    tail -n 200 "$LOG_PATH" | tee -a "$RUNNER_LOG"
    exit 1
  fi
  sleep 5
done

log "observation window complete; running read-only checks"
cargo run --quiet --manifest-path "$REPO_ROOT/spikes/tn12-minimal-covenant/rpc-readonly-suite/Cargo.toml" -- "$TARGET_URL" "$ARTIFACT_DIR" | tee -a "$RUNNER_LOG"

log "stopping kaspad"
kill -TERM -- "-$KASPAD_SHELL_PID" 2>/dev/null || kill -TERM "$KASPAD_SHELL_PID" 2>/dev/null || true
wait "$KASPAD_SHELL_PID" 2>/dev/null || true
unset KASPAD_SHELL_PID

ss -ltnp | grep -E '127\.0\.0\.1:(16210|16311|17210)' > "$LISTENER_CHECK" || true
log "post-stop listener capture saved to $LISTENER_CHECK"
if [[ -s "$LISTENER_CHECK" ]]; then
  log "post-stop listener verification failed"
  cat "$LISTENER_CHECK" | tee -a "$RUNNER_LOG"
  exit 1
fi

log "post-stop listener verification passed"
log "env-042 observation complete"
