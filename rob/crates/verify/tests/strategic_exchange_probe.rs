//! Exploratory probe (wiki/idea-hierarchical-fibers rung 2 — the symmetry
//! quotient; NOT receipt rows unless promoted by amendment): can two
//! worlds that differ only by exchanging "strategically identical" tiles
//! across hidden seats be game-equivalent under (ρ, σ)?
//!
//! A static observation narrows the conjecture before any code runs: two
//! distinct dominoes have equal follow-sets in every context only when
//! both are trump (a trump domino belongs to the trump suit alone; a
//! non-trump domino follows both its natural suits, and no two distinct
//! dominoes share both pips). So the only *static* exchange candidates
//! are equal-count trump pairs adjacent in the live key order — rank-2's
//! orbit generators are far scarcer than the idea page's sketch implies,
//! and everything else must come from context-relative (dynamic)
//! identification, which is a different, weaker theorem.
//!
//! The rig: at solved corpus positions, find every candidate pair, swap
//! it across hidden seats in every fiber world (when the swapped world is
//! still in the fiber — voids may block), replay the solved plan against
//! σ in both worlds, and classify: equal margin, or a taxonomized failure
//! (the pair collided in one trick — the swap flips the trick winner
//! between teams — or a σ tie-break/plan-branch divergence).
//!
//! Verdict, frozen 2026-07-29 (exploratory; quoted in the idea page §7):
//! **falsified as stated.** 160,012 swaps over 36 positions, 28 % change
//! the outcome. Crosstab (equal | collided | clean): cross-team id-gapped
//! 55,004 | 35,124 | 30; cross-team id-adjacent 10,044 | 4,024 | 12;
//! same-team id-gapped 43,324 | 5,130 | 22; same-team id-adjacent
//! 6,620 | 672 | 6. Collision breaks every conditional form (same-team
//! collisions still move the *leader*); the clean channel (σ id
//! tie-breaks + plan-branch divergence) survives every static side
//! condition tested. Blocked-by-voids: 1,090.

use rob_core::{
    algebra_for, derive_rule_cells, DeclarationAlgebra, DominoId, DominoSet, Play, RemainderWorld,
    Seat, TrickKey,
};
use rob_player::player::UtilityLens;
use rob_player::{greedy_sigma, solve, PlanChild, PlanNode};
use rob_verify::p2::boundary_position;

struct Sim {
    hands: [DominoSet; 4],
    leader: Seat,
    trick: Vec<Play>,
    banked: [u32; 2],
}

impl Sim {
    fn actor(&self) -> Seat {
        self.leader.offset(self.trick.len() as u8)
    }

    /// Apply one play; when the trick resolves, report whether `a` and
    /// `b` were both in it.
    fn apply(
        &mut self,
        algebra: &DeclarationAlgebra,
        d: DominoId,
        a: DominoId,
        b: DominoId,
    ) -> bool {
        let actor = self.actor();
        assert!(self.hands[actor.index()].contains(d));
        self.hands[actor.index()].remove(d);
        self.trick.push(Play { actor, domino: d });
        if self.trick.len() == 4 {
            let collided = self.trick.iter().any(|p| p.domino == a)
                && self.trick.iter().any(|p| p.domino == b);
            let result = algebra.resolve_trick(&self.trick).expect("resolves");
            self.banked[result.winner.team().index()] += result.points as u32;
            self.leader = result.winner;
            self.trick.clear();
            return collided;
        }
        false
    }
}

/// Replay plan-vs-σ; returns (margin for the viewer team, pair collided
/// in some trick).
fn replay(
    algebra: &DeclarationAlgebra,
    node: &PlanNode,
    mut sim: Sim,
    viewer: Seat,
    pair: (DominoId, DominoId),
) -> (i64, bool) {
    let mut current = node;
    let mut obs: Vec<(u8, u8)> = Vec::new();
    let mut pending_action = Some(current.action);
    let mut collided = false;
    loop {
        if sim.hands.iter().all(DominoSet::is_empty) {
            let margin = sim.banked[viewer.team().index()] as i64
                - sim.banked[viewer.team().opponent().index()] as i64;
            return (margin, collided);
        }
        let actor = sim.actor();
        let d = if actor == viewer {
            if pending_action.is_none() {
                match current.children.get(&obs).expect("realizable obs is a key") {
                    PlanChild::Node(n) => current = n,
                    PlanChild::Leaf(_) => unreachable!("full-depth plans settle"),
                }
                obs.clear();
            }
            pending_action.take().unwrap_or(current.action)
        } else {
            let choice = greedy_sigma(algebra, &sim.hands[actor.index()], &sim.trick);
            obs.push((actor.index() as u8, choice.index() as u8));
            choice
        };
        collided |= sim.apply(algebra, d, pair.0, pair.1);
        if actor == viewer {
            pending_action = None;
        }
    }
}

/// Static exchange candidates in the pool: equal count value, equal
/// follow-set in all 28 led contexts, and no *live* tile (pool or viewer
/// hand) strictly between their keys in any context where either ranks.
fn candidate_pairs(
    algebra: &DeclarationAlgebra,
    pool: &[DominoId],
    live: &[DominoId],
) -> Vec<(DominoId, DominoId)> {
    let mut pairs = Vec::new();
    for (i, &d) in pool.iter().enumerate() {
        for &e in &pool[i + 1..] {
            if rob_core::domino_from_id(d).count_points()
                != rob_core::domino_from_id(e).count_points()
            {
                continue;
            }
            let mut identical = true;
            for &led in live {
                let q = algebra.led_suit(led);
                if algebra.follows(d, q) != algebra.follows(e, q) {
                    identical = false;
                    break;
                }
                let (kd, ke) = (algebra.trick_key(d, q), algebra.trick_key(e, q));
                if kd == TrickKey::Slough && ke == TrickKey::Slough {
                    continue;
                }
                let (lo, hi) = if kd < ke { (kd, ke) } else { (ke, kd) };
                let blocked = live.iter().any(|&l| {
                    l != d && l != e && {
                        let kl = algebra.trick_key(l, q);
                        lo < kl && kl < hi
                    }
                });
                if blocked {
                    identical = false;
                    break;
                }
            }
            if identical {
                pairs.push((d, e));
            }
        }
    }
    pairs
}

