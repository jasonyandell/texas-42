//! `solver::proof_state` — the §49 architecture spike of the anytime
//! proof-state program (`walt/math/anytime_proof_state_score_v0.1.md`
//! Parts VI and XI, rulings APS-A1..A9): the smallest honest kernel of
//! a persistent, serializable, identity-scoped proof state over one
//! root, with an OPEN producer registry.
//!
//! EXPLORATORY tier. Mathematical source: the anytime parent §24–§28
//! (proof states, the top state, zero-cost closure, δ-states, decision
//! exactness), §29–§31 (root intervals, proof bar vs executable bar,
//! certified regret), §41 (closure-aware derivation), §50–§56 (the data
//! model this adapts), §49 (the spike checklist this exists to pass);
//! design register `walt/FACTOR-BELIEF.md`; intake companion
//! `walt/math/anytime_proof_state_score_v0.1_intake.md`.
//!
//! THE LAW. A proof state is an append-only store of typed FACTS under
//! one declared [`SemanticsIdentity`]. Everything else — installed
//! intervals, the proof bar, the executable bar, survivors, exclusions,
//! the typed result — is a DERIVED VIEW recomputed from the facts
//! ([`ProofState::closure`]), never stored beside them: storing both
//! authorities is forbidden (the repo-wide derived-views rule). The
//! installed lower of an action is the maximum over its valid lower
//! facts (vacuous 0 included); the installed upper is the minimum over
//! its valid upper facts (vacuous 1 included); the bar is the §29
//! `B = max_a L_a`; an action is excluded exactly when `U_a < B`.
//! Soundness is inherited from the producers: every fact names its
//! authority and proof class, a δ-qualified fact carries its full
//! [`ScopedDelta`] provenance, and the closure's result is
//! δ-DECISIVE-flagged whenever a decisive exclusion comparison rests on
//! a sampled fact (final-state view — see the δ note below).
//!
//! THE EXECUTABLE BAR (§30). A lower fact carries `executable`: true
//! exactly when a materialized lawful policy witnesses it (a sampled
//! evaluation of a pinned policy, an exact fixed-policy mass, a score
//! profile's tail projection). Grammar and full-response optima are
//! proof-bar-only until an argmax policy is extracted and re-priced
//! (§30's rule; the Slice G `bar_of` audit finding at APS-A6).
//! `B_exec = max` over executable lowers, and `B_exec ≤ B_proof` is
//! asserted inside every closure.
//!
//! CLOSURE-AWARE DERIVATION (§41, §26). A [`ScoreProfileFact`] is not a
//! root bound — closure projects it: the viewer-objective tail of its
//! 43 bins at the identity's contract is a deterministic EXECUTABLE
//! lower for its declared action. The derivation happens inside the
//! derived views (never stored), so closure is idempotent by
//! construction — and gated anyway.
//!
//! THE OPEN REGISTRY (§49's seventh requirement). A producer is any
//! implementor of [`ProofProducer`] — adding one edits NO enum in this
//! module (the deliberate break from RefineV1's closed work-item
//! universe, frozen at freeze 58). Fact KINDS remain a closed type
//! vocabulary ([`Fact`]) — kinds are the typed language of the state,
//! producers are the open population that speaks it.
//!
//! SERIALIZATION. `walt-proof-state-v1`: a versioned line format,
//! exact rationals as `num/den`, facts in insertion order, each line
//! carrying the fact's 128-bit FNV-1a content hash. Parsing
//! re-validates every hash and every identity; a resumed state
//! re-serializes bytewise identically (gated).
//!
//! THE δ NOTE (§27). RefineV1 classifies δ-decisiveness at exclusion
//! EVENT time and never retroactively upgrades (§27's rule). This
//! state recomputes from FINAL facts, which agrees with the event view
//! on every prefix-0 (all-exact) run and can only differ by finding a
//! comparison exact that was sampled at event time; when importing V1
//! outcomes wholesale, V1's own proof class travels as the authority
//! and this module's flag is a recomputation, not a promotion.
//!
//! WHAT THIS SPIKE IS NOT. No scheduler, no budgets, no forecasts, no
//! §34 refusals, no risk-ledger arithmetic (the ledger stays with the
//! producers; facts RECORD ScopedDelta provenance and never re-spend
//! it), no §33 producer implementations, no cross-root anything, and
//! no change to `solver::refine` — RefineV1 is frozen (freeze 58) and
//! this module must remain deletable without touching it or anything
//! else (§67.10, asserted by the module graph: nothing imports this
//! module except the crate root, `solver::extraction` (the §63
//! producer) and `solver::frontier` (the §39–§43 work frontier) —
//! both new-core, deletable with this module as one boundary).

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

use crate::rules::{legal_plays, Domino, DominoSet};
use crate::solver::adaptive::{root_identity, CanonicalRoot, RootPosition};
use crate::solver::evidence::ScopedDelta;

