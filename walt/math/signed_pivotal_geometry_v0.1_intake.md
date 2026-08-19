# Intake — *Signed Pivotal Geometry for Walt*, v0.1

**Status:** exploratory intake companion — the same tier as its parent.
Nothing here is promoted, and nothing here may be cited above exploratory
tier; not a P-A21 statement.
**Date:** 2026-08-18
**Parent:** `walt/math/signed_pivotal_geometry_v0.1.md` (filed 2026-08-18,
verbatim; `signed_pivotal_geometry_v0.1.sha256` =
`b9d93715bf65cc29b2bbb1ce6775d00070449a01ef2fe58f7b3bba2a2b7f9630`).
**The parent stays verbatim.** Observations live beside it and are never
written into it.
**Provenance:** house-mathematician pass (Claude side-channel session +
ChatGPT 5.6 Pro) over `HANDOFF-plan-geometry-and-names.md`, an imported
handoff that is **not filed in this repository** — it lived only in the side
channel. Consequences of the unfiled parent:

- the parent's obligation numbering starts at **O12**; O10–O11 are presumed
  reserved by the unfiled import. The gap is kept as-is (renumbering a
  verbatim parent is forbidden); the ledger below records it.
- the import's external literature-name mappings are **unverified**, and the
  parent itself says so (§15, last bullet). Nothing in walt may cite those
  mappings until the import is filed and audited.

---

## 1. Identity verification

The parent's boxed identities were checked two ways at intake: by hand, and
by an exact-rational spot check (stdlib `fractions`, no floats — script in
the appendix) over 2,000 random finite scenario spaces with random tape
laws, outcome pairs, world partitions, and pivotal-containing envelopes.
All of the following hold **exactly** on every instance:

- §2: \(g = p_+ - p_- = q\tau\); \(\mathbb E[Y^2] = q\);
  \(\operatorname{Var}(Y) = q - g^2\).
- §2.3: \(H = 1/(q\tau^2) - 1\) (whenever \(\tau \neq 0\)).
- §4: the world projection \(g = \mathbb E_\omega[d(\omega)]\),
  \(q = \mathbb E_\omega[s(\omega)]\) under tape-law independence.
- §5: strata linearity \(g = \sum_j w_j \mu_j\) for arbitrary partitions.
- §6.1: for any envelope \(P\) containing every pivotal world,
  \(w\,\mathbb E[Y \mid P] = g\), \(w\,\mathbb E[Y^2 \mid P] = q\), and
  \(w^2 \operatorname{Var}(Y \mid P) = wq - g^2\) (hence
  \(H_P = w/(q\tau^2) - 1\)).

Checked by hand only (standard probability/statistics, no novelty claimed
by the parent): the §2.2 multinomial decomposition, the §5 Neyman and
cost-aware allocations, the §5.1 interval composition, the §6.1 \(1/w\)
pivotal-rate speedup, and the §2.3/§9.4 sequential-validity caveats. No
defect found in any boxed statement.

## 2. Vocabulary flags

- **Clean on D3.** The word "certificate" does not appear in the parent.
- **θ collision (action needed before use).** The parent's
  \(\theta = (1+\tau)/2\) (§2, pivotal win share) collides with walt's
  standing auction threshold θ — the bid rule "bid while P(make) ≥ θ"
  (`SCENARIO-PLAYER.md` §6.2, the `bidcurve` calibration corpus, the wasm
  `theta` request field). **Proposed** (pending walt-math adoption): in walt
  artifacts the pivotal quantity is always written τ, and \((1+\tau)/2\),
  where needed, is spelled out as the **pivotal win share** — never bare θ.
  The auction threshold keeps θ.
- **"rank"** appears only in the linear-algebra sense (§10.2 decision
  dimension). Distinct from the retired standings sense (constellations own
  that ground); no conflict, noted for grep hygiene.
- **Scenario \(\xi = (\omega, r)\).** The parent's world/tape split is
  conceptually walt's world + modeled-mind randomness, but current code
  draws both from one derived SplitMix64 stream — world ID and tape seed
  are **not separable today** (see §4 gaps).

## 3. Cross-references into standing walt artifacts

- **`POLICY-GEOMETRY.md` (Gate E).** The parent's §10 three censuses and
  decision dimension refine Gate E's four cardinalities: the behavioral
  plan count is a panel-sampled analogue of N_vec; the signed boundary
  count and decision rank are new objects sitting between N_vec and N_exp.
  The parent's §10 argument that an exploding *syntactic* census kills only
  the small-plan-library hypothesis is the same lesson as Gate E's
  "four cardinalities, never conflated" (E-A8). The two documents should be
  read together; neither supersedes the other.
- **`SCENARIO-PLAYER.md` obligations.** O6 (sampling error quantification)
  is sharpened by the paired treatment — \(\operatorname{Var}(\widehat g) =
  (q - g^2)/n\) is strictly tighter than unpaired make-rate intervals. O8
  (tie-refinement bias) is an instance of the parent's §3 discovery/
  evaluation separation. New obligations O12–O19 are filed in the ledger
  (§10 of `SCENARIO-PLAYER.md`) with source labels.
- **E0 corpus anchors** exist in the artifact record: the level-2 trick-1
  saturation/tie episode; the walt-vs-champion divergence positions
  (dropped-30 arena, 2026-08-17/18); the divergence miner's 900 self-played
  hands / 4,156 level-2-shadowed decisions (2026-08-18 overnight). The
  parent's "existing n=800 panel" (§9.4) matches no filed artifact —
  presumed to reference side-channel work; **to confirm before E0 cites
  it** (the 900-hand corpus is the likely intended anchor).