/// Aggregate verdict cells: crosstab of
/// (same-team swap?, id-adjacent pair?) × (equal, differ_collided,
/// differ_clean), plus swap/blocked totals.
#[derive(Clone, Copy, Default)]
struct Verdict {
    swaps: u64,
    blocked: u64,
    // [same_team][id_adjacent] → (equal, differ_collided, differ_clean)
    cells: [[(u64, u64, u64); 2]; 2],
}

impl Verdict {
    fn add(&mut self, other: &Verdict) {
        self.swaps += other.swaps;
        self.blocked += other.blocked;
        for i in 0..2 {
            for j in 0..2 {
                self.cells[i][j].0 += other.cells[i][j].0;
                self.cells[i][j].1 += other.cells[i][j].1;
                self.cells[i][j].2 += other.cells[i][j].2;
            }
        }
    }

    fn print(&self, label: &str) {
        println!("{label}: swaps {} blocked {}", self.swaps, self.blocked);
        for (i, team) in ["cross-team", "same-team"].iter().enumerate() {
            for (j, id) in ["id-gapped", "id-adjacent"].iter().enumerate() {
                let (eq, coll, clean) = self.cells[i][j];
                if eq + coll + clean > 0 {
                    println!(
                        "  {team:10} {id:11}: equal {eq} | differ: collided {coll}, clean {clean}"
                    );
                }
            }
        }
    }
}

fn probe_position(boundary: usize, index: u64) -> Verdict {
    let state = boundary_position(index, boundary);
    let viewer = state.viewer();
    let cells = derive_rule_cells(&state);
    let worlds = cells.fiber_worlds();
    let algebra = algebra_for(state.contract().declaration());
    let pool: Vec<DominoId> = cells.unseen_pool().iter().collect();
    let mut live: Vec<DominoId> = pool.clone();
    live.extend(state.own_remaining_hand().iter());
    let pairs = candidate_pairs(&algebra, &pool, &live);
    let mut verdict = Verdict::default();
    if pairs.is_empty() {
        return verdict;
    }
    let plan = solve(&state, UtilityLens::Points).expect("Points lens");
    assert!(!plan.truncated, "probe needs the full tree");

    // Hidden index → seat, for team classification; id adjacency among
    // live tiles (no live tile's id strictly between the pair's).
    let hidden = state.hidden_seats();
    let id_adjacent = |d: DominoId, e: DominoId| -> bool {
        let (lo, hi) = if d.index() < e.index() {
            (d.index(), e.index())
        } else {
            (e.index(), d.index())
        };
        !live
            .iter()
            .any(|&l| l != d && l != e && lo < l.index() && l.index() < hi)
    };

    for world in &worlds {
        for &(d, e) in &pairs {
            let s = world
                .hidden_hands
                .iter()
                .position(|h| h.contains(d))
                .expect("pool tile is held");
            let t = world
                .hidden_hands
                .iter()
                .position(|h| h.contains(e))
                .expect("pool tile is held");
            if s == t {
                continue;
            }
            let mut swapped = world.clone();
            swapped.hidden_hands[s].remove(d);
            swapped.hidden_hands[s].insert(e);
            swapped.hidden_hands[t].remove(e);
            swapped.hidden_hands[t].insert(d);
            if !cells.fiber_contains(&swapped) {
                verdict.blocked += 1;
                continue;
            }
            verdict.swaps += 1;
            let same_team = usize::from(hidden[s].team().index() == hidden[t].team().index());
            let adj = usize::from(id_adjacent(d, e));
            let run = |w: &RemainderWorld| {
                let mut hands = [DominoSet::empty(); 4];
                hands[viewer.index()] = *state.own_remaining_hand();
                for (i, &seat) in state.hidden_seats().iter().enumerate() {
                    hands[seat.index()] = w.hidden_hands[i];
                }
                replay(
                    &algebra,
                    &plan.root,
                    Sim {
                        hands,
                        leader: state.leader(),
                        trick: state.current_trick().to_vec(),
                        banked: state.hand_points(),
                    },
                    viewer,
                    (d, e),
                )
            };
            let (m1, c1) = run(world);
            let (m2, c2) = run(&swapped);
            let cell = &mut verdict.cells[same_team][adj];
            if m1 == m2 {
                cell.0 += 1;
            } else if c1 || c2 {
                cell.1 += 1;
            } else {
                cell.2 += 1;
            }
        }
    }
    verdict.print(&format!(
        "boundary {boundary} index {index} (fiber {}, pairs {:?})",
        worlds.len(),
        pairs
    ));
    verdict
}

/// Sweep the small-fiber tail plus a boundary-3 sample; aggregate the
/// crosstab verdict. Run:
/// `cargo test --release --test strategic_exchange_probe -- --ignored --nocapture`
#[test]
#[ignore]
fn strategic_exchange_falsification() {
    let mut agg = Verdict::default();
    for boundary in [5usize, 4] {
        for index in 0..108u64 {
            agg.add(&probe_position(boundary, index));
        }
    }
    for index in 0..12u64 {
        agg.add(&probe_position(3, index));
    }
    agg.print("TOTAL");
}
