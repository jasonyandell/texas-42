//! The situation census: the first (d, Theta) candidate under §12.6A
//! equivariant controlled lumpability (`walt/math/equivariant_lumpability_v0.5.md`),
//! instantiated on the trick-six receipt kernels per `walt/CENSUS.md` and the
//! binding fork rulings of `walt/CENSUS-RULINGS.md`.
//!
//! **Scope: pip-trump only.** All 13 receipt trick-six kernels are pip-trump
//! (v0.4 §14.7); doubles-trump and no-trump have structurally different
//! context signatures and no receipt corpus, and pooling them would implicitly
//! claim cross-declaration-type transfer, which v0.4 §17.5 does not claim
//! (F1 amendment). Nothing here is checked outside pip-trump.
//!
//! **Tier: exploratory**, below every project evidentiary tier. Every count
//! and every verdict below is computed evidence about one finite domain under
//! the fixed uniform-legal field, never an axiom and never a status change
//! (TRUST-01). A class count is quotable only alongside its ECL verdict (F6).
//!
//! **Determinism.** Nothing is sampled and nothing is capped: the domain is
//! chosen to be exhaustively checkable, the carrier is closed under stepping,
//! and every class with two or more members is checked. Canonicalization is a
//! brute-force minimum over the lawful labelings, so the run has no seeds.
//!
//! **Not the PI census.** `walt_strat::census` is the worldwise
//! perfect-information response census of v0.4 §14.2, which partitions a fiber
//! by PI response. That partition is forbidden as a carrier here (v0.4 §12.4 /
//! §17.5: R_PI(omega) fibers are not hidden-decision classes), and no PI
//! response enters this descriptor. The equivalence tested here is dynamics
//! equivalence (ECL), not response equality.
//!
//! # The instantiation
//!
//! - **Carrier** (F1, F5). A situation is a world-level latent state: the
//!   declaration, the focal seat, the leader of the unresolved trick, the live
//!   tiles of each seat, and the tiles played to the unresolved trick in play
//!   order. The actor, the led context and the current winner are derived
//!   views, computed, never stored. The carrier is every state reachable from
//!   every kernel root (focal to lead at trick 6) under the primitive-step
//!   model, pooled across kernels and deduplicated: two runs that reach the
//!   same latent state reach one carrier element. The count-free banked
//!   increment is emission, not state (F5 amendment) — it appears in the
//!   kernel's emitted `k`, never in the situation or the descriptor.
//! - **Descriptor** `d` (F2). The canonical form of the live structure:
//!   matched object = live tiles ∪ unresolved-trick tiles (dead tiles of
//!   resolved tricks are excluded, A1); seats by the forced rotation aligning
//!   actors with focal ↔ focal as a match precondition and no reflection (A4);
//!   trump ↔ trump and live non-trump contexts (a context is live iff some
//!   live tile leads it, A3) matched by any consistent bijection; preserving
//!   holder-by-relative-seat and table position, pairwise trick-key
//!   comparisons in every live context plus the current led context (A1),
//!   follow membership in every such context, the led-context map
//!   Theta^C(l(d)) = l(Theta^D(d)) (A2), the double flag, and the table play
//!   order. `d(x)` is the lexicographic minimum encoding over the lawful
//!   labelings; the minimizing labeling is `c_x`.
//! - **Transports** (F3). `Theta_xy = c_y^{-1} . c_x`, so coherence is
//!   automatic and no runtime coherence check exists. `Theta^D`'s domain is
//!   all live ∪ unresolved-trick tiles. Because canonical identifiers are
//!   shared across a class, a transported object is compared simply by
//!   comparing canonical identifiers.
//! - **Probability model** (F4). The actor is a function of the state, one
//!   kernel per primitive step. At a focal-to-act state the action set is the
//!   focal seat's legal tiles and each action is a Dirac point mass at the
//!   determined `(k, o, x')`. At a non-focal-to-act state the three field
//!   seats play the fixed uniform-legal field: exact mass `1/|L|` on each
//!   legal move. Probabilities are exact rationals; no probability is ever
//!   attached to a focal action.
//! - **Increment.** `k` is `e_star` when the trick-completing fourth play is
//!   won by the focal partnership and zero otherwise, including trick 7's
//!   final play, where the recursion closes at hand end.
//! - **Baseline** (F6). The same invariant list read with identity interfaces
//!   — absolute tiles, absolute seats, absolute contexts — is the §12.6
//!   control; the equivariance dividend is its class count over the
//!   equivariant class count.
//! - **Failures** (F7, NO-RESCUE). A failing class records its counterexample
//!   with exact rationals and both concrete witnesses, and the check continues
//!   to the next class. The descriptor is never adjusted in-run.

use std::collections::{BTreeMap, BTreeSet};

use walt_core::{legal_plays, Context, Decl, Domino, DominoSet, Seat, Trick};
use walt_geom::{q, qi, Q};
use walt_kernel::Kernel;

/// The most matched tiles a trick-six situation can carry: four seats holding
/// two tiles each, of which the played ones move to the unresolved trick.
pub const MAX_MATCHED_TILES: usize = 8;

/// The hard ceiling on the labeling search: `8!`, the worst case at
/// [`MAX_MATCHED_TILES`]. Exceeding it means the domain is not the one this
/// census declares — stop and report, never sample.
pub const CANON_PERM_CAP: usize = 40_320;

