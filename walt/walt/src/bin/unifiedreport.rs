//! EXPLORATORY UNIFIED-PLAYER TRANSCRIPT (`solver::unified`, slice UP0)
//! — sits below every evidentiary tier and is cited by nothing above it.
//!
//! THIS IS A TRANSCRIPT, NOT AN EVALUATION. It records what the unified
//! player did at every decision of a walked hand: which tier answered,
//! which of the two recursions that tier stands in, the field
//! consultations it spent, every typed refusal it fell through, whether
//! the carried posterior was consulted and what it said, and the wall
//! time. It makes NO play-strength claim, contains no comparison to the
//! existing player, and settles nothing about defaults — arena work is
//! Jason's word alone.
//!
//! DECLARED EPOCH. The σ0 field is `Level0 { n0 = 2 }` under
//! `SupportOracle` — the same declared mind U0 and MB1 measured against.
//! The type library is MB0's registered pair, F₀ = σ0 = `Level0 { n0 = 2 }`
//! and F₁ = `Level1 { n_outer = 2, n0 = 2 }`, at equal integer prior
//! weight per hidden seat (ν = (1/2, 1/2), denominator 8 over three
//! seats). The corpus is MB0's six enumerable roots plus one trick-4
//! root; the true world at each root is the frozen `verify_player`
//! receipt's own deal, so the walk starts from a real position and every
//! seat holds the hand it actually held.
//!
//! DECLARED BUDGET LADDER. Two rungs, both in the constants below. Every
//! cap is either a structural fiber predicate (free to check, checked
//! before any spend) or an enforced ceiling in FIELD CONSULTATIONS — the
//! unit the ledger measures at the dispatch itself. Never wall-clock.
//! Wall time is reported and is the only number here that should be read
//! as approximate.
//!
//! Modes:
//!   `unifiedreport report <out.txt>`  — the declared run
//!   `unifiedreport walk <hand> <trick> <rung>` — one root, one rung, to
//!       stdout; the scout that chose the caps
//!
//! No floats anywhere; wall time is integer microseconds; values are
//! exact rationals printed beside integer permille.

use std::fmt::Write as _;
use std::io::Write as _;
use std::rc::Rc;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::replay::state_before_trick;
use walt::rules::{legal_plays, Context, ContextSet, Domino, DominoSet, Seat, Trick};
use walt::solver::adaptive::DrivenState;
use walt::solver::factor_belief::SupportOracle;
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::model_belief::{BehaviorType, PersistenceScope};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::unified::{Evidence, MoveBudget, Recursion, Tier, TypeLibrary, UnifiedPlayer};

/// MB0's six enumerable roots, plus the smallest trick-4 receipt root —
/// the stratum U0 measured a positive God gap at and MB1 priced a strict
/// model-fusion price at. `(hand, trick)`.
const CORPUS: [(usize, usize); 7] = [(12, 6), (10, 6), (5, 6), (4, 6), (8, 5), (3, 5), (8, 4)];

/// Rung one — LEAN. The exact world-space recursion is afforded only at
/// the smallest fibers; the model-space recursion is not afforded at all
/// (a zero read ceiling and a zero fiber cap), so its refusals are the
/// declared budget's own and nothing else.
const LEAN_ENUMERATION_FIBER_CAP: u128 = 32;
const LEAN_MIXTURE_FIBER_CAP: u128 = 0;
const LEAN_MIXTURE_READ_CAP: u64 = 0;

/// Rung two — AMPLE. The exact world-space recursion is afforded on the
/// whole corpus (largest fiber 1,200). The model-space recursion is
/// afforded only where MB1 measured it in seconds rather than minutes:
/// its fiber cap sits above trick 5 (200) and below trick 4 (1,200), so
/// the trick-4 root's model tier refuses STRUCTURALLY and the transcript
/// shows the affordability wall in the same table as the answers.
const AMPLE_ENUMERATION_FIBER_CAP: u128 = 40_000;
const AMPLE_MIXTURE_FIBER_CAP: u128 = 256;
const AMPLE_MIXTURE_READ_CAP: u64 = 4_000_000;

