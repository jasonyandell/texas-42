//! Scalar perfect-information minimax and the scalar response census of the
//! exp5 probe suite (`walt/probes/exp5`, exploratory tier).
//!
//! The exp5 censuses label each world by its exact root value vector under an
//! integer per-trick valuation, counted as the focal team's total minus the
//! opponents': a resolved trick is worth `trick + sum tile[d]` to the side
//! that captured it. `q_trick` is the symmetric baseline (each trick worth
//! +-1); `q_points` is the real straight-42 scoring differential (each trick
//! worth +-(1 + the count points of its four tiles)). The action label is the
//! set of optimal root actions.
//!
//! This is the same PI operator as `pi` (§9.9 at a single valuation), carried
//! in plain integers with a trick-boundary cache so whole-fiber and
//! 10,000-world censuses at horizon 5 stay affordable. The cache key is the
//! semantic state (remaining hands, leader) and the cached value is derived
//! from it alone -- additivity puts the banked score outside the subgame.

use std::collections::{BTreeMap, HashMap};
use std::hash::{BuildHasherDefault, Hasher};

use crate::kernel::Kernel;
use crate::rules::{legal_plays, Decl, Domino, DominoSet, Seat, Team, Trick};

/// A multiplicative hasher for the packed state key: the key is already a
/// near-uniform bit string, so SipHash's collision resistance buys nothing
/// here and costs a lot on the census's hottest path.
#[derive(Default)]
pub struct KeyHasher(u64);

impl Hasher for KeyHasher {
    fn write(&mut self, _: &[u8]) {
        unreachable!("the cache key hashes through write_u128 only");
    }

    fn write_u128(&mut self, k: u128) {
        const M: u64 = 0x9e37_79b9_7f4a_7c15;
        self.0 = ((k as u64) ^ ((k >> 64) as u64))
            .wrapping_mul(M)
            .rotate_left(31)
            ^ ((k >> 32) as u64).wrapping_mul(M);
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

/// An integer per-trick valuation, focal-minus-opponent.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ScalarValuation {
    pub trick: i64,
    pub tiles: [i64; Domino::COUNT],
}

impl ScalarValuation {
    /// exp5 `q_trick`: each trick worth +-1.
    pub fn trick_only() -> ScalarValuation {
        ScalarValuation {
            trick: 1,
            tiles: [0; Domino::COUNT],
        }
    }

    /// exp5 `q_points`: each trick worth +-(1 + count points of its tiles).
    pub fn trick_plus_count() -> ScalarValuation {
        let mut tiles = [0; Domino::COUNT];
        for d in Domino::ALL {
            tiles[d.index()] = i64::from(d.count());
        }
        ScalarValuation { trick: 1, tiles }
    }
}

/// One scalar PI solver with a boundary cache shared across every world and
/// root it is asked about (worlds of one fiber share suffix states heavily).
pub struct ScalarPi {
    decl: Decl,
    focal: Team,
    val: ScalarValuation,
    cache: HashMap<u128, i64, BuildHasherDefault<KeyHasher>>,
}

/// (hands, leader) packed as 4 x 28 bits + 2: the projected semantic state.
fn key(hands: &[DominoSet; Seat::COUNT], leader: Seat) -> u128 {
    let mut k = 0u128;
    for h in hands {
        k = (k << Domino::COUNT) | u128::from(h.bits());
    }
    (k << 2) | leader.index() as u128
}

impl ScalarPi {
    pub fn new(decl: Decl, focal: Team, val: ScalarValuation) -> ScalarPi {
        ScalarPi {
            decl,
            focal,
            val,
            cache: HashMap::default(),
        }
    }

    pub fn cache_len(&self) -> usize {
        self.cache.len()
    }

    /// Drops the boundary cache when it has grown past `max_entries`. A
    /// memory bound, never a semantic knob: every cache entry is the exact
    /// minimax value of its projected state, so clearing can only cost
    /// recomputation — values are identical with any trim policy.
    pub fn trim_cache(&mut self, max_entries: usize) {
        if self.cache.len() > max_entries {
            self.cache = HashMap::default();
        }
    }

    /// The exact root value for every legal lead of `leader`, ascending.
    pub fn root_values(
        &mut self,
        hands: [DominoSet; Seat::COUNT],
        leader: Seat,
    ) -> Vec<(Domino, i64)> {
        self.action_values(hands, leader, &[])
    }

