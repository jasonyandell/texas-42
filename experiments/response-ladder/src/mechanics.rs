//! Canonical game mechanics shared by the controller and rollout backends.
use serde::{Deserialize, Serialize};
use std::time::Duration;
use walt::rules::{Decl, Domino, Seat};
use walt::rules::rules::{legal_plays, Trick};
use walt::solver::{set_of, Deadline, Key, SplitMix64, record_hash, FULL_MASK};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PublicState {
    pub played: u32,
    pub leader: u8,
    pub plays: Vec<u8>,
    pub banked_t1: u8,
    pub banked_t0: u8,
    pub voids: [u32; 4],
    /// Complete chronological public tile history. The fixed root is part of
    /// the problem identity; history must never contain plan/scenario IDs.
    pub history: Vec<u8>,
}

impl PublicState {
    pub fn opening(leader: u8) -> Self {
        Self { played: 0, leader, plays: vec![], banked_t1: 0,
            banked_t0: 0, voids: [0; 4], history: vec![] }
    }
    pub fn actor(&self) -> u8 { (self.leader + self.plays.len() as u8) % 4 }
    pub fn key(&self) -> Key {
        Key { voids: None, played: self.played, leader: self.leader,
            plays: self.plays.clone(), banked_t1: self.banked_t1,
            banked_t0: self.banked_t0, alive: 0 }
    }
    pub fn legal(&self, decl: Decl, hand: u32) -> u32 {
        let led = self.plays.first().map(|&t| decl.led_context(domino(t)));
        legal_plays(decl, set_of(hand & !self.played), led).bits()
    }
    pub fn after(&self, decl: Decl, tile: u8) -> Self {
        assert!(tile < 28 && self.played & (1 << tile) == 0);
        let mut next = self.clone();
        let dm = domino(tile);
        if let Some(&led) = self.plays.first() {
            let context = decl.led_context(domino(led));
            if !decl.follows(dm, context) {
                next.voids[self.actor() as usize] |= decl.effective_incidence(context).bits();
            }
        }
        next.played |= 1 << tile;
        next.history.push(tile);
        next.plays.push(tile);
        if next.plays.len() == 4 {
            let trick = Trick::new(Seat::from_index(self.leader as usize).unwrap(),
                std::array::from_fn(|i| domino(next.plays[i]))).unwrap();
            let winner = trick.winner(decl).index() as u8;
            if winner % 2 == 1 { next.banked_t1 += trick.points() as u8; }
            else { next.banked_t0 += trick.points() as u8; }
            next.leader = winner;
            next.plays.clear();
        }
        next
    }
    /// Contract is normalized so seats 1 and 3 are the bidding team.
    pub fn payoff(&self, bid: u8, viewer: u8) -> Option<u64> {
        let made = if self.banked_t1 >= bid { true }
            else if self.banked_t0 > 42 - bid { false }
            else { return None };
        Some(u64::from(made == (viewer % 2 == 1)))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scenario {
    /// Remaining hands at the fixed root; never passed to a modeled mind.
    pub hands: [u32; 4],
    pub tape: u64,
    pub weight: u64,
}

#[derive(Clone, Debug)]
pub struct Problem {
    pub decl: Decl,
    pub bid: u8,
    pub viewer: u8,
    pub hand: u32,
    pub root: PublicState,
    /// Array position is original sample identity, including duplicate deals.
    pub scenarios: Vec<Scenario>,
    pub field_revision: String,
}
impl Problem {
    pub fn mass(&self) -> u64 { self.scenarios.iter().map(|s| s.weight).sum() }
    pub fn validate(&self) -> Result<(), String> {
        if !(1..=42).contains(&self.bid) || self.viewer >= 4 || self.root.leader >= 4
            || self.root.plays.len() > 3 || self.root.actor() != self.viewer {
            return Err("invalid contract or root actor".into());
        }
        if u16::from(self.root.banked_t1) + u16::from(self.root.banked_t0) > 42 {
            return Err("invalid banked score".into());
        }
        let (_, _, sizes) = crate::model::frame(&self.root)?;
        if self.hand == 0 || self.scenarios.is_empty() || self.root.played & !FULL_MASK != 0 {
            return Err("empty hand/bundle or invalid tiles".into());
        }
        let mut total = 0u64;
        for s in &self.scenarios {
            total = total.checked_add(s.weight).ok_or("weight overflow")?;
            if s.weight == 0 || s.hands[self.viewer as usize] != self.hand { return Err("invalid scenario mass or viewer hand".into()); }
            let mut union = self.root.played;
            for (seat, &h) in s.hands.iter().enumerate() {
                if h & union != 0 { return Err("overlapping hands/played tiles".into()); }
                if h.count_ones() as usize != sizes[seat] {
                    return Err("scenario capacities disagree with public frame".into());
                }
                union |= h;
            }
            if union != FULL_MASK { return Err("scenario does not conserve 28 tiles".into()); }
        }
        Ok(())
    }
}

/// A field receives only its own hand, public information, and the declared
/// latent Dice tape. Modeled levels must ignore the tape entirely.
pub struct FieldQuery<'a> {
    pub decl: Decl,
    pub bid: u8,
    pub seat: u8,
    pub hand: u32,
    pub public: &'a PublicState,
    pub tape: u64,
}
#[derive(Clone, Debug)]
pub struct OwnedFieldQuery {
    pub decl: Decl, pub bid: u8, pub seat: u8, pub hand: u32,
    pub public: PublicState, pub tape: u64,
}
impl OwnedFieldQuery {
    pub fn borrowed(&self) -> FieldQuery<'_> { FieldQuery { decl:self.decl, bid:self.bid,
        seat:self.seat, hand:self.hand, public:&self.public, tape:self.tape } }
}
pub trait Field {
    fn revision(&self) -> &str;
    fn choose(&mut self, query: FieldQuery<'_>, budget: &mut Budget) -> Option<u8>;
    /// Optional immutable four-seat compiled actor table for direct GPU field
    /// execution. `None` retains the ordinary CPU field-query epoch path.
    fn compiled_actors(&self) -> Option<&[crate::compiled::Actor; 4]> { None }
    fn choose_batch(&mut self, queries: &[OwnedFieldQuery], budget: &mut Budget) -> Option<Vec<u8>> {
        queries.iter().map(|q| self.choose(q.borrowed(), budget)).collect()
    }
    /// Distinct modeled identities submitted by the last batch, when known.
    /// Generic history/tape queries and modeled identities are different counts.
    fn last_batch_modeled_queries(&self) -> Option<usize> { None }
}

pub struct Budget {
    pub limit: u64,
    pub used: u64,
    pub deadline: Deadline,
}
impl Budget {
    pub fn new(limit: u64, duration: Duration) -> Self {
        Self { limit, used: 0, deadline: Deadline::after(duration) }
    }
    pub fn tick(&mut self) -> Option<()> {
        if self.exhausted() { return None; }
        self.used += 1;
        Some(())
    }
    pub fn exhausted(&self) -> bool { self.used >= self.limit || self.deadline.passed() }
}

#[derive(Default)]
pub struct DiceField;
impl Field for DiceField {
    fn revision(&self) -> &str { "historical-dice-v1" }
    fn choose(&mut self, q: FieldQuery<'_>, budget: &mut Budget) -> Option<u8> {
        budget.tick()?;
        let legal = q.public.legal(q.decl, q.hand);
        if legal == 0 { return None; }
        if legal.count_ones() == 1 { return Some(legal.trailing_zeros() as u8); }
        let index = SplitMix64(q.tape ^ record_hash(&q.public.key())).below(legal.count_ones() as u64);
        Some(tiles(legal)[index as usize])
    }
}

pub fn domino(tile: u8) -> Domino { Domino::from_index(tile as usize).expect("tile 0..27") }
pub fn tiles(mut mask: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity(mask.count_ones() as usize);
    while mask != 0 { out.push(mask.trailing_zeros() as u8); mask &= mask - 1; }
    out
}
