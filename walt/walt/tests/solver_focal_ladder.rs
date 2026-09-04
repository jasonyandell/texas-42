//! Slice FH2 — the focal-horizon ladder: budget honesty, interruption
//! and resume, proof-state facts, exact suffix reuse. The parent's gate
//! FH7 (`walt/math/focal_horizon_sandwich_v0.1.md` §23, §19, §25) as
//! narrowed by the companion's P8/P11/P12 and rulings FH-A3, FH-A9,
//! FH-A11 with Proposition FH-int (`walt/CENSUS-RULINGS.md`); sized to
//! laws per CLAUDE.md — one coordinate per law plus a PINNED witness,
//! expensive values in one `LazyLock` fixture.
//!
//! LP   ladder parity with FH1: a fresh ladder walked directly at `k`
//!      equals `focal_horizon` at `k` on every derived view INCLUDING the
//!      policy id; the sequential ladder (k = 0, 1, 2) equals it on every
//!      value view at every k — h8-t4 and h3-t4, receipt contract, σ0.
//! FH7  budget honesty at h8-t4 (the parent's five bullets): a ceiling
//!      too small to finish k = 1 after k = 0 completed drops no root
//!      child (residual frontier ∪ completed children = every child);
//!      unfinished children carry the k = 0 facts, equal to the uncapped
//!      k = 0 run's; every action's interval contains the independent
//!      `Q_a`; the boundary names reads spent, ceiling and the stopping
//!      node; resume + completion ≡ uninterrupted k = 1 on every derived
//!      view and on the fact set (bytewise render); the spend as a sum.
//! FH7b monotone under interruption: across a pinned ceiling schedule no
//!      lower falls, no upper rises (absent never returns), `Γ` never
//!      rises, survivors only shrink — at every root action.
//! FH7c the placeholder is not a fact: a k = 0 pass interrupted before
//!      any upper exists yields `Unresolved` with NO action interval and
//!      NO regret, at h4-t6 and at h8-t4; the uncapped pass completes;
//!      the contrast: a root decided after every action (h10-t6) holds
//!      the §5 arithmetic as a fact at zero reads.
//! PS1  facts install and close: zero rejections into `ProofState::open`;
//!      closure survivors = the ladder's; the executable-bar witness =
//!      `L_exec`; closure's certified regret = `Γ_k`; a second produce at
//!      k + 1 only tightens.
//! PS2  executability is honest: no upper is executable; every executable
//!      lower's stored policy re-prices to its value through
//!      `viewer_success_mass`, including the RETAINED k = 0 lower of the
//!      interrupted pass (authority `k=0` beside the others' `k=1`).
//! SR1  suffix reuse is invisible in value: memo on vs off — identical
//!      intervals, survivors, verdicts and choice tables at h8-t4 and
//!      h3-t4 over k = 0..2; hits > 0 with the first hit PINNED at a
//!      named node; the memo saves reads.
//! SR2  the identity is the full belief: the contract-30 memo consulted
//!      by a contract-36 ladder (`with_contract`) hits zero times over
//!      positive lookups; a belief differing in factors alone is a miss.
//! FH-D determinism of an interrupted run (fresh vs the fixture's).
//! FH-C  the in-pass fiber-cap refusal (FH-A3, §41(7); FH4-AUDIT N3): an
//!      h8-t4 k = 0 pass at cap 40 is `Interrupted` with `unaffordable`
//!      non-empty and `stopping_node == None`; every refused node is a
//!      viewer node of positive mass above the cap with no fact under it;
//!      the enclosing root children are unfinished with placeholders (no
//!      interval, no regret) while the other root children completed with
//!      the uncapped run's k = 0 facts; the uncapped pass then completes.
//!
//! EXPLORATORY tier throughout.

#[path = "common/fixture.rs"]
mod fixture;

