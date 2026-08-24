#!/usr/bin/env python3
"""Exact-rational intake verification for calculated_evidence_v0.1.md.

Stdlib only (Python 3.12), fractions.Fraction throughout, no floats
anywhere. Each check verifies a boxed identity or stated anchor of the
parent by at least one route independent of the parent's own derivation;
the central evidence forms are checked three ways (direct polynomial
integration, substituted integral, closed form). Exit 0 with the receipt
on stdout iff every check passes exactly.

This is an intake verifier (proof receipt), not implementation source.
The Rust implementation must derive from the parent's formulas directly.
"""

from fractions import Fraction as Fr
from math import comb, factorial
import sys

FAIL = 0


def check(name: str, ok: bool) -> None:
    global FAIL
    tag = "PASS" if ok else "FAIL"
    if not ok:
        FAIL += 1
    print(f"{tag}  {name}")


# ---------------------------------------------------------------- poly ops
# polynomials as coefficient lists of Fractions, index = power

def pmul(p, q):
    out = [Fr(0)] * (len(p) + len(q) - 1)
    for i, a in enumerate(p):
        if a:
            for j, b in enumerate(q):
                if b:
                    out[i + j] += a * b
    return out


def ppow(p, n):
    out = [Fr(1)]
    for _ in range(n):
        out = pmul(out, p)
    return out


def pint(p, lo: Fr, hi: Fr) -> Fr:
    """Exact definite integral of a rational-coefficient polynomial."""
    tot = Fr(0)
    for i, a in enumerate(p):
        if a:
            tot += a * (hi ** (i + 1) - lo ** (i + 1)) / (i + 1)
    return tot


# ------------------------------------------------ CE-T1 three-way agreement

def upper_integral_direct(s: int, f: int, c: Fr) -> Fr:
    """(1/(1-c)) * int_c^1 (r/c)^s ((1-r)/(1-c))^f dr, exactly."""
    pr = ppow([Fr(0), 1 / c], s)                       # (r/c)^s
    pf = ppow([1 / (1 - c), -1 / (1 - c)], f)          # ((1-r)/(1-c))^f
    return pint(pmul(pr, pf), c, Fr(1)) / (1 - c)


def upper_integral_subst(s: int, f: int, c: Fr) -> Fr:
    """int_0^1 (1+Rt)^s (1-t)^f dt with R=(1-c)/c, exactly."""
    R = (1 - c) / c
    return pint(pmul(ppow([Fr(1), R], s), ppow([Fr(1), Fr(-1)], f)), Fr(0), Fr(1))


def upper_finite_sum(s: int, f: int, c: Fr) -> Fr:
    """sum_i C(s,i) R^i i!f!/(i+f+1)!  (the parent's boxed finite sum)."""
    R = (1 - c) / c
    return sum(
        comb(s, i) * R ** i * Fr(factorial(i) * factorial(f), factorial(i + f + 1))
        for i in range(s + 1)
    )


GRID_C = [Fr(1, 4), Fr(1, 3), Fr(1, 2), Fr(2, 3), Fr(11, 16), Fr(7, 10)]

ok = all(
    upper_integral_direct(s, f, c)
    == upper_integral_subst(s, f, c)
    == upper_finite_sum(s, f, c)
    for c in GRID_C
    for s in range(13)
    for f in range(13)
)
check("CE-T1 §3.1: direct integral == substituted integral == finite sum "
      "(6 rational c, 0<=s,f<=12)", ok)


def lower_natural_mixture(s: int, f: int, c: Fr) -> Fr:
    """(1/c) * int_0^c (r/c)^s ((1-r)/(1-c))^f dr — the natural lower-test
    uniform mixture, built independently of the parent's CE-T2 definition."""
    pr = ppow([Fr(0), 1 / c], s)
    pf = ppow([1 / (1 - c), -1 / (1 - c)], f)
    return pint(pmul(pr, pf), Fr(0), c) / c


ok = all(
    lower_natural_mixture(s, f, c) == upper_finite_sum(f, s, 1 - c)
    for c in GRID_C
    for s in range(11)
    for f in range(11)
)
check("CE-T2: natural lower mixture == E>_{f,s}(1-c) (the boxed definition "
      "is the real lower-test mixture)", ok)

# ------------------------------------------- CE-T3 pivotal closed form (V1)

def pivotal_integral(a: int, b: int) -> Fr:
    """int_0^1 (1+t)^a (1-t)^b dt, exactly."""
    return pint(pmul(ppow([Fr(1), Fr(1)], a), ppow([Fr(1), Fr(-1)], b)), Fr(0), Fr(1))


