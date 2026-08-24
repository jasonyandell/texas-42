//! Scalar (H, fixed-uniform-legal) action values at arbitrary decision
//! points (S5c-m2): exact `Q^H` per legal viewer action under the actual
//! hidden-information treatment — every viewer choice below the root is
//! made once per pooled information state against the whole particle set —
//! with the §7.4 fixed uniform-legal field and the declared
//! uniform-over-fiber root weighting. The scalar sibling of `hidden`
//! (symbolic, root-only); mid-trick roots included, exactly like the
//! walker's decision points.
//!
//! Exactness carrier: a particle's weight along a branch is always a unit
//! fraction `1/d` — the product of per-world `1/|legal|` field shares —
//! so weights are stored as bare `u128` denominators and rational
//! arithmetic (walt-geom `Q`, i128 ratios) happens only at trick
//! resolutions and comparisons. Viewer maximization compares unnormalized
//! weighted sums: the particle set is common across the actions of one
//! pooled information state, so the argmax over conditional expectations
//! equals the argmax over the sums. Field-node mass conservation
//! (children sum to the parent) holds by construction — each particle
//! splits into exactly its `|legal|` children at share `1/|legal|` — and
//! is spot-asserted in debug builds.
//!
//! Budgeted: the observation-tree walk costs one budget unit per
//! (particle, node) visit; a call that would exceed its budget returns
//! `None` — the same exclusion honesty as the basin domain's fiber cap,
//! never a silent sample.
//!
//! # Pooled-state memoization (`dag-v1`, S5c-m3)
//!
//! `action_values_dag` is the same computation over the memoized DAG. The
//! memo key is projected semantic content only, per the walt-math ruling:
//! (solver frame) x (canonical weighted world-multiset) x (leader,
//! in-play-order partial-trick prefix, position). The frame is carried by
//! cache scope (one cache per call of one solver instance); the rest is the
//! `BoundaryKey`. Weights are mandatory in the key — viewer maximization is
//! pooled, so argmax and value depend on the weight profile — and enter in
//! projective normal form: the unit-fraction denominator vector divided by
//! its gcd (the computation is homogeneous degree 1 in the weight vector;
//! the stored value is rescaled exactly on hit). The canonical presentation
//! sorts particles under the packed-hands total order; the ruled
//! merge-identical-projections step is vacuous in THIS solver (particles
//! start as distinct deals and every branch applies seat-identical
//! removals, so distinct particles stay distinct — debug-asserted), which
//! also keeps every weight a unit fraction — rational work still
//! concentrates at trick resolutions. If the solver ever gains particle
//! sources where duplicates can arise, the merge requirement reactivates.
//!
//! Entries are stored at trick boundaries (position 0, empty prefix) only.
//! Soundness: every boundary entry carries the full ruled key with the
//! prefix component empty — never a coarsening. Completeness: (boundary
//! state, observed prefix) -> mid-trick pooled state is a well-defined
//! deterministic map but NOT injective — two boundary states differing
//! only in particles the prefix observation filters out converge to the
//! same mid-trick pooled state; surjective determination, not bijection.
//! Convergent mid-trick states are foregone hits by design (walt-math
//! adjudication, S5c-m3): a missed hit costs recomputation, never
//! correctness. Charged `dag-v1` particle-steps are defined over this
//! boundary-memoized walk — the solver version is part of the declared
//! budget inputs.
//!
//! Budget semantics `dag-v1`: the unit is one (particle, node) visit
//! actually computed on the memoized DAG; a cache hit returns the stored
//! exact value and costs zero BY UNIT DEFINITION — not free work, a
//! redeclared unit. Measurability stays a deterministic function of the
//! declared inputs (solver, semantics, budget, root): the cache is created
//! fresh inside each call (never run history), the walk order is fixed, and
//! each call is single-threaded, so no schedule or warmth can move a cap.
//!
//! Everything exploratory tier: exact computed evidence at a declared
//! label, never a promoted status.

use std::collections::HashMap;

use walt_core::{legal_plays, Decl, Domino, DominoSet, Seat, Team, Trick};
use walt_geom::{q, qi, Q};

use crate::scalar::ScalarValuation;

/// One particle: a full deal's remaining hands plus the unit-fraction
/// weight denominator accumulated from field shares.
type Particle = ([DominoSet; Seat::COUNT], u128);

/// The scalar hidden-treatment solver for one (declaration, viewer,
/// focal, valuation) frame.
pub struct ScalarHidden {
    decl: Decl,
    viewer: Seat,
    focal: Team,
    val: ScalarValuation,
}

impl ScalarHidden {
    pub fn new(decl: Decl, viewer: Seat, focal: Team, val: ScalarValuation) -> ScalarHidden {
        ScalarHidden {
            decl,
            viewer,
            focal,
            val,
        }
    }

