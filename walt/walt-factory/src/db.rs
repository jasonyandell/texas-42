//! The lesson database (S5c-m3): a working set over an append-only
//! archive, with projected-content identity.
//!
//! **Identity is projected content** (walt-math Fork-8 ruling): the
//! canonical key of a lesson is (canonical implicant cells, verdict,
//! DomainSpec, labels) — never its origin, trace, or discovery epoch, and
//! never its GRADE: labels means the operator pair, and a sampled
//! derivation merges with an exhaustive derivation of the same content
//! (grade is not identity; the entry quotes at the max archived rung). A
//! re-derivation of the same content MERGES: one working-set entry, every
//! derivation (origin + trace + basin + its own grade) appended in the
//! archive. Evidence accumulates; identity does not fork.
//!
//! **The working set is an economy object; the archive is an evidence
//! object** (Fork 5): the archive is monotone append-only — certificates,
//! traces, and origins never leave it — and only working-set membership
//! changes. Deletion is an economy action executed through the ledger
//! (`ledger` module), never here: the only public mutation is `insert`,
//! and working-set removal is crate-private so the token-gated ledger path
//! is the sole deletion route. Readmission is cheap — a deleted lesson's
//! verdict never lapsed — so re-deriving deleted content readmits it
//! without re-proving. Entering a NEW domain always re-verifies: the
//! `DomainSpec` is part of the content key, so a wider-domain claim is a
//! different lesson that only the generalizer can construct.
//!
//! Everything here is a derived view of the stored lessons: keys are
//! recomputed from content on demand, never stored beside it (no second
//! authority), and equality/hashing go through the projection only.

use core::cmp::Ordering;

use crate::lesson::{Lesson, LessonGrade, LessonVerdict};

/// FNV-1a 64-bit over bytes — the deterministic, integer-only content-key
/// hash used for filenames and ledger citations. Collisions are guarded by
/// full canonical-string comparison everywhere identity matters; the hash
/// is a handle, never the identity.
pub fn fnv64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// The label projection used by identity: the operator pair alone (or the
/// checker rung's fixed field/valuation string). **Grade is not identity**
/// (walt-math adjudication): the verification rung, weighting, and sample
/// seeds/draws are evidence of one particular derivation — a sampled
/// derivation and an exhaustive derivation with the same projected content
/// are the SAME lesson, one working-set entry, each verification keeping
/// its own grade in the archive. The entry's quotable grade is the MAX
/// over its archived verifications (`LessonDb::quotable`).
pub fn label_projection(grade: &LessonGrade) -> String {
    match grade {
        LessonGrade::Worldwise { operator }
        | LessonGrade::ExactExpectation { operator, .. }
        | LessonGrade::Sampled { operator, .. } => format!("{operator}"),
        LessonGrade::Checker => {
            "checker (§12.6 exhaustive lumpability, uniform-legal field, q_points valuation)"
                .to_string()
        }
    }
}

/// The grade ladder for the quotable-grade maximum: worldwise >
/// exact-expectation > sampled. The checker rung has no ladder (checker
/// lessons only ever merge with checker derivations).
fn grade_rank(grade: &LessonGrade) -> u8 {
    match grade {
        LessonGrade::Worldwise { .. } => 3,
        LessonGrade::ExactExpectation { .. } => 2,
        LessonGrade::Sampled { .. } => 1,
        LessonGrade::Checker => 0,
    }
}

/// The verdict-kind slug of a lesson — content-derived, used in
/// deterministic certificate filenames.
pub fn verdict_kind(lesson: &Lesson) -> &'static str {
    match lesson.verdict {
        LessonVerdict::Refutation { .. } => "refutation",
        LessonVerdict::Win { .. } => "win",
        LessonVerdict::NotLumpable { .. } => "checker",
    }
}

/// The canonical content key. `canonical` is a deterministic rendering of
/// exactly the identity-bearing projection: verdict, projected labels,
/// verified `DomainSpec`, and the final implicant cells sorted by their
/// canonical rendering (cell order is a trace artifact, not content).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ContentKey {
    pub canonical: String,
}

impl ContentKey {
    pub fn of(lesson: &Lesson) -> ContentKey {
        let mut cells: Vec<String> = lesson
            .implicant
            .cells
            .iter()
            .map(ToString::to_string)
            .collect();
        cells.sort();
        let cells = if cells.is_empty() {
            "  (empty)".to_string()
        } else {
            cells
                .iter()
                .map(|c| format!("  {c}"))
                .collect::<Vec<_>>()
                .join("\n")
        };
        ContentKey {
            canonical: format!(
                "content-key-v1\nverdict: {}\nlabels: {}\ndomain: {}\ncells:\n{}",
                lesson.verdict,
                label_projection(&lesson.grade),
                lesson.basin.domain,
                cells
            ),
        }
    }

