//! Slice UP1a — the lazy carry. Five gates.
//!
//! UC1 nothing read, nothing paid: on the lean rung — where no tier ever
//!     consults the posterior — every carried line ends the walk with its
//!     ledger at ZERO and nothing materialized, while the line itself was
//!     recorded; and the bill was real, not absent: materializing the
//!     same lines afterwards spends reads.
//! UC2 conservation: every consultation a seat's lineage ever makes is
//!     charged to one of that seat's decisions — the ledger total equals
//!     the sum of `carry_reads + mixture_reads` over the seat's decisions.
//! UC3 falsification is discovered at materialization, at the ply it
//!     happened: a lean walk that never reads its posterior never learns
//!     the library was falsified; the final materialization finds it at
//!     exactly the history an independent eager replay finds it at, with
//!     the same supported set.
//! UC4 materialization is idempotent and a consulting decision reads a
//!     CURRENT posterior: after `materialize_line` the line is current, a
//!     second call spends nothing, and every decision whose provenance
//!     says the posterior was consulted reports it materialized to the
//!     head of the line.
//! UC5 lazy ≡ eager on every answer: a driver that materializes every
//!     line after every ply (UP0's carry, reproduced from outside) and the
//!     lazy player produce identical actions, evidence, refusals and join
//!     readings at every decision, and identical posterior notes at every
//!     decision that consulted the posterior. Only the carry charge moves.
//!
//! EXPLORATORY tier throughout. Nothing here is a play-strength claim.

use std::rc::Rc;

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::replay::state_before_trick;
use walt::rules::{legal_plays, Context, ContextSet, Domino, DominoSet, Seat, Trick};
use walt::solver::adaptive::{driven_root, CanonicalRoot, DrivenState, RootPosition};
use walt::solver::factor_belief::SupportOracle;
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::model_belief::{BehaviorType, ModelBelief, PersistenceScope, SeatTypePrior};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::unified::{Decision, MoveBudget, ReceiptStore, TypeLibrary, UnifiedPlayer};

// ---------------------------------------------------------------------------
// The declared epoch, shared with UP0's gates and the probe.
// ---------------------------------------------------------------------------

fn field_spec_level0() -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level0 { n0: 2 },
        construction: "level0-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

fn field_spec_level1() -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level1 { n_outer: 2, n0: 2 },
        construction: "level1-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

fn registered_types() -> (Rc<BehaviorType>, Rc<BehaviorType>) {
    (
        Rc::new(BehaviorType::from_field(
            field_spec_level0(),
            PersistenceScope::PerHand,
        )),
        Rc::new(BehaviorType::from_field(
            field_spec_level1(),
            PersistenceScope::PerHand,
        )),
    )
}

fn library() -> TypeLibrary {
    let (f0, f1) = registered_types();
    TypeLibrary::new(vec![(f0, 1), (f1, 1)])
}

fn mixture_at(root: &CanonicalRoot, position: &RootPosition) -> ModelBelief {
    let (f0, f1) = registered_types();
    let priors: Vec<SeatTypePrior> = root
        .kernel()
        .hidden()
        .iter()
        .map(|slot| SeatTypePrior {
            seat: slot.seat,
            types: vec![(Rc::clone(&f0), 1), (Rc::clone(&f1), 1)],
        })
        .collect();
    ModelBelief::from_independent_prior(root, position, &priors)
}

fn receipt() -> Receipt {
    parse_file(&locate_verify_player().expect("the receipt above the workspace"))
        .expect("the receipt parses")
}

fn budget(label: &str, enum_cap: u128, mix_cap: u128, reads: u64, join: bool) -> MoveBudget {
    MoveBudget {
        label: label.to_string(),
        enumeration_fiber_cap: enum_cap,
        mixture_fiber_cap: mix_cap,
        mixture_read_cap: reads,
        regret_acceptance: BigRational::new(BigInt::from(1), BigInt::from(4)),
        join_reading: join,
    }
}

fn lean() -> MoveBudget {
    budget("lean", 32, 0, 0, false)
}

fn ample() -> MoveBudget {
    budget("ample", 40_000, 256, 4_000_000, true)
}

fn model_rung() -> MoveBudget {
    budget("model", 8, 256, 4_000_000, false)
}

/// MB0's six enumerable roots plus the smallest trick-4 root.
const CORPUS: [(usize, usize); 7] = [(12, 6), (10, 6), (5, 6), (4, 6), (8, 5), (3, 5), (8, 4)];

