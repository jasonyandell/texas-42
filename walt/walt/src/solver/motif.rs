//! `solver::motif` — the six-motif first-split morphology classifier over
//! correction traces, and the raw post-split suffix enrichment (x:024
//! Part 3). [L2 thread]
//!
//! EXPLORATORY tier. Implements Part 3 (§§3.1–3.9) of the x:024 response
//! (`exchange/inbox/024-response-deferred-producers-triple-v0.1.md`),
//! adopted by rulings TRIPLE-A6/A7 (`walt/CENSUS-RULINGS.md`, "The
//! deferred-producers adjudication (2026-08-25)"); intake companion
//! `walt/math/response_deferred_producers_triple_v0.1_intake.md`. Nothing
//! is promoted by being implemented: every statement here stays
//! exploratory instrument tier.
//!
//! Structural commitments:
//!
//! - **Motifs partition CORRECTION MASS, never field exposure (§3.1;
//!   TRIPLE-A6 BINDING).** [`FieldSplitTrace`] records exist only for
//!   worlds with `u0 ≠ u1`, so every aggregate built here decomposes the
//!   correction event `C⁺ ∪̇ C⁻`. "Motif k accounts for this fraction of
//!   field exposure" is unsupported until non-pivotal exposed worlds are
//!   classified too — no type in this module can state it.
//! - **Root semantics resolve or the classifier declines (§3.2;
//!   TRIPLE-A6).** Classification requires `(root_id,
//!   root_semantics_hash)` to resolve against an immutable [`RootFrame`];
//!   failure returns the residual with
//!   [`ResidualReason::MissingRootFrame`]. It never guesses.
//! - **The ordering is a taxonomy convention, not a causal ranking
//!   (§3.6).** The primary label is the least-index differing coordinate
//!   of the six-coordinate local signature; several coordinates may
//!   differ at once, and the mandatory [`MotifFlags`] retain those facts.
//!   A `LeadContextFork` label never says the lead context CAUSED the
//!   terminal correction.
//! - **The enrichment is raw and replayable (§3.9; TRIPLE-A7).** The
//!   persisted additions are the two post-split play suffixes and the
//!   root semantics hash — derived-from-execution data only. NO motif tag
//!   and NO judgment label is persisted on any trace record;
//!   `RevealResponse` (and every causal/response label) remains REFUSED
//!   (§3.8). The suffixes are the prerequisite for a possible later
//!   `PartnerResponseCandidate` vocabulary, nothing more. They also close
//!   the flagged item-11 gap: the distinguishing public observation
//!   becomes explicit rather than implicit in (tile0, tile1, history).
//! - **Refused aggregates (§3.7, adopted verbatim by TRIPLE-A6):** no
//!   "motif k caused the win"; no good/bad-play labels pooled across
//!   roots; no field-exposure mass by motif from correction-only traces;
//!   no dominance labels from sampled motif hazards; no unweighted motif
//!   rates pooled across fibers, bids, fields, or policy identities; and
//!   no invented numerical residual-rate forecast — the residual is an
//!   instrument, not an embarrassment (§3.6).
//! - **Sampled-prefix histograms are descriptive, never screen inputs
//!   (§3.7).** [`DescriptiveMotifHistogram`] has no screenable accessor
//!   and no conversion to any exposure-bound type; the exact
//!   decomposition [`ExactMotifDecomposition`] is inhabited only by
//!   exact-fiber correction traces beside their [`CancellationLadder`].
//!
//! The safe phrasing for every number produced here (§3.7): "Among exact
//! correction worlds for this root, field pair, and frozen policy, the
//! first mechanical split had motif k on mass m_k."

use std::collections::BTreeMap;
use std::fmt;

use num_bigint::BigInt;
use num_rational::BigRational;

use crate::kernel::World;
use crate::rules::rules::{legal_plays, Trick, TrickKey};
use crate::rules::{Context, Decl, Domino, DominoSet, Seat, Team};
use crate::solver::adaptive::{root_identity, CanonicalRoot, PublicRecord, RootPosition};
use crate::solver::exposure::{FirstSplit, FrozenPolicyExposure, WorldDomain};
use crate::solver::field::{FieldId, FieldModel};
use crate::solver::field_swap::{CancellationLadder, FieldSplitTrace};
use crate::solver::policy::{FrozenPolicy, PolicyId};

// ---------------------------------------------------------------------------
// The immutable root frame and its semantics hash (§3.2).
// ---------------------------------------------------------------------------

/// The rule version pinned into every root semantics hash. The rule
/// algebra this module derives signatures through is v0.4 §1.2–§1.5
/// (`rules::rules`).
pub const RULE_VERSION: &str = "straight-42-v0.4";

/// §3.2 — the immutable root-semantic record the classifier resolves
/// against: declaration/trump, the contract, the partnership map and
/// focal seat, the root trick frame, and the rule version. Every field is
/// public mechanical semantics; nothing here is belief data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RootFrame {
    pub rule_version: &'static str,
    pub decl: Decl,
    pub bid: u32,
    pub declaring_team: Team,
    /// The focal (viewer) seat — with the fixed seat→team map this is the
    /// complete partnership frame.
    pub viewer: Seat,
    /// Leader of the trick in progress at the root.
    pub leader: Seat,
    /// Banked points at the root, team-indexed.
    pub banked: [u32; 2],
    /// The root's partial trick in play order.
    pub trick_plays: Vec<Domino>,
    /// Tiles played in tricks completed before the root.
    pub prior_played: DominoSet,
}

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

