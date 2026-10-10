//! `solver::hazard` — the Hazard-Exclusion Invariant verifier (the single
//! dominance-bound authority), the δ = 0 structural-hazard result type,
//! and the one-round trump-extraction witness producer (slice 4b).
//!
//! EXPLORATORY tier. **[L2 thread; dominance objective-level.]**
//! Implements Part 2 of the x:024 response
//! (`exchange/inbox/024-response-deferred-producers-triple-v0.1.md`
//! §§2.1–2.8, proof ledger P5; intake companion
//! `walt/math/response_deferred_producers_triple_v0.1_intake.md`), adopted
//! by rulings **TRIPLE-A4/A5** (`walt/CENSUS-RULINGS.md`, "The
//! deferred-producers adjudication (2026-08-25)"), under PANEL-A7's
//! dominance vocabulary (`H(a|b) = 0 ∧ B(a|b) > 0` is strict dominance,
//! one-sided unforced risk, never "cancellation"). Nothing here promotes
//! the response's tier: the mathematics is implemented and cited, and the
//! gates are release tests, not receipts.
//!
//! ## The architecture (TRIPLE-A4)
//!
//! There is no canonical weakest LOCAL exchange predicate (§2.1). The
//! weakest exact condition for the PANEL-A7 "valid bound" route into
//! `Dominated` is unreachability of the hazard terminal
//! `u_a = 0, u_b = 1` in the deterministic paired product of the two
//! frozen focal executions on one world (§2.2). The machine-checkable
//! proof object is a **Hazard-Exclusion Invariant** (§2.3): a paired-state
//! predicate with initial coverage, forward closure, and terminal safety —
//! sound (H1) and semantically complete (H2: the reachable set is itself
//! an invariant; incompleteness lives only in the chosen witness
//! language). One general verifier ([`verify_hazard_witness`]) is the
//! single authority; pattern producers ([`one_round_trump_extraction`])
//! emit witnesses FOR that authority and own no dominance semantics of
//! their own.
//!
//! ## The v1 witness language and what the verifier actually checks
//!
//! A [`HazardExclusionWitness`] is the §2.4 symbolic proof DAG: cells,
//! initial cover, successor obligations, terminal implications. The v1
//! language expresses **two-remaining-trick focal-lead roots** (every hand
//! two tiles, the viewer leading, no partial trick) with cells at trick
//! granularity: per world-selector chain, an initial cell, a mid cell
//! claiming the viewer personally won the first paired trick of the
//! a-branch and no hostile seat retains a called tile, and a terminal cell
//! claiming the focal team won every trick of the a-branch. The verifier
//! checks exactly the response's three conditions:
//!
//! 1. **Initial coverage** — the EXACT fiber has zero worlds outside the
//!    initial cell cover: every world of the exact fiber enumeration
//!    satisfies some initial cell's selector (never sampling; TRIPLE-A4).
//! 2. **Forward closure** — rules-level symbolic closure: for every cell,
//!    for every rule-consistent hidden-tile configuration admitted by the
//!    cell (a SUPERSET of the fiber — kernel voids are deliberately
//!    ignored) and every rule-legal completion of the trick by the
//!    non-focal seats (a SUPERSET of the declared field's and either
//!    policy's choices), the reached paired state satisfies a listed
//!    successor cell. Over-approximating the transition relation is sound:
//!    an invariant closed under a superset of `T` is closed under `T`, so
//!    the deterministic paired successor is among the states checked. The
//!    focal branch-a plays are NOT over-approximated: the root lead is the
//!    policy's verified choice at the fully-determined root record, and
//!    the second play is forced (singleton hand).
//! 3. **Terminal safety** — every terminal cell claims the focal team won
//!    every a-branch trick since the root. That pins the a-branch terminal
//!    at the extremal banked totals (own team banks every remaining
//!    point), and the viewer-objective payoff is monotone in the declaring
//!    team's banked points (nondecreasing for a declaring viewer,
//!    nonincreasing for a defender), so `u_a` is the pointwise maximum
//!    over ALL continuations — in particular `u_b ≤ u_a` on every world of
//!    the cell. This is P5's terminal-safety step discharged by a
//!    monotonicity lemma rather than by b-branch tracking; the b-branch
//!    claims are honestly `Unconstrained`.
//!
//! Soundness is P5 (induction on finite play depth) plus the two lemmas
//! above. Verification is dumb and cheap by design (§2.4: discovery may be
//! expensive; verification must be cheap): the closure sweep touches at
//! most 90 configurations × at most 8 completions per trick and never
//! replays a policy to terminal, never consults the field model, and never
//! computes any `u` by simulation.
//!
//! ## The δ = 0 type and the dominance route
//!
//! [`StructuralHazardZero`] (`hazard_upper = 0, delta = 0`) has private
//! fields and NO public constructor: the only producer is
//! [`verify_hazard_witness`] accepting a witness. A sampled object can
//! never inhabit it (the compile-fail doctest below), and
//! `SampledPairwiseMasses` still has no dominance method — the PANEL-A7
//! type lock stands sharpened, not moved. The wiring into `Dominated` is
//! [`dominance_from_witnessed_hazard_zero`]: a verified `H = 0` witness
//! plus exact benefit evidence `B > 0` (one exhibited fiber world with
//! `u_a = 1, u_b = 0`, replayed exactly — [`exhibit_benefit_world`]). The
//! existing exact-enumeration route
//! (`ExactPairwiseMasses::dominance_kind`) stays untouched beside it.
//!
//! ## No cross-field composition (§2.8)
//!
//! A witness is per-field: every object here carries ONE `FieldId`, and
//! [`dominance_from_witnessed_hazard_zero`] asserts the witness and the
//! benefit exhibit name the same root, policies, and field — nothing
//! derives dominance under field 1 from a field-0 witness. (The v1
//! closure check incidentally quantifies over the full legal action family
//! — stronger than any one field — but the result TYPE still binds to the
//! single declared `FieldId`; a field-action-family witness type is
//! strictly stronger and deliberately out of scope.)
//!
//! ## The first producer (TRIPLE-A5) and its refusal path
//!
//! [`one_round_trump_extraction`] certifies the
//! highest-trump-versus-vulnerable-tile pattern under the response §2.5
//! hypotheses and DECLINES outside them — the refusal path is part of its
//! correctness. Declared check order (an implementation ordering of the
//! response's conjunction, cheap checks before field probes): focal-lead
//! shape; hypotheses 1–4 (which are depth-independent, so the §2.7
//! three-trick two-round specimen declines at hypothesis 3 exactly as the
//! response states); the v1 two-trick language boundary; hypothesis 7
//! (contract arithmetic); hypotheses 5–6 (the declared-field ruff probes,
//! including the companion subtlety: decline when follow-suit obligations
//! block the proposed ruff in every threat world); hypothesis 8.
//! Hypothesis 3 is ported to the
//! four-seat game as "the hostile seats hold at most one called tile in
//! total" — the two-cell `NoHostileTrump` / `OneHostileTrump` partition of
//! §2.5 requires it. The standing non-coverage instance (three-trick
//! two-round extraction: dominance real, producer declines) is a gate.

