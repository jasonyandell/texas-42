//! Gates for the anytime proof-state Phase 2 score profile [L2 thread]:
//! the §18 fixed-policy 43-bin recursion ([`viewer_score_profile`]) —
//! mass conservation and tail projection to the Slice D success mass
//! under both viewer parities (gate 1), the §3 tail-sum identity on
//! every computed profile (gate 2), contract reuse: one profile answers
//! every threshold, checked against independent re-runs whose decided
//! cutoffs differ per contract (gate 3), and entrywise parity with an
//! independent complete-world replay to true terminals (gate 4).
//!
//! Mathematical source: `walt/math/anytime_proof_state_score_v0.1.md`
//! §2–§4 (the exact score profile and the 42 signature), §18 (the
//! fixed-policy score recursion), §44 (contract reuse), adopted by
//! ruling APS-A2 (`walt/CENSUS-RULINGS.md`); the §20 envelope fence
//! (APS-A4) is respected by construction — every profile here is the
//! record of ONE policy.
//!
//! DECLARED TEST EPOCH: deterministic fields only — the trivial
//! `FixedPreference` fields and the σ0 Level0 { n0 = 2 } modeled mind
//! (the C1 declared cached field, fresh instances per route). Frozen
//! `verify_player` receipt roots: the six enumerable fibers (hands
//! 4/5/10/12 at trick 6, hands 3/8 at trick 5).

mod common;

