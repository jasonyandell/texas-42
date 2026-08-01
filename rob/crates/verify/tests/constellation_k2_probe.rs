//! Exploratory probe (constellation layer — NOT receipt rows unless
//! promoted by amendment): does the exact suffix minimax value at k=2
//! (two tiles per hand) factor through the constellation, pooling all
//! nine declarations?
//!
//! This is the k>=2 continuation of `constellation_k1_census.rs`, and the
//! first evidence on the load-bearing cross-declaration question where
//! CHOICE enters (leaders choose among two tiles, followers may hold two
//! followers): at k=1 outcome-constancy per relational key is expected by
//! construction; at k=2 it is not.
//!
//! Method: over fixed 10-tile sub-universes, enumerate EVERY k=2 suffix
//! position — all C(10,8) live sets, all 2,520 hand assignments, leader
//! fixed at seat 0, all 9 declarations — abstract each to its
//! constellation key (holder offsets, count labels, FOLLOW and pairwise
//! trick-key ORD relations over all eight live-tile lead contexts; no pip
//! names, no declaration name; canonical over within-hand tile swaps),
//! group positions by key with declarations pooled, and solve exact
//! both-teams-optimal minimax for every member of every multi-member
//! group. A census over sub-universes, not a sample of the full space:
//! within the sub-universe nothing is dropped.
//!
//! Any within-group value divergence falsifies the factorization
//! conjecture (idea page §5) as keyed and fails the test loudly with the
//! witness pair printed.

use std::collections::HashMap;

/// Group member locator: (universe, set index, assignment index, declaration index).
type Member = (usize, usize, u32, u32);

use rob_core::Seat;
use rob_core::{
    algebra_for, all_ids, domino_from_id, DeclarationAlgebra, DominoId, DominoSet, TrickKey,
    GAME_DECLARATIONS,
};
use rob_player::rollout::RolloutPosition;

fn count_points(d: DominoId) -> u8 {
    domino_from_id(d).count_points()
}

/// Exact minimax margin for team 0 (seats 0 and 2) over the suffix,
/// every actor optimizing for its own partnership — same shape as the
/// retrograde probe's solver, plain DFS over `legal`/`apply`.
fn minimax(algebra: &DeclarationAlgebra, pos: &RolloutPosition) -> i64 {
    if pos.hands.iter().all(DominoSet::is_empty) {
        assert!(pos.trick.is_empty(), "all tricks resolve");
        return i64::from(pos.points[0]) - i64::from(pos.points[1]);
    }
    let actor = pos.leader.offset(pos.trick.len() as u8);
    let maximizing = actor.team().index() == 0;
    let mut best: Option<i64> = None;
    for action in pos.legal(algebra) {
        let mut next = pos.clone();
        next.apply(algebra, action);
        let value = minimax(algebra, &next);
        best = Some(match best {
            None => value,
            Some(b) if maximizing => b.max(value),
            Some(b) => b.min(value),
        });
    }
    best.expect("nonempty legal set")
}

/// Constellation key of a k=2 arrangement: tiles indexed 0..8 as
/// [hand0 pair, hand1 pair, hand2 pair, hand3 pair] (seat offsets from
/// the leader), serialized as count labels, then FOLLOW and slough
/// per live lead context, then the full pairwise trick-key comparison
/// matrix per context. No pip names, no declaration name. Canonical =
/// minimum over the 16 within-hand swaps (holders are load-bearing;
/// hands are not interchangeable at k=2).
fn constellation_key(algebra: &DeclarationAlgebra, t: &[DominoId; 8]) -> Vec<u8> {
    let mut best: Option<Vec<u8>> = None;
    for mask in 0u8..16 {
        let mut u = *t;
        for hand in 0..4 {
            if mask & (1 << hand) != 0 {
                u.swap(2 * hand, 2 * hand + 1);
            }
        }
        let mut key = Vec::with_capacity(8 + 8 * (16 + 64));
        for &d in &u {
            key.push(count_points(d));
        }
        for &lead in &u {
            let q = algebra.led_suit(lead);
            for &m in &u {
                key.push(u8::from(algebra.follows(m, q)));
                key.push(u8::from(algebra.trick_key(m, q) == TrickKey::Slough));
            }
            for &m in &u {
                for &n in &u {
                    key.push(
                        match algebra.trick_key(m, q).cmp(&algebra.trick_key(n, q)) {
                            std::cmp::Ordering::Less => 0,
                            std::cmp::Ordering::Equal => 1,
                            std::cmp::Ordering::Greater => 2,
                        },
                    );
                }
            }
        }
        if best.as_ref().is_none_or(|b| &key < b) {
            best = Some(key);
        }
    }
    best.expect("nonempty swap orbit")
}

