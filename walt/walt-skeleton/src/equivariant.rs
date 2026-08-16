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

/// The most matched tiles a situation can carry at the deepest declared rung:
/// four seats holding three tiles each at a trick-five lead, of which the
/// played ones move to the unresolved trick. The trick-six rung uses eight.
pub const MAX_MATCHED_TILES: usize = 12;

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

/// The reachable closure of arbitrary seed situations under primitive steps,
/// pooled into one provenance slot. The declared census carrier is
/// [`build_carrier`]; this exists so the machinery can be exercised on
/// hand-built structures, and it takes the same closure path.
pub fn closure_carrier(seeds: &[Situation]) -> Carrier {
    let mut carrier = Carrier {
        hands: vec![0],
        states: Vec::new(),
        provenance: Vec::new(),
        is_root: Vec::new(),
        index: BTreeMap::new(),
    };
    let mut stack: Vec<usize> = seeds
        .iter()
        .map(|s| carrier.intern(s.clone(), true))
        .collect();
    while let Some(i) = stack.pop() {
        if (carrier.provenance[i] & 1) != 0 {
            continue;
        }
        carrier.provenance[i] |= 1;
        let sit = carrier.states[i].clone();
        for tile in sit.legal().iter() {
            let (_, next) = sit.step(tile);
            if let Some(next) = next {
                let j = carrier.intern(next, false);
                stack.push(j);
            }
        }
    }
    carrier
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

// ---------------------------------------------------------------------------
// r3 — the retrograde coarsest quotient (CENSUS-RULINGS.md "r3", Q1-Q5).
// ---------------------------------------------------------------------------

/// **Determinism freeze 1 — the content-addressed encoding.** A class identity
/// is the 128-bit FNV-1a hash of its signature bytes, and a signature names its
/// successors by *their* hashes, so a class identity is a function of the
/// future cone alone (Q4). Changing this function changes every class
/// identifier, so it is frozen here and named in the results header.
const FNV_OFFSET_128: u128 = 0x6c62_272e_07bb_0142_62b8_2175_6295_c58d;
const FNV_PRIME_128: u128 = 0x0000_0000_0100_0000_0000_0000_0000_013b;

fn fnv1a_128(bytes: &[u8]) -> u128 {
    let mut h = FNV_OFFSET_128;
    for b in bytes {
        h ^= *b as u128;
        h = h.wrapping_mul(FNV_PRIME_128);
    }
    h
}

/// The signature tag byte, part of the frozen encoding.
const R3_TAG: u8 = 0x33;

/// The literal classification code — the classification has no transport sort,
/// so it is preserved verbatim (Q3).
fn class_code(c: PlayClass) -> u8 {
    match c {
        PlayClass::Lead => 0,
        PlayClass::Follow => 1,
        PlayClass::Slough => 2,
    }
}

/// The hand-end class: one class by ruling, with an empty future cone.
fn terminal_hash() -> u128 {
    fnv1a_128(b"r3-terminal-hand-end")
}

/// One move's signature tuple: the count-free increment, the play
/// classification (preserved literally — it has no transport sort, Q3), and
/// the successor's r3 class. Tile identity and led context are deliberately
/// absent: they are transported per move, which is the whole point of r3.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct MoveTuple {
    pub increment: u8,
    pub class: PlayClass,
    /// The successor's r3 class, or `None` at hand end.
    pub successor: Option<usize>,
}

/// The retrograde coarsest quotient over a graded carrier.
pub struct R3 {
    pub class_of: Vec<usize>,
    pub class_members: Vec<Vec<usize>>,
    pub class_hash: Vec<u128>,
    pub class_grade: Vec<usize>,
    /// The canonical move order at each carrier situation — the order the
    /// transports are position matching through (Q1b).
    pub moves: Vec<Vec<Domino>>,
    /// The per-move tuples at each carrier situation, in canonical order.
    pub tuples: Vec<Vec<MoveTuple>>,
    pub terminal: u128,
}

/// The grade of a situation: its live tile count. Every primitive play removes
/// exactly one live tile, so successors sit at grade `g - 1` and one backward
/// pass closes (Q2) — asserted at every step of the build.
pub fn grade(sit: &Situation) -> usize {
    sit.live().len()
}