use common::receipt;
use walt::kernel::{Kernel, World};
use walt::rules::receipt::Receipt;
use walt::rules::rules::legal_plays;
use walt::rules::{Domino, DominoSet, Seat, Trick};
use walt::solver::adaptive::{
    CanonicalRoot, FixedPreference, PublicRecord, RootPosition, SlicePolicy,
};
use walt::solver::factor_belief::{
    viewer_score_profile, viewer_success_mass, FactorBelief, RecursionStats, ScoreProfile,
    SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{DecisionMode, TieRule};

/// The six enumerable frozen receipt roots: (hand, trick, fiber).
const ENUM_ROOTS: [(usize, usize, u128); 6] = [
    (12, 6, 6),
    (10, 6, 19),
    (5, 6, 27),
    (4, 6, 90),
    (8, 5, 92),
    (3, 5, 200),
];

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

/// The σ0 field of the C1 epoch, fresh per call.
fn level0_field() -> FieldModel {
    FieldModel::new(FieldSpec {
        kind: FieldKind::Level0 { n0: 2 },
        construction: "level0-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    })
}

/// The test's own public-state replay — the same trick arithmetic as the
/// recursion-gate checker, built by hand here so gate 4 is independent
/// of the machinery it gates.
#[derive(Clone)]
struct Pub {
    leader: Seat,
    plays: Vec<Domino>,
    banked: [u32; 2],
    played: DominoSet,
    history: Vec<Domino>,
}

impl Pub {
    fn start(position: &RootPosition) -> Pub {
        assert!(
            position.trick_plays.is_empty(),
            "the frozen fixtures are trick-start roots"
        );
        Pub {
            leader: position.leader,
            plays: Vec::new(),
            banked: position.banked,
            played: DominoSet::EMPTY,
            history: Vec::new(),
        }
    }

    fn seat(&self) -> Seat {
        self.leader.plus(self.plays.len())
    }

    fn play(&mut self, position: &RootPosition, tile: Domino) {
        assert!(self.played.insert(tile), "a tile is played once");
        self.plays.push(tile);
        self.history.push(tile);
        if self.plays.len() == 4 {
            let doms: [Domino; 4] = core::array::from_fn(|i| self.plays[i]);
            let trick = Trick::new(self.leader, doms).expect("four distinct tiles");
            let winner = trick.winner(position.decl);
            self.banked[winner.team().index()] += trick.points();
            self.leader = winner;
            self.plays.clear();
        }
    }

    fn record<'a>(&'a self, position: &'a RootPosition) -> PublicRecord<'a> {
        PublicRecord {
            leader: self.leader,
            trick_plays: &self.plays,
            banked: self.banked,
            root: position,
            history: &self.history,
        }
    }
}

/// One policy's choice at the current public state, from a root hand.
fn choice_at(
    position: &RootPosition,
    exec: &Pub,
    root_hand: DominoSet,
    policy: &dyn SlicePolicy,
) -> Domino {
    let remaining = root_hand.difference(exec.played);
    let led = exec.plays.first().map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, remaining, led);
    assert!(!legal.is_empty(), "a seat to move holds a legal tile");
    let record = exec.record(position);
    let tile = policy.choose(position.decl, remaining, legal, &record);
    assert!(legal.contains(tile), "a policy chooses a legal tile");
    tile
}

/// Post-root plays to terminal at this root.
fn plays_to_terminal(root: &CanonicalRoot) -> usize {
    root.kernel().viewer_hand().len()
        + root
            .kernel()
            .hidden()
            .iter()
            .map(|h| h.capacity)
            .sum::<usize>()
}

/// Replay one complete world to a TRUE terminal (no decided truncation)
/// and return the declaring team's banked total — the §4 score.
fn replay_declaring_score(
    root: &CanonicalRoot,
    position: &RootPosition,
    world: &World,
    focal: &dyn SlicePolicy,
    field: &dyn SlicePolicy,
) -> u32 {
    let viewer = root.kernel().viewer();
    let viewer_hand = root.kernel().viewer_hand();
    let total = plays_to_terminal(root);
    let mut exec = Pub::start(position);
    while exec.history.len() < total {
        let seat = exec.seat();
        let (hand, policy): (DominoSet, &dyn SlicePolicy) = if seat == viewer {
            (viewer_hand, focal)
        } else {
            (world.hand(seat), field)
        };
        let tile = choice_at(position, &exec, hand, policy);
        exec.play(position, tile);
    }
    assert_eq!(
        exec.banked[0] + exec.banked[1],
        42,
        "the 42-point pool is fully banked at terminal"
    );
    exec.banked[position.declaring_team.index()]
}

/// The viewer-objective projection of a profile at a contract: the
/// declaring viewer's success mass is the tail, the setting viewer's is
/// the complement.
fn viewer_mass_at(profile: &ScoreProfile, position: &RootPosition, viewer: Seat, bid: u32) -> u128 {
    let tail = profile.tail(bid);
    if viewer.team() == position.declaring_team {
        tail
    } else {
        profile.total() - tail
    }
}

/// Gate 1 — mass conservation and tail projection: on every enumerable
/// root, under the trivial field and under σ0, for two frozen focal
/// policies, the profile's total is exactly the fiber and its
/// viewer-objective projection at the root's own bid equals the Slice D
/// success mass. The early decided cutoff never fires (the §18 caveat
/// made mechanical), tails are monotone, and the profile walk reaches at
/// least as many nodes as the truncating walk.
#[test]
fn profiles_conserve_mass_and_project_to_the_success_mass() {
    let r = receipt();
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let high = FixedPreference::highest_first("focal:highest-first");
    let oracle = SupportOracle;
    for (hand_id, trick_no, fiber) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let viewer = root.kernel().viewer();
        for focal in [&low, &high] {
            for use_level0 in [false, true] {
                let field_p: Box<dyn SlicePolicy> = if use_level0 {
                    Box::new(level0_field())
                } else {
                    Box::new(FixedPreference::lowest_first("field:lowest-first"))
                };
                let belief = FactorBelief::uniform_root(&root, &position, field_p.as_ref());
                let mut pstats = RecursionStats::default();
                let profile =
                    viewer_score_profile(&oracle, &belief, focal, field_p.as_ref(), &mut pstats);
                assert_eq!(
                    profile.total(),
                    fiber,
                    "a profile conserves the fiber mass (hand {hand_id} trick {trick_no})"
                );
                assert_eq!(
                    pstats.decided_early, 0,
                    "the profile walk has no early decided cutoff"
                );
                assert!(
                    pstats.decided_terminal >= 1,
                    "a finite profile walk reaches terminals"
                );
                for k in 1..=42u32 {
                    assert!(
                        profile.tail(k) <= profile.tail(k - 1),
                        "tails are monotone nonincreasing"
                    );
                }
                let field_m: Box<dyn SlicePolicy> = if use_level0 {
                    Box::new(level0_field())
                } else {
                    Box::new(FixedPreference::lowest_first("field:lowest-first"))
                };
                let belief_m = FactorBelief::uniform_root(&root, &position, field_m.as_ref());
                let mut mstats = RecursionStats::default();
                let mass =
                    viewer_success_mass(&oracle, &belief_m, focal, field_m.as_ref(), &mut mstats);
                assert_eq!(
                    viewer_mass_at(&profile, &position, viewer, position.bid),
                    mass,
                    "the tail projection equals the Slice D success mass \
                     (hand {hand_id} trick {trick_no})"
                );
                assert!(
                    pstats.focal_nodes + pstats.hidden_nodes
                        >= mstats.focal_nodes + mstats.hidden_nodes,
                    "walking past the decided cutoff never shrinks the tree"
                );
            }
        }
    }
}

/// Gate 2 — the §3 tail-sum identity on every computed profile:
/// `Σ_s s·H(s) = Σ_{k=1}^{42} T(k)` exactly, so the expected score is
/// the area under the score-tail curve, in integers.
#[test]
fn the_tail_sum_identity_holds_on_every_computed_profile() {
    let r = receipt();
    let focal = FixedPreference::lowest_first("focal:lowest-first");
    let oracle = SupportOracle;
    for (hand_id, trick_no, _) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = level0_field();
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        let mut stats = RecursionStats::default();
        let profile = viewer_score_profile(&oracle, &belief, &focal, &field, &mut stats);
        let area: u128 = (1..=42u32).map(|k| profile.tail(k)).sum();
        assert_eq!(
            profile.point_mass_sum(),
            area,
            "the tail-sum identity holds (hand {hand_id} trick {trick_no})"
        );
    }
}

