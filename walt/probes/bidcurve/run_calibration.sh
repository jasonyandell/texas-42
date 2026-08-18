#!/bin/sh -e
# EXPLORATORY bidcurve calibration corpus — overnight run.
#
# Three passes of the bidcurve probe over the SAME 200 frozen hands
# (hand h deals from BID_SEED ^ mix(h); the CRN worlds nest across
# passes: the first 40 worlds at n=200 are exactly the n=40 worlds, so
# cross-pass deltas are pure sample-size effect). Purpose: calibrate the
# auction threshold theta and the per-bid n against the known small-n
# saturation overbid. Estimates only; nothing here is quotable above
# exploratory tier; not a P-A21 statement.
#
# Run from anywhere: cd's to the walt workspace root itself.
cd "$(dirname "$0")/../.."
BIN=./target/release/bidcurve
HANDS=200
for pass in "12 small-n12" "40 live-n40" "200 ref-n200"; do
  set -- $pass
  n=$1
  name=$2
  out="probes/bidcurve/$name.log"
  echo "=== pass $name (n_outer=$n, hands 0..$((HANDS - 1))) start $(date) ==="
  "$BIN" "$HANDS" 0 "$n" 120 >"$out" 2>&1
  echo "=== pass $name done $(date) ==="
done
echo "=== all passes complete $(date) ==="
