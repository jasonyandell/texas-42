//! Exploratory instrument (constellation layer — NOT receipt rows unless
//! promoted by amendment): exhaustive k=1 census through the declaration
//! algebra.
//!
//! Every last-trick position is enumerated — all C(28,4) = 20,475 live
//! 4-sets, all 12 role arrangements (which tile is led, which is the
//! leader's partner's; the two opponent tiles are role-identical and
//! canonicalized), all 9 declarations — and each is abstracted to its
//! constellation: the relational structure among the four living tiles
//! (per-lead-context follow pattern, slough pattern, pairwise trick-key
//! comparison), plus count points and roles, with no suit names and no
//! declaration name in the key. Two keys are computed per position:
//!
//! - **fine** — relations under all four hypothetical lead contexts (the
//!   dynamics-candidate quotient of wiki/idea-retrograde-rank.md §5);
//! - **coarse** — relations under the actual led context only (the
//!   value-relevant quotient at k=1).
//!
//! Checked exhaustively, pooling across all 9 declarations: the forced
//! trick outcome (does the leader's team win; trick award) is a function
//! of the coarse key, and a fortiori of the fine key — zero collisions
//! tolerated. This is the cross-declaration check of the idea page (§6.2)
//! done as a census at k=1, not a sample: trump never appears in the key,
//! so outcome-constancy per key IS the pooling license at this depth.
//!
//! Also measured: the rule-free carrier ladder (edge skeletons of
//! K7-with-loops up to pip relabeling — role-free and role-decorated),
//! and mu = how many fine constellations each role-decorated carrier
//! splits into once the algebra's precedence relations land.
//!
//! Reachability is deliberately ignored: every 4-set is treated as a
//! last-trick position whether or not it extends back to a legal deal —
//! this census is the enumeration side of the micro-CSP, not the
//! backward-extension question.

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use rob_core::{
    algebra_for, all_ids, domino_from_id, DeclarationAlgebra, DominoId, Play, Seat, TrickKey,
    GAME_DECLARATIONS,
};

/// Carrier key: four pip-edges (hi, lo, count, role), canonicalized.
type CarrierKey = [(u8, u8, u8, u8); 4];

fn count_points(d: DominoId) -> u8 {
    domino_from_id(d).count_points()
}

fn pip_pair(d: DominoId) -> (u8, u8) {
    let dom = domino_from_id(d);
    (dom.high().value(), dom.low().value())
}

/// Relational key over an arrangement (index 0 = led tile, 1 = partner's,
/// 2..3 = opponents'): per-tile count points, then for each requested lead
/// context the follow/slough pattern and the full pairwise trick-key
/// comparison matrix. No pip names, no declaration name.
fn relation_key(algebra: &DeclarationAlgebra, t: &[DominoId; 4], contexts: &[usize]) -> Vec<u8> {
    let mut key = Vec::with_capacity(4 + contexts.len() * 24);
    for &d in t {
        key.push(count_points(d));
    }
    for &lead in contexts {
        let q = algebra.led_suit(t[lead]);
        for &m in t {
            key.push(u8::from(algebra.follows(m, q)));
            key.push(u8::from(algebra.trick_key(m, q) == TrickKey::Slough));
        }
        for &m in t {
            for &n in t {
                key.push(
                    match algebra.trick_key(m, q).cmp(&algebra.trick_key(n, q)) {
                        Ordering::Less => 0,
                        Ordering::Equal => 1,
                        Ordering::Greater => 2,
                    },
                );
            }
        }
    }
    key
}

/// Canonical relational key: minimum over the opponent swap, the only
/// role-preserving symmetry of the arrangement.
fn canon_relation_key(algebra: &DeclarationAlgebra, t: &[DominoId; 4], fine: bool) -> Vec<u8> {
    let swapped = [t[0], t[1], t[3], t[2]];
    let contexts: &[usize] = if fine { &[0, 1, 2, 3] } else { &[0] };
    let a = relation_key(algebra, t, contexts);
    let b = relation_key(algebra, &swapped, contexts);
    a.min(b)
}

