#!/usr/bin/env bash
set -euo pipefail

TOOLS=(git node npm python3 cargo rustc codex)
missing=()

printf 'Environment check\n'
printf '================\n\n'

for tool in "${TOOLS[@]}"; do
  if command -v "$tool" >/dev/null 2>&1; then
    version="$($tool --version 2>/dev/null | head -n 1)"
    printf '[OK] %-8s %s\n' "$tool" "$version"
  else
    printf '[MISSING] %s\n' "$tool"
    missing+=("$tool")
  fi
done

printf '\nSummary:\n'
if (( ${#missing[@]} > 0 )); then
  printf 'Missing tools (%d): %s\n' "${#missing[@]}" "${missing[*]}"
  printf 'No changes made; install anything missing manually before running covenant experiments.\n'
  exit 1
fi

printf 'All listed tools are present.\n'
