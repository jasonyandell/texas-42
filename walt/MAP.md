# walt on one page — what exists, what it costs, what is now redundant

**For Jason, to hold in his head.** Written 2026-09-04; rewritten at
every landing (the orchestrating session owns it) — this rewrite
2026-09-13, repository state as of 2026-09-07 (`c00717d1`), counts
re-measured 2026-09-13. Ten minutes to reread. Everything here is
EXPLORATORY tier; nothing is promoted by being on this page. The record
is `FACTOR-BELIEF.md`; the rulings are `CENSUS-RULINGS.md`; the binding
briefs and reports of record are `briefs/BRIEF-*.md` / `briefs/*-REPORT.md`;
the map of everything — the book — is `wiki/Home.md` (walt's part opens
at `wiki/walt.md`; what this page summarizes is owned there by
`wiki/walt-focal-horizon-era.md`, `wiki/walt-gran-anchors.md`,
`wiki/walt-partnership-program.md`, `wiki/walt-gym.md`,
`wiki/walt-scheme-fix.md`, `wiki/walt-seat-play.md` and
`wiki/walt-instruments.md`). This page is none of those — it is the
shape, with numbers, and the trend.

## The one question

Given my seat's information, which play maximizes the chance of making
the bid? `Q(B) = max` over LAWFUL policies (the max is taken after
hidden worlds with the same public history are merged — reversing that
order is strategy fusion, the thing 42 players punish) against a
declared field σ, the model of the other three seats. The objective is
`pmake` (ruled 2026-08-17); a trick-difference proxy is never the
target.

## The objects that exist, and what each costs

Two stacks share the one crate (`walt/walt`). The **proof-state stack**
(rows 1–9b) answers the question exactly or with a certified interval;
the **sampling stack** (rows 10–11b) is what people play and what the
partnership program extends; Scheme and the gym (rows 12–13) sit beside
both. Neither stack replaces the other (FH-A10;
`experiments/partnership/PLAYERS.md`).

