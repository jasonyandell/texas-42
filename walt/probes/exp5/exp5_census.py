#!/usr/bin/env python3
"""
exp5_census.py -- Experiment 5: the response-class census curve.

Question
--------
How does the number of distinct exact response classes at a seat scale with the
horizon and with the size of the hidden-world fiber?  Two points were already
known at probe tier (receipt hand 0, focal seat = trick leader, perfect-
information parametric minimax): trick-6 kernel 90 worlds -> 8 parametric root-Q
classes, trick-5 kernel 1680 worlds -> 5.  This run measures the whole curve
over all 13 receipt hands at horizons 2..6.

Staging
-------
Horizons are run cheapest-first (H=2, then 3, 4, 5, 6) and every finished
(kernel, target) record is appended to exp5_records.jsonl immediately, so a
crash late in the run never costs the earlier stages.

Tiers
-----
Everything here is EXPLORATORY PROBE TIER.  Nothing is written to the repo.
The operator is perfect-information minimax over the suffix, which is NOT the
seat-facing decision operator; see the caveats in exp5_results.md.

Usage
-----
  python3 exp5_census.py --stages 2,3,4          # cheap end
  python3 exp5_census.py --stages 5 --h5-n 10000
  python3 exp5_census.py --stages 6 --h6-n 400 --h6-hands 8,7,2
"""

from __future__ import annotations

import argparse
import json
import multiprocessing as mp
import os
import random
import sys
import time
from collections import Counter

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

import exp5_core as C
import exp5_rules as R
import exp5_validate as V

RECORDS = os.path.join(HERE, "exp5_records.jsonl")
PROGRESS = os.path.join(HERE, "exp5_progress.log")

BASE_SEED = 42042
CACHE_LIMIT = 1_500_000        # boundary entries; ~400 MB resident per worker
SPOT_BUDGET_S = 240            # per job wall budget for independent checks
SPOT_TARGET = 3

_HANDS = None


def hands():
    global _HANDS
    if _HANDS is None:
        _HANDS = R.replay_validate_all()
    return _HANDS


def log(msg):
    line = f"[{time.strftime('%H:%M:%S')}] {msg}"
    print(line, flush=True)
    with open(PROGRESS, "a") as fh:
        fh.write(line + "\n")


def checkpoints(n):
    cps = [250, 500, 1000, 2000, 3000, 5000, 7500, 10000, 15000, 20000,
           30000, 50000]
    out = [c for c in cps if c < n]
    small = [25, 50, 100]
    out = [c for c in small if c < n] + out
    return sorted(set(out + [n]))


# ------------------------------------------------------------ class summary

def summarize(labels, reps=None, cap=40):
    """labels: list of hashable class keys, one per world in fiber/sample order.
    Returns class count, the multiset of class sizes, and (if small) the
    representative keys themselves."""
    ctr = Counter(labels)
    sizes = sorted(ctr.values(), reverse=True)
    out = {
        "n_classes": len(ctr),
        # full multiset: the exact fixed-window expectation in exp5_report
        # needs every class size, and a few thousand ints per record is cheap
        "class_sizes": sizes,
        "class_sizes_truncated": False,
        "largest_class": sizes[0] if sizes else 0,
        "singleton_classes": sum(1 for s in sizes if s == 1),
    }
    if len(ctr) <= cap:
        first = {}
        for l in labels:
            if l not in first:
                first[l] = None
        out["class_reps"] = [json.loads(json.dumps(k, default=str))
                             for k in first]
    return out


def saturation(labels, cps):
    """classes discovered after the first n samples, for each checkpoint n."""
    seen = set()
    curve = []
    ci = 0
    cps = sorted(cps)
    for i, l in enumerate(labels, 1):
        seen.add(l)
        while ci < len(cps) and cps[ci] == i:
            curve.append([i, len(seen)])
            ci += 1
    while ci < len(cps):
        curve.append([cps[ci], len(seen)])
        ci += 1
    return curve


# --------------------------------------------------------------- the jobs