/// The actor's offset from the focal seat: 0 is a focal choice node, 1..=3 are
/// hidden chance nodes. The lawful chair correspondence preserves it exactly
/// (Q3: focal↔focal, partner↔partner, left↔left), so it belongs in the
/// signature preamble.
pub fn actor_offset(sit: &Situation) -> u8 {
    let mut i = 0u8;
    while sit.focal.plus(i as usize) != sit.actor() {
        i += 1;
        assert!(i < 4, "the actor is one of the four seats");
    }
    i
}

/// **Determinism freeze 2 — the canonical move order.** Moves are sorted by
/// the full signature tuple (increment, classification, successor class), ties
/// broken by the state's concrete tile order. Moves with identical tuples emit
/// identical statistics, so the tie order never changes a law; fixing it is
/// what makes `Theta^A` / `Theta^obs` position matching coherent by
/// construction rather than an arbitrary per-pair choice (Q1b).
fn canonical_move_order(
    mut moves: Vec<(MoveTuple, u128, Domino)>,
) -> Vec<(MoveTuple, u128, Domino)> {
    moves.sort_by(|a, b| {
        (a.0.increment, a.0.class, a.1, a.2).cmp(&(b.0.increment, b.0.class, b.1, b.2))
    });
    moves
}

/// The backward pass: by increasing grade (so every successor is already
/// classified), signature by signature, content-addressed.
pub fn build_r3(carrier: &Carrier) -> R3 {
    let terminal = terminal_hash();
    // Grade is the live-tile count, bounded by the double-six set itself —
    // not by MAX_MATCHED_TILES, which is r1's canonicalization-domain guard
    // and plays no role in the retrograde pass.
    const MAX_GRADE: usize = 28;
    let mut by_grade: Vec<Vec<usize>> = vec![Vec::new(); MAX_GRADE + 1];
    for (i, sit) in carrier.states.iter().enumerate() {
        by_grade[grade(sit)].push(i);
    }
    assert!(
        by_grade[0].is_empty(),
        "a carrier situation always has a live tile; hand end is not a carrier state"
    );

    let mut r3 = R3 {
        class_of: vec![usize::MAX; carrier.len()],
        class_members: Vec::new(),
        class_hash: Vec::new(),
        class_grade: Vec::new(),
        moves: vec![Vec::new(); carrier.len()],
        tuples: vec![Vec::new(); carrier.len()],
        terminal,
    };
    let mut ids: BTreeMap<Vec<u8>, usize> = BTreeMap::new();
    let mut seen_hash: BTreeMap<u128, Vec<u8>> = BTreeMap::new();
    let mut state_hash: Vec<u128> = vec![0; carrier.len()];

    for (g, states) in by_grade.iter().enumerate().skip(1) {
        for &i in states {
            let sit = &carrier.states[i];
            let mut moves: Vec<(MoveTuple, u128, Domino)> = Vec::new();
            for tile in sit.legal().iter() {
                let (increment, next) = sit.step(tile);
                let (successor, hash) = match next {
                    None => (None, terminal),
                    Some(s) => {
                        assert_eq!(grade(&s), g - 1, "a primitive play drops the grade by one");
                        let j = carrier
                            .lookup(&s)
                            .expect("the carrier is closed under primitive steps");
                        (Some(r3.class_of[j]), state_hash[j])
                    }
                };
                assert!(
                    successor != Some(usize::MAX),
                    "successors are classified before their predecessors"
                );
                moves.push((
                    MoveTuple {
                        increment,
                        class: sit.play_class(tile),
                        successor,
                    },
                    hash,
                    tile,
                ));
            }
            let moves = canonical_move_order(moves);

            let mut sig = Vec::with_capacity(4 + 18 * moves.len());
            sig.push(R3_TAG);
            sig.push(g as u8);
            sig.push(actor_offset(sit));
            sig.push(moves.len() as u8);
            for (tuple, hash, _) in &moves {
                sig.push(tuple.increment);
                sig.push(class_code(tuple.class));
                sig.extend_from_slice(&hash.to_be_bytes());
            }
            let hash = fnv1a_128(&sig);
            match seen_hash.get(&hash) {
                Some(other) => assert_eq!(
                    other, &sig,
                    "a 128-bit content-address collision — the determinism freeze is broken; \
                     report it, never work around it"
                ),
                None => {
                    seen_hash.insert(hash, sig.clone());
                }
            }
            let next_id = ids.len();
            let id = *ids.entry(sig).or_insert(next_id);
            if id == r3.class_members.len() {
                r3.class_members.push(Vec::new());
                r3.class_hash.push(hash);
                r3.class_grade.push(g);
            }
            r3.class_members[id].push(i);
            r3.class_of[i] = id;
            state_hash[i] = hash;
            r3.moves[i] = moves.iter().map(|(_, _, d)| *d).collect();
            r3.tuples[i] = moves.iter().map(|(t, _, _)| *t).collect();
        }
    }
    assert!(
        r3.class_of.iter().all(|c| *c != usize::MAX),
        "every carrier situation is classified"
    );
    r3
}

