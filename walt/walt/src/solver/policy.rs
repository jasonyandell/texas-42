//! `solver::policy` — the frozen policy authority (parent §16.2).
//!
//! EXPLORATORY tier. Implements parent `walt/math/calculated_evidence_v0.1.md`
//! §12 (frozen-policy materialization), §22 step 4 of the build program
//! adopted at ruling CE-A7 (`walt/CENSUS-RULINGS.md`); obligations O13, O22,
//! and O27 of `walt/SCENARIO-PLAYER.md` §10 name its discipline.
//!
//! Owns, per §16.2: [`FreezeTuple`], [`PolicyId`], the
//! information-consistent action key [`InfoKey`], lazy policy
//! materialization, the immutable action cache, and frozen-policy replay
//! ([`FrozenPolicy`] implements [`SlicePolicy`], so the slice's
//! `evaluate_pair` / `exact_frozen_pair` accept it unchanged).
//!
//! Three §12 invariants are structural here, not conventions:
//!
//! - **Information consistency (§12.3).** [`InfoKey`]'s only constructor
//!   takes the focal seat's own remaining hand plus a [`PublicRecord`] —
//!   this module never imports the kernel's `World`, so the evaluation
//!   world's hidden hands are unreachable from any key or any discovery
//!   derivation, by type.
//! - **Immutability (O22, §12.5).** A cache entry, once written, can never
//!   change: the cache is private, the only write path is the
//!   miss-then-insert in [`FrozenPolicy::action`], and no replacing or
//!   removing API exists. A cache miss extends the representation of the
//!   same frozen policy; changing a defined action is impossible by
//!   construction. Changing any [`FreezeTuple`] field is a new
//!   [`PolicyId`] and a new evidence epoch.
//! - **Discovery/evidence disjointness (§12.4, O13).** Discovery streams
//!   derive only from (freeze tuple, information state, counter) under
//!   [`DISCOVERY_DOMAIN`], a domain tag distinct from the evidence
//!   stream's `solver::adaptive::STREAM_DOMAIN`. This step's action rule
//!   is a deterministic function of the information state (a fixed
//!   preference order frozen in the tuple); a SAMPLED discovery solver
//!   drawing worlds from [`FrozenPolicy::discovery_rng`] is §22 step 10
//!   territory — the seam is here, the solver is not.
//!
//! No floats anywhere; the declared inner sample schedule is an identity
//! field (a declared approximation visible in the PolicyId, CE-A5), never
//! a stopping rule.

use std::collections::HashMap;
use std::fmt;
use std::sync::Mutex;

use num_bigint::{BigInt, Sign};
use num_rational::BigRational;

use crate::kernel::SplitMix64 as KernelRng;
use crate::rules::{Decl, Domino, DominoSet, Team};
use crate::solver::adaptive::{PublicRecord, RootPosition, SlicePolicy};
use crate::solver::{arena_decl_id, mix};

/// Domain-separation tag for DISCOVERY seed derivation (§12.4). Distinct
/// from the evidence stream's `solver::adaptive::STREAM_DOMAIN` — the two
/// derivations must never collide, and a test asserts the tags differ.
pub const DISCOVERY_DOMAIN: u64 = 0xD15C_0FEE_D5EE_D001;

// ---------------------------------------------------------------------------
// A small vendored SHA-256 (integer-only, no dependencies).
// ---------------------------------------------------------------------------

