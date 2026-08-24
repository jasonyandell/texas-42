//! EXPLORATORY TILT AUDIT (E0 smoke) — sits below every evidentiary tier and
//! is cited by nothing above it. Estimates, never receipts; not a P-A21
//! statement.
//!
//! Design: `walt/TILT-AUDIT.md`, under the signed-pivotal adjudication
//! (`CENSUS-RULINGS.md` SP-A1..SP-A12). For a root decision, discover the
//! seat's choice across independent discovery seeds (Phase A), freeze the
//! implied policies (SP-A3/SP-A8: the freeze tuple IS the policy — focal
//! decisions re-derive their rng from (discovery seed, hand, record), so
//! the policy is a pure function of the observation record), replay policy
//! pairs on a common panel of worlds disjoint-by-seed from discovery
//! (O13), and report the signed pivotal profile per pair: pivotal mass q,
//! tilt tau, gap g, fixed-pair hardness H (SP-A4 names).
//!
//! Because the modeled field is DETERMINISTIC in (seat, hand, record) —
//! `Solver::modeled_choice`, no external tape — a scenario collapses to a
//! world here: every world is tape-stable by construction and the parent's
//! Phase E is vacuous until a stochastic field model exists (SP-A6 note).
//!
//! Statistics are exact integer/rational arithmetic: q, tau, g reported in
//! basis points / per-mille from raw counts (N+, N-, N0); H exactly as
//! (K*n - D^2)/D^2 where K = N+ + N-, D = N+ - N-.
//!
//! Roots: fresh self-played hands under the audit's own seeds (SP-A10
//! third bullet). Internal S1 bids 30 (the miner's protocol
//! simplification), declaration by the dropped-30 heuristic, path to the
//! root played focal-level-1 / field-level-0; the audited root is S1's
//! first decision with >= 2 legal moves at trick >= t_target.

use std::sync::Arc;
use std::time::Instant;

use num_rational::BigRational;

use walt::rules::{legal_plays, Context, Decl, Domino, Pip, Seat, Team};
use walt::solver::{
    best_of, bit, bp, level1_evaluate, level1_race, level1_race_refined, level1_raced, mask_bits,
    mask_of, mix, record_hash, sample_belief, set_of, Deadline, Field, Key, Shared, Solver,
    SplitMix64, FULL_MASK,
};

/// Frozen tilt-audit stream seeds (fresh constants; deal/path, discovery,
/// and panel draw from separate streams — the O13 separation is by seed).
const TILT_SEED: u64 = 0x7117_A0D1_7B1D_C0DE;
const DISC_SEED: u64 = 0xD15C_04E2_8AED_2A6B;
const PANEL_SEED: u64 = 0x9A4E_1000_51DE_B00C;

const BID: u8 = 30;

#[derive(Clone)]
struct State {
    played: u32,
    leader: u8,
    plays: Vec<u8>,
    banked_t1: u8,
    banked_t0: u8,
    voids: [u32; 4],
    trick_start_played: u32,
    completed: usize,
}

fn advance(st: &mut State, dcl: Decl, tile: Domino) {
    let seat = (usize::from(st.leader) + st.plays.len()) % 4;
    if let Some(&led0) = st.plays.first() {
        let led = dcl.led_context(Domino::from_index(usize::from(led0)).expect("led"));
        if !dcl.follows(tile, led) {
            st.voids[seat] |= mask_of(dcl.effective_incidence(led));
        }
    }
    st.plays.push(tile.index() as u8);
    st.played |= bit(tile);
    if st.plays.len() == 4 {
        let doms = [
            Domino::from_index(usize::from(st.plays[0])).expect("p0"),
            Domino::from_index(usize::from(st.plays[1])).expect("p1"),
            Domino::from_index(usize::from(st.plays[2])).expect("p2"),
            Domino::from_index(usize::from(st.plays[3])).expect("p3"),
        ];
        let leader = Seat::from_index(usize::from(st.leader)).expect("leader");
        let trick = walt::rules::Trick::new(leader, doms).expect("distinct tiles");
        let winner = trick.winner(dcl);
        let value = trick.points() as u8;
        if winner.team() == Team::T1 {
            st.banked_t1 += value;
        } else {
            st.banked_t0 += value;
        }
        st.leader = winner.index() as u8;
        st.plays.clear();
        st.trick_start_played = st.played;
        st.completed += 1;
    }
}

