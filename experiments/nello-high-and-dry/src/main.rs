//! EXPLORATORY. What does a dry 6-5 cost a strong Nel-O hand?
//!
//! Independent re-implementation of the doubles-suit trick rules used by the
//! Nel-O player on PR #90 (walt/math/SUIT_ALGEBRA_PURE.md §§3-6): doubles are
//! their own unpowered suit ranked by pip, a mixed lead is its high pip, the
//! highest follower wins. Declarer leads trick 1, partner sits out holding 7
//! hidden tiles, and the declarer's first trick won is the set.
//!
//! Integer arithmetic only. Seats: 0 = declarer D, 1 = left defender L (plays
//! after D), 2 = right defender R (plays before D). Clockwise, skipping the
//! partner: D -> L -> R -> D.
use std::{collections::HashMap, env, thread};

const N: usize = 28;
const FULL: u32 = (1 << N) - 1;
type Set = u32;

fn tile(i: usize) -> (u8, u8) {
    let mut k = 0;
    for hi in 0..7u8 {
        for lo in 0..=hi {
            if k == i {
                return (hi, lo);
            }
            k += 1;
        }
    }
    unreachable!()
}
fn index(hi: u8, lo: u8) -> usize {
    let (hi, lo) = if hi >= lo { (hi, lo) } else { (lo, hi) };
    (0..N).find(|&i| tile(i) == (hi, lo)).unwrap()
}
fn name(i: usize) -> String {
    let (h, l) = tile(i);
    format!("{h}-{l}")
}
/// Led suit: 7 = doubles, else the high pip.
fn suit_led(t: usize) -> u8 {
    let (h, l) = tile(t);
    if h == l { 7 } else { h }
}
fn follows(t: usize, s: u8) -> bool {
    let (h, l) = tile(t);
    if s == 7 { h == l } else { h != l && (h == s || l == s) }
}
/// Rank inside the suit it follows: doubles by pip, mixed by the other end.
fn rank(t: usize, s: u8) -> u8 {
    let (h, l) = tile(t);
    if s == 7 { h } else if h == s { l } else { h }
}
fn legal(hand: Set, s: Option<u8>) -> Set {
    if let Some(s) = s {
        let f = bits(hand).filter(|&t| follows(t, s)).fold(0, |m, t| m | 1 << t);
        if f != 0 {
            return f;
        }
    }
    hand
}
fn bits(m: Set) -> impl Iterator<Item = usize> {
    (0..N).filter(move |&i| m >> i & 1 == 1)
}
fn next(seat: usize) -> usize {
    (seat + 1) % 3
}
/// Index into `plays` of the current trick winner.
fn winning(plays: &[usize]) -> usize {
    let s = suit_led(plays[0]);
    let mut best = 0;
    for i in 1..plays.len() {
        if follows(plays[i], s) && rank(plays[i], s) > rank(plays[best], s) {
            best = i;
        }
    }
    best
}

// ---------------------------------------------------------------- double dummy
/// True iff declarer avoids every remaining trick against perfect defenders
/// (everyone sees every hand). `memo` caches trick-boundary positions of one
/// deal, keyed by (tiles already played, leader).
fn dd(h: &mut [Set; 3], leader: usize, plays: &mut Vec<usize>, memo: &mut HashMap<u32, bool>) -> bool {
    if plays.is_empty() {
        let key = (FULL & !(h[0] | h[1] | h[2])) | (leader as u32) << 28;
        if let Some(&r) = memo.get(&key) {
            return r;
        }
        let r = dd_inner(h, leader, plays, memo);
        memo.insert(key, r);
        return r;
    }
    dd_inner(h, leader, plays, memo)
}
fn dd_inner(h: &mut [Set; 3], leader: usize, plays: &mut Vec<usize>, memo: &mut HashMap<u32, bool>) -> bool {
    if plays.len() == 3 {
        let w = (leader + winning(plays)) % 3;
        if w == 0 {
            return false;
        }
        if h[0] == 0 {
            return true;
        }
        let saved = std::mem::take(plays);
        let r = dd(h, w, plays, memo);
        *plays = saved;
        return r;
    }
    let seat = (leader + plays.len()) % 3;
    let s = plays.first().map(|&t| suit_led(t));
    let want = seat == 0;
    for t in bits(legal(h[seat], s)) {
        h[seat] &= !(1 << t);
        plays.push(t);
        let r = dd(h, leader, plays, memo);
        plays.pop();
        h[seat] |= 1 << t;
        if r == want {
            return want;
        }
    }
    !want
}

// ------------------------------------------------------------- blind policies
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
    fn pick(&mut self, m: Set) -> usize {
        let v: Vec<usize> = bits(m).collect();
        v[self.below(v.len())]
    }
}

