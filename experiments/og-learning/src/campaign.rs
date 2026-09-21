//! The generation loop (parent §5) under the registered evidence rule:
//! freeze -> on-policy deals -> outcome gradient -> a bounded candidate
//! ladder -> development screening on fresh deals -> frozen-finalist
//! promotion stream -> promote or retain the incumbent -> repeat or stop.
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
use crate::features::ClauseDictionary;
use crate::gradient::{estimate, GradientEstimate};
use crate::promotion::{EvidenceRule, Verdict};
use crate::rollout::play_deal;
use crate::target::CampaignTarget;

pub const TRAIN_BASE: u64 = 1_000_000;
pub const DEV_BASE: u64 = 3_000_000;
pub const EXAM_BASE: u64 = 9_000_000;
pub const PROMO_BASE: u64 = 20_000_000;
pub const PANEL_BASE: u64 = 500_000;

#[derive(Clone, Debug)]
pub struct CampaignState {
    pub target_id: String,
    pub dictionary_version: String,
    pub generation: u64,
    pub candidate_counter: u64,
    pub consecutive_failures: u32,
    pub train_deals: u64,
    pub dev_deals: u64,
    pub stall_after: u32,
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
    pub fn fresh(
        target: &CampaignTarget,
        dict: &ClauseDictionary,
        train_deals: u64,
        dev_deals: u64,
        stall_after: u32,
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
        let weights: Vec<String> = self.weights.iter().map(rat_to_str).collect();
        let _ = writeln!(out, "weights={}", weights.join(","));
        std::fs::write(path, out).map_err(|e| e.to_string())
    }

    pub fn load(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let mut fields = std::collections::BTreeMap::new();
        for line in text.lines() {
            if let Some((k, v)) = line.split_once('=') {
                fields.insert(k.to_string(), v.to_string());
            }
        }
        let get = |k: &str| -> Result<String, String> {
            fields
                .get(k)
                .cloned()
                .ok_or_else(|| format!("state missing {k}"))
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
            weights,
        })
    }
}

/// Play a seed range with an actor; return (makes, total, learner decisions,
/// inference work). Scores are not collected.
fn measure(
    target: &CampaignTarget,
    dict: &ClauseDictionary,
    actor: &RationalActor,
    base: u64,
    n: u64,
) -> Result<(u64, u64, u64, u64), String> {
    let mut makes = 0u64;
    let mut decisions = 0u64;
    let mut work = 0u64;
    for i in 0..n {
        let rec = play_deal(target, dict, actor, base + i, false)?;
        if rec.y {
            makes += 1;
        }
        decisions += u64::from(rec.learner_decisions);
        work += rec.inference_work;
    }
    Ok((makes, n, decisions, work))
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

pub struct GenerationReport {
    pub json: String,
    pub promoted: bool,
    pub stalled: bool,
}

pub fn run_generation(
    dir: &Path,
    state: &mut CampaignState,
    target: &CampaignTarget,
    dict: &ClauseDictionary,
) -> Result<GenerationReport, String> {
    if state.target_id != target.id || state.dictionary_version != dict.version {
        return Err("state does not match target/dictionary".into());
    }
    let started = Instant::now();
    let g = state.generation;
    let incumbent = RationalActor {
        weights: state.weights.clone(),
        dictionary_version: dict.version.clone(),
    };

    // 1. Inner loop: audited small steps on fresh on-policy sub-bundles
    //    (each step re-samples with the WORKING weights - on-policy per
    //    step), with snapshots after 2/4/8 steps as the candidate ladder.
    assert!(state.train_deals * u64::from(INNER_STEPS) <= 100_000);
    assert!(g < 20, "training seed regions are declared for g < 20");
    let train_base = TRAIN_BASE + g * 100_000;
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
            records.push(play_deal(target, dict, &working_actor, step_base + i, true)?);
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
    let dev_base = DEV_BASE + g * 10_000;
    assert!(state.dev_deals <= 10_000);
    let (inc_makes, inc_n, _, _) = measure(target, dict, &incumbent, dev_base, state.dev_deals)?;

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
        let (makes, n, _, _) = measure(target, dict, &cand, dev_base, state.dev_deals)?;
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
        // per-step alpha bound is (moves that step) * ln_upper(9/8) / 4;
        // the composite is graded whole-policy, as the parent requires.
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
        let promo_base = PROMO_BASE + k * 100_000;
        let mut sum_d = BigRational::zero();
        let mut n_done = 0u64;
        let mut checkpoints_json = Vec::new();
        let mut verdict = Verdict::Unresolved;
        for (j0, n_target) in rule.checkpoints.iter().enumerate() {
            for i in n_done..*n_target {
                let a = play_deal(target, dict, &cand, promo_base + i, false)?;
                let b = play_deal(target, dict, &incumbent, promo_base + i, false)?;
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
        promoted = verdict == Verdict::Promoted;
        promo_json = format!(
            "{{\"candidate\":{},\"digest\":\"{:016x}\",\"snapshot_steps\":{},\
             \"inner_multiplier\":\"{}/{}\",\"moved_total\":{},\
             \"l1_upper\":\"{}\",\"alpha_bound\":\"{}\",\"checkpoints\":[{}],\
             \"verdict\":\"{:?}\",\"final_n\":{},\"final_sum_d\":\"{}\"}}",
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
            rat_to_str(&sum_d)
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
         \"incumbent_digest\":\"{:016x}\",\"train\":{{\"n\":{},\"inner_steps\":{},\
         \"mean_y\":\"{}\",\"mean_y_permille\":{},\"inference_work\":{}}},\
         \"first_step_gradient\":[{}],\
         \"dev\":{{\"incumbent_makes\":\"{}/{}\",\"candidates\":[{}]}},\
         \"promotion\":{},\"promoted\":{},\"consecutive_failures\":{},\
         \"stalled\":{},\"wall_ms\":{}}}",
        g,
        state.target_id,
        state.dictionary_version,
        incumbent.digest(),
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
    for i in 0..n {
        let a = play_deal(target, dict, &incumbent, EXAM_BASE + i, false)?;
        let b = play_deal(target, dict, &uniform, EXAM_BASE + i, false)?;
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
        "{{\"exam\":{{\"target\":\"{}\",\"n\":{},\"incumbent_digest\":\"{:016x}\",\
         \"incumbent_makes\":\"{}/{}\",\"incumbent_permille\":{},\
         \"uniform_makes\":\"{}/{}\",\"uniform_permille\":{},\
         \"paired_mean_d\":\"{}\",\"paired_mean_d_permille\":{},\
         \"alpha\":\"1/20\",\"radius_upper\":\"{}\",\
         \"ci_lower\":\"{}\",\"ci_upper\":\"{}\"}}}}",
        state.target_id,
        n,
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
    fn state_round_trips_through_the_text_format() {
        let target = CampaignTarget::og_v1();
        let dict = ClauseDictionary::standard().unwrap();
        let mut s = CampaignState::fresh(&target, &dict, 128, 64, 3);
        s.weights[2] = BigRational::new(BigInt::from(17), BigInt::from(16));
        s.generation = 4;
        let tmp = std::env::temp_dir().join("og_state_roundtrip_test.txt");
        s.save(&tmp).unwrap();
        let loaded = CampaignState::load(&tmp).unwrap();
        assert_eq!(loaded.weights, s.weights);
        assert_eq!(loaded.generation, 4);
        assert_eq!(loaded.target_id, s.target_id);
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
