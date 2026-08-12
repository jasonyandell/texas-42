//! The predictive-rank probe, part one (S6a): the dimension census of the
//! value closure V^val at grades 1-3, under the rulings R-A1..R-A24 and
//! Lemma R / Corollary R-fold (`walt/CENSUS-RULINGS.md`, 2026-08-12).
//! Design: `walt/PREDICTIVE-RANK.md`. Basis: `walt/math/predictive_algebra_v0.6.md`
//! (v0.6 track, exploratory) consumed as design guidance only.
//!
//! What is measured (R-A16): dim V^val for the count-free expected-focal-trick
//! contract (Lemma R(3)) — terminal seed = the ZERO space, no constant is ever
//! a generator — and, alongside it, contract (i'): (i) plus the expected
//! next-leader-offset readouts for offsets 0 and 2 only. Offsets 1 and 3 are
//! deliberately excluded: all four offsets together sum to the constant 1 and
//! re-enter Lemma R(c)'s degeneracy. The distribution contracts are THEOREM
//! rows (r = |X|, Lemma R(c)-(d)) and are not run.
//!
//! Everything exploratory tier. No object here is an identity-bearing witness
//! of anything (R-A2). No floats; exact arithmetic throughout: world weights
//! are unit fractions held as u128 denominators, closure vectors are
//! arbitrary-precision rationals (BigRational, R-A21); a boundary conversion
//! that would overflow walt_geom::Q is a stop-and-report, never a truncation.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;
use std::rc::Rc;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

use walt_core::{legal_plays, Context, ContextSet, Decl, Domino, DominoSet, Pip, Seat, Trick};
use walt_kernel::{Hidden, Kernel};
use walt_strat::hidden_scalar::ScalarHidden;
use walt_strat::scalar::ScalarValuation;

// ---------------------------------------------------------------------------
// Freezes 22-26 (R-A22). Freezes 1-21 are in force and restated in the
// rulings; nothing here reuses a spent number.
// ---------------------------------------------------------------------------

/// Freeze 25: multiplicative decimation constants per grade (P-A15 form:
/// indices {i*g mod N}, gcd(g, N) = 1 asserted in-run), and the declared
/// base-coordinate counts W. Every base coordinate is evaluated at all 7
/// pip declarations (Corollary R-fold receipt, R-A7).
const DECIMATION: [(u128, usize); 3] = [(7919, 12), (104_729, 6), (1_299_709, 3)];

/// Freeze 26: the concrete authority is walt-strat `ScalarHidden`
/// `action_values_dag` (dag-v1), valuation `trick_only` (focal-minus-opponent,
/// +-1 per trick), uniform fiber weighting, observation contract = the full
/// public record; the affine bridge to this probe's count convention is
/// Q_diff = 2 * Q_count - grade. Tie rule for the extracted policy: least
/// domino index among the argmax.
const AUTHORITY_BUDGET: u64 = 200_000_000;

/// Declared per-grade candidate-generator budget (P-A16 discipline), the
/// unit being one successor-basis pullback: a grade whose closures would
/// push more candidates than this stops the run with a printed declaration,
/// never silently. A backstop against runaway dimensions, not a tuning knob.
const GENERATOR_BUDGET: usize = 200_000_000;

type R = BigRational;
/// A sparse exact vector over a fiber, entries sorted by world index.
type SVec = Vec<(u32, R)>;

fn rint(n: i128) -> R {
    R::from_integer(BigInt::from(n))
}

fn rden(den: u128) -> R {
    R::new(BigInt::one(), BigInt::from(den))
}

// ---------------------------------------------------------------------------
// Information interfaces (R-A4; freeze 22 encoding).
// ---------------------------------------------------------------------------

/// The typed focal information interface of R-A4, void-free at rung roots
/// (P-A2), successors carrying the Lemma S-det induced voids. Focal is
/// always seat S0; hidden offsets 1..3 are seats S1..S3. O_Sigma = empty and
/// monitor = none are declared; no accumulated outcome is carried (F5).
#[derive(Clone, PartialEq, Eq, Debug)]
struct Interface {
    pip: u8,
    hand: DominoSet,
    pool: DominoSet,
    caps: [usize; 3],
    voids: [ContextSet; 3],
    /// Leader as an offset from focal in {0,1,2,3} (r3 Q3).
    leader_off: usize,
    grade: usize,
}

impl Interface {
    fn decl(&self) -> Decl {
        Decl::PipTrump(Pip::new(self.pip).expect("pip in range"))
    }