/// The §51 semantics identity: every fact names the coordinates it
/// depends on, and a fact whose identity differs from the state's in
/// ANY coordinate is rejected at install (gated).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemanticsIdentity {
    /// `root_identity(root, position)` — the root's public identity.
    pub root_id: u64,
    /// The rules authority. One value exists today.
    pub rules_id: String,
    /// The declared field's construction identity.
    pub field_id: String,
    /// The objective. One value exists today (`pmake-v1`).
    pub utility_id: String,
    /// The contract threshold in points.
    pub contract: u32,
    /// The belief's identity (`uniform-root` for the spike).
    pub belief_id: String,
    /// The focal policy class identity.
    pub policy_class_id: String,
    /// The score semantics (`declaring-banked-43bin-v1`).
    pub score_semantics_id: String,
}

impl SemanticsIdentity {
    fn serialize(&self) -> String {
        for s in [
            &self.rules_id,
            &self.field_id,
            &self.utility_id,
            &self.belief_id,
            &self.policy_class_id,
            &self.score_semantics_id,
        ] {
            assert_ident_string(s);
        }
        format!(
            "identity root={} rules={} field={} utility={} contract={} belief={} policyclass={} score={}",
            self.root_id,
            self.rules_id,
            self.field_id,
            self.utility_id,
            self.contract,
            self.belief_id,
            self.policy_class_id,
            self.score_semantics_id,
        )
    }
}

/// Identity strings live in the serialized line format: no whitespace,
/// no `|`, nonempty. Asserted at construction sites, checked at parse.
fn assert_ident_string(s: &str) {
    assert!(
        !s.is_empty() && !s.contains(char::is_whitespace) && !s.contains('|'),
        "an identity string is nonempty and free of whitespace and '|'"
    );
}

/// 128-bit FNV-1a over bytes (the freeze-1 hash family; the record
/// variant lives in `strat::info` — this is the plain byte form).
fn fnv128_bytes(bytes: &[u8]) -> u128 {
    const OFFSET: u128 = 0x6c62272e07bb014262b821756295c58d;
    const PRIME: u128 = 0x0000000001000000000000000000013b;
    let mut h = OFFSET;
    for b in bytes {
        h ^= *b as u128;
        h = h.wrapping_mul(PRIME);
    }
    h
}

/// A fact's proof class: deterministic (exact or structural under its
/// declared scope) or δ-qualified with its full sampled provenance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProofTag {
    Deterministic,
    /// The fact holds on a validity event of probability ≥ 1 − δ,
    /// charged to the named scope by its PRODUCER — this module records
    /// the provenance and never re-spends the risk.
    Sampled {
        scope: String,
        delta: BigRational,
    },
}

/// Which side of an interval a bound fact installs. The side is fixed
/// at construction ([`BoundFact::lower`] / [`BoundFact::upper`]) — no
/// path exists that reinterprets one side as the other (§58's gate).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundSide {
    Lower,
    Upper,
}

/// One root-action bound fact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundFact {
    pub action: Domino,
    pub side: BoundSide,
    /// The bound value in `[0, 1]`, exact.
    pub value: BigRational,
    /// The producing authority, e.g. `refine-v1:exact-policy:pinned-level1`.
    pub authority: String,
    /// Executable lowers only (§30): a materialized lawful policy
    /// witnesses this value. Always `false` on uppers.
    pub executable: bool,
    pub proof: ProofTag,
}

impl BoundFact {
    /// A lower bound fact. `executable` per §30: true only for
    /// materialized-policy witnesses.
    pub fn lower(
        action: Domino,
        value: BigRational,
        authority: &str,
        executable: bool,
        proof: ProofTag,
    ) -> BoundFact {
        BoundFact {
            action,
            side: BoundSide::Lower,
            value,
            authority: authority.to_string(),
            executable,
            proof,
        }
    }

    /// An upper bound fact. Uppers are never executable.
    pub fn upper(
        action: Domino,
        value: BigRational,
        authority: &str,
        proof: ProofTag,
    ) -> BoundFact {
        BoundFact {
            action,
            side: BoundSide::Upper,
            value,
            authority: authority.to_string(),
            executable: false,
            proof,
        }
    }
}

/// A fixed-policy 43-bin score profile as a FACT (§53): the exact
/// declaring-score bins of one materialized policy whose root action is
/// `action`. Closure projects it to a deterministic executable lower —
/// the fact itself is not a root bound (§41).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScoreProfileFact {
    pub action: Domino,
    /// The materialized policy's identity.
    pub policy_id: String,
    /// Exact world mass per declaring-team final score, `0..=42`.
    pub bins: [u128; 43],
}

impl ScoreProfileFact {
    fn total(&self) -> u128 {
        self.bins
            .iter()
            .try_fold(0u128, |a, b| a.checked_add(*b))
            .expect("an exact mass fits u128")
    }

