# Verification Scripts and Fresh-Run Results

[Home](Home.md) · Sources: `verification/` in both packages. Fresh runs performed
2026-07-26 with Python 3.12 on this machine.

## Status: everything passes

| Script | Package(s) | Result | Matches committed output? |
|---|---|---|---|
| `verify_foundation.py` (1,850 ln, byte-identical in both) | both | **PASS** | yes — exact |
| `verify_minimality_and_reachability.py` | both (differ only in docstring + "profiles"/"certificates" wording) | **PASS** | yes — exact |
| `verify_reduced_kernel.py` (871 ln) | rec only | **PASS** | yes — exact |
| `audit_package.py` | rec only | **PASS on a clean tree**; FAILS if `verification/__pycache__` exists (see below) | yes — exact, on clean copy |

"Exact" means the committed `VERIFICATION_OUTPUT.txt` is the concatenation of the
actual stdout plus `=== script name ===` header lines — no numeric or textual drift.

## What each script exhausts

### `verify_foundation.py` (both packages)
28 dominoes; 9 declarations; ordered-deal count 472,518,347,558,400; 737,100
unique-winner cases agreeing with an independent prose-rule resolver; auction census
for caps 1..7 (2380…3214) with caps 5/6/7 identical; 66,968 tiny Hall systems;
fiber-count coefficient/recurrence agreement; 22,620 exact uniform sampler
probabilities; 785,736 marginal-edge checks (world projection vs Hall); 512 native
capacity-DP profiles with the 512/1,533/1,344/48 bounds attained; 14,412+56,460+56,460
typed transitions on tiny systems; 972 reachable cell/deal parity prefixes; 864 typed
support transitions (648 hidden nonincrease, 216 viewer equality); count/trick/pip
transport facts; and the full 90-world history witness (fiber size, anchor Q-tables,
both posteriors, expected Q, make probabilities, opposite best leads).

### `verify_minimality_and_reachability.py` (both)
Normal-form decode equality and SCC compilation on the tiny corpus (14,578 feasible;
2,151 essential exclusions; rank/unrank on 22,620 worlds); native ternary signature
census (136,514 / 1,667,666 matrices / max 114; S₃: 23,842 orbits, 296,721 matrices,
279,048 stabilizer orbits, max 103); the full-schema census
(det 8,102,258,940,222,814; bin 11,495,078,055,913,018,482; ter
1,830,955,704,129,296,418,354,864; total 1,830,967,207,309,611,271,596,161 ⇒ 81 bits);
50 reachable capacity profiles; 7 lead contexts with fiber sizes 1..7; projected
schedule counts A_j/T_{j,1}/T_{j,2}; the outer-profile counts
(7,124,838,074,989/declaration; 64,123,542,674,901 total ⇒ ≤46 bits; context ceilings
43/43/40); the 44,352,165 no-void floor ⇒ ≥26 bits; the feasible-but-unreachable
witness (450 generators, 2 matches, all lead tiles hidden); interval 26..46;
0 supplemental bits given a certified mechanical state.

### `verify_reduced_kernel.py` (rec only)
Looped-K7/count-antidiagonal identity; 49 ordered unscored pip-trump transports with
307,328 contextual order comparisons and 3 mechanics classes; folded trick over all
737,100 trick cases + 2,211,300 sequential updates (max competitive chain 13; pending
counts 6); 84 legal open-trick actor/capacity shapes; 3,132 score-recovery prefixes;
support-normal dynamics on 1,331 feasible supports (170,058 typed observations,
157,809 nonempty successors, 1,406,592 holder-edge monotonicity checks); 108 symbolic
complete hands / 3,024 transitions / 6,804 = 108·63 edge deletions; 8 oriented
dihedral frames; future-equivalence minimization on 5,898 machines / 17,560 state
pairs.

### `audit_package.py` (rec only)
Structural hygiene: required files present; UTF-8/no control chars (14 files, 14,390
lines); local Markdown link integrity (14 links); 255 unique claim IDs; 8
project-neutral docs free of forbidden implementation names (Atlas, Walt, Hoyt, Forge,
mk5-main, CoordinateV1); 17 cross-document kernel markers; **no transient Python
files**.

## rob (Rust) — independent reproduction, slice 01

Recorded 2026-07-27. **rob** is this repository's exact Texas 42 engine
([rob/BRIEF.md](../rob/BRIEF.md) — the binding slice-01 assignment: rec's mathematics
under v0.7's type discipline, extended through the support normal form + capacity
DP). `rob/ci/check.sh` is green end-to-end (fmt; clippy `-D warnings
-D float_arithmetic`; no-float grep; vocabulary grep; full release test suite); the
four stage receipts under `rob/receipts/` are byte-diffed in CI against fresh runs.

**Status label: Finite verification receipt / conformance evidence — not a new
mathematical status.** Per the corpus's own claim discipline (TRUST-01 and §0 of
either Math package), these runs re-derive and confirm the committed ingest receipt
numbers in an independent implementation. They strengthen confidence in the finite
claims and in rob's conformance; they change no claim's status and are not
proof-assistant kernel proofs.

### What was reproduced