def scalar_job(hand_id, trick_no, mode, plan):
    """One (kernel, valuation) census.  Returns a list of JSON records."""
    t_start = time.time()
    h = hands()[hand_id]
    K = C.build_kernel(h, trick_no)
    assert C.true_world_in_fiber(K), (K.kid, "true world violates a void cut")

    seed = BASE_SEED * 1000 + hand_id * 10 + trick_no
    if plan["kind"] == "exhaustive":
        worlds = C.enumerate_worlds(K)
        n_used = len(worlds)
    else:
        rng = random.Random(seed)
        n_used = plan["n"]
        worlds = C.sample_worlds(K, n_used, rng)

    S = C.Solver(K.decl, K.focal_team, K.live, mode, cache_limit=CACHE_LIMIT)
    roots = list(K.focal_hand)
    ri = [S.idx[t] for t in roots]

    qvecs = []
    memo = {}
    for w in worlds:
        wm = S.world_masks(w)
        q = memo.get(wm)
        if q is None:
            q = S.root_vector(wm, K.focal, ri)
            memo[wm] = q
            S.maybe_clear()
        qvecs.append(q)

    acts = [C.argmax_set(q, tuple(range(len(roots)))) for q in qvecs]
    q_true = S.root_vector(S.world_masks(K.true_world), K.focal, ri)
    a_true = C.argmax_set(q_true, tuple(range(len(roots))))

    # ---- independent validation: the naive uncached tuple-based minimax,
    #      plus (for the trick valuation) the PWL parametric solver at lambda=0.
    rngv = random.Random(seed ^ 0x5EED)
    n_spot, spot_ok = 0, True
    t0 = time.time()
    for w in rngv.sample(worlds, min(SPOT_TARGET, len(worlds))):
        if time.time() - t0 > SPOT_BUDGET_S:
            break
        fast = S.root_vector(S.world_masks(w), K.focal, ri)
        slow = V.naive_root_vector(w, K.focal, roots, K.decl, K.focal_team, mode)
        spot_ok = spot_ok and (fast == slow)
        n_spot += 1
    n_param_spot = 0
    if mode == "trick" and time.time() - t0 < SPOT_BUDGET_S:
        PS = C.ParamSolver(K.decl, K.focal_team, C.highest_count_unseen(K))
        w = worlds[0]
        import exp5_pwl as P
        fast = S.root_vector(S.world_masks(w), K.focal, ri)
        pv = tuple(P.pwl_eval(PS.root(w, K.focal, a), 0) for a in roots)
        spot_ok = spot_ok and (fast == pv)
        n_param_spot = 1

    cov = C.covariates(K)
    base = {
        "kind": "census",
        "kid": K.kid,
        "hand": hand_id,
        "trick_no": trick_no,
        "horizon": K.horizon,
        "declaration": K.decl.label,
        "decl_class": K.decl.cls,
        "focal_seat": K.focal,
        "focal_team": K.focal_team,
        "fiber_unconstrained": K.fiber_unconstrained,
        "fiber_size": K.fiber_size,
        "voids": sorted((s, str(q)) for s, q in K.voids),
        "coverage": plan["kind"],
        "n_worlds_used": n_used,
        "n_distinct_worlds_solved": len(memo),
        "sample_seed": seed if plan["kind"] == "sampled" else None,
        "covariates": cov,
        "validation": {
            "naive_spot_checks": n_spot,
            "param_lambda0_spot_checks": n_param_spot,
            "all_matched": spot_ok,
        },
        "solver": {
            "max_cache_entries": S.max_cache,
            "cache_clears": S.clears,
            "cache_limit": CACHE_LIMIT,
        },
        "seconds": round(time.time() - t_start, 2),
    }
    assert spot_ok, (K.kid, mode, "SPOT CHECK MISMATCH")

    recs = []
    for target, labels, truth in (
            (f"q_{mode}", qvecs, q_true),
            (f"act_{mode}", acts, a_true)):
        r = dict(base)
        r["target"] = target
        r.update(summarize(labels))
        r["true_world_class"] = json.loads(json.dumps(truth, default=str))
        r["true_world_class_size"] = Counter(labels)[truth]
        r["true_world_class_found"] = truth in set(labels)
        if plan["kind"] == "sampled":
            r["saturation"] = saturation(labels, checkpoints(n_used))
        recs.append(r)
    return recs


def param_job(hand_id, trick_no, plan):
    """Parametric (valued-tile) census for one kernel."""
    t_start = time.time()
    h = hands()[hand_id]
    K = C.build_kernel(h, trick_no)
    d = C.highest_count_unseen(K)
    seed = BASE_SEED * 1000 + hand_id * 10 + trick_no
    if plan["kind"] == "exhaustive":
        worlds = C.enumerate_worlds(K)
        n_used = len(worlds)
    else:
        rng = random.Random(seed)
        n_used = plan["n"]
        worlds = C.sample_worlds(K, n_used, rng)

    PS = C.ParamSolver(K.decl, K.focal_team, d)
    one = C.param_targets(K, d, PS)
    sigs, corrs = [], []
    for i, w in enumerate(worlds):
        s, c = one(w)
        sigs.append(s)
        corrs.append(c)
        if len(PS.cache) > 200_000:
            PS.cache.clear()
    ts, tc = one(K.true_world)

    cov = C.covariates(K)
    base = {
        "kind": "census",
        "kid": K.kid,
        "hand": hand_id,
        "trick_no": trick_no,
        "horizon": K.horizon,
        "declaration": K.decl.label,
        "decl_class": K.decl.cls,
        "focal_seat": K.focal,
        "focal_team": K.focal_team,
        "fiber_unconstrained": K.fiber_unconstrained,
        "fiber_size": K.fiber_size,
        "voids": sorted((s, str(q)) for s, q in K.voids),
        "coverage": plan["kind"],
        "n_worlds_used": n_used,
        "n_distinct_worlds_solved": len(set(map(tuple, (tuple(map(tuple, w))
                                                        for w in worlds)))),
        "sample_seed": seed if plan["kind"] == "sampled" else None,
        "covariates": cov,
        "valued_tile": R.tname(d),
        "validation": {"naive_spot_checks": 0,
                       "param_lambda0_spot_checks": 0,
                       "all_matched": True,
                       "note": "parametric target is cross-checked at lambda=0 "
                               "inside the trick-valuation job for this kernel"},
        "solver": {"max_cache_entries": len(PS.cache)},
        "seconds": round(time.time() - t_start, 2),
    }
    recs = []
    for target, labels, truth in (("q_param", sigs, ts),
                                  ("act_param", corrs, tc)):
        r = dict(base)
        r["target"] = target
        r.update(summarize(labels, cap=12))
        r["true_world_class_size"] = Counter(labels)[truth]
        r["true_world_class_found"] = truth in set(labels)
        if plan["kind"] == "sampled":
            r["saturation"] = saturation(labels, checkpoints(n_used))
        recs.append(r)
    return recs


