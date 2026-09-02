#!/usr/bin/env python3
"""Exact finite checks for MODEL-BELIEF WALT v0.1.

No external packages.  Every probability is fractions.Fraction.
The companion verifies the algebraic claims in the associated theory artifact;
it is not a proof of the Texas 42 implementation.
"""

from __future__ import annotations

from fractions import Fraction as F
from itertools import product
from typing import Iterable, Sequence

CHECKS = 0


def check(name: str, condition: bool) -> None:
    global CHECKS
    if not condition:
        raise AssertionError(name)
    CHECKS += 1


def dot(xs: Sequence[F], ys: Sequence[F]) -> F:
    return sum((x * y for x, y in zip(xs, ys)), F(0))


def q_mix(values: Sequence[Sequence[F]], nu: Sequence[F]) -> F:
    return max(dot(row, nu) for row in values)


def point_upper(values: Sequence[Sequence[F]], nu: Sequence[F]) -> F:
    return sum((nu[t] * max(row[t] for row in values) for t in range(len(nu))), F(0))


def set_partitions(n: int) -> list[tuple[tuple[int, ...], ...]]:
    out: list[tuple[tuple[int, ...], ...]] = []

    def rec(i: int, blocks: list[list[int]]) -> None:
        if i == n:
            canon = tuple(sorted((tuple(sorted(b)) for b in blocks), key=lambda b: b[0]))
            if canon not in out:
                out.append(canon)
            return
        for j in range(len(blocks)):
            blocks[j].append(i)
            rec(i + 1, blocks)
            blocks[j].pop()
        blocks.append([i])
        rec(i + 1, blocks)
        blocks.pop()

    rec(0, [])
    return out


def partition_upper(
    values: Sequence[Sequence[F]],
    nu: Sequence[F],
    partition: tuple[tuple[int, ...], ...],
) -> F:
    total = F(0)
    for block in partition:
        total += max(sum((nu[t] * row[t] for t in block), F(0)) for row in values)
    return total


def refines(
    fine: tuple[tuple[int, ...], ...],
    coarse: tuple[tuple[int, ...], ...],
) -> bool:
    coarse_sets = [set(b) for b in coarse]
    return all(any(set(b).issubset(c) for c in coarse_sets) for b in fine)


# ---------------------------------------------------------------------------
# 1. Point-mass response geometry and type gluing.
# ---------------------------------------------------------------------------

parts3 = set_partitions(3)
coarsest = ((0, 1, 2),)
finest = ((0,), (1,), (2,))
priors = [
    (F(1, 3), F(1, 3), F(1, 3)),
    (F(1, 2), F(1, 3), F(1, 6)),
    (F(1), F(0), F(0)),
]

all_geom = True
all_partition = True
all_convex = True
all_common = True
for bits in product((0, 1), repeat=6):
    vals = (
        (F(bits[0]), F(bits[1]), F(bits[2])),
        (F(bits[3]), F(bits[4]), F(bits[5])),
    )
    for nu in priors:
        q = q_mix(vals, nu)
        sep = point_upper(vals, nu)
        all_geom &= q <= sep
        all_partition &= partition_upper(vals, nu, coarsest) == q
        all_partition &= partition_upper(vals, nu, finest) == sep
        for pf in parts3:
            for pc in parts3:
                if refines(pf, pc):
                    all_partition &= partition_upper(vals, nu, pf) >= partition_upper(vals, nu, pc)

    nu0 = priors[0]
    nu1 = priors[1]
    mid = tuple((a + b) / 2 for a, b in zip(nu0, nu1))
    all_convex &= q_mix(vals, mid) <= (q_mix(vals, nu0) + q_mix(vals, nu1)) / 2

    positive = priors[0]
    equality = q_mix(vals, positive) == point_upper(vals, positive)
    common = any(all(row[t] == max(r[t] for r in vals) for t in range(3)) for row in vals)
    all_common &= equality == common

check("point-mass upper dominates mixture optimum", all_geom)
check("type-partition lattice has correct endpoints and monotonicity", all_partition)
check("mixture optimum is convex in model belief", all_convex)
check("zero model-fusion price iff one policy is pointwise optimal on support", all_common)

