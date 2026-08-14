//! walt rule-economy probe at the n4 carrier — the map-free rule walk
//! aimed where the exact route cannot go. EXPLORATORY TIER.
//!
//! Rulings: RW-A1..RW-A8, freeze 49 (walt/CENSUS-RULINGS.md, 2026-08-14).
//! The aim, per RW-A5: the separation question is live ONLY at the four
//! positive-margin coordinates (h1, h4, h5, h8) — can a cheap rule match
//! what the exact seed achieved? — while at the five negative-margin
//! coordinates (h0, h2, h6, h9, h12) no candidate whatsoever can separate
//! (Corollary E4.1(3), from the filed Q^H and U alone) and the separation
//! column is a RECEIPT of that theorem, never a measurement. The gap
//! g(a) = Q^H(a) − L_rule(a) is a genuine measurement at every action of
//! every coordinate — and at h9, the coordinate the exact route could not
//! price (517,562,322 states > P_max v2), it is the ONLY primal number
//! obtainable at all.
//!
//! Arms (freeze 49): P1 least-tile, P2 greatest-tile, P3 beat-if-able,
//! P4 trump-hoard — re-declared against the canonical ascending
//! domino-index order (EC-A1(a)); rule argument list CLOSED at
//! (record, legal), the trick context being a derived view of (kernel
//! identity, record) (RW-A1); bag size is never passed (a rule reading it
//! would be belief-dependent). Arm X at h9 prints STRUCTURALLY
//! UNAVAILABLE; arms T and R are out of scope (freeze 45 writes no
//! library entry at n = 4).
//!
//! Receipts: (RW-R1) L_rule <= Q^H asserted exactly against the filed
//! Q^H at every row; (RW-R2), BLOCKING: at the declared shared-ground
//! coordinate (grade 3, S6a idx=0, root 00) each arm is priced twice —
//! at the callback via policy_value_by_rule and materialised into a map
//! via policy_value_by_record — and the two L values asserted exactly
//! equal, before any n4 number is quoted. By-construction notices
//! (RW-A2(ii), printed once): the rule's singleton return, the equality
//! of the two counted halves, distinctness of reached records (the
//! tree-walk property), and legality-by-assertion are structural, not
//! evidence.
//!
//! No floats. Regenerate:
//! `cargo run --release -p walt-factory --example rule_economy_n4`

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use walt_core::receipt::{locate_verify_player, parse_file, ReceiptHand};
use walt_core::replay::state_before_trick;
use walt_core::{legal_plays, Context, ContextSet, Decl, Domino, DominoSet, Pip, Seat, Trick};
use walt_geom::{q, qi, Q};
use walt_kernel::{Hidden, Kernel, HIDDEN_SEATS};

/// Freeze 44(e): B walk-steps per (coordinate, action) evaluator call.
const B_WALK: u64 = 10_000_000_000;

const N4_TRICK: usize = 4;
const N4_GRADE: usize = 4;
const N4_FIBER: u128 = 34_650;
// The nine in-scope coordinates are carried by N4_FILED itself.