    /// The exact continuation value for every legal play of the seat to act
    /// at an arbitrary point of the current trick. `prefix` holds the
    /// trick's plays so far in play order (empty when the seat leads); the
    /// acting seat is `leader.plus(prefix.len())` and its hand still
    /// contains the candidate tiles. Values are future increments from this
    /// decision (§8.5 future-increment mode): the current trick counts in
    /// full at its resolution — its capture is still undecided and shared by
    /// every action — while completed tricks are the caller's
    /// action-independent banked term.
    pub fn action_values(
        &mut self,
        hands: [DominoSet; Seat::COUNT],
        leader: Seat,
        prefix: &[Domino],
    ) -> Vec<(Domino, i64)> {
        assert!(prefix.len() < 4, "a decision point sits inside a trick");
        let k = prefix.len();
        let seat = leader.plus(k);
        let led = (k > 0).then(|| self.decl.led_context(prefix[0]));
        let legal = legal_plays(self.decl, hands[seat.index()], led);
        let mut tiles = [Domino::ALL[0]; 4];
        tiles[..k].copy_from_slice(prefix);
        let mut out = Vec::new();
        for d in legal.iter() {
            let mut hands = hands;
            hands[seat.index()].remove(d);
            let mut tiles = tiles;
            tiles[k] = d;
            out.push((d, self.ply(hands, leader, tiles, k + 1, i64::MIN, i64::MAX)));
        }
        out
    }

    /// Value of the subgame that starts a fresh trick: the leader is free.
    /// Always solved over the full window, so the cached value is the exact
    /// minimax value of the projected state, independent of any caller's
    /// pruning context.
    fn boundary(&mut self, hands: [DominoSet; Seat::COUNT], leader: Seat) -> i64 {
        if hands[leader.index()].is_empty() {
            return 0;
        }
        let k = key(&hands, leader);
        if let Some(&v) = self.cache.get(&k) {
            return v;
        }
        let v = self.ply(hands, leader, [Domino::ALL[0]; 4], 0, i64::MIN, i64::MAX);
        self.cache.insert(k, v);
        v
    }

    /// Alpha-beta minimax over the remainder of the current trick. The window
    /// never crosses a trick boundary (`boundary` restarts it full-width), so
    /// pruning is exact at every window root and never taints the cache.
    fn ply(
        &mut self,
        hands: [DominoSet; Seat::COUNT],
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        mut alpha: i64,
        mut beta: i64,
    ) -> i64 {
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
            return inc + self.boundary(hands, winner);
        }
        let seat = leader.plus(k);
        let led = (k > 0).then(|| self.decl.led_context(tiles[0]));
        let legal = legal_plays(self.decl, hands[seat.index()], led);
        let maximizing = seat.team() == self.focal;
        let mut best: Option<i64> = None;
        for d in legal.iter() {
            let mut hands = hands;
            hands[seat.index()].remove(d);
            let mut tiles = tiles;
            tiles[k] = d;
            let v = self.ply(hands, leader, tiles, k + 1, alpha, beta);
            best = Some(match best {
                None => v,
                Some(b) if maximizing => b.max(v),
                Some(b) => b.min(v),
            });
            let b = best.expect("just set");
            if maximizing {
                alpha = alpha.max(b);
            } else {
                beta = beta.min(b);
            }
            if beta <= alpha {
                break;
            }
        }
        best.expect("a seat holding tiles has a legal play")
    }
}

/// The scalar census: worlds partitioned by exact root value vector and by
/// optimal-root-action set (a bitmask in action order). Derived views of the
/// world set and valuation, never an authority.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ScalarCensus {
    pub worlds: usize,
    /// The root actions, ascending: the viewer's hand.
    pub actions: Vec<Domino>,
    pub value_classes: BTreeMap<Vec<i64>, usize>,
    pub action_classes: BTreeMap<u32, usize>,
}

impl ScalarCensus {
    pub fn argmax_mask(values: &[i64]) -> u32 {
        let best = values.iter().max().expect("at least one root");
        let mut mask = 0u32;
        for (i, v) in values.iter().enumerate() {
            if v == best {
                mask |= 1 << i;
            }
        }
        mask
    }
}

/// Census over an explicit world list (each world a full four-seat deal of
/// the kernel: viewer hand fixed, hidden pool split). The focal side is the
/// viewer's team, as in the probe suite.
pub fn scalar_census<I>(kernel: &Kernel, val: ScalarValuation, worlds: I) -> ScalarCensus
where
    I: IntoIterator<Item = [DominoSet; Seat::COUNT]>,
{
    let actions: Vec<Domino> = kernel.viewer_hand().iter().collect();
    let mut pi = ScalarPi::new(kernel.decl(), kernel.viewer().team(), val);
    let mut census = ScalarCensus {
        worlds: 0,
        actions: actions.clone(),
        value_classes: BTreeMap::new(),
        action_classes: BTreeMap::new(),
    };
    for hands in worlds {
        let solved = pi.root_values(hands, kernel.viewer());
        let (roots, values): (Vec<Domino>, Vec<i64>) = solved.into_iter().unzip();
        assert_eq!(roots, actions, "the root actions are the viewer's hand");
        census.worlds += 1;
        *census
            .action_classes
            .entry(ScalarCensus::argmax_mask(&values))
            .or_insert(0) += 1;
        *census.value_classes.entry(values).or_insert(0) += 1;
    }
    census
}

/// Census over the kernel's whole void-constrained fiber.
pub fn scalar_fiber_census(kernel: &Kernel, val: ScalarValuation) -> ScalarCensus {
    scalar_census(kernel, val, kernel.worlds().map(|w| w.hands()))
}