fn fnv_byte(h: u64, byte: u8) -> u64 {
    (h ^ u64::from(byte)).wrapping_mul(FNV_PRIME)
}

fn fnv_u64(mut h: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        h = fnv_byte(h, byte);
    }
    h
}

/// A stable code for the declaration (independent of any derive order):
/// pip trumps by pip value, doubles-trump 7, no-trump 8.
fn decl_code(decl: Decl) -> u64 {
    match decl {
        Decl::PipTrump(p) => u64::from(p.value()),
        Decl::DoublesTrump => 7,
        Decl::NoTrump => 8,
    }
}

impl RootFrame {
    /// The frame of one canonical root: public position plus the kernel's
    /// focal seat, at the pinned rule version.
    pub fn of(root: &CanonicalRoot, position: &RootPosition) -> RootFrame {
        RootFrame {
            rule_version: RULE_VERSION,
            decl: position.decl,
            bid: position.bid,
            declaring_team: position.declaring_team,
            viewer: root.kernel().viewer(),
            leader: position.leader,
            banked: position.banked,
            trick_plays: position.trick_plays.clone(),
            prior_played: position.prior_played,
        }
    }

    /// The deterministic semantics hash (FNV-1a 64 over a canonical byte
    /// serialization — plain integers, stable across runs and builds).
    pub fn semantics_hash(&self) -> u64 {
        let mut h = FNV_OFFSET;
        for byte in self.rule_version.as_bytes() {
            h = fnv_byte(h, *byte);
        }
        h = fnv_u64(h, decl_code(self.decl));
        h = fnv_u64(h, u64::from(self.bid));
        h = fnv_u64(h, self.declaring_team.index() as u64);
        h = fnv_u64(h, self.viewer.index() as u64);
        h = fnv_u64(h, self.leader.index() as u64);
        h = fnv_u64(h, u64::from(self.banked[0]));
        h = fnv_u64(h, u64::from(self.banked[1]));
        h = fnv_u64(h, self.trick_plays.len() as u64);
        for d in &self.trick_plays {
            h = fnv_u64(h, d.index() as u64);
        }
        h = fnv_u64(h, u64::from(self.prior_played.bits()));
        h
    }
}

/// §3.2 — the resolution table `(root_id, root_semantics_hash) →
/// immutable RootFrame`. Insertion is keyed by the frame's OWN hash;
/// resolution requires both keys to match. There is no fallback and no
/// nearest match: a failed lookup is the classifier's
/// `Other(missing_root_frame)` residual, never a guess (TRIPLE-A6).
#[derive(Default)]
pub struct RootFrameRegistry {
    frames: BTreeMap<(u64, u64), RootFrame>,
}

impl RootFrameRegistry {
    pub fn new() -> RootFrameRegistry {
        RootFrameRegistry {
            frames: BTreeMap::new(),
        }
    }

    /// Register one root's frame under `(root_id, semantics_hash)`.
    /// Re-registration must be identical (the frame is immutable).
    pub fn register(&mut self, root_id: u64, frame: RootFrame) -> u64 {
        let hash = frame.semantics_hash();
        if let Some(existing) = self.frames.get(&(root_id, hash)) {
            assert_eq!(*existing, frame, "a registered root frame is immutable");
        } else {
            self.frames.insert((root_id, hash), frame);
        }
        hash
    }

    /// The §3.2 lookup. `None` NEVER falls through to a guess.
    pub fn resolve(&self, root_id: u64, semantics_hash: u64) -> Option<&RootFrame> {
        self.frames.get(&(root_id, semantics_hash))
    }
}

// ---------------------------------------------------------------------------
// The raw suffix enrichment (§3.9; TRIPLE-A7).
// ---------------------------------------------------------------------------

/// §3.9 — the enriched trace record: the slice-3 [`FieldSplitTrace`] plus
/// the two post-split play suffixes and the root semantics hash. Raw
/// derived-from-execution data ONLY: no motif tag, no judgment label, no
/// causal field of any kind is persisted here (TRIPLE-A7). Each suffix
/// lists the plays STRICTLY AFTER the split play as (actor, tile) pairs —
/// the split play itself is already the parent trace's (seat, tile0) /
/// (seat, tile1). Probe records produced before this enrichment lack
/// these fields; they remain readable as bare traces and are never
/// back-filled by guesswork.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnrichedFieldSplitTrace {
    pub trace: FieldSplitTrace,
    /// Branch-0 plays after the split play, in play order.
    pub branch0_suffix: Vec<(Seat, Domino)>,
    /// Branch-1 plays after the split play, in play order.
    pub branch1_suffix: Vec<(Seat, Domino)>,
    /// [`RootFrame::semantics_hash`] of the frame the enrichment ran
    /// under.
    pub root_semantics_hash: u64,
}

impl fmt::Display for EnrichedFieldSplitTrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suffix = |plays: &[(Seat, Domino)]| -> String {
            plays
                .iter()
                .map(|(s, d)| format!("{s}:{d}"))
                .collect::<Vec<_>>()
                .join(",")
        };
        write!(
            f,
            "EnrichedFieldSplitTrace{{root={:#018x};semantics={:#018x};policy={};\
             action={};seat={};trick={};ply={};tile0={};tile1={};u0={};u1={};\
             suffix0=[{}];suffix1=[{}]}}",
            self.trace.root_id,
            self.root_semantics_hash,
            self.trace.policy,
            self.trace.action,
            self.trace.split.seat,
            self.trace.split.trick,
            self.trace.split.ply,
            self.trace.split.tile0,
            self.trace.split.tile1,
            self.trace.u0,
            self.trace.u1,
            suffix(&self.branch0_suffix),
            suffix(&self.branch1_suffix),
        )
    }
}