    fn key(&self) -> [u64; 3] {
        let mut v: u64 = 0;
        for (i, c) in self.voids.iter().enumerate() {
            let mut bits: u64 = 0;
            for q in c.iter() {
                bits |= 1 << q.index();
            }
            v |= bits << (8 * i);
        }
        [
            u64::from(self.hand.bits()) | (u64::from(self.pool.bits()) << 32),
            v | ((self.leader_off as u64) << 32)
                | ((self.grade as u64) << 40)
                | (u64::from(self.pip) << 48),
            ((self.caps[0] as u64) << 16) | ((self.caps[1] as u64) << 8) | (self.caps[2] as u64),
        ]
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

/// The fiber in kernel enumeration order (freeze 7 reuse, R-A19): each world
/// as the four remaining hands, focal's identical across the fiber.
fn fiber(itf: &Interface) -> Vec<[DominoSet; 4]> {
    let kernel = itf.kernel();
    let worlds: Vec<[DominoSet; 4]> = kernel.worlds().map(|w| w.hands()).collect();
    assert_eq!(worlds.len() as u128, kernel.count(), "fiber count drift");
    assert!(!worlds.is_empty(), "an interface fiber is nonempty");
    worlds
}

// ---------------------------------------------------------------------------
// Exact sparse linear algebra (R-A21; freeze 23 pivot discipline).
// ---------------------------------------------------------------------------

/// An exact row-echelon basis over a fiber: rows in insertion order, each
/// with a pivot column (its first nonzero world index, leading coefficient
/// normalized to 1), pivots distinct. Freeze 23: pivot = first nonzero in
/// the kernel world enumeration order.
struct Echelon {
    rows: Vec<SVec>,
    pivot_of_row: Vec<u32>,
    row_of_pivot: HashMap<u32, usize>,
}

impl Echelon {
    fn new() -> Echelon {
        Echelon {
            rows: Vec::new(),
            pivot_of_row: Vec::new(),
            row_of_pivot: HashMap::new(),
        }
    }

    fn dim(&self) -> usize {
        self.rows.len()
    }

    /// Reduces `v` against the basis; returns the remainder (possibly empty)
    /// and, when `coords` is given, records the multiplier applied to each
    /// basis row.
    fn reduce(&self, mut v: SVec, mut coords: Option<&mut Vec<(usize, R)>>) -> SVec {
        loop {
            let Some(&(col, _)) = v.first() else {
                return v;
            };
            let Some(&row) = self.row_of_pivot.get(&col) else {
                return v;
            };
            let c = v[0].1.clone();
            axpy(&mut v, &c, &self.rows[row]);
            debug_assert!(v.first().map(|&(j, _)| j > col).unwrap_or(true));
            if let Some(out) = coords.as_deref_mut() {
                out.push((row, c));
            }
        }
    }

    /// Reduces and, if independent, inserts the normalized remainder.
    /// Returns true when the dimension grew.
    fn absorb(&mut self, v: SVec) -> bool {
        let rem = self.reduce(v, None);
        let Some((piv, lead)) = rem.first().map(|(j, c)| (*j, c.clone())) else {
            return false;
        };
        let norm: SVec = rem
            .into_iter()
            .map(|(j, c)| (j, c / lead.clone()))
            .collect();
        self.row_of_pivot.insert(piv, self.rows.len());
        self.pivot_of_row.push(piv);
        self.rows.push(norm);
        true
    }

    fn nnz(&self) -> usize {
        self.rows.iter().map(Vec::len).sum()
    }
}

/// dst -= c * src, sparse merge.
fn axpy(dst: &mut SVec, c: &R, src: &[(u32, R)]) {
    let mut out: SVec = Vec::with_capacity(dst.len() + src.len());
    let (mut i, mut j) = (0usize, 0usize);
    while i < dst.len() || j < src.len() {
        let take_dst = j >= src.len() || (i < dst.len() && dst[i].0 < src[j].0);
        let take_src = i >= dst.len() || (j < src.len() && src[j].0 < dst[i].0);
        if take_dst {
            out.push(dst[i].clone());
            i += 1;
        } else if take_src {
            let val = -(c.clone() * &src[j].1);
            if !val.is_zero() {
                out.push((src[j].0, val));
            }
            j += 1;
        } else {
            let val = dst[i].1.clone() - c.clone() * &src[j].1;
            if !val.is_zero() {
                out.push((dst[i].0, val));
            }
            i += 1;
            j += 1;
        }
    }
    *dst = out;
}

// ---------------------------------------------------------------------------
// Record enumeration: one trick from an interface, at primitive-step
// granularity (R-A8), weights as unit-fraction denominators.
// ---------------------------------------------------------------------------

/// One complete trick record from an interface: the four tiles in play
/// order, the surviving worlds with their field-weight denominators, the
/// derived labels, and the successor interface (freeze 24: e := o, gamma
/// derived).
struct Record {
    tiles: [Domino; 4],
    /// (world index in the interface fiber, product of |legal| denominators).
    support: Vec<(u32, u128)>,
    /// Count-free increment: focal team took the trick (P-A6, R-A15).
    inc: bool,
    winner_off: usize,
    /// gamma (R-A13): (leader offset, follow/slough per follower, increment).
    gamma: (usize, [bool; 3], bool),
    succ: Interface,
    /// The focal lead choice when the interface is focal-lead (the B-bundle
    /// index (s, a) collapses to a: focal plays once, at its fixed position).
    focal_choice: Domino,
    /// The focal within-trick information state: the observed prefix before
    /// focal's turn (empty at focal-lead interfaces).
    focal_prefix: Vec<Domino>,
}

fn focal_position(leader_off: usize) -> usize {
    (4 - leader_off) % 4
}

/// Enumerates every complete one-trick record from `itf` over `worlds`,
/// depth-first in ascending tile order at every branch (freeze 22 order).
fn enumerate_records(itf: &Interface, worlds: &[[DominoSet; 4]]) -> Vec<Record> {
    let decl = itf.decl();
    let leader = Seat::S0.plus(itf.leader_off);
    let mut out = Vec::new();
    let all: Vec<(u32, u128)> = (0..worlds.len() as u32).map(|i| (i, 1u128)).collect();
    let mut played: Vec<[DominoSet; 4]> = worlds.to_vec();
    let _ = &mut played;
    rec_step(
        itf,
        decl,
        leader,
        worlds,
        &all,
        [Domino::ALL[0]; 4],
        0,
        &mut out,
    );
    out
}

#[allow(clippy::too_many_arguments)]
fn rec_step(
    itf: &Interface,
    decl: Decl,
    leader: Seat,
    worlds: &[[DominoSet; 4]],
    support: &[(u32, u128)],
    tiles: [Domino; 4],
    k: usize,
    out: &mut Vec<Record>,
) {
    if k == 4 {
        out.push(finish_record(itf, decl, leader, worlds, support, tiles));
        return;
    }
    let seat = leader.plus(k);
    let led = (k > 0).then(|| decl.led_context(tiles[0]));
    // Remaining hand of `seat` in world `w` at this point of the trick.
    let hand_now = |ws: &[DominoSet; 4]| {
        let mut h = ws[seat.index()];
        for t in tiles.iter().take(k) {
            h.remove(*t);
        }
        h
    };
    if seat == Seat::S0 {
        // The focal seat: a public choice, weight 1 (R-A8's controller is a
        // restriction here, never an enumeration of U(i)).
        let hand = hand_now(&worlds[support[0].0 as usize]);
        let legal = legal_plays(decl, hand, led);
        for a in legal.iter() {
            let mut tiles = tiles;
            tiles[k] = a;
            rec_step(itf, decl, leader, worlds, support, tiles, k + 1, out);
        }
        return;
    }
    // A hidden seat: chance over its world-relative legal set (F4).
    let mut by_tile: BTreeMap<usize, Vec<(u32, u128)>> = BTreeMap::new();
    for &(wi, den) in support {
        let hand = hand_now(&worlds[wi as usize]);
        let legal = legal_plays(decl, hand, led);
        let n = legal.len() as u128;
        for t in legal.iter() {
            by_tile.entry(t.index()).or_default().push((
                wi,
                den.checked_mul(n).expect("weight denominators fit u128"),
            ));
        }
    }
    for (ti, sup) in by_tile {
        let mut tiles = tiles;
        tiles[k] = Domino::from_index(ti).expect("tile index");
        rec_step(itf, decl, leader, worlds, &sup, tiles, k + 1, out);
    }
}

fn finish_record(
    itf: &Interface,
    decl: Decl,
    leader: Seat,
    _worlds: &[[DominoSet; 4]],
    support: &[(u32, u128)],
    tiles: [Domino; 4],
) -> Record {
    let trick = Trick::new(leader, tiles).expect("distinct tiles by construction");
    let winner = trick.winner(decl);
    let winner_off = (winner.index() + 4 - Seat::S0.index()) % 4;
    let inc = winner.team() == Seat::S0.team();
    let led = decl.led_context(tiles[0]);
    let mut follow = [true; 3];
    let mut voids = itf.voids;
    let mut hand = itf.hand;
    let mut pool = itf.pool;
    let mut caps = itf.caps;
    for (k, &t) in tiles.iter().enumerate() {
        let seat = leader.plus(k);
        if seat == Seat::S0 {
            assert!(hand.remove(t), "focal plays a focal tile");
        } else {
            assert!(pool.remove(t), "a hidden play is a pool tile");
            let off = (seat.index() + 4 - Seat::S0.index()) % 4;
            caps[off - 1] -= 1;
            if k > 0 {
                let follows = decl.follows(t, led);
                follow[k - 1] = follows;
                if !follows {
                    // Lemma S-det: a slough proves a void in the led context
                    // and proves nothing else.
                    voids[off - 1].insert(led);
                }
            }
        }
    }
    // gamma's follower flags are indexed by play position 1..3 regardless of
    // which is focal; the focal follower's flag is public and kept.
    let mut gflags = [true; 3];
    for k in 1..4 {
        gflags[k - 1] = decl.follows(tiles[k], led);
    }
    let succ = Interface {
        pip: itf.pip,
        hand,
        pool,
        caps,
        voids,
        leader_off: winner_off,
        grade: itf.grade - 1,
    };
    let fp = focal_position(itf.leader_off);
    Record {
        tiles,
        support: support.to_vec(),
        inc,
        winner_off,
        gamma: (itf.leader_off, gflags, inc),
        succ,
        focal_choice: tiles[fp],
        focal_prefix: tiles[..fp].to_vec(),
    }
}

// ---------------------------------------------------------------------------
// The value closure (Lemma R(3), R-A16; freeze 23 generator discipline).
// ---------------------------------------------------------------------------

/// Which contract the closure carries (R-A16). `Prime` adds the expected
/// next-leader-offset readouts for offsets 0 and 2 as immediate generator
/// families at every interface.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Contract {
    ExpectedTricks,
    Prime,
}

struct Closure {
    worlds: Vec<[DominoSet; 4]>,
    index_of: BTreeMap<[u32; 4], u32>,
    basis: Echelon,
    behavioral_rows: usize,
}

type Memo = HashMap<[u64; 3], Rc<Closure>>;

fn closure(
    itf: &Interface,
    contract: Contract,
    memo: &mut Memo,
    budget: &mut usize,
) -> Rc<Closure> {
    let key = itf.key();
    if let Some(c) = memo.get(&key) {
        return Rc::clone(c);
    }
    let worlds = fiber(itf);
    let index_of: BTreeMap<[u32; 4], u32> = worlds
        .iter()
        .enumerate()
        .map(|(i, w)| (pack_world(w), i as u32))
        .collect();
    let mut basis = Echelon::new();
    if itf.grade > 0 {
        let records = enumerate_records(itf, &worlds);
        // B-bundles per focal within-trick information state s and choice a:
        // B_{s,a} = sum over records through (s,a) of c_o * w_o (and the
        // offset-readout variants). span{g_u : u} = span({g_{u0}} union
        // {B_{s,a} - B_{s,u0(s)}}) since focal plays exactly once per trick.
        let mut bundles: BTreeMap<(Vec<Domino>, Domino), [SVec; 3]> = BTreeMap::new();
        let mut candidates: Vec<SVec> = Vec::new();
        for rec in &records {
            let entry = bundles
                .entry((rec.focal_prefix.clone(), rec.focal_choice))
                .or_insert_with(|| [Vec::new(), Vec::new(), Vec::new()]);
            let flags = [rec.inc, rec.winner_off == 0, rec.winner_off == 2];
            for (slot, on) in flags.iter().enumerate() {
                if *on {
                    for &(wi, den) in &rec.support {
                        add_entry(&mut entry[slot], wi, rden(den));
                    }
                }
            }
            // Residual generators w_o * iota_o(f) for each successor basis
            // vector f (the successor closure is memoized).
            let succ = closure(&rec.succ, contract, memo, budget);
            *budget = budget
                .checked_sub(succ.basis.dim().max(1))
                .unwrap_or_else(|| panic!("GENERATOR BUDGET EXCEEDED (declared stop, P-A16)"));
            // Lemma S-det receipt: the surviving worlds of a record biject
            // with the successor interface fiber.
            assert_eq!(
                rec.support.len(),
                succ.worlds.len(),
                "S-det receipt: record-consistent worlds must biject with the successor fiber"
            );
            for f in &succ.basis.rows {
                let mut v: SVec = Vec::new();
                for &(wi, den) in &rec.support {
                    let sw = succ_world(&worlds[wi as usize], &rec.tiles);
                    let si = *succ
                        .index_of
                        .get(&pack_world(&sw))
                        .expect("S-det receipt: a surviving world lands in the successor fiber");
                    if let Ok(fi) = f.binary_search_by_key(&si, |&(j, _)| j) {
                        add_entry(&mut v, wi, f[fi].1.clone() * rden(den));
                    }
                }
                if !v.is_empty() {
                    candidates.push(v);
                }
            }
        }
        // The g-family: g_{u0} plus differences (freeze 23; u0 = least legal
        // tile per focal state), for the contract's readout slots.
        let slots: &[usize] = match contract {
            Contract::ExpectedTricks => &[0],
            Contract::Prime => &[0, 1, 2],
        };
        let mut states: BTreeMap<Vec<Domino>, Vec<Domino>> = BTreeMap::new();
        for (s, a) in bundles.keys() {
            states.entry(s.clone()).or_default().push(*a);
        }
        for &slot in slots {
            let mut g0: SVec = Vec::new();
            let mut diffs: Vec<SVec> = Vec::new();
            for (s, choices) in &states {
                let a0 = *choices.iter().min().expect("a focal state has a choice");
                let b0 = bundles[&(s.clone(), a0)][slot].clone();
                sv_add(&mut g0, &b0);
                for a in choices {
                    if *a == a0 {
                        continue;
                    }
                    // d = B_{s,a} - B_{s,u0(s)}.
                    let mut d = bundles[&(s.clone(), *a)][slot].clone();
                    let one = R::one();
                    axpy(&mut d, &one, &b0);
                    diffs.push(d);
                }
            }
            if !g0.is_empty() {
                candidates.push(g0);
            }
            candidates.extend(diffs.into_iter().filter(|v| !v.is_empty()));
        }
        for v in candidates {
            basis.absorb(v);
        }
    }
    let behavioral_rows = count_behavioral_rows(worlds.len(), &basis);
    let out = Rc::new(Closure {
        worlds,
        index_of,
        basis,
        behavioral_rows,
    });
    memo.insert(key, Rc::clone(&out));
    out
}

/// dst += src (axpy with c = -1: dst -= (-1) * src).
fn sv_add(dst: &mut SVec, src: &[(u32, R)]) {
    let mone = -R::one();
    axpy(dst, &mone, src);
}

fn add_entry(v: &mut SVec, wi: u32, val: R) {
    match v.binary_search_by_key(&wi, |&(j, _)| j) {
        Ok(i) => {
            v[i].1 += val;
            if v[i].1.is_zero() {
                v.remove(i);
            }
        }
        Err(i) => v.insert(i, (wi, val)),
    }
}

fn pack_world(w: &[DominoSet; 4]) -> [u32; 4] {
    [w[0].bits(), w[1].bits(), w[2].bits(), w[3].bits()]
}

fn succ_world(w: &[DominoSet; 4], tiles: &[Domino; 4]) -> [DominoSet; 4] {
    let mut out = *w;
    for h in &mut out {
        for t in tiles {
            h.remove(*t);
        }
    }
    out
}

/// Distinct rows of the |X| x dim basis-evaluation matrix (R-A17(i)): two
/// worlds agree on every continuation test iff they agree on a basis.
fn count_behavioral_rows(nworlds: usize, basis: &Echelon) -> usize {
    let mut per_world: Vec<Vec<(usize, String)>> = vec![Vec::new(); nworlds];
    for (ri, row) in basis.rows.iter().enumerate() {
        for (wi, val) in row {
            per_world[*wi as usize].push((ri, val.to_string()));
        }
    }
    let distinct: BTreeSet<Vec<(usize, String)>> = per_world.into_iter().collect();
    distinct.len()
}

// ---------------------------------------------------------------------------
// Treatment H, this probe's side: pooled information-consistent solve over
// the root fiber under the F4 uniform-legal field, count convention
// c in {0,1}, choices recorded per observation record (freeze 26 tie rule:
// least domino index among the argmax).
// ---------------------------------------------------------------------------

struct HSide<'a> {
    decl: Decl,
    worlds: &'a [[DominoSet; 4]],
}

