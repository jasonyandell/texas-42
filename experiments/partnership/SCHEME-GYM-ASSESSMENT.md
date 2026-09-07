# Scheme/Fix and a partnership gym

Research assessment, 2026-09-06. Exploratory proposal; source inspection, not a
new implementation or a measured strength result. The objective remains lawful
partnership play with bid 30. Count capture is a mechanism to investigate;
make/set is the primary outcome. This assessment follows the discussion of
[conditional strength](campaigns/default-partner-battery/STRENGTH-ASSESSMENT.md).

## What Scheme contributes

Scheme/Fix is a typed relational query language over the worlds of an exact
decision state. It describes contexts, seats, and dominoes by their roles and
relations. It does not replace the state or supply a belief distribution.
This is a good fit for describing *families* of partnership opportunities,
instead of collecting only particular tile-number puzzles.

The mathematical source is [unified information geometry v0.4](../../walt/math/unified_information_geometry_v0.4.md),
especially sections 3–5 and 12.7–12.9; the maintained
[Scheme/Fix guide](../../wiki/walt-scheme-fix.md) separates semantics from built
machinery. A Scheme case is an equality pattern plus a conjunction of registered
relations. A Fix is a disjunction of such cases. Returned roles are explicit;
internal witnesses do not create extra answers or probability mass.

An illustrative gym query could bind a focal seat, its partner, a candidate
play, a partner entry, and a count tile, with a declared two-trick horizon.
Those names describe what to search for. Whether the candidate improves make
probability is a separate evaluator result, never an atom defined by asking
the target solver for the desired answer. Continuation relations such as
"enables the entry" require definitions and verification; they are not supplied
by merely naming the roles.

## What is available

| Component | Verified source status | Gym use and boundary |
| --- | --- | --- |
| Full Scheme/Fix language | Specified; no parser, role-schema query runtime, or step compiler in the current crate. The guide explicitly records these gaps. | A small executable relational fragment would be new work. The full dynamic language is not a prerequisite for an offline gym. |
| Exact support, legal worlds, replay | Current `walt::kernel` and `walt::rules`; arbitrary transcript decisions include partial tricks. | Reconstruct coordinates and legal hidden completions; retain the declared belief separately from support. |
| Exact policy evaluation and extraction | Current `solver/factor_belief.rs` and `solver/extraction.rs`. | Extract an executable focal continuation and re-price it through a separate fixed-policy recursion. Values are relative to the declared seat-local deterministic field. This is not a joint-team oracle or an already wired adapter for every arena configuration. |
| Paired outcome and split records | Current `solver/field_swap.rs` and `solver/motif.rs`. | Enumerate policy benefit/harm/tie worlds and preserve differing suffixes. Existing motif labels describe the first split; they do not establish what caused a partnership benefit. |
| Conflict-to-lesson generalization | Archived factory `generalize.rs`, `lesson.rs`, `basin.rs`, `certificate.rs`, available at producer commit `648f93a`. | Widen descriptions, re-verify over a finite domain, retain counterexamples, and report the matching set. Old pair verdicts use per-world perfect-information values and need replacement for this gym. |
| Descriptor soundness and synthesis | Archived skeleton `soundness.rs`, `synth.rs`, `lumpability.rs` at the same commit. | Test whether a proposed description preserves a declared response; return separating examples. These are finite-domain instruments, not a generalization proof. |
| Current hand-class refinement | `solver/factor_belief.rs::refine_to_action_exact`. | A useful existing example of counterexample records and exact residual mass. It classifies a specified field's actions, not optimal partnership decisions, and currently pays the full classification bill first. |

The archived files were inspected directly with Git, without changing the
checkout. [ARCHIVE.md](../../walt/ARCHIVE.md) records their provenance and
regeneration procedure. The [instrument inventory](../../wiki/walt-instruments.md)
distinguishes current modules from archived producers. No old producer was
restored or run during this investigation.

## Proposed construction

1. **Declare the exam.** Store the focal information state, relevant public
   history, declaration, score, bid 30, belief, teammate/opponent policies,
   continuation policy class, and identity/version conventions. A seed is
   provenance; hidden-world assignments are specimens within the coordinate.
2. **Find candidate opportunities.** Use Gran and divergence records, then
   relational queries for count donation, lead transfer, and promotion of a
   partner entry. Begin with short remaining hands where exhaustive grading is
   affordable; extend toward earlier decisions after the instrument works.
3. **Compute the answer independently of the description.** Compare lawful
   candidate continuations under the same belief and declared field. Retain
   exact make probabilities or sound bounds, executable policies, and replay
   witnesses. PI values may screen candidates or supply justified upper bounds;
   they do not label the lawful best move.
4. **Generalize and try to break the claim.** Adapt the factory's widening loop
   to the gym target. Preserve cases where the same apparent partnership pattern
   recommends a different action. Those counterexamples are useful exercises,
   not records to discard. Learned descriptions may remain narrow.
5. **Evaluate players on held-out coordinates.** Score lost make probability
   when the reference optimum is established, or report the available bounds.
   Credit the full optimal-action set. Keep count-only improvements separate.
   Hold out whole coordinates and related families, not individual hidden deals
   from a coordinate used to design the description.

For a claim that the benefit specifically involves partner, retain the relevant
count-capture continuations and an explicit comparison, such as a root-action
change under a fixed teammate or a teammate-policy change with other policies
fixed. A first-split label alone is insufficient causal evidence. Joint plan
comparisons must name both local policies; independently choosing each seat's
best action in each hidden world is not a lawful joint plan.

## Information and generalization boundaries

Scheme can mention `Holds(partner, entry)` in the offline examiner. The player
must not receive that hidden truth, an answer label, or a private plan identifier.
An eventual in-player use must integrate the relation under belief or establish
that the guard is available from that seat's information. Even a descriptor with
public-only updates can retain a hidden fact supplied at initialization; closed
updates alone do not make its value known to the seat.
Mining by a hidden pattern must not silently condition the player's belief on
that pattern. Any selection information disclosed to the player belongs in the
declared information state and the reference evaluation too.

The old factory's concrete-tile vocabulary limits transfer. Role binding is a
useful extension, but changing pips is not automatically a valid transformation:
count values, declaration, seat order, legal observations, and the make-30 target
must still be checked. Also, existential query matches and overlapping Fix
branches must not multiply a world's probability.

The [decision-sparse results](../../wiki/walt-math-decision-sparse.md) rule out a
particular small universal atom-mass filtering representation on their measured
carrier. That does not refute using purpose-specific relational descriptions for
an exercise family. This proposal makes no universal state-compression or
first-build speed claim.

**Recommendation:** use Scheme's relational semantics for the gym's descriptions,
the current lawful evaluator for its answer keys, and the archived factory as the
reference for generalization and counterexamples. Recover only the needed pieces
into the unified structure. First earn a small collection of independently checked
partnership opportunities; choose a player mechanism from its observed failures.