    fn tail(&self, k: u32) -> u128 {
        self.bins
            .iter()
            .enumerate()
            .filter(|(s, _)| *s as u32 >= k)
            .map(|(_, m)| m)
            .sum()
    }
}

/// The closed type vocabulary of facts. Producers are open
/// ([`ProofProducer`]); kinds are the typed language they speak.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fact {
    Bound(BoundFact),
    Profile(Box<ScoreProfileFact>),
}

/// Why an install was refused. Every rejection is recorded in the
/// trace; none changes the state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Reject {
    /// The fact's identity differs from the state's (§51: any
    /// coordinate).
    IdentityMismatch,
    /// A bound outside `[0, 1]`, or a profile with zero total mass.
    MalformedValue,
    /// The action is not legal at this root.
    UnknownAction,
}

/// One stored fact: content-hash id plus the fact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredFact {
    pub id: u128,
    pub fact: Fact,
}

/// Trace events: what was installed, what was refused, nothing hidden.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProofEvent {
    Installed { id: u128 },
    Rejected { reason: Reject },
}

/// The typed closure result (§28/§37.9 shape, matching RefineV1's
/// vocabulary): a settled unique action, an exact tie set, or the
/// honest surviving set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StateResult {
    Settled {
        action: Domino,
    },
    Equivalent {
        actions: Vec<Domino>,
        value: BigRational,
    },
    Unresolved {
        survivors: Vec<Domino>,
    },
}

/// The derived view of one action after closure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionView {
    pub action: Domino,
    pub lower: BigRational,
    pub upper: BigRational,
    /// The argmax lower fact is sampled (vacuous counts deterministic).
    pub lower_sampled: bool,
    /// The argmin upper fact is sampled (vacuous counts deterministic).
    pub upper_sampled: bool,
    pub excluded: bool,
}

/// The executable-bar witness (§30): the strongest lower fact backed by
/// a MATERIALIZED lawful policy, with everything the §33 report needs
/// to name it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecWitness {
    pub action: Domino,
    pub value: BigRational,
    /// The witnessing fact's authority (bound facts) or
    /// `profile:<policy_id>` (profile projections).
    pub authority: String,
    /// The witness itself is δ-qualified.
    pub sampled: bool,
    /// The witnessing fact's id — profile witnesses resolve to bins for
    /// score floors, ceilings, and bands.
    pub fact_id: u128,
}

/// The §26 zero-cost closure's complete report — a pure function of
/// the fact store, recomputed on demand.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClosureReport {
    pub views: Vec<ActionView>,
    pub bar: BigRational,
    pub bar_holder: Domino,
    pub survivors: Vec<Domino>,
    pub excluded: Vec<Domino>,
    /// The §30 executable bar's witness, when any executable lower
    /// exists. `B_exec` defaults to the vacuous 0 in the regret below.
    pub exec: Option<ExecWitness>,
    /// The §31 global upper `U* = max_a U_a` (equal over all actions
    /// and over survivors: an excluded upper sits below the bar, which
    /// the bar holder's own upper meets or exceeds).
    pub u_star: BigRational,
    /// The §31 certified pmake regret `Γ = U* − B_exec` (with the
    /// vacuous `B_exec = 0` at zero executable work): on the joint
    /// validity event, the recommended executable policy leaves at
    /// most this much pmake unclaimed. Monotone nonincreasing under
    /// refinement (gated).
    pub certified_regret: BigRational,
    pub result: StateResult,
    /// A decisive exclusion comparison rests on a sampled fact
    /// (final-state view — the module doc's δ note).
    pub delta_decisive: bool,
}

/// The §33 recommendation block, derived from a closure whose
/// executable bar is inhabited: the policy Walt would actually play,
/// with its floor, the world's ceiling, and the certified gap. Score
/// coordinates (floor/ceiling/bands) are DECLARING-side quantities
/// from the witnessing profile's bins — named so the setting-viewer
/// parity cannot be misread — and exist only for profile witnesses.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Recommendation {
    pub action: Domino,
    /// The witnessing authority (§33 "recommended policy").
    pub policy: String,
    /// `B_exec` — the certified pmake floor of the recommendation.
    pub pmake_lower: BigRational,
    /// `U*` — the valid upper on the unknown best response.
    pub global_upper: BigRational,
    /// `Γ = U* − B_exec`.
    pub certified_regret: BigRational,
    /// Lowest declaring score of nonzero mass (profile witnesses).
    pub declaring_score_floor: Option<u32>,
    /// Highest declaring score of nonzero mass (profile witnesses).
    pub declaring_score_ceiling: Option<u32>,
    /// §7 contract-sensitive residual of the witness: exactly 0 for an
    /// exact profile (its cells are points); `None` for bound-fact
    /// witnesses. Nonzero values arrive with envelope cells (Phase 4).
    pub contract_sensitive_residual: Option<BigRational>,
    /// §11 declaring fragile-make mass at d = 1: the exact share of
    /// the witness profile sitting on `[c, c+1)` — one point of
    /// slippage unmakes it (profile witnesses).
    pub declaring_fragile_d1: Option<BigRational>,
    /// §10 declaring rescue mass at d = 1: the share on `[c−1, c)`.
    pub declaring_rescue_d1: Option<BigRational>,
    /// The witness is δ-qualified.
    pub sampled: bool,
    /// Every sampled scope present in the store — the coarse §33 risk
    /// summary (decisiveness itself is `delta_decisive` on the
    /// closure).
    pub risk_scopes: Vec<String>,
}

