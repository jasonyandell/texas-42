//! Slice FH1 — the focal-horizon hierarchy engine. The parent's gates
//! FH1–FH6 (`walt/math/focal_horizon_sandwich_v0.1.md` §30–§35, as
//! narrowed by its companion and rulings FH-A1..A11) plus the free
//! record-parity gates FH-A5/FH-A11 name, the refusal-shape gate and
//! determinism.
//!
//! FH1  endpoint parity: at k = 0 every root action's lower equals
//!      `viewer_success_mass` under the tail computed independently, and
//!      its upper equals `Z − doomed` with doomed from `doom_enumeration`'s
//!      per-world truth (the U0 census's `GodUpper`, gate H2's check) —
//!      ten roots × two contracts, both tails for the lower.
//! FH1b record parity (Proposition FH-cut): on the viewer-lead trick-4
//!      roots the engine's `U_{a,0}` equals `horizon_census(cut 4)`'s
//!      per-action cut reading and `U_{a,1}` the cut-8 reading, computed
//!      live; and the companion's Q6 quotations of `horizon_run1.txt`
//!      are reproduced byte-for-byte as rationals.
//! FH2  nesting: `L_{a,k} ≤ L_{a,k+1} ≤ Q_a ≤ U_{a,k+1} ≤ U_{a,k}` for
//!      every action and consecutive k, both tails; at least one strict
//!      rise and one strict fall on the corpus.
//! FH3  exact collapse (Proposition FH-last): at t6 roots k = 0, t5
//!      roots k = 1, t4 roots k = 2, `L = Q = U` for every action and
//!      every tail consultation is at a FORCED node; at k = h_f = 7 − T
//!      tail consultations are exactly 0 (FH-A6's mechanical form);
//!      `h_f` after the root action is exactly 7 − T at undecided roots
//!      (0 at decided ones) and ≤ the viewer's tiles;
//!      collapse is NOT reached one layer earlier on at least one
//!      coordinate per trick; intervals are constant beyond `h_f`.
//! FH4  action containment and survivor monotonicity: `L ≤ Q ≤ U`; the
//!      exact argmax set ⊆ `S_k`; `S_{k+1} ⊆ S_k`; a `Settled{b}` has `b`
//!      the unique exact maximizer; an `Equivalent` lists exactly the
//!      exact maximizers (Proposition FH-tie).
//! FH5  executable lower witness — the lower-side no-strategy-fusion
//!      gate: `π_k` replayed through `viewer_success_mass` equals `B_k`
//!      at the root and `L_{a,k}` at every root child, both tails; `Γ_k ≥
//!      0` and `Q* − L_exec ≤ Γ_k`; under the σ0 tail an off-DAG state
//!      returns σ0's choice, not the lowest tile.
//! FH6  merge before max: a test-local FUSED k = 1 upper (per-world max
//!      over the next focal action, then sum) is ≥ the engine's `U_{a,1}`
//!      everywhere and strictly above on a specimen, and equals `U_{a,0}`
//!      (it IS the world-revealed value); the engine's `U_{a,1}` equals the
//!      salvation-mask upper computed test-locally (Theorem 5).
//! FH-A8 the anchor laws: at h8-t4 bids 36/39 the k = 1 verdict is not
//!      `Settled{2-1}` because `U_{5-5,1} > Q_{2-1}`; k = 2 settles 2-1.
//! FH-R refusal shape: a tiny cap yields one typed whole-root refusal
//!      naming history/fiber/cap at a viewer node; the ample cap completes.
//! FH-D determinism.
//!
//! EXPLORATORY tier throughout.

#[path = "common/fixture.rs"]
mod fixture;

use std::cmp::Reverse;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, LazyLock};

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::{legal_plays, Domino, DominoSet, Seat, Trick};
use walt::solver::adaptive::{
    decided_success, CanonicalRoot, FixedPreference, PublicRecord, RootPosition, SlicePolicy,
};
use walt::solver::doom::{doom_enumeration, DoomEnumeration, DoomSpec};
use walt::solver::factor_belief::{
    response_success_mass, viewer_success_mass, ExactCoverOracle, FactorBelief, RecursionStats,
    ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::focal_horizon::{
    focal_depth, focal_horizon, FocalHorizonResult, FocalRefusal, FocalSpec, FocalVerdict,
};
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

const T4: [(usize, usize); 4] = [(3, 4), (4, 4), (8, 4), (12, 4)];
const T56: [(usize, usize); 6] = [(8, 5), (3, 5), (12, 6), (10, 6), (5, 6), (4, 6)];
const AMPLE_CAP: u128 = 40_000;

/// The two contracts of every root: the receipt's and 36 (deduplicated).
fn contracts(position: &RootPosition) -> Vec<u32> {
    let mut out = vec![position.bid];
    if position.bid != 36 {
        out.push(36);
    }
    out
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Tail {
    Sigma0,
    Lowest,
}

const TAILS: [Tail; 2] = [Tail::Sigma0, Tail::Lowest];

fn lowest() -> FixedPreference {
    FixedPreference::lowest_first("focal:lowest-first")
}

type Key = (usize, usize, u32, usize, Tail);
type QKey = (usize, usize, u32);
type QTable = Vec<(Domino, u128)>;

// ---------------------------------------------------------------------------
// The recompute-once fixture (BRIEF-CI1): every expensive value the gates
// of this binary read, computed once per test-binary process — the engine
// per (root, contract, k, tail) under the ample cap, the exact `Q_a` per
// (root, contract), FH1b's record-parity censuses per (root, contract,
// cut), FH1's independent endpoints per (root, contract, action), FH3's
// independent depth walk per (root, contract, action) and FH5's replays
// per engine coordinate. A derived view of the declared
// epoch, immutable after construction; a key the fixture lacks is a
// plumbing error, never a silent recomputation. FH-D's and FH-R's fresh
// engine runs stay fresh.
// ---------------------------------------------------------------------------

/// FH1's independent endpoints at one root action: the σ0 and
/// lowest-first `viewer_success_mass` lowers and `doom_enumeration`'s
/// per-world truth.
struct Endpoint {
    action: Domino,
    l_sigma: u128,
    l_low: u128,
    doom: DoomEnumeration,
}

/// FH5's replays at one engine coordinate: `π_k` through
/// `viewer_success_mass` at the root and at every root child.
struct Replay {
    root: u128,
    per_action: Vec<(Domino, u128)>,
}

struct Fixture {
    engine: HashMap<Key, Arc<FocalHorizonResult>>,
    exact_q: HashMap<QKey, Arc<QTable>>,
    censuses: HashMap<(usize, usize, u32, usize), HorizonCensus>,
    endpoints: HashMap<QKey, Vec<Endpoint>>,
    depths: HashMap<(usize, usize, u32, Domino), usize>,
    replays: HashMap<Key, Replay>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Job {
    Engine(Key),
    Q(QKey),
    Census(usize, usize, u32, usize),
    Endpoints(QKey),
    Depth(QKey, Domino),
}

enum Value {
    Engine(Arc<FocalHorizonResult>),
    Q(QTable),
    Census(HorizonCensus),
    Endpoints(Vec<Endpoint>),
    Depth(usize),
}

/// The engine at one coordinate under the ample cap (an affordable root
/// completes; a refusal here is a gate failure).
fn engine_at(
    r: &Receipt,
    hand_id: usize,
    trick_no: usize,
    contract: u32,
    k: usize,
    tail: Tail,
) -> FocalHorizonResult {
    let (root, position) = root_at(r, hand_id, trick_no);
    let position = with_contract(&position, contract);
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec());
    let low = lowest();
    let tail_policy: &dyn SlicePolicy = match tail {
        Tail::Sigma0 => &field,
        Tail::Lowest => &low,
    };
    let spec = FocalSpec {
        horizon: k,
        node_fiber_cap: AMPLE_CAP,
    };
    focal_horizon(&oracle, &root, &position, tail_policy, &field, &spec)
        .unwrap_or_else(|e| panic!("an affordable root completes under the ample cap: {e:?}"))
}

/// The exact `Q_a` per root action by `response_success_mass`.
fn exact_q_at(r: &Receipt, hand_id: usize, trick_no: usize, contract: u32) -> QTable {
    let (root, position) = root_at(r, hand_id, trick_no);
    let position = with_contract(&position, contract);
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec());
    let belief = FactorBelief::uniform_root(&root, &position, &field);
    let mut out = Vec::new();
    for a in legal_actions(&root, &position) {
        let mut rs = ResponseStats::default();
        out.push((
            a,
            response_success_mass(&oracle, &belief.focal_play(a), &field, &mut rs),
        ));
    }
    out
}

fn census_at(
    r: &Receipt,
    hand_id: usize,
    trick_no: usize,
    contract: u32,
    cut: usize,
) -> HorizonCensus {
    let (root, position) = root_at(r, hand_id, trick_no);
    let position = with_contract(&position, contract);
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec());
    horizon_census(
        &oracle,
        &root,
        &position,
        &field,
        &HorizonSpec {
            cut_plays: cut,
            node_fiber_cap: AMPLE_CAP,
        },
    )
}

