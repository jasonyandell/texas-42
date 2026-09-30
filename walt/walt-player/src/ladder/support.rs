use super::{
    rules::{Public, Rules},
    Error, Mask, World,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
thread_local! {static PATTERNS:RefCell<HashMap<u32,Rc<Vec<u16>>>>=RefCell::new(HashMap::new());}
// Abstract assignment patterns only: no tile identities, own hands or deals.
pub(super) struct Support {
    actor: usize,
    count: usize,
    tiles: [usize; 21],
    allowed: [u8; 21],
    seats: [usize; 3],
    caps: [usize; 3],
    dp: Box<[[[u64; 8]; 8]; 22]>,
    patterns: Option<Rc<Vec<u16>>>,
}
impl Support {
    pub fn new(r: &Rules, p: Public, hand: Mask) -> Result<Self, Error> {
        let actor = r.turn(p);
        let sizes = r.sizes(p);
        if hand & p.played != 0
            || hand & p.voids[actor] != 0
            || hand.count_ones() as usize != sizes[actor]
        {
            return Err(Error::Invalid("own hand contradicts public state"));
        }
        let mut s = Self {
            actor,
            count: 0,
            tiles: [0; 21],
            allowed: [0; 21],
            seats: [0; 3],
            caps: [0; 3],
            dp: Box::new([[[0; 8]; 8]; 22]),
            patterns: None,
        };
        let mut at = 0;
        for (seat, &size) in sizes.iter().enumerate() {
            if seat != actor {
                s.seats[at] = seat;
                s.caps[at] = size;
                at += 1;
            }
        }
        for t in 0..28 {
            if (p.played | hand) & (1 << t) == 0 {
                if s.count == 21 {
                    return Err(Error::Invalid("invalid unseen tiles"));
                }
                s.tiles[s.count] = t;
                for j in 0..3 {
                    if p.voids[s.seats[j]] & (1 << t) == 0 {
                        s.allowed[s.count] |= 1 << j;
                    }
                }
                s.count += 1;
            }
        }
        if s.count != s.caps.iter().sum::<usize>() {
            return Err(Error::Invalid("invalid capacities"));
        }
        if s.count <= 6 && s.caps.iter().all(|&c| c <= 2) {
            let mut key = ((s.caps[0] | s.caps[1] << 2 | s.caps[2] << 4) as u32) << 18;
            for i in 0..s.count {
                key |= (s.allowed[i] as u32) << (3 * i);
            }
            s.patterns = Some(PATTERNS.with(|table| {
                let mut table = table.borrow_mut();
                if let Some(value) = table.get(&key) {
                    return value.clone();
                }
                fn walk(allowed: &[u8], caps: [usize; 3], code: u16, i: usize, out: &mut Vec<u16>) {
                    if i == allowed.len() {
                        out.push(code);
                        return;
                    }
                    for j in 0..3 {
                        if caps[j] > 0 && allowed[i] & (1 << j) != 0 {
                            let mut next = caps;
                            next[j] -= 1;
                            walk(allowed, next, code | ((j as u16) << (2 * i)), i + 1, out);
                        }
                    }
                }
                let mut values = Vec::new();
                walk(&s.allowed[..s.count], s.caps, 0, 0, &mut values);
                let value = Rc::new(values);
                if table.len() < 8192 {
                    table.insert(key, value.clone());
                }
                value
            }));
        } else {
            s.dp[s.count][0][0] = 1;
            for i in (0..s.count).rev() {
                for a in 0..=s.caps[0] {
                    for b in 0..=s.caps[1] {
                        if a + b > s.count - i {
                            continue;
                        }
                        let c = s.count - i - a - b;
                        if c > s.caps[2] {
                            continue;
                        }
                        if a > 0 && s.allowed[i] & 1 != 0 {
                            s.dp[i][a][b] += s.dp[i + 1][a - 1][b];
                        }
                        if b > 0 && s.allowed[i] & 2 != 0 {
                            s.dp[i][a][b] += s.dp[i + 1][a][b - 1];
                        }
                        if c > 0 && s.allowed[i] & 4 != 0 {
                            s.dp[i][a][b] += s.dp[i + 1][a][b];
                        }
                    }
                }
            }
        }
        if s.total() == 0 {
            Err(Error::Invalid("no consistent hidden deals"))
        } else {
            Ok(s)
        }
    }
    pub fn total(&self) -> u64 {
        self.patterns
            .as_ref()
            .map_or(self.dp[0][self.caps[0]][self.caps[1]], |p| p.len() as u64)
    }
    pub fn unrank(&self, hand: Mask, mut rank: u64) -> World {
        let mut w = [0; 4];
        w[self.actor] = hand;
        if let Some(patterns) = &self.patterns {
            let code = patterns[rank as usize];
            for i in 0..self.count {
                w[self.seats[((code >> (2 * i)) & 3) as usize]] |= 1 << self.tiles[i];
            }
            return w;
        }
        let (mut a, mut b) = (self.caps[0], self.caps[1]);
        for i in 0..self.count {
            let wa = if a > 0 && self.allowed[i] & 1 != 0 {
                self.dp[i + 1][a - 1][b]
            } else {
                0
            };
            let wb = if b > 0 && self.allowed[i] & 2 != 0 {
                self.dp[i + 1][a][b - 1]
            } else {
                0
            };
            let seat = if rank < wa {
                a -= 1;
                self.seats[0]
            } else if rank < wa + wb {
                rank -= wa;
                b -= 1;
                self.seats[1]
            } else {
                rank -= wa + wb;
                self.seats[2]
            };
            w[seat] |= 1 << self.tiles[i];
        }
        w
    }
}
