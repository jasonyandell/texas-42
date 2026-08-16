#!/usr/bin/env python3
"""
pwl.py -- exact piecewise-linear machinery on the ray [0, inf).

Independent of lambda_probe.py's `_combine` (whose no-cross branch was repaired
on 2026-08-09).  Here the winner on each piece is selected at the piece's LEFT
endpoint with an exact slope tiebreak, so no probe point is ever needed and no
point can fall outside the interval it is meant to test.

Representations
---------------
segments : [(x_start, slope, intercept)], x_start strictly increasing, first 0.
           value on [x_start, next_x_start) is slope*x + intercept.
Cvx      : convex PWL as (c, s, {breakpoint: slope_increment}).  Exact for
           convex continuous PWLs and closed under addition in O(1) amortized,
           which is what makes summing 1680 per-world convex curves tractable.

All coefficients may be int or Fraction; breakpoints are Fractions.  No floats.
"""

from fractions import Fraction

F = Fraction


# ------------------------------------------------------------ segment lists

def seg_at(f, x):
    """The (slope, intercept) active at x."""
    cur = None
    for xs, sl, ic in f:
        if xs <= x:
            cur = (sl, ic)
        else:
            break
    assert cur is not None
    return cur


def pwl_eval(f, x):
    sl, ic = seg_at(f, x)
    return sl * x + ic


def breaks(f):
    return [xs for xs, _, _ in f]


def interior_breaks(f):
    return [xs for xs, _, _ in f if xs > 0]


def _normalize(segs):
    out = []
    for x, sl, ic in segs:
        if out and out[-1][1] == sl and out[-1][2] == ic:
            continue
        out.append((x, sl, ic))
    return out


def pwl_max2(f, g):
    return _pwl_combine(f, g, True)


def pwl_min2(f, g):
    return _pwl_combine(f, g, False)


def _pwl_combine(f, g, take_max):
    xs = sorted(set(breaks(f)) | set(breaks(g)))
    out = []
    for i, x0 in enumerate(xs):
        x1 = xs[i + 1] if i + 1 < len(xs) else None
        fs, fc = seg_at(f, x0)
        gs, gc = seg_at(g, x0)
        pieces = [(x0, x1)]
        if fs != gs:
            xc = F(gc - fc, fs - gs)
            if xc > x0 and (x1 is None or xc < x1):
                pieces = [(x0, xc), (xc, x1)]
        for a0, _a1 in pieces:
            fv = fs * a0 + fc
            gv = gs * a0 + gc
            if fv != gv:
                pick_f = (fv > gv) if take_max else (fv < gv)
            else:                       # equal at the left endpoint: slope decides
                pick_f = (fs >= gs) if take_max else (fs <= gs)
            sl, ic = (fs, fc) if pick_f else (gs, gc)
            out.append((a0, sl, ic))
    return _normalize(out)


def pwl_max(fs):
    r = fs[0]
    for f in fs[1:]:
        r = pwl_max2(r, f)
    return r


def pwl_sub(f, g):
    """Exact f - g as a segment list."""
    xs = sorted(set(breaks(f)) | set(breaks(g)))
    out = []
    for x0 in xs:
        fs, fc = seg_at(f, x0)
        gs, gc = seg_at(g, x0)
        out.append((x0, fs - gs, fc - gc))
    return _normalize(out)


def pwl_add(f, g):
    xs = sorted(set(breaks(f)) | set(breaks(g)))
    out = []
    for x0 in xs:
        fs, fc = seg_at(f, x0)
        gs, gc = seg_at(g, x0)
        out.append((x0, fs + gs, fc + gc))
    return _normalize(out)


def pwl_scale(f, k):
    return _normalize([(x, sl * k, ic * k) for x, sl, ic in f])