/// Gate 3 — contract reuse (§44) holds exactly as far as the declared
/// semantics is bid-blind. Under bid-blind processes (the trivial field
/// and a `FixedPreference` focal), ONE profile answers every threshold:
/// independent success-mass re-runs at altered bids — whose decided
/// cutoffs fire at different depths per contract — match the profile's
/// projection at every threshold tried, including the degenerate
/// certain-make bid 0 and the 42 bid.
#[test]
fn one_profile_answers_every_contract_under_a_bid_blind_semantics() {
    let r = receipt();
    let focal = FixedPreference::lowest_first("focal:lowest-first");
    let oracle = SupportOracle;
    for (hand_id, trick_no, fiber) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let viewer = root.kernel().viewer();
        let field = FixedPreference::lowest_first("field:lowest-first");
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        let mut stats = RecursionStats::default();
        let profile = viewer_score_profile(&oracle, &belief, &focal, &field, &mut stats);
        for bid in [0u32, 1, 21, 30, 36, 42] {
            let mut repriced = position.clone();
            repriced.bid = bid;
            let belief_m = FactorBelief::uniform_root(&root, &repriced, &field);
            let mut mstats = RecursionStats::default();
            let mass = viewer_success_mass(&oracle, &belief_m, &focal, &field, &mut mstats);
            assert_eq!(
                viewer_mass_at(&profile, &repriced, viewer, bid),
                mass,
                "one profile answers contract {bid} (hand {hand_id} trick {trick_no})"
            );
        }
        assert_eq!(profile.tail(0), fiber, "the zero threshold is the fiber");
        assert_eq!(profile.tail(43), 0, "no score exceeds the 42-point pool");
    }
}

/// Gate 3b — the reuse boundary, as a frozen specimen: σ0 READS the bid
/// by construction (the modeled mind's settled/desperation branches,
/// `solver/mod.rs`, and the bid is canonized into its cache key), so
/// changing the contract changes the FIELD — a different semantics, not
/// a different projection of the same one (the §51 identity lesson: the
/// contract is a coordinate of this field's identity). At hand 10 trick
/// 6 the σ0 profile priced at the root's own bid projects 12 at
/// threshold 42, while the exact σ0 evaluation AT bid 42 is 9 — the
/// divergence is the point. Cross-contract answers under a bid-reading
/// field are new evaluations, never re-projections.
#[test]
fn a_bid_reading_field_voids_cross_contract_reuse() {
    let r = receipt();
    let focal = FixedPreference::lowest_first("focal:lowest-first");
    let oracle = SupportOracle;
    let (root, position) = root_at(&r, 10, 6);
    let viewer = root.kernel().viewer();
    let field = level0_field();
    let belief = FactorBelief::uniform_root(&root, &position, &field);
    let mut stats = RecursionStats::default();
    let profile = viewer_score_profile(&oracle, &belief, &focal, &field, &mut stats);
    let mut repriced = position.clone();
    repriced.bid = 42;
    let field_m = level0_field();
    let belief_m = FactorBelief::uniform_root(&root, &repriced, &field_m);
    let mut mstats = RecursionStats::default();
    let mass = viewer_success_mass(&oracle, &belief_m, &focal, &field_m, &mut mstats);
    let projected = viewer_mass_at(&profile, &repriced, viewer, 42);
    assert_eq!(projected, 12, "the frozen fixture's projected tail at 42");
    assert_eq!(mass, 9, "the frozen fixture's exact evaluation at bid 42");
    assert_ne!(
        projected, mass,
        "a bid-reading field makes re-pricing a re-run, not a projection"
    );
}

/// Gate 4 — complete-world parity: on every enumerable root, the
/// factorized profile equals, ENTRY BY ENTRY, the histogram of an
/// independent world-by-world replay to true terminals (the test's own
/// trick arithmetic), under the trivial field and under σ0.
#[test]
fn profiles_match_complete_world_replay() {
    let r = receipt();
    let focal = FixedPreference::lowest_first("focal:lowest-first");
    let oracle = SupportOracle;
    for (hand_id, trick_no, fiber) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        for use_level0 in [false, true] {
            let field_p: Box<dyn SlicePolicy> = if use_level0 {
                Box::new(level0_field())
            } else {
                Box::new(FixedPreference::lowest_first("field:lowest-first"))
            };
            let belief = FactorBelief::uniform_root(&root, &position, field_p.as_ref());
            let mut stats = RecursionStats::default();
            let profile =
                viewer_score_profile(&oracle, &belief, &focal, field_p.as_ref(), &mut stats);
            let field_w: Box<dyn SlicePolicy> = if use_level0 {
                Box::new(level0_field())
            } else {
                Box::new(FixedPreference::lowest_first("field:lowest-first"))
            };
            let mut hist = [0u128; 43];
            let mut count = 0u128;
            for world in root.worlds() {
                let s = replay_declaring_score(&root, &position, &world, &focal, field_w.as_ref());
                hist[s as usize] += 1;
                count += 1;
            }
            assert_eq!(count, fiber, "the enumeration covers the fiber");
            assert_eq!(
                profile.bins, hist,
                "the factorized profile equals the replay histogram \
                 (hand {hand_id} trick {trick_no}, level0 {use_level0})"
            );
        }
    }
}