impl HSide<'_> {
    fn hand_now(&self, wi: u32, seat: Seat, obs: &[Domino]) -> DominoSet {
        let mut h = self.worlds[wi as usize][seat.index()];
        for t in obs {
            h.remove(*t);
        }
        h
    }

    /// Unnormalized pooled value: sum over surviving worlds of
    /// (1/den) * (expected focal-team trick count below), with viewer
    /// maxima at pooled observation records.
    fn node(
        &self,
        support: &[(u32, u128)],
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        obs: &mut Vec<Domino>,
        choices: &mut BTreeMap<Vec<Domino>, Domino>,
    ) -> R {
        if k == 4 {
            let trick = Trick::new(leader, tiles).expect("distinct tiles");
            let winner = trick.winner(self.decl);
            let inc = winner.team() == Seat::S0.team();
            let mass: R = support.iter().map(|&(_, den)| rden(den)).sum();
            let banked = if inc { mass } else { R::zero() };
            let done = self.hand_now(support[0].0, Seat::S0, obs).is_empty();
            if done {
                return banked;
            }
            return banked + self.node(support, winner, [Domino::ALL[0]; 4], 0, obs, choices);
        }
        let seat = leader.plus(k);
        let led = (k > 0).then(|| self.decl.led_context(tiles[0]));
        if seat == Seat::S0 {
            let hand = self.hand_now(support[0].0, seat, obs);
            let legal = legal_plays(self.decl, hand, led);
            let mut best: Option<(R, Domino)> = None;
            for a in legal.iter() {
                let mut tiles = tiles;
                tiles[k] = a;
                obs.push(a);
                let v = self.node(support, leader, tiles, k + 1, obs, choices);
                obs.pop();
                let better = match &best {
                    None => true,
                    Some((bv, _)) => v > *bv,
                };
                if better {
                    best = Some((v, a));
                }
            }
            let (v, a) = best.expect("a viewer node has a legal move");
            choices.insert(obs.clone(), a);
            return v;
        }
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
        let mut sum = R::zero();
        for (ti, sup) in by_tile {
            let d = Domino::from_index(ti).expect("tile index");
            let mut tiles = tiles;
            tiles[k] = d;
            obs.push(d);
            sum += self.node(&sup, leader, tiles, k + 1, obs, choices);
            obs.pop();
        }
        sum
    }

    /// Expected focal-team trick count for ONE world under the fixed
    /// extracted policy and the uniform-legal field.
    fn eval_policy(
        &self,
        wi: u32,
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        obs: &mut Vec<Domino>,
        choices: &BTreeMap<Vec<Domino>, Domino>,
    ) -> R {
        if k == 4 {
            let trick = Trick::new(leader, tiles).expect("distinct tiles");
            let winner = trick.winner(self.decl);
            let inc = if winner.team() == Seat::S0.team() {
                R::one()
            } else {
                R::zero()
            };
            if self.hand_now(wi, Seat::S0, obs).is_empty() {
                return inc;
            }
            return inc + self.eval_policy(wi, winner, [Domino::ALL[0]; 4], 0, obs, choices);
        }
        let seat = leader.plus(k);
        let led = (k > 0).then(|| self.decl.led_context(tiles[0]));
        if seat == Seat::S0 {
            let a = *choices
                .get(obs.as_slice())
                .expect("the pooled walk visited every record this world can produce");
            let mut tiles = tiles;
            tiles[k] = a;
            obs.push(a);
            let v = self.eval_policy(wi, leader, tiles, k + 1, obs, choices);
            obs.pop();
            return v;
        }
        let hand = self.hand_now(wi, seat, obs);
        let legal = legal_plays(self.decl, hand, led);
        let share = rden(legal.len() as u128);
        let mut sum = R::zero();
        for d in legal.iter() {
            let mut tiles = tiles;
            tiles[k] = d;
            obs.push(d);
            sum += share.clone() * self.eval_policy(wi, leader, tiles, k + 1, obs, choices);
            obs.pop();
        }
        sum
    }

    /// The world-informed focal-max value of ONE world (the P-A6 aggregate
    /// semantics): the strategy-fusion upper-bound side of R-A10's one-sided
    /// diagnostic. Labelled per P-A5: not a seat value.
    fn fused(
        &self,
        wi: u32,
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        obs: &mut Vec<Domino>,
    ) -> R {
        if k == 4 {
            let trick = Trick::new(leader, tiles).expect("distinct tiles");
            let winner = trick.winner(self.decl);
            let inc = if winner.team() == Seat::S0.team() {
                R::one()
            } else {
                R::zero()
            };
            if self.hand_now(wi, Seat::S0, obs).is_empty() {
                return inc;
            }
            return inc + self.fused(wi, winner, [Domino::ALL[0]; 4], 0, obs);
        }
        let seat = leader.plus(k);
        let led = (k > 0).then(|| self.decl.led_context(tiles[0]));
        let hand = self.hand_now(wi, seat, obs);
        let legal = legal_plays(self.decl, hand, led);
        if seat == Seat::S0 {
            let mut best: Option<R> = None;
            for a in legal.iter() {
                let mut tiles = tiles;
                tiles[k] = a;
                obs.push(a);
                let v = self.fused(wi, leader, tiles, k + 1, obs);
                obs.pop();
                if best.as_ref().map(|b| v > *b).unwrap_or(true) {
                    best = Some(v);
                }
            }
            return best.expect("a focal node has a legal move");
        }
        let share = rden(legal.len() as u128);
        let mut sum = R::zero();
        for d in legal.iter() {
            let mut tiles = tiles;
            tiles[k] = d;
            obs.push(d);
            sum += share.clone() * self.fused(wi, leader, tiles, k + 1, obs);
            obs.pop();
        }
        sum
    }
}