/// FIPS 180-4 SHA-256 over the canonical serialization. Vendored because
/// the content address needs collision resistance and the workspace
/// carries no hashing dependency; all arithmetic is `u32` wrapping — no
/// floats. Verified in-module against the standard test vectors.
mod sha256 {
    const K: [u32; 64] = [
        0x428a_2f98,
        0x7137_4491,
        0xb5c0_fbcf,
        0xe9b5_dba5,
        0x3956_c25b,
        0x59f1_11f1,
        0x923f_82a4,
        0xab1c_5ed5,
        0xd807_aa98,
        0x1283_5b01,
        0x2431_85be,
        0x550c_7dc3,
        0x72be_5d74,
        0x80de_b1fe,
        0x9bdc_06a7,
        0xc19b_f174,
        0xe49b_69c1,
        0xefbe_4786,
        0x0fc1_9dc6,
        0x240c_a1cc,
        0x2de9_2c6f,
        0x4a74_84aa,
        0x5cb0_a9dc,
        0x76f9_88da,
        0x983e_5152,
        0xa831_c66d,
        0xb003_27c8,
        0xbf59_7fc7,
        0xc6e0_0bf3,
        0xd5a7_9147,
        0x06ca_6351,
        0x1429_2967,
        0x27b7_0a85,
        0x2e1b_2138,
        0x4d2c_6dfc,
        0x5338_0d13,
        0x650a_7354,
        0x766a_0abb,
        0x81c2_c92e,
        0x9272_2c85,
        0xa2bf_e8a1,
        0xa81a_664b,
        0xc24b_8b70,
        0xc76c_51a3,
        0xd192_e819,
        0xd699_0624,
        0xf40e_3585,
        0x106a_a070,
        0x19a4_c116,
        0x1e37_6c08,
        0x2748_774c,
        0x34b0_bcb5,
        0x391c_0cb3,
        0x4ed8_aa4a,
        0x5b9c_ca4f,
        0x682e_6ff3,
        0x748f_82ee,
        0x78a5_636f,
        0x84c8_7814,
        0x8cc7_0208,
        0x90be_fffa,
        0xa450_6ceb,
        0xbef9_a3f7,
        0xc671_78f2,
    ];

    const H0: [u32; 8] = [
        0x6a09_e667,
        0xbb67_ae85,
        0x3c6e_f372,
        0xa54f_f53a,
        0x510e_527f,
        0x9b05_688c,
        0x1f83_d9ab,
        0x5be0_cd19,
    ];

    pub fn digest(message: &[u8]) -> [u8; 32] {
        let mut h = H0;
        let bit_len = (message.len() as u64).wrapping_mul(8);
        let mut data = message.to_vec();
        data.push(0x80);
        while data.len() % 64 != 56 {
            data.push(0);
        }
        data.extend_from_slice(&bit_len.to_be_bytes());
        for block in data.chunks_exact(64) {
            let mut w = [0u32; 64];
            for (slot, word) in w.iter_mut().zip(block.chunks_exact(4)) {
                *slot = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
            }
            for i in 16..64 {
                let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
                let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
                w[i] = w[i - 16]
                    .wrapping_add(s0)
                    .wrapping_add(w[i - 7])
                    .wrapping_add(s1);
            }
            let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
            for i in 0..64 {
                let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
                let ch = (e & f) ^ (!e & g);
                let t1 = hh
                    .wrapping_add(s1)
                    .wrapping_add(ch)
                    .wrapping_add(K[i])
                    .wrapping_add(w[i]);
                let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
                let maj = (a & b) ^ (a & c) ^ (b & c);
                let t2 = s0.wrapping_add(maj);
                hh = g;
                g = f;
                f = e;
                e = d.wrapping_add(t1);
                d = c;
                c = b;
                b = a;
                a = t1.wrapping_add(t2);
            }
            for (slot, v) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
                *slot = slot.wrapping_add(v);
            }
        }
        let mut out = [0u8; 32];
        for (chunk, word) in out.chunks_exact_mut(4).zip(h) {
            chunk.copy_from_slice(&word.to_be_bytes());
        }
        out
    }
}

// ---------------------------------------------------------------------------
// Canonical serialization.
// ---------------------------------------------------------------------------

/// A deterministic byte writer: every variable-length field is
/// length-prefixed and every field carries a tag byte, so no two distinct
/// tuples serialize to the same bytes by field aliasing.
struct Canon {
    bytes: Vec<u8>,
}

impl Canon {
    fn new(header: &str) -> Canon {
        let mut canon = Canon { bytes: Vec::new() };
        canon.str_field(0x00, header);
        canon
    }

    fn tag(&mut self, tag: u8) {
        self.bytes.push(tag);
    }

    fn u8(&mut self, v: u8) {
        self.bytes.push(v);
    }

