# The program on one page

**For Jason, to hold in his head.** First written 2026-10-08; rewritten at
every landing — whichever session lands a result owns the rewrite, same
discipline as `walt/MAP.md`. Ten minutes to reread. EXPLORATORY tier;
nothing is promoted by being here.

**The rule that keeps this page alive: a result is not landed until its one
line is here — claim, cost, pointer, and what it kills.** Everything else
(wiki, briefs, receipts, logs, 40+ worktrees) is warehouse: trustable,
append-only, not required reading. The research wiki is retired as the
index (lesson, 2026-10: useful early, does not scale to this volume);
`wiki/Home.md` remains the warehouse's own map, and this page is the front
door. `kanban/` is the queue.

## The program

Fast lawful play from play 1 of 42, with beliefs factored out — so the next
twenty years of studying 42 are spent on beliefs and dynamics, not search
plumbing. The instrument is **Walt** (issue #99; card copied at plunge
`lab/WALT-CARD.md`): a ladder of best responses bottoming out at random,
under one law — **a decision reads only its decider's own holding, the
public record, and deal-independent noise**. The objective is pmake (ruled
2026-08-17), never a trick-difference proxy. 42 is the base platform;
bridge (plunge repo) is the harder stress lab; the current phase is
digesting *why* Walt works and walking the ladder with learned inner minds.

**It all starts with the math.** Walt came out of the math program, and so
did rob — rob is "rec's mathematics under v0.7's type discipline," an
executable mathematical specification with proof receipts. The engineering
below is the math executed, never the other way round.

## The math — the generative layer

`walt/math/` is the program's source of law, run under its own discipline:
a **parent** is a frozen design document, pinned by SHA-256 beside it and
never edited; it enters through an **intake companion** and adversarial
review; questions go to walt-math as question→ruling pairs; several parents
carry machine checks (`verify_*.py`); errata and second audits are filed
beside their parents. The standing ruling "**no new parent until the
consolidation slice lands**" is a ruling about these documents. Several
parents extend *The Mathematics of Walt v0.1*, the working framework.

The load-bearing parents and rulings:

- `unified_information_geometry_v0.4` (frozen) with
  `equivariant_lumpability_v0.5` (§12.6A) — together "the law" that
  `walt/CENSUS-RULINGS.md` binds the census against.
- `WALT-MATH-RULING-2026-08-17` (pmake and the walk to trick 1) — where
  pmake-as-objective was ruled.
- `focal_horizon_sandwich_v0.1` — FH-A1..A11, Theorems 1–6 proved in full;
  the FH hierarchy (result 2) is this document executed.
- `counted_belief_sandwich_v0.1` — the refinement calculus from sampled
  orientation to factorized exact best response (results 1 and 4 trace
  here).
- `calculated_evidence_v0.1` — anytime-valid adaptive settlement; "the end
  of magic sample counts."
- The rest of the bench: `model_belief_base_player`, `salvation_complex`,
  `signed_pivotal_geometry`, `targeted_level2_field_stability`,
  `anytime_proof_state_score`, `decision_sparse_exact_solving` (+ errata
  and second audit), `predictive_algebra_v0.6`, and `SUIT_ALGEBRA_PURE`
  (the suit structure stated for its own sake).

## Standing results

| # | result | the numbers | where |
|---|---|---|---|
| 1 | Exact trick-1 belief is countable: 399M worlds as 116,280 acting-seat hands × exact-cover counts; posterior = one factor per observed play | counting ≈ ms; classifying hands through the field σ is 99% of every bill | `walt/MAP.md` (objects 1–3) |
| 2 | Focal-horizon hierarchy: [L_k, U_k] per root action with certified regret Γ, collapsing to the exact solve | every live trick-4 root settles by k ≤ 2; the trick-3 anchor needs k = 3; residual width is the tail's policy gap, not fusion price; 19 GB at h8-t3 | `walt/MAP.md`, `walt/briefs/FH*-REPORT.md`, branch `walt-fh` |
| 3 | Ladder height, dice-bottom: L2 vs L1 tied 14/14/72 on 100 mirrored deals at ~5× cost | plunge `lab/WALT-CARD.md` |
| 4 | Response-ladder controller: anytime total executable policy from trick 1, exact priced value, valid remaining regret bound, resumable, JSON worker interface | `experiments/response-ladder/README.md`, branch `codex/walt-response-ladder` |
| 5 | Distillation: raw-encoding tiny nets do NOT generalize (held-out R² ≈ 1%); a 6,337-param **lawful-feature scorer** retains 77% (d1) → 88% (d16) of the random→teacher regret reduction (.0095 vs random .0543 / teacher .0032) | blocker ranking: representation > coverage > selection objective > capacity | `~/Documents/Codex/2026-10-04/task-15/fable-worktree`, branch `experiment/fable-tiny-net-ladder`, `experiments/fable-tiny-net-20261004/REPORT.md` |
| 6 | Walking the ladder with the scorer as inner mind (LAD1–6): +.058 h2h at checkpoint k35 (seed 6000), +.101 inside l2n:160, regret .0107. A BCE-calibrated net is −.04 as a *player* and +.090 as the *inner model* — different jobs. Sharper inner players win (argmax +.090 vs τ=.05 sampling +.055) | same worktree, `experiments/fable-rust-ladder-20261004/` |
| 7 | Sampling budget by trick: 160 worlds gives up ~.001–.002/decision, all of it in tricks 0–3; enumeration ≤ 400 supports is free and exact at tricks 4–5 | rust-ladder commit `3ad5a9ea3` |
| 8 | Bridge at full depth, lawfully, at playable speed: 0.8 s/move mean; +0.60 vs rule bot, +0.05 vs the shipped horizon Walt, −0.233 vs PIMC; binding constraint measured = inner-mind rung height (n0), not sampling (n, draws scaling do nothing) | plunge branch `walt-bridge-deep`, `lab/bridge/REPORT.md` |
| 9 | Stochastic similarity (bridge): cheap lawful features sense a position's *intensity* (pmake MSE .091→.050, calibrated) but not its *shape* (within-position delta R² .01–.05); every learned prior is dominated by sampling — an n8/n04 probe is 2% of full cost and holds the true best card in its top-2 at 80%; probe→refine recovers 85–90% of skip-regret at ½–⅔ cost | plunge `lab/bridge/similarity/REPORT.md`, same branch |
| 10 | Gran probes (G1/G2): the 6-4 was forced; saturation and the 6-4 mattering are mutually exclusive; trick 5 flips L1 hold to L2 release — the release margin moves Jason's way | branch `walt-g1-l2` (worktree `texas-42-g1l2`) |
| 11 | Sunshine Atlas: descriptive terrain of 36,000 actual games; six NMF components with split-half cosines .9937–.9993 (descriptive stability, not discovered threat types) | `experiments/kiln/ATLAS.md`; live phone viewer |
| 12 | Portable CPU speedups landed with shipped-WASM parity and hosted release verification | `walt/CPU-SPEEDUPS.md`, `walt/CPU-PHONE-RELEASE.md` |

