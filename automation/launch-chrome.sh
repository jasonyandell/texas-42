#!/bin/zsh
# Launch a second, CDP-enabled Chrome instance on a *copy* of Jason's profile.
# macOS Chrome >=136 refuses --remote-debugging-port on the default profile dir,
# and we don't want to kill the user's running Chrome anyway. Cookies are
# Keychain-encrypted ("Chrome Safe Storage") per-user, so a same-user copy
# still decrypts.
set -euo pipefail

SRC="$HOME/Library/Application Support/Google/Chrome"
DST="$HOME/Library/Application Support/Google/Chrome-buddy"
PORT="${CDP_PORT:-9222}"

if curl -sf "http://127.0.0.1:$PORT/json/version" >/dev/null 2>&1; then
  echo "CDP already listening on $PORT"
  exit 0
fi

if [[ "${1:-}" == "--fresh" || ! -d "$DST/Default" ]]; then
  rm -rf "$DST"
  mkdir -p "$DST/Default"
  cp "$SRC/Local State" "$DST/Local State"
  for f in Cookies Preferences "Secure Preferences" "Login Data" "Web Data" "Trust Tokens"; do
    [[ -f "$SRC/Default/$f" ]] && cp "$SRC/Default/$f" "$DST/Default/$f"
  done
  echo "Profile copied to $DST"
fi

nohup "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
  --user-data-dir="$DST" \
  --remote-debugging-port="$PORT" \
  --no-first-run --no-default-browser-check \
  --disable-session-crashed-bubble --hide-crash-restore-bubble \
  --window-size=1440,1000 \
  >/Users/jason/code/texas-42/automation/logs/chrome.out 2>&1 &

for i in {1..30}; do
  if curl -sf "http://127.0.0.1:$PORT/json/version" >/dev/null 2>&1; then
    echo "CDP up on $PORT"
    exit 0
  fi
  sleep 1
done
echo "Chrome failed to expose CDP on $PORT" >&2
exit 1