    fn u32(&mut self, v: u32) {
        self.bytes.extend_from_slice(&v.to_be_bytes());
    }

    fn u64(&mut self, v: u64) {
        self.bytes.extend_from_slice(&v.to_be_bytes());
    }

    fn len(&mut self, n: usize) {
        self.u64(n as u64);
    }

    fn str_field(&mut self, tag: u8, s: &str) {
        self.tag(tag);
        self.len(s.len());
        self.bytes.extend_from_slice(s.as_bytes());
    }

    fn u64s_field(&mut self, tag: u8, vs: &[u64]) {
        self.tag(tag);
        self.len(vs.len());
        for v in vs {
            self.u64(*v);
        }
    }

    fn dominoes_field(&mut self, tag: u8, ds: &[Domino]) {
        self.tag(tag);
        self.len(ds.len());
        for d in ds {
            self.u8(d.index() as u8);
        }
    }

    fn bigint(&mut self, v: &BigInt) {
        let (sign, magnitude) = v.to_bytes_be();
        self.u8(match sign {
            Sign::Minus => 0,
            Sign::NoSign => 1,
            Sign::Plus => 2,
        });
        self.len(magnitude.len());
        self.bytes.extend_from_slice(&magnitude);
    }

    fn rational_field(&mut self, tag: u8, r: Option<&BigRational>) {
        self.tag(tag);
        match r {
            None => self.u8(0),
            Some(r) => {
                self.u8(1);
                self.bigint(r.numer());
                self.bigint(r.denom());
            }
        }
    }

    fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

// ---------------------------------------------------------------------------
// The freeze tuple (§12.1) and its content address.
// ---------------------------------------------------------------------------

/// The tie-handling rule frozen into a policy's identity (§12.1). Both
/// rules are deterministic; under [`ActionRule::Preference`] the total
/// preference order leaves no ties to break, but the declared rule is
/// still part of the identity — changing it is a new [`PolicyId`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TieRule {
    /// Ties break toward the earlier tile in the frozen preference order.
    FirstInPreference,
    /// Ties break toward the lowest stable tile index.
    LowestTileIndex,
}

impl TieRule {
    fn code(self) -> u8 {
        match self {
            TieRule::FirstInPreference => 0,
            TieRule::LowestTileIndex => 1,
        }
    }
}

/// The exact/heuristic mode flag of §12.1: any flag that changes decisions
/// is identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecisionMode {
    Exact,
    Heuristic,
}

impl DecisionMode {
    fn code(self) -> u8 {
        match self {
            DecisionMode::Exact => 0,
            DecisionMode::Heuristic => 1,
        }
    }
}

/// The inner sample schedule (§12.1). A count here is a DECLARED
/// APPROXIMATION parameter made visible in the policy identity (CE-A5's
/// lawful residence for a fixed count) — it is never a stopping rule, and
/// this step's deterministic action rule uses `None`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InnerSchedule {
    /// No inner sampling: the action rule is a deterministic function of
    /// the information state.
    None,
    /// A declared per-depth inner sample schedule (later steps).
    Declared(Vec<u64>),
}

/// The frozen action rule: how a materialization miss computes its action
/// from the information state, parameterized entirely by the tuple.
///
/// This step's rule is deterministic (§22 step 4): a fixed total
/// preference order over all 28 tiles. A sampled discovery solver (§12.4's
/// inner sampled case) is a later variant (§22 step 10); its worlds will
/// come from [`FrozenPolicy::discovery_rng`], never from any evaluation
/// stream.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionRule {
    /// Play the most-preferred legal tile of a total order over all 28.
    Preference(Vec<Domino>),
}

