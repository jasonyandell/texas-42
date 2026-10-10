# Boundary reconciliation: which Walt computation the tiny model replaces

**Design only, 2026-10-04.** Claude Fable 5.1 (`claude-fable-5-1`), same session.
Sources read this turn, all preserved local artifacts: `EXCEL-BOUNDARY-RECOVERY.md`
(Jeb's read-only recovery of the authorized "Building an AI in Excel" chat),
`experiments/astra-sol-20261004/phase2/incoming/drive/nofusion_sc.py`,
`exact_tape.py`, `engine42.py`, `walt.c`, plus the pinned Rust solver read in
earlier turns (`walt/walt/src/solver/{mod,partnership,partnership_wire,selection}.rs`
at `cb1ef3b2`, identical at HEAD). No labels, training, builds, tests, benchmarks
or games were run. The completed diagnosis is untouched. Exploratory tier.

## 1. Jason's clarified intent, in one sentence

The tiny model replaces **one computation level of Walt's evaluator**: the inner
sampled pmake computation that a higher level calls many times; selection at
every level stays the ordinary deterministic legal argmax (argmin for a
defender) with lowest tile on exact ties. The outer hint vector (HINT-TARGET.md)
is the *display* of the top level; it does not by itself locate the inner
boundary. Message 23/24 of the chat (per the recovery memo) name the inner
sampled pmake as the replacement point and warn that lower evaluators can rank
alike while their values differ, so the consumer's contract decides what the
replacement must match.

## 2. Exact recovered evaluator contracts

| Artifact | Input | Output | Own future | Others | Fusion | Selector |
|---|---|---|---|---|---|---|
| `nofusion_sc.expectimax` | `rules, bidder, seat, hand_now (remaining mask), history [(seat,tile)], deal_full (public replay only), rem (D,4) worlds, tape or None` | `(vals: {legal tile → mean over D of declarer-made}, choice, legal)` | one decision per public-history information set (grouped fold, `pk` key) | branch over every legal tile at 1/n (tape None, "exact tail") or follow the shared tape | none at own nodes; root candidates not folded | `choose`: max for declaring seat, min for defender; ties lowest tile (`(vals[t], -t)` / `(vals[t], t)`) |
| `nofusion_sc.tape_nofusion` | `…, rng, W` | `(vals, choice)` | same | one `(W,28)` tape shared by all root siblings | none | same |
| `exact_tape.exact_tape` | `…, rng, W` | `(vals, choice)` | best response **per sampled world** | tape | **fused** (per-world extrema) | same |
| `exact_tape.v0` | `…, rng, W, T` | `(vals, choice)` | random | random tape, T draws per world | none | same |
| `walt.c value()` | `me, State, deals[], assume: Policy, voids` | scalar: sum of deal weights where the bid is made | one tile per public node, max over the whole deal set (declaring `me%2==1`), min otherwise | each deal's seat plays `assume(seat, hand, state, voids)`; deals grouped by the returned tile | none at own nodes (bundle-coupled) | none; caller selects |
| `walt.c walt_policy()` | `seat, hand, State, voids` + globals `G_assume, G_n` | **tile only**; the per-candidate values are computed inline and discarded | via `value` | via `value` | | strict `>`/`<` scan over ascending tiles: lowest tile on ties |
| `walt.c inner_L0` | same | tile | `walt_policy(random_policy, n=8)` | uniform random per call (no shared tape) | | same |
| `walt.c walt` | same | tile | `walt_policy(inner_L0, n=WALT_OUTER=30)` | each hidden seat plays `inner_L0` | | same |
| Pinned Rust `Solver::pi(k, key, seat, hand, legal)` | modeled mind's information state | `Option<u8>` tile | bundle recursion | level-(k−1) minds; `Field::Dice` at the bottom; `n0 = 8` inner worlds, Voidless inner belief in the `baseline` profile | bundle-coupled | `best_of`: lowest tile on exact ties |
| Pinned Rust root (`evaluate_contract_with_cache` → `action_values`) | focal request | full vector `[(tile, k/N)]` plus `choice` | bundle recursion | `SeatLevels([0;4])` level-0 minds | bundle-coupled | `best_of` |

`engine42.Game.made()` is `bid_pts >= bid`: every recovered Python value is a
**declarer-make** probability; defenders are handled by the selector's min, not
by complementing the vector. `walt.c` relabels so the bidding team is the odd
seats and `value` returns declarer-make mass likewise.

**Known provenance gap.** The chat's later files (`rungs42.py`, `tables.py`,
`levers.py`, `tourney.py`, `l2ab.py`, `l2.py`) are not in the committed
inventory per the recovery memo; `rungs42.py` was reported deleted in the chat.
I tried the two names the memo mentions under
`experiments/adversarial-20261004/incoming/` (`batched.py`, `walt42x.py`) and
neither exists at those paths; I did not list that directory this turn (no
shell), so its inventory is unverified here. No evolved evaluator signature is
recovered beyond the `(vals, choice)` tuple above. Nothing below assumes one.

