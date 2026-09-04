//! Slice FH3 — the FH8 anchors of `walt/math/focal_horizon_sandwich_v0.1.md`
//! §37 (cited by title; the object is the FOCAL-HORIZON HIERARCHY, FH-A2)
//! at the coordinates ruled in FH-A8 (`walt/CENSUS-RULINGS.md`), under
//! `walt/briefs/BRIEF-FH3.md`:
//!
//!   (ii)  h8-t4 at contracts 36 and 39 — where the record's trick-6 ply
//!         cut (`horizon_run1.txt`, cut 8) flipped the root action by 7‰;
//!   (iii) h4-t4 at contracts {receipt, 33, 36, 39, 42} — the
//!         contract-sensitive trick-5 specimen (cut 4).
//!
//! Anchor (i), h8-t3, is PROBE-ONLY (`focal_run1.txt`): measured before
//! this file was sized, its k = 0 ladder pass alone is ~154 s and 27M
//! reads, and the k ≤ 2 ladder ~9 min at 17 GB peak RSS — over the
//! brief's ~3-minute gate budget by any split (BRIEF-FH3 §3; the
//! decision is recorded in FH3-REPORT).
//!
//! THE ANSWERS ARE DISCOVERED, NEVER PINNED (FH-A8): no gate below asserts
//! WHICH k settles an anchor, or that a cut flips. What is asserted is the
//! soundness law at every horizon, with `Q_a` recomputed INDEPENDENTLY by
//! `response_success_mass` and the ply cut recomputed live by
//! `horizon_census`:
//!
//! FHA1 containment, nesting, collapse: `L_{a,k} ≤ Q_a ≤ U_{a,k}` for
//!      every action and k ∈ {0, 1, 2}; no lower falls and no upper
//!      rises with k (§41(2), (3), (4)); bar, `U*` and `Γ` monotone;
//!      survivors only shrink (Theorem 6); at k = 2 every trick-4 action
//!      is collapsed to `Q_a` (Proposition FH-last).
//! FHA2 `Settled ⇒ exact argmax`: at every k a `Settled{b}` names the
//!      unique exact maximizer, an `Equivalent` lists exactly the exact
//!      maximizers at `Q*`, an `Unresolved` survivor set contains every
//!      exact maximizer (§18, FH-tie, FH4). The FH-A8 law at (ii) is
//!      checked in its conditional form: wherever some other action's
//!      `U_{a,1}` is at or above `Q_b`, k = 1 is not `Settled{b}`.
//! FHA3 the ply-cut comparison: on the same (root, contract) the live
//!      census's per-action cut readings equal the engine's `U_{a,0}`
//!      (cut 4) and `U_{a,1}` (cut 8) — Proposition FH-cut, so the cut's
//!      argmax IS `argmax_a U_{a,m−1}`; the census's exact argmax agrees
//!      with the independent `Q_a`; and whenever the cut's argmax is not
//!      an exact maximizer, NO horizon's verdict certifies it: the ladder
//!      never selects the ply cut's wrong action (the brief's assertion).
//! FHA4 reuse parity: the sequential memo-on ladder's derived view at each
//!      k equals the direct engine's on every value (intervals, bar,
//!      survivors, verdict, `U*`, `Γ`) at every anchor coordinate.
//!
//! One `LazyLock` fixture holds everything expensive (BRIEF-CI1):
//! engines per (coordinate, k), `Q_a`, censuses, the ladder walk.
//! EXPLORATORY tier throughout; nothing here is a play-strength claim.

#[path = "common/fixture.rs"]
mod fixture;

use std::cmp::Reverse;
use std::collections::HashMap;
use std::sync::{Condvar, LazyLock, Mutex};