/// RW-A5/freeze-49: the filed exact side per (coordinate, action) —
/// Q^H and U in the COUNT convention, quoted from
/// `separation_n4_2026-08-14.txt` (S6h, exploratory tier), carried as a
/// frozen table in the probe source and never re-parsed from results text
/// at run time (the SEP-A14(ii) pattern).
type FiledRow = (u8, u8, (i128, i128), (i128, i128));
const N4_FILED: [(usize, u8, [FiledRow; 4]); 9] = [
    (
        0,
        3,
        [
            (0, 0, (301653329, 89812800), (7580063, 2138400)),
            (2, 1, (164419, 49896), (164419, 49896)),
            (3, 2, (83974837, 29937600), (16266721, 5702400)),
            (5, 3, (33701, 9900), (1592399, 453600)),
        ],
    ),
    (
        1,
        6,
        [
            (1, 1, (2251, 594), (2251, 594)),
            (4, 3, (113, 33), (113, 33)),
            (6, 0, (208, 55), (208, 55)),
            (6, 6, (208, 55), (208, 55)),
        ],
    ),
    (
        2,
        5,
        [
            (2, 1, (9448, 2835), (9448, 2835)),
            (3, 3, (911507, 249480), (6106181, 1663200)),
            (5, 3, (85117, 23100), (58639, 15840)),
            (5, 4, (85117, 23100), (58639, 15840)),
        ],
    ),
    (
        4,
        1,
        [
            (2, 1, (30554389, 11975040), (17198011, 6652800)),
            (4, 0, (2805661, 1108800), (30308263, 11975040)),
            (5, 1, (30554389, 11975040), (17198011, 6652800)),
            (6, 5, (2795629, 997920), (8276617, 2851200)),
        ],
    ),
    (
        5,
        5,
        [
            (3, 1, (573763969, 179625600), (192456773, 59875200)),
            (5, 1, (173402093, 59875200), (704173027, 239500800)),
            (5, 5, (546913, 158400), (211325419, 59875200)),
            (6, 3, (380069969, 119750400), (1565604113, 479001600)),
        ],
    ),
    (
        6,
        4,
        [
            (1, 1, (535997311, 239500800), (1090848503, 479001600)),
            (4, 0, (541161923, 239500800), (544949941, 239500800)),
            (4, 3, (256988827, 119750400), (5881639, 2721600)),
            (5, 3, (220757, 110880), (122035561, 59875200)),
        ],
    ),
    (
        8,
        5,
        [
            (2, 1, (94375703, 34214400), (1343147573, 479001600)),
            (3, 1, (9109429, 3421440), (649864861, 239500800)),
            (3, 3, (24053479, 7257600), (229607567, 68428800)),
            (5, 5, (15391589, 4435200), (208928077, 59875200)),
        ],
    ),
    (
        9,
        4,
        [
            (3, 0, (56497319, 19958400), (16378763, 5702400)),
            (4, 1, (28422259, 8870400), (545341, 158400)),
            (5, 4, (28422259, 8870400), (545341, 158400)),
            (6, 1, (31039087, 9979200), (31207009, 9979200)),
        ],
    ),
    (
        12,
        0,
        [
            (2, 0, (2360209, 855360), (41850301, 14968800)),
            (3, 0, (2360209, 855360), (41850301, 14968800)),
            (4, 0, (2360209, 855360), (41850301, 14968800)),
            (6, 5, (128344, 51975), (128344, 51975)),
        ],
    ),
];

fn tile(d: Domino) -> String {
    format!("{}{}", d.hi().value(), d.lo().value())
}

/// The freeze-26 bridge at the reporting boundary, as a function of the
/// declared grade (N4-A11).
fn to_diff(count: Q, grade: usize) -> Q {
    count * qi(2) - qi(i128::try_from(grade).expect("grade"))
}

fn n4_void_free_kernel(hand: &ReceiptHand) -> Kernel {
    let (hands, leader) = state_before_trick(hand, N4_TRICK).expect("the receipt replays");
    let focal = hand.bidder;
    assert_eq!(leader, focal, "leader offset 0 at the in-scope coordinates");
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
    Kernel::new(hand.decl, focal, hands[focal.index()], pool, hidden)
        .expect("the void-free capacity kernel is well formed")
}

/// The public trick context, a derived view of (kernel identity, record)
/// (RW-A1(ii)): replay the record from the root leader, tracking trick
/// winners, to the current partial trick.
fn trick_context(decl: Decl, viewer: Seat, record: &[Domino]) -> ([Domino; 4], usize) {
    let mut leader = viewer;
    let mut tiles = [Domino::ALL[0]; 4];
    let mut k = 0usize;
    for t in record {
        tiles[k] = *t;
        k += 1;
        if k == 4 {
            let trick = Trick::new(leader, tiles).expect("distinct");
            leader = trick.winner(decl);
            k = 0;
        }
    }
    (tiles, k)
}