use std::fmt;

use num_rational::BigRational;
use num_traits::Zero;

use crate::rules::rules::{legal_plays, Trick};
use crate::rules::{Decl, Domino, DominoSet, Seat};
use crate::solver::adaptive::{
    replay_viewer_success, root_identity, world_id, CanonicalRoot, PublicRecord, RootPosition,
    SlicePolicy,
};
use crate::solver::field::{FieldId, FieldModel};
use crate::solver::field_swap::CancellationKind;

// ---------------------------------------------------------------------------
// The v1 witness language (response §2.4's DAG, trick-granular cells).
// ---------------------------------------------------------------------------

/// A v1 world selector: a structural predicate over the assignment of the
/// root's hidden tiles to the three non-viewer seats. "Hostile" means the
/// two seats not on the viewer's team; called tiles are the declaration's
/// trump set.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldSelector {
    /// No hostile seat holds any called tile.
    NoHostileTrump,
    /// Exactly one called tile total across the two hostile hands (which
    /// implies the §2.5 hypothesis "every hostile hand holds at most
    /// one").
    OneHostileTrump,
}

impl WorldSelector {
    fn tag(self) -> &'static str {
        match self {
            WorldSelector::NoHostileTrump => "NoHostileTrump",
            WorldSelector::OneHostileTrump => "OneHostileTrump",
        }
    }

    /// Does the selector admit this assignment of hands? `hands` is
    /// seat-indexed; only the hostile seats' holdings are read.
    fn admits(self, decl: Decl, viewer: Seat, hands: &[DominoSet; 4]) -> bool {
        let called = decl.called_set();
        let hostile: usize = [viewer.plus(1), viewer.plus(3)]
            .into_iter()
            .map(|s| hands[s.index()].intersection(called).len())
            .sum();
        match self {
            WorldSelector::NoHostileTrump => hostile == 0,
            WorldSelector::OneHostileTrump => hostile == 1,
        }
    }
}

/// A v1 branch claim — the structural facts a cell asserts of one branch
/// of the paired execution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BranchClaim {
    /// The viewer personally won every paired trick completed since the
    /// root, leads the next trick, and no hostile seat retains a called
    /// tile (the extraction fact, carried forward).
    ViewerSweptLeadsHostilesTrumpless,
    /// The focal team won every trick completed since the root. With all
    /// tricks done this pins the extremal terminal: the viewer's team
    /// banked every remaining point.
    TeamWonEveryTrick,
    /// No claim. The v1 b-branch claim everywhere: terminal safety rests
    /// on the a-branch maximum, honestly.
    Unconstrained,
}

impl BranchClaim {
    fn tag(self) -> &'static str {
        match self {
            BranchClaim::ViewerSweptLeadsHostilesTrumpless => "ViewerSweptLeadsHostilesTrumpless",
            BranchClaim::TeamWonEveryTrick => "TeamWonEveryTrick",
            BranchClaim::Unconstrained => "Unconstrained",
        }
    }
}

/// One symbolic paired cell (response §2.4): a structural predicate over
/// paired states at a common depth, in the v1 language — how many paired
/// tricks are complete, which root worlds the cell ranges over, and one
/// claim per branch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolicPairedCell {
    pub tricks_done: usize,
    pub selector: WorldSelector,
    pub claim_a: BranchClaim,
    pub claim_b: BranchClaim,
}

/// One successor obligation: from a nonterminal cell, the list of cells
/// its deterministic paired successor must land in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SuccessorObligation {
    pub from: usize,
    pub to: Vec<usize>,
}

/// The §2.4 machine-checkable witness object: identities, the symbolic
/// paired cells, the initial cover, the successor obligations, and the
/// terminal implications, plus the two declared focal root leads the
/// closure argument is built on. All fields are public: a witness is a
/// CLAIM anyone may assemble; only [`verify_hazard_witness`] accepting it
/// mints a [`StructuralHazardZero`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HazardExclusionWitness {
    pub root_id: u64,
    pub policy_a: String,
    pub policy_b: String,
    pub field_id: FieldId,
    /// Policy a's root lead (the §2.5 pattern's high called tile).
    pub lead_a: Domino,
    /// Policy b's root lead (the vulnerable nontrump tile).
    pub lead_b: Domino,
    pub cells: Vec<SymbolicPairedCell>,
    /// Indices of the initial cells whose selectors must cover the fiber.
    pub initial_cover: Vec<usize>,
    pub successor_obligations: Vec<SuccessorObligation>,
    /// Indices of the terminal cells; each must establish `u_b ≤ u_a`.
    pub terminal_implications: Vec<usize>,
}

