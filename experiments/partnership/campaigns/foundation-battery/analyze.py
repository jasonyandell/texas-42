#!/usr/bin/env python3
"""Rebuild the foundation battery's descriptive summary from saved evidence."""

import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parents[1]))

import campaign as c  # noqa: E402
from match import report  # noqa: E402


def main():
    paths = sorted(HERE.glob("[0-9][0-9]-*"))
    reports = {p.name: report(p) for p in paths}
    phone = HERE / "01-phone"
    spec = c.load(phone, verify=False)
    rows = c.complete_results(phone, spec)
    audit = {
        "pairs": len(rows),
        "fallback_free_pairs": 0,
        "fallback_free_identical_pairs": 0,
        "first_divergences": [],
        "common_prefix_decisions": 0,
    }
    for row in rows:
        games = [
            c.read(phone / "seeds" / str(row["seed"]) / arm / "checkpoint.json")[
                "decisions"
            ]
            for arm in c.arms_for(spec)
        ]
        fallback_free = not any(
            d["response"]["route"].endswith("fallback") for g in games for d in g
        )
        audit["fallback_free_pairs"] += fallback_free
        for i, (x, y) in enumerate(zip(*games)):
            if x["response"]["choice"] != y["response"]["choice"]:
                audit["first_divergences"].append(
                    {
                        "seed": row["seed"],
                        "decision": i,
                        "routes": [x["response"]["route"], y["response"]["route"]],
                        "choices": [x["response"]["choice"], y["response"]["choice"]],
                        "pair_score": row["paired"]["seed_delta"],
                    }
                )
                break
            audit["common_prefix_decisions"] += 1
        else:
            audit["fallback_free_identical_pairs"] += fallback_free
    c.atomic(HERE / "phone-trajectory-audit.json", audit)
    wall = sum(c.read(p)["elapsed_seconds"] for p in HERE.glob("cap-*/run.json"))
    games = sum(2 * r["completed_pairs"] for r in reports.values())
    body = """# Unified player foundation: results

2026-09-06. **EXPLORATORY.** The shared selection engine passes the foundation
checks. Native racing L1 reproduces the archived phone on completed matched
decisions. No selection rule or void option has established a general strength
gain; the tested partner-refinement configurations exceed the fallback gate.

## Completed matched panels

Bid 30 throughout. Each deal is played twice, with player partnerships swapped.
Pair score is make(A) minus make(B); both-make and both-set pairs tie regardless
of points. Win fraction is `(1 + mean pair score) / 2`, a comparative contract
score, not an estimate of a hand's absolute pmake. The random comparisons share
seeds 720600–720649; the conditional panel uses 820600–820649.

| A versus B | Paired deals | A wins / losses / ties | A contract win fraction |
|---|---:|---:|---:|
"""
    for name, r in reports.items():
        if r["runtime"]["stop"]:
            continue
        a, b = (r["players"][side]["name"] for side in ("a", "b"))
        score = r["a_contract_win_fraction"]
        body += f"| [{a} vs {b}]({name}/MATCH.md) | {r['completed_pairs']} | {r['a_pair_wins']} / {r['a_pair_losses']} / {r['pair_ties']} | {score['numerator'] / score['denominator']:.1%} |\n"
    body += """
The four random-panel rough intervals all include 50%; this is unresolved
strength evidence. The conditional comparison is only **five focal hands**,
each with ten hidden-hand completions. Three hands favor voidless by one pair;
two tie. It supplies no broad voids advantage. The per-hand table is retained
in its match report. Repeated completions do not count as independent hands.
The descriptive report suppresses its normal approximation below ten units
or when observed variance is zero; this presentation guard does not alter
recorded scores or the predeclared stopping rule.

## Phone fidelity and latency

"""
    body += f"All **{audit['fallback_free_identical_pairs']}/{audit['fallback_free_pairs']} fallback-free mirrored pairs** played identically for all 28 moves. "
    body += f"There were {len(audit['first_divergences'])} first divergences; every one began at a phone L1 fallback. "
    assert all("l1-fallback" in x["routes"] for x in audit["first_divergences"])
    body += """The 3/1 pair edge is therefore evidence about the bounded
implementations, not a discovered difference in their completed decision rule.
This complements the 64 pre-battery native/WASM decision comparisons. It is
observed parity, not a whole-program equivalence proof.

Representative means from the completed 50-deal matches (seconds per move,
including forced moves, wrapper overhead, and fallback preparation):

| Player | Seconds / move | Fallbacks / nonforced decisions | Matched report |
|---|---:|---:|---|
"""
    for name, player in [
        ("02-fixed", "l1-fixed"),
        ("03-refine", "l1-refine"),
        ("01-phone", "l1-race"),
        ("05-voids", "l1-race-voids"),
        ("01-phone", "phone"),
    ]:
        v = reports[name]["runtime"]["player_counts"][player]
        body += f"| {player} | {v['elapsed_us'] / 1e6 / v['moves']:.3f} | {v['fallbacks']} / {v['nonforced']} | [{name}]({name}/MATCH.md) |\n"
    body += """
These are wall latencies under the shared ten-game pool, not isolated CPU
benchmarks. All completed L1-only comparisons had zero native fallbacks.
The archived phone had 17 fallbacks in its 923 nonforced decisions.

## Partner modeling: cost stop, not a strength verdict

The original profiles and two declared cost-driven extensions all crossed
the >5% fallback gate after at least 20 nonforced decisions for that player.
No completed outcome was rerolled and no gate was relaxed. These short,
technically stopped prefixes do not establish the deeper policy's strength.

| Tested A configuration | Completed pairs | A fallback / nonforced | Mean seconds / move |
|---|---:|---:|---:|
"""
    for name, r in reports.items():
        if not r["runtime"]["stop"]:
            continue
        player = r["players"]["a"]["name"]
        v = r["runtime"]["player_counts"][player]
        body += f"| [{player}]({name}/MATCH.md) | {r['completed_pairs']} | {v['fallbacks']} / {v['nonforced']} | {v['elapsed_us'] / 1e6 / v['moves']:.3f} |\n"
    body += """
The original profile uses n=40/n0=8/n1=2 with race/refinement at both levels.
The small profiles use 8/8/1. The final diagnostic keeps the real root at
40/8 and race/refine while modeled L1 uses a single fixed two-world bundle.
Even that final variant exceeded the declared threshold on its first pair.
Root refinement over a partner field is itself costly; this experiment does
not isolate an implementation bottleneck. The earlier fixed-root partner
player remains available and is not refuted by these different configurations.
Under a clock cap, strengthening a modeled procedure need not strengthen
the executed player: it can increase fallback frequency.

## Durability and evidence

"""
    body += f"The four foreground pool slices consumed **{wall:.2f} seconds ({wall / 60:.2f} minutes)** in total. "
    body += f"They published **{games} games / {games * 28:,} moves**, all independently replay-verified. "
    body += """This includes 500 games in five completed
panels and 24 games in technically stopped prefixes. Additional speculative
work and stopped checkpoints remain on disk but do not enter those totals or
the primary scores. Wall time includes that work. The battery's four completed
random matchups use 50 shared deal units, not 200 independent deals.

All 4,639 moves saved at the first slice boundary survived subsequent resume
unchanged ([resume proof](resume-proof.json)). The separate pilot also tested
ten simultaneously interrupted games and worker restart. All workers are
stopped. Source/binary identities remain pinned; immutable manifests preserve
every experimental configuration independently of later preset edits.

The [protocol](PROTOCOL.md), [first extension](EXTENSION-1.md), and
[final diagnostic](EXTENSION-2.md) explain exactly what ran and why it stopped.
Each match contains its manifest, every move, per-seed paired scores,
per-player timing/fallback counts, and an independent verification record.
`analyze.py` regenerates this summary from saved data; it launches no players.

## Foundation verdict

Use `l1-race` as the native counterpart of the recovered phone procedure in
controlled comparisons. Keep `l1-fixed` and `l1-refine` as named cheaper
alternatives; this panel does not settle their strength order. Belief strategy
and partner modeling remain independent, explicit choices. Existing defaults
are unchanged. The next substantial engineering question is why partner-field
work reaches its budget, followed by a fresh cost-matched strength comparison.
No additional experiment is running or scheduled.

See [the design and validation record](../../FOUNDATION.md) for shared-engine
contracts and the bounded scope of the checks. This establishes a stronger
engineering foundation, not the original requested partner-strength claim.
"""
    c.atomic(HERE / "RESULTS.md", body)
    c.atomic(
        HERE / "RESULTS.json",
        {
            "pool_wall_seconds": wall,
            "published_games": games,
            "reports": reports,
            "phone_trajectory_audit": audit,
        },
    )
    print(
        json.dumps(
            {
                "pool_wall_seconds": wall,
                "published_games": games,
                "report": str(HERE / "RESULTS.md"),
            }
        )
    )


if __name__ == "__main__":
    main()
