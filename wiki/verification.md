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