use std::collections::HashMap;
use std::sync::{Arc, LazyLock};

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::Domino;
use walt::solver::adaptive::{root_identity, CanonicalRoot, RootPosition};
use walt::solver::factor_belief::{
    response_success_mass, viewer_success_mass, ExactCoverOracle, FactorBelief, RecursionStats,
    ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::focal_horizon::{focal_horizon, FocalHorizonResult, FocalSpec, FocalVerdict};
use walt::solver::focal_ladder::{
    FocalHorizonProducer, FocalLadder, LadderContext, LadderView, Outcome, ResidualCause,
    SuffixMemo, WorkBudget,
};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::{BoundSide, Fact, ProofState, SemanticsIdentity, StateResult};

fn field_spec() -> FieldSpec {
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

fn receipt() -> Receipt {
    parse_file(&locate_verify_player().expect("the receipt above the workspace"))
        .expect("the receipt parses")
}

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn with_contract(position: &RootPosition, bid: u32) -> RootPosition {
    let mut out = position.clone();
    out.bid = bid;
    out
}

const AMPLE_CAP: u128 = 40_000;
const INF: u64 = u64::MAX;

/// The FH-C fiber cap at h8-t4 k = 0: two root children's frontiers fit
/// under it, two hold a node above it (found by the probe; not a law).
const H8_TINY_CAP: u128 = 40;

/// The FH7 interruption ceiling at h8-t4 k = 1 (the uncapped k = 1 pass
/// after k = 0 costs ~0.36M reads with the memo off; this stops inside
/// the last root child).
const H8_K1_CEILING: u64 = 250_000;

/// The FH7b pinned ceiling schedule at h8-t4.
const H8_SCHEDULE: [(usize, u64); 8] = [
    (0, 50_000),
    (0, 120_000),
    (0, INF),
    (1, 100_000),
    (1, 250_000),
    (1, INF),
    (2, 60_000),
    (2, INF),
];

fn budget(ceiling: u64) -> WorkBudget {
    WorkBudget {
        read_ceiling: ceiling,
        node_fiber_cap: AMPLE_CAP,
    }
}

/// An owned evaluation frame: everything a `LadderContext` borrows.
struct Frame {
    root: CanonicalRoot,
    position: RootPosition,
    oracle: SupportOracle,
    field: FieldModel,
}

impl Frame {
    fn new(r: &Receipt, hand_id: usize, trick_no: usize, contract: Option<u32>) -> Frame {
        let (root, position) = root_at(r, hand_id, trick_no);
        let position = match contract {
            Some(c) => with_contract(&position, c),
            None => position,
        };
        Frame {
            root,
            position,
            oracle: SupportOracle,
            field: FieldModel::new(field_spec()),
        }
    }

    fn ctx(&self) -> LadderContext<'_> {
        LadderContext {
            oracle: &self.oracle,
            root: &self.root,
            position: &self.position,
            lower_tail: &self.field,
            field: &self.field,
        }
    }

    fn belief(&self) -> FactorBelief {
        FactorBelief::uniform_root(&self.root, &self.position, &self.field)
    }

    /// The belief at a post-root history, rebuilt by the same operations
    /// the ladder uses (focal plays at viewer nodes, conditioning at
    /// hidden ones).
    fn belief_at(&self, history: &[Domino]) -> FactorBelief {
        let viewer = self.root.kernel().viewer();
        let mut b = self.belief();
        for t in history {
            b = if b.seat_to_move() == viewer {
                b.focal_play(*t)
            } else {
                self.oracle.condition(&b, *t, &self.field)
            };
        }
        b
    }
}

/// One ladder walked through a schedule; the view after every step and
/// the outcomes.
struct Walk {
    ladder: FocalLadder,
    outcomes: Vec<Outcome>,
}

fn walk(frame: &Frame, schedule: &[(usize, u64)], memo: Option<&mut SuffixMemo>) -> Walk {
    let ctx = frame.ctx();
    let mut ladder = FocalLadder::open(&ctx);
    let mut outcomes = Vec::new();
    let mut memo = memo;
    for (k, ceiling) in schedule {
        outcomes.push(ladder.advance(&ctx, *k, &budget(*ceiling), memo.as_deref_mut()));
    }
    Walk { ladder, outcomes }
}

type RootKey = (usize, usize);

struct Fixture {
    /// FH1's engine per (root, k) under the ample cap, σ0 tail.
    fh1: HashMap<(usize, usize, usize), Arc<FocalHorizonResult>>,
    /// Exact `Q_a` per root.
    exact_q: HashMap<RootKey, Vec<(Domino, u128)>>,
    /// The sequential ladder k = 0, 1, 2 with the memo OFF (the reference).
    seq_off: HashMap<RootKey, Walk>,
    /// The same with the memo ON, and its memo.
    seq_on: HashMap<RootKey, (Walk, SuffixMemo)>,
    /// FH7: h8-t4 k = 0 uncapped, then k = 1 at the small ceiling
    /// (memo off), then the resume — the ladder AFTER the interruption
    /// and AFTER the resume.
    h8_interrupted: Walk,
    h8_resumed: Walk,
    /// FH7b: the pinned ceiling schedule at h8-t4, memo on.
    h8_schedule: Walk,
    /// FH-C: a fresh h8-t4 k = 0 pass at the tiny cap, memo off.
    h8_capped: Walk,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Job {
    Fh1(usize, usize, usize),
    Q(usize, usize),
    SeqOff(usize, usize),
    SeqOn(usize, usize),
    Interrupted,
    Schedule,
    Capped,
}

enum Value {
    Fh1(Arc<FocalHorizonResult>),
    Q(Vec<(Domino, u128)>),
    Walk(Walk),
    WalkMemo(Walk, SuffixMemo),
    Pair(Walk, Walk),
}

const ROOTS: [RootKey; 2] = [(8, 4), (3, 4)];

fn fh1_at(r: &Receipt, hand_id: usize, trick_no: usize, k: usize) -> FocalHorizonResult {
    let frame = Frame::new(r, hand_id, trick_no, None);
    focal_horizon(
        &frame.oracle,
        &frame.root,
        &frame.position,
        &frame.field,
        &frame.field,
        &FocalSpec {
            horizon: k,
            node_fiber_cap: AMPLE_CAP,
        },
    )
    .expect("an affordable root completes under the ample cap")
}

fn exact_q_at(r: &Receipt, hand_id: usize, trick_no: usize) -> Vec<(Domino, u128)> {
    let frame = Frame::new(r, hand_id, trick_no, None);
    let belief = frame.belief();
    let ladder = FocalLadder::open(&frame.ctx());
    ladder
        .legal()
        .iter()
        .map(|a| {
            let mut rs = ResponseStats::default();
            (
                *a,
                response_success_mass(&frame.oracle, &belief.focal_play(*a), &frame.field, &mut rs),
            )
        })
        .collect()
}

fn build_fixture() -> Fixture {
    let r = receipt();
    // Heaviest first: h3-t4's sequential walks and engines.
    let mut jobs: Vec<Job> = Vec::new();
    for (hand_id, trick_no) in [(3usize, 4usize), (8, 4)] {
        jobs.push(Job::SeqOff(hand_id, trick_no));
        jobs.push(Job::SeqOn(hand_id, trick_no));
        for k in [2usize, 1, 0] {
            jobs.push(Job::Fh1(hand_id, trick_no, k));
        }
        jobs.push(Job::Q(hand_id, trick_no));
    }
    jobs.push(Job::Interrupted);
    jobs.push(Job::Schedule);
    jobs.push(Job::Capped);
    let values = fixture::compute_all(&jobs, |job| match *job {
        Job::Fh1(h, t, k) => Value::Fh1(Arc::new(fh1_at(&r, h, t, k))),
        Job::Q(h, t) => Value::Q(exact_q_at(&r, h, t)),
        Job::SeqOff(h, t) => {
            let frame = Frame::new(&r, h, t, None);
            Value::Walk(walk(&frame, &[(0, INF), (1, INF), (2, INF)], None))
        }
        Job::SeqOn(h, t) => {
            let frame = Frame::new(&r, h, t, None);
            let mut memo = SuffixMemo::new();
            let w = walk(&frame, &[(0, INF), (1, INF), (2, INF)], Some(&mut memo));
            Value::WalkMemo(w, memo)
        }
        Job::Interrupted => {
            let frame = Frame::new(&r, 8, 4, None);
            let ctx = frame.ctx();
            let mut ladder = FocalLadder::open(&ctx);
            let o0 = ladder.advance(&ctx, 0, &budget(INF), None);
            let o1 = ladder.advance(&ctx, 1, &budget(H8_K1_CEILING), None);
            let interrupted = Walk {
                ladder: ladder.clone(),
                outcomes: vec![o0.clone(), o1.clone()],
            };
            let o2 = ladder.advance(&ctx, 1, &budget(INF), None);
            let resumed = Walk {
                ladder,
                outcomes: vec![o0, o1, o2],
            };
            Value::Pair(interrupted, resumed)
        }
        Job::Schedule => {
            let frame = Frame::new(&r, 8, 4, None);
            let mut memo = SuffixMemo::new();
            Value::Walk(walk(&frame, &H8_SCHEDULE, Some(&mut memo)))
        }
        Job::Capped => {
            let frame = Frame::new(&r, 8, 4, None);
            let ctx = frame.ctx();
            let mut ladder = FocalLadder::open(&ctx);
            let o = ladder.advance(
                &ctx,
                0,
                &WorkBudget {
                    read_ceiling: INF,
                    node_fiber_cap: H8_TINY_CAP,
                },
                None,
            );
            Value::Walk(Walk {
                ladder,
                outcomes: vec![o],
            })
        }
    });
    let mut fx = Fixture {
        fh1: HashMap::new(),
        exact_q: HashMap::new(),
        seq_off: HashMap::new(),
        seq_on: HashMap::new(),
        h8_interrupted: Walk {
            ladder: FocalLadder::open(&Frame::new(&r, 8, 4, None).ctx()),
            outcomes: Vec::new(),
        },
        h8_resumed: Walk {
            ladder: FocalLadder::open(&Frame::new(&r, 8, 4, None).ctx()),
            outcomes: Vec::new(),
        },
        h8_schedule: Walk {
            ladder: FocalLadder::open(&Frame::new(&r, 8, 4, None).ctx()),
            outcomes: Vec::new(),
        },
        h8_capped: Walk {
            ladder: FocalLadder::open(&Frame::new(&r, 8, 4, None).ctx()),
            outcomes: Vec::new(),
        },
    };
    for (job, value) in jobs.into_iter().zip(values) {
        match (job, value) {
            (Job::Fh1(h, t, k), Value::Fh1(v)) => {
                fx.fh1.insert((h, t, k), v);
            }
            (Job::Q(h, t), Value::Q(q)) => {
                fx.exact_q.insert((h, t), q);
            }
            (Job::SeqOff(h, t), Value::Walk(w)) => {
                fx.seq_off.insert((h, t), w);
            }
            (Job::SeqOn(h, t), Value::WalkMemo(w, m)) => {
                fx.seq_on.insert((h, t), (w, m));
            }
            (Job::Interrupted, Value::Pair(a, b)) => {
                fx.h8_interrupted = a;
                fx.h8_resumed = b;
            }
            (Job::Schedule, Value::Walk(w)) => {
                fx.h8_schedule = w;
            }
            (Job::Capped, Value::Walk(w)) => {
                fx.h8_capped = w;
            }
            _ => unreachable!("a job's value is of the job's kind"),
        }
    }
    fx
}

static FIXTURE: LazyLock<Fixture> = LazyLock::new(build_fixture);

fn fh1(hand_id: usize, trick_no: usize, k: usize) -> &'static FocalHorizonResult {
    FIXTURE.fh1.get(&(hand_id, trick_no, k)).unwrap_or_else(|| {
        panic!("the fixture holds no FH1 result at h{hand_id}-t{trick_no} k={k}")
    })
}

fn exact_q(hand_id: usize, trick_no: usize) -> &'static [(Domino, u128)] {
    FIXTURE
        .exact_q
        .get(&(hand_id, trick_no))
        .unwrap_or_else(|| panic!("the fixture holds no exact Q at h{hand_id}-t{trick_no}"))
}

