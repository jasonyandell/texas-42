//! The watched-feature index (S5c-m3): near-free candidate generation for
//! lesson application, as a pure over-approximation.
//!
//! **The candidate-completeness contract** (walt-math Fork-4 ruling): for
//! every decision `d`, `candidates(d)` is a SUPERSET of every working-set
//! lesson with `lesson_applies(L, d) != None`. The index may exclude a
//! lesson only when the exclusion is provably implied by the gate
//! refusing; a missed entry is a completeness bug, never a soundness bug,
//! because **every candidate still passes the full `lesson_applies` gate
//! before use** — an index that skips the gate is forbidden, and the one
//! application helper here (`appliers`) routes candidates through it.
//!
//! Each watch below carries its refusal proof:
//! - `Hand/SeatIs/DeclIs/RoleIs/PlyIs/HorizonGe/HorizonLe`: a mismatch
//!   fails the corresponding decision-sort cell, so `decision_cells_hold`
//!   is false and the gate returns `None`.
//! - `TrickMin/TrickMax/FiberMax` (from the lesson's stored `DomainSpec`):
//!   a violation fails `DomainSpec::covers`, the gate's first check.
//! - `PoolTile(t)`: the implicant references tile `t` through an atom
//!   whose value column exists only for pool tiles (`vocabulary` builds
//!   holder/team/beater columns from the kernel's pool). With `t` outside
//!   the pool the column is absent, so the cell is unsatisfied at EVERY
//!   fiber world (partial semantics); a pair verdict then matches no world
//!   and the checker verdict is not fiber-valid (fibers are nonempty) —
//!   the gate returns `None` either way. This is the adjudicated
//!   numeric-bound rule instantiated at the decision level: the skip is
//!   sound exactly because the numeric is undefined at every world.
//!
//! Watched conservatively: exp3A control-shape cells and `opp-beaters`
//! bounds have columns at every kernel with per-world definedness, so they
//! contribute NO watch — when in doubt the candidate is emitted.
//! Empty-implicant lessons contribute no cell-derived watches at all and
//! live in the unkeyed always-candidate bucket (the gate, not the
//! implicant, excludes them); their only watches mirror the gate's own
//! `DomainSpec` check.
//!
//! **Invalidation** (Fork 4c): the index is keyed to the vocabulary
//! registry version and each working lesson's (content key, implicant
//! revision); `is_current` compares, and holders rebuild on any mismatch —
//! a `DomainSpec` or implicant change alters the snapshot by construction
//! (the key contains both). The index is a derived view of the working
//! set: rebuilt at will, never a second authority, and discarded on
//! restart like all search state.

use std::collections::BTreeMap;

use walt_core::{Decl, Domino, Seat};

use crate::basin::DomainDecision;
use crate::db::{ContentKey, LessonDb};
use crate::generalize::lesson_applies;
use crate::lesson::{Constraint, Lesson, LessonAtom, NumericAtom, Role};

/// The registered-vocabulary version this index build understands. Bump
/// whenever the registered atom/numeric vocabulary changes shape; every
/// stored index self-identifies and is rebuilt on mismatch.
pub const VOCAB_REGISTRY_VERSION: u32 = 1;

/// One decision-level watched feature (refusal proofs in the module doc).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Watch {
    Hand(usize),
    SeatIs(Seat),
    DeclIs(Decl),
    RoleIs(Role),
    PlyIs(usize),
    HorizonGe(usize),
    HorizonLe(usize),
    TrickMin(usize),
    TrickMax(usize),
    FiberMax(u128),
    PoolTile(Domino),
}

fn watch_holds(w: Watch, d: &DomainDecision) -> bool {
    match w {
        Watch::Hand(h) => d.hand == h,
        Watch::SeatIs(s) => d.seat == s,
        Watch::DeclIs(x) => d.kernel.decl() == x,
        Watch::RoleIs(r) => d.role == r,
        Watch::PlyIs(p) => d.ply == p,
        Watch::HorizonGe(n) => d.horizon >= n,
        Watch::HorizonLe(n) => d.horizon <= n,
        Watch::TrickMin(t) => d.trick_no >= t,
        Watch::TrickMax(t) => d.trick_no <= t,
        Watch::FiberMax(cap) => (d.worlds.len() as u128) <= cap,
        Watch::PoolTile(t) => d.kernel.pool().contains(t),
    }
}