fn endpoints_at(r: &Receipt, hand_id: usize, trick_no: usize, contract: u32) -> Vec<Endpoint> {
    let dspec = DoomSpec {
        node_budget: 100_000_000,
        walk_cap: 10_000_000,
        max_level: 3,
        critical: DominoSet::EMPTY,
        descend_top: None,
    };
    let (root, position) = root_at(r, hand_id, trick_no);
    let position = with_contract(&position, contract);
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec());
    let low = lowest();
    let belief = FactorBelief::uniform_root(&root, &position, &field);
    let mut out = Vec::new();
    for a in legal_actions(&root, &position) {
        let child = belief.focal_play(a);
        let mut rs = RecursionStats::default();
        let l_sigma = viewer_success_mass(&oracle, &child, &field, &field, &mut rs);
        let mut rs = RecursionStats::default();
        let l_low = viewer_success_mass(&oracle, &child, &low, &field, &mut rs);
        let mut progress = |_: u64, _: u64, _: u128, _: u64| {};
        let doom = doom_enumeration(&oracle, &root, &position, &field, a, &dspec, &mut progress);
        out.push(Endpoint {
            action: a,
            l_sigma,
            l_low,
            doom,
        });
    }
    out
}

/// FH3's independent depth walk after one root action.
fn depth_at(r: &Receipt, hand_id: usize, trick_no: usize, contract: u32, action: Domino) -> usize {
    let (root, position) = root_at(r, hand_id, trick_no);
    let position = with_contract(&position, contract);
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec());
    let belief = FactorBelief::uniform_root(&root, &position, &field);
    focal_depth(&oracle, &belief.focal_play(action), &field)
}

fn replay_at(r: &Receipt, key: Key, res: &FocalHorizonResult) -> Replay {
    let (hand_id, trick_no, contract, _, tail) = key;
    let (root, position) = root_at(r, hand_id, trick_no);
    let position = with_contract(&position, contract);
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec());
    let low = lowest();
    let tail_policy: &dyn SlicePolicy = match tail {
        Tail::Sigma0 => &field,
        Tail::Lowest => &low,
    };
    let belief = FactorBelief::uniform_root(&root, &position, &field);
    let pi_k = res.policy.with_tail(tail_policy);
    let mut rs = RecursionStats::default();
    let root_value = viewer_success_mass(&oracle, &belief, &pi_k, &field, &mut rs);
    let mut per_action = Vec::new();
    for i in &res.actions {
        let mut rs = RecursionStats::default();
        let v = viewer_success_mass(
            &oracle,
            &belief.focal_play(i.action),
            &pi_k,
            &field,
            &mut rs,
        );
        per_action.push((i.action, v));
    }
    Replay {
        root: root_value,
        per_action,
    }
}

/// The core engine coordinates: every root and contract, both tails,
/// k ∈ {0, 1, 2} (FH2, FH4, FH5).
fn core_keys(r: &Receipt) -> Vec<Key> {
    let mut keys = Vec::new();
    for (hand_id, trick_no) in corpus() {
        let (_, base) = root_at(r, hand_id, trick_no);
        for contract in contracts(&base) {
            for tail in TAILS {
                for k in [0usize, 1, 2] {
                    keys.push((hand_id, trick_no, contract, k, tail));
                }
            }
        }
    }
    keys
}

