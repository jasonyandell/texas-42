//! `solver::field` — field-model identity and materialization (§21 step 3).
//!
//! EXPLORATORY tier. Implements parent
//! `walt/math/targeted_level2_field_stability_v0.1.md` §8 Stage 0 (the
//! immutable field identity) and §18's `solver::field` module shape, under
//! rulings L2-A1..A7 (`walt/CENSUS-RULINGS.md`) and obligations O29–O38 of
//! `walt/SCENARIO-PLAYER.md` §10.
//!
//! Owns, per parent §18: [`FieldId`], field level and inner configuration,
//! immutable modeled-policy access ([`FieldModel`]), the insert-only field
//! action cache, and field-to-field action comparison
//! ([`field_actions`]). The disagreement frontier, coupled replay, and
//! exposure results live in `solver::exposure`.
//!
//! Two invariants are structural here, not conventions:
//!
//! - **Field purity (O29).** Every modeled field action is a pure function
//!   of (declared information state, immutable [`FieldId`]). The state key
//!   [`FieldStateKey`]'s only constructor takes the acting seat's own
//!   remaining hand plus a [`PublicRecord`] — this module never imports
//!   the kernel's `World`, so the evaluation world's hidden hands are
//!   unreachable from any field decision, by type. The σ1 seed derivation
//!   ([`FieldModel::state_seed`]) reads the [`FieldId`], the declared seed
//!   schedule, and the state key alone.
//! - **Cache immutability (O29's deterministic-replay route, in the O22
//!   shape).** A cache entry, once written, can never change: the cache is
//!   private, the only write path is the miss-then-insert in
//!   [`FieldModel::choose`], and no replacing or removing API exists. A
//!   changed [`FieldSpec`] is a new [`FieldId`] and a new experiment epoch
//!   (parent §8 Stage 0).
//!
//! The two concrete models materialized through the one interface are the
//! parent §2 instantiation: σ0 = the banked-correct level-0 modeled mind
//! (the field the step-7 shadow bin drives non-focal seats with —
//! [`Level0Field`], one authority, never a copy) and σ1 = the level-1
//! machinery (`solver::level1_evaluate`, the live player's one authority)
//! run per non-focal seat at a small declared inner schedule. The model
//! index is part of the result; raising the level changes the model, never
//! the rules (O36).

use std::collections::HashMap;
use std::fmt;
use std::sync::Mutex;

use num_rational::BigRational;

use crate::kernel::SplitMix64 as KernelRng;
use crate::rules::{Decl, Domino, DominoSet, Team};
use crate::solver::adaptive::{PublicRecord, RootPosition, SlicePolicy};
use crate::solver::policy::{
    content_digest, continuation_frame, t1_frame_bid, Canon, DecisionMode, Level0Field, TieRule,
    NO_DEADLINE_SECS,
};
use crate::solver::{
    arena_decl_id, best_of, level1_evaluate, mask_of, mix, SplitMix64 as SolverRng,
};

/// Domain-separation tag for the σ1 field's inner-evaluation seed
/// derivation. Distinct from the evidence stream's
/// `solver::adaptive::STREAM_DOMAIN` and the frozen-policy discovery
/// domain `solver::policy::DISCOVERY_DOMAIN` — the three derivations must
/// never collide, and a test asserts the tags differ pairwise.
pub const FIELD_DOMAIN: u64 = 0xF1E1_DFAC_E5EE_D003;

// ---------------------------------------------------------------------------
// The field identity (parent §8 Stage 0).
// ---------------------------------------------------------------------------

/// The model level plus the inner decision configuration — the behavioral
/// half of the field identity. The level is a derived view of the variant,
/// never a second stored field.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldKind {
    /// σ0 — the banked-correct level-0 modeled mind at declared `n0`
    /// belief worlds per decision (the existing [`Level0Field`]).
    Level0 { n0: u64 },
    /// σ1 — every non-focal seat evaluated by the existing level-1
    /// machinery at the declared `[n_outer, n0]` inner schedule, seeded
    /// per state from ([`FieldId`], state key).
    Level1 { n_outer: u64, n0: u64 },
}

impl FieldKind {
    /// The model level (O36: a named model index, never an equilibrium
    /// claim).
    pub fn level(&self) -> u64 {
        match self {
            FieldKind::Level0 { .. } => 0,
            FieldKind::Level1 { .. } => 1,
        }
    }
}