fn q_of(q: &[(Domino, u128)], a: Domino) -> u128 {
    q.iter()
        .find(|(t, _)| *t == a)
        .expect("every root action has a Q")
        .1
}

fn seq_off(hand_id: usize, trick_no: usize) -> &'static Walk {
    FIXTURE
        .seq_off
        .get(&(hand_id, trick_no))
        .unwrap_or_else(|| panic!("the fixture holds no memo-off walk at h{hand_id}-t{trick_no}"))
}

fn seq_on(hand_id: usize, trick_no: usize) -> &'static (Walk, SuffixMemo) {
    FIXTURE
        .seq_on
        .get(&(hand_id, trick_no))
        .unwrap_or_else(|| panic!("the fixture holds no memo-on walk at h{hand_id}-t{trick_no}"))
}

fn tile(name: &str) -> Domino {
    (0..28)
        .filter_map(Domino::from_index)
        .find(|d| format!("{d}") == name)
        .unwrap_or_else(|| panic!("tile {name}"))
}

fn tiles(names: &[&str]) -> Vec<Domino> {
    names.iter().map(|n| tile(n)).collect()
}

fn ratio(m: u128, z: u128) -> BigRational {
    BigRational::new(BigInt::from(m), BigInt::from(z))
}

/// Every VALUE view of a ladder root: intervals, bar, survivors,
/// verdict, `U*`, `Γ` — the policy id excluded (a tie may keep a prior
/// policy; the value it attains is what the law fixes).
fn assert_values_equal_fh1(view: &LadderView, res: &FocalHorizonResult, what: &str) {
    assert_eq!(
        view.actions.len(),
        res.actions.len(),
        "{what}: action count"
    );
    for (a, b) in view.actions.iter().zip(&res.actions) {
        assert_eq!(a.action, b.action, "{what}: action order");
        assert_eq!(a.lower_mass, b.lower_mass, "{what}: L at {}", a.action);
        assert_eq!(
            a.upper_mass,
            Some(b.upper_mass),
            "{what}: U at {}",
            a.action
        );
        assert_eq!(a.root_mass, b.root_mass, "{what}: Z");
    }
    assert_eq!(view.bar_mass, res.bar_mass, "{what}: bar");
    assert_eq!(view.bar_action, res.policy_action(), "{what}: bar action");
    assert_eq!(view.survivors, res.survivors, "{what}: survivors");
    assert_eq!(view.verdict, res.verdict, "{what}: verdict");
    assert_eq!(
        view.executable_lower_mass, res.executable_lower_mass,
        "{what}: L_exec"
    );
    assert_eq!(
        view.global_upper_mass,
        Some(res.global_upper_mass),
        "{what}: U*"
    );
    assert_eq!(
        view.certified_regret.as_ref(),
        Some(&res.certified_regret),
        "{what}: Γ"
    );
}

fn assert_views_equal(a: &LadderView, b: &LadderView, what: &str) {
    assert_eq!(a.actions, b.actions, "{what}: action views");
    assert_eq!(a.bar_mass, b.bar_mass, "{what}: bar");
    assert_eq!(a.bar_action, b.bar_action, "{what}: bar action");
    assert_eq!(a.survivors, b.survivors, "{what}: survivors");
    assert_eq!(a.verdict, b.verdict, "{what}: verdict");
    assert_eq!(a.global_upper_mass, b.global_upper_mass, "{what}: U*");
    assert_eq!(a.certified_regret, b.certified_regret, "{what}: Γ");
    assert_eq!(a.policy, b.policy, "{what}: choice table");
    assert_eq!(a.policy.id(), b.policy.id(), "{what}: policy id");
}

fn identity_of(frame: &Frame) -> SemanticsIdentity {
    let declaring = frame.root.kernel().viewer().team() == frame.position.declaring_team;
    SemanticsIdentity {
        root_id: root_identity(&frame.root, &frame.position),
        rules_id: "texas42-v1".to_string(),
        field_id: "level0-modeled-mind-v1".to_string(),
        utility_id: if declaring {
            "pmake-v1".to_string()
        } else {
            "pmake-setting-v1".to_string()
        },
        contract: frame.position.bid,
        belief_id: "uniform-root".to_string(),
        policy_class_id: "information-consistent-full".to_string(),
        score_semantics_id: "declaring-banked-43bin-v1".to_string(),
    }
}

