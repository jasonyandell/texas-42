# GPU-native trick-1 — adjudicated implementation contract v0.3

**Status:** binding design and maintained implementation contract for the first
build; exploratory as mathematics; not a Metal result.  The received v0.2 guide
remains preserved verbatim at
`math/gpu_native_trick1_implementers_guide_v0.2.md`.  This file records the
adjudication and is authoritative wherever it narrows, repairs, or rejects a
v0.2 statement.  Portable M0/M1 is implemented for the bounded slice below;
whole-gate closure is conditional on the final source-bound reproducibility
artifacts and integrated gate required by GT1-A9/freeze 55.

**Purpose:** build an exact imperfect-information opening player, not a
perfect-information oracle and not a heuristic imitation of one.  The first
production boundary is the exact opening-response projector.  The later
boundary is a stopped lower/upper search whose focal decisions are joined by a
mandatory perfect-recall information net.

**Standing disciplines:** `CENSUS-RULINGS.md` governs.  In particular:
adjudicate before build; exact arithmetic only; NO-RESCUE; one focal action per
lawful information state; complete action faces; no grade-4 quantity is
evidence about trick 1; and a failed gate is a result, never permission to
change the measured object silently.

---

## 1. Executive verdict

The v0.2 architecture survives, but only after four hard repairs:

1. The first API is a **narrow opening root**, not a generic public state.  Its
   prior, evidence policy, field policy, utility, horizon, and scale are named
   in the type and digest.
2. The mid-trick decidedness upper bound includes points already lying in the
   unresolved current trick.  The received formula can otherwise declare a
   contract safe when it is set.
3. Exact masses are role- and scale-typed.  A likelihood coefficient, support
   count, cell mass, conditional value, and full-horizon contribution are not
   interchangeable integers.
4. The information-net key is extensionally equal to the canonical
   perfect-recall observation key.  Merely including that observation among
   other fields is insufficient: an extra descriptor can split one true
   information state and reintroduce strategy fusion.

Portable M0/M1 Rust and the foundational Lean obligations are **GO**.  Their
bounded implementation now exists; it is reported **IMPLEMENTED, GATE PENDING**
until the checked source manifest, committed canonical envelope and grade-5 stop,
fresh byte comparison, Rust gate and Lean target pass together.  Metal is
**NO-GO at Gate 0** on the current machine until a compatible full Xcode is
installed and selected; Command Line Tools alone do not supply `metal`,
`metallib`, `metal-ar`, GPU capture, or Instruments.  A portable green run is
never reported as a Metal green run.

---

## 2. Exact first-slice model

### 2.1 `OpeningRootV1`

The first solver accepts exactly the following model-relative coordinate:

```text
OpeningRootV1
  declaration               one walt-core declaration
  focal/bidder/leader/actor  one seat; all four roles equal
  focal_hand                 exactly seven distinct dominoes
  contract_normal_form       PointBid(30..41) or Mark, with the bidder
  loss_budget                derived view only: 42-bid or 0, asserted <= 12
  evidence_profile           IgnoreAuctionEvidenceV1
  prior_profile              UniformCompatibleOpeningDealsV1
  field_profile              UniformRandomLegalV1
  utility_profile            DeclaringTeamMakesV1
  horizon_profile            OpeningStraightHand21FieldActionsV1
```

Preconditions, checked before any projection:

- the public play record and current trick are empty;
- the focal hand is legal and has seven tiles;
- the hidden support is the complete ordered `7/7/7` allocation fiber of the
  other 21 tiles;
- the root constructor validates the closed contract normal form, derives
  `loss_budget`, and asserts any packed copy equal to that derived view;
- the bidder's partnership is the declaring team;
- legal root leads, selected led contexts, trick resolution, and count values
  come from `walt-core` only.

This model intentionally ignores auction evidence.  That is a declared model
choice, not a claim that auction evidence is irrelevant.  A later conditioned
prior is a new profile and invalidates every uniform-cell shortcut until it is
separately adjudicated.

