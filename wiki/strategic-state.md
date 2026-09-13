# Strategic State, Utility, Quotients, and Gauges

[Home](Home.md) · owns: the marked hand, the decision state (c, e, β), utility
lenses, quotients and gauges · Sources: both packages Math §§9–12, rec §12.7.1
(rec-only). Related: [belief-vs-support](belief-vs-support.md),
[reduced-viewer-kernel](reduced-viewer-kernel.md).

## The native marked hand (Math §9)

A hand is **not** an isolated induced 7-node graph: two ambient states with identical
owned structure can differ in which external tiles are live threats and who can hold
them [Constructed counterexample, HAND-01]. The exact object is the *owned marking in
the full 28-node declaration algebra* plus public location, voids, certain-holder
marks, and the tagged ambiguity component [Definition, HAND-02]. Local slot order is a
gauge (physical values invariant, slot-indexed outputs equivariant) [Theorem — proved,
HAND-06]; additive "intrinsic + interaction" attributions are non-identifiable without
extra conventions [Theorem — proved, HAND-08].

## The exact decision state (Math §10)

`B = (c, e, β)`: mechanical/support state, **required retained continuation record**
(viewer-known residue the strategy/field/utility can still consult), and augmented
belief (measure over remainder worlds × latent field state on the admissible domain).
Under seven explicit assumptions (reconstruction, Markov physics/field, exact
filtering, retained utility residue, finite horizon, measurability/integrability),
fixed-strategy values and any attained best-response correspondence are functions of
`B` [Theorem — proved, STR-01]. `(c, β)` is exact shorthand only when `e` is
trivial/fixed [STR-02]. rec refines `c` to the reduced kernel `K`
([reduced-viewer-kernel](reduced-viewer-kernel.md), FAC-02).

Coordinate-only value criterion [Theorem — proved, STR-04]: a scalar value factors
through a projection iff it is constant on projection fibers — and scalar
factorization is strictly weaker than action-value factorization [Constructed
counterexample, STR-04A]. The [90-world witness](belief-vs-support.md) shows the
mechanical projection fails the criterion for history-sensitive fields.

## Utility (Math §11)

Named lenses: declaring points, signed differential (`= 2·points − 42`, same ordering,
UTIL-02), contract success, hand marks, match win. Expected points and contract
success can rank lotteries **oppositely** [Constructed counterexample, UTIL-03]; the
90-world witness flips the actual best action under all four hand lenses. Best
response: with finitely many reachable information records, bounded utility, fixed
field, a deterministic contingent policy attains the max; independent private
randomization can't beat it [Theorems — proved, UTIL-04A/B]; infinite signal models
need measurable-selection assumptions [Boundary, UTIL-04C]. **Shared partnership
utility does not merge partner information** — a both-hands controller is a different
game absent a proved equivalence [Proposition, TEAM-01].

## Quotients and gauges (Math §12)

- Physical congruence: the reduced play state is Markov for the hand; plus match
  residue (incl. shaker) for match continuation [QUO-01, PLAY-07].
- Strategic quotients are field/utility/information-relative isomorphism theorems,
  never automatic [QUO-02]; support can forget completed-play attribution, but
  evidence/likelihood generally cannot [QUO-03/04].
- Seat rotations form C₄ with team-orientation transport [SYM-01]; bidder anchoring is
  an exact post-auction gauge [SYM-02]; naked reflection **fails** (reverses clockwise
  successor) [Constructed counterexample, SYM-03].
- **rec-only [SYM-04, Theorem — proved + finite verification, rec Math §12.7.1]**:
  adding an orientation field `η ∈ {±1}` and transporting *everything* (actor order,
  partnerships, evidence, utility orientation) makes rotations + reflections an exact
  **D₄ coordinate gauge** on the oriented family — a canonicalization tool (≤2×
  further reduction), not an automorphism of the fixed clockwise game.
- Outcome-determined early settlement is a *scoped* quotient: preserves current-hand
  make/set, award, score update, match-end; later-hand match value only under explicit
  reset assumptions [QUO-07/08]. Full seven-trick play stays primitive.

## Mechanization status (proof-assistant kernel tier)

Rows are **v0.7** `65_MECHANIZATION_LEDGER.md` `PA-` rows (priority in parentheses);
"proved" = a declaration under `lean/Texas42/` checked by the Lean kernel over at most
`propext`/`Classical.choice`/`Quot.sound`, with no `sorry`, `native_decide` or local
axiom (grep re-verified 2026-09-12), as of commit d190b26 (2026-08-02; all 42
priority-0 rows closed). Map: [lean-row-index](lean-row-index.md). A kernel theorem
never promotes a corpus status. Two of this page's results are kernel-proved
(strategic sufficiency, generically; the information/mechanical non-injectivity);
the utility lenses, quotients and gauges are all open or unlisted.

| Result on this page | Ledger row (priority) | Kernel status (d190b26) | Declaration (`lean/Texas42/`) |
|---|---|---|---|
| STR-01 exact decision state `B = (c, e, β)`: fixed-strategy values and best responses are functions of `B` under the §10.1 assumptions | PA-E07 (0) | **proved generically** (2026-07-31): for a finite-horizon viewer decision process with latent state, `beliefVal σ n s β = β.exp (latentVal σ n s)` at every horizon, zero-probability segments contribute zero; hence any finite-class best response is a function of `(s, β)`. The **Straight-42 instantiation** — wiring `CertifiedState`/`physicalBelief` into a concrete `BeliefProc` — is **not done** (the PA-E08+ tier) | `Strategic.lean` `BeliefProc`, `:153` `beliefVal_eq_exp_latentVal`, `:284` `bestResponse_eq` |
| STR-02 `(c, β)` shorthand | — | not stated | — |
| STR-04/04A coordinate-only value criterion; scalar ≠ action-value factorization | PA-E09 (1) | **open** | — |
| UTIL-04A/B deterministic best response attains the max | PA-E08 (1) | **open** | — |
| UTIL-03 expected points vs contract success rank oppositely | PA-E12 (2, WITNESS) | **open** | — |
| TEAM-01 shared utility does not merge partner information | PA-F07 (1, PROVE/BOUNDARY) | **open** | — |
| HAND-06 local slot order is a gauge | PA-F01 (1) | **open** | — |
| HAND-01/02/08 marked hand; non-identifiable attributions | no rows | not mechanized | — |
| The 90-world witness this page cites | PA-E10 (0) | **proved** — see [belief-vs-support](belief-vs-support.md) | `Witness.lean:728` |
| QUO-01 / PLAY-07 physical congruence (reduced play state Markov for the hand) | PA-B12 (1) | **open** | — |
| INFO-10 mechanical projection ≠ information state (the boundary the quotients respect) | PA-F05 (0) | **proved**: two `DealLocalInfo` records differing only in which losing seat bid `P(30)` are distinct with identical mechanical projections | `Information.lean:47` `mech_not_injective` |
| QUO-02 field-relative strategic isomorphism; QUO-03/04 evidence cannot be forgotten | PA-F06 (2); no rows | **open** / not mechanized | — |
| SYM-01 rotations `C₄`; SYM-02 bidder anchoring; SYM-03 reflection fails | PA-F02, PA-F03, PA-F04 (2) | **open** | — |
| SYM-04 (rec) oriented `D₄` gauge | no row (rec Kernel spine K13) | not mechanized | — |
| QUO-07/08 early settlement is a scoped quotient | PA-B14 (2) | **open** | — |