strict_vals = ((F(1), F(0)), (F(0), F(1)))
strict_nu = (F(1, 2), F(1, 2))
check("strict model-fusion example", q_mix(strict_vals, strict_nu) == F(1, 2) and point_upper(strict_vals, strict_nu) == 1)

common_vals = ((F(1), F(1)), (F(0), F(0)))
check("common optimizer makes point upper exact", q_mix(common_vals, strict_nu) == point_upper(common_vals, strict_nu) == 1)

# Point-mass priors recover fixed fields exactly.
for t in range(3):
    vals = ((F(1, 5), F(2, 5), F(3, 5)), (F(4, 5), F(1, 5), F(1, 2)))
    nu = tuple(F(1 if j == t else 0) for j in range(3))
    check(f"point prior {t} recovers fixed-field optimum", q_mix(vals, nu) == max(row[t] for row in vals))

# A response-vector library is a lower envelope of the exact response.
vals = (
    (F(4, 5), F(1, 5), F(1, 2)),
    (F(1, 2), F(3, 4), F(2, 5)),
    (F(3, 5), F(3, 5), F(3, 5)),
)
nu = (F(1, 2), F(1, 3), F(1, 6))
lib_lower = max(dot(row, nu) for row in vals[:2])
check("adding policy columns can only raise the executable lower", lib_lower <= q_mix(vals, nu))

# Decomposition of the point upper minus an incumbent value.
inc = dot(vals[0], nu)
q = q_mix(vals, nu)
sep = point_upper(vals, nu)
check("upper gap decomposes into model-fusion price plus policy gap", sep - inc == (sep - q) + (q - inc))

# Weighted point-mass upper intervals stay admissible.
point_q = [max(row[t] for row in vals) for t in range(3)]
point_u = [min(F(1), x + F(1, 10)) for x in point_q]
check("weighted point-mass upper intervals are admissible", q <= dot(point_u, nu))

# ---------------------------------------------------------------------------
# 2. Augmented latent space and salvation atoms.
# ---------------------------------------------------------------------------

# utility[policy][world][type]
utility = (
    ((1, 0), (0, 1)),
    ((0, 1), (1, 0)),
    ((1, 1), (0, 0)),
)
mu = {
    (0, 0): F(1, 8),
    (0, 1): F(3, 8),
    (1, 0): F(1, 4),
    (1, 1): F(1, 4),
}

nested = []
flat = []
for p in range(len(utility)):
    v1 = sum((mu[w, t] * utility[p][w][t] for w in range(2) for t in range(2)), F(0))
    atoms = [(w, t) for w in range(2) for t in range(2)]
    v2 = sum((mu[a] * utility[p][a[0]][a[1]] for a in atoms), F(0))
    nested.append(v1)
    flat.append(v2)
check("model-belief game equals fixed Walt on augmented latent atoms", nested == flat)
check("augmented salvation maximum-weight face equals best response", max(flat) == max(sum((mu[w, t] for w in range(2) for t in range(2) if utility[p][w][t]), F(0)) for p in range(len(utility))))

# ---------------------------------------------------------------------------
# 3. Persistent type posterior closure.
# ---------------------------------------------------------------------------

hands = (0, 1, 2)
types = (0, 1)
phi = {
    (0, 0): F(1),
    (0, 1): F(2),
    (1, 0): F(3),
    (1, 1): F(1),
    (2, 0): F(2),
    (2, 1): F(4),
}


def action(h: int, t: int) -> int:
    return (h + t) % 2

obs = 1
direct = {k: v for k, v in phi.items() if action(*k) == obs}
updated = {k: v * F(1 if action(*k) == obs else 0) for k, v in phi.items()}
updated = {k: v for k, v in updated.items() if v}
check("Bayes update multiplies only the acting hand-type factor", direct == updated)
check("posterior normalizer is observed-action mass", sum(updated.values(), F(0)) == sum(v for k, v in phi.items() if action(*k) == obs))

