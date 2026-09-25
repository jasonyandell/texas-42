# Discrepancies Between the Two Packages

[Home](Home.md) · owns: every disagreement found, with resolution and confidence ·
Related: [package-provenance](package-provenance.md).

"v0.7" and "rec" as defined on [Home](Home.md). D1–D6 are the substantive conflicts;
D7–D15 are structural/editorial; D16–D17 are exchange-side notes (provenance; scope). Since
`ingest/` is immutable, these resolutions are realized *here in the wiki*, not by
editing the packages.

## D1. Reachability evidence: identity-bearing vs proof-irrelevant

- **rec Exec §10, §25**: every reachable state "carries an opaque certificate tied to
  one exact `ContractedHandOrigin`"; "Reachability-certificate equality compares exact
  lifecycle provenance and is distinct from contracted-play physical equality."
- **v0.7 Exec §10, §18, §25 + Math §7.13 [CLARIFICATION]**: reachability is a
  proof-irrelevant *proposition*; witnesses are erasable audit artifacts excluded from
  equality, hashing, serialization, and transition. v0.7's 55_V06_REVIEW §3.1 explains
  why the certificate design is wrong (proof terms split semantic states; provenance
  can leak hidden information; quotient theorems acquire proof-term side conditions).
- **Resolution: v0.7.** Confidence: **high**. The review's argument is independently
  checkable, and rec's own Math (§7.13 "a validator tag or witness may be erased after
  certification") points the same way — only rec's *executable spec* disagrees.

## D2. Derived support views vs stored cells

- **rec Exec §15**: `MechanicalState` stores `cells: RuleDerivedCellSystem` as a field;
  `NativeHand` stores both `physicalSupport` and a fiber view with a coherence invariant.
- **v0.7 Exec §15, §20, §25 + Math §7.10 [COROLLARY: zero supplemental support state]**:
  cells, reduced support, normal form, and fiber are *derived views* of the semantic
  fields; caches are excluded from equality; the type is renamed `NativeHandView`.
- **Resolution: v0.7.** Confidence: **high**. rec's own Math §7.16.4 states "an exact
  implementation should select one semantic source and treat the other as a derived or
  cached view; storing both as independent state invites inconsistency" — rec's spec
  simply predates that discipline.

## D3. "Outer reachability certificate" vs "necessary outer profile"

- **rec** (Math §7.13.6, Exec §18, ledger REACH-11/11A, verifier output line
  "outer reachability certificates: …"): calls the 46-bit upper-bound object a
  *certificate*.
- **v0.7**: renames it `ReachabilityOuterNecessaryProfile` / "necessary outer
  profiles"; passing its check is necessary only and "cannot construct a reachable
  state" (CHANGELOG; 55_V06_REVIEW §3.7).
- **Resolution: v0.7 naming.** Confidence: **high**. The object provably admits
  members that decode to infeasible or unreachable support (both packages' Math
  §7.13.6 says so); "certificate" is a soundness hazard.
- **Exchange note (dispatch 005):** the external 005 audit response used the deprecated
  "outer certificates" naming; the D3 resolution (v0.7 "necessary outer profiles")
  stands and all numbers agree — cosmetic only.

## D4. Verifier independence overclaim

