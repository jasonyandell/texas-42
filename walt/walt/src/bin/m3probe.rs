//! EXPLORATORY PROBE — sits below every evidentiary tier and is cited by
//! nothing above it (wiki/ideas tier; CLAUDE.md "Exploratory stays exploratory").
//!
//! Exact lawful best-response solve of the frozen M3 carrier: receipt hand 8
//! immediately before trick 4, viewer/focal S1 holding [21,31,33,55], trump
//! fives, 1,200 uniform support worlds, field seats uniform-random-legal.
//! Treatment H is the lawful perfect-recall expectimax (S1 decides on public
//! record + own hand only); treatment C is the world-revealed clairvoyant
//! control. Objectives: M3A future-trick differential for T1, M3B P30 make.
//!
//! This is NOT the freeze-57 M3 gate: no Metal parity, no receipt, no net
//! census, and nothing here is a trick-1 statement (P-A21). It answers one
//! question exactly: what does the lawful player LEAD, and with what exact
//! value, on the frozen carrier.
//!
//! Arithmetic: integers only. Every field seat picks uniformly from a legal
//! set of size 1..=4 and there are exactly 12 future field moves, so scaling
//! each field factor by 12 keeps all masses integral: a node mass is
//! sum_w weight_w * 12^(field moves below), bounded by 1200 * 12^12 < 2^54.
//! At S1 decision nodes every child carries the identical mass, so argmax is
//! an integer comparison of numerators. No floats exist in this crate.

use std::collections::HashMap;

use walt::carrier::{
    M3Carrier, HIDDEN_POOL_MASK, LEGAL_ROOT_MASK, RAW_RECEIPT_BYTES, ROOTS, SUPPORT_COUNT, VIEWER,
};
use walt::rules::rules::{legal_plays, Trick};
use walt::rules::{Context, Decl, Domino, DominoSet, Pip, Seat, Team};

const RAW_RECEIPT: &[u8] = include_bytes!("../../../../rob/receipts/verify_player.txt");

/// Trump fives: the carrier declaration (receipt hand 8, "called P5").
fn decl() -> Decl {
    Decl::PipTrump(Pip::new(5).expect("pip 5 exists"))
}

/// 12^12: the total field-factor scale below the root child.
const FIELD_SCALE_TOTAL: u128 = {
    let mut v: u128 = 1;
    let mut i = 0;
    while i < 12 {
        v *= 12;
        i += 1;
    }
    v
};

fn mask_of(set: DominoSet) -> u32 {
    let mut m = 0u32;
    for d in set.iter() {
        m |= 1u32 << d.index();
    }
    m
}

fn set_of(mask: u32) -> DominoSet {
    let mut s = DominoSet::default();
    let mut m = mask;
    while m != 0 {
        let i = m.trailing_zeros() as usize;
        s.insert(Domino::from_index(i).expect("index < 28"));
        m &= m - 1;
    }
    s
}