/// An open-registry producer: reads the state, proposes facts. Adding
/// a producer edits no enum here — the §49 seventh requirement.
pub trait ProofProducer {
    fn name(&self) -> &str;
    fn produce(&self, state: &ProofState) -> Vec<Fact>;
}

/// The persistent proof state over one root.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofState {
    pub identity: SemanticsIdentity,
    /// The legal root actions, in tile-index order (the declared
    /// deterministic order of every derived view).
    pub legal: Vec<Domino>,
    facts: Vec<StoredFact>,
    pub trace: Vec<ProofEvent>,
}

impl ProofState {
    /// The §25 top state: every legal action alive at `[0, 1]`, zero
    /// facts, zero paid work. Not a failure — the initial theorem.
    pub fn open(
        root: &CanonicalRoot,
        position: &RootPosition,
        identity: SemanticsIdentity,
    ) -> ProofState {
        assert_eq!(
            identity.root_id,
            root_identity(root, position),
            "the identity names this root"
        );
        assert!(
            position.trick_plays.is_empty(),
            "the spike's roots are trick-start roots with the viewer to move"
        );
        let legal_set = legal_plays(position.decl, root.kernel().viewer_hand(), None);
        let mut legal: Vec<Domino> = (0..DominoSet::FULL.len())
            .filter_map(Domino::from_index)
            .filter(|d| legal_set.contains(*d))
            .collect();
        legal.sort_by_key(|d| d.index());
        assert!(!legal.is_empty(), "a root holds a legal action");
        ProofState {
            identity,
            legal,
            facts: Vec::new(),
            trace: Vec::new(),
        }
    }

    /// Install one fact under a claimed identity. The identity must
    /// equal the state's in every coordinate; values must be sound by
    /// type. Rejections record and change nothing.
    pub fn install(&mut self, claimed: &SemanticsIdentity, fact: Fact) -> Result<u128, Reject> {
        if *claimed != self.identity {
            self.trace.push(ProofEvent::Rejected {
                reason: Reject::IdentityMismatch,
            });
            return Err(Reject::IdentityMismatch);
        }
        let ok = match &fact {
            Fact::Bound(b) => {
                assert_ident_string(&b.authority);
                if let ProofTag::Sampled { scope, delta } = &b.proof {
                    assert_ident_string(scope);
                    assert!(
                        *delta > BigRational::zero() && *delta < BigRational::one(),
                        "a declared delta lies in (0, 1)"
                    );
                }
                if !self.legal.contains(&b.action) {
                    self.trace.push(ProofEvent::Rejected {
                        reason: Reject::UnknownAction,
                    });
                    return Err(Reject::UnknownAction);
                }
                b.value >= BigRational::zero() && b.value <= BigRational::one()
            }
            Fact::Profile(p) => {
                assert_ident_string(&p.policy_id);
                if !self.legal.contains(&p.action) {
                    self.trace.push(ProofEvent::Rejected {
                        reason: Reject::UnknownAction,
                    });
                    return Err(Reject::UnknownAction);
                }
                p.total() > 0
            }
        };
        if !ok {
            self.trace.push(ProofEvent::Rejected {
                reason: Reject::MalformedValue,
            });
            return Err(Reject::MalformedValue);
        }
        let line = serialize_fact_body(&fact);
        let id = fnv128_bytes(line.as_bytes());
        self.facts.push(StoredFact { id, fact });
        self.trace.push(ProofEvent::Installed { id });
        Ok(id)
    }

    /// Run one producer: every proposed fact goes through the same
    /// install fence under the state's own identity.
    pub fn run_producer(&mut self, producer: &dyn ProofProducer) -> Vec<Result<u128, Reject>> {
        assert_ident_string(producer.name());
        let identity = self.identity.clone();
        producer
            .produce(&{ self.clone() })
            .into_iter()
            .map(|f| self.install(&identity, f))
            .collect()
    }

    pub fn facts(&self) -> &[StoredFact] {
        &self.facts
    }

