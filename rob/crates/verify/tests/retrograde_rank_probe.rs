//! Exploratory probe (retrograde rank quotient — "the pips don't matter,
//! the relationships do"; NOT receipt rows unless promoted by amendment):
//! does the exact minimax value of an endgame suffix depend only on the
//! *rank structure* of the live tiles — per-context follow pattern,
//! relative trick-key order among the living, and count carried — or does
//! pip identity leak in?
//!
//! The test is rank-preserving tile substitution. At a corpus endgame
//! position (boundaries 4/5/6 = 3/2/1 tiles per hand), take the live set
//! L (viewer hand ∪ unseen pool), pick a live tile d and an already-played
//! tile e ∉ L, and ask whether the relabeling d→e is an isomorphism of the
//! live relational structure: equal count points, and for every live lead
//! context, identical follow pattern, identical slough pattern, and
//! identical pairwise trick-key order over L vs L−d+e. If so, substitute
//! (same holder, same leader, same declaration) and compare exact minimax
//! values — a plain both-teams-optimal DFS over `RolloutPosition`, σ
//! nowhere in the loop.
//!
//! This is a *different* claim from the rung-2 tile-exchange probe
//! (`strategic_exchange_probe.rs`), which swapped tiles across hidden
//! seats within one world and replayed a fixed plan against σ — and was
//! falsified. Here nothing is swapped inside the deal and no plan or σ is
//! involved: the question is whether the suffix *game* factors through
//! the live rank structure (dead-cut / boss-among-the-living, x:003 and
//! idea page §5), with the rung-2 static no-go evaded because identity is
//! only required relative to the live suffix, not over all 28 tiles.
//!
//! Also measured: substitution scarcity — how many live tiles admit any
//! rank-preserving substitute at all (the "pinned to its pips" fraction),
//! per boundary. Scarcity is the first number behind the retrograde
//! reachability hunch: abstract rank states with few or no concrete
//! realizations.
//!
//! Findings, frozen 2026-07-31 (exploratory; quoted in
//! wiki/idea-retrograde-rank.md):
//! **zero value divergences.** Sweep (boundaries 6/5/4, 108 corpus
//! positions each, worlds capped 6/90/16): 6,809 solved suffix worlds,
//! **32,886 substitution checks — minimax value equal in every one**
//! (b6: 4,258; b5: 22,884; b4: 5,744). Scarcity: the substitutable
//! fraction of live tiles *falls* as hands deepen — boundary 6: 258/432
//! (60%), 3/108 positions fully pinned; boundary 5: 243/864 (28%),
//! 14/108 pinned; boundary 4: 201/1296 (16%), 20/108 pinned. More of the
//! live rank structure is load-bearing earlier — the first measured
//! surface of the retrograde reachability hunch. Whole sweep: ~2 s.

use std::cmp::Ordering;

use rob_core::{
    algebra_for, all_ids, derive_rule_cells, domino_from_id, DeclarationAlgebra, DominoId,
    DominoSet, MechanicalState, Play, RemainderWorld, TrickKey,
};
use rob_player::rollout::RolloutPosition;
use rob_verify::p2::boundary_position;

fn name(d: DominoId) -> String {
    let dom = domino_from_id(d);
    format!("{}-{}", dom.high().value(), dom.low().value())
}

/// Exact minimax margin for team 0 (seats 0 and 2) over the suffix:
/// every actor plays optimally for its own partnership, plain DFS over
/// the certified `legal`/`apply` primitives.
fn minimax(algebra: &DeclarationAlgebra, pos: &RolloutPosition, expected_total: u32) -> i64 {
    if pos.hands.iter().all(DominoSet::is_empty) {
        assert!(pos.trick.is_empty(), "all tricks resolve");
        assert_eq!(
            pos.points[0] + pos.points[1],
            expected_total,
            "suffix conserves its trick + count total"
        );
        return i64::from(pos.points[0]) - i64::from(pos.points[1]);
    }
    let actor = pos.leader.offset(pos.trick.len() as u8);
    let maximizing = actor.team().index() == 0;
    let mut best: Option<i64> = None;
    for action in pos.legal(algebra) {
        let mut next = pos.clone();
        next.apply(algebra, action);
        let value = minimax(algebra, &next, expected_total);
        best = Some(match best {
            None => value,
            Some(b) if maximizing => b.max(value),
            Some(b) => b.min(value),
        });
    }
    best.expect("nonempty legal set")
}

