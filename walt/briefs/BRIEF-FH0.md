# BRIEF-FH0 — intake and walt-math review of the focal-horizon sandwich parent

**Authorized:** 2026-09-04, Jason ("please prepare to orchestrate the
intake, a walt-math review ... and the implementations"; "you're the
engineer in charge"). **Parent (verbatim, checksum-pinned, NEVER
edited):** `walt/math/focal_horizon_sandwich_v0.1.md`, SHA-256
`892bc343f1ada12013b2bbd674d46962bc0256a55170aa48075fea17c2592f04`
(pinned by the `.sha256` beside it; filed from the upload
`DESIGN-walt-focal-horizon-sandwich-v0.1.md` under the `walt/math/`
snake_case convention — recorded, not repaired). **Companion
verifier (filed verbatim):** `walt/math/verify_focal_horizon_sandwich_v0.1.py`,
SHA-256 `7700a35ec0a718ca674b8a56f6fec87f0bbeab6dbe9f11d9f9ec0d28881ee34b`
(from `verify_walt_focal_horizon_sandwich_v0_1.py`). Scratch tier:
session evidence, never a receipt, never imported into the codebase
(TRUST-01).

**Provenance:** a Pro deliverable, hand-delivered by Jason 2026-09-04.
Not a courier dispatch — no number, the exchange ledger is untouched
(the calculated-evidence precedent). Jason's own framing of this one:
"less 'I have a bunch of proofs let's gooo' and more 'here is an idea
to try with measurements'"; "I'm sure there is good math in there. I'm
not sure the tasks are as focused as usual."

Read first: `CLAUDE.md` (hard rules), `walt/math/model_belief_base_player_v0.1_intake.md`
and `walt/math/salvation_complex_v0.1_intake.md` (the two most recent
companions — the shape to reproduce), the two most recent rulings
sections at the tail of `walt/CENSUS-RULINGS.md` (MB-A1..A8,
SC-A1..A8), `walt/briefs/U0B-REPORT.md` and `walt/briefs/UP1A-REPORT.md`
(what PR #87 established — the parent's Part I cites it), and the head
of `walt/FACTOR-BELIEF.md` (the status ledger).

## Mission

Two things, one agent, one document.

**(1) The intake, in the standing shape.** Write
`walt/math/focal_horizon_sandwich_v0.1_intake.md` — the maintained
companion, governing wherever it repairs or narrows the parent (the
DS-A17 citation rule). Append a section `## The focal-horizon
adjudication (2026-09-04)` to `walt/CENSUS-RULINGS.md` with rulings
**FH-A1..An** (binding on consumers of the parent). Add the lineage row
to the table in `wiki/walt-math-intakes.md` (and nothing else on that
page). Every substantive statement carries its tier label: everything
here is EXPLORATORY, nothing is promoted by being received, audited or
indexed.

The intake must contain, as every companion does:
- **Adjudication receipt:** the verifier executed twice (from the
  upload location and from `walt/math/`), the exact output, exit
  code, and a load-bearing/weak-checks split — say plainly which of the
  24 "check families" are theorem verifications on the exhaustive
  4,096-system × 8-policy sweep and which are definitional
  illustrations (several late checks assert hardcoded specimens, e.g.
  the supersolution-is-load-bearing check asserts `1/2 < 3/4`). Never
  let the count be quoted as 24 theorem verifications.
- **Theorems step-checked:** Theorems 1–6 (§9–§12, §13, §17), the §18
  exact-action criterion, §19 certified regret and its claimed
  monotonicity `Γ_{k+1} ≤ Γ_k` "with deterministic exact tails and
  preserved facts" (state exactly what "preserved facts" must mean for
  this to hold, or narrow it), §22 exact-mass form, §23 the
  interruption rule (the parent gives no proof — supply one or refuse
  it), §25 continuation substitution and its identity list. The proofs
  are elementary finite induction; write them fully where the parent
  sketches, in the companion, with dated provenance markers.
- **Snapshot check:** the parent declares its basis as merged PR #87
  `a80b98291a60c5fda22d44f528efc16016c53425`. Verify against `main`.
  Verify every Part I empirical citation against the committed records
  (`walt/probes/factor_belief/horizon_run1.txt`, `unified_run2.txt`,
  the two reports) — the parent restates measurements, and restated
  measurements drift.
