#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-.}"
FORBIDDEN_REGEX='(^|[^[:alnum:]_])(sqlx|diesel|postgres|mongodb|redis|sqlite|rusqlite|mysql|tokio[-_]postgres|sea[-_]orm|sentry|datadog|newrelic|posthog|mixpanel|segment)([^[:alnum:]_]|$)'

if command -v rg >/dev/null 2>&1; then
  if matches=$(rg -n --hidden --glob '!**/target/**' --glob 'Cargo.toml' --glob '*.rs' -e "$FORBIDDEN_REGEX" "$ROOT" 2>/dev/null); then
    if [ -n "$matches" ]; then
      printf 'Forbidden dependency detected:
%s
' "$matches" >&2
      exit 1
    fi
  else
    rc=$?
    if [ "$rc" -gt 1 ]; then
      exit "$rc"
    fi
  fi
else
  if matches=$(grep -RIn --exclude-dir=target --include='Cargo.toml' --include='*.rs' -E "$FORBIDDEN_REGEX" "$ROOT" 2>/dev/null); then
    if [ -n "$matches" ]; then
      printf 'Forbidden dependency detected:
%s
' "$matches" >&2
      exit 1
    fi
  else
    rc=$?
    if [ "$rc" -gt 1 ]; then
      exit "$rc"
    fi
  fi
fi