`OpeningRootV1` takes the declared contract as the starting public fact; it
does not claim to validate that a complete auction history reached it.  An
auction-aware constructor must replay and validate that history and is a later
profile.  The caller cannot supply `loss_budget` independently of the normal
form.

This distinction is already observable in Walt: the standing 90-world witness
has two legal histories with identical hidden support and opposite optimal
leads.  Support is not belief, even when its set of worlds is unchanged.

The first build does not invent a generic `PublicState`, `Contract`, or
arbitrary `FieldProfile` adapter.  Walt and Rob currently have separate type
universes; joining them is a later, separately checked interface.

### 2.2 Semantic digest

Every persisted value, receipt, and reusable buffer is keyed by a digest over:

```text
complete freeze-set digest
rules/table version and canonical table bytes
declaration
root profile and evidence profile
prior profile
field profile
utility and loss budget
information-key version
arithmetic ABI and scale contract
program-registry version, when programs enter the build
kernel/build identity
```

A digest mismatch means **corrupt for this run**, not stale-but-reusable.  The
reserved circuit-order and reachable-belief freezes 39 and 40 remain reserved;
this build neither fills nor silently depends on them.

---

## 3. Count objective and the decidedness repair

For the declaring team, a validated loss budget `R` is in `0..12`.  At a
completed trick won by the defenders, subtract `1 + count_in_trick`; at a trick
won by the declaring team, leave the budget unchanged.  A negative remainder
is exact failure.

At an arbitrary public state define

```text
P_unbanked = 42 - declaring_banked - defending_banked.
```

Equivalently, it is the base point for every unresolved or future trick plus
the count value of every tile whose trick has not been banked, including count
tiles already played into the current unresolved trick.  The only lawful free
tests are:

```text
defending_banked > R                  => exact failure
defending_banked + P_unbanked <= R   => exact success
otherwise                            => live
```

The v0.2 wording “remaining trick points plus count on unplayed count tiles” is
false mid-trick.  One fully legal counterexample is no-trump, P37 by S0:

```text
S0: 00 11 22 33 44 66 54
S1: 55 60 61 62 63 64 65
S2: 10 21 32 43 50 51 53
S3: 20 31 42 30 41 40 52
```

After tricks

```text
00 60 10 20
11 61 21 31
22 62 32 42
33 63 43 30
44 64 50 41
66 65 51 40
```

S0 has won six tricks and banked 31; defenders have banked zero.  At the final
prefix `54 55`, the only unplayed tiles are non-count `53 52`.  The received
formula returns `P_live = 1 <= R = 5`, yet S1's `55` wins an 11-point trick and
sets the contract.  This fixture is permanent and blocking for any generic
mid-trick projector.  Opening-root projection begins at a trick boundary, but
the engine may not carry the broken formula forward.

---

## 4. Exact arithmetic contract

### 4.1 Frozen scale

Under `UniformRandomLegalV1`, every field legal-set size is in `1..7` and
divides

```text
L = lcm(1,2,3,4,5,6,7) = 420.
```

One field action with `k` legal moves contributes the integer factor `420/k`.
After `e` field actions the common field denominator is `420^e`.

For a fixed seven-tile focal opening hand,

```text
N0 = 21! / (7!)^3 = 399,072,960.
D  = N0 * 420^21.
```

`D` has bit length 212.  `42D` has **217 magnitude bits**; a two's-complement
signed carrier would therefore need 218 total bits.  Eight 32-bit limbs are
ample for both.  The first binary make/set engine is nonnegative and uses an
unsigned carrier.

### 4.2 ABI integer

The portable and Metal representations are fieldwise identical:

```text
U256Mass = eight little-endian u32 limbs
```

Canonical bytes are the 32 little-endian bytes obtained by serializing limb 0
through limb 7.  The shader ABI uses fixed-width scalar fields only: no
`usize`, language enum layout, `bool`, implicit padding bytes, or vector-layout
assumption enters a persisted or hashed record.

