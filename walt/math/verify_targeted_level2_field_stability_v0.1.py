#!/usr/bin/env python3
"""Exact verifier for targeted_level2_field_stability_v0.1.md (stdlib only).

The parent's theorems L2-T1..L2-T5 are structural claims about coupled
executions of two deterministic field models. The mechanical route is exact
finite-game model checking: we build small explicit games (worlds, alternating
focal/field moves, Boolean payoff), enumerate EVERY information-consistent
focal policy and EVERY world, compute all quantities as exact Fractions, and
check every theorem instance universally. No sampling, no floats, no
randomness.

Checks 1-11 are universal (every instance in an enumerated game family must
satisfy the theorem). Checks 12-19 are existence checks: the eight L2-E0
fixture phenomena from parent Sec. 16 must each occur somewhere in the family
(or in a dedicated construction for the best-response two-cycle).

Run from walt/math/:  python3 verify_targeted_level2_field_stability_v0.1.py
Exit 0 iff all checks pass.
"""

from fractions import Fraction
from itertools import product

FAILURES = []
CHECKS_RUN = 0


def check(num, name, ok, detail=""):
    global CHECKS_RUN
    CHECKS_RUN += 1
    status = "PASS" if ok else "FAIL"
    print(f"[{num:>2}] {status} {name}" + (f" — {detail}" if detail else ""))
    if not ok:
        FAILURES.append((num, name))


# ---------------------------------------------------------------------------
# Finite-game model
#
# A game: worlds W (uniform outer belief), a fixed sequence of movers
# ('F' focal / 'N' non-focal), binary actions {0,1}, Boolean payoff(w, hist).
# A field sigma is a function (w, hist) -> action: the non-focal seat's
# information state contains its private knowledge of w plus the public
# history, matching parent Sec. 2/3. A focal policy rho is a function of the
# public history alone (information consistency: rho never reads w).
# ---------------------------------------------------------------------------

class Game:
    def __init__(self, worlds, movers, payoff, s0, s1):
        self.worlds = worlds
        self.movers = movers
        self.payoff = payoff
        self.s0 = s0
        self.s1 = s1
        # Focal decision points: (step, history) pairs, over-complete
        # (all histories of the right length; unreachable ones are harmless).
        self.focal_keys = []
        for step, m in enumerate(movers):
            if m == 'F':
                for h in product((0, 1), repeat=step):
                    self.focal_keys.append((step, h))

    def all_policies(self):
        """Every information-consistent focal policy, as a dict."""
        keys = self.focal_keys
        for bits in product((0, 1), repeat=len(keys)):
            yield dict(zip(keys, bits))

    def run(self, rho, sigma, w):
        """Terminal payoff with focal rho and all non-focal seats on sigma."""
        h = ()
        for step, m in enumerate(self.movers):
            a = rho[(step, h)] if m == 'F' else sigma(w, h)
            h = h + (a,)
        return self.payoff(w, h)

    def finish(self, rho, sigma, w, h, step):
        for s in range(step, len(self.movers)):
            a = rho[(s, h)] if self.movers[s] == 'F' else sigma(w, h)
            h = h + (a,)
        return self.payoff(w, h)

    def coupled(self, rho, w):
        """Coupled execution (parent Sec. 3.1). Returns (D, u0, u1)."""
        h = ()
        for step, m in enumerate(self.movers):
            if m == 'F':
                h = h + (rho[(step, h)],)
            else:
                a0, a1 = self.s0(w, h), self.s1(w, h)
                if a0 != a1:
                    u0 = self.finish(rho, self.s0, w, h + (a0,), step + 1)
                    u1 = self.finish(rho, self.s1, w, h + (a1,), step + 1)
                    return 1, u0, u1
                h = h + (a0,)
        u = self.payoff(w, h)
        return 0, u, u

    def clairvoyant_reach(self, w, root_action=None):
        """P^PI_a(w) (parent Sec. 7.3): can ANY focal action sequence, chosen
        with full knowledge of w, reach a field-disagreement state? Before the
        first split both fields move identically, so a single expansion under
        s0 with free focal choices explores every pre-split history."""
        frontier_hit = [False]

        def expand(h, step):
            if frontier_hit[0] or step == len(self.movers):
                return
            m = self.movers[step]
            if m == 'F':
                if step == 0 and root_action is not None:
                    choices = (root_action,)
                else:
                    choices = (0, 1)
                for a in choices:
                    expand(h + (a,), step + 1)
            else:
                a0, a1 = self.s0(w, h), self.s1(w, h)
                if a0 != a1:
                    frontier_hit[0] = True
                    return
                expand(h + (a0,), step + 1)

        expand((), 0)
        return 1 if frontier_hit[0] else 0