use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::Domino;
use walt::solver::adaptive::{CanonicalRoot, RootPosition};
use walt::solver::factor_belief::{
    response_success_mass, FactorBelief, ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::focal_horizon::{focal_horizon, FocalHorizonResult, FocalSpec, FocalVerdict};
use walt::solver::focal_ladder::{FocalLadder, LadderContext, LadderView, SuffixMemo, WorkBudget};
use walt::solver::godgap::legal_actions;
use walt::solver::horizon::{horizon_census, with_contract, HorizonCensus, HorizonSpec};
use walt::solver::policy::{DecisionMode, TieRule};

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

const AMPLE_CAP: u128 = 40_000;
const HORIZONS: [usize; 3] = [0, 1, 2];

/// One anchor coordinate: (hand, trick, contract) and the ply cuts the
/// record flagged there (cut 8 = the trick-6 frontier at (ii); cut 4 =
/// the trick-5 frontier at (iii); both at (ii) for the FH-cut identity).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coord {
    hand_id: usize,
    trick_no: usize,
    contract: u32,
}

impl Coord {
    fn label(self) -> String {
        format!("h{}-t{} bid {}", self.hand_id, self.trick_no, self.contract)
    }
}

/// The h4-t4 receipt contract is 30 (`horizon_run1.txt`); the coordinate
/// list is explicit so the fixture's keys are stable.
const ANCHORS: [(Coord, &[usize]); 7] = [
    (
        Coord {
            hand_id: 8,
            trick_no: 4,
            contract: 36,
        },
        &[4, 8],
    ),
    (
        Coord {
            hand_id: 8,
            trick_no: 4,
            contract: 39,
        },
        &[4, 8],
    ),
    (
        Coord {
            hand_id: 4,
            trick_no: 4,
            contract: 30,
        },
        &[4],
    ),
    (
        Coord {
            hand_id: 4,
            trick_no: 4,
            contract: 33,
        },
        &[4],
    ),
    (
        Coord {
            hand_id: 4,
            trick_no: 4,
            contract: 36,
        },
        &[4],
    ),
    (
        Coord {
            hand_id: 4,
            trick_no: 4,
            contract: 39,
        },
        &[4],
    ),
    (
        Coord {
            hand_id: 4,
            trick_no: 4,
            contract: 42,
        },
        &[4],
    ),
];

// ---------------------------------------------------------------------------
// The recompute-once fixture.
// ---------------------------------------------------------------------------

struct LadderWalk {
    /// The derived view after the sequential pass at each of `HORIZONS`.
    views: Vec<LadderView>,
    /// Reads spent by each pass (with reuse).
    reads: Vec<u64>,
}

struct Fixture {
    engines: HashMap<(Coord, usize), Box<FocalHorizonResult>>,
    exact_q: HashMap<Coord, Vec<(Domino, u128)>>,
    censuses: HashMap<(Coord, usize), HorizonCensus>,
    ladders: HashMap<Coord, LadderWalk>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Job {
    Engine(Coord, usize),
    Q(Coord),
    Census(Coord, usize),
    Ladder(Coord),
}

enum Value {
    Engine(Box<FocalHorizonResult>),
    Q(Vec<(Domino, u128)>),
    Census(HorizonCensus),
    Ladder(LadderWalk),
}

fn frame(r: &Receipt, c: Coord) -> (CanonicalRoot, RootPosition) {
    let (root, position) = root_at(r, c.hand_id, c.trick_no);
    (root, with_contract(&position, c.contract))
}

fn engine_at(r: &Receipt, c: Coord, k: usize) -> FocalHorizonResult {
    let (root, position) = frame(r, c);
    let field = FieldModel::new(field_spec());
    focal_horizon(
        &SupportOracle,
        &root,
        &position,
        &field,
        &field,
        &FocalSpec {
            horizon: k,
            node_fiber_cap: AMPLE_CAP,
        },
    )
    .unwrap_or_else(|e| panic!("an anchor root completes under the ample cap: {e:?}"))
}

/// `Q_a` per root action by the independent exact recursion.
fn exact_q_at(r: &Receipt, c: Coord) -> Vec<(Domino, u128)> {
    let (root, position) = frame(r, c);
    let field = FieldModel::new(field_spec());
    let belief = FactorBelief::uniform_root(&root, &position, &field);
    legal_actions(&root, &position)
        .into_iter()
        .map(|a| {
            let mut rs = ResponseStats::default();
            (
                a,
                response_success_mass(&SupportOracle, &belief.focal_play(a), &field, &mut rs),
            )
        })
        .collect()
}

fn census_at(r: &Receipt, c: Coord, cut: usize) -> HorizonCensus {
    let (root, position) = frame(r, c);
    let field = FieldModel::new(field_spec());
    horizon_census(
        &SupportOracle,
        &root,
        &position,
        &field,
        &HorizonSpec {
            cut_plays: cut,
            node_fiber_cap: AMPLE_CAP,
        },
    )
}

/// One ladder walked k = 0, 1, 2 with the suffix memo on, uncapped.
fn ladder_at(r: &Receipt, c: Coord) -> LadderWalk {
    let (root, position) = frame(r, c);
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec());
    let ctx = LadderContext {
        oracle: &oracle,
        root: &root,
        position: &position,
        lower_tail: &field,
        field: &field,
    };
    let mut ladder = FocalLadder::open(&ctx);
    let mut memo = SuffixMemo::new();
    let mut views = Vec::new();
    let mut reads = Vec::new();
    for k in HORIZONS {
        let outcome = ladder.advance(
            &ctx,
            k,
            &WorkBudget {
                read_ceiling: u64::MAX,
                node_fiber_cap: AMPLE_CAP,
            },
            Some(&mut memo),
        );
        assert!(
            outcome.is_completed(),
            "an uncapped pass at an anchor completes ({} k={k})",
            c.label()
        );
        reads.push(outcome.report().reads_spent);
        views.push(ladder.root_view());
    }
    LadderWalk { views, reads }
}

