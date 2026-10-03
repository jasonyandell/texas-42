//! Synthetic fixed-policy throughput panel. Does not measure a full player,
//! online ready width, policy quality, or the old DFS search.
#[cfg(not(feature = "gpu"))]
fn main() { eprintln!("rollout_bench requires --features gpu"); }

#[cfg(feature = "gpu")]
fn main() { bench::run(); }

#[cfg(feature = "gpu")]
mod bench {
    use serde_json::json;
    use std::hint::black_box;
    use std::time::Instant;
    use walt::rules::Decl;
    use walt::solver::SplitMix64;
    use walt_response_ladder::gpu::DiceRollout;
    use walt_response_ladder::mechanics::{tiles, Problem, PublicState, Scenario};
    use walt_response_ladder::rollout::{evaluate_dice, evaluate_dice_fast, priority_plans, PayoffMatrix};

    fn problem(depth: usize, count: usize) -> Problem {
        let decl = Decl::NoTrump;
        let mut rng = SplitMix64(420601 + depth as u64);
        let mut deck: Vec<_> = (0u8..28).collect();
        for i in (1..28).rev() { let j = rng.below((i+1) as u64) as usize; deck.swap(i,j); }
        let mut hands = [0u32;4];
        for (i,&tile) in deck.iter().enumerate() { hands[i/7] |= 1 << tile; }
        let mut root = PublicState::opening(1);
        for _ in 0..(7-depth)*4 {
            let legal = tiles(root.legal(decl,hands[root.actor() as usize]));
            let tile = legal[rng.below(legal.len() as u64) as usize];
            root = root.after(decl,tile);
        }
        for hand in &mut hands { *hand &= !root.played; }
        let viewer = root.actor();
        let hand = hands[viewer as usize];
        let bid = root.banked_t1+(42-root.banked_t1-root.banked_t0).div_ceil(2);
        // Lawful exchanges preserve hand sizes and all public void deductions.
        // These are synthetic compatible scenarios, not a calibrated sampler.
        let seats: Vec<_> = (0..4).filter(|&s| s != viewer as usize).collect();
        let mut scenarios = Vec::with_capacity(count);
        for _ in 0..count {
            let mut h = hands;
            for _ in 0..48 {
                let a = seats[rng.below(3) as usize];
                let b = seats[rng.below(3) as usize];
                if a == b { continue; }
                let aa = tiles(h[a]); let bb = tiles(h[b]);
                let x = 1 << aa[rng.below(aa.len() as u64) as usize];
                let y = 1 << bb[rng.below(bb.len() as u64) as usize];
                if root.voids[b] & x == 0 && root.voids[a] & y == 0 { h[a] ^= x|y; h[b] ^= x|y; }
            }
            scenarios.push(Scenario { hands:h, tape:rng.next_u64(), weight:1 });
        }
        Problem { decl,bid,viewer,hand,root,scenarios,field_revision:"historical-dice-v1".into() }
    }

    // The same exact integer weighted reduction is included in every arm.
    // This produces plan-family lower witnesses only. No column maximum is
    // labeled an unrestricted upper for the candidate-pool cases.
    fn reduce(problem:&Problem,plans:&[Vec<u8>],matrix:&PayoffMatrix)->Vec<(u8,u64)> {
        let mut values = [None;28];
        for (plan,row) in plans.iter().zip(matrix) {
            let value = row.iter().zip(&problem.scenarios).map(|(&p,s)| u64::from(p)*s.weight).sum();
            let at = &mut values[plan[0] as usize];
            *at = Some(at.unwrap_or(0).max(value));
        }
        values.into_iter().enumerate().filter_map(|(i,x)|x.map(|v|(i as u8,v))).collect()
    }

    pub fn run() {
        let repetitions:usize = std::env::args().nth(1).map(|s|s.parse().unwrap()).unwrap_or(4);
        assert!((2..=20).contains(&repetitions));
        let started = Instant::now();
        let gpu = DiceRollout::new().unwrap();
        let initialization_ms = started.elapsed().as_secs_f64()*1000.0;
        println!("{}", json!({"kind":"initialization","adapter":gpu.adapter_name(),
            "wall_ms":initialization_ms,"internal_ms":gpu.initialization_ms,
            "gpu_scope":"validation, packing, buffer allocation and upload, dispatch, mapped readback, row assembly, integer weighted reduction",
            "cpu_scope":"validation, rule table construction for compact arm, result allocation, scalar rollout, integer weighted reduction",
            "limits":"synthetic compatible bundles; historical Dice only; not full-player or strength evidence"}));
        for depth in [2,4,7] {
            for count in [8,40,512] {
                let started = Instant::now();
                let problem = problem(depth,count);
                let scenario_generation_ms = started.elapsed().as_secs_f64()*1000.0;
                for complete in [false,true] {
                    let started = Instant::now();
                    let roots = tiles(problem.root.legal(problem.decl,problem.hand));
                    let plans = if complete { priority_plans(&problem,None).unwrap() } else {
                        let mut out=Vec::new();
                        // Two inexpensive total priority policies per root,
                        // ascending and descending remaining tiles.
                        for &root in &roots {
                            let tail=tiles(problem.hand & !(1<<root));
                            let mut a=vec![root];a.extend(&tail);out.push(a);
                            if tail.len()>1 {let mut b=vec![root];b.extend(tail.into_iter().rev());out.push(b);}
                        }
                        out
                    };
                    let plan_generation_ms=started.elapsed().as_secs_f64()*1000.0;
                    let expected=evaluate_dice_fast(&problem,&plans).unwrap();
                    assert_eq!(expected,gpu.evaluate_dice(&problem,&plans).unwrap());
                    // Canonical comparison is deliberately complete at every
                    // batch size, including full 7! x 512-scenario matrices.
                    assert_eq!(expected,evaluate_dice(&problem,&plans).unwrap());
                    let reduction=reduce(&problem,&plans,&expected);
                    let mut timings=[Vec::new(),Vec::new(),Vec::new()];
                    for repetition in 0..repetitions {
                        let order=if repetition%2==0 {[0,1,2]} else {[2,1,0]};
                        for arm in order {
                            let started=Instant::now();
                            let matrix=match arm {
                                0=>evaluate_dice(&problem,&plans).unwrap(),
                                1=>evaluate_dice_fast(&problem,&plans).unwrap(),
                                _=>gpu.evaluate_dice(&problem,&plans).unwrap(),
                            };
                            let actual=black_box(reduce(&problem,&plans,black_box(&matrix)));
                            timings[arm].push(started.elapsed().as_secs_f64()*1000.0);
                            assert_eq!(actual,reduction);
                            assert_eq!(matrix,expected);
                        }
                    }
                    println!("{}",json!({"kind":"batch","depth":depth,"declaration":"NT",
                        "scenarios":count,"roots":roots.len(),"plans":plans.len(),
                        "family":if complete {"complete"} else {"two_per_root"},
                        "lanes":plans.len()*count,"max_physical_steps":plans.len()*count*depth*4,
                        "packet_bytes":4*(16+6*count+8*plans.len()),"readback_bytes":4*plans.len()*count,
                        "scenario_generation_ms":scenario_generation_ms,"plan_generation_ms":plan_generation_ms,
                        "canonical_scalar_ms":timings[0],"compact_scalar_ms":timings[1],"gpu_ms":timings[2],
                        "root_lower_witnesses":reduction,"all_lane_payoffs_match":true}));
                }
            }
        }
    }
}
