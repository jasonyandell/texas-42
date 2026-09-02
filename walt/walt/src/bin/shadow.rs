//! EXPLORATORY SHADOW INSTRUMENT (§22 step 7 of the calculated-evidence
//! build program, `walt/math/calculated_evidence_v0.1.md`; rulings
//! CE-A1..A8) — sits below every evidentiary tier and is cited by nothing
//! above it. Estimates, never receipts; not a P-A21 statement.
//!
//! Runs the new decision controller BESIDE the existing level-1 player and
//! changes nothing live: hands are driven exactly in the playout shape (the
//! focal seat plays the library's `level1_evaluate` — the live player's one
//! authority — and the other seats play the banked-correct level-0 policy
//! on their true hands), and at every focal decision with more than one
//! legal tile the controller ALSO evaluates one frozen level-1 continuation
//! policy per legal root action ([`ActionRule::PinnedThenLevel1`]) under a
//! run-scoped risk plan (§6: the run is the hand, δ_d = δ_run/(d(d+1))).
//! The live player's choice remains the played line; agreement or
//! disagreement with the controller is recorded, never acted on (§20.16).
//!
//! Phase-1 fence (§18): the inner minds stay sampled with declared
//! schedules; every record carries the complete freeze-tuple identity, so
//! no result can be quoted without its inner-approximation visibility.
//!
//! Two modes:
//!   `shadow receipt <out.jsonl> [knobs]` — the frozen `verify_player`
//!       receipt hands (deterministic deal anchor; the bidder seat is
//!       driven by the live player, NOT the receipt's recorded line).
//!   `shadow driven  <out.jsonl> [n_hands] [knobs]` — playout's scenario
//!       (trump fives, P30 by T1, S1 the bidder with the receipt-hand-8
//!       tiles) over fresh deals from fixed seeds.
//!
//! Knobs (positional, after the mode's fixed args):
//!   n_outer_live n0_live n_outer_frozen n0_frozen world_cap exact_cap
//!
//! No floats anywhere; wall time is integer microseconds.

use std::io::Write as _;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use walt::rules::replay::deal;
use walt::rules::rules::{legal_plays, Trick};
use walt::rules::{Context, ContextSet, Decl, Domino, DominoSet, Pip, Seat, Team};
use walt::solver::adaptive::{driven_root, DrivenState, PublicRecord, RootPosition, SlicePolicy};
use walt::solver::controller::{
    evaluate_set, exact_frozen_set, CandidateSet, EscalationConfig, RiskPlan, SetEvaluation,
    SetResult, SetSpec,
};
use walt::solver::evidence::{decision_delta, ScopedDelta};
use walt::solver::policy::{
    continuation_frame, t1_frame_bid, ActionRule, DecisionMode, FreezeTuple, FrozenPolicy,
    InnerSchedule, Level0Field, TieRule, NO_DEADLINE_SECS,
};
use walt::solver::{
    arena_decl_id, best_of, level1_evaluate, mask_bits, mask_of, mix, record_hash,
    sample_open_belief, set_of, SplitMix64,
};

/// Frozen shadow seed (a distinct stream from every other bin's constant).
const SHADOW_SEED: u64 = 0xC90F_DAA2_2168_C234;

// ---------------------------------------------------------------------------
// Configuration.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct Config {
    /// The live player's declared outer/inner sample sizes (the playout
    /// defaults).
    n_outer_live: usize,
    n0_live: usize,
    /// The frozen continuation policies' declared schedule (identity
    /// fields of every candidate's FreezeTuple, CE-A5).
    n_outer_frozen: u64,
    n0_frozen: u64,
    /// Controller resource cap in raw worlds (Unresolved, never a
    /// settlement rule).
    world_cap: u64,
    /// Declared routing: fibers at or below this run the exact frozen-set
    /// endpoint directly (always sound; spends no risk, §6.1).
    exact_cap: u128,
}

/// δ_run = 1/100 for every hand (the run scope of §6).
fn delta_run() -> BigRational {
    BigRational::new(BigInt::from(1), BigInt::from(100))
}

// ---------------------------------------------------------------------------
// JSON helpers (hand-rolled, like playout.rs — no serde in the workspace).
// ---------------------------------------------------------------------------

fn tile_json(d: Domino) -> String {
    format!("[{},{}]", d.hi().value(), d.lo().value())
}

fn tiles_json(ds: &[Domino]) -> String {
    let parts: Vec<String> = ds.iter().map(|d| tile_json(*d)).collect();
    format!("[{}]", parts.join(","))
}

