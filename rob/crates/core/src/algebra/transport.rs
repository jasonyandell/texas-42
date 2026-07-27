//! Pip transports, scored `2 <-> 3` machinery, and the unscored mechanics
//! classification.
//!
//! Implements Math §3.9 (ALG-17/18/19) and rec Math §3.10 (ALG-22/23/24),
//! under the standing guardrail that the `2 <-> 3` transport is a scoped
//! game-order isomorphism between declaration layers 2 and 3 only — never a
//! global symmetry or a literal numeric-rank isomorphism.

use crate::algebra::suits::{LedSuit, CONTEXT_COUNT};
use crate::algebra::{algebra_for, DeclarationAlgebra};
use crate::declaration::Declaration;
use crate::domino::{
    all_ids, domino_from_id, domino_id, Domino, DominoId, DominoSet, DOMINO_COUNT,
};
use crate::pip::{Pip, PipPermutation, PIPS};

/// Apply a pip permutation to a domino endpoint-wise (Math §3.9).
pub fn transport_domino(perm: &PipPermutation, id: DominoId) -> DominoId {
    let d = domino_from_id(id);
    domino_id(Domino::new(perm.apply(d.high()), perm.apply(d.low())))
}

/// Apply a pip permutation to a suit context, fixing the called label 7
/// (Math §3.9 `σ̂(q)`).
pub fn transport_context(perm: &PipPermutation, q: LedSuit) -> LedSuit {
    match q {
        LedSuit::Natural(p) => LedSuit::Natural(perm.apply(p)),
        LedSuit::Called => LedSuit::Called,
    }
}

/// Transport of the declaration index: `p -> σ(p)`, fixing DT and NT
/// (Math §3.9).
pub fn transport_declaration(perm: &PipPermutation, declaration: Declaration) -> Declaration {
    match declaration {
        Declaration::PipTrump(p) => Declaration::PipTrump(perm.apply(p)),
        other => other,
    }
}

/// The ordered pip-trump transport `f_{t,u}` (rec Math §3.10; ALG-22):
/// sends the source trump pip to the target trump pip and maps the
/// complements order-preservingly.
pub struct PipTrumpTransport {
    source: Pip,
    target: Pip,
    map: PipPermutation,
}

/// Construct `f_{t,u}` for source trump `t` and target trump `u`
/// (rec Math §3.10; rec Exec §5 `PipTrumpTransport`).
pub fn pip_trump_transport(source: Pip, target: Pip) -> PipTrumpTransport {
    let complement_source: Vec<Pip> = PIPS.iter().copied().filter(|&p| p != source).collect();
    let complement_target: Vec<Pip> = PIPS.iter().copied().filter(|&p| p != target).collect();
    let mut image = PIPS;
    image[source.value() as usize] = target;
    for (s, t) in complement_source.iter().zip(complement_target.iter()) {
        image[s.value() as usize] = *t;
    }
    let map = PipPermutation::new(image)
        .expect("f_{t,u} is a bijection by construction (rec Math §3.10)");
    PipTrumpTransport {
        source,
        target,
        map,
    }
}

impl PipTrumpTransport {
    /// Source trump pip `t`.
    pub fn source(&self) -> Pip {
        self.source
    }

    /// Target trump pip `u`.
    pub fn target(&self) -> Pip {
        self.target
    }

    /// The underlying pip map `f_{t,u}` (rec Exec §5 `pipMap`).
    pub fn pip_map(&self) -> &PipPermutation {
        &self.map
    }

    /// Endpoint-wise domino transport (rec Exec §5 `dominoMap`).
    pub fn domino_map(&self, id: DominoId) -> DominoId {
        transport_domino(&self.map, id)
    }

    /// Context transport fixing the called label (rec Exec §5 `contextMap`).
    pub fn context_map(&self, q: LedSuit) -> LedSuit {
        transport_context(&self.map, q)
    }
}

/// The unscored mechanics structure `M_δ` (rec Math §3.10; ALG-22):
/// called/powered membership, effective-suit incidence, led context, follow,
/// and the strict contextual comparison order — count decoration and literal
/// numeric rank labels are omitted; only order is retained.
///
/// Distinct from [`ScoredMechanics`] by INV-10: no code conflates the two.
pub struct UnscoredMechanics {
    called: DominoSet,
    powered: DominoSet,
    follow: [[bool; CONTEXT_COUNT]; DOMINO_COUNT],
    led_suit: [LedSuit; DOMINO_COUNT],
    /// `strict[q][a][b]` iff `τ(a, q) < τ(b, q)`.
    strict: Box<[[[bool; DOMINO_COUNT]; DOMINO_COUNT]; CONTEXT_COUNT]>,
}