/// One branch execution of the enrichment replay, evolved from the root by
/// legal plays — the same semantics as `solver::exposure`'s coupled
/// replay, re-derived here so the suffixes are captured without giving the
/// exposure producer a second responsibility. Consistency between the two
/// derivations is ASSERTED per world (split coordinates and both
/// terminals), so neither copy can drift silently.
struct Replay {
    hands: [DominoSet; 4],
    leader: Seat,
    plays: Vec<Domino>,
    banked: [u32; 2],
    history: Vec<Domino>,
}

impl Replay {
    fn start(position: &RootPosition, world: &World) -> Replay {
        Replay {
            hands: world.hands(),
            leader: position.leader,
            plays: position.trick_plays.clone(),
            banked: position.banked,
            history: Vec::new(),
        }
    }

    fn done(&self) -> bool {
        self.hands.iter().all(|h| h.is_empty())
    }

    fn seat(&self) -> Seat {
        self.leader.plus(self.plays.len())
    }

    fn record<'a>(&'a self, position: &'a RootPosition) -> PublicRecord<'a> {
        PublicRecord {
            leader: self.leader,
            trick_plays: &self.plays,
            banked: self.banked,
            root: position,
            history: &self.history,
        }
    }

    fn play(&mut self, position: &RootPosition, tile: Domino) {
        let seat = self.seat();
        assert!(
            self.hands[seat.index()].remove(tile),
            "the chosen tile is held"
        );
        self.plays.push(tile);
        self.history.push(tile);
        if self.plays.len() == 4 {
            let doms: [Domino; 4] = core::array::from_fn(|i| self.plays[i]);
            let trick = Trick::new(self.leader, doms).expect("four distinct tiles");
            let winner = trick.winner(position.decl);
            self.banked[winner.team().index()] += trick.points();
            self.leader = winner;
            self.plays.clear();
        }
    }

    fn success(&self, position: &RootPosition, viewer: Seat) -> bool {
        assert!(self.done() && self.plays.is_empty(), "a terminal state");
        let made = self.banked[position.declaring_team.index()] >= position.bid;
        if viewer.team() == position.declaring_team {
            made
        } else {
            !made
        }
    }

    /// Drive to terminal, recording every play from here on as an
    /// (actor, tile) suffix entry.
    fn run_recording(
        &mut self,
        position: &RootPosition,
        viewer: Seat,
        focal: &FrozenPolicy,
        field: &FieldModel,
        suffix: &mut Vec<(Seat, Domino)>,
    ) {
        use crate::solver::adaptive::SlicePolicy as _;
        while !self.done() {
            let seat = self.seat();
            let led = self.plays.first().map(|d| position.decl.led_context(*d));
            let hand = self.hands[seat.index()];
            let legal = legal_plays(position.decl, hand, led);
            assert!(!legal.is_empty(), "a seat to move holds a legal tile");
            let record = self.record(position);
            let tile = if seat == viewer {
                focal.choose(position.decl, hand, legal, &record)
            } else {
                field.choose(position.decl, hand, legal, &record)
            };
            assert!(legal.contains(tile), "a policy chooses a legal tile");
            suffix.push((seat, tile));
            self.play(position, tile);
        }
    }
}