/// All permutations of 0..n as index vectors, n <= 7.
fn permutation_table(n: usize) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    let mut items: Vec<u8> = (0..n as u8).collect();
    fn heap(items: &mut Vec<u8>, k: usize, out: &mut Vec<Vec<u8>>) {
        if k <= 1 {
            out.push(items.clone());
            return;
        }
        for i in 0..k {
            heap(items, k - 1, out);
            if k.is_multiple_of(2) {
                items.swap(i, k - 1);
            } else {
                items.swap(0, k - 1);
            }
        }
    }
    heap(&mut items, n, &mut out);
    out
}

/// Carrier key: the four tiles as pip-edges of K7-with-loops, labeled with
/// count points and a role tag, canonical over pip (color) relabeling —
/// the rule-free skeleton beneath the constellation. Role tags: 0 = led,
/// 1 = partner's, 2 = opponent's (both opponents tagged 2; the sorted edge
/// multiset absorbs their swap). Pass equal tags to erase roles.
fn carrier_key(t: &[DominoId; 4], roles: [u8; 4], perms: &[Vec<Vec<u8>>]) -> CarrierKey {
    let raw: Vec<(u8, u8, u8)> = t
        .iter()
        .zip(roles)
        .map(|(&d, r)| {
            let (h, l) = pip_pair(d);
            (h, l, r)
        })
        .collect();
    let mut present: Vec<u8> = raw.iter().flat_map(|&(h, l, _)| [h, l]).collect();
    present.sort_unstable();
    present.dedup();
    let n = present.len();
    let mut best: Option<CarrierKey> = None;
    for perm in &perms[n] {
        let map = |p: u8| perm[present.binary_search(&p).expect("pip present")];
        let mut edges = [(0u8, 0u8, 0u8, 0u8); 4];
        for (slot, &(h, l, r)) in edges.iter_mut().zip(&raw) {
            let (a, b) = (map(h), map(l));
            *slot = (a.max(b), a.min(b), count_points_of_pair(h, l), r);
        }
        edges.sort_unstable();
        if best.is_none_or(|b| edges < b) {
            best = Some(edges);
        }
    }
    best.expect("nonempty permutation table")
}

/// Count points from a pip pair (tile-intrinsic; avoids re-deriving ids).
fn count_points_of_pair(h: u8, l: u8) -> u8 {
    match (h, l) {
        (5, 5) | (6, 4) => 10,
        (5, 0) | (4, 1) | (3, 2) => 5,
        _ => 0,
    }
}

/// Forced-trick outcome of an arrangement: (leader's team wins, award).
/// Seats clockwise from the leader; the partner's tile sits at seat 2.
fn outcome(algebra: &DeclarationAlgebra, t: &[DominoId; 4]) -> (u8, u8) {
    let plays = [
        Play {
            actor: Seat::ALL[0],
            domino: t[0],
        },
        Play {
            actor: Seat::ALL[1],
            domino: t[2],
        },
        Play {
            actor: Seat::ALL[2],
            domino: t[1],
        },
        Play {
            actor: Seat::ALL[3],
            domino: t[3],
        },
    ];
    let result = algebra
        .resolve_trick(&plays)
        .expect("four distinct forced plays resolve");
    (
        u8::from(result.winner.index().is_multiple_of(2)),
        result.points,
    )
}

#[derive(Default)]
struct Census {
    sets: u64,
    positions: u64,
    fine_outcome: HashMap<Vec<u8>, (u8, u8)>,
    coarse_outcome: HashMap<Vec<u8>, (u8, u8)>,
    fine_collisions: u64,
    coarse_collisions: u64,
    carriers_plain: HashSet<CarrierKey>,
    carrier_fine: HashMap<CarrierKey, HashSet<Vec<u8>>>,
    outcomes: HashSet<(u8, u8)>,
}