/// The four freeze-49 arms, closed at (record, legal); the trick context is
/// derived. Information-consistency arguments (printed in the header):
/// P1/P2 read only the legal set and the canonical order; P3 reads the
/// tiles so far in the current trick — part of the public record — and the
/// declaration; P4 reads trump membership, a function of the declaration
/// and the tile. No hidden hand, no bag, no belief is consulted.
fn rule_choice(
    arm: usize,
    decl: Decl,
    viewer: Seat,
    record: &[Domino],
    legal: DominoSet,
) -> Domino {
    let least = || {
        legal
            .iter()
            .min_by_key(|d| d.index())
            .expect("a live state has a legal move")
    };
    match arm {
        1 => least(),
        2 => legal
            .iter()
            .max_by_key(|d| d.index())
            .expect("a live state has a legal move"),
        3 => {
            let (tiles, k) = trick_context(decl, viewer, record);
            if k == 0 {
                return least();
            }
            let led = decl.led_context(tiles[0]);
            legal
                .iter()
                .filter(|c| (0..k).all(|i| decl.beats(led, tiles[i]).contains(*c)))
                .min_by_key(|d| d.index())
                .unwrap_or_else(least)
        }
        4 => {
            let Decl::PipTrump(p) = decl else {
                panic!("pip-trump only (F1)")
            };
            legal
                .iter()
                .filter(|d| !d.has(p))
                .min_by_key(|d| d.index())
                .unwrap_or_else(least)
        }
        _ => unreachable!("arms are P1..P4"),
    }
}

const ARM_NAMES: [&str; 5] = [
    "",
    "P1 least-tile",
    "P2 greatest-tile",
    "P3 beat-if-able",
    "P4 trump-hoard",
];

// -- (RW-R2): the blocking shared-ground cross-check ------------------------
// Grade-3 S6a idx=0, root 00 (declared): each arm priced at the callback and
// through a materialised map; the two L values asserted exactly equal.

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

fn g3_idx0_kernel() -> Kernel {
    // S6a freezes 22-25 unranking at grade 3, index 0.
    let grade = 3usize;
    let live_idx = unrank_comb(28, 4 * grade, 0);
    let hand_pos: std::collections::BTreeSet<usize> =
        unrank_comb(4 * grade, grade, 0).into_iter().collect();
    let (mut pool, mut hand) = (DominoSet::EMPTY, DominoSet::EMPTY);
    for (pos, di) in live_idx.iter().enumerate() {
        let d = Domino::from_index(*di).expect("domino");
        if hand_pos.contains(&pos) {
            hand.insert(d);
        } else {
            pool.insert(d);
        }
    }
    let hidden = [Seat::S1, Seat::S2, Seat::S3].map(|s| Hidden {
        seat: s,
        capacity: grade,
        voids: ContextSet::EMPTY,
    });
    Kernel::new(
        Decl::PipTrump(Pip::new(0).expect("pip")),
        Seat::S0,
        hand,
        pool,
        hidden,
    )
    .expect("kernel")
}

/// Enumerate every reachable focal record with its legal set (the pooled
/// traversal shape of the freeze-36(f) extraction, values discarded), to
/// materialise a rule into a record-keyed map for (RW-R2).
#[allow(clippy::too_many_arguments)]
fn collect_states(
    decl: Decl,
    focal: Seat,
    worlds: &[[DominoSet; 4]],
    support: &[(u32, u128)],
    leader: Seat,
    tiles: [Domino; 4],
    k: usize,
    obs: &mut Vec<Domino>,
    out: &mut Vec<(Vec<Domino>, DominoSet)>,
) {
    let hand_now = |wi: u32, seat: Seat, obs: &[Domino]| -> DominoSet {
        let mut h = worlds[wi as usize][seat.index()];
        for t in obs {
            h.remove(*t);
        }
        h
    };
    if k == 4 {
        let trick = Trick::new(leader, tiles).expect("distinct");
        let winner = trick.winner(decl);
        if hand_now(support[0].0, focal, obs).is_empty() {
            return;
        }
        collect_states(
            decl,
            focal,
            worlds,
            support,
            winner,
            [Domino::ALL[0]; 4],
            0,
            obs,
            out,
        );
        return;
    }
    let seat = leader.plus(k);
    let led: Option<Context> = (k > 0).then(|| decl.led_context(tiles[0]));
    if seat == focal {
        let hand = hand_now(support[0].0, seat, obs);
        let legal = legal_plays(decl, hand, led);
        out.push((obs.clone(), legal));
        for a in legal.iter() {
            let mut tiles = tiles;
            tiles[k] = a;
            obs.push(a);
            collect_states(decl, focal, worlds, support, leader, tiles, k + 1, obs, out);
            obs.pop();
        }
        return;
    }
    let mut by_tile: BTreeMap<usize, Vec<(u32, u128)>> = BTreeMap::new();
    for &(wi, den) in support {
        let hand = hand_now(wi, seat, obs);
        let legal = legal_plays(decl, hand, led);
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
        collect_states(decl, focal, worlds, &sup, leader, tiles, k + 1, obs, out);
        obs.pop();
    }
}

