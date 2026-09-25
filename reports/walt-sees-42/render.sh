#!/usr/bin/env bash
# Render how-walt-sees-42.mp4 in parallel segments, then join them losslessly.
set -euo pipefail
cd "$(dirname "$0")"
export NODE_PATH="${NODE_PATH:-$(npm root -g)}"
FF="${FFMPEG:-ffmpeg}"
TMP="$(mktemp -d)"
cuts=(0 42 84 125 166)
for i in 0 1 2 3; do
  FFMPEG="$FF" node render.mjs "$TMP/seg$i.mp4" 30 "${cuts[$i]}" "${cuts[$((i+1))]}" &
done
wait
for i in 0 1 2 3; do echo "file '$TMP/seg$i.mp4'"; done > "$TMP/list.txt"
"$FF" -y -loglevel error -f concat -safe 0 -i "$TMP/list.txt" -c copy -movflags +faststart how-walt-sees-42.mp4
echo "wrote how-walt-sees-42.mp4"