/// A **declared** descriptor candidate. Each toggle names one invariant the
/// canonical form may carry or drop; a coarsening is always a new declared
/// candidate run end to end, never a mutation of an earlier candidate's
/// verdict (v0.4 §12.9's counterexample-guided method, F7's NO-RESCUE
/// discipline). The finest candidate of `walt/CENSUS.md` is [`CandidateSpec::FINEST`],
/// and every coarser candidate is measured against it as the reference row.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CandidateSpec {
    pub name: &'static str,
    /// Carry the double flag. F2 confirms it is not forced by pip-trump
    /// dynamics — doubles act only through rank relations, already captured by
    /// the trick-key comparisons — and names it the first coarsening
    /// candidate.
    pub double_flag: bool,
    /// Carry the identities of the beaten (non-winning) tiles of the
    /// unresolved trick. F2 names these the other first coarsening candidate:
    /// a beaten table tile never enters a future trick key and constrains no
    /// legality. Dropping them keeps the play count, the led context and the
    /// current winning tile with all its comparisons.
    pub beaten_table_tiles: bool,
}

impl CandidateSpec {
    /// The finest lawful candidate: the r1 descriptor, every invariant of F2
    /// as amended.
    pub const FINEST: CandidateSpec = CandidateSpec {
        name: "c1 finest (r1 reference)",
        double_flag: true,
        beaten_table_tiles: true,
    };

    pub const NO_DOUBLE_FLAG: CandidateSpec = CandidateSpec {
        name: "c2 drop the double flag",
        double_flag: false,
        beaten_table_tiles: true,
    };

    pub const NO_BEATEN_TILES: CandidateSpec = CandidateSpec {
        name: "c3 drop beaten table-tile identities",
        double_flag: true,
        beaten_table_tiles: false,
    };

    pub const NEITHER: CandidateSpec = CandidateSpec {
        name: "c2+c3 both coarsenings",
        double_flag: false,
        beaten_table_tiles: false,
    };

    pub const ALL: [CandidateSpec; 4] = [
        CandidateSpec::FINEST,
        CandidateSpec::NO_DOUBLE_FLAG,
        CandidateSpec::NO_BEATEN_TILES,
        CandidateSpec::NEITHER,
    ];

    /// The declaration line that travels with every count this candidate
    /// produces.
    pub fn render(&self) -> String {
        format!(
            "{} — double flag: {}; beaten unresolved-trick tile identities: {}",
            self.name,
            if self.double_flag {
                "carried"
            } else {
                "dropped"
            },
            if self.beaten_table_tiles {
                "carried"
            } else {
                "dropped (play count, led context and the winning tile's comparisons kept)"
            }
        )
    }
}

/// How a play stands to the led context — the observation token's typed class
/// (v0.4 §6.1). A leader's play is `Lead`; a follower's is `Follow` when it
/// lies in the effective incidence of the led context and `Slough` otherwise.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PlayClass {
    Lead,
    Follow,
    Slough,
}

/// One observation token, already transported: the play class plus the
/// canonical identifier of the played tile. The acting seat's relative index
/// is zero in the state-anchored frame (the actor is a function of the state),
/// so it carries no information and is not repeated here; the focal offset is
/// carried by the descriptor itself.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Token {
    pub class: PlayClass,
    pub tile: u8,
}

/// A world-level latent situation. Every derived view — actor, led context,
/// current winner, legal set — is a function of these fields.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Situation {
    pub decl: Decl,
    pub focal: Seat,
    /// The leader of the unresolved trick.
    pub leader: Seat,
    pub hands: [DominoSet; Seat::COUNT],
    /// The unresolved trick's tiles in play order; length 0..=3, since the
    /// fourth play resolves the trick inside one primitive step.
    pub table: Vec<Domino>,
}

impl Situation {
    /// The kernel root: the focal seat leads trick 6 in one fiber world.
    pub fn root(kernel: &Kernel, hands: [DominoSet; Seat::COUNT]) -> Situation {
        Situation {
            decl: kernel.decl(),
            focal: kernel.viewer(),
            leader: kernel.viewer(),
            hands,
            table: Vec::new(),
        }
    }

    /// The seat to act — derived from the leader and how many tiles are down.
    pub fn actor(&self) -> Seat {
        self.leader.plus(self.table.len())
    }

    /// The led context of the unresolved trick, or `None` at a lead.
    pub fn led(&self) -> Option<Context> {
        self.table.first().map(|d| self.decl.led_context(*d))
    }

    /// The seat currently winning the unresolved trick — the running maximum
    /// trick key, a derived view, never stored.
    pub fn current_winner(&self) -> Option<Seat> {
        Some(self.leader.plus(self.winning_position()?))
    }

    /// Which table position currently holds the trick — the running maximum
    /// trick key, a derived view.
    pub fn winning_position(&self) -> Option<usize> {
        let led = self.led()?;
        let mut best = self.decl.trick_key(self.table[0], led);
        let mut at = 0;
        for (i, d) in self.table.iter().enumerate().skip(1) {
            let key = self.decl.trick_key(*d, led);
            if key > best {
                best = key;
                at = i;
            }
        }
        Some(at)
    }