/// §3.9 — the enrichment producer: re-derive, for every CORRECTION world
/// of one exposure result, the two post-split play suffixes, by replaying
/// the coupled execution under the same frozen focal policy and field
/// pair. Every re-derived quantity the exposure already recorded (the
/// split's seat, trick, ply, tiles, hand, history, and both terminals) is
/// asserted equal to the exposure's row — the enrichment can only agree
/// or die, never diverge silently. Returns one enriched trace per world
/// with `u0 ≠ u1`, in row order (the same set `field_split_traces`
/// covers).
pub fn enrich_field_split_traces(
    root: &CanonicalRoot,
    position: &RootPosition,
    action: Domino,
    focal: &FrozenPolicy,
    field0: &FieldModel,
    field1: &FieldModel,
    exposure: &FrozenPolicyExposure,
) -> Vec<EnrichedFieldSplitTrace> {
    use crate::solver::adaptive::SlicePolicy as _;
    assert_eq!(exposure.policy, focal.policy_id(), "the exposure's policy");
    assert_eq!(exposure.field0, field0.field_id(), "the exposure's σ0");
    assert_eq!(exposure.field1, field1.field_id(), "the exposure's σ1");
    let root_id = root_identity(root, position);
    assert_eq!(exposure.root_id, root_id, "the exposure's root");
    let frame = RootFrame::of(root, position);
    let semantics = frame.semantics_hash();
    let viewer = root.kernel().viewer();
    // Reconstruct the declared domain's worlds by index, exactly as the
    // exposure producer enumerated them.
    let worlds: Vec<World> = match &exposure.domain {
        WorldDomain::ExactFiber => root.worlds().collect(),
        WorldDomain::StreamPrefix { epoch, worlds } => (0..*worlds)
            .map(|i| root.world_at(root_id, *epoch, i))
            .collect(),
    };
    assert_eq!(
        worlds.len(),
        exposure.rows.len(),
        "the enrichment walks the exposure's own domain"
    );
    let mut enriched: Vec<EnrichedFieldSplitTrace> = Vec::new();
    for (row, world) in exposure.rows.iter().zip(&worlds) {
        if row.u0 == row.u1 {
            continue;
        }
        let split = row
            .split
            .as_ref()
            .expect("L2-T1: a changed outcome occurs only on an exposed world");
        let mut e0 = Replay::start(position, world);
        let mut e1 = Replay::start(position, world);
        // Coupled pre-split walk: both branches replay identically until
        // the recorded split point, asserted step by step.
        loop {
            let seat = e0.seat();
            let led = e0.plays.first().map(|d| position.decl.led_context(*d));
            let hand = e0.hands[seat.index()];
            let legal = legal_plays(position.decl, hand, led);
            assert!(!legal.is_empty(), "a seat to move holds a legal tile");
            if seat == viewer {
                let a0 = focal.choose(position.decl, hand, legal, &e0.record(position));
                e0.play(position, a0);
                e1.play(position, a0);
                continue;
            }
            let t0 = field0.choose(position.decl, hand, legal, &e0.record(position));
            let t1 = field1.choose(position.decl, hand, legal, &e1.record(position));
            if t0 == t1 {
                e0.play(position, t0);
                e1.play(position, t1);
                continue;
            }
            // The replayed split must BE the recorded split, field by
            // field.
            assert_eq!(seat, split.seat, "the replayed split seat agrees");
            assert_eq!(
                (t0, t1),
                (split.tile0, split.tile1),
                "the replayed split tiles agree"
            );
            assert_eq!(hand, split.hand, "the replayed split hand agrees");
            assert_eq!(
                e0.history, split.history,
                "the replayed pre-split history agrees"
            );
            break;
        }
        e0.play(position, split.tile0);
        e1.play(position, split.tile1);
        let mut suffix0: Vec<(Seat, Domino)> = Vec::new();
        let mut suffix1: Vec<(Seat, Domino)> = Vec::new();
        e0.run_recording(position, viewer, focal, field0, &mut suffix0);
        e1.run_recording(position, viewer, focal, field1, &mut suffix1);
        let (u0, u1) = (
            e0.success(position, viewer),
            e1.success(position, viewer),
        );
        assert_eq!(
            (u0, u1),
            (row.u0, row.u1),
            "the enrichment replay reproduces both recorded terminals"
        );
        // Both suffixes exhaust the remaining tiles: the hand holds 28
        // tiles less the prior mask, the root trick, the pre-split
        // history, and the split play itself.
        let remaining = Domino::COUNT
            - position.prior_played.len()
            - position.trick_plays.len()
            - split.history.len()
            - 1;
        assert_eq!(suffix0.len(), remaining, "branch 0 plays out the hand");
        assert_eq!(suffix1.len(), remaining, "branch 1 plays out the hand");
        enriched.push(EnrichedFieldSplitTrace {
            trace: FieldSplitTrace {
                root_id,
                world: row.world,
                action,
                policy: exposure.policy,
                field0: exposure.field0,
                field1: exposure.field1,
                split: split.clone(),
                u0: row.u0,
                u1: row.u1,
            },
            branch0_suffix: suffix0,
            branch1_suffix: suffix1,
            root_semantics_hash: semantics,
        });
    }
    assert_eq!(
        u64::try_from(enriched.len()).expect("fits"),
        exposure.corrections_plus + exposure.corrections_minus,
        "one enriched trace per correction world"
    );
    enriched
}

// ---------------------------------------------------------------------------
// The six-coordinate local split signature (§3.3).
// ---------------------------------------------------------------------------

/// §3.3 — the ordered local signature Σ(t) of one candidate tile at the
/// first split: (next led context, provisional winning partnership,
/// count payload, effective-trump flag, residual suit shape,
/// declaration-relative played strength). Field order IS the taxonomy's
/// coordinate order; every coordinate is a total function of (frame,
/// split state, tile).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SplitSignature {
    /// `L(t)` — the trick context established (on lead) or inherited.
    pub led: Context,
    /// `W(t)` — the partnership provisionally winning after `r · t`.
    pub control: Team,
    /// `C(t)` — the count payload the tile commits to the trick.
    pub count: u32,
    /// `T(t)` — whether the tile is effective trump (called).
    pub trump: bool,
    /// `S(t)` — residual effective-context length profile of the acting
    /// hand after the tile leaves it, indexed by [`Context::ALL`].
    pub shape: [u8; 8],
    /// `K(t)` — the declaration-relative trick key of the tile in its
    /// active context.
    pub strength: TrickKey,
}

impl fmt::Display for SplitSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Sigma{{led={};control=T{};count={};trump={};shape={:?};\
             strength=({:?},{})}}",
            self.led,
            self.control.index(),
            self.count,
            self.trump,
            self.shape,
            self.strength.tier,
            self.strength.rank.value(),
        )
    }
}

/// The trick frame at the split, derived by replaying the trace's public
/// history through the resolved root frame (derived views, never stored
/// state).
struct SplitState {
    leader: Seat,
    plays: Vec<Domino>,
}

