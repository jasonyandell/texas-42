#!/usr/bin/env python3
"""
Exact-rational companion for
DESIGN-walt-anytime-proof-state-and-score-calculus-v0.1.md

No external dependencies.  The program checks finite instances of the
score-threshold, proof-state, regret, laydown, and merge-before-max claims.
It is a verifier companion, not an implementation of Texas 42.
"""

from __future__ import annotations

from dataclasses import dataclass
from fractions import Fraction
from itertools import product
from typing import Callable, Iterable, Sequence


CHECKS = 0


def check(name: str, condition: bool) -> None:
    global CHECKS
    if not condition:
        raise AssertionError(name)
    CHECKS += 1
    print(f"PASS {CHECKS:02d}: {name}")


def indicator(x: bool) -> int:
    return 1 if x else 0


def tails(profile: Sequence[int]) -> list[int]:
    out = [0] * len(profile)
    acc = 0
    for i in range(len(profile) - 1, -1, -1):
        acc += profile[i]
        out[i] = acc
    return out


def expected_score(profile: Sequence[int]) -> Fraction:
    z = sum(profile)
    return Fraction(sum(i * m for i, m in enumerate(profile)), z)


def make_prob(profile: Sequence[int], c: int) -> Fraction:
    z = sum(profile)
    return Fraction(sum(profile[c:]), z)


# ---------------------------------------------------------------------------
# 1. Texas 42 score signature.
# ---------------------------------------------------------------------------

COUNT_WEIGHTS = [5, 5, 5, 10, 10]
signature_scores: list[int] = []
for tricks in range(8):
    for mask in range(32):
        score = tricks + sum(
            COUNT_WEIGHTS[i] for i in range(5) if mask & (1 << i)
        )
        signature_scores.append(score)

check("42 count signature has 8*32 entries", len(signature_scores) == 256)
check("42 score signature ranges exactly from 0 through 42",
      min(signature_scores) == 0 and max(signature_scores) == 42)
check("every integer score 0..42 is represented",
      set(signature_scores) == set(range(43)))


# ---------------------------------------------------------------------------
# 2. Tail-sum identity.
# ---------------------------------------------------------------------------

profiles = [
    [1] + [0] * 42,
    [0] * 42 + [1],
    [1 if i in (0, 21, 42) else 0 for i in range(43)],
    [(i * i + 3 * i + 7) % 11 for i in range(43)],
]
for p in profiles:
    t = tails(p)
    lhs = sum(i * p[i] for i in range(43))
    rhs = sum(t[1:])
    assert lhs == rhs
check("tail-sum identity holds on exact 43-bin profiles", True)


# ---------------------------------------------------------------------------
# 3. Cell projection, contract-sensitive width, and score-width area.
# ---------------------------------------------------------------------------

@dataclass(frozen=True)
class Cell:
    mass: int
    lo: int
    hi: int


def project_cells(cells: Sequence[Cell], contract: int) -> tuple[Fraction, Fraction]:
    z = sum(x.mass for x in cells)
    lower = sum(x.mass for x in cells if x.lo >= contract)
    upper = sum(x.mass for x in cells if x.hi >= contract)
    return Fraction(lower, z), Fraction(upper, z)


def contract_width(cells: Sequence[Cell], contract: int) -> Fraction:
    z = sum(x.mass for x in cells)
    return Fraction(
        sum(x.mass for x in cells if x.lo < contract <= x.hi), z
    )


def point_width(cells: Sequence[Cell]) -> Fraction:
    z = sum(x.mass for x in cells)
    return Fraction(sum(x.mass * (x.hi - x.lo) for x in cells), z)


cells = [
    Cell(5, 30, 30),  # certain fail at 31
    Cell(7, 31, 35),  # certain make at 31
    Cell(3, 29, 33),  # straddles
    Cell(2, 0, 42),   # straddles
]
L, U = project_cells(cells, 31)
check("cell make interval is ordered", L <= U)
check("make interval width equals contract-sensitive residual",
      U - L == contract_width(cells, 31))

area = sum(contract_width(cells, k) for k in range(1, 43))
check("aggregate score width equals area under unresolved-threshold curve",
      area == point_width(cells))