/// The watches of one lesson — only features whose decision-level absence
/// provably makes the gate refuse.
fn watches_of(lesson: &Lesson) -> Vec<Watch> {
    let spec = lesson.basin.domain;
    let mut out = vec![
        Watch::TrickMin(spec.min_trick),
        Watch::TrickMax(spec.max_trick),
        Watch::FiberMax(spec.max_fiber),
    ];
    for cell in &lesson.implicant.cells {
        match cell {
            Constraint::Hand(h) => out.push(Watch::Hand(*h)),
            Constraint::Seat(s) => out.push(Watch::SeatIs(*s)),
            Constraint::Decl(x) => out.push(Watch::DeclIs(*x)),
            Constraint::Role(r) => out.push(Watch::RoleIs(*r)),
            Constraint::Ply(p) => out.push(Watch::PlyIs(*p)),
            Constraint::HorizonGe(n) => out.push(Watch::HorizonGe(*n)),
            Constraint::HorizonLe(n) => out.push(Watch::HorizonLe(*n)),
            Constraint::Atom(atom, _) => match atom {
                LessonAtom::Holder(t) | LessonAtom::Team(t) | LessonAtom::Beaters(t) => {
                    out.push(Watch::PoolTile(*t))
                }
                // Control shapes: columns exist at every kernel with
                // per-world definedness — never indexed (conservative).
                LessonAtom::Ctl(_) => {}
            },
            Constraint::NumericGe(num, _) | Constraint::NumericLe(num, _) => match num {
                NumericAtom::BeatersTotal(t) => out.push(Watch::PoolTile(*t)),
                // Per-world definedness: emit the candidate.
                NumericAtom::OppBeaters => {}
            },
        }
    }
    out
}

/// The built index over one working set.
pub struct WatchIndex {
    registry_version: u32,
    /// (content-key hash, implicant revision) per working entry at build
    /// time — the invalidation snapshot.
    snapshot: Vec<(u64, u32)>,
    /// (archive entry index, watches), one per working-set member.
    entries: Vec<(usize, Vec<Watch>)>,
    /// Level-1 buckets by required hand (the most selective equality watch
    /// when the implicant pins one) — positions into `entries`.
    by_hand: BTreeMap<usize, Vec<usize>>,
    /// Positions with no hand requirement — the always-candidate side,
    /// including every empty-implicant lesson.
    unkeyed: Vec<usize>,
}

impl WatchIndex {
    pub fn build(db: &LessonDb) -> WatchIndex {
        let mut snapshot = Vec::new();
        let mut entries = Vec::new();
        let mut by_hand: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        let mut unkeyed = Vec::new();
        for (entry, lesson) in db.working() {
            snapshot.push((ContentKey::of(lesson).hash(), db.entry(entry).revision));
            let watches = watches_of(lesson);
            let pos = entries.len();
            let hand = watches.iter().find_map(|w| match w {
                Watch::Hand(h) => Some(*h),
                _ => None,
            });
            match hand {
                Some(h) => by_hand.entry(h).or_default().push(pos),
                None => unkeyed.push(pos),
            }
            entries.push((entry, watches));
        }
        WatchIndex {
            registry_version: VOCAB_REGISTRY_VERSION,
            snapshot,
            entries,
            by_hand,
            unkeyed,
        }
    }

    /// Is this index current for the database? False on any registry
    /// version, membership, key, or revision change — holders rebuild.
    pub fn is_current(&self, db: &LessonDb) -> bool {
        if self.registry_version != VOCAB_REGISTRY_VERSION {
            return false;
        }
        let now: Vec<(u64, u32)> = db
            .working()
            .map(|(entry, lesson)| (ContentKey::of(lesson).hash(), db.entry(entry).revision))
            .collect();
        now == self.snapshot
    }

    /// Candidate archive entries for one decision — a superset of every
    /// working lesson the gate would accept (the completeness contract).
    /// Deterministic: ascending archive-entry order.
    pub fn candidates(&self, d: &DomainDecision) -> Vec<usize> {
        let mut out: Vec<usize> = self
            .by_hand
            .get(&d.hand)
            .into_iter()
            .flatten()
            .chain(self.unkeyed.iter())
            .filter(|&&pos| {
                let (_, watches) = &self.entries[pos];
                watches.iter().all(|&w| watch_holds(w, d))
            })
            .map(|&pos| self.entries[pos].0)
            .collect();
        out.sort_unstable();
        out
    }

    /// The number of indexed working-set members.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// The one application path over an index: candidates, then the FULL gate
/// on each — returns (archive entry, matched world indices) per applying
/// lesson, ascending by entry. Skipping the gate is forbidden; this helper
/// exists so no caller is tempted to.
pub fn appliers(index: &WatchIndex, db: &LessonDb, d: &DomainDecision) -> Vec<(usize, Vec<usize>)> {
    index
        .candidates(d)
        .into_iter()
        .filter_map(|entry| lesson_applies(db.representative(entry), d).map(|idx| (entry, idx)))
        .collect()
}
