#!/usr/bin/env python3
"""Aggregate the shadow JSONL records into the README's plain counts.

EXPLORATORY instrument tooling — stdlib only, integer arithmetic only.
Usage: python3 summarize.py receipt.jsonl driven.jsonl ...
"""

import json
import sys
from collections import Counter


def median(xs):
    xs = sorted(xs)
    n = len(xs)
    if n == 0:
        return None
    if n % 2 == 1:
        return xs[n // 2]
    # Integer midpoint of the two central values (floor); an instrument
    # number, not a statistic with claims attached.
    return (xs[n // 2 - 1] + xs[n // 2]) // 2


def main(paths):
    decisions = []
    hands = []
    for path in paths:
        with open(path) as f:
            for line in f:
                r = json.loads(line)
                (decisions if r["kind"] == "decision" else hands).append(r)
    print(f"hands: {len(hands)}   decisions shadowed: {len(decisions)}")
    tags = Counter(r["shadow"]["tag"] for r in decisions)
    routes = Counter(r["shadow"]["route"] for r in decisions)
    print("result kinds:", dict(sorted(tags.items())))
    print("routes:      ", dict(sorted(routes.items())))
    escalations = sum(1 for r in decisions if r["shadow"]["route"] == "escalated")
    print("controller escalations fired:", escalations)

    with_winner = [r for r in decisions if r["agreement"] is not None]
    agree = sum(1 for r in with_winner if r["agreement"])
    print(f"decisions with a controller winner: {len(with_winner)}; "
          f"live agreement {agree}/{len(with_winner)}")
    surv = [r for r in decisions if r["live_in_survivors"] is not None]
    live_in = sum(1 for r in surv if r["live_in_survivors"])
    print(f"decisions left open (survivor sets): {len(surv)}; "
          f"live choice among survivors {live_in}/{len(surv)}")

    settled = [r["shadow"]["settled_at"] for r in decisions
               if r["shadow"]["tag"] == "DeltaSettled"]
    if settled:
        print(f"settlement indices (DeltaSettled): n={len(settled)} "
              f"min={min(settled)} median={median(settled)} max={max(settled)}")
    exact_ties = sum(1 for r in decisions
                     if r["shadow"]["tag"] == "ExactFrozenSet"
                     and r["shadow"]["winner"] is None)
    print("exact ties among ExactFrozenSet:", exact_ties)

    by_trick = Counter((r["trick"], r["shadow"]["tag"]) for r in decisions)
    print("per-trick result kinds:")
    for trick in range(1, 8):
        row = {tag: n for (t, tag), n in by_trick.items() if t == trick}
        if row:
            print(f"  trick {trick}: {dict(sorted(row.items()))}")

    live_us = [r["live"]["micros"] for r in decisions]
    shadow_us = [r["shadow"]["micros"] for r in decisions]
    print(f"live-eval micros:   median={median(live_us)} max={max(live_us)}")
    print(f"shadow-eval micros: median={median(shadow_us)} max={max(shadow_us)}")


if __name__ == "__main__":
    main(sys.argv[1:])
