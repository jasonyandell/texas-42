# FH2-REPORT — the focal-horizon ladder, as built

**Slice:** FH2, the parent's §23 sound interruption, §19 preserved facts,
§24 gap measurements and §25 continuation substitution
(`walt/math/focal_horizon_sandwich_v0.1.md`, cited by title; the object
is the FOCAL-HORIZON HIERARCHY, FH-A2) as narrowed by the companion's
proofs P8/P11/P12 and rulings FH-A3, FH-A9, FH-A11 with Proposition
FH-int (`walt/CENSUS-RULINGS.md`), under `walt/briefs/BRIEF-FH2.md`.
Authorized 2026-09-04 by Jason ("take this to the finish line").
**Status: COMPLETE.** New module `solver/focal_ladder.rs`, two new
`focalreport` modes (`ladder`, `ladder-record`), nine gates (ten after the post-audit fix FH5 added FH-C, the in-pass cap-refusal gate — audit N3) in
`walt/walt/tests/solver_focal_ladder.rs` (24 s release standalone),
record `walt/probes/factor_belief/focal_ladder_run1.txt` (h8-t4 and
h3-t4, receipt contract, σ0 tail, pinned schedules). One seam opened in
FH1's engine (`Engine::price_frontier`, `pub(crate)`); FH1's ten gates
green and unweakened. Freeze 58 untouched; `ingest/` untouched; the math
parent, verifier and companion untouched.

**EXPLORATORY tier throughout.** Every number below is exploratory; the
live default player is untouched; no arena claim (FH-A10).

---

## What this means and what it cost

