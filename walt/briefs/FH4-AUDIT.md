# FH4-AUDIT — independent audit of the focal-horizon program (branch `walt-fh`)

**Auditor:** an agent that built none of it (BRIEF-FH4-AUDIT; the standing
audit-independence policy). **Scope:** every commit `a80b982..a2623bb` on
`walt-fh` — FH0, FH1, CI1, FH2, FH3, the orchestrator's process commits,
and the two commits that landed while this audit ran (`35d7679` the Pro
response draft, `a2623bb` the wiki/LOG entry). **Date:** 2026-09-04.
**EXPLORATORY tier throughout; this audit changes no tier.** No code, gate,
record, ledger or wiki file was changed; this file is the only write.
Scratch logs: `<scratchpad>/fh4/` (not committed).

**Verdict in one line:** one BLOCK (vocabulary, FH-A2 — a rename, not a
rebuild), thirteen NOTEs, everything else PASS. Every record reproduces;
the gate is honest; no §41 correctness failure has an ungated path except
the ladder's cap-refusal branch (N3).

## What the auditor ran (measured, this machine: 18 cores, 48 GB)

| what | result |
|---|---|
| `walt/ci/check.sh`, foreground, clean tree | PASS, **306.65 s** wall, 123 binaries, 0 failed, sum of suite walls 2,115 s; doc tests 13/13; Lean built |
| `cargo test --workspace --release --no-run` test executables (counted from the JSON stream, independently of the runner) | **123** = the runner's 123 (122 distinct target names; two packages share one) |
| `check.sh` with a scratch failing test `tests/fh4_scratch_failing.rs` (deleted after; tree clean) | **exit 1** at the test stage; the failing binary's stdout marker and panic printed in full between `FAIL … full output follows` and `---- end of output`; 124 binaries, 1 failed; doc tests and Lean did not run; 275.9 s |
| `focalreport scout 8 4 1 30 40000 sigma0 exact` vs `focal_run0.txt` h8-t4 k=1 | identical modulo `…us` walls |
| `focalreport ladder 8 4 receipt 0:150000 0:inf 1:250000 1:inf 2:inf` vs `focal_ladder_run1.txt` h8-t4 | identical modulo walls (the rerun appends its own table) |
| `focalreport report <out> h8-t4` (5 contracts) vs `focal_run1.txt` | all five sections and the five summary rows identical modulo walls |
| `solver_focal_anchors` standalone under `/usr/bin/time -l` | 4/4 pass, **82.6 s**, **18.22 GB** peak RSS (FH3 said 17.8 GB) |
| `check.sh` rusage, largest single descendant peak | 8.81 GB (clean run), 9.07 GB (failing run); a 3 s RSS sampler over the last 2.4 min of the clean run saw ≤ 21.0 GB summed over the runner's nine processes, largest single 8.5 GB (`solver_focal_horizon`) |
| `focalreport ladder 3 4 receipt [nomemo] 0:inf 1:inf 2:inf` and `scout 3 4 2` under `time -l` | memo on **662 MB**, memo off **509 MB**, direct engine **411 MB** — FH2's prose numbers reproduced |

## The table — moves × slices

