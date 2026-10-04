#!/usr/bin/env python3
"""Independent scalar witnesses; never imports or executes incoming bundle."""
from fractions import Fraction
from pathlib import Path
import hashlib
import json

incoming = Path(__file__).resolve().parents[2] / "incoming/walt-sense/walt-sense"
# Equal-prior hidden worlds, a common visible chance move H, then a lawful
# focal decision shared by all three worlds. The likelihood of H differs.
reach = [Fraction(1), Fraction(1, 3), Fraction(1, 3)]
payoffs = {"X": [0, 1, 1], "Y": [1, 0, 0]}
unweighted = {a: sum(v) for a, v in payoffs.items()}
weighted = {a: sum(p*x for p, x in zip(reach, v)) for a, v in payoffs.items()}
assert unweighted["X"] > unweighted["Y"]
assert weighted["X"] < weighted["Y"]
# This matches nofusion_sc's unweighted continuation sums at focal nodes.
assert "np.add.at(tot, cid, val[mine])" in (incoming / "nofusion_sc.py").read_text()
assert "g[ng].w /= popc(L)" in (incoming / "walt.c").read_text()

# The harness passes a seed derived from the same SEED/h used to deal. If
# the advertised match seed is known, this public field recovers h and side.
for match_seed in (1, 17, 100):
    for h in range(120):
        for side in range(2):
            for plays_len in range(0, 56, 2):
                request_seed = (match_seed*100 + h*2 + side)*1000 + plays_len
                recovered = request_seed//1000 - match_seed*100
                assert (recovered//2, recovered%2) == (h, side)

print(json.dumps({
    "incoming_sha256": {p.name: hashlib.sha256(p.read_bytes()).hexdigest()
                        for p in sorted(incoming.iterdir()) if p.is_file()},
    "likelihood_witness": {"unweighted": unweighted,
        "weighted": {k: str(v) for k, v in weighted.items()},
        "wrong_unweighted_choice": "X", "correct_weighted_choice": "Y"},
    "seed_recovers_deal_index_when_match_seed_known": True,
    "imported_scripts_executed": False,
    "scope": "Abstract fold witness and harness seed algebra; not a Texas42 replay or match result."
}, indent=2))
