//! Slice U0b — the in-solve horizon census. Five gates.
//!
//! H1 the frontier re-descent reproduces the root: `root_exact_mass`
//!     equals `response_success_mass` at the root, on every corpus root
//!     and contract, at two cut depths; and every priced node's `Q` is
//!     bounded by its God upper.
//! H2 the per-node doom agrees with the doom census's own per-world
//!     truth: at cut depth 1 the frontier nodes ARE U0's root-action
//!     coordinates, and their doomed counts equal `doom_enumeration`'s
//!     while their `Q` equals the exact response — so the census's world
//!     enumeration and line-state construction are the doom module's,
//!     checked rather than trusted.
//! H3 the cut never under-prices and is exact iff the frontier is: the
//!     root's cut value is at least its exact value; equal when every
//!     priced node is God-tight and nothing was refused; strictly above
//!     when some priced node carries a positive gap and the cut argmax
//!     path reaches it (asserted on a specimen).
//! H4 refusals are typed and nothing is dropped: under a tiny node cap
//!     every over-cap frontier node is `Refused`, the cut side is absent,
//!     the exact side is still complete, and the frontier count equals
//!     the uncapped run's.
//! H5 determinism: two censuses of one root render identically.
//!
//! EXPLORATORY tier throughout. The horizon is a measurement, never a
//! theorem (SC-A4).

#[path = "common/fixture.rs"]
mod fixture;

