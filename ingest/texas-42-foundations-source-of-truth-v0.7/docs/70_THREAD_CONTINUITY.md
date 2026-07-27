# Thread Continuity and Provenance

This document prevents the independent foundation from losing the important
corrections, boundaries, and research directions established before v0.6. It
is **non-normative**: the rules and mathematics remain authoritative in files
10, 20, 30, and 40.

The founding research mandate is archived verbatim at:

`provenance/RESEARCH_MANDATE_2026-07-23.md`

## 1. Continuity verdict

v0.7 preserves every load-bearing conclusion from the prior formalization and
strengthens several of them. No earlier repository implementation is treated as
an axiom. The independent project is therefore a semantic extraction, not a
fork of the game.

## 2. Founding claims and their current homes

| Founding claim | Current authoritative location |
|---|---|
| A physical domino is a stable identity, not a stable strategic type. | Math §§2–3, 9; Scope thesis |
| Declaration selects/re-marks one game algebra from a bundle. | Math §§3.8, 9.4; diagrams §13.4 |
| Natural suits are a covering, not a partition. | Math §2.2 and §3.3 |
| A hand is an ambient marked embedding, not an isolated seven-node induced graph. | Math §9.1–§9.3 |
| Play is global relocation and local node expenditure, not annihilation. | Math §5.4 and §9.5; Executable play transitions |
| Lead threat is monotone but not a complete ontology. | Math §3.7; claim ledger ALG-14/15 |
| Hidden cells are dependent capacity constraints. | Math §7.1–§7.3 |
| `(P,k)` cells are lossless for Straight rule support in scope. | Math §7.5–§7.6 |
| Enumeration is a query, not state. | Math §7.3, §7.8; README correctness boundary |
| Hall feasibility is exact but does not imply legal reachability. | Math §7.7 and §7.13 |
| Mechanical support and posterior belief are different objects. | Math §6.7, §8 |
| Public action history supplies policy-relative evidence. | Math §8.1–§8.8 |
| Coordinate alone does not generally determine strategic value. | Math §10.3–§10.4 |
| Exact decision state may require retained record and latent field state in addition to physical belief. | Math §10.1, §15 |
| Value is derived from state, belief, continuation field, and utility. | Math §11.2, §15 |
| Local hand order is a gauge; action values are equivariant. | Math §9.6 |
| Seat rotation is a scoped symmetry; reflection fails for the oriented game. | Math §12.5–§12.7 |
| The `2 <-> 3` map is a scoped declaration transport, not a global pip symmetry. | Math §3.9 |
| Full play and early settlement are different semantics connected by a scoped quotient. | Math §12.9; Rules §§6–8 |
| Shared partnership utility does not imply shared private information. | Math §11.5 |
| A physical coordinate is not automatically a perfect-recall information set. | Math §6.6 and §12.4 |
| Reachability evidence is erasable and must not become semantic identity. | Math §7.13; Executable §§10, 18, 25; v0.7 correction |

## 3. Original mandate coverage

The original mandate asked for a front-to-back formal object rather than a
polish of project prose. The current package covers its primary requirements as
follows.

| Mandate area | v0.7 coverage |
|---|---|
| double-six algebra | Math §§2–3; Executable §§3–5 |
| auction and declaration bundle | Rules §3; Math §4; Executable §§7–9 |
| objective world and play transition | Math §5; Executable §§10–14 |
| perfect-recall information | Math §6.1–§6.2; Executable information interfaces |
| mechanical/support projection | Math §6.4; Executable §§15, 17 |
| delimited cells and exact fiber | Math §7.1–§7.8; Executable §§15–16 |
| support minimality and reachability | Math §7.9–§7.15; Executable §18 |
| belief and action evidence | Math §8; Executable §19 |
| marked hand in situ | Math §9; Executable §20 |
| strategic state and counterexample | Math §10; verifier witness |
| utility and best response | Math §11; Executable §§21–22 |
| congruence, quotient, gauge, symmetry | Math §12 |
| proof-status ledger | `40_CLAIM_STATUS.md` |
| proof-assistant boundary | `60_PROOF_ASSISTANT_HANDOFF.md` |

## 4. Earlier corrections that remain protected

### 4.1 C4 did not prove coordinate-only strategic sufficiency

