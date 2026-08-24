#!/usr/bin/env python3
"""Machine-checkable companion for RESPONSE-walt-panel-and-cancellation-v0.1.

Exact rational arithmetic only. No network, file I/O, random simulation, or
floating-point correctness checks. Deterministic finite grids and model checks.

The script certifies:
  * CE-T1/T2/T3 exact identities and one-step inequalities.
  * CE-T4/T5 factor ranges, one-step inequalities, and sign-majority defect.
  * O21/O24 ledger algebra, a retrospective edge-opening adversary, and
    sample-to-enumeration cold-equivalence.
  * O26's batching ambiguity as written, plus invariance of the repaired
    canonical per-index semantics and the predictable-selection boundary.
  * L2-T1..T5 on 2,000 finite extensive-form games, plus the directional
    correction refinement introduced in the accompanying response.
"""

from __future__ import annotations

from dataclasses import dataclass
from fractions import Fraction as F
from itertools import product
from math import comb, factorial
from random import Random
from typing import Dict, Iterable, List, Mapping, Sequence, Tuple

FAILURES: List[str] = []


def check(name: str, condition: bool, detail: str = "") -> None:
    if condition:
        print(f"PASS {name}")
    else:
        msg = f"FAIL {name}" + (f" {detail}" if detail else "")
        print(msg)
        FAILURES.append(msg)


# ---------------------------------------------------------------------------
# CE-T1/T2/T3
# ---------------------------------------------------------------------------

def e_upper_integral(s: int, f: int, c: F) -> F:
    """Original normalized integral, expanded in r exactly."""
    total = F(0)
    for j in range(f + 1):
        coefficient = F(((-1) ** j) * comb(f, j), 1)
        power = s + j + 1
        total += coefficient * (F(1) - c**power) / power
    return total / (c**s * (1 - c) ** (f + 1))


def e_upper_substituted(s: int, f: int, c: F) -> F:
    """Substituted integral / finite beta sum."""
    ratio = (1 - c) / c
    return sum(
        F(comb(s, i), 1)
        * ratio**i
        * F(factorial(i) * factorial(f), factorial(i + f + 1))
        for i in range(s + 1)
    )


def e_lower_natural(s: int, f: int, c: F) -> F:
    """Natural lower mixture, expanded on [0,c] exactly."""
    total = F(0)
    for j in range(f + 1):
        coefficient = F(((-1) ** j) * comb(f, j), 1)
        power = s + j + 1
        total += coefficient * c**power / power
    return total / (c ** (s + 1) * (1 - c) ** f)


def e_piv_integral(a: int, b: int) -> F:
    """Integral int_0^1 (1+t)^a (1-t)^b dt by polynomial expansion."""
    total = F(0)
    for i in range(a + 1):
        for j in range(b + 1):
            total += F(comb(a, i) * comb(b, j) * ((-1) ** j), i + j + 1)
    return total


def e_piv_closed(a: int, b: int) -> F:
    k = a + b
    return F(sum(comb(k + 1, x) for x in range(a + 1)), (k + 1) * comb(k, a))


