"""Aggregate the step-8 probe records (stdlib only).

Usage: python3 summarize.py v5.jsonl e0.jsonl
Reproduces the counts quoted in README.md from the committed JSONL.
"""
import json
import sys
from fractions import Fraction


def frac(s):
    n, d = s.split("/")
    return Fraction(int(n), int(d))


def summarize_v5(path):
    print(f"== V5 ladder ({path})")
    kinds = {}
    for line in open(path):
        r = json.loads(line)
        tags = [step["tag"] for step in r["ladder"]]
        verdict = r["verdict"].split(" ")[0].rstrip("{")
        kinds[verdict] = kinds.get(verdict, 0) + 1
        name = (
            f"{r['mode']} h{r['hand']} d{r['d']}"
            if r["kind"] == "flip"
            else f"count-timing g{r['g']}"
        )
        exact = ""
        if r["kind"] == "flip":
            exact = f" | exact winner {r['exact']['winner_tile']} vs live {r['live_tile']}"
        settled = [
            f"cap {step['cap']}: {step['tag']}"
            + (f" -> {step['winner_tile']}@{step['settled_at']}"
               if step["tag"] == "DeltaSettled" else "")
            + (f" -> exact {step['winner_tile']} (switch @{step['escalated_at']})"
               if step["tag"] == "ExactFrozenSet" else "")
            for step in r["ladder"]
        ]
        print(f"  {name}: {verdict}{exact}")
        for s in settled:
            print(f"    {s}")
        assert len(set(t for t in tags if t != "Unresolved")) <= 1, "V5: one settled story"
    print(f"  verdicts: {kinds}")


def summarize_e0(path):
    print(f"== E0 per-pair calibration ({path})")
    settled_within_dp = 0
    settled_total = 0
    unresolved = 0
    for line in open(path):
        r = json.loads(line)
        obs = r["observed"]
        s = [o for o in obs if o["tag"] == "DeltaSettled"]
        u = [o for o in obs if o["tag"] == "Unresolved"]
        settled_total += len(s)
        unresolved += len(u)
        dp = r["forecast"]["dp_half"]["crossing"]
        lead = r["forecast"]["leading_order"]
        tau = r["exact"]["tau"]
        name = f"{r['mode']} h{r['hand']} pair({r['i']},{r['j']})"
        obs_str = (
            ",".join(str(o["settled_at"]) for o in s) if s else "unresolved x%d" % len(u)
        )
        lead_str = (
            f"[{float(frac(lead[0])):.0f},{float(frac(lead[1])):.0f}]" if lead else "none"
        )
        print(
            f"  {name}: q={r['exact']['q']} tau={tau} | dp1/2={dp} lead~{lead_str} | obs {obs_str}"
        )
        if dp is not None and s:
            settled_within_dp += 1
    print(
        f"  replicates: {settled_total} DeltaSettled, {unresolved} honest Unresolved"
    )


if __name__ == "__main__":
    v5 = sys.argv[1] if len(sys.argv) > 1 else "v5.jsonl"
    e0 = sys.argv[2] if len(sys.argv) > 2 else "e0.jsonl"
    summarize_v5(v5)
    summarize_e0(e0)