// ---------------------------------------------------------------------------
// The walk driver, with the carry discipline as a parameter.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
enum Carry {
    /// The player's own: record the line, materialize when read.
    Lazy,
    /// UP0's, reproduced from outside: materialize every line after
    /// every ply.
    Eager,
}

struct Play {
    seat: usize,
    state: OwnedState,
    decision: Decision,
    /// Whether ANY line reported a falsification right after this ply's
    /// `observe_play` (before any later materialization).
    falsified_known_after_ply: bool,
}

#[derive(Clone)]
struct OwnedState {
    bid: u32,
    declaring_team: walt::rules::Team,
    viewer_hand: DominoSet,
    leader: Seat,
    trick_plays: Vec<Domino>,
    banked: [u32; 2],
    prior_played: DominoSet,
    voids: [ContextSet; 4],
}

impl OwnedState {
    fn driven<'a>(&'a self, decl: walt::rules::Decl) -> DrivenState<'a> {
        DrivenState {
            decl,
            bid: self.bid,
            declaring_team: self.declaring_team,
            viewer_hand: self.viewer_hand,
            leader: self.leader,
            trick_plays: &self.trick_plays,
            banked: self.banked,
            prior_played: self.prior_played,
            voids: self.voids,
        }
    }
}

/// What one seat's line looked like at the end of the walk, read BEFORE
/// and AFTER a final materialization.
struct LineView {
    seat: usize,
    line_plays: usize,
    materialized_before: usize,
    ledger_before: u64,
    falsified_before: bool,
    /// After the final materialization.
    materialized_after: usize,
    ledger_after: u64,
    ledger_after_second: u64,
    is_current_after: bool,
    falsified_after: Option<(Vec<Domino>, usize, Domino, Vec<Domino>)>,
    history_after: Vec<Domino>,
    live_after: usize,
    total_after: u128,
}

fn walk(
    r: &Receipt,
    hand_id: usize,
    trick_no: usize,
    b: &MoveBudget,
    carry: Carry,
    max_plies: usize,
) -> (Vec<Play>, Vec<LineView>) {
    let hand = &r.hands[hand_id];
    let (start_hands, start_leader) =
        state_before_trick(hand, trick_no).expect("a valid receipt trick");
    let decl = hand.decl;
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec_level0());
    let mut player = UnifiedPlayer::new(&oracle, &field, library());
    player.seed_store(ReceiptStore::new());

    let mut hands = start_hands;
    let mut leader = start_leader;
    let mut banked = [0u32; 2];
    let mut prior_played = DominoSet::EMPTY;
    let mut voids = [ContextSet::EMPTY; 4];
    for t in hand.tricks.iter().take(trick_no - 1) {
        let doms: [Domino; 4] = core::array::from_fn(|i| t.plays[i].1);
        let trick = Trick::new(t.plays[0].0, doms).expect("four distinct tiles");
        banked[trick.winner(decl).team().index()] += trick.points();
        for d in doms {
            prior_played.insert(d);
        }
        let led = decl.led_context(doms[0]);
        for (k, d) in doms.iter().enumerate() {
            if !decl.follows(*d, led) {
                voids[t.plays[0].0.plus(k).index()].insert(led);
            }
        }
    }

    let mut trick_plays: Vec<Domino> = Vec::new();
    let mut plays: Vec<Play> = Vec::new();
    'outer: for _trick_index in trick_no..=7usize {
        for _ in 0..4usize {
            if plays.len() >= max_plies {
                break 'outer;
            }
            let seat = leader.plus(trick_plays.len());
            let owned = OwnedState {
                bid: hand.bid_points,
                declaring_team: hand.declaring_team,
                viewer_hand: hands[seat.index()],
                leader,
                trick_plays: trick_plays.clone(),
                banked,
                prior_played,
                voids,
            };
            let state = owned.driven(decl);
            let decision = player.decide(&state, b);
            let led: Option<Context> = trick_plays.first().map(|t| decl.led_context(*t));
            let legal = legal_plays(decl, hands[seat.index()], led);
            assert!(legal.contains(decision.action()), "a legal tile");
            player.observe_play(&state, decision.action());
            if carry == Carry::Eager {
                for s in player.lines() {
                    let _ = player.materialize_line(s);
                }
            }
            let falsified_known_after_ply = player
                .lines()
                .into_iter()
                .any(|s| player.line(s).is_some_and(|l| l.falsified().is_some()));
            if let Some(led) = led {
                if !decl.follows(decision.action(), led) {
                    voids[seat.index()].insert(led);
                }
            }
            assert!(hands[seat.index()].remove(decision.action()));
            trick_plays.push(decision.action());
            plays.push(Play {
                seat: seat.index(),
                state: owned,
                decision,
                falsified_known_after_ply,
            });
        }
        let doms: [Domino; 4] = core::array::from_fn(|i| trick_plays[i]);
        let trick = Trick::new(leader, doms).expect("four distinct tiles");
        banked[trick.winner(decl).team().index()] += trick.points();
        for d in doms {
            prior_played.insert(d);
        }
        leader = trick.winner(decl);
        trick_plays.clear();
    }

    let mut views: Vec<LineView> = Vec::new();
    for seat in player.lines() {
        let (line_plays, materialized_before, ledger_before, falsified_before) = {
            let line = player.line(seat).expect("an open line");
            (
                line.line_plays(),
                line.materialized(),
                line.ledger_total(),
                line.falsified().is_some(),
            )
        };
        let _ = player.materialize_line(seat);
        let (
            materialized_after,
            ledger_after,
            is_current_after,
            falsified_after,
            history,
            live,
            total,
        ) = {
            let line = player.line(seat).expect("an open line");
            (
                line.materialized(),
                line.ledger_total(),
                line.is_current(),
                line.falsified()
                    .map(|f| (f.history.clone(), f.seat, f.observed, f.supported.clone())),
                line.model().map_or(Vec::new(), |m| m.history().to_vec()),
                line.model().map_or(0, |m| m.profiles().len()),
                line.model().map_or(0, |m| m.weighted_total(&oracle)),
            )
        };
        let _ = player.materialize_line(seat);
        let ledger_after_second = player.line(seat).map_or(0, |l| l.ledger_total());
        views.push(LineView {
            seat: seat.index(),
            line_plays,
            materialized_before,
            ledger_before,
            falsified_before,
            materialized_after,
            ledger_after,
            ledger_after_second,
            is_current_after,
            falsified_after,
            history_after: history,
            live_after: live,
            total_after: total,
        });
    }
    (plays, views)
}