impl HazardExclusionWitness {
    /// FNV-1a 64 over the canonical serialization — the identity the δ = 0
    /// result carries. Integer arithmetic only.
    pub fn digest(&self) -> u64 {
        const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
        const PRIME: u64 = 0x0000_0100_0000_01b3;
        let mut h = OFFSET;
        let mut eat = |bytes: &[u8]| {
            for b in bytes {
                h ^= u64::from(*b);
                h = h.wrapping_mul(PRIME);
            }
        };
        eat(b"walt-hazard-witness-v1");
        eat(&self.root_id.to_be_bytes());
        eat(self.policy_a.as_bytes());
        eat(&[0]);
        eat(self.policy_b.as_bytes());
        eat(&[0]);
        eat(self.field_id.bytes());
        eat(&[
            u8::try_from(self.lead_a.index()).expect("tile index < 28"),
            u8::try_from(self.lead_b.index()).expect("tile index < 28"),
        ]);
        for cell in &self.cells {
            eat(&[1, u8::try_from(cell.tricks_done).expect("small depth")]);
            eat(cell.selector.tag().as_bytes());
            eat(cell.claim_a.tag().as_bytes());
            eat(cell.claim_b.tag().as_bytes());
        }
        for i in &self.initial_cover {
            eat(&[2, u8::try_from(*i).expect("small id")]);
        }
        for ob in &self.successor_obligations {
            eat(&[3, u8::try_from(ob.from).expect("small id")]);
            for t in &ob.to {
                eat(&[4, u8::try_from(*t).expect("small id")]);
            }
        }
        for i in &self.terminal_implications {
            eat(&[5, u8::try_from(*i).expect("small id")]);
        }
        h
    }
}

// ---------------------------------------------------------------------------
// The verifier — the single dominance-bound authority (TRIPLE-A4).
// ---------------------------------------------------------------------------

/// Why the verifier refused a witness. A rejection is about the WITNESS
/// (or the v1 language's reach), never a statement that hazard mass is
/// positive.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WitnessRejection {
    /// The witness names a different root, policy pair, or field than the
    /// objects presented, or a declared lead is not the policy's actual
    /// root choice.
    IdentityMismatch { what: String },
    /// The root or the witness DAG is outside the v1 witness language
    /// (not a two-trick focal-lead root, or cells not in the v1 shape).
    /// H2 note: this is the language's incompleteness, not a hazard
    /// finding.
    LanguageShape { why: String },
    /// Some exact-fiber world lies outside the initial cell cover.
    InitialCoverIncomplete { world: [u32; 4] },
    /// Some rule-legal configuration and completion reaches a paired state
    /// satisfying no listed successor cell.
    SuccessorObligationBroken { cell: usize, why: String },
    /// A terminal cell's claims do not establish `u_b ≤ u_a`.
    TerminalUnsafe { cell: usize },
}

impl fmt::Display for WitnessRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WitnessRejection::IdentityMismatch { what } => {
                write!(f, "WitnessRejection{{identity-mismatch;{what}}}")
            }
            WitnessRejection::LanguageShape { why } => {
                write!(f, "WitnessRejection{{language-shape;{why}}}")
            }
            WitnessRejection::InitialCoverIncomplete { world } => write!(
                f,
                "WitnessRejection{{initial-cover-incomplete;world={:08x}/{:08x}/{:08x}/{:08x}}}",
                world[0], world[1], world[2], world[3]
            ),
            WitnessRejection::SuccessorObligationBroken { cell, why } => {
                write!(f, "WitnessRejection{{successor-broken;cell={cell};{why}}}")
            }
            WitnessRejection::TerminalUnsafe { cell } => {
                write!(f, "WitnessRejection{{terminal-unsafe;cell={cell}}}")
            }
        }
    }
}

/// The δ = 0 structural-hazard result (response §2.4's type): a VERIFIED
/// hazard-exclusion claim `H(a|b) = 0` with `hazard_upper = 0, delta = 0`
/// for one root, one ordered frozen policy pair, and ONE field. Fields are
/// private and there is no public constructor: the only producer is
/// [`verify_hazard_witness`] accepting a witness, so a sampled object can
/// never inhabit this type — the PANEL-A7/TRIPLE-A4 lock, at compile time:
///
/// ```compile_fail
/// use walt::solver::field_swap::SampledPairwiseMasses;
/// use walt::solver::hazard::StructuralHazardZero;
///
/// fn sampled_zero_hazards_reach_delta_zero(s: &SampledPairwiseMasses) -> StructuralHazardZero {
///     // Private fields, no public constructor: this does not compile,
///     // whatever the sampled hazard count says.
///     StructuralHazardZero {
///         witness_hash: 0,
///         root_id: s.root_id(),
///         policy_a: s.policy_a().to_string(),
///         policy_b: s.policy_b().to_string(),
///         field_id: s.field(),
///     }
/// }
/// ```
///
/// And the sampled sibling still has no dominance method of its own:
///
/// ```compile_fail
/// use walt::solver::field_swap::SampledPairwiseMasses;
///
/// fn sampled_masses_have_no_dominance_route(s: &SampledPairwiseMasses) {
///     // No such method exists on the sampled type (PANEL-A7): this does
///     // not compile.
///     let _ = s.dominance_kind();
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructuralHazardZero {
    witness_hash: u64,
    root_id: u64,
    policy_a: String,
    policy_b: String,
    field_id: FieldId,
}

impl StructuralHazardZero {
    /// The verified upper bound on the hazard mass: exactly zero.
    pub fn hazard_upper(&self) -> BigRational {
        BigRational::zero()
    }

    /// The failure probability of the bound: exactly zero — this is a
    /// structural result, not a sampled one.
    pub fn delta(&self) -> BigRational {
        BigRational::zero()
    }

    pub fn witness_hash(&self) -> u64 {
        self.witness_hash
    }

    pub fn root_id(&self) -> u64 {
        self.root_id
    }

    pub fn policy_a(&self) -> &str {
        &self.policy_a
    }

    pub fn policy_b(&self) -> &str {
        &self.policy_b
    }

    pub fn field_id(&self) -> FieldId {
        self.field_id
    }
}