fn key_of(st: &State) -> Key {
    Key {
        played: st.played,
        leader: st.leader,
        plays: st.plays.clone(),
        banked_t1: st.banked_t1,
        banked_t0: st.banked_t0,
        alive: 0,
    }
}

fn sizes_at(st: &State) -> [usize; 4] {
    let mut sizes = [7 - st.completed; 4];
    for i in 0..st.plays.len() {
        sizes[(usize::from(st.leader) + i) % 4] -= 1;
    }
    sizes
}

/// The dropped-30 declaration heuristic (miner protocol): longest suit,
/// then holding the double, then higher pip.
fn declare_heuristic(hand: u32) -> Decl {
    let mut best: Option<(usize, bool, u8)> = None;
    for p in 0u8..=6 {
        let pip = Pip::new(p).expect("pip");
        let mut len = 0usize;
        let mut has_double = false;
        for t in mask_bits(hand) {
            let dm = Domino::from_index(usize::from(t)).expect("tile");
            if dm.hi() == pip || dm.lo() == pip {
                len += 1;
                if dm.hi() == dm.lo() {
                    has_double = true;
                }
            }
        }
        let cand = (len, has_double, p);
        if best.is_none_or(|b| cand > b) {
            best = Some(cand);
        }
    }
    let (_, _, p) = best.expect("some pip");
    Decl::PipTrump(Pip::new(p).expect("pip"))
}

fn tile_name(t: u8) -> String {
    let dm = Domino::from_index(usize::from(t)).expect("tile");
    format!("{}{}", dm.hi().value(), dm.lo().value())
}

fn legal_at(st: &State, dcl: Decl, hand_rem: u32) -> u32 {
    let led: Option<Context> = st
        .plays
        .first()
        .map(|&i| dcl.led_context(Domino::from_index(usize::from(i)).expect("led index")));
    mask_of(legal_plays(dcl, set_of(hand_rem), led))
}

/// The frozen policy's rng at an information state: pure in
/// (discovery seed, hand index, observation record) — the SP-A3 seed
/// schedule. Discovery at the root and replayed continuations use the SAME
/// derivation, so the discovered choice IS the frozen policy's root choice.
fn policy_rng(h: u64, seed_i: u64, key: &Key) -> SplitMix64 {
    SplitMix64(DISC_SEED ^ mix(h) ^ mix(seed_i) ^ record_hash(key))
}

struct Cfg {
    n_disc: usize,
    n0: usize,
    secs: u64,
}

/// Play one world forward from the root under (frozen focal policy, level-0
/// field), first focal action forced. Returns Some(made) or None on budget
/// death. `rem` are the four remaining hands of the world at the root.
#[allow(clippy::too_many_arguments)]
fn replay_world(
    dcl: Decl,
    root: &State,
    world_rem: [u32; 4],
    first: u8,
    h: u64,
    seed_i: u64,
    cfg: &Cfg,
    field_host: &Solver,
) -> Option<bool> {
    let mut st = root.clone();
    let mut rem = world_rem;
    let mut forced_first = Some(first);
    while st.played != FULL_MASK {
        let seat_idx = (usize::from(st.leader) + st.plays.len()) % 4;
        let hand = rem[seat_idx];
        let legal = legal_at(&st, dcl, hand);
        let choice: u8 = if seat_idx == 1 {
            if let Some(f) = forced_first.take() {
                assert!(legal & (1u32 << f) != 0, "forced root action is legal");
                f
            } else if legal.count_ones() == 1 {
                legal.trailing_zeros() as u8
            } else {
                let key = key_of(&st);
                let mut rng = policy_rng(h, seed_i, &key);
                let opts = level1_evaluate(
                    dcl,
                    BID,
                    Seat::from_index(1).expect("seat"),
                    hand,
                    legal,
                    &key,
                    sizes_at(&st),
                    st.voids,
                    st.trick_start_played,
                    7 - st.completed,
                    cfg.n_disc,
                    cfg.n0,
                    cfg.secs,
                    &mut rng,
                )?;
                best_of(&opts, true)
            }
        } else if legal.count_ones() == 1 {
            legal.trailing_zeros() as u8
        } else {
            let key = key_of(&st);
            let seat = Seat::from_index(seat_idx).expect("seat");
            field_host.modeled_choice(0, &key, seat, hand, legal)?
        };
        let tile = Domino::from_index(usize::from(choice)).expect("tile");
        rem[seat_idx] &= !bit(tile);
        advance(&mut st, dcl, tile);
    }
    Some(st.banked_t1 >= BID)
}