/// The certified-regret acceptance of tier (d) on both rungs: ε = 1/4,
/// the same target §65 set for the opening root.
fn regret_acceptance() -> BigRational {
    BigRational::new(BigInt::from(1), BigInt::from(4))
}

fn lean() -> MoveBudget {
    MoveBudget {
        label: "lean".to_string(),
        enumeration_fiber_cap: LEAN_ENUMERATION_FIBER_CAP,
        mixture_fiber_cap: LEAN_MIXTURE_FIBER_CAP,
        mixture_read_cap: LEAN_MIXTURE_READ_CAP,
        regret_acceptance: regret_acceptance(),
        join_reading: false,
    }
}

fn ample() -> MoveBudget {
    MoveBudget {
        label: "ample".to_string(),
        enumeration_fiber_cap: AMPLE_ENUMERATION_FIBER_CAP,
        mixture_fiber_cap: AMPLE_MIXTURE_FIBER_CAP,
        mixture_read_cap: AMPLE_MIXTURE_READ_CAP,
        regret_acceptance: regret_acceptance(),
        join_reading: true,
    }
}

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

fn registered_library() -> TypeLibrary {
    TypeLibrary::new(vec![
        (
            Rc::new(BehaviorType::from_field(
                field_spec_level0(),
                PersistenceScope::PerHand,
            )),
            1,
        ),
        (
            Rc::new(BehaviorType::from_field(
                field_spec_level1(),
                PersistenceScope::PerHand,
            )),
            1,
        ),
    ])
}

/// One decision's transcript row — every field a derived view of the
/// decision the player returned.
struct Row {
    trick: usize,
    ply: usize,
    seat: usize,
    action: Domino,
    tier: Tier,
    recursion: Recursion,
    authority: String,
    fiber: Option<u128>,
    value: Option<BigRational>,
    enumeration_reads: u64,
    mixture_reads: u64,
    field_reads: u64,
    refusals: Vec<String>,
    posterior_carried: bool,
    posterior_consulted: bool,
    live_profiles: usize,
    falsified: bool,
    join: Option<(BigRational, BigRational, bool, Domino, Domino, bool)>,
    wall_us: u128,
}

/// One walked hand: every decision from the root to the last tile.
struct Walk {
    rows: Vec<Row>,
    banked: [u32; 2],
    made: bool,
    /// Falsification events observed on the walk, as
    /// `(trick, ply, seat, observed, supported)`.
    falsifications: Vec<(usize, usize, usize, Domino, Vec<Domino>)>,
    wall_us: u128,
}

fn permille(v: &BigRational) -> i128 {
    let scaled = (v * BigRational::from_integer(BigInt::from(1000))).floor();
    i128::try_from(scaled.to_integer()).unwrap_or(-1)
}

fn exact(v: &BigRational) -> String {
    format!("{v} ({}‰)", permille(v))
}

