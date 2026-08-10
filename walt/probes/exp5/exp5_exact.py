#!/usr/bin/env python3
"""
exp5_exact.py -- opportunistic EXHAUSTIVE censuses at horizon 5.

Motivation: at H=4 the exhaustive run cost ~0.001 s/world while scattered
sampling at H=5 costs ~0.085 s/world.  The difference is cache reuse: worlds
enumerated in lexicographic order share long runs of boundary states, whereas
uniformly sampled worlds share almost nothing.  So for the two H=5 kernels with
the smallest void-constrained fibers, an exhaustive census may be cheaper than
the 10,000-world sample -- and it turns a lower bound into an exact count.

Runs the same `scalar_job` as the main driver with an exhaustive plan, and
appends records in the same format.

  python3 exp5_exact.py --hands 8,11 --trick 3 --procs 4 --out exp5_records_exact.jsonl
"""

from __future__ import annotations

import argparse
import json
import multiprocessing as mp
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

import exp5_census as D


def job(args):
    hand_id, trick_no, mode = args
    try:
        return D.scalar_job(hand_id, trick_no, mode, {"kind": "exhaustive"})
    except Exception as e:
        import traceback
        return [{"kind": "error", "job": ["scalar", hand_id, trick_no, mode],
                 "error": repr(e), "traceback": traceback.format_exc()}]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--hands", default="8,11")
    ap.add_argument("--trick", type=int, default=3)
    ap.add_argument("--procs", type=int, default=4)
    ap.add_argument("--cache-limit", type=int, default=6_000_000)
    ap.add_argument("--out", default=os.path.join(HERE,
                                                  "exp5_records_exact.jsonl"))
    a = ap.parse_args()
    D.CACHE_LIMIT = a.cache_limit
    jobs = [(int(h), a.trick, m) for h in a.hands.split(",")
            for m in ("trick", "points")]
    D.log(f"=== exp5 exhaustive H={8-a.trick}: {len(jobs)} jobs, "
          f"cache_limit {a.cache_limit} ===")
    t0 = time.time()
    with mp.Pool(processes=min(a.procs, len(jobs)), maxtasksperchild=1) as pool:
        for recs in pool.imap_unordered(job, jobs):
            with open(a.out, "a") as fh:
                for r in recs:
                    fh.write(json.dumps(r) + "\n")
            r0 = recs[0]
            if r0.get("kind") == "error":
                D.log(f"  ERROR {r0['job']}: {r0['error']}")
            else:
                D.log(f"  EXACT {r0['kid']} |X|={r0['fiber_size']} "
                      + " ".join(f"{r['target']}={r['n_classes']}"
                                 for r in recs)
                      + f"  ({r0['seconds']}s, max cache "
                      f"{r0['solver']['max_cache_entries']}, "
                      f"{r0['solver']['cache_clears']} clears)")
    D.log(f"=== exhaustive H={8-a.trick} done in {time.time()-t0:.1f}s ===")


if __name__ == "__main__":
    main()