struct RootPos {
    dcl: Decl,
    st: State,
    rem1: u32,
    legal: u32,
}

/// Deal hand h and play forward (focal = level-1 at the path freeze, field
/// = level-0) to S1's first decision with >= 2 legal moves at trick >=
/// t_target. None if the hand ends first or a budget dies.
fn find_root(h: u64, t_target: usize, cfg: &Cfg, field_host: &Solver) -> Option<RootPos> {
    let mut rng = SplitMix64(TILT_SEED ^ mix(h));
    let mut tiles: Vec<u8> = (0..28).collect();
    for i in (1..tiles.len()).rev() {
        let j = rng.below((i + 1) as u64) as usize;
        tiles.swap(i, j);
    }
    let mut hands = [0u32; 4];
    for (s, hand) in hands.iter_mut().enumerate() {
        for &t in &tiles[7 * s..7 * (s + 1)] {
            *hand |= 1u32 << t;
        }
    }
    let dcl = declare_heuristic(hands[1]);
    let mut st = State {
        played: 0,
        leader: 1,
        plays: Vec::new(),
        banked_t1: 0,
        banked_t0: 0,
        voids: [0; 4],
        trick_start_played: 0,
        completed: 0,
    };
    while st.played != FULL_MASK {
        let seat_idx = (usize::from(st.leader) + st.plays.len()) % 4;
        let hand = hands[seat_idx] & !st.played;
        let legal = legal_at(&st, dcl, hand);
        if seat_idx == 1 && st.completed >= t_target && legal.count_ones() >= 2 {
            return Some(RootPos {
                dcl,
                st,
                rem1: hand,
                legal,
            });
        }
        let choice: u8 = if legal.count_ones() == 1 {
            legal.trailing_zeros() as u8
        } else if seat_idx == 1 {
            // Path scaffolding: the pre-root focal policy is the seed-0
            // frozen policy itself, so the root is on-policy for seed 0.
            let key = key_of(&st);
            let mut prng = policy_rng(h, 0, &key);
            let opts = level1_evaluate(
                dcl,
                BID,
                Seat::from_index(1).expect("seat"),
                hand,
                legal,
                &key,
                sizes_at(&st),
                st.voids,
                st.trick_start_played,
                7 - st.completed,
                cfg.n_disc,
                cfg.n0,
                cfg.secs,
                &mut prng,
            )?;
            best_of(&opts, true)
        } else {
            let key = key_of(&st);
            let seat = Seat::from_index(seat_idx).expect("seat");
            field_host.modeled_choice(0, &key, seat, hand, legal)?
        };
        advance(
            &mut st,
            dcl,
            Domino::from_index(usize::from(choice)).expect("tile"),
        );
    }
    None
}

/// Exact pair statistics from two outcome bit-vectors over the same panel.
struct PairStats {
    n_plus: usize,
    n_minus: usize,
    n: usize,
}

impl PairStats {
    fn of(a: &[bool], b: &[bool]) -> Self {
        let n_plus = a.iter().zip(b).filter(|(x, y)| **x && !**y).count();
        let n_minus = a.iter().zip(b).filter(|(x, y)| !**x && **y).count();
        PairStats {
            n_plus,
            n_minus,
            n: a.len(),
        }
    }
    fn k(&self) -> usize {
        self.n_plus + self.n_minus
    }
    fn d(&self) -> i64 {
        self.n_plus as i64 - self.n_minus as i64
    }
    /// q in basis points (rounded down).
    fn q_bp(&self) -> u64 {
        (self.k() as u64 * 10_000) / self.n as u64
    }
    /// tau in per-mille (signed, rounded toward zero); None if K == 0.
    fn tau_pm(&self) -> Option<i64> {
        if self.k() == 0 {
            None
        } else {
            Some(self.d() * 1000 / self.k() as i64)
        }
    }
    /// H = (K*n - D^2)/D^2 exactly; None if D == 0.
    fn hardness(&self) -> Option<(i128, i128)> {
        let d2 = i128::from(self.d()) * i128::from(self.d());
        if d2 == 0 {
            None
        } else {
            Some(((self.k() as i128) * (self.n as i128) - d2, d2))
        }
    }
}

