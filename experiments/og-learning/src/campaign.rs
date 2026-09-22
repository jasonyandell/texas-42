//! The generation loop (parent §5) under the registered evidence rule:
//! freeze -> constructor (generate, dedup on the versioned panel, score
//! candidate derivatives at coefficient zero, admit a bounded number) ->
//! on-policy inner steps -> snapshot finalists -> development screening ->
//! frozen-finalist promotion stream -> promote or retain -> repeat or stop.
//! State is a plain text file; every generation appends one JSON record.
//! All statistics are exact rationals rendered as "num/den" (permille given
//! as an exact integer floor for reading convenience only).

use std::fmt::Write as _;
use std::path::Path;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};

use crate::actor::RationalActor;
use crate::bounds::ln_upper;
use crate::constructor::{generate_pool, panel_filter};
use crate::features::{set_for, ClauseDictionary};
use crate::gradient::{estimate, GradientEstimate};
use crate::anytime::{judge as anytime_judge, AnytimeVerdict, DiffCounts};
use crate::promotion::{BatchStats, EvidenceRule, MfEvidenceRule, Verdict};
use crate::rollout::{collect_panel_decisions, play_deal, DealRecord, DEAL_WORK_BUDGET};
use crate::target::CampaignTarget;

pub const TRAIN_BASE: u64 = 1_000_000;
pub const DEV_BASE: u64 = 3_000_000;
pub const EXAM_BASE: u64 = 9_000_000;
pub const PROMO_BASE: u64 = 20_000_000;
pub const PANEL_BASE: u64 = 500_000;
/// Constructor-scoring bundles live inside the generation's train region,
/// above the inner-step ranges.
pub const SCORE_OFFSET_IN_REGION: u64 = 50_000;

pub const PANEL_SEEDS: u64 = 256;
/// The constructor's dedup filter reads a declared prefix of the panel
/// (cost control; the full panel remains the versioned record).
pub const PANEL_FILTER_SEEDS: u64 = 64;
/// Candidate derivatives are scored on bundles of at most this many deals.
pub const SCORE_DEALS_CAP: u64 = 2_048;

#[derive(Clone, Debug)]
pub struct CampaignState {
    pub target_id: String,
    /// The SEED grammar version; learned expressions extend it and the
    /// composed version lives on the dictionary and in the records.
    pub dictionary_version: String,
    pub generation: u64,
    pub candidate_counter: u64,
    pub consecutive_failures: u32,
    pub train_deals: u64,
    pub dev_deals: u64,
    pub stall_after: u32,
    /// All seed ranges shift by this per-campaign offset, so distinct
    /// campaigns never share deals.
    pub seed_offset: u64,
    /// Constructor switch and caps (frozen at init).
    pub constructor: bool,
    pub dict_cap: usize,
    pub admit_k: usize,
    /// Promotion mode for proxy-bearing targets: "mf" (two-batch
    /// multifidelity) or "direct-eb" (direct expensive stream, EB radii).
    pub promotion_mode: String,
    /// Admitted expressions in admission order: (id, canonical text).
    pub learned: Vec<(String, String)>,
    pub weights: Vec<BigRational>,
}

fn rat_to_str(r: &BigRational) -> String {
    format!("{}/{}", r.numer(), r.denom())
}

fn rat_from_str(s: &str) -> Result<BigRational, String> {
    let (n, d) = s.split_once('/').ok_or_else(|| format!("bad rational {s}"))?;
    let n: BigInt = n.parse().map_err(|e| format!("bad numerator {s}: {e}"))?;
    let d: BigInt = d.parse().map_err(|e| format!("bad denominator {s}: {e}"))?;
    if d.is_zero() {
        return Err(format!("zero denominator in {s}"));
    }
    Ok(BigRational::new(n, d))
}

/// Exact permille floor, for human-readable records only.
pub fn permille(r: &BigRational) -> i64 {
    let scaled = r * BigRational::from_integer(BigInt::from(1000));
    let floored = scaled.floor();
    let s = floored.numer().to_string();
    s.parse::<i64>().unwrap_or(i64::MAX)
}