    /// Exact `Q^H` (future increment, focal-minus-opponent) for every
    /// legal viewer action at the decision, ascending — the viewer to act
    /// is `leader.plus(prefix.len())` and must be this solver's viewer.
    /// `worlds` is the exhaustively enumerated fiber (each world a full
    /// four-seat deal, the viewer's hand identical across them); the root
    /// weighting is uniform over it. `None` when `budget` particle-steps
    /// would be exceeded.
    pub fn action_values(
        &self,
        worlds: &[[DominoSet; Seat::COUNT]],
        leader: Seat,
        prefix: &[Domino],
        budget: &mut u64,
    ) -> Option<Vec<(Domino, Q)>> {
        assert!(!worlds.is_empty(), "a nonempty fiber");
        let k = prefix.len();
        assert!(k < 4, "a decision point sits inside a trick");
        assert_eq!(leader.plus(k), self.viewer, "the root is the viewer's");
        let hand = worlds[0][self.viewer.index()];
        debug_assert!(
            worlds.iter().all(|w| w[self.viewer.index()] == hand),
            "the viewer's hand is common across the fiber"
        );
        let led = (k > 0).then(|| self.decl.led_context(prefix[0]));
        let legal = legal_plays(self.decl, hand, led);
        let mut tiles = [Domino::ALL[0]; 4];
        tiles[..k].copy_from_slice(prefix);
        let n = i128::try_from(worlds.len()).expect("fiber sizes fit i128");
        let mut out = Vec::new();
        for a in legal.iter() {
            let parts: Vec<Particle> = worlds
                .iter()
                .map(|w| {
                    let mut w = *w;
                    w[self.viewer.index()].remove(a);
                    (w, 1u128)
                })
                .collect();
            let mut tiles = tiles;
            tiles[k] = a;
            let v = self.node(&parts, leader, tiles, k + 1, budget)?;
            out.push((a, v * q(1, n)));
        }
        Some(out)
    }

    /// Unnormalized value of one observation node: the sum over particles
    /// of `weight x` (focal future increment given the world), under the
    /// common pooled-information viewer policy below.
    fn node(
        &self,
        parts: &[Particle],
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        budget: &mut u64,
    ) -> Option<Q> {
        let cost = parts.len() as u64;
        if *budget < cost {
            return None;
        }
        *budget -= cost;

        if k == 4 {
            let trick = Trick::new(leader, tiles).expect("distinct tiles by construction");
            let winner = trick.winner(self.decl);
            let mut inc = self.val.trick;
            for d in tiles {
                inc += self.val.tiles[d.index()];
            }
            if winner.team() != self.focal {
                inc = -inc;
            }
            let mass: Q = parts.iter().map(|(_, den)| unit(*den)).sum();
            let banked = qi(i128::from(inc)) * mass;
            if parts[0].0.iter().all(|h| h.is_empty()) {
                return Some(banked);
            }
            let rest = self.node(parts, winner, [Domino::ALL[0]; 4], 0, budget)?;
            return Some(banked + rest);
        }

        let seat = leader.plus(k);
        let led = (k > 0).then(|| self.decl.led_context(tiles[0]));

        if seat == self.viewer {
            // A pooled information state: the hand is common, one choice
            // for the whole particle set, maximized exactly.
            let hand = parts[0].0[seat.index()];
            debug_assert!(parts.iter().all(|(w, _)| w[seat.index()] == hand));
            let legal = legal_plays(self.decl, hand, led);
            let mut best: Option<Q> = None;
            for a in legal.iter() {
                let child: Vec<Particle> = parts
                    .iter()
                    .map(|(w, den)| {
                        let mut w = *w;
                        w[seat.index()].remove(a);
                        (w, *den)
                    })
                    .collect();
                let mut tiles = tiles;
                tiles[k] = a;
                let v = self.node(&child, leader, tiles, k + 1, budget)?;
                best = Some(match best {
                    None => v,
                    Some(b) if v > b => v,
                    Some(b) => b,
                });
            }
            return best;
        }

        // A field seat: §7.4 chance. Each particle splits over its own
        // legal set at share 1/|legal|; branching on the observed tile.
        let mut union = DominoSet::EMPTY;
        for (w, _) in parts {
            union = union.union(legal_plays(self.decl, w[seat.index()], led));
        }
        let mut acc = qi(0);
        for m in union.iter() {
            let mut child: Vec<Particle> = Vec::new();
            for (w, den) in parts {
                let lg = legal_plays(self.decl, w[seat.index()], led);
                if lg.contains(m) {
                    let mut w = *w;
                    w[seat.index()].remove(m);
                    child.push((w, den * lg.len() as u128));
                }
            }
            if child.is_empty() {
                continue;
            }
            let mut tiles = tiles;
            tiles[k] = m;
            acc += self.node(&child, leader, tiles, k + 1, budget)?;
        }
        Some(acc)
    }
}