use std::collections::HashMap;
use std::sync::LazyLock;

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::DominoSet;
use walt::solver::adaptive::{CanonicalRoot, RootPosition};
use walt::solver::doom::{doom_enumeration, DoomSpec};
use walt::solver::factor_belief::{
    response_success_mass, FactorBelief, ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::godgap::legal_actions;
use walt::solver::horizon::{
    horizon_census, with_contract, HorizonCensus, HorizonSpec, NodeVerdict,
};
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

fn spec(cut: usize, cap: u128) -> HorizonSpec {
    HorizonSpec {
        cut_plays: cut,
        node_fiber_cap: cap,
    }
}

const T4: [(usize, usize); 4] = [(3, 4), (4, 4), (8, 4), (12, 4)];
const T56: [(usize, usize); 6] = [(8, 5), (3, 5), (12, 6), (10, 6), (5, 6), (4, 6)];

// ---------------------------------------------------------------------------
// The recompute-once fixture (BRIEF-CI1): every census the gates read,
// per (root, contract, cut) under the ample cap, and the root's
// `response_success_mass` per (root, contract) — the independent path
// H1 compares the re-descent to — computed once per test-binary process.
// A derived view of the declared epoch, immutable after construction.
// ---------------------------------------------------------------------------

struct Fixture {
    censuses: HashMap<(usize, usize, u32, usize), HorizonCensus>,
    root_masses: HashMap<(usize, usize, u32), u128>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Job {
    Census(usize, usize, u32, usize),
    RootMass(usize, usize, u32),
}

enum Value {
    Census(Box<HorizonCensus>),
    RootMass(u128),
}

fn build_fixture() -> Fixture {
    let r = receipt();
    // Heaviest first: the cut-4 censuses of the trick-4 roots.
    let mut jobs: Vec<Job> = Vec::new();
    let mut push = |job: Job| {
        if !jobs.contains(&job) {
            jobs.push(job);
        }
    };
    for cut in [4usize, 1] {
        for (hand_id, trick_no) in T4.iter().chain(T56.iter()).copied() {
            let (_, position) = root_at(&r, hand_id, trick_no);
            for contract in [position.bid, 36] {
                push(Job::Census(hand_id, trick_no, contract, cut));
            }
        }
    }
    for (hand_id, trick_no) in T4.iter().chain(T56.iter()).copied() {
        let (_, position) = root_at(&r, hand_id, trick_no);
        for contract in [position.bid, 36] {
            push(Job::RootMass(hand_id, trick_no, contract));
        }
    }
    let values = fixture::compute_all(&jobs, |job| {
        let oracle = SupportOracle;
        let field = FieldModel::new(field_spec());
        match *job {
            Job::Census(hand_id, trick_no, contract, cut) => {
                let (root, position) = root_at(&r, hand_id, trick_no);
                let position = with_contract(&position, contract);
                Value::Census(Box::new(horizon_census(
                    &oracle,
                    &root,
                    &position,
                    &field,
                    &spec(cut, 40_000),
                )))
            }
            Job::RootMass(hand_id, trick_no, contract) => {
                let (root, position) = root_at(&r, hand_id, trick_no);
                let position = with_contract(&position, contract);
                let belief = FactorBelief::uniform_root(&root, &position, &field);
                let mut rs = ResponseStats::default();
                Value::RootMass(response_success_mass(&oracle, &belief, &field, &mut rs))
            }
        }
    });
    let mut fixture = Fixture {
        censuses: HashMap::new(),
        root_masses: HashMap::new(),
    };
    for (job, value) in jobs.into_iter().zip(values) {
        match (job, value) {
            (Job::Census(hand_id, trick_no, contract, cut), Value::Census(c)) => {
                fixture
                    .censuses
                    .insert((hand_id, trick_no, contract, cut), *c);
            }
            (Job::RootMass(hand_id, trick_no, contract), Value::RootMass(m)) => {
                fixture.root_masses.insert((hand_id, trick_no, contract), m);
            }
            _ => unreachable!("a job's value is of the job's kind"),
        }
    }
    fixture
}

static FIXTURE: LazyLock<Fixture> = LazyLock::new(build_fixture);

/// The census at one root, contract and cut under the ample cap.
fn census(hand_id: usize, trick_no: usize, contract: u32, cut: usize) -> &'static HorizonCensus {
    FIXTURE
        .censuses
        .get(&(hand_id, trick_no, contract, cut))
        .unwrap_or_else(|| {
            panic!("the fixture holds no census at h{hand_id}-t{trick_no} contract {contract} cut {cut}")
        })
}

/// `response_success_mass` at one root and contract.
fn root_response_mass(hand_id: usize, trick_no: usize, contract: u32) -> u128 {
    *FIXTURE
        .root_masses
        .get(&(hand_id, trick_no, contract))
        .unwrap_or_else(|| {
            panic!("the fixture holds no root mass at h{hand_id}-t{trick_no} contract {contract}")
        })
}

#[test]
fn h1_the_frontier_re_descent_reproduces_the_root() {
    let r = receipt();
    let mut checked = 0usize;
    let mut priced = 0usize;
    for (hand_id, trick_no) in T4.iter().chain(T56.iter()).copied() {
        let (_, position) = root_at(&r, hand_id, trick_no);
        for contract in [position.bid, 36] {
            for cut in [1usize, 4] {
                let c = census(hand_id, trick_no, contract, cut);
                let independent = root_response_mass(hand_id, trick_no, contract);
                assert_eq!(
                    c.root_exact_mass, independent,
                    "H1: the re-descent through the frontier equals the root's own exact \
                     optimum (h{hand_id}-t{trick_no} contract {contract} cut {cut})"
                );
                assert_eq!(c.root_check_mass, independent);
                assert_eq!(c.contract, contract);
                assert_eq!(c.refused(), 0, "H1: nothing refused under an ample cap");
                for (n, p) in c.priced() {
                    priced += 1;
                    assert!(p.doomed <= n.mass);
                    assert!(p.q_mass + p.doomed <= n.mass, "H1: Q ≤ U^God at every node");
                    assert_eq!(
                        &p.upper - &p.q,
                        p.phi,
                        "H1: Φ is the difference it says it is"
                    );
                    assert_eq!(p.nothing_saveable, p.doomed == n.mass);
                    assert_eq!(
                        n.history.len(),
                        cut,
                        "H1: a priced node sits exactly at the cut"
                    );
                }
                checked += 1;
            }
        }
    }
    assert!(checked >= 40, "H1: a real sweep, got {checked}");
    assert!(
        priced >= 200,
        "H1: real frontier nodes were priced, got {priced}"
    );
}

#[test]
fn h2_per_node_doom_is_the_doom_census_per_world_truth() {
    let r = receipt();
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec());
    let dspec = DoomSpec {
        node_budget: 10_000_000,
        walk_cap: 1_000_000,
        max_level: 3,
        critical: DominoSet::EMPTY,
        descend_top: None,
    };
    let mut coordinates = 0usize;
    for (hand_id, trick_no) in T56.iter().chain([(8usize, 4usize)].iter()).copied() {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let c = census(hand_id, trick_no, position.bid, 1);
        let actions = legal_actions(&root, &position);
        assert_eq!(
            c.frontier_nodes() + c.decided_before_cut().0,
            actions.len(),
            "H2: at cut 1 the frontier IS the set of root actions"
        );
        for a in actions {
            let node = c
                .nodes
                .iter()
                .find(|n| n.history == vec![a])
                .expect("H2: every root action has its frontier node");
            let mut progress = |_: u64, _: u64, _: u128, _: u64| {};
            let e = doom_enumeration(&oracle, &root, &position, &field, a, &dspec, &mut progress);
            let belief = FactorBelief::uniform_root(&root, &position, &field);
            let mut rs = ResponseStats::default();
            let q = response_success_mass(&oracle, &belief.focal_play(a), &field, &mut rs);
            match &node.verdict {
                NodeVerdict::Priced(p) => {
                    assert_eq!(
                        node.mass, e.fiber,
                        "H2: same fiber (h{hand_id}-t{trick_no} {a})"
                    );
                    assert_eq!(
                        p.doomed, e.doomed,
                        "H2: the census's per-world doom IS doom_enumeration's (h{hand_id}-t{trick_no} {a})"
                    );
                    assert_eq!(p.upper, e.upper, "H2: same God upper");
                    assert_eq!(p.q_mass, q, "H2: same exact Q");
                }
                NodeVerdict::DecidedBeforeCut { .. } => {
                    panic!(
                        "H2: a root child at cut 1 is a frontier node, never decided-before-cut"
                    );
                }
                NodeVerdict::Refused { .. } => panic!("H2: nothing is refused under the ample cap"),
            }
            coordinates += 1;
        }
    }
    assert!(
        coordinates >= 18,
        "H2: U0's fourteen plus h8-t4's four, got {coordinates}"
    );
}

