//! EXPLORATORY DIAGNOSTIC (below every tier; cited by nothing): inspect the
//! dominant policies at S6b's singleton-frontier roots. Extracts the
//! treatment-H playbook per (coordinate, lead) — which at a singleton root
//! IS the dominant policy (pointwise-under with equal mean forces equality)
//! — then interrogates it: per free decision, does the choice matter under
//! one-deviation (strictly better somewhere, never worse) or tie, and does
//! it agree with trivial rules (least tile / greatest tile /
//! beat-if-able-else-least)? Prints sample decisions verbatim.
//!
//! No claims: this is a readout of an existing exact object for
//! conversation. Same freezes/machinery as S6a/S6b; no floats.

use std::collections::{BTreeMap, BTreeSet};

use walt_core::{legal_plays, Context, ContextSet, Decl, Domino, DominoSet, Pip, Seat, Trick};
use walt_geom::{q, qi, Q};
use walt_kernel::{Hidden, Kernel};

// -- coordinate machinery (S6a freeze 22, copied) ---------------------------

#[derive(Clone, PartialEq, Eq, Debug)]
struct Interface {
    pip: u8,
    hand: DominoSet,
    pool: DominoSet,
    grade: usize,
}

impl Interface {
    fn decl(&self) -> Decl {
        Decl::PipTrump(Pip::new(self.pip).expect("pip"))
    }
    fn kernel(&self) -> Kernel {
        let hidden = [Seat::S1, Seat::S2, Seat::S3].map(|s| Hidden {
            seat: s,
            capacity: self.grade,
            voids: ContextSet::EMPTY,
        });
        Kernel::new(self.decl(), Seat::S0, self.hand, self.pool, hidden).expect("kernel")
    }
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

fn unrank_comb(n: usize, k: usize, mut rank: u128) -> Vec<usize> {
    let mut out = Vec::with_capacity(k);
    let (mut x, mut k, mut n_left) = (0usize, k, n);
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
    let live_idx = unrank_comb(28, 4 * grade, rem / hand_c);
    let hand_pos: BTreeSet<usize> = unrank_comb(4 * grade, grade, rem % hand_c)
        .into_iter()
        .collect();
    let (mut pool, mut hand) = (DominoSet::EMPTY, DominoSet::EMPTY);
    for (pos, di) in live_idx.iter().enumerate() {
        let d = Domino::from_index(*di).expect("domino");
        if hand_pos.contains(&pos) {
            hand.insert(d);
        } else {
            pool.insert(d);
        }
    }
    Interface {
        pip,
        hand,
        pool,
        grade,
    }
}

fn tile(d: Domino) -> String {
    format!("{}{}", d.hi().value(), d.lo().value())
}

// -- the pooled H walk: extract choices, then interrogate -------------------

struct Ctx<'a> {
    decl: Decl,
    worlds: &'a [[DominoSet; 4]],
}