/// Walk one receipt root to terminal with the unified player choosing
/// every seat's action under one declared budget rung.
fn walk_root(r: &Receipt, hand_id: usize, trick_no: usize, budget: &MoveBudget) -> Walk {
    let hand = &r.hands[hand_id];
    let (start_hands, start_leader) =
        state_before_trick(hand, trick_no).expect("a valid receipt trick");
    let decl = hand.decl;
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec_level0());
    let mut player = UnifiedPlayer::new(&oracle, &field, registered_library());

    // The public state at the root, replayed from the receipt so that
    // banked totals, the played mask and the observed voids are derived
    // rather than stored.
    let mut hands = start_hands;
    let mut leader = start_leader;
    let mut banked = [0u32; 2];
    let mut prior_played = DominoSet::EMPTY;
    let mut voids = [ContextSet::EMPTY; 4];
    for t in hand.tricks.iter().take(trick_no - 1) {
        let doms: [Domino; 4] = core::array::from_fn(|i| t.plays[i].1);
        let trick = Trick::new(t.plays[0].0, doms).expect("four distinct tiles");
        banked[trick.winner(decl).team().index()] += trick.points();
        for d in doms {
            prior_played.insert(d);
        }
        let led = decl.led_context(doms[0]);
        for (k, d) in doms.iter().enumerate() {
            if !decl.follows(*d, led) {
                voids[t.plays[0].0.plus(k).index()].insert(led);
            }
        }
    }

    let mut trick_plays: Vec<Domino> = Vec::new();
    let mut rows: Vec<Row> = Vec::new();
    let mut falsifications: Vec<(usize, usize, usize, Domino, Vec<Domino>)> = Vec::new();
    let started = Instant::now();
    for trick_index in trick_no..=7usize {
        for _ in 0..4usize {
            let seat = leader.plus(trick_plays.len());
            let state = DrivenState {
                decl,
                bid: hand.bid_points,
                declaring_team: hand.declaring_team,
                viewer_hand: hands[seat.index()],
                leader,
                trick_plays: &trick_plays,
                banked,
                prior_played,
                voids,
            };
            let t0 = Instant::now();
            let decision = player.decide(&state, budget);
            let wall_us = t0.elapsed().as_micros();
            let p = decision.provenance();
            let led: Option<Context> = trick_plays.first().map(|t| decl.led_context(*t));
            let legal = legal_plays(decl, hands[seat.index()], led);
            assert!(
                legal.contains(decision.action()),
                "the unified player chooses a legal tile"
            );
            let post = p.posterior();
            rows.push(Row {
                trick: trick_index,
                ply: trick_plays.len(),
                seat: seat.index(),
                action: decision.action(),
                tier: p.tier(),
                recursion: p.recursion(),
                authority: p.authority().to_string(),
                fiber: p.frame().fiber_mass,
                value: p.evidence().value(),
                enumeration_reads: p.spend().enumeration_reads,
                mixture_reads: p.spend().mixture_reads,
                field_reads: p.spend().field_reads,
                refusals: p.refusals().iter().map(|x| format!("{x:?}")).collect(),
                posterior_carried: post.carried,
                posterior_consulted: post.consulted,
                live_profiles: post.live_profiles,
                falsified: post.falsified.is_some(),
                join: post.join.as_ref().map(|j| {
                    (
                        j.mixture_value.clone(),
                        j.fixed_field_value.clone(),
                        j.value_moved,
                        j.mixture_action,
                        j.fixed_field_action,
                        j.argmax_flipped,
                    )
                }),
                wall_us,
            });

            // Advance every carried line past the play, then the world.
            let before: Vec<(usize, bool)> = (0..4)
                .map(|s| {
                    let seat = Seat::ALL[s];
                    (
                        s,
                        player.line(seat).is_some_and(|l| l.falsified().is_some()),
                    )
                })
                .collect();
            player.observe_play(&state, decision.action());
            for (s, was) in before {
                let seat = Seat::ALL[s];
                if was {
                    continue;
                }
                if let Some(line) = player.line(seat) {
                    if let Some(f) = line.falsified() {
                        falsifications.push((
                            trick_index,
                            trick_plays.len(),
                            f.seat,
                            f.observed,
                            f.supported.clone(),
                        ));
                    }
                }
            }

            if let Some(led) = led {
                if !decl.follows(decision.action(), led) {
                    voids[seat.index()].insert(led);
                }
            }
            assert!(
                hands[seat.index()].remove(decision.action()),
                "the chosen tile is held"
            );
            trick_plays.push(decision.action());
        }
        let doms: [Domino; 4] = core::array::from_fn(|i| trick_plays[i]);
        let trick = Trick::new(leader, doms).expect("four distinct tiles");
        let winner = trick.winner(decl);
        banked[winner.team().index()] += trick.points();
        for d in doms {
            prior_played.insert(d);
        }
        leader = winner;
        trick_plays.clear();
    }
    let made = banked[hand.declaring_team.index()] >= hand.bid_points;
    Walk {
        rows,
        banked,
        made,
        falsifications,
        wall_us: started.elapsed().as_micros(),
    }
}