impl CampaignState {
    #[allow(clippy::too_many_arguments)]
    pub fn fresh(
        target: &CampaignTarget,
        dict: &ClauseDictionary,
        train_deals: u64,
        dev_deals: u64,
        stall_after: u32,
        seed_offset: u64,
        constructor: bool,
    ) -> Self {
        CampaignState {
            target_id: target.id.clone(),
            dictionary_version: dict.version.clone(),
            generation: 0,
            candidate_counter: 0,
            consecutive_failures: 0,
            train_deals,
            dev_deals,
            stall_after,
            seed_offset,
            constructor,
            dict_cap: 60,
            admit_k: 3,
            promotion_mode: "mf".to_string(),
            learned: Vec::new(),
            weights: vec![BigRational::one(); dict.len()],
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        let mut out = String::new();
        let _ = writeln!(out, "target={}", self.target_id);
        let _ = writeln!(out, "dictionary={}", self.dictionary_version);
        let _ = writeln!(out, "generation={}", self.generation);
        let _ = writeln!(out, "candidate_counter={}", self.candidate_counter);
        let _ = writeln!(out, "consecutive_failures={}", self.consecutive_failures);
        let _ = writeln!(out, "train_deals={}", self.train_deals);
        let _ = writeln!(out, "dev_deals={}", self.dev_deals);
        let _ = writeln!(out, "stall_after={}", self.stall_after);
        let _ = writeln!(out, "seed_offset={}", self.seed_offset);
        let _ = writeln!(out, "constructor={}", u8::from(self.constructor));
        let _ = writeln!(out, "dict_cap={}", self.dict_cap);
        let _ = writeln!(out, "admit_k={}", self.admit_k);
        let _ = writeln!(out, "promotion_mode={}", self.promotion_mode);
        for (id, text) in &self.learned {
            assert!(!id.contains(' '), "expression ids carry no spaces");
            let _ = writeln!(out, "expr={id} {text}");
        }
        let weights: Vec<String> = self.weights.iter().map(rat_to_str).collect();
        let _ = writeln!(out, "weights={}", weights.join(","));
        std::fs::write(path, out).map_err(|e| e.to_string())
    }

    pub fn load(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let mut fields = std::collections::BTreeMap::new();
        let mut learned = Vec::new();
        for line in text.lines() {
            if let Some(rest) = line.strip_prefix("expr=") {
                let (id, expr) = rest
                    .split_once(' ')
                    .ok_or_else(|| format!("bad expr line {rest}"))?;
                learned.push((id.to_string(), expr.to_string()));
            } else if let Some((k, v)) = line.split_once('=') {
                fields.insert(k.to_string(), v.to_string());
            }
        }
        let get = |k: &str| -> Result<String, String> {
            fields
                .get(k)
                .cloned()
                .ok_or_else(|| format!("state missing {k}"))
        };
        let get_or = |k: &str, default: &str| -> String {
            fields.get(k).cloned().unwrap_or_else(|| default.to_string())
        };
        let weights = get("weights")?
            .split(',')
            .map(rat_from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(CampaignState {
            target_id: get("target")?,
            dictionary_version: get("dictionary")?,
            generation: get("generation")?.parse().map_err(|e| format!("{e}"))?,
            candidate_counter: get("candidate_counter")?
                .parse()
                .map_err(|e| format!("{e}"))?,
            consecutive_failures: get("consecutive_failures")?
                .parse()
                .map_err(|e| format!("{e}"))?,
            train_deals: get("train_deals")?.parse().map_err(|e| format!("{e}"))?,
            dev_deals: get("dev_deals")?.parse().map_err(|e| format!("{e}"))?,
            stall_after: get("stall_after")?.parse().map_err(|e| format!("{e}"))?,
            seed_offset: get_or("seed_offset", "0").parse().map_err(|e| format!("{e}"))?,
            constructor: get_or("constructor", "0") == "1",
            dict_cap: get_or("dict_cap", "60").parse().map_err(|e| format!("{e}"))?,
            admit_k: get_or("admit_k", "3").parse().map_err(|e| format!("{e}"))?,
            promotion_mode: get_or("promotion_mode", "mf"),
            learned,
            weights,
        })
    }
}

/// Play a seed range with an actor; return (makes, total).
fn measure(
    target: &CampaignTarget,
    dict: &ClauseDictionary,
    actor: &RationalActor,
    base: u64,
    n: u64,
) -> Result<(u64, u64), String> {
    let mut makes = 0u64;
    for i in 0..n {
        let rec = play_deal(target, dict, actor, base + i, false, &[])?;
        if rec.y {
            makes += 1;
        }
    }
    Ok((makes, n))
}

/// One inner-step multiplier (declared); its ln upper bound audits the step.
const INNER_MULTIPLIER: (i64, i64) = (9, 8);
/// Snapshot ladder: candidates are the working weights after these many
/// inner steps (parent §5 steps 5-6: bounded coefficient updates per
/// generation; the composite is still graded whole-policy).
const SNAPSHOTS: [u32; 3] = [2, 4, 8];
const INNER_STEPS: u32 = 8;
const WEIGHT_FLOOR: (i64, i64) = (1, 64);
const WEIGHT_CEIL: (i64, i64) = (64, 1);

fn apply_step(
    weights: &mut [BigRational],
    grad: &GradientEstimate,
    multiplier: (i64, i64),
) -> Vec<usize> {
    let m = BigRational::new(BigInt::from(multiplier.0), BigInt::from(multiplier.1));
    let m_inv = BigRational::new(BigInt::from(multiplier.1), BigInt::from(multiplier.0));
    let floor = BigRational::new(BigInt::from(WEIGHT_FLOOR.0), BigInt::from(WEIGHT_FLOOR.1));
    let ceil = BigRational::new(BigInt::from(WEIGHT_CEIL.0), BigInt::from(WEIGHT_CEIL.1));
    let max_abs = grad
        .per_clause
        .iter()
        .map(|g| g.abs())
        .max()
        .unwrap_or_else(BigRational::zero);
    let threshold = &max_abs / BigRational::from_integer(BigInt::from(4));
    let mut moved = Vec::new();
    if max_abs.is_zero() {
        return moved;
    }
    for (j, g) in grad.per_clause.iter().enumerate() {
        if g.abs() < threshold || g.is_zero() {
            continue;
        }
        let stepped = if g.is_positive() {
            &weights[j] * &m
        } else {
            &weights[j] * &m_inv
        };
        let clamped = if stepped < floor {
            floor.clone()
        } else if stepped > ceil {
            ceil.clone()
        } else {
            stepped
        };
        if clamped != weights[j] {
            weights[j] = clamped;
            moved.push(j);
        }
    }
    moved
}

/// The constructor phase: pool -> panel dedup -> candidate derivatives at
/// coefficient zero on a fresh frozen-incumbent bundle -> admit top-K.
/// Returns a JSON fragment describing what happened.
fn run_constructor(
    state: &mut CampaignState,
    target: &CampaignTarget,
    dict: &mut ClauseDictionary,
    incumbent: &RationalActor,
    train_base: u64,
) -> Result<String, String> {
    if dict.len() >= state.dict_cap {
        return Ok(format!(
            "{{\"skipped\":\"dictionary at declared cap {}\"}}",
            state.dict_cap
        ));
    }
    // The versioned panel: uniform play is dictionary-independent, so these
    // decisions are fixed for the campaign.
    let uniform = RationalActor::uniform(dict.len(), &dict.version);
    let mut panel = Vec::new();
    for i in 0..PANEL_FILTER_SEEDS {
        panel.extend(collect_panel_decisions(
            target,
            dict,
            &uniform,
            PANEL_BASE + state.seed_offset + i,
        )?);
    }
    let mut budget = walt::scheme::Budget::new(DEAL_WORK_BUDGET * 64);
    let mut dict_columns = Vec::with_capacity(dict.len());
    for fix in dict.compiled() {
        let mut column = Vec::with_capacity(panel.len());
        for (frame, legal) in &panel {
            column.push(set_for(fix, frame, *legal, &mut budget)?.bits());
        }
        dict_columns.push(column);
    }
    let (pool, dropped) = generate_pool(dict.registry());
    let survivors = panel_filter(&pool, &dict_columns, &panel, &mut budget)?;

    // Candidate derivatives at coefficient zero (parent §3 g_F), on a fresh
    // frozen-incumbent bundle disjoint from the inner-step ranges.
    let probes: Vec<walt::scheme::CompiledFix> = survivors
        .iter()
        .map(|&i| pool[i].compiled.clone())
        .collect();
    let score_base = train_base + SCORE_OFFSET_IN_REGION;
    let score_deals = state.train_deals.min(SCORE_DEALS_CAP);
    let mut records: Vec<DealRecord> = Vec::with_capacity(score_deals as usize);
    for i in 0..score_deals {
        let mut rec = play_deal(target, dict, incumbent, score_base + i, false, &probes)?;
        rec.score_sums = std::mem::take(&mut rec.probe_scores);
        records.push(rec);
    }
    let g = estimate(&records, probes.len())?;

    let mut ranked: Vec<(usize, BigRational)> = g
        .per_clause
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, v)| (i, v.abs()))
        .collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    let mut admitted_json = Vec::new();
    for (rank_idx, _) in ranked.iter().take(state.admit_k) {
        if dict.len() >= state.dict_cap {
            break;
        }
        let cand = &pool[survivors[*rank_idx]];
        let g_f = &g.per_clause[*rank_idx];
        if g_f.is_zero() {
            continue;
        }
        dict.admit(cand.id.clone(), cand.text.clone(), cand.compiled.clone());
        state.learned.push((cand.id.clone(), cand.text.clone()));
        state.weights.push(BigRational::one());
        admitted_json.push(format!(
            "{{\"id\":\"{}\",\"g\":\"{}\",\"g_permille\":{}}}",
            cand.id,
            rat_to_str(g_f),
            permille(g_f)
        ));
    }
    Ok(format!(
        "{{\"pool\":{},\"dropped\":{dropped},\"panel_decisions\":{},\"survivors\":{},\
         \"scored_deals\":{},\"admitted\":[{}],\"dict_len\":{},\"dict_version\":\"{}\"}}",
        pool.len(),
        panel.len(),
        survivors.len(),
        score_deals,
        admitted_json.join(","),
        dict.len(),
        dict.version
    ))
}

