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
    BArm {
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
    }
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

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("h") => run_h(),
        _ => run_ladder(),
    }
}