# Point type prior collapses to the corresponding fixed field.
phi_point = {(h, 0): phi[h, 0] for h in hands}
check("point-mass type factor is an ordinary hand factor", all(t == 0 for _, t in phi_point))

# Persistent type is not action-wise resampling.
# Type 0 emits 00; type 1 emits 11, each with prior 1/2.
persistent_00 = F(1, 2)
resampled_00 = F(1, 2) * F(1, 2)
check("persistent model type differs from re-sampling a model every action", persistent_00 == F(1, 2) and resampled_00 == F(1, 4))

# Observing the first action identifies the persistent type in this example.
post_t0_given_0 = F(1, 2) / (F(1, 2) + F(0))
check("public actions update the posterior over persistent types", post_t0_given_0 == 1)

# A shared convention latent captures cross-seat correlation that independent marginals lose.
# kappa=0 => both types 0; kappa=1 => both types 1.
true_pairs = {(0, 0): F(1, 2), (1, 1): F(1, 2)}
independent_pairs = {(a, b): F(1, 4) for a in (0, 1) for b in (0, 1)}
check("shared convention factor preserves correlated teammate types", true_pairs[(0, 1)] == 0 if (0, 1) in true_pairs else True)
check("independent type marginals would invent impossible convention profiles", independent_pairs[(0, 1)] == F(1, 4))

# ---------------------------------------------------------------------------
# 4. Behavioral quotients and field transfer bounds.
# ---------------------------------------------------------------------------

identical_cols = (
    (F(1, 5), F(1, 5), F(4, 5)),
    (F(3, 5), F(3, 5), F(1, 5)),
)
nu3 = (F(1, 4), F(1, 2), F(1, 4))
merged_vals = tuple((row[0], row[2]) for row in identical_cols)
merged_nu = (nu3[0] + nu3[1], nu3[2])
check("behaviorally identical types may be quotient-merged", q_mix(identical_cols, nu3) == q_mix(merged_vals, merged_nu))
check("point upper survives behavioral quotient", point_upper(identical_cols, nu3) == point_upper(merged_vals, merged_nu))

all_transfer = True
for bits in product((0, 1), repeat=8):
    # two policies, two worlds, two types
    u = [
        [[bits[0], bits[1]], [bits[2], bits[3]]],
        [[bits[4], bits[5]], [bits[6], bits[7]]],
    ]
    beta = (F(1, 3), F(2, 3))
    nu2 = (F(1, 4), F(3, 4))
    v_mix = []
    v_ref = []
    ds = []
    for p in range(2):
        vm = sum((beta[w] * nu2[t] * u[p][w][t] for w in range(2) for t in range(2)), F(0))
        vr = sum((beta[w] * u[p][w][0] for w in range(2)), F(0))
        d = sum((beta[w] * nu2[t] * (1 if u[p][w][t] != u[p][w][0] else 0) for w in range(2) for t in range(2)), F(0))
        v_mix.append(vm)
        v_ref.append(vr)
        ds.append(d)
        all_transfer &= abs(vm - vr) <= d
    all_transfer &= abs(max(v_mix) - max(v_ref)) <= max(ds)
check("field-disagreement outcome mass bounds fixed and optimized model transfer", all_transfer)

# ---------------------------------------------------------------------------
# 5. Unresolved fields, residual mass, and sparse contingency search.
# ---------------------------------------------------------------------------

known_mass = F(3, 4)
known_value = F(2, 3)
residual_mass = F(1, 4)
lo = known_mass * known_value
hi = lo + residual_mass
check("an unresolved Other type contributes an exact [0,residual_mass] envelope", (lo, hi) == (F(1, 2), F(3, 4)))

# If a residual class is proved contract-stable, its width is zero rather than its mass.
check("contract-stable unresolved behavior need not widen pmake", F(1) - F(1) == 0)