def pwl_is_nonneg(f):
    """EXACT proof of f >= 0 on the whole ray [0, inf), segment by segment.

    On a bounded segment [x0, x1] a linear function attains its minimum at an
    endpoint, so checking both endpoints is a proof.  On the final unbounded
    segment [x0, inf) nonnegativity is equivalent to value(x0) >= 0 and
    slope >= 0.  Returns (ok, list-of-per-segment-certificates)."""
    certs = []
    ok = True
    for i, (x0, sl, ic) in enumerate(f):
        v0 = sl * x0 + ic
        if i + 1 < len(f):
            x1 = f[i + 1][0]
            v1 = sl * x1 + ic
            good = (v0 >= 0 and v1 >= 0)
            certs.append({"seg": [str(x0), str(x1)], "v_left": str(v0),
                          "v_right": str(v1), "slope": str(sl), "ok": good})
        else:
            good = (v0 >= 0 and sl >= 0)
            certs.append({"seg": [str(x0), "inf"], "v_left": str(v0),
                          "slope": str(sl), "ok": good})
        ok = ok and good
    return ok, certs


def pwl_str(f):
    parts = []
    for i, (xs, sl, ic) in enumerate(f):
        end = f[i + 1][0] if i + 1 < len(f) else "inf"
        parts.append(f"[{xs},{end}): {ic}{'+' if sl >= 0 else ''}{sl}L")
    return "  ".join(parts)


# --------------------------------------------------- upper envelope of lines

def upper_env_lines(lines):
    """Exact upper envelope on [0, inf) of the lines [(a, b)] (value a + b*x).
    Returns a segment list.  Coefficients may be ints; breakpoints Fractions."""
    best = max(lines, key=lambda ab: (ab[0], ab[1]))
    segs = [(F(0), best[1], best[0])]
    cur = best
    x = F(0)
    while True:
        cand = None
        for (a, b) in lines:
            if b <= cur[1]:
                continue
            xc = F(a - cur[0], cur[1] - b)   # cur[0]+cur[1]x = a+bx
            if xc > x:
                if cand is None or xc < cand[0] or (xc == cand[0] and b > cand[1][1]):
                    cand = (xc, (a, b))
        if cand is None:
            break
        x, cur = cand[0], cand[1]
        segs.append((x, cur[1], cur[0]))
    return segs


# --------------------------------------------------------- convex increments

class Cvx:
    """Convex PWL on [0, inf) stored as an initial line plus slope increments.
    Addition is O(#breakpoints of the addend); this is what makes the sum of
    1680 per-world convex curves tractable."""

    __slots__ = ("c", "s", "inc")

    def __init__(self):
        self.c = 0
        self.s = 0
        self.inc = {}

    def add_line(self, a, b):
        self.c += a
        self.s += b

    def add_env(self, segs):
        """Add a convex segment list (e.g. from upper_env_lines)."""
        self.c += segs[0][2]
        self.s += segs[0][1]
        prev = segs[0][1]
        for x, sl, _ic in segs[1:]:
            d = sl - prev
            assert d > 0, ("non-convex increment", segs)
            self.inc[x] = self.inc.get(x, 0) + d
            prev = sl

    def add_cvx(self, o):
        self.c += o.c
        self.s += o.s
        for x, d in o.inc.items():
            self.inc[x] = self.inc.get(x, 0) + d

    def add_segments(self, segs):
        """Add an arbitrary convex segment list (checks convexity)."""
        self.add_env(segs)

    def to_segments(self, scale=1):
        xs = sorted(self.inc)
        out = [(F(0), self.s * scale, self.c * scale)]
        sl, ic = self.s * scale, self.c * scale
        for x in xs:
            nsl = sl + self.inc[x] * scale
            nic = (sl * x + ic) - nsl * x
            out.append((x, nsl, nic))
            sl, ic = nsl, nic
        return _normalize(out)


def cvx_from_segments(segs):
    """Convert a convex segment list into increment form."""
    c = Cvx()
    c.add_env(segs)
    return c