    /// The tile currently winning the unresolved trick.
    pub fn winning_tile(&self) -> Option<Domino> {
        self.winning_position().map(|at| self.table[at])
    }

    /// All tiles still in a hand.
    pub fn live(&self) -> DominoSet {
        self.hands.iter().fold(DominoSet::EMPTY, |a, h| a.union(*h))
    }

    /// The matched object of the descriptor: live tiles together with the
    /// unresolved trick's tiles (A1). Dead tiles of resolved tricks are gone.
    /// Under a candidate that drops beaten table-tile identities, only the
    /// currently winning table tile joins the live tiles.
    pub fn matched(&self, spec: CandidateSpec) -> Vec<Domino> {
        let mut out: Vec<Domino> = self.live().iter().collect();
        if spec.beaten_table_tiles {
            out.extend(self.table.iter().copied());
        } else if let Some(w) = self.winning_tile() {
            out.push(w);
        }
        out.sort_unstable();
        out
    }

    /// The contexts in force: the live contexts (a context is live iff some
    /// live tile leads it, A3) together with the current led context (A1).
    pub fn contexts(&self) -> Vec<Context> {
        let mut set = BTreeSet::new();
        for d in self.live().iter() {
            set.insert(self.decl.led_context(d));
        }
        if let Some(q) = self.led() {
            set.insert(q);
        }
        set.into_iter().collect()
    }

    /// `A(x)` at a focal-to-act state, or the actor's legal set generally.
    pub fn legal(&self) -> DominoSet {
        legal_plays(self.decl, self.hands[self.actor().index()], self.led())
    }

    /// The observation class of playing `tile` here.
    pub fn play_class(&self, tile: Domino) -> PlayClass {
        match self.led() {
            None => PlayClass::Lead,
            Some(q) => {
                if self.decl.follows(tile, q) {
                    PlayClass::Follow
                } else {
                    PlayClass::Slough
                }
            }
        }
    }

    /// One primitive step: the actor plays `tile`. Returns the count-free
    /// increment (`1` for one `e_star`, zero otherwise) and the successor, or
    /// `None` at hand end (trick 7 resolves and the recursion closes).
    pub fn step(&self, tile: Domino) -> (u8, Option<Situation>) {
        let actor = self.actor();
        let mut hands = self.hands;
        let removed = hands[actor.index()].remove(tile);
        assert!(removed, "a step plays a tile the actor holds");
        let mut table = self.table.clone();
        table.push(tile);
        if table.len() < 4 {
            return (
                0,
                Some(Situation {
                    decl: self.decl,
                    focal: self.focal,
                    leader: self.leader,
                    hands,
                    table,
                }),
            );
        }
        let trick = Trick::new(self.leader, [table[0], table[1], table[2], table[3]])
            .expect("the four tiles of a trick are distinct by construction");
        let winner = trick.winner(self.decl);
        let increment = u8::from(winner.team() == self.focal.team());
        if hands.iter().all(|h| h.is_empty()) {
            return (increment, None);
        }
        (
            increment,
            Some(Situation {
                decl: self.decl,
                focal: self.focal,
                leader: winner,
                hands,
                table: Vec::new(),
            }),
        )
    }

    /// A readable rendering for counterexample witnesses.
    pub fn render(&self) -> String {
        let mut out = format!(
            "{} focal={} actor={} leader={}",
            self.decl,
            self.focal,
            self.actor(),
            self.leader
        );
        for s in Seat::ALL {
            let tiles: Vec<String> = self.hands[s.index()]
                .iter()
                .map(|d| d.to_string())
                .collect();
            out.push_str(&format!(" | {s}:{{{}}}", tiles.join(",")));
        }
        let table: Vec<String> = self.table.iter().map(|d| d.to_string()).collect();
        out.push_str(&format!(" | table:[{}]", table.join(",")));
        if let Some(q) = self.led() {
            out.push_str(&format!(" led:{q}"));
        }
        if let Some(w) = self.current_winner() {
            out.push_str(&format!(" winning:{w}"));
        }
        out
    }
}

/// One labeling of a situation's live structure: which canonical identifier
/// each matched tile, in-force context and seat carries. The equivariant frame
/// numbers seats from the actor and tiles/contexts by the minimizing search;
/// the identity frame uses absolute identifiers throughout (F6's control).
struct Frame {
    tiles: Vec<Domino>,
    tile_id: [u8; Domino::COUNT],
    ctxs: Vec<Context>,
    ctx_id: [u8; Context::COUNT],
    seat_id: [u8; Seat::COUNT],
}

/// The sentinel for an object outside the labeled structure: a context no live
/// tile leads and that is not the current led context is dynamically inert and
/// carries no identity (A3).
const OUTSIDE: u8 = 255;

/// Three-valued comparison of two trick keys in one context.
fn cmp_code(a: walt_core::TrickKey, b: walt_core::TrickKey) -> u8 {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => 0,
        std::cmp::Ordering::Equal => 1,
        std::cmp::Ordering::Greater => 2,
    }
}