/// Every key the gates read, heaviest first.
fn fixture_jobs(r: &Receipt) -> Vec<Job> {
    let mut jobs: Vec<Job> = Vec::new();
    let mut push = |job: Job| {
        if !jobs.contains(&job) {
            jobs.push(job);
        }
    };
    for key in core_keys(r) {
        push(Job::Engine(key));
    }
    for (hand_id, trick_no) in corpus() {
        let (root, base) = root_at(r, hand_id, trick_no);
        let h_f = 7 - trick_no;
        for contract in contracts(&base) {
            push(Job::Q((hand_id, trick_no, contract)));
            push(Job::Endpoints((hand_id, trick_no, contract)));
            for a in legal_actions(&root, &with_contract(&base, contract)) {
                push(Job::Depth((hand_id, trick_no, contract), a));
            }
            // FH3's coordinates at k = h_f (trick-4 roots under σ0 only)
            // and one beyond it (trick-5 roots, both tails).
            if trick_no == 4 {
                push(Job::Engine((
                    hand_id,
                    trick_no,
                    contract,
                    h_f,
                    Tail::Sigma0,
                )));
            }
            if trick_no == 5 {
                for tail in TAILS {
                    push(Job::Engine((hand_id, trick_no, contract, h_f + 1, tail)));
                }
            }
        }
    }
    // FH1b's record-parity censuses on the viewer-lead trick-4 roots.
    for (hand_id, trick_no) in [(3usize, 4usize), (4, 4), (8, 4)] {
        let (_, base) = root_at(r, hand_id, trick_no);
        for contract in contracts(&base) {
            for cut in [4usize, 8] {
                push(Job::Census(hand_id, trick_no, contract, cut));
            }
        }
    }
    // The anchors at contracts outside a root's own pair (FH1b, FH6, FH-A8).
    for contract in [36u32, 39] {
        push(Job::Q((8, 4, contract)));
        for tail in TAILS {
            for k in [0usize, 1, 2] {
                push(Job::Engine((8, 4, contract, k, tail)));
            }
        }
    }
    push(Job::Engine((4, 4, 39, 0, Tail::Sigma0)));
    // Heaviest first — the largest root fibers, then the kinds measured
    // costliest at one root (the deep-horizon engines and the cut-8
    // censuses) — so the makespan approaches the longest single job.
    let mut fiber: HashMap<(usize, usize), u128> = HashMap::new();
    for (hand_id, trick_no) in corpus() {
        let (root, position) = root_at(r, hand_id, trick_no);
        let field = FieldModel::new(field_spec());
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        fiber.insert((hand_id, trick_no), SupportOracle.mass(&belief));
    }
    let root_of = |job: &Job| match *job {
        Job::Engine((hand_id, trick_no, ..)) => (hand_id, trick_no),
        Job::Census(hand_id, trick_no, ..) => (hand_id, trick_no),
        Job::Endpoints((hand_id, trick_no, _)) | Job::Q((hand_id, trick_no, _)) => {
            (hand_id, trick_no)
        }
        Job::Depth((hand_id, trick_no, _), _) => (hand_id, trick_no),
    };
    let kind = |job: &Job| match *job {
        Job::Engine((.., k, _)) if k >= 2 => 30,
        Job::Engine((.., 1, _)) | Job::Census(.., 8) => 25,
        Job::Q(_) | Job::Census(..) => 20,
        Job::Endpoints(_) | Job::Engine(_) => 15,
        Job::Depth(..) => 5,
    };
    jobs.sort_by_key(|job| (Reverse(fiber[&root_of(job)]), Reverse(kind(job))));
    jobs
}

fn build_fixture() -> Fixture {
    let r = receipt();
    let jobs = fixture_jobs(&r);
    let values = fixture::compute_all(&jobs, |job| match *job {
        Job::Engine((hand_id, trick_no, contract, k, tail)) => Value::Engine(Arc::new(engine_at(
            &r, hand_id, trick_no, contract, k, tail,
        ))),
        Job::Q((hand_id, trick_no, contract)) => {
            Value::Q(exact_q_at(&r, hand_id, trick_no, contract))
        }
        Job::Census(hand_id, trick_no, contract, cut) => {
            Value::Census(census_at(&r, hand_id, trick_no, contract, cut))
        }
        Job::Endpoints((hand_id, trick_no, contract)) => {
            Value::Endpoints(endpoints_at(&r, hand_id, trick_no, contract))
        }
        Job::Depth((hand_id, trick_no, contract), action) => {
            Value::Depth(depth_at(&r, hand_id, trick_no, contract, action))
        }
    });
    let mut fixture = Fixture {
        engine: HashMap::new(),
        exact_q: HashMap::new(),
        censuses: HashMap::new(),
        endpoints: HashMap::new(),
        depths: HashMap::new(),
        replays: HashMap::new(),
    };
    for (job, value) in jobs.into_iter().zip(values) {
        match (job, value) {
            (Job::Engine(key), Value::Engine(res)) => {
                fixture.engine.insert(key, res);
            }
            (Job::Q(key), Value::Q(q)) => {
                fixture.exact_q.insert(key, Arc::new(q));
            }
            (Job::Census(hand_id, trick_no, contract, cut), Value::Census(c)) => {
                fixture
                    .censuses
                    .insert((hand_id, trick_no, contract, cut), c);
            }
            (Job::Endpoints(key), Value::Endpoints(e)) => {
                fixture.endpoints.insert(key, e);
            }
            (Job::Depth((hand_id, trick_no, contract), action), Value::Depth(d)) => {
                fixture
                    .depths
                    .insert((hand_id, trick_no, contract, action), d);
            }
            _ => unreachable!("a job's value is of the job's kind"),
        }
    }
    // The replays read the engine's policies, so they follow.
    let keys = core_keys(&r);
    let replays = fixture::compute_all(&keys, |key| replay_at(&r, *key, &fixture.engine[key]));
    for (key, replay) in keys.into_iter().zip(replays) {
        fixture.replays.insert(key, replay);
    }
    fixture
}

static FIXTURE: LazyLock<Fixture> = LazyLock::new(build_fixture);

/// The engine at one coordinate under the ample cap.
fn fh(
    _r: &Receipt,
    hand_id: usize,
    trick_no: usize,
    contract: u32,
    k: usize,
    tail: Tail,
) -> Arc<FocalHorizonResult> {
    FIXTURE
        .engine
        .get(&(hand_id, trick_no, contract, k, tail))
        .cloned()
        .unwrap_or_else(|| {
            panic!(
                "the fixture holds no engine result at h{hand_id}-t{trick_no} {contract} k={k} {tail:?}"
            )
        })
}

/// The exact `Q_a` per root action by `response_success_mass`.
fn exact_q(
    _r: &Receipt,
    hand_id: usize,
    trick_no: usize,
    contract: u32,
) -> Arc<Vec<(Domino, u128)>> {
    FIXTURE
        .exact_q
        .get(&(hand_id, trick_no, contract))
        .cloned()
        .unwrap_or_else(|| {
            panic!("the fixture holds no exact Q at h{hand_id}-t{trick_no} {contract}")
        })
}

/// The census at one trick-4 root, contract and cut under the ample cap.
fn census(hand_id: usize, trick_no: usize, contract: u32, cut: usize) -> &'static HorizonCensus {
    FIXTURE
        .censuses
        .get(&(hand_id, trick_no, contract, cut))
        .unwrap_or_else(|| {
            panic!("the fixture holds no census at h{hand_id}-t{trick_no} {contract} cut {cut}")
        })
}

/// FH1's independent endpoints at one root action.
fn endpoint(hand_id: usize, trick_no: usize, contract: u32, action: Domino) -> &'static Endpoint {
    FIXTURE
        .endpoints
        .get(&(hand_id, trick_no, contract))
        .and_then(|e| e.iter().find(|e| e.action == action))
        .unwrap_or_else(|| {
            panic!("the fixture holds no endpoints at h{hand_id}-t{trick_no} {contract} {action}")
        })
}

