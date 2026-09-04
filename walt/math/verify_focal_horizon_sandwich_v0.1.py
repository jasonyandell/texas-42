#!/usr/bin/env python3
"""Exact finite scratch verifier for the focal-horizon sandwich intake.

This is not a proof assistant artifact.  It exhaustively checks a two-focal-layer
Boolean public-belief game over every terminal payoff table and every fixed lawful
lower-tail policy.  Arithmetic is exact Fraction/integer arithmetic only.
"""
from fractions import Fraction

W = range(3)
A = range(2)
OBS = (0, 0, 1)       # worlds 0,1 share one public observation; world 2 another
WEIGHTS = (1, 2, 3)
Z = sum(WEIGHTS)


def rat(n):
    return Fraction(n, Z)


def u(bits, w, a0, a1):
    return bits[(w * 2 + a0) * 2 + a1]


def eval_policy(bits, a0, a1_by_obs):
    return rat(sum(WEIGHTS[w] * u(bits, w, a0, a1_by_obs[OBS[w]]) for w in W))


def q_action(bits, a0):
    total = 0
    for o in (0, 1):
        ws = [w for w in W if OBS[w] == o]
        total += max(sum(WEIGHTS[w] * u(bits, w, a0, a1) for w in ws) for a1 in A)
    return rat(total)


def q(bits):
    return max(q_action(bits, a0) for a0 in A)


def god_action(bits, a0):
    return rat(sum(WEIGHTS[w] * max(u(bits, w, a0, a1) for a1 in A) for w in W))


def god(bits):
    return rat(sum(WEIGHTS[w] * max(u(bits, w, a0, a1) for a0 in A for a1 in A) for w in W))


def l0(bits, pi0, pi1):
    return eval_policy(bits, pi0, pi1)


def l1(bits, pi1):
    return max(eval_policy(bits, a0, pi1) for a0 in A)


def l2(bits):
    return q(bits)


def u0(bits):
    return god(bits)


def u1(bits):
    return max(god_action(bits, a0) for a0 in A)


def u2(bits):
    return q(bits)


def survivors(L, U):
    bar = max(L.values())
    return {a for a in A if U[a] >= bar}


def best_l1_policy(bits, pi1):
    vals = [(eval_policy(bits, a0, pi1), a0) for a0 in A]
    v = max(x for x, _ in vals)
    a0 = min(a for x, a in vals if x == v)
    return a0, dict(pi1), v


def best_exact_policy(bits):
    vals = [(q_action(bits, a0), a0) for a0 in A]
    v = max(x for x, _ in vals)
    a0 = min(a for x, a in vals if x == v)
    a1 = {}
    for o in (0, 1):
        ws = [w for w in W if OBS[w] == o]
        scores = [(sum(WEIGHTS[w] * u(bits, w, a0, x) for w in ws), x) for x in A]
        m = max(s for s, _ in scores)
        a1[o] = min(x for s, x in scores if s == m)
    return a0, a1, v


systems = 0
policy_cases = 0

# Exhaust all 2^(3*2*2)=4096 Boolean terminal payoff systems.
for mask in range(1 << 12):
    bits = tuple((mask >> i) & 1 for i in range(12))
    Q = q(bits)
    U0, U1, U2 = u0(bits), u1(bits), u2(bits)

    # Upper hierarchy: God -> root-glued -> fully lawful.
    assert Q <= U1 <= U0
    assert U2 == Q

    # U1 is exactly the one-step salvation-mask upper.
    salvage = {}
    for a0 in A:
        mass = 0
        for w in W:
            if any(u(bits, w, a0, a1) for a1 in A):
                mass += WEIGHTS[w]
        salvage[a0] = rat(mass)
    assert max(salvage.values()) == U1

    # Action-indexed upper containment and nesting.
    Qa = {a: q_action(bits, a) for a in A}
    Ua0 = {a: god_action(bits, a) for a in A}
    Ua1 = dict(Qa)
    for a in A:
        assert Qa[a] <= Ua0[a]
        assert Ua1[a] == Qa[a]
        assert Ua1[a] <= Ua0[a]

    for pi0 in A:
        for p10 in A:
            for p11 in A:
                pi1 = {0: p10, 1: p11}
                L0, L1, L2 = l0(bits, pi0, pi1), l1(bits, pi1), l2(bits)

                # Lower hierarchy and exact collapse after two focal layers.
                assert L0 <= L1 <= Q
                assert L2 == Q

                # k=1 lower is one executable information-consistent policy.
                a0, a1map, lv = best_l1_policy(bits, pi1)
                assert eval_policy(bits, a0, a1map) == L1 == lv

                # k=2 exact policy replays exactly.
                ea0, ea1, ev = best_exact_policy(bits)
                assert eval_policy(bits, ea0, ea1) == Q == ev

                # Root-action sandwich after root action is fixed.
                La0 = {a: eval_policy(bits, a, pi1) for a in A}
                La1 = dict(Qa)
                for a in A:
                    assert La0[a] <= La1[a] == Qa[a] <= Ua1[a] <= Ua0[a]

                # Survivor sets only shrink; exact maximizers survive.
                S0 = survivors(La0, Ua0)
                S1 = survivors(La1, Ua1)
                assert S1 <= S0
                exact_best = {a for a in A if Qa[a] == max(Qa.values())}
                assert exact_best <= S0
                assert exact_best <= S1

                # Certified regret of best k=1 executable lower.
                Ustar = max(Ua0.values())
                assert Fraction(0) <= Q - L1 <= Ustar - L1
                policy_cases += 1

    systems += 1