    /// The viewer-objective projection of a profile fact at the
    /// identity's contract: declaring viewers make on the tail,
    /// setting viewers on the complement. The spike's identities carry
    /// the parity in `utility_id` (`pmake-v1` = declaring viewer,
    /// `pmake-setting-v1` = setting viewer).
    fn profile_projection(&self, p: &ScoreProfileFact) -> BigRational {
        let z = p.total();
        let tail = p.tail(self.identity.contract);
        let mass = match self.identity.utility_id.as_str() {
            "pmake-v1" => tail,
            "pmake-setting-v1" => z - tail,
            other => panic!("an unknown utility identity: {other}"),
        };
        BigRational::new(BigInt::from(mass), BigInt::from(z))
    }

    /// The §26 zero-cost closure: recompute every derived view from
    /// the facts. Pure — calling it twice yields equal reports
    /// (gated), because nothing is stored.
    pub fn closure(&self) -> ClosureReport {
        let zero = BigRational::zero();
        let one = BigRational::one();
        let mut views = Vec::with_capacity(self.legal.len());
        let mut exec: Option<ExecWitness> = None;
        for a in &self.legal {
            // Installed lower: max over lower facts and profile
            // projections (§41 derivation), vacuous 0 included.
            let mut lower = zero.clone();
            let mut lower_sampled = false;
            let mut upper = one.clone();
            let mut upper_sampled = false;
            for sf in &self.facts {
                match &sf.fact {
                    Fact::Bound(b) if b.action == *a => match b.side {
                        BoundSide::Lower => {
                            if b.value > lower {
                                lower = b.value.clone();
                                lower_sampled = matches!(b.proof, ProofTag::Sampled { .. });
                            }
                            if b.executable {
                                let better = match &exec {
                                    None => true,
                                    Some(w) => b.value > w.value,
                                };
                                if better {
                                    exec = Some(ExecWitness {
                                        action: *a,
                                        value: b.value.clone(),
                                        authority: b.authority.clone(),
                                        sampled: matches!(b.proof, ProofTag::Sampled { .. }),
                                        fact_id: sf.id,
                                    });
                                }
                            }
                        }
                        BoundSide::Upper => {
                            if b.value < upper {
                                upper = b.value.clone();
                                upper_sampled = matches!(b.proof, ProofTag::Sampled { .. });
                            }
                        }
                    },
                    Fact::Profile(p) if p.action == *a => {
                        let v = self.profile_projection(p);
                        if v > lower {
                            lower = v.clone();
                            lower_sampled = false;
                        }
                        let better = match &exec {
                            None => true,
                            Some(w) => v > w.value,
                        };
                        if better {
                            exec = Some(ExecWitness {
                                action: *a,
                                value: v,
                                authority: format!("profile:{}", p.policy_id),
                                sampled: false,
                                fact_id: sf.id,
                            });
                        }
                    }
                    _ => {}
                }
            }
            assert!(
                lower <= upper,
                "the §37 invariant: simultaneously valid bounds are ordered"
            );
            views.push(ActionView {
                action: *a,
                lower,
                upper,
                lower_sampled,
                upper_sampled,
                excluded: false,
            });
        }
        let (bar, bar_holder, bar_sampled) = {
            let best = views
                .iter()
                .max_by(|x, y| x.lower.cmp(&y.lower))
                .expect("a root holds a legal action");
            // Ties keep the lowest tile: max_by returns the LAST max,
            // so scan explicitly in tile order for the first argmax.
            let first = views
                .iter()
                .find(|v| v.lower == best.lower)
                .expect("the max exists");
            (first.lower.clone(), first.action, first.lower_sampled)
        };
        if let Some(w) = &exec {
            assert!(w.value <= bar, "the §30 chain: B_exec ≤ B_proof");
        }
        let mut delta_decisive = false;
        for v in views.iter_mut() {
            if v.upper < bar {
                v.excluded = true;
                if v.upper_sampled || bar_sampled {
                    delta_decisive = true;
                }
            }
        }
        let survivors: Vec<Domino> = views
            .iter()
            .filter(|v| !v.excluded)
            .map(|v| v.action)
            .collect();
        let excluded: Vec<Domino> = views
            .iter()
            .filter(|v| v.excluded)
            .map(|v| v.action)
            .collect();
        assert!(
            survivors.contains(&bar_holder),
            "the bar holder survives its own bar"
        );
        let result = if survivors.len() == 1 {
            StateResult::Settled {
                action: survivors[0],
            }
        } else {
            let all_points = views
                .iter()
                .filter(|v| !v.excluded)
                .all(|v| v.lower == v.upper && !v.lower_sampled && !v.upper_sampled);
            if all_points {
                StateResult::Equivalent {
                    actions: survivors.clone(),
                    value: bar.clone(),
                }
            } else {
                StateResult::Unresolved {
                    survivors: survivors.clone(),
                }
            }
        };
        let u_star = views
            .iter()
            .map(|v| v.upper.clone())
            .max()
            .expect("a root holds a legal action");
        let exec_value = exec
            .as_ref()
            .map(|w| w.value.clone())
            .unwrap_or_else(BigRational::zero);
        assert!(
            bar <= u_star,
            "the §31 chain: the bar holder's own upper meets the bar"
        );
        let certified_regret = &u_star - &exec_value;
        ClosureReport {
            views,
            bar,
            bar_holder,
            survivors,
            excluded,
            exec,
            u_star,
            certified_regret,
            result,
            delta_decisive,
        }
    }