Required operations for M0 are zero, comparison, checked addition, checked
subtraction where the caller proves order, and checked multiplication by a
small `u32`.  Scaling to the horizon is a checked fixed-count sequence of
`mul_small(420)`, not an unnecessary general wide-by-wide multiplication.

A legal-set size of zero is an error.  The denominator table has valid entries
only for `1..7`; index zero cannot return a zero multiplier that silently
annihilates mass.

No wide atomic updates are used.  Exact values are combined by deterministic
segmented reductions.

### 4.3 Role and frame typing

The host distinguishes at least:

```text
LikelihoodCoeff       per-world field likelihood numerator
SupportCount          number of physical worlds in one support
CellMass              SupportCount * LikelihoodCoeff
ConditionalValue      normalized value within a named component
WeightedContribution  already weighted contribution at a named horizon
```

An additive exact value carries or is statically bound to a frame containing:

```text
prior profile/digest
field profile
utility profile
measure role
elapsed field exponent
full-horizon field exponent
task/root identity
root-action identity where required
```

`SupportCount` is a checked role-specific newtype, not a bare integer accepted
wherever a mass is expected.  Profile IDs and frames are constructed through
closed v1 constructors or checked decoding; an unknown raw ID cannot launder
one measure into another.

Host addition, subtraction, and comparison reject unequal frames.  Frames have
canonical bytes.  A horizon-lift operation both multiplies by the required
power of 420 **and advances the elapsed exponent**; a numerator-changing method
that leaves frame metadata stale is forbidden.  GPU stages may omit
repeated metadata only behind a stage-specific buffer descriptor that fixes it
for the whole batch.  A value cannot leave that stage without the frame being
reattached and checked.

For example, raw numerators `1` at exponents 2 and 3 do not add to `2`; at the
common exponent 3 they add to `421`.  Likewise a per-world coefficient `3` on
a two-world support is not a total mass of `3`.

### 4.4 Residue backend

The first residue backend, if built, is restricted to **nonnegative sum-only
masses**.  Reconstruction precedes every comparison.  The product of moduli
must exceed the proved nonnegative upper bound.  If signed values are later
admitted, unique reconstruction on `[-B,B]` requires modulus product greater
than `2B`, not merely greater than `B`.  Moduli, channel order, reconstruction,
and magnitude proof are frozen and receipted.

No floating-point type or operation appears in Rust, MSL, generated tables, or
proof-path interchange.  A learned or heuristic ordering source, if later
used, receives a distinct integer/fixed-point type and cannot convert into an
exact value.

---

## 5. Exact opening-response projector

Fix one legal root lead `a` and let `q` be the selected effective led context.
Let `U` be the 21 hidden tiles, `M = U intersect context(q)`, and `m = |M|`.
Because the led tile belongs to its context, `m <= 6`.

For an ordered distinct response triple `x = (x1,x2,x3)`:

- `F` is the set of seats whose response lies in `M`;
- `Z` is the set of void seats;
- for `s in F`, `e_s` is the number of additional matching tiles left in that
  seat after its response;
- `sum e_s = m - |F|`; a void seat has no matching tile.

After removing the three responses, every hidden seat has capacity six.  With
`M'` and `N'` the remaining matching and nonmatching pools, the support count is

```text
A(e,x) = |M'|! / product(e_s!)
         * |N'|! / (product((6-e_s)!) * (6!)^|Z|).
```

It is implemented as checked products of binomial coefficients.  The
per-world coefficient at scale `420^3` is

```text
C(e,x) = product_{s in F}(420/(e_s+1)) * 60^|Z|,
```

and `W(e,x) = A(e,x) * C(e,x)`.

The mandatory conservation identity is

```text
sum_{x,e} W(e,x)
  = N0 * 420^3
  = 29,566,517,460,480,000.
```