refined = [
    Cell(5, 30, 30),
    Cell(7, 31, 35),
    Cell(2, 29, 30),
    Cell(1, 33, 33),
    Cell(1, 0, 20),
    Cell(1, 40, 42),
]
L2, U2 = project_cells(refined, 31)
check("cell refinement raises make lower", L2 >= L)
check("cell refinement lowers make upper", U2 <= U)
check("cell refinement lowers aggregate point width",
      point_width(refined) <= point_width(cells))


# ---------------------------------------------------------------------------
# 4. Rescue and fragile-make theorems, exhaustive finite check.
# ---------------------------------------------------------------------------

rescue_ok = True
fragile_ok = True
for c in range(0, 13):
    for d in range(0, 5):
        for base in range(0, 13):
            for alt in range(0, 13):
                if alt <= base + d:
                    # Any alternative make not already made by base must lie
                    # in the base policy's d-point rescue band.
                    lhs = indicator(alt >= c and base < c)
                    rhs = indicator(c - d <= base < c)
                    rescue_ok &= lhs <= rhs
                if alt >= base - d:
                    # Any base make lost by alternative must lie in the
                    # fragile-make band.
                    lhs = indicator(base >= c and alt < c)
                    rhs = indicator(c <= base < c + d)
                    fragile_ok &= lhs <= rhs

check("uniform rescue-band inclusion holds exhaustively", rescue_ok)
check("uniform fragile-make inclusion holds exhaustively", fragile_ok)


# ---------------------------------------------------------------------------
# 5. Cellwise rescue bound.
# ---------------------------------------------------------------------------

base_scores = [28, 30, 31, 35, 41]
alt_scores = [31, 30, 34, 34, 42]
masses = [5, 7, 3, 11, 2]
contract = 31
d_caps = [3, 0, 3, 0, 1]
assert all(alt_scores[i] <= base_scores[i] + d_caps[i] for i in range(5))
base_make = Fraction(
    sum(masses[i] for i, s in enumerate(base_scores) if s >= contract),
    sum(masses),
)
alt_make = Fraction(
    sum(masses[i] for i, s in enumerate(alt_scores) if s >= contract),
    sum(masses),
)
rescue = Fraction(
    sum(
        masses[i]
        for i, s in enumerate(base_scores)
        if contract - d_caps[i] <= s < contract
    ),
    sum(masses),
)
check("cellwise rescue mass bounds make improvement",
      alt_make - base_make <= rescue)


# ---------------------------------------------------------------------------
# 6. Loss from perfection / bid 42-N.
# ---------------------------------------------------------------------------

safe = True
for n in range(43):
    contract = 42 - n
    for d in range(n + 1):
        safe &= 42 - d >= contract
check("loss-from-42 at most N guarantees a 42-N contract", safe)


# ---------------------------------------------------------------------------
# 7. Certified regret.
# ---------------------------------------------------------------------------

grids = [Fraction(i, 4) for i in range(5)]
regret_ok = True
tested = 0
for q0, q1, q2 in product(grids, repeat=3):
    qstar = max(q0, q1, q2)
    for pad0, pad1, pad2 in product(grids, repeat=3):
        uppers = [
            min(Fraction(1), q0 + pad0),
            min(Fraction(1), q1 + pad1),
            min(Fraction(1), q2 + pad2),
        ]
        for policy_value in grids:
            if policy_value <= qstar:
                lower = max(Fraction(0), policy_value - Fraction(1, 4))
                gamma = max(uppers) - lower
                regret_ok &= qstar - policy_value <= gamma
                tested += 1
check(f"certified regret inequality holds on {tested} finite cases", regret_ok)


# ---------------------------------------------------------------------------
# 8. Proof bar and executable bar are distinct.
# ---------------------------------------------------------------------------

# Exact grammar existence result = .8, but the best materialized policy is .7.
proof_bar = Fraction(4, 5)
exec_bar = Fraction(7, 10)
global_upper = Fraction(9, 10)
check("proof bar may exceed executable bar", exec_bar < proof_bar)
check("executable regret uses executable bar",
      global_upper - exec_bar == Fraction(1, 5))