- **Vocabulary sweep:** no bare "certificate"; no floats; and rule on
  the word "sandwich" — CBS-A retired "sandwich" in favour of "root
  interval / survivor set" for the counted-belief calculus. The parent
  is titled with it. Decide what the code and the ledger call this
  object (the orchestrator's recommendation: "focal-horizon hierarchy"
  / "focal-horizon interval `[L_k, U_k]`" everywhere except when citing
  the parent's title; but it is your ruling).
- **Thread labels:** CE (sampling depth) vs L2 (model choice). The
  orchestrator's reading: fixed-field L2-thread mathematics with no
  sampling anywhere in the hierarchy (the parent says so at §4); the
  Ω×Θ lift is deferred and stays deferred.

**(2) The walt-math review.** Jason's words: "good stuff in here, could
it be better without losing what it is? if walt-math agent wants to add
its thoughts those are entirely welcome." Put this in the companion as
its own section — a review, not a rewrite: what is sharp, what is
underspecified, what should be stated more precisely before a builder
relies on it, and (clearly labelled as walt-math's opinion) what you
would add. Propositions you deliver get the standing "— delivered here"
treatment in CENSUS-RULINGS (see the FF/FC chapters for the form).

## Rulings the builders need (rule on each; they are FH-A rows)

The engineering slices FH1–FH3 launch after this intake returns and
will read the companion before code. These are the questions whose
answers change what they build:

1. **The lower tail π.** The parent asks for "one existing lawful
   deterministic policy with an independent exact fixed-policy
   evaluator". The evaluator exists: `viewer_success_mass(oracle,
   belief, focal, field, stats)` in `solver/factor_belief.rs` (the §23
   fixed-policy recursion, Slice D). Candidate tails: (a) σ0 — the
   declared field policy `FieldModel` driving the VIEWER seat as well
   (the parent's "the same baseline may serve as executable fallback,
   lower tail, response column"); (b) `FixedPreference::lowest_first`
   (the trivial C0 tail); (c) an extracted policy from a prior solve.
   Recommend a primary tail and a gate-only second tail. Note that σ0
   reads the bid, so the tail identity includes the contract.
2. **The upper tail is admissible.** State and prove that the
   world-revealed continuation under a DETERMINISTIC field (every
   surviving world has unit weight; each world's field action is a
   function of the world, so each world lies in exactly one public
   branch) is a Bellman supersolution in the §5 sense: terminal
   exactness, public-branch harmonicity, focal optimism. This is the
   hypothesis of Theorem 2 and the parent asserts it in one line.
   Also rule on the parent's "retain a previously valid upper or
   refuse" rule for unaffordable branches — what "previously valid"
   may mean on the first pass (the trivial upper 1 is always valid;
   say whether installing it is a refusal or a fact).
3. **What the existing instruments already are, in the new
   vocabulary.** Verify and rule: `U_{a,0}^{God}` at a root action IS
   the U0 God-gap census's `GodUpper` for that coordinate
   (`solver/godgap.rs`); `U^{God}` at any belief node is what the
   in-solve horizon census prices (`solver/horizon.rs`,
   `doom_over_belief`, which is private today); Theorem 5 says
   `U_1^{God}` is the salvation-mask upper, so the queued U1
   salvation-mask slice is SUBSUMED by the hierarchy at k = 1 — say so
   or say why not; `L_0^π` is `viewer_success_mass`; the U0b census's
   cut at `cut_plays` is a PLY cut and the parent's is a FOCAL-DECISION
   cut — state precisely when they coincide (the orchestrator's
   reading: cut-4 at a viewer-leads-trick-4 root equals `U_{a,0}`
   because only hidden nodes lie between the root action and the
   frontier and God is harmonic over hidden nodes — which is why U0b
   found the cut-4 over-pricing equal to U0's Φ; cut-8 is NOT `U_{a,1}`
   in general because the viewer's next focal decision falls at a
   variable ply).
4. **`h_f` exposure.** FH3 wants the remaining focal depth "exposed and
   independently verified". Rule on the definition (§6: only focal
   decisions consume horizon; decided nodes have `h_f = 0`) including
   the interaction with the §5 decided cutoff — a node decided early
   has `h_f = 0` even though plays remain.
5. **The tie rule.** §7 materializes π_k under "a deterministic tie
   rule". The stack's declared rule is lowest tile index
   (`TieRule::LowestTileIndex`). Rule that π_k uses it, and that FH5
   (`V^{π_k} = L_k` through the independent evaluator) is the lower-side
   no-strategy-fusion gate exactly as the parent says.
6. **The anchors (FH8).** Confirm from `horizon_run1.txt` the three
   named anchors and give the builders their coordinates: (i) the h8-t3
   fixed-field root (exact Q* = 28859/29988, argmax 1-1; a trick-4 cut
   flips to 3-3); (ii) a trick-6 frontier row where a few-permille cut
   changed the root action (h8-t4 at contracts 36 and 39, cut 8); (iii)
   a contract-sensitive trick-5 specimen (h4-t4, cut 4, over-pricing
   13/65/85/105/33‰ at bids 30/33/36/39/42). The rule the parent gives
   and you must keep: do not pin the focal-horizon answer in advance
   beyond the soundness laws.
7. **Non-goals (§XVIII) are binding** — restate them as a ruling so no
   builder "helpfully" adds glue-coalition selection, the Ω×Θ lift, a
   live default change, or an arena claim.
8. **The engineering split is the orchestrator's** (Jason: "tasks and
   orderings are entirely your call"). The planned split, for your
   information and comment, not for ruling: FH1 = the generic engine
   (`solver/focal_horizon.rs`) for k ∈ {0,1,2} with gates FH1–FH6; FH2
   = budget honesty, interruption/resume, proof-state facts, exact
   suffix reuse (§23, §25, gate FH7); FH3 = the report of record over
   the corpus and the FH8 anchors. If the mathematics implies a
   different order (e.g. the interruption rule must exist before any
   budgeted run is trusted), say so.

## Discipline

- `ingest/` untouched. The parent and verifier are never edited (a
  transcription erratum is recorded in the companion, never repaired
  in the parent). Running the verifier inside `walt/math/` creates
  `__pycache__` — delete it; commit no transients.
- Exploratory tier on every page; tier labels on every substantive
  statement; nothing promoted.
- Cite `§n` of the parent; cite `FH-An` for provenance; where parent
  and companion differ the companion governs.
- Do not touch code. Do not write briefs for the builders (the
  orchestrator does; your rulings feed them).
- Commit on the current branch with a message starting `walt FH0:`;
  do not push, do not open a PR. Report back with: the FH-A rulings
  in one screen, the review's headline (three to six sentences), any
  correctness failure you found in the parent, and anything you
  believe the builders must not do.