/// The encoding of a situation's live structure under one labeling. Two
/// situations share an encoding exactly when the labelings match every
/// invariant of F2 (as amended): actor and focal offsets, table depth, holder
/// or table position, double flags, the led-context map, follow membership,
/// and every pairwise trick-key comparison in every context in force.
fn encode(sit: &Situation, f: &Frame, spec: CandidateSpec) -> Vec<u8> {
    let decl = sit.decl;
    let led = sit.led();
    let n = f.tiles.len();
    let mut out = Vec::with_capacity(8 + 2 * f.ctxs.len() + 5 * n + f.ctxs.len() * n * n);
    out.push(0xEC);
    out.push(sit.table.len() as u8);
    out.push(f.seat_id[sit.actor().index()]);
    out.push(f.seat_id[sit.focal.index()]);
    out.push(n as u8);
    out.push(f.ctxs.len() as u8);
    for c in &f.ctxs {
        out.push(f.ctx_id[c.index()]);
        let mut flags = 0u8;
        if *c == Context::Called {
            flags |= 1;
        }
        if led == Some(*c) {
            flags |= 2;
        }
        out.push(flags);
    }
    for d in &f.tiles {
        out.push(f.tile_id[d.index()]);
        out.push(location(sit, *d, f));
        if spec.double_flag {
            out.push(u8::from(d.is_double()));
        }
        out.push(f.ctx_id[decl.led_context(*d).index()]);
        let mut mask = 0u8;
        for c in &f.ctxs {
            if decl.follows(*d, *c) {
                mask |= 1 << f.ctx_id[c.index()];
            }
        }
        out.push(mask);
    }
    for c in &f.ctxs {
        for i in 0..n {
            for j in (i + 1)..n {
                out.push(cmp_code(
                    decl.trick_key(f.tiles[i], *c),
                    decl.trick_key(f.tiles[j], *c),
                ));
            }
        }
    }
    out
}

/// Where a matched tile sits: `0..4` is its holder's seat identifier, `4 + j`
/// is position `j` in the unresolved trick's play order.
fn location(sit: &Situation, d: Domino, f: &Frame) -> u8 {
    for s in Seat::ALL {
        if sit.hands[s.index()].contains(d) {
            return f.seat_id[s.index()];
        }
    }
    let at = sit
        .table
        .iter()
        .position(|t| *t == d)
        .expect("a matched tile is live or on the table");
    4 + at as u8
}

/// Seat identifiers numbered from the actor: `actor.plus(i)` carries `i`. The
/// rotation is forced by actor alignment and reflection is forbidden (A4).
fn actor_seat_ids(actor: Seat) -> [u8; Seat::COUNT] {
    let mut out = [0u8; Seat::COUNT];
    for i in 0..Seat::COUNT {
        out[actor.plus(i).index()] = i as u8;
    }
    out
}

/// The context order induced by a tile order: every context in force is led by
/// some matched tile, so naming each context by the first tile that leads it
/// removes the context labeling from the search space entirely.
fn frame_for(sit: &Situation, order: &[Domino]) -> Frame {
    let mut tile_id = [OUTSIDE; Domino::COUNT];
    for (i, d) in order.iter().enumerate() {
        tile_id[d.index()] = i as u8;
    }
    let in_force: BTreeSet<Context> = sit.contexts().into_iter().collect();
    let mut ctxs: Vec<Context> = Vec::new();
    for d in order {
        let l = sit.decl.led_context(*d);
        if in_force.contains(&l) && !ctxs.contains(&l) {
            ctxs.push(l);
        }
    }
    // Under a candidate that drops the beaten table tiles, the led context can
    // survive its leading tile: it stays in force (A1) but is led by nothing
    // matched, so it takes the last identifier. It is distinguished by its own
    // flag, so the position is forced and no search freedom is added.
    if let Some(q) = sit.led() {
        if !ctxs.contains(&q) {
            ctxs.push(q);
        }
    }
    debug_assert_eq!(
        ctxs.len(),
        in_force.len(),
        "every context in force is labeled"
    );
    let mut ctx_id = [OUTSIDE; Context::COUNT];
    for (i, c) in ctxs.iter().enumerate() {
        ctx_id[c.index()] = i as u8;
    }
    Frame {
        tiles: order.to_vec(),
        tile_id,
        ctxs,
        ctx_id,
        seat_id: actor_seat_ids(sit.actor()),
    }
}

/// A labeling-independent invariant of one matched tile, used to cut the
/// canonicalization search: everything about the tile that no relabeling can
/// change.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct TileInvariant {
    location: u8,
    is_double: bool,
    profile: Vec<(bool, bool, bool, bool, u8, u8, u8)>,
}

fn tile_invariant(
    sit: &Situation,
    d: Domino,
    tiles: &[Domino],
    ctxs: &[Context],
    spec: CandidateSpec,
) -> TileInvariant {
    let decl = sit.decl;
    let led = sit.led();
    let seat_id = actor_seat_ids(sit.actor());
    let mut location = OUTSIDE;
    for s in Seat::ALL {
        if sit.hands[s.index()].contains(d) {
            location = seat_id[s.index()];
        }
    }
    if location == OUTSIDE {
        location = 4 + sit
            .table
            .iter()
            .position(|t| *t == d)
            .expect("a matched tile is live or on the table") as u8;
    }
    let mut profile = Vec::with_capacity(ctxs.len());
    for c in ctxs {
        let key = decl.trick_key(d, *c);
        let (mut over, mut ties, mut under) = (0u8, 0u8, 0u8);
        for e in tiles {
            if *e == d {
                continue;
            }
            match key.cmp(&decl.trick_key(*e, *c)) {
                std::cmp::Ordering::Greater => over += 1,
                std::cmp::Ordering::Equal => ties += 1,
                std::cmp::Ordering::Less => under += 1,
            }
        }
        profile.push((
            *c == Context::Called,
            led == Some(*c),
            decl.led_context(d) == *c,
            decl.follows(d, *c),
            over,
            ties,
            under,
        ));
    }
    profile.sort();
    TileInvariant {
        location,
        is_double: spec.double_flag && d.is_double(),
        profile,
    }
}