// ---------------------------------------------------------------------------
// Corollary R-fold transport (Lemma S-fold's phi restricted to the live set).
// ---------------------------------------------------------------------------

/// Lemma S-fold's pi_{p->p2}: p maps to p2 and the remaining six pips map by
/// the unique order isomorphism (NOT a transposition — the order isomorphism
/// is what preserves within-context ranking). The §1.3 tier-0 comparison
/// reading is inert at this probe's level: a trick winner never compares two
/// tier-0 tiles (the led tile always follows its own context).
fn sigma_pip(x: u8, p: u8, p2: u8) -> u8 {
    if x == p {
        return p2;
    }
    let idx = if x < p { x } else { x - 1 };
    if idx < p2 {
        idx
    } else {
        idx + 1
    }
}

fn transport_tile(d: Domino, p: u8, p2: u8) -> Domino {
    let a = sigma_pip(d.hi().value(), p, p2);
    let b = sigma_pip(d.lo().value(), p, p2);
    let (hi, lo) = if a >= b { (a, b) } else { (b, a) };
    Domino::new(Pip::new(hi).expect("pip"), Pip::new(lo).expect("pip"))
}

fn transport_set(s: DominoSet, p: u8, p2: u8) -> DominoSet {
    let mut out = DominoSet::EMPTY;
    for d in s.iter() {
        out.insert(transport_tile(d, p, p2));
    }
    out
}

