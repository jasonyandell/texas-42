//! The fiber-crush probe: a three-arm cost ladder over the void-free capacity
//! fiber, at rungs n = 4, 5, 6 tricks remaining.
//!
//! Design: `walt/FIBER-PROBE.md`. Binding rulings: `walt/CENSUS-RULINGS.md`
//! "Fiber-probe rulings (P-Q1..P-Q6)", amendments P-A1..P-A21, all in force
//! here. Exploratory tier; the fold weighting is a declared INSTRUMENT
//! (P-A12). Scope: pip-trump only (F1), receipt corpus
//! `rob/receipts/verify_player.txt` hands 0-12.
//!
//! Arms (P-A7): A0 = per-world backward induction, no cache. A1 = the same
//! recursion with a boundary cache on the packed semantic state, shared
//! across the coordinate's whole evaluated set (F6's identity-transport
//! control, the `scalar.rs` key). B = the r3-signature content-addressed
//! class DAG (closure carrier + retrograde pass + Lemma-V fold). B/A1 is the
//! equivariance dividend proper; B/A0 includes ordinary memoisation and is
//! never quoted alone.
//!
//! Writes `results/fiber_probe_2026-08-11.txt`. Subcommand `h` runs the
//! P-A14 cold-H attempt and writes `results/fiber_probe_h_2026-08-11.txt`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::time::Instant;

use walt_core::receipt::{locate_verify_player, parse_file, Receipt, ReceiptHand};
use walt_core::replay::{state_before_trick, voids_before_trick};
use walt_core::{ContextSet, Decl, DominoSet, Seat};
use walt_geom::{q, qi, Q};
use walt_kernel::{Hidden, Kernel, HIDDEN_SEATS};
use walt_skeleton::equivariant::{actor_offset, build_r3, closure_carrier, Carrier, Situation, R3};
use walt_strat::{ScalarHidden, ScalarValuation};

const RESULTS: &str = "results/fiber_probe_2026-08-11.txt";
const H_RESULTS: &str = "results/fiber_probe_h_2026-08-11.txt";

/// The `dag-v1` particle-step budget of the P-A14 cold-H attempt — a declared
/// stop, printed on exhaustion, never a silent cap.
const H_BUDGET: u64 = 200_000_000;

/// One rung of the ladder. `g` and `w` are freeze 8 (P-A15): the decimation
/// evaluates enumeration indices (i*g mod N) for i = 0..W, gcd(g, N) = 1
/// asserted in-run; the same index set is evaluated by every arm. The hand
/// subsets are the declared fast-iteration stops of this run: every omission
/// is a stop, printed, never a silent cap.
struct Rung {
    trick_no: usize,
    n: usize,
    g: u128,
    w: usize,
    hands: &'static [usize],
}

const RUNGS: [Rung; 3] = [
    Rung {
        trick_no: 4,
        n: 4,
        g: 7919,
        w: 240,
        hands: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
    },
    Rung {
        trick_no: 3,
        n: 5,
        g: 104_729,
        w: 24,
        hands: &[0, 1, 2, 3],
    },
    Rung {
        trick_no: 2,
        n: 6,
        g: 1_299_709,
        w: 6,
        hands: &[0],
    },
];

fn receipt() -> Receipt {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    parse_file(&path).expect("the receipt parses")
}

fn binom(n: u128, k: u128) -> u128 {
    let mut out: u128 = 1;
    for i in 0..k {
        out = out * (n - i) / (i + 1);
    }
    out
}