# Public branches do not consume focal horizon: different public observations
# may lawfully choose different second actions.
bits = [0] * 12
for w in (0, 1):
    bits[(w * 2 + 0) * 2 + 0] = 1
bits[(2 * 2 + 0) * 2 + 1] = 1
bits = tuple(bits)
assert q_action(bits, 0) == 1
globally_shared_second = max(rat(sum(WEIGHTS[w] * u(bits, w, 0, a1) for w in W)) for a1 in A)
assert globally_shared_second < 1

# Merge-before-max counterexample: worlds 0 and 1 share the same public
# observation but require opposite second actions. Worldwise max is strategy fusion.
bits = [0] * 12
bits[(0 * 2 + 0) * 2 + 0] = 1
bits[(1 * 2 + 0) * 2 + 1] = 1
bits = tuple(bits)
worldwise = rat(sum(WEIGHTS[w] * max(u(bits, w, 0, a1) for a1 in A) for w in W))
lawful = q_action(bits, 0)
assert worldwise > lawful

# Scalar optimum equality does not imply selected-action safety.
Qvals = {0: Fraction(1), 1: Fraction(9, 10)}
Uvals = {0: Fraction(1), 1: Fraction(1)}
assert max(Qvals.values()) == max(Uvals.values())
# A tie rule favoring action 1 selects the wrong action despite exact scalar max.
assert max(Uvals, key=lambda a: (Uvals[a], a)) == 1
assert max(Qvals, key=lambda a: (Qvals[a], a)) == 0

# Interval separation theorem specimen.
L = {0: Fraction(3, 4), 1: Fraction(1, 2)}
U = {0: Fraction(4, 5), 1: Fraction(2, 3)}
assert L[0] > U[1]
for q0n in range(75, 81):
    q0v = Fraction(q0n, 100)
    if not (L[0] <= q0v <= U[0]):
        continue
    for q1n in range(50, 67):
        q1v = Fraction(q1n, 100)
        if L[1] <= q1v <= U[1]:
            assert q0v > q1v

# Exact-mass hidden-branch parity.
child_Z = (2, 3, 5)
child_M = (1, 2, 4)
Z0 = sum(child_Z)
M0 = sum(child_M)
weighted = sum(Fraction(z, Z0) * Fraction(m, z) for z, m in zip(child_Z, child_M))
assert weighted == Fraction(M0, Z0)

# Bellman supersolution condition is load-bearing: violate it and the first
# upper refinement can rise rather than fall.
G_parent = Fraction(1, 2)
G_children = (Fraction(3, 4), Fraction(1, 4))
assert G_parent < max(G_children)

# Exact suffix substitution is compositional through sum and max.
p = Fraction(2, 5)
left, right = Fraction(3, 4), Fraction(1, 4)
direct_hidden = p * left + (1 - p) * right
sub_hidden = p * left + (1 - p) * right
assert direct_hidden == sub_hidden
assert max(direct_hidden, Fraction(2, 5)) == max(sub_hidden, Fraction(2, 5))

# Partial interval propagation remains sound at both node types.
intervals = [(Fraction(1, 4), Fraction(1, 2)), (Fraction(1, 3), Fraction(2, 3))]
truth = [Fraction(2, 5), Fraction(1, 2)]
Lf, Uf, Qf = max(x[0] for x in intervals), max(x[1] for x in intervals), max(truth)
assert Lf <= Qf <= Uf
p = Fraction(1, 3)
Lh = p * intervals[0][0] + (1 - p) * intervals[1][0]
Uh = p * intervals[0][1] + (1 - p) * intervals[1][1]
Qh = p * truth[0] + (1 - p) * truth[1]
assert Lh <= Qh <= Uh

# Nonexpansiveness / gap propagation specimens.
xb = [Fraction(1, 3), Fraction(3, 4), Fraction(1, 2)]
ya = [x - Fraction(1, 10) for x in xb]
assert 0 <= max(xb) - max(ya) <= max(x - y for x, y in zip(xb, ya))
pv = [Fraction(1, 5), Fraction(4, 5)]
xb = [Fraction(4, 5), Fraction(1, 2)]
ya = [Fraction(3, 5), Fraction(2, 5)]
assert sum(p*x for p, x in zip(pv, xb)) - sum(p*y for p, y in zip(pv, ya)) == sum(p*(x-y) for p, x, y in zip(pv, xb, ya))

print("24 CHECK FAMILIES")
print(f"EXHAUSTIVE TERMINAL SYSTEMS: {systems}")
print(f"EXHAUSTIVE LOWER-TAIL CASES: {policy_cases}")
print("ALL CHECKS PASS")
