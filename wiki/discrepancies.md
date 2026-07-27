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

- Not a package conflict but an operational trap: running the rec verifiers creates
  `verification/__pycache__`, after which `audit_package.py` **fails** its
  "no transient Python files" check. On a clean tree it passes and reproduces
  `AUDIT_OUTPUT.txt` exactly. **Resolution:** run the audit first or delete
  `__pycache__` before auditing; note that the checked-in `ingest/` copies currently
  contain `__pycache__` directories (not covered by the MANIFESTs). Confidence: **high**.

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