def analyze(game):
    """Exact per-policy and per-root-action quantities for one game."""
    W = game.worlds
    n = Fraction(len(W))
    pols = list(game.all_policies())
    per_policy = []
    for rho in pols:
        d = Fraction(0)
        v0 = Fraction(0)
        v1 = Fraction(0)
        pointwise_ok = True
        for w in W:
            D, u0, u1 = game.coupled(rho, w)
            # Check 1: coupled == direct runs under each field.
            if u0 != game.run(rho, game.s0, w) or u1 != game.run(rho, game.s1, w):
                pointwise_ok = False
            # L2-T1 pointwise: |u1-u0| <= D.
            if abs(u1 - u0) > D:
                pointwise_ok = False
            d += Fraction(D)
            v0 += Fraction(u0)
            v1 += Fraction(u1)
        per_policy.append({
            "rho": rho, "d": d / n, "V0": v0 / n, "V1": v1 / n,
            "c": (v1 - v0) / n, "ok": pointwise_ok,
        })
    return per_policy


def root_quantities(game, per_policy):
    """Q_a^(0), Q_a^(1), exact R_a per root action (movers must start 'F')."""
    assert game.movers[0] == 'F'
    out = {}
    for a in (0, 1):
        rows = [p for p in per_policy if p["rho"][(0, ())] == a]
        out[a] = {
            "Q0": max(p["V0"] for p in rows),
            "Q1": max(p["V1"] for p in rows),
            "R": max(p["d"] for p in rows),
            "rows": rows,
        }
    return out


# ---------------------------------------------------------------------------
# Game family: worlds {0,1}, movers F-N-F (root action = focal step 0).
# All 16 field functions on the single field step x all 16 = 256 ordered
# pairs, crossed with six structurally diverse payoff functions.
# ---------------------------------------------------------------------------

def make_sigma(bits):
    # bits: action for (w, a0) in [(0,0),(0,1),(1,0),(1,1)]
    table = {(0, (0,)): bits[0], (0, (1,)): bits[1],
             (1, (0,)): bits[2], (1, (1,)): bits[3]}
    return lambda w, h: table[(w, h)]


PAYOFFS_FNF = [
    ("parity", lambda w, h: (w + h[0] + h[1] + h[2]) % 2),
    ("w_xor_last", lambda w, h: (w + h[2]) % 2),
    ("field_and_last", lambda w, h: h[1] & h[2]),
    ("majority", lambda w, h: 1 if h[0] + h[1] + h[2] >= 2 else 0),
    ("ignore_field", lambda w, h: (w + h[0] * h[2]) % 2),
    ("root_only", lambda w, h: h[0]),
]

SIGMAS = [make_sigma(bits) for bits in product((0, 1), repeat=4)]

# Second family: movers N-F-N (two field steps at different depths, exercising
# the L2-T1 induction through a non-trivial first-split fork position).
# Fields drawn from a small structured set over the 5 (w,h) field states.

def make_sigma_nfn(k, mode):
    def s(w, h):
        base = (w + sum(h) + k) % 2
        if mode == 1:
            base = (base + len(h)) % 2
        return base
    return s


SIGMAS_NFN = [make_sigma_nfn(k, m) for k in (0, 1) for m in (0, 1)]
PAYOFFS_NFN = [
    ("parity", lambda w, h: (w + h[0] + h[1] + h[2]) % 2),
    ("ends", lambda w, h: h[0] ^ h[2]),
    ("focal_only", lambda w, h: h[1]),
]


def build_family():
    games = []
    for (pname, pay), s0, s1 in product(PAYOFFS_FNF, SIGMAS, SIGMAS):
        games.append(("FNF:" + pname, Game([0, 1], ['F', 'N', 'F'], pay, s0, s1)))
    for (pname, pay), s0, s1 in product(PAYOFFS_NFN, SIGMAS_NFN, SIGMAS_NFN):
        games.append(("NFN:" + pname, Game([0, 1], ['N', 'F', 'N'], pay, s0, s1)))
    return games