/// FH3's independent depth walk after one root action.
fn depth(hand_id: usize, trick_no: usize, contract: u32, action: Domino) -> usize {
    *FIXTURE
        .depths
        .get(&(hand_id, trick_no, contract, action))
        .unwrap_or_else(|| {
            panic!("the fixture holds no depth at h{hand_id}-t{trick_no} {contract} {action}")
        })
}

/// FH5's replays at one engine coordinate.
fn replay(hand_id: usize, trick_no: usize, contract: u32, k: usize, tail: Tail) -> &'static Replay {
    FIXTURE
        .replays
        .get(&(hand_id, trick_no, contract, k, tail))
        .unwrap_or_else(|| {
            panic!(
                "the fixture holds no replay at h{hand_id}-t{trick_no} {contract} k={k} {tail:?}"
            )
        })
}

/// A tile by its printed name.
fn tile(name: &str) -> Domino {
    (0..28)
        .filter_map(Domino::from_index)
        .find(|d| format!("{d}") == name)
        .unwrap_or_else(|| panic!("tile {name}"))
}

fn q_of(q: &[(Domino, u128)], a: Domino) -> u128 {
    q.iter()
        .find(|(t, _)| *t == a)
        .expect("every root action has a Q")
        .1
}

fn exact_argmax_set(q: &[(Domino, u128)]) -> Vec<Domino> {
    let best = q.iter().map(|(_, m)| *m).max().expect("an action");
    q.iter()
        .filter(|(_, m)| *m == best)
        .map(|(t, _)| *t)
        .collect()
}

fn corpus() -> Vec<(usize, usize)> {
    T4.iter().chain(T56.iter()).copied().collect()
}

fn ratio(m: u128, z: u128) -> BigRational {
    BigRational::new(BigInt::from(m), BigInt::from(z))
}

#[test]
fn fh1_endpoint_parity_with_the_independent_tails() {
    let r = receipt();
    let oracle = SupportOracle;
    let mut coordinates = 0usize;
    for (hand_id, trick_no) in corpus() {
        let (root, base) = root_at(&r, hand_id, trick_no);
        for contract in contracts(&base) {
            let position = with_contract(&base, contract);
            let field = FieldModel::new(field_spec());
            let low = lowest();
            let belief = FactorBelief::uniform_root(&root, &position, &field);
            let z = oracle.mass(&belief);
            let s0 = fh(&r, hand_id, trick_no, contract, 0, Tail::Sigma0);
            let lf = fh(&r, hand_id, trick_no, contract, 0, Tail::Lowest);
            assert_eq!(s0.identity.contract, contract);
            assert_eq!(s0.identity.tail_id, field.id());
            assert_eq!(lf.identity.tail_id, low.id());
            assert_eq!(s0.identity.horizon, 0);
            for a in legal_actions(&root, &position) {
                let endpoint = endpoint(hand_id, trick_no, contract, a);
                let (l_sigma, l_low, e) = (endpoint.l_sigma, endpoint.l_low, &endpoint.doom);
                assert_eq!(e.fiber, z);
                let i_s = s0.interval(a).expect("every root action has an interval");
                let i_l = lf.interval(a).expect("every root action has an interval");
                assert_eq!(i_s.root_mass, z);
                assert_eq!(
                    i_s.lower_mass, l_sigma,
                    "FH1: L_{{a,0}} under the σ0 tail IS viewer_success_mass (h{hand_id}-t{trick_no} {contract} {a})"
                );
                assert_eq!(
                    i_l.lower_mass, l_low,
                    "FH1: L_{{a,0}} under the lowest-first tail IS viewer_success_mass (h{hand_id}-t{trick_no} {contract} {a})"
                );
                assert_eq!(
                    i_s.upper_mass,
                    z - e.doomed,
                    "FH1: U_{{a,0}} IS Z − doomed by doom_enumeration's per-world truth (h{hand_id}-t{trick_no} {contract} {a})"
                );
                assert_eq!(
                    i_l.upper_mass, i_s.upper_mass,
                    "FH1: the upper tail does not depend on the lower tail"
                );
                assert_eq!(i_s.upper, e.upper, "FH1: the same God upper as a rational");
                assert_eq!(i_s.lower, ratio(l_sigma, z));
                coordinates += 1;
            }
        }
    }
    assert!(coordinates >= 40, "FH1: a real sweep, got {coordinates}");
}

#[test]
fn fh1b_record_parity_cut4_is_u0_and_cut8_is_u1() {
    let r = receipt();
    let mut checked = 0usize;
    for (hand_id, trick_no) in [(3usize, 4usize), (4, 4), (8, 4)] {
        let (_, base) = root_at(&r, hand_id, trick_no);
        for contract in contracts(&base) {
            let u0 = fh(&r, hand_id, trick_no, contract, 0, Tail::Sigma0);
            let u1 = fh(&r, hand_id, trick_no, contract, 1, Tail::Sigma0);
            for (cut, res) in [(4usize, &u0), (8usize, &u1)] {
                let c = census(hand_id, trick_no, contract, cut);
                assert_eq!(c.refused(), 0);
                for a in &c.actions {
                    let i = res.interval(a.action).expect("interval");
                    assert_eq!(
                        Some(i.upper_mass),
                        a.cut_mass,
                        "FH1b: cut {cut} IS U_{{a,{}}} (h{hand_id}-t{trick_no} {contract} {})",
                        cut / 4 - 1,
                        a.action
                    );
                    checked += 1;
                }
                assert_eq!(c.actions.len(), res.actions.len());
            }
        }
    }
    assert!(checked >= 40, "FH1b: real coordinates, got {checked}");
    // The companion's Q6 quotations of horizon_run1.txt, as rationals.
    let u1 = fh(&r, 8, 4, 36, 1, Tail::Sigma0);
    let want: [(&str, u128, u128); 4] = [
        ("2-1", 451, 600),
        ("3-1", 181, 300),
        ("3-3", 901, 1200),
        ("5-5", 303, 400),
    ];
    for (name, n, d) in want {
        let i = u1
            .actions
            .iter()
            .find(|i| format!("{}", i.action) == name)
            .expect("the anchor action");
        assert_eq!(
            i.upper,
            ratio(n, d),
            "FH1b: h8-t4 bid 36 cut-8 record value {name}"
        );
    }
    let u0 = fh(&r, 4, 4, 39, 0, Tail::Sigma0);
    let want: [(&str, u128, u128); 4] = [
        ("2-1", 1369, 2310),
        ("4-0", 11353, 17325),
        ("5-1", 10244, 17325),
        ("6-5", 1165, 1386),
    ];
    for (name, n, d) in want {
        let i = u0
            .actions
            .iter()
            .find(|i| format!("{}", i.action) == name)
            .expect("the anchor action");
        assert_eq!(
            i.upper,
            ratio(n, d),
            "FH1b: h4-t4 bid 39 cut-4 record value {name}"
        );
    }
    assert_eq!(u0.root_mass(), 34_650);
    assert_eq!(u1.root_mass(), 1_200);
}

