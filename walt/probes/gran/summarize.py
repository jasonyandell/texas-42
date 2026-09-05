#!/usr/bin/env python3
"""Summarize a granrun census file (replay or driven). Stdlib only, integers
only — shares are exact permille, floored. EXPLORATORY tier."""
import json
import sys
from collections import Counter

TILE = [f"{h}-{l}" for h in range(7) for l in range(h + 1)]


def permille(part, whole):
    return part * 1000 // whole if whole else 0


def main(path):
    rows = [json.loads(line) for line in open(path)]
    cfg = next((r for r in rows if r.get("kind") == "config"), None)
    census = [r for r in rows if "path" in r]
    compare = [r for r in rows if r.get("kind") == "compare"]
    hand = next((r for r in rows if r.get("kind") == "hand"), None)
    summary = next((r for r in rows if r.get("kind") == "replay-summary"), None)

    if cfg:
        print(f"mode {cfg['mode']}, anchor {cfg['anchor']}")
    print(f"census records (decisions): {len(census)}")

    base = sum(r["baseline_us"] for r in census)
    wake = sum(r["wake_us"] for r in census)
    esc = sum(r["escalation_us"] for r in census)
    total = base + wake + esc
    print("PHASE ATTRIBUTION (integer micros; share in exact permille, floor):")
    print(f"  baseline (sigma0 act):  {base} ({permille(base, total)} permille)")
    print(f"  wake check:             {wake} ({permille(wake, total)} permille)")
    print(f"  escalation:             {esc} ({permille(esc, total)} permille)")
    print(f"  grand total:            {total}")
    print("  by trick (baseline / wake / escalation micros):")
    for t in range(1, 8):
        tr = [r for r in census if r["trick"] == t]
        if not tr:
            continue
        b = sum(r["baseline_us"] for r in tr)
        w = sum(r["wake_us"] for r in tr)
        e = sum(r["escalation_us"] for r in tr)
        print(
            f"    trick {t}: {b} / {w} / {e}  (trick total {b + w + e}, "
            f"{permille(b + w + e, total)} permille of the run)"
        )

    print("PATHS: " + ", ".join(f"{k} {v}" for k, v in sorted(Counter(r["path"] for r in census).items())))
    print(
        "WAKE EVIDENCE: "
        + ", ".join(f"{k} {v}" for k, v in sorted(Counter(str(r["wake_kind"]) for r in census).items()))
    )
    woke = [r for r in census if r["path"] == "wake"]
    checked = [r for r in census if r["path"] != "forced"]
    print(f"wake rate: {len(woke)}/{len(checked)} checked decisions")
    print(f"agreement with sigma0: {sum(1 for r in census if r['agreed'])}/{len(census)}")
    for r in woke:
        e = r["escalation"]
        print(
            f"  wake at trick {r['trick']} (d{r['d']}, seat S{r['seat']}, fiber {r['fiber']}): "
            f"sigma0 {TILE[r['sigma0']]} -> played {TILE[r['played']]}; "
            f"escalation {e['outcome']} / {e['stop']} / {e['via']}"
        )
        for p in e["spend"]:
            print(f"    spend {p['phase']}: {p['micros']} us over {p['items']} items")

    if compare:
        print("PER-DECISION (record vs waking):")
        for c in compare:
            legal = " ".join(TILE[i] for i in c["legal"])
            print(
                f"  trick {c['trick']} d{c['d']}: legal {{{legal}}}; "
                f"record {TILE[c['record_played']]}, sigma0 {TILE[c['sigma0']]}, "
                f"waking {TILE[c['waking_played']]}, "
                f"{'AGREES' if c['agreed_with_record'] else 'DIFFERS'}; "
                f"wall {c['wall_us']} us"
            )
    if summary:
        print(
            f"replay summary: {summary['decisions']} decisions, "
            f"{summary['agreed_with_record']} matched the record, "
            f"sigma1 cache {summary['sigma1_cache']}"
        )
    if hand:
        print(
            f"hand: banked T0 {hand['banked'][0]} - {hand['banked'][1]} T1, "
            f"made={hand['made']}, {hand['decisions']} decisions, "
            f"sigma1 cache {hand['sigma1_cache']}"
        )


if __name__ == "__main__":
    main(sys.argv[1])