/// §12.1 — the complete frozen identity of a policy. Every field that can
/// change a decision is here; hashing the canonical serialization of the
/// whole tuple yields the [`PolicyId`]. Changing ANY field produces a new
/// `PolicyId` and a new evidence epoch (§12.5).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FreezeTuple {
    /// Solver semantic version / source identity.
    pub solver_source: String,
    /// The declaration the policy is frozen for.
    pub decl: Decl,
    /// The contract in points.
    pub bid: u32,
    /// The declaring team of the frozen contract.
    pub declaring_team: Team,
    /// Field model name (how non-focal seats are modeled during
    /// discovery).
    pub field_model: String,
    /// Field model level.
    pub field_level: u64,
    /// Inner sample schedule (declared approximation; identity only).
    pub inner_schedule: InnerSchedule,
    /// Discovery stream identity (§12.4).
    pub discovery_stream: String,
    /// Discovery seed schedule: base seeds folded into every per-state
    /// discovery derivation.
    pub discovery_seed_schedule: Vec<u64>,
    /// Tie-handling rule.
    pub tie_rule: TieRule,
    /// Any practical-equivalence parameter affecting actions (a rational
    /// ε; `None` this step).
    pub practical_equivalence: Option<BigRational>,
    /// Policy-library version.
    pub policy_library: String,
    /// Exact/heuristic mode flag.
    pub mode: DecisionMode,
    /// The frozen action rule and its parameters.
    pub action_rule: ActionRule,
}

impl FreezeTuple {
    /// The deterministic canonical serialization the [`PolicyId`] hashes:
    /// a fixed field order, tag bytes, and length prefixes — equal tuples
    /// serialize identically, distinct tuples distinctly.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut canon = Canon::new("walt-freeze-tuple-v1");
        canon.str_field(0x01, &self.solver_source);
        canon.tag(0x02);
        canon.u8(arena_decl_id(self.decl) as u8);
        canon.tag(0x03);
        canon.u32(self.bid);
        canon.tag(0x04);
        canon.u8(self.declaring_team.index() as u8);
        canon.str_field(0x05, &self.field_model);
        canon.tag(0x06);
        canon.u64(self.field_level);
        match &self.inner_schedule {
            InnerSchedule::None => canon.u64s_field(0x07, &[]),
            InnerSchedule::Declared(counts) => {
                canon.tag(0x07);
                canon.u8(1);
                canon.len(counts.len());
                for c in counts {
                    canon.u64(*c);
                }
            }
        }
        canon.str_field(0x08, &self.discovery_stream);
        canon.u64s_field(0x09, &self.discovery_seed_schedule);
        canon.tag(0x0A);
        canon.u8(self.tie_rule.code());
        canon.rational_field(0x0B, self.practical_equivalence.as_ref());
        canon.str_field(0x0C, &self.policy_library);
        canon.tag(0x0D);
        canon.u8(self.mode.code());
        match &self.action_rule {
            ActionRule::Preference(order) => canon.dominoes_field(0x0E, order),
        }
        canon.finish()
    }

    /// The content address of the complete tuple (§12.1).
    pub fn policy_id(&self) -> PolicyId {
        PolicyId(sha256::digest(&self.canonical_bytes()))
    }
}

/// §12.1 — the content address (SHA-256) of a complete [`FreezeTuple`].
/// Every evidence observation names immutable `PolicyId`s (O22).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PolicyId([u8; 32]);

impl PolicyId {
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

impl fmt::Display for PolicyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// The information-consistent action key (§12.3).
// ---------------------------------------------------------------------------

/// §12.3 — the focal action key: the focal seat's own remaining hand, the
/// FULL public record (root frame plus every play since it — no
/// reduction is claimed at this step), and the freeze tuple (by its
/// [`PolicyId`]).
///
/// INFORMATION CONSISTENCY IS TYPE-ENFORCED: the only constructor is
/// [`InfoKey::from_public`], whose inputs are one `DominoSet` (the focal
/// seat's own hand) and a [`PublicRecord`] (public data only). No
/// constructor accepts the kernel's `World` or any other seat's hand —
/// this module does not import `World` at all. The evaluation world
/// determines which public observations occur; it can never directly
/// select the focal action.
///
/// Equality and hashing go through the projected public state itself
/// (root frame, play history, hand), never through a digest, so distinct
/// information states can never collide in the memo table. The seat to
/// move and the current legal set are derived views of this key, not
/// stored fields.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct InfoKey {
    policy: PolicyId,
    hand: DominoSet,
    root: RootPosition,
    history: Vec<Domino>,
}