/// Replay the split's pre-history through the frame and assert the
/// result IS the recorded split point (seat, trick, ply, held tiles,
/// legality). A resolved frame that contradicts its trace is corruption
/// and dies loudly — the honest decline path is resolution FAILURE, which
/// never reaches here.
fn state_at_split(frame: &RootFrame, split: &FirstSplit) -> SplitState {
    let mut leader = frame.leader;
    let mut plays = frame.trick_plays.clone();
    for d in &split.history {
        plays.push(*d);
        if plays.len() == 4 {
            let doms: [Domino; 4] = core::array::from_fn(|i| plays[i]);
            let trick = Trick::new(leader, doms).expect("four distinct tiles");
            leader = trick.winner(frame.decl);
            plays.clear();
        }
    }
    let seat = leader.plus(plays.len());
    assert_eq!(seat, split.seat, "the frame reproduces the split seat");
    let total = frame.prior_played.len() + frame.trick_plays.len() + split.history.len();
    assert_eq!(
        (total / 4 + 1, total % 4),
        (split.trick, split.ply),
        "the frame reproduces the split's trick and ply"
    );
    assert!(
        split.hand.contains(split.tile0) && split.hand.contains(split.tile1),
        "the split hand holds both chosen tiles"
    );
    let led = plays.first().map(|d| frame.decl.led_context(*d));
    let legal = legal_plays(frame.decl, split.hand, led);
    assert!(
        legal.contains(split.tile0) && legal.contains(split.tile1),
        "both split tiles are legal in the reproduced state"
    );
    SplitState { leader, plays }
}

/// §3.3 — compute Σ(t) for one candidate tile at the reproduced split
/// state. Total and deterministic wherever the frame resolves.
fn split_signature(frame: &RootFrame, state: &SplitState, split: &FirstSplit, tile: Domino) -> SplitSignature {
    let decl = frame.decl;
    let current_led = state.plays.first().map(|d| decl.led_context(*d));
    // 1. Next led context: established by the tile on lead, inherited
    //    otherwise.
    let led = current_led.unwrap_or_else(|| decl.led_context(tile));
    // 2. Provisional winning partnership after r · t: the §1.3 argmax
    //    over the partial trick extended by the tile (first strict
    //    maximum, exactly as `Trick::winner`).
    let mut tiles: Vec<Domino> = state.plays.clone();
    tiles.push(tile);
    let trick_led = decl.led_context(tiles[0]);
    let mut best = decl.trick_key(tiles[0], trick_led);
    let mut at = 0usize;
    for (k, d) in tiles.iter().enumerate().skip(1) {
        let key = decl.trick_key(*d, trick_led);
        if key > best {
            best = key;
            at = k;
        }
    }
    let control = state.leader.plus(at).team();
    // 3. Count payload.
    let count = tile.count();
    // 4. Effective trump.
    let trump = decl.is_called(tile);
    // 5. Residual suit shape over the effective contexts.
    let mut residual = split.hand;
    assert!(residual.remove(tile), "the candidate tile is held");
    let mut shape = [0u8; 8];
    for (i, q) in Context::ALL.iter().enumerate() {
        shape[i] = u8::try_from(
            residual.intersection(decl.effective_incidence(*q)).len(),
        )
        .expect("a hand holds at most seven tiles");
    }
    // 6. Declaration-relative played strength in the active context.
    let strength = decl.trick_key(tile, led);
    SplitSignature {
        led,
        control,
        count,
        trump,
        shape,
        strength,
    }
}

// ---------------------------------------------------------------------------
// The primary alphabet, flags, and classifier (§3.4–§3.5; TRIPLE-A6).
// ---------------------------------------------------------------------------

/// §3.4 — the six primary first-split morphology motifs plus the
/// residual. Mutually exclusive by construction (least differing
/// coordinate); exhaustive with `Other`. The variant order IS the
/// coordinate order of [`SplitSignature`] — a taxonomy convention, not a
/// causal ranking (§3.6): an earlier coordinate CAPTURES a trace whose
/// later coordinates also differ, and only the flags retain those facts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SplitMotif {
    /// M1 — the fields establish different led contexts.
    LeadContextFork,
    /// M2 — same context, different provisional winning partnership.
    ImmediateControlFork,
    /// M3 — same control, different count payload committed.
    CountCommitmentFork,
    /// M4 — same count coordinate, one field spends trump.
    TrumpCommitmentFork,
    /// M5 — same trump coordinate, different residual suit shapes.
    SuitShapeFork,
    /// M6 — everything above agrees, different rank strength spent.
    StrengthCommitmentFork,
    /// The residual: no declared coordinate differs, or the derivation is
    /// unavailable. No nearest readable label is substituted.
    Other,
}

impl SplitMotif {
    pub const ALL: [SplitMotif; 7] = [
        SplitMotif::LeadContextFork,
        SplitMotif::ImmediateControlFork,
        SplitMotif::CountCommitmentFork,
        SplitMotif::TrumpCommitmentFork,
        SplitMotif::SuitShapeFork,
        SplitMotif::StrengthCommitmentFork,
        SplitMotif::Other,
    ];

    /// The mechanical type tag, always the serialization's prefix.
    pub fn tag(self) -> &'static str {
        match self {
            SplitMotif::LeadContextFork => "LeadContextFork",
            SplitMotif::ImmediateControlFork => "ImmediateControlFork",
            SplitMotif::CountCommitmentFork => "CountCommitmentFork",
            SplitMotif::TrumpCommitmentFork => "TrumpCommitmentFork",
            SplitMotif::SuitShapeFork => "SuitShapeFork",
            SplitMotif::StrengthCommitmentFork => "StrengthCommitmentFork",
            SplitMotif::Other => "Other",
        }
    }

    /// Index into [`SplitMotif::ALL`] — the histogram axis.
    pub fn index(self) -> usize {
        match self {
            SplitMotif::LeadContextFork => 0,
            SplitMotif::ImmediateControlFork => 1,
            SplitMotif::CountCommitmentFork => 2,
            SplitMotif::TrumpCommitmentFork => 3,
            SplitMotif::SuitShapeFork => 4,
            SplitMotif::StrengthCommitmentFork => 5,
            SplitMotif::Other => 6,
        }
    }
}