/// Trick value under straight 42: one for the trick plus captured count.
fn trick_value(doms: [Domino; 4]) -> u32 {
    1 + doms.iter().map(|d| d.count()).sum::<u32>()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Objective {
    /// M3A: future tricks won by T1 minus future tricks won by T0, in -4..=4.
    FutureTrickDifferential,
    /// M3B: 1 if T1's final banked total reaches 30, else 0.
    P30Make,
}

/// Banked points entering trick 4 (prefix winners S0/S1/S1, values 1/6/1).
const BANKED_T0_INIT: u32 = 1;
const BANKED_T1_INIT: u32 = 7;

fn terminal_value(obj: Objective, banked_t1: u32, tricks_t1: i32, tricks_t0: i32) -> i64 {
    match obj {
        Objective::FutureTrickDifferential => i64::from(tricks_t1 - tricks_t0),
        Objective::P30Make => i64::from(banked_t1 >= 30),
    }
}

/// One in-progress public state below the root. `plays` are the current-trick
/// plays in seat order from `leader`; `played` is the mask of tiles played
/// since the root; banked/tricks accumulate resolved future tricks only.
#[derive(Clone, PartialEq, Eq, Hash)]
struct NodeKey {
    played: u32,
    leader: u8,
    plays: Vec<u8>,
    banked_t1: u32,
    tricks_t1: i8,
    tricks_t0: i8,
    /// The exact posterior: (support ordinal, integer weight). Part of the
    /// information state — two records with different field-likelihoods are
    /// different lawful states even at equal tile masks.
    alive: Vec<(u16, u64)>,
}

struct HSolver {
    decl: Decl,
    /// Per-world hidden-hand masks at the root, S0/S1/S2/S3.
    worlds: Vec<[u32; 4]>,
    s1_root: u32,
    memo: HashMap<NodeKey, (i128, u128)>,
    obj: Objective,
}

impl HSolver {
    /// Returns (U, M): U/M is the exact conditional expectation of the
    /// objective at this node; M = sum_w weight_w * 12^(field moves below).
    fn solve(&mut self, key: &NodeKey) -> (i128, u128) {
        if let Some(&hit) = self.memo.get(key) {
            return hit;
        }
        let n_played = key.played.count_ones() as usize;
        debug_assert!(n_played <= 16);
        let result = if n_played == 16 {
            let mass: u128 = key.alive.iter().map(|&(_, w)| u128::from(w)).sum();
            let v = i128::from(terminal_value(
                self.obj,
                key.banked_t1,
                i32::from(key.tricks_t1),
                i32::from(key.tricks_t0),
            ));
            let u: i128 = key.alive.iter().map(|&(_, w)| i128::from(w) * v).sum();
            (u, mass)
        } else {
            let seat = Seat::from_index((usize::from(key.leader) + key.plays.len()) % 4)
                .expect("seat index");
            let led: Option<Context> = key.plays.first().map(|&i| {
                self.decl
                    .led_context(Domino::from_index(usize::from(i)).expect("led index"))
            });
            if seat == VIEWER {
                self.solve_viewer(key, led)
            } else {
                self.solve_field(key, seat, led)
            }
        };
        self.memo.insert(key.clone(), result);
        result
    }

    fn child_after_play(&self, key: &NodeKey, tile: Domino, alive: Vec<(u16, u64)>) -> NodeKey {
        let mut plays = key.plays.clone();
        plays.push(tile.index() as u8);
        let played = key.played | (1u32 << tile.index());
        if plays.len() == 4 {
            let doms = [
                Domino::from_index(usize::from(plays[0])).expect("p0"),
                Domino::from_index(usize::from(plays[1])).expect("p1"),
                Domino::from_index(usize::from(plays[2])).expect("p2"),
                Domino::from_index(usize::from(plays[3])).expect("p3"),
            ];
            let leader = Seat::from_index(usize::from(key.leader)).expect("leader");
            let trick = Trick::new(leader, doms).expect("distinct tiles in trick");
            let winner = trick.winner(self.decl);
            let value = trick_value(doms);
            let t1_won = winner.team() == Team::T1;
            NodeKey {
                played,
                leader: winner.index() as u8,
                plays: Vec::new(),
                banked_t1: key.banked_t1 + if t1_won { value } else { 0 },
                tricks_t1: key.tricks_t1 + i8::from(t1_won),
                tricks_t0: key.tricks_t0 + i8::from(!t1_won),
                alive,
            }
        } else {
            NodeKey {
                played,
                leader: key.leader,
                plays,
                banked_t1: key.banked_t1,
                tricks_t1: key.tricks_t1,
                tricks_t0: key.tricks_t0,
                alive,
            }
        }
    }

    /// S1 decides: all children share one mass, so argmax is integer compare.
    fn solve_viewer(&mut self, key: &NodeKey, led: Option<Context>) -> (i128, u128) {
        let hand = self.s1_root & !key.played;
        let legal = legal_plays(self.decl, set_of(hand), led);
        let mut best: Option<(i128, u128)> = None;
        for tile in legal.iter() {
            let child = self.child_after_play(key, tile, key.alive.clone());
            let (u, m) = self.solve(&child);
            match best {
                None => best = Some((u, m)),
                Some((bu, bm)) => {
                    debug_assert_eq!(m, bm, "S1 children must share mass");
                    if u > bu {
                        best = Some((u, m));
                    }
                }
            }
        }
        best.expect("viewer always has a legal play")
    }

    /// A field seat plays uniformly from its per-world legal set: bucket the
    /// posterior by chosen tile, scaling each world's weight by 12/|legal_w|.
    fn solve_field(&mut self, key: &NodeKey, seat: Seat, led: Option<Context>) -> (i128, u128) {
        let mut buckets: HashMap<u8, Vec<(u16, u64)>> = HashMap::new();
        for &(w_idx, weight) in &key.alive {
            let hand = self.worlds[usize::from(w_idx)][seat.index()] & !key.played;
            let legal = legal_plays(self.decl, set_of(hand), led);
            let k = u64::from(legal.iter().count() as u32);
            debug_assert!((1..=4).contains(&k));
            let scaled = weight * (12 / k);
            for tile in legal.iter() {
                buckets
                    .entry(tile.index() as u8)
                    .or_default()
                    .push((w_idx, scaled));
            }
        }
        let mass_in: u128 = key.alive.iter().map(|&(_, w)| u128::from(w)).sum();
        let mut u_total: i128 = 0;
        let mut m_total: u128 = 0;
        let mut tiles: Vec<u8> = buckets.keys().copied().collect();
        tiles.sort_unstable();
        for t in tiles {
            let alive = buckets.remove(&t).expect("bucket exists");
            let child =
                self.child_after_play(key, Domino::from_index(usize::from(t)).expect("t"), alive);
            let (u, m) = self.solve(&child);
            u_total += u;
            m_total += m;
        }
        // Mass receipt: every world redistributes exactly 12x its weight.
        let below = self.field_moves_below(key);
        assert_eq!(
            m_total,
            mass_in * 12u128 * pow12(below),
            "field mass conservation"
        );
        (u_total, m_total)
    }

    /// Field moves strictly below this node's own move (for the mass receipt).
    fn field_moves_below(&self, key: &NodeKey) -> u32 {
        let n_played = key.played.count_ones();
        let total_moves_left = 16 - n_played; // this seat's move included
        let mut field_left = 0u32;
        let mut leader = usize::from(key.leader);
        let mut in_trick = key.plays.len();
        for _ in 0..total_moves_left {
            let seat = (leader + in_trick) % 4;
            if seat != VIEWER.index() {
                field_left += 1;
            }
            in_trick += 1;
            if in_trick == 4 {
                // Winner unknown here; leadership does not change which seats
                // play in remaining tricks (all four play every trick), so any
                // continuation leader gives the same field-move count.
                in_trick = 0;
                leader = 0;
            }
        }
        field_left - 1
    }
}

fn pow12(n: u32) -> u128 {
    let mut v: u128 = 1;
    for _ in 0..n {
        v *= 12;
    }
    v
}

/// Clairvoyant control: S1 sees world `w`; field still uniform-random-legal.
/// Returns the value scaled by 12^(field moves remaining below this node).
/// (played, leader, current-trick plays, banked_t1, tricks_t1, tricks_t0).
type CMemoKey = (u32, u8, Vec<u8>, u32, i8, i8);

struct CSolver {
    decl: Decl,
    hands: [u32; 4],
    obj: Objective,
    memo: HashMap<CMemoKey, i128>,
}

impl CSolver {
    fn solve(
        &mut self,
        played: u32,
        leader: u8,
        plays: &[u8],
        banked_t1: u32,
        tricks_t1: i8,
        tricks_t0: i8,
    ) -> i128 {
        if played.count_ones() == 16 {
            return i128::from(terminal_value(
                self.obj,
                banked_t1,
                i32::from(tricks_t1),
                i32::from(tricks_t0),
            ));
        }
        let key = (
            played,
            leader,
            plays.to_vec(),
            banked_t1,
            tricks_t1,
            tricks_t0,
        );
        if let Some(&hit) = self.memo.get(&key) {
            return hit;
        }
        let seat = Seat::from_index((usize::from(leader) + plays.len()) % 4).expect("seat");
        let led: Option<Context> = plays.first().map(|&i| {
            self.decl
                .led_context(Domino::from_index(usize::from(i)).expect("led"))
        });
        let hand = self.hands[seat.index()] & !played;
        let legal: Vec<Domino> = legal_plays(self.decl, set_of(hand), led).iter().collect();
        let mut values: Vec<i128> = Vec::with_capacity(legal.len());
        for &tile in &legal {
            let mut next_plays = plays.to_vec();
            next_plays.push(tile.index() as u8);
            let next_played = played | (1u32 << tile.index());
            let v = if next_plays.len() == 4 {
                let doms = [
                    Domino::from_index(usize::from(next_plays[0])).expect("p0"),
                    Domino::from_index(usize::from(next_plays[1])).expect("p1"),
                    Domino::from_index(usize::from(next_plays[2])).expect("p2"),
                    Domino::from_index(usize::from(next_plays[3])).expect("p3"),
                ];
                let trick = Trick::new(Seat::from_index(usize::from(leader)).expect("l"), doms)
                    .expect("distinct");
                let winner = trick.winner(self.decl);
                let value = trick_value(doms);
                let t1_won = winner.team() == Team::T1;
                self.solve(
                    next_played,
                    winner.index() as u8,
                    &[],
                    banked_t1 + if t1_won { value } else { 0 },
                    tricks_t1 + i8::from(t1_won),
                    tricks_t0 + i8::from(!t1_won),
                )
            } else {
                self.solve(
                    next_played,
                    leader,
                    &next_plays,
                    banked_t1,
                    tricks_t1,
                    tricks_t0,
                )
            };
            values.push(v);
        }
        let result = if seat == VIEWER {
            *values.iter().max().expect("legal nonempty")
        } else {
            // Uniform over k legal moves; children are scaled 12^(f-1), this
            // node is scaled 12^f: multiply the sum by 12/k.
            let k = i128::from(values.len() as u32);
            (12 / k) * values.iter().sum::<i128>()
        };
        self.memo.insert(key, result);
        result
    }
}

fn gcd(a: u128, b: u128) -> u128 {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn show_fraction(num: i128, den: u128) -> String {
    let neg = num < 0;
    let n = num.unsigned_abs();
    let g = gcd(n, den).max(1);
    format!("{}{}/{}", if neg { "-" } else { "" }, n / g, den / g)
}

fn main() {
    let decl = decl();
    assert_eq!(RAW_RECEIPT.len(), RAW_RECEIPT_BYTES);
    let carrier = M3Carrier::from_receipt_bytes(RAW_RECEIPT).expect("frozen carrier must admit");
    let records = carrier.support().records();
    assert_eq!(records.len(), SUPPORT_COUNT);

    let worlds: Vec<[u32; 4]> = records
        .iter()
        .map(|r| {
            let h = r.hands();
            [mask_of(h[0]), mask_of(h[1]), mask_of(h[2]), mask_of(h[3])]
        })
        .collect();
    let s1_root = worlds[0][VIEWER.index()];
    assert_eq!(s1_root, LEGAL_ROOT_MASK, "S1 hand is the frozen root mask");
    for w in &worlds {
        assert_eq!(w[VIEWER.index()], s1_root);
        assert_eq!(
            w[0] | w[2] | w[3],
            HIDDEN_POOL_MASK,
            "hidden hands tile the frozen pool"
        );
    }

    // Independent prefix cross-check: replay the three public tricks through
    // walt-core and demand the frozen winners and values.
    {
        let t1 =
            Trick::new(Seat::S1, [d(5, 2), d(6, 3), d(5, 1), d(6, 5)]).expect("prefix trick 1");
        let t2 =
            Trick::new(Seat::S0, [d(4, 2), d(5, 4), d(4, 4), d(3, 2)]).expect("prefix trick 2");
        let t3 =
            Trick::new(Seat::S1, [d(1, 1), d(2, 2), d(1, 0), d(6, 1)]).expect("prefix trick 3");
        assert_eq!(t1.winner(decl), Seat::S0);
        assert_eq!(t2.winner(decl), Seat::S1);
        assert_eq!(t3.winner(decl), Seat::S1);
        assert_eq!(trick_value(t1.plays().map(|(_, x)| x)), 1);
        assert_eq!(trick_value(t2.plays().map(|(_, x)| x)), 6);
        assert_eq!(trick_value(t3.plays().map(|(_, x)| x)), 1);
    }

    println!("walt-m3-probe: EXPLORATORY lawful solve of the frozen M3 carrier");
    println!("carrier: hand 8 before trick 4; S1 leads holding [21,31,33,55]; trump fives");
    println!("support: 1,200 uniform worlds (dual-constructor, digest-pinned by walt-m3-carrier)");
    println!("banked entering trick 4: T1={BANKED_T1_INIT} T0={BANKED_T0_INIT}; unbanked 34; T1 makes at 30");
    println!("treatments: H = lawful perfect recall; C = world-revealed control (per-world max, averaged)");
    println!("tie rule: all argmax members reported; no convention selects among them silently");
    println!();

    for (obj, obj_name) in [
        (
            Objective::FutureTrickDifferential,
            "M3A future-trick differential (T1 minus T0)",
        ),
        (Objective::P30Make, "M3B P30 make probability"),
    ] {
        println!("== {obj_name}");
        let mut h_values: Vec<(Domino, i128, u128)> = Vec::new();
        let mut c_values: Vec<(Domino, i128)> = Vec::new();
        for &root in ROOTS.iter() {
            // H: S1 plays `root`; the posterior starts uniform (weight 1 each).
            let alive: Vec<(u16, u64)> = (0..SUPPORT_COUNT as u16).map(|i| (i, 1u64)).collect();
            let mut solver = HSolver {
                decl,
                worlds: worlds.clone(),
                s1_root,
                memo: HashMap::new(),
                obj,
            };
            let root_key = NodeKey {
                played: 0,
                leader: VIEWER.index() as u8,
                plays: Vec::new(),
                banked_t1: BANKED_T1_INIT,
                tricks_t1: 0,
                tricks_t0: 0,
                alive: Vec::new(),
            };
            let child = solver.child_after_play(&root_key, root, alive);
            let (u, m) = solver.solve(&child);
            assert_eq!(
                m,
                u128::from(SUPPORT_COUNT as u64) * FIELD_SCALE_TOTAL,
                "root mass receipt: 1200 * 12^12"
            );
            h_values.push((root, u, m));

            // C: per-world clairvoyant value of the same lead, averaged.
            let mut c_sum: i128 = 0;
            for w in &worlds {
                let mut cs = CSolver {
                    decl,
                    hands: *w,
                    obj,
                    memo: HashMap::new(),
                };
                let played = 1u32 << root.index();
                let plays = [root.index() as u8];
                c_sum += cs.solve(played, VIEWER.index() as u8, &plays, BANKED_T1_INIT, 0, 0);
            }
            c_values.push((root, c_sum));
        }

        let best_u = h_values.iter().map(|&(_, u, _)| u).max().expect("roots");
        for (i, &(root, u, m)) in h_values.iter().enumerate() {
            let (_, c_sum) = c_values[i];
            let c_den = u128::from(SUPPORT_COUNT as u64) * FIELD_SCALE_TOTAL;
            // C >= H pointwise in expectation: clairvoyance never hurts.
            assert!(
                c_sum * (m as i128) >= u * (c_den as i128),
                "clairvoyant control must dominate lawful H"
            );
            println!(
                "  lead {}{}  H = {}  C = {}{}",
                root.hi().value(),
                root.lo().value(),
                show_fraction(u, m),
                show_fraction(c_sum, c_den),
                if u == best_u { "   <-- H-optimal" } else { "" },
            );
        }
        let ties: Vec<String> = h_values
            .iter()
            .filter(|&&(_, u, _)| u == best_u)
            .map(|&(r, _, _)| format!("{}{}", r.hi().value(), r.lo().value()))
            .collect();
        println!(
            "  lawful play under {obj_name}: {}{}",
            ties.join(" / "),
            if ties.len() > 1 { "  (exact tie)" } else { "" }
        );
        println!();
    }
    println!(
        "mass receipts: every field node conserved 12x mass; root mass = 1200 * 12^12 exactly"
    );
    println!(
        "nothing above exploratory tier; not a trick-1 statement (P-A21); no receipt is emitted"
    );
}

fn d(hi: u8, lo: u8) -> Domino {
    Domino::new(Pip::new(hi).expect("hi pip"), Pip::new(lo).expect("lo pip"))
}