| # | object | where | what it is | cost, honestly |
|---|---|---|---|---|
| 1 | counted belief | `solver/factor_belief.rs` | the 399,072,960 trick-1 worlds as 116,280 acting-seat hands × exact-cover counts; posterior = one seat's factor changes per observed play | counting ≈ ms; classifying each hand through σ is 99% of every bill |
| 2 | the field σ0 | `solver/field.rs` | the level-0 modeled mind for the other three seats; deterministic; reads the bid and the FULL public record | one read ≈ µs; every recursion's cost = its read count. Cross-history cache reuse measured 0 because the record is in the key |
| 3 | exact response | `response_success_mass` | `Q` exactly at a root; the oracle every gate checks against | h8-t3 289M reads ≈ 14 min (`horizon_run1.txt`); at trick-4 roots the same tree as the FH1 engine at k = 2 (0.66M reads h8-t4, 10M h4-t4, `focal_run0.txt` — the engine's count, not a separate measurement); trick 1 unreachable by this path |
| 4 | fixed-policy value | `viewer_success_mass` | `V^π` of one lawful policy π (a "tail") | the cheap one: hidden branching only |
| 5 | God upper | `doom.rs`, `godgap.rs`, `horizon.rs` | per-world clairvoyant make check; `1 − doomed/Z` bounds `Q` from above | one line walk per world of the node |
| 6 | proof state | `proof_state.rs` + frontier/refine/opening/residual/covers/laydown/extraction | append-only facts; survivors, verdict and certified regret `Γ = U* − B_exec` are DERIVED from them | container, not compute |
| 7 | model belief Ξ = Ω×Θ | `model_belief.rs`, `model_recursion.rs` | the field itself as hidden state (types); fusion price strictly positive at trick 4 (MB1) | the wall: affordable at t4, refused at t3 |
| 8 | unified player | `unified.rs` | one decision core over every instrument: decided → endgame exact → mixture → certified regret → σ0 fallback; posterior carried lazily (UP1a) | lean rung 11.6 ms for 72 decisions; ample rung 20 s |
| 9 | **focal-horizon hierarchy** | `focal_horizon.rs` (FH1, 2026-09-04) | `[L_k, U_k]` per root action, `k` = focal decisions made exact; `k = 0` is (4) below and (5) above; collapses to (3) at `k = 6 − T` because trick 7 is forced | k = 0 at h4-t4 5.1M reads; k = 2 = the exact solve |
| 9b | focal-horizon ladder | `focal_ladder.rs` (FH2, 2026-09-04) | the same recursion as budgeted PASSES over a store of node facts: stop at a read ceiling, resume to the identical result; exact suffix reuse | reads at k = 2 fall 2.83M → 0.42M (h3-t4) with reuse; memory GREW: 662 MB peak vs 411 MB for the direct engine (per-node policy tables) |
| 10 | **the live default player** | `SCENARIO-PLAYER.md` (the spec); `bin/walt_bridge` (arena seat), `walt-wasm` (the phone/plunge build), `bin/playtable` / `bin/webtable`; `walt2-wasm` = its level-2 browser variant | the level-1 sampling-stack seat: n_outer sampled worlds × level-0 modeled minds, exact pmake on the sample, argmax (Def 6.1); the pre-program player people actually play | untouched by rows 1–9b (FH-A10) and by the partnership program (no default changed); native L1 0.080 s per decision against the archived phone's 0.636 s (150-game calibration, `experiments/partnership/campaigns/native-l1-vs-phone-620600-649/CALIBRATION.md`); 0.228 s per move in the 400-game default battery |
| 10b | controller variant | `CONTROLLER-PLAYER.md`, `solver/act.rs`, `bin/controller_bridge` | the CE-thread δ-controlled player (six routes, three settled); the waking seat (`solver/waking.rs`, `bin/waking_bridge`, `bin/granrun`) escalates it to σ1 on evidence | the same bill plus the controller; interactive cap 128 ≈ 10 s-scale trick-1 decisions; the waking seat: minutes per natural hand, not affordable as-is |
| 11 | selection foundation + partnership player | `solver/selection.rs` (246 lines), `solver/partnership.rs` (331), `bin/partnership`; driven from `experiments/partnership/*.py` | one shared fixed / refine / race selection rule for the real root and every modeled mind; player families **L1 default**, **L2 Partner** (partner modeled at L1, opponents at L0), **L2 Partner with voids** | per move 0.228 / 1.127 / 1.318 s (default battery, 400 games on 100 shared deals); no strength gain established — L2 Partner vs L1 14 / 14 / 72, with voids vs without 12 / 17 / 71 |
| 11b | inner belief | `solver/inner_belief.rs` (165) | `InnerBelief::{Voidless (default), VoidsCounted}` — whether the modeled minds sample the void-conditioned fiber; the O5 alternative on main. A second implementation (`Level0Field::void_aware` behind `Key::voids`) lives on the UNMERGED branch `walt-o5` | opt-in; racing L1 with voids 9 / 5 / 36 (random) and 5 / 8 / 37 (fixed hands); neither implementation discharges O5's cost/strength obligation; the default is Jason's word ([[inner-voids-default]]) |
| 12 | Scheme/Fix | `src/scheme/` (7 files, 2,704 lines), `bin/scheme`; guide `scheme/README.md` | typed relational expressions over kernel worlds: role bindings, exact event probabilities, belief conditioning, counterexamples (2026-09-06); one-play finite dynamics and executable policies with rigid bindings and a persistent controller mode (2026-09-07) | the README example (partner holds a count tile, receipt hand 0 trick 6) answers 1/3 at work 1084, sub-second; enumeration behind explicit world / work caps (40,000 / 10,000,000); no compression, strength or compact-transducer claim ([[scheme-compact-compiler]]) |
| 13 | the partnership gym + policy search | `src/gym.rs` (548), `bin/partnership_gym`, `experiments/partnership/gym.py`; `src/policy_search/` (3,141 lines), `bin/policy_lab`, `bin/relational_lab` | exact finite-domain exercises with answer keys under a declared field: 6 starters → 170 (Scheme-driven discovery over 1,929 coordinates) → 433 bid-making (367 unique best, 26 certain swings) → the 30-case composed exam; sampled-table and shared-relational policy constructors on top | 60 coordinates audited in 1.305 s (ten workers); discovery ≈ 13 s; 433 keys in 50.616 s (spec regeneration 67.707 s); the exam: L1 24/30 vs L2 Partner 26/30, 60 decisions in 1.458 s; persistence halves policy search; shared relational actors still trail sampled tables |

**Side tracks, not in the critical path.**

- **The partnership program** (`experiments/partnership/`, 2026-09-06/07,
  a Codex session under a CI waiver — `walt/ci/check.sh` was NOT run on
  `d8400713..c00717d1`, so whether main is green at HEAD is unverified):
  the launch packet, the archived Plunge WASM as the frozen "phone"
  reference, five seed panels and 2,000+ independently replay-verified
  games, the strength question open ([[partnership-strength-question]]).
  Status ledger `experiments/partnership/SESSION-STATUS.md`.
- **Two UNMERGED branches** from the 2026-09-05 readout
  (`briefs/MORNING-2026-09-05.md`, `9d6a5a2e` — the only main-side
  record; counts verified 2026-09-13 with `git log main..<branch>`):
  `walt-o5`, **9 commits** (`0b65efb9..2981e090`; O5 measured — the
  modeled minds' no-void fiber is median 250–363‰ dead from trick 3,
  the σ0 flip rate GROWS with n0 (102‰ → 189‰), the mirrored match is
  epoch-dependent after a seed repair — live 33/19/20 pairs aware, +262
  of 6048; reduced 76/83/33 blind, −226 of 16128 — "dead heat"
  withdrawn; `check.sh` PASS on the branch); `walt-g1-l2`, **8 commits**
  (`2d3907bd..6abdd78f`; level 2 at Gran's seat HOLDS the 6-4 at both G1
  nodes, G2 exactly locked from trick 3, the synthetic lock's release
  margin moves L1 → L2, tie-break = `TieRule::LowestTileIndex`). Jason's
  calls (A)–(F) in the readout are open; the merged half is
  `walt-gran` (G1 anchor, `probes/gran/`, [[gran-anchor-reconstruction]]).
- GPU-native trick 1 (M0–M2 parity gated, M3 gate frozen, no result),
  the Lean side project ([[lean-catchup]]), rob (the exact
  perfect-information engine with byte-diffed receipts; dormant since
  2026-07-30), the Pro exchange channel (24 dispatches; the FH response
  `briefs/FH-RESPONSE-TO-PRO.md` drafted 2026-09-04 for hand-ferry, not a
  numbered dispatch).

## What the hierarchy makes redundant (the tree-shake list)

Object 9 was built to be the one object. In its vocabulary: the God-gap
census (U0) is `U_{a,0}`; the in-solve ply-cut census (U0b) is
`U_{a,m−1}` on viewer-lead roots (Proposition FH-cut); the salvation-mask
upper (queued U1, never built) is `U_{a,1}` (Theorem 5); rollout
improvement is `L_k`; argmax extraction is `π_k`; the exact endpoint is
the collapse. So `godgap.rs` (933 lines), `horizon.rs` (635) and
`extraction.rs` (135) are now measurement scaffolding around one
recursion, and `refine.rs` (917, frozen as freeze 58) was already
declared removable. `doom.rs` (1,048) stays: it is the God tail's engine.
That is the consolidation slice — carded 2026-09-13 as
[[consolidation-slice]], not started as of `c00717d1`.

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
| `solver/` size | **38,013 lines, 39 files** (38 modules + `mod.rs`), measured 2026-09-13 on `c00717d1`; was 37,260 / 36 after FH2 — `inner_belief.rs`, `selection.rs`, `partnership.rs` added 2026-09-06. Beside it: `scheme/` 2,704 lines, `gym.rs` 548, `policy_search` 3,141; 54 bin sources (55 entries with `webtable.html`); 70 test files | growing one module per intake; consolidation not yet started ([[consolidation-slice]]) |
| peak memory | ladder 662 MB at h3-t4; **19.4 GB at h8-t3** (3.8M facts + 3.55M memo receipts); the anchors gate 18.2 GB → 8.8 GB after FH5 capped h4-t4 jobs in flight (each h4-t4 evaluation ≈ 1.6 GB whatever its kind) | the cost that is growing fastest; a version-referenced policy store is the fix, not built ([[ladder-policy-store]]) |
| gate wall (`check.sh`) | 308 s with FH3's anchors gate (230 s before it; ~15 min serial on 2026-09-04 morning); needs ~18–19 GB RAM for the anchors suite | fixed by concurrency + fixtures; corpus trimming still owed ([[gate-corpus-trim]]); NOT run on the 2026-09-06/07 commits (session waiver) |
| partnership player per move | 0.228 s (L1) / 1.127 s (L2 Partner) / 1.318 s (with voids); the wrapper caps a decision at 14 s and the largest four-play trick measured 33.961 s against the 60 s brief | the L2 rung costs 5× for no measured gain; the open question is strength at matched cost ([[partnership-strength-question]]) |

## Next, in order

FH2 ✓ (dc515ac) → FH3 ✓ (fc171e1) → audit ✓ (8aae7c7: one BLOCK,
vocabulary, fixed; 13 NOTEs, 5 fixed, rest carded) → one PR ✓ (**#88,
recorded MERGED 2026-09-07T06:17Z by fast-forward, merge commit =
branch head `a0d594b2`**)
→ the σ0 read-key study ([[sigma0-read-key-study]]: does σ0's answer
depend on the full record? if not, the cache key coarsens and every
recursion gets 10–100× cheaper; FH3's warm-instance 15× says the field
cache is the lever)
→ the consolidation slice ([[consolidation-slice]]: retire `godgap.rs`,
`horizon.rs`, `extraction.rs`, `refine.rs` as endpoints of object 9).
**No new mathematical parent until the consolidation lands** (Jason,
2026-09-04: "follow through on what we have, then invest in a
simplification/unification attempt"). Open beside the line, on Jason's
word: the O5 flag default ([[inner-voids-default]]), the shape of the two
unmerged branches, the partnership strength question
([[partnership-strength-question]]), and whether the phone still ships
the archived build ([[plunge-walt-sync]]).