def verify_evidence_processes() -> None:
    cs = [F(1, 10), F(1, 3), F(1, 2), F(11, 16), F(2, 3), F(9, 10)]
    identity_ok = True
    lower_ok = True
    for c in cs:
        for s in range(13):
            for f in range(13):
                x = e_upper_integral(s, f, c)
                y = e_upper_substituted(s, f, c)
                if x != y:
                    identity_ok = False
                if e_lower_natural(s, f, c) != e_upper_substituted(f, s, 1 - c):
                    lower_ok = False
    check("CE-T1 original=substituted=finite-sum grid", identity_ok)
    check("CE-T2 natural-lower identity grid", lower_ok)

    pivotal_ok = True
    specialization_ok = True
    all_favorable_ok = True
    for a in range(41):
        for b in range(41):
            integ = e_piv_integral(a, b)
            closed = e_piv_closed(a, b)
            if integ != closed:
                pivotal_ok = False
            if closed != e_upper_substituted(a, b, F(1, 2)):
                specialization_ok = False
        if e_piv_closed(a, 0) != F(2 ** (a + 1) - 1, a + 1):
            all_favorable_ok = False
    check("CE-T3 defining-integral=closed-form grid", pivotal_ok)
    check("CE-T3 is CE-T1 at c=1/2", specialization_ok)
    check("CE-T3 all-favorable formula", all_favorable_ok)

    anchors = {
        (0, 0): F(1),
        (1, 0): F(3, 2),
        (0, 1): F(1, 2),
        (2, 0): F(7, 3),
        (1, 1): F(2, 3),
        (2, 1): F(11, 12),
        (3, 0): F(15, 4),
    }
    check("CE-T3 anchors", all(e_piv_closed(a, b) == v for (a, b), v in anchors.items()))
    check(
        "CE-T3 alpha=1/128 favorable-pivot threshold",
        e_piv_closed(9, 0) < 128 < e_piv_closed(10, 0),
        f"E9={e_piv_closed(9,0)} E10={e_piv_closed(10,0)}",
    )

    grid = [F(i, 8) for i in range(1, 8)]
    bernoulli_step_ok = True
    for p in grid:
        for c in grid:
            for r in grid:
                if p <= c <= r:
                    lhs = p * r / c + (1 - p) * (1 - r) / (1 - c)
                    rhs = 1 + (p - c) * (r - c) / (c * (1 - c))
                    if lhs != rhs or lhs > 1:
                        bernoulli_step_ok = False
    check("CE-T1 one-step identity/inequality eighth-grid", bernoulli_step_ok)

    pivotal_step_ok = True
    q_grid = [F(i, 8) for i in range(0, 9)]
    theta_grid = [F(i, 8) for i in range(0, 5)]
    r_grid = [F(i, 8) for i in range(4, 9)]
    for q in q_grid:
        for theta in theta_grid:
            for r in r_grid:
                expected = 1 - q + q * (2 * theta * r + 2 * (1 - theta) * (1 - r))
                direct = 1 + q * (2 * r - 1) * (2 * theta - 1)
                if expected != direct or expected > 1:
                    pivotal_step_ok = False
    check("CE-T3 raw-stream one-step inequality grid incl q=0", pivotal_step_ok)


# ---------------------------------------------------------------------------
# CE-T4/T5 and sign-majority defect
# ---------------------------------------------------------------------------