impl fmt::Display for SplitMotif {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tag())
    }
}

/// Why a trace landed in the residual (§3.4's two `Other` routes, kept
/// mechanically distinct).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResidualReason {
    /// Both signatures computed and agree on every coordinate: the local
    /// alphabet does not explain the split at this resolution.
    NoCoordinateDiffers,
    /// `(root_id, root_semantics_hash)` did not resolve; the signature is
    /// underivable and the classifier declines (§3.2 — never guessed).
    MissingRootFrame,
}

impl ResidualReason {
    pub fn tag(self) -> &'static str {
        match self {
            ResidualReason::NoCoordinateDiffers => "no_coordinate_differs",
            ResidualReason::MissingRootFrame => "missing_root_frame",
        }
    }
}

impl fmt::Display for ResidualReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tag())
    }
}

/// §3.5 — the mandatory orthogonal coordinate-difference flags. The flags
/// may co-occur; they are derived views of the same signature pair, never
/// a second authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MotifFlags {
    pub diff_context: bool,
    pub diff_control: bool,
    pub diff_count: bool,
    pub diff_trump: bool,
    pub diff_suit_shape: bool,
    pub diff_strength: bool,
}

impl MotifFlags {
    pub fn any(&self) -> bool {
        self.diff_context
            || self.diff_control
            || self.diff_count
            || self.diff_trump
            || self.diff_suit_shape
            || self.diff_strength
    }

    /// The flags in coordinate order — the same axis as
    /// [`SplitMotif::ALL`]'s first six entries.
    pub fn ordered(&self) -> [bool; 6] {
        [
            self.diff_context,
            self.diff_control,
            self.diff_count,
            self.diff_trump,
            self.diff_suit_shape,
            self.diff_strength,
        ]
    }
}

/// §3.5 — the split actor's partnership relative to the focal seat
/// (derivable only when the root frame resolves).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplitActorRelation {
    Partner,
    Opponent,
}

impl fmt::Display for SplitActorRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SplitActorRelation::Partner => write!(f, "partner"),
            SplitActorRelation::Opponent => write!(f, "opponent"),
        }
    }
}

/// §3.5 — which field the terminal correction favors. On a correction
/// trace `u0 ≠ u1`, so the sign is total.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerminalSign {
    FavorsField1,
    FavorsField0,
}

impl fmt::Display for TerminalSign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TerminalSign::FavorsField1 => write!(f, "favors_field1"),
            TerminalSign::FavorsField0 => write!(f, "favors_field0"),
        }
    }
}

/// One trace's complete classification: the primary label, the residual
/// reason when primary is `Other`, and — whenever the root frame resolved
/// — the flags, both raw signatures, the split actor relation, and the
/// terminal sign. Everything beyond `motif` is a derived view of the same
/// signature pair and trace; nothing is a second authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MotifClassification {
    pub motif: SplitMotif,
    /// `Some` exactly when `motif == Other`.
    pub residual: Option<ResidualReason>,
    /// `None` exactly on the `missing_root_frame` residual (no signature
    /// is derivable there).
    pub flags: Option<MotifFlags>,
    /// The raw signature pair (Σ(t0), Σ(t1)) — retained so `Other`'s raw
    /// pairs are reportable per §3.6.
    pub signatures: Option<(SplitSignature, SplitSignature)>,
    pub split_actor_relation: Option<SplitActorRelation>,
    pub terminal_sign: Option<TerminalSign>,
}

/// §3.4 — the primary rule on one signature pair: the FIRST (least-index)
/// differing coordinate names the motif; no differing coordinate is the
/// residual. Deterministic and total (P6: every classifiable pair gets
/// exactly one primary label).
pub fn classify_signature_pair(
    s0: &SplitSignature,
    s1: &SplitSignature,
) -> (SplitMotif, Option<ResidualReason>, MotifFlags) {
    let flags = MotifFlags {
        diff_context: s0.led != s1.led,
        diff_control: s0.control != s1.control,
        diff_count: s0.count != s1.count,
        diff_trump: s0.trump != s1.trump,
        diff_suit_shape: s0.shape != s1.shape,
        diff_strength: s0.strength != s1.strength,
    };
    let ordered = flags.ordered();
    let motif = match ordered.iter().position(|d| *d) {
        Some(0) => SplitMotif::LeadContextFork,
        Some(1) => SplitMotif::ImmediateControlFork,
        Some(2) => SplitMotif::CountCommitmentFork,
        Some(3) => SplitMotif::TrumpCommitmentFork,
        Some(4) => SplitMotif::SuitShapeFork,
        Some(5) => SplitMotif::StrengthCommitmentFork,
        Some(_) => unreachable!("six coordinates"),
        None => SplitMotif::Other,
    };
    let residual = (motif == SplitMotif::Other).then_some(ResidualReason::NoCoordinateDiffers);
    // P6 coherence: `Other` here exactly when no flag is set.
    assert_eq!(motif == SplitMotif::Other, !flags.any());
    (motif, residual, flags)
}