/// Parent §8 Stage 0 — the complete immutable identity of a field model.
/// Every listed identity component is here: model level and inner decision
/// configuration ([`FieldKind`]), policy construction version, risk and
/// equivalence settings, fallback semantics, seed schedules, tie handling,
/// policy-library identity, and the exact-versus-heuristic mode. Hashing
/// the canonical serialization yields the [`FieldId`]; changing ANY field
/// produces a new `FieldId` and a new experiment epoch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldSpec {
    /// Model level plus inner decision configuration.
    pub kind: FieldKind,
    /// Policy construction version (source identity of the modeled mind).
    pub construction: String,
    /// Risk/equivalence settings affecting actions (a rational ε; `None`
    /// in this slice — the models play their exact argmax).
    pub practical_equivalence: Option<BigRational>,
    /// Fallback semantics (what the model does when its machinery cannot
    /// answer; this slice's models run with no wall-clock cutoff, so the
    /// declared fallback is "none").
    pub fallback: String,
    /// Seed schedule folded into every per-state inner derivation.
    pub seed_schedule: Vec<u64>,
    /// Tie handling. Must name what the algorithm actually does.
    pub tie_rule: TieRule,
    /// Policy-library identity.
    pub policy_library: String,
    /// Exact/heuristic mode flag (both this slice's models are sampled
    /// inner minds: `Heuristic`).
    pub mode: DecisionMode,
}

impl FieldSpec {
    /// The deterministic canonical serialization the [`FieldId`] hashes:
    /// a fresh header (`walt-field-model-v1`, aliasing no other
    /// serialization family), fixed field order, tag bytes, and length
    /// prefixes.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut canon = Canon::new("walt-field-model-v1");
        match &self.kind {
            FieldKind::Level0 { n0 } => {
                canon.tag(0x01);
                canon.u8(0);
                canon.u64(*n0);
            }
            FieldKind::Level1 { n_outer, n0 } => {
                canon.tag(0x01);
                canon.u8(1);
                canon.u64(*n_outer);
                canon.u64(*n0);
            }
        }
        canon.str_field(0x02, &self.construction);
        canon.rational_field(0x03, self.practical_equivalence.as_ref());
        canon.str_field(0x04, &self.fallback);
        canon.u64s_field(0x05, &self.seed_schedule);
        canon.tag(0x06);
        canon.u8(match self.tie_rule {
            TieRule::FirstInPreference => 0,
            TieRule::LowestTileIndex => 1,
        });
        canon.str_field(0x07, &self.policy_library);
        canon.tag(0x08);
        canon.u8(match self.mode {
            DecisionMode::Exact => 0,
            DecisionMode::Heuristic => 1,
        });
        canon.finish()
    }

    /// The content address of the complete spec.
    pub fn field_id(&self) -> FieldId {
        FieldId(content_digest(&self.canonical_bytes()))
    }
}

/// The immutable content address (SHA-256) of a complete [`FieldSpec`].
/// Every cross-field result names both `FieldId`s (parent §10 items 4;
/// acceptance item 2).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FieldId([u8; 32]);

impl FieldId {
    pub const fn bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// The first eight bytes as a big-endian `u64`, for folding the id
    /// into integer seed derivations.
    pub fn fold64(&self) -> u64 {
        u64::from_be_bytes([
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7],
        ])
    }
}

impl fmt::Display for FieldId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// The non-focal information-state key (parent §3: J = the seat's hand plus
// the public record).
// ---------------------------------------------------------------------------

/// A non-focal information state `J` (parent §3): exactly the information
/// available to the modeled seat — its own remaining hand and the FULL
/// public record (root frame plus every play since it; no reduction is
/// claimed at this step). The acting seat and its legal set are derived
/// views of this key, never stored fields.
///
/// FIELD PURITY IS TYPE-ENFORCED (O29): the only constructor is
/// [`FieldStateKey::from_public`], whose inputs are one `DominoSet` (the
/// seat's own hand) and a [`PublicRecord`] (public data only). No
/// constructor accepts the kernel's `World` or any other seat's hand.
///
/// Equality and hashing go through the projected public state itself,
/// never through a digest, so distinct information states can never
/// collide in the action cache.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FieldStateKey {
    hand: DominoSet,
    root: RootPosition,
    history: Vec<Domino>,
}

