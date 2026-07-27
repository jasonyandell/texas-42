# Package Provenance and Merge Order

[Home](Home.md) · owns: how v0.7 and rec relate, and which package wins on each
topic · Related: [discrepancies](discrepancies.md).

## The two packages

| | **v0.7** (`…-v0.7`) | **rec** (`…-v0.7-reconstructed`) |
|---|---|---|
| Self-description | "proof-assistant boundary revision of the reachability-minimized foundation" (README §Version) | "reduced play/support foundation" (README §Version) |
| Dated | CHANGELOG: 2026-07-26; review dated 2026-07-26 | undated (no CHANGELOG) |
| Docs | 00, 10, 20 (5,428 ln), 30 (1,928 ln), 40, 50, **55 review, 60 HANDOFF, 65 ledger, 70 continuity, provenance/mandate** | 00, 10, 20 (6,002 ln), 30 (2,096 ln), 40, 50, **60 KERNEL** |
| Verifiers | verify_foundation, verify_minimality_and_reachability | same two **+ verify_reduced_kernel + audit_package** |
| MANIFEST | 17 entries, verifies clean | 14 entries, verifies clean |

`docs/10_RULES.md` and `verification/verify_foundation.py` are **byte-identical** across
the packages. Everything else diverges.

## Relationship verdict

**They are divergent siblings of a common v0.6 ancestor, not a linear succession.**
Both were produced with ChatGPT 5.6 Pro from the founding mandate
(v0.7 `provenance/RESEARCH_MANDATE_2026-07-23.md`); "reconstructed" reflects a rebuild
after a lost thread. The evidence:

1. **v0.7 = v0.6 + adversarial review + type-boundary repairs, no new mathematics.**
   Its CHANGELOG says v0.6's results and "all recorded verifier numerical outputs" are
   unchanged; what changed is the semantic type boundary (proof-irrelevant reachability,
   derived support views, total normal-form well-formedness, external-verification
   boundary) plus new docs 55/60/65/70 and the archived mandate.
2. **rec = v0.6 + a major new research layer, *without* the v0.7 repairs.**
   rec adds Math §3.10 (unscored mechanics classes), §7.7.1 (matching kernel),
   §7.13.7 (symbolic reachability), §7.14.1–.2 (dynamic support, monotonicity),
   §7.16 (folded trick / reduced viewer kernel), §12.7.1 (D4 gauge), §12.10 (future
   equivalence), a rewritten §15, ~30 new ledger rows, and two new verifiers.
   But it still contains, verbatim, the defects v0.7's review (55_V06_REVIEW §3)
   attributes to **v0.6**: identity-bearing reachability certificates
   (rec Exec §10, §25: "Reachability-certificate equality compares exact lifecycle
   provenance"), `cells` stored inside `MechanicalState` (rec Exec §15), `NativeHand`
   rather than `NativeHandView` (rec Exec §20), the "dependency-free" overclaim in
   `verify_minimality_and_reachability.py`'s docstring (it imports helpers from
   `verify_foundation`), the "outer reachability **certificates**" naming that v0.7
   renamed to "necessary outer **profiles**", and the unescaped `|U|` pipes that break
   the claim-ledger Markdown tables.
3. Neither package references the other's unique content. v0.7's review of v0.6 never
   mentions the reduced-kernel results; rec never mentions proof-irrelevance or the
   mechanization ledger.

So the fork point is v0.6: one branch hardened the boundary for mechanization (v0.7),
the other extended the mathematics (rec). Neither strictly supersedes the other.

## Authoritative merge order

For each topic, the canonical source and why:

| Topic | Canonical | Why |
|---|---|---|
| Rules profile | either (identical) | byte-identical `10_RULES.md` |
| Thesis/scope framing | rec `00_THESIS` for the factorization statement; v0.7 for the honesty/standards prose | rec's viewer-relative factorization reflects its proved refinements; both agree elsewhere |
| Core mathematics Math §§1–13 shared sections | either; **v0.7 for notation** (`\lvert…\rvert` fixes) | mathematically identical where shared; v0.7 fixed table-breaking notation |
| New mathematics (Math §3.10, §7.7.1, §7.13.7, §7.14.1–.2, §7.16, §12.7.1, §12.10, §15) | **rec** (only source) | exists nowhere else; finite receipts pass |
| Executable type discipline (equality, certificates, derived views, naming) | **v0.7 Exec** | rec retains the exact defects v0.7's adversarial review repaired; rec's own Math §7.16.4 ("select one semantic source… storing both invites inconsistency") agrees with v0.7's repair |
| Reachability vocabulary ("necessary outer profile", proof-irrelevant proposition) | **v0.7** | deliberate correction of a naming hazard (55_V06_REVIEW §3.7) |
| Verifier independence description | **v0.7** | matches the code in both packages |
| New executable surfaces (Exec §17A/§17B, symbolic certificates, reduced kernel API) | **rec**, *re-expressed under v0.7's type discipline* | new content, but its state/equality treatment must be repaired per v0.7 |
| Claim ledger | **union**: v0.7 rows (incl. TYPE-01..03, TRUST-01) + rec-only rows (ALG-20..24, PLAY-12..17, CELL-09A, REACH-14..16, TRANS-08..14, SYM-04, QUO-09..11, FAC-02) | no shared-ID contradictions except wording refinements — see [discrepancies](discrepancies.md) |
| Proof-assistant plan | **both**: v0.7 60/65 for trust boundary, milestones, priorities; rec 60 for the K0–K15 dependency spine covering the new math | complementary; see [proof-assistant-plan](proof-assistant-plan.md) |
| Open problems | **rec where it made progress** (OPEN-01, OPEN-12 rewrites), v0.7 otherwise | rec's rewrites reflect genuinely new theorems |
| Implementation slice 01 | **rec 50** (superset: adds looped-K7, competitive ordinals, unscored transports) | strictly extends v0.7's assignment consistently |
| Provenance/continuity (mandate, crosswalk, deferred annexes) | **v0.7** (only source) | rec has none |

**Synthesis rule used throughout this wiki:** rec's mathematics is treated as the
current mathematical frontier; v0.7's *type boundary and vocabulary* are treated as
normative for any implementation or mechanization. Where a rec-only theorem is quoted,
its executable consequences are stated in v0.7 vocabulary (proof-irrelevant
reachability, derived views, "necessary outer profile").

## Provenance artifacts

- v0.7 archives the verbatim founding mandate
  (`provenance/RESEARCH_MANDATE_2026-07-23.md`, non-normative), which required
  adversarial repair of overclaims from the originating repository (`jasonyandell/mk5-main`,
  PRs #84/#91/#92/#95) and explicitly demoted implementation gates (Atlas C1–C6, Walt,
  Hoyt, role/threat gates) to evidence, never axioms.
- v0.7 `70_THREAD_CONTINUITY.md` maps every founding claim to its current home and
  lists a 12-point drift acceptance test (non-normative).
- rec `audit_package.py` enforces structural hygiene (required files, no
  project-specific names like Atlas/Walt/Hoyt/Forge/mk5-main, claim-ID uniqueness:
  255 IDs, link integrity) — a package-quality gate v0.7 lacks.