/// Total points the suffix will settle: one per remaining trick plus the
/// count carried by the live tiles.
fn suffix_total(live: &[DominoId], tiles_per_hand: usize) -> u32 {
    let count: u32 = live
        .iter()
        .map(|&d| u32::from(domino_from_id(d).count_points()))
        .sum();
    tiles_per_hand as u32 + count
}

/// Is d→e (d live, e unused) an isomorphism of the live rank structure?
/// Checks, for every live lead context on both sides of the relabeling:
/// identical follow pattern, identical slough pattern, and identical
/// pairwise trick-key order over the live set.
fn rank_preserving(
    algebra: &DeclarationAlgebra,
    live: &[DominoId],
    d: DominoId,
    e: DominoId,
) -> bool {
    if domino_from_id(d).count_points() != domino_from_id(e).count_points() {
        return false;
    }
    let phi = |x: DominoId| if x == d { e } else { x };
    for &lead in live {
        let q1 = algebra.led_suit(lead);
        let q2 = algebra.led_suit(phi(lead));
        for &m in live {
            if algebra.follows(m, q1) != algebra.follows(phi(m), q2) {
                return false;
            }
            let slough1 = algebra.trick_key(m, q1) == TrickKey::Slough;
            let slough2 = algebra.trick_key(phi(m), q2) == TrickKey::Slough;
            if slough1 != slough2 {
                return false;
            }
        }
        for &m in live {
            for &n in live {
                let order1: Ordering = algebra.trick_key(m, q1).cmp(&algebra.trick_key(n, q1));
                let order2 = algebra
                    .trick_key(phi(m), q2)
                    .cmp(&algebra.trick_key(phi(n), q2));
                if order1 != order2 {
                    return false;
                }
            }
        }
    }
    true
}

/// Complete-information suffix position of `state` + `world`, points
/// zeroed so the value is the pure suffix margin.
fn suffix_position(state: &MechanicalState, world: &RemainderWorld) -> RolloutPosition {
    let mut hands = [DominoSet::empty(); 4];
    hands[state.viewer().index()] = *state.own_remaining_hand();
    for (i, &seat) in state.hidden_seats().iter().enumerate() {
        hands[seat.index()] = world.hidden_hands[i];
    }
    RolloutPosition {
        hands,
        leader: state.leader(),
        trick: state.current_trick().to_vec(),
        points: [0, 0],
    }
}

#[derive(Default)]
struct Tally {
    positions: u64,
    worlds: u64,
    live_tiles: u64,
    tiles_with_substitute: u64,
    fully_pinned_positions: u64,
    checks: u64,
    equal: u64,
    differ: u64,
}

impl Tally {
    fn add(&mut self, other: &Tally) {
        self.positions += other.positions;
        self.worlds += other.worlds;
        self.live_tiles += other.live_tiles;
        self.tiles_with_substitute += other.tiles_with_substitute;
        self.fully_pinned_positions += other.fully_pinned_positions;
        self.checks += other.checks;
        self.equal += other.equal;
        self.differ += other.differ;
    }

    fn print(&self, label: &str) {
        println!(
            "{label}: positions {} worlds {} | live tiles {} with-substitute {} \
             fully-pinned positions {} | checks {} equal {} DIFFER {}",
            self.positions,
            self.worlds,
            self.live_tiles,
            self.tiles_with_substitute,
            self.fully_pinned_positions,
            self.checks,
            self.equal,
            self.differ
        );
    }
}