fn run_census(ids: &[DominoId]) -> Census {
    let algebras: Vec<DeclarationAlgebra> =
        GAME_DECLARATIONS.iter().map(|&d| algebra_for(d)).collect();
    let perms: Vec<Vec<Vec<u8>>> = (0..=7).map(permutation_table).collect();
    let mut census = Census::default();
    let n = ids.len();
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                for l in (k + 1)..n {
                    let set = [ids[i], ids[j], ids[k], ids[l]];
                    census.sets += 1;
                    census
                        .carriers_plain
                        .insert(carrier_key(&set, [9, 9, 9, 9], &perms));
                    for lead in 0..4 {
                        let rest: Vec<DominoId> =
                            (0..4).filter(|&x| x != lead).map(|x| set[x]).collect();
                        for p in 0..3 {
                            let opp: Vec<DominoId> =
                                (0..3).filter(|&x| x != p).map(|x| rest[x]).collect();
                            let t = [set[lead], rest[p], opp[0], opp[1]];
                            let carrier = carrier_key(&t, [0, 1, 2, 2], &perms);
                            for algebra in &algebras {
                                census.positions += 1;
                                let out = outcome(algebra, &t);
                                census.outcomes.insert(out);
                                let fine = canon_relation_key(algebra, &t, true);
                                let coarse = canon_relation_key(algebra, &t, false);
                                census
                                    .carrier_fine
                                    .entry(carrier)
                                    .or_default()
                                    .insert(fine.clone());
                                match census.fine_outcome.entry(fine) {
                                    std::collections::hash_map::Entry::Occupied(e) => {
                                        if *e.get() != out {
                                            census.fine_collisions += 1;
                                        }
                                    }
                                    std::collections::hash_map::Entry::Vacant(e) => {
                                        e.insert(out);
                                    }
                                }
                                match census.coarse_outcome.entry(coarse) {
                                    std::collections::hash_map::Entry::Occupied(e) => {
                                        if *e.get() != out {
                                            census.coarse_collisions += 1;
                                        }
                                    }
                                    std::collections::hash_map::Entry::Vacant(e) => {
                                        e.insert(out);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    census
}

fn print_census(label: &str, census: &Census) {
    println!(
        "{label}: sets {} positions {} (sets x 12 roles x 9 declarations)",
        census.sets, census.positions
    );
    println!(
        "  carriers: role-free {} role-decorated {}",
        census.carriers_plain.len(),
        census.carrier_fine.len()
    );
    println!(
        "  constellations: fine {} coarse {} | distinct outcomes {}",
        census.fine_outcome.len(),
        census.coarse_outcome.len(),
        census.outcomes.len()
    );
    println!(
        "  exactness: fine collisions {} coarse collisions {}",
        census.fine_collisions, census.coarse_collisions
    );
    let mut mu: Vec<usize> = census.carrier_fine.values().map(HashSet::len).collect();
    mu.sort_unstable();
    let total_fine_slots: usize = mu.iter().sum();
    let mut histogram: HashMap<usize, usize> = HashMap::new();
    for &m in &mu {
        *histogram.entry(m).or_default() += 1;
    }
    let mut hist: Vec<(usize, usize)> = histogram.into_iter().collect();
    hist.sort_unstable();
    println!(
        "  mu (fine constellations per role-decorated carrier): min {} max {} \
         median {} sum {}",
        mu.first().copied().unwrap_or(0),
        mu.last().copied().unwrap_or(0),
        mu.get(mu.len() / 2).copied().unwrap_or(0),
        total_fine_slots
    );
    println!("  mu histogram (mu: carriers): {hist:?}");
}

/// Fast CI smoke: the census over the first 12 dominoes only. The claims
/// checked are the load-bearing ones — zero outcome collisions per fine
/// key and per coarse key, pooled across all nine declarations.
#[test]
fn constellation_k1_smoke() {
    let ids: Vec<DominoId> = all_ids().take(12).collect();
    let census = run_census(&ids);
    assert_eq!(census.sets, 495);
    assert_eq!(census.positions, 495 * 12 * 9);
    assert_eq!(census.fine_collisions, 0, "fine key must determine outcome");
    assert_eq!(
        census.coarse_collisions, 0,
        "coarse key must determine outcome"
    );
    print_census("SMOKE (12-domino sub-universe)", &census);
}

/// Full census: all 20,475 live 4-sets. Run:
/// `cargo test --release --test constellation_k1_census constellation_k1_full -- --ignored --nocapture`
#[test]
#[ignore]
fn constellation_k1_full() {
    let ids: Vec<DominoId> = all_ids().collect();
    let census = run_census(&ids);
    assert_eq!(census.sets, 20_475);
    assert_eq!(census.positions, 20_475 * 12 * 9);
    assert_eq!(census.fine_collisions, 0, "fine key must determine outcome");
    assert_eq!(
        census.coarse_collisions, 0,
        "coarse key must determine outcome"
    );
    print_census("FULL k=1 census", &census);
}