- **rec `verify_minimality_and_reachability.py` docstring**: "It is dependency-free."
  In fact it imports abstract-world helpers from `verify_foundation` (same as v0.7's).
- **v0.7**: docstring and README state the dependence explicitly ("the two entry
  points are therefore not independent implementations").
- **Resolution: v0.7 wording.** Confidence: **high** (directly verifiable from code).

## D5. Is reachable support transition-sufficient?

- **v0.7 REACH-03A [BOUNDARY]**: "Reachable support … omits mechanical fields required
  for legal actions and successor transitions"; Exec §18: "no API may define an exact
  game transition from a standalone support identifier and action."
- **rec REACH-03A [BOUNDARY] + TRANS-08 [THEOREM — proved]** (Math §7.14.1): standalone
  support is still not a complete game state, **but** given declaration, actor, played
  domino, and current led context/lead boundary, the exact successor support *is*
  uniquely determined — support becomes a closed dynamic transition state.
- **Resolution: rec — this is a refinement, not a contradiction.** The two rows agree
  on the standalone claim; rec adds a proved positive result under explicit extra
  inputs (finite-verified on 1,331 supports × 170,058 typed observations).
  Confidence: **high** for compatibility; **medium-high** for the theorem itself
  (prose proof + tiny-domain exhaustion; not yet mechanized).

## D6. Both packages claim to be "v0.7" with different content

- v0.7 README §Version: "the proof-assistant boundary revision…". rec README §Version:
  "the reduced play/support foundation. It promotes the … support normal form … to an
  exact dynamic state, … folds the open trick, … adds symbolic trace reachability…".
- **Resolution:** treat the labels as branch names, not versions: **v0.7-boundary**
  and **v0.7-kernel**. A future v0.8 should be the union: rec's mathematics under
  v0.7's type boundary. Confidence: **high** (this is the whole point of
  [package-provenance](package-provenance.md)).

## D7. OPEN-01 and OPEN-12 wording

- **OPEN-01** — v0.7: minimal complete mechanical state not established. rec: the
  reduced viewer kernel *is* exact, and the global transition minimum is defined by
  future equivalence per output contract; only the *equality* of kernel and quotient
  is open. **Resolution: rec** (progress supersedes). Confidence: **medium-high**.
- **OPEN-12** — v0.7: no closed-form local criterion replaces exact legal-witness
  validation. rec: same, but symbolic public-trace replay removes the need to supply a
  hidden deal. **Resolution: rec.** Confidence: **high** (REACH-14/15 proved +
  finite-verified).

## D8. Claim-ledger row sets

- v0.7-only rows: TYPE-01, TYPE-02, TYPE-03, TRUST-01 (type/trust boundary), and the
  `\lvert…\rvert` notation repairs.
- rec-only rows: ALG-20..24, PLAY-12..17, CELL-09A, REACH-14..16, TRANS-08..14,
  SYM-04, QUO-09..11, FAC-02.
- FAC-01 differs in wording (v0.7: "certified mechanical/support residue …"; rec:
  "exact physical/support kernel …") — same content at different refinement stages.
- **Resolution: union**, with rec's FAC-02 as the sharper final factorization and
  v0.7's TYPE/TRUST rows kept normative. Confidence: **high**.

## D9. 00_THESIS factorization statement

- v0.7 §1: native factorization = physics + objective marked world + player
  information + exact support fiber + belief + policy + utility.
- rec §1: *viewer-relative* factorization = declaration algebra + owned marked hand +
  minimal exact hidden support + folded physical play residue + utility accumulator +
  retained evidence + augmented belief (field/utility/strategy as typed parameters;
  the objective world remains the latent witness).
- **Resolution: rec**, as the proved sharpening (backed by rec Math §7.16, §15);
  v0.7's form remains a correct coarser statement. Confidence: **medium-high**.

## D10. Executable spec §17/§18 surfaces

- rec adds Exec §17A (minimal exact support transition state) and §17B (folded trick /
  reduced viewer kernel), plus symbolic-trace certification in §18; v0.7 instead has
  the certified-type discipline (`UncertifiedMechanicalState` / proof-irrelevant
  subtypes) and context-relative bit-ceiling table.
- **Resolution: union** — rec's new surfaces re-expressed with v0.7's certified-type
  discipline (see D1/D2). Confidence: **high**.

## D11. First implementation slice (50_CODEX)

- rec extends the slice: `pip_sum`, `competitive_ordinal`, `PipTrumpTransport`,
  `unscored_mechanics_class`, looped-K7/antidiagonal tests, 49-transport check, and
  forbids modifying three verifier scripts (v0.7: two).
- **Resolution: rec**, since it is a consistent superset; keep v0.7's sentence that
  the verification files are "external proof receipts … not proof-assistant kernel
  proofs." Confidence: **high**.

## D12. Missing-document asymmetry

- rec lacks: CHANGELOG, 55_V06_REVIEW, 60_PROOF_ASSISTANT_HANDOFF, 65_MECHANIZATION_LEDGER,
  70_THREAD_CONTINUITY, provenance/. v0.7 lacks: 60_PROOF_ASSISTANT_KERNEL,
  verify_reduced_kernel.py, audit_package.py, AUDIT_OUTPUT.txt.
- **Resolution: union**; the two 60-series docs are complementary
  (see [proof-assistant-plan](proof-assistant-plan.md)). Confidence: **high**.

## D13. Markdown notation in the claim ledger

- rec uses raw `|U|`, `|J|`, `|Φ|` inside table cells (splits Markdown columns —
  the exact defect v0.7 CHANGELOG fixed with `\lvert\cdot\rvert`).
- **Resolution: v0.7 notation.** Confidence: **high** (mechanical).

## D14. Verifier output wording

- Both packages' verify_foundation outputs are identical and all scripts PASS
  (fresh runs match committed outputs; see [verification](verification.md)).
  The only divergence: v0.7's minimality verifier prints "necessary outer
  reachability profiles: …" where rec prints "outer reachability certificates: …"
  (same numbers). **Resolution: numbers agree; adopt v0.7 wording per D3.**
  Confidence: **high**.

## D15. rec audit vs generated `__pycache__`

- Not a package conflict but an operational trap: running the rec verifiers *in
  place* with a default `python3` invocation writes `verification/__pycache__`, after
  which `audit_package.py` **fails** its "no transient Python files" check
  (`check_no_transients`, which rejects any `__pycache__` directory or `.pyc` file
  anywhere in the package). On a clean tree it passes and reproduces
  `AUDIT_OUTPUT.txt` exactly.
- The checked-in `ingest/` tree is clean and always has been: the repository has never
  tracked a `__pycache__` directory or `.pyc` file under `ingest/` (zero tracked paths
  match `pycache`; `find ingest -name __pycache__` finds nothing at c00717d1; re-checked
  2026-09-12). An earlier wording of this entry said the copies "currently contain"
  such directories — that described a working tree after an in-place run, never the
  repository, and is withdrawn. ([verification](verification.md) caveat 1 and
  [FINDINGS](FINDINGS.md) §3 carry the same correction.)
- **Resolution:** never run the verifiers in place. Run them from a copy
  (`cp -r ingest/<package> /tmp/…`) or with `python3 -B` (which writes no bytecode);
  either keeps the audit green. Measured 2026-09-13 on this machine (re-run of the
  2026-09-12 measurement): rec package copied out of the tree,
  `verify_foundation.py` / `verify_minimality_and_reachability.py` /
  `verify_reduced_kernel.py` run with `python3 -B` in 4.6 s / 3.3 s / 8.2 s, all PASS
  with every output line identical to the committed `VERIFICATION_OUTPUT.txt` (which
  adds only its three per-script headers), zero `__pycache__` created, and
  `audit_package.py` then PASS in 0.02 s reproducing `AUDIT_OUTPUT.txt` byte-for-byte
  (255 claim IDs, 8 documents, 17 kernel markers); the rec `MANIFEST.sha256` verifies
  14/14. Confidence: **high**.

## D16. Exchange-side SHA provenance blemishes (non-load-bearing)

Two exchange responses cited a verifier SHA-256 that matches no retrievable artifact;
in both the adjudication treats the *inline fenced program* (saved under
`exchange/adjudication/programs/`) as the authoritative artifact of record, and both
pass with the exact claimed statistics, so the taint is confined to the "identical
download" narrative and does not affect either verdict.

- **Dispatch 003**: response-claimed SHA-256 `8e9992ed…` matches neither the fenced
  program (`b4ae4913…`) nor any retrievable copy (sandbox download link dead).
- **Dispatch 004**: cited sandbox SHA-256 `c56c0c50…` matches the inline fenced program
  (actual SHA-256 `13420aa7…`) under no normalization; the download link is dead. The
  inline block at `exchange/adjudication/programs/004.py` is the sole artifact of record.

**Resolution:** fenced/inline block is authoritative in both cases. Confidence: **high**
(does not affect the verdicts).

## D17. Inbox/001 step 11's "exactly 559,316,142" (scope, not error)

001's proof said the no-void reachable count was "exactly 559,316,142". REACH-20
(dispatch 008, CONFIRMED) shows the full no-void slice is saturated at 624,892,870;
001's integer is exact *as the count of its regular-module grammar family* — a proper
subfamily, undercount 65,576,728 (reconciles term-by-term: 008 reproduces 001's
per-block partials). REACH-17's floor is unaffected (floors need only membership,
not completeness), but any future reading of 001's "exactly" as slice-completeness
would be wrong. **Resolution:** scope annotation; both integers stand at their own
scopes. Confidence: **high**.