impl fmt::Display for StructuralHazardZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StructuralHazardZero{{a={};b={};field={};root={:#018x};hazard_upper=0;delta=0;\
             witness={:#018x}}}",
            self.policy_a, self.policy_b, self.field_id, self.root_id, self.witness_hash
        )
    }
}

/// All ways to deal the six-tile pool as three two-tile hands to the three
/// non-viewer seats, seat-indexed alongside the viewer's own hand. This is
/// the closure sweep's configuration space: a SUPERSET of the fiber (voids
/// are deliberately ignored — over-approximation is sound here).
fn two_tile_assignments(
    viewer: Seat,
    viewer_hand: DominoSet,
    pool: DominoSet,
) -> Vec<[DominoSet; 4]> {
    let tiles: Vec<Domino> = pool.iter().collect();
    assert_eq!(tiles.len(), 6, "a two-trick pool holds six tiles");
    let mut out = Vec::new();
    for i in 0..6 {
        for j in (i + 1)..6 {
            let first = DominoSet::single(tiles[i]).union(DominoSet::single(tiles[j]));
            let rest: Vec<Domino> = tiles
                .iter()
                .copied()
                .filter(|t| !first.contains(*t))
                .collect();
            for k in 0..4 {
                for l in (k + 1)..4 {
                    let second = DominoSet::single(rest[k]).union(DominoSet::single(rest[l]));
                    let third = rest
                        .iter()
                        .copied()
                        .filter(|t| !second.contains(*t))
                        .collect::<DominoSet>();
                    let mut hands = [DominoSet::EMPTY; 4];
                    hands[viewer.index()] = viewer_hand;
                    hands[viewer.plus(1).index()] = first;
                    hands[viewer.plus(2).index()] = second;
                    hands[viewer.plus(3).index()] = third;
                    out.push(hands);
                }
            }
        }
    }
    out
}

/// The rules-level first-trick closure sweep for one selector: from every
/// admitted configuration, the a-branch trick led by `lead_a` must be won
/// by the viewer under EVERY rule-legal completion, and every hostile
/// residual must be trumpless afterward (the mid cell's claim). Returns
/// the first violation as prose.
fn trick_one_sweep(
    decl: Decl,
    viewer: Seat,
    viewer_hand: DominoSet,
    pool: DominoSet,
    selector: WorldSelector,
    lead_a: Domino,
) -> Result<(), String> {
    let led = decl.led_context(lead_a);
    let called = decl.called_set();
    for hands in two_tile_assignments(viewer, viewer_hand, pool) {
        if !selector.admits(decl, viewer, &hands) {
            continue;
        }
        let seats = [viewer.plus(1), viewer.plus(2), viewer.plus(3)];
        let legal: Vec<Vec<Domino>> = seats
            .iter()
            .map(|s| {
                legal_plays(decl, hands[s.index()], Some(led))
                    .iter()
                    .collect()
            })
            .collect();
        for t1 in &legal[0] {
            for t2 in &legal[1] {
                for t3 in &legal[2] {
                    let trick =
                        Trick::new(viewer, [lead_a, *t1, *t2, *t3]).expect("four distinct tiles");
                    let winner = trick.winner(decl);
                    if winner != viewer {
                        return Err(format!(
                            "completion {t1}/{t2}/{t3} takes the {lead_a} lead away from the \
                             viewer (winner seat {})",
                            winner.index()
                        ));
                    }
                    for (s, played) in seats.iter().zip([t1, t2, t3]) {
                        if s.team() == viewer.team() {
                            continue;
                        }
                        let residual = hands[s.index()].difference(DominoSet::single(*played));
                        if !residual.intersection(called).is_empty() {
                            return Err(format!(
                                "hostile seat {} retains a called tile after the {lead_a} lead",
                                s.index()
                            ));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// The rules-level second-trick closure sweep: from EVERY residual
/// configuration satisfying the mid cell's claim (one tile per non-viewer
/// seat, hostile residuals trumpless — a superset of the reachable
/// states), the viewer's forced last lead must be won by the focal team.
fn trick_two_sweep(
    decl: Decl,
    viewer: Seat,
    viewer_hand: DominoSet,
    pool: DominoSet,
    lead_a: Domino,
) -> Result<(), String> {
    let last = viewer_hand.difference(DominoSet::single(lead_a));
    assert_eq!(last.len(), 1, "a two-trick focal hand has one tile left");
    let last = last.iter().next().expect("one tile");
    let called = decl.called_set();
    let tiles: Vec<Domino> = pool.iter().collect();
    let seats = [viewer.plus(1), viewer.plus(2), viewer.plus(3)];
    for r1 in &tiles {
        for r2 in &tiles {
            for r3 in &tiles {
                if r1 == r2 || r1 == r3 || r2 == r3 {
                    continue;
                }
                let residuals = [*r1, *r2, *r3];
                if seats
                    .iter()
                    .zip(residuals)
                    .any(|(s, t)| s.team() != viewer.team() && called.contains(t))
                {
                    // Excluded by the mid cell's hostiles-trumpless claim,
                    // which the first-trick sweep established.
                    continue;
                }
                let trick = Trick::new(viewer, [last, *r1, *r2, *r3]).expect("distinct tiles");
                if trick.winner(decl).team() != viewer.team() {
                    return Err(format!(
                        "residuals {r1}/{r2}/{r3} take the {last} lead from the focal team"
                    ));
                }
            }
        }
    }
    Ok(())
}

/// The root public record — fully determined (no world enters it), the one
/// place the verifier consults the focal policies.
fn root_record<'a>(position: &'a RootPosition) -> PublicRecord<'a> {
    PublicRecord {
        leader: position.leader,
        trick_plays: &position.trick_plays,
        banked: position.banked,
        root: position,
        history: &[],
    }
}

/// Verify a [`HazardExclusionWitness`] — the three §2.4 checks, exactly,
/// as documented on the module. On acceptance, mint the δ = 0 result; on
/// refusal, say which check failed and why. This function is the single
/// authority for the PANEL-A7 valid-bound route (TRIPLE-A4); producers
/// only ever emit inputs to it.
pub fn verify_hazard_witness(
    root: &CanonicalRoot,
    position: &RootPosition,
    policy_a: &dyn SlicePolicy,
    policy_b: &dyn SlicePolicy,
    field: &FieldModel,
    witness: &HazardExclusionWitness,
) -> Result<StructuralHazardZero, WitnessRejection> {
    let mismatch = |what: &str| WitnessRejection::IdentityMismatch {
        what: what.to_string(),
    };
    let shape = |why: &str| WitnessRejection::LanguageShape {
        why: why.to_string(),
    };
    // Identities: the witness names exactly the presented objects.
    if witness.root_id != root_identity(root, position) {
        return Err(mismatch("root_id"));
    }
    if witness.policy_a != policy_a.id() {
        return Err(mismatch("policy_a"));
    }
    if witness.policy_b != policy_b.id() {
        return Err(mismatch("policy_b"));
    }
    if witness.field_id != field.field_id() {
        return Err(mismatch("field_id"));
    }
    // The v1 language's root shape: two remaining tricks, the viewer
    // leading, no partial trick.
    let kernel = root.kernel();
    let viewer = kernel.viewer();
    let viewer_hand = kernel.viewer_hand();
    let pool = kernel.pool();
    if !position.trick_plays.is_empty() || position.leader != viewer {
        return Err(shape("v1 expresses focal-lead roots only"));
    }
    if viewer_hand.len() != 2 || pool.len() != 6 {
        return Err(shape("v1 expresses two-remaining-trick roots only"));
    }
    // The declared leads are legal, held, and are the policies' ACTUAL
    // root choices at the fully-determined root record. The second focal
    // play is forced (singleton hand), so no further policy consultation
    // is needed or performed.
    let record = root_record(position);
    if !viewer_hand.contains(witness.lead_a)
        || policy_a.choose(position.decl, viewer_hand, viewer_hand, &record) != witness.lead_a
    {
        return Err(mismatch("lead_a is not policy a's root choice"));
    }
    if !viewer_hand.contains(witness.lead_b)
        || policy_b.choose(position.decl, viewer_hand, viewer_hand, &record) != witness.lead_b
    {
        return Err(mismatch("lead_b is not policy b's root choice"));
    }
    // DAG well-formedness in the v1 shape: initial cells at depth 0 with
    // unconstrained claims; every nonterminal cell has one obligation; a
    // mid cell claims the viewer swept and leads; terminal cells are
    // checked by the terminal-safety pass below.
    let cell = |i: usize| -> Result<&SymbolicPairedCell, WitnessRejection> {
        witness
            .cells
            .get(i)
            .ok_or_else(|| shape("a cell id is out of range"))
    };
    if witness.initial_cover.is_empty() {
        return Err(shape("an initial cover names at least one cell"));
    }
    for i in &witness.initial_cover {
        if cell(*i)?.tricks_done != 0 {
            return Err(shape("initial cells sit at depth zero"));
        }
    }
    for i in &witness.terminal_implications {
        if cell(*i)?.tricks_done != 2 {
            return Err(shape("v1 terminal cells sit at depth two"));
        }
    }
    // 1. Initial coverage over the EXACT fiber (never sampling): zero
    //    worlds outside the initial cell cover.
    for world in root.worlds() {
        let hands = world.hands();
        let covered = witness.initial_cover.iter().any(|i| {
            witness.cells[*i]
                .selector
                .admits(position.decl, viewer, &hands)
        });
        if !covered {
            return Err(WitnessRejection::InitialCoverIncomplete {
                world: world_id(&world),
            });
        }
    }
    // 2. Terminal safety: a terminal cell establishes u_b ≤ u_a exactly
    //    when it claims the focal team won every a-branch trick — then the
    //    a-branch banked totals are extremal and the viewer objective is
    //    monotone (the module-doc lemma; P5's terminal step). Checked
    //    before closure so an unsafe terminal is named as such.
    for i in &witness.terminal_implications {
        if witness.cells[*i].claim_a != BranchClaim::TeamWonEveryTrick {
            return Err(WitnessRejection::TerminalUnsafe { cell: *i });
        }
    }
    // 3. Forward closure, rules-level, per obligation.
    for ob in &witness.successor_obligations {
        let from = cell(ob.from)?;
        if from.tricks_done >= 2 {
            return Err(shape("a terminal cell carries no successor obligation"));
        }
        let successors: Vec<&SymbolicPairedCell> =
            ob.to.iter().map(|i| cell(*i)).collect::<Result<_, _>>()?;
        if successors.is_empty() {
            return Err(WitnessRejection::SuccessorObligationBroken {
                cell: ob.from,
                why: "an obligation lists no successor cell".to_string(),
            });
        }
        for succ in &successors {
            if succ.tricks_done != from.tricks_done + 1 || succ.selector != from.selector {
                return Err(shape(
                    "v1 successors advance one trick within one selector chain",
                ));
            }
            if succ.claim_b != BranchClaim::Unconstrained {
                return Err(shape("v1 carries no b-branch claims"));
            }
        }
        match from.tricks_done {
            0 => {
                // The successor claim v1 can discharge from the root is the
                // viewer-swept mid claim; the sweep below establishes it
                // for every admitted configuration and legal completion.
                if successors
                    .iter()
                    .any(|s| s.claim_a != BranchClaim::ViewerSweptLeadsHostilesTrumpless)
                {
                    return Err(shape(
                        "v1 discharges only the viewer-swept mid claim from the root",
                    ));
                }
                trick_one_sweep(
                    position.decl,
                    viewer,
                    viewer_hand,
                    pool,
                    from.selector,
                    witness.lead_a,
                )
                .map_err(|why| WitnessRejection::SuccessorObligationBroken {
                    cell: ob.from,
                    why,
                })?;
            }
            1 => {
                if from.claim_a != BranchClaim::ViewerSweptLeadsHostilesTrumpless {
                    return Err(shape(
                        "v1 checks obligations only from a cell that pins the viewer as leader",
                    ));
                }
                if successors
                    .iter()
                    .any(|s| s.claim_a != BranchClaim::TeamWonEveryTrick)
                {
                    return Err(shape(
                        "v1 discharges only the team-swept terminal claim from the mid cell",
                    ));
                }
                trick_two_sweep(position.decl, viewer, viewer_hand, pool, witness.lead_a).map_err(
                    |why| WitnessRejection::SuccessorObligationBroken { cell: ob.from, why },
                )?;
            }
            _ => unreachable!("depth checked above"),
        }
    }
    // Every nonterminal cell reachable through the DAG must carry an
    // obligation; v1 keeps this simple by demanding one per nonterminal
    // cell outright.
    for (i, c) in witness.cells.iter().enumerate() {
        if c.tricks_done < 2 && !witness.successor_obligations.iter().any(|ob| ob.from == i) {
            return Err(shape("every nonterminal cell carries an obligation"));
        }
        if c.tricks_done == 2 && !witness.terminal_implications.contains(&i) {
            return Err(shape("every terminal cell carries a terminal implication"));
        }
    }
    Ok(StructuralHazardZero {
        witness_hash: witness.digest(),
        root_id: witness.root_id,
        policy_a: witness.policy_a.clone(),
        policy_b: witness.policy_b.clone(),
        field_id: witness.field_id,
    })
}

// ---------------------------------------------------------------------------
// Exact benefit evidence and the dominance wiring (PANEL-A7's valid-bound
// route).
// ---------------------------------------------------------------------------

/// One exhibited fiber world with `u_a = 1, u_b = 0`, replayed exactly
/// under the declared field — the cheap exact `B(a|b) > 0` evidence.
/// Private fields: the only constructor replays the world it names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BenefitWorldExhibit {
    root_id: u64,
    policy_a: String,
    policy_b: String,
    field_id: FieldId,
    world: [u32; 4],
}

impl BenefitWorldExhibit {
    pub fn world(&self) -> [u32; 4] {
        self.world
    }

    pub fn root_id(&self) -> u64 {
        self.root_id
    }

    pub fn field_id(&self) -> FieldId {
        self.field_id
    }
}

impl fmt::Display for BenefitWorldExhibit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BenefitWorldExhibit{{a={};b={};field={};root={:#018x};\
             world={:08x}/{:08x}/{:08x}/{:08x}}}",
            self.policy_a,
            self.policy_b,
            self.field_id,
            self.root_id,
            self.world[0],
            self.world[1],
            self.world[2],
            self.world[3]
        )
    }
}

/// Scan the exact fiber for a world with `u_a = 1, u_b = 0` and exhibit
/// the first one found (exact terminal replays, early exit — B > 0 needs
/// one witness world, never a census). `None` honestly reports that no
/// benefit world exists, in which case dominance is out of reach whatever
/// the hazard bound says (§34's `B > 0` requirement).
pub fn exhibit_benefit_world(
    root: &CanonicalRoot,
    position: &RootPosition,
    policy_a: &dyn SlicePolicy,
    policy_b: &dyn SlicePolicy,
    field: &FieldModel,
) -> Option<BenefitWorldExhibit> {
    let viewer = root.kernel().viewer();
    for world in root.worlds() {
        let ua = replay_viewer_success(position, viewer, &world, policy_a, field);
        if !ua {
            continue;
        }
        let ub = replay_viewer_success(position, viewer, &world, policy_b, field);
        if !ub {
            return Some(BenefitWorldExhibit {
                root_id: root_identity(root, position),
                policy_a: policy_a.id().to_string(),
                policy_b: policy_b.id().to_string(),
                field_id: field.field_id(),
                world: world_id(&world),
            });
        }
    }
    None
}

/// The PANEL-A7 valid-bound route into `Dominated`: a VERIFIED `H = 0`
/// witness result plus exact `B > 0` evidence, for the SAME root, ordered
/// policy pair, and field — asserted, because a mismatch is a caller
/// contract violation, and §2.8 forbids deriving dominance under one field
/// from a witness under another. The exact-enumeration route
/// (`ExactPairwiseMasses::dominance_kind`) stands untouched beside this
/// one.
pub fn dominance_from_witnessed_hazard_zero(
    hazard: &StructuralHazardZero,
    benefit: &BenefitWorldExhibit,
) -> CancellationKind {
    assert_eq!(
        hazard.root_id, benefit.root_id,
        "one root per dominance statement"
    );
    assert_eq!(
        hazard.policy_a, benefit.policy_a,
        "one ordered pair per dominance statement"
    );
    assert_eq!(
        hazard.policy_b, benefit.policy_b,
        "one ordered pair per dominance statement"
    );
    assert_eq!(
        hazard.field_id, benefit.field_id,
        "no cross-field composition (response §2.8): a witness under one \
         field proves nothing under another"
    );
    CancellationKind::Dominated
}

// ---------------------------------------------------------------------------
// The one-round trump-extraction producer (response §2.5; TRIPLE-A5).
// ---------------------------------------------------------------------------

/// The producer's typed refusal — naming the failed hypothesis is part of
/// its correctness (TRIPLE-A5). Variants follow the response §2.5
/// hypothesis numbering where one applies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtractionDecline {
    /// The root is not a focal lead (partial trick, or a non-viewer
    /// leader) — the §2.5 frame's precondition.
    ShapeNotFocalLead,
    /// Hypothesis 1: policy a's root lead is not the globally highest
    /// remaining called tile.
    LeadNotHighestTrump,
    /// Hypothesis 2: policy b's root lead is not a vulnerable nontrump
    /// tile (called, or nothing remaining threatens it when led).
    LeadNotVulnerableNontrump,
    /// Hypothesis 3 (ported to four seats): some fiber world puts more
    /// than one called tile in the hostile hands.
    HostileTrumpsExceedOneRound,
    /// Hypothesis 4: some fiber world gives a hostile seat a non-called
    /// tile that beats the vulnerable lead in suit.
    HostileSuitBeater,
    /// The v1 witness language's boundary: not a two-remaining-trick root.
    /// A language limit, not a hypothesis failure (H2 note).
    TwoTrickShapeRequired,
    /// Hypothesis 5's premise is vacuous: no fiber world puts the called
    /// threat in a hostile hand, so no ruff threat exists and `B > 0`
    /// cannot come from this pattern.
    NoHostileThreatWorld,
    /// The companion subtlety: in every threat world the hostile holder
    /// also holds the vulnerable lead's suit, so follow-suit obligations
    /// block the proposed ruff.
    FollowSuitBlocksRuff,
    /// Hypothesis 5: the declared field, probed at an actual void-holder
    /// state, declines the ruff.
    FieldDoesNotRuffWhenVoid,
    /// Hypothesis 6: some threat world lets the hostile holder answer the
    /// high-trump lead with a non-called tile (unreachable under the
    /// current follow rules; the check guards against rules drift).
    ExtractionIncomplete,
    /// Hypothesis 7: losing the vulnerable trick does not make policy b's
    /// Boolean contract impossible (checked by a conservative exact
    /// sufficient condition: the vulnerable trick is worth at least its
    /// own count plus the trick point).
    LostVulnerableTrickNotFatal,
    /// Hypothesis 8: the no-hostile-trump cell's residual implication
    /// fails — the a-branch does not sweep at rules level (unreachable
    /// when hypotheses 1–6 hold; the check mirrors the verifier's
    /// authority).
    ResidualImplicationFails,
}

impl ExtractionDecline {
    /// The mechanical tag, for decline histograms.
    pub fn tag(self) -> &'static str {
        match self {
            ExtractionDecline::ShapeNotFocalLead => "ShapeNotFocalLead",
            ExtractionDecline::LeadNotHighestTrump => "LeadNotHighestTrump",
            ExtractionDecline::LeadNotVulnerableNontrump => "LeadNotVulnerableNontrump",
            ExtractionDecline::HostileTrumpsExceedOneRound => "HostileTrumpsExceedOneRound",
            ExtractionDecline::HostileSuitBeater => "HostileSuitBeater",
            ExtractionDecline::TwoTrickShapeRequired => "TwoTrickShapeRequired",
            ExtractionDecline::NoHostileThreatWorld => "NoHostileThreatWorld",
            ExtractionDecline::FollowSuitBlocksRuff => "FollowSuitBlocksRuff",
            ExtractionDecline::FieldDoesNotRuffWhenVoid => "FieldDoesNotRuffWhenVoid",
            ExtractionDecline::ExtractionIncomplete => "ExtractionIncomplete",
            ExtractionDecline::LostVulnerableTrickNotFatal => "LostVulnerableTrickNotFatal",
            ExtractionDecline::ResidualImplicationFails => "ResidualImplicationFails",
        }
    }

    pub const ALL: [ExtractionDecline; 12] = [
        ExtractionDecline::ShapeNotFocalLead,
        ExtractionDecline::LeadNotHighestTrump,
        ExtractionDecline::LeadNotVulnerableNontrump,
        ExtractionDecline::HostileTrumpsExceedOneRound,
        ExtractionDecline::HostileSuitBeater,
        ExtractionDecline::TwoTrickShapeRequired,
        ExtractionDecline::NoHostileThreatWorld,
        ExtractionDecline::FollowSuitBlocksRuff,
        ExtractionDecline::FieldDoesNotRuffWhenVoid,
        ExtractionDecline::ExtractionIncomplete,
        ExtractionDecline::LostVulnerableTrickNotFatal,
        ExtractionDecline::ResidualImplicationFails,
    ];
}

impl fmt::Display for ExtractionDecline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tag())
    }
}