fn position(t: &[DominoId; 8]) -> RolloutPosition {
    let mut hands = [DominoSet::empty(); 4];
    for (i, &d) in t.iter().enumerate() {
        hands[i / 2].insert(d);
    }
    RolloutPosition {
        hands,
        leader: Seat::ALL[0],
        trick: Vec::new(),
        points: [0, 0],
    }
}

/// All ways to split 8 tiles into 4 ordered hands of 2 (8!/2^4 = 2,520),
/// emitted as arrangements [h0a,h0b,h1a,h1b,h2a,h2b,h3a,h3b].
fn assignments(set: &[DominoId; 8]) -> Vec<[DominoId; 8]> {
    let mut out = Vec::with_capacity(2520);
    let mut current = [set[0]; 8];
    let mut used = [false; 8];
    fn rec(
        set: &[DominoId; 8],
        used: &mut [bool; 8],
        current: &mut [DominoId; 8],
        slot: usize,
        out: &mut Vec<[DominoId; 8]>,
    ) {
        if slot == 8 {
            out.push(*current);
            return;
        }
        // Within a hand, force ascending source index for the second tile
        // to halve the orbit (the key canonicalizes swaps anyway).
        let start = if slot % 2 == 1 { 1 } else { 0 };
        let _ = start;
        for i in 0..8 {
            if used[i] {
                continue;
            }
            if slot % 2 == 1 {
                // second tile of a hand: require a higher index than the first
                let first = current[slot - 1];
                let first_idx = set.iter().position(|&x| x == first).expect("in set");
                if i < first_idx {
                    continue;
                }
            }
            used[i] = true;
            current[slot] = set[i];
            rec(set, used, current, slot + 1, out);
            used[i] = false;
        }
    }
    rec(set, &mut used, &mut current, 0, &mut out);
    out
}

struct Probe {
    positions: u64,
    groups: HashMap<Vec<u8>, Vec<Member>>,
}