/// §3.2–§3.5 — classify one enriched correction trace against the
/// registry. Resolution failure returns the residual with
/// `missing_root_frame` and derives NOTHING else; a resolved frame yields
/// the full signature pair, primary label, flags, actor relation, and
/// terminal sign.
pub fn classify_trace(
    enriched: &EnrichedFieldSplitTrace,
    registry: &RootFrameRegistry,
) -> MotifClassification {
    let trace = &enriched.trace;
    assert!(
        trace.u0 != trace.u1,
        "the present domain is correction traces (§3.1)"
    );
    let Some(frame) = registry.resolve(trace.root_id, enriched.root_semantics_hash) else {
        return MotifClassification {
            motif: SplitMotif::Other,
            residual: Some(ResidualReason::MissingRootFrame),
            flags: None,
            signatures: None,
            split_actor_relation: None,
            terminal_sign: None,
        };
    };
    let state = state_at_split(frame, &trace.split);
    let s0 = split_signature(frame, &state, &trace.split, trace.split.tile0);
    let s1 = split_signature(frame, &state, &trace.split, trace.split.tile1);
    let (motif, residual, flags) = classify_signature_pair(&s0, &s1);
    let relation = if trace.split.seat.team() == frame.viewer.team() {
        SplitActorRelation::Partner
    } else {
        SplitActorRelation::Opponent
    };
    let sign = if trace.u1 {
        TerminalSign::FavorsField1
    } else {
        TerminalSign::FavorsField0
    };
    MotifClassification {
        motif,
        residual,
        flags: Some(flags),
        signatures: Some((s0, s1)),
        split_actor_relation: Some(relation),
        terminal_sign: Some(sign),
    }
}

// ---------------------------------------------------------------------------
// Aggregate typing (§3.7; TRIPLE-A6 BINDING).
// ---------------------------------------------------------------------------

/// §3.7 — the exact motif decomposition of one EXACT-FIBER correction
/// event: per motif k, the signed masses `m_k⁺ = #{M = k, u1 = 1, u0 = 0}
/// / N` and `m_k⁻`, with the identities `Σ m_k⁺ = c⁺`, `Σ m_k⁻ = c⁻`,
/// `Σ c_k = c` ASSERTED against the ladder at construction. This is a
/// decomposition of CORRECTION MASS under the declared (root, action,
/// policy, field pair) — never field exposure, never a causal claim, and
/// never poolable across fibers, fields, or policy identities (§3.7's
/// refused list).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactMotifDecomposition {
    pub policy: PolicyId,
    pub field0: FieldId,
    pub field1: FieldId,
    pub root_id: u64,
    /// The exact fiber size N — the denominator of every mass.
    pub worlds: u64,
    /// `N · m_k⁺` by [`SplitMotif::ALL`] index.
    pub plus: [u64; 7],
    /// `N · m_k⁻` by [`SplitMotif::ALL`] index.
    pub minus: [u64; 7],
}

impl ExactMotifDecomposition {
    /// Build from one exact-fiber ladder and the classified enriched
    /// traces of the SAME exposure. A sampled-prefix ladder is refused by
    /// assertion — the sampled sibling is [`DescriptiveMotifHistogram`],
    /// which cannot make these identity claims.
    pub fn from_classified(
        ladder: &CancellationLadder,
        classified: &[(EnrichedFieldSplitTrace, MotifClassification)],
    ) -> ExactMotifDecomposition {
        assert_eq!(
            ladder.domain,
            WorldDomain::ExactFiber,
            "the exact decomposition is inhabited only by exact-fiber traces (§3.7)"
        );
        assert_eq!(
            u64::try_from(classified.len()).expect("fits"),
            ladder.outcome_changed,
            "every correction world is classified exactly once"
        );
        let mut plus = [0u64; 7];
        let mut minus = [0u64; 7];
        for (enriched, classification) in classified {
            let trace = &enriched.trace;
            assert_eq!(trace.root_id, ladder.root_id, "one root");
            assert_eq!(trace.policy, ladder.policy, "one policy");
            assert_eq!(trace.field0, ladder.field0, "one σ0");
            assert_eq!(trace.field1, ladder.field1, "one σ1");
            let k = classification.motif.index();
            if trace.u1 {
                plus[k] += 1;
            } else {
                minus[k] += 1;
            }
        }
        let decomposition = ExactMotifDecomposition {
            policy: ladder.policy,
            field0: ladder.field0,
            field1: ladder.field1,
            root_id: ladder.root_id,
            worlds: ladder.worlds,
            plus,
            minus,
        };
        // §3.7 identities, asserted exactly: Σ m_k⁺ = c⁺, Σ m_k⁻ = c⁻,
        // Σ c_k = c.
        assert_eq!(
            plus.iter().sum::<u64>(),
            ladder.c_plus,
            "Σ m_k⁺ = c⁺ holds exactly"
        );
        assert_eq!(
            minus.iter().sum::<u64>(),
            ladder.c_minus,
            "Σ m_k⁻ = c⁻ holds exactly"
        );
        let net: BigInt = SplitMotif::ALL
            .iter()
            .map(|m| decomposition.c_worlds(*m))
            .sum();
        assert_eq!(
            BigRational::new(net, BigInt::from(decomposition.worlds)),
            ladder.c(),
            "Σ c_k = c holds exactly"
        );
        decomposition
    }

    /// `N · c_k` as a signed integer.
    fn c_worlds(&self, motif: SplitMotif) -> BigInt {
        let k = motif.index();
        BigInt::from(self.plus[k]) - BigInt::from(self.minus[k])
    }

    /// `m_k⁺` under the uniform fiber measure.
    pub fn m_plus(&self, motif: SplitMotif) -> BigRational {
        BigRational::new(
            BigInt::from(self.plus[motif.index()]),
            BigInt::from(self.worlds),
        )
    }