walt's focal-horizon engine can now be STOPPED and RESUMED. A root is no
longer priced all-or-nothing: a pass at horizon `k` runs under a read
ceiling, stops deterministically when it reaches it, keeps every fact it
completed, and reports an honest root view — intervals where they exist,
NO interval and NO regret where only the placeholder does, `Settled`
only when every competing upper is a real fact. Resuming the same
horizon later lands on exactly the uninterrupted result, byte for byte on
the fact set. The decision-relevant readings: at h3-t4 an INTERRUPTED
k = 1 pass at 1.20M reads already settles 3-1 (Γ 12‰) because the
unfinished action's retained k = 0 upper sits below the new bar — FH1
needed 2.63M reads for that verdict; at h8-t4 the interrupted k = 1 pass
at 250k reads keeps 5-5 at its k = 0 interval and the resume (110k more)
reproduces FH1's k = 1 view exactly. Exact suffix reuse is a cost story:
the k = 2 pass falls from 2.83M to 0.42M reads at h3-t4 and from 0.66M to
0.13M at h8-t4 with identical views. The cost trends, up front: the
ladder is BIGGER than FH1's engine — peak RSS at h3-t4 is 662 MB with
the memo and 509 MB without, against 411 MB for FH1's k = 2 run (the
per-node policy tables the fact store must keep, and the memo's belief
clones); a resume re-reads the stopped ancestor chain (h8-t4: under 0.4% over
the uninterrupted pass); the gate file adds 24 s standalone (32.7 s
inside the concurrent gate, which passed with 122 binaries, 0 failed,
sum of suite walls 1,367 s against CI1's 1,317 s); the record runs in
under 10 s. Nothing got slower per read.

---

## THE CHANGE IN ONE PARAGRAPH

FH1's `focal_horizon` returns a whole root or refuses it. The ladder
keeps a per-root STORE OF NODE FACTS under FH1's identity minus the
horizon — `[L(C), U(C)]` in mass form, the policy attaining `L(C)`
stored beside it, and the residual horizon each side was established at
— and runs the same recursion as PASSES under a work budget. A fact is
written only when a node's whole subtree completed at its residual
horizon, by INTERSECTION with whatever the node already held (lower =
max, the winning policy travelling with it and the PRIOR winning ties;
upper = min); nothing partial is ever written, so the fact set is a
function of the set of completed nodes — the derived-view law behind
"resume ≡ uninterrupted". The root is a derived view of that store. A
collapsed fact becomes a receipt in a suffix memo keyed by the belief's
full identity, and later passes return the receipt instead of
descending. A `ProofProducer` turns the root view into `Fact::Bound`s
with checked executability.

## WHAT WAS BUILT

- **`FocalLadder`** (`solver/focal_ladder.rs`, a new module — FH1's
  engine stays as it is; the ladder composes its frontier pricing rather
  than rewriting it): `open(ctx)`, `advance(ctx, k, budget, memo) ->
  Outcome`, `root_view()`, `render()`, `fact_at(history)`, `facts()`.
  `LadderIdentity` = (root id, field id, contract, tail id); every
  `advance` asserts the context matches.
- **`NodeFact`** — `lower_mass`, `lower_horizon`, `policy` (the
  `FocalChoices`-shaped sub-table over the node's subtree, tail
  elsewhere), `upper_mass`, `upper_horizon`, `completed_at`. A node
  never priced has no entry: its view is the placeholder (lower 0 with
  the tail, NO upper). Decided nodes store the §5 arithmetic as a fact at
  zero reads — the already-made h10-t6 root reports `Equivalent` at `Z`
  under a zero ceiling, which is a fact, not a placeholder.
- **The pass.** Walking a node at residual horizon `j`: decided → the
  arithmetic; `completed_at ≥ j` → the stored fact, read-free (the
  resume rule); a receipt under the full identity → returned; reads at
  the ceiling → `Stop`, deterministic; hidden → Σ over every positive-
  mass branch; focal `j = 0` → `Engine::price_frontier` (FH1's leaf, one
  implementation) or the typed cap refusal, which leaves the enclosing
  ROOT CHILD unfinished and lets the pass continue at the next root
  child (FH-A3); focal `j ≥ 1` → max over every legal action, argmax
  under the lowest-tile rule. A stop unwinds through the ancestors,
  listing every node whose parent was entered and which did not complete
  in the **residual frontier** — typed `Stopped` / `Enclosing` /
  `Unvisited` / `Unaffordable`, with its mass and its retained fact.
- **`Outcome`**: `Completed { report }` or `Interrupted { report,
  residual_frontier, stopping_node, unaffordable }`; `PassReport` holds
  the derived view after the pass, the exact `FocalSpend`, reads spent,
  the ceiling, suffix hits/lookups and the fact-store movement (new,
  revisited, tightened, root children completed).
- **`LadderView`** (derived, never stored): per action lower / upper
  (`Option`) with horizons and `interval() -> Option<ActionInterval>`;
  bar and bar action; survivors (an absent upper survives); verdict
  (`Settled` iff every other action has a real upper below the bar
  action's lower; `Equivalent` by FH-tie; else `Unresolved`); the policy
  as the union of the children's stored tables plus the root choice;
  `L_exec = bar`; `U*` and `Γ` only when every upper exists.
- **`SuffixMemo`**: receipts bucketed by post-root history, matched by
  `FactorBelief`'s componentwise equality; `lookup`, `insert`, `holds`,
  `freeze` (answers lookups, takes no receipt — for consulting another
  identity's receipts); hit/lookup/receipt counts and the first hit's
  history. Memoized: priced collapses only (decided facts and returned
  receipts are not re-inserted).
- **`FocalHorizonProducer: ProofProducer`** — per action a lower under
  `focal-horizon:<tail id>:k=<k>:lower` (executable iff `reprice(a)`,
  the stored policy through `viewer_success_mass`, equals the value —
  computed at production, asserted never below it) and an upper under
  `focal-horizon:god:k=<k>:upper` (never executable); `k` is the horizon
  the winning side was established at, so an interrupted pass emits the
  retained `k=0` beside the others' `k=1`. Zero-valued placeholders are
  not emitted.
- **FH1 seam**: `Engine`, its fields, `price_frontier`, `reads()`,
  `viewer_legal`, `history_key`, `ratio` and `FocalChoices::new` became
  `pub(crate)`; FH1's `walk` now calls `price_frontier` for its `k = 0`
  leaf. No behaviour change (FH1's ten gates green, 135.6 s).
- **Probe**: `focalreport ladder <hand> <trick> <contract|receipt>
  [nomemo] <k:ceiling>...` and `focalreport ladder-record <out.txt>`;
  per step: outcome, reads (field + tail), residual frontier size / mass
  / causes / unfinished root children, fact-store movement, suffix
  hits/lookups/receipts and the first hit, the derived root view.

## THE GATES (`solver_focal_ladder.rs`, 9, 24 s release wall standalone)

One `LazyLock` fixture: FH1 engines and exact `Q_a` at h8-t4 and h3-t4;
the sequential ladder k = 0, 1, 2 memo off (the reference) and memo on;
the h8-t4 interrupted-and-resumed pair; the pinned eight-step schedule.

- **LP** parity with FH1: a fresh ladder walked directly at `k`
  reproduces `focal_horizon`'s every value view AND its `π_k` content
  address (h8-t4 k = 1, 2; h3-t4 k = 1); the sequential ladder equals
  FH1 on every value view at every k on both roots, zero memo lookups.
- **FH7** at h8-t4, k = 0 uncapped then k = 1 at ceiling 250,000: (1)
  residual frontier ∪ completed children = every root child, exactly one
  `Stopped` node, no cap refusal; (2) the unfinished child's retained
  fact equals FH1's k = 0 interval, `completed_at = 0`, and the view
  carries it while completed children carry their k = 1 intervals; (3)
  `L ≤ Q_a ≤ U` for every action; (4) the report names ceiling, reads
  spent (≥ ceiling, overshoot under one frontier evaluation), and the
  stopping node, which is below a root child listed as `Enclosing`; (5)
  resume + completion renders byte-for-byte equal to a fresh
  uninterrupted k = 0, 1 ladder and equals it on every derived view
  including the choice table; the spend as a sum is ≥ the uninterrupted
  pass and the excess is a strict fraction of the interrupted pass.
- **FH7b** the pinned schedule `(0,50k) (0,120k) (0,∞) (1,100k)
  (1,250k) (1,∞) (2,60k) (2,∞)` with the memo on: no lower falls, no
  upper rises, no upper is ever discarded, `Γ` never rises and never
  becomes absent again, survivors only shrink; ends `Settled{3-3}` at
  `Γ = 0`; strict rise and fall seen.
- **FH7c** h4-t6 and h8-t4 at ceiling 0: `Interrupted`, zero reads,
  empty store, no interval and no regret anywhere, `Unresolved` over
  every legal action, every residual node placeholder; the uncapped pass
  then completes with intervals and a regret. Contrast: h10-t6 (decided
  after every action) completes at ceiling 0 with collapsed facts and
  `Γ = 0`.
- **PS1** the interrupted ladder's facts install into `ProofState::open`
  with zero rejections (one lower and one upper per action); closure
  survivors, bar, `B_exec` witness, `U*` and `Γ` equal the ladder's;
  producing again from the resumed ladder (k = 1) and the sequential
  ladder (k = 2) only tightens and ends `Settled{3-3}` at `Γ = 0`.
- **PS2** no upper is executable; every lower is executable and its
  stored policy re-prices to its value; the retained `k=0` lower of the
  interrupted pass is present with an empty table and re-prices to the
  tail's own `viewer_success_mass`.
- **SR1** memo on vs off, h8-t4 and h3-t4, k = 0..2: identical action
  views, bar, survivors, verdict, `U*`, `Γ`, choice table and policy id
  at every k; zero hits at k = 0, hits > 0 after; the memo saves reads
  at k ≥ 1; the memo-on store is a proper subset of the memo-off store,
  identical wherever the memo-on walk composed or hit a node, sound and
  no tighter elsewhere; the first hit PINNED at `[2-1 0-0 2-0 3-0]`.
- **SR2** a contract-30 memo, frozen, consulted by a contract-36 ladder
  at the same root over > 1,000 shared histories: zero hits; the pinned
  node's belief is a receipt, the same belief narrowed in one hidden
  seat's factor (`with_factor_table`) misses, and the contract-36 belief
  at that history misses.
- **FH-D** a fresh k = 0 + interrupted k = 1 run equals the fixture's
  outcomes (frontier included), render and ladder.

## THE RECORD (`focal_ladder_run1.txt`, receipt contract 30, σ0 tail, memo on)

| root | step | outcome | survivors | verdict | bar ‰ | Γ ‰ | reads | hits | facts |
|---|---|---|---|---|---:|---:|---:|---:|---:|
| h8-t4 | 0:150000 | interrupted | all four | UNRESOLVED | 850 | — | 150,000 | 0 | 1,596 |
| | 0:inf | completed | all four | UNRESOLVED | 885 | 100 | 182,931 | 0 | 2,432 |
| | 1:250000 | interrupted | 2-1 3-3 5-5 | UNRESOLVED | 932 | 39 | 250,046 | 570 | 10,760 |
| | 1:inf | completed | 2-1 3-3 5-5 | UNRESOLVED | 932 | 39 | 109,854 | 54 | 15,859 |
| | 2:inf | completed | 3-3 | SETTLED 3-3 | 969 | 0 | 125,666 | 1,353 | 19,369 |
| h3-t4 | 0:800000 | interrupted | all four | UNRESOLVED | 280 | — | 800,252 | 0 | 6,991 |
| | 0:inf | completed | 3-1 4-4 6-4 | UNRESOLVED | 288 | 76 | 980,643 | 0 | 14,511 |
| | 1:1200000 | interrupted | 3-1 | SETTLED 3-1 | 338 | 12 | 1,200,264 | 3,037 | 70,145 |
| | 1:inf | completed | 3-1 | SETTLED 3-1 | 338 | 12 | 273,498 | 534 | 81,770 |
| | 2:inf | completed | 3-1 | SETTLED 3-1 | 350 | 0 | 421,175 | 5,272 | 89,923 |

1. **An interrupted pass can settle.** h3-t4 k = 1 at 1.2M reads: 3-1,
   4-1, 4-4 complete, 6-4 keeps its k = 0 fact (`U_0 = 328‰`), and the
   derived view is `SETTLED 3-1` — the retained upper is what excludes
   it. §23's promise, live.
2. **Retained facts are the uncapped run's, to the mass.** h8-t4 k = 1
   at 250k: 5-5 stays at `[487/600, 289/300]` from k = 0; the resume
   spends 110k and reproduces FH1's `[137/150, 283/300]`.
3. **The memo is invisible in value and large in reads.** h8-t4: k = 1
   0.36M (FH1 0.58M), k = 2 0.13M (FH1 0.66M); h3-t4: k = 1 1.47M
   (FH1 2.63M), k = 2 0.42M (FH1 2.83M). Memo off, the ladder's reads
   per horizon equal FH1's exactly.
4. **Where the reads went at k = 2.** h3-t4: 16,351 lookups, 5,272 hits;
   88,331 of 89,923 facts collapsed; tail evaluations 3,398, every one
   forced (FH-last, seen again through the ladder).
5. **Resume overhead.** h8-t4 k = 1: 250,046 + 109,854 = 359,900 reads
   against 358,691 uninterrupted (memo on), and 250,027 + 330,537 =
   580,564 against 578,989 with the memo off — the stopped chain's
   `branch_masses` are read again on re-entry; under 0.4% either way.

## DEVIATIONS AND JUDGMENT CALLS

- **Unfinished interior nodes keep their PRIOR fact; nothing partial is
  composed into the store.** The brief's composition rule (max / Σ over
  every action / branch with placeholders) is how a COMPLETED node's fact
  is built from its children's facts, and how the root view is built
  from the root children's facts; an unfinished node's view is its
  retained fact. This is the reading under which FH7's "unfinished
  children carry the k = 0 facts (equal to the uncapped run's)" holds
  (a partial composition could exceed `L_0`), under which resume ≡
  uninterrupted holds bytewise with a prior-wins-ties intersection, and
  under which every emitted lower re-prices EXACTLY to its value.
- **The prior wins ties** on the lower side (a policy is replaced only
  by a strict improvement). Consequence: the sequential ladder's `π_k`
  can differ from FH1's `π_k` on a tie while attaining the same value —
  LP checks the id only for a fresh direct-`k` ladder, values for the
  sequential one. Without this rule the memo would be visible in the
  choice tables.
- **The spend "as a sum"** is gated as `interrupted + resumed ≥
  uninterrupted` with the excess a strict fraction of the interrupted
  pass; equality is not claimed (re-entry re-reads the stopped chain).
- **A cap refusal does not stop the pass**: it leaves the enclosing root
  child unfinished and the walk continues at the next root child
  (FH-A3's wording); the `Interrupted` outcome lists every refusal.
  `stopping_node` is `None` when only cap refusals occurred.
- **Decided nodes are stored facts** (not in the brief): without them a
  root decided after an action showed a placeholder where FH1 shows
  `[u·Z, u·Z]`.
- **Memory** is the one thing that got bigger (top section). A
  version-referenced policy store would remove the per-node table
  duplication; not built here.

## FOR FH3

- The ladder's types: `FocalLadder` (open / advance / root_view /
  render / fact_at), `WorkBudget { read_ceiling, node_fiber_cap }`,
  `Outcome` / `PassReport` / `ResidualNode` / `ResidualCause`,
  `LadderView` / `LadderAction` (uppers are `Option`), `NodeFact`,
  `SuffixMemo` / `SuffixReceipt`, `FocalHorizonProducer`.
- Horizons need not be ascending; a pass at any `k` intersects. A
  ceiling of `u64::MAX` is "no ceiling". `render()` is the §67.5
  comparison object (facts only, history-sorted, tables by digest).
- The memo is per-identity by construction but is an external object:
  pass one across ladders only if the beliefs can coincide (same root,
  contract, field); `freeze` it to consult without polluting.
- Costs to plan around: the fact store at h4-t4 will be several times
  h3-t4's 89,923 facts; the memo clones one `FactorBelief` per receipt.
