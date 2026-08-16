//! The regret walker (S5a): per-decision fiber-expected action values along
//! a receipt transcript, on the scalar PI solver, under declared knobs.
//!
//! For each decision point of one seat in one hand, the walker builds the
//! §2.1 kernel from the actor-attributed legal public prefix
//! (`ReceiptDecision`), evaluates every legal action's exact PI continuation
//! value in each fiber world (`ScalarPi::action_values`), and aggregates:
//!
//! - the **expectation** of each action under the declared weighting
//!   (uniform-over-fiber for now) — exact rationals, since under a uniform
//!   weighting the expectation is an integer sum over an exact count;
//! - the **regret** of the transcript's chosen action against the best
//!   expectation. Per-decision regret on the future increment alone is exact
//!   for the additive family because the banked term is action-independent
//!   (§8.5), so the transcript total decomposes exactly by decision;
//! - the **dominance primitive**: the world-count triple (#gt, #eq, #lt)
//!   per ordered action pair (walt-math amendment 2026-08-10; the T/W/S/I
//!   classes are derived views, conflicts fire on W). Weighting-free, but
//!   never operator-free (§7.6);
//! - the **localization primitive**: the live-world count — worlds whose
//!   best action still clears the viewer team's win threshold (banked +
//!   future differential against the bid, in the role-shifted strict-<
//!   form). Live count zero is the raw material of the labeled "lost ...
//!   under PI semantics, worldwise" verdict.
//!
//! Fibers at or below a declared threshold are enumerated exhaustively;
//! above it the kernel's exactly-uniform sampler runs at a recorded
//! per-decision seed and every downstream quantity is marked `Sampled` —
//! never silently.
//!
//! Determinism: the per-world solves are partitioned into chunks solved by
//! scoped threads; every reduction is an exact integer sum or count —
//! associative and commutative — so the result is independent of the
//! partition and the thread schedule (and of the solver caches, whose
//! entries are exact values of projected states; the memory-bounding
//! `trim_cache` therefore cannot change any output).

use std::sync::atomic::{AtomicUsize, Ordering};

use walt_core::receipt::ReceiptHand;
use walt_core::replay::TRICKS_PER_HAND;
use walt_core::{legal_plays, Domino, DominoSet, Seat, Team, Trick};
use walt_geom::{q, qi, Q};
use walt_kernel::{FiberDp, ReceiptDecision, SplitMix64};
use walt_strat::{ScalarPi, ScalarValuation, WeightingLabel};

use crate::conflict::{Grade, RegretConflict};
use crate::lesson::{DominanceClass, DominanceTriple, OperatorPair};

/// Declared walker knobs. The operator is scalar PI and the weighting is
/// uniform-over-fiber — both recorded on every output, not implied.
#[derive(Clone, Debug)]
pub struct WalkerConfig {
    /// Fibers at or below this count are enumerated exhaustively.
    pub enumeration_threshold: u128,
    /// Uniform draws (with replacement) for fibers above the threshold.
    pub sample_draws: u32,
    /// Base seed; each decision derives and records its own stream seed.
    pub seed: u64,
    /// First trick to walk (1 walks the whole transcript).
    pub min_trick: usize,
    /// Worker threads; 0 uses the machine's available parallelism.
    pub threads: usize,
}

impl WalkerConfig {
    /// The CI-subset configuration: exhaustive from trick 4 on (fibers at
    /// most 34,650), a small recorded sample earlier. Pins produced under
    /// this config say so.
    pub fn ci() -> WalkerConfig {
        WalkerConfig {
            enumeration_threshold: 40_000,
            sample_draws: 64,
            seed: 0x5EA7_425A,
            min_trick: 3,
            threads: 0,
        }
    }

    /// The full-corpus configuration for the release binary: exhaustive
    /// through trick-3-scale fibers, a serious recorded sample at tricks
    /// 1-2.
    pub fn full() -> WalkerConfig {
        WalkerConfig {
            enumeration_threshold: 1_000_000,
            sample_draws: 2_000,
            seed: 0x5EA7_425A,
            min_trick: 1,
            threads: 0,
        }
    }

    /// The designated-fixture configuration: the whole transcript at the CI
    /// thresholds, so the byte-frozen artifact exercises both bases and
    /// still runs in CI time.
    pub fn fixture() -> WalkerConfig {
        WalkerConfig {
            min_trick: 1,
            ..WalkerConfig::ci()
        }
    }
}