## Mechanization status (which resolutions the kernel realizes)

Tier: proof-assistant kernel bookkeeping as of commit d190b26 (2026-08-02), re-checked
2026-09-12; map in [lean-row-index](lean-row-index.md). These are the places where a
resolution above stopped being a wiki ruling and became a checked definition — none
of them changes a corpus status.

- **D1 (proof-irrelevant reachability) — realized.** PA-D10 `CertifiedState :=
  {P // Reachable K v P}` with `ext`/`ext_iff` (`lean/Texas42/Reachability.lean`):
  equality flows through the projection alone; the witness is propositional and
  erasable. The identity-bearing certificate design of rec Exec §10/§25 has no Lean
  counterpart, by design.
- **D2 (derived views, not stored cells) — realized by construction.** `Cells.lean`
  computes pool, allowed sets and capacities from the public record plus the viewer
  hand (PA-C02), and `Play.lean` derives `tricksDone`/`scoredTiles` (TYPE-02); no
  Lean structure stores a cell system beside the state it derives from. The total
  well-formedness contract (TYPE-03) is PA-D02 `SupportNF.WellFormed`.
- **D3 (vocabulary) — respected.** No Lean source under `lean/Texas42/` uses the
  word "certificate" for an outer profile (grep 2026-09-12: no occurrence outside
  the walt-facing trick-1 modules); the TYPE-01 objects are named `CertifiedState`
  and "witness". The 46-bit outer language itself (REACH-11) is not mechanized
  (PA-D18, priority 4).
- **D5 (TRANS-08 dynamic sufficiency) — unchanged: not mechanized.** No `PA-` row
  exists for any rec-only TRANS row; the confidence caveat above stands
  ([support-dynamics](support-dynamics.md)).
- **D7 (OPEN-01) — the COLLAPSE (x:003) is exchange tier, not kernel.** Nothing of
  the reduced viewer kernel or future-equivalence quotient is in Lean
  ([reduced-viewer-kernel](reduced-viewer-kernel.md)).
- **D8 (ledger union) — the kernel side is v0.7-only.** The mechanization ledger
  exists in v0.7 alone; rec's K0–K15 spine (its `60_PROOF_ASSISTANT_KERNEL.md`)
  guides layering but carries no rows ([package-provenance](package-provenance.md)).
- **D16/D17 — exchange-side; no kernel bearing.**
