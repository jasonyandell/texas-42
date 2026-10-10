#!/usr/bin/env python3
"""Pool paired arm differences over seeds: belief_pool.py A1:B1 A2:B2 ... (results dirs); per deal diff = score_B - score_A
(score = hybrid wins in both seatings - 1); mean and bootstrap 95% CI over all deals of all pairs (exploratory)."""
import sys, numpy as np
from belief_compare import games, score
ds = []
for pair in sys.argv[1:]:
    a, b = pair.split(':'); sa, sb = score(games(a)), score(games(b)); ds += [sb[d] - sa[d] for d in sorted(set(sa) & set(sb))]
d = np.array(ds); boot = d[np.random.default_rng(7).integers(len(d), size=(5000, len(d)))].mean(1)
print(dict(deals=len(d), diff=round(float(d.mean()), 4), ci95=[round(float(np.quantile(boot, .025)), 4), round(float(np.quantile(boot, .975)), 4)], plus=int((d > 0).sum()), minus=int((d < 0).sum())))
