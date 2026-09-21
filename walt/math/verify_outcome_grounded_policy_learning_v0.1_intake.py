#!/usr/bin/env python3
"""Intake-authored scratch-tier checks for outcome_grounded_policy_learning_v0.1.md.

PROVENANCE. The parent's own Section 10 names `verify_learning_bridge.py` and
`verification_results.json` as included verification; NEITHER WAS DELIVERED
with the 2026-09-21 upload (the upload was the one Markdown file). This
program is therefore written AT INTAKE by the intake session — it is not the
parent's verifier and does not reproduce its described 96-game suite. It
mechanically checks the parent's central algebra on an independent family of
tiny, fully enumerated, partially observed games.

Tier: scratch (session evidence, never a receipt — TRUST-01). A green run is
evidence the parent's identities are checkable; it is not a status change,
not a Texas 42 experiment, and not a proof of learning efficiency.

Discipline: Python standard library only, exact Fraction arithmetic
throughout, no floats, no randomness, deterministic enumeration.
Run with:  python3 -I -B verify_outcome_grounded_policy_learning_v0.1_intake.py

Check families (parent section in brackets):

  SCORE     [S2/S3]  exact score identity for the linear relational softmax:
                     d/dtheta_j log pi(a|I) = x_j(I,a) - E_{b~pi} x_j(I,b),
                     via exact dual-number differentiation of the rational
                     multiplicative parameterization r_j = exp(theta_j).
  BRIDGE    [S3]     the outcome-gradient identity
                     grad_j J = E[ Y * sum_t score_j(I_t, a_t) ]
                     with two decentralized teammate seats SHARING theta and a
                     reactive fixed opponent inside c_T; plus baseline
                     invariance and the conditional-zero identity.
  COV       [S3]     a fresh expression F at coefficient zero has derivative
                     g_F = sum_t E[ Cov_{a~pi(.|I_t)}( x_F(I_t,a), Q^pi(I_t,a) ) ],
                     Q^pi computed by forcing then executing the frozen actor.
  TELESCOPE [S4]     the exact finite Bellman telescope
                     J(pi') - J(pi) = sum_t E_{d_t^{pi'}} f_t.
  REMAINDER [S4]     the finite-step bound
                     | J(pi') - J(pi) - S_pi(pi') | <= H(H-1) alpha^2,
                     alpha = max exact TV over every reachable controlled
                     observation, plus |f_t| <= alpha pointwise.
  TVSPEED   [S4]     the derivative-level engine of the L1 coefficient bound:
                     (1/2) sum_a pi_a |delta_a - E_pi delta| <= (1/4) range(delta)
                     (the integrated alpha <= ||dtheta||_1 / 4 is step-checked
                     by hand in the intake companion; its log/exp interpolation
                     is not exact-rational, so only its engine is machined here).
  MF        [S6]     multifidelity: E D_H = E D_L + E(D_H - D_L); the two-batch
                     estimator is unbiased with
                     Var = Var(D_L)/N + Var(D_H - D_L)/M (exact, enumerated over
                     independent batches); allocation-ratio stationarity by
                     exact perturbation on a rational instance.
  DISAGREE  [S6]     |J_sigma(P) - J_proxy(P)| <= Pr_P(first differing field
                     action) under the maximal per-decision coupling, two field
                     decisions deep.
  PIVOT     [S7]     for D in {-1,0,1}: E D = benefit - hazard and
                     Var D = pivotal_mass - (E D)^2, exhaustively on a grid.
  RISK      [S8]     alpha[k,j] = delta/(k(k+1)j(j+1)) has exact partial sums
                     delta (1 - 1/(K+1)) (1 - 1/(J+1)) <= delta, monotone.
"""

from fractions import Fraction

F = Fraction
ZERO, ONE = F(0), F(1)


# ---------------------------------------------------------------- dual numbers
class Dual:
    """First-order jet over exact rationals: value + one directional derivative."""

    __slots__ = ("v", "d")

    def __init__(self, v, d=ZERO):
        self.v = F(v)
        self.d = F(d)

    def __add__(self, o):
        o = dual(o)
        return Dual(self.v + o.v, self.d + o.d)

    __radd__ = __add__

    def __sub__(self, o):
        o = dual(o)
        return Dual(self.v - o.v, self.d - o.d)

    def __rsub__(self, o):
        return dual(o) - self

    def __mul__(self, o):
        o = dual(o)
        return Dual(self.v * o.v, self.v * o.d + self.d * o.v)

    __rmul__ = __mul__

    def __truediv__(self, o):
        o = dual(o)
        return Dual(self.v / o.v, (self.d * o.v - self.v * o.d) / (o.v * o.v))

    def __rtruediv__(self, o):
        return dual(o) / self


