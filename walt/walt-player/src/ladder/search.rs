use super::{
    key_hash, mix,
    rules::{Public, Rules},
    support::Support,
    Error, Key, Mask, Random, World,
};
use std::{
    collections::HashMap,
    hash::{BuildHasherDefault, Hasher},
};
use walt::clock::Instant;

#[derive(Default)]
struct FastHash(u64);
impl Hasher for FastHash {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = mix(self.0 ^ u64::from(b));
        }
    }
    fn write_u64(&mut self, x: u64) {
        self.0 = mix(self.0 ^ x);
    }
    fn write_usize(&mut self, x: usize) {
        self.write_u64(x as u64);
    }
}
#[derive(Clone, Copy)]
struct Choice {
    tile: usize,
}
struct Frame {
    children: [Vec<usize>; 28],
    choices: Vec<(Mask, usize)>,
    used: Mask,
}
impl Default for Frame {
    fn default() -> Self {
        Self {
            children: std::array::from_fn(|_| Vec::new()),
            choices: Vec::new(),
            used: 0,
        }
    }
}
#[derive(Clone, Copy)]
struct Lite {
    played: Mask,
    leader: usize,
    len: usize,
    t1: u8,
    t0: u8,
    suit: usize,
    rank: u8,
    winner: usize,
    points: u8,
}
#[derive(Clone, Copy)]
pub(super) struct Value {
    pub count: i32,
    pub exact: bool,
}
impl Value {
    fn exact(count: i32) -> Self {
        Self { count, exact: true }
    }
    fn bound(count: i32) -> Self {
        Self {
            count,
            exact: false,
        }
    }
}
pub(super) struct Search {
    pub rules: Rules,
    pub samples: Vec<usize>,
    base: u64,
    deal_salts: Vec<u64>,
    roll_salts: Vec<u64>,
    frames: Vec<Option<Box<Frame>>>,
    policies: HashMap<Key, Choice, BuildHasherDefault<FastHash>>,
    deadline: Instant,
    ticks: u64,
    pub nodes: u64,
    pub pruned: u64,
}
impl Search {
    pub fn new(
        rules: Rules,
        decl: usize,
        samples: Vec<usize>,
        seed: u64,
        deadline: Instant,
    ) -> Self {
        // 1.0's IEEE-754 bits retain the measured C++ addressed-field identity.
        let mut base = mix(seed) ^ mix(decl as u64) ^ mix(0x3ff0000000000000);
        if rules.nello {
            base ^= mix(0x6e656c6c6f);
        } else if rules.bid != 30 {
            base ^= mix(rules.bid as u64);
        }
        let (mut ds, mut rs) = (0x73616d706c652d31, 0x726f6c6c6f757431);
        let mut deal_salts = Vec::new();
        let mut roll_salts = Vec::new();
        for &n in &samples {
            ds = mix(ds ^ n as u64);
            rs = mix(rs ^ n as u64);
            deal_salts.push(ds);
            roll_salts.push(rs);
        }
        let frames = (0..samples.len() * 29)
            .map(|_| Some(Box::default()))
            .collect();
        Self {
            rules,
            samples,
            base,
            deal_salts,
            roll_salts,
            frames,
            policies: HashMap::default(),
            deadline,
            ticks: 0,
            nodes: 0,
            pruned: 0,
        }
    }
    pub fn check(&self) -> Result<(), Error> {
        if Instant::now() >= self.deadline {
            Err(Error::Timeout)
        } else {
            Ok(())
        }
    }
    fn tick(&mut self) -> Result<(), Error> {
        self.ticks += 1;
        self.nodes += 1;
        if self.ticks & 1023 == 0 {
            self.check()?;
        }
        Ok(())
    }
    pub fn path(&self, level: usize, p: Public, hand: Mask) -> u64 {
        mix(key_hash(&p.key(level, hand)) ^ self.base ^ self.roll_salts[level - 1])
    }
    pub fn deals(&self, level: usize, p: Public, hand: Mask) -> Result<Vec<World>, Error> {
        let support = Support::new(&self.rules, p, hand)?;
        let mut rng = Random(mix(key_hash(&p.key(level, hand))
            ^ self.base
            ^ self.deal_salts[level - 1]));
        Ok((0..self.samples[level - 1])
            .map(|_| support.unrank(hand, rng.bounded(support.total())))
            .collect())
    }
    #[cfg(test)]
    pub(super) fn action_for_test(
        &mut self,
        level: usize,
        p: Public,
        hand: Mask,
    ) -> Result<usize, Error> {
        self.action(level, p, hand)
    }
    fn action(&mut self, level: usize, p: Public, hand: Mask) -> Result<usize, Error> {
        self.check()?;
        let legal = self.rules.legal(hand, p);
        if legal.count_ones() == 1 {
            return Ok(legal.trailing_zeros() as usize);
        }
        let key = p.key(level, hand);
        if let Some(result) = self.policies.get(&key) {
            return Ok(result.tile);
        }
        let worlds = self.deals(level, p, hand)?;
        let fibers = (0..worlds.len()).collect::<Vec<_>>();
        let me = self.rules.turn(p);
        let maximize = me % 2 == 1;
        let path = self.path(level, p, hand);
        let mut best = if maximize {
            -1
        } else {
            worlds.len() as i32 + 1
        };
        let mut choice = 28;
        let mut moves = legal;
        while moves != 0 {
            let t = moves.trailing_zeros() as usize;
            moves &= moves - 1;
            let v = self.search(
                level,
                self.rules.play(p, t),
                me,
                &worlds,
                &fibers,
                child_path(path, t),
                Some(best),
            )?;
            if v.exact
                && (choice == 28
                    || (if maximize {
                        v.count > best
                    } else {
                        v.count < best
                    }))
            {
                best = v.count;
                choice = t;
            }
            if best == if maximize { worlds.len() as i32 } else { 0 } {
                break;
            }
        }
        if choice == 28 {
            return Err(Error::Invalid("no completed policy action"));
        }
        if self.policies.len() < 50000 {
            self.policies.insert(key, Choice { tile: choice });
        }
        Ok(choice)
    }
    fn single(
        &mut self,
        p: Lite,
        me: usize,
        world: &World,
        path: u64,
        deal: usize,
    ) -> Result<i32, Error> {
        self.tick()?;
        if self.rules.nello {
            if p.t1 > 0 {
                return Ok(0);
            }
            if p.played.count_ones() == 21 {
                return Ok(1);
            }
        } else if p.t1 >= self.rules.bid {
            return Ok(1);
        } else if p.t0 > 42 - self.rules.bid {
            return Ok(0);
        }
        let actor = self.rules.actor(p.leader, p.len);
        let mut moves = world[actor] & !p.played;
        if p.len > 0 {
            let follow = moves & self.rules.suit[p.suit];
            if follow != 0 {
                moves = follow;
            }
        }
        if actor != me && moves.count_ones() > 1 {
            moves = draw(moves, path, deal);
        }
        let mut value = if me % 2 == 1 { -1 } else { 2 };
        while moves != 0 {
            let t = moves.trailing_zeros() as usize;
            moves &= moves - 1;
            let mut q = p;
            q.played |= 1 << t;
            if q.len == 0 {
                q.suit = self.rules.lead[t];
                q.rank = self.rules.strength[q.suit][t];
                q.winner = actor;
                q.points = 1 + self.rules.points[t];
            } else {
                q.points += self.rules.points[t];
                if self.rules.strength[q.suit][t] > q.rank {
                    q.rank = self.rules.strength[q.suit][t];
                    q.winner = actor;
                }
            }
            q.len += 1;
            if q.len == self.rules.trick_size() {
                if q.winner % 2 == 1 {
                    q.t1 += q.points;
                } else {
                    q.t0 += q.points;
                }
                q.leader = q.winner;
                q.len = 0;
            }
            let v = self.single(q, me, world, child_path(path, t), deal)?;
            value = if me % 2 == 1 {
                value.max(v)
            } else {
                value.min(v)
            };
            if actor == me && value == if me % 2 == 1 { 1 } else { 0 } {
                break;
            }
        }
        Ok(value)
    }
    #[allow(
        clippy::too_many_arguments,
        reason = "keep the focal seat, public node and hidden fiber explicit at this boundary"
    )]
    pub fn search(
        &mut self,
        level: usize,
        p: Public,
        me: usize,
        worlds: &[World],
        fibers: &[usize],
        path: u64,
        bound: Option<i32>,
    ) -> Result<Value, Error> {
        let maximize = me % 2 == 1;
        if let Some(b) = bound {
            if if maximize {
                fibers.len() as i32 <= b
            } else {
                0 >= b
            } {
                self.pruned += 1;
                return Ok(Value::bound(b));
            }
        }
        if level == 1 && fibers.len() == 1 {
            let mut lite = Lite {
                played: p.played,
                leader: p.leader,
                len: p.len,
                t1: p.t1,
                t0: p.t0,
                suit: 0,
                rank: 0,
                winner: 0,
                points: 1,
            };
            if p.len > 0 {
                lite.suit = self.rules.lead[p.trick[0]];
            }
            for i in 0..p.len {
                let t = p.trick[i];
                lite.points += self.rules.points[t];
                if i == 0 || self.rules.strength[lite.suit][t] > lite.rank {
                    lite.rank = self.rules.strength[lite.suit][t];
                    lite.winner = self.rules.actor(p.leader, i);
                }
            }
            return Ok(Value::exact(self.single(
                lite,
                me,
                &worlds[fibers[0]],
                path,
                fibers[0],
            )?));
        }
        self.tick()?;
        if let Some(made) = self.rules.outcome(p) {
            return Ok(Value::exact(if made { fibers.len() as i32 } else { 0 }));
        }
        let index = (level - 1) * 29 + p.depth;
        // Owned frame avoids aliasing across recursive lower-level queries. It
        // returns to the arena after this node; no per-node vector allocation.
        let mut frame = self.frames[index]
            .take()
            .expect("distinct level/depth frame");
        for children in &mut frame.children {
            children.clear();
        }
        frame.used = 0;
        frame.choices.clear();
        let actor = self.rules.turn(p);
        let mine = actor == me;
        for &deal in fibers {
            let hand = worlds[deal][actor] & !p.played;
            let mut legal = self.rules.legal(hand, p);
            if legal == 0 {
                return Err(Error::Invalid("empty live fiber"));
            }
            if !mine && legal.count_ones() > 1 {
                if level > 1 {
                    let chosen =
                        if let Some(&(_, t)) = frame.choices.iter().find(|&&(h, _)| h == hand) {
                            t
                        } else {
                            let t = self.action(level - 1, p, hand)?;
                            frame.choices.push((hand, t));
                            t
                        };
                    legal = 1 << chosen;
                } else {
                    legal = draw(legal, path, deal);
                }
            }
            frame.used |= legal;
            while legal != 0 {
                let t = legal.trailing_zeros() as usize;
                legal &= legal - 1;
                frame.children[t].push(deal);
            }
        }
        let mut value = if mine {
            if maximize {
                -1
            } else {
                fibers.len() as i32 + 1
            }
        } else {
            0
        };
        let mut moves = frame.used;
        let mut remaining_mass = fibers.len() as i32;
        let outcome = (|| {
            while moves != 0 {
                let t = moves.trailing_zeros() as usize;
                moves &= moves - 1;
                remaining_mass -= frame.children[t].len() as i32;
                let child_bound = if mine {
                    Some(bound.map_or(
                        value,
                        |b| if maximize { b.max(value) } else { b.min(value) },
                    ))
                } else {
                    bound.map(|b| b - value - if maximize { remaining_mass } else { 0 })
                };
                let v = self.search(
                    level,
                    self.rules.play(p, t),
                    me,
                    worlds,
                    &frame.children[t],
                    child_path(path, t),
                    child_bound,
                )?;
                if !v.exact {
                    if !mine {
                        self.pruned += 1;
                        return Ok(Value::bound(bound.expect("sum constraint")));
                    }
                } else if mine {
                    value = if maximize {
                        value.max(v.count)
                    } else {
                        value.min(v.count)
                    };
                } else {
                    value += v.count;
                }
                if mine && value == if maximize { fibers.len() as i32 } else { 0 } {
                    self.pruned += 1;
                    break;
                }
            }
            if let Some(b) = bound {
                if mine && if maximize { value <= b } else { value >= b } {
                    return Ok(Value::bound(b));
                }
            }
            Ok(Value::exact(value))
        })();
        self.frames[index] = Some(frame);
        outcome
    }
}
#[inline]
pub(super) fn child_path(path: u64, t: usize) -> u64 {
    mix(path ^ ((t as u64 + 1).wrapping_mul(0x9e3779b97f4a7c15)))
}
#[inline]
fn draw(mut moves: Mask, path: u64, deal: usize) -> Mask {
    let mut random = Random(mix(
        path ^ mix((deal as u64).wrapping_add(0xd1b54a32d192ed03))
    ));
    let skip = random.bounded(moves.count_ones() as u64);
    for _ in 0..skip {
        moves &= moves - 1;
    }
    moves & moves.wrapping_neg()
}