fn permutations(items: &[Domino]) -> Vec<Vec<Domino>> {
    if items.len() <= 1 {
        return vec![items.to_vec()];
    }
    let mut out = Vec::new();
    for (i, head) in items.iter().enumerate() {
        let mut rest = items.to_vec();
        rest.remove(i);
        for mut tail in permutations(&rest) {
            let mut one = vec![*head];
            one.append(&mut tail);
            out.push(one);
        }
    }
    out
}

/// `d(x)` together with `c_x`: the minimum encoding over the lawful labelings
/// and the labeling that attains it. Ties among labelings are structure
/// automorphisms; the first minimizer in the deterministic enumeration order
/// is taken, which is all the transports need (they factor through the
/// canonical representative either way).
#[derive(Clone, Debug)]
pub struct Canonical {
    pub key: Vec<u8>,
    /// Absolute domino index to canonical identifier; [`OUTSIDE`] off the
    /// matched object.
    pub tile_id: [u8; Domino::COUNT],
    /// How many lawful labelings the minimization ranged over.
    pub labelings: usize,
}

impl Canonical {
    pub fn id_of(&self, d: Domino) -> u8 {
        self.tile_id[d.index()]
    }
}

/// Computes `d(x)` and `c_x` by brute-force minimization over the labelings
/// consistent with the tile invariants. The seat rotation is forced and the
/// context labeling is induced, so the only freedom is the ordering of
/// invariant-equivalent tiles.
pub fn canonicalize(sit: &Situation, spec: CandidateSpec) -> Canonical {
    let tiles = sit.matched(spec);
    assert!(
        tiles.len() <= MAX_MATCHED_TILES,
        "the census domain carries at most {MAX_MATCHED_TILES} matched tiles; \
         a larger situation is outside the declared domain — report it, do not sample"
    );
    let ctxs = sit.contexts();
    let mut keyed: Vec<(TileInvariant, Domino)> = tiles
        .iter()
        .map(|d| (tile_invariant(sit, *d, &tiles, &ctxs, spec), *d))
        .collect();
    keyed.sort();
    let mut groups: Vec<Vec<Domino>> = Vec::new();
    let mut last: Option<&TileInvariant> = None;
    for (inv, d) in &keyed {
        match (last, groups.last_mut()) {
            (Some(prev), Some(g)) if prev == inv => g.push(*d),
            _ => {
                groups.push(vec![*d]);
                last = Some(inv);
            }
        }
    }
    let group_perms: Vec<Vec<Vec<Domino>>> = groups.iter().map(|g| permutations(g)).collect();
    let labelings: usize = group_perms.iter().map(Vec::len).product();
    assert!(
        labelings <= CANON_PERM_CAP,
        "the labeling search exceeded {CANON_PERM_CAP} — outside the declared domain; \
         report it, never cap or sample"
    );
    let mut odometer = vec![0usize; group_perms.len()];
    let mut best: Option<(Vec<u8>, Frame)> = None;
    loop {
        let mut order: Vec<Domino> = Vec::with_capacity(tiles.len());
        for (g, pick) in group_perms.iter().zip(&odometer) {
            order.extend_from_slice(&g[*pick]);
        }
        let frame = frame_for(sit, &order);
        let key = encode(sit, &frame, spec);
        let improves = match &best {
            Some((b, _)) => key < *b,
            None => true,
        };
        if improves {
            best = Some((key, frame));
        }
        let mut i = 0;
        loop {
            if i == odometer.len() {
                let (key, frame) = best.expect("at least one labeling");
                return Canonical {
                    key,
                    tile_id: frame.tile_id,
                    labelings,
                };
            }
            odometer[i] += 1;
            if odometer[i] < group_perms[i].len() {
                break;
            }
            odometer[i] = 0;
            i += 1;
        }
    }
}

/// The identity-interface reading of the same invariant list (F6's §12.6
/// control): absolute tiles, absolute seats, absolute contexts, no relabeling
/// and no minimization. The declaration enters only through the relations, as
/// it does under the equivariant reading, so two situations under different
/// declarations merge here exactly when the declaration makes no difference to
/// any encoded relation.
pub fn identity_key(sit: &Situation, spec: CandidateSpec) -> Vec<u8> {
    let tiles = sit.matched(spec);
    let mut tile_id = [OUTSIDE; Domino::COUNT];
    for d in &tiles {
        tile_id[d.index()] = d.index() as u8;
    }
    let mut ctxs = sit.contexts();
    ctxs.sort_by_key(|c| c.index());
    let mut ctx_id = [OUTSIDE; Context::COUNT];
    for c in &ctxs {
        ctx_id[c.index()] = c.index() as u8;
    }
    let mut seat_id = [0u8; Seat::COUNT];
    for s in Seat::ALL {
        seat_id[s.index()] = s.index() as u8;
    }
    encode(
        sit,
        &Frame {
            tiles,
            tile_id,
            ctxs,
            ctx_id,
            seat_id,
        },
        spec,
    )
}