fn sweep(universes: &[Vec<DominoId>]) -> Probe {
    let algebras: Vec<DeclarationAlgebra> =
        GAME_DECLARATIONS.iter().map(|&d| algebra_for(d)).collect();
    let mut probe = Probe {
        positions: 0,
        groups: HashMap::new(),
    };
    for (ui, ids) in universes.iter().enumerate() {
        let n = ids.len();
        let mut set_index = 0usize;
        // all C(n,8) 8-subsets
        let mut idx = [0usize; 8];
        for a in 0..n {
            for b in (a + 1)..n {
                for c in (b + 1)..n {
                    for d in (c + 1)..n {
                        for e in (d + 1)..n {
                            for f in (e + 1)..n {
                                for g in (f + 1)..n {
                                    for h in (g + 1)..n {
                                        idx = [a, b, c, d, e, f, g, h];
                                        let set: [DominoId; 8] =
                                            core::array::from_fn(|x| ids[idx[x]]);
                                        for (ai, t) in assignments(&set).iter().enumerate() {
                                            for (di, algebra) in algebras.iter().enumerate() {
                                                probe.positions += 1;
                                                let key = constellation_key(algebra, t);
                                                probe
                                                    .groups
                                                    .entry(key)
                                                    .or_default()
                                                    .push((ui, set_index, ai as u32, di as u32));
                                            }
                                        }
                                        set_index += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let _ = idx;
    }
    probe
}

/// Re-materialize a group member and solve it exactly.
fn solve_member(universes: &[Vec<DominoId>], member: (usize, usize, u32, u32)) -> i64 {
    let (ui, set_index, ai, di) = member;
    let ids = &universes[ui];
    let n = ids.len();
    let mut count = 0usize;
    for a in 0..n {
        for b in (a + 1)..n {
            for c in (b + 1)..n {
                for d in (c + 1)..n {
                    for e in (d + 1)..n {
                        for f in (e + 1)..n {
                            for g in (f + 1)..n {
                                for h in (g + 1)..n {
                                    if count == set_index {
                                        let set: [DominoId; 8] = [
                                            ids[a], ids[b], ids[c], ids[d], ids[e], ids[f], ids[g],
                                            ids[h],
                                        ];
                                        let t = assignments(&set)[ai as usize];
                                        let algebra = algebra_for(GAME_DECLARATIONS[di as usize]);
                                        return minimax(&algebra, &position(&t));
                                    }
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    unreachable!("set index in range");
}

fn run(universes: &[Vec<DominoId>], label: &str) {
    let probe = sweep(universes);
    let multi: Vec<(&Vec<u8>, &Vec<Member>)> =
        probe.groups.iter().filter(|(_, v)| v.len() >= 2).collect();
    let mut checks = 0u64;
    let mut divergences = 0u64;
    let mut cross_declaration_groups = 0u64;
    let mut largest = 0usize;
    for (_, members) in &multi {
        largest = largest.max(members.len());
        let decls: std::collections::HashSet<u32> =
            members.iter().map(|&(_, _, _, di)| di).collect();
        if decls.len() >= 2 {
            cross_declaration_groups += 1;
        }
        let base = solve_member(universes, members[0]);
        for &m in members.iter().skip(1) {
            checks += 1;
            let v = solve_member(universes, m);
            if v != base {
                divergences += 1;
                println!(
                    "DIVERGENCE {label}: members {:?} vs {:?} values {base} vs {v}",
                    members[0], m
                );
            }
        }
    }
    println!(
        "{label}: positions {} | keys {} multi-member groups {} (largest {}) \
         cross-declaration groups {} | value checks {} DIVERGENCES {}",
        probe.positions,
        probe.groups.len(),
        multi.len(),
        largest,
        cross_declaration_groups,
        checks,
        divergences
    );
    assert_eq!(
        divergences, 0,
        "suffix minimax must be constant per constellation"
    );
}

/// Sub-universes chosen for structure: counters, doubles, shared suits,
/// and a disjoint-flavored second universe. Tiles named as (high, low).
fn universe(pairs: &[(u8, u8)]) -> Vec<DominoId> {
    let wanted: Vec<(u8, u8)> = pairs.to_vec();
    all_ids()
        .filter(|&id| {
            let d = domino_from_id(id);
            wanted.contains(&(d.high().value(), d.low().value()))
        })
        .collect()
}

/// Fast CI smoke: one 9-tile sub-universe (C(9,8) = 9 sets, 204,120
/// positions), full grouping, exact solves on every multi-member group.
#[test]
fn constellation_k2_smoke() {
    let u = universe(&[
        (6, 6),
        (6, 4),
        (5, 5),
        (5, 0),
        (4, 1),
        (3, 2),
        (2, 2),
        (2, 1),
        (1, 0),
    ]);
    assert_eq!(u.len(), 9);
    run(&[u], "SMOKE k=2 (9-tile sub-universe)");
}

/// Full probe: two 10-tile sub-universes (2 × C(10,8) × 2,520 × 9 =
/// 2,041,200 positions), pooled grouping across both. Run:
/// `cargo test --release --test constellation_k2_probe constellation_k2_full -- --ignored --nocapture`
#[test]
#[ignore]
fn constellation_k2_full() {
    let u1 = universe(&[
        (6, 6),
        (6, 5),
        (6, 4),
        (5, 5),
        (5, 4),
        (5, 0),
        (4, 4),
        (4, 1),
        (3, 2),
        (1, 0),
    ]);
    let u2 = universe(&[
        (6, 3),
        (6, 2),
        (6, 0),
        (5, 3),
        (5, 2),
        (4, 3),
        (4, 2),
        (3, 3),
        (2, 1),
        (0, 0),
    ]);
    assert_eq!(u1.len(), 10);
    assert_eq!(u2.len(), 10);
    run(&[u1, u2], "FULL k=2 (two 10-tile sub-universes)");
}
