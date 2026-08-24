//! EXPLORATORY E0 CALIBRATION INSTRUMENT (§22 step 8; parent §19 V6) —
//! sits below every evidentiary tier and is cited by nothing above it.
//!
//! The corrected per-fixed-pair E0 calibration: for every unordered pair
//! of the step-7 flip fixtures' frozen candidate sets (exact fibers, so
//! the pair's true sampling-law coordinates are computable exactly via
//! the kernel), compare — PER PAIR, NEVER POOLED (V6) —
//!
//!   - the exact coordinates `(q, τ, g, H)` from full-fiber enumeration;
//!   - the initial evidence state (`E± = 1`, `R_debt = T`, `h±_min(0,0)`);
//!   - the §7 information rate and leading-order raw-world forecast, as
//!     exact rational interval bounds (series with rational tails);
//!   - the §8.4 exact forecast DP under the exact predictive law, at
//!     γ = 1/2 and γ = 9/10;
//!   - OBSERVED settlement indices of the anytime-valid pair evaluator
//!     (`solver::adaptive::evaluate_pair`) over replicate declared
//!     streams, with empirical `q̂/τ̂` and, for unresolved replicates,
//!     the same DP forecast refit from the observed counts (the per-pair
//!     grounding the controller's cost forecasts consume).
//!
//! Forecasts are forecasts; settlement is governed solely by the exact
//! evidence threshold. Usage:
//!   `e0cal <out.jsonl> [reps] [world_cap] [dp_h_max]`
//! Declared instrument constants: δ_pair = 1/200 per directed test scope
//! (so the m = 2 edge threshold is T = 400). No floats anywhere.

use std::io::Write as _;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use walt::rules::Domino;
use walt::solver::adaptive::{evaluate_pair, PairSpec, ResultKind, SlicePolicy};
use walt::solver::calibrate::{
    dp_settlement_forecast, exact_set_outcomes, information_rate_bounds,
    leading_order_forecast_bounds, pair_coordinates, reconstruct_flip, shadow_tuple, DpForecast,
    PairCoordinates, PredictiveLaw, FLIP_FIXTURES,
};
use walt::solver::evidence::{edge_threshold, h_plus_min, ScopedDelta};
use walt::solver::mix;
use walt::solver::policy::{FrozenPolicy, Level0Field};

/// Frozen instrument seed for replicate stream epochs (a declared
/// constant, distinct from every other bin's).
const E0_SEED: u64 = 0xE0CA_1B24_2026_0824;

/// Series depth for the rational tail bounds.
const TERMS: u32 = 24;

fn delta_pair() -> BigRational {
    BigRational::new(BigInt::from(1), BigInt::from(200))
}

fn tile_json(d: Domino) -> String {
    format!("[{},{}]", d.hi().value(), d.lo().value())
}

fn rational_json(v: &BigRational) -> String {
    format!("\"{}/{}\"", v.numer(), v.denom())
}

fn opt_rational_json(v: &Option<BigRational>) -> String {
    v.as_ref().map_or("null".to_string(), rational_json)
}

fn interval_json(v: &Option<(BigRational, BigRational)>) -> String {
    v.as_ref().map_or("null".to_string(), |(lo, hi)| {
        format!("[{},{}]", rational_json(lo), rational_json(hi))
    })
}

fn dp_json(gamma: &str, forecast: &DpForecast) -> String {
    format!(
        "{{\"gamma\":\"{gamma}\",\"crossing\":{},\"f_at_end\":{},\"h_max\":{}}}",
        forecast
            .crossing
            .map_or("null".to_string(), |h| h.to_string()),
        rational_json(&forecast.f_at_end),
        forecast.h_max,
    )
}

fn half() -> BigRational {
    BigRational::new(BigInt::from(1), BigInt::from(2))
}

fn nine_tenths() -> BigRational {
    BigRational::new(BigInt::from(9), BigInt::from(10))
}

