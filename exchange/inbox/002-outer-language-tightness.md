---
number: 2
slug: outer-language-tightness
conversation: https://chatgpt.com/c/6a66e7f0-57cc-83ea-b6c8-eab6080b8b76
submitted-at: 2026-07-27T05:09:06.140Z
harvested-at: 2026-07-27T13:01:36.593Z
extraction: copy-button
caveats: none
---
I found a **full shallow counterexample**. It lives at (k=(6,6,6)), uses one no-trump void in sixes, passes all four outer checks—including the lead-witness check—and is nevertheless unreachable under every declaration.

The new obstruction is stronger than REACH-10’s: one legal lead tile exists outside the pool, but any realizing trick would require **two distinct public six-followers**—the lead and at least one successful hidden follow—while only `6:6` is outside the pool.

The normative rules and the support/reachability results used below are those in the supplied rules profile and mathematical foundation.   The supplied verifier informed the finite-proof style, but the artifact below is self-contained and imports none of it. 

## MACHINE-CHECKABLE ARTIFACTS

FINAL ANSWER: COUNTEREXAMPLE

```json
{
  "declaration": "NT",
  "capacities": [6, 6, 6],
  "voids": [
    [6],
    [],
    []
  ],
  "pool": [
    [0, 0],
    [1, 0],
    [1, 1],
    [2, 0],
    [2, 1],
    [2, 2],
    [3, 0],
    [3, 1],
    [3, 2],
    [3, 3],
    [4, 0],
    [4, 1],
    [6, 0],
    [6, 1],
    [6, 2],
    [6, 3],
    [6, 4],
    [6, 5]
  ],
  "normal_form": {
    "certain": [
      [],
      [],
      []
    ],
    "ambiguity": {
      "tag": "Ternary",
      "residuals": [6, 6, 6],
      "exclusions": [
        [[6, 0], 1],
        [[6, 1], 1],
        [[6, 2], 1],
        [[6, 3], 1],
        [[6, 4], 1],
        [[6, 5], 1]
      ]
    }
  }
}
```

Executable copy: [verify_new_counterexample.py](sandbox:/mnt/data/verify_new_counterexample.py)