impl UnscoredMechanics {
    /// Extract the unscored relation surface of one algebra (rec Math §3.10).
    pub fn of(algebra: &DeclarationAlgebra) -> UnscoredMechanics {
        let mut follow = [[false; CONTEXT_COUNT]; DOMINO_COUNT];
        let mut strict = Box::new([[[false; DOMINO_COUNT]; DOMINO_COUNT]; CONTEXT_COUNT]);
        for id in all_ids() {
            for q in LedSuit::all() {
                follow[id.index()][q.context_index()] = algebra.follows(id, q);
            }
        }
        for q in LedSuit::all() {
            for a in all_ids() {
                let ka = algebra.trick_key(a, q);
                for b in all_ids() {
                    strict[q.context_index()][a.index()][b.index()] = ka < algebra.trick_key(b, q);
                }
            }
        }
        UnscoredMechanics {
            called: *algebra.called(),
            powered: *algebra.powered(),
            follow,
            led_suit: core::array::from_fn(|i| {
                algebra.led_suit(DominoId::from_index(i).expect("index < 28"))
            }),
            strict,
        }
    }

    /// Called membership.
    pub fn called(&self) -> &DominoSet {
        &self.called
    }

    /// Powered membership.
    pub fn powered(&self) -> &DominoSet {
        &self.powered
    }

    /// Follow / effective-incidence relation.
    pub fn follows(&self, id: DominoId, q: LedSuit) -> bool {
        self.follow[id.index()][q.context_index()]
    }

    /// Led context.
    pub fn led_suit(&self, id: DominoId) -> LedSuit {
        self.led_suit[id.index()]
    }

    /// Strict contextual comparison `τ(a, q) < τ(b, q)`.
    pub fn strictly_below(&self, q: LedSuit, a: DominoId, b: DominoId) -> bool {
        self.strict[q.context_index()][a.index()][b.index()]
    }
}

/// The scored mechanics structure: the unscored surface plus the count
/// decoration `c` (Math §3.7 reduct `G_δ`). Distinct from
/// [`UnscoredMechanics`] by INV-10.
pub struct ScoredMechanics {
    unscored: UnscoredMechanics,
    count: [u8; DOMINO_COUNT],
}

impl ScoredMechanics {
    /// Extract the scored (count-decorated) surface of one algebra.
    pub fn of(algebra: &DeclarationAlgebra) -> ScoredMechanics {
        ScoredMechanics {
            unscored: UnscoredMechanics::of(algebra),
            count: core::array::from_fn(|i| {
                domino_from_id(DominoId::from_index(i).expect("index < 28")).count_points()
            }),
        }
    }

    /// The unscored relation surface.
    pub fn unscored(&self) -> &UnscoredMechanics {
        &self.unscored
    }

    /// Count label of one domino.
    pub fn count(&self, id: DominoId) -> u8 {
        self.count[id.index()]
    }
}

/// The three unscored declaration-mechanics classes (rec Math §3.10;
/// ALG-23/24).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UnscoredMechanicsClass {
    /// All seven pip trumps (one isomorphism class).
    PipTrumpClass,
    /// Doubles-trump.
    DoublesTrumpClass,
    /// No-trump.
    NoTrumpClass,
}

/// Structural class signature `(|π_δ|, #{unpowered tiles with exactly one
/// effective suit})` computed from the implemented relation surface
/// (rec Math §3.10 classification invariant).
pub fn mechanics_signature(declaration: Declaration) -> (usize, usize) {
    let algebra = algebra_for(declaration);
    let powered_size = algebra.powered().len();
    let one_suit_unpowered = all_ids()
        .filter(|&id| !algebra.powered().contains(id) && algebra.effective_suits(id).len() == 1)
        .count();
    (powered_size, one_suit_unpowered)
}

/// Classify one declaration by its structural signature
/// (rec ALG-23/24: `(7,6)` pip trump, `(7,0)` doubles-trump, `(0,7)`
/// no-trump).
pub fn unscored_mechanics_class(declaration: Declaration) -> UnscoredMechanicsClass {
    match mechanics_signature(declaration) {
        (7, 6) => UnscoredMechanicsClass::PipTrumpClass,
        (7, 0) => UnscoredMechanicsClass::DoublesTrumpClass,
        (0, 7) => UnscoredMechanicsClass::NoTrumpClass,
        other => panic!("InvariantViolation: unclassified mechanics signature {other:?}"),
    }
}

