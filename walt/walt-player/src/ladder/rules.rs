use super::{Key, Mask};

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Public {
    pub played: Mask,
    pub leader: usize,
    pub len: usize,
    pub t1: u8,
    pub t0: u8,
    pub depth: usize,
    pub trick: [usize; 4],
    pub voids: [Mask; 4],
    pub history: [u64; 4],
}
impl Public {
    pub fn initial() -> Self {
        Self {
            leader: 1,
            ..Self::default()
        }
    }
    pub fn key(self, level: usize, hand: Mask) -> Key {
        let mut k = [0; 13];
        k[0] = hand as u64;
        k[1] = self.played as u64;
        k[2] = self.leader as u64
            | (self.len as u64) << 3
            | (self.depth as u64) << 6
            | (self.t1 as u64) << 12
            | (self.t0 as u64) << 18;
        for i in 0..4 {
            k[3] |= (self.trick[i] as u64) << (5 * i);
            k[4 + i] = self.voids[i] as u64;
            k[8 + i] = self.history[i];
        }
        k[12] = level as u64;
        k
    }
}
#[derive(Clone)]
pub(super) struct Rules {
    pub suit: [Mask; 8],
    pub lead: [usize; 28],
    pub points: [u8; 28],
    pub strength: [[u8; 28]; 8],
    pub bid: u8,
    pub nello: bool,
}
impl Rules {
    pub fn new(decl: usize, bid: u8, nello: bool) -> Self {
        let mut r = Self {
            suit: [0; 8],
            lead: [0; 28],
            points: [0; 28],
            strength: [[0; 28]; 8],
            bid,
            nello,
        };
        let mut tiles = [(0, 0); 28];
        let mut at = 0;
        let mut called = 0;
        for h in 0..7 {
            for l in 0..=h {
                tiles[at] = (h, l);
                if (decl <= 6 && (h == decl || l == decl)) || ((decl == 7 || decl == 8) && h == l) {
                    called |= 1 << at;
                }
                at += 1;
            }
        }
        r.suit[7] = called;
        for (t, &(h, l)) in tiles.iter().enumerate() {
            for q in 0..7 {
                if (h == q || l == q) && called & (1 << t) == 0 {
                    r.suit[q] |= 1 << t;
                }
            }
            r.lead[t] = if called & (1 << t) != 0 { 7 } else { h };
            r.points[t] = if h + l == 5 || h + l == 10 {
                (h + l) as u8
            } else {
                0
            };
        }
        for (t, &(h, l)) in tiles.iter().enumerate() {
            for q in 0..8 {
                let tier = if !nello && called & (1 << t) != 0 {
                    2
                } else if r.suit[q] & (1 << t) != 0 {
                    1
                } else {
                    0
                };
                let rank = if h == l {
                    if decl == 7 || decl == 8 {
                        h
                    } else {
                        12
                    }
                } else {
                    h + l
                };
                r.strength[q][t] = (20 * tier + rank) as u8;
            }
        }
        r
    }
    #[inline]
    pub fn actor(&self, leader: usize, len: usize) -> usize {
        if !self.nello {
            (leader + len) % 4
        } else {
            let mut s = leader;
            for _ in 0..len {
                s = (s + 1) % 4;
                if s == 3 {
                    s = 0;
                }
            }
            s
        }
    }
    #[inline]
    pub fn turn(&self, p: Public) -> usize {
        self.actor(p.leader, p.len)
    }
    #[inline]
    pub fn trick_size(&self) -> usize {
        if self.nello {
            3
        } else {
            4
        }
    }
    #[inline]
    pub fn outcome(&self, p: Public) -> Option<bool> {
        if self.nello {
            if p.t1 > 0 {
                Some(false)
            } else if p.depth == 21 {
                Some(true)
            } else {
                None
            }
        } else if p.t1 >= self.bid {
            Some(true)
        } else if p.t0 > 42 - self.bid {
            Some(false)
        } else {
            None
        }
    }
    pub fn sizes(&self, p: Public) -> [usize; 4] {
        let mut sizes = [7 - p.depth / self.trick_size(); 4];
        if self.nello {
            sizes[3] = 7;
        }
        for i in 0..p.len {
            sizes[self.actor(p.leader, i)] -= 1;
        }
        sizes
    }
    #[inline]
    pub fn legal(&self, hand: Mask, p: Public) -> Mask {
        if p.len > 0 {
            let follow = hand & self.suit[self.lead[p.trick[0]]];
            if follow != 0 {
                return follow;
            }
        }
        hand
    }
    pub fn play(&self, p: Public, t: usize) -> Public {
        let mut q = p;
        let actor = self.turn(p);
        if p.len > 0 && self.suit[self.lead[p.trick[0]]] & (1 << t) == 0 {
            q.voids[actor] |= self.suit[self.lead[p.trick[0]]];
        }
        q.history[p.depth / 9] |= ((actor * 32 + t) as u64) << (7 * (p.depth % 9));
        q.depth += 1;
        q.played |= 1 << t;
        q.trick[q.len] = t;
        q.len += 1;
        if q.len == self.trick_size() {
            let led = self.lead[q.trick[0]];
            let mut best = 0;
            let mut won = 1;
            for i in 0..q.len {
                won += self.points[q.trick[i]];
                if self.strength[led][q.trick[i]] > self.strength[led][q.trick[best]] {
                    best = i;
                }
            }
            let winner = self.actor(q.leader, best);
            if winner % 2 == 1 {
                q.t1 += won;
            } else {
                q.t0 += won;
            }
            q.leader = winner;
            q.len = 0;
            q.trick = [0; 4];
        }
        q
    }
}