```python
#!/usr/bin/env python3
"""Self-contained verification of a shallow Straight-42 support counterexample.

No dependencies, network, or file I/O. Exit status is zero iff every check
passes. Hidden seats 1,2,3 are viewer-relative clockwise offsets.
"""

from __future__ import annotations

from itertools import permutations
import json
import time


PIPS = tuple(range(7))
CALLED = 7
DECLARATIONS = PIPS + ("DT", "NT")
DOMINOES = tuple((h, l) for h in PIPS for l in range(h + 1))
INDEX = {d: i for i, d in enumerate(DOMINOES)}
ALL_MASK = (1 << len(DOMINOES)) - 1


def mask_of(dominoes):
    mask = 0
    for domino in dominoes:
        mask |= 1 << INDEX[domino]
    return mask


def dominoes_of(mask):
    return tuple(d for d in DOMINOES if mask & (1 << INDEX[d]))


def contains(domino, pip):
    return domino[0] == pip or domino[1] == pip


SIGMA = tuple(
    mask_of(d for d in DOMINOES if contains(d, p))
    for p in PIPS
)
DOUBLES = mask_of(d for d in DOMINOES if d[0] == d[1])


def called_mask(declaration):
    if declaration in PIPS:
        return SIGMA[declaration]
    if declaration == "DT":
        return DOUBLES
    if declaration == "NT":
        return 0
    raise ValueError(declaration)


def effective_follow_mask(declaration, context):
    called = called_mask(declaration)
    if context == CALLED:
        return called
    return SIGMA[context] & ~called


def led_context(declaration, domino):
    bit = 1 << INDEX[domino]
    return CALLED if called_mask(declaration) & bit else domino[0]


def lead_fibers(declaration):
    fibers = {}
    for domino in DOMINOES:
        q = led_context(declaration, domino)
        fibers[q] = fibers.get(q, 0) | (1 << INDEX[domino])
    return fibers


LEAD_FIBERS = {
    declaration: lead_fibers(declaration)
    for declaration in DECLARATIONS
}


def declaration_rank(declaration, domino):
    h, l = domino
    if declaration == "DT" and h == l:
        return h
    if h == l:
        return 100  # TOP, above every mixed pip sum
    return h + l


def trick_key(declaration, domino, context):
    bit = 1 << INDEX[domino]
    called = called_mask(declaration)

    if declaration != "NT" and called & bit:
        return (2, declaration_rank(declaration, domino))

    if effective_follow_mask(declaration, context) & bit:
        return (1, declaration_rank(declaration, domino))

    return (0, 0)


def trick_winner(declaration, actors, tiles):
    assert len(actors) == len(tiles) == 4
    context = led_context(declaration, tiles[0])
    keys = tuple(
        trick_key(declaration, tile, context)
        for tile in tiles
    )
    winning_index = max(range(4), key=lambda i: keys[i])
    return actors[winning_index]


def hall_feasible(universe, possible, capacities):
    if universe.bit_count() != sum(capacities):
        return False

    for seat_subset in range(1, 1 << 3):
        neighbors = 0
        quota = 0

        for seat in range(3):
            if seat_subset & (1 << seat):
                neighbors |= possible[seat] & universe
                quota += capacities[seat]

        if neighbors.bit_count() < quota:
            return False

    return True


def marginal_holder_masks(universe, possible, capacities):
    """Canonical reduction by forced-edge Hall feasibility."""

    if not hall_feasible(universe, possible, capacities):
        return None

    result = [0, 0, 0]
    remaining = universe

    while remaining:
        tile_bit = remaining & -remaining
        remaining -= tile_bit

        for seat in range(3):
            if capacities[seat] == 0:
                continue
            if not (possible[seat] & tile_bit):
                continue

            successor_universe = universe ^ tile_bit
            successor_possible = tuple(
                p & ~tile_bit for p in possible
            )
            successor_capacities = list(capacities)
            successor_capacities[seat] -= 1

            if hall_feasible(
                successor_universe,
                successor_possible,
                tuple(successor_capacities),
            ):
                result[seat] |= tile_bit

    return tuple(result)


def support_normal_form(universe, possible, capacities):
    marginal = marginal_holder_masks(
        universe,
        possible,
        capacities,
    )

    if marginal is None:
        return {"tag": "Empty"}

    certain_masks = [0, 0, 0]
    holders_by_domino = {}

    for domino in dominoes_of(universe):
        bit = 1 << INDEX[domino]
        holders = tuple(
            seat
            for seat in range(3)
            if marginal[seat] & bit
        )
        holders_by_domino[domino] = holders

        if len(holders) == 1:
            certain_masks[holders[0]] |= bit

    certain = [
        [[h, l] for h, l in dominoes_of(certain_masks[seat])]
        for seat in range(3)
    ]

    certain_union = (
        certain_masks[0]
        | certain_masks[1]
        | certain_masks[2]
    )
    ambiguous_mask = universe & ~certain_union
    ambiguous = dominoes_of(ambiguous_mask)

    residuals = tuple(
        capacities[seat] - certain_masks[seat].bit_count()
        for seat in range(3)
    )
    active = tuple(
        seat
        for seat in range(3)
        if residuals[seat] > 0
    )

    if not ambiguous:
        ambiguity = {
            "tag": "Determinate"
        }

    elif len(active) == 2:
        inactive = next(
            seat for seat in range(3)
            if seat not in active
        )

        for domino in ambiguous:
            if tuple(holders_by_domino[domino]) != active:
                raise AssertionError(
                    "noncanonical binary holder relation"
                )

        ambiguity = {
            "tag": "Binary",
            "inactive": inactive + 1,
            "split": residuals[active[0]],
        }

    elif len(active) == 3:
        exclusions = []

        for domino in ambiguous:
            holders = set(holders_by_domino[domino])

            if len(holders) == 2:
                excluded = next(
                    iter({0, 1, 2} - holders)
                )
                exclusions.append(
                    [[domino[0], domino[1]], excluded + 1]
                )

            elif len(holders) != 3:
                raise AssertionError(
                    "invalid ternary holder relation"
                )

        ambiguity = {
            "tag": "Ternary",
            "residuals": list(residuals),
            "exclusions": exclusions,
        }

    else:
        raise AssertionError(
            f"invalid active-seat count {len(active)}"
        )

    return {
        "certain": certain,
        "ambiguity": ambiguity,
    }


def cells_from_voids(declaration, universe, voids):
    possible = []

    for seat_voids in voids:
        forbidden = 0

        for context in seat_voids:
            forbidden |= effective_follow_mask(
                declaration,
                context,
            )

        possible.append(universe & ~forbidden)

    return tuple(possible)


FOLLOWER_MAXIMUM = {
    frozenset(): frozenset(),
    frozenset({0}): frozenset({0}),
    frozenset({1}): frozenset(),
    frozenset({2}): frozenset(),
    frozenset({0, 1}): frozenset({0, 1}),
    frozenset({0, 2}): frozenset({0}),
    frozenset({1, 2}): frozenset({2}),
    frozenset({0, 1, 2}): frozenset({1, 2}),
}


def schedule_admissible(declaration, capacities, voids):
    high = max(capacities)
    completed = 7 - high

    if capacities[0] == capacities[1] == capacities[2]:
        already_followed = frozenset()
    else:
        low = frozenset(
            i
            for i, k in enumerate(capacities)
            if k == high - 1
        )
        already_followed = FOLLOWER_MAXIMUM[low]

    used = sorted(set().union(*voids))

    if any(
        q not in LEAD_FIBERS[declaration]
        for q in used
    ):
        return False

    if len(used) <= completed:
        return True

    if len(used) != completed + 1:
        return False

    for q in used:
        members = frozenset(
            i
            for i in range(3)
            if q in voids[i]
        )

        if members and members.issubset(already_followed):
            return True

    return False


def lead_witness_ok(declaration, universe, voids):
    used = set().union(*voids)

    return all(
        LEAD_FIBERS[declaration][q] & ~universe
        for q in used
    )


def replay_public_prefix(
    declaration,
    actors,
    tiles,
    viewer_initial_hand,
):
    """Replay actor order, viewer legality, and trick winners.

    Each hidden actor appears exactly once in every enumerated
    prefix. A hidden nonfollow records a void.

    Once the resulting void cells pass Hall, hidden-hand legality
    is exact in this phase: a successful hidden follow carries its
    own follower witness, while a hidden slough has no earlier
    removed hidden tile and its current remainder is constrained
    to avoid the whole follow set.
    """

    viewer_hand = set(viewer_initial_hand)
    voids = [set(), set(), set()]

    expected_actor = actors[0]
    trick_actors = []
    trick_tiles = []

    for actor, tile in zip(actors, tiles):
        if actor != expected_actor:
            return None

        if actor == 0:
            if tile not in viewer_hand:
                return None

            if trick_tiles:
                context = led_context(
                    declaration,
                    trick_tiles[0],
                )
                followers = effective_follow_mask(
                    declaration,
                    context,
                )
                has_follower = any(
                    followers & (1 << INDEX[d])
                    for d in viewer_hand
                )

                if (
                    has_follower
                    and not (
                        followers
                        & (1 << INDEX[tile])
                    )
                ):
                    return None

            viewer_hand.remove(tile)

        else:
            if trick_tiles:
                context = led_context(
                    declaration,
                    trick_tiles[0],
                )
                followers = effective_follow_mask(
                    declaration,
                    context,
                )

                if not (
                    followers
                    & (1 << INDEX[tile])
                ):
                    voids[actor - 1].add(context)

        trick_actors.append(actor)
        trick_tiles.append(tile)

        if len(trick_tiles) < 4:
            expected_actor = (actor + 1) % 4

        else:
            expected_actor = trick_winner(
                declaration,
                tuple(trick_actors),
                tuple(trick_tiles),
            )
            trick_actors.clear()
            trick_tiles.clear()

    return tuple(
        frozenset(v)
        for v in voids
    )


def exhaustive_shallow_trace_count(
    declaration,
    context,
    membership,
    universe,
    capacities,
    target_normal,
):
    """Exhaust every prefix with hidden capacities (6,6,6).

    The ten tiles outside U split into:
      * one played tile for each hidden seat; and
      * the viewer's seven-tile initial hand.

    The only possible actor skeletons are:

      A. (1,2,3), the three-hidden-play prefix of trick 1;
      B. one complete first trick, from any leader;
      C. B followed by a viewer lead of trick 2, which replay
         accepts only when the viewer won B.
    """

    complement = dominoes_of(ALL_MASK ^ universe)

    expected_voids = tuple(
        frozenset({context})
        if membership & (1 << seat)
        else frozenset()
        for seat in range(3)
    )

    candidate_count = 0
    realizing_count = 0

    def test_trace(actors, tiles, viewer_hand):
        nonlocal candidate_count, realizing_count

        candidate_count += 1

        derived_voids = replay_public_prefix(
            declaration,
            actors,
            tiles,
            viewer_hand,
        )

        if derived_voids != expected_voids:
            return

        possible = cells_from_voids(
            declaration,
            universe,
            derived_voids,
        )

        if not hall_feasible(
            universe,
            possible,
            capacities,
        ):
            return

        if (
            support_normal_form(
                universe,
                possible,
                capacities,
            )
            != target_normal
        ):
            return

        realizing_count += 1

    for hidden_tiles in permutations(complement, 3):
        played_by_hidden = {
            1: hidden_tiles[0],
            2: hidden_tiles[1],
            3: hidden_tiles[2],
        }

        viewer_hand = tuple(
            d
            for d in complement
            if d not in hidden_tiles
        )

        # A: only leader 1 yields three hidden actors before
        # the viewer acts.
        actors_a = (1, 2, 3)
        tiles_a = tuple(
            played_by_hidden[actor]
            for actor in actors_a
        )
        test_trace(
            actors_a,
            tiles_a,
            viewer_hand,
        )

        # B and C.
        for leader in range(4):
            actors_b = tuple(
                (leader + offset) % 4
                for offset in range(4)
            )

            for first_viewer_tile in viewer_hand:
                tiles_b = tuple(
                    first_viewer_tile
                    if actor == 0
                    else played_by_hidden[actor]
                    for actor in actors_b
                )

                test_trace(
                    actors_b,
                    tiles_b,
                    viewer_hand,
                )

                for second_viewer_tile in viewer_hand:
                    if (
                        second_viewer_tile
                        == first_viewer_tile
                    ):
                        continue

                    test_trace(
                        actors_b + (0,),
                        tiles_b + (second_viewer_tile,),
                        viewer_hand,
                    )

    return candidate_count, realizing_count


class Results:
    def __init__(self):
        self.failures = 0

    def check(self, name, condition, detail=""):
        suffix = f" {detail}" if detail else ""

        if condition:
            print(f"PASS {name}{suffix}")
        else:
            self.failures += 1
            print(f"FAIL {name}{suffix}")


def main():
    started = time.perf_counter()
    results = Results()

    declaration = "NT"
    capacities = (6, 6, 6)
    voids = (
        frozenset({6}),
        frozenset(),
        frozenset(),
    )

    pool_tiles = (
        (0, 0),
        (1, 0),
        (1, 1),
        (2, 0),
        (2, 1),
        (2, 2),
        (3, 0),
        (3, 1),
        (3, 2),
        (3, 3),
        (4, 0),
        (4, 1),
        (6, 0),
        (6, 1),
        (6, 2),
        (6, 3),
        (6, 4),
        (6, 5),
    )

    universe = mask_of(pool_tiles)

    claimed_normal = {
        "certain": [[], [], []],
        "ambiguity": {
            "tag": "Ternary",
            "residuals": [6, 6, 6],
            "exclusions": [
                [[6, 0], 1],
                [[6, 1], 1],
                [[6, 2], 1],
                [[6, 3], 1],
                [[6, 4], 1],
                [[6, 5], 1],
            ],
        },
    }

    possible = cells_from_voids(
        declaration,
        universe,
        voids,
    )

    # Outer check (1).
    capacity_ok = (
        max(capacities) - min(capacities) <= 1
    )
    results.check(
        "check-1-capacity-shape",
        capacity_ok,
        f"capacities={capacities}",
    )

    results.check(
        "pool-capacity-conservation",
        universe.bit_count() == sum(capacities),
        (
            f"pool={universe.bit_count()} "
            f"total_capacity={sum(capacities)}"
        ),
    )

    # Outer check (2).
    results.check(
        "check-2-schedule-admissibility",
        schedule_admissible(
            declaration,
            capacities,
            voids,
        ),
        "j=1 used_contexts=[6]",
    )

    # Outer check (3).
    witness_tiles = dominoes_of(
        LEAD_FIBERS[declaration][6]
        & ~universe
    )
    results.check(
        "check-3-lead-witness",
        lead_witness_ok(
            declaration,
            universe,
            voids,
        ),
        f"outside_lead_fiber={witness_tiles}",
    )

    # Outer check (4).
    results.check(
        "check-4-Hall-feasibility",
        hall_feasible(
            universe,
            possible,
            capacities,
        ),
    )

    computed_normal = support_normal_form(
        universe,
        possible,
        capacities,
    )
    results.check(
        "normal-form-reduction",
        computed_normal == claimed_normal,
        (
            "computed="
            + json.dumps(
                computed_normal,
                separators=(",", ":"),
            )
        ),
    )

    marginal = marginal_holder_masks(
        universe,
        possible,
        capacities,
    )
    results.check(
        "raw-cells-already-reduced",
        marginal == possible,
    )

    # Complete static family:
    #   9 declarations
    #   x (one no-void generator
    #      + seven contexts x seven nonempty memberships)
    target_marginal = marginal
    generator_count = 0
    matches = []

    for candidate_declaration in DECLARATIONS:
        generator_count += 1

        no_void_possible = (
            universe,
            universe,
            universe,
        )

        if (
            marginal_holder_masks(
                universe,
                no_void_possible,
                capacities,
            )
            == target_marginal
        ):
            matches.append(
                (candidate_declaration, None, 0)
            )

        for context in sorted(
            LEAD_FIBERS[candidate_declaration]
        ):
            follow = effective_follow_mask(
                candidate_declaration,
                context,
            )

            for membership in range(1, 8):
                generator_count += 1

                candidate_possible = tuple(
                    universe & ~follow
                    if membership & (1 << seat)
                    else universe
                    for seat in range(3)
                )

                candidate_marginal = (
                    marginal_holder_masks(
                        universe,
                        candidate_possible,
                        capacities,
                    )
                )

                if candidate_marginal == target_marginal:
                    matches.append(
                        (
                            candidate_declaration,
                            context,
                            membership,
                        )
                    )

    expected_matches = [
        (6, 7, 0b001),
        ("DT", 6, 0b001),
        ("NT", 6, 0b001),
    ]

    results.check(
        "complete-static-generator-family",
        generator_count == 450,
        f"family_size={generator_count}",
    )

    results.check(
        "static-decoder-exhaustion",
        matches == expected_matches,
        f"matches={matches}",
    )

    total_trace_candidates = 0
    total_realizers = 0

    for (
        candidate_declaration,
        context,
        membership,
    ) in matches:
        candidates, realizers = (
            exhaustive_shallow_trace_count(
                candidate_declaration,
                context,
                membership,
                universe,
                capacities,
                claimed_normal,
            )
        )

        total_trace_candidates += candidates
        total_realizers += realizers

        results.check(
            (
                f"trace-exhaustion-"
                f"{candidate_declaration}-"
                f"{context}-{membership}"
            ),
            (
                candidates == 141_840
                and realizers == 0
            ),
            (
                f"candidates={candidates} "
                f"realizers={realizers}"
            ),
        )

    results.check(
        "non-reachability-all-declarations",
        (
            total_trace_candidates == 425_520
            and total_realizers == 0
        ),
        (
            f"trace_candidates="
            f"{total_trace_candidates} "
            f"realizers={total_realizers}"
        ),
    )

    # Independently expose the local obstruction.
    supply_details = []

    for (
        candidate_declaration,
        context,
        membership,
    ) in matches:
        follow_outside = dominoes_of(
            effective_follow_mask(
                candidate_declaration,
                context,
            )
            & ~universe
        )

        lead_outside = dominoes_of(
            LEAD_FIBERS[
                candidate_declaration
            ][context]
            & ~universe
        )

        supply_details.append(
            (
                candidate_declaration,
                context,
                follow_outside,
                lead_outside,
            )
        )

    results.check(
        "follower-supply-obstruction",
        supply_details == [
            (
                6,
                7,
                ((6, 6),),
                ((6, 6),),
            ),
            (
                "DT",
                6,
                tuple(),
                tuple(),
            ),
            (
                "NT",
                6,
                ((6, 6),),
                ((6, 6),),
            ),
        ],
        f"details={supply_details}",
    )

    elapsed = time.perf_counter() - started

    results.check(
        "runtime",
        elapsed < 7200,
        f"seconds={elapsed:.3f}",
    )

    if results.failures:
        print(
            f"FAIL overall failures="
            f"{results.failures}"
        )
        return 1

    print(
        "PASS overall counterexample_verified "
        f"generators={generator_count} "
        f"traces={total_trace_candidates} "
        f"seconds={elapsed:.3f}"
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except SystemExit:
        raise
    except Exception as exc:
        print(
            "FAIL exception "
            f"{type(exc).__name__}: {exc}"
        )
        raise SystemExit(1)
```

