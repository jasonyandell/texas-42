#!/usr/bin/env python3
"""Predeclared single-look analysis of the bidcurve calibration corpus.

EXPLORATORY — estimates, never receipts; not a P-A21 statement.

O14 discipline (SCENARIO-PLAYER.md ledger; CENSUS-RULINGS.md SP-A rulings):
this analysis is committed BEFORE the n=200 reference pass completes and is
run ONCE on the finished corpus. No resample-until-separated, no peeking
loop. If a follow-up analysis is wanted, it is declared as a new script
with its own commit, never by editing the numbers out of this one.

Inputs: probes/bidcurve/{small-n12,live-n40,ref-n200}.log — three passes
over the SAME 200 frozen hands with nested CRN worlds (first 40 worlds of
the n=200 pass are exactly the n=40 pass's worlds; first 12 likewise), so
cross-pass deltas are pure sample-size effect.

Declared outputs, in order:
  1. Coverage + DIED-cell counts and soft monotonicity-violation counts
     per pass (noise measure).
  2. Reliability table n=40 -> n=200: bucket each cell's n=40 bp into
     bands; report count and exact mean n=200 bp per band. The saturated
     band (bp == 10000) is reported separately — it is the known
     saturation-overbid suspect.
  3. Same table for n=12 -> n=200.
  4. Auction simulation on the n=40 curves for theta in
     {1/2, 9/16, 5/8, 11/16, 3/4, 13/16, 7/8, 15/16, 1}: the live bid rule
     (price all nine declarations at need=30; pass if best < theta; else
     best declaration first-max in id order, walk b up while
     P(make b+1) >= theta). Score each simulated bid by the n=200
     reference bp of the CHOSEN (declaration, b) cell. Report: bids/200,
     mean reference bp of chosen cells (exact fraction + rounded),
     overbid count (reference bp < 5000), mean final bid level,
     and passes where the reference's own theta=1/2 rule would have bid
     (missed-bid count).
  5. Same simulation on the n=12 curves (degradation direction).
  6. Declaration-choice agreement: n=12 vs n=40 vs n=200 first-max
     declaration at b=30, over the 200 hands.

All arithmetic exact (integers and Fraction). No floats.
"""
import re
import sys
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
PASSES = {"n12": "small-n12.log", "n40": "live-n40.log", "n200": "ref-n200.log"}
DECLS = ["P0", "P1", "P2", "P3", "P4", "P5", "P6", "DT", "NT"]
BIDS = list(range(30, 43))
THETAS = [F(1, 2), F(9, 16), F(5, 8), F(11, 16), F(3, 4), F(13, 16), F(7, 8), F(15, 16), F(1)]

CELL = re.compile(r"(\d{2}):\s*(-?\d+|DIED)")
HAND = re.compile(r"^hand (\d+):")
ROW = re.compile(r"^\s+(P\d|DT|NT)\s+(.*)$")
MONO = re.compile(r"\[mono-viol x(\d+)\]")


def parse(path):
    """-> {hand: {decl: {bid: bp or None}}}, mono_total, died_total"""
    hands, mono, died = {}, 0, 0
    cur = None
    for line in path.read_text().splitlines():
        m = HAND.match(line)
        if m:
            cur = int(m.group(1))
            hands[cur] = {}
            continue
        m = ROW.match(line)
        if m and cur is not None:
            decl, rest = m.group(1), m.group(2)
            mv = MONO.search(rest)
            if mv:
                mono += int(mv.group(1))
            row = {}
            for b, v in CELL.findall(rest):
                if v == "DIED":
                    row[int(b)] = None
                    died += 1
                else:
                    row[int(b)] = int(v)
            hands[cur][decl] = row
    return hands, mono, died


def first_max_decl(curves, b):
    best, bd = None, None
    for d in DECLS:
        v = curves.get(d, {}).get(b)
        if v is None:
            continue
        if best is None or v > best:
            best, bd = v, d
    return bd, best


def simulate_bid(curves, theta):
    """Live rule at need=30. -> None (pass) or (decl, final_bid, bp_at_final)."""
    d, best = first_max_decl(curves, 30)
    if d is None or F(best, 10000) < theta:
        return None
    b = 30
    while b < 42:
        nxt = curves.get(d, {}).get(b + 1)
        if nxt is None or F(nxt, 10000) < theta:
            break
        b += 1
    return d, b, curves[d][b]