def dual(x):
    return x if isinstance(x, Dual) else Dual(x)


# ------------------------------------------------------- deterministic mixing
def mix(*xs):
    h = 7
    for x in xs:
        h = (h * 37 + int(x) + 11) % 1009
    return h


def bit(*xs):
    return mix(*xs) % 2


# ------------------------------------------------------------- the game family
# Hidden world w in {0,1,2}; seat A (controlled) observes oA = [w > 0], acts
# a1 in {0,1}; a fixed opponent observes (w, a1) and plays m in {0,1}; seat B
# (controlled TEAMMATE, sharing the same parameter vector) observes
# I_B = (w mod 2, a1, m) and acts a2; terminal utility Y(w,a1,m,a2) in [0,1].
WORLDS = (0, 1, 2)
ACTS = (0, 1)
PRIORS = ((F(1, 2), F(1, 3), F(1, 6)), (F(1, 5), F(2, 5), F(2, 5)))


def obs_a(w):
    return 0 if w == 0 else 1


def obs_b(w):
    return w % 2


def opp_prob(variant, w, a1, m):
    q = F(1 + mix(variant, w, a1) % 3, 4)  # P(m=0) in {1/4, 2/4, 3/4}
    return q if m == 0 else ONE - q


def utility(variant, w, a1, m, a2):
    y = F(mix(variant, w, a1, m, a2) % 5, 4)  # in {0, 1/4, 1/2, 3/4, 1}
    assert ZERO <= y <= ONE
    return y


def feat_a(j, oa, a):
    return bit(j, oa, a, 3)


def feat_b(j, ob, a1, m, a):
    return bit(j, ob, a1, m, a, 5)


def base_a(oa, a):
    return F(1 + bit(oa, a, 17))  # in {1, 2}


def base_b(ob, a1, m, a):
    return F(1 + bit(ob, a1, m, a, 19))


NDICT = 3  # dictionary features j = 0..2; fresh candidates use j in FRESH
FRESH = (10, 11)


def policy_a(rvec, oa, extra=None):
    """pi(.|I_A) with weights rho = base * prod_j r_j^{x_j}; exact, Dual-safe.

    extra = (j_fresh, r_fresh) optionally extends the dictionary."""
    rho = {}
    for a in ACTS:
        wgt = dual(base_a(oa, a))
        for j, r in enumerate(rvec):
            if feat_a(j, oa, a):
                wgt = wgt * r
        if extra is not None and feat_a(extra[0], oa, a):
            wgt = wgt * extra[1]
        rho[a] = wgt
    tot = rho[0] + rho[1]
    return {a: rho[a] / tot for a in ACTS}


def policy_b(rvec, ib, extra=None):
    ob, a1, m = ib
    rho = {}
    for a in ACTS:
        wgt = dual(base_b(ob, a1, m, a))
        for j, r in enumerate(rvec):
            if feat_b(j, ob, a1, m, a):
                wgt = wgt * r
        if extra is not None and feat_b(extra[0], ob, a1, m, a):
            wgt = wgt * extra[1]
        rho[a] = wgt
    tot = rho[0] + rho[1]
    return {a: rho[a] / tot for a in ACTS}


def trajectories(game, rvec, extra=None):
    """Yield (prob, w, a1, m, a2, y, IA, IB) with Dual-safe probabilities."""
    prior, opp_v, y_v = game
    for w in WORLDS:
        ia = obs_a(w)
        pa = policy_a(rvec, ia, extra)
        for a1 in ACTS:
            for m in ACTS:
                pm = opp_prob(opp_v, w, a1, m)
                ib = (obs_b(w), a1, m)
                pb = policy_b(rvec, ib, extra)
                for a2 in ACTS:
                    p = dual(prior[w]) * pa[a1] * pm * pb[a2]
                    yield p, w, a1, m, a2, utility(y_v, w, a1, m, a2), ia, ib


def value(game, rvec, extra=None):
    j = dual(0)
    for p, *_rest in trajectories(game, rvec, extra):
        y = _rest[4]
        j = j + p * y
    return j