impl InfoKey {
    /// The one constructor (see the type-level contract above). Panics if
    /// the claimed hand overlaps publicly played tiles — an information
    /// state must be internally consistent.
    pub fn from_public(policy: PolicyId, hand: DominoSet, record: &PublicRecord<'_>) -> InfoKey {
        let played: DominoSet = record
            .root
            .trick_plays
            .iter()
            .chain(record.history.iter())
            .copied()
            .collect();
        assert!(
            hand.is_disjoint(played),
            "a remaining hand is disjoint from the public play history"
        );
        InfoKey {
            policy,
            hand,
            root: record.root.clone(),
            history: record.history.to_vec(),
        }
    }

    /// A 64-bit fold of the key's canonical serialization, for discovery
    /// seed derivation (§12.4). Collisions here cost discovery-stream
    /// overlap only — never memo-table identity, which uses the full key.
    pub fn digest64(&self) -> u64 {
        let mut canon = Canon::new("walt-info-key-v1");
        canon.tag(0x01);
        canon.bytes.extend_from_slice(self.policy.bytes());
        canon.tag(0x02);
        canon.u32(self.hand.bits());
        canon.tag(0x03);
        canon.u8(arena_decl_id(self.root.decl) as u8);
        canon.u32(self.root.bid);
        canon.u8(self.root.declaring_team.index() as u8);
        canon.u8(self.root.leader.index() as u8);
        canon.u32(self.root.banked[0]);
        canon.u32(self.root.banked[1]);
        canon.dominoes_field(0x04, &self.root.trick_plays);
        canon.dominoes_field(0x05, &self.history);
        let digest = sha256::digest(&canon.finish());
        u64::from_be_bytes([
            digest[0], digest[1], digest[2], digest[3], digest[4], digest[5], digest[6], digest[7],
        ])
    }
}

// ---------------------------------------------------------------------------
// The frozen policy: lazy materialization over an immutable cache
// (§12.2, §12.5).
// ---------------------------------------------------------------------------

/// A frozen policy: a [`FreezeTuple`], its [`PolicyId`], and the per-id
/// memo table (information state → chosen legal action) of §12.2.
///
/// First visit to an information state computes the action under the
/// frozen discovery configuration and caches it; every later replay reads
/// the cache (§13.1's cure: replay is cheap after materialization). The
/// cache is IMMUTABLE in the O22 sense: entries are only ever inserted on
/// a miss, and no API can replace or remove one.
pub struct FrozenPolicy {
    tuple: FreezeTuple,
    id: PolicyId,
    label: String,
    cache: Mutex<HashMap<InfoKey, Domino>>,
}