/// How a decision's fiber expectation was carried out.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EvidenceBasis {
    /// Every fiber world enumerated and solved.
    Exhaustive { worlds: u128 },
    /// Exactly-uniform draws with replacement at a recorded seed. Marked:
    /// nothing computed on this basis is worldwise or exact-expectation.
    Sampled { seed: u64, draws: u32 },
}

/// One walked decision point: the typed per-decision record. Values are
/// future increments from this decision (§8.5); `banked` is the
/// action-independent completed-trick differential that restores full-hand
/// totals.
#[derive(Clone, Debug)]
pub struct DecisionRecord {
    pub trick_no: usize,
    pub ply: usize,
    /// Exact `|Phi(C)|` from the counting DP, even when evaluation sampled.
    pub fiber: u128,
    pub basis: EvidenceBasis,
    pub operator: OperatorPair,
    pub weighting: WeightingLabel,
    /// Completed-trick differential for the viewer's team, in valuation
    /// units (action- and world-independent: it is public history).
    pub banked: i64,
    /// The legal actions, ascending; identical across the fiber (the legal
    /// set is a function of the viewer's hand and the led context).
    pub actions: Vec<Domino>,
    pub chosen: Domino,
    pub chosen_index: usize,
    /// Expected future-increment value per action under `basis`.
    pub values: Vec<Q>,
    /// Best expectation minus the chosen action's; zero when optimal.
    pub regret: Q,
    /// The dominance primitive (walt-math amendment, 2026-08-10): the
    /// world-count triple for every ordered action pair, flattened
    /// `i * n + j` = (action i over action j) over the evaluated world set.
    /// The named classes (T/W/S/I) are derived views, never stored.
    pub triples: Vec<DominanceTriple>,
    /// Derived view of `triples`, cached at construction for cross-module
    /// API stability: the ordered pairs `(i, j)` where action `i`
    /// dominates action `j` — strict somewhere, never worse (class W or S;
    /// §9.6: conflicts fire on W, ties never collapse into dominance).
    pub dominance: Vec<(usize, usize)>,
    /// Some alternative worldwise-dominates the chosen action. Derived
    /// from `triples` at construction, nothing else.
    pub chosen_dominated: bool,
    /// The localization primitive (walt-math amendment): the number of
    /// evaluated worlds in which some legal action still clears the win
    /// threshold. Zero means every evaluated world x action loses.
    pub live_worlds: usize,
}

impl DecisionRecord {
    /// The triple for ordered pair (action `i` over action `j`).
    pub fn triple(&self, i: usize, j: usize) -> DominanceTriple {
        self.triples[i * self.actions.len() + j]
    }

    /// Derived view: no evaluated world has a winning action left.
    pub fn all_actions_lose(&self) -> bool {
        self.live_worlds == 0
    }

    /// The decision's conflict, when the chosen action is regretted. The
    /// grade quotes exactly what was computed: worldwise dominance only on
    /// an exhaustive basis, sampled bases always marked.
    pub fn conflict(&self, hand: usize, seat: Seat) -> Option<RegretConflict> {
        if self.regret == qi(0) {
            return None;
        }
        let chosen_value = self.values[self.chosen_index];
        let grade = match self.basis {
            EvidenceBasis::Exhaustive { .. } if self.chosen_dominated => {
                Grade::WorldwiseDominance {
                    operator: self.operator,
                }
            }
            EvidenceBasis::Exhaustive { .. } => Grade::ExactExpectation {
                operator: self.operator,
                weighting: self.weighting,
            },
            EvidenceBasis::Sampled { seed, draws } => Grade::Sampled {
                operator: self.operator,
                weighting: self.weighting,
                seed,
                draws,
            },
        };
        Some(RegretConflict {
            hand,
            seat,
            trick_no: self.trick_no,
            ply: self.ply,
            chosen: self.chosen,
            better: self
                .actions
                .iter()
                .zip(&self.values)
                .filter(|(_, v)| **v > chosen_value)
                .map(|(a, _)| *a)
                .collect(),
            regret: self.regret,
            grade,
            fiber: self.fiber,
        })
    }
}