Its conceptual proof double-counts `(deal,response)` pairs.  For every fixed
deal, the three scaled uniform response laws sum to `420^3`; `(x,e)` partitions
those pairs, and the closed formula gives each block's cardinality.

### 5.1 Exact output bound and order

The received rectangular bound `7,980 * 10 = 79,800` is safe but loose.  The
exact nonempty raw-cell counts by `m` are:

| `m` | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
|---:|---:|---:|---:|---:|---:|---:|---:|
| cells | 7,980 | 1,140 | 2,166 | 3,408 | 5,172 | 7,800 | 11,730 |

For `f = |F| >= 1`, the count is generated by

```text
C(3,f) * falling(m,f) * falling(21-m,3-f) * C(m-1,f-1),
```

summed over feasible `f`; `(m,f)=(0,0)` contributes `falling(21,3)`.

Canonical emission order is a generating rule, not a redundant list:

1. response tile indices lexicographically by seat 1, then 2, then 3;
2. for that response, matching-count vectors lexicographically by seat 1,
   then 2, then 3;
3. impossible vectors are not emitted.

### 5.2 Context reuse and merge boundary

Under the frozen field, the response law may be reused by root leads with the
same selected led context.  The actual lead tile still remains in the semantic
successor: it determines winner, its count contribution, the next leader, the
remaining focal hand, budget change, and the public record.

Projector computations may be deduplicated globally by identical shape/query
bytes.  **Semantic masses may merge only inside one identical full public
information key and one identical scale frame.**  Equal physical allocation
shapes reached through different actor-attributed histories are not one state.

### 5.3 Uniformity scope

Every `(x,e)` support is uniformly weighted only under
`UniformCompatibleOpeningDealsV1`.  In general the posterior is

```text
beta0(H1,H2,H3) * allocation_indicator * product_s L_s(H_s).
```

The prior factor disappears only because the frozen opening prior is constant.
Repeated legal-count stratification remains exact only when cells have positive
additive-mixture semantics, every overlapping persistent factor is updated,
and canonicalization preserves exact support and coefficient.  These are Lean
and differential-test obligations, not comments.

A one-mask histogram answers one marginal query.  It does not supply joint
events for arbitrary compound program predicates.  Compound predicates must be
compiled into an explicit sequential-conditioning/query DAG whose every edge
updates the complete overlapping factor set, or be handed to an exact global
projector.  Separate marginal histograms are never multiplied into a joint law.

---

## 6. Perfect-recall information net

At every focal decision the canonical net key satisfies the biconditional

```text
key(x) == key(y)  iff  x and y are the same lawful perfect-recall information state.
```

The first authority is Walt's canonical focal observation: focal seat/current
hand plus the complete actor-attributed public play record and the public
contract/declaration state needed by the objective.  Hidden world IDs, cell
IDs, hidden holders, relaxed values, internal proof data, and tie-broken
clairvoyant actions are excluded.

A derived policy descriptor may enter the key only after a proof or exhaustive
finite equivalence check establishes that it is a deterministic function of
the canonical observation key and cannot refine the partition.  A candidate
policy/program ID is an evaluation dimension outside the key.

Why inclusion alone is insufficient: take two hidden components in the same
true information state, with payoff vectors `(1,0)` and `(0,1)`.  If an
internal descriptor splits them, the two segments can choose different actions
and report value 1; either lawful common action has value 1/2.  That is strategy
fusion with the correct hand and history still present in both keys.

A hash is only a prefilter.  Equality is discharged by full canonical-key
comparison.  Every sort/scatter/reduction has a frozen total secondary order
over `(task, root action, full key bytes, source ordinal)`.  Atomic slot
reservation may not choose receipt order, reduction order, or which work
survives a cap.

---

## 7. Interval algebra and root verdicts

Let posterior components form a positive additive decomposition of one named
measure.  For one common action `a`, compatible scaled action upper
contributions sum componentwise before any focal maximum.  Lower contributions
sum only when one jointly lawful global continuation realizes all of them; a
complete `ProgramInstanceId` (program plus every initial descriptor and
parameter) is sufficient but not necessary.  A bare grammar/program ID is not.
Independent per-component policy maxima are forbidden.