#[test]
fn fh2_nesting_with_strict_movement_somewhere() {
    let r = receipt();
    let mut strict_l = 0usize;
    let mut strict_u = 0usize;
    let mut checked = 0usize;
    for (hand_id, trick_no) in corpus() {
        let (_, base) = root_at(&r, hand_id, trick_no);
        for contract in contracts(&base) {
            let q = exact_q(&r, hand_id, trick_no, contract);
            for tail in TAILS {
                for k in [0usize, 1] {
                    let lo = fh(&r, hand_id, trick_no, contract, k, tail);
                    let hi = fh(&r, hand_id, trick_no, contract, k + 1, tail);
                    for i in &lo.actions {
                        let j = hi.interval(i.action).expect("the same actions");
                        let qa = q_of(&q, i.action);
                        assert!(
                            i.lower_mass <= j.lower_mass
                                && j.lower_mass <= qa
                                && qa <= j.upper_mass
                                && j.upper_mass <= i.upper_mass,
                            "FH2: L_k ≤ L_k+1 ≤ Q ≤ U_k+1 ≤ U_k fails at h{hand_id}-t{trick_no} {contract} {tail:?} k={k} {}: \
                             {} ≤ {} ≤ {qa} ≤ {} ≤ {}",
                            i.action,
                            i.lower_mass,
                            j.lower_mass,
                            j.upper_mass,
                            i.upper_mass
                        );
                        if j.lower_mass > i.lower_mass {
                            strict_l += 1;
                        }
                        if j.upper_mass < i.upper_mass {
                            strict_u += 1;
                        }
                        checked += 1;
                    }
                    assert!(hi.bar_mass >= lo.bar_mass, "FH2: the bar never falls");
                    assert!(
                        hi.global_upper_mass <= lo.global_upper_mass,
                        "FH2: the global upper never rises"
                    );
                    assert!(
                        hi.certified_regret <= lo.certified_regret,
                        "FH2: Γ_k+1 ≤ Γ_k under deterministic exact tails (§19)"
                    );
                }
            }
        }
    }
    assert!(checked >= 100, "FH2: a real sweep, got {checked}");
    assert!(
        strict_l >= 1,
        "FH2: no strict lower rise anywhere — the gate would prove nothing"
    );
    assert!(
        strict_u >= 1,
        "FH2: no strict upper fall anywhere — the gate would prove nothing"
    );
}

#[test]
fn fh3_exact_collapse_one_layer_before_the_forced_trick() {
    let r = receipt();
    let oracle = SupportOracle;
    let mut not_collapsed_earlier: BTreeMap<usize, usize> = BTreeMap::new();
    let mut depth_equalities: BTreeMap<usize, usize> = BTreeMap::new();
    for (hand_id, trick_no) in corpus() {
        let (root, base) = root_at(&r, hand_id, trick_no);
        let collapse_k = 6 - trick_no;
        let h_f_root_child = 7 - trick_no;
        for contract in contracts(&base) {
            let position = with_contract(&base, contract);
            let field = FieldModel::new(field_spec());
            let belief = FactorBelief::uniform_root(&root, &position, &field);
            let q = exact_q(&r, hand_id, trick_no, contract);
            let viewer_tiles_after = root.kernel().viewer_hand().len() - 1;
            // (a) the independent depth walk.
            let mut max_depth = 0usize;
            for a in legal_actions(&root, &position) {
                let d = depth(hand_id, trick_no, contract, a);
                assert!(
                    d <= h_f_root_child,
                    "FH3: h_f after the root action ≤ 7 − T (h{hand_id}-t{trick_no} {contract} {a}: {d})"
                );
                assert!(
                    d <= viewer_tiles_after,
                    "FH3: h_f ≤ the viewer's remaining tiles"
                );
                max_depth = max_depth.max(d);
            }
            let root_decided =
                decided_success(&position, root.kernel().viewer(), position.banked, false)
                    .is_some();
            if root_decided {
                assert_eq!(
                    max_depth, 0,
                    "FH3: a decided root's children have h_f = 0 with plays remaining"
                );
            } else {
                assert_eq!(
                    max_depth, h_f_root_child,
                    "FH3: h_f = 7 − T after the root action at an undecided viewer-lead trick-T \
                     root (h{hand_id}-t{trick_no} {contract})"
                );
            }
            if trick_no >= 5 || root_decided {
                let root_depth = focal_depth(&oracle, &belief, &field);
                if root_decided {
                    assert_eq!(root_depth, 0, "FH3: a decided root has h_f = 0");
                } else {
                    assert_eq!(
                        root_depth,
                        1 + max_depth,
                        "FH3: the root is one focal layer above its children"
                    );
                }
            }
            if max_depth == h_f_root_child {
                *depth_equalities.entry(trick_no).or_insert(0) += 1;
            }
            for tail in TAILS {
                // (b) collapse at k = 6 − T (FH-last), every consultation forced.
                let res = fh(&r, hand_id, trick_no, contract, collapse_k, tail);
                for i in &res.actions {
                    let qa = q_of(&q, i.action);
                    assert!(
                        i.lower_mass == qa && i.upper_mass == qa,
                        "FH3: L = Q = U at k = 6 − T (h{hand_id}-t{trick_no} {contract} {tail:?} {}: [{}, {}] vs {qa})",
                        i.action,
                        i.lower_mass,
                        i.upper_mass
                    );
                }
                assert_eq!(
                    res.spend.tail_consultations(),
                    res.spend.lower_tail_evaluations + res.spend.upper_tail_evaluations
                );
                assert_eq!(
                    res.spend.lower_tail_evaluations, res.spend.forced_tail_evaluations,
                    "FH3: at k = 6 − T every tail consultation is at a FORCED node (h{hand_id}-t{trick_no} {contract} {tail:?})"
                );
                assert!(matches!(
                    res.verdict,
                    FocalVerdict::Settled { .. } | FocalVerdict::Equivalent { .. }
                ));
                // (d) the depth is load-bearing: one layer earlier is not a collapse somewhere.
                if collapse_k >= 1 {
                    let earlier = fh(&r, hand_id, trick_no, contract, collapse_k - 1, tail);
                    if earlier.actions.iter().any(|i| !i.collapsed()) {
                        *not_collapsed_earlier.entry(trick_no).or_insert(0) += 1;
                    }
                }
                // (c) the mechanical zero at k = h_f (FH-A6), and constancy beyond.
                // At k ≥ h_f no tail is consulted, so the coordinate is
                // tail-independent; the trick-4 roots run it under σ0 only.
                if trick_no == 4 && tail == Tail::Lowest {
                    continue;
                }
                let at_hf = fh(&r, hand_id, trick_no, contract, h_f_root_child, tail);
                assert_eq!(
                    at_hf.spend.tail_consultations(),
                    0,
                    "FH3: tail consultations = 0 whenever k ≥ h_f (h{hand_id}-t{trick_no} {contract} {tail:?} k={h_f_root_child})"
                );
                assert_eq!(
                    at_hf.actions, res.actions,
                    "FH3: intervals constant from the collapse on"
                );
                if trick_no >= 5 {
                    let beyond = fh(&r, hand_id, trick_no, contract, h_f_root_child + 1, tail);
                    assert_eq!(
                        beyond.actions, at_hf.actions,
                        "FH3: intervals constant beyond h_f"
                    );
                    assert_eq!(beyond.spend.tail_consultations(), 0);
                }
            }
        }
    }
    for t in [4usize, 5] {
        assert!(
            not_collapsed_earlier.get(&t).copied().unwrap_or(0) >= 1,
            "FH3: at trick {t} some coordinate is NOT collapsed one layer earlier"
        );
    }
    for t in [4usize, 5, 6] {
        assert!(
            depth_equalities.get(&t).copied().unwrap_or(0) >= 1,
            "FH3: h_f = 7 − T after the root action is attained at some trick-{t} root"
        );
    }
}