#[allow(clippy::too_many_lines)]
/// One field host per hand: the pi cache is pure in (k, PiKey), so the
/// path and every replay share field decisions (SP-A8 economy).
fn hand_host(h: u64, n_inner: Vec<usize>) -> Solver {
    let mut rng = SplitMix64(TILT_SEED ^ mix(h));
    let mut tiles: Vec<u8> = (0..28).collect();
    for i in (1..tiles.len()).rev() {
        let j = rng.below((i + 1) as u64) as usize;
        tiles.swap(i, j);
    }
    let bidder_hand = tiles[7..14].iter().fold(0u32, |a, &t| a | (1u32 << t));
    let sh = Arc::new(Shared::new(
        declare_heuristic(bidder_hand),
        BID,
        n_inner,
        0,
        7,
        Deadline::after(std::time::Duration::from_secs(86_400)),
    ));
    Solver::new(
        sh,
        Seat::from_index(1).expect("seat"),
        0,
        true,
        Vec::new(),
        Vec::new(),
        Field::Level(0),
    )
}

/// `tiltaudit bench` — the racing evaluator vs the full evaluator, head to
/// head on the same roots: agreement, worlds consumed, wall clock.
fn bench(args: &[String]) {
    let arg = |i: usize, dflt: usize| args.get(i).map_or(dflt, |s| s.parse().expect("usize arg"));
    let n_hands = arg(2, 12) as u64;
    let t_target = arg(3, 3);
    let n_full = arg(4, 40);
    let n0 = arg(5, 8);
    let n_max = arg(6, 100);
    let n_self = arg(7, 20);
    let secs = arg(8, 120) as u64;
    let cfg = Cfg {
        n_disc: n_full,
        n0,
        secs,
    };
    println!("TILT RACE BENCH — EXPLORATORY; estimates, never receipts");
    println!(
        "hands={n_hands} t_target={t_target} full: n={n_full} n0={n0} | race: n_max={n_max} n0={n0} n_self={n_self} ({} threads)\n",
        rayon::current_num_threads()
    );
    let mut roots = 0usize;
    let mut agree = 0usize;
    let mut blk_agree = 0usize;
    let mut full_ms_sum = 0u128;
    let mut race_ms_sum = 0u128;
    let mut blk_ms_sum = 0u128;
    let mut worlds_sum = 0usize;
    let mut blk_worlds_sum = 0usize;
    for h in 0..n_hands {
        let field_host = hand_host(h, vec![cfg.n0]);
        let Some(root) = find_root(h, t_target, &cfg, &field_host) else {
            println!("hand {h}: no eligible root — skipped");
            continue;
        };
        let key = key_of(&root.st);
        let seat = Seat::from_index(1).expect("seat");

        let t_full = Instant::now();
        let mut frng = policy_rng(h, 0, &key);
        let Some(opts) = level1_evaluate(
            root.dcl,
            BID,
            seat,
            root.rem1,
            root.legal,
            &key,
            sizes_at(&root.st),
            root.st.voids,
            root.st.trick_start_played,
            7 - root.st.completed,
            n_full,
            cfg.n0,
            secs,
            &mut frng,
        ) else {
            println!("hand {h}: full evaluation died — skipped");
            continue;
        };
        let full_choice = best_of(&opts, true);
        let full_ms = t_full.elapsed().as_millis();

        let t_race = Instant::now();
        let mut rrng = SplitMix64(PANEL_SEED ^ mix(h) ^ record_hash(&key));
        let Some(race) = level1_race(
            root.dcl,
            BID,
            seat,
            root.rem1,
            root.legal,
            &key,
            sizes_at(&root.st),
            root.st.voids,
            root.st.trick_start_played,
            7 - root.st.completed,
            n_max,
            cfg.n0,
            n_self,
            secs,
            &mut rrng,
        ) else {
            println!("hand {h}: race died — skipped");
            continue;
        };
        let race_ms = t_race.elapsed().as_millis();

        let t_blk = Instant::now();
        let mut brng = SplitMix64(PANEL_SEED ^ mix(h) ^ 0xB10C_0000_0000_0001 ^ record_hash(&key));
        let Some(blk) = level1_raced(
            root.dcl,
            BID,
            seat,
            root.rem1,
            root.legal,
            &key,
            sizes_at(&root.st),
            root.st.voids,
            root.st.trick_start_played,
            7 - root.st.completed,
            n_max,
            cfg.n0,
            8,
            secs,
            &mut brng,
        ) else {
            println!("hand {h}: block race died — skipped");
            continue;
        };
        let blk_ms = t_blk.elapsed().as_millis();

        roots += 1;
        let same = race.choice == full_choice;
        if same {
            agree += 1;
        }
        let bsame = blk.choice == full_choice;
        if bsame {
            blk_agree += 1;
        }
        full_ms_sum += full_ms;
        race_ms_sum += race_ms;
        blk_ms_sum += blk_ms;
        worlds_sum += race.worlds_used;
        blk_worlds_sum += blk.worlds_used;
        let tally: Vec<String> = race
            .tally
            .iter()
            .map(|(t, s, n)| format!("{}:{s}/{n}", tile_name(*t)))
            .collect();
        let survivors: Vec<String> = blk
            .values
            .iter()
            .map(|(t, v, w)| format!("{}:{}bp/{w}w", tile_name(*t), bp(v)))
            .collect();
        println!(
            "hand {h} trick {} ({} legal): full {} {full_ms}ms | replay-race {} {}w {race_ms}ms | block-race {} {}w {blk_ms}ms",
            root.st.completed + 1,
            root.legal.count_ones(),
            tile_name(full_choice),
            tile_name(race.choice),
            race.worlds_used,
            tile_name(blk.choice),
            blk.worlds_used,
        );
        println!(
            "  replay tally [{}]  block survivors [{}]{}{}",
            tally.join(" "),
            survivors.join(" "),
            if same { "" } else { "  << replay DISAGREES" },
            if bsame { "" } else { "  << block DISAGREES" }
        );
    }
    if roots > 0 {
        println!(
            "\nvs full: replay-race agrees {agree}/{roots} (mean {}ms, {} worlds), block-race agrees {blk_agree}/{roots} (mean {}ms, {} worlds); full mean {}ms",
            race_ms_sum / roots as u128,
            worlds_sum / roots,
            blk_ms_sum / roots as u128,
            blk_worlds_sum / roots,
            full_ms_sum / roots as u128,
        );
    }
    println!("nothing above exploratory tier; race choices are a play policy, never receipts");
}

