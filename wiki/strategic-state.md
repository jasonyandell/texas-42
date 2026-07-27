# Strategic State, Utility, Quotients, and Gauges

[Home](Home.md) · Sources: both packages Math §§9–12 (shared), rec §12.7.1 (rec-only).
Related: [belief-vs-support](belief-vs-support.md), [reduced-viewer-kernel](reduced-viewer-kernel.md).

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
