"""Aggregate the step-9 detection-layer probe records (stdlib only).

Usage: python3 summarize.py records.jsonl
Reproduces the tables and counts quoted in README.md from the committed
JSONL. Exact rationals print as fractions; wide ones print as exact
truncated decimals (integer arithmetic, marked with ~); nothing here
uses floating point.
"""
import json
import sys
from collections import Counter
from fractions import Fraction


def frac(s):
    n, d = s.split("/")
    return Fraction(int(n), int(d))


def dec(f, digits=5):
    """Exact decimal truncation by integer arithmetic."""
    sign = "-" if f < 0 else ""
    f = abs(f)
    whole = f.numerator // f.denominator
    rem = f.numerator % f.denominator
    out = []
    for _ in range(digits):
        rem *= 10
        out.append(str(rem // f.denominator))
        rem %= f.denominator
    return f"{sign}{whole}.{''.join(out)}"


def fstr(s):
    if s is None:
        return "-"
    f = frac(s)
    if f.denominator <= 10000:
        return f"{f.numerator}/{f.denominator}"
    return "~" + dec(f)


def interval(v):
    if v is None:
        return "-"
    return f"[~{dec(frac(v[0]))},~{dec(frac(v[1]))}]"


def load(path):
    return [json.loads(line) for line in open(path)]


def summarize(path):
    rows = load(path)
    roots = [r for r in rows if r["kind"] == "root"]
    refusals = [r for r in rows if r["kind"] == "refusal"]
    exact = [r for r in rows if r["kind"] == "pair" and r["route"] == "exact"]
    sampled = [r for r in rows if r["kind"] == "pair" and r["route"] == "sampled"]
    print(f"== corpus ({path})")
    for r in roots:
        print(f"  {r['root']}: route {r['route']}, fiber {r['fiber']}, legal {r['legal']}")
    for r in refusals:
        print(f"  REFUSAL {r.get('root', r['root_id'])}: {r['route']} — {r['reason']}")
    print(f"  {len(roots)} roots, {len(exact)} exact pairs, "
          f"{len(sampled)} sampled pairs, {len(refusals)} refusals")

    print("== exact route (complete-fiber coupled enumeration)")
    wake = Counter()
    for r in exact:
        c0, c1 = r["coords0"], r["coords1"]
        resp = r["response"]
        val = r["value"]
        dec_ = r["decision"]
        info = r["information"]["verdict"]
        if resp["positive"]:
            wake["response q1>q0"] += 1
        if resp["exceeds_eps"]:
            wake["response >eps_q"] += 1
        if val["wake"]:
            wake["value g1!=g0"] += 1
        if dec_["changed"]:
            wake["decision changed"] += 1
        wake[f"info {info}"] += 1
        print(
            f"  {r['root']} {r['tile_a']}v{r['tile_b']}: "
            f"q {fstr(c0['q'])}->{fstr(c1['q'])} "
            f"tau {fstr(c0['tau'])}->{fstr(c1['tau'])} "
            f"g {fstr(c0['g'])}->{fstr(c1['g'])} "
            f"| dq {fstr(resp['dq'])} dg {fstr(val['gap_change'])} "
            f"| sel {dec_['winner0']}->{dec_['winner1']} "
            f"| Z {r['z_counts']} | info {info}"
        )
    print(f"  wake-up counts over {len(exact)} exact pairs: {dict(wake)}")

    print("== sampled route (dig-until-settled paired stream)")
    swake = Counter()
    for r in sampled:
        c0, c1 = r["coords0"], r["coords1"]
        resp, val, dec_ = r["response"], r["value"], r["decision"]
        info = r["information"]["verdict"]
        swake[f"response {resp['tag']}"] += 1
        swake[f"value {val['tag']}"] += 1
        swake[f"decision {dec_['kind']}"] += 1
        swake[f"info {info}"] += 1
        if r["q0_practical"] is not None:
            swake["q0 practical-zero"] += 1
        vz = (
            f"settled {val['direction']}@{val['settled_at']}"
            if val["tag"] == "sampled-settled"
            else f"open z_mean_hat {fstr(val['z_mean_hat'])}"
        )
        print(
            f"  {r['root']} {r['tile_a']}v{r['tile_b']}: consumed {r['consumed']}"
            f"/{r['world_cap']} | q_hat {fstr(c0['q_hat'])}->{fstr(c1['q_hat'])} "
            f"tau_hat {fstr(c0['tau_hat'])}->{fstr(c1['tau_hat'])} "
            f"g_hat {fstr(c0['g_hat'])}->{fstr(c1['g_hat'])} "
            f"| value {vz} | response {resp['tag']} | decision {dec_['kind']} "
            f"| Z {r['z_counts']} | info {info}"
        )
        print(
            f"    rates: I0 {interval(r['information']['rate0'])} "
            f"I1 {interval(r['information']['rate1'])} "
            f"| splits_a exposed {r['splits_a']['exposed']} "
            f"splits_b exposed {r['splits_b']['exposed']}"
        )
    print(f"  wake-up counts over {len(sampled)} sampled pairs: {dict(swake)}")


if __name__ == "__main__":
    summarize(sys.argv[1] if len(sys.argv) > 1 else "records.jsonl")