fn gcd_u(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// Modular inverse of `g` mod `n` for gcd(g, n) = 1, by extended Euclid.
fn modinv(g: u128, n: u128) -> u128 {
    let (mut a, mut b) = (
        i128::try_from(g % n).expect("fits"),
        i128::try_from(n).expect("fits"),
    );
    let (mut x0, mut x1) = (1i128, 0i128);
    while b != 0 {
        let quot = a / b;
        let r = a - quot * b;
        a = b;
        b = r;
        let x2 = x0 - quot * x1;
        x0 = x1;
        x1 = x2;
    }
    assert_eq!(a, 1, "gcd(g, N) = 1 (freeze 8)");
    let n_i = i128::try_from(n).expect("fits");
    u128::try_from((x0 % n_i + n_i) % n_i).expect("nonnegative")
}

/// Exact fixed-point rendering of `a / b` in thousandths; integer arithmetic
/// only (P-A19).
fn fp(a: u128, b: u128) -> String {
    if b == 0 {
        return "n/a".to_string();
    }
    let scaled = a.saturating_mul(1000) / b;
    format!("{}.{:03}", scaled / 1000, scaled % 1000)
}

/// The void-free capacity kernel of one coordinate (P-A1): focal = the
/// declaring seat (P-A4), hidden slots in offset order 1..=3 from focal,
/// capacities from the receipt state, voids deliberately EMPTY. Returns the
/// kernel and the trick's leader.
fn void_free_kernel(hand: &ReceiptHand, trick_no: usize) -> (Kernel, Seat) {
    let (hands, leader) = state_before_trick(hand, trick_no).expect("the receipt replays");
    let focal = hand.bidder;
    let mut hidden = [Hidden {
        seat: focal,
        capacity: 0,
        voids: ContextSet::EMPTY,
    }; HIDDEN_SEATS];
    let mut pool = DominoSet::EMPTY;
    for (slot, k) in hidden.iter_mut().zip(1..=3) {
        let seat = focal.plus(k);
        *slot = Hidden {
            seat,
            capacity: hands[seat.index()].len(),
            voids: ContextSet::EMPTY,
        };
        pool = pool.union(hands[seat.index()]);
    }
    let kernel = Kernel::new(hand.decl, focal, hands[focal.index()], pool, hidden)
        .expect("the void-free capacity kernel is well formed");
    (kernel, leader)
}

/// The same coordinate with the history's void constraints kept — the seat's
/// actual support cell system, counted for the P-A2 gap report.
fn voided_count(hand: &ReceiptHand, trick_no: usize) -> u128 {
    let (hands, _) = state_before_trick(hand, trick_no).expect("the receipt replays");
    let voids = voids_before_trick(hand, trick_no);
    let focal = hand.bidder;
    let mut hidden = [Hidden {
        seat: focal,
        capacity: 0,
        voids: ContextSet::EMPTY,
    }; HIDDEN_SEATS];
    let mut pool = DominoSet::EMPTY;
    for (slot, k) in hidden.iter_mut().zip(1..=3) {
        let seat = focal.plus(k);
        *slot = Hidden {
            seat,
            capacity: hands[seat.index()].len(),
            voids: voids[seat.index()],
        };
        pool = pool.union(hands[seat.index()]);
    }
    Kernel::new(hand.decl, focal, hands[focal.index()], pool, hidden)
        .expect("the voided kernel is well formed")
        .count()
}

/// Freeze 8: the decimated index set, materialized in enumeration order
/// (freeze 7: `FiberIter` — hidden slots in declared order, each slot's
/// k-combinations of its ascending candidate list in lexicographic order).
fn select_worlds(kernel: &Kernel, g: u128, w: usize) -> (u128, Vec<[DominoSet; Seat::COUNT]>) {
    let n_total = kernel.count();
    let w_eff = if u128::try_from(w).expect("fits") > n_total {
        usize::try_from(n_total).expect("fits")
    } else {
        w
    };
    assert_eq!(gcd_u(g, n_total), 1, "gcd(g, N) = 1 (freeze 8)");
    let inv = modinv(g, n_total);
    let w_bound = u128::try_from(w_eff).expect("fits");
    let mut out = Vec::with_capacity(w_eff);
    for (j, world) in kernel.worlds().enumerate() {
        let i = (u128::try_from(j).expect("fits") * inv) % n_total;
        if i < w_bound {
            out.push(world.hands());
        }
    }
    assert_eq!(out.len(), w_eff, "the decimation selects exactly W worlds");
    (n_total, out)
}

/// The declared operator (P-A5, freeze 10): the F4 world-informed
/// focal-max / hidden-uniform-expectation node rule under the count-free
/// `q_trick` valuation (P-A6) — max over moves at actor offset 0, uniform
/// mean at offsets 1..=3, each move contributing its increment plus the
/// successor value. Lemma V descends it to r3 classes.
fn aggregate(offset: u8, vals: Vec<Q>) -> Q {
    assert!(!vals.is_empty(), "a live situation has a legal move");
    if offset == 0 {
        vals.into_iter().max().expect("nonempty")
    } else {
        let n = i128::try_from(vals.len()).expect("fits");
        let sum = vals.into_iter().fold(qi(0), |a, b| a + b);
        sum * q(1, n)
    }
}

/// Arm A0: plain backward induction on one world's tree. No cache, no
/// canonicalization, no class objects (P-A8); one edge counted per primitive
/// step explored; no pruning of any kind (P-A10).
fn value_a0(sit: &Situation, edges: &mut u64) -> Q {
    let mut vals = Vec::new();
    for tile in sit.legal().iter() {
        *edges += 1;
        let (k, next) = sit.step(tile);
        let mut v = qi(i128::from(k));
        if let Some(s) = next {
            v += value_a0(&s, edges);
        }
        vals.push(v);
    }
    aggregate(actor_offset(sit), vals)
}

/// Freeze 11, A1's key: the packed semantic state at a trick boundary —
/// leader index and the four hand bitsets, nothing canonicalized.
fn pack_boundary(sit: &Situation) -> u128 {
    let mut key = u128::try_from(sit.leader.index()).expect("fits");
    for h in &sit.hands {
        key = (key << 28) | u128::from(h.bits());
    }
    key
}

/// Arm A1: the identical recursion with a boundary cache shared across the
/// coordinate's whole evaluated set — F6's identity-transport control.
fn value_a1(sit: &Situation, memo: &mut BTreeMap<u128, Q>, edges: &mut u64) -> Q {
    let key = if sit.table.is_empty() {
        let k = pack_boundary(sit);
        if let Some(v) = memo.get(&k) {
            return *v;
        }
        Some(k)
    } else {
        None
    };
    let mut vals = Vec::new();
    for tile in sit.legal().iter() {
        *edges += 1;
        let (k, next) = sit.step(tile);
        let mut v = qi(i128::from(k));
        if let Some(s) = next {
            v += value_a1(&s, memo, edges);
        }
        vals.push(v);
    }
    let v = aggregate(actor_offset(sit), vals);
    if let Some(k) = key {
        memo.insert(k, v);
    }
    v
}

/// The Lemma-V fold: class values by ascending grade, each from a
/// representative's canonical per-move tuples. Constant on classes by the
/// ruling's induction; the P-A9 receipt checks it against the per-world arms.
fn fold_values(carrier: &Carrier, r3: &R3) -> Vec<Q> {
    let nclasses = r3.class_members.len();
    let mut order: Vec<usize> = (0..nclasses).collect();
    order.sort_by_key(|&c| r3.class_grade[c]);
    let mut vals: Vec<Option<Q>> = vec![None; nclasses];
    for c in order {
        let rep = r3.class_members[c][0];
        let sit = &carrier.states[rep];
        let mut acc = Vec::with_capacity(r3.tuples[rep].len());
        for tup in &r3.tuples[rep] {
            let mut v = qi(i128::from(tup.increment));
            if let Some(sc) = tup.successor {
                v += vals[sc].expect("successors fold before predecessors");
            }
            acc.push(v);
        }
        vals[c] = Some(aggregate(actor_offset(sit), acc));
    }
    vals.into_iter()
        .map(|v| v.expect("every class folded"))
        .collect()
}

struct BArm {
    carrier_len: usize,
    classes: usize,
    edges: u64,
    ns_carrier: u128,
    ns_r3: u128,
    ns_fold: u128,
    ns_lookup: u128,
    per_world: Vec<Q>,
    world_class: Vec<usize>,
    class_values: Vec<Q>,
}

impl BArm {
    fn total_ns(&self) -> u128 {
        self.ns_carrier + self.ns_r3 + self.ns_fold + self.ns_lookup
    }
}

/// Arm B: closure carrier over the evaluated set, retrograde r3 pass,
/// Lemma-V fold, per-world root lookup. Every cost inside B's wall-clock is
/// B's own (P-A8).
fn run_b(kernel: &Kernel, leader: Seat, worlds: &[[DominoSet; Seat::COUNT]]) -> BArm {
    run_b_full(kernel, leader, worlds).0
}

/// `run_b` returning the carrier and r3 pass as well — the refinement probe
/// needs the class DAG itself, not only the per-world values.
fn run_b_full(
    kernel: &Kernel,
    leader: Seat,
    worlds: &[[DominoSet; Seat::COUNT]],
) -> (BArm, Carrier, R3) {
    let decl = kernel.decl();
    let focal = kernel.viewer();
    let t0 = Instant::now();
    let seeds: Vec<Situation> = worlds
        .iter()
        .map(|h| Situation {
            decl,
            focal,
            leader,
            hands: *h,
            table: Vec::new(),
        })
        .collect();
    let carrier = closure_carrier(&seeds);
    let ns_carrier = t0.elapsed().as_nanos();
    let t1 = Instant::now();
    let r3 = build_r3(&carrier);
    let ns_r3 = t1.elapsed().as_nanos();
    let t2 = Instant::now();
    let class_values = fold_values(&carrier, &r3);
    let ns_fold = t2.elapsed().as_nanos();
    let t3 = Instant::now();
    let world_class: Vec<usize> = seeds
        .iter()
        .map(|s| r3.class_of[carrier.lookup(s).expect("a seed is a carrier state")])
        .collect();
    let per_world: Vec<Q> = world_class.iter().map(|&c| class_values[c]).collect();
    let ns_lookup = t3.elapsed().as_nanos();
    let edges = r3.tuples.iter().map(|t| t.len() as u64).sum();
    let arm = BArm {
        carrier_len: carrier.len(),
        classes: r3.class_members.len(),
        edges,
        ns_carrier,
        ns_r3,
        ns_fold,
        ns_lookup,
        per_world,
        world_class,
        class_values,
    };
    (arm, carrier, r3)
}

fn trump_count(decl: Decl, hand: DominoSet) -> i128 {
    let Decl::PipTrump(t) = decl else {
        panic!("F1 scope: the probe corpus is pip-trump only")
    };
    i128::try_from(hand.iter().filter(|d| d.has(t)).count()).expect("fits")
}

struct HandStats {
    h: usize,
    /// The trick leader's offset from focal (P-A4: never an
    /// orientation-flavoured word).
    leader_offset: u8,
    n_total: u128,
    n_voided: u128,
    w: usize,
    a0_ns: u128,
    a0_edges: u64,
    a0_per_world: Vec<u64>,
    a1_ns: u128,
    a1_edges: u64,
    a1_entries: usize,
    b: BArm,
    root_distinct: usize,
    refold_ns: u128,
}

fn run_hand(hand: &ReceiptHand, rung: &Rung) -> HandStats {
    let (kernel, leader) = void_free_kernel(hand, rung.trick_no);
    let n_voided = voided_count(hand, rung.trick_no);
    let (n_total, worlds) = select_worlds(&kernel, rung.g, rung.w);
    let closed =
        binom(3 * rung.n as u128, rung.n as u128) * binom(2 * rung.n as u128, rung.n as u128);
    assert_eq!(n_total, closed, "|Phi(C0)| equals its closed form");
    let decl = kernel.decl();
    let focal = kernel.viewer();
    let roots: Vec<Situation> = worlds
        .iter()
        .map(|h| Situation {
            decl,
            focal,
            leader,
            hands: *h,
            table: Vec::new(),
        })
        .collect();

    // Arm A0.
    let t = Instant::now();
    let mut a0_per_world = Vec::with_capacity(roots.len());
    let mut a0_vals = Vec::with_capacity(roots.len());
    let mut a0_edges: u64 = 0;
    for sit in &roots {
        let before = a0_edges;
        a0_vals.push(value_a0(sit, &mut a0_edges));
        a0_per_world.push(a0_edges - before);
    }
    let a0_ns = t.elapsed().as_nanos();

    // Arm A1: one boundary cache across the coordinate's evaluated set.
    let t = Instant::now();
    let mut memo: BTreeMap<u128, Q> = BTreeMap::new();
    let mut a1_edges: u64 = 0;
    let mut a1_vals = Vec::with_capacity(roots.len());
    for sit in &roots {
        a1_vals.push(value_a1(sit, &mut memo, &mut a1_edges));
    }
    let a1_ns = t.elapsed().as_nanos();
    let a1_entries = memo.len();

    // Arm B.
    let b = run_b(&kernel, leader, &worlds);

    // P-A9: the same-object receipt — bit-exact equality across arms, and
    // value(world) = value(root class). Any mismatch stops the run.
    for i in 0..roots.len() {
        assert_eq!(a0_vals[i], a1_vals[i], "A0 = A1 at world {i} (P-A9)");
        assert_eq!(
            a0_vals[i], b.per_world[i],
            "A0 = B root-class value at world {i} (P-A9)"
        );
    }
    let root_distinct = b.world_class.iter().collect::<BTreeSet<_>>().len();

    // The declared fold weighting (P-A12, freeze 9): w = 1 + trumps the
    // world assigns to offset 1 from focal. INSTRUMENT tier; times a second
    // aggregation over the fixed class store.
    let t = Instant::now();
    let mut acc = qi(0);
    for (w, &c) in worlds.iter().zip(&b.world_class) {
        let wt = qi(1 + trump_count(decl, w[focal.plus(1).index()]));
        acc += wt * b.class_values[c];
    }
    let refold_ns = t.elapsed().as_nanos();
    // The accumulator is a timing instrument; its value is asserted finite
    // by construction and deliberately not reported (P-A12).
    let _ = acc;

    let mut leader_offset = 0u8;
    while focal.plus(usize::from(leader_offset)) != leader {
        leader_offset += 1;
        assert!(leader_offset < 4, "the leader is one of the four seats");
    }

    HandStats {
        h: hand.id,
        leader_offset,
        n_total,
        n_voided,
        w: worlds.len(),
        a0_ns,
        a0_edges,
        a0_per_world,
        a1_ns,
        a1_edges,
        a1_entries,
        b,
        root_distinct,
        refold_ns,
    }
}

fn quartiles(mut xs: Vec<u64>) -> (u64, u64, u64, u64, u64) {
    xs.sort_unstable();
    let n = xs.len();
    (xs[0], xs[n / 4], xs[n / 2], xs[(3 * n) / 4], xs[n - 1])
}

fn median_ratio(mut rs: Vec<(u128, u128)>) -> (u128, u128) {
    rs.sort_by(|a, b| (a.0 * b.1).cmp(&(b.0 * a.1)));
    rs[rs.len() / 2]
}

fn cpu_model() -> String {
    std::process::Command::new("sysctl")
        .args(["-n", "machdep.cpu.brand_string"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn header() -> String {
    let mut out = String::new();
    let w = &mut out;
    let _ = writeln!(w, "walt fiber-crush probe — three-arm cost ladder over the void-free capacity fiber — exploratory tier");
    let _ = writeln!(w, "design: walt/FIBER-PROBE.md; binding rulings: walt/CENSUS-RULINGS.md \"Fiber-probe rulings (P-Q1..P-Q6)\", amendments P-A1..P-A21 all in force");
    let _ = writeln!(w, "scope: pip-trump only (F1, asserted in-run); corpus rob/receipts/verify_player.txt hands 0-12; focal = the declaring seat (P-A4); the leader is declared per coordinate as an offset from focal");
    let _ = writeln!(w);
    let _ = writeln!(w, "P-A1 (verbatim): This is Phi(C0), the void-free capacity-cell fiber, a declared superset of the seat's actual support Phi(C) (v0.4 §2.1); the void constraints derivable from the actual play history are deliberately dropped. It is a declared cost domain. No support fact about any seat may be read from it, and Y3's exclusion conclusions do not apply to it.");
    let _ = writeln!(w, "P-A3: both arm costs on Phi(C0) are upper bounds for the same arm on Phi(C); every crush factor below is a ratio of two upper bounds, measured on the void-free capacity fiber, and is a bound in neither direction for the seat's real fiber.");
    let _ = writeln!(w);
    let _ = writeln!(w, "operator (P-A5, freeze 10): the F4 world-informed focal-max / hidden-uniform-expectation node rule — the quantity computed per world is world-informed; it is not a seat value. No V or Q of any seat is claimed.");
    let _ = writeln!(w, "valuation (P-A6, freeze 10): q_trick only — the focal partnership's trick count, affine in t_T*e_star; q_points is out of scope for this probe. Exact i128 integers and rationals throughout; no floats anywhere, including timings and ratios.");
    let _ = writeln!(w, "fold weighting (P-A12, freeze 9): weight(world) = 1 + |trumps the world assigns to the seat at offset 1 from focal|. The fold weighting is neither support nor belief (v0.4 §2.5); it is an aggregation argument (§5.5) chosen to time a second fold. No number produced under it is a value claim, a belief, or a support fact. INSTRUMENT tier, below every tier, cited by nothing above it.");
    let _ = writeln!(w);
    let _ = writeln!(w, "CAVEAT (mandated, r3 ruling Q4): Classes are dynamics-equivalence classes under §12.6A on this carrier, uniform-legal field, count-free contract, per-step interface typing (r3 ruling Q3); they need not be closed under any tile relabeling and carry no structural description — the compact-description question (v0.4 §12.7) is separate and open. Coarsest is relative to that scope. Class identities are intrinsic to continuations; counts are carrier-relative; carrier growth adds classes, never splits existing ones. Exploratory tier. ECL holds by construction; see verification lines. These are not hidden-decision PI classes (v0.4 §12.4): the equivalence is dynamics, not response equality.");
    let _ = writeln!(w, "ECL import (P-A20): the r3 machinery's ECL verdicts are imported, not re-earned — r3 independent re-check PASS (census_2026-08-10_r3.txt); t5 independent re-check PASS (census_t5_2026-08-10.txt). Class counts below are exploratory-tier inventory on this probe's evaluated sets.");
    let _ = writeln!(w);
    let _ = writeln!(w, "freezes: 1-2 (r3: content-addressed 128-bit FNV-1a class encoding; canonical move order by (increment, classification, successor class hash), concrete tile order as the tie rule) and 3 (the yard tree encoding) restated unchanged. New: (7) the fiber enumeration order — FiberIter over the void-free kernel with hidden slots in offset order 1,2,3 from focal, each slot's k-combinations of its ascending-domino candidate list in lexicographic order, slot 0 outermost; the index of a world is its position in this stream. (8) the decimation — evaluate indices (i*g mod N), i = 0..W, per-rung (g, W) printed in each rung header, gcd(g, N) = 1 asserted. (9) the fold weighting above. (10) the operator and valuation above. (11) the per-arm keys — A1: the packed semantic state (leader index, the four hand bitsets) at trick boundaries only, nothing canonicalized; B: the r3 128-bit FNV-1a signature.");
    let _ = writeln!(w);
    let _ = writeln!(w, "timing discipline (P-A19): one run, one machine, single-threaded; wall-clock in integer nanoseconds; every ratio an exact fixed-point rendering by integer division. CPU: {}; build profile: release; threads: 1.", cpu_model());
    let _ = writeln!(w, "declared stops of this run: the per-rung hand subsets and (g, W) below are the fast-iteration budget; every hand or rung not run is listed by omission from a printed rung header, never silently. Full-fiber arm B (P-A16) was not run this iteration — a declared stop; the n=6 raw total is therefore reported as raw-mean x N, flagged ESTIMATED with its estimator, and the full-fiber B side is reported as not run.");
    let _ = writeln!(w, "provenance: SINGLE-IMPLEMENTATION — one Rust implementation (this runner over walt-skeleton's equivariant module)");
    let _ = writeln!(
        w,
        "regenerate: cargo run --release -p walt-factory --example fiber_probe"
    );
    let _ = writeln!(w);
    out
}

fn rung_section(rung: &Rung, stats: &[HandStats]) -> String {
    let mut out = String::new();
    let w = &mut out;
    let _ = writeln!(
        w,
        "================ rung n={} (trick {}) — decimation g={}, W={}; hands {:?} ================",
        rung.n, rung.trick_no, rung.g, rung.w, rung.hands
    );
    for s in stats {
        let (mn, q1, md, q3, mx) = quartiles(s.a0_per_world.clone());
        let _ = writeln!(
            w,
            "  h{}: leader at offset {} from focal; |Phi(C0)| = {}, |Phi(C)| voided = {}, voided : void-free = {} (P-A2)",
            s.h,
            s.leader_offset,
            s.n_total,
            s.n_voided,
            fp(s.n_voided, s.n_total)
        );
        let _ = writeln!(
            w,
            "      A0: {} edges, {} ns; per-world edges min/q1/med/q3/max = {}/{}/{}/{}/{}, exact mean = {}/{}",
            s.a0_edges, s.a0_ns, mn, q1, md, q3, mx, s.a0_edges, s.w
        );
        let _ = writeln!(
            w,
            "      A1: {} edges, {} ns; boundary-cache entries {}",
            s.a1_edges, s.a1_ns, s.a1_entries
        );
        let _ = writeln!(
            w,
            "      B : carrier {} situations, {} classes, {} edges; ns carrier/r3/fold/lookup = {}/{}/{}/{}; total {} ns",
            s.b.carrier_len,
            s.b.classes,
            s.b.edges,
            s.b.ns_carrier,
            s.b.ns_r3,
            s.b.ns_fold,
            s.b.ns_lookup,
            s.b.total_ns()
        );
        let _ = writeln!(
            w,
            "      root collapse (its own line per P-A10's disposition of M2): {} worlds -> {} distinct root classes",
            s.w, s.root_distinct
        );
        let _ = writeln!(
            w,
            "      receipts (P-A9): cross-arm per-world equality {}/{} held; value(world) = value(root class) {}/{} held",
            s.w, s.w, s.w, s.w
        );
        let _ = writeln!(
            w,
            "      wall ratios: A1:A0 = {} (memoisation dividend); B:A1 = {} (THE EQUIVARIANCE DIVIDEND PROPER); B:A0 = {} (includes ordinary memoisation, never quoted alone)",
            fp(s.a1_ns, s.a0_ns),
            fp(s.b.total_ns(), s.a1_ns),
            fp(s.b.total_ns(), s.a0_ns)
        );
        let _ = writeln!(
            w,
            "      refold (declared fold weighting, INSTRUMENT): {} ns; refold : B-build = {}",
            s.refold_ns,
            fp(s.refold_ns, s.b.total_ns())
        );
    }
    let a1a0 = median_ratio(stats.iter().map(|s| (s.a1_ns, s.a0_ns)).collect());
    let ba1 = median_ratio(stats.iter().map(|s| (s.b.total_ns(), s.a1_ns)).collect());
    let ba0 = median_ratio(stats.iter().map(|s| (s.b.total_ns(), s.a0_ns)).collect());
    let _ = writeln!(
        w,
        "  rung medians (wall): A1:A0 = {}; B:A1 = {}; B:A0 = {}",
        fp(a1a0.0, a1a0.1),
        fp(ba1.0, ba1.1),
        fp(ba0.0, ba0.1)
    );
    let _ = writeln!(w);
    out
}

fn extrapolation(all: &[(usize, Vec<HandStats>)]) -> String {
    let mut out = String::new();
    let w = &mut out;
    let _ = writeln!(
        w,
        "================ extrapolation (P-A21) — exploratory tier ================"
    );
    let _ = writeln!(w, "ladder on hand 0 only (the one coordinate present at every rung); per-world mean wall-clock = arm ns / W by integer division.");
    let mut h0: Vec<(usize, u128, u128, u128, u128)> = Vec::new();
    for (n, stats) in all {
        if let Some(s) = stats.iter().find(|s| s.h == 0) {
            let wc = u128::try_from(s.w).expect("fits");
            h0.push((
                *n,
                s.a0_ns / wc,
                s.a1_ns / wc,
                s.b.total_ns() / wc,
                s.n_total,
            ));
        }
    }
    for (n, a0, a1, b, ntot) in &h0 {
        let _ = writeln!(
            w,
            "  n={}: per-world ns A0/A1/B = {}/{}/{}; |Phi(C0)| = {}; ESTIMATED raw fiber total (A0 mean x N, P-A16) = {} ns",
            n, a0, a1, b, ntot,
            a0.saturating_mul(*ntot)
        );
    }
    if h0.len() >= 2 {
        let last = &h0[h0.len() - 1];
        let prev = &h0[h0.len() - 2];
        let _ = writeln!(
            w,
            "  fitted law (declared: one more step of the last measured growth, n={} -> n={}): per-world A0 growth = {}, per-world B growth = {}",
            prev.0, last.0,
            fp(last.1, prev.1),
            fp(last.3, prev.3)
        );
        let n7_worlds: u128 = binom(21, 7) * binom(14, 7);
        let implied_a0 = last.1.saturating_mul(last.1) / prev.1.max(1);
        let implied_b = last.3.saturating_mul(last.3) / prev.3.max(1);
        let _ = writeln!(
            w,
            "  implied n=7 cost under the fitted law — extrapolation, exploratory tier, never a statement about an unrun computation, never a feasibility claim: per-world ns A0 ~ {}, B ~ {}; the n=7 void-free fiber is the deal itself, {} worlds, where P-A1's scope restriction is at its widest relative to any real seat.",
            implied_a0, implied_b, n7_worlds
        );
    }
    let _ = writeln!(w);
    let _ = writeln!(w, "H row (P-A14): the measured amortisation above is for the world-informed per-world operator. It does not establish amortisation for treatment H, where a change of weighting is a re-solve over the fixed class DAG (Y3), not a re-fold. The weighted re-solve over the pre-built class DAG is unavailable this run: the existing H solvers take a uniform fiber weighting only, and the Y3 K-bar search integration is not yet built. A cold uniform-H attempt is subcommand `h` of this runner (declared dag-v1 particle-step budget {H_BUDGET}); it writes results/fiber_probe_h_2026-08-11.txt on completion, and prints its declared stop on budget exhaustion. The number the belief/policy-iteration platform claim actually rests on is therefore NOT yet measured; both facts are stated here per the amendment.");
    let _ = writeln!(w);
    out
}

fn run_ladder() {
    let r = receipt();
    for hand in &r.hands {
        assert!(
            matches!(hand.decl, Decl::PipTrump(_)),
            "F1 scope: pip-trump only"
        );
    }
    let mut sections: Vec<String> = vec![header()];
    let mut all: Vec<(usize, Vec<HandStats>)> = Vec::new();
    for rung in &RUNGS {
        eprintln!(
            "rung n={} (trick {}): hands {:?}",
            rung.n, rung.trick_no, rung.hands
        );
        let mut stats = Vec::new();
        for &h in rung.hands {
            let t = Instant::now();
            let s = run_hand(&r.hands[h], rung);
            eprintln!(
                "  h{h}: W={} A0 {} ns, A1 {} ns, B {} ns ({} classes) [{} ns total]",
                s.w,
                s.a0_ns,
                s.a1_ns,
                s.b.total_ns(),
                s.b.classes,
                t.elapsed().as_nanos()
            );
            stats.push(s);
        }
        sections.push(rung_section(rung, &stats));
        all.push((rung.n, stats));
        // Partial results survive an external stop; the final write appends
        // the extrapolation and completion line.
        let mut partial = sections.concat();
        let _ = writeln!(
            &mut partial,
            "run complete: no (rungs so far: {})",
            all.len()
        );
        std::fs::write(RESULTS, partial).expect("results file writes");
    }
    sections.push(extrapolation(&all));
    let mut full = sections.concat();
    let _ = writeln!(&mut full, "run complete: yes");
    std::fs::write(RESULTS, full).expect("results file writes");
    println!("wrote {RESULTS}");
}

fn run_h() {
    let r = receipt();
    let mut out = String::new();
    let w = &mut out;
    let _ = writeln!(
        w,
        "walt fiber-crush probe — P-A14 cold-H attempt — exploratory tier"
    );
    let _ = writeln!(w, "operator: treatment H (S5c-m3 memoized dag-v1 scalar solver), uniform fiber weighting, ScalarValuation::trick_only; declared particle-step budget {H_BUDGET}; a None return is the declared stop, printed, never a silent cap.");
    let _ = writeln!(w, "coordinates: rung n=4 (trick 4) hands whose leader IS the declaring seat — H's root must be the viewer's decision; hands where another seat leads are out of this attempt's scope, printed below.");
    let _ = writeln!(
        w,
        "regenerate: cargo run --release -p walt-factory --example fiber_probe h"
    );
    let _ = writeln!(w);
    let mut attempted = 0usize;
    for hand in &r.hands {
        let (_, leader) = state_before_trick(hand, 4).expect("the receipt replays");
        if leader != hand.bidder {
            let _ = writeln!(
                w,
                "  h{}: out of scope (leader is not the declaring seat)",
                hand.id
            );
            continue;
        }
        attempted += 1;
        let (kernel, _) = void_free_kernel(hand, 4);
        let worlds: Vec<[DominoSet; Seat::COUNT]> = kernel.worlds().map(|wd| wd.hands()).collect();
        let solver = ScalarHidden::new(
            hand.decl,
            hand.bidder,
            hand.bidder.team(),
            ScalarValuation::trick_only(),
        );
        let mut budget = H_BUDGET;
        let t = Instant::now();
        let (vals, stats) = solver.action_values_dag(&worlds, leader, &[], &mut budget);
        let ns = t.elapsed().as_nanos();
        match vals {
            Some(v) => {
                let _ = writeln!(
                    w,
                    "  h{}: COMPLETED — {} root actions, {} worlds, {} ns; dag-v1 steps {}, tree-v0 steps {}, boundary hits {}",
                    hand.id,
                    v.len(),
                    worlds.len(),
                    ns,
                    stats.steps,
                    stats.tree_steps,
                    stats.hits
                );
            }
            None => {
                let _ = writeln!(
                    w,
                    "  h{}: STOPPED at the declared budget ({} particle-steps) after {} ns; dag-v1 steps {}, boundary hits {} — the attempt is the record (P-A14)",
                    hand.id, H_BUDGET, ns, stats.steps, stats.hits
                );
            }
        }
    }
    if attempted == 0 {
        let _ = writeln!(w, "  no rung-4 coordinate has the declaring seat leading; the attempt is the record (P-A14)");
    }
    std::fs::write(H_RESULTS, out).expect("results file writes");
    println!("wrote {H_RESULTS}");
}

// ---------------------------------------------------------------------------
// The fiber-refinement probe (`refine`): declared exclusion remnants over the
// class store. Design: walt/FIBER-REFINE.md; binding rulings:
// CENSUS-RULINGS.md "Fiber-refinement rulings (X-Q1..X-Q7)", X-A1..X-A19.
// ---------------------------------------------------------------------------

const REFINE_RESULTS: &str = "results/fiber_refine_2026-08-11.txt";

/// Freeze 12 receipts: the declared flag-receipt stride over class indices.
const FLAG_RECEIPT_STRIDE: usize = 97;

/// One coordinate's class store with its predicate flags. Flags are keyed in
/// spirit by (predicate id, freeze-set id) per X-A6(i); this run computes
/// them in-process under the printed freeze set and persists nothing.
struct RefineStore {
    values: Vec<Q>,
    grades: Vec<usize>,
    /// Per class: the representative's canonical tuples (class-invariant).
    rep: Vec<usize>,
    /// Grade-ascending fold order.
    order: Vec<usize>,
}

fn refine_store(r3: &R3, values: Vec<Q>) -> RefineStore {
    let nclasses = r3.class_members.len();
    let mut order: Vec<usize> = (0..nclasses).collect();
    order.sort_by_key(|&c| r3.class_grade[c]);
    RefineStore {
        values,
        grades: r3.class_grade.clone(),
        rep: (0..nclasses).map(|c| r3.class_members[c][0]).collect(),
        order,
    }
}

/// Freeze 12: F0, declared intensionally — the trick-boundary classes of the
/// last trick (grade 4) whose Lemma-V value is zero: "losing last tricks".
fn f0_flags(store: &RefineStore) -> Vec<bool> {
    store
        .values
        .iter()
        .zip(&store.grades)
        .map(|(v, &g)| g == 4 && *v == qi(0))
        .collect()
}

/// X_val0 — Lemma X's excisable predicate: the class's value is zero.
fn val0_flags(store: &RefineStore) -> Vec<bool> {
    store.values.iter().map(|v| *v == qi(0)).collect()
}

/// X_val_max — bite-only, NOT excisable (one-sided; Lemma X consequence 1):
/// the class attains the maximum trick count its grade allows.
fn valmax_flags(store: &RefineStore) -> Vec<bool> {
    store
        .values
        .iter()
        .zip(&store.grades)
        .map(|(v, &g)| *v == qi(i128::try_from(g.div_ceil(4)).expect("fits")))
        .collect()
}

/// X_reach-exists(F): some continuation passes through F. DP by ascending
/// grade over the class DAG.
fn reach_exists(store: &RefineStore, r3: &R3, f: &[bool]) -> Vec<bool> {
    let mut out = vec![false; store.values.len()];
    for &c in &store.order {
        out[c] = f[c]
            || r3.tuples[store.rep[c]]
                .iter()
                .any(|t| t.successor.is_some_and(|s| out[s]));
    }
    out
}

/// X_conf-forall(F): every continuation passes through F. A move straight to
/// hand end (successor None) escapes F unless the class itself is in F.
fn conf_forall(store: &RefineStore, r3: &R3, f: &[bool]) -> Vec<bool> {
    let mut out = vec![false; store.values.len()];
    for &c in &store.order {
        let tuples = &r3.tuples[store.rep[c]];
        out[c] = f[c]
            || (!tuples.is_empty() && tuples.iter().all(|t| t.successor.is_some_and(|s| out[s])));
    }
    out
}

struct PredicateRow {
    name: &'static str,
    note: &'static str,
    class_bite: usize,
    world_bite: usize,
    ns_pass: u128,
}

#[allow(clippy::too_many_lines)]
fn run_refine() {
    let r = receipt();
    for hand in &r.hands {
        assert!(
            matches!(hand.decl, Decl::PipTrump(_)),
            "F1 scope: pip-trump only"
        );
    }
    let mut out = String::new();
    let w = &mut out;
    let _ = writeln!(w, "walt fiber-refinement probe — declared exclusion remnants over the class store — exploratory tier");
    let _ = writeln!(w, "design: walt/FIBER-REFINE.md; binding rulings: CENSUS-RULINGS.md \"Fiber-refinement rulings (X-Q1..X-Q7)\", X-A1..X-A19, plus the fiber-probe rulings P-A1..P-A21 inherited unchanged");
    let _ = writeln!(w);
    let _ = writeln!(w, "X-A1 (verbatim): A remnant is a declared exclusion remnant of the void-free capacity fiber: analyst conditioning (v0.4 §6.8) on a declared, non-evidential predicate. Exclusion by X does not mean the world cannot occur (that is support, §2.1) and does not mean it is improbable (that is belief, §2.4). No support fact, no belief, no seat value, and no reachability claim may be read from a remnant. Excluding X never places X's falsity into any seat's information state; doing so would be player revelation and would recreate strategy fusion (§6.8, §7.6).");
    let _ = writeln!(w, "X-A4 (verbatim, for every value predicate): Value equality in conclusion 7 is over the transported abstract-policy class, exactly as in v0.4 §12.6's conclusion 4. Whether the unrestricted concrete optimum is attained inside that class is a separate sufficiency question, deliberately not claimed here. With P-A5: the quantity read is world-informed and is not a seat value.");
    let _ = writeln!(w, "X-A8 (verbatim): Bite is measured on the evaluated set of the void-free capacity fiber (P-A1, P-A3). A shrink factor is a statement about a declared cost domain, never about the seat's real support Phi(C), never about belief, and never about what can happen in the game.");
    let _ = writeln!(w, "X-A14 (verbatim): B : A1 ~ 4.3-4.9 at every rung — the class store is not a build accelerator; cone identity cannot short-circuit descent. Every payoff sought here is a pass->=2 transport payoff, and the first build is a cost this probe does not recover.");
    let _ = writeln!(w);
    let _ = writeln!(w, "Lemma X (zero-contribution excision, adjudicated): with the non-negative q_trick valuation, deleting the worlds whose Lemma-V value is zero leaves the unnormalised objective and its argmax exactly unchanged for every information-consistent policy. ONE-SIDED: the value-max dual forces nothing and is never excised (X-A5). The §6.8 rule governs everything below: evaluate a fixed policy on a remnant, never re-optimise over a remnant and call the result a seat value.");
    let _ = writeln!(w);
    let _ = writeln!(w, "freezes: 1-3 and 7-11 in force and unchanged (see fiber_probe_2026-08-11.txt). New: (12) predicate definitions, intensional — F0 = the trick-boundary classes of the last trick (grade 4) with Lemma-V value 0 (\"losing last tricks\"; |F0| printed per coordinate); X_val0 = value 0; X_val_max = value equals the grade's maximum trick count, bite-only, labelled not excisable — one-sided, see Lemma X consequence 1; X_reach-exists(F0) and X_conf-forall(F0) with quantifiers in the name (X-A3); flag-receipt stride {FLAG_RECEIPT_STRIDE} over class indices. (13) flags keyed by (predicate id, freeze-set id) — this run computes all flags in-process under this header's freeze set and persists nothing. (14) the store record format: DEFERRED — persistence is not implemented this run (declared); X-A16..X-A19 govern it when it is built.");
    let _ = writeln!(w, "coincidence note (X-A3): the ruling's X_conf-forall(zero-trick terminals) is not expressible as a class set in this encoding (trick outcomes live on edges; hand-end is one terminal), and its content is carried by X_val0 directly. X_conf-forall(F0) here is the distinct \"every continuation loses its LAST trick\" predicate.");
    let _ = writeln!(w, "storeless alternatives (X-A13): X_val0 is decidable without the class store (arm A1 per-world values; timed below). X_reach-exists and X_conf-forall are NOT decidable on bare semantic-state keys — a state key carries no cone identity; that is S5h's finding. X_val_max is decidable storelessly the same way as X_val0.");
    let _ = writeln!(w, "argmax clause of X-A6(iii): no action-valued policy solve is run in this probe, so the argmax-agreement clause has no object here (declared); the objective-agreement clause is asserted below.");
    let _ = writeln!(w, "timing discipline (P-A19): one run, one machine, single-threaded; integer ns; exact fixed-point ratios. CPU: {}; build profile: release; threads: 1.", cpu_model());
    let _ = writeln!(w, "provenance: SINGLE-IMPLEMENTATION; regenerate: cargo run --release -p walt-factory --example fiber_probe refine");
    let _ = writeln!(w, "declared stops of this run: rungs n=4 (13 hands, g=7919, W=240) and n=5 (4 hands, g=104729, W=24); n=6 omitted this run (X-A10: a six-point sample supports no bite ratio). Bites on decimated sets are ESTIMATED (X-A10); integers always printed.");
    let _ = writeln!(w);

    for rung in &RUNGS[..2] {
        let _ = writeln!(
            w,
            "================ rung n={} (trick {}) — g={}, W={}; hands {:?} ================",
            rung.n, rung.trick_no, rung.g, rung.w, rung.hands
        );
        for &h in rung.hands {
            let hand = &r.hands[h];
            let (kernel, leader) = void_free_kernel(hand, rung.trick_no);
            let (_, worlds) = select_worlds(&kernel, rung.g, rung.w);
            let wcount = worlds.len();

            // Pass 1: the store (arm B, unchanged machinery).
            let (b, carrier, r3) = run_b_full(&kernel, leader, &worlds);
            let store = refine_store(&r3, b.class_values.clone());

            // Predicates (pass 2 objects).
            let t = Instant::now();
            let f0 = f0_flags(&store);
            let ns_f0 = t.elapsed().as_nanos();
            let f0_count = f0.iter().filter(|x| **x).count();

            let mut rows: Vec<PredicateRow> = Vec::new();
            let t = Instant::now();
            let val0 = val0_flags(&store);
            let ns_val0 = t.elapsed().as_nanos();
            let t = Instant::now();
            let valmax = valmax_flags(&store);
            let ns_valmax = t.elapsed().as_nanos();
            let t = Instant::now();
            let reach = reach_exists(&store, &r3, &f0);
            let ns_reach = t.elapsed().as_nanos();
            let t = Instant::now();
            let conf = conf_forall(&store, &r3, &f0);
            let ns_conf = t.elapsed().as_nanos();

            for (name, note, flags, ns_pass) in [
                ("X_val0", "excisable (Lemma X)", &val0, ns_val0),
                (
                    "X_val_max",
                    "not excisable — one-sided; see Lemma X consequence 1",
                    &valmax,
                    ns_valmax,
                ),
                (
                    "X_reach-exists(F0)",
                    "cost-domain predicate",
                    &reach,
                    ns_reach,
                ),
                ("X_conf-forall(F0)", "cost-domain predicate", &conf, ns_conf),
            ] {
                rows.push(PredicateRow {
                    name,
                    note,
                    class_bite: flags.iter().filter(|x| **x).count(),
                    world_bite: b.world_class.iter().filter(|&&c| flags[c]).count(),
                    ns_pass,
                });
            }

            // X-A13: the storeless alternative for the value predicates — one
            // A1 pass over the same worlds, thresholding per-world values.
            let decl = kernel.decl();
            let focal = kernel.viewer();
            let t = Instant::now();
            let mut memo: BTreeMap<u128, Q> = BTreeMap::new();
            let mut edges: u64 = 0;
            let storeless_val0: usize = worlds
                .iter()
                .map(|hs| Situation {
                    decl,
                    focal,
                    leader,
                    hands: *hs,
                    table: Vec::new(),
                })
                .filter(|sit| value_a1(sit, &mut memo, &mut edges) == qi(0))
                .count();
            let ns_storeless = t.elapsed().as_nanos();

            // X-A6(ii): flag receipt — recompute sampled classes' values from
            // the cone through the independent A1 path and assert flags.
            let mut receipt_checked = 0usize;
            for c in (0..store.values.len()).step_by(FLAG_RECEIPT_STRIDE) {
                let sit = &carrier.states[store.rep[c]];
                let v = value_a1(sit, &mut memo, &mut edges);
                assert_eq!(v, store.values[c], "flag receipt: cone recompute (X-A6 ii)");
                assert_eq!(v == qi(0), val0[c], "flag receipt: X_val0 (X-A6 ii)");
                receipt_checked += 1;
            }

            // X-A6(iii): the exercised Lemma-X excision and its receipt.
            let excluded: Vec<usize> = (0..wcount).filter(|&i| val0[b.world_class[i]]).collect();
            for &i in &excluded {
                assert_eq!(
                    b.per_world[i],
                    qi(0),
                    "Lemma X receipt: excluded world has U = 0 (X-A6 iii)"
                );
            }
            let sum_full = b.per_world.iter().fold(qi(0), |a, v| a + *v);
            let t = Instant::now();
            let sum_remnant = (0..wcount)
                .filter(|&i| !val0[b.world_class[i]])
                .fold(qi(0), |a, i| a + b.per_world[i]);
            let ns_remnant_eval = t.elapsed().as_nanos();
            assert_eq!(
                sum_full, sum_remnant,
                "Lemma X receipt: unnormalised objective agrees (X-A6 iii)"
            );

            let _ = writeln!(
                w,
                "  h{h}: store {} classes over {} situations (pass-1 build {} ns, S5h arm B unchanged); |F0| = {} of {} grade-4 boundary classes; evaluated set W = {wcount}",
                b.classes,
                b.carrier_len,
                b.total_ns(),
                f0_count,
                store.grades.iter().filter(|&&g| g == 4).count()
            );
            let _ = writeln!(w, "      F0 marking pass: {ns_f0} ns");
            for row in &rows {
                let _ = writeln!(
                    w,
                    "      {}: class bite {} / {} (ESTIMATED ratio {}), world bite {} / {} (ESTIMATED ratio {}), predicate pass {} ns [{}]",
                    row.name,
                    row.class_bite,
                    b.classes,
                    fp(u128::try_from(row.class_bite).expect("fits"), u128::try_from(b.classes).expect("fits")),
                    row.world_bite,
                    wcount,
                    fp(u128::try_from(row.world_bite).expect("fits"), u128::try_from(wcount).expect("fits")),
                    row.ns_pass,
                    row.note
                );
            }
            let _ = writeln!(
                w,
                "      pass-2 economics (X-A13): value predicates storeless via one A1 pass = {ns_storeless} ns (found {storeless_val0} val0 worlds, agreeing with the store's {}); store-side value flag pass = {ns_val0} ns; reachability/confinement have NO storeless alternative (cone identity is not on a state key) and cost {ns_reach} / {ns_conf} ns over the built store",
                rows[0].world_bite
            );
            assert_eq!(storeless_val0, rows[0].world_bite, "X-A13 agreement");
            let _ = writeln!(
                w,
                "      Lemma-X exercise (X-A6 iii): {} of {wcount} worlds excised; per-world U = 0 asserted on ALL excised worlds; unnormalised objective full = remnant asserted; remnant summation over the pass-1 store (reused, X-A15): {ns_remnant_eval} ns — exclusion saves nothing at evaluation time once the store is paid, and that is a result (F7)",
                excluded.len()
            );
            let _ = writeln!(
                w,
                "      flag receipt (X-A6 ii): {receipt_checked} classes at stride {FLAG_RECEIPT_STRIDE} recomputed from the cone through the independent A1 path; values and flags agree"
            );
            eprintln!(
                "refine n={} h{h}: {} classes, val0 world bite {}/{wcount}, reach {}/{} classes",
                rung.n, b.classes, rows[0].world_bite, rows[2].class_bite, b.classes
            );
        }
        let _ = writeln!(w);
    }
    let _ = writeln!(w, "Both outcomes remain results (F7, NO-RESCUE): a nil bite, or a bite that saves nothing at evaluation time, is a proved negative about declared-exclusion refinement on this route and changes nothing about the classes, their ECL receipts, or Lemma X.");
    let _ = writeln!(w, "run complete: yes");
    std::fs::write(REFINE_RESULTS, out).expect("results file writes");
    println!("wrote {REFINE_RESULTS}");
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("h") => run_h(),
        Some("refine") => run_refine(),
        _ => run_ladder(),
    }
}