fn fixture_jobs() -> Vec<Job> {
    let mut jobs = Vec::new();
    for (c, cuts) in ANCHORS {
        for k in HORIZONS {
            jobs.push(Job::Engine(c, k));
        }
        jobs.push(Job::Q(c));
        for cut in cuts {
            jobs.push(Job::Census(c, *cut));
        }
        jobs.push(Job::Ladder(c));
    }
    // Heaviest first: the h4-t4 fiber (34,650) over h8-t4's (1,200); the
    // ladder walk and the deep engines over the rest.
    let weight = |job: &Job| -> (u128, u32) {
        let (c, kind) = match *job {
            Job::Ladder(c) => (c, 40),
            Job::Engine(c, k) => (c, 20 + u32::try_from(k).expect("small")),
            Job::Q(c) | Job::Census(c, _) => (c, 20),
        };
        let fiber: u128 = if c.hand_id == 4 { 34_650 } else { 1_200 };
        (fiber, kind)
    };
    jobs.sort_by_key(|job| Reverse(weight(job)));
    jobs
}

/// At most this many HEAVY jobs in flight at once (FH4-AUDIT N13). The
/// suite's transient peak is threads × one job's footprint, and EVERY
/// h4-t4 evaluation costs ~1.6 GB whatever its kind (measured standalone:
/// the k = 0 engine 1.6 GB, the k = 2 engine 1.6 GB, the ladder walk
/// 1.7 GB, exact `Q_a` inside the same 1.6 GB) — so with 18 threads the
/// unlimited fixture peaked at 18.2 GB, serializing the ladders alone
/// still 13.8 GB, and capping ladders plus deep engines at four 13.7 GB.
/// Five h4-t4 jobs at a time hold the transient near 8 GB; the h8-t4
/// jobs stay fully parallel; a waiting thread holds no memory. This
/// changes no assertion and no value.
const HEAVY_IN_FLIGHT: usize = 5;

fn heavy(job: &Job) -> bool {
    let c = match *job {
        Job::Ladder(c) | Job::Q(c) => c,
        Job::Engine(c, _) | Job::Census(c, _) => c,
    };
    c.hand_id == 4
}