fn set_json(s: DominoSet) -> String {
    let tiles: Vec<Domino> = s.iter().collect();
    tiles_json(&tiles)
}

fn rational_json(v: &BigRational) -> String {
    format!("\"{}/{}\"", v.numer(), v.denom())
}

fn opt_usize_json(v: Option<usize>) -> String {
    v.map_or("null".to_string(), |k| k.to_string())
}

fn micros(since: Instant) -> u64 {
    u64::try_from(since.elapsed().as_micros()).expect("a decision fits in u64 microseconds")
}

// ---------------------------------------------------------------------------
// One hand.
// ---------------------------------------------------------------------------

struct HandSpec {
    mode: &'static str,
    index: usize,
    decl: Decl,
    bid: u32,
    declaring_team: Team,
    bidder: Seat,
    deal: [DominoSet; 4],
}

fn shared_tuple(spec: &HandSpec, cfg: Config, pinned: Domino) -> FreezeTuple {
    FreezeTuple {
        solver_source: "walt-level1-continuation-v1 (solver::level1_evaluate; \
                        saturation-tie refinement 4x per round capped at 16x)"
            .to_string(),
        decl: spec.decl,
        bid: spec.bid,
        declaring_team: spec.declaring_team,
        field_model: "level0".to_string(),
        field_level: 0,
        inner_schedule: InnerSchedule::Declared(vec![cfg.n_outer_frozen, cfg.n0_frozen]),
        discovery_stream: "policy-discovery-splitmix64-counter-v1".to_string(),
        discovery_seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        practical_equivalence: None,
        policy_library: "level1-continuation-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
        action_rule: ActionRule::PinnedThenLevel1 { pinned },
    }
}