/// `tiltaudit arena` — mirrored self-play: on each deal, one team's open
/// decisions use race-then-refine and the other team's use the full
/// evaluator; then the sides swap on the same deal. Paired outcome: did
/// the bidding team (always internal T1, bid 30) make, under each mode?
/// Same exact-binomial summary as the races. No modeled field — all four
/// seats are real deciders.
fn arena(args: &[String]) {
    let arg = |i: usize, dflt: usize| args.get(i).map_or(dflt, |s| s.parse().expect("usize arg"));
    let n_hands = arg(2, 24) as u64;
    let n = arg(3, 40);
    let n0 = arg(4, 8);
    let secs = arg(5, 120) as u64;
    println!("TILT ARENA — race-refined vs full, mirrored deals — EXPLORATORY");
    println!("hands={n_hands} n={n} n0={n0} (race cap 2n, refine n)\n");
    const ARENA_SEED: u64 = 0xA2E4_A000_0000_0042;
    let mut made = [[false; 2]; 64]; // [hand][flip: 0 = race bids]
    let mut ms_race = 0u128;
    let mut ms_full = 0u128;
    let mut dec_race = 0u64;
    let mut dec_full = 0u64;
    for h in 0..n_hands {
        for flip in 0..2u64 {
            let mut rng = SplitMix64(TILT_SEED ^ mix(h));
            let mut tiles: Vec<u8> = (0..28).collect();
            for i in (1..tiles.len()).rev() {
                let j = rng.below((i + 1) as u64) as usize;
                tiles.swap(i, j);
            }
            let mut hands = [0u32; 4];
            for (s, hand) in hands.iter_mut().enumerate() {
                for &t in &tiles[7 * s..7 * (s + 1)] {
                    *hand |= 1u32 << t;
                }
            }
            let dcl = declare_heuristic(hands[1]);
            let mut st = State {
                played: 0,
                leader: 1,
                plays: Vec::new(),
                banked_t1: 0,
                banked_t0: 0,
                voids: [0; 4],
                trick_start_played: 0,
                completed: 0,
            };
            let mut dead = false;
            while st.played != FULL_MASK {
                let seat_idx = (usize::from(st.leader) + st.plays.len()) % 4;
                let hand = hands[seat_idx] & !st.played;
                let legal = legal_at(&st, dcl, hand);
                let choice: u8 = if legal.count_ones() == 1 {
                    legal.trailing_zeros() as u8
                } else {
                    let seat = Seat::from_index(seat_idx).expect("seat");
                    let on_t1 = seat.team() == Team::T1;
                    let racing = on_t1 == (flip == 0);
                    let key = key_of(&st);
                    let mut drng = SplitMix64(ARENA_SEED ^ mix(h) ^ mix(flip) ^ record_hash(&key));
                    let t0 = Instant::now();
                    let pick = if racing {
                        level1_race_refined(
                            dcl,
                            BID,
                            seat,
                            hand,
                            legal,
                            &key,
                            sizes_at(&st),
                            st.voids,
                            st.trick_start_played,
                            7 - st.completed,
                            2 * n,
                            n,
                            n0,
                            secs,
                            &mut drng,
                        )
                    } else {
                        level1_evaluate(
                            dcl,
                            BID,
                            seat,
                            hand,
                            legal,
                            &key,
                            sizes_at(&st),
                            st.voids,
                            st.trick_start_played,
                            7 - st.completed,
                            n,
                            n0,
                            secs,
                            &mut drng,
                        )
                        .map(|opts| best_of(&opts, on_t1))
                    };
                    let ms = t0.elapsed().as_millis();
                    if racing {
                        ms_race += ms;
                        dec_race += 1;
                    } else {
                        ms_full += ms;
                        dec_full += 1;
                    }
                    match pick {
                        Some(t) => t,
                        None => {
                            dead = true;
                            break;
                        }
                    }
                };
                advance(
                    &mut st,
                    dcl,
                    Domino::from_index(usize::from(choice)).expect("tile"),
                );
            }
            if dead {
                println!("hand {h} flip {flip}: budget death — pair dropped");
                continue;
            }
            made[h as usize][flip as usize] = st.banked_t1 >= BID;
            println!(
                "hand {h} flip {flip}: bidding team ({}) {} — {}:{}",
                if flip == 0 { "RACE" } else { "full" },
                if st.banked_t1 >= BID { "MADE" } else { "SET " },
                st.banked_t1,
                st.banked_t0
            );
        }
    }
    let n_plus = (0..n_hands as usize)
        .filter(|&i| made[i][0] && !made[i][1])
        .count();
    let n_minus = (0..n_hands as usize)
        .filter(|&i| !made[i][0] && made[i][1])
        .count();
    let both = (0..n_hands as usize)
        .filter(|&i| made[i][0] && made[i][1])
        .count();
    println!(
        "\npaired makes as bidder over {n_hands} deals: race-only {n_plus}, full-only {n_minus}, both {both}"
    );
    println!(
        "decision cost: race {}ms/{} decisions (mean {}ms) vs full {}ms/{} (mean {}ms)",
        ms_race,
        dec_race,
        ms_race / u128::from(dec_race.max(1)),
        ms_full,
        dec_full,
        ms_full / u128::from(dec_full.max(1)),
    );
    println!("nothing above exploratory tier; arena outcomes are never receipts");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("bench") {
        bench(&args);
        return;
    }
    if args.get(1).map(String::as_str) == Some("arena") {
        arena(&args);
        return;
    }
    let arg = |i: usize, dflt: usize| args.get(i).map_or(dflt, |s| s.parse().expect("usize arg"));
    let n_hands = arg(1, 4) as u64;
    let t_target = arg(2, 4);
    let n_seeds = arg(3, 4) as u64;
    let panel_n = arg(4, 200);
    let cfg = Cfg {
        n_disc: arg(5, 200),
        n0: arg(6, 8),
        secs: arg(7, 120) as u64,
    };
    println!("TILT AUDIT (E0 smoke) — EXPLORATORY; estimates, never receipts");
    println!(
        "hands={n_hands} t_target={t_target} seeds={n_seeds} panel={panel_n} n_disc={} n0={} ({} rayon threads)",
        cfg.n_disc, cfg.n0, rayon::current_num_threads()
    );
    println!("field: deterministic level-0 (no tape — every world tape-stable; Phase E vacuous)\n");

    let mut audited = 0usize;
    for h in 0..n_hands {
        let field_host = hand_host(h, vec![cfg.n0]);
        let sh = Arc::clone(&field_host.sh);

        let t0 = Instant::now();
        let Some(root) = find_root(h, t_target, &cfg, &field_host) else {
            println!("hand {h}: no eligible root (ended or died) — skipped\n");
            continue;
        };
        audited += 1;
        let path_ms = t0.elapsed().as_millis();
        let key = key_of(&root.st);
        let names: Vec<String> = mask_bits(root.rem1).iter().map(|&t| tile_name(t)).collect();
        println!(
            "hand {h}: root at trick {} pos {} — S1 holds [{}], {} legal, decl {:?}, banked T1:{} T0:{} (path {path_ms}ms)",
            root.st.completed + 1,
            root.st.plays.len(),
            names.join(" "),
            root.legal.count_ones(),
            root.dcl,
            root.st.banked_t1,
            root.st.banked_t0
        );

        // Phase A — discovery replicates.
        let t_a = Instant::now();
        let mut choices: Vec<u8> = Vec::new();
        let mut pooled: Vec<(u8, BigRational)> = Vec::new();
        for i in 0..n_seeds {
            let mut rng = policy_rng(h, i, &key);
            let Some(opts) = level1_evaluate(
                root.dcl,
                BID,
                Seat::from_index(1).expect("seat"),
                root.rem1,
                root.legal,
                &key,
                sizes_at(&root.st),
                root.st.voids,
                root.st.trick_start_played,
                7 - root.st.completed,
                cfg.n_disc,
                cfg.n0,
                cfg.secs,
                &mut rng,
            ) else {
                println!("  discovery seed {i} died — hand skipped\n");
                continue;
            };
            let c = best_of(&opts, true);
            choices.push(c);
            let line: Vec<String> = opts
                .iter()
                .map(|(t, v)| format!("{}:{}", tile_name(*t), bp(v)))
                .collect();
            println!(
                "  disc seed {i}: choice {} [{}]",
                tile_name(c),
                line.join(" ")
            );
            for (t, v) in opts {
                match pooled.iter_mut().find(|(pt, _)| *pt == t) {
                    Some(slot) => slot.1 += v,
                    None => pooled.push((t, v)),
                }
            }
        }
        if choices.is_empty() {
            continue;
        }
        // Top-2 root actions: by choice frequency, then pooled value.
        let mut freq: Vec<(u8, usize)> = Vec::new();
        for &c in &choices {
            match freq.iter_mut().find(|(t, _)| *t == c) {
                Some(f) => f.1 += 1,
                None => freq.push((c, 1)),
            }
        }
        freq.sort_by_key(|&(t, n)| (std::cmp::Reverse(n), t));
        let a_star = freq[0].0;
        let b_star = if freq.len() > 1 {
            freq[1].0
        } else {
            let mut ranked = pooled.clone();
            ranked.sort_by(|x, y| y.1.cmp(&x.1).then(x.0.cmp(&y.0)));
            let second = ranked.iter().find(|(t, _)| *t != a_star);
            match second {
                Some(&(t, _)) => t,
                None => {
                    println!("  single legal option after pooling — skipped\n");
                    continue;
                }
            }
        };
        println!(
            "  root pair: a*={} (chosen {}/{}) vs b*={}  (discovery {}ms)",
            tile_name(a_star),
            freq[0].1,
            choices.len(),
            tile_name(b_star),
            t_a.elapsed().as_millis()
        );

        // Panel — disjoint-by-seed from discovery (O13).
        let mut prng = SplitMix64(PANEL_SEED ^ mix(h));
        let worlds = sample_belief(
            1,
            root.rem1,
            key.played,
            sizes_at(&root.st),
            root.st.voids,
            panel_n,
            &mut prng,
        );

        // Phase B — replay both frozen policies per seed on the panel.
        let t_b = Instant::now();
        let mut bits_a: Vec<Vec<bool>> = Vec::new();
        let mut bits_b: Vec<Vec<bool>> = Vec::new();
        let mut died = false;
        for i in 0..n_seeds {
            let run = |first: u8| -> Option<Vec<bool>> {
                worlds
                    .iter()
                    .map(|w| replay_world(root.dcl, &root.st, *w, first, h, i, &cfg, &field_host))
                    .collect()
            };
            match (run(a_star), run(b_star)) {
                (Some(a), Some(b)) => {
                    bits_a.push(a);
                    bits_b.push(b);
                }
                _ => {
                    died = true;
                    break;
                }
            }
        }
        if died {
            println!("  replay died on budget — hand skipped\n");
            continue;
        }
        let replay_ms = t_b.elapsed().as_millis();

        // Pair profile per seed, and pooled.
        let mut d_total: i64 = 0;
        println!(
            "  pair profile ({} worlds; q bp, tau per-mille, H exact):",
            panel_n
        );
        for (i, (a, b)) in bits_a.iter().zip(&bits_b).enumerate() {
            let s = PairStats::of(a, b);
            d_total += s.d();
            let tau = s.tau_pm().map_or("--".to_string(), |t| t.to_string());
            let hh = s
                .hardness()
                .map_or("--".to_string(), |(num, den)| format!("{num}/{den}"));
            println!(
                "    seed {i}: N+={} N-={} N0={}  q={}bp tau={} H={}",
                s.n_plus,
                s.n_minus,
                s.n - s.k(),
                s.q_bp(),
                tau,
                hh
            );
        }
        let winner = match d_total.cmp(&0) {
            std::cmp::Ordering::Greater => format!("panel confirms a*={}", tile_name(a_star)),
            std::cmp::Ordering::Less => format!("panel PREFERS b*={}", tile_name(b_star)),
            std::cmp::Ordering::Equal => "panel is exactly tied".to_string(),
        };
        println!(
            "    pooled D={d_total} over {} paired replays — {winner}",
            bits_a.len() * panel_n
        );

        // Phase C — winner recovery by panel prefix.
        let ref_sign = d_total.signum();
        if ref_sign != 0 {
            let prefixes: Vec<usize> = [25usize, 50, 100, 200, 400]
                .iter()
                .copied()
                .filter(|&m| m <= panel_n)
                .collect();
            let mut line = String::new();
            for &m in &prefixes {
                let rec = bits_a
                    .iter()
                    .zip(&bits_b)
                    .filter(|(a, b)| {
                        let s = PairStats::of(&a[..m], &b[..m]);
                        s.d().signum() == ref_sign
                    })
                    .count();
                line.push_str(&format!("  {m}:{rec}/{}", bits_a.len()));
            }
            println!("    winner recovery by prefix:{line}");
        }

        // Phase D — behavioral instability across seeds.
        for (label, bits) in [("a*", &bits_a), ("b*", &bits_b)] {
            let mut distinct: Vec<&Vec<bool>> = Vec::new();
            for v in bits.iter() {
                if !distinct.contains(&v) {
                    distinct.push(v);
                }
            }
            let mut max_dis = 0usize;
            for i in 0..bits.len() {
                for j in (i + 1)..bits.len() {
                    let d = bits[i].iter().zip(&bits[j]).filter(|(x, y)| x != y).count();
                    max_dis = max_dis.max(d);
                }
            }
            println!(
                "    {label}: {} behaviorally distinct polic{} of {}; max pairwise disagreement {}/{panel_n}",
                distinct.len(),
                if distinct.len() == 1 { "y" } else { "ies" },
                bits.len(),
                max_dis
            );
        }
        println!(
            "  (replays {replay_ms}ms; pi cache {} entries)\n",
            sh.pi_cache_len()
        );
    }
    println!("audited {audited}/{n_hands} hands");
    println!("nothing above exploratory tier; tilt-audit numbers are never receipts");
}