// ---------------------------------------------------------------------------
// UC1 — nothing read, nothing paid.
// ---------------------------------------------------------------------------

#[test]
fn uc1_a_posterior_nobody_reads_costs_nothing_to_carry() {
    let r = receipt();
    let mut recorded_lines = 0usize;
    let mut deferred_bills = 0usize;
    for (hand_id, trick_no) in CORPUS {
        let (plays, views) = walk(&r, hand_id, trick_no, &lean(), Carry::Lazy, usize::MAX);
        for p in &plays {
            let prov = p.decision.provenance();
            assert_eq!(
                prov.spend().carry_reads,
                0,
                "UC1: no decision on the lean rung charged a carry (h{hand_id}-t{trick_no})"
            );
            assert!(
                !prov.posterior().consulted,
                "UC1: the lean rung never consults the posterior"
            );
            assert_eq!(
                prov.posterior().materialized,
                0,
                "UC1: an unread posterior is never advanced (h{hand_id}-t{trick_no})"
            );
        }
        for v in &views {
            assert_eq!(
                v.ledger_before, 0,
                "UC1: seat {}'s lineage spent nothing across the whole walk \
                 (h{hand_id}-t{trick_no})",
                v.seat
            );
            assert_eq!(v.materialized_before, 0, "UC1: and folded nothing");
            assert!(
                !v.falsified_before,
                "UC1: a line nobody read cannot have discovered a falsification"
            );
            if v.line_plays > 0 {
                recorded_lines += 1;
            }
            // The bill is real and was deferred, not absent: bringing the
            // line to its head afterwards spends the classifications UP0
            // paid at every ply.
            if v.ledger_after > 0 {
                deferred_bills += 1;
            }
            assert!(
                v.materialized_after == v.line_plays || v.falsified_after.is_some(),
                "UC1: a final materialization reaches the head of the line unless the \
                 library was falsified on it"
            );
        }
    }
    assert!(
        recorded_lines >= 7,
        "UC1: lines were recorded across the corpus, got {recorded_lines}"
    );
    assert!(
        deferred_bills >= 7,
        "UC1: the deferred bill was real on at least seven lines, got {deferred_bills}"
    );
}