impl Ctx<'_> {
    fn hand_now(&self, wi: u32, seat: Seat, obs: &[Domino]) -> DominoSet {
        let mut h = self.worlds[wi as usize][seat.index()];
        for t in obs {
            h.remove(*t);
        }
        h
    }

    /// Pooled H solve recording argmax choices per observation record
    /// (least-tile tie rule, freeze 26).
    fn solve(
        &self,
        support: &[(u32, u128)],
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        obs: &mut Vec<Domino>,
        choices: &mut BTreeMap<Vec<Domino>, Domino>,
    ) -> Q {
        if k == 4 {
            let trick = Trick::new(leader, tiles).expect("distinct");
            let winner = trick.winner(self.decl);
            let mass: Q = support.iter().map(|&(_, den)| unit(den)).sum();
            let banked = if winner.team() == Seat::S0.team() {
                mass
            } else {
                qi(0)
            };
            if self.hand_now(support[0].0, Seat::S0, obs).is_empty() {
                return banked;
            }
            return banked + self.solve(support, winner, [Domino::ALL[0]; 4], 0, obs, choices);
        }
        let seat = leader.plus(k);
        let led = (k > 0).then(|| self.decl.led_context(tiles[0]));
        if seat == Seat::S0 {
            let hand = self.hand_now(support[0].0, seat, obs);
            let legal = legal_plays(self.decl, hand, led);
            let mut best: Option<(Q, Domino)> = None;
            for a in legal.iter() {
                let mut tiles = tiles;
                tiles[k] = a;
                obs.push(a);
                let v = self.solve(support, leader, tiles, k + 1, obs, choices);
                obs.pop();
                if best.as_ref().map(|(bv, _)| v > *bv).unwrap_or(true) {
                    best = Some((v, a));
                }
            }
            let (v, a) = best.expect("legal move exists");
            choices.insert(obs.clone(), a);
            return v;
        }
        let mut by_tile: BTreeMap<usize, Vec<(u32, u128)>> = BTreeMap::new();
        for &(wi, den) in support {
            let hand = self.hand_now(wi, seat, obs);
            let legal = legal_plays(self.decl, hand, led);
            let n = legal.len() as u128;
            for t in legal.iter() {
                by_tile.entry(t.index()).or_default().push((wi, den * n));
            }
        }
        let mut sum = qi(0);
        for (ti, sup) in by_tile {
            let d = Domino::from_index(ti).expect("tile");
            let mut tiles = tiles;
            tiles[k] = d;
            obs.push(d);
            sum += self.solve(&sup, leader, tiles, k + 1, obs, choices);
            obs.pop();
        }
        sum
    }

    /// Per-world expected count under the fixed extracted playbook, from a
    /// mid-tree node.
    #[allow(clippy::too_many_arguments)]
    fn eval_world(
        &self,
        wi: u32,
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        obs: &mut Vec<Domino>,
        choices: &BTreeMap<Vec<Domino>, Domino>,
        override_first: Option<Domino>,
    ) -> Q {
        if k == 4 {
            let trick = Trick::new(leader, tiles).expect("distinct");
            let winner = trick.winner(self.decl);
            let inc = if winner.team() == Seat::S0.team() {
                qi(1)
            } else {
                qi(0)
            };
            if self.hand_now(wi, Seat::S0, obs).is_empty() {
                return inc;
            }
            return inc
                + self.eval_world(
                    wi,
                    winner,
                    [Domino::ALL[0]; 4],
                    0,
                    obs,
                    choices,
                    override_first,
                );
        }
        let seat = leader.plus(k);
        let led = (k > 0).then(|| self.decl.led_context(tiles[0]));
        if seat == Seat::S0 {
            let a = match override_first {
                Some(d) => d,
                None => *choices.get(obs.as_slice()).expect("record was pooled"),
            };
            let mut tiles = tiles;
            tiles[k] = a;
            obs.push(a);
            let v = self.eval_world(wi, leader, tiles, k + 1, obs, choices, None);
            obs.pop();
            return v;
        }
        let hand = self.hand_now(wi, seat, obs);
        let legal = legal_plays(self.decl, hand, led);
        let share = q(1, legal.len() as i128);
        let mut sum = qi(0);
        for d in legal.iter() {
            let mut tiles = tiles;
            tiles[k] = d;
            obs.push(d);
            sum += share * self.eval_world(wi, leader, tiles, k + 1, obs, choices, override_first);
            obs.pop();
        }
        sum
    }

    /// Walks the pooled tree under the playbook; at every free focal state,
    /// classifies the decision and scores the trivial rules.
    #[allow(clippy::too_many_arguments)]
    fn interrogate(
        &self,
        support: &[(u32, u128)],
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        obs: &mut Vec<Domino>,
        choices: &BTreeMap<Vec<Domino>, Domino>,
        tally: &mut Tally,
    ) {
        if k == 4 {
            let trick = Trick::new(leader, tiles).expect("distinct");
            let winner = trick.winner(self.decl);
            if self.hand_now(support[0].0, Seat::S0, obs).is_empty() {
                return;
            }
            self.interrogate(support, winner, [Domino::ALL[0]; 4], 0, obs, choices, tally);
            return;
        }
        let seat = leader.plus(k);
        let led = (k > 0).then(|| self.decl.led_context(tiles[0]));
        if seat == Seat::S0 {
            let hand = self.hand_now(support[0].0, seat, obs);
            let legal = legal_plays(self.decl, hand, led);
            let picked = *choices.get(obs.as_slice()).expect("record was pooled");
            if legal.len() == 2 {
                let ts: Vec<Domino> = legal.iter().collect();
                let other = if ts[0] == picked { ts[1] } else { ts[0] };
                // One-deviation comparison on the local support.
                let (mut strict, mut worse, mut tie_all) = (false, false, true);
                for &(wi, _) in support {
                    let mut o1 = obs.clone();
                    let va = self.eval_world(wi, leader, tiles, k, &mut o1, choices, Some(picked));
                    let mut o2 = obs.clone();
                    let vb = self.eval_world(wi, leader, tiles, k, &mut o2, choices, Some(other));
                    if va > vb {
                        strict = true;
                        tie_all = false;
                    }
                    if va < vb {
                        worse = true;
                        tie_all = false;
                    }
                }
                let class = if tie_all {
                    tally.indifferent += 1;
                    "TIE"
                } else if strict && !worse {
                    tally.strict += 1;
                    "MATTERS"
                } else {
                    tally.incomparable += 1;
                    "INCOMPARABLE(one-dev)"
                };
                // Trivial-rule agreement (only where the choice matters).
                if class == "MATTERS" {
                    let least = ts[0].min(ts[1]);
                    let greatest = ts[0].max(ts[1]);
                    if picked == least {
                        tally.agree_least += 1;
                    }
                    if picked == greatest {
                        tally.agree_greatest += 1;
                    }
                    let beat = beat_if_able(self.decl, tiles, k, led, ts[0], ts[1]);
                    if picked == beat {
                        tally.agree_beat += 1;
                    }
                    if tally.samples.len() < 8 {
                        let rec: Vec<String> = obs.iter().map(|d| tile(*d)).collect();
                        tally.samples.push(format!(
                            "  after [{}] (support {} worlds): plays {} over {} [least={} beat-rule={}]",
                            rec.join(" "),
                            support.len(),
                            tile(picked),
                            tile(other),
                            tile(ts[0].min(ts[1])),
                            tile(beat)
                        ));
                    }
                }
            }
            let mut tiles = tiles;
            tiles[k] = picked;
            obs.push(picked);
            self.interrogate(support, leader, tiles, k + 1, obs, choices, tally);
            obs.pop();
            return;
        }
        let mut by_tile: BTreeMap<usize, Vec<(u32, u128)>> = BTreeMap::new();
        for &(wi, den) in support {
            let hand = self.hand_now(wi, seat, obs);
            let legal = legal_plays(self.decl, hand, led);
            let n = legal.len() as u128;
            for t in legal.iter() {
                by_tile.entry(t.index()).or_default().push((wi, den * n));
            }
        }
        for (ti, sup) in by_tile {
            let d = Domino::from_index(ti).expect("tile");
            let mut tiles = tiles;
            tiles[k] = d;
            obs.push(d);
            self.interrogate(&sup, leader, tiles, k + 1, obs, choices, tally);
            obs.pop();
        }
    }
}