## Rulings and dead branches — read before spending

- **No value compression at the front of the game.** Proved, not just hard:
  at candidate scale the only lumpable skeletons were world-reconstructing;
  retrograde quotients compress trick 6 (2.1:1) but weaken toward the front
  (1.25:1 by trick 5), nothing at the trick-1 interface. "Compression is
  bought with deadness, and nothing is dead at the first play."
  (`walt/LOG.md` compression era S5e–S5k; `wiki/walt-negative-results.md`;
  the compiler half is parked, not queued: `kanban/backlog/scheme-compact-compiler.md`.)
- **Strategy fusion is the one sin.** The information rule is the whole
  anti-fusion content; census pooling is bound by `walt/CENSUS-RULINGS.md`.
- **pmake is the objective** (2026-08-17). Margin tie-breaks are also
  incompatible with full depth (bridge: >50× cost; binary payoff deep).
- **Raw-encoding nets: dead** (result 5). Lawful features or nothing.
- **kNN / cheap-feature priors for card choice: dead** (result 9) —
  dominated at every cost point by the rule bot or a tiny sampling probe.
  The "stochastic similarity" framing is retired.
- **Value transposition tables under the tape: net loss** (bridge; the tape
  makes identical recurrences rare; opt-in only).
- **Model-belief Ξ (the field as hidden state): affordable at trick 4,
  refused at trick 3** (MB1) — the wall is real.
- **No new mathematical parent until a consolidation slice lands**
  (Jason, 2026-09-04; the slice is `kanban/backlog/consolidation-slice.md`).
- **Research wiki as index: retired** (2026-10). Landings update maps;
  the wiki is warehouse.
- **"Walt as a sense / jellyfish": retired as framing** (woo). The residue
  worth keeping is operational: *the thing worth learning is never the
  player — it is the cheap inner mind the search models others with*
  (results 5, 6, 8, 9 all say this).

## The forge (tools that exist and run)

- **`walt/scheme/` — Scheme/Fix, "express the game":** executable
  relational language over exact worlds (`walt::scheme` in the unified
  crate): typed dynamics, exact belief pushforward, executable
  information-state policies, donor composition, relational learning.
  "Invented to compress; commissioned here to express."
- **The gym:** `experiments/full-game-speed/` — native/PGO/WASM/gpu-replay
  speed program with its own GOAL/PROGRESS.
- **response-player:** complete games from trick 1 under partnership time
  banks, plus a line-oriented JSON worker (`experiments/response-ladder/`).
- **rob:** the exact perfect-information engine, byte-diffed receipts.
- **Bridge lab (plunge, branch `walt-bridge-deep`):** `tape.js` full-depth
  engine; `collect/h2h/replay/similarity` harnesses; pmake corpora under
  `lab/bridge/results/pmake/`; DDS oracle via endplay venv.
- **Kiln:** campaign mining → Atlas snapshots (`experiments/kiln/`).
- **Lean side project:** `lean/` (zora worktree carries WIP).
- Exchange channel (`exchange/`), automation harvesters (`automation/`),
  `kanban/` queue. GPU-native trick 1 is a frozen side track (M3 gate, no
  result).

## The live frontier

1. **Port the LAD recipe to bridge:** train the lawful-feature scorer on
   the bridge pmake corpus and install it as `tape.js`'s inner mind in
   place of flat L0 — a direct shot at the −0.233 vs PIMC, and the test of
   whether "representation > capacity" is a 42 fact or a Walt fact.
2. **How high does the ladder pay?** Dice-bottom L2 ≈ L1 at 5× (result 3),
   but a *learned* rung is worth +.058 and climbing (result 6). Where does
   it flatten, and at what cost per rung?
3. **Better lawful tails:** at k = 1 the certified residual is the tail's
   policy gap, not fusion price (result 2) — a better tail buys more than a
   deeper search.
4. **The consolidation slice** (ruled, pending): tree-shake
   godgap/horizon/extraction/refine into the one FH recursion.
