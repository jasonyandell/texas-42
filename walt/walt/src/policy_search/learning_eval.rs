//! Finite on-policy teacher data and independent saved-program evaluation.
//! Each actor is stateless here; controller extensions need a separate protocol.
use super::{
    learning_io::{lesson_text, quote, request_text},
    prices::{ActionBounds, FiniteOracle, NodeKind, PriceProgram, PriceWork},
    program, program_digest, Fixture, Search, State, TablePolicy, Work,
};
use crate::{
    rules::{Domino, Seat},
    scheme::{
        step_frame, Budget, CompiledPolicy, DecisionProvenance, Frame, ObservedPlay, PlayClass,
        PolicyInput, PolicyProgram, Registry,
    },
    solver::{adaptive::SlicePolicy, policy::Level0Field},
};
use num_bigint::BigInt;
use num_rational::BigRational as Rat;
use num_traits::{One, Zero};
use std::{collections::BTreeMap, path::Path, time::Instant};

fn ratio(a: usize, b: usize) -> Rat {
    Rat::new(BigInt::from(a), BigInt::from(b))
}
fn observation(frame: &Frame, actor: Seat, tile: Domino) -> ObservedPlay {
    ObservedPlay {
        actor,
        tile,
        class: match frame.led_context() {
            None => PlayClass::Lead,
            Some(q) if frame.kernel().decl().follows(tile, q) => PlayClass::Follow(q),
            Some(q) => PlayClass::Slough(q),
        },
    }
}
type PublicState = (Frame, Vec<(Seat, Domino)>);
fn public_at(f: &Fixture, history: &[Domino]) -> Result<PublicState, String> {
    let mut frame = f.exercise.frame.clone();
    let mut public = f.history.clone();
    let mut state = State::from_root(&f.exercise.position);
    let mut budget = Budget::new(10_000_000);
    for &tile in history {
        let actor = state.actor();
        frame = step_frame(&frame, observation(&frame, actor, tile), &mut budget)
            .map_err(|e| e.to_string())?;
        public.push((actor, tile));
        state = state.step(f.exercise.position.decl, tile);
    }
    Ok((frame, public))
}
fn stateless(p: &PolicyProgram) -> Result<(), String> {
    if !p.bindings.is_empty()
        || p.rules
            .iter()
            .any(|r| r.in_mode != p.initial_mode || r.next_mode != p.initial_mode)
    {
        return Err(
            "learning panel only accepts a stateless continuation-initialized actor".into(),
        );
    }
    Ok(())
}
type SelectedDecisions = BTreeMap<usize, (Domino, DecisionProvenance, u64)>;
type ProgramValues = (Vec<usize>, SelectedDecisions);

fn program_values(
    f: &Fixture,
    oracle: &FiniteOracle,
    public: &BTreeMap<usize, PublicState>,
    actor: &CompiledPolicy,
) -> Result<ProgramValues, String> {
    let mut values = vec![0; oracle.nodes().len()];
    let mut selected = BTreeMap::new();
    for node in oracle.nodes().iter().rev() {
        values[node.id] = match &node.kind {
            NodeKind::Terminal { success } => {
                if *success {
                    node.active_ids.len()
                } else {
                    0
                }
            }
            NodeKind::Hidden { edges } => edges.iter().map(|(_, child)| values[*child]).sum(),
            NodeKind::Focal { actions } => {
                let (frame, history) = &public[&node.id];
                let input = PolicyInput::new(
                    frame,
                    history,
                    node.state.banked,
                    30,
                    f.exercise.position.declaring_team,
                )
                .map_err(|e| e.to_string())?;
                let mut budget = Budget::new(1_000_000);
                let mut controller = actor
                    .initialize(&input, &mut budget)
                    .map_err(|e| e.to_string())?;
                let trace = actor
                    .choose_traced(&mut controller, &input, &mut budget)
                    .map_err(|e| e.to_string())?;
                let action = actions
                    .iter()
                    .find(|a| a.tile == trace.action)
                    .ok_or("program action absent from exact teacher")?;
                selected.insert(node.id, (trace.action, trace.provenance, budget.spent()));
                values[action.child]
            }
        };
        if values[node.id] > node.exact_makes {
            return Err("program exceeded its exact lawful teacher".into());
        }
    }
    Ok((values, selected))
}