/// The labeled localization verdict: from this decision on, every action in
/// every fiber world loses under PI semantics. Constructed only from an
/// exhaustive basis, so the worldwise quantifier is real; the operator label
/// travels with it because the label-free version would need the §7.7
/// max-min imperfect-information operator, which does not exist here.
#[derive(Clone, Copy, Debug)]
pub struct LostVerdict {
    pub trick_no: usize,
    pub ply: usize,
    pub operator: OperatorPair,
    /// This was the seat's first decision of the hand and the walk started
    /// at trick 1. With `ply == 0` the kernel is exactly the seat's dealt
    /// knowledge: "lost at the deal (PI, worldwise)". With `ply > 0` the
    /// verdict is conditioned on the opening prefix the seat had already
    /// observed.
    pub at_the_deal: bool,
}

/// One seat's walked transcript with its localization summary. Total regret
/// is the exact sum of per-decision regrets — exact for the additive family
/// because each decision's banked term is action-independent (§8.5).
#[derive(Clone, Debug)]
pub struct HandSeatWalk {
    pub hand: usize,
    pub seat: Seat,
    pub decisions: Vec<DecisionRecord>,
    pub total_regret: Q,
    pub zero_regret_decisions: usize,
    pub dominated_choices: usize,
    pub lost_from: Option<LostVerdict>,
}

impl HandSeatWalk {
    /// The walk's regretted decisions as typed conflicts, decision order.
    pub fn conflicts(&self) -> Vec<RegretConflict> {
        self.decisions
            .iter()
            .filter_map(|d| d.conflict(self.hand, self.seat))
            .collect()
    }
}

/// The win condition of the real objective game for one team, in the
/// adjudicated role-shifted strict-< form: the focal team loses exactly
/// when its full-hand q_points differential `v` sits strictly below one
/// integer threshold. The team's points are `(42 + v) / 2` (each trick's
/// weight `1 + count` sums to 42 over the hand), and the declaring side
/// makes at or above its bid, so: declaring loses iff `v < 2*bid - 42`;
/// defending loses iff the declarers make, `v <= 42 - 2*bid`, which over
/// the integers is the shifted strict form `v < 43 - 2*bid`.
#[derive(Clone, Copy, Debug)]
struct WinCondition {
    lose_below: i64,
}

impl WinCondition {
    fn of(hand: &ReceiptHand, focal: Team) -> WinCondition {
        let bid = i64::from(hand.bid_points);
        WinCondition {
            lose_below: if hand.declaring_team == focal {
                2 * bid - 42
            } else {
                43 - 2 * bid
            },
        }
    }

    fn wins(self, v_total: i64) -> bool {
        v_total >= self.lose_below
    }
}

/// The focal team's completed-trick differential before `trick_no`, derived
/// by replay (the receipt's winner/points fields are never trusted here).
fn banked_differential(hand: &ReceiptHand, trick_no: usize, focal: Team) -> i64 {
    let mut out = 0i64;
    for rec in hand.tricks.iter().take(trick_no - 1) {
        let tiles = rec.plays.map(|(_, d)| d);
        let trick = Trick::new(rec.plays[0].0, tiles).expect("receipt tricks hold distinct tiles");
        let v = i64::from(trick.points());
        out += if trick.winner(hand.decl).team() == focal {
            v
        } else {
            -v
        };
    }
    out
}

/// The per-decision recorded sample seed: a fixed mix of the base seed and
/// the decision's identity, so any single decision's stream can be replayed
/// without walking the rest.
fn decision_seed(base: u64, hand: usize, seat: Seat, trick_no: usize) -> u64 {
    let id = ((hand * 4 + seat.index()) * 8 + trick_no) as u64;
    SplitMix64::new(base ^ id.wrapping_mul(0x9E37_79B9_7F4A_7C15)).next_u64()
}

/// Per-chunk partial aggregate: integer sums and world counts only, so the
/// fold is exact and order-insensitive; it is still folded in chunk order.
/// The pairwise counts are the dominance primitive — the triple's `lt`
/// coordinate is the transpose's `gt`, so only `gt` and `eq` are carried.
struct ChunkAggregate {
    sums: Vec<i128>,
    /// `gt[i * n + j]`: #worlds with `v(a_i) > v(a_j)` in the chunk.
    gt: Vec<usize>,
    /// `eq[i * n + j]`: #worlds with `v(a_i) = v(a_j)` in the chunk.
    eq: Vec<usize>,
    /// #worlds whose best action still clears the win threshold.
    live: usize,
}

impl ChunkAggregate {
    fn identity(n: usize) -> ChunkAggregate {
        ChunkAggregate {
            sums: vec![0; n],
            gt: vec![0; n * n],
            eq: vec![0; n * n],
            live: 0,
        }
    }

