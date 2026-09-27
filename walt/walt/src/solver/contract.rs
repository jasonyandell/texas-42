//! Play objectives and active turns. Doubles-suit itself is a declaration;
//! sitting out and losing every trick belong to the Nel-O contract.
use super::{bit, inner_belief, Key};
use crate::rules::{Decl, Domino, Seat, Team};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Contract {
    Straight {
        bid: u8,
    },
    /// Solver coordinates place the declarer on T1. The partner stays hidden.
    Nello {
        declarer: Seat,
    },
}

impl Contract {
    pub fn is_nello(self) -> bool {
        matches!(self, Self::Nello { .. })
    }
    pub fn inactive(self) -> Option<Seat> {
        match self {
            Self::Straight { .. } => None,
            Self::Nello { declarer } => Some(declarer.plus(2)),
        }
    }
    pub fn trick_size(self) -> usize {
        if self.is_nello() {
            3
        } else {
            4
        }
    }
    pub fn actor(self, leader: usize, offset: usize) -> Seat {
        let mut seat = Seat::from_index(leader).expect("leader is a physical seat");
        assert_ne!(Some(seat), self.inactive(), "inactive seat cannot lead");
        for _ in 0..offset {
            seat = seat.successor();
            if Some(seat) == self.inactive() {
                seat = seat.successor();
            }
        }
        seat
    }
    /// Every nonterminal Nel-O history has no declarer wins. The previous
    /// trick's winner is its next leader, so its first declarer win is an
    /// absorbing failure without using count or a synthetic points target.
    pub fn terminal(self, key: &Key) -> Option<bool> {
        match self {
            Self::Straight { bid } => {
                if key.banked_t1 >= bid {
                    Some(true)
                } else if key.banked_t0 > 42 - bid {
                    Some(false)
                } else {
                    None
                }
            }
            Self::Nello { declarer } => {
                let completed = (key.played.count_ones() as usize - key.plays.len()) / 3;
                if completed > 0 && key.leader as usize == declarer.index() {
                    Some(false)
                } else if completed == 7 {
                    Some(true)
                } else {
                    None
                }
            }
        }
    }
    pub fn sizes(self, key: &Key, boundary_played: u32, boundary_size: usize) -> [usize; 4] {
        let n = key.played.count_ones() as usize
            - boundary_played.count_ones() as usize
            - key.plays.len();
        assert_eq!(n % self.trick_size(), 0, "completed tricks are whole");
        let mut sizes = [boundary_size - n / self.trick_size(); 4];
        if let Some(seat) = self.inactive() {
            sizes[seat.index()] = 7;
        }
        for i in 0..key.plays.len() {
            sizes[self.actor(key.leader as usize, i).index()] -= 1;
        }
        sizes
    }
    pub fn winner(self, decl: Decl, leader: usize, plays: &[u8]) -> Seat {
        assert_eq!(plays.len(), self.trick_size());
        let tile = |id: u8| Domino::from_index(id as usize).expect("tile");
        let led = decl.led_context(tile(plays[0]));
        let mut best = 0;
        for i in 1..plays.len() {
            if decl.trick_key(tile(plays[i]), led) > decl.trick_key(tile(plays[best]), led) {
                best = i;
            }
        }
        self.actor(leader, best)
    }
    pub fn step(self, key: &mut Key, decl: Decl, tile: Domino) {
        let actor = self.actor(key.leader as usize, key.plays.len());
        key.voids = inner_belief::after_play_at(key, decl, tile, actor.index());
        key.played |= bit(tile);
        key.plays.push(tile.index() as u8);
        if key.plays.len() == self.trick_size() {
            let winner = self.winner(decl, key.leader as usize, &key.plays);
            let points = 1 + key
                .plays
                .iter()
                .map(|&t| Domino::from_index(t as usize).unwrap().count() as u8)
                .sum::<u8>();
            if winner.team() == Team::T1 {
                key.banked_t1 += points;
            } else {
                key.banked_t0 += points;
            }
            key.leader = winner.index() as u8;
            key.plays.clear();
        }
    }
}