    /// `m_k⁻`.
    pub fn m_minus(&self, motif: SplitMotif) -> BigRational {
        BigRational::new(
            BigInt::from(self.minus[motif.index()]),
            BigInt::from(self.worlds),
        )
    }

    /// `r_k = m_k⁺ + m_k⁻`.
    pub fn r_k(&self, motif: SplitMotif) -> BigRational {
        let k = motif.index();
        BigRational::new(
            BigInt::from(self.plus[k]) + BigInt::from(self.minus[k]),
            BigInt::from(self.worlds),
        )
    }

    /// `c_k = m_k⁺ − m_k⁻` (signed).
    pub fn c_k(&self, motif: SplitMotif) -> BigRational {
        BigRational::new(self.c_worlds(motif), BigInt::from(self.worlds))
    }

    /// §3.7 — the exact conditional directional tilt `τ_k = c_k / r_k`,
    /// defined only when `r_k > 0`.
    pub fn tilt(&self, motif: SplitMotif) -> Option<BigRational> {
        let k = motif.index();
        let r_worlds = self.plus[k] + self.minus[k];
        (r_worlds > 0).then(|| {
            BigRational::new(self.c_worlds(motif), BigInt::from(r_worlds))
        })
    }

    /// Correction worlds carrying motif k (the numerator of `r_k`).
    pub fn correction_worlds(&self, motif: SplitMotif) -> u64 {
        let k = motif.index();
        self.plus[k] + self.minus[k]
    }

    /// §3.6 — the residual fraction among correction worlds (`None` when
    /// nothing corrected). An instrument reading, not a forecast: no
    /// numerical residual-rate expectation exists to compare it against.
    pub fn residual_fraction(&self) -> Option<BigRational> {
        let total: u64 = (0..7).map(|k| self.plus[k] + self.minus[k]).sum();
        (total > 0).then(|| {
            BigRational::new(
                BigInt::from(self.correction_worlds(SplitMotif::Other)),
                BigInt::from(total),
            )
        })
    }
}

impl fmt::Display for ExactMotifDecomposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ExactMotifDecomposition{{policy={};field0={};field1={};root={:#018x};\
             worlds={};",
            self.policy, self.field0, self.field1, self.root_id, self.worlds
        )?;
        for motif in SplitMotif::ALL {
            let k = motif.index();
            write!(f, "{}=+{}/-{};", motif.tag(), self.plus[k], self.minus[k])?;
        }
        write!(f, "}}")
    }
}

/// §3.7 — the DESCRIPTIVE sampled-prefix sibling: raw motif counts over a
/// declared stream prefix. Descriptive tier by type: it asserts no
/// identity against any ladder, exposes no mass as a bound of anything,
/// offers no screenable accessor, and cannot be converted into a
/// [`RootActionExposureUpper`] or any other screen input — the same lock
/// shape as `SampledPairwiseMasses`' missing dominance method.
///
/// ```compile_fail
/// use walt::rules::Domino;
/// use walt::solver::field_swap::ActionExposureUpper;
/// use walt::solver::motif::DescriptiveMotifHistogram;
///
/// fn feed_the_screen(action: Domino, h: DescriptiveMotifHistogram) -> ActionExposureUpper {
///     // A descriptive histogram is not a RootActionExposureUpper and
///     // has no conversion to one: this does not compile.
///     ActionExposureUpper { action, bound: h }
/// }
/// ```
///
/// [`RootActionExposureUpper`]: crate::solver::exposure::RootActionExposureUpper
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DescriptiveMotifHistogram {
    policy: PolicyId,
    field0: FieldId,
    field1: FieldId,
    root_id: u64,
    epoch: u64,
    worlds: u64,
    counts: [u64; 7],
}

impl DescriptiveMotifHistogram {
    /// Count primary labels over a DECLARED stream prefix. An exact-fiber
    /// domain is refused: exact-fiber corrections take
    /// [`ExactMotifDecomposition`], which carries the identity
    /// obligations this type deliberately cannot state.
    pub fn from_stream(
        policy: PolicyId,
        field0: FieldId,
        field1: FieldId,
        root_id: u64,
        domain: &WorldDomain,
        labels: &[SplitMotif],
    ) -> DescriptiveMotifHistogram {
        let WorldDomain::StreamPrefix { epoch, worlds } = domain else {
            panic!("a descriptive histogram ranges over a declared stream prefix only (§3.7)");
        };
        let mut counts = [0u64; 7];
        for label in labels {
            counts[label.index()] += 1;
        }
        DescriptiveMotifHistogram {
            policy,
            field0,
            field1,
            root_id,
            epoch: *epoch,
            worlds: *worlds,
            counts,
        }
    }

    /// The declared domain, part of the claim.
    pub fn domain(&self) -> WorldDomain {
        WorldDomain::StreamPrefix {
            epoch: self.epoch,
            worlds: self.worlds,
        }
    }

    /// The raw count of one primary label — a plain description of the
    /// sample, an estimate of nothing in particular without a declared
    /// inference process (§3.7).
    pub fn count(&self, motif: SplitMotif) -> u64 {
        self.counts[motif.index()]
    }
}

impl fmt::Display for DescriptiveMotifHistogram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DescriptiveMotifHistogram{{descriptive;policy={};field0={};field1={};\
             root={:#018x};domain={};",
            self.policy,
            self.field0,
            self.field1,
            self.root_id,
            self.domain(),
        )?;
        for motif in SplitMotif::ALL {
            write!(f, "{}={};", motif.tag(), self.counts[motif.index()])?;
        }
        write!(f, "}}")
    }
}