impl FieldStateKey {
    /// The one constructor (see the type-level contract above). Panics if
    /// the claimed hand overlaps publicly played tiles — an information
    /// state must be internally consistent.
    pub fn from_public(hand: DominoSet, record: &PublicRecord<'_>) -> FieldStateKey {
        let since_root: DominoSet = record
            .root
            .trick_plays
            .iter()
            .chain(record.history.iter())
            .copied()
            .collect();
        let played = since_root.union(record.root.prior_played);
        assert!(
            hand.is_disjoint(played),
            "a remaining hand is disjoint from the public play history"
        );
        FieldStateKey {
            hand,
            root: record.root.clone(),
            history: record.history.to_vec(),
        }
    }

    /// A 64-bit fold of the key's canonical serialization, for the σ1 seed
    /// derivation. Collisions here cost inner-stream overlap only — never
    /// cache identity, which uses the full key.
    pub fn digest64(&self) -> u64 {
        let mut canon = Canon::new("walt-field-state-v1");
        canon.tag(0x01);
        canon.u32(self.hand.bits());
        canon.tag(0x02);
        canon.u8(arena_decl_id(self.root.decl) as u8);
        canon.u32(self.root.bid);
        canon.u8(self.root.declaring_team.index() as u8);
        canon.u8(self.root.leader.index() as u8);
        canon.u32(self.root.banked[0]);
        canon.u32(self.root.banked[1]);
        canon.dominoes_field(0x03, &self.root.trick_plays);
        canon.dominoes_field(0x04, &self.history);
        canon.tag(0x05);
        canon.u32(self.root.prior_played.bits());
        for voids in &self.root.voids {
            canon.u8(voids.iter().fold(0u8, |acc, q| acc | (1 << q.index())));
        }
        let digest = content_digest(&canon.finish());
        u64::from_be_bytes([
            digest[0], digest[1], digest[2], digest[3], digest[4], digest[5], digest[6], digest[7],
        ])
    }
}

// ---------------------------------------------------------------------------
// The materialized field model.
// ---------------------------------------------------------------------------

/// A field model: a [`FieldSpec`], its immutable [`FieldId`], and the
/// per-id insert-only action cache (information state → chosen legal
/// action). Implements [`SlicePolicy`], so the coupled replay, the direct
/// replay (`replay_viewer_success`), and every existing evaluator drive it
/// unchanged.
pub struct FieldModel {
    spec: FieldSpec,
    id: FieldId,
    label: String,
    /// σ0's one authority (constructed for [`FieldKind::Level0`] only).
    level0: Option<Level0Field>,
    cache: Mutex<HashMap<FieldStateKey, Domino>>,
}