// ---------------------------------------------------------------------------
// UC2 — conservation: every read is charged to one decision.
// ---------------------------------------------------------------------------

#[test]
fn uc2_every_lineage_read_is_charged_to_a_decision() {
    let r = receipt();
    let mut charged_lines = 0usize;
    for b in [ample(), model_rung()] {
        for (hand_id, trick_no) in CORPUS {
            let (plays, views) = walk(&r, hand_id, trick_no, &b, Carry::Lazy, usize::MAX);
            for v in &views {
                let charged: u64 = plays
                    .iter()
                    .filter(|p| p.seat == v.seat)
                    .map(|p| {
                        let s = p.decision.provenance().spend();
                        s.carry_reads.saturating_add(s.mixture_reads)
                    })
                    .sum();
                assert_eq!(
                    charged, v.ledger_before,
                    "UC2: seat {}'s ledger total equals the sum of its decisions' carry + \
                     mixture charges ({} h{hand_id}-t{trick_no})",
                    v.seat, b.label
                );
                if charged > 0 {
                    charged_lines += 1;
                }
            }
        }
    }
    assert!(
        charged_lines >= 10,
        "UC2: the law was checked on lines that actually spent, got {charged_lines}"
    );
}

// ---------------------------------------------------------------------------
// UC3 — falsification is discovered at materialization, at its own ply.
// ---------------------------------------------------------------------------

#[test]
fn uc3_a_falsification_is_found_at_materialization_where_an_eager_replay_finds_it() {
    let r = receipt();
    let oracle = SupportOracle;
    let mut discovered = 0usize;
    // h3-t5 and h8-t4: the roots UP0's transcript recorded falsifications
    // at. Lean rung — nothing is read during the walk.
    for (hand_id, trick_no) in [(3usize, 5usize), (8, 4)] {
        let (plays, views) = walk(&r, hand_id, trick_no, &lean(), Carry::Lazy, usize::MAX);
        assert!(
            plays.iter().all(|p| !p.falsified_known_after_ply),
            "UC3: a lean walk that never reads its posterior never learns of a \
             falsification during play (h{hand_id}-t{trick_no})"
        );
        let decl = r.hands[hand_id].decl;
        for v in &views {
            let Some((history, seat, observed, supported)) = &v.falsified_after else {
                continue;
            };
            discovered += 1;
            // The independent eager replay from this seat's root.
            let first = plays
                .iter()
                .position(|p| p.seat == v.seat)
                .expect("the seat decided at least once");
            let driven = plays[first].state.driven(decl);
            let (root, position) = driven_root(&driven).expect("a lawful driven root");
            let mut replay = mixture_at(&root, &position);
            let mut found: Option<(Vec<Domino>, usize, Domino, Vec<Domino>)> = None;
            for p in plays.iter().skip(first) {
                let tile = p.decision.action();
                if p.seat == v.seat {
                    replay = replay.focal_play(tile);
                    continue;
                }
                let table = replay.branch_masses(&oracle);
                if !table.iter().any(|(t, _)| *t == tile) {
                    found = Some((
                        replay.history().to_vec(),
                        p.seat,
                        tile,
                        table.into_iter().map(|(t, _)| t).collect(),
                    ));
                    break;
                }
                replay = replay.observe(&oracle, tile);
            }
            let found = found.expect("UC3: the eager replay is falsified too");
            assert_eq!(
                (history, *seat, *observed, supported),
                (&found.0, found.1, found.2, &found.3),
                "UC3: the lazy discovery and the eager replay agree on the history, the \
                 seat, the tile and the supported set (h{hand_id}-t{trick_no}, seat {})",
                v.seat
            );
            assert_eq!(
                v.materialized_after,
                history.len() + 1,
                "UC3: the line was materialized exactly up to and including the \
                 falsifying play"
            );
        }
    }
    assert!(
        discovered >= 2,
        "UC3: at least two falsifications were discovered on the two roots, got {discovered}"
    );
}

// ---------------------------------------------------------------------------
// UC4 — idempotence, and a consulted posterior is current.
// ---------------------------------------------------------------------------