    fn absorb_world(&mut self, values: &[i64], banked: i64, win: WinCondition) {
        let n = values.len();
        let mut best = i64::MIN;
        for (i, v) in values.iter().enumerate() {
            self.sums[i] += i128::from(*v);
            best = best.max(*v);
            for (j, w) in values.iter().enumerate() {
                if v > w {
                    self.gt[i * n + j] += 1;
                } else if v == w {
                    self.eq[i * n + j] += 1;
                }
            }
        }
        if win.wins(banked + best) {
            self.live += 1;
        }
    }

    fn merge(&mut self, other: &ChunkAggregate) {
        for (a, b) in self.sums.iter_mut().zip(&other.sums) {
            *a += b;
        }
        for (a, b) in self.gt.iter_mut().zip(&other.gt) {
            *a += b;
        }
        for (a, b) in self.eq.iter_mut().zip(&other.eq) {
            *a += b;
        }
        self.live += other.live;
    }
}

/// Largest work-partitioning chunk. The partition only balances load: every
/// reduction below is an exact integer sum or a boolean lattice operation —
/// associative and commutative — so the aggregate is identical under any
/// partition and any schedule.
const CHUNK: usize = 512;

/// Per-thread bound on the solver's trick-boundary cache, in entries
/// (~4 GB total at 18 workers). Unbounded caches at horizon-6/7 decisions
/// were measured at 14-19 GB for a single decision and drew a
/// memory-pressure kill mid-corpus on 2026-08-10; the bound is a memory
/// lever only — cache entries are exact values of projected states, so
/// trimming cannot change any output (verified by the byte-stable resume
/// diff of that run's part 2).
const CACHE_TRIM: usize = 4_000_000;

/// Solves every world's action values and folds the aggregates, in parallel
/// scoped threads over fixed chunks. Each thread owns a `ScalarPi` (caches
/// hold exact values of projected states, so sharing or not sharing them
/// cannot change any output).
#[allow(clippy::too_many_arguments)]
fn aggregate_worlds(
    worlds: &[[DominoSet; Seat::COUNT]],
    decision: &ReceiptDecision,
    actions: &[Domino],
    val: &ScalarValuation,
    banked: i64,
    win: WinCondition,
    threads: usize,
) -> ChunkAggregate {
    let n = actions.len();
    let workers = if threads == 0 {
        std::thread::available_parallelism().map_or(1, |p| p.get())
    } else {
        threads
    }
    .max(1);
    let chunk = (worlds.len() / (8 * workers)).clamp(1, CHUNK);
    let chunks: Vec<&[[DominoSet; Seat::COUNT]]> = worlds.chunks(chunk).collect();
    let next = AtomicUsize::new(0);
    let workers = workers.min(chunks.len());

    let decl = decision.kernel.decl();
    let focal = decision.kernel.viewer().team();

    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..workers)
            .map(|_| {
                scope.spawn(|| {
                    let mut pi = ScalarPi::new(decl, focal, val.clone());
                    let mut agg = ChunkAggregate::identity(n);
                    loop {
                        let c = next.fetch_add(1, Ordering::Relaxed);
                        if c >= chunks.len() {
                            break;
                        }
                        for hands in chunks[c] {
                            let solved =
                                pi.action_values(*hands, decision.leader, &decision.prefix);
                            debug_assert!(
                                solved.iter().map(|(d, _)| *d).eq(actions.iter().copied()),
                                "the legal set is view-determined, hence world-independent"
                            );
                            let values: Vec<i64> = solved.into_iter().map(|(_, v)| v).collect();
                            agg.absorb_world(&values, banked, win);
                        }
                        pi.trim_cache(CACHE_TRIM);
                    }
                    agg
                })
            })
            .collect();
        let mut total = ChunkAggregate::identity(n);
        for handle in handles {
            total.merge(&handle.join().expect("worker thread"));
        }
        total
    })
}