## 3. The replacement point versus the outer hint vector

Walt's structure, in both the standalone C port and the pinned Rust solver, is
two nested sampled computations with the same shape:

```
outer(seat, hand, public) :  sample N outer worlds (30 in walt.c, 40 in the pinned baseline)
    for each legal root tile:  value = own-committed recursion over the bundle,
        where every hidden seat's move in every world is   inner(seat_w, hand_w, public_w)
    select argmax/argmin, lowest tile on ties
inner(seat, hand, public) :  sample n inner worlds (8), others uniform random,
    own future committed per node; select the same way; RETURN ONLY THE TILE
```

- **Replacement point (Jason's intent):** `inner`, i.e. `walt.c inner_L0` and the
  pinned solver's level-0 modeled mind (`Field::Level(0)` with `n0 = 8`,
  reached through `Solver::pi(0, …)`). A tiny net standing in for `inner` is
  called once per (hidden world × hidden seat × node) inside the outer
  recursion; that is where the cost is (walt/MAP.md: classifying hands through
  the field is "99% of every bill").
- **Outer hint vector (HINT-TARGET.md):** the output of `outer` at the human's
  seat, displayed as `successes / N`. It is what a *later* rung would imitate
  if the net replaced `outer`; it is not the inner level's target.
- **Which consumer uses values versus choices.** In `walt.c value()` and in the
  Rust bucket recursion the upper level consumes **only the inner tile**
  (deals/worlds are grouped by the returned tile). Values are consumed only at
  the root selector and, in Plunge, by the hint display and the saved-hint
  replay. The partner-rollout review consumes its own rollout values, not the
  inner vector, and is excluded by the hint route anyway. So for the inner
  replacement the binding contract is **choice substitution under the normal
  selector**, with the full vector still produced because the sense interface
  and the next-rung display want it. Message 24's caveat stands: if a future
  consumer weights or tie-breaks on inner values, those values must carry the
  inner level's semantics, not the outer's.

## 4. Target semantics and version differences, named

| Name (proposed) | Own future | Others | Worlds | Values | Lawful | Where it exists |
|---|---|---|---|---|---|---|
| `V0` = the pilots' T0 | uniform random | uniform random, shared tape | 128 | k/128, paired per world | yes | `tiny-net-ladder-20261004/kernel.c` (the completed studies' teacher) |
| `L0MIND-n8` (walt.c `inner_L0`, Rust level-0 mind) | committed per public node over the bundle | uniform random per call (C) / `Field::Dice` (Rust) | 8 | k/8 | yes (no per-world own choice) but bundle-optimistic | `walt.c`, pinned Rust `pi(0)` |
| `OUTER-L1-n40` (hint) | bundle recursion | `L0MIND-n8` per hidden seat | 40 | k/40 | same caveat | pinned `baseline` wire |
| `NOFUSION-exact/tape` | information-set grouped fold | exact tail or tape | enumerated or W | mean over D | yes | `nofusion_sc.py` (sampler flagged for audit) |
| `EXACT-TAPE L1` | per-world best response | tape | W | mean of per-world extrema | **no** (fused) | `exact_tape.py`; not a teacher |

The completed studies distilled `V0`. Jason's replacement level is `L0MIND`.
They share the random-others assumption and differ in own-future treatment
(random versus committed) and world count (128 versus 8). Message 24's claim
that they rank alike is a historical statement, not a measurement here; the
first gate below measures it.