def main():
    games = build_family()
    n_fnf = sum(1 for name, _ in games if name.startswith("FNF"))
    n_nfn = len(games) - n_fnf

    # Universal accumulators.
    ok_model = ok_t1 = ok_fp = ok_pair = ok_margin = True
    ok_t2 = ok_t3 = ok_t4 = ok_t4_loose = ok_e2 = True
    inst_t1 = inst_fp = inst_pair = inst_t2 = inst_t3 = inst_t4 = inst_e2 = 0

    # Existence flags for L2-E0 items 1-7.
    ex_never_disagree = False       # item 1: fields never disagree, R == 0
    ex_split_no_payoff = False      # item 2: some d>0 yet every c == 0
    ex_pos_corr = False             # item 3
    ex_neg_corr = False             # item 4
    ex_margin_beats = False         # item 5 (with nonzero exposure)
    ex_margin_fails_flip = False    # item 6: margin <= sum and decision flips
    ex_nontrivial_exclusion = False  # item 7

    SLACK = Fraction(1, 8)

    for name, g in games:
        pp = analyze(g)
        if not all(p["ok"] for p in pp):
            ok_model = False
            ok_t1 = False
        inst_t1 += len(pp) * len(g.worlds)

        for p in pp:
            # Sec 3.2: |c_rho| <= d_rho.
            if abs(p["c"]) > p["d"]:
                ok_fp = False
            inst_fp += 1
        for pa in pp:
            for pb in pp:
                lam = pa["c"] - pb["c"]
                if abs(lam) > pa["d"] + pb["d"]:
                    ok_pair = False
                g0 = pa["V0"] - pb["V0"]
                g1 = pa["V1"] - pb["V1"]
                if g0 > pa["d"] + pb["d"] and not (g1 > 0):
                    ok_margin = False
                inst_pair += 1

        any_d = any(p["d"] > 0 for p in pp)
        if not any_d:
            all_states_agree = all(
                g.s0(w, h) == g.s1(w, h)
                for step, m in enumerate(g.movers) if m == 'N'
                for w in g.worlds for h in product((0, 1), repeat=step))
            if all_states_agree:
                ex_never_disagree = True
        if any_d and all(p["c"] == 0 for p in pp):
            ex_split_no_payoff = True
        if any(p["c"] > 0 for p in pp):
            ex_pos_corr = True
        if any(p["c"] < 0 for p in pp):
            ex_neg_corr = True

        if g.movers[0] != 'F':
            continue

        rq = root_quantities(g, pp)
        for a in (0, 1):
            if abs(rq[a]["Q1"] - rq[a]["Q0"]) > rq[a]["R"]:
                ok_t2 = False
            inst_t2 += 1
            # E2 rung: R_a <= Pr(clairvoyant split reach | root a).
            mass = Fraction(sum(g.clairvoyant_reach(w, root_action=a)
                                for w in g.worlds), len(g.worlds))
            if rq[a]["R"] > mass:
                ok_e2 = False
            inst_e2 += 1

        for a in (0, 1):
            b = 1 - a
            margin = rq[a]["Q0"] - rq[b]["Q0"]
            exposure_sum = rq[a]["R"] + rq[b]["R"]
            if margin > exposure_sum:
                # L2-T3: a must be strictly optimal under field 1.
                if not (rq[a]["Q1"] > rq[b]["Q1"]):
                    ok_t3 = False
                if exposure_sum > 0:
                    ex_margin_beats = True
            inst_t3 += 1
            if (rq[a]["Q0"] > rq[b]["Q0"] and margin <= exposure_sum
                    and rq[b]["Q1"] > rq[a]["Q1"]):
                ex_margin_fails_flip = True

        # L2-T4 admissible set, tight bounds (L=U=Q0, R^U=R) and loosened.
        for slack in (Fraction(0), SLACK):
            L1 = {a: rq[a]["Q0"] - slack - (rq[a]["R"] + slack) for a in (0, 1)}
            U1 = {a: rq[a]["Q0"] + slack + (rq[a]["R"] + slack) for a in (0, 1)}
            bar = max(L1.values())
            admissible = {a for a in (0, 1) if U1[a] >= bar}
            best_q1 = max(rq[a]["Q1"] for a in (0, 1))
            optimal_under_1 = {a for a in (0, 1) if rq[a]["Q1"] == best_q1}
            if not optimal_under_1 <= admissible:
                if slack == 0:
                    ok_t4 = False
                else:
                    ok_t4_loose = False
            if slack == 0:
                inst_t4 += 1
                if len(admissible) < 2:
                    ex_nontrivial_exclusion = True

    check(1, "model self-consistency: coupled == direct runs (all games)",
          ok_model, f"{n_fnf} FNF + {n_nfn} NFN games")
    check(2, "L2-T1 first-disagreement localization: |u1-u0| <= D pointwise",
          ok_t1, f"{inst_t1} (rho, omega) instances")
    check(3, "Sec 3.2 fixed-policy bound: |c_rho| <= d_rho exactly",
          ok_fp, f"{inst_fp} policies")
    check(4, "Sec 3.3 pair bound: |Lambda_ab| <= d_a + d_b",
          ok_pair, f"{inst_pair} ordered pairs")
    check(5, "Sec 3.3 margin transfer: g0 > d_a+d_b implies g1 > 0",
          ok_margin, "same pair sweep")
    check(6, "L2-T2 root Lipschitz: |Q1_a - Q0_a| <= R_a (exact sup over Pi_a)",
          ok_t2, f"{inst_t2} root actions")
    check(7, "L2-T3 winner stability: margin > R_a+R_b implies strict field-1 win",
          ok_t3, f"{inst_t3} margin instances")
    check(8, "L2-T4 screening sound with tight bounds", ok_t4,
          f"{inst_t4} admissible-set constructions")
    check(9, "L2-T4 screening sound with loosened bounds (slack 1/8)",
          ok_t4_loose, "looseness costs pruning, never soundness")
    check(10, "Rung E2: exact R_a <= clairvoyant split-reach mass",
          ok_e2, f"{inst_e2} bounds")

    # Sec 9.2: Z = Y1 - Y0 ranges over [-2,2]; X = Z/2 in [-1,1].
    ok_z = all(
        -2 <= (y1 - y0) <= 2 and abs(Fraction(y1 - y0, 2)) <= 1
        for y1 in (-1, 0, 1) for y0 in (-1, 0, 1))
    check(11, "Sec 9.2 range: Z = Y1-Y0 in [-2,2], X = Z/2 in [-1,1]",
          ok_z, "9 enumerated cases")

    # L2-T5: deterministic self-maps on finite sets are eventually periodic.
    def eventually_periodic(f, x0, size):
        seen = {}
        x = x0
        for t in range(size + 1):
            if x in seen:
                return True, t - seen[x]
            seen[x] = t
            x = f(x)
        return False, 0

    maps = [
        ("identity", lambda x: x, 0, 5),
        ("two-cycle", lambda x: {0: 1, 1: 0, 2: 0}[x], 2, 3),
        ("tail-then-3-cycle", lambda x: {0: 1, 1: 2, 2: 3, 3: 4, 4: 2}[x], 0, 5),
    ]
    ok_t5 = all(eventually_periodic(f, x0, size)[0] for _, f, x0, size in maps)
    check(12, "L2-T5 eventual periodicity of deterministic finite towers",
          ok_t5, "identity, 2-cycle, tail+3-cycle operators")

    # L2-E0 existence items 1-7 (mined from the enumerated family).
    check(13, "L2-E0 item 1 exists: fields never disagree, R_a = 0",
          ex_never_disagree)
    check(14, "L2-E0 item 2 exists: fields split but payoff never changes",
          ex_split_no_payoff)
    check(15, "L2-E0 items 3+4 exist: positive and negative corrections",
          ex_pos_corr and ex_neg_corr)
    check(16, "L2-E0 item 5 exists: margin beats nonzero exposure sum",
          ex_margin_beats)
    check(17, "L2-E0 item 6 exists: margin <= exposure sum and decision flips",
          ex_margin_fails_flip)
    check(18, "L2-E0 item 7 exists: L2-T4 excludes a provably nonoptimal rival",
          ex_nontrivial_exclusion)

    # L2-E0 item 8: a genuine best-response two-orbit. Matching pennies with
    # deterministic argmax (prefer action 0 on ties): profile map cycles.
    def br_pennies(profile):
        a, b = profile
        # Row wants to match column; column wants to mismatch row.
        return (b, 1 - a)

    ok_cycle, period = eventually_periodic(
        lambda p: br_pennies(p), (0, 0), 4)
    check(19, "L2-E0 item 8: deterministic best-response operator cycles "
              "(matching pennies)", ok_cycle and period > 1,
          f"period {period}")

    print()
    if FAILURES:
        print(f"RESULT: {len(FAILURES)}/{CHECKS_RUN} CHECKS FAILED: "
              + ", ".join(str(n) for n, _ in FAILURES))
        raise SystemExit(1)
    print(f"RESULT: ALL {CHECKS_RUN}/{CHECKS_RUN} CHECKS PASS")


if __name__ == "__main__":
    main()