| move | slice | verdict | citation (one line) |
|---|---|---|---|
| 1 gate-not-prose | FH1 report | NOTE N1 | scout findings 1–5 are record numbers (`focal_run0.txt`), labelled EXPLORATORY, none presented as gated; the FH-A8 gate pins only what FH-last implies |
| 1 | CI1 report | PASS | "no assertion changed": every removed line in the three suites' diffs (`git diff a80b982..HEAD`, 636 lines) is setup/import; zero removed `assert`/`panic` lines; the focal suite's CI1 diff (100 removed lines) likewise, FH-D's second fresh run → fixture disclosed in CI1-REPORT item 2 |
| 1 | FH2 report | NOTE N2 | the 662/509/411 MB figures are prose-only (no record carries them) — reproduced here to the MB |
| 1 | FH3 report | NOTE N4, N5 | every settling-horizon and Γ number matches the record's table (checked row by row: 5/6/3 split, Γ₁ ≤ 45‰, h8-t3 141/100/34/0); "Q − L 13–34‰ / U − Q 1–3‰ at h8-t3 k=2" are the record's WIDTH column, Q − L is 12–33‰ and U − Q 1–2‰ (`focal_run1.txt` lines 22–26); "again in the gate" holds for 7 of 33 coordinates only, as the report's own anchor-(i)-probe-only paragraph says |
| 1 | FACTOR-BELIEF.md | NOTE N6 | header says "32 (root, contract) coordinates"; the record has 33 `==` sections (32 corpus + h8-t3); commit, FH3-REPORT and MAP say 33 |
| 2 frozen oracle | FH1 gates | PASS + NOTE N7 | `U_{a,0}` vs `doom::doom_enumeration` (own world enumeration; shares only `line_can_make` with `horizon::doom_over_belief`); `Q_a` vs `response_success_mass` (the §36 recursion, a different path); `L_{a,0}` vs `viewer_success_mass` is the SAME call the engine makes at a k=0 root child — plumbing parity, not independence (N7) |
| 2 | CI1 fixtures | PASS | `tests/common/fixture.rs` computes only the independent functions per key; no fixture reads an engine output as an oracle; FH5 replays read the engine's POLICY (the thing under test) and price it through the evaluator; nothing regenerates its own oracle |
| 2 | FH2 gates | PASS | `Q_a` by `response_success_mass` (`exact_q_at`), FH1 engines as the parity reference (a different recursion, LP), `viewer_success_mass` as the re-pricer (PS2) |
| 2 | FH3 gates | PASS | `exact_q_at` = `response_success_mass`; `census_at` = `horizon_census` live; FHA3's "census exact reading = Q_a" is same-path consistency (both `response_success_mass`), stated here, not a defect |
| 2 | records | PASS | all three reproduce (table above); FH1b's quoted rationals match their sources — h8-t4 bid 36 `451/600`, `303/400` in the companion's Q6, h4-t4 bid 39 `11353/17325`, `1165/1386` in `horizon_run1.txt:2372–2374` (Q6 carries them as 655‰/840‰) |
| 3 §41(1) unreplayable lower | FH1/FH2 | PASS | FH5 (`V(π_k)` = `B_k` and `L_{a,k}` at every child, both tails, ≥100 replays) and PS2 (`reprice` = stored value; asserted `priced >= fact` in `focal_ladder.rs:1115`) |
| 3 §41(2) Q outside interval | FH1/FH3 | PASS | FH2, FH4, FHA1 `solver_focal_anchors.rs:426`; FH7(3) for the ladder |
| 3 §41(3)/(4) lower falls / upper rises | all | PASS | FH2 nesting with strict witnesses, FHA1, FH7b (`(Some(_), None) ⇒ panic`, an upper never discarded) |
| 3 §41(5) hidden branch consumes horizon | FH1 | PASS | hidden nodes pass `k` unchanged (`focal_horizon.rs` walk); FH3 collapse at k = 6−T and zero consultations at k = 7−T would fail if a hidden node consumed a unit; FH1b cut-8 = `U_{a,1}` |
| 3 §41(6) cellwise max before merge | FH1 | PASS | FH6: test-local fused upper ≥ `U_{a,1}`, STRICTLY above on a specimen, = `U_{a,0}`; mask identity = `U_{a,1}` |
| 3 §41(7) truncated number on refusal | FH1/FH2 | PASS engine, **NOTE N3** ladder | FH-R (typed whole-root refusal, node rebuilt and checked); FH7c (ceiling 0 → no interval, no regret); FH7 (retained fact = the uncapped run's). The ladder's CAP refusal branch (`ResidualCause::Unaffordable`, `stopping_node = None`, "continue at the next root child") has NO gate and never fired in any record (N3) |
| 3 §41(8) suffix receipt under mismatched identity | FH2 | PASS | `SuffixMemo` matches by `FactorBelief` equality (derived `PartialEq` over root_id, kernel, position incl. contract, history, field_id, factors — `factor_belief.rs:309`); SR2: frozen contract-30 memo, contract-36 ladder, >1,000 lookups, 0 hits; factor-narrowed belief misses |
| 4 FH-A2 vocabulary | FH3, ledger | **BLOCK B1** | `fha1_sandwich_nesting_containment_collapse` (`solver_focal_anchors.rs:419`, module doc line 23), FH3-REPORT ("FHA1 sandwich", "every sandwich gate holds"), `FACTOR-BELIEF.md:1087`, `BRIEF-FH3.md:44` — "sandwich" as the property's name, not the parent's title |
| 4 FH-A3 no trivial upper as a fact | FH1/FH2 | PASS | `price_frontier` returns `Err` before any install; ladder `NodeFact.upper_mass` exists only on completed nodes; `LadderAction.upper_mass: Option`; FH7c |
| 4 FH-A4 tail identity includes the contract | FH1/FH2 | PASS | `FocalIdentity.contract`, `LadderIdentity`, `advance` asserts identity; policy id embeds the tail id; SR2 cross-contract miss |
| 4 FH-A6 shared `decided_success` | FH1 | PASS | engine walk, `depth_walk`, and the ladder all call `decided_success(position, viewer, banked, at_terminal)` |
| 4 FH-A7 off-DAG = tail | FH1 | PASS | `FocalPolicy::choose` falls through to the tail; FH5's off-DAG check discriminates σ0 from the lowest tile |
| 4 FH-A9 intersection, policy with every lower, full-belief key | FH2 | PASS | `install`: lower = max (prior wins ties), upper = min, `completed_at` = max; parent composes from the FACT (`focal_ladder.rs:1040`); nothing partial written; §41(8) row |
| 4 FH-A10 non-goals | all | PASS | `git diff --stat`: no change under `unified.rs`, `model_belief*`, `CONTROLLER-PLAYER.md`; no arena claim in any report; the Pro draft makes none |
| 4 FH-A11 no partial install in FH1 | FH1 | PASS | `walk` propagates with `?`; `focal_horizon` returns `Err` whole-root; FH-R |
| 5 derived views | FH1 | PASS | beliefs are stack-transient; the result's bar/survivors/verdict are fields of an immutable value computed once from `actions`, not a second mutable authority |
| 5 | FH2 | PASS + NOTE N8 | `root_view()` recomputed on every call, never stored; `PassReport.view` is a report snapshot. `NodeFact.policy` copies each subtree table into every ancestor's fact — a witness, consistent by construction (a parent's lower is attained by its own stored policy even after a child tightens), so not two authorities, but the memory cost is real (N8) |
| 6 tier discipline | ledger, wiki, MAP | PASS | every FACTOR-BELIEF paragraph ends "EXPLORATORY tier"; MAP header; `walt-counted-belief-era.md` section "EXPLORATORY throughout"; `walt-math-intakes.md` row inside the exploratory fence; no number quoted above its tier anywhere on the branch |
| 7 CI honesty | CI1 | PASS | table above: 306.65 s, 123 = 123, failing test fails the gate |
| 8 process commits | CLAUDE.md | NOTE N9 | the wedge rule is accurate in substance; the times "02:11 … five hours" are NOT in FH1-REPORT's deviation 5 (which carries no times); they are consistent with the git times (FH1 commit 07:31, process commit 07:07) |
| 8 | MAP.md | NOTE N10, N11 | settling/Γ/read numbers match the records; "solver/ 36,076 lines, 35 modules" is pre-FH2 (now 37,260 lines, 35 modules + `mod.rs`); row 3's "exact response … h8-t4 0.66M / h4-t4 10M reads" are the FH1 engine's k=2 reads (`focal_run1.txt` 662,203 / 10,209,381), not a measured `response_success_mass` count; "289M ≈ 14 min" is `horizon_run1.txt:11932` |
| 8 | Pro draft (`35d7679`) | NOTE N12 | labelled DRAFT, EXPLORATORY, not a dispatch; claims "one audit" and "every §41 correctness failure named to a gate" ahead of this audit — N3 makes the second claim false as written |
| 9 memory as a gate hazard | FH3 | NOTE N13 | measured: 18.22 GB standalone; the gate passed twice today on this 48 GB machine; no rule bounds it |
| 10 fresh-field record and k=3 fix | FH3 | PASS | record header states the discipline; h8-t3 ladder k=0 wall 161.0 s ≈ direct k=0 162.8 s (cold, not the 15× warm figure); `focal_run1.txt` committed once (`fc171e1`); the `extended` flag in `focalreport.rs` (`report_of_record`, "fires at most ONCE") is in the committed probe and the record shows exactly one extension ("k = 2 pass wall … < gate → k = 3 RUN") |