/// The class-level transition graph of an r3 quotient: the object a seat could
/// actually search. A class's successors are well defined — (ECL) makes every
/// member of a class emit the same multiset of `(k, classification, successor
/// class)` tuples — and [`class_dag`] asserts that agreement rather than
/// assuming it.
pub struct ClassDag {
    /// Per class, its distinct successor classes, ascending.
    pub successors: Vec<Vec<usize>>,
    /// Per class, whether some move ends the hand.
    pub terminal: Vec<bool>,
}

impl ClassDag {
    /// Every class reachable from `seeds`, seeds included.
    pub fn reachable(&self, seeds: &[usize]) -> BTreeSet<usize> {
        let mut seen: BTreeSet<usize> = seeds.iter().copied().collect();
        let mut stack: Vec<usize> = seeds.to_vec();
        while let Some(c) = stack.pop() {
            for s in &self.successors[c] {
                if seen.insert(*s) {
                    stack.push(*s);
                }
            }
        }
        seen
    }

    /// Class-level edges inside a set of classes, hand-end edges counted
    /// separately.
    pub fn edges(&self, live: &BTreeSet<usize>) -> (usize, usize) {
        let mut edges = 0;
        let mut ends = 0;
        for c in live {
            edges += self.successors[*c].len();
            if self.terminal[*c] {
                ends += 1;
            }
        }
        (edges, ends)
    }
}

/// Builds the class-level transition graph, asserting the successor structure
/// is a function of the class and not of the representative.
pub fn class_dag(r3: &R3) -> ClassDag {
    let mut dag = ClassDag {
        successors: vec![Vec::new(); r3.class_members.len()],
        terminal: vec![false; r3.class_members.len()],
    };
    for (c, members) in r3.class_members.iter().enumerate() {
        let of = |i: usize| -> Vec<Option<usize>> {
            let mut out: Vec<Option<usize>> = r3.tuples[i].iter().map(|t| t.successor).collect();
            out.sort();
            out
        };
        let reference = of(members[0]);
        for m in &members[1..] {
            assert_eq!(
                reference,
                of(*m),
                "a class's successor structure is a function of the class (ECL), not of the \
                 representative"
            );
        }
        let mut successors: Vec<usize> = reference.iter().filter_map(|s| *s).collect();
        successors.sort_unstable();
        successors.dedup();
        dag.terminal[c] = reference.iter().any(Option::is_none);
        dag.successors[c] = successors;
    }
    dag
}

// ---------------------------------------------------------------------------
// The railyard factoring (CENSUS-RULINGS.md "The railyard factoring —
// shaping", Y1-Y3). Level j = tricks remaining; at a trick boundary every seat
// holds j tiles, so a boundary state sits at grade 4j and A_j is the set of
// level-j boundary classes, A_0 the one terminal class.
// ---------------------------------------------------------------------------

/// The handoff symbol for hand end — the single terminal class, A_0.
pub const YARD_TERMINAL: u64 = u64::MAX;

/// The declared ceiling on the leaf-relabeling search inside a shape's
/// canonical form. Past it the shape is not canonicalizable at this budget:
/// stop and report rather than approximating a shape count.
pub const SHAPE_PERM_CAP: usize = 5_040;

