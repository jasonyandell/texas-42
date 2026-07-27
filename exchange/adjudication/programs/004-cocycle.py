#!/usr/bin/env python3
"""Finite verification of the transport cocycle identity f_{u,v} o f_{t,u} = f_{t,v}.

Closes the Step-15 quotient corollary gap left CONDITIONAL by exchange dispatch 004
(exchange/inbox/004-transport-reachability-commutation.md). Step 15's reachable-quotient
cardinality r_pip + |R_DT| + |R_NT| silently assumes that the order-preserving
complement transports compose functorially. This script discharges that assumption by
brute force: over all 343 = 7^3 ordered pip-trump triples (t, u, v) it verifies that
composing the tile transport f_{t,u} with f_{u,v} reproduces f_{t,v} on every one of the
28 dominoes (and likewise on the 7-pip and 8-context index maps). It also confirms the
identity leg f_{t,t} = id and the inverse leg f_{u,t} = f_{t,u}^{-1}.

Standard-library only, deterministic, no file/network I/O. The transport definition is
copied verbatim from exchange/adjudication/programs/004.py (make_transport + Transport),
not imported, so this receipt stands alone.
"""

from __future__ import annotations

from dataclasses import dataclass
import sys

# --- Transport definition, copied verbatim from programs/004.py -------------------

PIPS = tuple(range(7))
CALLED = 7
DOMINOES = tuple((high, low) for high in PIPS for low in range(high + 1))
ID_OF = {domino: index for index, domino in enumerate(DOMINOES)}
N_DOMINOES = len(DOMINOES)


@dataclass(frozen=True)
class Transport:
    pips: tuple[int, ...]
    dominoes: tuple[int, ...]
    contexts: tuple[int, ...]


def make_transport(source: int, target: int) -> Transport:
    source_complement = tuple(p for p in PIPS if p != source)
    target_complement = tuple(p for p in PIPS if p != target)
    pip_map = [-1] * 7
    pip_map[source] = target
    for left, right in zip(source_complement, target_complement):
        pip_map[left] = right
    pips = tuple(pip_map)
    domino_map = []
    for high, low in DOMINOES:
        image = tuple(sorted((pips[high], pips[low]), reverse=True))
        domino_map.append(ID_OF[image])
    return Transport(pips, tuple(domino_map), pips + (CALLED,))


# --- Cocycle check ----------------------------------------------------------------


class CheckFailure(Exception):
    pass


def require(condition: bool, detail: str) -> None:
    if not condition:
        raise CheckFailure(detail)


def compose(outer: tuple[int, ...], inner: tuple[int, ...]) -> tuple[int, ...]:
    """(outer o inner)(x) = outer[inner[x]] over an index-permutation encoding."""
    return tuple(outer[inner[x]] for x in range(len(inner)))


def main() -> int:
    triple_checks = 0
    identity_checks = 0
    inverse_checks = 0
    try:
        # Sanity: every make_transport output is a genuine permutation of its domain.
        for t in PIPS:
            for u in PIPS:
                f = make_transport(t, u)
                require(sorted(f.pips) == list(PIPS), f"pip map not bijective {t}->{u}")
                require(
                    sorted(f.dominoes) == list(range(N_DOMINOES)),
                    f"domino map not bijective {t}->{u}",
                )
                require(
                    sorted(f.contexts) == list(range(8)),
                    f"context map not bijective {t}->{u}",
                )

        # Identity leg: f_{t,t} is the identity on all three index sets.
        for t in PIPS:
            f = make_transport(t, t)
            require(f.pips == PIPS, f"f_{{{t},{t}}} pips not identity")
            require(f.dominoes == tuple(range(N_DOMINOES)), f"f_{{{t},{t}}} dominoes not identity")
            require(f.contexts == tuple(PIPS) + (CALLED,), f"f_{{{t},{t}}} contexts not identity")
            identity_checks += 1

        # Inverse leg: f_{u,t} = f_{t,u}^{-1} on the tile domain.
        for t in PIPS:
            for u in PIPS:
                fwd = make_transport(t, u)
                bwd = make_transport(u, t)
                require(
                    compose(bwd.dominoes, fwd.dominoes) == tuple(range(N_DOMINOES)),
                    f"inverse fails on dominoes {t}<->{u}",
                )
                inverse_checks += 1

        # Cocycle leg: f_{u,v} o f_{t,u} = f_{t,v} on pips, dominoes, and contexts.
        for t in PIPS:
            for u in PIPS:
                for v in PIPS:
                    f_tu = make_transport(t, u)
                    f_uv = make_transport(u, v)
                    f_tv = make_transport(t, v)
                    require(
                        compose(f_uv.pips, f_tu.pips) == f_tv.pips,
                        f"cocycle fails on pips ({t},{u},{v})",
                    )
                    require(
                        compose(f_uv.dominoes, f_tu.dominoes) == f_tv.dominoes,
                        f"cocycle fails on dominoes ({t},{u},{v})",
                    )
                    require(
                        compose(f_uv.contexts, f_tu.contexts) == f_tv.contexts,
                        f"cocycle fails on contexts ({t},{u},{v})",
                    )
                    triple_checks += 1
    except CheckFailure as exc:
        print(f"FAIL COCYCLE {exc}")
        return 1

    require_total = 7 * 7 * 7
    if triple_checks != require_total:
        print(f"FAIL COCYCLE only {triple_checks}/{require_total} triples checked")
        return 1

    print(f"PASS COCYCLE_IDENTITY {identity_checks}_identity_legs")
    print(f"PASS COCYCLE_INVERSE {inverse_checks}_inverse_legs")
    print(f"PASS COCYCLE {triple_checks}_ordered_pip_triples f_uv_o_f_tu_eq_f_tv")
    print("ALL_PASS 343 ordered triples over all 28-tile transports")
    return 0


if __name__ == "__main__":
    sys.exit(main())