fn transport_interface(itf: &Interface, p2: u8) -> Interface {
    let p = itf.pip;
    let mut voids = [ContextSet::EMPTY; 3];
    for (i, c) in itf.voids.iter().enumerate() {
        for q in c.iter() {
            let qi = q.index();
            let mapped = if qi == Context::CALLED_INDEX {
                qi
            } else {
                sigma_pip(qi as u8, p, p2) as usize
            };
            voids[i].insert(Context::from_index(mapped).expect("context"));
        }
    }
    Interface {
        pip: p2,
        hand: transport_set(itf.hand, p, p2),
        pool: transport_set(itf.pool, p, p2),
        caps: itf.caps,
        voids,
        leader_off: itf.leader_off,
        grade: itf.grade,
    }
}

// ---------------------------------------------------------------------------
// Coordinate enumeration (freeze 22 order) and P-A15 decimation (freeze 25).
// ---------------------------------------------------------------------------

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

/// Lex-order combination unranking: the `rank`-th k-subset of {0..n-1}.
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
    let mut pool = DominoSet::EMPTY;
    let mut hand = DominoSet::EMPTY;
    let hand_set: BTreeSet<usize> = hand_pos.into_iter().collect();
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
// Root analysis: closures, the R-A18 correctness gate, the E0 receipts
// (R-A14), the operator-multiplicity census, the R-A10 fusion diagnostic.
// ---------------------------------------------------------------------------

struct RootReport {
    fiber: usize,
    dim_i: usize,
    dim_ip: usize,
    rows_i: usize,
    records: usize,
    basis_nnz: usize,
    /// (lead, Q under the count convention), ascending by lead.
    q_table: Vec<(Domino, R)>,
    /// E_beta[V*] - V_H, the strategy-fusion gap (>= 0 asserted).
    fusion_gap: R,
    /// gamma token -> number of DISTINCT per-record closure matrices
    /// (freeze-relative, R-A14/R-A21; never fold-compared, R-A7).
    census: BTreeMap<String, usize>,
    /// gamma token -> record count (fold-invariant: gamma is pip-free).
    gamma_records: BTreeMap<String, usize>,
    gate_met: bool,
}

fn gamma_str(g: &(usize, [bool; 3], bool)) -> String {
    format!(
        "(off={},{}{}{},inc={})",
        g.0,
        if g.1[0] { 'f' } else { 's' },
        if g.1[1] { 'f' } else { 's' },
        if g.1[2] { 'f' } else { 's' },
        u8::from(g.2)
    )
}

