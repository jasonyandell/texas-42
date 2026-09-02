//! EXPLORATORY MODEL-BELIEF INSTRUMENT (`solver::model_belief`, slice
//! MB0) — sits below every evidentiary tier and is cited by nothing
//! above it. The §75 required first report, per enumerable receipt
//! root: physical and augmented masses, the prior and active types by
//! seat, exact branch masses by public action along one observed line
//! with posterior type weights after each observation, the fixed-policy
//! mixture value, the exact mixture response, the type-revealed
//! separated upper, the model-fusion price per root action, the
//! distinct-type-actions versus merged-public-branches census, wall
//! time and declared memory, and the exact (ω,θ) enumeration parity
//! verdict — then the §76 go/no-go criteria as explicit YES/NO lines.
//! Never a play-strength claim.
//!
//! DECLARED EPOCH: F₀ = σ0 = `Level0 { n0 = 2 }` and F₁ =
//! `Level1 { n_outer = 2, n0 = 2 }` (the corrected rung table of the
//! intake companion), registered as hand-persistent behavior types;
//! prior ν = (1/2, 1/2) per hidden seat, independent (weights 1,
//! denominator 8); `SupportOracle`; focal ρ = the lowest-first frozen
//! preference; the six enumerable `verify_player` receipt roots. The
//! SYNTHETIC carrier mixture (lowest-first / highest-first declared
//! types) is reported separately and labeled — it exists because the
//! registered mixture's fusion price is zero on this corpus.
//!
//! Modes:
//!   `modelbeliefreport report <out.txt>` — the declared full run.
//!
//! No floats anywhere; wall time is integer microseconds; values are
//! exact rationals, printed alongside integer permille; memory is a
//! DECLARED ACCOUNTING (a formula over table entries), never presented
//! as a measurement.

use std::fmt::Write as _;
use std::rc::Rc;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::{Kernel, World};
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::rules::legal_plays;
use walt::rules::{Domino, DominoSet, Seat, Trick};
use walt::solver::adaptive::{
    decided_success, CanonicalRoot, FixedPreference, PublicRecord, RootPosition, SlicePolicy,
};
use walt::solver::factor_belief::SupportOracle;
use walt::solver::field::{FieldKind, FieldSpec};
use walt::solver::model_belief::{
    BehaviorType, MixtureStats, ModelBelief, PersistenceScope, SeatTypePrior,
};
use walt::solver::policy::{DecisionMode, TieRule};

/// The six enumerable receipt roots: (hand, trick, fiber).
const ROOTS: [(usize, usize, u128); 6] = [
    (12, 6, 6),
    (10, 6, 19),
    (5, 6, 27),
    (4, 6, 90),
    (8, 5, 92),
    (3, 5, 200),
];

/// Declared-accounting bytes per stored table entry (one `DominoSet`
/// plus one `u128`, padded) and fixed per-profile overhead (types,
/// weight, field pointer, belief frame). A formula, not a measurement.
const ENTRY_BYTES: u128 = 24;
const PROFILE_BYTES: u128 = 256;

fn field_spec_level0() -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level0 { n0: 2 },
        construction: "level0-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

fn field_spec_level1() -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level1 { n_outer: 2, n0: 2 },
        construction: "level1-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn mixture_of(
    root: &CanonicalRoot,
    position: &RootPosition,
    a: &Rc<BehaviorType>,
    b: &Rc<BehaviorType>,
) -> ModelBelief {
    let priors: Vec<SeatTypePrior> = root
        .kernel()
        .hidden()
        .iter()
        .map(|slot| SeatTypePrior {
            seat: slot.seat,
            types: vec![(Rc::clone(a), 1), (Rc::clone(b), 1)],
        })
        .collect();
    ModelBelief::from_independent_prior(root, position, &priors)
}