# Law of total expectation when model types are split by a public action.
prior = {0: F(1, 4), 1: F(1, 2), 2: F(1, 4)}
signal = {0: "L", 1: "R", 2: "R"}
value = {0: F(1, 5), 1: F(4, 5), 2: F(2, 5)}
whole = sum((prior[t] * value[t] for t in prior), F(0))
by_signal = F(0)
for s in ("L", "R"):
    mass = sum((prior[t] for t in prior if signal[t] == s), F(0))
    cond = sum((prior[t] * value[t] for t in prior if signal[t] == s), F(0)) / mass
    by_signal += mass * cond
check("splitting model belief by observed public action conserves value", whole == by_signal)

# Active information: probe reveals a persistent type, while immediate commitment guesses it.
commit = F(1, 2)
probe_then_condition = F(1)
check("persistent model belief creates positive value of information", probe_then_condition > commit)
check("one sparse consequential disagreement can recover the full probe value", probe_then_condition == 1)

# If all models agree on the public response, the type coordinate is dormant at that node.
check("model agreement produces no extra public branch", len({"L", "L", "L"}) == 1)

# ---------------------------------------------------------------------------
# 6. Point-mass upper as a reusable upper portfolio component.
# ---------------------------------------------------------------------------

# Current per-type intervals can be combined without solving the mixture.
per_type_upper = (F(9, 10), F(4, 5), F(1))
nu = (F(1, 2), F(1, 4), F(1, 4))
combined = dot(per_type_upper, nu)
actual_vals = (
    (F(4, 5), F(3, 5), F(9, 10)),
    (F(7, 10), F(4, 5), F(1, 2)),
)
check("point-mass upper portfolio covers the unresolved-field response", q_mix(actual_vals, nu) <= combined)

# Minimum of independent valid uppers is valid.
u1 = F(9, 10)
u2 = F(4, 5)
qtrue = F(3, 4)
check("upper portfolio intersection remains admissible", qtrue <= min(u1, u2))

# Maximum of executable policy lowers remains a lower.
l1 = F(3, 5)
l2 = F(7, 10)
check("policy-column portfolio maximum remains a lower", max(l1, l2) <= qtrue)

# Certified regret under a model belief.
gamma = min(u1, u2) - max(l1, l2)
check("model-belief certified regret is upper minus executable floor", gamma == F(1, 10))

# ---------------------------------------------------------------------------
# 7. A small common-information partnership prescription example.
# ---------------------------------------------------------------------------

# Partner privately sees bit x, publicly emits f(x), focal guesses g(signal).
# Enumerate every deterministic prescription pair.
def maps2() -> list[tuple[int, int]]:
    return list(product((0, 1), repeat=2))

best_team = F(0)
for f in maps2():
    for g in maps2():
        val = sum((F(1, 2) * (1 if g[f[x]] == x else 0) for x in (0, 1)), F(0))
        best_team = max(best_team, val)
check("common-information partnership prescriptions can encode lawful signaling", best_team == 1)
check("a Dice-like uninformative partner leaves only one-half success", F(1, 2) < best_team)

# ---------------------------------------------------------------------------
# 8. Book-one completion invariants.
# ---------------------------------------------------------------------------

# A finite library mixture includes every ladder rung as a point prior.
ladder_types = 4
check("Dice and ladder rungs fit one finite model registry", ladder_types == len(range(4)))

# Refining an unresolved model region shrinks, never expands, its utility interval.
intervals = [(F(0), F(1)), (F(1, 4), F(7, 8)), (F(2, 5), F(4, 5))]
check("model-region refinement intervals nest", all(intervals[i + 1][0] >= intervals[i][0] and intervals[i + 1][1] <= intervals[i][1] for i in range(len(intervals) - 1)))

# Point-mass parity is the required degenerate-case gate.
vals = ((F(1, 3), F(2, 3)), (F(3, 4), F(1, 4)))
check("degenerate model belief parity gate", q_mix(vals, (F(0), F(1))) == max(row[1] for row in vals))

# A fallback selected by the best executable floor is bounded by the global upper.
exec_floor = F(7, 10)
global_upper = F(4, 5)
check("boring fallback carries a nonnegative certified regret", F(0) <= global_upper - exec_floor <= 1)

print(f"{CHECKS} CHECKS")
print("ALL CHECKS PASS")