// ---------------------------------------------------------------------------
// LP — ladder parity with FH1.
// ---------------------------------------------------------------------------

#[test]
fn lp_ladder_parity_with_fh1() {
    let r = receipt();
    // A fresh ladder walked directly at k: everything equal, policy id
    // included (no prior facts, so no tie can keep an older policy).
    for (hand_id, trick_no, k) in [(8usize, 4usize, 2usize), (8, 4, 1), (3, 4, 1)] {
        let frame = Frame::new(&r, hand_id, trick_no, None);
        let w = walk(&frame, &[(k, INF)], None);
        assert!(
            w.outcomes[0].is_completed(),
            "LP: the uncapped pass completes"
        );
        let view = w.ladder.root_view();
        let res = fh1(hand_id, trick_no, k);
        assert_values_equal_fh1(
            &view,
            res,
            &format!("LP direct h{hand_id}-t{trick_no} k={k}"),
        );
        assert_eq!(
            view.policy.id(),
            res.policy.id(),
            "LP: a fresh direct-k ladder materializes FH1's π_k exactly"
        );
        assert_eq!(view.horizon, Some(k));
        assert_eq!(view.policy.states(), res.policy.states());
    }
    // The sequential ladder: every value view equals FH1's at every k.
    for (hand_id, trick_no) in ROOTS {
        let w = seq_off(hand_id, trick_no);
        for (k, outcome) in w.outcomes.iter().enumerate() {
            assert!(outcome.is_completed(), "LP: uncapped passes complete");
            let res = fh1(hand_id, trick_no, k);
            assert_values_equal_fh1(
                &outcome.report().view,
                res,
                &format!("LP sequential h{hand_id}-t{trick_no} k={k}"),
            );
            assert_eq!(
                outcome.report().suffix_hits,
                0,
                "LP: memo off consults nothing"
            );
        }
        assert_eq!(w.ladder.passes(), &[0, 1, 2]);
    }
}

// ---------------------------------------------------------------------------
// FH7 — budget honesty.
// ---------------------------------------------------------------------------

#[test]
fn fh7_budget_honesty_at_h8_t4() {
    let fx = &*FIXTURE;
    let w = &fx.h8_interrupted;
    assert!(
        w.outcomes[0].is_completed(),
        "FH7: k = 0 completed uncapped"
    );
    let Outcome::Interrupted {
        report,
        residual_frontier,
        stopping_node,
        unaffordable,
    } = &w.outcomes[1]
    else {
        panic!("FH7: the small ceiling interrupts k = 1")
    };
    let legal = w.ladder.legal().to_vec();
    let k0 = fh1(8, 4, 0);
    let q = exact_q(8, 4);
    // (1) no root child dropped: residual frontier ∪ completed = every child.
    let mut children: Vec<Domino> = report.children_completed.clone();
    for n in residual_frontier {
        if n.history.len() == 1 {
            children.push(n.history[0]);
        }
    }
    children.sort_by_key(|d| d.index());
    assert_eq!(
        children, legal,
        "FH7: every root child is completed or residual"
    );
    assert!(
        !report.children_completed.is_empty() && report.children_completed.len() < legal.len(),
        "FH7: the ceiling stops inside k = 1, not before it and not after it"
    );
    assert!(
        unaffordable.is_empty(),
        "FH7: no cap refusal under the ample cap"
    );
    // Every residual node is typed and carries its mass.
    for n in residual_frontier {
        assert!(n.mass > 0);
        assert!(n.history.len() <= 12);
    }
    let stopped: Vec<&_> = residual_frontier
        .iter()
        .filter(|n| n.cause == ResidualCause::Stopped)
        .collect();
    assert_eq!(stopped.len(), 1, "FH7: exactly one stopping node");
    // (2) unfinished children carry the k = 0 facts, equal to the uncapped run's.
    let view = &report.view;
    for n in residual_frontier.iter().filter(|n| n.history.len() == 1) {
        let a = n.history[0];
        let retained = n
            .retained
            .as_ref()
            .expect("FH7: an unfinished root child after k = 0 holds its k = 0 fact");
        let fh = k0.interval(a).expect("a root action");
        assert_eq!(
            retained.lower_mass, fh.lower_mass,
            "FH7: retained L_0 at {a}"
        );
        assert_eq!(
            retained.upper_mass, fh.upper_mass,
            "FH7: retained U_0 at {a}"
        );
        assert_eq!(retained.completed_at, 0);
        assert_eq!(retained.lower_horizon, 0);
        let v = view.action(a).expect("a root action");
        assert_eq!(
            v.lower_mass, fh.lower_mass,
            "FH7: the view carries the retained lower"
        );
        assert_eq!(
            v.upper_mass,
            Some(fh.upper_mass),
            "FH7: the view carries the retained upper"
        );
        assert_eq!(v.lower_horizon, Some(0));
    }
    for a in &report.children_completed {
        let v = view.action(*a).expect("a root action");
        assert_eq!(
            v.lower_horizon,
            Some(1),
            "FH7: a completed child is at k = 1"
        );
        let fh = fh1(8, 4, 1).interval(*a).expect("a root action");
        assert_eq!(v.lower_mass, fh.lower_mass, "FH7: a completed child's L_1");
        assert_eq!(
            v.upper_mass,
            Some(fh.upper_mass),
            "FH7: a completed child's U_1"
        );
    }
    // (3) every action's interval contains the independent Q_a.
    for v in &view.actions {
        let qa = q_of(q, v.action);
        assert!(v.lower_mass <= qa, "FH7: L ≤ Q at {}", v.action);
        assert!(
            v.upper_mass.expect("all priced at k = 0") >= qa,
            "FH7: Q ≤ U at {}",
            v.action
        );
    }
    // (4) the boundary names reads spent, ceiling and the stopping node.
    assert_eq!(report.ceiling, H8_K1_CEILING);
    assert!(
        report.reads_spent >= H8_K1_CEILING,
        "FH7: the pass ran to the ceiling"
    );
    assert!(
        report.reads_spent - H8_K1_CEILING < 5_000,
        "FH7: the overshoot is at most one frontier evaluation's reads"
    );
    let stop = stopping_node
        .as_ref()
        .expect("FH7: a budget stop names its node");
    assert_eq!(
        &stopped[0].history, stop,
        "FH7: the stopping node is the frontier's"
    );
    assert!(stop.len() > 1, "FH7: the stop is below a root child");
    assert!(
        residual_frontier
            .iter()
            .any(|n| n.history == stop[..1] && n.cause == ResidualCause::Enclosing),
        "FH7: the enclosing root child is listed as unfinished"
    );
    // (5) resume + completion ≡ uninterrupted on every derived view and the fact set.
    let resumed = &fx.h8_resumed;
    let o2 = &resumed.outcomes[2];
    assert!(o2.is_completed(), "FH7: the resume completes k = 1");
    let uninterrupted = seq_off(8, 4);
    assert_eq!(
        resumed.ladder.render(),
        {
            // The uninterrupted reference through k = 1 only: replay the
            // fixture's memo-off walk to that step.
            let r = receipt();
            let frame = Frame::new(&r, 8, 4, None);
            walk(&frame, &[(0, INF), (1, INF)], None).ladder.render()
        },
        "FH7: the resumed fact set is the uninterrupted fact set, byte for byte"
    );
    assert_views_equal(
        &o2.report().view,
        &uninterrupted.outcomes[1].report().view,
        "FH7 resume ≡ uninterrupted k = 1",
    );
    // The spend as a sum: the interrupted pass plus the resume is at
    // least the uninterrupted pass (re-entering the stopped chain costs
    // its reads again) and the excess is a strict fraction of the
    // interrupted pass.
    let sum = w.outcomes[1].report().reads_spent + o2.report().reads_spent;
    let one = uninterrupted.outcomes[1].report().reads_spent;
    assert!(
        sum >= one,
        "FH7: resume never spends less than the uninterrupted pass"
    );
    assert!(
        sum - one < w.outcomes[1].report().reads_spent,
        "FH7: re-entry is a fraction"
    );
}