def frac_policy_a(rvec, oa):
    return {a: p.v for a, p in policy_a([dual(r) for r in rvec], oa).items()}


def frac_policy_b(rvec, ib):
    return {a: p.v for a, p in policy_b([dual(r) for r in rvec], ib).items()}


def score_a(rvec, j, oa, a):
    pi = frac_policy_a(rvec, oa)
    e = sum(pi[b] * feat_a(j, oa, b) for b in ACTS)
    return feat_a(j, oa, a) - e


def score_b(rvec, j, ib, a):
    pi = frac_policy_b(rvec, ib)
    e = sum(pi[b] * feat_b(j, ib[0], ib[1], ib[2], b) for b in ACTS)
    return feat_b(j, ib[0], ib[1], ib[2], a) - e


GAMES = [
    (prior, opp_v, y_v)
    for prior in PRIORS
    for opp_v in (0, 1)
    for y_v in (0, 1)
]
RVECS = [(F(2), F(1, 3), F(5, 7)), (F(1), F(3), F(1, 2))]
RPAIRS = [
    ((F(2), F(1, 3), F(5, 7)), (F(3, 2), F(1, 2), F(5, 7))),
    ((F(1), F(1), F(1)), (F(2), F(1, 2), F(1))),
    ((F(1), F(3), F(1, 2)), (F(1), F(3), F(1, 2))),  # identical: alpha = 0
]


def duals_for(rvec, j):
    """theta_j-derivative direction: d r_j / d theta_j = r_j."""
    return [Dual(r, r if k == j else ZERO) for k, r in enumerate(rvec)]


# ---------------------------------------------------------------- CHECK: SCORE
def check_score():
    n = 0
    for rvec in RVECS:
        for j in range(NDICT):
            dv = duals_for(rvec, j)
            for oa in (0, 1):
                pi = policy_a(dv, oa)
                for a in ACTS:
                    assert pi[a].d / pi[a].v == score_a(rvec, j, oa, a)
                    n += 1
            for ob in (0, 1):
                for a1 in ACTS:
                    for m in ACTS:
                        pi = policy_b(dv, (ob, a1, m))
                        for a in ACTS:
                            assert pi[a].d / pi[a].v == score_b(
                                rvec, j, (ob, a1, m), a
                            )
                            n += 1
    return n


# --------------------------------------------------------------- CHECK: BRIDGE
def check_bridge():
    n = 0
    for game in GAMES:
        for rvec in RVECS:
            for j in range(NDICT):
                grad = value(game, duals_for(rvec, j)).d
                plain, with_base, cond_zero = ZERO, ZERO, ZERO
                for p, w, a1, m, a2, y, ia, ib in trajectories(
                    game, [dual(r) for r in rvec]
                ):
                    sa = score_a(rvec, j, ia, a1)
                    sb = score_b(rvec, j, ib, a2)
                    plain += p.v * y * (sa + sb)
                    # baselines measurable before each action (functions of I_t)
                    ca = F(1 + ia, 3)
                    cb = F(1 + ib[0] + ib[1], 5)
                    with_base += p.v * ((y - ca) * sa + (y - cb) * sb)
                    cond_zero += p.v * (ca * sa + cb * sb)
                assert grad == plain, "outcome-gradient identity"
                assert grad == with_base, "baseline invariance"
                assert cond_zero == ZERO, "conditional-zero identity"
                n += 1
    return n


# ------------------------------------------------------------------ CHECK: COV
def q_and_reach(game, rvec):
    """Exact reach probabilities and forced-action continuation values Q^pi
    at every information set of both controlled seats."""
    prior, opp_v, y_v = game
    # seat A
    qa, reach_a = {}, {}
    for oa in (0, 1):
        ws = [w for w in WORLDS if obs_a(w) == oa and prior[w] > 0]
        pr = sum(prior[w] for w in ws)
        if pr == 0:
            continue
        reach_a[oa] = pr
        for a1 in ACTS:
            v = ZERO
            for w in ws:
                pw = prior[w] / pr
                for m in ACTS:
                    pm = opp_prob(opp_v, w, a1, m)
                    pb = frac_policy_b(rvec, (obs_b(w), a1, m))
                    for a2 in ACTS:
                        v += pw * pm * pb[a2] * utility(y_v, w, a1, m, a2)
            qa[(oa, a1)] = v
    # seat B
    qb, reach_b = {}, {}
    for w in WORLDS:
        pa = frac_policy_a(rvec, obs_a(w))
        for a1 in ACTS:
            for m in ACTS:
                p = prior[w] * pa[a1] * opp_prob(opp_v, w, a1, m)
                if p == 0:
                    continue
                ib = (obs_b(w), a1, m)
                reach_b[ib] = reach_b.get(ib, ZERO) + p
                for a2 in ACTS:
                    key = (ib, a2)
                    qb[key] = qb.get(key, ZERO) + p * utility(y_v, w, a1, m, a2)
    for (ib, a2), tot in qb.items():
        qb[(ib, a2)] = tot / reach_b[ib]
    return reach_a, qa, reach_b, qb