#[test]
fn fh4_action_containment_and_survivor_monotonicity() {
    let r = receipt();
    let mut settled = 0usize;
    let mut equivalent = 0usize;
    for (hand_id, trick_no) in corpus() {
        let (_, base) = root_at(&r, hand_id, trick_no);
        for contract in contracts(&base) {
            let q = exact_q(&r, hand_id, trick_no, contract);
            let argmax = exact_argmax_set(&q);
            let q_star = q.iter().map(|(_, m)| *m).max().expect("an action");
            for tail in TAILS {
                for k in [0usize, 1, 2] {
                    let res = fh(&r, hand_id, trick_no, contract, k, tail);
                    for i in &res.actions {
                        let qa = q_of(&q, i.action);
                        assert!(
                            i.lower_mass <= qa && qa <= i.upper_mass,
                            "FH4: L ≤ Q ≤ U (h{hand_id}-t{trick_no} {contract} {tail:?} k={k} {})",
                            i.action
                        );
                    }
                    for a in &argmax {
                        assert!(
                            res.survivors.contains(a),
                            "FH4: the exact best survives every horizon (h{hand_id}-t{trick_no} {contract} {tail:?} k={k} {a})"
                        );
                    }
                    assert!(
                        res.global_upper_mass >= q_star && res.bar_mass <= q_star,
                        "FH4: the root interval contains Q*"
                    );
                    if k < 2 {
                        let next = fh(&r, hand_id, trick_no, contract, k + 1, tail);
                        for a in &next.survivors {
                            assert!(res.survivors.contains(a), "FH4: S_k+1 ⊆ S_k");
                        }
                    }
                    match &res.verdict {
                        FocalVerdict::Settled { action } => {
                            assert_eq!(
                                argmax,
                                vec![*action],
                                "FH4: Settled names the unique exact maximizer (h{hand_id}-t{trick_no} {contract} {tail:?} k={k})"
                            );
                            settled += 1;
                        }
                        FocalVerdict::Equivalent {
                            actions,
                            value_mass,
                        } => {
                            assert_eq!(
                                *actions, argmax,
                                "FH4: Equivalent lists exactly the exact maximizers (FH-tie) (h{hand_id}-t{trick_no} {contract} {tail:?} k={k})"
                            );
                            assert_eq!(*value_mass, q_star, "FH4: Q* = B_k under FH-tie");
                            for a in &res.survivors {
                                assert!(res.interval(*a).expect("interval").collapsed());
                            }
                            equivalent += 1;
                        }
                        FocalVerdict::Unresolved { survivors } => {
                            assert_eq!(*survivors, res.survivors);
                            assert!(
                                res.survivors
                                    .iter()
                                    .any(|a| !res.interval(*a).expect("interval").collapsed()),
                                "FH4: Unresolved means some survivor is not collapsed"
                            );
                        }
                    }
                }
            }
        }
    }
    assert!(settled >= 1, "FH4: a Settled verdict was seen");
    assert!(equivalent >= 1, "FH4: an Equivalent verdict was seen");
}

#[test]
fn fh5_executable_lower_witness_and_off_dag_tail() {
    let r = receipt();
    let oracle = SupportOracle;
    let mut replays = 0usize;
    let mut discriminating = 0usize;
    for (hand_id, trick_no) in corpus() {
        let (root, base) = root_at(&r, hand_id, trick_no);
        for contract in contracts(&base) {
            let position = with_contract(&base, contract);
            let field = FieldModel::new(field_spec());
            let low = lowest();
            let belief = FactorBelief::uniform_root(&root, &position, &field);
            let q = exact_q(&r, hand_id, trick_no, contract);
            let q_star = q.iter().map(|(_, m)| *m).max().expect("an action");
            for tail in TAILS {
                let tail_policy: &dyn SlicePolicy = match tail {
                    Tail::Sigma0 => &field,
                    Tail::Lowest => &low,
                };
                for k in [0usize, 1, 2] {
                    let res = fh(&r, hand_id, trick_no, contract, k, tail);
                    let pi_k = res.policy.with_tail(tail_policy);
                    assert_eq!(pi_k.id(), res.policy.id());
                    let replayed = replay(hand_id, trick_no, contract, k, tail);
                    let v_root = replayed.root;
                    assert_eq!(
                        v_root, res.bar_mass,
                        "FH5: V(π_k) at the root IS B_k (h{hand_id}-t{trick_no} {contract} {tail:?} k={k})"
                    );
                    assert_eq!(res.executable_lower_mass, res.bar_mass);
                    assert_eq!(
                        res.interval(res.policy_action())
                            .expect("interval")
                            .lower_mass,
                        res.bar_mass,
                        "FH5: π_k plays a bar action"
                    );
                    for i in &res.actions {
                        let v = q_of(&replayed.per_action, i.action);
                        assert_eq!(
                            v, i.lower_mass,
                            "FH5: V(π_k) at every root child IS L_{{a,k}} (h{hand_id}-t{trick_no} {contract} {tail:?} k={k} {})",
                            i.action
                        );
                        replays += 1;
                    }
                    assert!(res.certified_regret >= BigRational::from_integer(BigInt::from(0)));
                    assert!(
                        q_star - res.executable_lower_mass
                            <= res.global_upper_mass - res.executable_lower_mass,
                        "FH5: Q* − L_exec ≤ Γ_k"
                    );
                    assert_eq!(
                        res.certified_regret,
                        ratio(
                            res.global_upper_mass - res.executable_lower_mass,
                            res.root_mass()
                        )
                    );
                }
            }
            // The off-DAG continuation under the σ0 tail: at k = 0 every
            // trick-(T+1) focal node is off the table, and the policy's
            // choice there is σ0's — discriminated from the lowest tile
            // wherever σ0 disagrees with it.
            let res = fh(&r, hand_id, trick_no, contract, 0, Tail::Sigma0);
            let pi_0 = res.policy.with_tail(&field);
            for a in legal_actions(&root, &position) {
                let mut node = belief.focal_play(a);
                // Walk the heaviest hidden branch until the viewer is to move.
                loop {
                    let state = node.public_state();
                    let seat = state.leader.plus(state.plays.len());
                    if seat == root.kernel().viewer() {
                        break;
                    }
                    let branches = oracle.branch_masses(&node, &field);
                    if branches.is_empty() {
                        break;
                    }
                    let heaviest = branches.iter().max_by_key(|(_, m)| *m).expect("branch").0;
                    node = oracle.condition(&node, heaviest, &field);
                }
                let state = node.public_state();
                let viewer = root.kernel().viewer();
                if state.leader.plus(state.plays.len()) != viewer {
                    continue;
                }
                assert!(
                    res.policy.choice_at(node.history()).is_none(),
                    "FH5: a k = 0 frontier node is off the table"
                );
                let remaining = root
                    .kernel()
                    .viewer_hand()
                    .difference(state.played_by[viewer.index()]);
                let led = state.plays.first().map(|d| position.decl.led_context(*d));
                let legal = legal_plays(position.decl, remaining, led);
                let record = PublicRecord {
                    leader: state.leader,
                    trick_plays: &state.plays,
                    banked: state.banked,
                    root: &position,
                    history: node.history(),
                };
                let chosen = pi_0.choose(position.decl, remaining, legal, &record);
                let sigma = field.choose(position.decl, remaining, legal, &record);
                assert_eq!(chosen, sigma, "FH5: off the DAG π_k IS σ0 (FH-A7)");
                let lowest_tile = legal.iter().next().expect("a legal tile");
                if sigma != lowest_tile {
                    discriminating += 1;
                }
            }
        }
    }
    assert!(replays >= 100, "FH5: a real sweep, got {replays}");
    assert!(
        discriminating >= 1,
        "FH5: some off-DAG state distinguishes σ0 from the lowest tile"
    );
}