/// The shadow half of one focal decision: candidates, risk plan, route,
/// controller run, record assembly.
#[allow(clippy::too_many_arguments)]
fn shadow_decision(
    spec: &HandSpec,
    cfg: Config,
    state: &DrivenState<'_>,
    d: u64,
    legal_tiles: &[Domino],
    live_choice: Domino,
    live_opts: &[(u8, BigRational)],
    live_us: u64,
    trick: usize,
    ply: usize,
) -> String {
    let (root, position) = driven_root(state).expect("a driven decision has a lawful kernel");
    let candidates_owned: Vec<FrozenPolicy> = legal_tiles
        .iter()
        .map(|t| FrozenPolicy::new(shared_tuple(spec, cfg, *t)))
        .collect();
    let candidates = CandidateSet::new(candidates_owned.iter().collect());
    let run_scope = format!("run:shadow-{}-h{}", spec.mode, spec.index);
    let dec_scope = format!("decision:shadow-{}-h{}-d{}", spec.mode, spec.index, d);
    let dec_delta = decision_delta(d, &delta_run());
    let plan = RiskPlan::strict(ScopedDelta::new(dec_scope, dec_delta.clone()))
        .under_run(ScopedDelta::new(run_scope, delta_run()), d);
    let field = Level0Field::new(cfg.n0_frozen as usize);
    let set_spec = SetSpec {
        root: &root,
        position: &position,
        candidates: &candidates,
        field: &field,
        plan,
        world_cap: cfg.world_cap,
        batch: 8,
        escalation: Some(EscalationConfig {
            cost_sample: 1,
            cost_enumerate: 1,
            check_every: 8,
        }),
    };
    let started = Instant::now();
    let (evaluation, route): (SetEvaluation, &str) = if root.count() <= cfg.exact_cap {
        (exact_frozen_set(&set_spec), "preroute")
    } else {
        let evaluation = evaluate_set(&set_spec);
        let route = if evaluation.escalation.is_some() {
            "escalated"
        } else {
            "sampled"
        };
        (evaluation, route)
    };
    let shadow_us = micros(started);

    // Winner (by candidate index) where one exists; survivors otherwise.
    #[derive(Default)]
    struct Outcome {
        winner: Option<usize>,
        survivors: Option<Vec<usize>>,
        settled_at: Option<u64>,
        wins: Option<Vec<u128>>,
    }
    let outcome = match &evaluation.result {
        SetResult::ExactFrozenSet { winner, wins, .. } => Outcome {
            winner: *winner,
            wins: Some(wins.clone()),
            ..Outcome::default()
        },
        SetResult::DeltaSettled {
            winner, settled_at, ..
        } => Outcome {
            winner: Some(*winner),
            settled_at: Some(*settled_at),
            ..Outcome::default()
        },
        SetResult::EpsilonEquivalent {
            survivors,
            settled_at,
            ..
        } => Outcome {
            survivors: Some(survivors.clone()),
            settled_at: Some(*settled_at),
            ..Outcome::default()
        },
        SetResult::Unresolved { survivors, .. } => Outcome {
            survivors: Some(survivors.clone()),
            ..Outcome::default()
        },
    };
    let Outcome {
        winner,
        survivors,
        settled_at,
        wins,
    } = outcome;
    let winner_tile = winner.map(|k| legal_tiles[k]);
    let live_index = legal_tiles
        .iter()
        .position(|t| *t == live_choice)
        .expect("the live choice is legal");
    let agreement = winner_tile.map(|t| t == live_choice);
    let live_in_survivors = survivors.as_ref().map(|s| s.contains(&live_index));

    let opts_json: Vec<String> = live_opts
        .iter()
        .map(|(t, v)| {
            format!(
                "{{\"t\":{},\"v\":{}}}",
                tile_json(Domino::from_index(usize::from(*t)).expect("tile")),
                rational_json(v)
            )
        })
        .collect();
    let edges_json: Vec<String> = evaluation
        .edges
        .iter()
        .map(|e| {
            format!(
                "{{\"from\":{},\"to\":{},\"at\":{},\"a\":{},\"b\":{}}}",
                e.from, e.to, e.at, e.a, e.b
            )
        })
        .collect();
    let elim_json: Vec<String> = evaluation
        .eliminations
        .iter()
        .map(|e| {
            format!(
                "{{\"candidate\":{},\"by\":{},\"at\":{}}}",
                e.candidate, e.by, e.at
            )
        })
        .collect();
    let pairs_json: Vec<String> = evaluation
        .pair_counts
        .iter()
        .map(|p| {
            format!(
                "{{\"i\":{},\"j\":{},\"a\":{},\"b\":{},\"n\":{}}}",
                p.i, p.j, p.a, p.b, p.n
            )
        })
        .collect();
    let escalation_json = evaluation.escalation.as_ref().map_or_else(
        || "null".to_string(),
        |r| {
            format!(
                "{{\"switched_at\":{},\"reused\":{},\"fresh\":{}}}",
                r.switched_at, r.reused_worlds, r.fresh_worlds
            )
        },
    );
    let refinements_json = match &evaluation.result {
        SetResult::Unresolved { refinements, .. } => {
            let rows: Vec<String> = refinements
                .iter()
                .map(|r| {
                    let opt = |v: &Option<BigRational>| {
                        v.as_ref().map_or("null".to_string(), rational_json)
                    };
                    format!(
                        "{{\"i\":{},\"j\":{},\"n\":{},\"a\":{},\"b\":{},\"n0\":{},\
                         \"q_hat\":{},\"tau_hat\":{},\"g_hat\":{},\
                         \"e_plus\":{},\"e_minus\":{},\"threshold\":{},\
                         \"r_debt_plus\":{},\"r_debt_minus\":{},\
                         \"h_plus_min\":{},\"h_minus_min\":{},\
                         \"n_hat_plus\":{},\"n_hat_minus\":{},\
                         \"c_exact\":{},\"c_sample_forecast\":{}}}",
                        r.i,
                        r.j,
                        r.n,
                        r.a,
                        r.b,
                        r.n0,
                        opt(&r.q_hat),
                        opt(&r.tau_hat),
                        opt(&r.g_hat),
                        rational_json(&r.e_plus),
                        rational_json(&r.e_minus),
                        rational_json(&r.threshold),
                        rational_json(&r.r_debt_plus),
                        rational_json(&r.r_debt_minus),
                        r.h_plus_min,
                        r.h_minus_min,
                        opt(&r.n_hat_plus),
                        opt(&r.n_hat_minus),
                        rational_json(&r.c_exact),
                        opt(&r.c_sample_forecast),
                    )
                })
                .collect();
            format!("[{}]", rows.join(","))
        }
        _ => "null".to_string(),
    };
    let wins_json = wins.map_or("null".to_string(), |w| {
        let parts: Vec<String> = w.iter().map(|x| format!("\"{x}\"")).collect();
        format!("[{}]", parts.join(","))
    });
    let candidates_json: Vec<String> = legal_tiles
        .iter()
        .zip(&candidates_owned)
        .map(|(t, p)| {
            format!(
                "{{\"tile\":{},\"policy\":\"{}\"}}",
                tile_json(*t),
                p.policy_id()
            )
        })
        .collect();
    let survivors_json = survivors.map_or("null".to_string(), |s| {
        let parts: Vec<String> = s.iter().map(usize::to_string).collect();
        format!("[{}]", parts.join(","))
    });
    let ledger = evaluation.result.ledger().to_string();

    format!(
        "{{\"kind\":\"decision\",\"mode\":\"{mode}\",\"hand\":{hand},\"trick\":{trick},\
         \"ply\":{ply},\"d\":{d},\"seat\":{seat},\"fiber\":\"{fiber}\",\"m\":{m},\
         \"legal\":{legal},\
         \"live\":{{\"tile\":{live_tile},\"micros\":{live_us},\"opts\":[{opts}]}},\
         \"shadow\":{{\"micros\":{shadow_us},\"tag\":\"{tag}\",\"route\":\"{route}\",\
         \"consumed\":{consumed},\"winner\":{winner},\"winner_tile\":{winner_tile},\
         \"winner_policy\":{winner_policy},\"survivors\":{survivors},\
         \"settled_at\":{settled_at},\"wins\":{wins},\
         \"edges\":[{edges}],\"eliminations\":[{elims}],\"pair_counts\":[{pairs}],\
         \"escalation\":{escalation},\"refinements\":{refinements},\
         \"ledger\":\"{ledger}\"}},\
         \"agreement\":{agreement},\"live_in_survivors\":{live_in_survivors},\
         \"delta_run\":\"1/100\",\"delta_dec\":\"{dec_delta}\",\
         \"tuple\":{{\"solver_source\":\"walt-level1-continuation-v1\",\
         \"field_model\":\"level0\",\"field_level\":0,\
         \"inner_schedule\":[{n_outer_f},{n0_f}],\
         \"discovery_stream\":\"policy-discovery-splitmix64-counter-v1\",\
         \"discovery_seeds\":[],\"tie_rule\":\"lowest-tile-index\",\
         \"mode\":\"heuristic\",\"library\":\"level1-continuation-library-v1\",\
         \"eval_field\":\"{eval_field}\",\"candidates\":[{cands}]}}}}",
        mode = spec.mode,
        hand = spec.index,
        trick = trick,
        ply = ply,
        d = d,
        seat = state.leader.plus(state.trick_plays.len()).index(),
        fiber = root.count(),
        m = legal_tiles.len(),
        legal = tiles_json(legal_tiles),
        live_tile = tile_json(live_choice),
        live_us = live_us,
        opts = opts_json.join(","),
        shadow_us = shadow_us,
        tag = evaluation.result.tag(),
        route = route,
        consumed = evaluation.consumed,
        winner = opt_usize_json(winner),
        winner_tile = winner_tile.map_or("null".to_string(), tile_json),
        winner_policy = winner.map_or("null".to_string(), |k| format!(
            "\"{}\"",
            candidates_owned[k].policy_id()
        )),
        survivors = survivors_json,
        settled_at = settled_at.map_or("null".to_string(), |x| x.to_string()),
        wins = wins_json,
        edges = edges_json.join(","),
        elims = elim_json.join(","),
        pairs = pairs_json.join(","),
        escalation = escalation_json,
        refinements = refinements_json,
        ledger = ledger,
        agreement = agreement.map_or("null".to_string(), |x| x.to_string()),
        live_in_survivors = live_in_survivors.map_or("null".to_string(), |x| x.to_string()),
        dec_delta = dec_delta,
        n_outer_f = cfg.n_outer_frozen,
        n0_f = cfg.n0_frozen,
        eval_field = field.id(),
        cands = candidates_json.join(","),
    )
}

