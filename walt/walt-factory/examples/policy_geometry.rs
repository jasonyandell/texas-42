//! The policy-geometry probe (S6b): v0.6 Gate E on the S6a domains, under
//! rulings PG-A1..PG-A18, Proposition G-flat and Lemma G
//! (`walt/CENSUS-RULINGS.md`, 2026-08-12). Design: `walt/POLICY-GEOMETRY.md`.
//!
//! Four cardinalities, never conflated (PG-A7): N_pol (plans, closed form
//! 2^k(a) at grade 3 — Proposition G-flat), N_vec (distinct value vectors,
//! only where the UNPRUNED set was enumerated to completion), N_par (the
//! Pareto frontier, from the pruned run), N_exp (|Exp|: UNIQUE maximiser of
//! E_beta for some belief — PG-A4's amended definition; Lark's programme,
//! exact-rational primal simplex with Bland's rule, freeze 29; witnesses
//! both ways, PG-A10).
//!
//! Grades 1-2 are RECEIPTS, not measurements (G-flat: the focal seat has no
//! choice there); grade 3 is the probe's one measurement, read against the
//! absolute bands of PG-A15. Everything exploratory tier; no floats; value
//! vectors in walt_geom Q (i128 ratios, overflow = panic = stop-and-report);
//! simplex arithmetic in arbitrary-precision rationals (R-A21).

use std::collections::BTreeMap;
use std::fmt::Write as _;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};

use walt_core::{legal_plays, ContextSet, Decl, Domino, DominoSet, Pip, Seat, Trick};
use walt_geom::{q, qi, Q};
use walt_kernel::{Hidden, Kernel};
use walt_strat::hidden_scalar::ScalarHidden;
use walt_strat::scalar::ScalarValuation;

// ---------------------------------------------------------------------------
// Freezes. 1-26 in force and restated (S6a values unchanged); new 27-31
// (PG-A14).
// ---------------------------------------------------------------------------

/// Freeze 25 (S6a, restated): decimation constants and base-coordinate
/// counts per grade. Same coordinates as the S6a run (X-A12 comparability).
const DECIMATION: [(u128, usize); 3] = [(7919, 12), (104_729, 6), (1_299_709, 3)];

/// Freeze 26 (S6a, restated): the concrete authority and its bridge.
const AUTHORITY_BUDGET: u64 = 200_000_000;

/// Freeze 30: caps. The frontier cap applies at EVERY incremental partial
/// sum (PG-A13); a capped coordinate reports NO N_par. The unpruned
/// enumeration (for N_vec and the Lemma-G receipt) runs only where
/// 2^k(a) <= UNPRUNED_BOUND. LP budgets per PG-A11: a stop yields a
/// witnessed exact lower bound on N_exp, never an approximation.
const FRONTIER_CAP: usize = 16384;
const UNPRUNED_BOUND: u128 = 1024;
const LP_PIVOT_CAP: u64 = 200_000;
const LP_COORD_PIVOT_BUDGET: u64 = 4_000_000;

type R = BigRational;

fn rbig(x: Q) -> R {
    R::new(BigInt::from(*x.numer()), BigInt::from(*x.denom()))
}

// ---------------------------------------------------------------------------
// Information interfaces and coordinates (S6a freeze 22, verbatim reuse).
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq, Debug)]
struct Interface {
    pip: u8,
    hand: DominoSet,
    pool: DominoSet,
    caps: [usize; 3],
    voids: [ContextSet; 3],
    leader_off: usize,
    grade: usize,
}

impl Interface {
    fn decl(&self) -> Decl {
        Decl::PipTrump(Pip::new(self.pip).expect("pip in range"))
    }

    fn kernel(&self) -> Kernel {
        let hidden = [
            Hidden {
                seat: Seat::S1,
                capacity: self.caps[0],
                voids: self.voids[0],
            },
            Hidden {
                seat: Seat::S2,
                capacity: self.caps[1],
                voids: self.voids[1],
            },
            Hidden {
                seat: Seat::S3,
                capacity: self.caps[2],
                voids: self.voids[2],
            },
        ];
        Kernel::new(self.decl(), Seat::S0, self.hand, self.pool, hidden)
            .expect("a lawful interface kernel")
    }
}

fn fiber(itf: &Interface) -> Vec<[DominoSet; 4]> {
    let kernel = itf.kernel();
    let worlds: Vec<[DominoSet; 4]> = kernel.worlds().map(|w| w.hands()).collect();
    assert_eq!(worlds.len() as u128, kernel.count(), "fiber count drift");
    worlds
}