### Proof

1. **[USES: witness JSON; definition of (\widehat\sigma_q^\delta); definition of capacity cells]**
   Put
   [
   X={6!:!0,6!:!1,6!:!2,6!:!3,6!:!4,6!:!5}
   ]
   and
   [
   \begin{aligned}
   N={&
   0!:!0,,
   1!:!0,1!:!1,,
   2!:!0,2!:!1,2!:!2,\
   &3!:!0,3!:!1,3!:!2,3!:!3,,
   4!:!0,4!:!1
   }.
   \end{aligned}
   ]
   Then (U=X\sqcup N), with (|X|=6), (|N|=12), and (|U|=18).

   Under no-trump,
   [
   \widehat\sigma_6^{\mathrm{NT}}=\sigma_6
   =X\sqcup{6!:!6}.
   ]
   With (V_1={6}) and (V_2=V_3=\varnothing), the raw cells are
   [
   P_1=U\setminus\sigma_6=N,\qquad
   P_2=P_3=U,
   ]
   with capacities ((6,6,6)).

2. **[USES: outer check (1); reachable-capacity theorem §7.13.1]**
   The capacity shape passes:
   [
   \max_s k_s-\min_s k_s=6-6=0\le1.
   ]
   Also (|U|=18=6+6+6). Here (h=6), so the outer phase parameter is
   [
   j=7-h=1.
   ]
   The profile is equal, hence (B=\varnothing) and the prescribed boundary convention gives (F(B)=\varnothing).