# ---------------------------------------------------------------------------
# 9. Threshold-wise maxima are not one executable score profile.
# ---------------------------------------------------------------------------

profile_a = [0] * 43
profile_a[0] = 1
profile_a[42] = 1  # half 0, half 42
profile_b = [0] * 43
profile_b[21] = 2  # always 21
tail_a = tails(profile_a)
tail_b = tails(profile_b)
envelope = [max(a, b) for a, b in zip(tail_a, tail_b)]
# All profiles have common total mass 2.
envelope_expectation = Fraction(sum(envelope[1:]), 2)
check("each source policy has expected score 21",
      expected_score(profile_a) == 21 and expected_score(profile_b) == 21)
check("threshold-wise envelope claims unattained expected score 63/2",
      envelope_expectation == Fraction(63, 2))
check("threshold-wise envelope is not either executable profile",
      envelope != tail_a and envelope != tail_b)


# ---------------------------------------------------------------------------
# 10. Exact fixed-policy score Bellman on a toy public branch tree.
# ---------------------------------------------------------------------------

# Hidden public action x has two terminal score masses; y has one.
branch_x = [0] * 43
branch_x[20] = 2
branch_x[35] = 3
branch_y = [0] * 43
branch_y[42] = 5
parent = [a + b for a, b in zip(branch_x, branch_y)]
check("hidden-node score profiles add branchwise", sum(parent) == 10)
check("score-profile tail reproduces Boolean pmake",
      make_prob(parent, 31) == Fraction(8, 10))


# ---------------------------------------------------------------------------
# 11. Merge-before-max counterexample.
# ---------------------------------------------------------------------------

# Two hidden classes produce the SAME public observation.  At the later focal
# information state, action A wins class 0 and action B wins class 1.
# A fused cellwise max scores both; one lawful common action scores only one.
class0 = {"A": 1, "B": 0}
class1 = {"A": 0, "B": 1}
naive = max(class0.values()) + max(class1.values())
lawful = max(class0["A"] + class1["A"], class0["B"] + class1["B"])
check("cellwise hidden max overstates same-public-action value",
      naive == 2 and lawful == 1)


# ---------------------------------------------------------------------------
# 12. Action ambiguity need not be contract ambiguity.
# ---------------------------------------------------------------------------

stable_scores = [35, 36]
sensitive_scores = [29, 31]
check("different field actions can be contract-stable",
      all(s >= 30 for s in stable_scores))
check("different field actions can be contract-sensitive",
      min(sensitive_scores) < 30 <= max(sensitive_scores))


# ---------------------------------------------------------------------------
# 13. Laydown quantifier hierarchy.
# ---------------------------------------------------------------------------

worlds = ("w0", "w1")
fields = ("friendly", "adversarial")
policies = ("careful", "throw")
score = {
    ("careful", "friendly", "w0"): 42,
    ("careful", "friendly", "w1"): 42,
    ("careful", "adversarial", "w0"): 42,
    ("careful", "adversarial", "w1"): 30,
    ("throw", "friendly", "w0"): 42,
    ("throw", "friendly", "w1"): 20,
    ("throw", "adversarial", "w0"): 20,
    ("throw", "adversarial", "w1"): 20,
}
c = 31
policy_certain = all(score[("careful", "friendly", w)] >= c for w in worlds)
adversarial_policy = all(
    score[("careful", f, w)] >= c for f in fields for w in worlds
)
universal = all(
    score[(p, f, w)] >= c for p in policies for f in fields for w in worlds
)
check("fixed-field certain make need not be adversarially robust",
      policy_certain and not adversarial_policy)
check("adversarially robust or fixed-field certainty need not be universal laydown",
      not universal)


# ---------------------------------------------------------------------------
# 14. Top proof state and monotone refinement.
# ---------------------------------------------------------------------------

@dataclass(frozen=True)
class AbstractState:
    possible_values: frozenset[int]


true_value = 2
top = AbstractState(frozenset(range(5)))
mid = AbstractState(frozenset({1, 2, 3}))
exact = AbstractState(frozenset({2}))
check("top proof state contains the truth", true_value in top.possible_values)
check("refinement shrinks concretization and preserves truth",
      exact.possible_values <= mid.possible_values <= top.possible_values
      and true_value in exact.possible_values)