/// Exact rational as "num/den (~NNN permille)" — integer arithmetic.
fn ratio(mass: u128, total: u128) -> String {
    assert!(total > 0);
    let permille = mass.checked_mul(1000).expect("fits") / total;
    let r = BigRational::new(BigInt::from(mass), BigInt::from(total));
    format!("{r} (~{permille}‰)")
}

// ---------------------------------------------------------------------------
// The probe's own public replay and (ω,θ) enumeration oracle — the same
// checker shape as the gate file, carried here so the probe's parity
// verdict is independent of the machinery it reports on.
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct Pub {
    leader: Seat,
    plays: Vec<Domino>,
    banked: [u32; 2],
    played_by: [DominoSet; Seat::COUNT],
    history: Vec<Domino>,
}

impl Pub {
    fn start(position: &RootPosition) -> Pub {
        assert!(position.trick_plays.is_empty(), "trick-start roots");
        Pub {
            leader: position.leader,
            plays: Vec::new(),
            banked: position.banked,
            played_by: [DominoSet::EMPTY; Seat::COUNT],
            history: Vec::new(),
        }
    }

    fn seat(&self) -> Seat {
        self.leader.plus(self.plays.len())
    }

    fn play(&mut self, position: &RootPosition, tile: Domino) {
        let seat = self.seat();
        assert!(
            self.played_by[seat.index()].insert(tile),
            "a tile is played once"
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

    fn record<'a>(&'a self, position: &'a RootPosition) -> PublicRecord<'a> {
        PublicRecord {
            leader: self.leader,
            trick_plays: &self.plays,
            banked: self.banked,
            root: position,
            history: &self.history,
        }
    }
}

fn choice_at(
    position: &RootPosition,
    exec: &Pub,
    root_hand: DominoSet,
    policy: &dyn SlicePolicy,
) -> Domino {
    let remaining = root_hand.difference(exec.played_by[exec.seat().index()]);
    let led = exec.plays.first().map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, remaining, led);
    assert!(!legal.is_empty(), "a seat to move holds a legal tile");
    let record = exec.record(position);
    let tile = policy.choose(position.decl, remaining, legal, &record);
    assert!(legal.contains(tile), "a policy chooses a legal tile");
    tile
}

#[allow(clippy::too_many_arguments)]
fn enum_walk(
    model: &ModelBelief,
    position: &RootPosition,
    viewer: Seat,
    viewer_hand: DominoSet,
    total: usize,
    pairs: &[(usize, World)],
    exec: &Pub,
    fixed: Option<&dyn SlicePolicy>,
) -> u128 {
    let at_terminal = exec.history.len() == total;
    if let Some(u) = decided_success(position, viewer, exec.banked, at_terminal) {
        return if u {
            pairs
                .iter()
                .fold(0u128, |acc, (p, _)| acc + model.profiles()[*p].weight())
        } else {
            0
        };
    }
    let seat = exec.seat();
    if seat == viewer {
        let remaining = viewer_hand.difference(exec.played_by[viewer.index()]);
        let led = exec.plays.first().map(|d| position.decl.led_context(*d));
        let legal = legal_plays(position.decl, remaining, led);
        let descend = |tile: Domino| {
            let mut child = exec.clone();
            child.play(position, tile);
            enum_walk(
                model,
                position,
                viewer,
                viewer_hand,
                total,
                pairs,
                &child,
                fixed,
            )
        };
        match fixed {
            Some(policy) => {
                let record = exec.record(position);
                let tile = policy.choose(position.decl, remaining, legal, &record);
                descend(tile)
            }
            None => {
                let mut best: Option<u128> = None;
                for tile in legal.iter() {
                    let m = descend(tile);
                    best = Some(best.map_or(m, |b| b.max(m)));
                }
                best.expect("a legal set holds an action")
            }
        }
    } else {
        let mut groups: Vec<(Domino, Vec<(usize, World)>)> = Vec::new();
        for (p, world) in pairs {
            let field: &dyn SlicePolicy = model.profiles()[*p].field().as_ref();
            let tile = choice_at(position, exec, world.hand(seat), field);
            match groups.iter_mut().find(|(t, _)| *t == tile) {
                Some((_, group)) => group.push((*p, *world)),
                None => groups.push((tile, vec![(*p, *world)])),
            }
        }
        let mut mass: u128 = 0;
        for (tile, group) in groups {
            let mut child = exec.clone();
            child.play(position, tile);
            mass += enum_walk(
                model,
                position,
                viewer,
                viewer_hand,
                total,
                &group,
                &child,
                fixed,
            );
        }
        mass
    }
}

fn all_pairs(model: &ModelBelief, root: &CanonicalRoot) -> Vec<(usize, World)> {
    let worlds: Vec<World> = root.worlds().collect();
    (0..model.profiles().len())
        .flat_map(|p| worlds.iter().map(move |w| (p, *w)))
        .collect()
}

/// The declared-accounting byte total of a model belief's stored
/// factor tables (formula in the header constants).
fn declared_bytes(model: &ModelBelief) -> u128 {
    let mut entries: u128 = 0;
    for profile in model.profiles() {
        for factor in profile.belief().factors() {
            entries += factor.support().len() as u128;
        }
    }
    entries * ENTRY_BYTES + (model.profiles().len() as u128) * PROFILE_BYTES
}

/// One root's §75 block for one labeled mixture.
#[allow(clippy::too_many_lines, clippy::too_many_arguments)]
fn report_root(
    out: &mut String,
    label: &str,
    hand_id: usize,
    trick_no: usize,
    fiber: u128,
    root: &CanonicalRoot,
    position: &RootPosition,
    a: &Rc<BehaviorType>,
    b: &Rc<BehaviorType>,
) {
    let oracle = SupportOracle;
    let focal = FixedPreference::lowest_first("focal:lowest-first");
    let start = Instant::now();
    let model = mixture_of(root, position, a, b);
    let viewer = root.kernel().viewer();
    let viewer_hand = root.kernel().viewer_hand();
    let total_plays = viewer_hand.len()
        + root
            .kernel()
            .hidden()
            .iter()
            .map(|h| h.capacity)
            .sum::<usize>();
    let augmented = model.weighted_total(&oracle);
    let _ = writeln!(out, "== h{hand_id}-t{trick_no} [{label}]");
    let _ = writeln!(
        out,
        "physical world mass Z = {fiber}; augmented mass Σw·Z = {augmented} \
         (prior denominator {} — ν(θ) = 1/{} per profile, {} profiles)",
        model.prior_denominator(),
        model.prior_denominator(),
        model.profiles().len()
    );
    let _ = writeln!(
        out,
        "active types by seat: every hidden seat ∈ {{{}, {}}} at (1/2, 1/2), \
         independent, persistence per hand",
        a.construction(),
        b.construction()
    );
    // Fixed-policy mixture value.
    let mut stats = MixtureStats::default();
    let fixed = model.mixture_policy_mass(&oracle, &focal, &mut stats);
    let _ = writeln!(
        out,
        "fixed-policy mixture value V_nu(lowest-first) = {}",
        ratio(fixed.weighted_mass, fixed.weighted_total)
    );
    // Per-root-action response, sep upper, fusion price.
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, viewer_hand, led);
    let mut q_root: Option<(Domino, u128)> = None;
    for tile in legal.iter() {
        let at_action = model.focal_play(tile);
        let mut rstats = MixtureStats::default();
        let response = at_action.mixture_response(&oracle, &mut rstats);
        let sep = at_action.separated_upper(&oracle);
        assert!(response.outcome.weighted_mass <= sep.weighted_mass);
        let phi = sep.weighted_mass - response.outcome.weighted_mass;
        let _ = writeln!(
            out,
            "  action {tile:?}: Q_a = {}; U^sep_a = {}; fusion price Phi_a = {}",
            ratio(
                response.outcome.weighted_mass,
                response.outcome.weighted_total
            ),
            ratio(sep.weighted_mass, sep.weighted_total),
            ratio(phi, sep.weighted_total)
        );
        let better = match q_root {
            None => true,
            Some((_, incumbent)) => response.outcome.weighted_mass > incumbent,
        };
        if better {
            q_root = Some((tile, response.outcome.weighted_mass));
        }
    }
    let (chosen, q_mass) = q_root.expect("a legal root action");
    let _ = writeln!(
        out,
        "exact mixture response Q(nu) = {} — selected action {chosen:?}",
        ratio(q_mass, augmented)
    );
    // One observed line: focal plays the frozen policy; hidden states
    // report merged branch masses, the typed-vs-merged census, and the
    // per-seat posterior marginals after observing the heaviest branch.
    let _ = writeln!(
        out,
        "observed line (focal = lowest-first, heaviest branch observed):"
    );
    let mut line_model = model.focal_play(choice_at(
        position,
        &Pub::start(position),
        viewer_hand,
        &focal,
    ));
    let mut exec = Pub::start(position);
    exec.play(position, *line_model.history().last().expect("a play"));
    let mut typed_sum = 0usize;
    let mut merged_sum = 0usize;
    let mut observations = 0usize;
    while observations < 4 {
        let at_terminal = exec.history.len() == total_plays;
        if decided_success(position, viewer, exec.banked, at_terminal).is_some() {
            let _ = writeln!(out, "  (decided after {} plays)", exec.history.len());
            break;
        }
        if exec.seat() == viewer {
            let tile = choice_at(position, &exec, viewer_hand, &focal);
            line_model = line_model.focal_play(tile);
            exec.play(position, tile);
            continue;
        }
        let branches = line_model.branch_masses(&oracle);
        let (typed, merged) = line_model.typed_branch_census(&oracle);
        typed_sum += typed;
        merged_sum += merged;
        let mut branch_text = String::new();
        for (tile, mass) in &branches {
            let _ = write!(branch_text, "{tile:?}:{mass} ");
        }
        let _ = writeln!(
            out,
            "  seat {:?} branch masses {{ {}}} — typed rows {typed} vs merged {merged}",
            exec.seat(),
            branch_text
        );
        let heaviest = branches
            .iter()
            .max_by_key(|(t, m)| (*m, usize::MAX - t.index()))
            .expect("a branch")
            .0;
        line_model = line_model.observe(&oracle, heaviest);
        exec.play(position, heaviest);
        observations += 1;
        let marginals = line_model.seat_type_marginals(&oracle);
        let total_now = line_model.weighted_total(&oracle);
        let mut marg_text = String::new();
        for (seat, entries) in &marginals {
            let _ = write!(marg_text, "{seat:?}[");
            for (id, mass) in entries {
                let _ = write!(marg_text, "{}:{mass}/{total_now} ", id.short());
            }
            let _ = write!(marg_text, "] ");
        }
        let _ = writeln!(
            out,
            "  observed {heaviest:?} -> posterior type marginals {marg_text}"
        );
    }
    if merged_sum > 0 {
        let _ = writeln!(
            out,
            "aggregation census along the line: {typed_sum} typed rows vs \
             {merged_sum} merged public branches"
        );
    }
    // Parity verdict: the (ω,θ) enumeration re-derives V and Q.
    let pairs = all_pairs(&model, root);
    let enum_fixed = enum_walk(
        &model,
        position,
        viewer,
        viewer_hand,
        total_plays,
        &pairs,
        &Pub::start(position),
        Some(&focal),
    );
    let enum_best = enum_walk(
        &model,
        position,
        viewer,
        viewer_hand,
        total_plays,
        &pairs,
        &Pub::start(position),
        None,
    );
    assert_eq!(enum_fixed, fixed.weighted_mass, "enumeration parity: V");
    assert_eq!(enum_best, q_mass, "enumeration parity: Q");
    let _ = writeln!(
        out,
        "parity verdict: EXACT — (omega,theta) enumeration over {} pairs \
         reproduces V_nu and Q(nu) to equality",
        pairs.len()
    );
    let _ = writeln!(
        out,
        "wall {} us; declared memory {} bytes (formula: entries x {ENTRY_BYTES} B \
         + profiles x {PROFILE_BYTES} B — an accounting, not a measurement)",
        start.elapsed().as_micros(),
        declared_bytes(&model)
    );
    let _ = writeln!(out);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 3, "usage: modelbeliefreport report <out.txt>");
    assert_eq!(args[1], "report");
    let receipt_path = locate_verify_player().expect("the frozen receipt is present");
    let r = parse_file(&receipt_path).expect("the frozen receipt parses");
    let f0: Rc<BehaviorType> = Rc::new(BehaviorType::from_field(
        field_spec_level0(),
        PersistenceScope::PerHand,
    ));
    let f1: Rc<BehaviorType> = Rc::new(BehaviorType::from_field(
        field_spec_level1(),
        PersistenceScope::PerHand,
    ));
    let low: Rc<BehaviorType> = Rc::new(BehaviorType::declared(
        "carrier:lowest-first-v1",
        "none",
        TieRule::LowestTileIndex,
        PersistenceScope::PerHand,
        Rc::new(FixedPreference::lowest_first("mind:lowest-first")),
    ));
    let high: Rc<BehaviorType> = Rc::new(BehaviorType::declared(
        "carrier:highest-first-v1",
        "none",
        TieRule::FirstInPreference,
        PersistenceScope::PerHand,
        Rc::new(FixedPreference::highest_first("mind:highest-first")),
    ));
    let mut out = String::new();
    let run_start = Instant::now();
    let _ = writeln!(
        out,
        "modelbeliefreport run 1 — the MB0 §75 report (EXPLORATORY tier)\n\
         epoch: F0 = {} (id {}), F1 = {} (id {}); prior (1/2,1/2) per hidden seat;\n\
         SupportOracle; focal = lowest-first frozen preference; six enumerable\n\
         verify_player receipt roots. Synthetic carrier mixture labeled below.\n\
         carriers: {} (id {}), {} (id {})\n",
        f0.construction(),
        f0.id().short(),
        f1.construction(),
        f1.id().short(),
        low.construction(),
        low.id().short(),
        high.construction(),
        high.id().short()
    );
    let mut registered_phi_zero = 0usize;
    let mut registered_phi_positive = 0usize;
    let mut carrier_phi_positive = 0usize;
    let mut q_differs_from_a_point_mass = 0usize;
    let oracle = SupportOracle;
    for (hand_id, trick_no, fiber) in ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        report_root(
            &mut out,
            "registered F0/F1 mixture",
            hand_id,
            trick_no,
            fiber,
            &root,
            &position,
            &f0,
            &f1,
        );
        // The §76 tallies for the registered mixture.
        let model = mixture_of(&root, &position, &f0, &f1);
        let led = position
            .trick_plays
            .first()
            .map(|d| position.decl.led_context(*d));
        let legal = legal_plays(position.decl, root.kernel().viewer_hand(), led);
        for tile in legal.iter() {
            let at_action = model.focal_play(tile);
            let mut stats = MixtureStats::default();
            let response = at_action.mixture_response(&oracle, &mut stats);
            let sep = at_action.separated_upper(&oracle);
            if response.outcome.weighted_mass == sep.weighted_mass {
                registered_phi_zero += 1;
            } else {
                registered_phi_positive += 1;
            }
            // Does Q_a(nu) differ from some point-mass response q_a(θ)?
            // Compare as exact values on the shared augmented total:
            // q_a(θ) as a value is per_profile_mass/per_profile_total.
            for (m, z) in sep
                .per_profile_mass
                .iter()
                .zip(sep.per_profile_total.iter())
            {
                // Q/W != m/z  <=>  Q·z != m·W (cross-multiplied).
                if response.outcome.weighted_mass * z != m * response.outcome.weighted_total {
                    q_differs_from_a_point_mass += 1;
                    break;
                }
            }
        }
    }
    let _ = writeln!(
        out,
        "---- SYNTHETIC carrier mixture (lowest-first / highest-first declared \
         carriers; labeled, never a solver rung) ----\n"
    );
    for (hand_id, trick_no, fiber) in ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        report_root(
            &mut out,
            "SYNTHETIC carrier mixture",
            hand_id,
            trick_no,
            fiber,
            &root,
            &position,
            &low,
            &high,
        );
        let model = mixture_of(&root, &position, &low, &high);
        let led = position
            .trick_plays
            .first()
            .map(|d| position.decl.led_context(*d));
        let legal = legal_plays(position.decl, root.kernel().viewer_hand(), led);
        for tile in legal.iter() {
            let at_action = model.focal_play(tile);
            let mut stats = MixtureStats::default();
            let response = at_action.mixture_response(&oracle, &mut stats);
            let sep = at_action.separated_upper(&oracle);
            if response.outcome.weighted_mass < sep.weighted_mass {
                carrier_phi_positive += 1;
            }
        }
    }
    // The §76 go/no-go criteria.
    let _ = writeln!(out, "== the §76 go/no-go criteria ==");
    let _ = writeln!(
        out,
        "1. point-mass parity exact: YES — G2 (F0 on all six roots; F1 on the raw \
         authority's entire terminating domain, with the sigma1 boundary pinned \
         and the F1 delta endpoint enumeration-anchored in G8)."
    );
    let _ = writeln!(
        out,
        "2. persistent posterior closure exact: YES — G1/G3 (Theorem 12.1 \
         mechanical; branch conservation MB-I6; the 1/2-vs-1/4 specimen)."
    );
    let _ = writeln!(
        out,
        "3. mixture response differs nontrivially from a point-mass response: {} — \
         {q_differs_from_a_point_mass} root actions on the registered corpus have \
         Q_a(nu) != q_a(theta) for some type in support.",
        if q_differs_from_a_point_mass > 0 {
            "YES"
        } else {
            "NO"
        }
    );
    let _ = writeln!(
        out,
        "4. point-mass upper sometimes strict, finite, interpretable: {} — \
         registered mixture: {registered_phi_zero} zero / {registered_phi_positive} \
         strict root actions; SYNTHETIC carrier mixture: {carrier_phi_positive} \
         strict root actions (every price finite and exact by construction).",
        if registered_phi_positive + carrier_phi_positive > 0 {
            "YES"
        } else {
            "NO"
        }
    );
    let _ = writeln!(
        out,
        "5. type dimension small after public-action aggregation: YES — at most 8 \
         profiles live anywhere, and every hidden branch table is by public \
         action (the typed-vs-merged census above; merged branches never exceed \
         the acting seat's distinct tiles)."
    );
    let _ = writeln!(
        out,
        "\ntotal wall {} us (single-threaded; sigma0/sigma1 field caches shared \
         within this run)",
        run_start.elapsed().as_micros()
    );
    // The C2 house rule: the measured figure lives beside the declared
    // accounting and is never merged with it.
    let rss_kb = std::process::Command::new("/bin/ps")
        .args(["-o", "rss=", "-p", &std::process::id().to_string()])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unavailable".to_string());
    let _ = writeln!(
        out,
        "measured resident set at exit: {rss_kb} KB (/bin/ps -o rss= — a \
         measurement, kept strictly apart from the declared accountings above)"
    );
    std::fs::write(&args[2], &out).expect("the report writes");
    print!("{out}");
}
