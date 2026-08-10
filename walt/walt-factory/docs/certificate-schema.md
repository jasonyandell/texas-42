# walt lesson certificate schema — schema-v1

**Tier: exploratory.** A certificate is computed evidence about a declared
finite domain, never an axiom and never a promoted status (TRUST-01: an
external checker's PASS is evidence, not an axiom, in both directions).
This document is the normative reference the independent checker (a
planned Python implementation, a separate work unit) will be written
against: a reader with this document, a certificate file, the receipt
file, and the preserved probe suite can check every record marked CHECKED
without reading any Rust.

Versioning: every certificate's first line names `schema-v1`. Any change
to record grammar, canonical-key grammar, or coverage vocabulary bumps
the version; checkers refuse versions they do not know.

## 0. Normative external references

- **Receipt**: `rob/receipts/verify_player.txt` — the 13-hand rob
  self-play receipt (read-only ground truth; pip-trump only, a caveat
  every corpus statistic inherits). All kernels reconstruct from it by
  replay.
- **Game semantics** (legality, follow, trick winner, declarations,
  count points): as embodied in the frozen probe suite
  `walt/probes/exp5/` (the preserved second implementation). The checker
  is expected to be built on those probes; they are frozen — never
  edited, never copied into an implementation.
- **Atom semantics for exp3a-covered cells**:
  `walt/probes/exp3a/lambda_probe_v3.py` Part 1 (the 22-atom registry).
  Holder/team coordinates are the registry's `holder`/`team` atoms;
  control shapes are its ten named shapes under the kernel's derived
  parameters (valued tile = highest count-point unseen tile, ties by pip
  sum then tile index; decisive context per the probe's derivation).
- **Beater cells** (`beaters(t)` vectors, `beaters-total(t)`) are
  walt-native: `beaters(t)[i]` = |THREAT(t) ∩ hidden hand i| (THREAT =
  the declaration's set of tiles that beat `t` when `t` is led),
  `beaters-total(t)` = the vector summed. No independent implementation
  exists today, so records depending on them are UNCHECKED-EXTERNALLY.

## 1. Certificate identity: the content key

A lesson's identity is its **projected content** — never origin, trace,
or discovery epoch. The canonical key is the exact string:

```
content-key-v1
verdict: <verdict rendering>
labels: <label projection>
domain: <DomainSpec rendering>
cells:
  <cell rendering>        (one per line, sorted lexicographically;
  ...                      "  (empty)" for an empty implicant)
```

- The **label projection** is the operator pair alone — e.g. `(C,
  minimax-omniscient)` — or the checker rung's fixed field/valuation
  string. **Grade is not identity** (adjudicated): the verification rung,
  weighting, and sample seeds/draws are evidence of one derivation; a
  sampled derivation and an exhaustive derivation with the same projected
  content are the SAME lesson — one working-set entry whose quotable
  grade is the MAX over its archived verifications, each verification
  keeping its own grade in the archive.
- The **DomainSpec rendering** is e.g. `receipt corpus, hands 0-12, all
  seats, tricks 3-6, fiber <= 40000`. The verified domain is part of
  identity: the same implicant verified on a different domain is a
  different lesson.
- Cell renderings are the implicant grammar of §5 below.

The certificate header carries the FNV-1a-64 hash of the canonical
string (`content-key hash`) and the canonical string itself (prefixed
`  | `). The hash is a filename handle; identity is the full string.
Filenames are `cert_<kind>_<hash-hex-16>.txt`, `kind` ∈
`refutation | win | checker` (derived from the verdict).

FNV-1a-64: `h = 0xcbf29ce484222325`; per byte `h ^= byte;
h = h * 0x100000001b3 (mod 2^64)`.

## 2. File layout

Header (three parts): the schema line, the content-key hash line, the
canonical-content block. Then the ELEVEN records, in order, each opening
with `[k/11] <record-kind> — <coverage>`:

 1. `kernel-reconstruction`
 2. `world-enumeration-count`
 3. `field-and-belief`
 4. `policy-witness`
 5. `terminal-feature-law-scalar`
 6. `rational-affine-segments`
 7. `breakpoint-continuity-witnesses`
 8. `response-class-labels`
 9. `descriptor-truth-vectors`
10. `cell-purity-counterexample-witnesses`
11. `information-price-segments`

A record inapplicable to the lesson kind is still PRESENT, with
`NOT-APPLICABLE (<reason>)` in its header and no body — absence is
indistinguishable from omission, so nothing is ever omitted. The file
ends with a `coverage summary` line restating the checked subset: the
certificate claims exactly that subset and nothing more.

## 3. Coverage vocabulary

- `CHECKED (<checker>)` — an independent implementation can verify the
  record today; the annotation names it. "Well-formedness only" marks
  declaration records (labels), where checking means grammar + internal
  consistency, not computation.
- `UNCHECKED-EXTERNALLY (<reason>)` — no independent implementation
  exists today. H value rows carry this until an independent H checker
  is registered; the annotation is emitted from the live H-checker
  registry, not hard-coded. (The registry also mechanically blocks the
  first H-rent-driven deletion until a checker registers — that
  enforcement lives in the ledger, in types.)
- `NOT-APPLICABLE (<reason>)` — the record kind does not apply to this
  lesson kind (see per-record notes).

## 4. Decision points and kernels (records 1, 2)

A **decision point** is (hand, seat, trick, ply): the named seat's play
in the named trick of the named receipt hand. To reconstruct:

1. Replay the receipt hand's tricks 1..trick-1 completely.
2. Replay the named trick's plays before the named seat (the prefix);
   `ply` = the seat's position in the trick (0 = led).
3. The **kernel** is: the viewer's remaining hand (public from the
   receipt), the hidden pool (all remaining tiles not in the viewer's
   hand), per hidden seat its capacity (remaining hand size) and its
   observed voids (contexts it failed to follow earlier in the replay,
   under the declaration's effective incidence).

The **fiber** is every assignment of the pool to the three hidden seats
meeting capacities and voids exactly. Record 1 lists, per basin
decision, the tuple plus `decl`, `viewer-hand` size (= horizon), and
pool size. Record 2 lists the fiber count and matched-world count per
decision, plus the domain line: total decisions/worlds, and the
EXCLUDED in-range decision count (fiber-cap exclusions — exclusion,
never sampling), with the standing control-bias annotation when the
count is nonzero (the excluded set skews low-control; fiber size
anti-correlates with focal control, exp5 covariate).

**Check** (both records): re-replay, re-derive the kernel, re-enumerate
or re-count the fiber (the probe suite's exact fiber machinery), compare
counts.

## 5. The implicant grammar (records 9 and the canonical key)

Two-sorted cells; every cell is partial — where its precondition fails
it is UNSATISFIED, never defaulted:

- Decision sort: `hand=<n>`, `seat=<S0..S3>`, `decl=<declaration>`,
  `role=<declaring|defending>`, `ply=<0..3>`, `horizon>=<n>`,
  `horizon<=<n>` (horizon = viewer tiles remaining).
- Atom sort (world-selecting): `<atom>=<value>` equality cells over the
  union vocabulary (holder(t) = which hidden slot holds `t`; team(t) =
  whether the viewer's team holds `t`; beaters(t) = the per-slot beater
  vector `[a,b,c]`; the ten exp3a control shapes), and one-sided bounds
  `beaters-total(t)>=k` / `<=k`, `opp-beaters>=k` / `<=k` over the
  registered numerics. A bound over an undefined numeric is unsatisfied
  per world.

Matching: a decision matches when every decision-sort cell holds; a
world of its fiber matches when every atom-sort cell holds there. Pair
verdicts (refutation, win) apply at a decision when the domain gate
passes, the selectors resolve, and at least one world matches; the
checker verdict requires atom cells to hold at EVERY fiber world.

**Canonical world order** (used by record 9): a world's sort key is the
concatenation, over the kernel's hidden slots in slot order, of that
slot's hand as an ascending list of tile indices, each hand terminated
by the sentinel 255; worlds are sorted lexicographically by key. Any
enumerator can reproduce the order by enumerating its own way and
sorting — no walt-internal order is ever load-bearing.

Record 9 is **per world, never compressed** (the adjudicated dependency
of record 5's multiset form — matched-set membership is what lets a
checker reconstruct which worlds are in the multiset). Per matched
decision it emits, per final-implicant cell, a `'0'/'1'` truth vector
over the fiber in canonical world order (with its satisfied count), and
the `matched (all cells)` vector — the conjunction — whose popcount
equals the matched-world count. **Check** (exp3a-covered cells):
enumerate the fiber, sort canonically, re-evaluate each cell per world,
compare vectors byte-for-byte; AND the vectors and compare with the
matched vector.

## 6. Selectors and values (records 4, 5)

**Selectors** (record 4) name actions kernel-generically: `decisive`
(the kernel's derived decisive tile, when legal), `max-count` /
`min-count` (legal action maximal/minimal by (count points, pip sum,
tile index)), `tile(x-y)` (concrete, when legal). Record 4 lists each
matched decision's resolved tiles plus the decisive tile used.

**Record 5 — the terminal feature law in its scalar specialization.**
The valuation is `q_points`: each trick worth `1 + count points of its
four tiles`, focal team minus opponents, future-increment mode (§8.5:
completed tricks are the caller's action-independent bank).

- **(C) content** (label `(C, minimax-omniscient)`): per matched
  decision, the exact distribution of the value pair the verdict reads,
  as `(a,b)xN` entries sorted ascending: `(v_better, v_worse)` per
  matched world for refutations, `(v_action, v_optimum)` for wins.
  `v_x` = the world's perfect-information minimax continuation value of
  the action (an integer at this valuation). The per-world pairing is
  inside each element, so the multiset determines the worldwise
  verdict: refutation holds iff every entry has `a >= b`; win holds iff
  every entry has `a = b`. **Comparison protocol (declared verbatim in
  the record)**: checker aggregates its own per-world rows to a
  multiset; multiset equality; asserts no pair has `v_better < v_worse`
  (win form: asserts every pair has `v_action = v_optimum`). The
  coverage annotation states that **world-alignment is unchecked**: a
  bug assigning pairs to wrong worlds while agreeing on the aggregate
  passes multiset comparison — it can mask implementation disagreement,
  never certify a false claim. Matched-set membership (which worlds are
  in the multiset) comes from record 9's per-world truth vectors.
  **Check**: re-enumerate the fiber, restrict to matched worlds (record
  9), solve each world with the preserved exp5 scalar PI solver,
  rebuild the distribution, compare exactly.
- **(H) rows** (label `(H, fixed-uniform-legal)`): per fiber-valid
  applied decision, `Q^H` per legal action as exact rationals — the
  pooled-information viewer maximization against the fixed
  uniform-over-legal field, root weighting uniform over the fiber. The
  measurability envelope in record 3 (budget value, budget-semantics
  version, cache configuration) is PART OF THE CLAIM. Rows may read
  `H-CAPPED` (budget exhausted: unmeasured, never zero) or
  `NOT-FIBER-VALID` (the per-decision H quantifier does not extend
  there). No independent H checker exists today: these rows are
  UNCHECKED-EXTERNALLY until one registers.
- **Checker lessons** have no scalar value rows; the verdict reads the
  §12.6 carrier, reproducible from record 1's kernels and the named
  descriptor family.

## 7. Verdicts and quantifiers (record 8)

Record 8 restates the verdict with its quantifier shape, verbatim from
the type: refutation and win hold per matching (decision, world) at
(C); at H the shape changes by necessity to one statement per matching
decision with atom cells read fiber-valid. A win is per-world
sufficiency ONLY — never a seat-facing guarantee (§7.6 strategy-fusion
gap); the checker verdict holds per matching decision (ply 0, horizon
<= 2).

## 8. The widening trace (record 10)

The full generalizer trace, one line per step:

- `drop <cell> -> dropped` — the widening re-verified.
- `drop <cell> -> SURVIVES; witness ...` — refuted; the cell stands.
- `relax <cell> -> BOUND HELD at <held>; witness ...` — a bound's
  relaxation ladder stopped; the held bound is named with its value.
- `introduce <cell> (cut refinement, distinct from drops) excluding
  witness ...` — a world-selecting cell introduced on a failed widening;
  the witness it excludes is named.

Every witness is complete reconstruction data: the four-seat deal and
the full (C) action-value row at the witnessing decision (checkable with
the exp5 solver), or a lumpability witness (nodes -> classes) for the
checker species. The `intro budget: k/4 spent` line closes the record.

## 9. Records 6, 7, 11 for scalar lessons

`rational-affine-segments`, `breakpoint-continuity-witnesses`, and
`information-price-segments` describe λ-parametric envelope claims
(affine segments, breakpoint ordering, price nonnegativity). A scalar
lesson evaluates one fixed integer valuation: these records are emitted
present-and-empty, `NOT-APPLICABLE` with the reason. A future
parametric lesson kind must fill them and bump the schema version.

## 10. Rent rows, clearances, and deletion records (the ledger side)

Rent rows live in the economy ledger (`results/economy_*.txt`), not in
certificates, but the independent checker's obligations extend to them:

- **Checker obligation for any cited rent row**: applied-decision-SET
  EQUALITY over the row's declared domain — recompute which domain
  decisions the lesson applies to and compare sets (the empty set for
  zero-applied rows; no special non-application verifier exists), then
  re-derive the row's H figures over that set.
- **Coverage stamps and per-row clearance**: every H rent row carries an
  at-collection coverage stamp (`SINGLE-IMPLEMENTATION` today) — an
  append-only historical record, never rewritten. When an independent
  checker re-derives a cited row's figures, a CLEARANCE RECORD is
  appended beside the row (epoch, content-key hash, checker name).
  Lesson-level coverage is display only.
- **Deletion records** cite the evidence pattern verbatim, gaps
  included: `measured-zero e1, e3; e2 capped at <budget, semantics>`.
  The trigger property is measured-consecutive — zero rent in the last
  N MEASURED epochs, no intervening measured nonzero; capped epochs are
  cited as gaps, never as evidence. A deletion executes only when a
  checker is registered (token) AND every cited measured-zero row was
  collected under cleared coverage or carries an appended clearance —
  no deletion decision may cite a rent figure that exists in a single
  implementation.

## 11. What a checker must do (summary)

1. Parse the header; refuse unknown schema versions. Recompute the
   FNV-1a-64 of the canonical block; compare to the stated hash.
2. Reconstruct every kernel (record 1) by replay; recount every fiber
   (record 2); compare.
3. Re-evaluate implicant cells over each matched decision's fiber in
   canonical world order (record 9, exp3a-covered cells); compare the
   per-world truth vectors byte-for-byte; AND them and compare with the
   emitted matched vector.
4. Re-solve matched worlds with the preserved exp5 scalar PI solver;
   rebuild record 5's (C) distributions; compare exactly; confirm the
   verdict's inequality/equality over every distribution element.
5. Re-check record 10's witnesses: each witness world is in its
   decision's fiber, its value row re-solves to the stated integers,
   and each introduced cell excludes its witness.
6. Report PASS/FAIL per record. A PASS is evidence at the walt tier,
   never a status promotion (TRUST-01).

H rows (and walt-native beater cells) are outside today's checkable
subset; a future registered H checker extends step 4 to them and
re-annotates coverage.