3. **[USES: outer check (2); schedule-language characterization §7.13.3; observable-context theorem §7.13.2]**
   The used-context set is
   [
   Q_{\mathrm{used}}={6}.
   ]
   Context (6) is leadable under no-trump, and
   [
   |Q_{\mathrm{used}}|=1\le j=1.
   ]
   Therefore the void masks are schedule-admissible.

4. **[USES: outer check (3); lead-fiber definition; no-trump led-context rule]**
   Under no-trump,
   [
   L_{\mathrm{NT},6}
   =================

   {6!:!0,6!:!1,\ldots,6!:!6}.
   ]
   Since (U) contains exactly `6:0` through `6:5`,
   [
   L_{\mathrm{NT},6}\setminus U={6!:!6}\ne\varnothing.
   ]
   Thus the lead-witness check passes. This is the point at which the witness differs materially from REACH-10: the required lead fiber is not contained in (U).

5. **[USES: outer check (4); Hall theorem §7.7]**
   The seven nonempty Hall inequalities hold:
   [
   |P_1|=12\ge6,\qquad |P_2|=|P_3|=18\ge6,
   ]
   and every two-seat union and the three-seat union equal (U), so
   [
   |P_i\cup P_j|=18\ge12,\qquad
   |P_1\cup P_2\cup P_3|=18=18.
   ]
   Hence (\Phi\ne\varnothing).

   One explicit world is:
   [
   \begin{aligned}
   H_1={}&
   {0!:!0,1!:!0,1!:!1,2!:!0,2!:!1,2!:!2},\
   H_2={}&X,\
   H_3={}&
   {3!:!0,3!:!1,3!:!2,3!:!3,4!:!0,4!:!1}.
   \end{aligned}
   ]