/// Cache statistics of one `dag-v1` measurement call — reporting material
/// (provenance for results files), never an input to any verdict.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MemoStats {
    /// Particle-steps charged: the `dag-v1` budget consumption.
    pub steps: u64,
    /// Exact particle-steps the unmemoized tree walk would have charged
    /// for the identical computation (each entry stores its subtree's
    /// tree cost; hits propagate it) — the tree-v0 cost of this
    /// measurement, quantifying the semantics change. A lower bound when
    /// the call was capped.
    pub tree_steps: u128,
    /// Boundary-cache hits (subtrees returned from the stored value).
    pub hits: u64,
    /// Entries in the per-call cache when the call ended.
    pub entries: usize,
    /// Total particles across stored keys — 32 bytes each of key payload,
    /// the dominant cache memory term.
    pub key_particles: u64,
}

/// A trick-boundary pooled information state, canonically encoded: the
/// leader, then per particle (packed remaining hands, gcd-normalized
/// unit-fraction weight denominator), sorted ascending by packed hands.
#[derive(PartialEq, Eq, Hash)]
struct BoundaryKey {
    leader: Seat,
    parts: Box<[(u128, u128)]>,
}

/// Scoped to ONE `action_values_dag` call of ONE solver instance: the
/// ruled key's solver-frame components (declaration, viewer, focal team,
/// valuation) are carried by this scope and so need not repeat per entry.
/// A table shared across frames without the frame in the key would be the
/// one genuinely unsound configuration (walt-math, S5c-m3) — nothing here
/// can construct one. The value is (stored Q under normalized weights,
/// the subtree's exact tree-v0 particle-step count).
struct DagCache {
    map: HashMap<BoundaryKey, (Q, u128)>,
    stats: MemoStats,
}

