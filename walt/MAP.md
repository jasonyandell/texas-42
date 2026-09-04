# walt on one page — what exists, what it costs, what is now redundant

**For Jason, to hold in his head.** Written 2026-09-04; rewritten at
every landing (the orchestrating session owns it). Ten minutes to
reread. Everything here is EXPLORATORY tier; nothing is promoted by
being on this page. The record is `FACTOR-BELIEF.md`; the rulings are
`CENSUS-RULINGS.md`; the map of the wiki is `wiki/Home.md`. This page
is none of those — it is the shape, with numbers, and the trend.

## The one question

Given my seat's information, which play maximizes the chance of making
the bid? `Q(B) = max` over LAWFUL policies (the max is taken after
hidden worlds with the same public history are merged — reversing that
order is strategy fusion, the thing 42 players punish) against a
declared field σ, the model of the other three seats. The objective is
`pmake` (ruled 2026-08-17); a trick-difference proxy is never the
target.

## The objects that exist, and what each costs

| # | object | where | what it is | cost, honestly |
|---|---|---|---|---|
| 1 | counted belief | `solver/factor_belief.rs` | the 399,072,960 trick-1 worlds as 116,280 acting-seat hands × exact-cover counts; posterior = one seat's factor changes per observed play | counting ≈ ms; classifying each hand through σ is 99% of every bill |
| 2 | the field σ0 | `solver/field.rs` | the level-0 modeled mind for the other three seats; deterministic; reads the bid and the FULL public record | one read ≈ µs; every recursion's cost = its read count. Cross-history cache reuse measured 0 because the record is in the key |
| 3 | exact response | `response_success_mass` | `Q` exactly at a root; the oracle every gate checks against | h8-t3 289M reads ≈ 14 min (`horizon_run1.txt`); at trick-4 roots the same tree as the FH1 engine at k = 2 (0.66M reads h8-t4, 10M h4-t4, `focal_run0.txt` — the engine's count, not a separate measurement); trick 1 unreachable by this path |
| 4 | fixed-policy value | `viewer_success_mass` | `V^π` of one lawful policy π (a "tail") | the cheap one: hidden branching only |
| 5 | God upper | `doom.rs`, `godgap.rs`, `horizon.rs` | per-world clairvoyant make check; `1 − doomed/Z` bounds `Q` from above | one line walk per world of the node |
| 6 | proof state | `proof_state.rs` + frontier/refine/opening/residual/covers/laydown/extraction | append-only facts; survivors, verdict and certified regret `Γ = U* − B_exec` are DERIVED from them | container, not compute |
| 7 | model belief Ξ = Ω×Θ | `model_belief.rs`, `model_recursion.rs` | the field itself as hidden state (types); fusion price strictly positive at trick 4 (MB1) | the wall: affordable at t4, refused at t3 |
| 8 | unified player | `unified.rs` | one decision core over every instrument: decided → endgame exact → mixture → certified regret → σ0 fallback; posterior carried lazily | lean rung 11.6 ms for 72 decisions; ample rung 20 s |
| 9 | **focal-horizon hierarchy** | `focal_horizon.rs` (FH1, 2026-09-04) | `[L_k, U_k]` per root action, `k` = focal decisions made exact; `k = 0` is (4) below and (5) above; collapses to (3) at `k = 6 − T` because trick 7 is forced | k = 0 at h4-t4 5.1M reads; k = 2 = the exact solve |
| 9b | focal-horizon ladder | `focal_ladder.rs` (FH2, 2026-09-04) | the same recursion as budgeted PASSES over a store of node facts: stop at a read ceiling, resume to the identical result; exact suffix reuse | reads at k = 2 fall 2.83M → 0.42M (h3-t4) with reuse; memory GREW: 662 MB peak vs 411 MB for the direct engine (per-node policy tables) |
| 10 | the live default player | `CONTROLLER-PLAYER.md` | the pre-program player people actually play | untouched by everything above (FH-A10) |

Side tracks, not in the critical path: GPU-native trick 1 (M0–M2 parity
gated, M3 gate frozen, no result), the Lean side project, rob (the
exact perfect-information engine with byte-diffed receipts), the Pro
exchange channel.

## What the hierarchy makes redundant (the tree-shake list)

Object 9 was built to be the one object. In its vocabulary: the God-gap
census (U0) is `U_{a,0}`; the in-solve ply-cut census (U0b) is
`U_{a,m−1}` on viewer-lead roots (Proposition FH-cut); the salvation-mask
upper (queued U1, never built) is `U_{a,1}` (Theorem 5); rollout
improvement is `L_k`; argmax extraction is `π_k`; the exact endpoint is
the collapse. So `godgap.rs` (933 lines), `horizon.rs` (635) and
`extraction.rs` (135) are now measurement scaffolding around one
recursion, and `refine.rs` (917, frozen as freeze 58) was already
declared removable. `doom.rs` stays: it is the God tail's engine. That
is the consolidation slice, after FH3 lands.

## What FH3 measured (2026-09-04, the report of record `focal_run1.txt`, 33 coordinates × k ≤ 3)

- **Every live trick-4 coordinate settles by k ≤ 2** (5 at k = 0, 6 at
  k = 1, 3 at k = 2 across contracts 30–42); `Γ_1 ≤ 45‰` everywhere.
  Trick 5/6 roots settle at k = 0 or give exact tie sets by k ≤ 1.
- **The trick-3 anchor h8-t3 settles only at k = 3** (the collapse):
  survivors 5 / 5 / 3 / 1, `Γ` 141 / 100 / 34 / 0‰. `π_1` already
  plays the exact action 1-1, uncertified. At k = 2 the residual width
  is again the tail's policy gap (Q − L 13–34‰ vs U − Q 1–3‰).
- **The ply-cut flips of U0b live entirely on the upper side**: a cut's
  argmax is `argmax U_{a,m−1}`, which the hierarchy carries as an upper
  and never as a verdict. No wrong action is ever certified.
- **Cost, stated as findings:** reads per horizon unchanged from FH1;
  memory grew again — the fact store holds 3.82M facts at h8-t3, peak
  RSS 19.4 GB for the record and 17.8 GB for the gate (five concurrent
  h4-t4 ladders); gate wall 230 → 308 s. A warm σ0 instance runs a pass
  15× faster at identical reads — the field cache is the lever.

## What FH1 measured (2026-09-04, receipt contract, σ0 tail)

- **k = 0 settles h4-t4 with no search**; k = 1 settles h3-t4; h8-t4
  needs k = 2 (survivors 4 → 3 → 1).
- **At k = 1 the remaining width is the tail's policy gap, not fusion
  price**: `Q − L` is 9–41‰ per action, `U − Q` is 0–3‰. A better
  lawful tail buys more than a deeper search on this corpus.
- The certified regret contains the true regret at a live specimen
  where the bar's argmax (2-1) is not the exact argmax (3-3): Γ 39‰ ⊇
  13‰. Scalar closeness is not decision safety — now a measured fact.

## The costs to watch (the trend, stated as a finding)

| what | now | direction |
|---|---|---|
| reads per trick-4 decision, exact | 0.7M–10M | flat since Slice G; the ceiling is σ0 |
| trick-3 exact | 289M reads, 14 min | one root; the wall |
| `solver/` size | 37,260 lines, 36 modules (after FH2) | growing one module per intake; consolidation not yet started |
| peak memory, ladder | 662 MB at h3-t4; **19.4 GB at h8-t3**; the gate 17.8 GB | the cost that is growing fastest; a version-referenced policy store is the fix, not built ([[ladder-policy-store]]) |
| gate wall (`check.sh`) | 308 s with FH3's anchors gate (230 s before it; ~15 min serial on 2026-09-04 morning) | fixed by concurrency + fixtures; corpus trimming still owed ([[gate-corpus-trim]]) |

## Next, in order

FH2 ✓ (dc515ac) → FH3 ✓ (fc171e1) → audit ✓ (8aae7c7: one BLOCK, vocabulary, fixed; 13 NOTEs, 5 fixed, rest carded) → one PR
→ the σ0 read-key study (does its answer depend on the full record? if
not, the cache key coarsens and every recursion gets 10–100× cheaper)
→ the consolidation slice. **No new mathematical parent until the
consolidation lands** (Jason, 2026-09-04: "follow through on what we
have, then invest in a simplification/unification attempt").