fn build_fixture() -> Fixture {
    let r = receipt();
    let jobs = fixture_jobs();
    let heavy_slots = (Mutex::new(0usize), Condvar::new());
    let values = fixture::compute_all(&jobs, |job| {
        let (lock, ready) = &heavy_slots;
        if heavy(job) {
            let mut busy = lock.lock().expect("the slot count is not poisoned");
            while *busy >= HEAVY_IN_FLIGHT {
                busy = ready.wait(busy).expect("the slot count is not poisoned");
            }
            *busy += 1;
        }
        let value = match *job {
            Job::Engine(c, k) => Value::Engine(Box::new(engine_at(&r, c, k))),
            Job::Q(c) => Value::Q(exact_q_at(&r, c)),
            Job::Census(c, cut) => Value::Census(census_at(&r, c, cut)),
            Job::Ladder(c) => Value::Ladder(ladder_at(&r, c)),
        };
        if heavy(job) {
            *lock.lock().expect("the slot count is not poisoned") -= 1;
            ready.notify_one();
        }
        value
    });
    let mut fixture = Fixture {
        engines: HashMap::new(),
        exact_q: HashMap::new(),
        censuses: HashMap::new(),
        ladders: HashMap::new(),
    };
    for (job, value) in jobs.into_iter().zip(values) {
        match (job, value) {
            (Job::Engine(c, k), Value::Engine(e)) => {
                fixture.engines.insert((c, k), e);
            }
            (Job::Q(c), Value::Q(q)) => {
                fixture.exact_q.insert(c, q);
            }
            (Job::Census(c, cut), Value::Census(h)) => {
                fixture.censuses.insert((c, cut), h);
            }
            (Job::Ladder(c), Value::Ladder(l)) => {
                fixture.ladders.insert(c, l);
            }
            _ => unreachable!("a job's value is of the job's kind"),
        }
    }
    fixture
}

static FIXTURE: LazyLock<Fixture> = LazyLock::new(build_fixture);

fn engine(c: Coord, k: usize) -> &'static FocalHorizonResult {
    FIXTURE
        .engines
        .get(&(c, k))
        .unwrap_or_else(|| panic!("the fixture holds no engine at {} k={k}", c.label()))
}

fn exact_q(c: Coord) -> &'static [(Domino, u128)] {
    FIXTURE
        .exact_q
        .get(&c)
        .unwrap_or_else(|| panic!("the fixture holds no Q at {}", c.label()))
}

fn census(c: Coord, cut: usize) -> &'static HorizonCensus {
    FIXTURE
        .censuses
        .get(&(c, cut))
        .unwrap_or_else(|| panic!("the fixture holds no census at {} cut {cut}", c.label()))
}

fn ladder(c: Coord) -> &'static LadderWalk {
    FIXTURE
        .ladders
        .get(&c)
        .unwrap_or_else(|| panic!("the fixture holds no ladder at {}", c.label()))
}

fn q_of(c: Coord, a: Domino) -> u128 {
    exact_q(c)
        .iter()
        .find(|(t, _)| *t == a)
        .map(|(_, m)| *m)
        .unwrap_or_else(|| panic!("Q_{a} at {}", c.label()))
}

/// The exact maximizers by the independent `Q_a`, in tile order.
fn exact_set(c: Coord) -> Vec<Domino> {
    let q = exact_q(c);
    let qmax = q.iter().map(|(_, m)| *m).max().expect("a root action");
    q.iter()
        .filter(|(_, m)| *m == qmax)
        .map(|(a, _)| *a)
        .collect()
}

fn coords() -> impl Iterator<Item = Coord> {
    ANCHORS.iter().map(|(c, _)| *c)
}

// ---------------------------------------------------------------------------
// FHA1 — containment, nesting, collapse.
// ---------------------------------------------------------------------------