fn print_walk(out: &mut String, label: &str, budget: &MoveBudget, walk: &Walk) {
    let _ = writeln!(
        out,
        "\n== {label} [{}] — {} decisions, banked {}/{}, declaring {} the contract, \
         wall {}us",
        budget.label,
        walk.rows.len(),
        walk.banked[0],
        walk.banked[1],
        if walk.made { "MADE" } else { "MISSED" },
        walk.wall_us
    );
    for row in &walk.rows {
        let _ = writeln!(
            out,
            "  t{} p{} s{} plays {} | tier ({}) {} [{} / {}] via {}",
            row.trick,
            row.ply,
            row.seat,
            row.action,
            row.tier.letter(),
            row.tier.label(),
            row.recursion.direction(),
            row.recursion.space(),
            row.authority
        );
        let _ = writeln!(
            out,
            "      fiber={} value={} | reads: enum={} mix={} field={} | wall={}us",
            match row.fiber {
                Some(z) => z.to_string(),
                None => "-".to_string(),
            },
            match &row.value {
                Some(v) => exact(v),
                None => "-".to_string(),
            },
            row.enumeration_reads,
            row.mixture_reads,
            row.field_reads,
            row.wall_us
        );
        let _ = writeln!(
            out,
            "      posterior: carried={} consulted={} live={} falsified={}",
            row.posterior_carried, row.posterior_consulted, row.live_profiles, row.falsified
        );
        if let Some((mv, fv, moved, ma, fa, flipped)) = &row.join {
            let _ = writeln!(
                out,
                "      JOIN: Q(nu)={} vs fixed-field {} -> value_moved={} | \
                 argmax model {} vs fixed {} -> flipped={}",
                exact(mv),
                exact(fv),
                moved,
                ma,
                fa,
                flipped
            );
        }
        if !row.refusals.is_empty() {
            let _ = writeln!(out, "      refusals: {}", row.refusals.join("; "));
        }
    }
    if walk.falsifications.is_empty() {
        let _ = writeln!(
            out,
            "  no line was falsified on this walk: every observed action stayed \
             inside the declared type library's support"
        );
    } else {
        for (trick, ply, seat, observed, supported) in &walk.falsifications {
            let names: Vec<String> = supported.iter().map(|d| format!("{d}")).collect();
            let _ = writeln!(
                out,
                "  LIBRARY FALSIFIED at t{trick} p{ply}: seat {seat} played {observed}, \
                 library supported {{{}}}",
                names.join(" ")
            );
        }
    }
}