fn probe_position(boundary: usize, index: u64, world_cap: usize) -> Tally {
    let state = boundary_position(index, boundary);
    assert!(
        state.current_trick().is_empty(),
        "boundary positions sit at trick boundaries"
    );
    let algebra = algebra_for(state.contract().declaration());
    let cells = derive_rule_cells(&state);

    // The live set (viewer hand ∪ unseen pool) is constant across the
    // fiber, so substitution candidates are computed once per position.
    let mut live: Vec<DominoId> = cells.unseen_pool().iter().collect();
    live.extend(state.own_remaining_hand().iter());
    let unused: Vec<DominoId> = all_ids().filter(|d| !live.contains(d)).collect();
    let tiles_per_hand = 7 - boundary;
    let expected_total = suffix_total(&live, tiles_per_hand);

    let mut substitutes: Vec<(DominoId, DominoId)> = Vec::new();
    let mut tiles_with_substitute = 0u64;
    for &d in &live {
        let before = substitutes.len();
        for &e in &unused {
            if rank_preserving(&algebra, &live, d, e) {
                substitutes.push((d, e));
            }
        }
        if substitutes.len() > before {
            tiles_with_substitute += 1;
        }
    }

    let mut tally = Tally {
        positions: 1,
        live_tiles: live.len() as u64,
        tiles_with_substitute,
        fully_pinned_positions: u64::from(tiles_with_substitute == 0),
        ..Tally::default()
    };

    let worlds = cells.fiber_worlds();
    for world in worlds.iter().take(world_cap) {
        tally.worlds += 1;
        let pos = suffix_position(&state, world);
        if substitutes.is_empty() {
            continue;
        }
        let base_value = minimax(&algebra, &pos, expected_total);
        for &(d, e) in &substitutes {
            let holder = pos
                .hands
                .iter()
                .position(|h| h.contains(d))
                .expect("live tile is held");
            let mut swapped = pos.clone();
            swapped.hands[holder].remove(d);
            swapped.hands[holder].insert(e);
            let swapped_value = minimax(&algebra, &swapped, expected_total);
            tally.checks += 1;
            if swapped_value == base_value {
                tally.equal += 1;
            } else {
                tally.differ += 1;
                println!(
                    "DIVERGENCE boundary {boundary} index {index}: {} -> {} \
                     value {base_value} -> {swapped_value}",
                    name(d),
                    name(e)
                );
            }
        }
    }
    tally
}

/// Boundary-6 self-check: with one tile per hand the suffix is a single
/// forced trick, so minimax must equal direct trick resolution.
fn forced_trick_value(algebra: &DeclarationAlgebra, pos: &RolloutPosition) -> i64 {
    let plays: Vec<Play> = (0..4u8)
        .map(|k| {
            let actor = pos.leader.offset(k);
            let domino = pos.hands[actor.index()]
                .iter()
                .next()
                .expect("one tile per hand");
            Play { actor, domino }
        })
        .collect();
    let result = algebra
        .resolve_trick(&plays)
        .expect("forced trick resolves");
    let margin = i64::from(result.points);
    if result.winner.team().index() == 0 {
        margin
    } else {
        -margin
    }
}

/// Fast smoke pass (runs in CI): the boundary-6 forced-trick cross-check
/// over every corpus hand and fiber world, plus a small boundary-5 slice
/// of the substitution tally.
#[test]
fn retrograde_rank_smoke() {
    for index in 0..108u64 {
        let state = boundary_position(index, 6);
        let algebra = algebra_for(state.contract().declaration());
        let cells = derive_rule_cells(&state);
        let mut live: Vec<DominoId> = cells.unseen_pool().iter().collect();
        live.extend(state.own_remaining_hand().iter());
        let expected_total = suffix_total(&live, 1);
        for world in &cells.fiber_worlds() {
            let pos = suffix_position(&state, world);
            assert_eq!(
                minimax(&algebra, &pos, expected_total),
                forced_trick_value(&algebra, &pos),
                "one-tile minimax is the forced trick (hand {index})"
            );
        }
    }
    let mut agg = Tally::default();
    for index in 0..12u64 {
        agg.add(&probe_position(5, index, 6));
    }
    agg.print("SMOKE boundary 5 (12 positions)");
}

/// Full sweep: boundaries 6/5/4 over all 108 corpus hands, worlds capped
/// 6/12/4. Run:
/// `cargo test --release --test retrograde_rank_probe retrograde_rank_sweep -- --ignored --nocapture`
#[test]
#[ignore]
fn retrograde_rank_sweep() {
    let mut total = Tally::default();
    for (boundary, cap) in [(6usize, 6usize), (5, 90), (4, 16)] {
        let start = std::time::Instant::now();
        let mut agg = Tally::default();
        for index in 0..108u64 {
            agg.add(&probe_position(boundary, index, cap));
        }
        agg.print(&format!(
            "boundary {boundary} ({} tiles/hand, {:?})",
            7 - boundary,
            start.elapsed()
        ));
        total.add(&agg);
    }
    total.print("TOTAL");
}