pub struct GenerationReport {
    pub json: String,
    pub promoted: bool,
    pub stalled: bool,
}

pub fn run_generation(
    dir: &Path,
    state: &mut CampaignState,
    target: &CampaignTarget,
    dict: &mut ClauseDictionary,
) -> Result<GenerationReport, String> {
    if state.target_id != target.id || !dict.version.starts_with(&state.dictionary_version) {
        return Err("state does not match target/dictionary".into());
    }
    let started = Instant::now();
    let g = state.generation;
    assert!(state.train_deals * u64::from(INNER_STEPS) <= SCORE_OFFSET_IN_REGION);
    assert!(g < 20, "training seed regions are declared for g < 20");
    let train_base = TRAIN_BASE + state.seed_offset + g * 100_000;

    // Multifidelity targets (parent §6): when the target declares a cheap
    // proxy, training, construction and screening run on the proxy; only
    // the promotion stream and the exam touch the expensive lineup.
    let proxy = target.proxy();
    let play_target = proxy.clone().unwrap_or_else(|| target.clone());

    // 0. Constructor: grow the dictionary before this generation's steps.
    let incumbent_before = RationalActor {
        weights: state.weights.clone(),
        dictionary_version: dict.version.clone(),
    };
    let constructor_json = if state.constructor {
        run_constructor(state, &play_target, dict, &incumbent_before, train_base)?
    } else {
        "null".to_string()
    };
    let incumbent = RationalActor {
        weights: state.weights.clone(),
        dictionary_version: dict.version.clone(),
    };

    // 1. Inner loop: audited small steps on fresh on-policy sub-bundles
    //    (each step re-samples with the WORKING weights - on-policy per
    //    step), with snapshots after 2/4/8 steps as the candidate ladder.
    let mut working = state.weights.clone();
    let mut total_moved = 0u64;
    let mut snapshots: Vec<(u32, Vec<BigRational>, u64)> = Vec::new();
    let mut first_grad: Option<GradientEstimate> = None;
    let mut train_work = 0u64;
    let mut train_makes = 0u64;
    let mut train_n = 0u64;
    for step in 0..INNER_STEPS {
        let step_base = train_base + u64::from(step) * state.train_deals;
        let working_actor = RationalActor {
            weights: working.clone(),
            dictionary_version: dict.version.clone(),
        };
        let mut records = Vec::with_capacity(state.train_deals as usize);
        for i in 0..state.train_deals {
            records.push(play_deal(
                &play_target,
                dict,
                &working_actor,
                step_base + i,
                true,
                &[],
            )?);
        }
        train_work += records.iter().map(|r| r.inference_work).sum::<u64>();
        train_makes += records.iter().filter(|r| r.y).count() as u64;
        train_n += state.train_deals;
        let grad = estimate(&records, dict.len())?;
        let moved = apply_step(&mut working, &grad, INNER_MULTIPLIER);
        total_moved += moved.len() as u64;
        if first_grad.is_none() {
            first_grad = Some(grad);
        }
        if SNAPSHOTS.contains(&(step + 1)) {
            snapshots.push((step + 1, working.clone(), total_moved));
        }
    }
    let grad = first_grad.expect("at least one inner step");

    // 2. Development screening of the snapshot ladder on fresh paired seeds.
    let dev_base = DEV_BASE + state.seed_offset + g * 10_000;
    assert!(state.dev_deals <= 10_000);
    let (inc_makes, inc_n) = measure(&play_target, dict, &incumbent, dev_base, state.dev_deals)?;

    let mut dev_rows = Vec::new();
    let mut best: Option<(u32, u64, Vec<BigRational>, u64)> = None;
    for (steps, weights, moved_total) in &snapshots {
        if weights == &state.weights {
            continue;
        }
        let cand = RationalActor {
            weights: weights.clone(),
            dictionary_version: dict.version.clone(),
        };
        let (makes, n) = measure(&play_target, dict, &cand, dev_base, state.dev_deals)?;
        dev_rows.push(format!(
            "{{\"snapshot_steps\":{steps},\"moved_total\":{moved_total},\"dev_makes\":\"{makes}/{n}\"}}"
        ));
        let better_than_best = match &best {
            None => true,
            Some((_, bm, _, _)) => makes > *bm,
        };
        if makes > inc_makes && better_than_best {
            best = Some((*steps, makes, weights.clone(), *moved_total));
        }
    }

    // 3. Frozen-finalist promotion stream, or a screened failure.
    let rule = EvidenceRule::standard();
    let mut promo_json = String::from("null");
    let mut promoted = false;
    if let Some((snapshot_steps, _dev_makes, weights, moved_total)) = best {
        state.candidate_counter += 1;
        let k = state.candidate_counter;
        let cand = RationalActor {
            weights: weights.clone(),
            dictionary_version: dict.version.clone(),
        };
        // Cumulative L1 audit (parent §4): each inner step moved a set of
        // coordinates by exactly ln(9/8), so ||dtheta||_1 over the whole
        // composite is bounded by (total moves) * ln_upper(9/8). The
        // composite is graded whole-policy, as the parent requires.
        let m = &INNER_MULTIPLIER;
        let l1_upper = BigRational::from_integer(BigInt::from(moved_total))
            * ln_upper(&BigRational::new(BigInt::from(m.0), BigInt::from(m.1)));
        let alpha_bound = {
            let quarter = &l1_upper / BigRational::from_integer(BigInt::from(4));
            if quarter > BigRational::one() {
                BigRational::one()
            } else {
                quarter
            }
        };

        assert!(k <= 100, "promotion seed regions are declared for k <= 100");
        let promo_base = PROMO_BASE + state.seed_offset + k * 100_000;
        let mut checkpoints_json = Vec::new();
        let mut verdict = Verdict::Unresolved;
        let mut n_done = 0u64;
        let mut sum_d = BigRational::zero();
        let mut mf_json = String::from("null");
        if proxy.is_some() && state.promotion_mode == "anytime-direct" {
            // Anytime-valid direct stream on the expensive target (the
            // adjudicated CE-T4/T5 betting mixtures - one alpha_k per
            // candidate, optional stopping valid at every n, judged every
            // JUDGE_EVERY deals up to the declared cap). This is the
            // harvester for the 1-2% band the fixed-checkpoint rules leave
            // unresolved.
            const JUDGE_EVERY: u64 = 512;
            const STREAM_CAP: u64 = 65_536;
            let delta = BigRational::new(BigInt::from(1), BigInt::from(20));
            let tau = BigRational::new(BigInt::from(1), BigInt::from(100));
            let mut counts = DiffCounts::default();
            let mut resolved: Option<Verdict> = None;
            while counts.n() < STREAM_CAP {
                let i = counts.n();
                let a = play_deal(target, dict, &cand, promo_base + i, false, &[])?;
                let b = play_deal(target, dict, &incumbent, promo_base + i, false, &[])?;
                counts.push(i64::from(a.y) - i64::from(b.y));
                if counts.n() % JUDGE_EVERY == 0 {
                    let v = anytime_judge(&delta, &tau, k, &counts);
                    if counts.n() % (JUDGE_EVERY * 8) == 0
                        || v != AnytimeVerdict::Continue
                    {
                        checkpoints_json.push(format!(
                            "{{\"n\":{},\"sum_d\":{},\"verdict\":\"{:?}\"}}",
                            counts.n(),
                            counts.sum(),
                            v
                        ));
                    }
                    match v {
                        AnytimeVerdict::Promoted => {
                            resolved = Some(Verdict::Promoted);
                            break;
                        }
                        AnytimeVerdict::NotPromoted => {
                            resolved = Some(Verdict::NotPromoted);
                            break;
                        }
                        AnytimeVerdict::Continue => {}
                    }
                }
            }
            verdict = resolved.unwrap_or(Verdict::Unresolved);
            n_done = counts.n();
            sum_d = BigRational::new(BigInt::from(counts.sum()), BigInt::from(counts.n().max(1)));
            mf_json = format!(
                "{{\"mode\":\"anytime-direct\",\"tau\":\"1/100\",\"alpha_k\":\"delta/(k(k+1))\",\
                 \"judge_every\":{JUDGE_EVERY},\"stream_cap\":{STREAM_CAP},\
                 \"counts\":{{\"minus\":{},\"zero\":{},\"plus\":{}}},\
                 \"proxy_used_for\":\"training/construction/screening only\",\"target\":\"{}\"}}",
                counts.minus, counts.zero, counts.plus, target.id
            );
        } else if proxy.is_some() && state.promotion_mode == "direct-eb" {
            // Pilot-informed direct stream on the expensive target with
            // variance-sensitive empirical-Bernstein radii (og-v4 gen-0
            // measured this proxy's correlation too weak for MF promotion
            // to pay; the record carries the measurement).
            let eb_rule = EvidenceRule::gym_direct();
            let mut stats = BatchStats::default();
            for (j0, n_target) in eb_rule.checkpoints.iter().enumerate() {
                for i in stats.n..*n_target {
                    let a = play_deal(target, dict, &cand, promo_base + i, false, &[])?;
                    let b = play_deal(target, dict, &incumbent, promo_base + i, false, &[])?;
                    stats.push(i64::from(a.y) - i64::from(b.y));
                }
                let v = eb_rule.judge_eb(k, (j0 + 1) as u64, &stats);
                checkpoints_json.push(format!(
                    "{{\"n\":{},\"sum_d\":{},\"var\":\"{}\",\"verdict\":\"{:?}\"}}",
                    stats.n,
                    stats.sum,
                    rat_to_str(&stats.sample_variance()),
                    v
                ));
                match v {
                    Verdict::Continue { .. } => continue,
                    other => {
                        verdict = other;
                        break;
                    }
                }
            }
            n_done = stats.n;
            sum_d = stats.mean();
            mf_json = format!("{{\"mode\":\"direct-eb\",\"tau\":\"1/100\",\"proxy_used_for\":\"training/construction/screening only\",\"target\":\"{}\"}}", target.id);
        } else if let Some(proxy_t) = &proxy {
            // Multifidelity stream: cheap paired differences on the proxy
            // plus correction pairs evaluating BOTH lineups on the same
            // exogenous deals, in independent declared subregions.
            let mf_rule = MfEvidenceRule::gym();
            let corr_base = promo_base + 70_000;
            let mut cheap = BatchStats::default();
            let mut corr = BatchStats::default();
            let mut diag_sum_dh = 0i64;
            let mut diag_sum_dl = 0i64;
            let mut diag_sum_dhdl = 0i64;
            let mut cheap_ms = 0u128;
            let mut corr_ms = 0u128;
            for (j0, (n_j, m_j)) in mf_rule.checkpoints.iter().enumerate() {
                let t0 = Instant::now();
                for i in cheap.n..*n_j {
                    let a = play_deal(proxy_t, dict, &cand, promo_base + i, false, &[])?;
                    let b = play_deal(proxy_t, dict, &incumbent, promo_base + i, false, &[])?;
                    cheap.push(i64::from(a.y) - i64::from(b.y));
                }
                cheap_ms += t0.elapsed().as_millis();
                let t1 = Instant::now();
                for i in corr.n..*m_j {
                    let seed = corr_base + i;
                    let ha = play_deal(target, dict, &cand, seed, false, &[])?;
                    let hb = play_deal(target, dict, &incumbent, seed, false, &[])?;
                    let la = play_deal(proxy_t, dict, &cand, seed, false, &[])?;
                    let lb = play_deal(proxy_t, dict, &incumbent, seed, false, &[])?;
                    let dh = i64::from(ha.y) - i64::from(hb.y);
                    let dl = i64::from(la.y) - i64::from(lb.y);
                    corr.push(dh - dl);
                    diag_sum_dh += dh;
                    diag_sum_dl += dl;
                    diag_sum_dhdl += dh * dl;
                }
                corr_ms += t1.elapsed().as_millis();
                let v = mf_rule.judge(k, (j0 + 1) as u64, &cheap, &corr);
                let estimate = cheap.mean() + corr.mean();
                checkpoints_json.push(format!(
                    "{{\"n_cheap\":{},\"m_corr\":{},\"cheap_sum\":{},\"corr_sum\":{},\
                     \"cheap_var\":\"{}\",\"corr_var\":\"{}\",\"estimate\":\"{}\",\
                     \"estimate_permille\":{},\"verdict\":\"{:?}\"}}",
                    cheap.n,
                    corr.n,
                    cheap.sum,
                    corr.sum,
                    rat_to_str(&cheap.sample_variance()),
                    rat_to_str(&corr.sample_variance()),
                    rat_to_str(&estimate),
                    permille(&estimate),
                    v
                ));
                match v {
                    Verdict::Continue { .. } => continue,
                    other => {
                        verdict = other;
                        break;
                    }
                }
            }
            n_done = cheap.n;
            sum_d = cheap.mean() + corr.mean();
            // Diagnostics the parent asks us to keep honest: the proxy's
            // sample covariance with the target on the correction seeds,
            // measured per-pair costs, and the §6 allocation ratio computed
            // from the measured quantities (analysis, not an adaptive law).
            let m = corr.n;
            let cov = if m >= 2 {
                let mr = BigRational::from_integer(BigInt::from(m));
                let mh = BigRational::new(BigInt::from(diag_sum_dh), BigInt::from(m));
                let ml = BigRational::new(BigInt::from(diag_sum_dl), BigInt::from(m));
                (BigRational::from_integer(BigInt::from(diag_sum_dhdl)) - &mr * &mh * &ml)
                    / (mr - BigRational::one())
            } else {
                BigRational::zero()
            };
            let c_l_ms = if cheap.n > 0 { cheap_ms / u128::from(cheap.n) } else { 0 };
            let c_c_ms = if corr.n > 0 { corr_ms / u128::from(corr.n) } else { 0 };
            let alloc = {
                let vl = cheap.sample_variance();
                let vc = corr.sample_variance();
                if vc.is_zero() || c_l_ms == 0 {
                    "\"degenerate\"".to_string()
                } else {
                    let ratio = &vl * BigRational::from_integer(BigInt::from(c_c_ms as u64))
                        / (&vc * BigRational::from_integer(BigInt::from(c_l_ms.max(1) as u64)));
                    format!("\"{}\"", rat_to_str(&crate::bounds::sqrt_upper(&ratio)))
                }
            };
            mf_json = format!(
                "{{\"proxy\":\"{}\",\"tau\":\"1/25\",\"sum_dh\":{},\"sum_dl_corr\":{},\
                 \"cov_dh_dl\":\"{}\",\"cheap_ms_per_pair\":{},\"corr_ms_per_pair\":{},\
                 \"alloc_ratio_n_over_m_upper\":{}}}",
                proxy_t.id,
                diag_sum_dh,
                diag_sum_dl,
                rat_to_str(&cov),
                c_l_ms,
                c_c_ms,
                alloc
            );
        } else {
            for (j0, n_target) in rule.checkpoints.iter().enumerate() {
                for i in n_done..*n_target {
                    let a = play_deal(target, dict, &cand, promo_base + i, false, &[])?;
                    let b = play_deal(target, dict, &incumbent, promo_base + i, false, &[])?;
                    let d = i64::from(a.y) - i64::from(b.y);
                    sum_d += BigRational::from_integer(BigInt::from(d));
                }
                n_done = *n_target;
                let v = rule.judge(k, (j0 + 1) as u64, n_done, &sum_d);
                checkpoints_json.push(format!(
                    "{{\"n\":{},\"sum_d\":\"{}\",\"verdict\":\"{:?}\"}}",
                    n_done,
                    rat_to_str(&sum_d),
                    v
                ));
                match v {
                    Verdict::Continue { .. } => continue,
                    other => {
                        verdict = other;
                        break;
                    }
                }
            }
        }
        promoted = verdict == Verdict::Promoted;
        promo_json = format!(
            "{{\"candidate\":{},\"digest\":\"{:016x}\",\"snapshot_steps\":{},\
             \"inner_multiplier\":\"{}/{}\",\"moved_total\":{},\
             \"l1_upper\":\"{}\",\"alpha_bound\":\"{}\",\"checkpoints\":[{}],\
             \"verdict\":\"{:?}\",\"final_n\":{},\"final_estimate\":\"{}\",\
             \"multifidelity\":{}}}",
            k,
            cand.digest(),
            snapshot_steps,
            m.0,
            m.1,
            moved_total,
            rat_to_str(&l1_upper),
            rat_to_str(&alpha_bound),
            checkpoints_json.join(","),
            verdict,
            n_done,
            rat_to_str(&sum_d),
            mf_json
        );
        if promoted {
            state.weights = weights;
            state.consecutive_failures = 0;
        } else {
            state.consecutive_failures += 1;
        }
    } else {
        state.consecutive_failures += 1;
    }

    state.generation += 1;
    let stalled = state.consecutive_failures >= state.stall_after;

    let grad_strings: Vec<String> = grad
        .per_clause
        .iter()
        .map(|r| format!("\"{}\"", rat_to_str(r)))
        .collect();
    let train_mean = BigRational::new(BigInt::from(train_makes), BigInt::from(train_n));
    let json = format!(
        "{{\"generation\":{},\"target\":\"{}\",\"dictionary\":\"{}\",\
         \"incumbent_digest\":\"{:016x}\",\"constructor\":{},\
         \"train\":{{\"n\":{},\"inner_steps\":{},\
         \"mean_y\":\"{}\",\"mean_y_permille\":{},\"inference_work\":{}}},\
         \"first_step_gradient\":[{}],\
         \"dev\":{{\"incumbent_makes\":\"{}/{}\",\"candidates\":[{}]}},\
         \"promotion\":{},\"promoted\":{},\"consecutive_failures\":{},\
         \"stalled\":{},\"wall_ms\":{}}}",
        g,
        state.target_id,
        dict.version,
        incumbent.digest(),
        constructor_json,
        train_n,
        INNER_STEPS,
        rat_to_str(&train_mean),
        permille(&train_mean),
        train_work,
        grad_strings.join(","),
        inc_makes,
        inc_n,
        dev_rows.join(","),
        promo_json,
        promoted,
        state.consecutive_failures,
        stalled,
        started.elapsed().as_millis()
    );

    let record_path = dir.join("generations.jsonl");
    let mut existing = std::fs::read_to_string(&record_path).unwrap_or_default();
    existing.push_str(&json);
    existing.push('\n');
    std::fs::write(&record_path, existing).map_err(|e| e.to_string())?;
    state.save(&dir.join("state.txt"))?;

    Ok(GenerationReport {
        json,
        promoted,
        stalled,
    })
}