- **The bidcurve calibration corpus** (running 2026-08-18, three
  nested-CRN passes n = 12/40/200 over 200 frozen hands) is an instance of
  the parent's §11.1 reuse discipline avant la lettre — common scenarios
  across cells and passes. Its planned θ sweep inherits O14: any
  resample-until-separated protocol needs predeclared checkpoints or an
  anytime-valid method.

## 4. E0 implementation gaps (what exists vs what E0 needs)

Exists now: CRN world panels; level-1 and level-2 solvers; per-move
basis-point option arrays; the arena/divergence position corpus; frozen
seeds and deterministic replay.

Missing for E0 (parent §9, §13): executable **plan extraction** — the
solver returns root values, not serialized information-consistent policy
DAGs, and frozen-plan replay is the unit E0 evaluates; the **bitset replay
kernel** (§12.2); **world/tape seed separation** (§2 flag above; Phase E
needs repeated tapes per world); **sequential-valid candidate racing**
(§12.3); content-addressed plan identifiers. None of these are large, but
none exist; E0 Phase A cannot start until plan extraction does.

## 5. Ruling requests carried out of intake

None adjudicated here (intake does not rule). Flagged for walt-math:
(i) adopt the θ/τ vocabulary resolution of §2; (ii) decide whether the
unfiled import is retrieved and filed or O10–O11 are permanently retired;
(iii) confirm or correct the "n=800 panel" reference before E0 is
scheduled.

---

## Appendix — the intake spot-check script (verbatim, stdlib only)

```python
"""Exact-rational spot check of signed_pivotal_geometry_v0.1 identities.

Exploratory tooling, not a receipt. Random finite scenario spaces with exact
Fraction masses; Boolean outcome pairs; verifies (all exactly, no floats):
  g = p+ - p-  = q*tau                       (sec 2, boxed)
  E[Y^2] = q;  Var(Y) = q - g^2              (sec 2.1)
  H = Var/g^2 = 1/(q tau^2) - 1              (sec 2.3)
  world projection: g = E_w[d(w)], q = E_w[s(w)]   (sec 4)
  strata: g = sum_j w_j mu_j                 (sec 5)
  envelope: w*q_P_var identity  wq - g^2 = w^2 Var(Y|P), P >= Delta (sec 6.1)
"""
import random
from fractions import Fraction as F

random.seed(42)

def rand_dist(n):
    ws = [F(random.randint(1, 9)) for _ in range(n)]
    t = sum(ws)
    return [w / t for w in ws]

for trial in range(2000):
    nw = random.randint(1, 6)          # physical worlds
    nt = random.randint(1, 4)          # tapes per world
    wmass = rand_dist(nw)
    tmass = rand_dist(nt)              # tape law independent of world
    ua = [[random.randint(0, 1) for _ in range(nt)] for _ in range(nw)]
    ub = [[random.randint(0, 1) for _ in range(nt)] for _ in range(nw)]

    # scenario-level moments
    pp = sum(wmass[i] * tmass[j] for i in range(nw) for j in range(nt)
             if ua[i][j] == 1 and ub[i][j] == 0)
    pm = sum(wmass[i] * tmass[j] for i in range(nw) for j in range(nt)
             if ua[i][j] == 0 and ub[i][j] == 1)
    q, g = pp + pm, pp - pm
    ey2 = sum(wmass[i] * tmass[j] * (ua[i][j] - ub[i][j]) ** 2
              for i in range(nw) for j in range(nt))
    assert ey2 == q and ey2 - g * g == q - g * g
    if q > 0:
        tau = (pp - pm) / q
        assert g == q * tau
        if tau != 0:
            H = (q - g * g) / (g * g)
            assert H == 1 / (q * tau * tau) - 1

    # world projection: d(w), s(w)
    d = [sum(tmass[j] * (ua[i][j] - ub[i][j]) for j in range(nt)) for i in range(nw)]
    s = [sum(tmass[j] * (ua[i][j] - ub[i][j]) ** 2 for j in range(nt)) for i in range(nw)]
    assert sum(wmass[i] * d[i] for i in range(nw)) == g
    assert sum(wmass[i] * s[i] for i in range(nw)) == q

    # strata over worlds: any partition; mu_j = E[Y | A_j]
    cut = random.randint(1, nw)
    strata = [list(range(0, cut)), list(range(cut, nw))]
    strata = [S for S in strata if S]
    tot = F(0)
    for S in strata:
        wj = sum(wmass[i] for i in S)
        muj = sum(wmass[i] * d[i] for i in S) / wj
        tot += wj * muj
    assert tot == g

    # envelope P containing all pivotal worlds (s(w) > 0), maybe looser
    piv = [i for i in range(nw) if s[i] > 0]
    extra = [i for i in range(nw) if s[i] == 0 and random.random() < 0.5]
    P = piv + extra
    if P:
        w = sum(wmass[i] for i in P)
        eyP = sum(wmass[i] * d[i] for i in P) / w
        ey2P = sum(wmass[i] * s[i] for i in P) / w
        assert w * eyP == g and w * ey2P == q
        varYP = ey2P - eyP * eyP
        assert w * w * varYP == w * q - g * g

print("all identities hold exactly over 2000 random exact-rational instances")
```

Run 2026-08-18: `all identities hold exactly over 2000 random
exact-rational instances`.