// ---------------------------------------------------------------------------
// FH7b — monotone under interruption.
// ---------------------------------------------------------------------------

#[test]
fn fh7b_monotone_under_interruption() {
    let w = &FIXTURE.h8_schedule;
    assert_eq!(w.outcomes.len(), H8_SCHEDULE.len());
    let mut prev: Option<&LadderView> = None;
    let mut interruptions = 0;
    for (i, o) in w.outcomes.iter().enumerate() {
        if !o.is_completed() {
            interruptions += 1;
        }
        let v = &o.report().view;
        if let Some(p) = prev {
            for (a, b) in p.actions.iter().zip(&v.actions) {
                assert_eq!(a.action, b.action);
                assert!(
                    b.lower_mass >= a.lower_mass,
                    "FH7b: a lower fell at {} step {i}",
                    a.action
                );
                match (a.upper_mass, b.upper_mass) {
                    (Some(x), Some(y)) => {
                        assert!(y <= x, "FH7b: an upper rose at {} step {i}", a.action)
                    }
                    (Some(_), None) => {
                        panic!("FH7b: an upper was discarded at {} step {i}", a.action)
                    }
                    _ => {}
                }
            }
            assert!(v.bar_mass >= p.bar_mass, "FH7b: the bar fell at step {i}");
            if let (Some(x), Some(y)) = (&p.certified_regret, &v.certified_regret) {
                assert!(y <= x, "FH7b: Γ rose at step {i}");
            }
            assert!(
                p.certified_regret.is_none() || v.certified_regret.is_some(),
                "FH7b: a regret never becomes absent"
            );
            assert!(
                v.survivors.iter().all(|s| p.survivors.contains(s)),
                "FH7b: survivors only shrink at step {i}"
            );
        }
        prev = Some(v);
    }
    assert!(
        interruptions >= 3,
        "FH7b: the schedule interrupts at every horizon"
    );
    let last = w.outcomes.last().expect("a step").report();
    assert_eq!(
        last.view.verdict,
        FocalVerdict::Settled {
            action: tile("3-3")
        }
    );
    assert_eq!(last.view.certified_regret, Some(BigRational::zero()));
    // Strictness: some lower rose and some upper fell across the schedule.
    let first = w.outcomes[2].report().view.clone();
    assert!(first
        .actions
        .iter()
        .zip(&last.view.actions)
        .any(|(a, b)| b.lower_mass > a.lower_mass));
    assert!(first
        .actions
        .iter()
        .zip(&last.view.actions)
        .any(|(a, b)| b.upper_mass < a.upper_mass));
}

// ---------------------------------------------------------------------------
// FH7c — the placeholder is not a fact.
// ---------------------------------------------------------------------------

#[test]
fn fh7c_placeholder_is_not_a_fact() {
    let r = receipt();
    // The contrast first: a root decided after every action (h10-t6, the
    // already-made receipt root) holds the §5 arithmetic as a FACT at zero
    // reads under a zero ceiling — nothing to interrupt, nothing trivial.
    {
        let frame = Frame::new(&r, 10, 6, None);
        let ctx = frame.ctx();
        let mut ladder = FocalLadder::open(&ctx);
        let o = ladder.advance(&ctx, 0, &budget(0), None);
        assert!(
            o.is_completed(),
            "FH7c contrast: decided children complete read-free"
        );
        assert_eq!(o.report().reads_spent, 0);
        let v = &o.report().view;
        assert!(v
            .actions
            .iter()
            .all(|a| a.collapsed() && a.lower_mass == v.root_mass));
        assert_eq!(v.certified_regret, Some(BigRational::zero()));
        assert!(matches!(v.verdict, FocalVerdict::Equivalent { .. }));
    }
    for (hand_id, trick_no) in [(4usize, 6usize), (8, 4)] {
        let frame = Frame::new(&r, hand_id, trick_no, None);
        let ctx = frame.ctx();
        let mut ladder = FocalLadder::open(&ctx);
        let o = ladder.advance(&ctx, 0, &budget(0), None);
        let Outcome::Interrupted {
            report,
            residual_frontier,
            stopping_node,
            ..
        } = &o
        else {
            panic!("FH7c: a zero ceiling interrupts before any upper exists")
        };
        assert_eq!(report.reads_spent, 0);
        assert!(ladder.facts().is_empty(), "FH7c: no fact was installed");
        let v = &report.view;
        for a in &v.actions {
            assert!(
                a.interval().is_none(),
                "FH7c: no action interval at {}",
                a.action
            );
            assert_eq!(a.lower_mass, 0);
            assert!(a.lower_horizon.is_none());
        }
        assert!(
            v.certified_regret.is_none(),
            "FH7c: no regret from the trivial upper"
        );
        assert!(v.global_upper_mass.is_none());
        assert_eq!(
            v.verdict,
            FocalVerdict::Unresolved {
                survivors: ladder.legal().to_vec()
            }
        );
        assert_eq!(v.survivors, ladder.legal().to_vec());
        assert_eq!(stopping_node.as_deref(), Some(&[ladder.legal()[0]][..]));
        assert_eq!(residual_frontier.len(), ladder.legal().len());
        assert!(residual_frontier.iter().all(|n| n.retained.is_none()));
        // The uncapped pass completes with an interval everywhere.
        let o = ladder.advance(&ctx, 0, &budget(INF), None);
        assert!(o.is_completed());
        assert!(o
            .report()
            .view
            .actions
            .iter()
            .all(|a| a.interval().is_some()));
        assert!(o.report().view.certified_regret.is_some());
    }
}