/// The pooled carrier: every situation reachable from every root of every
/// declared kernel, deduplicated, with the receipt hands each situation came
/// from.
pub struct Carrier {
    /// The receipt hand indices pooled, in the order given.
    pub hands: Vec<usize>,
    pub states: Vec<Situation>,
    /// Bit `i` is set when the situation is reachable from `hands[i]`'s
    /// kernel.
    pub provenance: Vec<u32>,
    pub is_root: Vec<bool>,
    index: BTreeMap<Situation, usize>,
}

impl Carrier {
    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    pub fn lookup(&self, sit: &Situation) -> Option<usize> {
        self.index.get(sit).copied()
    }

    pub fn roots(&self) -> usize {
        self.is_root.iter().filter(|r| **r).count()
    }

    /// Situations with no tile down: a trick boundary, root or not.
    pub fn boundaries(&self) -> usize {
        self.states.iter().filter(|s| s.table.is_empty()).count()
    }

    /// Situations with the current trick part-played.
    pub fn mid_trick(&self) -> usize {
        self.states.iter().filter(|s| !s.table.is_empty()).count()
    }

    fn intern(&mut self, sit: Situation, root: bool) -> usize {
        if let Some(i) = self.index.get(&sit) {
            return *i;
        }
        let i = self.states.len();
        self.index.insert(sit.clone(), i);
        self.states.push(sit);
        self.provenance.push(0);
        self.is_root.push(root);
        i
    }
}

/// Enumerates every fiber world of every declared kernel and closes the
/// reachable set under primitive steps. The enumerated world count is asserted
/// against the kernel's exact fiber count, so nothing is silently skipped.
pub fn build_carrier(kernels: &[(usize, Kernel)]) -> Carrier {
    assert!(
        kernels.len() <= 32,
        "the provenance mask carries at most 32 pooled kernels"
    );
    let mut carrier = Carrier {
        hands: kernels.iter().map(|(h, _)| *h).collect(),
        states: Vec::new(),
        provenance: Vec::new(),
        is_root: Vec::new(),
        index: BTreeMap::new(),
    };
    for (slot, (_, kernel)) in kernels.iter().enumerate() {
        assert!(
            matches!(kernel.decl(), Decl::PipTrump(_)),
            "the census scope is pip-trump only (F1 amendment)"
        );
        let bit = 1u32 << slot;
        let mut stack: Vec<usize> = Vec::new();
        let mut worlds: u128 = 0;
        for world in kernel.worlds() {
            let root = Situation::root(kernel, world.hands());
            let i = carrier.intern(root, true);
            stack.push(i);
            worlds += 1;
        }
        assert_eq!(
            worlds,
            kernel.count(),
            "the census carrier covers the whole fiber"
        );
        while let Some(i) = stack.pop() {
            if (carrier.provenance[i] & bit) != 0 {
                continue;
            }
            carrier.provenance[i] |= bit;
            let sit = carrier.states[i].clone();
            for tile in sit.legal().iter() {
                let (_, next) = sit.step(tile);
                if let Some(next) = next {
                    let j = carrier.intern(next, false);
                    stack.push(j);
                }
            }
        }
    }
    carrier
}

/// The step law at one situation. At a focal-to-act situation the map is from
/// transported action to the Dirac outcome; at a non-focal-to-act situation it
/// is the exact joint law over `(increment, transported token, successor
/// class)` under the fixed uniform-legal field.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Law {
    Focal(BTreeMap<u8, (u8, Token, Option<usize>)>),
    Field(BTreeMap<(u8, Token, Option<usize>), Q>),
}

/// The whole census: carrier, canonical forms, equivariant classes, and the
/// identity-interface control on the same carrier.
pub struct Census {
    /// The declared candidate every count in this census belongs to.
    pub spec: CandidateSpec,
    pub carrier: Carrier,
    pub canon: Vec<Canonical>,
    /// Equivariant class of each carrier situation.
    pub class_of: Vec<usize>,
    pub class_members: Vec<Vec<usize>>,
    pub class_keys: Vec<Vec<u8>>,
    /// Identity-interface class of each carrier situation (F6's control).
    pub identity_of: Vec<usize>,
    pub identity_members: Vec<Vec<usize>>,
    pub laws: Vec<Law>,
}

impl Census {
    pub fn build(carrier: Carrier, spec: CandidateSpec) -> Census {
        let canon: Vec<Canonical> = carrier
            .states
            .iter()
            .map(|s| canonicalize(s, spec))
            .collect();
        let (class_of, class_members, class_keys) =
            group(canon.iter().map(|c| c.key.clone()).collect());
        let (identity_of, identity_members, _) = group(
            carrier
                .states
                .iter()
                .map(|s| identity_key(s, spec))
                .collect::<Vec<Vec<u8>>>(),
        );
        let mut census = Census {
            spec,
            carrier,
            canon,
            class_of,
            class_members,
            class_keys,
            identity_of,
            identity_members,
            laws: Vec::new(),
        };
        let laws: Vec<Law> = (0..census.carrier.len()).map(|i| census.law(i)).collect();
        census.laws = laws;
        census
    }