## BLOCKs

### B1 — "sandwich" used as an object name (FH-A2 not obeyed)

**What.** FH-A2 (and CBS-A3 before it) ruled that "sandwich" survives only
when the parent's title is cited. The branch uses it as the name of the
containment property `L ≤ Q ≤ U`:

- `walt/walt/tests/solver_focal_anchors.rs:419` — the gate function
  `fha1_sandwich_nesting_containment_collapse`, and its module doc line 23
  ("FHA1 sandwich, nesting, containment");
- `walt/briefs/FH3-REPORT.md` — "FHA1 sandwich, nesting, containment" and
  "every sandwich gate holds";
- `walt/FACTOR-BELIEF.md:1087` (the SLICE FH3 LANDED paragraph) — "FHA1
  sandwich/nesting/containment";
- `walt/briefs/BRIEF-FH3.md:44` — "sandwich, nesting, containment".

**Why it matters.** Vocabulary is a hard rule (CLAUDE.md: typed
distinctions, not emphasis); the ledger paragraph is the record of what
landed, and a gate name is what a future reader greps for. walt's
`check.sh` carries no vocabulary grep (rob's does), so nothing mechanical
stops this recurring.

**Minimal fix.** Rename the gate to `fha1_containment_nesting_collapse`
(or "root interval"), and replace the four prose occurrences with
"containment" / "root-interval"; no assertion changes. Optional, as a
card: a `sandwich`/`certificate` grep in `walt/ci/check.sh` over
`walt/walt`, `walt/briefs`, `walt/FACTOR-BELIEF.md` with the parent-title
citations whitelisted (pre-branch uses in `solver_fieldswap_cancel.rs`,
`fieldswap_cancel.rs`, `solver_calibrate.rs`, `solver_targeted.rs` would
need the same treatment — out of this branch's scope).

## NOTEs

- **N1 (FH1 report).** The five scout findings and their numbers are
  `focal_run0.txt` readings, correctly labelled exploratory and quotable
  only via receipts; the only gated pins among them are FH-A8's h8-t4
  bids 36/39 (k=1 not `Settled{2-1}`, k=2 `Settled{2-1}`), which FH-last
  plus the exact argmax already imply.
- **N2 (FH2 report).** Peak-RSS figures 662 / 509 / 411 MB exist only in
  prose. Reproduced here exactly (`time -l`); a probe record line for
  memory would make them citable.
- **N3 (FH2, §41(7) in the ladder — the one ungated correctness path).**
  `focal_ladder.rs` handles a fiber-cap refusal inside a pass by
  listing the node `Unaffordable`, marking every later sibling on the
  unwind `Unvisited`, leaving the enclosing root child unfinished, and
  continuing at the next root child with `stopping_node = None`. No gate
  exercises it (FH7/FH7c assert `unaffordable.is_empty()`), and no record
  shows a refused node (every record runs at cap 40,000 with "refused
  frontier 0"). The FH2 report presents it as a judgment call, not as
  gated, so it is a NOTE; but it is FH-A3's discipline and the only §41
  item whose ladder branch is unexecuted. Minimal gate: one h8-t4 pass at
  a tiny cap — `Interrupted` with `unaffordable` non-empty,
  `stopping_node == None`, no fact under the refused child, the other
  root children completed, the view carrying placeholders there.
- **N4 (FH3 report).** "Q − L 13–34‰ and U − Q 1–3‰ at h8-t3 k=2" quote
  the record's WIDTH column; the record's own columns read Q − L 12–33‰
  and U − Q 1–2‰ (`focal_run1.txt`, h8-t3 k=2 action lines). Same drift in
  the Pro draft ("13–34‰ at trick 3").
- **N5 (FH3 report).** "…passed inside the record itself and again in the
  gate" — the gate covers seven coordinates; the h8-t3 anchor's §41
  checks are probe-internal (`check_laws` in `focalreport.rs`) against
  `Q_a` CITED from `horizon_run1.txt`. The k=3 collapse reproducing
  `28859/29988` is a genuine independent reproduction, but a probe one.
- **N6 (ledger).** `FACTOR-BELIEF.md` header line says "32 (root,
  contract) coordinates × k ∈ {0,1,2,3}"; the record, the commit message,
  FH3-REPORT and MAP say 33 (32 corpus + h8-t3). One word.
- **N7 (FH1 gate, oracle sharing — stated, not a defect).** `L_{a,0} =
  viewer_success_mass` is the same function call at the same node (the
  root child IS the k=0 frontier), so FH1's lower half checks plumbing;
  its upper half is independent at the enumeration level
  (`doom_enumeration` vs `doom_over_belief`) but both use `doom.rs`'s
  `line_can_make` for the per-world make check; FH6's test-local walker
  is the only fully independent God computation on the branch (and it is
  a strong one: it reproduces `U_{a,0}` and `U_{a,1}` from scratch).
- **N8 (FH2, memory diagnosis on the card).** `[[ladder-policy-store]]`
  attributes the growth to per-node policy tables. Measured at h3-t4: the
  fact store without the memo adds ~98 MB over the direct engine
  (509 − 411), the memo's `FactorBelief` clones add ~153 MB more
  (662 − 509). At h8-t3 there are 3.55M receipts against 3.82M facts. The
  memo's within-ladder hits are exactly "a collapsed priced fact exists at
  this node with `completed_at < j`" — the same belief is always reached
  at the same history inside one ladder — so a `collapsed ⇒ return the
  fact` clause in the resume rule would deliver the same hits with no
  clone; the full-belief key matters only for cross-ladder consultation
  (SR2). The card should name both sinks and measure which dominates at
  h8-t3 before fixing the smaller one.
- **N9 (CLAUDE.md).** The "02:11 / five hours" wedge timeline cannot be
  verified against FH1-REPORT (no times there); git times corroborate it
  loosely. Either add the times to the FH1 report's deviation 5 or drop
  them from CLAUDE.md.
- **N10 (MAP.md).** "solver/ 36,076 lines, 35 modules" predates FH2
  (37,260 lines today); the MAP is "rewritten at every landing" and this
  row was not.
- **N11 (MAP.md row 3).** The "exact response" cost column quotes the
  FH1 engine's k=2 reads (h8-t4 0.66M, h4-t4 10M) as
  `response_success_mass`'s cost; the branch records no read count for
  that function at those roots. Label the source or measure it.
- **N12 (Pro draft, `35d7679`).** Written before this audit yet says "one
  audit" and "every §41 correctness failure named to a gate that would
  catch it" (N3 is the exception); carries N4's drift. Amend before the
  hand-ferry; nothing else in it exceeds EXPLORATORY.
- **N13 (memory as a gate hazard, move 9).** The anchors gate needs
  18.2 GB standalone (measured; FH3's 17.8 GB reproduced), and inside the
  nine-at-a-time runner the machine carried ≥ 21 GB of test-binary RSS at
  once (sampled) — it passed twice today on 48 GB with no rule bounding
  it. The peak is transient: `ladder_at` drops the `FocalLadder` and keeps
  views, so the 18 GB is five h4-t4 ladders (and their memos) in flight
  simultaneously because `compute_all` hands the five heaviest jobs to
  five threads first. In-gate the largest single-process peak the gate's
  rusage recorded was ~9 GB, consistent with contention de-synchronizing
  the five peaks. Judgment: acceptable to land with the card, NOT a
  BLOCK — it runs on the machine it targets — but the gate is one
  hardware change from silent failure. Cheapest mitigation without
  touching laws: run the `Job::Ladder` keys serially (or two at a time)
  in `fixture_jobs`/`compute_all`, which caps the transient at one or two
  ladders (≈4–8 GB) for a modest wall cost; the real fix is the card's.

## Slice-by-slice one-liners

- **FH0** — rulings FH-A1..A11 read in full; the five delivered
  propositions are used exactly as stated by the code (FH-God = the leaf,
  FH-int = `install`, FH-tie = both verdict functions, FH-cut = FH1b/FHA3,
  FH-last = FH3/FHA1). PASS.
- **FH1** — engine composes the existing authorities and holds no second
  implementation; ten gates as described; record reproduces. PASS (N1, N7).
- **CI1** — a scheduler, not a semantics change; counts match; failure
  propagates. PASS.
- **FH2** — intersection discipline is real, the root is a derived view,
  the memo key is the full belief; nine gates as described; record
  reproduces. PASS (N2, N3, N8).
- **FH3** — anchors discovered not pinned (no gate asserts which k
  settles); four gates as described; record reproduces; fresh-field and
  the one-shot extension confirmed in the committed probe. PASS (B1, N4,
  N5, N13).
- **Process** — accurate in substance; three numbers stale or unsourced
  (N6, N9, N10, N11); the Pro draft needs the post-audit amendment (N12).

EXPLORATORY — below every evidentiary tier; this audit is evidence for
the orchestrator's landing decision and for nothing above it.