/// One node of the yard machine's depth-four signature tree. Interior nodes
/// are the primitive steps inside one trick; leaves are handoff classes drawn
/// from the level below (Y1's correction: four primitive steps with
/// handoff-class terminals, never a trick-level macro step).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum YardNode {
    /// A level-(j-1) boundary class, or [`YARD_TERMINAL`] at hand end.
    Handoff(u64),
    Step {
        /// The actor's offset from focal — the Q3 preamble, carried at every
        /// primitive step.
        offset: u8,
        /// Canonically ordered `(increment, classification, subtree)`.
        moves: Vec<(u8, PlayClass, YardNode)>,
    },
}

impl YardNode {
    /// The frozen encoding: `[0x4c, symbol big-endian]` for a handoff leaf,
    /// `[0x59, offset, move count, then per move: increment, classification
    /// code, child encoding]` for a step. Children are sorted by (increment,
    /// classification, child encoding) — the same freeze as r3's canonical
    /// move order, applied to the tree.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.encode_into(&mut out);
        out
    }

    fn encode_into(&self, out: &mut Vec<u8>) {
        match self {
            YardNode::Handoff(symbol) => {
                out.push(0x4c);
                out.extend_from_slice(&symbol.to_be_bytes());
            }
            YardNode::Step { offset, moves } => {
                out.push(0x59);
                out.push(*offset);
                out.push(moves.len() as u8);
                for (increment, class, child) in moves {
                    out.push(*increment);
                    out.push(class_code(*class));
                    child.encode_into(out);
                }
            }
        }
    }

    /// Every handoff symbol in the tree, in canonical-traversal order with
    /// repeats — the raw material of the equality pattern.
    fn leaves(&self, out: &mut Vec<u64>) {
        match self {
            YardNode::Handoff(s) => out.push(*s),
            YardNode::Step { moves, .. } => {
                for (_, _, child) in moves {
                    child.leaves(out);
                }
            }
        }
    }

    /// The tree with every handoff symbol replaced through `map`.
    fn relabel(&self, map: &BTreeMap<u64, u64>) -> YardNode {
        match self {
            YardNode::Handoff(s) => YardNode::Handoff(map[s]),
            YardNode::Step { offset, moves } => {
                let mut moves: Vec<(u8, PlayClass, YardNode)> = moves
                    .iter()
                    .map(|(i, c, child)| (*i, *c, child.relabel(map)))
                    .collect();
                moves.sort_by_cached_key(|m| (m.0, class_code(m.1), m.2.encode()));
                YardNode::Step {
                    offset: *offset,
                    moves,
                }
            }
        }
    }
}

/// **The one shared grade-free routine.** Unfolds one trick from a boundary
/// situation as four primitive steps, bottoming out in handoff classes. It
/// takes no grade and no level argument — the within-trick position is read
/// off the table depth, and the level enters only through the caller's handoff
/// alphabet (Y2 P1 obligations (a) and (b)).
pub fn yard_tree<F>(sit: &Situation, handoff: &F) -> YardNode
where
    F: Fn(&Situation) -> u64,
{
    let resolving = sit.table.len() == 3;
    let mut moves: Vec<(u8, PlayClass, YardNode)> = Vec::new();
    for tile in sit.legal().iter() {
        let (increment, next) = sit.step(tile);
        assert!(
            resolving || increment == 0,
            "a count-free increment is emittable only at the trick-completing play"
        );
        let child = match next {
            None => YardNode::Handoff(YARD_TERMINAL),
            Some(s) if resolving => YardNode::Handoff(handoff(&s)),
            Some(s) => yard_tree(&s, handoff),
        };
        moves.push((increment, sit.play_class(tile), child));
    }
    moves.sort_by_cached_key(|m| (m.0, class_code(m.1), m.2.encode()));
    YardNode::Step {
        offset: actor_offset(sit),
        moves,
    }
}