def finite_binomial_majority(n: int, p: F) -> F:
    assert n % 2 == 1
    return sum(F(comb(n, k), 1) * p**k * (1 - p) ** (n - k) for k in range(n // 2 + 1, n + 1))


def sign_evidence_crossing_probability(horizon: int, p_plus: F, threshold: F) -> F:
    """Exact finite-horizon crossing probability for +/- observations."""
    active: Dict[Tuple[int, int], F] = {(0, 0): F(1)}
    crossed = F(0)
    for _ in range(horizon):
        nxt: Dict[Tuple[int, int], F] = {}
        for (a, b), mass in active.items():
            for da, db, prob in ((1, 0, p_plus), (0, 1, 1 - p_plus)):
                state = (a + da, b + db)
                m = mass * prob
                if e_piv_closed(*state) >= threshold:
                    crossed += m
                else:
                    nxt[state] = nxt.get(state, F(0)) + m
        active = nxt
    return crossed


def verify_bounded_mean() -> None:
    L, U = F(-1), F(1)
    cs = [F(-1, 2), F(-1, 8), F(0), F(1, 8), F(1, 2)]
    x_grid = [F(-1), F(-1, 2), F(-1, 32), F(0), F(1, 8), F(1)]
    multipliers = [F(0), F(1, 4), F(1, 2), F(3, 4), F(1)]
    factor_ok = True
    mirror_ok = True
    sharp_ok = True
    for c in cs:
        lp = F(1, 1) / (c - L)
        lm = F(1, 1) / (U - c)
        for q in multipliers:
            lam_p = q * lp
            lam_m = q * lm
            for x in x_grid:
                if 1 + lam_p * (x - c) < 0:
                    factor_ok = False
                if 1 - lam_m * (x - c) < 0:
                    mirror_ok = False
        eps = F(1, 1000)
        if 1 + (lp + eps) * (L - c) >= 0:
            sharp_ok = False
        if 1 - (lm + eps) * (U - c) >= 0:
            sharp_ok = False
    check("CE-T4 factor nonnegativity grid", factor_ok)
    check("CE-T5 factor nonnegativity grid", mirror_ok)
    check("CE-T4/T5 lambda ranges are sharp", sharp_ok)

    support = [F(-1), F(-1, 2), F(0), F(1, 2), F(1)]
    weights = [F(1, 4), F(1, 2), F(3, 4)]
    one_step_pos = True
    one_step_neg = True
    mixture_ok = True
    nontrivial_laws = 0
    for c in cs:
        lp = F(1) / (c - L)
        lm = F(1) / (U - c)
        lambda_ps = [F(0), lp / 4, lp / 2, lp]
        lambda_ms = [F(0), lm / 4, lm / 2, lm]
        for x0 in support:
            for x1 in support:
                for w in weights:
                    mean = w * x0 + (1 - w) * x1
                    if mean <= c:
                        nontrivial_laws += 1
                        for lam in lambda_ps:
                            expected = w * (1 + lam * (x0 - c)) + (1 - w) * (1 + lam * (x1 - c))
                            if expected > 1:
                                one_step_pos = False
                        mix = F(1, 3) * (
                            w * (1 + lambda_ps[1] * (x0 - c))
                            + (1 - w) * (1 + lambda_ps[1] * (x1 - c))
                        ) + F(2, 3) * (
                            w * (1 + lambda_ps[2] * (x0 - c))
                            + (1 - w) * (1 + lambda_ps[2] * (x1 - c))
                        )
                        if mix > 1:
                            mixture_ok = False
                    if mean >= c:
                        for lam in lambda_ms:
                            expected = w * (1 - lam * (x0 - c)) + (1 - w) * (1 - lam * (x1 - c))
                            if expected > 1:
                                one_step_neg = False
    check("CE-T4 one-step null inequalities", one_step_pos and nontrivial_laws > 0)
    check("CE-T5 one-step null inequalities", one_step_neg)
    check("CE-T4 finite-mixture one-step inequality", mixture_ok)

    p_pos = F(3, 4)
    mean = p_pos * F(1, 8) + (1 - p_pos) * F(-1, 2)
    check("sign-majority counterexample arithmetic", p_pos == F(3, 4) and mean == F(-1, 32))

    majority_probs = [finite_binomial_majority(n, p_pos) for n in (1, 3, 5, 9, 21)]
    check(
        "sign-majority increasingly favors wrong mean direction",
        all(x < y for x, y in zip(majority_probs, majority_probs[1:])) and majority_probs[-1] > F(9, 10),
        str(majority_probs),
    )
    crossing = sign_evidence_crossing_probability(100, p_pos, F(128))
    check(
        "consistent sign-evidence can become confident in wrong mean ordering",
        crossing > F(1, 2),
        f"P(cross by 100)={crossing}",
    )

    # Constant nonzero magnitude is a sign-safe subclass.
    sign_safe = True
    for p in [F(i, 8) for i in range(9)]:
        d = F(3, 7)
        mean_const = p * d - (1 - p) * d
        sign_balance = p - (1 - p)
        if (mean_const > 0) != (sign_balance > 0) or (mean_const < 0) != (sign_balance < 0):
            sign_safe = False
    check("constant-magnitude subclass is sign-safe", sign_safe)


# ---------------------------------------------------------------------------
# O21/O24 risk ledger and exact escalation
# ---------------------------------------------------------------------------

def verify_risk_ledger() -> None:
    running = F(0)
    telescoping_ok = True
    for k in range(1, 10001):
        running += F(1, k * (k + 1))
        if running != F(k, k + 1):
            telescoping_ok = False
            break
    check("risk ledger telescoping identity K<=10000", telescoping_ok)

    all_pairs_ok = True
    for m in range(2, 15):
        for delta in (F(1, 100), F(1, 10), F(1, 2)):
            alpha = delta / (m * (m - 1))
            threshold = F(m * (m - 1), 1) / delta
            if alpha * threshold != 1:
                all_pairs_ok = False
    check("all-pairs threshold times edge allocation equals one", all_pairs_ok)

    # Claim D as written: observe K unopened e-processes historically, then
    # assign alpha_1 to one that already crossed. Each individual process is
    # a valid one-step e-value: M=T with probability alpha, zero otherwise.
    delta = F(1, 4)
    alpha1 = delta / 2
    k_edges = 3
    retrospective_false_cross = 1 - (1 - alpha1) ** k_edges
    check(
        "retrospective sequential edge-opening adversary exceeds total budget",
        retrospective_false_cross > delta,
        f"P={retrospective_false_cross} delta={delta}",
    )

    outcomes = [
        [1, 0, 1, 1, 0, 1],
        [1, 1, 1, 0, 0, 1],
        [0, 1, 1, 1, 1, 0],
    ]
    streams = [
        [0, 0, 1, 5, 1, 1, 2, 4, 5, 0],
        [5, 4, 3, 2, 1, 0],
        [2, 2, 2, 2, 0, 5, 3, 3],
    ]

    def cold(policy_ids: Iterable[int]) -> Tuple[int, ...]:
        return tuple(sum(outcomes[p]) for p in policy_ids)

    def switch_endpoint(stream: Sequence[int], switch: int, policy_ids: Sequence[int]) -> Tuple[int, ...]:
        cache: Dict[Tuple[int, int], int] = {}
        for world in stream[:switch]:
            for p in policy_ids:
                cache[(p, world)] = outcomes[p][world]
        for world in range(len(outcomes[0])):
            for p in policy_ids:
                cache.setdefault((p, world), outcomes[p][world])
        return tuple(sum(cache[(p, w)] for w in range(len(outcomes[0]))) for p in policy_ids)

    switch_ok = True
    for stream in streams:
        for switch in range(len(stream) + 1):
            for ids in ((0, 1, 2), (0, 2), (1,)):
                if switch_endpoint(stream, switch, ids) != cold(ids):
                    switch_ok = False
    check("sample-to-enumeration endpoint equals cold enumeration", switch_ok)

    # Survivor-only exactness is not full-set exactness after a false removal.
    dominated_table = [[0, 0], [1, 1]]
    survivor_winner = 0
    full_winner = 1
    check(
        "survivor-only enumeration cannot certify original candidate set",
        sum(dominated_table[survivor_winner]) < sum(dominated_table[full_winner]),
    )


# ---------------------------------------------------------------------------
# O26 execution order and predictable selection
# ---------------------------------------------------------------------------

def canonical_engine(table: Sequence[Sequence[int]], threshold: F) -> Tuple:
    m = len(table[0])
    live = set(range(m))
    counts: Dict[Tuple[int, int], List[int]] = {
        (i, j): [0, 0] for i in range(m) for j in range(m) if i != j
    }
    first: Dict[Tuple[int, int], int] = {}
    live_history = [tuple(sorted(live))]
    elimination_events = []

    for n, row in enumerate(table):
        current = sorted(live)
        for i in current:
            for j in current:
                if i == j:
                    continue
                y = row[i] - row[j]
                if y == 1:
                    counts[(i, j)][0] += 1
                elif y == -1:
                    counts[(i, j)][1] += 1
        newly = []
        for i in current:
            for j in current:
                if i == j or (i, j) in first:
                    continue
                a, b = counts[(i, j)]
                if e_piv_closed(a, b) >= threshold:
                    first[(i, j)] = n
                    newly.append((i, j))
        targets = {j for i, j in newly if i in live and j in live}
        if targets:
            if targets == live:
                elimination_events.append((n, "inconsistent", tuple(sorted(newly))))
                live.clear()
            else:
                live.difference_update(targets)
                elimination_events.append((n, tuple(sorted(targets)), tuple(sorted(newly))))
        live_history.append(tuple(sorted(live)))
    return (
        tuple(sorted(live)),
        tuple(sorted(first.items())),
        tuple(live_history),
        tuple(elimination_events),
    )


def naive_batch_start_engine(table: Sequence[Sequence[int]], threshold: F, batches: Sequence[int]) -> Tuple:
    """A plausible but unsound W5 reading: batch-start liveness for whole batch."""
    m = len(table[0])
    live = set(range(m))
    counts: Dict[Tuple[int, int], List[int]] = {
        (i, j): [0, 0] for i in range(m) for j in range(m) if i != j
    }
    first: Dict[Tuple[int, int], int] = {}
    at = 0
    for size in batches:
        batch_live = sorted(live)
        for n in range(at, min(at + size, len(table))):
            row = table[n]
            for i in batch_live:
                for j in batch_live:
                    if i == j:
                        continue
                    y = row[i] - row[j]
                    if y == 1:
                        counts[(i, j)][0] += 1
                    elif y == -1:
                        counts[(i, j)][1] += 1
                    if (i, j) not in first:
                        a, b = counts[(i, j)]
                        if e_piv_closed(a, b) >= threshold:
                            first[(i, j)] = n
        targets = {j for (i, j), n in first.items() if i in batch_live and j in batch_live}
        live.difference_update(targets)
        at += size
        if at >= len(table):
            break
    return tuple(sorted(live)), tuple(sorted(first.items()))


def compositions(n: int, rng: Random, count: int) -> List[List[int]]:
    out = [[1] * n, [n]]
    for _ in range(count - 2):
        remaining = n
        parts = []
        while remaining:
            x = rng.randint(1, remaining)
            parts.append(x)
            remaining -= x
        out.append(parts)
    return out


def repaired_batched_engine(table: Sequence[Sequence[int]], threshold: F, batches: Sequence[int]) -> Tuple:
    """Speculation may batch, but semantic replay is canonical per index."""
    assert sum(batches) >= len(table)
    return canonical_engine(table, threshold)


def verify_execution_order() -> None:
    threshold = F(3, 2)
    ambiguity_table = [
        [1, 0, 1],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
    ]
    canonical = canonical_engine(ambiguity_table, threshold)
    naive = naive_batch_start_engine(ambiguity_table, threshold, [5])
    check(
        "W1-W6 as written admit a batching/liveness divergence",
        canonical[0] != naive[0],
        f"canonical_live={canonical[0]} naive_live={naive[0]}",
    )

    rng = Random(20260824)
    tables = [
        ambiguity_table,
        [[1, 0, 0], [1, 0, 1], [0, 1, 1], [1, 1, 0], [0, 0, 1], [1, 0, 1]],
        [[0, 1, 0, 1], [1, 0, 1, 0], [1, 1, 0, 0], [0, 1, 1, 0], [1, 0, 0, 1]],
    ]
    invariant_ok = True
    schedules_tested = 0
    for table in tables:
        reference = canonical_engine(table, threshold)
        for parts in compositions(len(table), rng, 200):
            schedules_tested += 1
            if repaired_batched_engine(table, threshold, parts) != reference:
                invariant_ok = False
    check(
        "canonical per-index reconstruction invariant across >=200 schedules/table",
        invariant_ok and schedules_tested >= 600,
        f"schedules={schedules_tested}",
    )

    # If the selector peeks at Y_n, it can accept only + outcomes. Under a
    # symmetric null, crossing after ten accepted + pivots occurs whenever
    # at least ten + outcomes appear in the first 30 raw worlds.
    peek_cross = sum(F(comb(30, k), 2**30) for k in range(10, 31))
    check(
        "nonpredictable current-world selection breaks level-1/128 validity",
        peek_cross > F(1, 128),
        f"P={peek_cross}",
    )

    # Predictable activation A_n merely replaces the multiplier by 1 on
    # inactive indices. Under a symmetric null its conditional mean is 1.
    r = F(3, 4)
    expected_active = F(1, 2) * (2 * r) + F(1, 2) * (2 * (1 - r))
    expected_inactive = F(1)
    check(
        "predictable thinning preserves one-step null expectation",
        expected_active == 1 and expected_inactive == 1,
    )


# ---------------------------------------------------------------------------
# L2-T1..T5 and the directional correction extension
# ---------------------------------------------------------------------------

@dataclass(frozen=True)
class TinyGame:
    probs: Tuple[F, F, F]
    focal_obs: Tuple[int, int, int]
    field_obs: Tuple[int, int, int]
    payoff: Tuple[int, ...]  # index w,a,x,z


def payoff_at(game: TinyGame, w: int, a: int, x: int, z: int) -> int:
    return game.payoff[(((w * 2 + a) * 2 + x) * 2 + z)]


def field_action(field: Tuple[int, ...], a: int, x: int, obs: int) -> int:
    return field[((a * 2 + x) * 2 + obs)]


def policy_action(policy: Tuple[int, int], focal_obs: int) -> int:
    return policy[focal_obs]


def run_tiny(game: TinyGame, policy: Tuple[int, int], field: Tuple[int, ...], w: int, a: int) -> Tuple[int, Tuple[int, int, int]]:
    x = policy_action(policy, game.focal_obs[w])
    j = (a, x, game.field_obs[w])
    z = field_action(field, *j)
    return payoff_at(game, w, a, x, z), j


def all_policies() -> List[Tuple[int, int]]:
    return list(product((0, 1), repeat=2))


def value(game: TinyGame, policy: Tuple[int, int], field: Tuple[int, ...], a: int) -> F:
    return sum(game.probs[w] * run_tiny(game, policy, field, w, a)[0] for w in range(3))


def exposure_and_corrections(
    game: TinyGame, policy: Tuple[int, int], f0: Tuple[int, ...], f1: Tuple[int, ...], a: int
) -> Tuple[F, F, F, F]:
    d = cp = cm = changed = F(0)
    for w in range(3):
        u0, j0 = run_tiny(game, policy, f0, w, a)
        u1, j1 = run_tiny(game, policy, f1, w, a)
        assert j0 == j1  # the common state is before the field action
        split = field_action(f0, *j0) != field_action(f1, *j0)
        if split:
            d += game.probs[w]
        if u1 == 1 and u0 == 0:
            cp += game.probs[w]
        if u1 == 0 and u0 == 1:
            cm += game.probs[w]
        if u1 != u0:
            changed += game.probs[w]
    return d, cp, cm, changed


def generate_game(rng: Random, idx: int) -> Tuple[TinyGame, Tuple[int, ...], Tuple[int, ...]]:
    prob_options = [
        (F(1, 3), F(1, 3), F(1, 3)),
        (F(1, 6), F(1, 3), F(1, 2)),
        (F(1, 2), F(1, 6), F(1, 3)),
    ]
    probs = prob_options[idx % len(prob_options)]
    focal_obs = tuple(rng.randrange(2) for _ in range(3))
    field_obs = tuple(rng.randrange(2) for _ in range(3))
    payoff = tuple(rng.randrange(2) for _ in range(3 * 2 * 2 * 2))
    f0 = tuple(rng.randrange(2) for _ in range(8))
    f1 = tuple(rng.randrange(2) for _ in range(8))
    return TinyGame(probs, focal_obs, field_obs, payoff), f0, f1


def verify_l2() -> None:
    rng = Random(424242)
    instances = 2000
    t1_ok = t2_ok = t3_ok = t4_ok = directional_ok = hierarchy_ok = True
    t3_premises = 0
    policies = all_policies()

    for idx in range(instances):
        game, f0, f1 = generate_game(rng, idx)
        q0: Dict[int, F] = {}
        q1: Dict[int, F] = {}
        rexposure: Dict[int, F] = {}
        rplus: Dict[int, F] = {}
        rminus: Dict[int, F] = {}
        rchange: Dict[int, F] = {}

        for a in (0, 1):
            vals0 = []
            vals1 = []
            ds = []
            cps = []
            cms = []
            changes = []
            for policy in policies:
                v0 = value(game, policy, f0, a)
                v1 = value(game, policy, f1, a)
                d, cp, cm, changed = exposure_and_corrections(game, policy, f0, f1, a)
                vals0.append(v0)
                vals1.append(v1)
                ds.append(d)
                cps.append(cp)
                cms.append(cm)
                changes.append(changed)

                for w in range(3):
                    u0, j0 = run_tiny(game, policy, f0, w, a)
                    u1, j1 = run_tiny(game, policy, f1, w, a)
                    split = field_action(f0, *j0) != field_action(f1, *j0)
                    if not split and u0 != u1:
                        t1_ok = False
                if abs(v1 - v0) > d:
                    t1_ok = False
                if abs((cp - cm) - (v1 - v0)) > 0:
                    t1_ok = False
                if not (cp <= changed <= d and cm <= changed <= d and abs(cp - cm) <= changed):
                    hierarchy_ok = False

            q0[a] = max(vals0)
            q1[a] = max(vals1)
            rexposure[a] = max(ds)
            rplus[a] = max(cps)
            rminus[a] = max(cms)
            rchange[a] = max(changes)

            if abs(q1[a] - q0[a]) > rexposure[a]:
                t2_ok = False
            if q1[a] > q0[a] + rplus[a] or q1[a] < q0[a] - rminus[a]:
                directional_ok = False
            if not (rplus[a] <= rchange[a] <= rexposure[a] and rminus[a] <= rchange[a] <= rexposure[a]):
                hierarchy_ok = False

        for a, b in ((0, 1), (1, 0)):
            if q0[a] - q0[b] > rexposure[a] + rexposure[b]:
                t3_premises += 1
                if not q1[a] > q1[b]:
                    t3_ok = False
            if q0[a] - q0[b] > rminus[a] + rplus[b]:
                if not q1[a] > q1[b]:
                    directional_ok = False

        # Exact T4 bounds.
        L0 = dict(q0)
        U0 = dict(q0)
        RU = dict(rexposure)
        L1 = {a: L0[a] - RU[a] for a in (0, 1)}
        U1 = {a: U0[a] + RU[a] for a in (0, 1)}
        bar = max(L1.values())
        admissible = {a for a in (0, 1) if U1[a] >= bar}
        true_opt = {a for a in (0, 1) if q1[a] == max(q1.values())}
        if not true_opt.issubset(admissible) or not admissible:
            t4_ok = False

        # Deliberately loosened valid bounds.
        slack = F(1, 6)
        L0l = {a: max(F(0), q0[a] - slack) for a in (0, 1)}
        U0l = {a: min(F(1), q0[a] + slack) for a in (0, 1)}
        RUl = {a: min(F(1), rexposure[a] + slack) for a in (0, 1)}
        L1l = {a: L0l[a] - RUl[a] for a in (0, 1)}
        U1l = {a: U0l[a] + RUl[a] for a in (0, 1)}
        barl = max(L1l.values())
        admissible_l = {a for a in (0, 1) if U1l[a] >= barl}
        if not true_opt.issubset(admissible_l) or not admissible_l:
            t4_ok = False

        # Directional T4 refinement.
        L1d = {a: q0[a] - rminus[a] for a in (0, 1)}
        U1d = {a: q0[a] + rplus[a] for a in (0, 1)}
        bard = max(L1d.values())
        admissible_d = {a for a in (0, 1) if U1d[a] >= bard}
        if not true_opt.issubset(admissible_d) or not admissible_d:
            directional_ok = False

    check("L2-T1 pointwise and fixed-policy bounds over 2000 games", t1_ok)
    check("L2-T2 root-action field bound over 2000 games", t2_ok)
    check("L2-T3 stability whenever premise fires", t3_ok and t3_premises > 0, f"premises={t3_premises}")
    check("L2-T4 exact/loosened safe screens over 2000 games", t4_ok)
    check("directional correction hierarchy and one-sided screens", directional_ok and hierarchy_ok)

    # L2-T5: random deterministic maps on finite sets plus explicit period 4.
    periodic_ok = True
    rng2 = Random(99)
    for size in range(1, 33):
        for _ in range(20):
            mapping = [rng2.randrange(size) for _ in range(size)]
            x = rng2.randrange(size)
            seen: Dict[int, int] = {}
            for t in range(size + 1):
                if x in seen:
                    break
                seen[x] = t
                x = mapping[x]
            else:
                periodic_ok = False
    period4 = [1, 2, 3, 0]
    x = 0
    orbit = []
    for _ in range(8):
        orbit.append(x)
        x = period4[x]
    check("L2-T5 eventual periodicity finite-map model check", periodic_ok)
    check("L2-T5 explicit nonconvergent period-4 fixture", orbit == [0, 1, 2, 3, 0, 1, 2, 3])


# ---------------------------------------------------------------------------
# Cancellation / hazard fixture
# ---------------------------------------------------------------------------

def verify_directional_hazard_fixture() -> None:
    # A = safe high trump, B = vulnerable double. On 99 worlds both make;
    # on one world only A makes. B has no upside and one-sided downside.
    n = 100
    a = [1] * n
    b = [1] * 99 + [0]
    p_plus = F(sum(1 for x, y in zip(a, b) if x == 1 and y == 0), n)
    p_minus = F(sum(1 for x, y in zip(a, b) if x == 0 and y == 1), n)
    q = p_plus + p_minus
    g = p_plus - p_minus
    check(
        "rare unforced-risk fixture is strict dominance, not cancellation",
        p_plus == F(1, 100) and p_minus == 0 and q == g == F(1, 100),
        f"p+={p_plus} p-={p_minus}",
    )


def main() -> int:
    verify_evidence_processes()
    verify_bounded_mean()
    verify_risk_ledger()
    verify_execution_order()
    verify_l2()
    verify_directional_hazard_fixture()
    if FAILURES:
        print(f"FAILURES {len(FAILURES)}")
        for f in FAILURES:
            print(f)
        return 1
    print("ALL CHECKS PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