/// Heuristic declarer who sees only its own hand: duck with the highest tile
/// that stays under; if it cannot duck, play its lowest follower and hope; when
/// void, discard its biggest tile (a dry 6-5 goes first).
fn declarer_play(hand: Set, plays: &[usize]) -> usize {
    let s = suit_led(plays[0]);
    let lg = legal(hand, Some(s));
    if bits(lg).any(|t| follows(t, s)) {
        let top = rank(plays[winning(plays)], s);
        let under = bits(lg).filter(|&t| rank(t, s) < top).max_by_key(|&t| rank(t, s));
        under.unwrap_or_else(|| bits(lg).min_by_key(|&t| rank(t, s)).unwrap())
    } else {
        bits(lg).max_by_key(|&t| { let (h, l) = tile(t); (h + l, h) }).unwrap()
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Defense {
    /// Uniformly random legal tile everywhere (the random arm of the PR #90 screen).
    Random,
    /// Random leads (they cannot see the declarer), but correct following:
    /// never overtake a declarer tile that is winning, else play lowest.
    Blind,
}

fn defender_play(hand: Set, plays: &[usize], leader: usize, d: Defense, rng: &mut Rng) -> usize {
    if plays.is_empty() || d == Defense::Random {
        let s = plays.first().map(|&t| suit_led(t));
        return rng.pick(legal(hand, s));
    }
    let s = suit_led(plays[0]);
    let lg = legal(hand, Some(s));
    let w = winning(plays);
    let declarer_winning = (leader + w) % 3 == 0;
    let top = rank(plays[w], s);
    if declarer_winning {
        // keep under the declarer; among safe tiles shed the highest
        let safe: Vec<usize> = bits(lg).filter(|&t| !follows(t, s) || rank(t, s) < top).collect();
        if let Some(&t) = safe.iter().max_by_key(|&&t| if follows(t, s) { rank(t, s) } else { 0 }) {
            return t;
        }
        return bits(lg).min_by_key(|&t| rank(t, s)).unwrap();
    }
    // declarer still to play (or already under): play as low as possible
    bits(lg).min_by_key(|&t| if follows(t, s) { rank(t, s) } else { 0 }).unwrap()
}

/// One game from the given opening lead; true iff the declarer makes.
fn play_blind(mut h: [Set; 3], lead: usize, d: Defense, rng: &mut Rng) -> bool {
    let mut leader = 0;
    let mut first = Some(lead);
    for _ in 0..7 {
        let mut plays = Vec::with_capacity(3);
        for k in 0..3 {
            let seat = (leader + k) % 3;
            let t = if seat == 0 {
                first.take().unwrap_or_else(|| declarer_play(h[0], &plays))
            } else {
                defender_play(h[seat], &plays, leader, d, rng)
            };
            debug_assert!(h[seat] >> t & 1 == 1);
            h[seat] &= !(1 << t);
            plays.push(t);
        }
        let w = (leader + winning(&plays)) % 3;
        if w == 0 {
            return false;
        }
        leader = w;
    }
    true
}

// ------------------------------------------------------------------- driver
fn deal(rng: &mut Rng, rest: &[usize]) -> [Set; 3] {
    let mut v = rest.to_vec();
    for i in (1..v.len()).rev() {
        v.swap(i, rng.below(i + 1));
    }
    let set = |s: &[usize]| s.iter().fold(0, |m, &t| m | 1 << t);
    [0, set(&v[0..7]), set(&v[7..14])] // v[14..21] is the partner's, out of play
}

fn parse(hand: &str) -> Set {
    hand.split(',').map(|s| {
        let p: Vec<u8> = s.trim().split('-').map(|x| x.parse().unwrap()).collect();
        index(p[0], p[1])
    }).fold(0, |m, t| m | 1 << t)
}

/// Every C(21,7)*C(14,7) = 399,072,960 split of the defenders' hands.
fn dd_exact(me: Set, threads: usize) -> (u64, u64) {
    let rest: Vec<usize> = bits(!me & ((1 << N) - 1)).collect();
    let ls = combos(&rest, 7);
    let chunks: Vec<Vec<Set>> = ls.chunks(ls.len().div_ceil(threads)).map(|c| c.to_vec()).collect();
    let hs: Vec<_> = chunks.into_iter().map(|chunk| {
        let rest = rest.clone();
        thread::spawn(move || {
            let (mut make, mut all) = (0u64, 0u64);
            let mut memo = HashMap::new();
            for l in chunk {
                let r_pool: Vec<usize> = rest.iter().copied().filter(|&t| l >> t & 1 == 0).collect();
                for r in combos(&r_pool, 7) {
                    let mut h = [me, l, r];
                    let mut plays = Vec::with_capacity(3);
                    memo.clear();
                    make += dd(&mut h, 0, &mut plays, &mut memo) as u64;
                    all += 1;
                }
            }
            (make, all)
        })
    }).collect();
    hs.into_iter().map(|h| h.join().unwrap()).fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
}
fn combos(pool: &[usize], k: usize) -> Vec<Set> {
    let mut out = Vec::new();
    fn go(pool: &[usize], k: usize, start: usize, acc: Set, out: &mut Vec<Set>) {
        if k == 0 { out.push(acc); return; }
        for i in start..=pool.len() - k { go(pool, k - 1, i + 1, acc | 1 << pool[i], out); }
    }
    go(pool, k, 0, 0, &mut out);
    out
}

/// Per mille with one decimal, integer only: "12.3%" style from a/b.
fn pct(a: u64, b: u64) -> String {
    let t = (a * 1000 + b / 2) / b;
    format!("{}.{}%", t / 10, t % 10)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args[1].as_str();
    let me = parse(&args[2]);
    assert_eq!(me.count_ones(), 7, "hand must be 7 distinct tiles");
    match mode {
        "dd" => {
            let (m, a) = dd_exact(me, 4);
            println!("dd\t{}\t{m}/{a}\t{}", args[2], pct(m, a));
        }
        "ddsample" => {
            let games: u64 = args[3].parse().unwrap();
            let rest: Vec<usize> = bits(!me & ((1 << N) - 1)).collect();
            let mut rng = Rng(0x42_6E65_6C6C_6F);
            let mut m = 0u64;
            let mut memo = HashMap::new();
            for _ in 0..games {
                let mut h = deal(&mut rng, &rest);
                h[0] = me;
                memo.clear();
                m += dd(&mut h, 0, &mut Vec::with_capacity(3), &mut memo) as u64;
            }
            println!("ddsample\t{}\t{m}/{games}\t{}", args[2], pct(m, games));
        }
        "blind" => {
            let games: u64 = args[3].parse().unwrap();
            let rest: Vec<usize> = bits(!me & ((1 << N) - 1)).collect();
            for d in [Defense::Random, Defense::Blind] {
                let mut best = (0u64, 0usize);
                let mut row = String::new();
                for lead in bits(me) {
                    // common random numbers across leads: same deals, same seed
                    let mut rng = Rng(0x42_6E65_6C6C_6F);
                    let mut m = 0u64;
                    for _ in 0..games {
                        let mut h = deal(&mut rng, &rest);
                        h[0] = me;
                        m += play_blind(h, lead, d, &mut rng) as u64;
                    }
                    row += &format!(" {}={}", name(lead), pct(m, games));
                    if m > best.0 { best = (m, lead); }
                }
                // two standard errors, integer sqrt, in tenths of a percent
                let (m, n) = (best.0, games);
                let var = m * (n - m) / n; // n * p * (1-p)
                let se2 = 2 * isqrt(var) * 1000 / n;
                println!("{}\t{}\tbest lead {}\t{m}/{n}\t{} (+/- {}.{}%)\tleads:{row}",
                    if d == Defense::Random { "random" } else { "blind" },
                    args[2], name(best.1), pct(m, n), se2 / 10, se2 % 10);
            }
        }
        _ => panic!("mode dd|blind"),
    }
}
fn isqrt(x: u64) -> u64 { let mut r = 0u64; while (r + 1) * (r + 1) <= x { r += 1; } r }

#[cfg(test)]
mod tests {
    use super::*;
    fn t(s: &str) -> usize { let p: Vec<u8> = s.split('-').map(|x| x.parse().unwrap()).collect(); index(p[0], p[1]) }
    #[test]
    fn rules() {
        // 6-5 tops both the sixes and the fives
        assert_eq!(winning(&[t("6-0"), t("6-5"), t("6-4")]), 1);
        assert_eq!(winning(&[t("5-0"), t("5-4"), t("6-5")]), 2);
        // doubles are their own suit: 6-6 cannot follow (or win) a six lead
        assert_eq!(winning(&[t("6-0"), t("6-6"), t("1-0")]), 0);
        assert!(!follows(t("6-6"), 6));
        // doubles rank by pip, mixed tiles cannot win a doubles lead
        assert_eq!(winning(&[t("1-1"), t("0-0"), t("6-5")]), 0);
        assert_eq!(winning(&[t("1-1"), t("2-2"), t("6-5")]), 1);
        // mixed lead is its high pip
        assert_eq!(suit_led(t("3-1")), 3);
        assert_eq!(winning(&[t("3-1"), t("1-0"), t("3-2")]), 2);
        assert_eq!(bits((1 << N) - 1).count(), 28);
    }
    #[test]
    fn dd_trivial() {
        // declarer holds all six sixes' worst enemy: every 0-x low... sanity: a
        // declarer holding 6-6 with defenders void in doubles leads it and wins.
        let me = [t("6-6"), t("6-5"), t("6-4"), t("6-3"), t("6-2"), t("6-1"), t("6-0")].iter().fold(0, |m, &x| m | 1 << x);
        let rest: Vec<usize> = bits(!me & ((1 << N) - 1)).collect();
        let mut rng = Rng(1);
        let mut h = deal(&mut rng, &rest);
        h[0] = me;
        let mut p = Vec::new();
        assert!(!dd(&mut h, 0, &mut p, &mut HashMap::new()));
    }
}