/// The SHAPE of a yard tree: the tree with its leaves abstracted to their
/// equality pattern (§3.4 — which leaves coincide, not what they are). Leaf
/// symbols are renumbered by a canonical relabeling: colours are refined until
/// stable, then the minimum encoding over the orderings still tied is taken.
/// Returns `None` when the remaining search exceeds [`SHAPE_PERM_CAP`] — stop
/// and report, never approximate.
pub fn yard_shape(tree: &YardNode) -> Option<Vec<u8>> {
    let mut occurrences = Vec::new();
    tree.leaves(&mut occurrences);
    let distinct: BTreeSet<u64> = occurrences.iter().copied().collect();
    let symbols: Vec<u64> = distinct.into_iter().collect();

    // Colour refinement: recolour each symbol by the multiset of positions it
    // occupies in the tree drawn with the current colours, until stable.
    let mut colour: BTreeMap<u64, u64> = symbols.iter().map(|s| (*s, 0u64)).collect();
    for _ in 0..4 {
        let drawn = tree.relabel(&colour);
        let mut positions: BTreeMap<u64, Vec<Vec<u8>>> = BTreeMap::new();
        collect_positions(&drawn, tree, &colour, &mut Vec::new(), &mut positions);
        let mut refined: BTreeMap<u64, u64> = BTreeMap::new();
        let mut keys: Vec<(u64, Vec<Vec<u8>>, u64)> = symbols
            .iter()
            .map(|s| {
                let mut p = positions.get(s).cloned().unwrap_or_default();
                p.sort();
                (colour[s], p, *s)
            })
            .collect();
        keys.sort();
        let mut next = 0u64;
        let mut previous: Option<(u64, Vec<Vec<u8>>)> = None;
        for (c, p, s) in keys {
            match &previous {
                Some((pc, pp)) if *pc == c && *pp == p => {}
                _ => {
                    next += 1;
                    previous = Some((c, p.clone()));
                }
            }
            refined.insert(s, next);
        }
        if refined == colour {
            break;
        }
        colour = refined;
    }

    // Whatever colour refinement left tied is searched exhaustively.
    let mut groups: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
    for s in &symbols {
        groups.entry(colour[s]).or_default().push(*s);
    }
    let ordered: Vec<Vec<u64>> = groups.into_values().collect();
    let total: usize = ordered.iter().map(|g| factorial(g.len())).product();
    if total > SHAPE_PERM_CAP {
        return None;
    }
    let group_perms: Vec<Vec<Vec<u64>>> = ordered.iter().map(|g| symbol_perms(g)).collect();
    let mut odometer = vec![0usize; group_perms.len()];
    let mut best: Option<Vec<u8>> = None;
    loop {
        let mut map: BTreeMap<u64, u64> = BTreeMap::new();
        let mut n = 0u64;
        for (g, pick) in group_perms.iter().zip(&odometer) {
            for s in &g[*pick] {
                map.insert(*s, n);
                n += 1;
            }
        }
        let key = tree.relabel(&map).encode();
        if best.as_ref().is_none_or(|b| key < *b) {
            best = Some(key);
        }
        let mut i = 0;
        loop {
            if i == odometer.len() {
                return best;
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

/// Where each original symbol sits in the tree drawn with current colours: the
/// child-index path from the root, which is well defined once the drawing is
/// canonically ordered.
fn collect_positions(
    drawn: &YardNode,
    original: &YardNode,
    colour: &BTreeMap<u64, u64>,
    path: &mut Vec<u8>,
    out: &mut BTreeMap<u64, Vec<Vec<u8>>>,
) {
    match (drawn, original) {
        (YardNode::Handoff(_), YardNode::Handoff(s)) => {
            out.entry(*s).or_default().push(path.clone());
        }
        (
            YardNode::Step { moves: drawn_m, .. },
            YardNode::Step {
                moves: original_m, ..
            },
        ) => {
            // Re-sort the original's children the same way the drawing is
            // sorted, so positions correspond.
            let mut sorted: Vec<&(u8, PlayClass, YardNode)> = original_m.iter().collect();
            sorted.sort_by_key(|m| m.2.relabel(colour).encode());
            for (i, (m, _)) in sorted.iter().zip(drawn_m).enumerate() {
                path.push(i as u8);
                collect_positions(&m.2.relabel(colour), &m.2, colour, path, out);
                path.pop();
            }
        }
        _ => unreachable!("a drawing has the same skeleton as its tree"),
    }
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn symbol_perms(items: &[u64]) -> Vec<Vec<u64>> {
    if items.len() <= 1 {
        return vec![items.to_vec()];
    }
    let mut out = Vec::new();
    for (i, head) in items.iter().enumerate() {
        let mut rest = items.to_vec();
        rest.remove(i);
        for mut tail in symbol_perms(&rest) {
            let mut one = vec![*head];
            one.append(&mut tail);
            out.push(one);
        }
    }
    out
}

/// The constraint type of a yard node, derived — never stored — from the
/// classification its moves carry. A leader may play any remaining tile and a
/// follower unable to follow may slough any tile (v0.4 §1.5), so lead and
/// slough nodes are the UNCONSTRAINED ones whose arity is the level by rule; a
/// forced-follow node's arity is a suit-split fact that can coincide across
/// levels.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Constraint {
    Lead,
    ForcedFollow,
    Slough,
}

impl Constraint {
    pub fn unconstrained(self) -> bool {
        matches!(self, Constraint::Lead | Constraint::Slough)
    }
}

/// A node's constraint type. Legality makes the classification uniform across
/// a node's moves — a follower who can follow has only following tiles legal,
/// one who cannot has the whole hand — which this asserts rather than assumes.
pub fn node_constraint(moves: &[(u8, PlayClass, YardNode)]) -> Constraint {
    let first = moves.first().expect("a node has at least one move").1;
    for (_, class, _) in moves {
        assert_eq!(
            *class, first,
            "legality makes a node's classification uniform"
        );
    }
    match first {
        PlayClass::Lead => Constraint::Lead,
        PlayClass::Follow => Constraint::ForcedFollow,
        PlayClass::Slough => Constraint::Slough,
    }
}

/// The suffix library of one level, in both declared variants and at each
/// declared depth. **Instrument tier**: neither variant satisfies (ECL) — the
/// open variant even alters chance arities — so no value or class claim may
/// ever be read from a library size.
#[derive(Default)]
pub struct SuffixLibrary {
    /// Indexed by depth minus one: distinct depth-`d` suffixes, multisets kept
    /// everywhere.
    pub strict: Vec<BTreeSet<Vec<u8>>>,
    /// Indexed by depth minus one: distinct depth-`d` suffixes with the option
    /// multiset replaced by the set of distinct option-shapes at unconstrained
    /// nodes only.
    pub open: Vec<BTreeSet<Vec<u8>>>,
}

/// An interning table giving each distinct subtree an exact identity, so hole
/// coincidence inside a suffix is decided by equality of whole subtrees and
/// never by a hash comparison.
#[derive(Default)]
pub struct Identities {
    ids: BTreeMap<Vec<u8>, u64>,
}

impl Identities {
    pub fn of(&mut self, node: &YardNode) -> u64 {
        let key = node.encode();
        let next = self.ids.len() as u64;
        *self.ids.entry(key).or_insert(next)
    }
}

/// The depth-`d` suffix below a node: the decorated subtree cut at depth `d`,
/// with everything below the cut replaced by a hole carrying the cut
/// subtree's identity. Handoff leaves above the cut are holes too. The
/// equality pattern over those holes is recomputed locally by
/// [`yard_shape`] — coincidences crossing the cut are dropped, without which
/// the library is ill defined.
pub fn cut_suffix(node: &YardNode, depth: usize, ids: &mut Identities) -> YardNode {
    if depth == 0 {
        return YardNode::Handoff(ids.of(node));
    }
    match node {
        YardNode::Handoff(_) => YardNode::Handoff(ids.of(node)),
        YardNode::Step { offset, moves } => YardNode::Step {
            offset: *offset,
            moves: moves
                .iter()
                .map(|(increment, class, child)| {
                    (*increment, *class, cut_suffix(child, depth - 1, ids))
                })
                .collect(),
        },
    }
}

/// The open variant: at unconstrained nodes only, the option multiset becomes
/// the set of distinct option-shapes. Forced-follow nodes keep their
/// multisets. Applied bottom up, so an option's shape is already in its own
/// open form when duplicates are collapsed.
pub fn open_variant(node: &YardNode) -> YardNode {
    match node {
        YardNode::Handoff(s) => YardNode::Handoff(*s),
        YardNode::Step { offset, moves } => {
            let mut opened: Vec<(u8, PlayClass, YardNode)> = moves
                .iter()
                .map(|(i, c, child)| (*i, *c, open_variant(child)))
                .collect();
            opened.sort_by_cached_key(|m| (m.0, class_code(m.1), m.2.encode()));
            if node_constraint(moves).unconstrained() {
                opened.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1 && a.2 == b.2);
            }
            YardNode::Step {
                offset: *offset,
                moves: opened,
            }
        }
    }
}

/// Every node of a tree, root included.
fn walk_nodes<'a>(node: &'a YardNode, out: &mut Vec<&'a YardNode>) {
    out.push(node);
    if let YardNode::Step { moves, .. } = node {
        for (_, _, child) in moves {
            walk_nodes(child, out);
        }
    }
}

/// Cuts every tree at every node and content-addresses the resulting depth-`d`
/// suffixes, in both variants. Returns `None` if any suffix exceeds the
/// declared canonicalization ceiling — stop and report, never approximate.
pub fn suffix_library(trees: &[YardNode], depths: usize) -> Option<SuffixLibrary> {
    let mut library = SuffixLibrary {
        strict: vec![BTreeSet::new(); depths],
        open: vec![BTreeSet::new(); depths],
    };
    let mut ids = Identities::default();
    let mut memo: BTreeMap<(Vec<u8>, bool), Vec<u8>> = BTreeMap::new();
    for tree in trees {
        let mut nodes = Vec::new();
        walk_nodes(tree, &mut nodes);
        for node in nodes {
            for d in 1..=depths {
                let suffix = cut_suffix(node, d, &mut ids);
                for (open, target) in [
                    (false, &mut library.strict[d - 1]),
                    (true, &mut library.open[d - 1]),
                ] {
                    let shaped = if open {
                        open_variant(&suffix)
                    } else {
                        suffix.clone()
                    };
                    let key = (shaped.encode(), open);
                    let entry = match memo.get(&key) {
                        Some(hit) => hit.clone(),
                        None => {
                            let computed = yard_shape(&shaped)?;
                            memo.insert(key, computed.clone());
                            computed
                        }
                    };
                    target.insert(entry);
                }
            }
        }
    }
    Some(library)
}

/// A violation of the mandatory refinement assertion (Q5.1): two situations
/// one r1 class holds that r3 separates. Its existence means an implementation
/// bug or a math error in the ruling — stop and report, never patch.
#[derive(Clone, Debug)]
pub struct RefinementViolation {
    pub r1_class: usize,
    pub a: String,
    pub b: String,
    pub r3_a: usize,
    pub r3_b: usize,
}

/// **Mandatory (Q5.1).** r1's structural transports are componentwise and
/// classification/offset preserving, so r1 is a lawful `(d, Theta)` under Q3's
/// typing and must refine r3: every r1 class must land inside exactly one r3
/// class. Returns the violations, empty when the assertion holds.
pub fn r1_refines_r3(census: &Census, r3: &R3) -> Vec<RefinementViolation> {
    let mut out = Vec::new();
    for (r1_class, members) in census.class_members.iter().enumerate() {
        let first = r3.class_of[members[0]];
        for m in &members[1..] {
            if r3.class_of[*m] != first {
                out.push(RefinementViolation {
                    r1_class,
                    a: census.carrier.states[members[0]].render(),
                    b: census.carrier.states[*m].render(),
                    r3_a: first,
                    r3_b: r3.class_of[*m],
                });
            }
        }
    }
    out
}

/// The r3 step law at one situation, rebuilt from the rules rather than read
/// back from the signature: the ordered per-move tuples (position matching is
/// the transport, Q1b) and, at a hidden node, the exact joint law.
enum R3Law {
    Focal(Vec<MoveTuple>),
    Field(Vec<MoveTuple>, BTreeMap<MoveTuple, Q>),
}

fn r3_law(carrier: &Carrier, r3: &R3, i: usize) -> (u8, R3Law) {
    let sit = &carrier.states[i];
    let offset = actor_offset(sit);
    let order = &r3.moves[i];
    let legal = sit.legal();
    assert_eq!(
        order.len(),
        legal.len(),
        "the move order covers the legal set"
    );
    let mut tuples = Vec::with_capacity(order.len());
    for tile in order {
        assert!(legal.contains(*tile), "a move is a legal play");
        let (increment, next) = sit.step(*tile);
        let successor = next.map(|s| {
            let j = carrier
                .lookup(&s)
                .expect("the carrier is closed under primitive steps");
            r3.class_of[j]
        });
        tuples.push(MoveTuple {
            increment,
            class: sit.play_class(*tile),
            successor,
        });
    }
    if offset == 0 {
        return (offset, R3Law::Focal(tuples));
    }
    let share = q(1, legal.len() as i128);
    let mut law: BTreeMap<MoveTuple, Q> = BTreeMap::new();
    for t in &tuples {
        *law.entry(*t).or_insert_with(|| qi(0)) += share;
    }
    let mass: Q = law.values().copied().sum();
    assert_eq!(mass, qi(1), "a field row is a probability law");
    (offset, R3Law::Field(tuples, law))
}

/// **Mandatory (Q5.2).** An independent (ECL) re-check over the r3 partition
/// with the declared position-matching transports — the same checker shape as
/// r1's, rebuilding every law from the rules with exact rationals. "By
/// construction" is not a receipt.
pub fn check_ecl_r3(carrier: &Carrier, r3: &R3) -> EclVerdict {
    let mut verdict = EclVerdict {
        classes: r3.class_members.len(),
        singleton_classes: r3.class_members.iter().filter(|m| m.len() == 1).count(),
        ..EclVerdict::default()
    };
    for (class, members) in r3.class_members.iter().enumerate() {
        if members.len() < 2 {
            continue;
        }
        verdict.classes_checked += 1;
        let rep = members[0];
        let (rep_offset, rep_law) = r3_law(carrier, r3, rep);
        for &m in &members[1..] {
            verdict.cond1_checks += 1;
            verdict.cond2_checks += 1;
            let (offset, law) = r3_law(carrier, r3, m);
            let failure = if offset != rep_offset {
                Some((
                    "ECL preamble (chair correspondence)".to_string(),
                    format!("actor offset from focal {rep_offset} against {offset}"),
                ))
            } else {
                match (&rep_law, &law) {
                    (R3Law::Focal(a), R3Law::Focal(b)) => focal_r3_failure(a, b),
                    (R3Law::Field(a, la), R3Law::Field(b, lb)) => field_r3_failure(a, b, la, lb),
                    _ => Some((
                        "ECL actor type".to_string(),
                        "one member is focal-to-act and the other is not".to_string(),
                    )),
                }
            };
            if let Some((condition, detail)) = failure {
                verdict.failures.push(EclFailure {
                    class,
                    class_key: format!("{:032x}", r3.class_hash[class]),
                    representative: carrier.states[rep].render(),
                    member: carrier.states[m].render(),
                    condition,
                    detail,
                });
            }
        }
    }
    verdict
}

fn render_tuple(t: &MoveTuple) -> String {
    format!(
        "(k={}e*, {:?}, succ {})",
        t.increment,
        t.class,
        t.successor
            .map_or("TERMINAL".to_string(), |c| format!("class#{c}"))
    )
}

fn focal_r3_failure(a: &[MoveTuple], b: &[MoveTuple]) -> Option<(String, String)> {
    if a.len() != b.len() {
        return Some((
            "ECL condition 1 (legality)".to_string(),
            format!(
                "|A(rep)| = {} but |A(member)| = {} — position matching is not a bijection",
                a.len(),
                b.len()
            ),
        ));
    }
    for (i, (x, y)) in a.iter().zip(b).enumerate() {
        if x != y {
            return Some((
                "ECL condition 2 (commutation, Dirac)".to_string(),
                format!(
                    "abstract action {i}: rep {} against member {} (mass 1/1 each)",
                    render_tuple(x),
                    render_tuple(y)
                ),
            ));
        }
    }
    None
}

fn field_r3_failure(
    a: &[MoveTuple],
    b: &[MoveTuple],
    la: &BTreeMap<MoveTuple, Q>,
    lb: &BTreeMap<MoveTuple, Q>,
) -> Option<(String, String)> {
    if a.len() != b.len() {
        return Some((
            "ECL condition 1 (hidden legality)".to_string(),
            format!("|L(rep)| = {} but |L(member)| = {}", a.len(), b.len()),
        ));
    }
    let mut keys: BTreeSet<&MoveTuple> = la.keys().collect();
    keys.extend(lb.keys());
    for key in keys {
        let x = la.get(key).copied().unwrap_or_else(|| qi(0));
        let y = lb.get(key).copied().unwrap_or_else(|| qi(0));
        if x != y {
            return Some((
                "ECL condition 2 (joint law)".to_string(),
                format!(
                    "event {}: rep mass {} but member mass {} (exact rationals)",
                    render_tuple(key),
                    x,
                    y
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