6. **[USES: marginal-holder definition §7.9; support normal form §7.10]**
   The raw cells are already reduced.

   For (x\in X), the only locally allowed holders are (h_2,h_3). Assigning all of (X) to either one gives a world containing (x) at that seat, so both edges are marginally supported.

   For (n\in N):

   * to place (n) at (h_1), choose it together with any other five members of (N);
   * to place (n) at (h_2) or (h_3), choose six of the other eleven members of (N) for (h_1), then include (n) in the desired seat’s six-tile share of the remaining twelve tiles.

   Thus
   [
   P_1^\star=N,\qquad P_2^\star=P_3^\star=U.
   ]
   Every tile has at least two possible holders, so
   [
   K_1=K_2=K_3=\varnothing,\qquad W=U.
   ]
   All residuals are six. Exactly the six members of (X) omit (h_1); every member of (N) has all three holders. This is precisely the claimed ternary normal form.

7. **[USES: capacity definition (k_s=7-|B_s|); cyclic actor order; trick-winner rule]**
   Any legal prefix having hidden capacities ((6,6,6)) has exactly one public play by each hidden seat.

   The viewer can then have made only zero, one, or two plays:

   * **zero viewer plays:** the prefix is exactly `(h1,h2,h3)`, so (h_1) led the still-open first trick;
   * **one viewer play:** exactly one four-play trick has completed, from any leader;
   * **two viewer plays:** exactly one trick has completed, the viewer won it, and the viewer has led trick two.

   A third viewer play would require some hidden seat to make a second play first. Consequently every hidden failure to follow occurred in the same first-trick context. No legal prefix with this capacity profile can contain two used hidden-void contexts.