fn analyze_root(
    itf: &Interface,
    memo_i: &mut Memo,
    memo_ip: &mut Memo,
    budget: &mut usize,
    log: &mut String,
) -> RootReport {
    assert_eq!(itf.leader_off, 0, "part one roots are focal-lead (R-A8)");
    eprintln!(
        "[progress] analyze grade {} pip {} hand {:08x} (budget left {})",
        itf.grade,
        itf.pip,
        itf.hand.bits(),
        budget
    );
    let cl_i = closure(itf, Contract::ExpectedTricks, memo_i, budget);
    let cl_ip = closure(itf, Contract::Prime, memo_ip, budget);
    let worlds = &cl_i.worlds;
    let n = worlds.len();
    let decl = itf.decl();
    let records = enumerate_records(itf, worlds);

    // E0 mass receipt: per (lead, world), the record masses sum to 1.
    let mut mass: BTreeMap<(usize, u32), R> = BTreeMap::new();
    // Path-law buckets, enumeration side: (lead, successor key, world) -> mass.
    let mut buckets_a: BTreeMap<(usize, [u64; 3], u32), R> = BTreeMap::new();
    for rec in &records {
        let lead = rec.tiles[0].index();
        let skey = rec.succ.key();
        for &(wi, den) in &rec.support {
            *mass.entry((lead, wi)).or_insert_with(R::zero) += rden(den);
            *buckets_a.entry((lead, skey, wi)).or_insert_with(R::zero) += rden(den);
        }
    }
    for ((_, _), m) in &mass {
        assert!(m == &R::one(), "E0 mass receipt: record masses sum to 1");
    }
    // Path-law buckets, independent per-world side.
    let mut buckets_b: BTreeMap<(usize, [u64; 3], u32), R> = BTreeMap::new();
    let hand0 = itf.hand;
    for wi in 0..n as u32 {
        for a in legal_plays(decl, hand0, None).iter() {
            per_world_trick(itf, decl, worlds, wi, [a, a, a, a], 1, 1, &mut buckets_b);
        }
    }
    assert_eq!(
        buckets_a, buckets_b,
        "E0 path-law receipt: folded and primitive bucket masses agree"
    );

    // Treatment H (this side), per lead: pooled solve + choices.
    let hs = HSide { decl, worlds };
    let support_all: Vec<(u32, u128)> = (0..n as u32).map(|i| (i, 1)).collect();
    let mut q_table: Vec<(Domino, R)> = Vec::new();
    let mut vh: Option<R> = None;
    let inv_n = rden(n as u128);
    for a in legal_plays(decl, hand0, None).iter() {
        let mut choices = BTreeMap::new();
        let mut obs = vec![a];
        let mut tiles = [Domino::ALL[0]; 4];
        tiles[0] = a;
        let v = hs.node(&support_all, Seat::S0, tiles, 1, &mut obs, &mut choices);
        let qa = v * inv_n.clone();
        // Membership + pairing receipt (Lemma R(b)): the extracted policy's
        // per-world value function lies in V^val and pairs to Q exactly.
        let mut vvec: SVec = Vec::new();
        let mut total = R::zero();
        for wi in 0..n as u32 {
            let mut obs = vec![a];
            let val = hs.eval_policy(wi, Seat::S0, tiles, 1, &mut obs, &choices);
            total += val.clone();
            if !val.is_zero() {
                vvec.push((wi, val));
            }
        }
        assert!(
            total.clone() * inv_n.clone() == qa,
            "policy evaluation agrees with the pooled solve"
        );
        let mut coords: Vec<(usize, R)> = Vec::new();
        let rem = cl_i.basis.reduce(vvec, Some(&mut coords));
        assert!(
            rem.is_empty(),
            "Lemma R(b) receipt: a lawful policy value lies in V^val"
        );
        let mut paired = R::zero();
        for (row, c) in &coords {
            let rowsum: R = cl_i.basis.rows[*row].iter().map(|(_, v)| v.clone()).sum();
            paired += c.clone() * rowsum;
        }
        assert!(
            paired == total,
            "pairing receipt: psi . c equals the concrete expectation"
        );
        if vh.as_ref().map(|b| qa > *b).unwrap_or(true) {
            vh = Some(qa.clone());
        }
        q_table.push((a, qa));
    }
    let vh = vh.expect("a root has a lead");

    // The concrete authority (R-A10, freeze 26): ScalarHidden dag-v1 at
    // trick_only, bridged Q_diff = 2*Q_count - grade.
    let auth = ScalarHidden::new(
        decl,
        Seat::S0,
        Seat::S0.team(),
        ScalarValuation::trick_only(),
    );
    let mut ab = AUTHORITY_BUDGET;
    let gate_met = match auth.action_values_dag(worlds, Seat::S0, &[], &mut ab).0 {
        None => {
            let _ = writeln!(
                log,
                "  DECLARED STOP: authority budget {AUTHORITY_BUDGET} exceeded; correctness gate unmet"
            );
            false
        }
        Some(av) => {
            assert_eq!(av.len(), q_table.len(), "authority covers every lead");
            for ((a, qm), (b, qd)) in q_table.iter().zip(av.iter()) {
                assert_eq!(a, b, "lead order agrees");
                let qd_big = R::new(BigInt::from(*qd.numer()), BigInt::from(*qd.denom()));
                let bridged = rint(2) * qm.clone() - rint(itf.grade as i128);
                assert!(
                    qd_big == bridged,
                    "R-A18 gate: authority Q_diff equals 2*Q_count - grade exactly"
                );
            }
            true
        }
    };

    // R-A10 one-sided fusion diagnostic: E_beta[V*] >= V_H.
    let mut fused_total = R::zero();
    for wi in 0..n as u32 {
        let mut obs = Vec::new();
        fused_total += hs.fused(wi, Seat::S0, [Domino::ALL[0]; 4], 0, &mut obs);
    }
    let fused_mean = fused_total * inv_n.clone();
    assert!(
        fused_mean >= vh,
        "Lemma X's fusion inequality: E[V*] >= V_H (one-sided, R-A10)"
    );
    let fusion_gap = fused_mean - vh;

    // Operator-multiplicity census (R-A14): distinct per-record closure
    // matrices in the FINAL contract-(i) basis, grouped by gamma.
    let mut seen: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for rec in &records {
        let succ = memo_i
            .get(&rec.succ.key())
            .expect("successor closures are memoized")
            .clone();
        let mut ser = String::new();
        for f in &succ.basis.rows {
            let mut v: SVec = Vec::new();
            for &(wi, den) in &rec.support {
                let sw = succ_world(&worlds[wi as usize], &rec.tiles);
                let si = *succ.index_of.get(&pack_world(&sw)).expect("S-det");
                if let Ok(fi) = f.binary_search_by_key(&si, |&(j, _)| j) {
                    add_entry(&mut v, wi, f[fi].1.clone() * rden(den));
                }
            }
            let mut coords: Vec<(usize, R)> = Vec::new();
            let rem = cl_i.basis.reduce(v, Some(&mut coords));
            assert!(rem.is_empty(), "a residual generator lies in the closure");
            let _ = write!(ser, "[");
            for (row, c) in &coords {
                let _ = write!(ser, "{row}:{c},");
            }
            let _ = write!(ser, "]");
        }
        seen.entry(gamma_str(&rec.gamma)).or_default().insert(ser);
    }
    let census: BTreeMap<String, usize> = seen.into_iter().map(|(k, v)| (k, v.len())).collect();
    let mut gamma_records: BTreeMap<String, usize> = BTreeMap::new();
    for rec in &records {
        *gamma_records.entry(gamma_str(&rec.gamma)).or_insert(0) += 1;
    }

    RootReport {
        fiber: n,
        dim_i: cl_i.basis.dim(),
        dim_ip: cl_ip.basis.dim(),
        rows_i: cl_i.behavioral_rows,
        records: records.len(),
        basis_nnz: cl_i.basis.nnz(),
        q_table,
        fusion_gap,
        census,
        gamma_records,
        gate_met,
    }
}