#[test]
fn fha1_containment_nesting_collapse() {
    for c in coords() {
        let label = c.label();
        for k in HORIZONS {
            let res = engine(c, k);
            for a in &res.actions {
                let q = q_of(c, a.action);
                assert!(
                    a.lower_mass <= q && q <= a.upper_mass,
                    "FHA1 §41(2) at {label} k={k} {}: [{}, {}] must contain Q = {q}",
                    a.action,
                    a.lower_mass,
                    a.upper_mass
                );
            }
        }
        for w in HORIZONS.windows(2) {
            let (p, n) = (engine(c, w[0]), engine(c, w[1]));
            for (x, y) in p.actions.iter().zip(&n.actions) {
                assert_eq!(
                    x.action, y.action,
                    "{label}: the same root actions at every k"
                );
                assert!(
                    y.lower_mass >= x.lower_mass,
                    "FHA1 §41(3) at {label} {}: L fell {} → {} from k={} to k={}",
                    x.action,
                    x.lower_mass,
                    y.lower_mass,
                    w[0],
                    w[1]
                );
                assert!(
                    y.upper_mass <= x.upper_mass,
                    "FHA1 §41(4) at {label} {}: U rose {} → {} from k={} to k={}",
                    x.action,
                    x.upper_mass,
                    y.upper_mass,
                    w[0],
                    w[1]
                );
            }
            assert!(
                n.bar_mass >= p.bar_mass,
                "FHA1 at {label}: the bar never falls"
            );
            assert!(
                n.global_upper_mass <= p.global_upper_mass,
                "FHA1 at {label}: U* never rises"
            );
            assert!(
                n.certified_regret <= p.certified_regret,
                "FHA1 at {label}: Γ never rises"
            );
            for a in &n.survivors {
                assert!(
                    p.survivors.contains(a),
                    "FHA1 at {label}: survivors only shrink (Theorem 6), {a} appeared at k={}",
                    w[1]
                );
            }
        }
        // FH-last at a trick-4 root: k = 2 is exact.
        let res = engine(c, 2);
        for a in &res.actions {
            let q = q_of(c, a.action);
            assert!(
                a.lower_mass == q && a.upper_mass == q,
                "FHA1 FH-last at {label} k=2 {}: [{}, {}] collapses to Q = {q}",
                a.action,
                a.lower_mass,
                a.upper_mass
            );
        }
        assert_eq!(
            res.spend.forced_tail_evaluations, res.spend.upper_tail_evaluations,
            "FHA1 at {label} k=2: every tail consultation is at a forced node"
        );
    }
}

// ---------------------------------------------------------------------------
// FHA2 — the verdict against the independent exact argmax.
// ---------------------------------------------------------------------------