The earlier repository spot gate compared equal coordinates under a restricted
fixed field and physics-world treatment. It was useful as an implementation
leakage check, not a theorem that arbitrary history-conditioned value factors
through a path-free coordinate.

The current 90-world theorem is the permanent guardrail. Equal mechanical
state and equal exact support can coexist with different posterior weights and
opposite optimal actions.

### 4.2 Atlas-like coordinates are physical/support residue, not the full strategic state

Any implementation coordinate that carries the fields of Math §6.4 may be an
exact source for future physical transitions and exact support. It does not by
itself retain every policy likelihood, perfect-recall observation, or utility
residue.

### 4.3 The hand object requires its ambient boundary

The current formalization does not regress to an induced owned-hand graph.
External live nodes, their possible holders, contextual follow relations, and
current trick context remain part of the action's meaning.

### 4.4 Node deletion remains scoped

A tile is deleted from the actor's remaining controllable set. It is relocated
in the global 28-node location state. The terminology is now protected in both
mathematical and executable layers.

### 4.5 Team-game honesty remains in force

No statement in this foundation licenses treating two partners as one
fully-informed player. Any later regret or equilibrium annex must distinguish:

- one-seat best response against a fixed field;
- unilateral four-agent regret under shared team utility;
- joint team deviation;
- an ex ante coordinator/prescription game.

A low unilateral gap is not automatically a team exploitability certificate.

## 5. Intentionally deferred prior-thread material

These subjects are not lost; they are outside the normative foundation.

### 5.1 Repository implementation correspondence

Prior work connected the theory to the TypeScript engine, declaration tables,
Atlas, role/threat tables, Walt, Hoyt, C4, and the equivalence census. Recreate
that as a versioned conformance annex against a chosen repository commit after
the proof-assistant definitions stabilize. It should remain evidence, never
rule authority.

### 5.2 Regret and coordinator annex

A future annex should restore the exact distinctions above and prove any
behavioral equivalence before importing two-player zero-sum CFR guarantees.
The foundation currently stops at finite fixed-field information-set best
response and the partnership information boundary.

### 5.3 Architecture consequences and experiments

The following remain valid research hypotheses rather than theorems:

- permutation-invariant/equivariant owned-hand encoding;
- contextual node/action heads;
- explicit ambient-boundary features;
- declaration-conditioned shared encoders;
- leave-one-out or post-expenditure residual representations;
- P-ENC, P-DEL, and P-DECL experiments.

They should be derived from the proved interfaces after mechanization, not
embedded in the game definition.

## 6. Terminology crosswalk

| Earlier phrase | Current precise term |
|---|---|
| native coordinate | mechanical/support projection, with scope stated |
| node deletion | node expenditure from remaining hand; location transition globally |
| exact unknowns | dependent capacity cells and current-remainder fiber |
| fiber state | fiber as an intensional derived set/query |
| non-signaling field | named continuation field with explicit retained-memory dependence |
| value factors through coordinate | field-/belief-/utility-relative quotient theorem, never automatic |
| higher-order knowledge collapses | hierarchy is uniquely induced on path under common prior/model; not an independent primitive |
| two-team game | decentralized partnership game unless a coordinator equivalence is separately constructed |
| reachability certificate | exact replay witness proves reachability; the 46-bit upper-bound object is only a necessary outer profile |

## 7. Continuity acceptance test

A future revision has drifted if it does any of the following without an
explicit theorem or counterexample:

1. stores support and mechanical residue as independent semantic truth;
2. lets proof or witness identity distinguish game states;
3. infers uniform probability from support alone;
4. merges public histories merely because physical residues match;
5. assigns context-free strategic value to a domino;
6. reduces the native hand to its owned induced graph;
7. treats lead-threat roles as the whole play algebra;
8. makes early settlement identical to full play for every utility;
9. centralizes partner information without changing the game label;
10. imports two-player CFR guarantees into the decentralized four-seat game;
11. promotes external verification output to a kernel theorem;
12. turns an implementation artifact into rule authority.

## 8. Provenance policy

- The archived mandate records why the project exists and what adversarial
  checks were required.
- This continuity document records semantic lineage.
- Current numbered documents are the source of truth.
- Repository snapshots, implementation receipts, and experimental results
  should always name their exact version and remain below the mathematical
  authority layer.