fn out_dir(name: &str) -> PathBuf {
    let a = PathBuf::from(format!("walt-factory/{name}"));
    if a.exists() {
        return a;
    }
    let b = PathBuf::from(name);
    if b.exists() {
        b
    } else {
        a
    }
}

fn main() {
    let t0 = Instant::now();
    let dir = walt_strat::Direction::trick_diff();
    let mut out = String::new();
    let _ = writeln!(
        out,
        "walt rule-economy probe at the n4 carrier — the map-free rule walk — exploratory tier"
    );
    let _ = writeln!(
        out,
        "rulings: RW-A1..RW-A8, freeze 49 (walt/CENSUS-RULINGS.md 2026-08-14); EC-A1(a)/EC-A11/EC-A13 ported; Corollary E4.1(2)-(3); the filed exact side quoted from separation_n4_2026-08-14.txt (S6h)"
    );
    let _ = writeln!(
        out,
        "regenerate: cargo run --release -p walt-factory --example rule_economy_n4"
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "BY-CONSTRUCTION NOTICES (RW-A2(ii), once): the rule's singleton return, the equality of the two counted halves, distinctness of reached records (the tree-walk property of N4-A15(iii) — which is why no seen set is held and the walk is O(1) in memory), and totality of a total rule are structural, not evidence; legality is asserted per call. The contentful receipts are (RW-R1) L_rule <= Q^H against the filed Q^H, and (RW-R2), blocking, below."
    );
    let _ = writeln!(
        out,
        "ARM CONSISTENCY (RW-A1(iii)): P1/P2 read the legal set and the canonical ascending domino-index order only; P3 reads the tiles so far in the current trick (public record) and the declaration; P4 reads trump membership (declaration and tile). No hidden hand, no bag size, no belief is consulted; the rule argument list is CLOSED at (record, legal), the trick context a derived view of (kernel identity, record)."
    );
    let _ = writeln!(
        out,
        "TYPING (RW-A5/A7): the separation question is live only at the four positive-margin coordinates; at the five negative-margin coordinates the separation column is a RECEIPT of Corollary E4.1(3) — a theorem about what no candidate can do, from the filed Q^H and U alone — while the gap column is a genuine measurement everywhere. A rule failure is candidate-failure, never class-failure (EC-A11); the exact negative is never obtainable from a rule failure. EC-A13's fence: this tests the PRIMAL half only; the run's exact side is quoted, not recomputed."
    );
    let _ = writeln!(
        out,
        "LABELS (RW-A3(iii)): h9 remains NOT PRICED on the exact route (priced = the exact primal pipeline ran) and becomes additionally RULE-EVALUATED; a rule-seeded L is never called a price."
    );

    // (RW-R2), BLOCKING: the shared-ground cross-check at grade-3 idx=0/00.
    {
        let kernel = g3_idx0_kernel();
        let root = kernel
            .viewer_hand()
            .iter()
            .min_by_key(|d| d.index())
            .expect("hand");
        let worlds: Vec<[DominoSet; 4]> = kernel.worlds().map(|w| w.hands()).collect();
        let support: Vec<(u32, u128)> = (0..worlds.len() as u32).map(|i| (i, 1)).collect();
        let mut tiles = [Domino::ALL[0]; 4];
        tiles[0] = root;
        let mut obs = vec![root];
        let mut states: Vec<(Vec<Domino>, DominoSet)> = Vec::new();
        collect_states(
            kernel.decl(),
            kernel.viewer(),
            &worlds,
            &support,
            kernel.viewer(),
            tiles,
            1,
            &mut obs,
            &mut states,
        );
        for arm in 1..=4usize {
            let mut map: BTreeMap<Vec<Domino>, Domino> = BTreeMap::new();
            for (record, legal) in &states {
                map.insert(
                    record.clone(),
                    rule_choice(arm, kernel.decl(), kernel.viewer(), record, *legal),
                );
            }
            let mut b1 = B_WALK;
            let (l_map, _) = walt_strat::policy_value_by_record(
                &kernel,
                kernel.viewer().team(),
                &dir,
                root,
                &map,
                &mut b1,
            )
            .expect("non-binding at grade 3");
            let mut b2 = B_WALK;
            let decl = kernel.decl();
            let viewer = kernel.viewer();
            let (l_rule, _) = walt_strat::policy_value_by_rule(
                &kernel,
                kernel.viewer().team(),
                &dir,
                root,
                &mut |record, legal| rule_choice(arm, decl, viewer, record, legal),
                &mut b2,
            )
            .expect("non-binding at grade 3");
            assert_eq!(
                l_rule.eval(qi(0)),
                l_map.eval(qi(0)),
                "(RW-R2) stop-and-report: the rule walk and the materialised map disagree on shared ground"
            );
        }
        let _ = writeln!(
            out,
            "(RW-R2) BLOCKING — HELD: at the declared shared ground (grade-3 S6a idx=0, root 00) all four arms price identically at the callback and through the materialised map ({} states)",
            states.len()
        );
    }

    // The n4 pass: 9 coordinates x 4 actions x arms P1..P4 (W-parallel over
    // coordinates; every output a pure function; canonical assembly).
    let receipt = {
        let path =
            locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
        parse_file(&path).expect("the receipt parses")
    };
    let texts: Mutex<BTreeMap<usize, String>> = Mutex::new(BTreeMap::new());
    let next = AtomicUsize::new(0);
    let receipt_ref = &receipt;
    let texts_ref = &texts;
    let next_ref = &next;
    let dir_ref = &dir;
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(move || loop {
                let ci = next_ref.fetch_add(1, Ordering::SeqCst);
                if ci >= N4_FILED.len() {
                    break;
                }
                let (hand_id, filed_pip, rows) = &N4_FILED[ci];
                let hand = &receipt_ref.hands[*hand_id];
                let kernel = n4_void_free_kernel(hand);
                let Decl::PipTrump(p) = kernel.decl() else {
                    panic!("pip-trump only")
                };
                assert_eq!(p.value(), *filed_pip, "filed declaration identity");
                assert_eq!(kernel.count(), N4_FIBER, "|X| = 34,650");
                let actions: Vec<Domino> = kernel.viewer_hand().iter().collect();
                let qh: Vec<Q> = rows.iter().map(|(_, _, (n, d), _)| q(*n, *d)).collect();
                let uu: Vec<Q> = rows.iter().map(|(_, _, _, (n, d))| q(*n, *d)).collect();
                for (i, a) in actions.iter().enumerate() {
                    let (hi, lo, _, _) = rows[i];
                    assert_eq!(
                        *a,
                        Domino::new(Pip::new(hi).expect("pip"), Pip::new(lo).expect("pip")),
                        "filed action identity"
                    );
                }
                let vh = qh.iter().copied().max().expect("nonempty");
                let mut text = String::new();
                let _ = writeln!(
                    text,
                    "\n== rule-economy coord h{hand_id} decl=PipTrump({}) hand=[{}] |X|={N4_FIBER} ==",
                    p.value(),
                    actions.iter().map(|d| tile(*d)).collect::<Vec<_>>().join(" ")
                );
                if *hand_id == 9 {
                    let _ = writeln!(
                        text,
                        "  COORDINATE VERDICT (RW-A3(i), filed from the S6h numbers before any rule ran): NOT SEPARATED at either H-optimal action; binding margin -2116837/8870400 < 0; by Corollary E4.1(3) no candidate set whatsoever separates this coordinate — exact negative, from Q^H and U alone, at Tier 1 with the authority gate MET. (This is Corollary E4.1(3)'s exact negative from the two witnesses, NOT a N4-A6(ii) pair verdict, which would require the whole primal pipeline.)"
                    );
                    let _ = writeln!(
                        text,
                        "  arm X: STRUCTURALLY UNAVAILABLE — extraction map exceeds P_max v2 = 192,000,000 at a measured 517,562,322 states (N4-A16(iv)); never a failure and never a gap; the anchor for every gap below is the filed Q^H itself. Labels: NOT PRICED on the exact route; RULE-EVALUATED below."
                    );
                }
                for (i, a) in actions.iter().enumerate() {
                    let margin = qh[i]
                        - uu.iter()
                            .enumerate()
                            .filter(|(j, _)| *j != i)
                            .map(|(_, u)| *u)
                            .max()
                            .expect("competitors");
                    let optimal = qh[i] == vh;
                    #[allow(clippy::needless_range_loop)]
                    for arm in 1..=4usize {
                        let mut budget = B_WALK;
                        let decl = kernel.decl();
                        let viewer = kernel.viewer();
                        let priced = walt_strat::policy_value_by_rule(
                            &kernel,
                            kernel.viewer().team(),
                            dir_ref,
                            *a,
                            &mut |record, legal| rule_choice(arm, decl, viewer, record, legal),
                            &mut budget,
                        );
                        let Some((line, rec)) = priced else {
                            let _ = writeln!(
                                text,
                                "  {} @ {}: DECLARED STOP — walk budget {B_WALK} exhausted; no L, no gap (freeze 44)",
                                ARM_NAMES[arm],
                                tile(*a)
                            );
                            continue;
                        };
                        // count convention via the grade-4 bridge
                        let l_diff = line.eval(qi(0));
                        let l_count = (l_diff + qi(4)) * q(1, 2);
                        let qh_diff = to_diff(qh[i], N4_GRADE);
                        let _ = qh_diff;
                        assert!(
                            l_count <= qh[i],
                            "(RW-R1) stop-and-report at {} @ h{hand_id}/{}: L_rule > filed Q^H — the rule is reading the world, the walk is wrong, or the authorities disagree (SEP-A11(i) style)",
                            ARM_NAMES[arm],
                            tile(*a)
                        );
                        let gap = qh[i] - l_count;
                        let sep_cell = if !optimal {
                            "-".to_owned()
                        } else if margin < qi(0) {
                            format!(
                                "E4.1(3) RECEIPT (margin {margin} < 0: no candidate can separate; a rule failure here is uninformative)"
                            )
                        } else if gap <= margin {
                            format!(
                                "SEPARATED (g <= margin {margin}; R8) — certified by a candidate that cost no map and no extraction; membership only (Theorem E6.4); EC-A13's primal-half fence applies"
                            )
                        } else {
                            format!(
                                "not separated (g > margin {margin}) — candidate-failure, never class-failure (EC-A11)"
                            )
                        };
                        let _ = writeln!(
                            text,
                            "  {} @ {}: L_rule = {}  Q^H = {}  gap g = {}  | {} | reached {} = {} singleton (by-construction), walk-steps {}, residual {}",
                            ARM_NAMES[arm],
                            tile(*a),
                            l_count,
                            qh[i],
                            gap,
                            sep_cell,
                            rec.focal_states,
                            rec.singleton_expansions,
                            B_WALK - budget,
                            budget
                        );
                    }
                }
                texts_ref.lock().expect("texts").insert(ci, text);
            });
        }
    });
    let texts = texts.into_inner().expect("texts");
    for i in 0..N4_FILED.len() {
        out.push_str(texts.get(&i).expect("every coordinate ran"));
    }

    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "total wall-clock (provenance only, never a dividend; W-parallel pure functions, canonical assembly): {} ms",
        t0.elapsed().as_millis()
    );
    let _ = writeln!(out, "run complete: yes");
    let results = out_dir("results").join("rule_economy_n4_2026-08-14.txt");
    std::fs::write(&results, &out).expect("write results");
    print!("{}", &out[..out.len().min(4000)]);
    println!("results: {}", results.display());
}