impl FieldModel {
    /// Materialize a field. Validates the declared configuration: sample
    /// counts positive, and the declared tie rule names what the stack's
    /// algorithms actually do (ascending-tile evaluation order, exact
    /// value ties toward the lowest tile index).
    pub fn new(spec: FieldSpec) -> FieldModel {
        assert_eq!(
            spec.tie_rule,
            TieRule::LowestTileIndex,
            "the modeled minds break exact ties toward the lowest tile index; \
             the declared tie rule must name what the algorithm does"
        );
        let level0 = match spec.kind {
            FieldKind::Level0 { n0 } => {
                assert!(n0 >= 1, "a declared sample count is positive");
                Some(Level0Field::new(
                    usize::try_from(n0).expect("a declared count fits usize"),
                ))
            }
            FieldKind::Level1 { n_outer, n0 } => {
                assert!(
                    n_outer >= 1 && n0 >= 1,
                    "declared sample counts are positive"
                );
                None
            }
        };
        let id = spec.field_id();
        let label = format!("field:level{}:{id}", spec.kind.level());
        FieldModel {
            spec,
            id,
            label,
            level0,
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub fn field_id(&self) -> FieldId {
        self.id
    }

    pub fn spec(&self) -> &FieldSpec {
        &self.spec
    }

    /// The σ1 inner-evaluation seed for one information state: a pure
    /// function of (domain tag, [`FieldId`], the declared seed schedule,
    /// the state key). Its inputs are the key and the spec alone — no
    /// evaluation world, no hidden hand, can reach it, by signature (O29).
    pub fn state_seed(&self, key: &FieldStateKey) -> u64 {
        let mut seed = mix(FIELD_DOMAIN ^ self.id.fold64());
        for s in &self.spec.seed_schedule {
            seed = mix(seed ^ s);
        }
        mix(seed ^ key.digest64())
    }

    /// How many information states have been materialized so far. A
    /// replay that revisits only known states leaves this unchanged.
    pub fn cache_len(&self) -> usize {
        self.cache
            .lock()
            .expect("the field action cache is unpoisoned")
            .len()
    }

    /// A snapshot of the action cache, for insert-only gates: after any
    /// amount of further replay, every entry of an earlier snapshot must
    /// still be present with the same action.
    pub fn cache_snapshot(&self) -> HashMap<FieldStateKey, Domino> {
        self.cache
            .lock()
            .expect("the field action cache is unpoisoned")
            .clone()
    }

    /// The σ1 miss computation: the EXISTING level-1 machinery
    /// (`solver::level1_evaluate` — belief sampling, bundled level-0 field
    /// solve, saturation-tie refinement) run at the seat's information
    /// state, with its evaluation worlds drawn from the field's own
    /// domain-separated stream. Information-consistent by construction:
    /// every argument is a derived view of (key, spec).
    fn level1_action(
        &self,
        key: &FieldStateKey,
        legal: DominoSet,
        n_outer: u64,
        n0: u64,
    ) -> Domino {
        let decl = key.root.decl;
        let frame = continuation_frame(decl, &key.root, &key.history);
        let seat = frame.seat;
        assert_eq!(
            frame.sizes()[seat.index()],
            key.hand.len(),
            "the modeled hand size matches the replayed record"
        );
        let mut rng = SolverRng(KernelRng::new(self.state_seed(key)).next_u64());
        let opts = level1_evaluate(
            decl,
            t1_frame_bid(key.root.bid, key.root.declaring_team),
            seat,
            mask_of(key.hand),
            mask_of(legal),
            &frame.key,
            frame.sizes(),
            frame.voids,
            frame.trick_start_played,
            frame.boundary_hand_size,
            usize::try_from(n_outer).expect("a declared count fits usize"),
            usize::try_from(n0).expect("a declared count fits usize"),
            NO_DEADLINE_SECS,
            &mut rng,
        )
        .expect("a modeled mind runs without a wall-clock cutoff");
        let choice = best_of(&opts, seat.team() == Team::T1);
        Domino::from_index(usize::from(choice)).expect("tile < 28")
    }
}

impl SlicePolicy for FieldModel {
    /// The field names itself by its content address.
    fn id(&self) -> &str {
        &self.label
    }

    /// Materialize the field action at the acting seat's information
    /// state. On a miss, the action is computed under the declared
    /// configuration from the information state alone and cached; on a
    /// hit, the cached action is returned unchanged. The miss computation
    /// runs OUTSIDE the cache lock (a level-1 materialization can be
    /// long); if two threads race the same key, both compute the same pure
    /// value, and the insert-only contract is asserted as value equality
    /// on the second insert.
    fn choose(
        &self,
        decl: Decl,
        hand: DominoSet,
        legal: DominoSet,
        record: &PublicRecord<'_>,
    ) -> Domino {
        assert_eq!(decl, record.root.decl, "one declaration governs the record");
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        assert!(
            legal.is_subset_of(hand),
            "the legal set is drawn from the acting hand"
        );
        let key = FieldStateKey::from_public(hand, record);
        if let Some(&cached) = self
            .cache
            .lock()
            .expect("the field action cache is unpoisoned")
            .get(&key)
        {
            assert!(
                legal.contains(cached),
                "a cached action is legal: the legal set is a function of its key"
            );
            return cached;
        }
        let chosen = if legal.len() == 1 {
            // A forced play needs no modeled solve.
            legal.iter().next().expect("one legal tile")
        } else {
            match self.spec.kind {
                FieldKind::Level0 { .. } => self
                    .level0
                    .as_ref()
                    .expect("a level-0 field holds its one authority")
                    .choose(decl, hand, legal, record),
                FieldKind::Level1 { n_outer, n0 } => self.level1_action(&key, legal, n_outer, n0),
            }
        };
        let mut cache = self
            .cache
            .lock()
            .expect("the field action cache is unpoisoned");
        match cache.insert(key, chosen) {
            None => {}
            Some(previous) => assert_eq!(
                previous, chosen,
                "a field cache entry, once written, never changes (O29)"
            ),
        }
        chosen
    }
}

/// Field-to-field action comparison at one information state (parent §18):
/// both fields' chosen tiles for the same (hand, record). The pair
/// disagrees exactly when the state lies on the disagreement frontier
/// `F_{0,1}` of parent §3.
pub fn field_actions(
    decl: Decl,
    hand: DominoSet,
    legal: DominoSet,
    record: &PublicRecord<'_>,
    field0: &FieldModel,
    field1: &FieldModel,
) -> (Domino, Domino) {
    (
        field0.choose(decl, hand, legal, record),
        field1.choose(decl, hand, legal, record),
    )
}