/// Explicit unscored-transport mismatch report (Exec §1.4: fail explicitly).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TransportMismatch {
    /// The endpoint map is not a bijection or fails round-trip.
    NotBijective,
    /// Called or powered membership does not commute.
    CalledPowered,
    /// Effective incidence / follow does not commute.
    Follow,
    /// Led context does not commute.
    LedContext,
    /// A pairwise contextual order comparison does not commute.
    Order,
}

/// Verify one ordered pip-trump transport on the count-blind relation
/// surface (rec Math §3.10; ALG-22): called/powered, incidence, led context,
/// follow, every pairwise contextual order comparison, and round-trip
/// bijectivity. Returns the number of pairwise order comparisons performed.
pub fn check_unscored_transport(t: &PipTrumpTransport) -> Result<u64, TransportMismatch> {
    let src = UnscoredMechanics::of(&algebra_for(Declaration::PipTrump(t.source())));
    let tgt = UnscoredMechanics::of(&algebra_for(Declaration::PipTrump(t.target())));

    // Round-trip bijectivity: the reverse transport inverts the domino map.
    let back = pip_trump_transport(t.target(), t.source());
    let mut image_seen = [false; DOMINO_COUNT];
    for d in all_ids() {
        let td = t.domino_map(d);
        if image_seen[td.index()] || back.domino_map(td) != d {
            return Err(TransportMismatch::NotBijective);
        }
        image_seen[td.index()] = true;
    }

    for d in all_ids() {
        let td = t.domino_map(d);
        if src.called().contains(d) != tgt.called().contains(td)
            || src.powered().contains(d) != tgt.powered().contains(td)
        {
            return Err(TransportMismatch::CalledPowered);
        }
        if tgt.led_suit(td) != t.context_map(src.led_suit(d)) {
            return Err(TransportMismatch::LedContext);
        }
        for q in LedSuit::all() {
            if src.follows(d, q) != tgt.follows(td, t.context_map(q)) {
                return Err(TransportMismatch::Follow);
            }
        }
    }

    let mut comparisons = 0u64;
    for q in LedSuit::all() {
        let tq = t.context_map(q);
        for a in all_ids() {
            let ta = t.domino_map(a);
            for b in all_ids() {
                comparisons += 1;
                if src.strictly_below(q, a, b) != tgt.strictly_below(tq, ta, t.domino_map(b)) {
                    return Err(TransportMismatch::Order);
                }
            }
        }
    }
    Ok(comparisons)
}

/// Whether a pip permutation preserves every count label (Math §3.9; ALG-17).
pub fn preserves_count_labels(perm: &PipPermutation) -> bool {
    all_ids().all(|id| {
        domino_from_id(transport_domino(perm, id)).count_points()
            == domino_from_id(id).count_points()
    })
}

/// Whether a pip permutation is a game-order isomorphism from declaration
/// layer `from` onto its transported layer — the full `G_δ` reduct including
/// count labels (Math §3.7, §3.9; ALG-18/19). Order is transported, never
/// literal numeric rank labels.
pub fn is_scored_game_order_transport(perm: &PipPermutation, from: Declaration) -> bool {
    let to = transport_declaration(perm, from);
    let src = ScoredMechanics::of(&algebra_for(from));
    let tgt = ScoredMechanics::of(&algebra_for(to));

    for d in all_ids() {
        let td = transport_domino(perm, d);
        if src.count(d) != tgt.count(td) {
            return false;
        }
        if src.unscored().called().contains(d) != tgt.unscored().called().contains(td)
            || src.unscored().powered().contains(d) != tgt.unscored().powered().contains(td)
        {
            return false;
        }
        if tgt.unscored().led_suit(td) != transport_context(perm, src.unscored().led_suit(d)) {
            return false;
        }
        for q in LedSuit::all() {
            if src.unscored().follows(d, q)
                != tgt.unscored().follows(td, transport_context(perm, q))
            {
                return false;
            }
        }
    }
    for q in LedSuit::all() {
        let tq = transport_context(perm, q);
        for a in all_ids() {
            let ta = transport_domino(perm, a);
            for b in all_ids() {
                if src.unscored().strictly_below(q, a, b)
                    != tgt
                        .unscored()
                        .strictly_below(tq, ta, transport_domino(perm, b))
                {
                    return false;
                }
            }
        }
    }
    true
}