fn unit(den: u128) -> Q {
    q(1, i128::try_from(den).expect("den fits"))
}

/// The "beat if able, else least" rule: pick the tile that wins the trick
/// so far, preferring the least such; else the least tile.
fn beat_if_able(
    decl: Decl,
    tiles: [Domino; 4],
    k: usize,
    led: Option<Context>,
    a: Domino,
    b: Domino,
) -> Domino {
    let (lo, hi) = (a.min(b), a.max(b));
    let Some(led) = led else {
        return lo;
    };
    let wins = |cand: Domino| (0..k).all(|i| decl.beats(led, tiles[i]).contains(cand));
    if wins(lo) {
        lo
    } else if wins(hi) {
        hi
    } else {
        lo
    }
}

#[derive(Default)]
struct Tally {
    strict: usize,
    indifferent: usize,
    incomparable: usize,
    agree_least: usize,
    agree_greatest: usize,
    agree_beat: usize,
    samples: Vec<String>,
}

fn main() {
    // The seven S6b singleton roots: (grade-3 coordinate index, lead).
    let roots: [(u128, (u8, u8)); 7] = [
        (0, (0, 0)),
        (1_299_709, (2, 1)),
        (1_299_709, (2, 2)),
        (1_299_709, (6, 3)),
        (2_599_418, (1, 1)),
        (2_599_418, (2, 2)),
        (2_599_418, (4, 2)),
    ];
    println!("EXPLORATORY DIAGNOSTIC — dominant-playbook interrogation at the S6b singleton roots (below every tier; cited by nothing)");
    for (idx, (hi, lo)) in roots {
        let itf = coordinate(3, idx);
        let kernel = itf.kernel();
        let worlds: Vec<[DominoSet; 4]> = kernel.worlds().map(|w| w.hands()).collect();
        let lead = Domino::new(Pip::new(hi).expect("pip"), Pip::new(lo).expect("pip"));
        assert!(itf.hand.contains(lead), "lead is a hand tile");
        let ctx = Ctx {
            decl: itf.decl(),
            worlds: &worlds,
        };
        let support: Vec<(u32, u128)> = (0..worlds.len() as u32).map(|i| (i, 1)).collect();
        let mut tiles = [Domino::ALL[0]; 4];
        tiles[0] = lead;
        let mut obs = vec![lead];
        let mut choices = BTreeMap::new();
        let _ = ctx.solve(&support, Seat::S0, tiles, 1, &mut obs, &mut choices);
        let hand: Vec<String> = itf.hand.iter().map(tile).collect();
        let mut tally = Tally::default();
        let mut obs = vec![lead];
        ctx.interrogate(&support, Seat::S0, tiles, 1, &mut obs, &choices, &mut tally);
        let free = tally.strict + tally.indifferent + tally.incomparable;
        println!();
        println!(
            "root idx={idx} trump={} hand=[{}] lead={}: {} recorded decisions, {} free (2-tile) states",
            itf.pip,
            hand.join(" "),
            tile(lead),
            choices.len(),
            free
        );
        println!(
            "  choice MATTERS (strictly better somewhere, never worse): {} | TIE (value-identical either way): {} | incomparable under one-deviation: {}",
            tally.strict, tally.indifferent, tally.incomparable
        );
        if tally.strict > 0 {
            println!(
                "  of the {} mattering decisions: agrees with always-LEAST {} | always-GREATEST {} | BEAT-if-able-else-least {}",
                tally.strict, tally.agree_least, tally.agree_greatest, tally.agree_beat
            );
        }
        for s in &tally.samples {
            println!("{s}");
        }
    }
    println!();
    println!("diagnostic complete");
}