def fmt_frac(fr):
    return f"{fr.numerator}/{fr.denominator} (~{round(float(fr))})"  # display only


def reliability(src, ref, label):
    bands = [(0, 1000), (1000, 2500), (2500, 5000), (5000, 7500), (7500, 9000), (9000, 10000)]
    print(f"\n  reliability {label} -> n200 (bands on {label} bp; mean = exact mean of n200 bp):")
    for lo, hi in bands:
        pairs = [(s, r) for s, r in zip(src, ref) if s is not None and r is not None and lo <= s < hi]
        if pairs:
            mean = F(sum(r for _, r in pairs), len(pairs))
            print(f"    [{lo:5},{hi:5}) n={len(pairs):5}  mean_ref={fmt_frac(mean)}")
    sat = [(s, r) for s, r in zip(src, ref) if s == 10000 and r is not None]
    if sat:
        mean = F(sum(r for _, r in sat), len(sat))
        below = sum(1 for _, r in sat if r < 5000)
        print(f"    SATURATED (=10000) n={len(sat):3}  mean_ref={fmt_frac(mean)}  ref<5000: {below}")


def main():
    data, meta = {}, {}
    for key, fname in PASSES.items():
        p = HERE / fname
        if not p.exists():
            sys.exit(f"missing {p}")
        data[key], mono, died = parse(p)
        meta[key] = (mono, died)

    hands = sorted(data["n200"].keys())
    print(f"bidcurve calibration analysis (predeclared single look) — {len(hands)} hands")
    print("EXPLORATORY: estimates, never receipts; not a P-A21 statement\n")

    print("1. coverage and noise:")
    for key in ("n12", "n40", "n200"):
        mono, died = meta[key]
        ncells = sum(len(r) for h in data[key].values() for r in h.values())
        print(f"  {key:5} hands={len(data[key]):3} cells={ncells:5} DIED={died:3} mono-violations={mono}")

    flat = {k: [] for k in PASSES}
    for h in hands:
        for d in DECLS:
            for b in BIDS:
                for k in PASSES:
                    flat[k].append(data[k].get(h, {}).get(d, {}).get(b))
    print("\n2./3. reliability:")
    reliability(flat["n40"], flat["n200"], "n40")
    reliability(flat["n12"], flat["n200"], "n12")

    for src_key, title in (("n40", "4. auction simulation on n=40 curves"), ("n12", "5. auction simulation on n=12 curves")):
        print(f"\n{title} (scored by the n=200 reference at the chosen cell):")
        print("  theta      bids  mean_ref_bp     overbids(ref<5000)  mean_final_bid  missed")
        for theta in THETAS:
            bids, refs, over, levels, missed = 0, [], 0, [], 0
            for h in hands:
                sim = simulate_bid(data[src_key][h], theta)
                ref_would = simulate_bid(data["n200"][h], F(1, 2))
                if sim is None:
                    if ref_would is not None:
                        missed += 1
                    continue
                d, b, _ = sim
                rb = data["n200"][h].get(d, {}).get(b)
                if rb is None:
                    continue
                bids += 1
                refs.append(rb)
                levels.append(b)
                if rb < 5000:
                    over += 1
            if bids:
                mr = F(sum(refs), bids)
                ml = F(sum(levels), bids)
                print(f"  {str(theta):8} {bids:5}  {fmt_frac(mr):>18}  {over:5}                {fmt_frac(ml):>12}  {missed}")
            else:
                print(f"  {str(theta):8} {bids:5}  (no bids)  missed={missed}")

    print("\n6. first-max declaration agreement at b=30 (vs n200):")
    for src_key in ("n12", "n40"):
        agree = sum(
            1 for h in hands
            if first_max_decl(data[src_key][h], 30)[0] == first_max_decl(data["n200"][h], 30)[0]
        )
        print(f"  {src_key:5} agrees with n200 on {agree}/{len(hands)} hands")

    print("\nnothing above exploratory tier; single-look analysis complete")


if __name__ == "__main__":
    main()