8. **[USES: Step 7; declaration domain; seven-observable-context theorem §7.13.2; cell construction]**
   Therefore every possible raw generator for this target is static and belongs to the complete family
   [
   9\bigl(1+7(2^3-1)\bigr)
   =9(1+49)
   =450.
   ]
   For each declaration there is:

   * one no-void generator; and
   * for each of its seven leadable contexts, one generator for each of the seven nonempty hidden-seat void memberships.

   Any legal prefix producing the target normal form must reduce from one member of this family.

9. **[USES: Step 8; forced-edge Hall criterion §7.9; CELL-14 / canonical-normal-form equivalence]**
   Exhaustive reduction of all 450 generators yields exactly three whose normal form equals the target:
   [
   \begin{array}{c|c|c}
   \text{declaration}&\text{context}&\text{void membership}\\hline
   6&7&{h_1}\
   \mathrm{DT}&6&{h_1}\
   \mathrm{NT}&6&{h_1}.
   \end{array}
   ]
   No no-void generator and no other declaration/context/membership decodes to the target.

   By CELL-14, a legal prefix producing (S) must therefore realize one of these three generators.

10. **[USES: Step 9; lead-witness necessity §7.13.4]**
    The doubles-trump candidate is impossible. Under doubles trump,
    [
    L_{\mathrm{DT},6}
    =================

    {6!:!0,6!:!1,\ldots,6!:!5}
    =X
    \subseteq U.
    ]
    It violates lead-witness necessity and cannot be generated by a legal prefix.