/// The One-Round Trump-Extraction producer (response §2.5; TRIPLE-A5): the
/// first deliberately incomplete witness emitter. On accept it emits the
/// two-cell `NoHostileTrump` / `OneHostileTrump` witness for the general
/// verifier — it OWNS NO dominance semantics; on refusal it names the
/// failed hypothesis. Discovery may consult the exact fiber and probe the
/// declared field (discovery is allowed to be expensive); the emitted
/// witness is then checked by the cheap authority.
pub fn one_round_trump_extraction(
    root: &CanonicalRoot,
    position: &RootPosition,
    policy_a: &dyn SlicePolicy,
    policy_b: &dyn SlicePolicy,
    field: &FieldModel,
) -> Result<HazardExclusionWitness, ExtractionDecline> {
    let kernel = root.kernel();
    let viewer = kernel.viewer();
    let viewer_hand = kernel.viewer_hand();
    let pool = kernel.pool();
    let decl = position.decl;
    // Frame precondition: the focal seat is leading.
    if !position.trick_plays.is_empty() || position.leader != viewer {
        return Err(ExtractionDecline::ShapeNotFocalLead);
    }
    let record = root_record(position);
    let lead_a = policy_a.choose(decl, viewer_hand, viewer_hand, &record);
    let lead_b = policy_b.choose(decl, viewer_hand, viewer_hand, &record);
    // Hypothesis 1: a leads the globally highest remaining called tile.
    if !decl.is_called(lead_a) || !decl.threat(lead_a).intersection(pool).is_empty() {
        return Err(ExtractionDecline::LeadNotHighestTrump);
    }
    // Hypothesis 2: b leads a nontrump tile some remaining tile threatens.
    if decl.is_called(lead_b) || decl.threat(lead_b).intersection(pool).is_empty() {
        return Err(ExtractionDecline::LeadNotVulnerableNontrump);
    }
    // Hypotheses 3 and 4, over the EXACT fiber (depth-independent, so the
    // §2.7 three-trick specimen declines here, before any language check).
    let called = decl.called_set();
    let suit_beaters = decl.threat(lead_b).difference(called);
    let hostile_seats = [viewer.plus(1), viewer.plus(3)];
    for world in root.worlds() {
        let hands = world.hands();
        let hostile_trumps: usize = hostile_seats
            .iter()
            .map(|s| hands[s.index()].intersection(called).len())
            .sum();
        if hostile_trumps > 1 {
            return Err(ExtractionDecline::HostileTrumpsExceedOneRound);
        }
        if hostile_seats
            .iter()
            .any(|s| !hands[s.index()].intersection(suit_beaters).is_empty())
        {
            return Err(ExtractionDecline::HostileSuitBeater);
        }
    }
    // The v1 witness language's boundary.
    if viewer_hand.len() != 2 || pool.len() != 6 {
        return Err(ExtractionDecline::TwoTrickShapeRequired);
    }
    // Hypothesis 7 (cheap arithmetic, checked before any field probe):
    // losing the vulnerable trick kills b's Boolean contract.
    // Conservative exact sufficient condition: the lost trick is worth at
    // least the trick point plus the vulnerable tile's own count.
    let remaining_points = 2 + pool.union(viewer_hand).count_points();
    let decl_banked = position.banked[position.declaring_team.index()];
    let lost_trick_min = 1 + lead_b.count();
    let fatal = if viewer.team() == position.declaring_team {
        decl_banked + remaining_points - lost_trick_min < position.bid
    } else {
        decl_banked + lost_trick_min >= position.bid
    };
    if !fatal {
        return Err(ExtractionDecline::LostVulnerableTrickNotFatal);
    }
    // Hypotheses 5 and 6 over the threat worlds, with the companion
    // subtlety: probe the DECLARED field at the actual b-branch trick-one
    // states (preceding non-focal seats replayed under the same field).
    let led_b = decl.led_context(lead_b);
    let led_a = decl.led_context(lead_a);
    let mut threat_world_seen = false;
    let mut void_holder_seen = false;
    for world in root.worlds() {
        let hands = world.hands();
        let holder = hostile_seats
            .iter()
            .copied()
            .find(|s| !hands[s.index()].intersection(called).is_empty());
        let Some(holder) = holder else {
            continue;
        };
        threat_world_seen = true;
        // Hypothesis 6: the high-trump lead forces the holder's called
        // tile out.
        if !legal_plays(decl, hands[holder.index()], Some(led_a)).is_subset_of(called) {
            return Err(ExtractionDecline::ExtractionIncomplete);
        }
        if !hands[holder.index()]
            .intersection(decl.effective_incidence(led_b))
            .is_empty()
        {
            // Follow-suit blocks the ruff in this world; no field probe.
            continue;
        }
        void_holder_seen = true;
        // Replay the b-branch trick up to the holder under the declared
        // field, then probe the holder's choice.
        let mut exec_hands = hands;
        assert!(
            exec_hands[viewer.index()].remove(lead_b),
            "the b lead is held"
        );
        let mut plays = vec![lead_b];
        let mut history = vec![lead_b];
        let mut ruffed = None;
        for k in 1..4 {
            let seat = viewer.plus(k);
            let hand = exec_hands[seat.index()];
            let legal = legal_plays(decl, hand, Some(led_b));
            let probe = PublicRecord {
                leader: viewer,
                trick_plays: &plays,
                banked: position.banked,
                root: position,
                history: &history,
            };
            let tile = field.choose(decl, hand, legal, &probe);
            if seat == holder {
                ruffed = Some(decl.is_called(tile));
                break;
            }
            assert!(
                exec_hands[seat.index()].remove(tile),
                "a chosen tile is held"
            );
            plays.push(tile);
            history.push(tile);
        }
        if ruffed != Some(true) {
            return Err(ExtractionDecline::FieldDoesNotRuffWhenVoid);
        }
    }
    if !threat_world_seen {
        return Err(ExtractionDecline::NoHostileThreatWorld);
    }
    if !void_holder_seen {
        return Err(ExtractionDecline::FollowSuitBlocksRuff);
    }
    // Hypothesis 8: the no-hostile-trump cell's residual implication — the
    // a-branch sweeps at rules level (the same sweeps the verifier runs;
    // mirrored here only to predict refusal, never as a second authority).
    if trick_one_sweep(
        decl,
        viewer,
        viewer_hand,
        pool,
        WorldSelector::NoHostileTrump,
        lead_a,
    )
    .is_err()
        || trick_two_sweep(decl, viewer, viewer_hand, pool, lead_a).is_err()
    {
        return Err(ExtractionDecline::ResidualImplicationFails);
    }
    // Emit the §2.5 two-cell witness: one three-stage chain per selector.
    let chain = |selector: WorldSelector| -> [SymbolicPairedCell; 3] {
        [
            SymbolicPairedCell {
                tricks_done: 0,
                selector,
                claim_a: BranchClaim::Unconstrained,
                claim_b: BranchClaim::Unconstrained,
            },
            SymbolicPairedCell {
                tricks_done: 1,
                selector,
                claim_a: BranchClaim::ViewerSweptLeadsHostilesTrumpless,
                claim_b: BranchClaim::Unconstrained,
            },
            SymbolicPairedCell {
                tricks_done: 2,
                selector,
                claim_a: BranchClaim::TeamWonEveryTrick,
                claim_b: BranchClaim::Unconstrained,
            },
        ]
    };
    let [n0, n1, n2] = chain(WorldSelector::NoHostileTrump);
    let [o0, o1, o2] = chain(WorldSelector::OneHostileTrump);
    Ok(HazardExclusionWitness {
        root_id: root_identity(root, position),
        policy_a: policy_a.id().to_string(),
        policy_b: policy_b.id().to_string(),
        field_id: field.field_id(),
        lead_a,
        lead_b,
        cells: vec![n0, n1, n2, o0, o1, o2],
        initial_cover: vec![0, 3],
        successor_obligations: vec![
            SuccessorObligation {
                from: 0,
                to: vec![1],
            },
            SuccessorObligation {
                from: 1,
                to: vec![2],
            },
            SuccessorObligation {
                from: 3,
                to: vec![4],
            },
            SuccessorObligation {
                from: 4,
                to: vec![5],
            },
        ],
        terminal_implications: vec![2, 5],
    })
}
