"""EXPLORATORY: does replay (memoized value + decisions) change walt's move?

Positions come from self-play by memoized walt (all four seats, level 1).
At every non-forced decision, walt is re-run with the SAME own-seat deals:
  memo  (inner seed A)  — the baseline move
  memo  (inner seed B)  — noise floor: only the model's inner randomness changes
  fresh (inner seed A)  — nothing replayed: no value memo, no decision memo
  fresh (inner seed B)
Usage: python3 experiment.py FIRST_SEED COUNT N_OUTER OUT.jsonl
"""
import json, random, sys, time
from multiprocessing import Pool
from simple_walt import *

N0 = 8


def run_deal(args):
    seed, n_outer = args
    rules, hands = deal_and_contract(seed)
    state, voids, rows = (0, 1, (), 0, 0), [0] * 4, []
    while decided(state) is None:
        s = to_act(state)
        hand = hands[s] & ~state[0]
        legal = tiles_of(rules.legal(hand, state[2]))
        if len(legal) == 1:
            t = legal[0]
        else:
            dseed = hash((seed, state)) & 0xFFFFFFFF
            row = {"seed": seed, "trick": bin(state[0]).count("1") // 4 + 1, "legal": len(legal)}
            for name, memo, inner in (("memoA", True, 1), ("memoB", True, 2),
                                      ("freshA", False, 1), ("freshB", False, 2)):
                w = Walt(rules, n_outer, N0, memo, memo, seed=dseed, inner_seed=dseed ^ inner)
                t0 = time.time()
                best, opts, n = w.decide(s, hand, state, 1, voids=voids, detail=True)
                row[name] = best
                row[name + "_s"] = round(time.time() - t0, 2)
                row[name + "_nodes"] = w.nodes
                row[name + "_made"] = dict(opts)[best]
            row["n"] = n
            rows.append(row)
            t = row["memoA"]
        observe_void(rules, voids, s, state[2], t)
        state = step(rules, state, t)
    return rows


if __name__ == "__main__":
    first, count, n_outer, out = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
    with Pool(4) as pool, open(out, "a") as f:
        for rows in pool.imap_unordered(run_deal, [(s, n_outer) for s in range(first, first + count)]):
            for r in rows:
                f.write(json.dumps(r) + "\n")
            f.flush()
            print("deal", rows[0]["seed"] if rows else "?", len(rows), "decisions", flush=True)