/// The final untouched exam: incumbent vs the generation-0 uniform actor on
/// the exam range, paired; two-sided Hoeffding interval at alpha = 1/20.
pub fn run_exam(
    dir: &Path,
    state: &CampaignState,
    target: &CampaignTarget,
    dict: &ClauseDictionary,
    n: u64,
) -> Result<String, String> {
    let incumbent = RationalActor {
        weights: state.weights.clone(),
        dictionary_version: dict.version.clone(),
    };
    let uniform = RationalActor::uniform(dict.len(), &dict.version);
    let mut sum_d = BigRational::zero();
    let mut inc_makes = 0u64;
    let mut uni_makes = 0u64;
    let exam_base = EXAM_BASE + state.seed_offset;
    for i in 0..n {
        let a = play_deal(target, dict, &incumbent, exam_base + i, false, &[])?;
        let b = play_deal(target, dict, &uniform, exam_base + i, false, &[])?;
        inc_makes += u64::from(a.y);
        uni_makes += u64::from(b.y);
        let d = i64::from(a.y) - i64::from(b.y);
        sum_d += BigRational::from_integer(BigInt::from(d));
    }
    let alpha = BigRational::new(BigInt::from(1), BigInt::from(20));
    let radius = crate::bounds::hoeffding_radius_upper(&alpha, n);
    let mean = &sum_d / BigRational::from_integer(BigInt::from(n));
    let lower = &mean - &radius;
    let upper = &mean + &radius;
    let json = format!(
        "{{\"exam\":{{\"target\":\"{}\",\"n\":{},\"exam_base\":{},\
         \"dictionary\":\"{}\",\"incumbent_digest\":\"{:016x}\",\
         \"incumbent_makes\":\"{}/{}\",\"incumbent_permille\":{},\
         \"uniform_makes\":\"{}/{}\",\"uniform_permille\":{},\
         \"paired_mean_d\":\"{}\",\"paired_mean_d_permille\":{},\
         \"alpha\":\"1/20\",\"radius_upper\":\"{}\",\
         \"ci_lower\":\"{}\",\"ci_upper\":\"{}\"}}}}",
        state.target_id,
        n,
        exam_base,
        dict.version,
        incumbent.digest(),
        inc_makes,
        n,
        permille(&BigRational::new(BigInt::from(inc_makes), BigInt::from(n))),
        uni_makes,
        n,
        permille(&BigRational::new(BigInt::from(uni_makes), BigInt::from(n))),
        rat_to_str(&mean),
        permille(&mean),
        rat_to_str(&radius),
        rat_to_str(&lower),
        rat_to_str(&upper)
    );
    std::fs::write(dir.join("exam.json"), format!("{json}\n")).map_err(|e| e.to_string())?;
    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_round_trips_including_learned_expressions() {
        let target = CampaignTarget::og_v1();
        let mut dict = ClauseDictionary::standard().unwrap();
        let (pool, _) = generate_pool(dict.registry());
        let cand = pool.iter().find(|c| c.id == "x[takes-trick]").unwrap();
        dict.admit(cand.id.clone(), cand.text.clone(), cand.compiled.clone());
        let mut s = CampaignState::fresh(&target, &dict, 128, 64, 3, 100_000_000, true);
        s.learned = dict.learned.clone();
        s.weights = vec![BigRational::one(); dict.len()];
        s.weights[14] = BigRational::new(BigInt::from(9), BigInt::from(8));
        s.generation = 4;
        let tmp = std::env::temp_dir().join("og_state_roundtrip_v2_test.txt");
        s.save(&tmp).unwrap();
        let loaded = CampaignState::load(&tmp).unwrap();
        assert_eq!(loaded.weights, s.weights);
        assert_eq!(loaded.learned, s.learned);
        assert_eq!(loaded.seed_offset, 100_000_000);
        assert!(loaded.constructor);
        // Law: the dictionary rebuilt from the loaded state matches.
        let rebuilt = ClauseDictionary::with_learned(&loaded.learned).unwrap();
        assert_eq!(rebuilt.version, dict.version);
        let _ = std::fs::remove_file(tmp);
    }

    #[test]
    fn old_states_load_with_default_offsets_and_no_constructor() {
        // PINNED compatibility witness: an og-v1-era state file (no
        // seed_offset/constructor/expr lines) loads with the defaults.
        let text = "target=t\ndictionary=d\ngeneration=1\ncandidate_counter=2\n\
                    consecutive_failures=0\ntrain_deals=8\ndev_deals=4\nstall_after=3\n\
                    weights=1/1,9/8\n";
        let tmp = std::env::temp_dir().join("og_state_compat_test.txt");
        std::fs::write(&tmp, text).unwrap();
        let loaded = CampaignState::load(&tmp).unwrap();
        assert_eq!(loaded.seed_offset, 0);
        assert!(!loaded.constructor);
        assert!(loaded.learned.is_empty());
        assert_eq!(loaded.weights.len(), 2);
        let _ = std::fs::remove_file(tmp);
    }

    #[test]
    fn candidate_builder_moves_only_strong_coordinates() {
        let grad = GradientEstimate {
            per_clause: vec![
                BigRational::new(BigInt::from(1), BigInt::from(10)),
                BigRational::new(BigInt::from(-1), BigInt::from(10)),
                BigRational::new(BigInt::from(1), BigInt::from(1000)),
            ],
            mean_y: BigRational::zero(),
            n: 2,
        };
        let mut weights = vec![BigRational::one(); 3];
        let moved = apply_step(&mut weights, &grad, (17, 16));
        assert_eq!(moved, vec![0, 1]);
        assert_eq!(
            weights[0],
            BigRational::new(BigInt::from(17), BigInt::from(16))
        );
        assert_eq!(
            weights[1],
            BigRational::new(BigInt::from(16), BigInt::from(17))
        );
        // PINNED strictness witness: the weak coordinate is untouched.
        assert_eq!(weights[2], BigRational::one());
    }
}