#[allow(clippy::too_many_arguments)]
pub fn run(
    request: &str,
    actors_file: &str,
    output: &str,
    field_name: &str,
    seed: u64,
    samples: usize,
    work_limit: u64,
    price_mode: &str,
) -> Result<String, String> {
    if !(1..=512).contains(&samples)
        || !(1..=100_000_000).contains(&work_limit)
        || !["on", "off"].contains(&price_mode)
    {
        return Err("invalid evaluation bounds".into());
    }
    let start = Instant::now();
    let f =
        super::request::from_text(&std::fs::read_to_string(request).map_err(|e| e.to_string())?)?;
    let field: Box<dyn SlicePolicy> = match field_name {
        "gym" => Box::new(crate::gym::GymField::new(
            f.exercise.root.kernel().viewer(),
            40,
        )),
        "l0-8" => Box::new(Level0Field::new(8)),
        _ => return Err("unsupported strategic field".into()),
    };
    let mut work = PriceWork::new(work_limit);
    let build_start = Instant::now();
    let oracle = FiniteOracle::exact_root_fiber(&f.exercise, &*field, 10000, &mut work)?;
    let build_us = build_start.elapsed().as_micros();
    let n = oracle.worlds().len();
    let root = oracle.node_for_history(&[]).ok_or("missing root")?;
    let optimum = ratio(root.exact_makes, n);
    let mut public = BTreeMap::new();
    for node in oracle.nodes() {
        if matches!(node.kind, NodeKind::Focal { .. }) {
            public.insert(node.id, public_at(&f, &node.state.history)?);
        }
    }
    // Independent exact-search cross-check uses the pre-existing constructor.
    let mut truth = Search::new(&f.exercise, &*field);
    for &world in oracle.worlds() {
        truth.append(world)?;
    }
    let mut truth_work = Work::new(work_limit);
    if truth.solve(&mut truth_work)?.makes != root.exact_makes {
        return Err("finite teacher disagrees with existing exact search".into());
    }
    let mut sampled = Search::new(&f.exercise, &*field);
    let mut frequencies = vec![0usize; n];
    let identity = crate::solver::adaptive::root_identity(&f.exercise.root, &f.exercise.position);
    for i in 0..samples {
        let world = f
            .exercise
            .root
            .world_at(identity, seed ^ 0x0054_5241_494e, i as u64);
        frequencies[oracle
            .worlds()
            .iter()
            .position(|w| *w == world)
            .ok_or("sample outside full fiber")?] += 1;
        sampled.append(world)?;
    }
    let mut sampled_work = Work::new(work_limit);
    let table = TablePolicy::from_node(sampled.solve(&mut sampled_work)?.as_ref());
    let table_program = program::export(&f, &table, "sampled-policy")?;
    let mut actors = BTreeMap::new();
    for line in std::fs::read_to_string(actors_file)
        .map_err(|e| e.to_string())?
        .lines()
    {
        let (name, path) = line
            .split_once('\t')
            .ok_or("actor list needs name TAB path")?;
        if name.is_empty()
            || name == "table"
            || !name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err("unsafe or reserved actor name".into());
        }
        let source = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let p: PolicyProgram = source
            .parse()
            .map_err(|e: crate::scheme::Error| e.to_string())?;
        stateless(&p)?;
        if !p.exact_rules.is_empty() {
            return Err("shared actor must have no exact patches".into());
        }
        if actors.insert(name.to_string(), p).is_some() {
            return Err("duplicate actor".into());
        }
    }
    if actors.is_empty() || actors.len() > 128 {
        return Err("actor count outside1..128".into());
    }
    if price_mode == "off" {
        for name in ["exact", "unpriced", "priced"] {
            if let Some(p) = actors.get(name) {
                let hybrid = program::combine_exact_table_with_relational(
                    &table_program,
                    p,
                    &format!("hybrid-{name}"),
                )?;
                actors.insert(format!("hybrid-{name}"), hybrid);
            }
        }
    }
    actors.insert("table".into(), table_program);
    let price = PriceProgram::new([0, 1, -1, 1]);
    let mut price_work = PriceWork::new(work_limit - work.spent());
    let mut unpriced_work = PriceWork::new(work_limit - work.spent());
    let centers = if price_mode == "on" {
        Some(oracle.centers(&price, &mut price_work)?)
    } else {
        None
    };
    let mut bounds: BTreeMap<usize, Vec<ActionBounds>> = BTreeMap::new();
    if let Some(c) = &centers {
        for node in oracle
            .nodes()
            .iter()
            .filter(|node| matches!(node.kind, NodeKind::Focal { .. }))
        {
            let plain = oracle.action_values_for_history(
                &node.state.history,
                None,
                None,
                &mut unpriced_work,
            )?;
            let priced = oracle.action_values_for_history(
                &node.state.history,
                Some((&price, c)),
                None,
                &mut price_work,
            )?;
            if plain.iter().zip(&priced).any(|(a, b)| {
                a.exact_lawful_q != b.exact_lawful_q
                    || a.perfect_information_upper != b.perfect_information_upper
            }) {
                return Err("pricing changed truth or unpriced comparison".into());
            }
            bounds.insert(node.id, priced);
        }
    }
    let mut pricing_actions = 0usize;
    let mut pricing_tightened = 0usize;
    let mut plain_width = Rat::zero();
    let mut price_width = Rat::zero();
    let mut root_plain_width = Rat::zero();
    let mut root_price_width = Rat::zero();
    let mut root_actions = 0usize;
    for (id, rows) in &bounds {
        for a in rows {
            let plain = a.perfect_information_upper.clone().min(Rat::one());
            let priced = a.priced_upper.clone().unwrap().min(plain.clone());
            pricing_actions += 1;
            pricing_tightened += usize::from(priced < plain);
            plain_width += plain.clone() - a.exact_lawful_q.clone();
            price_width += priced.clone() - a.exact_lawful_q.clone();
            if *id == root.id {
                root_actions += 1;
                root_plain_width += plain - a.exact_lawful_q.clone();
                root_price_width += priced - a.exact_lawful_q.clone();
            }
        }
    }
    let pricing=format!("{{\"actions\":{},\"tightened\":{},\"mean_unpriced_width\":{},\"mean_priced_width\":{},\"root_mean_unpriced_width\":{},\"root_mean_priced_width\":{}}}",pricing_actions,pricing_tightened,
        quote(&(plain_width/BigInt::from(pricing_actions.max(1))).to_string()),quote(&(price_width/BigInt::from(pricing_actions.max(1))).to_string()),
        quote(&(root_plain_width/BigInt::from(root_actions.max(1))).to_string()),quote(&(root_price_width/BigInt::from(root_actions.max(1))).to_string()));
    let dir = Path::new(output);
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let mut reports = Vec::new();
    for (name, p) in actors {
        let source = p.to_string();
        let path = dir.join(format!("{name}.policy"));
        std::fs::write(&path, &source).map_err(|e| e.to_string())?;
        let restored: PolicyProgram = std::fs::read_to_string(&path)
            .map_err(|e| e.to_string())?
            .parse()
            .map_err(|e: crate::scheme::Error| e.to_string())?;
        if restored != p {
            return Err("saved program roundtrip changed actor".into());
        }
        let actor = restored
            .compile(&Registry::standard())
            .map_err(|e| e.to_string())?;
        let (values, selected) = program_values(&f, &oracle, &public, &actor)?;
        let mut visit = vec![(root.id, 0usize, false)];
        let mut first_fallback = BTreeMap::<usize, usize>::new();
        let mut visits = 0usize;
        let mut exact_hits = 0usize;
        let mut rule_hits = 0usize;
        let mut fallback_hits = 0usize;
        let mut inference_work = 0u64;
        let mut regret_sum = Rat::zero();
        let mut certificate = Rat::zero();
        let mut empirical_mass = 0usize;
        let mut unique_mass = 0usize;
        let mut concentration = Rat::zero();
        let mut lesson_files = BTreeMap::new();
        let mut lessons = [String::new(), String::new(), String::new()];
        let mut decision_rows = Vec::new();
        while let Some((id, depth, already_missed)) = visit.pop() {
            let node = oracle.node(id).ok_or("bad oracle edge")?;
            match &node.kind {
                NodeKind::Terminal { .. } => {}
                NodeKind::Hidden { edges } => {
                    for (_, child) in edges {
                        visit.push((*child, depth, already_missed));
                    }
                }
                NodeKind::Focal { actions } => {
                    let (tile, provenance, spent) = &selected[&id];
                    let mass = node.active_ids.len();
                    let weight = ratio(mass, n);
                    visits += mass;
                    inference_work += *spent * mass as u64;
                    let is_miss = matches!(provenance, DecisionProvenance::Fallback);
                    let label = match provenance {
                        DecisionProvenance::Exact => {
                            exact_hits += mass;
                            "exact".to_owned()
                        }
                        DecisionProvenance::RelationalRule { name } => {
                            rule_hits += mass;
                            format!("rule:{name}")
                        }
                        DecisionProvenance::Fallback => {
                            fallback_hits += mass;
                            "fallback".to_owned()
                        }
                    };
                    if is_miss && !already_missed {
                        *first_fallback.entry(depth).or_default() += mass;
                    }
                    let sample_mass: usize = node.active_ids.iter().map(|i| frequencies[*i]).sum();
                    let unique = node
                        .active_ids
                        .iter()
                        .filter(|i| frequencies[**i] > 0)
                        .count();
                    let maximum = node
                        .active_ids
                        .iter()
                        .map(|i| frequencies[*i])
                        .max()
                        .unwrap_or(0);
                    empirical_mass += sample_mass * mass;
                    unique_mass += unique * mass;
                    if sample_mass > 0 {
                        concentration +=
                            ratio(maximum, sample_mass) * Rat::from_integer(BigInt::from(mass));
                    }
                    let action = actions
                        .iter()
                        .find(|a| a.tile == *tile)
                        .ok_or("selected edge missing")?;
                    let costs = actions
                        .iter()
                        .map(|a| (a.tile, ratio(node.exact_makes - a.exact_makes, mass)))
                        .collect::<BTreeMap<_, _>>();
                    regret_sum += costs[tile].clone() * weight.clone();
                    if name != "table" && price_mode == "on" {
                        let priced = &bounds[&id];
                        let upper_plain = priced
                            .iter()
                            .map(|a| a.perfect_information_upper.clone().min(Rat::one()))
                            .max()
                            .unwrap();
                        let upper_price = priced
                            .iter()
                            .map(|a| {
                                a.priced_upper
                                    .clone()
                                    .unwrap()
                                    .min(a.perfect_information_upper.clone())
                                    .min(Rat::one())
                            })
                            .max()
                            .unwrap();
                        let plain_costs = actions
                            .iter()
                            .map(|a| {
                                (
                                    a.tile,
                                    (upper_plain.clone() - ratio(values[a.child], mass))
                                        .clamp(Rat::zero(), Rat::one()),
                                )
                            })
                            .collect::<BTreeMap<_, _>>();
                        let price_costs = actions
                            .iter()
                            .map(|a| {
                                (
                                    a.tile,
                                    (upper_price.clone() - ratio(values[a.child], mass))
                                        .clamp(Rat::zero(), Rat::one()),
                                )
                            })
                            .collect::<BTreeMap<_, _>>();
                        if actions.iter().any(|a| {
                            price_costs[&a.tile] < costs[&a.tile]
                                || plain_costs[&a.tile] < costs[&a.tile]
                        }) {
                            return Err("interval costs failed exact regret audit".into());
                        }
                        certificate += price_costs[tile].clone() * weight.clone();
                        let req = request_text(&f, &public[&id].1, seed);
                        for (text, cost) in
                            lessons.iter_mut().zip([&costs, &plain_costs, &price_costs])
                        {
                            text.push_str(&lesson_text(&req, &weight, cost));
                        }
                    }
                    let h = node
                        .state
                        .history
                        .iter()
                        .map(|d| d.index())
                        .collect::<Vec<_>>();
                    decision_rows.push(format!("{{\"history\":{h:?},\"depth\":{depth},\"action\":{},\"source\":{},\"posterior_worlds\":{mass},\"sample_mass\":{sample_mass},\"sample_unique\":{unique},\"max_sample_mass\":{maximum},\"mode\":{},\"resolved\":false}}",tile.index(),quote(&label),quote(&p.initial_mode)));
                    visit.push((action.child, depth + 1, already_missed || is_miss));
                }
            }
        }
        let value = ratio(values[root.id], n);
        let policy_regret = optimum.clone() - value.clone();
        if regret_sum != policy_regret {
            return Err("native on-policy performance-difference identity failed".into());
        }
        if name != "table" && price_mode == "on" && certificate < policy_regret {
            return Err("whole-policy interval certificate failed".into());
        }
        for (label, text) in ["exact", "unpriced", "priced"].into_iter().zip(lessons) {
            if !text.is_empty() {
                let file = dir.join(format!("{name}.{label}.lessons"));
                std::fs::write(&file, text).map_err(|e| e.to_string())?;
                lesson_files.insert(label, file.to_string_lossy().to_string());
            }
        }
        let root_tile = selected.get(&root.id).map(|s| s.0);
        let root_regret = match (&root.kind, root_tile) {
            (NodeKind::Focal { actions }, Some(tile)) => ratio(
                root.exact_makes - actions.iter().find(|a| a.tile == tile).unwrap().exact_makes,
                n,
            ),
            _ => Rat::zero(),
        };
        let mut replays = Vec::new();
        for world in oracle.worlds().iter().take(3) {
            let replay = program::replay_program_traced(&f, &*field, world, &actor)?;
            // Compare the independently replayed trajectory with the saved-tree policy.
            let mut state = State::from_root(&f.exercise.position);
            for (_, tile) in &replay.plays {
                if state.success(&f.exercise.position).is_none()
                    && state.actor() == f.exercise.root.kernel().viewer()
                {
                    let node = oracle.decision(&state.history)?;
                    if selected[&node.id].0 != *tile {
                        return Err("independent program replay diverged from tree".into());
                    }
                }
                state = state.step(f.exercise.position.decl, *tile);
            }
            let hands =
                Seat::ALL.map(|s| world.hand(s).iter().map(Domino::index).collect::<Vec<_>>());
            let history = f
                .history
                .iter()
                .chain(replay.plays.iter())
                .map(|(s, d)| [s.index(), d.index()])
                .collect::<Vec<_>>();
            let decision_trace = replay.focal_decisions.iter().map(|d| {
                let provenance=match &d.provenance {
                    DecisionProvenance::Exact=>"exact".to_owned(),
                    DecisionProvenance::RelationalRule{name}=>format!("rule:{name}"),
                    DecisionProvenance::Fallback=>"fallback".to_owned(),
                };
                format!("{{\"depth\":{},\"action\":{},\"source\":{},\"work\":{},\"mode_before\":{},\"mode_after\":{},\"contract_resolved\":{}}}",
                    d.focal_decision_depth,d.action.index(),quote(&provenance),d.policy_work_used,
                    quote(&d.controller_before.mode),quote(&d.controller_after.mode),
                    d.contract_resolved.map_or("null".into(),|v|v.to_string()))
            }).collect::<Vec<_>>().join(",");
            replays.push(format!(
                "{{\"made\":{},\"remaining_hands\":{hands:?},\"history\":{history:?},\"focal_decisions\":[{decision_trace}]}}",
                replay.made
            ));
        }
        let file_json = lesson_files
            .iter()
            .map(|(k, v)| format!("{}:{}", quote(k), quote(v)))
            .collect::<Vec<_>>()
            .join(",");
        let first_json = first_fallback
            .iter()
            .map(|(d, c)| {
                format!(
                    "{}:{}",
                    quote(&d.to_string()),
                    quote(&ratio(*c, n).to_string())
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        reports.push(format!("{}:{{\"digest\":{},\"bytes\":{},\"clauses\":{},\"value\":{},\"policy_regret\":{},\"root_regret\":{},\"root_action\":{},\"first_fallback_probability\":{},\"first_fallback_by_depth\":{{{first_json}}},\"unresolved_decisions\":{visits},\"exact_hits\":{exact_hits},\"rule_hits\":{rule_hits},\"fallback_hits\":{fallback_hits},\"inference_work\":{inference_work},\"mean_empirical_mass\":{},\"mean_empirical_unique\":{},\"mean_max_empirical_weight\":{},\"on_policy_regret_sum\":{},\"interval_certificate\":{},\"lessons\":{{{file_json}}},\"decisions\":[{}],\"replays\":[{}]}}",quote(&name),quote(&program_digest(&source)),source.len(),p.rules.len(),quote(&value.to_string()),quote(&policy_regret.to_string()),quote(&root_regret.to_string()),root_tile.map_or("null".into(),|d|d.index().to_string()),quote(&ratio(first_fallback.values().sum(),n).to_string()),quote(&ratio(empirical_mass,visits.max(1)).to_string()),quote(&ratio(unique_mass,visits.max(1)).to_string()),quote(&(concentration/BigInt::from(visits.max(1))).to_string()),quote(&regret_sum.to_string()),if price_mode=="on"&&name!="table"{quote(&certificate.to_string())}else{"null".into()},decision_rows.join(","),replays.join(",")));
    }
    Ok(format!("{{\"schema\":\"relational-evaluation-v1\",\"seed\":{seed},\"worlds\":{n},\"field\":{},\"optimum\":{},\"oracle_nodes\":{},\"tree_work\":{},\"truth_check_nodes\":{},\"sample_nodes\":{},\"build_us\":{build_us},\"elapsed_us\":{},\"unpriced_work\":{},\"price_work\":{},\"price_coefficients\":[0,1,-1,1],\"pricing\":{pricing},\"price_work_detail\":{},\"actors\":{{{}}}}}",quote(field.id()),quote(&optimum.to_string()),oracle.nodes().len(),work.spent(),truth_work.nodes,sampled_work.nodes,start.elapsed().as_micros(),unpriced_work.spent(),price_work.spent(),quote(&format!("{price_work:?}")),reports.join(",")))
}