def pivotal_closed(a: int, b: int) -> Fr:
    """The parent's §4.1 boxed closed integer form."""
    k = a + b
    return Fr(sum(comb(k + 1, x) for x in range(a + 1)), (k + 1) * comb(k, a))


def pivotal_finite_sum(a: int, b: int) -> Fr:
    """CE-T1 finite sum at c=1/2 (R=1) — an independent third route."""
    return sum(
        comb(a, i) * Fr(factorial(i) * factorial(b), factorial(i + b + 1))
        for i in range(a + 1)
    )


ok = all(
    pivotal_integral(a, b) == pivotal_closed(a, b)
    for a in range(41)
    for b in range(41)
)
check("CE-T3 §4.1: integral == closed integer form (0<=a,b<=40, exact "
      "polynomial integration)", ok)

ok = all(
    pivotal_finite_sum(a, b) == pivotal_closed(a, b)
    for a in range(101)
    for b in range(101)
)
check("CE-T3 §4.1 == CE-T1 at c=1/2 (V1 grid 0<=a,b<=100)", ok)

ANCHORS = {
    (0, 0): Fr(1), (1, 0): Fr(3, 2), (0, 1): Fr(1, 2),
    (2, 0): Fr(7, 3), (1, 1): Fr(2, 3), (2, 1): Fr(11, 12),
    (3, 0): Fr(15, 4), (9, 0): Fr(1023, 10), (10, 0): Fr(2047, 11),
}
check("§4.1 anchors (all nine stated values)",
      all(pivotal_closed(a, b) == v for (a, b), v in ANCHORS.items()))

check("§4.1 unanimous form E+_{a,0} == (2^(a+1)-1)/(a+1) (a<=100)",
      all(pivotal_closed(a, 0) == Fr(2 ** (a + 1) - 1, a + 1) for a in range(101)))

check("§4.1 calculated pivotal requirement at alpha=1/128: "
      "E+_{9,0} < 128 <= E+_{10,0}, and min-h search returns 10",
      pivotal_closed(9, 0) < 128 <= pivotal_closed(10, 0)
      and min(h for h in range(20) if pivotal_closed(h, 0) >= 128) == 10)

# --------------------------------------------- one-step supermartingale (V2)

TWELFTHS = [Fr(i, 12) for i in range(13)]

ok = True
for p in TWELFTHS:
    for c in TWELFTHS:
        if c in (0, 1):
            continue
        for r in TWELFTHS:
            lhs = p * (r / c) + (1 - p) * ((1 - r) / (1 - c))
            rhs = 1 + (p - c) * (r - c) / (c * (1 - c))
            if lhs != rhs:
                ok = False
            if p <= c <= r and lhs > 1:
                ok = False
check("§3 proof: E[L_r(B)] == 1+(p-c)(r-c)/(c(1-c)) on the twelfths grid; "
      "<= 1 whenever p<=c<=r", ok)

# ------------------------------------- CE-T3 raw-world multiplier soundness

ok = True
for q in TWELFTHS:
    for th in TWELFTHS:
        for r in TWELFTHS:
            # E[L_r(Y)] computed from the three-outcome law directly
            lhs = (1 - q) + q * (th * 2 * r + (1 - th) * 2 * (1 - r))
            rhs = 1 - q + q * (2 * th * r + 2 * (1 - th) * (1 - r))
            if lhs != rhs:
                ok = False
            if th <= Fr(1, 2) <= r and lhs > 1:
                ok = False
check("CE-T3: raw-world multiplier expectation identity; <= 1 whenever "
      "theta<=1/2<=r (nonpivotal worlds create no fake evidence)", ok)

check("§4: sign(g) == sign(theta-1/2) when q>0  (g=q*tau, theta=(1+tau)/2)",
      all((q * t > 0) == (((1 + t) / 2) - Fr(1, 2) > 0)
          and (q * t < 0) == (((1 + t) / 2) - Fr(1, 2) < 0)
          for q in TWELFTHS if q > 0
          for t in [Fr(i, 7) for i in range(-7, 8)]))

# ------------------------------------------------- bounded-mean engine (V3)

def bm_ok(L: Fr, U: Fr) -> bool:
    xs = [L + (U - L) * Fr(i, 8) for i in range(9)]
    for c in [L + (U - L) * Fr(j, 6) for j in range(1, 6)]:
        lam_max = 1 / (c - L)
        for lam in [lam_max * Fr(k, 4) for k in range(5)]:
            if any(1 + lam * (x - c) < 0 for x in xs):        # CE-T4 nonneg
                return False
            if any(1 - (1 / (U - c)) * Fr(k, 4) * (x - c) < 0  # CE-T5 nonneg
                   for x in xs for k in range(5)):
                return False
            # one-step expectation under a two-point law with mean <= c
            for x1 in xs:
                for x2 in xs:
                    for w in [Fr(1, 4), Fr(1, 2), Fr(3, 4)]:
                        mu = w * x1 + (1 - w) * x2
                        e = 1 + lam * (mu - c)
                        if mu <= c and e > 1:
                            return False
    return True


