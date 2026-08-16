//! The declaration-relative rule algebra: called sets, effective incidence,
//! follow, tier, rank, trick key, BEATS/THREAT, trick resolution, legality
//! (v0.4 §1.2--§1.5).

use crate::context::Context;
use crate::decl::Decl;
use crate::domino::Domino;
use crate::pip::Pip;
use crate::seat::Seat;
use crate::set::DominoSet;

const fn natural_set(p: Pip) -> DominoSet {
    let mut out = DominoSet::EMPTY;
    let mut i = 0;
    while i < Domino::COUNT {
        let d = Domino::ALL[i];
        if d.has(p) {
            out = out.union(DominoSet::single(d));
        }
        i += 1;
    }
    out
}

const fn doubles_set() -> DominoSet {
    let mut out = DominoSet::EMPTY;
    let mut i = 0;
    while i < Domino::COUNT {
        let d = Domino::ALL[i];
        if d.is_double() {
            out = out.union(DominoSet::single(d));
        }
        i += 1;
    }
    out
}

/// `sigma_p`, the natural incidence set of pip `p` (v0.4 §1.1).
pub const NATURAL: [DominoSet; Pip::COUNT] = [
    natural_set(Pip::ALL[0]),
    natural_set(Pip::ALL[1]),
    natural_set(Pip::ALL[2]),
    natural_set(Pip::ALL[3]),
    natural_set(Pip::ALL[4]),
    natural_set(Pip::ALL[5]),
    natural_set(Pip::ALL[6]),
];

/// `D^o`, the set of doubles.
pub const DOUBLES: DominoSet = doubles_set();

/// Rank sentinel for a natural double: strictly above every mixed pip sum,
/// whose maximum is `6 + 5 = 11`.
const DOUBLE_TOP: u8 = 12;

/// Declaration-relative rank `r_delta(d)` (v0.4 §1.3). Ranks are only ever
/// compared inside one nonzero tier, where they are injective.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Rank(u8);

impl Rank {
    pub const fn value(self) -> u8 {
        self.0
    }
}

/// `tier_delta(d, q)`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Tier {
    Slough = 0,
    Follows = 1,
    Called = 2,
}

/// The lexicographic trick key `tau_delta(d, q) = (tier, rank)`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TrickKey {
    pub tier: Tier,
    pub rank: Rank,
}

impl Decl {
    /// The called set `kappa_delta`. In Straight 42 every nonempty called set
    /// is powered, so `pi_delta = kappa_delta`.
    pub const fn called_set(self) -> DominoSet {
        match self {
            Decl::PipTrump(p) => NATURAL[p.value() as usize],
            Decl::DoublesTrump => DOUBLES,
            Decl::NoTrump => DominoSet::EMPTY,
        }
    }

    pub const fn is_called(self, d: Domino) -> bool {
        self.called_set().contains(d)
    }

    /// The effective incidence family: `sigma^_p = sigma_p \ kappa` and
    /// `sigma^_called = kappa`. Called tiles are absorbed out of every
    /// natural context.
    pub const fn effective_incidence(self, q: Context) -> DominoSet {
        match q {
            Context::Called => self.called_set(),
            Context::Natural(p) => NATURAL[p.value() as usize].difference(self.called_set()),
        }
    }

    /// The follow predicate `F_delta(d, q)`.
    pub const fn follows(self, d: Domino, q: Context) -> bool {
        self.effective_incidence(q).contains(d)
    }

    /// The led context `l_delta(d)`: called tiles lead the called suit,
    /// everything else leads its high pip.
    pub const fn led_context(self, d: Domino) -> Context {
        if self.is_called(d) {
            Context::Called
        } else {
            Context::Natural(d.hi())
        }
    }

    pub const fn tier(self, d: Domino, led: Context) -> Tier {
        if self.is_called(d) {
            Tier::Called
        } else if self.follows(d, led) {
            Tier::Follows
        } else {
            Tier::Slough
        }
    }

    pub const fn rank(self, d: Domino) -> Rank {
        if d.is_double() {
            match self {
                // Under doubles-trump the doubles are the called suit and are
                // ranked by pip; under every other declaration a natural
                // double is top of its effective natural suit.
                Decl::DoublesTrump => Rank(d.hi().value()),
                _ => Rank(DOUBLE_TOP),
            }
        } else {
            Rank(d.pip_sum())
        }
    }

    pub const fn trick_key(self, d: Domino, led: Context) -> TrickKey {
        TrickKey {
            tier: self.tier(d, led),
            rank: self.rank(d),
        }
    }

    /// `BEATS_delta(q, d)`.
    pub fn beats(self, led: Context, d: Domino) -> DominoSet {
        let key = self.trick_key(d, led);
        Domino::ALL
            .into_iter()
            .filter(|e| self.trick_key(*e, led) > key)
            .collect()
    }

    /// The when-led threat set `THREAT_delta(d) = BEATS_delta(l_delta(d), d)`.
    pub fn threat(self, d: Domino) -> DominoSet {
        self.beats(self.led_context(d), d)
    }
}

/// Four plays in actor order starting from the leader. Distinctness of the
/// four tiles and the seat rotation are established by construction.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Trick {
    leader: Seat,
    dominoes: [Domino; 4],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RepeatedDomino(pub Domino);

impl Trick {
    pub fn new(leader: Seat, dominoes: [Domino; 4]) -> Result<Trick, RepeatedDomino> {
        for i in 1..4 {
            for j in 0..i {
                if dominoes[i] == dominoes[j] {
                    return Err(RepeatedDomino(dominoes[i]));
                }
            }
        }
        Ok(Trick { leader, dominoes })
    }

    pub const fn leader(self) -> Seat {
        self.leader
    }

    pub const fn dominoes(self) -> [Domino; 4] {
        self.dominoes
    }

    pub fn actor(self, k: usize) -> Seat {
        self.leader.plus(k)
    }

    pub fn plays(self) -> [(Seat, Domino); 4] {
        [
            (self.actor(0), self.dominoes[0]),
            (self.actor(1), self.dominoes[1]),
            (self.actor(2), self.dominoes[2]),
            (self.actor(3), self.dominoes[3]),
        ]
    }

    pub const fn led(self, decl: Decl) -> Context {
        decl.led_context(self.dominoes[0])
    }

    /// The unique maximum trick key wins (v0.4 §1.3).
    pub fn winner(self, decl: Decl) -> Seat {
        let led = self.led(decl);
        let mut best = decl.trick_key(self.dominoes[0], led);
        let mut at = 0;
        for k in 1..4 {
            let key = decl.trick_key(self.dominoes[k], led);
            if key > best {
                best = key;
                at = k;
            }
        }
        self.actor(at)
    }

    /// One trick point plus the count labels on the four tiles (v0.4 §1.4).
    pub fn points(self) -> u32 {
        1 + self.dominoes.iter().map(|d| d.count()).sum::<u32>()
    }
}

/// The follower's legal set: a member of the led effective context when able,
/// anything otherwise. A leader (`led == None`) may play any remaining tile.
pub fn legal_plays(decl: Decl, hand: DominoSet, led: Option<Context>) -> DominoSet {
    match led {
        None => hand,
        Some(q) => {
            let follows = hand.intersection(decl.effective_incidence(q));
            if follows.is_empty() {
                hand
            } else {
                follows
            }
        }
    }
}