// ---------------------------------------------------------------------------
// FH6 — a test-local per-world walker (the fused order, deliberately).
// ---------------------------------------------------------------------------

/// One world's full public-plus-private state for the test's own walk.
#[derive(Clone)]
struct World {
    leader: Seat,
    plays: Vec<Domino>,
    banked: [u32; 2],
    remaining: [DominoSet; 4],
    history: Vec<Domino>,
    played: usize,
}

impl World {
    fn seat(&self) -> Seat {
        self.leader.plus(self.plays.len())
    }

    fn play(&mut self, position: &RootPosition, tile: Domino) {
        let seat = self.seat();
        assert!(
            self.remaining[seat.index()].remove(tile),
            "a played tile leaves its hand"
        );
        self.plays.push(tile);
        self.history.push(tile);
        self.played += 1;
        if self.plays.len() == 4 {
            let doms: [Domino; 4] = core::array::from_fn(|i| self.plays[i]);
            let trick = Trick::new(self.leader, doms).expect("four distinct tiles");
            let winner = trick.winner(position.decl);
            self.banked[winner.team().index()] += trick.points();
            self.leader = winner;
            self.plays.clear();
        }
    }

    fn legal(&self, position: &RootPosition) -> DominoSet {
        let seat = self.seat();
        let led = self.plays.first().map(|d| position.decl.led_context(*d));
        legal_plays(position.decl, self.remaining[seat.index()], led)
    }

    fn field_tile(&self, position: &RootPosition, field: &dyn SlicePolicy) -> Domino {
        let seat = self.seat();
        let legal = self.legal(position);
        let record = PublicRecord {
            leader: self.leader,
            trick_plays: &self.plays,
            banked: self.banked,
            root: position,
            history: &self.history,
        };
        let t = field.choose(position.decl, self.remaining[seat.index()], legal, &record);
        assert!(legal.contains(t));
        t
    }
}

/// World-revealed make from a state: the viewer maximizes knowing the
/// world; every other seat plays the field on its known hand.
fn god_makes(
    w: &World,
    position: &RootPosition,
    viewer: Seat,
    total: usize,
    field: &dyn SlicePolicy,
) -> bool {
    if let Some(u) = decided_success(position, viewer, w.banked, w.played == total) {
        return u;
    }
    if w.seat() == viewer {
        for t in w.legal(position).iter() {
            let mut c = w.clone();
            c.play(position, t);
            if god_makes(&c, position, viewer, total, field) {
                return true;
            }
        }
        false
    } else {
        let mut c = w.clone();
        let t = w.field_tile(position, field);
        c.play(position, t);
        god_makes(&c, position, viewer, total, field)
    }
}

/// Every world of the uniform root, as per-seat root hands in kernel slot
/// order (the same triple loop the doom module runs; test-local).
fn worlds_of(belief: &FactorBelief) -> Vec<[DominoSet; 3]> {
    let supports: [Vec<(DominoSet, u128)>; 3] =
        core::array::from_fn(|i| belief.factors()[i].support());
    let pool = belief.kernel().pool();
    let mut out = Vec::new();
    for (h0, _) in &supports[0] {
        let rest = pool.difference(*h0);
        for (h1, _) in &supports[1] {
            if !h1.is_subset_of(rest) {
                continue;
            }
            let h2 = rest.difference(*h1);
            if supports[2].iter().any(|(h, _)| *h == h2) {
                out.push([*h0, *h1, h2]);
            }
        }
    }
    out
}

/// `(fused, mask)` for one root action: the per-world max over the next
/// focal action then summed (strategy fusion), and the salvation-mask
/// upper — worlds grouped by the public history at their next focal
/// node, the max taken over the group's common legal set of the number
/// of worlds individually salvageable after the common action.
fn fused_and_mask(
    root: &CanonicalRoot,
    position: &RootPosition,
    field: &dyn SlicePolicy,
    action: Domino,
) -> (u128, u128) {
    let belief = FactorBelief::uniform_root(root, position, field);
    let kernel = root.kernel();
    let viewer = kernel.viewer();
    let total =
        kernel.viewer_hand().len() + kernel.hidden().iter().map(|h| h.capacity).sum::<usize>();
    let mut fused: u128 = 0;
    let mut decided_mass: u128 = 0;
    // group key: public history at the next focal node → per next action: salvageable count.
    let mut groups: BTreeMap<Vec<u8>, BTreeMap<u8, u128>> = BTreeMap::new();
    for hands in worlds_of(&belief) {
        let mut remaining = [DominoSet::EMPTY; 4];
        remaining[viewer.index()] = kernel.viewer_hand();
        for (i, h) in kernel.hidden().iter().enumerate() {
            remaining[h.seat.index()] = hands[i];
        }
        let mut w = World {
            leader: position.leader,
            plays: position.trick_plays.clone(),
            banked: position.banked,
            remaining,
            history: Vec::new(),
            played: 0,
        };
        assert_eq!(w.seat(), viewer);
        w.play(position, action);
        // Advance the determined field plays to the next focal node.
        let decided = loop {
            if let Some(u) = decided_success(position, viewer, w.banked, w.played == total) {
                break Some(u);
            }
            if w.seat() == viewer {
                break None;
            }
            let t = w.field_tile(position, field);
            w.play(position, t);
        };
        match decided {
            Some(u) => {
                if u {
                    fused += 1;
                    decided_mass += 1;
                }
            }
            None => {
                let key: Vec<u8> = w
                    .history
                    .iter()
                    .map(|d| u8::try_from(d.index()).expect("index"))
                    .collect();
                let entry = groups.entry(key).or_default();
                let mut best = false;
                for t in w.legal(position).iter() {
                    let mut c = w.clone();
                    c.play(position, t);
                    let ok = god_makes(&c, position, viewer, total, field);
                    let slot = entry
                        .entry(u8::try_from(t.index()).expect("index"))
                        .or_insert(0);
                    if ok {
                        *slot += 1;
                    }
                    best |= ok;
                }
                if best {
                    fused += 1;
                }
            }
        }
    }
    let mut mask: u128 = decided_mass;
    for (_, per_action) in groups {
        mask += per_action.values().copied().max().unwrap_or(0);
    }
    (fused, mask)
}