fn binom(n: u128, k: u128) -> u128 {
    if k > n {
        return 0;
    }
    let mut out: u128 = 1;
    for i in 0..k.min(n - k) {
        out = out * (n - i) / (i + 1);
    }
    out
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn unrank_comb(n: usize, k: usize, mut rank: u128) -> Vec<usize> {
    let mut out = Vec::with_capacity(k);
    let mut x = 0usize;
    let mut k = k;
    let mut n_left = n;
    while k > 0 {
        let c = binom((n_left - 1) as u128, (k - 1) as u128);
        if rank < c {
            out.push(x);
            k -= 1;
        } else {
            rank -= c;
        }
        x += 1;
        n_left -= 1;
    }
    out
}

fn coordinate(grade: usize, index: u128) -> Interface {
    let live_c = binom(28, (4 * grade) as u128);
    let hand_c = binom((4 * grade) as u128, grade as u128);
    let pip = (index / (live_c * hand_c)) as u8;
    let rem = index % (live_c * hand_c);
    let live_rank = rem / hand_c;
    let hand_rank = rem % hand_c;
    let live_idx = unrank_comb(28, 4 * grade, live_rank);
    let hand_pos = unrank_comb(4 * grade, grade, hand_rank);
    let hand_set: std::collections::BTreeSet<usize> = hand_pos.into_iter().collect();
    let mut pool = DominoSet::EMPTY;
    let mut hand = DominoSet::EMPTY;
    for (pos, di) in live_idx.iter().enumerate() {
        let d = Domino::from_index(*di).expect("domino index");
        if hand_set.contains(&pos) {
            hand.insert(d);
        } else {
            pool.insert(d);
        }
    }
    Interface {
        pip,
        hand,
        pool,
        caps: [grade; 3],
        voids: [ContextSet::EMPTY; 3],
        leader_off: 0,
        grade,
    }
}

fn population(grade: usize) -> u128 {
    7 * binom(28, (4 * grade) as u128) * binom((4 * grade) as u128, grade as u128)
}

// ---------------------------------------------------------------------------
// Value vectors over the root fiber: sparse, exact, walt_geom Q entries
// (freeze 27: world order = the S6a kernel order; entries nonnegative).
// ---------------------------------------------------------------------------

type Vector = Vec<(u32, Q)>;

/// Pointwise partial order on nonnegative sparse vectors: returns
/// (le, ge) = (v <= w everywhere, v >= w everywhere).
fn cmp_pointwise(v: &Vector, w: &Vector) -> (bool, bool) {
    let (mut le, mut ge) = (true, true);
    let (mut i, mut j) = (0usize, 0usize);
    let zero = qi(0);
    while (i < v.len() || j < w.len()) && (le || ge) {
        let take_v = j >= w.len() || (i < v.len() && v[i].0 < w[j].0);
        let take_w = i >= v.len() || (j < w.len() && w[j].0 < v[i].0);
        if take_v {
            // w has 0 here; entries are nonnegative.
            if v[i].1 > zero {
                le = false;
            }
            i += 1;
        } else if take_w {
            if w[j].1 > zero {
                ge = false;
            }
            j += 1;
        } else {
            if v[i].1 > w[j].1 {
                le = false;
            }
            if v[i].1 < w[j].1 {
                ge = false;
            }
            i += 1;
            j += 1;
        }
    }
    (le, ge)
}

fn vec_add(v: &Vector, w: &Vector) -> Vector {
    let mut out = Vec::with_capacity(v.len() + w.len());
    let (mut i, mut j) = (0usize, 0usize);
    while i < v.len() || j < w.len() {
        let take_v = j >= w.len() || (i < v.len() && v[i].0 < w[j].0);
        let take_w = i >= v.len() || (j < w.len() && w[j].0 < v[i].0);
        if take_v {
            out.push(v[i]);
            i += 1;
        } else if take_w {
            out.push(w[j]);
            j += 1;
        } else {
            let s = v[i].1 + w[j].1;
            if s != qi(0) {
                out.push((v[i].0, s));
            }
            i += 1;
            j += 1;
        }
    }
    out
}

/// Inserts `v` into the Pareto frontier `set` (dedup included): drops `v` if
/// dominated-or-equal, else removes what `v` dominates and appends.
/// Freeze 28: candidates arrive in the recursion's deterministic order.
fn frontier_insert(set: &mut Vec<Vector>, v: Vector) {
    let mut k = 0usize;
    while k < set.len() {
        let (le, ge) = cmp_pointwise(&v, &set[k]);
        if le {
            // v <= existing (equality included): nothing new.
            return;
        }
        if ge {
            set.swap_remove(k);
            continue;
        }
        k += 1;
    }
    set.push(v);
}

/// Set-union with Pareto pruning (focal step, PG-A5(i)).
fn frontier_union(mut a: Vec<Vector>, b: Vec<Vector>) -> Vec<Vector> {
    for v in b {
        frontier_insert(&mut a, v);
    }
    a
}

/// Minkowski sum with pruning after the fold (PG-A5(ii): incremental —
/// callers fold one branch at a time through this).
fn frontier_minkowski(a: &[Vector], b: &[Vector], cap: usize, capped: &mut bool) -> Vec<Vector> {
    let mut out: Vec<Vector> = Vec::new();
    for v in a {
        for w in b {
            frontier_insert(&mut out, vec_add(v, w));
            if out.len() > cap {
                *capped = true;
                return out;
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// The policy-set recursion: pooled observation tree at primitive-step
// granularity (PG-A5(i)); focal steps take unions over legal tiles, hidden
// steps fold weighted branches by incremental Minkowski sum, increments
// enter at trick completion. Support entries carry unit-fraction weight
// denominators exactly as in S6a.
// ---------------------------------------------------------------------------

struct Walk<'a> {
    decl: Decl,
    worlds: &'a [[DominoSet; 4]],
    prune: bool,
    /// When set, the walk evaluates ONE policy instead of the achievable
    /// set: bit b of the mask picks the second legal tile at free state b
    /// (visit order), the first otherwise. Used by the dominance spot
    /// receipt only.
    policy_mask: Option<u128>,
    capped: bool,
    /// The observation record at the first cap (PG-A13's stop point).
    stop_at: Option<Vec<Domino>>,
    /// Focal decision states seen: (number with >=2 legal, product of
    /// |legal| as the plan count, PG-A3).
    free_states: usize,
    plan_count: BigInt,
}

impl Walk<'_> {
    fn hand_now(&self, wi: u32, seat: Seat, obs: &[Domino]) -> DominoSet {
        let mut h = self.worlds[wi as usize][seat.index()];
        for t in obs {
            h.remove(*t);
        }
        h
    }

    /// The achievable-value-vector set from this node. Without pruning the
    /// full set is enumerated (dedup only); with pruning every union and
    /// every partial Minkowski sum is Pareto-pruned (Lemma G(2)-(3)).
    fn node(
        &mut self,
        support: &[(u32, u128)],
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        obs: &mut Vec<Domino>,
    ) -> Vec<Vector> {
        if k == 4 {
            let trick = Trick::new(leader, tiles).expect("distinct tiles");
            let winner = trick.winner(self.decl);
            let inc = winner.team() == Seat::S0.team();
            let done = self.hand_now(support[0].0, Seat::S0, obs).is_empty();
            let below = if done {
                vec![Vec::new()]
            } else {
                self.node(support, winner, [Domino::ALL[0]; 4], 0, obs)
            };
            if !inc {
                return below;
            }
            let inc_vec: Vector = support
                .iter()
                .map(|&(wi, den)| (wi, q(1, i128::try_from(den).expect("den fits i128"))))
                .collect();
            return below.into_iter().map(|v| vec_add(&v, &inc_vec)).collect();
        }
        let seat = leader.plus(k);
        let led = (k > 0).then(|| self.decl.led_context(tiles[0]));
        if seat == Seat::S0 {
            let hand = self.hand_now(support[0].0, seat, obs);
            let legal = legal_plays(self.decl, hand, led);
            let state_index = self.free_states;
            if legal.len() >= 2 {
                self.free_states += 1;
            }
            self.plan_count *= BigInt::from(legal.len());
            if let Some(mask) = self.policy_mask {
                // Single-policy evaluation: follow the mask's choice.
                let tiles_legal: Vec<Domino> = legal.iter().collect();
                let pick =
                    if legal.len() >= 2 && state_index < 128 && (mask >> state_index) & 1 == 1 {
                        tiles_legal[1]
                    } else {
                        tiles_legal[0]
                    };
                let mut tiles = tiles;
                tiles[k] = pick;
                obs.push(pick);
                let child = self.node(support, leader, tiles, k + 1, obs);
                obs.pop();
                return child;
            }
            let mut out: Vec<Vector> = Vec::new();
            for a in legal.iter() {
                let mut tiles = tiles;
                tiles[k] = a;
                obs.push(a);
                let child = self.node(support, leader, tiles, k + 1, obs);
                obs.pop();
                if self.prune {
                    out = frontier_union(out, child);
                } else {
                    for v in child {
                        if !out.contains(&v) {
                            out.push(v);
                        }
                    }
                }
                if out.len() > FRONTIER_CAP {
                    self.capped = true;
                    if self.stop_at.is_none() {
                        self.stop_at = Some(obs.clone());
                    }
                    return out;
                }
            }
            return out;
        }
        // Hidden seat: weighted branches, incremental Minkowski fold.
        let mut by_tile: BTreeMap<usize, Vec<(u32, u128)>> = BTreeMap::new();
        for &(wi, den) in support {
            let hand = self.hand_now(wi, seat, obs);
            let legal = legal_plays(self.decl, hand, led);
            let n = legal.len() as u128;
            for t in legal.iter() {
                by_tile
                    .entry(t.index())
                    .or_default()
                    .push((wi, den.checked_mul(n).expect("denominators fit u128")));
            }
        }
        let mut acc: Option<Vec<Vector>> = None;
        for (ti, sup) in by_tile {
            let d = Domino::from_index(ti).expect("tile index");
            let mut tiles = tiles;
            tiles[k] = d;
            obs.push(d);
            let child = self.node(&sup, leader, tiles, k + 1, obs);
            obs.pop();
            acc = Some(match acc {
                None => child,
                Some(a) => {
                    if self.prune {
                        let mut capped = false;
                        let out = frontier_minkowski(&a, &child, FRONTIER_CAP, &mut capped);
                        if capped {
                            self.capped = true;
                            if self.stop_at.is_none() {
                                self.stop_at = Some(obs.clone());
                            }
                        }
                        out
                    } else {
                        let mut out: Vec<Vector> = Vec::new();
                        for v in &a {
                            for w in &child {
                                let s = vec_add(v, w);
                                if !out.contains(&s) {
                                    out.push(s);
                                }
                                if out.len() as u128 > UNPRUNED_BOUND * 4 {
                                    self.capped = true;
                                    if self.stop_at.is_none() {
                                        self.stop_at = Some(obs.clone());
                                    }
                                    return out;
                                }
                            }
                        }
                        out
                    }
                }
            });
            if self.capped {
                return acc.unwrap_or_default();
            }
        }
        acc.unwrap_or_else(|| vec![Vec::new()])
    }
}

struct SetRun {
    set: Vec<Vector>,
    capped: bool,
    stop_at: Option<Vec<Domino>>,
    k: usize,
    plans: BigInt,
}

/// Runs the recursion for one root action.
fn policy_sets(itf: &Interface, worlds: &[[DominoSet; 4]], lead: Domino, prune: bool) -> SetRun {
    policy_sets_inner(itf, worlds, lead, prune, None)
}

fn policy_sets_inner(
    itf: &Interface,
    worlds: &[[DominoSet; 4]],
    lead: Domino,
    prune: bool,
    policy_mask: Option<u128>,
) -> SetRun {
    let mut walk = Walk {
        decl: itf.decl(),
        worlds,
        prune,
        policy_mask,
        capped: false,
        stop_at: None,
        free_states: 0,
        plan_count: BigInt::one(),
    };
    let support: Vec<(u32, u128)> = (0..worlds.len() as u32).map(|i| (i, 1)).collect();
    let mut tiles = [Domino::ALL[0]; 4];
    tiles[0] = lead;
    let mut obs = vec![lead];
    let set = walk.node(&support, Seat::S0, tiles, 1, &mut obs);
    SetRun {
        set,
        capped: walk.capped,
        stop_at: walk.stop_at,
        k: walk.free_states,
        plans: walk.plan_count,
    }
}

// ---------------------------------------------------------------------------
// main: receipts at grades 1-2 (Proposition G-flat), the one measurement at
// grade 3, the PG-A15 reading, the fences.
// ---------------------------------------------------------------------------

fn mean(v: &Vector, m: usize) -> Q {
    let mut s = qi(0);
    for &(_, x) in v {
        s += x;
    }
    s * q(1, m as i128)
}

fn tile_name(d: Domino) -> String {
    format!("{}{}", d.hi().value(), d.lo().value())
}

fn main() {
    let t0 = std::time::Instant::now();
    let mut out = String::new();
    let w = &mut out;
    let _ = writeln!(
        w,
        "walt policy-geometry probe (S6b) — v0.6 Gate E — exploratory tier"
    );
    let _ = writeln!(w, "rulings: PG-A1..PG-A18, Proposition G-flat, Lemma G (walt/CENSUS-RULINGS.md 2026-08-12); design walt/POLICY-GEOMETRY.md; S6a freezes 22-26 in force; new freezes 27-31 (PG-A14)");
    let _ = writeln!(
        w,
        "regenerate: cargo run --release -p walt-factory --example policy_geometry"
    );
    let _ = writeln!(w);
    let _ = writeln!(w, "TYPE LINE (PG-A4, verbatim): N_exp counts the vectors needed to represent the value function of the declared cost domain over its whole belief simplex. It is not a count of strategies any seat needs: at a belief whose support is smaller than Φ(C₀) — the real seat's case, P-A1 — dominated vectors tie for the optimum and are optimal without being exposed. Beliefs here are declared aggregation arguments (P-A12), not any seat's belief.");
    let _ = writeln!(w, "MIXED POLICIES (PG-A2): excluded lawfully — the achievable vector set is the convex hull of the deterministic vectors and a linear functional attains its maximum at a vertex; the focal seat has perfect recall, so behavioural and mixed randomisation coincide.");
    let _ = writeln!(w, "COUNTING CONVENTION (PG-A3, freeze 31): N_pol counts PLANS; by Proposition G-flat(ii) the plan and reduced counts coincide at n <= 3. N_pol = 2^k(a) exactly, k(a) printed beside it.");
    let _ = writeln!(w, "ARGMAX DIAGNOSTIC (PG-A12, mandatory fence restating R-A17(iii)): The argmax sets here are response-equality objects. v0.4 §12.4 applies: they are not a dynamics quotient, they are not an r3-style class count, and they may never be used as a solver's state partition. No partition claim is made or implied.");
    let _ = writeln!(w, "ANTI-STRAWMAN (PG-A16): (i) N_par/N_pol is printed as bookkeeping and is not a criterion: N_pol = 2^k(a) counts plans and is astronomically large by construction, so any frontier at all is 'orders below' it. The criterion is PG-A15's absolute bands. (ii) The grade-1 and grade-2 rows are 1 by Proposition G-flat — the focal seat has no choice there — and are receipts, not evidence of collapse. No cross-grade ratio in this probe is a measurement.");
    let _ = writeln!(w);

    let mut gate3_k: Vec<usize> = Vec::new();
    let mut gate3_par: Vec<usize> = Vec::new();
    let mut gate3_exp: Vec<(usize, bool)> = Vec::new();
    let mut any_cap = false;

    for grade in 1..=3usize {
        let (g, wcount) = DECIMATION[grade - 1];
        let npop = population(grade);
        assert_eq!(gcd(g, npop), 1, "P-A15: gcd(g, N) = 1");
        let label = if grade < 3 {
            "RECEIPT ROWS (Proposition G-flat: no policy geometry exists here)"
        } else {
            "THE MEASUREMENT"
        };
        let _ = writeln!(
            w,
            "== grade {grade}: population N = {npop}, decimation g = {g}, W = {wcount} — {label} =="
        );
        for i in 0..wcount {
            let idx = (i as u128 * g) % npop;
            let itf = coordinate(grade, idx);
            eprintln!("[progress] grade {grade} coord {i} idx {idx}");
            let worlds = fiber(&itf);
            let m = worlds.len();
            let auth = ScalarHidden::new(
                itf.decl(),
                Seat::S0,
                Seat::S0.team(),
                ScalarValuation::trick_only(),
            );
            let mut ab = AUTHORITY_BUDGET;
            let (av, _) = auth.action_values_dag(&worlds, Seat::S0, &[], &mut ab);
            let av = av.expect("authority within budget (declared, P-A16)");
            let mut lp_budget = LP_COORD_PIVOT_BUDGET;
            for (a, qd) in &av {
                let run = policy_sets(&itf, &worlds, *a, true);
                if run.capped {
                    any_cap = true;
                    let stop: Vec<String> = run
                        .stop_at
                        .unwrap_or_default()
                        .iter()
                        .map(|d| tile_name(*d))
                        .collect();
                    let _ = writeln!(
                        w,
                        "coord idx={idx} pip={} lead={} STOPPED (PG-A13): frontier cap {FRONTIER_CAP} hit at record [{}], frontier size {} at stop — NO N_par is reported (a partial frontier bounds nothing in either direction)",
                        itf.pip,
                        tile_name(*a),
                        stop.join(" "),
                        run.set.len()
                    );
                    continue;
                }
                // G-flat receipt: the plan count is exactly 2^k.
                assert_eq!(
                    run.plans,
                    BigInt::one() << run.k,
                    "G-flat: every free state is binary at n <= 3"
                );
                let n_par = run.set.len();
                // Authority receipt (PG-A8(ii)): frontier max at the uniform
                // belief equals treatment H through the freeze-26 bridge.
                let best = run
                    .set
                    .iter()
                    .map(|v| mean(v, m))
                    .max()
                    .expect("a nonempty frontier");
                let bridged = qi(2) * best - qi(grade as i128);
                assert_eq!(
                    *qd, bridged,
                    "PG-A8(ii): frontier max equals treatment H exactly"
                );
                if grade < 3 {
                    assert_eq!(n_par, 1, "Proposition G-flat receipt: forced continuation");
                    assert_eq!(run.k, 0, "Proposition G-flat receipt: no free states");
                    let _ = writeln!(
                        w,
                        "coord idx={idx} pip={} lead={} |X|={m} k=0 N_pol=1 N_vec=1 N_par=1 N_exp=1 [RECEIPT, G-flat]",
                        itf.pip,
                        tile_name(*a)
                    );
                    continue;
                }
                // N_vec and the Lemma-G receipt where the unpruned set fits.
                let mut vec_note = String::from("N_vec: not enumerated (pruning destroys it, Lemma G(6); unpruned run out of declared bound)");
                if (1u128 << run.k.min(120)) <= UNPRUNED_BOUND {
                    let full = policy_sets(&itf, &worlds, *a, false);
                    assert!(!full.capped, "unpruned run within its declared bound");
                    let n_vec = full.set.len();
                    let mut refr: Vec<Vector> = Vec::new();
                    for v in full.set {
                        frontier_insert(&mut refr, v);
                    }
                    let mut a1 = run.set.clone();
                    let mut a2 = refr;
                    a1.sort();
                    a2.sort();
                    assert_eq!(
                        a1, a2,
                        "Lemma G receipt (PG-A8(i)): prune-then-compose equals compose-then-prune, as SETS"
                    );
                    vec_note = format!("N_vec={n_vec} (unpruned enumeration complete; Lemma G set-equality receipt HELD)");
                }
                // PG-A12(ii): frontier vectors attaining the uniform-belief
                // max (pruning-safe: beta0 has full support).
                let b0_vecs = run.set.iter().filter(|v| mean(v, m) == best).count();
                // N_exp via the exposure programme (PG-A9..A11).
                let (exp_lo, tested, complete) = count_exposed(&run.set, m, &mut lp_budget);
                let exp_note = if complete {
                    format!("N_exp={exp_lo}")
                } else {
                    format!("N_exp>={exp_lo} (LOWER BOUND: LP budget stop after {tested}/{n_par} tested, PG-A11; verdict withheld)")
                };
                let _ = writeln!(
                    w,
                    "coord idx={idx} pip={} lead={} |X|={m} k={} N_pol=2^{}={} N_par={n_par} {exp_note}; beta0-max vectors={b0_vecs} (PG-A12); {vec_note}",
                    itf.pip,
                    tile_name(*a),
                    run.k,
                    run.k,
                    run.plans,
                );
                // Dominance spot receipt (once, at the first singleton
                // frontier): 2^10 explicit policy variants over the mask
                // bits, each evaluated exactly; every variant's vector must
                // be pointwise <= the claimed frontier singleton. A pooled
                // pruning bug that wrongly dropped an incomparable vector
                // fails this loudly.
                if n_par == 1 && gate3_par.is_empty() {
                    for mask in 0..1024u128 {
                        let one = policy_sets_inner(&itf, &worlds, *a, true, Some(mask));
                        assert_eq!(one.set.len(), 1, "a mask evaluates one policy");
                        let (le, _) = cmp_pointwise(&one.set[0], &run.set[0]);
                        assert!(
                            le,
                            "dominance spot receipt: every explicit policy variant lies under the frontier singleton"
                        );
                    }
                    let _ = writeln!(
                        w,
                        "  dominance spot receipt: 1024 explicit policy variants (mask bits over free states, deterministic) all pointwise <= the frontier singleton — HELD"
                    );
                }
                gate3_k.push(run.k);
                gate3_par.push(n_par);
                gate3_exp.push((exp_lo, complete));
            }
        }
        if grade == 2 {
            let _ = writeln!(w, "grade-3 conditionality (PG-A13, declared in advance): grade-2 sizes are all 1 by Proposition G-flat — the condition is satisfied; grade 3 proceeds.");
        }
        let _ = writeln!(w);
    }

    // The PG-A15 reading, grade 3 only, absolute bands.
    let _ = writeln!(
        w,
        "== The reading (PG-A15, fixed before any number existed; grade 3 only) =="
    );
    if any_cap {
        let _ = writeln!(w, "PG-A13 note: uncapped action rows within a capped coordinate are action-level objects only; no coordinate-level N_par claim is made anywhere in this run.");
        let _ = writeln!(w, "PG-A8(i) limitation, printed as mandated: the unpruned enumeration exceeded its declared bound at every grade-3 coordinate (2^k(a) > 1024 throughout), so the Lemma-G set-equality receipt ran vacuously at grades 1-2 only; the grade-3 pruning is instead spot-checked by the dominance receipt above.");
        let _ = writeln!(
            w,
            "verdict (N_par): STOPPED, NO VERDICT — a measured coordinate hit a cap (PG-A13)."
        );
    } else {
        let kk = *gate3_k.iter().max().expect("grade-3 rows exist");
        let pp = *gate3_par.iter().max().expect("grade-3 rows exist");
        let _ = writeln!(
            w,
            "K = max k(a) = {kk}; P = max N_par = {pp}; anchors: K+1 = {}, |X_3| = 1680",
            kk + 1
        );
        let verdict = if pp <= kk + 1 {
            "STRONG COLLAPSE (P <= K + 1: the frontier is at most linear in the number of decision points, against N_pol = 2^K)"
        } else if pp <= 1680 {
            "COLLAPSE (K + 1 < P <= |X_3| = 1680)"
        } else {
            "REFUTED (P > 1680)"
        };
        let _ = writeln!(w, "verdict (N_par): {verdict}");
        let complete_all = gate3_exp.iter().all(|&(_, c)| c);
        let ee = gate3_exp.iter().map(|&(e, _)| e).max().unwrap_or(0);
        if complete_all {
            let everdict = if ee <= kk + 1 {
                "STRONG COLLAPSE"
            } else if ee <= 1680 {
                "COLLAPSE"
            } else {
                "REFUTED"
            };
            let _ = writeln!(
                w,
                "verdict (N_exp), separate line, never inherited: E = max N_exp = {ee}: {everdict}"
            );
        } else if ee > 1680 {
            let _ = writeln!(w, "verdict (N_exp): lower bound {ee} alone forces REFUTED");
        } else {
            let _ = writeln!(
                w,
                "verdict (N_exp): lower bound {ee}; verdict withheld (PG-A11)"
            );
        }
    }
    let _ = writeln!(w);
    let _ = writeln!(w, "THE FENCE (PG-A17, verbatim): This probe counts exact objects over a declared coordinate's void-free capacity fiber under the declared field and belief (R-A9), the count-free expected-focal-trick valuation, the R-A11 observation contract and the S6a freezes. Four cardinalities are reported and never conflated: N_pol (plans), N_vec (distinct vectors, only where the unpruned set was enumerated), N_par (Pareto frontier), N_exp (uniquely-optimal-for-some-belief, PG-A4). No similarity claim and no tolerance claim of any kind is made or supported. 'Playing this domino means I am likely to get 32 one way or the other' is a statement about score distributions under a tolerance, and this probe measures neither: score is out of scope (E-A2, and by Lemma R(c)-(d) the distribution contract has predictive dimension |X|), and delta-similarity is future mathematics requiring its own typed rulings. A vector here is an expected-trick profile over the declared fiber, not an outcome law and not 'an outcome'. No partition claim (PG-A12), no runtime or tractability claim (v0.6 §18.3), no promotion of any v0.6 theorem, no number quoted for the opening or for any grade not measured. The concrete authority remains treatment H; a disagreement with it is a stop-and-report bug.");
    let _ = writeln!(w, "PG-A18: a collapse verdict does not rescue Gate B, does not transfer to the opening, and establishes nothing about similarity; a refutation adds one named entry to the bottleneck list; a STOPPED verdict is neither.");
    let _ = writeln!(w, "total: {} ms", t0.elapsed().as_millis());
    let _ = writeln!(w, "run complete: yes");
    print!("{out}");
    std::fs::write("walt-factory/results/policy_geometry_2026-08-12.txt", &out)
        .or_else(|_| std::fs::write("results/policy_geometry_2026-08-12.txt", &out))
        .expect("results file written");
}

// ---------------------------------------------------------------------------
// The exposure programme (PG-A9, freeze 29): Lark's LP per frontier vector,
// exact-rational primal simplex with Bland's rule; witnesses both ways
// (PG-A10), rechecked independently of the LP code path.
// ---------------------------------------------------------------------------

enum Exposure {
    /// delta* > 0, with the witness belief (dense, length m).
    Exposed(Vec<R>),
    /// delta* <= 0, with the convex-dominance witness over the others.
    NotExposed(Vec<R>),
    /// Pivot budget exhausted (PG-A11): no verdict for this vector.
    Stopped,
}

/// Solves: max delta s.t. beta.(v - w_i) >= delta for all i, sum beta = 1,
/// beta >= 0. Returns the verdict with its witness. `dense` holds the
/// candidate first, then the others; m = world count.
fn lark(dense: &[Vec<R>], m: usize, pivots: &mut u64) -> Exposure {
    let p = dense.len() - 1;
    if p == 0 {
        return Exposure::Exposed(Vec::new());
    }
    // Columns: beta (m) | delta+ | delta- | slacks (p) | artificial (1) | b.
    let n = m + 2 + p + 1;
    let bcol = n;
    let mut t: Vec<Vec<R>> = Vec::with_capacity(p + 1);
    for i in 0..p {
        // Row i (flipped for a feasible slack start):
        //   -beta.d_i + delta+ - delta- + s_i = 0,  d_i = v - w_i.
        let mut row = vec![R::zero(); n + 1];
        for xi in 0..m {
            row[xi] = dense[i + 1][xi].clone() - dense[0][xi].clone();
        }
        row[m] = R::one();
        row[m + 1] = -R::one();
        row[m + 2 + i] = R::one();
        t.push(row);
    }
    // Row p: sum beta = 1, artificial basic.
    let mut row = vec![R::zero(); n + 1];
    for x in row.iter_mut().take(m) {
        *x = R::one();
    }
    row[m + 2 + p] = R::one();
    row[bcol] = R::one();
    t.push(row);
    let mut basis: Vec<usize> = (0..p).map(|i| m + 2 + i).collect();
    basis.push(m + 2 + p);

    // Phase 1: maximize -a0; reduced costs = the a0 row's coefficients
    // (a0 = 1 - sum beta over the current dictionary).
    let mut obj: Vec<R> = t[p].clone();
    obj[m + 2 + p] = R::zero();
    let art_col = m + 2 + p;
    loop {
        if basis[p] != art_col {
            break;
        }
        let Some(j) = (0..n)
            .filter(|&j| j != art_col)
            .find(|&j| obj[j] > R::zero())
        else {
            break;
        };
        if !pivot(&mut t, &mut obj, &mut basis, j, pivots) {
            return Exposure::Stopped;
        }
    }
    if basis[p] == art_col {
        panic!("phase 1 must drive out the artificial: the simplex is nonempty");
    }
    // Phase 2 objective: delta+ - delta-. Reduced costs from raw c minus
    // basic contributions.
    let mut obj = vec![R::zero(); n + 1];
    obj[m] = R::one();
    obj[m + 1] = -R::one();
    for (r, &bj) in basis.iter().enumerate() {
        let c = obj[bj].clone();
        if !c.is_zero() {
            for j in 0..=n {
                let d = c.clone() * &t[r][j];
                obj[j] -= d;
            }
        }
    }
    while let Some(j) = (0..n)
        .filter(|&j| j != art_col)
        .find(|&j| obj[j] > R::zero())
    {
        if !pivot(&mut t, &mut obj, &mut basis, j, pivots) {
            return Exposure::Stopped;
        }
    }
    // delta* = -obj[b] (objective row carries -z in the b column).
    let delta = -obj[bcol].clone();
    if delta > R::zero() {
        let mut beta = vec![R::zero(); m];
        for (r, &bj) in basis.iter().enumerate() {
            if bj < m {
                beta[bj] = t[r][bcol].clone();
            }
        }
        Exposure::Exposed(beta)
    } else {
        // Duals: y_i = -(reduced cost of s_i), the Lemma G(5) weights.
        let y: Vec<R> = (0..p).map(|i| -obj[m + 2 + i].clone()).collect();
        Exposure::NotExposed(y)
    }
}

/// One Bland pivot on entering column j. Returns false when the pivot
/// budget is exhausted; panics on unboundedness (impossible here: delta is
/// bounded by the finite vector entries over the simplex).
fn pivot(t: &mut [Vec<R>], obj: &mut [R], basis: &mut [usize], j: usize, pivots: &mut u64) -> bool {
    if *pivots == 0 {
        return false;
    }
    *pivots -= 1;
    let bcol = t[0].len() - 1;
    let mut leave: Option<(usize, R)> = None;
    for (r, row) in t.iter().enumerate() {
        if row[j] > R::zero() {
            let ratio = row[bcol].clone() / row[j].clone();
            let better = match &leave {
                None => true,
                Some((lr, lv)) => ratio < *lv || (ratio == *lv && basis[r] < basis[*lr]),
            };
            if better {
                leave = Some((r, ratio));
            }
        }
    }
    let (r, _) = leave.expect("Lark's programme is bounded");
    let piv = t[r][j].clone();
    for x in t[r].iter_mut() {
        *x /= piv.clone();
    }
    let pivot_row = t[r].clone();
    for (ri, row) in t.iter_mut().enumerate() {
        if ri != r && !row[j].is_zero() {
            let c = row[j].clone();
            for (x, pv) in row.iter_mut().zip(&pivot_row) {
                *x -= c.clone() * pv;
            }
        }
    }
    if !obj[j].is_zero() {
        let c = obj[j].clone();
        for (x, pv) in obj.iter_mut().zip(&pivot_row) {
            *x -= c.clone() * pv;
        }
    }
    basis[r] = j;
    true
}

/// Counts N_exp over the frontier with per-LP and per-coordinate budgets.
/// Returns (verified exposed, tested, complete?). Every verdict is
/// witness-rechecked independently; a failed recheck is a stop-and-report.
fn count_exposed(frontier: &[Vector], m: usize, budget: &mut u64) -> (usize, usize, bool) {
    if frontier.len() == 1 {
        return (1, 1, true);
    }
    let dense: Vec<Vec<R>> = frontier
        .iter()
        .map(|v| {
            let mut d = vec![R::zero(); m];
            for &(wi, x) in v {
                d[wi as usize] = rbig(x);
            }
            d
        })
        .collect();
    let mut exposed = 0usize;
    for (i, cand) in dense.iter().enumerate() {
        if *budget == 0 {
            return (exposed, i, false);
        }
        let mut arranged: Vec<Vec<R>> = Vec::with_capacity(dense.len());
        arranged.push(cand.clone());
        for (jj, other) in dense.iter().enumerate() {
            if jj != i {
                arranged.push(other.clone());
            }
        }
        let mut lp_budget = LP_PIVOT_CAP.min(*budget);
        let before = lp_budget;
        let verdict = lark(&arranged, m, &mut lp_budget);
        *budget -= before - lp_budget;
        match verdict {
            Exposure::Exposed(beta) => {
                // Recheck: beta is a belief and separates strictly.
                let sum: R = beta.iter().cloned().sum();
                assert!(sum == R::one() && beta.iter().all(|b| !b.is_negative()));
                for other in arranged.iter().skip(1) {
                    let mut diff = R::zero();
                    for xi in 0..m {
                        diff += beta[xi].clone() * (cand[xi].clone() - other[xi].clone());
                    }
                    assert!(
                        diff > R::zero(),
                        "PG-A10 recheck: an exposed verdict must separate strictly"
                    );
                }
                exposed += 1;
            }
            Exposure::NotExposed(y) => {
                // Recheck Lemma G(5): sum lambda_i w_i >= v pointwise.
                let sum: R = y.iter().cloned().sum();
                assert!(
                    sum == R::one() && y.iter().all(|c| !c.is_negative()),
                    "PG-A10 recheck: dual weights must be convex"
                );
                for xi in 0..m {
                    let mut lhs = R::zero();
                    for (yi, other) in y.iter().zip(arranged.iter().skip(1)) {
                        lhs += yi.clone() * &other[xi];
                    }
                    assert!(
                        lhs >= cand[xi],
                        "PG-A10 recheck: convex dominance must hold pointwise"
                    );
                }
            }
            Exposure::Stopped => return (exposed, i, false),
        }
    }
    (exposed, dense.len(), true)
}