/// Independent per-world one-trick walk for the E0 path-law receipt: only
/// this world's legal sets, share 1/|legal| per hidden play.
#[allow(clippy::too_many_arguments)]
fn per_world_trick(
    itf: &Interface,
    decl: Decl,
    worlds: &[[DominoSet; 4]],
    wi: u32,
    tiles: [Domino; 4],
    k: usize,
    den: u128,
    out: &mut BTreeMap<(usize, [u64; 3], u32), R>,
) {
    if k == 4 {
        let rec = finish_record(itf, decl, Seat::S0, worlds, &[(wi, den)], tiles);
        *out.entry((tiles[0].index(), rec.succ.key(), wi))
            .or_insert_with(R::zero) += rden(den);
        return;
    }
    let seat = Seat::S0.plus(k);
    let led = decl.led_context(tiles[0]);
    let mut hand = worlds[wi as usize][seat.index()];
    for t in tiles.iter().take(k) {
        hand.remove(*t);
    }
    let legal = legal_plays(decl, hand, Some(led));
    let n = legal.len() as u128;
    for d in legal.iter() {
        let mut tiles = tiles;
        tiles[k] = d;
        per_world_trick(itf, decl, worlds, wi, tiles, k + 1, den * n, out);
    }
}

// ---------------------------------------------------------------------------
// main: the census run, the fold receipts, the Gate-B verdict, the fences.
// ---------------------------------------------------------------------------

fn median(sorted: &[usize]) -> usize {
    sorted[sorted.len() / 2]
}