def check_cov():
    n = 0
    for game in GAMES:
        for rvec in RVECS:
            reach_a, qa, reach_b, qb = q_and_reach(game, rvec)
            for jf in FRESH:
                # left side: derivative of J with F adjoined at weight zero
                extra = (jf, Dual(ONE, ONE))
                g = value(game, [dual(r) for r in rvec], extra).d
                # right side: sum of reach-weighted covariances Cov(x_F, Q^pi)
                rhs = ZERO
                for oa, pr in reach_a.items():
                    pi = frac_policy_a(rvec, oa)
                    ex = sum(pi[a] * feat_a(jf, oa, a) for a in ACTS)
                    eq = sum(pi[a] * qa[(oa, a)] for a in ACTS)
                    exq = sum(
                        pi[a] * feat_a(jf, oa, a) * qa[(oa, a)] for a in ACTS
                    )
                    rhs += pr * (exq - ex * eq)
                for ib, pr in reach_b.items():
                    pi = frac_policy_b(rvec, ib)
                    xf = {a: feat_b(jf, ib[0], ib[1], ib[2], a) for a in ACTS}
                    ex = sum(pi[a] * xf[a] for a in ACTS)
                    eq = sum(pi[a] * qb[(ib, a)] for a in ACTS)
                    exq = sum(pi[a] * xf[a] * qb[(ib, a)] for a in ACTS)
                    rhs += pr * (exq - ex * eq)
                assert g == rhs, "fresh-expression covariance identity"
                n += 1
    return n


# --------------------------------------------- CHECK: TELESCOPE and REMAINDER
def tv(p, q):
    return sum(abs(p[a] - q[a]) for a in ACTS) / 2


def check_telescope_remainder():
    n_tel = n_rem = 0
    for game in GAMES:
        prior, opp_v, y_v = game
        for r_old, r_new in RPAIRS:
            # state-based Q_t^pi (old policy executes after the action)
            def q1(w, a1):
                v = ZERO
                for m in ACTS:
                    pm = opp_prob(opp_v, w, a1, m)
                    pb = frac_policy_b(r_old, (obs_b(w), a1, m))
                    for a2 in ACTS:
                        v += pm * pb[a2] * utility(y_v, w, a1, m, a2)
                return v

            def f1(w):
                po = frac_policy_a(r_old, obs_a(w))
                pn = frac_policy_a(r_new, obs_a(w))
                return sum((pn[a] - po[a]) * q1(w, a) for a in ACTS)

            def f2(w, a1, m):
                ib = (obs_b(w), a1, m)
                po = frac_policy_b(r_old, ib)
                pn = frac_policy_b(r_new, ib)
                return sum(
                    (pn[a] - po[a]) * utility(y_v, w, a1, m, a) for a in ACTS
                )

            def d2(rvec):  # analysis-state law at the second controlled turn
                out = {}
                for w in WORLDS:
                    pa = frac_policy_a(rvec, obs_a(w))
                    for a1 in ACTS:
                        for m in ACTS:
                            out[(w, a1, m)] = (
                                prior[w] * pa[a1] * opp_prob(opp_v, w, a1, m)
                            )
                return out

            j_old = value(game, [dual(r) for r in r_old]).v
            j_new = value(game, [dual(r) for r in r_new]).v
            e_f1 = sum(prior[w] * f1(w) for w in WORLDS)
            tele = e_f1 + sum(p * f2(*x) for x, p in d2(r_new).items())
            assert j_new - j_old == tele, "finite Bellman telescope"
            n_tel += 1

            s = e_f1 + sum(p * f2(*x) for x, p in d2(r_old).items())
            # alpha over EVERY reachable controlled observation, both policies
            alpha = ZERO
            for oa in set(obs_a(w) for w in WORLDS if prior[w] > 0):
                alpha = max(
                    alpha, tv(frac_policy_a(r_old, oa), frac_policy_a(r_new, oa))
                )
            reachable_ib = set()
            for rv in (r_old, r_new):
                for x, p in d2(rv).items():
                    if p > 0:
                        reachable_ib.add((obs_b(x[0]), x[1], x[2]))
            for ib in reachable_ib:
                alpha = max(
                    alpha, tv(frac_policy_b(r_old, ib), frac_policy_b(r_new, ib))
                )
            # |f_t| <= alpha pointwise (Q in [0,1]), then the H(H-1) alpha^2 bound
            for w in WORLDS:
                assert abs(f1(w)) <= alpha
            for x in d2(r_old):
                assert abs(f2(*x)) <= alpha
            h = 2
            assert abs(j_new - j_old - s) <= h * (h - 1) * alpha * alpha
            n_rem += 1
    return n_tel, n_rem