    /// The primitive-step law at carrier situation `i`, with tiles named by
    /// `c_x` and successors named by their descriptor class.
    fn law(&self, i: usize) -> Law {
        let sit = &self.carrier.states[i];
        let canon = &self.canon[i];
        let legal = sit.legal();
        assert!(!legal.is_empty(), "a seat holding tiles has a legal play");
        let focal = sit.actor() == sit.focal;
        let mut dirac = BTreeMap::new();
        let mut field: BTreeMap<(u8, Token, Option<usize>), Q> = BTreeMap::new();
        let share = q(1, legal.len() as i128);
        for tile in legal.iter() {
            let (increment, next) = sit.step(tile);
            let successor = next.map(|s| {
                let j = self
                    .carrier
                    .lookup(&s)
                    .expect("the carrier is closed under primitive steps");
                self.class_of[j]
            });
            let token = Token {
                class: sit.play_class(tile),
                tile: canon.id_of(tile),
            };
            if focal {
                dirac.insert(canon.id_of(tile), (increment, token, successor));
            } else {
                *field
                    .entry((increment, token, successor))
                    .or_insert_with(|| qi(0)) += share;
            }
        }
        if focal {
            assert_eq!(dirac.len(), legal.len(), "c_x names tiles injectively");
            Law::Focal(dirac)
        } else {
            let mass: Q = field.values().copied().sum();
            assert_eq!(mass, qi(1), "a field row is a probability law");
            Law::Field(field)
        }
    }

    /// The receipt hands a class draws situations from.
    pub fn class_provenance(&self, class: usize) -> u32 {
        self.class_members[class]
            .iter()
            .fold(0, |a, i| a | self.carrier.provenance[*i])
    }

    pub fn identity_provenance(&self, class: usize) -> u32 {
        self.identity_members[class]
            .iter()
            .fold(0, |a, i| a | self.carrier.provenance[*i])
    }

    /// Classes drawing situations from two or more pooled receipt hands.
    pub fn cross_kernel_classes(&self) -> Vec<usize> {
        (0..self.class_members.len())
            .filter(|c| self.class_provenance(*c).count_ones() >= 2)
            .collect()
    }

    pub fn cross_kernel_identity_classes(&self) -> Vec<usize> {
        (0..self.identity_members.len())
            .filter(|c| self.identity_provenance(*c).count_ones() >= 2)
            .collect()
    }

    pub fn singleton_classes(&self) -> usize {
        self.class_members.iter().filter(|m| m.len() == 1).count()
    }

    /// Distinct classes among the situations of one pooled kernel slot,
    /// optionally restricted to that kernel's roots.
    pub fn classes_of_slot(&self, slot: usize, roots_only: bool) -> usize {
        let bit = 1u32 << slot;
        let mut seen = BTreeSet::new();
        for i in 0..self.carrier.len() {
            if (self.carrier.provenance[i] & bit) == 0 {
                continue;
            }
            if roots_only && !self.carrier.is_root[i] {
                continue;
            }
            seen.insert(self.class_of[i]);
        }
        seen.len()
    }

    pub fn identity_classes_of_slot(&self, slot: usize, roots_only: bool) -> usize {
        let bit = 1u32 << slot;
        let mut seen = BTreeSet::new();
        for i in 0..self.carrier.len() {
            if (self.carrier.provenance[i] & bit) == 0 {
                continue;
            }
            if roots_only && !self.carrier.is_root[i] {
                continue;
            }
            seen.insert(self.identity_of[i]);
        }
        seen.len()
    }

    pub fn states_of_slot(&self, slot: usize) -> usize {
        let bit = 1u32 << slot;
        (0..self.carrier.len())
            .filter(|i| (self.carrier.provenance[*i] & bit) != 0)
            .count()
    }

    pub fn root_classes(&self) -> usize {
        let mut seen = BTreeSet::new();
        for i in 0..self.carrier.len() {
            if self.carrier.is_root[i] {
                seen.insert(self.class_of[i]);
            }
        }
        seen.len()
    }

    pub fn root_identity_classes(&self) -> usize {
        let mut seen = BTreeSet::new();
        for i in 0..self.carrier.len() {
            if self.carrier.is_root[i] {
                seen.insert(self.identity_of[i]);
            }
        }
        seen.len()
    }
}

fn group(keys: Vec<Vec<u8>>) -> (Vec<usize>, Vec<Vec<usize>>, Vec<Vec<u8>>) {
    let mut ids: BTreeMap<Vec<u8>, usize> = BTreeMap::new();
    let mut of = Vec::with_capacity(keys.len());
    let mut members: Vec<Vec<usize>> = Vec::new();
    let mut unique: Vec<Vec<u8>> = Vec::new();
    for (i, key) in keys.into_iter().enumerate() {
        let next = ids.len();
        let id = *ids.entry(key.clone()).or_insert(next);
        if id == members.len() {
            members.push(Vec::new());
            unique.push(key);
        }
        members[id].push(i);
        of.push(id);
    }
    (of, members, unique)
}