/// Every pair record of one fixture: the fiber is enumerated ONCE for the
/// whole candidate set (shared work; the coordinates remain per-pair
/// objects, V6), then each unordered pair gets its own record.
fn fixture_records(fixture: usize, reps: u64, world_cap: u64, dp_h_max: u64) -> Vec<String> {
    let f = &FLIP_FIXTURES[fixture];
    let flip = reconstruct_flip(f);
    let candidates_owned: Vec<FrozenPolicy> = flip
        .legal_tiles
        .iter()
        .map(|t| FrozenPolicy::new(shadow_tuple(&flip.position, *t)))
        .collect();
    let field = Level0Field::new(2);
    let refs: Vec<&dyn SlicePolicy> = candidates_owned
        .iter()
        .map(|p| p as &dyn SlicePolicy)
        .collect();
    let outcomes = exact_set_outcomes(&flip.root, &flip.position, &refs, &field);
    let mut records: Vec<String> = Vec::new();
    for i in 0..f.m {
        for j in (i + 1)..f.m {
            records.push(pair_record(
                fixture,
                &flip,
                &candidates_owned,
                &field,
                &outcomes,
                (i, j),
                reps,
                world_cap,
                dp_h_max,
            ));
        }
    }
    records
}

#[allow(clippy::too_many_lines, clippy::too_many_arguments)]
fn pair_record(
    fixture: usize,
    flip: &walt::solver::calibrate::FlipRoot,
    candidates_owned: &[FrozenPolicy],
    field: &Level0Field,
    outcomes: &[Vec<bool>],
    (i, j): (usize, usize),
    reps: u64,
    world_cap: u64,
    dp_h_max: u64,
) -> String {
    let f = &FLIP_FIXTURES[fixture];
    let coords: PairCoordinates = pair_coordinates(&outcomes[i], &outcomes[j]);
    let threshold = edge_threshold(2, &delta_pair());
    let h_min0 = h_plus_min(0, 0, &threshold);
    // Exact-law forecasts (regime 4 honestly yields None).
    let info_rate = coords
        .tau
        .as_ref()
        .and_then(|tau| information_rate_bounds(&coords.q, tau, TERMS));
    let leading = coords
        .tau
        .as_ref()
        .and_then(|tau| leading_order_forecast_bounds(&threshold, &coords.q, tau, TERMS));
    let a64 = u64::try_from(coords.a).expect("counts fit u64");
    let b64 = u64::try_from(coords.b).expect("counts fit u64");
    let n64 = u64::try_from(coords.n).expect("counts fit u64");
    let exact_law = PredictiveLaw::from_counts(coords.a, coords.b, coords.n);
    let dp_half = dp_settlement_forecast(&exact_law, &threshold, (0, 0), &half(), dp_h_max);
    let dp_ninety =
        dp_settlement_forecast(&exact_law, &threshold, (0, 0), &nine_tenths(), dp_h_max);
    // Observed replicate settlement on declared streams.
    let mut observed: Vec<String> = Vec::new();
    for rep in 0..reps {
        let epoch =
            mix(E0_SEED ^ mix(fixture as u64) ^ mix(((i as u64) << 8) | j as u64) ^ mix(rep));
        let scope = format!("pair:e0-{}-h{}-d{}-i{i}j{j}-rep{rep}", f.mode, f.hand, f.d);
        let spec = PairSpec {
            root: &flip.root,
            position: &flip.position,
            policy_a: &candidates_owned[i],
            policy_b: &candidates_owned[j],
            field,
            delta: ScopedDelta::new(scope, delta_pair()),
            epoch,
            world_cap,
            batch: 8,
        };
        let evaluation = evaluate_pair(&spec);
        let row = match &evaluation.result {
            ResultKind::DeltaSettled {
                winner,
                settled_at,
                a,
                b,
                ..
            } => {
                let winner_index = if winner == candidates_owned[i].id() {
                    i
                } else {
                    assert_eq!(
                        winner,
                        candidates_owned[j].id(),
                        "the winner is one of the pair"
                    );
                    j
                };
                format!(
                    "{{\"rep\":{rep},\"epoch\":\"{epoch:016x}\",\"tag\":\"DeltaSettled\",\
                     \"settled_at\":{settled_at},\"winner\":{winner_index},\"a\":{a},\"b\":{b}}}"
                )
            }
            ResultKind::Unresolved { consumed, a, b, .. } => {
                let pivots = a + b;
                let q_hat = BigRational::new(BigInt::from(pivots), BigInt::from(*consumed));
                let tau_hat = (pivots > 0).then(|| {
                    BigRational::new(BigInt::from(*a) - BigInt::from(*b), BigInt::from(pivots))
                });
                let rate_hat = tau_hat
                    .as_ref()
                    .and_then(|t| information_rate_bounds(&q_hat, t, TERMS));
                // The per-pair grounding of the controller's forecast:
                // the same DP refit from the OBSERVED counts (a labeled
                // estimate, parent §8.4), continuing from (a, b).
                let dp_obs = (*consumed > 0 && !q_hat.is_zero()).then(|| {
                    let law = PredictiveLaw::from_counts(
                        u128::from(*a),
                        u128::from(*b),
                        u128::from(*consumed),
                    );
                    dp_settlement_forecast(&law, &threshold, (*a, *b), &half(), dp_h_max)
                });
                format!(
                    "{{\"rep\":{rep},\"epoch\":\"{epoch:016x}\",\"tag\":\"Unresolved\",\
                     \"consumed\":{consumed},\"a\":{a},\"b\":{b},\
                     \"q_hat\":{},\"tau_hat\":{},\"info_rate_hat\":{},\
                     \"dp_from_counts\":{}}}",
                    rational_json(&q_hat),
                    opt_rational_json(&tau_hat),
                    interval_json(&rate_hat),
                    dp_obs.map_or("null".to_string(), |d| dp_json("1/2", &d)),
                )
            }
            other => panic!("the pair evaluator produced {}", other.tag()),
        };
        observed.push(row);
    }
    format!(
        "{{\"kind\":\"pair\",\"mode\":\"{}\",\"hand\":{},\"d\":{},\
         \"i\":{i},\"j\":{j},\"tile_i\":{},\"tile_j\":{},\
         \"policy_i\":\"{}\",\"policy_j\":\"{}\",\"fiber\":\"{}\",\
         \"exact\":{{\"a\":\"{}\",\"b\":\"{}\",\"n0\":\"{}\",\"q\":{},\"g\":{},\
         \"tau\":{},\"hardness\":{}}},\
         \"threshold\":{},\
         \"initial\":{{\"e\":\"1\",\"r_debt\":{},\"h_min\":{h_min0}}},\
         \"forecast\":{{\"info_rate\":{},\"leading_order\":{},\
         \"dp_half\":{},\"dp_ninety\":{},\
         \"law\":{{\"p_plus\":\"{a64}/{n64}\",\"p_minus\":\"{b64}/{n64}\"}}}},\
         \"observed\":[{}]}}",
        f.mode,
        f.hand,
        f.d,
        tile_json(flip.legal_tiles[i]),
        tile_json(flip.legal_tiles[j]),
        candidates_owned[i].policy_id(),
        candidates_owned[j].policy_id(),
        coords.n,
        coords.a,
        coords.b,
        coords.n0,
        rational_json(&coords.q),
        rational_json(&coords.g),
        opt_rational_json(&coords.tau),
        opt_rational_json(&coords.hardness),
        rational_json(&threshold),
        rational_json(&threshold),
        interval_json(&info_rate),
        interval_json(&leading),
        dp_json("1/2", &dp_half),
        dp_json("9/10", &dp_ninety),
        observed.join(","),
    )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out_path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "e0.jsonl".to_string());
    let knob = |k: usize, default: u64| -> u64 {
        args.get(k)
            .map(|v| v.parse().expect("an integer knob"))
            .unwrap_or(default)
    };
    let reps = knob(2, 3);
    let world_cap = knob(3, 1024);
    let dp_h_max = knob(4, 192);
    let fixtures: Vec<usize> = (0..FLIP_FIXTURES.len()).collect();
    eprintln!(
        "e0cal: {} fixtures; reps {reps}, world_cap {world_cap}, dp_h_max {dp_h_max}",
        fixtures.len()
    );
    let run = |fixture: &usize| -> Vec<String> {
        let records = fixture_records(*fixture, reps, world_cap, dp_h_max);
        eprintln!("e0cal: fixture {} done ({} pairs)", fixture, records.len());
        records
    };
    #[cfg(feature = "parallel")]
    let per_fixture: Vec<Vec<String>> = fixtures.par_iter().map(run).collect();
    #[cfg(not(feature = "parallel"))]
    let per_fixture: Vec<Vec<String>> = fixtures.iter().map(run).collect();
    let mut out = std::fs::File::create(&out_path).expect("the output file opens");
    for records in per_fixture {
        for record in records {
            writeln!(out, "{record}").expect("the output file writes");
        }
    }
    eprintln!("e0cal: wrote {out_path}");
}