For exact action values `Q`, bounds obey `L(a) <= Q(a) <= U(a)` in one common
frame.  The root distinguishes five outcomes:

```text
OptimalMember(a)       L(a) >= U(b) for every competitor b
UniqueOptimal(a)       L(a) >  U(b) for every competitor b
CanonicalOptimal(a)    member, and every lower-index competitor is strictly excluded
ExactOptimalSet(S)     relevant intervals collapse and establish exactly S
Unresolved             none of the above
```

`U(a) < max_b L(b)` safely removes `a` from the optimal set.  Touching or
overlapping intervals never manufacture a tie.  The received strict condition
is retained as the uniqueness test, not as the only useful stopping rule.

`OptimalMember` is an internal mathematical result and a research receipt; it
does not by itself authorize a player action.  Freeze 26 chooses the
least-domino-index member of the exact argmax.  A playable `CanonicalOptimal(a)`
therefore needs non-strict separation from every higher-index competitor and
strict separation from every lower-index competitor, unless an exact optimal
set is already known.  Reporting precedence is deterministic:
`ExactOptimalSet`, then `UniqueOptimal`, then `CanonicalOptimal`, then
`OptimalMember`, then `Unresolved`.  The selected play from an exact set is its
least domino index.

The first-rung tax and every later nonanticipativity reduction multiply by the
carried arrival mass.  At a frontier the posterior is generally legal-set
weighted and nonuniform; summing unweighted world maxima or replacing it with a
fresh uniform residual prices another measure and the result is void.

---

## 8. Claim adjudication

The v0.2 numbered claims are classified before implementation:

| Claim | Verdict | Binding reading |
|---:|---|---|
| 1 | conditional derivation | `R <= 12` follows for a semantically validated legal contract; a raw integer is not the premise. |
| 2 | derived | Legal-set sizes `1..7` divide 420. |
| 3 | repaired | The 212/217 magnitude bounds hold under the frozen opening prior/horizon; signed storage needs one sign bit. |
| 4 | repaired | Common numerators require identical prior, field, utility, horizon, and frame. |
| 5 | derived under profile | Same-context response reuse holds only for `UniformRandomLegalV1`; finite parity is blocking. |
| 6 | derived and finitely checked | 79,800 is safe; 11,730 is the sharp nonempty-cell maximum. |
| 7 | repaired | Cell uniformity also requires the frozen constant opening prior. |
| 8 | proof debt | Repeated stratification needs a positive-measure/support-preservation invariant. |
| 9 | repaired | General factorization includes the prior; pure seat potentials require the frozen prior and seat-local field. |
| 10 | proof and parity debt | Coefficient extraction is exact; sparse-DP and join invariants remain to prove and cross-check. |
| 11 | derived | A 11/10 split gives at most `3^11 = 177,147` assignments in one half; this is not a speed claim. |
| 12 | derived with typing | Positive additive components may sum action upper contributions in one frame. |
| 13 | refined proof debt | Lower sums require one jointly lawful global continuation, not merely coincident local maxima; a complete program instance is sufficient. |
| 14 | derived | Strict upper-vs-lower dominance removes an action. |
| 15 | rejected as sufficient | The full key must equal, not merely contain, the perfect-recall information partition. |

The performance proposals remain hypotheses.  No compression ratio, GPU
speedup, basis-rank behavior, or root closure is promoted by this table.

---

## 9. Build ladder and blocking gates

### M0 — portable arithmetic and semantics

Deliver:

- `walt-gpu-spec`: `U256Mass`, role/frame types, canonical ABI bytes, and
  semantic tables generated from `walt-core`;
- checked arithmetic against an independent big-integer test oracle;
- byte-stable table/digest receipts across fresh builds;
- exhaustive semantic parity, including an independent prose-rules bridge to
  discharge T1-A12 before this implementation is cited as rules evidence;