/// The tier-occupancy census: where each recursion actually answered.
fn census(out: &mut String, walks: &[(String, String, Walk)]) {
    let _ = writeln!(
        out,
        "\n#### TIER OCCUPANCY BY TRICK — where each recursion answered ####\n"
    );
    let _ = writeln!(
        out,
        " rung  | trick | decisions | (a) dec | (b) exact | (c) mix | (d) regret | (e) field"
    );
    let _ = writeln!(
        out,
        "-------+-------+-----------+---------+-----------+---------+------------+----------"
    );
    let mut rungs: Vec<String> = Vec::new();
    for (rung, _, _) in walks {
        if !rungs.contains(rung) {
            rungs.push(rung.clone());
        }
    }
    for rung in &rungs {
        let mut tricks: Vec<usize> = Vec::new();
        for (r, _, w) in walks {
            if r != rung {
                continue;
            }
            for row in &w.rows {
                if !tricks.contains(&row.trick) {
                    tricks.push(row.trick);
                }
            }
        }
        tricks.sort_unstable();
        for trick in tricks {
            let mut counts = [0usize; 5];
            let mut total = 0usize;
            for (r, _, w) in walks {
                if r != rung {
                    continue;
                }
                for row in w.rows.iter().filter(|x| x.trick == trick) {
                    total += 1;
                    for (i, t) in Tier::ALL.iter().enumerate() {
                        if row.tier == *t {
                            counts[i] += 1;
                        }
                    }
                }
            }
            let _ = writeln!(
                out,
                " {rung:<5} |  t{trick}   |   {total:>5}   |  {:>5}  |   {:>5}   |  {:>5}  |   \
                 {:>5}    |  {:>5}",
                counts[0], counts[1], counts[2], counts[3], counts[4]
            );
        }
    }

    let _ = writeln!(out, "\n#### RECURSION OCCUPANCY ####\n");
    for rung in &rungs {
        let mut backward_world = 0usize;
        let mut backward_model = 0usize;
        let mut forward = 0usize;
        for (r, _, w) in walks {
            if r != rung {
                continue;
            }
            for row in &w.rows {
                match row.recursion {
                    Recursion::BackwardWorld => backward_world += 1,
                    Recursion::BackwardModel => backward_model += 1,
                    Recursion::Forward => forward += 1,
                }
            }
        }
        let _ = writeln!(
            out,
            "  {rung}: backward/world {backward_world}, backward/model {backward_model}, \
             forward/play {forward}"
        );
    }

    let _ = writeln!(out, "\n#### SPEND AND WALL, PER RUNG ####\n");
    for rung in &rungs {
        let mut enum_reads = 0u64;
        let mut mix_reads = 0u64;
        let mut field_reads = 0u64;
        let mut wall = 0u128;
        let mut decisions = 0usize;
        let mut worst = 0u128;
        for (r, _, w) in walks {
            if r != rung {
                continue;
            }
            wall += w.wall_us;
            for row in &w.rows {
                decisions += 1;
                enum_reads += row.enumeration_reads;
                mix_reads += row.mixture_reads;
                field_reads += row.field_reads;
                if row.wall_us > worst {
                    worst = row.wall_us;
                }
            }
        }
        let per_move = if decisions == 0 {
            0
        } else {
            wall / decisions as u128
        };
        let _ = writeln!(
            out,
            "  {rung}: {decisions} decisions, field consultations enum={enum_reads} \
             mix={mix_reads} fallback={field_reads}; wall {wall}us total, {per_move}us mean, \
             {worst}us worst move"
        );
    }

    let _ = writeln!(
        out,
        "\n#### THE JOIN — where both recursions priced the same state ####\n"
    );
    let mut joins = 0usize;
    let mut moved = 0usize;
    let mut flipped = 0usize;
    for (rung, label, w) in walks {
        for row in &w.rows {
            if let Some((mv, fv, m, ma, fa, f)) = &row.join {
                joins += 1;
                if *m {
                    moved += 1;
                }
                if *f {
                    flipped += 1;
                    let _ = writeln!(
                        out,
                        "  ARGMAX FLIP [{rung}] {label} t{} p{} s{}: model {} vs fixed-field {} \
                         | Q(nu)={} vs {}",
                        row.trick,
                        row.ply,
                        row.seat,
                        ma,
                        fa,
                        exact(mv),
                        exact(fv)
                    );
                } else if *m {
                    let _ = writeln!(
                        out,
                        "  VALUE MOVE [{rung}] {label} t{} p{} s{}: Q(nu)={} vs fixed-field {} \
                         (same argmax {})",
                        row.trick,
                        row.ply,
                        row.seat,
                        exact(mv),
                        exact(fv),
                        ma
                    );
                }
            }
        }
    }
    let _ = writeln!(
        out,
        "\n  {joins} join readings taken: {moved} moved the value, {flipped} flipped the argmax."
    );
    if joins > 0 && flipped == 0 {
        let _ = writeln!(
            out,
            "  The absence is censused and corpus-scoped: on THESE states, under THIS \
             declared library and prior, the posterior did not move the argmax. It is not a \
             statement that posteriors do not matter."
        );
    }

    let _ = writeln!(out, "\n#### REFUSAL CENSUS ####\n");
    let mut kinds: Vec<(String, usize)> = Vec::new();
    for (_, _, w) in walks {
        for row in &w.rows {
            for r in &row.refusals {
                let head = match r.find([' ', '(', '{']) {
                    Some(i) => r[..i].to_string(),
                    None => r.clone(),
                };
                match kinds.iter_mut().find(|(k, _)| *k == head) {
                    Some((_, n)) => *n += 1,
                    None => kinds.push((head, 1)),
                }
            }
        }
    }
    kinds.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    for (kind, n) in &kinds {
        let _ = writeln!(out, "  {n:>5}  {kind}");
    }
    if kinds.is_empty() {
        let _ = writeln!(out, "  none");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("");
    let r = parse_file(&locate_verify_player().expect("the receipt above the workspace"))
        .expect("the receipt parses");
    match mode {
        "walk" => {
            let hand_id: usize = args[2].parse().expect("a hand id");
            let trick_no: usize = args[3].parse().expect("a trick number");
            let rung = args.get(4).map(String::as_str).unwrap_or("ample");
            let budget = if rung == "lean" { lean() } else { ample() };
            let walk = walk_root(&r, hand_id, trick_no, &budget);
            let mut out = String::new();
            print_walk(&mut out, &format!("h{hand_id}-t{trick_no}"), &budget, &walk);
            print!("{out}");
        }
        "report" => {
            let path = args.get(2).expect("an output path").clone();
            let mut out = String::new();
            let flush = |s: &str| {
                let mut f = std::fs::File::create(&path).expect("the output file opens");
                f.write_all(s.as_bytes()).expect("the output file writes");
            };
            let _ = writeln!(
                out,
                "UNIFIED-PLAYER TRANSCRIPT (slice UP0) — EXPLORATORY\n\
                 \n\
                 A record of what the unified player DID, decision by decision. Not an \
                 evaluation: no play-strength claim is made anywhere in this file, and \
                 nothing here is compared to the existing player.\n\
                 \n\
                 declared field: level0-modeled-mind-v1 (Level0 n0=2) under SupportOracle\n\
                 declared type library: F0 = Level0 n0=2, F1 = Level1 n_outer=2 n0=2, \
                 equal integer prior weight per hidden seat (8 profiles over denominator 8)\n\
                 declared ladder:\n\
                 \x20 lean : enumeration_fiber_cap={LEAN_ENUMERATION_FIBER_CAP} \
                 mixture_fiber_cap={LEAN_MIXTURE_FIBER_CAP} \
                 mixture_read_cap={LEAN_MIXTURE_READ_CAP} join_reading=false\n\
                 \x20 ample: enumeration_fiber_cap={AMPLE_ENUMERATION_FIBER_CAP} \
                 mixture_fiber_cap={AMPLE_MIXTURE_FIBER_CAP} \
                 mixture_read_cap={AMPLE_MIXTURE_READ_CAP} join_reading=true\n\
                 \x20 certified-regret acceptance on both rungs: 1/4\n\
                 corpus: MB0's six enumerable roots plus h8-t4, walked to terminal with the \
                 unified player choosing EVERY seat's action\n"
            );
            flush(&out);
            let mut walks: Vec<(String, String, Walk)> = Vec::new();
            for budget in [lean(), ample()] {
                let _ = writeln!(
                    out,
                    "\n\n#### RUNG: {} ####################################################",
                    budget.label
                );
                for (hand_id, trick_no) in CORPUS {
                    let label = format!("h{hand_id}-t{trick_no}");
                    eprintln!("  {} {label} ...", budget.label);
                    let walk = walk_root(&r, hand_id, trick_no, &budget);
                    print_walk(&mut out, &label, &budget, &walk);
                    walks.push((budget.label.clone(), label, walk));
                    flush(&out);
                }
            }
            census(&mut out, &walks);
            let _ = writeln!(
                out,
                "\nEXPLORATORY — below every evidentiary tier; quotable only via gate \
                 receipts. This file is a transcript of decisions and their provenance. It \
                 makes no claim about how well the unified player plays, and none about how \
                 it compares to anything else."
            );
            flush(&out);
            println!("{out}");
        }
        _ => {
            eprintln!(
                "usage: unifiedreport report <out.txt> | unifiedreport walk <hand> <trick> \
                 [lean|ample]"
            );
            std::process::exit(2);
        }
    }
}

/// Kept beside the report so the evidence enum's shape is exercised by
/// the binary that prints it: the tier a decision claims is a derived
/// view of the evidence, never a stored label.
#[allow(dead_code)]
fn tier_of(evidence: &Evidence) -> Tier {
    evidence.tier()
}