#[test]
fn fh6_merge_before_max_and_the_salvation_mask_identity() {
    let r = receipt();
    let mut strict = 0usize;
    let mut checked = 0usize;
    for (hand_id, trick_no, contract) in
        [(8usize, 4usize, 30u32), (8, 4, 36), (8, 4, 39), (3, 4, 36)]
    {
        let (root, base) = root_at(&r, hand_id, trick_no);
        let position = with_contract(&base, contract);
        let field = FieldModel::new(field_spec());
        let u0 = fh(&r, hand_id, trick_no, contract, 0, Tail::Sigma0);
        let u1 = fh(&r, hand_id, trick_no, contract, 1, Tail::Sigma0);
        for a in legal_actions(&root, &position) {
            let (fused, mask) = fused_and_mask(&root, &position, &field, a);
            let i0 = u0.interval(a).expect("interval");
            let i1 = u1.interval(a).expect("interval");
            assert!(
                fused >= i1.upper_mass,
                "FH6: the fused order never sits below the lawful U_{{a,1}} (h{hand_id}-t{trick_no} {contract} {a})"
            );
            assert_eq!(
                fused, i0.upper_mass,
                "FH6: per-world max then sum IS the world-revealed value U_{{a,0}} (FH-God) (h{hand_id}-t{trick_no} {contract} {a})"
            );
            assert_eq!(
                mask, i1.upper_mass,
                "FH6: U_{{a,1}} IS the salvation-mask upper max_a' |S_a'| summed over public branches (Theorem 5) (h{hand_id}-t{trick_no} {contract} {a})"
            );
            if fused > i1.upper_mass {
                strict += 1;
            }
            checked += 1;
        }
    }
    assert!(checked >= 12);
    assert!(
        strict >= 1,
        "FH6: the fused order is STRICTLY optimistic on a specimen"
    );
}

#[test]
fn fh_a8_anchor_laws_at_h8_t4() {
    let r = receipt();
    let two_one = tile("2-1");
    let five_five = tile("5-5");
    for contract in [36u32, 39] {
        let q = exact_q(&r, 8, 4, contract);
        assert_eq!(exact_argmax_set(&q), vec![two_one]);
        assert_eq!(q_of(&q, two_one), 900, "Q_{{2-1}} = 3/4 of 1200");
        for tail in TAILS {
            let k1 = fh(&r, 8, 4, contract, 1, tail);
            let u55 = k1.interval(five_five).expect("5-5").upper_mass;
            assert_eq!(u55, 909, "U_{{5-5,1}} = 303/400 of 1200");
            assert!(u55 > q_of(&q, two_one));
            assert!(
                !matches!(k1.verdict, FocalVerdict::Settled { action } if action == two_one),
                "FH-A8: k = 1 cannot settle 2-1 at h8-t4 bid {contract} ({tail:?})"
            );
            assert!(k1.survivors.contains(&five_five));
            let k2 = fh(&r, 8, 4, contract, 2, tail);
            assert_eq!(
                k2.verdict,
                FocalVerdict::Settled { action: two_one },
                "FH-A8: k = 2 collapses and settles 2-1 at h8-t4 bid {contract} ({tail:?})"
            );
            assert!(k2.actions.iter().all(|i| i.collapsed()));
        }
    }
}

#[test]
fn fh_r_refusal_is_typed_whole_root_and_names_the_boundary() {
    let r = receipt();
    let (root, position) = root_at(&r, 8, 4);
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec());
    let tiny = focal_horizon(
        &oracle,
        &root,
        &position,
        &field,
        &field,
        &FocalSpec {
            horizon: 0,
            node_fiber_cap: 8,
        },
    );
    match tiny {
        Err(FocalRefusal::UpperUnaffordable {
            history,
            fiber,
            cap,
        }) => {
            assert_eq!(cap, 8);
            assert!(fiber > 8, "FH-R: the refusing node is above the cap");
            assert!(
                !history.is_empty(),
                "FH-R: the boundary is a post-root node"
            );
            // The named node is a viewer node of positive mass reached by
            // the engine's own descent: rebuild it and check.
            let belief = FactorBelief::uniform_root(&root, &position, &field);
            let mut node = belief.focal_play(history[0]);
            for t in &history[1..] {
                node = oracle.condition(&node, *t, &field);
            }
            assert_eq!(node.seat_to_move(), root.kernel().viewer());
            assert_eq!(
                oracle.mass(&node),
                fiber,
                "FH-R: the refusal names the node's exact fiber"
            );
        }
        Ok(_) => panic!("FH-R: a tiny cap refuses, never a partial result"),
    }
    let ample = fh(&r, 8, 4, position.bid, 0, Tail::Sigma0);
    assert_eq!(ample.actions.len(), 4);
    assert_eq!(ample.spec.node_fiber_cap, AMPLE_CAP);
}

#[test]
fn fh_d_two_runs_render_identically() {
    let r = receipt();
    let (root, position) = root_at(&r, 3, 4);
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec());
    let spec = FocalSpec {
        horizon: 1,
        node_fiber_cap: AMPLE_CAP,
    };
    // One fresh run against the fixture's.
    let a = focal_horizon(&oracle, &root, &position, &field, &field, &spec).expect("completes");
    let b = fh(&r, 3, 4, position.bid, 1, Tail::Sigma0).as_ref().clone();
    assert_eq!(a, b, "FH-D: deterministic");
    assert_eq!(format!("{a:?}"), format!("{b:?}"));
    assert_eq!(a.policy.id(), b.policy.id());
}