# -------------------------------------------------------------- CHECK: TVSPEED
def compositions(total, parts):
    if parts == 1:
        yield (total,)
        return
    for head in range(total + 1):
        for rest in compositions(total - head, parts - 1):
            yield (head,) + rest


def check_tvspeed():
    n = 0
    grid = [F(k, 2) for k in range(-2, 3)]
    for comp in compositions(6, 3):
        pi = [F(c, 6) for c in comp]
        for d0 in grid:
            for d1 in grid:
                for d2_ in grid:
                    delta = (d0, d1, d2_)
                    mu = sum(p * d for p, d in zip(pi, delta))
                    mad = sum(p * abs(d - mu) for p, d in zip(pi, delta))
                    rng = max(delta) - min(delta)
                    assert mad <= rng / 2, "mean absolute deviation <= range/2"
                    assert mad / 2 <= rng / 4, "TV speed <= range(delta)/4"
                    n += 1
    return n


# ------------------------------------------------------------------- CHECK: MF
def check_mf():
    n = 0
    pu = (F(1, 2), F(1, 3), F(1, 6))
    tables = [
        ((F(1, 2), F(-1, 4), F(1)), (F(3, 4), F(-1, 2), F(3, 4))),  # correlated
        ((F(1), F(-1), F(0)), (F(-1, 2), F(1, 2), F(1, 4))),  # anti-correlated
    ]
    for d_l, d_h in tables:
        e_l = sum(p * x for p, x in zip(pu, d_l))
        e_h = sum(p * x for p, x in zip(pu, d_h))
        d_c = tuple(h - l for h, l in zip(d_h, d_l))
        e_c = sum(p * x for p, x in zip(pu, d_c))
        assert e_h == e_l + e_c, "expectation decomposition"
        var_l = sum(p * (x - e_l) ** 2 for p, x in zip(pu, d_l))
        var_c = sum(p * (x - e_c) ** 2 for p, x in zip(pu, d_c))
        # enumerate independent batches, N = M = 2
        mean = ZERO
        second = ZERO
        for i1 in range(3):
            for i2 in range(3):
                for j1 in range(3):
                    for j2 in range(3):
                        p = pu[i1] * pu[i2] * pu[j1] * pu[j2]
                        est = (d_l[i1] + d_l[i2]) / 2 + (d_c[j1] + d_c[j2]) / 2
                        mean += p * est
                        second += p * est * est
        assert mean == e_h, "two-batch estimator unbiased"
        assert second - mean * mean == var_l / 2 + var_c / 2, "variance split"
        n += 1
    # allocation stationarity: Var_L=4, Var_C=1, c_L=c_C=1, budget 8
    # => N/M = sqrt(4*1/(1*1)) = 2, N* = 16/3.  Exact perturbation check.
    def g(nn):
        return F(4) / nn + F(1) / (8 - nn)

    star = F(16, 3)
    for eps in (F(1, 3), F(2, 3), F(1), F(-1, 3), F(-2, 3), F(-1)):
        assert g(star) < g(star + eps), "allocation ratio is the minimizer"
        n += 1
    return n