// ---------------------------------------------------------------------------
// PS1 / PS2 — proof-state facts.
// ---------------------------------------------------------------------------

fn state_with(frame: &Frame, ladder: &FocalLadder) -> (ProofState, Vec<Fact>) {
    let ctx = frame.ctx();
    let producer = FocalHorizonProducer { ladder, ctx: &ctx };
    let mut state = ProofState::open(&frame.root, &frame.position, identity_of(frame));
    let results = state.run_producer(&producer);
    assert!(
        results.iter().all(Result::is_ok),
        "facts install with zero rejections"
    );
    let facts = state.facts().iter().map(|sf| sf.fact.clone()).collect();
    (state, facts)
}

#[test]
fn ps1_facts_install_and_close() {
    let r = receipt();
    let frame = Frame::new(&r, 8, 4, None);
    let fx = &*FIXTURE;
    // The interrupted ladder (k = 0 complete, k = 1 partial): retained values.
    let w = &fx.h8_interrupted;
    let view = w.ladder.root_view();
    let (state, facts) = state_with(&frame, &w.ladder);
    assert_eq!(
        facts.len(),
        2 * w.ladder.legal().len(),
        "one lower and one upper per action"
    );
    let report = state.closure();
    assert_eq!(
        report.survivors, view.survivors,
        "PS1: closure survivors = the ladder's"
    );
    let exec = report.exec.as_ref().expect("an executable lower exists");
    assert_eq!(
        exec.value,
        ratio(view.executable_lower_mass, view.root_mass),
        "PS1: B_exec = L_exec"
    );
    assert_eq!(exec.action, view.bar_action);
    assert_eq!(report.bar, view.bar());
    assert_eq!(
        Some(report.certified_regret.clone()),
        view.certified_regret,
        "PS1: Γ"
    );
    assert_eq!(
        report.u_star,
        view.global_upper().expect("all uppers present")
    );
    for v in &report.views {
        let a = view.action(v.action).expect("a root action");
        assert_eq!(v.lower, a.lower());
        assert_eq!(Some(v.upper.clone()), a.upper());
    }
    assert!(matches!(report.result, StateResult::Unresolved { .. }));
    // A second produce after the resume (k = 1 complete) only tightens,
    // and a third at k = 2 settles.
    let mut state = state;
    let mut prev = report;
    for (ladder, k) in [(&fx.h8_resumed.ladder, 1usize), (&seq_off(8, 4).ladder, 2)] {
        let ctx = frame.ctx();
        let producer = FocalHorizonProducer { ladder, ctx: &ctx };
        let results = state.run_producer(&producer);
        assert!(results.iter().all(Result::is_ok));
        let next = state.closure();
        for (a, b) in prev.views.iter().zip(&next.views) {
            assert!(b.lower >= a.lower, "PS1: a lower fell at k = {k}");
            assert!(b.upper <= a.upper, "PS1: an upper rose at k = {k}");
        }
        assert!(
            next.certified_regret <= prev.certified_regret,
            "PS1: Γ rose at k = {k}"
        );
        assert!(next.survivors.iter().all(|s| prev.survivors.contains(s)));
        let view = ladder.root_view();
        assert_eq!(next.survivors, view.survivors);
        assert_eq!(Some(next.certified_regret.clone()), view.certified_regret);
        prev = next;
    }
    assert_eq!(
        prev.result,
        StateResult::Settled {
            action: tile("3-3")
        }
    );
    assert_eq!(prev.certified_regret, BigRational::zero());
}

#[test]
fn ps2_executability_is_honest() {
    let r = receipt();
    let frame = Frame::new(&r, 8, 4, None);
    let w = &FIXTURE.h8_interrupted;
    let ctx = frame.ctx();
    let producer = FocalHorizonProducer {
        ladder: &w.ladder,
        ctx: &ctx,
    };
    let (_, facts) = state_with(&frame, &w.ladder);
    let belief = frame.belief();
    let mut retained_seen = false;
    let mut executable_lowers = 0;
    for f in &facts {
        let Fact::Bound(b) = f else {
            panic!("the producer emits bound facts only")
        };
        match b.side {
            BoundSide::Upper => {
                assert!(!b.executable, "PS2: no upper is executable");
                assert!(b.authority.starts_with("focal-horizon:god:k="));
                assert!(b.authority.ends_with(":upper"));
            }
            BoundSide::Lower => {
                assert!(
                    b.executable,
                    "PS2: every completed-pass lower is executable"
                );
                executable_lowers += 1;
                let tail_id = w.ladder.identity().tail_id.clone();
                assert!(b
                    .authority
                    .starts_with(&format!("focal-horizon:{tail_id}:k=")));
                assert!(b.authority.ends_with(":lower"));
                // Independent re-pricing of the STORED policy through
                // `viewer_success_mass` (the producer's own check, and
                // below it the evaluator called directly on the k = 0
                // witness, which is the tail itself).
                let fact = w.ladder.fact_at(&[b.action]).expect("a fact at the child");
                let priced = producer.reprice(b.action).expect("a fact exists");
                assert_eq!(
                    priced, fact.lower_mass,
                    "PS2: the stored policy attains its lower"
                );
                assert_eq!(
                    b.value,
                    ratio(fact.lower_mass, w.ladder.root_mass()),
                    "PS2: the fact carries the stored value"
                );
                if b.authority.contains(":k=0:") {
                    retained_seen = true;
                    assert!(
                        fact.policy.is_empty(),
                        "PS2: the k = 0 witness is the tail itself"
                    );
                    let mut rs = RecursionStats::default();
                    let v = viewer_success_mass(
                        &frame.oracle,
                        &belief.focal_play(b.action),
                        &frame.field,
                        &frame.field,
                        &mut rs,
                    );
                    assert_eq!(
                        v, fact.lower_mass,
                        "PS2: the retained lower is the tail's value"
                    );
                } else {
                    assert!(b.authority.contains(":k=1:"));
                    assert!(!fact.policy.is_empty());
                }
            }
        }
    }
    assert!(
        retained_seen,
        "PS2: one RETAINED k = 0 lower travels with the interrupted pass"
    );
    assert_eq!(executable_lowers, w.ladder.legal().len());
}

// ---------------------------------------------------------------------------
// SR1 / SR2 — exact suffix reuse.
// ---------------------------------------------------------------------------