11. **[USES: Step 7; Step 9; follow-if-possible rule; definition of a public void]**
    Consider either remaining candidate:

    * sixes trump with called context (7); or
    * no-trump with natural context (6).

    In both cases the relevant effective follow set is
    [
    F=\sigma_6=X\sqcup{6!:!6},
    ]
    and hence
    [
    F\setminus U={6!:!6}.
    ]

    Only (h_1) is void. Because (h_1) has made exactly one play and acquired the void on that play, (h_1) was a follower and was not the trick leader.

    At least one of (h_2,h_3) was also a follower in that same trick:

    * if the viewer led, both were followers;
    * if (h_2) led, (h_3) was a follower;
    * if (h_3) led, (h_2) was a follower.

    That hidden follower has no context-(q) void, so it must have played a tile in (F). The lead tile also lies in (F). These are two distinct public played tiles, so both lie outside (U). Therefore every realization would require
    [
    |F\setminus U|\ge2.
    ]
    But (|F\setminus U|=1). Contradiction.

12. **[USES: Steps 7–11; exact legal-set and trick-winner rules; Hall losslessness §7.5]**
    The program independently executes the complete shallow-prefix search for each of the three static matches.

    Since (|\mathcal D\setminus U|=10), choose the distinct played tile of each hidden seat in
    [
    10P3=10\cdot9\cdot8=720
    ]
    ordered ways. The other seven complement tiles are then exactly the viewer’s initial hand.

    For each such choice the program exhausts:
    [
    1
    +4\cdot7
    +4\cdot7\cdot6
    =197
    ]
    syntactic prefixes:

    * the sole three-hidden-play skeleton;
    * each leader and viewer tile for one completed trick;
    * each leader and ordered pair of viewer tiles for a completed trick followed by a viewer lead.

    It replays actor order, viewer follow legality, declaration-relative trick winner, derived hidden voids, Hall feasibility, and final normal-form equality. This gives
    [
    720\cdot197=141{,}840
    ]
    candidates per static match and
    [
    3\cdot141{,}840=425{,}520
    ]
    total candidates. The number of realizing traces is zero.

13. **[USES: Steps 2–6 and 9–12]**
    The declaration-tagged tuple
    [
    (\mathrm{NT},(6,6,6),({6},\varnothing,\varnothing),U)
    ]
    passes all four outer checks and reduces to the supplied normal form (S). The complete generator family contains no legal realization under any of the nine Straight declarations. Therefore
    [
    S\notin\mathscr R_{\mathrm{Str}}^m.
    ]

    Hence the four-check conjunction is **not sufficient**, even in the (j=1), equal-capacity, exactly-one-used-void-context phase.