# ------------------------------------------------------------- CHECK: DISAGREE
def check_disagree():
    # fixed policy P at the single controlled seat; two field decisions.
    p_fixed = {0: (F(1, 3), F(2, 3)), 1: (F(3, 5), F(2, 5))}

    def field1(variant, w, a1, m1):
        q = F(1 + mix(variant, w, a1, 23) % 3, 4)
        return q if m1 == 0 else ONE - q

    def field2(variant, w, a1, m1, m2):
        q = F(1 + mix(variant, w, a1, m1, 29) % 3, 4)
        return q if m2 == 0 else ONE - q

    def y2(w, a1, m1, m2):
        return F(mix(w, a1, m1, m2, 31) % 5, 4)

    n = 0
    prior = (F(1, 2), F(1, 3), F(1, 6))
    for sig, prox in ((0, 1), (0, 2), (1, 2)):

        def j_under(variant):
            tot = ZERO
            for w in WORLDS:
                pa = p_fixed[obs_a(w)]
                for a1 in ACTS:
                    for m1 in ACTS:
                        for m2 in ACTS:
                            tot += (
                                prior[w]
                                * pa[a1]
                                * field1(variant, w, a1, m1)
                                * field2(variant, w, a1, m1, m2)
                                * y2(w, a1, m1, m2)
                            )
            return tot

        # maximal per-decision coupling: P(first differing field action)
        p_diff = ZERO
        for w in WORLDS:
            pa = p_fixed[obs_a(w)]
            for a1 in ACTS:
                s1 = {m: field1(sig, w, a1, m) for m in ACTS}
                r1 = {m: field1(prox, w, a1, m) for m in ACTS}
                tv1 = sum(abs(s1[m] - r1[m]) for m in ACTS) / 2
                inner = tv1
                for m1 in ACTS:
                    agree = min(s1[m1], r1[m1])
                    s2 = {m: field2(sig, w, a1, m1, m) for m in ACTS}
                    r2 = {m: field2(prox, w, a1, m1, m) for m in ACTS}
                    tv2 = sum(abs(s2[m] - r2[m]) for m in ACTS) / 2
                    inner += agree * tv2
                p_diff += prior[w] * pa[a1] * inner
        gap = abs(j_under(sig) - j_under(prox))
        assert gap <= p_diff, "first-disagreement coupling bound"
        assert p_diff < ONE, "bound is nontrivial on this instance"
        n += 1
    return n


# ---------------------------------------------------------------- CHECK: PIVOT
def check_pivot():
    n = 0
    for np_, nz in [(a, b) for a in range(13) for b in range(13 - a)]:
        nm = 12 - np_ - nz
        p_plus, p_zero, p_minus = F(np_, 12), F(nz, 12), F(nm, 12)
        ed = p_plus - p_minus
        var = (
            p_plus * (ONE - ed) ** 2
            + p_zero * ed ** 2
            + p_minus * (-ONE - ed) ** 2
        )
        assert ed == p_plus - p_minus, "E D = benefit - hazard"
        assert var == (p_plus + p_minus) - ed * ed, "Var = pivotal mass - (E D)^2"
        n += 1
    return n


# ----------------------------------------------------------------- CHECK: RISK
def check_risk():
    n = 0
    delta = F(1, 20)
    prev = ZERO
    for kk in range(1, 7):
        for jj in range(1, 7):
            s = sum(
                delta / (k * (k + 1) * j * (j + 1))
                for k in range(1, kk + 1)
                for j in range(1, jj + 1)
            )
            closed = delta * (ONE - F(1, kk + 1)) * (ONE - F(1, jj + 1))
            assert s == closed, "closed-form partial sum"
            assert s <= delta, "risk budget never exceeded"
            n += 1
        row_full = delta * (ONE - F(1, kk + 1)) * (ONE - F(1, 7))
        assert row_full >= prev
        prev = row_full
    return n


def main():
    results = []
    results.append(("SCORE", check_score()))
    results.append(("BRIDGE", check_bridge()))
    results.append(("COV", check_cov()))
    n_tel, n_rem = check_telescope_remainder()
    results.append(("TELESCOPE", n_tel))
    results.append(("REMAINDER", n_rem))
    results.append(("TVSPEED", check_tvspeed()))
    results.append(("MF", check_mf()))
    results.append(("DISAGREE", check_disagree()))
    results.append(("PIVOT", check_pivot()))
    results.append(("RISK", check_risk()))
    for name, count in results:
        print(f"{name:10s} {count} exact instances PASS")
    print(f"{len(results)} CHECK FAMILIES / ALL CHECKS PASS")


if __name__ == "__main__":
    main()