#[test]
fn uc4_materialization_is_idempotent_and_a_consulted_posterior_is_current() {
    let r = receipt();
    let mut consulted = 0usize;
    for b in [ample(), model_rung()] {
        for (hand_id, trick_no) in CORPUS {
            let (plays, views) = walk(&r, hand_id, trick_no, &b, Carry::Lazy, usize::MAX);
            for v in &views {
                assert!(
                    v.is_current_after || v.falsified_after.is_some(),
                    "UC4: after materialize_line the line is current, or retired"
                );
                assert_eq!(
                    v.ledger_after, v.ledger_after_second,
                    "UC4: a second materialization spends nothing (seat {}, {} \
                     h{hand_id}-t{trick_no})",
                    v.seat, b.label
                );
                if v.falsified_after.is_none() {
                    assert_eq!(v.materialized_after, v.line_plays);
                    assert_eq!(
                        v.history_after.len(),
                        v.line_plays,
                        "UC4: the materialized history IS the line"
                    );
                    assert!(v.live_after > 0 && v.total_after > 0);
                }
            }
            for p in &plays {
                let note = p.decision.provenance().posterior();
                if note.consulted {
                    consulted += 1;
                    assert!(
                        note.materialized == note.line_plays || note.falsified.is_some(),
                        "UC4: a decision that consulted the posterior read it at the head \
                         of the line ({} h{hand_id}-t{trick_no})",
                        b.label
                    );
                }
            }
        }
    }
    assert!(
        consulted >= 10,
        "UC4: the currency law was checked on real consultations, got {consulted}"
    );
}

// ---------------------------------------------------------------------------
// UC5 — lazy ≡ eager on every answer.
// ---------------------------------------------------------------------------

#[test]
fn uc5_the_lazy_carry_changes_no_answer_the_eager_carry_gave() {
    let r = receipt();
    let mut compared = 0usize;
    let mut consulted_compared = 0usize;
    let mut carry_moved = 0usize;
    for b in [lean(), ample(), model_rung()] {
        for (hand_id, trick_no) in CORPUS {
            let (lazy, _) = walk(&r, hand_id, trick_no, &b, Carry::Lazy, usize::MAX);
            let (eager, _) = walk(&r, hand_id, trick_no, &b, Carry::Eager, usize::MAX);
            assert_eq!(lazy.len(), eager.len());
            for (x, y) in lazy.iter().zip(eager.iter()) {
                compared += 1;
                let (px, py) = (x.decision.provenance(), y.decision.provenance());
                assert_eq!(
                    x.decision.action(),
                    y.decision.action(),
                    "UC5: same action ({} h{hand_id}-t{trick_no} seat {})",
                    b.label,
                    x.seat
                );
                assert_eq!(px.evidence(), py.evidence(), "UC5: same evidence");
                assert_eq!(px.refusals(), py.refusals(), "UC5: same refusals");
                assert_eq!(px.authority(), py.authority(), "UC5: same authority");
                assert_eq!(px.frame(), py.frame(), "UC5: same frame");
                assert_eq!(
                    px.posterior().join,
                    py.posterior().join,
                    "UC5: same join reading"
                );
                // Every charge but the carry is identical; the carry moves
                // from outside `decide` (eager, unmeasured by the decision)
                // to inside it (lazy, charged to the reader).
                let (sx, sy) = (px.spend(), py.spend());
                assert_eq!(sx.enumeration_reads, sy.enumeration_reads);
                assert_eq!(sx.mixture_reads, sy.mixture_reads);
                assert_eq!(sx.field_reads, sy.field_reads);
                assert_eq!(
                    sy.carry_reads, 0,
                    "UC5: an eagerly carried line charges no carry"
                );
                if sx.carry_reads > 0 {
                    carry_moved += 1;
                }
                if px.posterior().consulted {
                    consulted_compared += 1;
                    assert!(py.posterior().consulted);
                    // The note is the posterior's own state, and at a
                    // consulting decision both carries read the same one.
                    let (nx, ny) = (px.posterior(), py.posterior());
                    assert_eq!(nx.live_profiles, ny.live_profiles);
                    assert_eq!(nx.observations, ny.observations);
                    assert_eq!(nx.focal_plays, ny.focal_plays);
                    assert_eq!(nx.falsified, ny.falsified);
                    assert_eq!(nx.line_plays, ny.line_plays);
                    assert_eq!(nx.materialized, ny.materialized);
                }
            }
        }
    }
    assert!(compared >= 3 * 7 * 4, "UC5: a real sweep, got {compared}");
    assert!(
        consulted_compared >= 10,
        "UC5: consulting decisions were compared, got {consulted_compared}"
    );
    assert!(
        carry_moved >= 5,
        "UC5: the carry charge actually landed on lazy consulting decisions, got \
         {carry_moved}"
    );
}