/// Drive one hand: the live player's choices are the played line; each
/// focal multi-option decision is also shadowed. Returns the JSONL lines.
fn drive_hand(spec: &HandSpec, cfg: Config) -> Vec<String> {
    let decl = spec.decl;
    let focal = spec.bidder;
    assert_eq!(
        focal.team(),
        spec.declaring_team,
        "the bidder's team declares"
    );
    let bid_param = t1_frame_bid(spec.bid, spec.declaring_team);
    let field_live = Level0Field::new(cfg.n0_live);
    let mut hands = spec.deal;
    let mut prior_played = DominoSet::EMPTY;
    let mut trick_plays: Vec<Domino> = Vec::new();
    let mut leader = spec.bidder;
    let mut banked = [0u32; 2];
    let mut voids = [ContextSet::EMPTY; 4];
    let mut d = 0u64;
    let mut line: Vec<(usize, Domino)> = Vec::new();
    let mut records: Vec<String> = Vec::new();
    for trick_no in 1..=7usize {
        for ply in 0..4usize {
            let seat = leader.plus(trick_plays.len());
            let hand = hands[seat.index()];
            let led: Option<Context> = trick_plays.first().map(|t| decl.led_context(*t));
            let legal = legal_plays(decl, hand, led);
            assert!(!legal.is_empty(), "a seat to move holds a legal tile");
            let position = RootPosition {
                decl,
                bid: spec.bid,
                declaring_team: spec.declaring_team,
                leader,
                banked,
                trick_plays: trick_plays.clone(),
                prior_played,
                voids,
            };
            let choice: Domino = if legal.len() == 1 {
                legal.iter().next().expect("one legal tile")
            } else if seat == focal {
                // The LIVE decision: the library's level-1 player, seeded
                // per decision from (constant, own hand, record hash) —
                // the bridge's information-consistent pattern.
                let frame = continuation_frame(decl, &position, &[]);
                let mut rng =
                    SplitMix64(SHADOW_SEED ^ mix(u64::from(hand.bits())) ^ record_hash(&frame.key));
                let started = Instant::now();
                let opts = level1_evaluate(
                    decl,
                    bid_param,
                    seat,
                    mask_of(hand),
                    mask_of(legal),
                    &frame.key,
                    frame.sizes(),
                    frame.voids,
                    frame.trick_start_played,
                    frame.boundary_hand_size,
                    cfg.n_outer_live,
                    cfg.n0_live,
                    NO_DEADLINE_SECS,
                    &mut rng,
                )
                .unwrap_or_else(|refusal| {
                    panic!("the live evaluation has no answer at this state: {refusal}")
                });
                let live_us = micros(started);
                let live_choice =
                    Domino::from_index(usize::from(best_of(&opts, seat.team() == Team::T1)))
                        .expect("tile < 28");
                d += 1;
                let legal_tiles: Vec<Domino> = mask_bits(mask_of(legal))
                    .into_iter()
                    .map(|i| Domino::from_index(usize::from(i)).expect("tile < 28"))
                    .collect();
                let state = DrivenState {
                    decl,
                    bid: spec.bid,
                    declaring_team: spec.declaring_team,
                    viewer_hand: hand,
                    leader,
                    trick_plays: &trick_plays,
                    banked,
                    prior_played,
                    voids,
                };
                records.push(shadow_decision(
                    spec,
                    cfg,
                    &state,
                    d,
                    &legal_tiles,
                    live_choice,
                    &opts,
                    live_us,
                    trick_no,
                    ply,
                ));
                live_choice
            } else {
                // Field seat: the banked-correct level-0 policy on its
                // true hand, through the library's one authority.
                let record = PublicRecord {
                    leader,
                    trick_plays: &trick_plays,
                    banked,
                    root: &position,
                    history: &[],
                };
                field_live.choose(decl, hand, legal, &record)
            };
            assert!(legal.contains(choice), "the chosen tile is legal");
            if let Some(led) = led {
                if !decl.follows(choice, led) {
                    voids[seat.index()].insert(led);
                }
            }
            assert!(
                hands[seat.index()].remove(choice),
                "the chosen tile is held"
            );
            trick_plays.push(choice);
            line.push((seat.index(), choice));
        }
        let doms: [Domino; 4] = core::array::from_fn(|i| trick_plays[i]);
        let trick = Trick::new(leader, doms).expect("four distinct tiles");
        let winner = trick.winner(decl);
        banked[winner.team().index()] += trick.points();
        for t in doms {
            prior_played.insert(t);
        }
        leader = winner;
        trick_plays.clear();
        let _ = trick_no;
    }
    assert_eq!(banked[0] + banked[1], 42, "all points banked");
    let made = banked[spec.declaring_team.index()] >= spec.bid;
    let line_json: Vec<String> = line
        .iter()
        .map(|(s, t)| format!("[{},{}]", s, tile_json(*t)))
        .collect();
    records.push(format!(
        "{{\"kind\":\"hand\",\"mode\":\"{}\",\"hand\":{},\"decl\":{},\"bid\":{},\
         \"declaring_team\":{},\"bidder\":{},\"deal\":[{},{},{},{}],\"made\":{},\
         \"banked\":[{},{}],\"decisions\":{},\"line\":[{}]}}",
        spec.mode,
        spec.index,
        arena_decl_id(spec.decl),
        spec.bid,
        spec.declaring_team.index(),
        spec.bidder.index(),
        set_json(spec.deal[0]),
        set_json(spec.deal[1]),
        set_json(spec.deal[2]),
        set_json(spec.deal[3]),
        made,
        banked[0],
        banked[1],
        d,
        line_json.join(","),
    ));
    records
}