fn main() {
    let t0 = std::time::Instant::now();
    let mut out = String::new();
    let w = &mut out;
    let _ = writeln!(
        w,
        "walt predictive-rank probe, part one (S6a) — exploratory tier"
    );
    let _ = writeln!(w, "rulings: R-A1..R-A24, Lemma R, Corollary R-fold (walt/CENSUS-RULINGS.md 2026-08-12); design walt/PREDICTIVE-RANK.md; basis walt/math/predictive_algebra_v0.6.md (design guidance only, nothing promoted)");
    let _ = writeln!(
        w,
        "regenerate: cargo run --release -p walt-factory --example predictive_rank"
    );
    let _ = writeln!(w);
    let _ = writeln!(w, "MEASURED OBJECT (R-A16): dim V^val, the value closure of the count-free expected-focal-trick contract (Lemma R(3)), terminal seed = the zero space; and contract (i') = (i) plus expected next-leader-offset readouts for offsets 0 and 2 only (offsets 1 and 3 excluded: all four sum to the constant 1 and re-enter Lemma R(c)).");
    let _ = writeln!(w, "THEOREM ROWS (R-A24, Lemma R(c)-(d), RECEIPT NOT RUN): the trick-count-distribution contract (ii) and its predicate enrichment (iii) have predictive dimension exactly |X| at every interface. This row is forced by the observation structure of the game, not by the game's strategic complexity: every tile is eventually played and every play is publicly attributed, so a complete record determines the world, and any closure seeded with a nonzero constant contains every singleton indicator. It is not evidence that the decision problem is high-dimensional, and it must not be repaired by coarsening the observation contract, which would change the information model and therefore the operator (v0.4 §7.7).");
    let _ = writeln!(w);
    let _ = writeln!(w, "FIELD AND BELIEF (R-A9, two declarations, never one sentence): the FIELD is the v0.4 §7.4 fixed uniform-legal profile at the three hidden offsets (F4). The BELIEF is the uniform weighting over the void-free capacity fiber, a declared aggregation argument on a fabricated kernel (P-A12), not any seat's actual belief.");
    let _ = writeln!(w, "CONCRETE AUTHORITY (R-A10, freeze 26): the concrete authority for V and Q is treatment H (v0.4 §10.3), the information-consistent solve under the §7.4 uniform-legal field and the uniform fiber weighting — walt-strat ScalarHidden action_values_dag (dag-v1), trick_only valuation, budget {AUTHORITY_BUDGET}, bridge Q_diff = 2*Q_count - grade. The P-A6 world-informed aggregate is a different operator: it maximises per world and is the strategy-fusion upper bound (§7.6, §7.7). It is recorded only as the one-sided diagnostic V^pred <= E_beta[V*], and a gap between them is expected behaviour of two correct programs, never a defect.");
    let _ = writeln!(w, "VOID-FREE TYPING (P-A1, verbatim): \"This is Φ(C₀), the void-free capacity-cell fiber, a declared superset of the seat's actual support Φ(C) (v0.4 §2.1); the void constraints derivable from the actual play history are deliberately dropped. It is a declared cost domain. No support fact about any seat may be read from it, and Y3's exclusion conclusions do not apply to it.\" Roots here are fabricated void-free boundaries (R-A5): their members are feasible; no boundary measured here is asserted to arise in play. Successor interfaces carry the Lemma S-det induced voids.");
    let _ = writeln!(w, "RECEIPTS (R-A2): no object produced by this probe is an identity-bearing witness of anything. Reachability is a proof-irrelevant proposition (D3's 'necessary outer profile'); a (PCM) receipt asserts a linear identity over a declared finite domain and asserts nothing about whether any state of that domain arises in play — the domain is the void-free capacity fiber, whose members are FEASIBLE and never reachable (P-A1). A walt probe receipt is exploratory tier and is not a rob CI receipt; it never promotes anything (TRUST-01).");
    let _ = writeln!(w, "COUNT (R-A15, E-A2 restated where it bites): every number in this probe is sound only under the count-free contract; if count re-enters (v0.5 role re-entry), every basis, closure matrix and dimension here is void wholesale, never extended.");
    let _ = writeln!(w, "S5h STANDS (R-A19, X-A14): cone identity cannot short-circuit descent; the r3 class store is a storage/transport object, never a build accelerator (B:A1 ≈ 4.3–4.9). The 64 labels enter here only as the derived transition alphabet gamma (R-A13: leader offset, follow/slough flags, count-free increment).");
    let _ = writeln!(w, "FREEZE-RELATIVITY (R-A21): the predictive dimension is freeze-independent; the basis, the closure matrices and every sparsity figure are freeze-dependent (freeze 23 pivot rule: first nonzero in kernel world order, leading coefficient 1).");
    let _ = writeln!(w);

    let mut max_dim: [Option<usize>; 3] = [None, None, None];
    let mut gates: [bool; 3] = [true, true, true];
    for grade in 1..=3usize {
        let (g, wcount) = DECIMATION[grade - 1];
        let npop = population(grade);
        assert_eq!(gcd(g, npop), 1, "P-A15: gcd(g, N) = 1");
        let _ = writeln!(
            w,
            "== grade {grade}: population N = {npop}, decimation g = {g}, W = {wcount} base coordinates x 7 declarations (Corollary R-fold receipt, R-A7) =="
        );
        let mut memo_i: Memo = HashMap::new();
        let mut memo_ip: Memo = HashMap::new();
        let mut budget = GENERATOR_BUDGET;
        let mut dims_i: Vec<usize> = Vec::new();
        let mut dims_ip: Vec<usize> = Vec::new();
        let mut rows_v: Vec<usize> = Vec::new();
        for i in 0..wcount {
            let idx = (i as u128 * g) % npop;
            let base = coordinate(grade, idx);
            let tc = std::time::Instant::now();
            let rep = analyze_root(&base, &mut memo_i, &mut memo_ip, &mut budget, w);
            gates[grade - 1] &= rep.gate_met;
            // Corollary R-fold: every declaration image, dimension /
            // behavioural-row / value / census-count equality (never
            // matrix-byte equality, R-A7).
            let mut fold_ok = 0usize;
            for p2 in 0..7u8 {
                if p2 == base.pip {
                    fold_ok += 1;
                    continue;
                }
                let img = transport_interface(&base, p2);
                let irep = analyze_root(&img, &mut memo_i, &mut memo_ip, &mut budget, w);
                assert_eq!(rep.dim_i, irep.dim_i, "R-fold: dim(i) equal");
                assert_eq!(rep.dim_ip, irep.dim_ip, "R-fold: dim(i') equal");
                assert_eq!(rep.rows_i, irep.rows_i, "R-fold: behavioural rows equal");
                assert_eq!(rep.records, irep.records, "R-fold: record count equal");
                let mut qa: Vec<R> = rep.q_table.iter().map(|(_, q)| q.clone()).collect();
                let mut qb: Vec<R> = irep.q_table.iter().map(|(_, q)| q.clone()).collect();
                qa.sort();
                qb.sort();
                assert_eq!(qa, qb, "R-fold: policy values correspond");
                // The distinct-matrix census is freeze-relative and is NOT
                // fold-compared (R-A7): the pivot rule is not phi-equivariant,
                // and run evidence showed basis-dependent collisions moving
                // counts by 1-2. Per-gamma RECORD counts are fold-invariant
                // (gamma is pip-free) and are the lawful receipt.
                assert_eq!(
                    rep.gamma_records, irep.gamma_records,
                    "R-fold: per-gamma record counts equal"
                );
                fold_ok += 1;
            }
            let qs: Vec<String> = rep
                .q_table
                .iter()
                .map(|(a, q)| format!("{}{}:{}", a.hi().value(), a.lo().value(), q))
                .collect();
            let census: Vec<String> = rep.census.iter().map(|(k, v)| format!("{k}x{v}")).collect();
            let _ = writeln!(
                w,
                "coord idx={idx} pip={} |X|={} dim_i={} dim_i'={} rows={} records={} basis_nnz={} fold={}/7 gate={} fusion_gap={} Q[{}] census[{}] ({} ms)",
                base.pip,
                rep.fiber,
                rep.dim_i,
                rep.dim_ip,
                rep.rows_i,
                rep.records,
                rep.basis_nnz,
                fold_ok,
                if rep.gate_met { "MET" } else { "UNMET" },
                rep.fusion_gap,
                qs.join(" "),
                census.join(" "),
                tc.elapsed().as_millis()
            );
            dims_i.push(rep.dim_i);
            dims_ip.push(rep.dim_ip);
            rows_v.push(rep.rows_i);
        }
        dims_i.sort_unstable();
        dims_ip.sort_unstable();
        rows_v.sort_unstable();
        let _ = writeln!(
            w,
            "grade {grade} summary over W={wcount} coordinates: dim_i multiset {:?} (min {} median {} max {}); dim_i' multiset {:?}; behavioural rows {:?}; interfaces memoized {}; NO MEAN IS PRINTED (R-A6)",
            dims_i,
            dims_i[0],
            median(&dims_i),
            dims_i[dims_i.len() - 1],
            dims_ip,
            rows_v,
            memo_i.len()
        );
        max_dim[grade - 1] = Some(dims_i[dims_i.len() - 1]);
        let _ = writeln!(w);
    }

    // Gate B, exactly as pre-declared in the design addendum.
    let _ = writeln!(w, "== Gate B (criterion pre-declared in walt/PREDICTIVE-RANK.md addendum; D(n) = per-grade MAX dim_i; |X| ratios 15 and 56/3) ==");
    let d = [
        max_dim[0].unwrap(),
        max_dim[1].unwrap(),
        max_dim[2].unwrap(),
    ];
    let _ = writeln!(w, "D(1)={} D(2)={} D(3)={}", d[0], d[1], d[2]);
    let verdict = if !(gates[0] && gates[1] && gates[2]) {
        "UNRESOLVED (a correctness gate is unmet)".to_string()
    } else if d[0] == 0 || d[1] == 0 {
        "UNRESOLVED (a zero maximum makes a growth ratio undefined; reported as exactly that)"
            .to_string()
    } else {
        // Exact integer forms of the declared thresholds:
        // CONFIRMED: D2/D1 <= 5 and D3/D2 <= 56/9;
        // REFUTED:   D2/D1 >= 10 or D3/D2 >= 112/9.
        let confirmed = d[1] <= 5 * d[0] && 9 * d[2] <= 56 * d[1];
        let refuted = d[1] >= 10 * d[0] || 9 * d[2] >= 112 * d[1];
        if confirmed {
            "payoff CONFIRMED at both steps (D(2)/D(1) <= 5 and D(3)/D(2) <= 56/9)".to_string()
        } else if refuted {
            "payoff REFUTED (a growth ratio is of the same order as the fiber ratio)".to_string()
        } else {
            "UNRESOLVED (between the declared thresholds)".to_string()
        }
    };
    let _ = writeln!(w, "Gate B verdict: {verdict}");
    let _ = writeln!(w);
    let _ = writeln!(w, "THE FENCE (R-A23, verbatim): A predictive dimension is a statement about the linear span of a declared family of continuation tests over a declared coordinate's void-free capacity fiber, under the declared field, belief, count-free contract, observation contract and grade. It licenses NO runtime or tractability claim of any kind: moment compilation (v0.6 Gate D) is a separate, unmeasured experiment, and a small dimension whose moments require enumerating the fiber solves nothing (v0.6 §18.3). It is not a count of states, not a class count, not an r3-style dynamics quotient, and not a value partition. It promotes no v0.6 theorem. The numbers are coordinate-relative and are never quoted for the opening or for any grade not measured. The concrete authority is treatment H; a disagreement with it is a stop-and-report bug, never reconciled by adjustment; a divergence from the world-informed P-A6 aggregate is not a disagreement at all but the expected strategy-fusion gap (R-A10).");
    let _ = writeln!(w, "P-A21: three rungs are not a law; an implied grade-7 dimension is an extrapolation at exploratory tier and is never a statement about an unrun computation; no dimension at any grade is quoted for the opening.");
    let _ = writeln!(w, "total: {} ms", t0.elapsed().as_millis());
    let _ = writeln!(w, "run complete: yes");
    print!("{out}");
    std::fs::write("walt-factory/results/predictive_rank_2026-08-12.txt", &out)
        .or_else(|_| std::fs::write("results/predictive_rank_2026-08-12.txt", &out))
        .expect("results file written");
}