    /// The §33 recommendation block: derived from the closure, present
    /// exactly when an executable witness exists (at zero executable
    /// work there is no policy to recommend — the regret is still
    /// defined on the closure with the vacuous floor). Score
    /// coordinates come from the witnessing profile's bins; a
    /// bound-fact witness recommends its policy without them.
    pub fn recommend(&self) -> Option<Recommendation> {
        let report = self.closure();
        let w = report.exec?;
        let c = self.identity.contract;
        let mut floor = None;
        let mut ceiling = None;
        let mut residual = None;
        let mut fragile = None;
        let mut rescue = None;
        if let Some(sf) = self.facts.iter().find(|sf| sf.id == w.fact_id) {
            if let Fact::Profile(p) = &sf.fact {
                let z = BigInt::from(p.total());
                floor = p
                    .bins
                    .iter()
                    .position(|m| *m > 0)
                    .map(|s| u32::try_from(s).expect("s <= 42"));
                ceiling = p
                    .bins
                    .iter()
                    .rposition(|m| *m > 0)
                    .map(|s| u32::try_from(s).expect("s <= 42"));
                // An exact profile's cells are points: the §7 straddle
                // mass is exactly zero. Envelope cells (Phase 4) are
                // what make it positive.
                residual = Some(BigRational::zero());
                fragile = Some(BigRational::new(
                    BigInt::from(p.tail(c) - p.tail(c + 1)),
                    z.clone(),
                ));
                rescue = Some(BigRational::new(
                    BigInt::from(p.tail(c.saturating_sub(1)) - p.tail(c)),
                    z,
                ));
            }
        }
        let mut risk_scopes: Vec<String> = Vec::new();
        for sf in &self.facts {
            if let Fact::Bound(b) = &sf.fact {
                if let ProofTag::Sampled { scope, .. } = &b.proof {
                    if !risk_scopes.contains(scope) {
                        risk_scopes.push(scope.clone());
                    }
                }
            }
        }
        risk_scopes.sort();
        Some(Recommendation {
            action: w.action,
            policy: w.authority,
            pmake_lower: w.value,
            global_upper: report.u_star,
            certified_regret: report.certified_regret,
            declaring_score_floor: floor,
            declaring_score_ceiling: ceiling,
            contract_sensitive_residual: residual,
            declaring_fragile_d1: fragile,
            declaring_rescue_d1: rescue,
            sampled: w.sampled,
            risk_scopes,
        })
    }

    /// Serialize: the versioned line format of the module doc. Facts
    /// in insertion order; every line self-hashed; deterministic.
    pub fn serialize(&self) -> String {
        let mut out = String::from("walt-proof-state-v1\n");
        out.push_str(&self.identity.serialize());
        out.push('\n');
        let legal: Vec<String> = self.legal.iter().map(|d| format!("{d}")).collect();
        out.push_str(&format!("legal {}\n", legal.join(" ")));
        for sf in &self.facts {
            out.push_str(&format!(
                "fact {:032x} {}\n",
                sf.id,
                serialize_fact_body(&sf.fact)
            ));
        }
        out.push_str(&format!("end {}\n", self.facts.len()));
        out
    }

    /// Resume from a serialized state: re-validate the version, every
    /// content hash, and every value fence. The trace of the original
    /// session is not state — a resumed state starts a fresh trace
    /// (facts, not events, are the theorem).
    pub fn parse(
        text: &str,
        root: &CanonicalRoot,
        position: &RootPosition,
    ) -> Result<ProofState, String> {
        let mut lines = text.lines();
        if lines.next() != Some("walt-proof-state-v1") {
            return Err("an unknown serialization version".to_string());
        }
        let identity = parse_identity(lines.next().ok_or("a missing identity line")?)?;
        let legal_line = lines.next().ok_or("a missing legal line")?;
        let legal_body = legal_line
            .strip_prefix("legal ")
            .ok_or("a malformed legal line")?;
        let mut state = ProofState::open(root, position, identity);
        let expect_legal: Vec<String> = state.legal.iter().map(|d| format!("{d}")).collect();
        if legal_body != expect_legal.join(" ") {
            return Err("the legal set does not match the root".to_string());
        }
        let mut count = 0usize;
        for line in lines {
            if let Some(rest) = line.strip_prefix("fact ") {
                let (id_hex, body) = rest.split_at(32);
                let body = body.strip_prefix(' ').ok_or("a malformed fact line")?;
                let id = u128::from_str_radix(id_hex, 16).map_err(|_| "a malformed fact id")?;
                if fnv128_bytes(body.as_bytes()) != id {
                    return Err("a fact hash mismatch".to_string());
                }
                let fact = parse_fact_body(body)?;
                let claimed = state.identity.clone();
                let installed = state
                    .install(&claimed, fact)
                    .map_err(|r| format!("a rejected fact on resume: {r:?}"))?;
                if installed != id {
                    return Err("a fact re-hash mismatch".to_string());
                }
                count += 1;
            } else if let Some(n) = line.strip_prefix("end ") {
                if n.parse::<usize>().map_err(|_| "a malformed end count")? != count {
                    return Err("an end-count mismatch".to_string());
                }
                state.trace.clear();
                return Ok(state);
            } else {
                return Err(format!("an unknown line: {line}"));
            }
        }
        Err("a missing end line".to_string())
    }
}