// ---------------------------------------------------------------------------
// Modes.
// ---------------------------------------------------------------------------

/// The playout scenario's frozen S1 hand (receipt hand 8's S1 tiles).
fn s1_scenario_hand() -> DominoSet {
    let p = |v: u8| Pip::new(v).expect("pip");
    [
        Domino::new(p(5), p(2)),
        Domino::new(p(5), p(4)),
        Domino::new(p(1), p(1)),
        Domino::new(p(2), p(1)),
        Domino::new(p(3), p(1)),
        Domino::new(p(3), p(3)),
        Domino::new(p(5), p(5)),
    ]
    .into_iter()
    .collect()
}

fn receipt_specs() -> Vec<HandSpec> {
    let path = walt::rules::receipt::locate_verify_player()
        .expect("rob/receipts/verify_player.txt above the workspace");
    let receipt = walt::rules::receipt::parse_file(&path).expect("the receipt parses");
    receipt
        .hands
        .iter()
        .map(|hand| HandSpec {
            mode: "receipt",
            index: hand.id,
            decl: hand.decl,
            bid: hand.bid_points,
            declaring_team: hand.declaring_team,
            bidder: hand.bidder,
            deal: deal(hand).expect("the receipt deal replays"),
        })
        .collect()
}

fn driven_specs(n_hands: usize) -> Vec<HandSpec> {
    let decl = Decl::PipTrump(Pip::new(5).expect("pip 5 exists"));
    let s1 = s1_scenario_hand();
    (0..n_hands)
        .map(|g| {
            let mut rng = SplitMix64(SHADOW_SEED ^ mix(g as u64));
            let drawn = sample_open_belief(1, mask_of(s1), 0, [7, 7, 7, 7], 1, &mut rng)
                .pop()
                .expect("one deal");
            let mut deal: [DominoSet; 4] = core::array::from_fn(|s| set_of(drawn[s]));
            deal[1] = s1;
            HandSpec {
                mode: "driven",
                index: g,
                decl,
                bid: 30,
                declaring_team: Team::T1,
                bidder: Seat::S1,
                deal,
            }
        })
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("receipt");
    let out_path = args
        .get(2)
        .cloned()
        .unwrap_or_else(|| format!("shadow-{mode}.jsonl"));
    let mut knobs = args.iter().skip(3);
    let mut n_hands = 20usize;
    if mode == "driven" {
        if let Some(v) = knobs.next() {
            n_hands = v.parse().expect("hand count");
        }
    }
    let mut knob = |default: u64| -> u64 {
        knobs
            .next()
            .map(|v| v.parse().expect("an integer knob"))
            .unwrap_or(default)
    };
    let cfg = Config {
        n_outer_live: knob(200) as usize,
        n0_live: knob(8) as usize,
        n_outer_frozen: knob(8),
        n0_frozen: knob(2),
        world_cap: knob(512),
        exact_cap: u128::from(knob(2000)),
    };
    let specs = match mode {
        "receipt" => receipt_specs(),
        "driven" => driven_specs(n_hands),
        other => panic!("unknown mode {other:?}; expected receipt|driven"),
    };
    eprintln!(
        "shadow: {} hands ({mode}); live {}x{}, frozen {}x{}, world_cap {}, exact_cap {}",
        specs.len(),
        cfg.n_outer_live,
        cfg.n0_live,
        cfg.n_outer_frozen,
        cfg.n0_frozen,
        cfg.world_cap,
        cfg.exact_cap
    );
    let run = |spec: &HandSpec| -> Vec<String> {
        let started = Instant::now();
        let records = drive_hand(spec, cfg);
        eprintln!(
            "shadow: {} hand {} done ({} records, {} us)",
            spec.mode,
            spec.index,
            records.len(),
            micros(started)
        );
        records
    };
    #[cfg(feature = "parallel")]
    let per_hand: Vec<Vec<String>> = specs.par_iter().map(run).collect();
    #[cfg(not(feature = "parallel"))]
    let per_hand: Vec<Vec<String>> = specs.iter().map(run).collect();
    let mut out = std::fs::File::create(&out_path).expect("the output file opens");
    for records in per_hand {
        for record in records {
            writeln!(out, "{record}").expect("the output file writes");
        }
    }
    eprintln!("shadow: wrote {out_path}");
}