impl FrozenPolicy {
    /// Freeze a tuple. Validates the action rule's parameters (a
    /// preference order must be a permutation of all 28 tiles).
    pub fn new(tuple: FreezeTuple) -> FrozenPolicy {
        match &tuple.action_rule {
            ActionRule::Preference(order) => {
                let mut seen = DominoSet::EMPTY;
                for d in order {
                    assert!(seen.insert(*d), "a preference order lists a tile twice");
                }
                assert_eq!(seen, DominoSet::FULL, "a preference order covers all 28");
            }
        }
        let id = tuple.policy_id();
        let label = format!("frozen:{id}");
        FrozenPolicy {
            tuple,
            id,
            label,
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub fn policy_id(&self) -> PolicyId {
        self.id
    }

    pub fn tuple(&self) -> &FreezeTuple {
        &self.tuple
    }

    /// §12.2 — materialize the action at `key`. On a miss, the action is
    /// computed under the frozen configuration from the information state
    /// alone and cached; on a hit, the cached action is returned
    /// unchanged. `legal` is the caller's derived legal set — a pure
    /// function of the key's public data plus the hand, asserted
    /// consistent with whatever the cache holds.
    pub fn action(&self, key: InfoKey, legal: DominoSet) -> Domino {
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        assert!(
            legal.is_subset_of(key.hand),
            "the legal set is drawn from the focal hand"
        );
        let mut cache = self.cache.lock().expect("the action cache is unpoisoned");
        if let Some(&cached) = cache.get(&key) {
            assert!(
                legal.contains(cached),
                "a cached action is legal: the legal set is a function of its key"
            );
            return cached;
        }
        let chosen = match &self.tuple.action_rule {
            ActionRule::Preference(order) => *order
                .iter()
                .find(|d| legal.contains(**d))
                .expect("a total preference order meets a nonempty legal set"),
        };
        let previous = cache.insert(key, chosen);
        assert!(
            previous.is_none(),
            "a cache entry, once written, never changes (O22)"
        );
        chosen
    }

    /// How many information states have been materialized so far. A
    /// replay that revisits only known states leaves this unchanged.
    pub fn cache_len(&self) -> usize {
        self.cache
            .lock()
            .expect("the action cache is unpoisoned")
            .len()
    }

    /// A snapshot of the memo table, for immutability gates: after any
    /// amount of further replay, every entry of an earlier snapshot must
    /// still be present with the same action.
    pub fn cache_snapshot(&self) -> HashMap<InfoKey, Domino> {
        self.cache
            .lock()
            .expect("the action cache is unpoisoned")
            .clone()
    }

    /// §12.4 — the counter-based DISCOVERY stream for one information
    /// state: a pure function of (domain tag, [`PolicyId`], the tuple's
    /// discovery seed schedule, the information state, `counter`). Its
    /// inputs are the key and the tuple alone — no evaluation world, no
    /// hidden hand, can reach it, by signature. This step's deterministic
    /// action rule never draws from it; §22 step 10's sampled discovery
    /// solver will.
    pub fn discovery_rng(&self, key: &InfoKey, counter: u64) -> KernelRng {
        let mut seed = mix(DISCOVERY_DOMAIN ^ self.id.fold64());
        for s in &self.tuple.discovery_seed_schedule {
            seed = mix(seed ^ s);
        }
        seed = mix(seed ^ key.digest64());
        seed = mix(seed ^ counter);
        KernelRng::new(seed)
    }
}

impl SlicePolicy for FrozenPolicy {
    /// The policy names itself by its content address: every result
    /// winner string carries the immutable [`PolicyId`] (O22).
    fn id(&self) -> &str {
        &self.label
    }

    fn choose(
        &self,
        decl: Decl,
        hand: DominoSet,
        legal: DominoSet,
        record: &PublicRecord<'_>,
    ) -> Domino {
        assert_eq!(
            decl, self.tuple.decl,
            "a frozen policy is asked only about its frozen declaration"
        );
        assert_eq!(
            record.root.decl, self.tuple.decl,
            "the record's root declaration matches the freeze tuple"
        );
        assert_eq!(
            record.root.bid, self.tuple.bid,
            "the record's root bid matches the freeze tuple"
        );
        assert_eq!(
            record.root.declaring_team, self.tuple.declaring_team,
            "the record's declaring team matches the freeze tuple"
        );
        let key = InfoKey::from_public(self.id, hand, record);
        self.action(key, legal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    /// The vendored SHA-256 against the standard FIPS 180-4 vectors.
    #[test]
    fn sha256_matches_the_standard_test_vectors() {
        assert_eq!(
            hex(&sha256::digest(b"")),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            hex(&sha256::digest(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            hex(&sha256::digest(
                b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"
            )),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
        // A two-block message (length > 64 bytes).
        assert_eq!(
            hex(&sha256::digest(&[0x61u8; 112])),
            sha256_reference_112_a()
        );
    }

    /// SHA-256 of 112 'a' bytes, computed independently (python3 hashlib).
    fn sha256_reference_112_a() -> String {
        "f54353008a2553262ecdc4a34749563ba0950e8b0fc8652780b0a614b99683c1".to_string()
    }

    /// Length prefixes prevent adjacent-field aliasing in the canonical
    /// serialization.
    #[test]
    fn canonical_serialization_does_not_alias_adjacent_strings() {
        let mut one = Canon::new("t");
        one.str_field(0x01, "ab");
        one.str_field(0x02, "c");
        let mut two = Canon::new("t");
        two.str_field(0x01, "a");
        two.str_field(0x02, "bc");
        assert_ne!(one.finish(), two.finish());
    }
}