/// Walks one decision point. `decision` must come from `hand` (the walker
/// derives everything else from the pair).
pub fn walk_decision(
    hand: &ReceiptHand,
    decision: &ReceiptDecision,
    config: &WalkerConfig,
) -> DecisionRecord {
    let kernel = &decision.kernel;
    let viewer = kernel.viewer();
    let fiber = kernel.count();
    assert!(fiber > 0, "the true world inhabits every receipt fiber");

    let (worlds, basis): (Vec<[DominoSet; Seat::COUNT]>, EvidenceBasis) =
        if fiber <= config.enumeration_threshold {
            (
                kernel.worlds().map(|w| w.hands()).collect(),
                EvidenceBasis::Exhaustive { worlds: fiber },
            )
        } else {
            let seed = decision_seed(config.seed, hand.id, viewer, decision.trick_no);
            let mut rng = SplitMix64::new(seed);
            let dp = FiberDp::new(kernel);
            let worlds = (0..config.sample_draws)
                .map(|_| {
                    kernel
                        .sample_with(&dp, &mut rng)
                        .expect("nonempty fiber")
                        .hands()
                })
                .collect();
            (
                worlds,
                EvidenceBasis::Sampled {
                    seed,
                    draws: config.sample_draws,
                },
            )
        };

    let led = (decision.ply > 0).then(|| kernel.decl().led_context(decision.prefix[0]));
    let actions: Vec<Domino> = legal_plays(kernel.decl(), kernel.viewer_hand(), led)
        .iter()
        .collect();
    let chosen_index = actions
        .iter()
        .position(|d| *d == decision.chosen)
        .expect("the receipt's play is legal (replay-verified)");

    let focal = viewer.team();
    let banked = banked_differential(hand, decision.trick_no, focal);
    let win = WinCondition::of(hand, focal);
    let val = ScalarValuation::trick_plus_count();
    let agg = aggregate_worlds(
        &worlds,
        decision,
        &actions,
        &val,
        banked,
        win,
        config.threads,
    );

    let count = i128::try_from(worlds.len()).expect("world count fits i128");
    let values: Vec<Q> = agg.sums.iter().map(|s| q(*s, count)).collect();
    let best = *values.iter().max().expect("at least one legal action");
    let n = actions.len();
    let mut triples = Vec::with_capacity(n * n);
    for i in 0..n {
        for j in 0..n {
            let gt = agg.gt[i * n + j];
            let eq = agg.eq[i * n + j];
            let lt = agg.gt[j * n + i];
            assert_eq!(
                gt + eq + lt,
                worlds.len(),
                "the triple partitions the worlds"
            );
            triples.push(DominanceTriple { gt, eq, lt });
        }
    }

    let mut dominance = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if i != j
                && matches!(
                    triples[i * n + j].class(),
                    DominanceClass::WeakStrictSomewhere | DominanceClass::StrictEverywhere
                )
            {
                dominance.push((i, j));
            }
        }
    }
    let chosen_dominated = dominance.iter().any(|(_, j)| *j == chosen_index);

    DecisionRecord {
        trick_no: decision.trick_no,
        ply: decision.ply,
        fiber,
        basis,
        operator: OperatorPair::walker_scalar(),
        weighting: WeightingLabel::UniformOverFiber,
        banked,
        chosen: decision.chosen,
        chosen_index,
        regret: best - values[chosen_index],
        values,
        actions,
        triples,
        dominance,
        chosen_dominated,
        live_worlds: agg.live,
    }
}

/// Walks every decision of `seat` in `hand` from `config.min_trick` on and
/// assembles the localization summary.
pub fn walk_seat(hand: &ReceiptHand, seat: Seat, config: &WalkerConfig) -> HandSeatWalk {
    let mut decisions = Vec::new();
    for trick_no in config.min_trick..=TRICKS_PER_HAND {
        let decision = ReceiptDecision::at(hand, trick_no, seat)
            .expect("replay-verified receipts yield every decision point");
        decisions.push(walk_decision(hand, &decision, config));
    }
    let total_regret = decisions.iter().map(|d| d.regret).sum();
    let zero_regret_decisions = decisions.iter().filter(|d| d.regret == qi(0)).count();
    let dominated_choices = decisions.iter().filter(|d| d.chosen_dominated).count();
    let lost_from = decisions
        .iter()
        .find(|d| d.all_actions_lose() && matches!(d.basis, EvidenceBasis::Exhaustive { .. }))
        .map(|d| LostVerdict {
            trick_no: d.trick_no,
            ply: d.ply,
            operator: d.operator,
            at_the_deal: d.trick_no == 1,
        });
    HandSeatWalk {
        hand: hand.id,
        seat,
        decisions,
        total_regret,
        zero_regret_decisions,
        dominated_choices,
        lost_from,
    }
}