fn rational(v: &BigRational) -> String {
    format!("{}/{}", v.numer(), v.denom())
}

fn parse_rational(s: &str) -> Result<BigRational, String> {
    let (n, d) = s.split_once('/').ok_or("a malformed rational")?;
    let n: BigInt = n.parse().map_err(|_| "a malformed numerator")?;
    let d: BigInt = d.parse().map_err(|_| "a malformed denominator")?;
    if d <= BigInt::from(0) {
        return Err("a nonpositive denominator".to_string());
    }
    Ok(BigRational::new(n, d))
}

fn serialize_fact_body(fact: &Fact) -> String {
    match fact {
        Fact::Bound(b) => {
            let side = match b.side {
                BoundSide::Lower => "lower",
                BoundSide::Upper => "upper",
            };
            let proof = match &b.proof {
                ProofTag::Deterministic => "deterministic".to_string(),
                ProofTag::Sampled { scope, delta } => {
                    format!("delta|{}|{}", scope, rational(delta))
                }
            };
            format!(
                "bound {} {} {} authority={} executable={} proof={}",
                b.action,
                side,
                rational(&b.value),
                b.authority,
                b.executable,
                proof
            )
        }
        Fact::Profile(p) => {
            let bins: Vec<String> = p
                .bins
                .iter()
                .enumerate()
                .filter(|(_, m)| **m > 0)
                .map(|(s, m)| format!("{s}:{m}"))
                .collect();
            format!(
                "profile {} policy={} bins={}",
                p.action,
                p.policy_id,
                bins.join(",")
            )
        }
    }
}

fn parse_domino(s: &str) -> Result<Domino, String> {
    for i in 0..DominoSet::FULL.len() {
        let d = Domino::from_index(i).expect("index < 28");
        if format!("{d}") == s {
            return Ok(d);
        }
    }
    Err(format!("an unknown domino: {s}"))
}

fn parse_fact_body(body: &str) -> Result<Fact, String> {
    let mut parts = body.split(' ');
    match parts.next() {
        Some("bound") => {
            let action = parse_domino(parts.next().ok_or("a missing action")?)?;
            let side = match parts.next() {
                Some("lower") => BoundSide::Lower,
                Some("upper") => BoundSide::Upper,
                _ => return Err("a malformed side".to_string()),
            };
            let value = parse_rational(parts.next().ok_or("a missing value")?)?;
            let authority = parts
                .next()
                .and_then(|s| s.strip_prefix("authority="))
                .ok_or("a missing authority")?
                .to_string();
            let executable = match parts.next().and_then(|s| s.strip_prefix("executable=")) {
                Some("true") => true,
                Some("false") => false,
                _ => return Err("a malformed executable flag".to_string()),
            };
            let proof_s = parts
                .next()
                .and_then(|s| s.strip_prefix("proof="))
                .ok_or("a missing proof tag")?;
            let proof = if proof_s == "deterministic" {
                ProofTag::Deterministic
            } else if let Some(rest) = proof_s.strip_prefix("delta|") {
                let (scope, delta_s) = rest.rsplit_once('|').ok_or("a malformed delta tag")?;
                ProofTag::Sampled {
                    scope: scope.to_string(),
                    delta: parse_rational(delta_s)?,
                }
            } else {
                return Err("an unknown proof tag".to_string());
            };
            if side == BoundSide::Upper && executable {
                return Err("an upper is never executable".to_string());
            }
            Ok(Fact::Bound(BoundFact {
                action,
                side,
                value,
                authority,
                executable,
                proof,
            }))
        }
        Some("profile") => {
            let action = parse_domino(parts.next().ok_or("a missing action")?)?;
            let policy_id = parts
                .next()
                .and_then(|s| s.strip_prefix("policy="))
                .ok_or("a missing policy id")?
                .to_string();
            let bins_s = parts
                .next()
                .and_then(|s| s.strip_prefix("bins="))
                .ok_or("missing bins")?;
            let mut bins = [0u128; 43];
            for pair in bins_s.split(',') {
                let (s, m) = pair.split_once(':').ok_or("a malformed bin")?;
                let s: usize = s.parse().map_err(|_| "a malformed bin score")?;
                if s > 42 {
                    return Err("a bin score above 42".to_string());
                }
                bins[s] = m.parse().map_err(|_| "a malformed bin mass")?;
            }
            Ok(Fact::Profile(Box::new(ScoreProfileFact {
                action,
                policy_id,
                bins,
            })))
        }
        _ => Err("an unknown fact kind".to_string()),
    }
}