| Receipt | Stage | Headline exact integers |
|---|---|---|
| `verify_algebra` | S1 declaration algebra | 737,100 unique-winner and 737,100 prose-agreement cases; counts 35/7/42; 56,448 `BEATS` equivalences; 5,040 → 2 count-preserving permutations (`2↔3` a game-order isomorphism only between layers 2 and 3); 49 unscored transports / 307,328 order comparisons; 3 mechanics classes; competitive-ordinal max 13 |
| `verify_objective` | S2 objective machine | 472,518,347,558,400 ordered deals; 399,072,960 hidden assignments; auction census (2380, 3060, 3196, 3213, 3214, 3214, 3214) with maxima (1, 2, 3, 4, 5, 5, 5) and caps 5/6/7 identical; conservation 42 with `P_D = 42` ⇔ seven-trick sweep |
| `verify_support` | S3 cells + losslessness | initial cells 21/7/63; typed-update algebra 14,412 leads + 56,460 follows + 56,460 sloughs; 972 parity prefixes with exact fiber ↔ replayed-deal set equality; transitions 864 (648 hidden nonincrease, 216 viewer equality); 90-world support witness via both auction histories |
| `verify_normal_form` | S4 NF + capacity DP | tiny corpus 66,968 / 14,578 feasible / 22,620 worlds; three counting routes ≡ enumeration; DP bounds 512/1,533/1,344/48 exactly attained, max count 399,072,960; 785,736 marginal edges; 22,620 SCC compilations; 2,151 essential exclusions; 22,620 rank/unrank; ternary census 136,514 signatures / 1,667,666 matrices / max 114 with S₃ quotient 23,842 / 296,721 / 279,048 / max 103; 81-bit census (total 1,830,967,207,309,611,271,596,161, computed from the §7.12.5 formulas); 50 capacity profiles, each realized by an explicit legal witness prefix; 44,352,165 no-void floor (≥ 26 bits); 0 supplemental support bits |

### Independence conditions

- No ingest Python was read or translated. Only the committed
  `VERIFICATION_OUTPUT.txt` numbers and corpus-shape parameters were consumed, as
  rob/BRIEF §5's guardrails permit; the reproduction is Rust (pinned 1.95.0,
  runtime dependencies `num-*` only), so "never copy the verifiers" is structurally
  true.
- The independent prose-rule resolver lives in `rob-verify` and shares nothing with
  the algebra implementation beyond domino identity, the declaration enum, and seat
  labels (BRIEF §1 D4); it agrees on winner **and** trick points in all 737,100
  cases.
- rob's S3 corpus generator is its own deterministic construction; only the corpus
  *shape* (972 = 9·12·9) is an ingest-anchored assertion.

### Enforcement mechanisms

Each of INV-1..INV-10 (rob/BRIEF §5) has a named test, lint, or CI check:
`inv_derived_coherence` (derived-not-stored), `inv_projected_equality`,
`inv_no_reachability_field` (proof-irrelevant reachability, D1),
no-float grep + clippy `float_arithmetic` deny + overflow checks in every profile
(exact arithmetic), committed receipts byte-diffed in CI (counts-are-CI),
`inv_reachable_implies_feasible` (including a `should_panic` certified path),
`inv_id_not_rank` plus a `compile_fail` doctest that `DominoId` has no `Ord`,
`inv_event_replay` (one source of truth), a `compile_fail` doctest that `DealWorld`
does not convert to `RemainderWorld` (type-distinct domains), and a vocabulary grep
forbidding certificate-style identifiers (D3: "necessary outer profile", never
"certificate").

### Frozen generator-specific value

The with-voids parity count is **970** (`FROZEN_WITH_VOIDS` in
`rob/crates/verify/src/s3.rs`). This is a property of rob's own deterministic
generator, frozen at first green run per BRIEF §8. It coincides numerically with the
ingest generator's 970 but is **explicitly not an ingest-corpus number** and carries
no cross-implementation meaning; only the 972 corpus shape does.

Slice 02 (support dynamics: TRANS-08..14, REACH-06..16) is scoped in rob/BRIEF §4
and has **not** been begun.

## Caveats

1. **The `__pycache__` trap** ([discrepancies D15](discrepancies.md)): running the
   verifiers creates `verification/__pycache__`, which then makes `audit_package.py`
   fail its no-transients check. The `ingest/` copies in this repo currently contain
   `__pycache__` directories (generated; not in the MANIFESTs). Audit on a clean copy
   reproduces `AUDIT_OUTPUT.txt` exactly.
2. **The two verifier entry points are not independent**: the minimality script
   imports abstract-world helpers from `verify_foundation`. v0.7 states this; rec's
   docstring wrongly claims "dependency-free" ([discrepancies D4](discrepancies.md)).
3. **Receipts are not kernel proofs** [TRUST-01, Boundary]: `PASS` output supports
   finite claims but must be re-proved or reflected inside a proof assistant
   ([proof-assistant-plan](proof-assistant-plan.md)).
4. Both MANIFEST.sha256 files verify clean (17 and 14 entries).
5. Runtimes are minutes-scale; all scripts are stdlib-only Python.
