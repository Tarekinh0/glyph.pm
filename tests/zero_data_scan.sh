#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-.}"
FORBIDDEN_REGEX='\b(sqlx|diesel|postgres|mongodb|redis|sqlite|rusqlite|mysql|tokio[-_]postgres|sea[-_]orm|sentry|datadog|newrelic|posthog|mixpanel|segment)\b'

if matches=$(rg -n --hidden --glob '!**/target/**' --glob 'Cargo.toml' --glob '*.rs' -e "$FORBIDDEN_REGEX" "$ROOT" 2>/dev/null); then
  if [ -n "$matches" ]; then
    printf 'Forbidden dependency detected:\n%s\n' "$matches" >&2
    exit 1
  fi
else
  rc=$?
  if [ "$rc" -gt 1 ]; then
    exit "$rc"
  fi
fi