# ---------------------------------------------------------------------------
# 15. Zero-cost closure is idempotent in a toy interval system.
# ---------------------------------------------------------------------------

def closure(intervals: tuple[tuple[Fraction, Fraction], ...]
            ) -> tuple[tuple[Fraction, Fraction], ...]:
    # Free rule: if any lower bar exceeds another upper, collapse the
    # dominated action's upper/lower to its existing upper point marker.
    # Repeating the rule reaches the same fixed point.
    work = [list(x) for x in intervals]
    changed = True
    while changed:
        changed = False
        bar = max(x[0] for x in work)
        for x in work:
            if x[1] < bar and x[0] != x[1]:
                x[0] = x[1]
                changed = True
    return tuple((x[0], x[1]) for x in work)


x = (
    (Fraction(3, 4), Fraction(1)),
    (Fraction(0), Fraction(1, 2)),
)
cx = closure(x)
check("zero-cost closure is idempotent", closure(cx) == cx)


# ---------------------------------------------------------------------------
# 16. Closure-aware scheduling counterexample.
# ---------------------------------------------------------------------------

# Work A proves a prerequisite but directly changes no root interval.
# Work B can tighten the root only after A.
state = {"prereq": False, "upper": Fraction(1)}
direct_effect_a = Fraction(0)

def work_a(s: dict) -> dict:
    t = dict(s)
    t["prereq"] = True
    return t

def work_b(s: dict) -> dict:
    t = dict(s)
    if t["prereq"]:
        t["upper"] = Fraction(1, 2)
    return t

after_plan = work_b(work_a(state))
check("a zero-direct-effect prerequisite can enable decisive tightening",
      direct_effect_a == 0 and after_plan["upper"] < state["upper"])


# ---------------------------------------------------------------------------
# 17. Selection debt and value debt differ.
# ---------------------------------------------------------------------------

# One action already survives, but its value interval is not a point.
survivors = 1
incumbent_lower = Fraction(3, 4)
incumbent_upper = Fraction(9, 10)
selection_debt = survivors - 1
value_debt = incumbent_upper - incumbent_lower
check("action selection can be complete while value pricing remains open",
      selection_debt == 0 and value_debt > 0)


# ---------------------------------------------------------------------------
# 18. Contract reuse from one exact score profile.
# ---------------------------------------------------------------------------

profile = [0] * 43
profile[20] = 2
profile[30] = 3
profile[42] = 5
values = [make_prob(profile, c) for c in (21, 31, 42)]
check("one score profile answers multiple contracts",
      values == [Fraction(8, 10), Fraction(5, 10), Fraction(5, 10)])


# ---------------------------------------------------------------------------
# 19. A count-threat rescue example.
# ---------------------------------------------------------------------------

# Incumbent scores 28, 31, 40 with masses 4, 5, 1.  A residual region can
# gain at most one five-count and no trick points in each cell.
scores = [28, 31, 40]
masses = [4, 5, 1]
d = 5
c = 31
inc = Fraction(sum(m for s, m in zip(scores, masses) if s >= c), sum(masses))
rescue_upper = Fraction(
    sum(m for s, m in zip(scores, masses) if c - d <= s < c),
    sum(masses),
)
check("one five-count cap localizes all possible pmake improvement",
      inc == Fraction(6, 10) and rescue_upper == Fraction(4, 10))


# ---------------------------------------------------------------------------
# 20. Monotone certified regret sequence.
# ---------------------------------------------------------------------------

upper_seq = [Fraction(1), Fraction(9, 10), Fraction(4, 5), Fraction(3, 4)]
exec_seq = [Fraction(0), Fraction(1, 4), Fraction(1, 2), Fraction(3, 4)]
regrets = [u - l for u, l in zip(upper_seq, exec_seq)]
check("certified regret decreases under upper-down/lower-up refinement",
      all(regrets[i + 1] <= regrets[i] for i in range(len(regrets) - 1)))
check("zero certified regret marks an executable optimum",
      regrets[-1] == 0)


print(f"\n{CHECKS} CHECKS")
print("ALL CHECKS PASS")