/// The pinned first suffix hit at h8-t4 (receipt contract, σ0): the
/// first node of the k = 1 pass whose k = 0 fact had collapsed.
const H8_FIRST_HIT: [&str; 4] = ["2-1", "0-0", "2-0", "3-0"];

#[test]
fn sr1_suffix_reuse_is_invisible_in_value() {
    for (hand_id, trick_no) in ROOTS {
        let off = seq_off(hand_id, trick_no);
        let (on, memo) = seq_on(hand_id, trick_no);
        for k in 0..3 {
            assert!(on.outcomes[k].is_completed());
            assert_views_equal(
                &on.outcomes[k].report().view,
                &off.outcomes[k].report().view,
                &format!("SR1 h{hand_id}-t{trick_no} k={k}"),
            );
        }
        assert_eq!(
            on.outcomes[0].report().suffix_hits,
            0,
            "SR1: nothing to hit at k = 0"
        );
        let hits: u64 = on.outcomes.iter().map(|o| o.report().suffix_hits).sum();
        assert!(hits > 0, "SR1: the memo hits at h{hand_id}-t{trick_no}");
        assert_eq!(memo.hits, hits);
        // Every receipt is a collapsed fact; the collapsed facts that are
        // not receipts are the decided nodes (the §5 arithmetic, never
        // memoized because the decided check precedes every lookup).
        assert!(memo.receipts > 0);
        assert!(
            (memo.receipts as usize) < on.ladder.collapsed_count(),
            "SR1: receipts ⊂ collapsed facts (decided facts are not receipts)"
        );
        // The memo saves reads at k ≥ 1.
        for k in 1..3 {
            assert!(
                on.outcomes[k].report().reads_spent < off.outcomes[k].report().reads_spent,
                "SR1: the memo saves reads at h{hand_id}-t{trick_no} k={k}"
            );
        }
        // The memo-on store is a SUBSET of the memo-off store (a receipt
        // returns before descending, so nodes below a collapsed one are
        // never revisited). Where the memo-on walk last composed or hit a
        // node (equal `completed_at`), the facts are IDENTICAL — values and
        // choice tables; where it never returned (a sibling below a
        // collapsed focal node), its fact is sound and the memo-off fact
        // is at least as tight. Both seen.
        assert!(
            on.ladder.facts().len() < off.ladder.facts().len(),
            "SR1: receipts skip descents"
        );
        let mut identical = 0usize;
        let mut stale = 0usize;
        for (key, f) in on.ladder.facts() {
            let g = &off.ladder.facts()[key];
            assert!(
                f.completed_at <= g.completed_at,
                "SR1: memo-off revisits every node"
            );
            assert!(
                g.lower_mass >= f.lower_mass,
                "SR1: memo-off is at least as tight (lower)"
            );
            assert!(
                g.upper_mass <= f.upper_mass,
                "SR1: memo-off is at least as tight (upper)"
            );
            if f.completed_at == g.completed_at {
                identical += 1;
                assert_eq!(f.lower_mass, g.lower_mass);
                assert_eq!(f.upper_mass, g.upper_mass);
                assert_eq!(
                    f.policy, g.policy,
                    "SR1: identical choice tables at every composed node"
                );
            } else {
                stale += 1;
            }
        }
        assert!(
            identical > 0,
            "SR1: facts composed or hit in the last pass are identical"
        );
        assert!(
            stale > 0,
            "SR1: a node below a receipt stays at its sound prior"
        );
    }
    // The pinned witness.
    let (_, memo) = seq_on(8, 4);
    assert_eq!(
        memo.first_hit.as_deref(),
        Some(&tiles(&H8_FIRST_HIT)[..]),
        "SR1: the first hit at h8-t4 is the pinned node"
    );
}

#[test]
fn sr2_identity_is_the_full_belief() {
    let r = receipt();
    // The contract-30 memo (h8-t4, receipt contract) consulted by a
    // contract-36 ladder at the same root: positive lookups, zero hits.
    let (_, memo30) = seq_on(8, 4);
    let mut memo = SuffixMemo::new();
    // A fresh memo-on walk at contract 30 (the fixture's memo stays
    // immutable), then FROZEN: it answers lookups and takes no receipt,
    // so every hit below would be a contract-30 receipt.
    let frame30 = Frame::new(&r, 8, 4, None);
    let w30 = walk(&frame30, &[(0, INF), (1, INF)], Some(&mut memo));
    assert_eq!(
        w30.ladder.root_view().survivors,
        seq_on(8, 4).0.outcomes[1].report().view.survivors
    );
    let hits_before = memo.hits;
    let receipts = memo.receipts;
    assert!(hits_before > 0 && receipts > 0);
    memo.freeze();
    let frame36 = Frame::new(&r, 8, 4, Some(36));
    let w36 = walk(&frame36, &[(0, INF), (1, INF)], Some(&mut memo));
    assert!(w36.outcomes.iter().all(Outcome::is_completed));
    let lookups: u64 = w36.outcomes.iter().map(|o| o.report().suffix_lookups).sum();
    let hits: u64 = w36.outcomes.iter().map(|o| o.report().suffix_hits).sum();
    assert!(
        lookups > 1_000,
        "SR2: the contract-36 ladder consulted the memo at every node"
    );
    assert_eq!(
        hits, 0,
        "SR2: a contract-30 receipt is never a contract-36 hit"
    );
    assert_eq!(memo.hits, hits_before, "SR2: zero hits across contracts");
    assert_eq!(
        memo.receipts, receipts,
        "SR2: the frozen memo took no receipt"
    );
    assert_eq!(w36.ladder.identity().contract, 36);
    assert_ne!(w36.ladder.identity(), w30.ladder.identity());
    assert_eq!(memo30.first_hit, memo.first_hit);
    // The same histories exist in both ladders — the miss is identity,
    // not absence.
    assert!(
        w36.ladder
            .facts()
            .keys()
            .filter(|k| w30.ladder.facts().contains_key(*k))
            .count()
            > 1_000
    );
    // A belief differing in factors alone is a miss: take the pinned
    // hit node, confirm the memo holds its belief, narrow one hidden
    // seat's factor to a single hand of its support — same root, same
    // contract, same history, same field — and the memo does not.
    let pinned = tiles(&H8_FIRST_HIT);
    let belief = frame30.belief_at(&pinned);
    assert!(
        memo.holds(&belief),
        "SR2: the pinned node's belief is a receipt"
    );
    let factor = &belief.factors()[0];
    let support = factor.support();
    assert!(
        support.len() > 1,
        "SR2: the seat's factor has a narrowable support"
    );
    let narrowed = belief.with_factor_table(factor.seat(), vec![support[0]]);
    assert_eq!(narrowed.history(), belief.history());
    assert_eq!(narrowed.position(), belief.position());
    assert_ne!(narrowed, belief);
    assert!(
        !memo.holds(&narrowed),
        "SR2: a belief differing in factors alone misses"
    );
    let before = memo.hits;
    assert!(memo.lookup(&narrowed).is_none());
    assert_eq!(memo.hits, before);
    assert!(memo.lookup(&belief).is_some());
    assert_eq!(memo.hits, before + 1);
    // And the contract-36 belief at the same history misses.
    let belief36 = frame36.belief_at(&pinned);
    assert_eq!(belief36.history(), belief.history());
    assert!(!memo.holds(&belief36) || belief36 == belief);
    assert_ne!(belief36, belief);
    assert!(!memo.holds(&belief36));
}