- CI rejection of float types in Rust, manifests, and MSL.

M0 failure stops M1.  No table is repaired shader-side.

**Implemented boundary (2026-08-16).** `walt-gpu-spec` supplies the fixed-width
integer, closed role/frame constructors, canonical v2 tables generated from
`walt-core`, canonical hashing, FIPS SHA-256 anchors and independent big-integer
oracle tests.  The independent prose-rules bridge covers led context,
compelled-follow legality, winner and points over its complete declared finite
domains.  This is portable executable evidence only, under GT1-A3/GT1-A6.

### M1 — scalar opening projector

Deliver:

- an independently written closed-form scalar projector;
- direct physical-world parity at reduced grades 2, 3, and 4; grade 5 is an
  attempted rung with the declared preflight stop below;
- full opening arithmetic checks for every reachable `m=0..6`;
- exact support, coefficient, cell-mass, response-mass, and global-mass
  conservation;
- same-context lead reuse parity for every declaration and legal lead pair;
- canonical receipt generated twice from fresh state and compared bytewise.

M1 is a correctness instrument, not evidence that grade 4 predicts opening
behavior.

The direct-enumeration work unit is one complete physical world emitted in
freeze-7/23 kernel order.  `M1_DIRECT_WORLD_CAP_V1 = 100,000`; the exact kernel
count is checked before iteration.  Grades 2, 3, and 4 contain 90, 1,680, and
34,650 worlds and are mandatory.  Grade 5 contains 756,756 worlds, so v1 files
`DECLARED STOP` before enumeration and retains no partial comparison.  This cap
governs only the independent direct-parity arm, never the closed-form projector.
The projector has the derived hard cap of 11,730 nonempty cells; exceeding it
is a receipt failure, not truncation.

**Implemented boundary (2026-08-16).** `walt-gpu-ref` supplies the independent
closed-form and direct projectors, full-opening `m=0..6` count/mass checks,
reduced-grade parity at every feasible declared two-root carrier coordinate,
the grade-5 zero-output stop, exhaustive same-context response reuse with
distinct physical-action envelope identities, and the canonical persisted
run-envelope/stop validators.  The raw projector payload is explicitly
non-persistable.  Freeze 55 binds these encodings and their build identity.  M1
is complete only if the GT1-A9 integrated reproducibility gate is green.

### M2 — Metal parity

This gate does not open until full Xcode is installed and selected.  Then:

1. record `xcode-select`, `metal`, `metallib`, `metal-ar`, capture, and profiler
   availability;
2. compile and run one integer-only diagnostic kernel;
3. record actual device and pipeline limits;
4. review and pin the minimal Rust Metal bindings;
5. compile checked-in MSL with fixed arguments;
6. require cell-for-cell and byte-for-byte CPU/GPU parity, including overflow,
   arena high-water, and deterministic-order receipts.

No approximate tolerance exists.  Missing toolchain/device is a named NO-GO,
not a skipped green test.

GPU work units, scheduler order, arena caps, and no-partial-result semantics
must receive a later explicit freeze before any M2+ persisted result.  Freeze
44's CPU walk-step budget does not type this work and is not inherited.

### M3 — explicit grade-4 perfect-recall net

Build the full-key net before any larger symbolic controller.  Compare its
lawful value with the existing scalar hidden-information authority on the same
coordinates.  Also run a deliberately world-split negative control; it must
produce the relaxed value and must not be accepted as treatment H.

### M4 — representation growth gate

Only after M0–M3 pass, measure persistent basis rank, cell count, query
deduplication, materialization rate, arena high-water, sort volume, and CPU/GPU
crossover on declared carriers.  Full 723-million-arrival wavefronts are not
materialized: illustrative 44-byte particles already consume about 29.6 GiB,
and wider bound-bearing particles exceed the 48 GB machine before sort and
ping-pong buffers.  Use slabs, early exact reduction, immutable-slice sharing,
and fail-closed caps.