#[test]
fn h3_the_cut_never_under_prices_and_is_exact_iff_the_frontier_is() {
    let r = receipt();
    let mut exact_cuts = 0usize;
    let mut strict_cuts = 0usize;
    for (hand_id, trick_no) in T4.iter().chain(T56.iter()).copied() {
        let (_, position) = root_at(&r, hand_id, trick_no);
        for cut in [1usize, 4] {
            let c = census(hand_id, trick_no, position.bid, cut);
            let cut_mass = c
                .root_cut_mass
                .expect("H3: nothing refused, so the cut is priced");
            assert!(
                cut_mass >= c.root_exact_mass,
                "H3: a God-upper cut never sits below the exact root (h{hand_id}-t{trick_no} cut {cut})"
            );
            for a in &c.actions {
                let am = a.cut_mass.expect("priced");
                assert!(am >= a.exact_mass, "H3: per action too");
            }
            let all_tight = c.priced().iter().all(|(_, p)| p.god_tight());
            if all_tight {
                assert_eq!(
                    cut_mass, c.root_exact_mass,
                    "H3: a frontier of God-tight nodes makes the cut exact at the root \
                     (h{hand_id}-t{trick_no} cut {cut})"
                );
                assert_eq!(c.cut_argmax, c.exact_argmax);
                exact_cuts += 1;
            }
            if cut_mass > c.root_exact_mass {
                assert!(
                    !all_tight,
                    "H3: a strict over-pricing needs a positive-gap frontier node"
                );
                strict_cuts += 1;
            }
            let over = c.root_over_pricing().expect("priced");
            assert_eq!(
                over,
                BigRational::new(
                    BigInt::from(cut_mass - c.root_exact_mass),
                    BigInt::from(c.root_fiber)
                )
            );
        }
    }
    assert!(
        exact_cuts >= 8,
        "H3: exact cuts were seen, got {exact_cuts}"
    );
    // The specimen: at cut 1 a trick-4 root's frontier is U0's positive-gap
    // coordinates, so the cut over-prices strictly there.
    assert!(
        strict_cuts >= 1,
        "H3: at least one strict over-pricing was seen, got {strict_cuts}"
    );
}

#[test]
fn h4_refusals_are_typed_and_nothing_is_dropped() {
    let r = receipt();
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec());
    let (root, position) = root_at(&r, 8, 4);
    let ample = census(8, 4, position.bid, 4);
    let tiny = horizon_census(&oracle, &root, &position, &field, &spec(4, 8));
    assert_eq!(
        ample.nodes.len(),
        tiny.nodes.len(),
        "H4: the same nodes are reached under either cap"
    );
    assert_eq!(ample.frontier_nodes(), tiny.frontier_nodes());
    let mut refused = 0usize;
    for (a, t) in ample.nodes.iter().zip(tiny.nodes.iter()) {
        assert_eq!(a.history, t.history);
        assert_eq!(a.mass, t.mass);
        match (&a.verdict, &t.verdict) {
            (NodeVerdict::Priced(_), NodeVerdict::Refused { fiber, cap }) => {
                assert!(
                    *fiber > 8 && *cap == 8,
                    "H4: the refusal names fiber and cap"
                );
                assert!(a.mass > 8);
                refused += 1;
            }
            (NodeVerdict::Priced(p), NodeVerdict::Priced(q)) => {
                assert!(a.mass <= 8);
                assert_eq!(
                    p, q,
                    "H4: an affordable node prices identically under either cap"
                );
            }
            (NodeVerdict::DecidedBeforeCut { .. }, NodeVerdict::DecidedBeforeCut { .. }) => {}
            other => panic!("H4: verdicts diverge only by refusal: {other:?}"),
        }
    }
    assert!(refused > 0, "H4: the tiny cap refused something");
    assert_eq!(tiny.refused(), refused);
    assert_eq!(
        tiny.root_exact_mass, ample.root_exact_mass,
        "H4: the exact side is complete regardless of the cap"
    );
    assert!(
        tiny.root_cut_mass.is_none(),
        "H4: the cut side is absent, never a number, when a node was refused"
    );
    assert!(tiny.cut_argmax.is_none());
    assert!(tiny.root_over_pricing().is_none());
    assert!(tiny.cut_flips_root().is_none());
    assert_eq!(ample.refused(), 0);
}

#[test]
fn h5_two_censuses_of_one_root_render_identically() {
    let r = receipt();
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec());
    let (root, position) = root_at(&r, 3, 4);
    // One fresh census against the fixture's.
    let a = horizon_census(&oracle, &root, &position, &field, &spec(4, 40_000));
    let b = census(3, 4, position.bid, 4).clone();
    assert_eq!(a, b, "H5: deterministic");
    assert_eq!(format!("{a:?}"), format!("{b:?}"));
}