/// The four remaining hands as one 112-bit word — the declared total order
/// for canonical particle presentation.
fn pack(hands: &[DominoSet; Seat::COUNT]) -> u128 {
    let mut k = 0u128;
    for h in hands {
        k = (k << Domino::COUNT) | u128::from(h.bits());
    }
    k
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

impl ScalarHidden {
    /// `action_values` over the memoized DAG (`dag-v1` budget semantics):
    /// identical exact values, with whole subtrees below a repeated
    /// trick-boundary pooled state returned from the per-call cache at
    /// zero particle-step cost. `None` values when the `dag-v1` budget
    /// would be exceeded — excluded, never sampled; the stats are
    /// returned either way.
    pub fn action_values_dag(
        &self,
        worlds: &[[DominoSet; Seat::COUNT]],
        leader: Seat,
        prefix: &[Domino],
        budget: &mut u64,
    ) -> (Option<Vec<(Domino, Q)>>, MemoStats) {
        assert!(!worlds.is_empty(), "a nonempty fiber");
        let k = prefix.len();
        assert!(k < 4, "a decision point sits inside a trick");
        assert_eq!(leader.plus(k), self.viewer, "the root is the viewer's");
        let hand = worlds[0][self.viewer.index()];
        debug_assert!(
            worlds.iter().all(|w| w[self.viewer.index()] == hand),
            "the viewer's hand is common across the fiber"
        );
        // Canonical presentation once at the root: along any branch every
        // particle loses the same (seat, tile) pairs, which preserves
        // pairwise packed-hands order, so every node list stays sorted.
        let mut sorted: Vec<[DominoSet; Seat::COUNT]> = worlds.to_vec();
        sorted.sort_unstable_by_key(pack);
        debug_assert!(
            sorted.windows(2).all(|p| pack(&p[0]) < pack(&p[1])),
            "fiber worlds are distinct, so merging never fires"
        );
        let led = (k > 0).then(|| self.decl.led_context(prefix[0]));
        let legal = legal_plays(self.decl, hand, led);
        let mut tiles = [Domino::ALL[0]; 4];
        tiles[..k].copy_from_slice(prefix);
        let n = i128::try_from(worlds.len()).expect("fiber sizes fit i128");
        let mut cache = DagCache {
            map: HashMap::new(),
            stats: MemoStats::default(),
        };
        let mut out = Vec::new();
        for a in legal.iter() {
            let parts: Vec<Particle> = sorted
                .iter()
                .map(|w| {
                    let mut w = *w;
                    w[self.viewer.index()].remove(a);
                    (w, 1u128)
                })
                .collect();
            let mut tiles = tiles;
            tiles[k] = a;
            let Some((v, t)) = self.node_dag(&mut cache, &parts, leader, tiles, k + 1, budget)
            else {
                return (None, cache.stats);
            };
            cache.stats.tree_steps += t;
            out.push((a, v * q(1, n)));
        }
        (Some(out), cache.stats)
    }

    /// The memoized observation node: the same recursion as `node`, with
    /// trick-boundary states (k == 0) looked up in — and stored to — the
    /// per-call cache under the canonical projective key. Returns the
    /// node's unnormalized value and its subtree's exact tree-v0
    /// particle-step count.
    fn node_dag(
        &self,
        cache: &mut DagCache,
        parts: &[Particle],
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        budget: &mut u64,
    ) -> Option<(Q, u128)> {
        // Boundary lookup before any budget charge: a hit costs zero by
        // the dag-v1 unit definition.
        let normalized = (k == 0).then(|| {
            let g = parts.iter().fold(0u128, |g, (_, den)| gcd(g, *den));
            let key = BoundaryKey {
                leader,
                parts: parts
                    .iter()
                    .map(|(w, den)| (pack(w), den / g))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            };
            (g, key)
        });
        let scale = if let Some((g, key)) = &normalized {
            let g = i128::try_from(*g).expect("field-share denominators fit i128");
            if let Some((v, t)) = cache.map.get(key) {
                cache.stats.hits += 1;
                // Stored under weights scaled by g, homogeneous degree 1.
                return Some((*v * q(1, g), *t));
            }
            Some(g)
        } else {
            None
        };

        let cost = parts.len() as u64;
        if *budget < cost {
            return None;
        }
        *budget -= cost;
        cache.stats.steps += cost;
        let mut tree = u128::from(cost);

        let v = if k == 4 {
            let trick = Trick::new(leader, tiles).expect("distinct tiles by construction");
            let winner = trick.winner(self.decl);
            let mut inc = self.val.trick;
            for d in tiles {
                inc += self.val.tiles[d.index()];
            }
            if winner.team() != self.focal {
                inc = -inc;
            }
            let mass: Q = parts.iter().map(|(_, den)| unit(*den)).sum();
            let banked = qi(i128::from(inc)) * mass;
            if parts[0].0.iter().all(|h| h.is_empty()) {
                banked
            } else {
                let (rest, t) =
                    self.node_dag(cache, parts, winner, [Domino::ALL[0]; 4], 0, budget)?;
                tree += t;
                banked + rest
            }
        } else {
            let seat = leader.plus(k);
            let led = (k > 0).then(|| self.decl.led_context(tiles[0]));

            if seat == self.viewer {
                let hand = parts[0].0[seat.index()];
                debug_assert!(parts.iter().all(|(w, _)| w[seat.index()] == hand));
                let legal = legal_plays(self.decl, hand, led);
                let mut best: Option<Q> = None;
                for a in legal.iter() {
                    let child: Vec<Particle> = parts
                        .iter()
                        .map(|(w, den)| {
                            let mut w = *w;
                            w[seat.index()].remove(a);
                            (w, *den)
                        })
                        .collect();
                    let mut tiles = tiles;
                    tiles[k] = a;
                    let (v, t) = self.node_dag(cache, &child, leader, tiles, k + 1, budget)?;
                    tree += t;
                    best = Some(match best {
                        None => v,
                        Some(b) if v > b => v,
                        Some(b) => b,
                    });
                }
                best.expect("a seat holding tiles has a legal play")
            } else {
                let mut union = DominoSet::EMPTY;
                for (w, _) in parts {
                    union = union.union(legal_plays(self.decl, w[seat.index()], led));
                }
                let mut acc = qi(0);
                for m in union.iter() {
                    let mut child: Vec<Particle> = Vec::new();
                    for (w, den) in parts {
                        let lg = legal_plays(self.decl, w[seat.index()], led);
                        if lg.contains(m) {
                            let mut w = *w;
                            w[seat.index()].remove(m);
                            child.push((w, den * lg.len() as u128));
                        }
                    }
                    if child.is_empty() {
                        continue;
                    }
                    let mut tiles = tiles;
                    tiles[k] = m;
                    let (v, t) = self.node_dag(cache, &child, leader, tiles, k + 1, budget)?;
                    tree += t;
                    acc += v;
                }
                acc
            }
        };

        if let (Some((_, key)), Some(g)) = (normalized, scale) {
            cache.stats.key_particles += key.parts.len() as u64;
            cache.map.insert(key, (v * qi(g), tree));
            cache.stats.entries = cache.map.len();
        }
        Some((v, tree))
    }
}

fn unit(den: u128) -> Q {
    q(
        1,
        i128::try_from(den).expect("field-share denominators fit i128"),
    )
}