The symbolic representation survives only if the declared growth gate does.
No low-grade result rescues it and no low-grade result kills the opening target
without the gate's stated transport argument.

### M5+ — stopped controller, gluing, and opening attempt

Join the count-aware projector, exact plan lower witnesses, action-conditioned
upper witnesses, perfect-recall net, and only then the first and second gluing
rungs.  Work is prioritized by possible root-verdict effect.  Every first
action is covered; gluing only the currently best face is unsafe.

Before M5, the implementation imports explicitly—not by shorthand—the
weighted-arrival law, first-frontier free-product action space, unique parent at
the second frontier, counted forced-frontier convention, early-terminal mass,
and complete-action coverage contracts from the FT/SR rulings.

A plan lower witness alone does not remove a leaf from upper-bound tightening.
A leaf is absent from gluing only when its relevant upper side is already exact
or otherwise settled strongly enough that further tightening cannot affect the
root verdict.

The proof-controller outcomes include `OptimalMember`, `UniqueOptimal`,
`CanonicalOptimal`, `ExactOptimalSet`, `UnresolvedAtDeclaredCap`, or a named
receipt failure.  Only the three outcomes that determine the freeze-26
least-index action are playable.  A root that does not close is a measured
result.

---

## 10. Lean ledger

The proof work is split so executable parity is never mistaken for a theorem:

1. **Foundation:** loss-budget bound, divisibility by 420, numeric width,
   unbanked-point invariant, interval dominance, member versus uniqueness.
2. **Opening partition:** hidden-context bound, unique `(x,e)` partition,
   `A(e,x)`, `C(e,x)`, conservation, and exact cell counts.
3. **Weighted posterior:** positive additive mixtures, repeated
   stratification, the uniform-prior specialization, and prior-aware
   seat-potential factorization.
4. **Information net:** component upper sums, jointly lawful lower sums, key
   equivalence, and the deterministic-derived-descriptor corollary.
5. **Projector portfolio:** sparse-DP fold and meet-in-the-middle complement
   join before either becomes the sole proof path.

No `sorry`, new axiom, or native-code decision shortcut is accepted.  Every
exported theorem receives an axiom audit.  A finite executable check remains a
receipt even when it covers the entire current carrier; it is not silently
renamed a Lean proof.

**Foundation status (2026-08-16).** `lean/Texas42/Trick1Foundation.lean`
kernel-proves the legal nonpass loss-budget bound, the initialized and
transition-preserved seven-tile hand cap, actual live legal-set nonemptiness and
`1..7` divisibility into 420, the 212/217 numeric windows, the state-tied
current-trick-aware unbanked-point invariant and its legal-step preservation,
all seven opening-cell counts through the sharp 11,730 maximum, positive
component upper summation, one-shared-policy lower summation, dominance, and
member versus uniqueness.  Still open are the semantic `(response,e)` partition,
the `A/C/W` formulas and global conservation, posterior
stratification/factorization, information-key equivalence and deterministic
descriptor corollary, canonical least-index verdict, sparse-DP/MITM refinement,
and Rust/Lean plus Metal/Rust correspondence.  Executable parity does not
discharge those obligations.

---

## 11. First implementation layout

```text
walt-gpu-spec   portable tables, ABI, exact arithmetic, frames, digests
walt-gpu-ref    scalar opening projector, direct enumerator, receipts
walt-metal      feature-gated Metal runtime after Gate 0 opens
```

No full controller crate is created until the portable M0 and M1 gate passes.
No `walt-metal` crate exists while Gate 0 is closed.  The initial Metal
runtime is direct and narrow; a portability layer is not introduced before the
exact workload and binding surface are measured.

This order is deliberately asymmetric: the mathematics fixes what a correct
value is, the portable implementation makes that value independently
checkable, and Metal earns authority only by reproducing the same canonical
objects.