#[test]
fn fha2_settled_implies_exact_argmax() {
    for c in coords() {
        let label = c.label();
        let exact = exact_set(c);
        let qmax = q_of(c, exact[0]);
        for k in HORIZONS {
            let res = engine(c, k);
            match &res.verdict {
                FocalVerdict::Settled { action } => {
                    assert_eq!(
                        exact,
                        vec![*action],
                        "FHA2 at {label} k={k}: Settled {action} must be the unique exact maximizer"
                    );
                    // The §18 criterion itself, against the independent Q.
                    for a in &res.actions {
                        if a.action != *action {
                            assert!(
                                qmax > a.upper_mass,
                                "FHA2 at {label} k={k}: a settled winner's Q* exceeds every other upper"
                            );
                        }
                    }
                }
                FocalVerdict::Equivalent {
                    actions,
                    value_mass,
                } => {
                    assert_eq!(
                        *actions, exact,
                        "FHA2 at {label} k={k}: Equivalent lists exactly the exact maximizers"
                    );
                    assert_eq!(*value_mass, qmax, "FHA2 at {label} k={k}: at Q*");
                }
                FocalVerdict::Unresolved { survivors } => {
                    for a in &exact {
                        assert!(
                            survivors.contains(a),
                            "FHA2 at {label} k={k}: exact maximizer {a} survives"
                        );
                    }
                }
            }
            // FH-A8's law in conditional form: an action whose k-upper is
            // at or above another action's Q blocks that action's Settled.
            for b in &res.actions {
                let qb = q_of(c, b.action);
                let blocked = res
                    .actions
                    .iter()
                    .any(|a| a.action != b.action && a.upper_mass >= qb);
                if blocked {
                    assert_ne!(
                        res.verdict,
                        FocalVerdict::Settled { action: b.action },
                        "FHA2 at {label} k={k}: some other upper is ≥ Q_{}, so it is not Settled",
                        b.action
                    );
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// FHA3 — the ply cut against the focal verdicts.
// ---------------------------------------------------------------------------

#[test]
fn fha3_ply_cut_never_certified_when_wrong() {
    for (c, cuts) in ANCHORS {
        let label = c.label();
        let exact = exact_set(c);
        for cut in cuts {
            let census = census(c, *cut);
            let m = cut / 4 - 1;
            let res = engine(c, m);
            // Proposition FH-cut: the cut readings ARE U_{a,m−1}.
            for a in &res.actions {
                let reading = census
                    .actions
                    .iter()
                    .find(|x| x.action == a.action)
                    .unwrap_or_else(|| panic!("the census reads {} at {label}", a.action));
                assert_eq!(
                    reading.cut_mass,
                    Some(a.upper_mass),
                    "FHA3 FH-cut at {label} cut {cut} {}: the cut reading is U_{{a,{m}}}",
                    a.action
                );
                assert_eq!(
                    reading.exact_mass,
                    q_of(c, a.action),
                    "FHA3 at {label}: the census's exact reading is the independent Q_a"
                );
            }
            let cut_argmax = census
                .cut_argmax
                .expect("an ample-cap census names the cut argmax");
            assert_eq!(
                census.exact_argmax,
                Some(exact[0]),
                "FHA3 at {label}: the census's exact argmax (lowest tile) is the first exact maximizer"
            );
            let wrong = !exact.contains(&cut_argmax);
            assert_eq!(
                census.cut_flips_root(),
                Some(wrong),
                "FHA3 at {label} cut {cut}: the flip flag is 'cut argmax not exact'"
            );
            if wrong {
                for k in HORIZONS {
                    let verdict = &engine(c, k).verdict;
                    assert_ne!(
                        *verdict,
                        FocalVerdict::Settled { action: cut_argmax },
                        "FHA3 at {label}: the ladder certified the ply cut's wrong action {cut_argmax} at k={k}"
                    );
                    if let FocalVerdict::Equivalent { actions, .. } = verdict {
                        assert!(
                            !actions.contains(&cut_argmax),
                            "FHA3 at {label}: the wrong action {cut_argmax} is not in an exact tie set"
                        );
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// FHA4 — the reuse column agrees with the direct engine in value.
// ---------------------------------------------------------------------------

#[test]
fn fha4_ladder_reuse_parity() {
    for c in coords() {
        let label = c.label();
        let walk = ladder(c);
        for (i, k) in HORIZONS.iter().enumerate() {
            let res = engine(c, *k);
            let view = &walk.views[i];
            for (d, l) in res.actions.iter().zip(&view.actions) {
                assert_eq!(d.action, l.action, "{label} k={k}: action order");
                assert_eq!(
                    (d.lower_mass, Some(d.upper_mass)),
                    (l.lower_mass, l.upper_mass),
                    "FHA4 at {label} k={k} {}: the ladder's interval is the engine's",
                    d.action
                );
            }
            assert_eq!(res.bar_mass, view.bar_mass, "FHA4 at {label} k={k}: bar");
            assert_eq!(
                res.survivors, view.survivors,
                "FHA4 at {label} k={k}: survivors"
            );
            assert_eq!(res.verdict, view.verdict, "FHA4 at {label} k={k}: verdict");
            assert_eq!(
                Some(res.global_upper_mass),
                view.global_upper_mass,
                "FHA4 at {label} k={k}: U*"
            );
            assert_eq!(
                Some(&res.certified_regret),
                view.certified_regret.as_ref(),
                "FHA4 at {label} k={k}: Γ"
            );
            assert_eq!(
                res.policy_action(),
                view.bar_action,
                "FHA4 at {label} k={k}: the lower policy's root action"
            );
            let direct = res.spend.field_reads + res.spend.tail_reads;
            eprintln!(
                "FHA4 {label} k={k}: reads direct {direct} | ladder {} | verdict {:?}",
                walk.reads[i], res.verdict
            );
        }
    }
}