    /// The FNV-1a handle of the canonical string (filenames, citations).
    pub fn hash(&self) -> u64 {
        fnv64(self.canonical.as_bytes())
    }
}

/// One archived content: every derivation of the same content key, in
/// arrival order (append-only — evidence is monotone).
pub struct ArchiveEntry {
    pub derivations: Vec<Lesson>,
    /// Implicant revision for index keying (Fork 4c). Lessons are
    /// immutable in every current path, so this stays 0; the field keys
    /// the index-invalidation contract, not a live edit path.
    pub revision: u32,
}

/// The outcome of one insertion — every re-derivation is visible as data.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InsertOutcome {
    /// First derivation of this content: a new working-set member.
    New { entry: usize },
    /// Same content re-derived while in the working set: merged — one
    /// entry, another derivation archived.
    Merged { entry: usize, derivations: usize },
    /// Same content re-derived after deletion: merged and READMITTED —
    /// the verdict never lapsed (Fork 5a). The caller records the
    /// readmission in the ledger; rent collection simply resumes.
    Readmitted { entry: usize, derivations: usize },
}

/// The database: append-only archive + working-set membership.
#[derive(Default)]
pub struct LessonDb {
    archive: Vec<ArchiveEntry>,
    /// Archive indices currently in the working set, in admission order
    /// (an archive index doubles as certificate age: smaller = older).
    working: Vec<usize>,
}

impl LessonDb {
    pub fn new() -> LessonDb {
        LessonDb::default()
    }

    /// The one public mutation besides ledger-driven removal: archive the
    /// derivation under its content key and keep the working set merged.
    pub fn insert(&mut self, lesson: Lesson) -> InsertOutcome {
        let key = ContentKey::of(&lesson);
        if let Some(entry) = self
            .archive
            .iter()
            .position(|e| ContentKey::of(&e.derivations[0]) == key)
        {
            self.archive[entry].derivations.push(lesson);
            let derivations = self.archive[entry].derivations.len();
            if self.working.contains(&entry) {
                InsertOutcome::Merged { entry, derivations }
            } else {
                self.working.push(entry);
                InsertOutcome::Readmitted { entry, derivations }
            }
        } else {
            self.archive.push(ArchiveEntry {
                derivations: vec![lesson],
                revision: 0,
            });
            let entry = self.archive.len() - 1;
            self.working.push(entry);
            InsertOutcome::New { entry }
        }
    }

    /// Working-set members as (archive entry index, representative
    /// lesson), in admission order. The representative is the FIRST
    /// derivation — a derived view; later derivations are evidence with
    /// identical content projection.
    pub fn working(&self) -> impl Iterator<Item = (usize, &Lesson)> {
        self.working
            .iter()
            .map(|&i| (i, &self.archive[i].derivations[0]))
    }

    pub fn working_len(&self) -> usize {
        self.working.len()
    }

    pub fn archive_len(&self) -> usize {
        self.archive.len()
    }

    pub fn entry(&self, entry: usize) -> &ArchiveEntry {
        &self.archive[entry]
    }

    pub fn representative(&self, entry: usize) -> &Lesson {
        &self.archive[entry].derivations[0]
    }

    /// The quotable derivation of an entry: the archived verification with
    /// the MAXIMUM grade (worldwise > exact-expectation > sampled; ties to
    /// the earliest derivation). Grade is not identity — derivations at
    /// different rungs merge under one content key, and the entry quotes
    /// at the strongest verified rung while every derivation keeps its own
    /// grade in the archive. A derived view, never stored.
    pub fn quotable(&self, entry: usize) -> &Lesson {
        let derivations = &self.archive[entry].derivations;
        let mut best = &derivations[0];
        for l in &derivations[1..] {
            if grade_rank(&l.grade) > grade_rank(&best.grade) {
                best = l;
            }
        }
        best
    }

    pub fn in_working(&self, entry: usize) -> bool {
        self.working.contains(&entry)
    }

    /// Working-set removal — crate-private on purpose: the only public
    /// deletion route is the ledger's, which enforces the H-checker
    /// sequencing law before calling this. Returns whether the entry was
    /// present. The archive is untouched (deletion is never an evidence
    /// action).
    pub(crate) fn remove_from_working(&mut self, entry: usize) -> bool {
        match self.working.iter().position(|&i| i == entry) {
            Some(pos) => {
                self.working.remove(pos);
                true
            }
            None => false,
        }
    }

    /// The declared deterministic tie-break for deletion among mutually
    /// redundant lessons (Fork 7): smallest implicant (cell count), then
    /// oldest certificate (archive insertion order — a total order, so the
    /// result is deterministic).
    pub fn tie_break(&self, a: usize, b: usize) -> Ordering {
        let cells = |i: usize| self.archive[i].derivations[0].implicant.cells.len();
        cells(a).cmp(&cells(b)).then(a.cmp(&b))
    }
}