// ---------------------------------------------------------------------------
// FH-D — determinism of an interrupted run.
// ---------------------------------------------------------------------------

#[test]
fn fh_d_interrupted_run_is_deterministic() {
    let r = receipt();
    let frame = Frame::new(&r, 8, 4, None);
    let ctx = frame.ctx();
    let mut ladder = FocalLadder::open(&ctx);
    let o0 = ladder.advance(&ctx, 0, &budget(INF), None);
    let o1 = ladder.advance(&ctx, 1, &budget(H8_K1_CEILING), None);
    let w = &FIXTURE.h8_interrupted;
    assert_eq!(o0, w.outcomes[0], "FH-D: the k = 0 pass");
    assert_eq!(
        o1, w.outcomes[1],
        "FH-D: the interrupted k = 1 pass, frontier included"
    );
    assert_eq!(ladder.render(), w.ladder.render(), "FH-D: the fact set");
    assert_eq!(ladder, w.ladder);
    assert_eq!(format!("{o1:?}"), format!("{:?}", w.outcomes[1]));
    let one = BigRational::one();
    assert!(o1
        .report()
        .view
        .certified_regret
        .as_ref()
        .is_some_and(|g| g < &one));
}

// ---------------------------------------------------------------------------
// FH-C — the in-pass fiber-cap refusal (FH-A3, §41(7); FH4-AUDIT N3).
// ---------------------------------------------------------------------------

#[test]
fn fh_c_cap_refusal_leaves_the_enclosing_child_unfinished() {
    let r = receipt();
    let w = &FIXTURE.h8_capped;
    let Outcome::Interrupted {
        report,
        residual_frontier,
        stopping_node,
        unaffordable,
    } = &w.outcomes[0]
    else {
        panic!("FH-C: a tiny cap refuses somewhere")
    };
    assert!(
        stopping_node.is_none(),
        "FH-C: no budget stop, only cap refusals"
    );
    assert!(!unaffordable.is_empty(), "FH-C: at least one refusal");
    assert_eq!(report.ceiling, INF);
    let legal = w.ladder.legal().to_vec();
    let k0 = fh1(8, 4, 0);
    let frame = Frame::new(&r, 8, 4, None);
    // Every refused node: a viewer node of positive mass above the cap,
    // listed `Unaffordable` in the frontier with no fact at or under it.
    let viewer = frame.root.kernel().viewer();
    let mut refused_children: Vec<Domino> = Vec::new();
    for (history, fiber) in unaffordable {
        assert!(
            *fiber > H8_TINY_CAP,
            "FH-C: the refused fiber is above the cap"
        );
        assert!(
            history.len() > 1,
            "FH-C: the refused node is below a root child"
        );
        let belief = frame.belief_at(history);
        assert_eq!(belief.seat_to_move(), viewer, "FH-C: a viewer node refuses");
        assert_eq!(
            frame.oracle.mass(&belief),
            *fiber,
            "FH-C: the refusal names the exact fiber"
        );
        let node = residual_frontier
            .iter()
            .find(|n| &n.history == history)
            .expect("FH-C: the refused node is in the residual frontier");
        assert_eq!(
            node.cause,
            ResidualCause::Unaffordable {
                fiber: *fiber,
                cap: H8_TINY_CAP
            }
        );
        assert!(
            node.retained.is_none(),
            "FH-C: nothing was priced at a refused node"
        );
        assert!(w.ladder.fact_at(history).is_none());
        let key_prefix = history.clone();
        assert!(
            !w.ladder.facts().keys().any(|k| {
                k.len() >= key_prefix.len()
                    && key_prefix
                        .iter()
                        .zip(k.iter())
                        .all(|(d, b)| u8::try_from(d.index()).expect("fits") == *b)
            }),
            "FH-C: no fact at or under a refused node"
        );
        if !refused_children.contains(&history[0]) {
            refused_children.push(history[0]);
        }
    }
    // The enclosing root children are unfinished with placeholders; the
    // others completed with the uncapped run's k = 0 facts.
    let view = &report.view;
    assert!(
        !refused_children.is_empty() && refused_children.len() < legal.len(),
        "FH-C: some root children refuse and some complete at this cap"
    );
    for a in &legal {
        let v = view.action(*a).expect("a root action");
        if refused_children.contains(a) {
            assert!(
                w.ladder.fact_at(&[*a]).is_none(),
                "FH-C: no fact under the refused child {a}"
            );
            assert!(v.interval().is_none(), "FH-C: no interval at {a}");
            assert_eq!(v.lower_mass, 0);
            assert!(v.lower_horizon.is_none() && v.upper_mass.is_none());
            assert!(
                residual_frontier
                    .iter()
                    .any(|n| n.history == [*a] && n.cause == ResidualCause::Enclosing),
                "FH-C: the enclosing root child {a} is listed unfinished"
            );
            assert!(!report.children_completed.contains(a));
        } else {
            assert!(report.children_completed.contains(a), "FH-C: {a} completed");
            let fh = k0.interval(*a).expect("a root action");
            assert_eq!(
                v.lower_mass, fh.lower_mass,
                "FH-C: the completed child's L_0 at {a}"
            );
            assert_eq!(
                v.upper_mass,
                Some(fh.upper_mass),
                "FH-C: the completed child's U_0 at {a}"
            );
        }
    }
    assert!(
        view.certified_regret.is_none(),
        "FH-C: no regret from a placeholder"
    );
    assert!(view.global_upper_mass.is_none());
    assert_eq!(view.survivors, legal, "FH-C: an absent upper survives");
    assert!(matches!(view.verdict, FocalVerdict::Unresolved { .. }));
    // The uncapped pass at the same horizon then completes, and equals
    // the uncapped k = 0 run on every value view.
    let mut ladder = w.ladder.clone();
    let ctx = frame.ctx();
    let o = ladder.advance(&ctx, 0, &budget(INF), None);
    assert!(o.is_completed(), "FH-C: the ample cap completes");
    assert_values_equal_fh1(&o.report().view, k0, "FH-C uncapped after refusal");
}