check("CE-T4/T5: factor nonnegativity on the declared lambda ranges and "
      "one-step E<=1 under the null, for [L,U]=[-1,1] and [0,1]",
      bm_ok(Fr(-1), Fr(1)) and bm_ok(Fr(0), Fr(1)))

# ------------------------------------------------ §10.1 sign-vs-mean example

check("§10.1 counterexample: P(X>0)=3/4 yet E[X] == -1/32",
      Fr(3, 4) * Fr(1, 8) - Fr(1, 4) * Fr(1, 2) == Fr(-1, 32))

# ------------------------------------------------------- risk ledgers §5/§6

check("§5.2/§6 telescoping ledger: sum_{l=1}^{N} 1/(l(l+1)) == N/(N+1) "
      "(N=10^4), hence total risk < delta and -> delta",
      sum(Fr(1, l * (l + 1)) for l in range(1, 10_001)) == Fr(10_000, 10_001))

check("§5 edge threshold: alpha=delta/(m(m-1)) => T_edge == m(m-1)/delta "
      "(m=2..8, delta on the twelfths grid)",
      all(1 / (d / (m * (m - 1))) == Fr(m * (m - 1), 1) / d
          for m in range(2, 9) for d in TWELFTHS if d > 0))

# ------------------------------------------------------------- H identity §2

ok = all(
    (q - (q * t) ** 2) / (q * t) ** 2 == 1 / (q * t ** 2) - 1
    for q in TWELFTHS if q > 0
    for t in [Fr(i, 7) for i in range(-7, 8)] if t != 0
)
check("§2: H == (q-g^2)/g^2 == 1/(q tau^2) - 1 with g=q*tau", ok)

# --------------------------------------------- §7.1 small-tau expansion of D
# D(tau) = (1+tau)/2 ln(1+tau) + (1-tau)/2 ln(1-tau); exact rational Taylor.

N = 10
ln1p = [Fr(0)] + [Fr((-1) ** (k + 1), k) for k in range(1, N + 1)]   # ln(1+t)
ln1m = [Fr(0)] + [Fr(-1, k) for k in range(1, N + 1)]                # ln(1-t)
half_p = [Fr(1, 2), Fr(1, 2)]        # (1+t)/2
half_m = [Fr(1, 2), Fr(-1, 2)]       # (1-t)/2
D = [Fr(0)] * (N + 2)
for i, a in enumerate(pmul(half_p, ln1p)):
    if i < len(D):
        D[i] += a
for i, b in enumerate(pmul(half_m, ln1m)):
    if i < len(D):
        D[i] += b
expected = {0: Fr(0), 1: Fr(0), 2: Fr(1, 2), 3: Fr(0), 4: Fr(1, 12),
            5: Fr(0), 6: Fr(1, 30)}
check("§7.1: Taylor of D_{1/2}(tau) == tau^2/2 + tau^4/12 + tau^6/30 + ... "
      "(exact rational series through tau^6; odd terms vanish)",
      all(D[k] == v for k, v in expected.items()))

# consistency of the two H-forecast statements
check("§7.1: leading term => n ~ 2 ln T/(q tau^2) == 2(H+1) ln T "
      "(algebraic identity: H+1 == 1/(q tau^2))",
      all(1 / (q * t * t) == (1 / (q * t ** 2) - 1) + 1
          for q in TWELFTHS if q > 0
          for t in [Fr(i, 7) for i in range(1, 8)]))

# ------------------------------------------------------ §8.2 best-case pivots

def h_plus_min(a: int, b: int, T: Fr) -> int:
    h = 0
    while pivotal_closed(a + h, b) < T:
        h += 1
    return h


check("§8.2: h+_min monotone sanity — nonincreasing in a, and "
      "h+_min(0,0;128) == 10 reproduces the §4.1 requirement",
      h_plus_min(0, 0, Fr(128)) == 10
      and all(h_plus_min(a + 1, 0, Fr(128)) <= h_plus_min(a, 0, Fr(128))
              for a in range(12))
      and all(h_plus_min(0, b + 1, Fr(128)) >= h_plus_min(0, b, Fr(128))
              for b in range(6)))

# ---------------------------------------------------------------- verdict

print()
if FAIL:
    print(f"calculated_evidence_v0.1 verification: {FAIL} FAILURE(S)")
    sys.exit(1)
print("calculated_evidence_v0.1 verification: ALL CHECKS PASS (exact rational; no floats)")