/// Why one class failed (ECL), with its witnessing pair. Recorded and carried
/// back to the math; the check continues to the next class (F7, NO-RESCUE).
#[derive(Clone, Debug)]
pub struct EclFailure {
    pub class: usize,
    pub class_key: String,
    pub representative: String,
    pub member: String,
    /// Which condition diverged and how, in words plus exact values.
    pub condition: String,
    pub detail: String,
}

/// The exhaustive (ECL) verdict on the census carrier.
#[derive(Clone, Debug, Default)]
pub struct EclVerdict {
    pub classes: usize,
    pub singleton_classes: usize,
    pub classes_checked: usize,
    /// Pairs checked for legality preservation, `A(y) = Theta^A(A(x))`.
    pub cond1_checks: u128,
    /// Pairs checked for joint-law agreement.
    pub cond2_checks: u128,
    pub failures: Vec<EclFailure>,
}

impl EclVerdict {
    pub fn passed(&self) -> bool {
        self.failures.is_empty()
    }

    pub fn verdict(&self) -> &'static str {
        if self.passed() {
            "PASS"
        } else {
            "FAIL"
        }
    }
}

fn hex(key: &[u8]) -> String {
    key.iter().map(|b| format!("{b:02x}")).collect()
}

fn render_token(t: &Token) -> String {
    format!("{:?}(tile#{})", t.class, t.tile)
}

fn render_outcome(o: &(u8, Token, Option<usize>)) -> String {
    let (k, token, succ) = o;
    format!(
        "k={}e*, obs={}, succ={}",
        k,
        render_token(token),
        succ.map_or("TERMINAL".to_string(), |c| format!("class#{c}"))
    )
}

/// Checks (ECL) exhaustively on the census: for every class with two or more
/// members, the canonical representative against every other member.
/// Condition 1 is `A(y) = Theta^A(A(x))` read as equality of the transported
/// action sets; condition 2 is agreement of the joint law of (count-free
/// increment, transported observation, successor class). Every successor is
/// itself a carrier situation checked in its own class, so the per-step check
/// plus induction over the finite graded carrier is the whole verification.
pub fn check_ecl(census: &Census) -> EclVerdict {
    let mut verdict = EclVerdict {
        classes: census.class_members.len(),
        singleton_classes: census.singleton_classes(),
        ..EclVerdict::default()
    };
    for (class, members) in census.class_members.iter().enumerate() {
        if members.len() < 2 {
            continue;
        }
        verdict.classes_checked += 1;
        let rep = members[0];
        for &m in &members[1..] {
            verdict.cond1_checks += 1;
            verdict.cond2_checks += 1;
            let (a, b) = (&census.laws[rep], &census.laws[m]);
            let failure = match (a, b) {
                (Law::Focal(x), Law::Focal(y)) => focal_failure(x, y),
                (Law::Field(x), Law::Field(y)) => field_failure(x, y),
                _ => Some((
                    "ECL actor type".to_string(),
                    "one member is focal-to-act and the other is not".to_string(),
                )),
            };
            if let Some((condition, detail)) = failure {
                verdict.failures.push(EclFailure {
                    class,
                    class_key: hex(&census.class_keys[class]),
                    representative: census.carrier.states[rep].render(),
                    member: census.carrier.states[m].render(),
                    condition,
                    detail,
                });
            }
        }
    }
    verdict
}

type Dirac = BTreeMap<u8, (u8, Token, Option<usize>)>;

fn focal_failure(x: &Dirac, y: &Dirac) -> Option<(String, String)> {
    let xs: BTreeSet<u8> = x.keys().copied().collect();
    let ys: BTreeSet<u8> = y.keys().copied().collect();
    if xs != ys {
        return Some((
            "ECL condition 1 (legality)".to_string(),
            format!(
                "A(rep) = {xs:?} but A(member) = {ys:?} under Theta^A (canonical tile identifiers)"
            ),
        ));
    }
    for (a, out_x) in x {
        let out_y = &y[a];
        if out_x != out_y {
            return Some((
                "ECL condition 2 (commutation, Dirac)".to_string(),
                format!(
                    "action tile#{a}: rep steps to [{}] but member steps to [{}] (mass 1/1 each)",
                    render_outcome(out_x),
                    render_outcome(out_y)
                ),
            ));
        }
    }
    None
}

type Joint = BTreeMap<(u8, Token, Option<usize>), Q>;

fn field_failure(x: &Joint, y: &Joint) -> Option<(String, String)> {
    let mut keys: BTreeSet<&(u8, Token, Option<usize>)> = x.keys().collect();
    keys.extend(y.keys());
    for key in keys {
        let a = x.get(key).copied().unwrap_or_else(|| qi(0));
        let b = y.get(key).copied().unwrap_or_else(|| qi(0));
        if a != b {
            return Some((
                "ECL condition 2 (joint law)".to_string(),
                format!(
                    "event [{}]: rep mass {} but member mass {} (exact rationals)",
                    render_outcome(&(key.0, key.1, key.2)),
                    a,
                    b
                ),
            ));
        }
    }
    None
}

/// The 13 trick-six receipt kernels, the declared census domain.
pub fn trick_six_kernels(receipt: &walt_core::receipt::Receipt) -> Vec<(usize, Kernel)> {
    (0..receipt.hands.len())
        .map(|h| {
            (
                h,
                Kernel::from_receipt_trick(&receipt.hands[h], 6).expect("a valid trick-six kernel"),
            )
        })
        .collect()
}