fn parse_identity(line: &str) -> Result<SemanticsIdentity, String> {
    let body = line
        .strip_prefix("identity ")
        .ok_or("a malformed identity line")?;
    let mut root_id = None;
    let mut fields = std::collections::BTreeMap::new();
    for part in body.split(' ') {
        let (k, v) = part.split_once('=').ok_or("a malformed identity field")?;
        if k == "root" {
            root_id = Some(v.parse::<u64>().map_err(|_| "a malformed root id")?);
        } else {
            fields.insert(k.to_string(), v.to_string());
        }
    }
    let get = |k: &str| -> Result<String, String> {
        fields
            .get(k)
            .cloned()
            .ok_or(format!("a missing identity field: {k}"))
    };
    Ok(SemanticsIdentity {
        root_id: root_id.ok_or("a missing root id")?,
        rules_id: get("rules")?,
        field_id: get("field")?,
        utility_id: get("utility")?,
        contract: get("contract")?
            .parse()
            .map_err(|_| "a malformed contract")?,
        belief_id: get("belief")?,
        policy_class_id: get("policyclass")?,
        score_semantics_id: get("score")?,
    })
}

// ---------------------------------------------------------------------------
// Import adapters: RefineV1 outcomes as facts (§48's migration step 4).
// ---------------------------------------------------------------------------

use crate::solver::refine::{ActionInterval, LowerBound, UpperBound};

/// Adapt one RefineV1 typed interval into bound facts. The endpoint
/// types map per §30: sampled endpoints carry their ScopedDelta
/// provenance and ARE executable on the lower side (a pinned
/// materialized policy); exact fixed-policy lowers are executable;
/// grammar and response optima are proof-bar-only. Vacuous endpoints
/// produce no fact — the vacuous sides live in the closure.
pub fn facts_from_refine_interval(interval: &ActionInterval) -> Vec<Fact> {
    let mut out = Vec::new();
    let a = interval.action;
    match &interval.lower {
        LowerBound::Vacuous => {}
        LowerBound::Sampled(rl) => {
            let d = rl.delta();
            out.push(Fact::Bound(BoundFact::lower(
                a,
                rl.lower(),
                "refine-v1:sampled-lower",
                true,
                ProofTag::Sampled {
                    scope: d.scope().to_string(),
                    delta: d.delta().clone(),
                },
            )));
        }
        LowerBound::ExactPolicy { mass, policy } => {
            assert_ident_string(policy);
            out.push(Fact::Bound(BoundFact::lower(
                a,
                BigRational::new(BigInt::from(*mass), BigInt::from(interval.z)),
                &format!("refine-v1:exact-policy:{policy}"),
                true,
                ProofTag::Deterministic,
            )));
        }
        LowerBound::ExactGrammar { mass, grammar } => {
            assert_ident_string(grammar);
            out.push(Fact::Bound(BoundFact::lower(
                a,
                BigRational::new(BigInt::from(*mass), BigInt::from(interval.z)),
                &format!("refine-v1:exact-grammar:{grammar}"),
                false,
                ProofTag::Deterministic,
            )));
        }
        LowerBound::ExactResponse { mass } => {
            out.push(Fact::Bound(BoundFact::lower(
                a,
                BigRational::new(BigInt::from(*mass), BigInt::from(interval.z)),
                "refine-v1:exact-response",
                false,
                ProofTag::Deterministic,
            )));
        }
    }
    match &interval.upper {
        UpperBound::Vacuous => {}
        UpperBound::Sampled(ru) => {
            let d = ru.delta();
            out.push(Fact::Bound(BoundFact::upper(
                a,
                ru.upper(),
                "refine-v1:sampled-upper",
                ProofTag::Sampled {
                    scope: d.scope().to_string(),
                    delta: d.delta().clone(),
                },
            )));
        }
        UpperBound::ExactResponse { mass } => {
            out.push(Fact::Bound(BoundFact::upper(
                a,
                BigRational::new(BigInt::from(*mass), BigInt::from(interval.z)),
                "refine-v1:exact-response",
                ProofTag::Deterministic,
            )));
        }
    }
    out
}

/// Keep the ScopedDelta type in this module's public signature so a
/// producer converting evidence facts needs no refine import.
pub type RiskProvenance = ScopedDelta;