Two subtleties for the label record. First, the pinned `L0MIND` is a
**deterministic function of its seed** ("level-0 seeding bit-identical across
the stack"); a net cannot and should not reproduce seed noise, so the
distillation target is the expectation over seeds (`K`-seed reference), and
single-seed agreement is reported separately as the realistic replacement
test. Second, `k/8` labels are coarse (per-action standard error up to .18), so
teacher noise is a live factor at this level; the completed studies' noise
bound (128-world T0) does not transfer and is not claimed to.

## 5. What transfers from the completed diagnosis, and what does not

Established on `V0` targets: representation was the strongest tested factor;
source-deal coverage helped; capacity alone did not help at the prior data
scale under MSE selection and helped only with 16x data and regret selection.
Not established, and not claimed: that ordinary larger raw nets cannot help at
the inner level (they were never tested on `L0MIND` targets, and at 16x with
regret selection they did help on `V0`), that teacher noise is irrelevant
(8-world labels are far noisier than 128-world ones), or that the feature
scorer's advantage transfers. The self-audit used independent code paths, not an
independent reviewer. Accordingly the next experiment keeps **raw public/own-hand
inputs as the starting candidate** with an **ordinary larger-capacity control**;
the engineered per-action features are a diagnostic arm, not the architecture.

## 6. Next experiment: inner-level replacement, preregistered gates

Engine. `walt.c`'s `Policy` function pointer is the cheapest lawful seam: a
`net_policy(seat, hand, state, voids)` can be swapped for `inner_L0` without
touching the pinned Rust crate. Before any label, `walt.c` needs a bounded audit
(sampler legality against the independent referee, void handling, `decide`-mode
request boundary) and a `WALT_OUTER = 40` setting to match the pinned outer
count; the fixed-contract match heuristics are irrelevant to `decide`. The Rust
`Field` enum stays closed; porting there is a later step if the C result holds.

Positions. Fresh deals excluding every prior source; uniform-play histories; one
live nonforced root per band per deal for **every seat** (the inner mind is
called for all four seats); whole-deal splits including symmetries; a final
test block drawn up front and labeled only after freeze.

Labels (per root, all under ≤295 s caps in resumable batches):
- `L0MIND-n8` single seed: the exact inner call's vector and tile (what the
  outer actually consumes).
- `L0MIND-n8 × K=16` reference: K independent seeds, mean vector, per-seed
  choices (the distillation target and the tie/near-tie reference).
- `V0-128` on the same roots (cheap, from the existing kernel) to measure the
  rank agreement message 24 asserted.
Record raw counts, N, seed domain, legal set, chosen tile per seed.

Models. raw-h32 (prior tiny), raw-h256 and raw-d256 (ordinary capacity
controls), feat-s64 (diagnostic), matched update budgets, full legal-vector
targets. Target form and loss per DESIGN.md sections 1 and 5 are options:
because the consumer here uses choices only, **centered advantages are a
legitimate primary candidate for the inner replacement**, with direct Q or
baseline-plus-advantage as the arms that also serve the sense display; declare
one primary and the others as controls before labels. Selection by validation
vector error on the K-seed reference; regret curve reported.

Gates (all on fresh validation, then once on the frozen test):
- **G1 teacher sanity.** `L0MIND` single-seed choice agrees with its own
  K-seed majority on at least the fraction observed; report the fraction and
  the resolved-gap subset. If single-seed self-agreement is below about 70%,
  the inner level is itself noisy and the replacement bar is set against the
  K-seed majority, stated as such.
- **G2 inner fidelity.** Net choice agreement with the K-seed majority and with
  the single-seed call, overall and on resolved gaps (gap > 3 paired SE of the
  K-seed mean); vector error and tie recovery against the K-seed mean.
- **G3 nested replacement (the real test).** Run `outer` with `assume =
  inner_L0` and with `assume = net` on the **same outer worlds and seeds** for
  fresh roots; compare outer choices and outer vectors; measure paired outer
  regret of each against a high-budget outer reference (e.g. 512 outer worlds
  with `inner_L0`). Pass if the paired upper 95% bound of (net-inner minus
  L0-inner) outer regret is below a preregistered δ (proposed .01 in make
  probability) and outer choice agreement on resolved gaps is above a
  preregistered floor; fail otherwise. Small average inner vector error alone
  does not pass.
- **G4 cost.** Inner call latency net versus `inner_L0` (8-world recursion) on
  the same positions, warm, sequential; the replacement must be cheaper by a
  preregistered factor or the exercise is moot.
- **No rung** beyond this in the phase; a passing G3 returns to Jason with the
  choice of replacing `outer`'s own recursion next or distilling `outer` into
  the hint-vector model of HINT-TARGET.md.

Not authorized by this document: any run. It sets the contract and the gates.
