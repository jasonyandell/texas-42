#!/usr/bin/env python3
"""Summarize a waking-seat census JSONL (exploratory tier; below every
evidentiary tier and cited by nothing above it).

Stdlib-only, Python 3.12. No floats: fractions report as exact
``Fraction`` strings, spend percentiles as exact integers (the
nearest-rank percentile of the sorted integer microsecond list).

Usage: python3 summarize.py driven.jsonl
"""

import json
import sys
from collections import Counter, defaultdict
from fractions import Fraction


def nearest_rank(sorted_values: list[int], percent: int) -> int:
    """The nearest-rank percentile: an actual observed integer."""
    assert sorted_values
    rank = -(-percent * len(sorted_values) // 100)  # ceil without floats
    rank = max(1, min(rank, len(sorted_values)))
    return sorted_values[rank - 1]


def spend_line(label: str, values: list[int]) -> str:
    if not values:
        return f"  {label}: (none)"
    values = sorted(values)
    return (
        f"  {label}: n={len(values)} min={values[0]} p50={nearest_rank(values, 50)} "
        f"p90={nearest_rank(values, 90)} p99={nearest_rank(values, 99)} "
        f"max={values[-1]} total={sum(values)}"
    )


def frac(n: int, d: int) -> str:
    if d == 0:
        return "0/0"
    f = Fraction(n, d)
    return f"{f.numerator}/{f.denominator} ({n} of {d})"


def main(path: str) -> None:
    census: list[dict] = []
    hands: list[dict] = []
    with open(path, encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            record = json.loads(line)
            if "path" in record:
                census.append(record)
            elif record.get("kind") == "hand":
                hands.append(record)

    total = len(census)
    print(f"census records (decisions): {total}")
    print(f"hand summaries: {len(hands)}")
    if hands:
        made = sum(1 for h in hands if h["made"])
        print(f"  bids made: {frac(made, len(hands))}")
    if not census:
        return

    # --- Phase attribution: where the microseconds actually go. -------
    def permille(part: int, whole: int) -> int:
        return (1000 * part) // whole if whole else 0

    b_total = sum(r["baseline_us"] for r in census)
    w_total = sum(r["wake_us"] for r in census)
    e_total = sum(r["escalation_us"] for r in census)
    grand = b_total + w_total + e_total
    print("PHASE ATTRIBUTION (integer micros; share in exact permille, floor):")
    print(f"  baseline (sigma0 act):  {b_total} ({permille(b_total, grand)} permille)")
    print(f"  wake check:             {w_total} ({permille(w_total, grand)} permille)")
    print(f"  escalation:             {e_total} ({permille(e_total, grand)} permille)")
    print(f"  grand total:            {grand}")
    print("  by trick (baseline / wake / escalation micros, dominant phase):")
    trick_phase: dict[int, list[int]] = defaultdict(lambda: [0, 0, 0])
    for r in census:
        t = trick_phase[r["trick"]]
        t[0] += r["baseline_us"]
        t[1] += r["wake_us"]
        t[2] += r["escalation_us"]
    for trick in sorted(trick_phase):
        b, w, e = trick_phase[trick]
        t_sum = b + w + e
        names = ["baseline", "wake-check", "escalation"]
        dominant = names[max(range(3), key=lambda k: trick_phase[trick][k])]
        print(
            f"    trick {trick}: {b} / {w} / {e}"
            f"  (trick total {t_sum}; dominant: {dominant},"
            f" {permille(max(b, w, e), t_sum)} permille)"
        )
    esc_phases: dict[str, list[int]] = defaultdict(lambda: [0, 0, 0])
    for r in census:
        if r["escalation"] is not None:
            for entry in r["escalation"].get("spend", []):
                slot = esc_phases[entry["phase"]]
                slot[0] += entry["micros"]
                slot[1] += entry["items"]
                slot[2] += 1
    if esc_phases:
        print("  escalation PhaseSpend breakdown (controller pipeline phases):")
        for phase, (micros, items, records) in sorted(
            esc_phases.items(), key=lambda kv: -kv[1][0]
        ):
            print(
                f"    {phase}: {micros} micros"
                f" ({permille(micros, e_total)} permille of escalation),"
                f" items={items}, records={records}"
            )

    paths = Counter(r["path"] for r in census)
    print("paths:")
    for path_tag, count in sorted(paths.items()):
        print(f"  {path_tag}: {frac(count, total)}")

    forced = paths.get("forced", 0)
    checked = total - forced
    wakes = paths.get("wake", 0)
    print(f"forced fraction: {frac(forced, total)}")
    print(f"wake-check rate (non-forced): {frac(checked, total)}")
    print(f"wake rate over checked decisions: {frac(wakes, checked)}")
    print(f"wake rate over all decisions: {frac(wakes, total)}")

    by_trick: dict[int, Counter] = defaultdict(Counter)
    for r in census:
        by_trick[r["trick"]][r["path"]] += 1
    print("by trick (decisions / checked / wakes):")
    for trick in sorted(by_trick):
        c = by_trick[trick]
        t_total = sum(c.values())
        t_checked = t_total - c.get("forced", 0)
        t_wakes = c.get("wake", 0)
        print(
            f"  trick {trick}: {t_total} / {t_checked} / {t_wakes}"
            f"  (wake rate {frac(t_wakes, t_checked)})"
        )

    kinds = Counter(r["wake_kind"] for r in census if r["wake_kind"] is not None)
    print("wake-check evidence kinds:")
    for kind, count in sorted(kinds.items()):
        print(f"  {kind}: {count}")

    escalations = [r["escalation"] for r in census if r["escalation"] is not None]
    if escalations:
        print("escalation outcomes (StageFourOutcome / EscalationStop / route):")
        outcome_stop = Counter((e["outcome"], e["stop"], e["via"]) for e in escalations)
        for (outcome, stop, via), count in sorted(outcome_stop.items()):
            print(f"  {outcome} / {stop} / {via}: {count}")

    agreed = sum(1 for r in census if r["agreed"])
    print(f"agreement with sigma0: {frac(agreed, total)}")
    moved = [r for r in census if not r["agreed"]]
    if moved:
        print(f"  moved off sigma0: {len(moved)} decision(s), at tricks "
              f"{sorted(r['trick'] for r in moved)}")

    print("spend (integer microseconds):")
    print(spend_line("baseline_us (all)", [r["baseline_us"] for r in census]))
    print(spend_line(
        "wake_us (checked)", [r["wake_us"] for r in census if r["path"] != "forced"]
    ))
    print(spend_line(
        "escalation_us (wakes)",
        [r["escalation_us"] for r in census if r["path"] == "wake"],
    ))
    print(spend_line(
        "decision total (all)",
        [r["baseline_us"] + r["wake_us"] + r["escalation_us"] for r in census],
    ))
    print(spend_line(
        "wake_worlds (checked)",
        [r["wake_worlds"] for r in census if r["path"] != "forced"],
    ))

    fibers = sorted(int(r["fiber"]) for r in census)
    print(
        f"fibers: min={fibers[0]} p50={nearest_rank(fibers, 50)} "
        f"p90={nearest_rank(fibers, 90)} max={fibers[-1]}"
    )


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "driven.jsonl")