def dispatch(job):
    kind = job[0]
    try:
        if kind == "scalar":
            return scalar_job(job[1], job[2], job[3], job[4])
        return param_job(job[1], job[2], job[3])
    except Exception as e:      # a pathological kernel must not kill the run
        import traceback
        return [{"kind": "error", "job": list(job[:4]),
                 "error": repr(e), "traceback": traceback.format_exc()}]


# ----------------------------------------------------------------- driver

def build_jobs(stage, args):
    hs = hands()
    trick_no = 8 - stage
    jobs = []
    if stage in (2, 3, 4):
        plan = {"kind": "exhaustive"}
        for h in hs:
            for mode in ("trick", "points"):
                jobs.append(("scalar", h.hid, trick_no, mode, plan))
            jobs.append(("param", h.hid, trick_no, plan))
    elif stage == 5:
        plan = {"kind": "sampled", "n": args.h5_n}
        pplan = {"kind": "sampled", "n": args.h5_param_n}
        order = sorted(hs, key=lambda h: C.build_kernel(h, trick_no).fiber_size)
        skip = {int(x) for x in args.skip_hands.split(",") if x != ""}
        for h in order:
            if h.hid in skip:
                continue
            for mode in ("trick", "points"):
                jobs.append(("scalar", h.hid, trick_no, mode, plan))
            if not args.no_param:
                jobs.append(("param", h.hid, trick_no, pplan))
    elif stage == 6:
        plan = {"kind": "sampled", "n": args.h6_n}
        want = [int(x) for x in args.h6_hands.split(",")] if args.h6_hands \
            else None
        order = sorted(hs, key=lambda h: C.build_kernel(h, trick_no).fiber_size)
        if want is not None:
            order = [h for h in order if h.hid in want]
        for h in order[: args.h6_kernels]:
            for mode in ("trick", "points"):
                jobs.append(("scalar", h.hid, trick_no, mode, plan))
    return jobs


def main():
    global RECORDS
    ap = argparse.ArgumentParser()
    ap.add_argument("--stages", default="2,3,4")
    ap.add_argument("--procs", type=int, default=13)
    ap.add_argument("--h5-n", type=int, default=10000)
    ap.add_argument("--h5-param-n", type=int, default=2000)
    ap.add_argument("--h6-n", type=int, default=400)
    ap.add_argument("--h6-kernels", type=int, default=4)
    ap.add_argument("--h6-hands", default="")
    ap.add_argument("--no-param", action="store_true")
    ap.add_argument("--skip-hands", default="")
    ap.add_argument("--out", default=RECORDS,
                    help="records file; use a distinct one when two stages "
                         "run concurrently, then concatenate")
    args = ap.parse_args()
    RECORDS = args.out

    stages = [int(s) for s in args.stages.split(",")]
    log(f"=== exp5 census run: stages {stages} procs {args.procs} "
        f"h5_n {args.h5_n} h5_param_n {args.h5_param_n} h6_n {args.h6_n} ===")

    hs = hands()
    log(f"replay-validated all {len(hs)} receipt hands under the generalized "
        f"declaration-relative rules; coverage "
        f"{ {k: len(v) for k, v in R.declaration_coverage(hs).items()} }")

    for stage in stages:
        jobs = build_jobs(stage, args)
        log(f"--- stage H={stage}: {len(jobs)} jobs ---")
        t0 = time.time()
        done = 0
        with mp.Pool(processes=min(args.procs, len(jobs)),
                     maxtasksperchild=1) as pool:
            for recs in pool.imap_unordered(dispatch, jobs):
                with open(RECORDS, "a") as fh:
                    for r in recs:
                        fh.write(json.dumps(r) + "\n")
                done += 1
                r0 = recs[0]
                if r0.get("kind") == "error":
                    log(f"  ERROR {r0['job']}: {r0['error']}")
                else:
                    log(f"  [{done}/{len(jobs)}] {r0['kid']} "
                        f"|X|={r0['fiber_size']} "
                        + " ".join(f"{r['target']}={r['n_classes']}"
                                   for r in recs)
                        + f"  ({r0['seconds']}s)")
        log(f"--- stage H={stage} complete in {time.time()-t0:.1f}s ---")
    log("=== run complete ===")


if __name__ == "__main__":
    main()
