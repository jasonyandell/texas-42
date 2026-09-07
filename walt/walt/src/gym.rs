//! Partnership gym: exact, field-relative answer keys and replayable witnesses.
//! Scheme describes opportunities; the existing lawful evaluator grades them.
//! Exploratory finite-domain evidence, not a joint-team oracle or strength rank.
use num_traits::{One, Zero};
use std::{collections::HashMap, sync::Mutex, time::Duration};

use crate::kernel::World;
use crate::rules::{legal_plays, ContextSet, Decl, Domino, DominoSet, Seat, Trick};
use crate::scheme::{
    self, Access, Budget, Fix, Frame, Predicate, PredicateContext, PredicateSpec, Registry, Sort,
    Value,
};
use crate::solver::adaptive::{
    driven_root, CanonicalRoot, DrivenState, PublicRecord, RootPosition, SlicePolicy,
};
use crate::solver::factor_belief::{
    extract_success_policy, viewer_score_profile, ExtractedPolicy, ExtractionSource, FactorBelief,
    RecursionStats, ResponseStats, SupportOracle,
};
use crate::solver::field::FieldStateKey;
use crate::solver::partnership::{self, Config, FieldProfile};
use crate::solver::policy::{continuation_frame, t1_frame_bid, Level0Field, NO_DEADLINE_SECS};
use crate::solver::selection::Rule;
use crate::solver::{mask_of, mix, Deadline, InnerBelief};

pub const OFFER_QUERY: &str = include_str!("../../gym/offer-count.scheme");

pub struct ExerciseRoot {
    pub root: CanonicalRoot,
    pub position: RootPosition,
    pub frame: Frame,
}

/// Constructs the whole exam coordinate from exactly one original hand and
/// actor-attributed public play. No actual hidden deal is an input.
pub fn from_request(
    decl: Decl,
    bidder: Seat,
    viewer: Seat,
    original: DominoSet,
    history: &[(Seat, Domino)],
) -> Result<ExerciseRoot, String> {
    if original.len() != 7 || history.len() >= 28 {
        return Err("need one seven-tile original hand and an unfinished history".into());
    }
    let mut remaining = original;
    let mut played = DominoSet::EMPTY;
    let mut prior = DominoSet::EMPTY;
    let mut prefix = Vec::new();
    let mut leader = bidder;
    let mut banked = [0u32; 2];
    let mut voids = [ContextSet::EMPTY; 4];
    let mut counts = [0usize; 4];
    for (seat, tile) in history {
        if *seat != leader.plus(prefix.len()) || played.contains(*tile) || counts[seat.index()] == 7
        {
            return Err("history violates turn order, capacities, or tile uniqueness".into());
        }
        if voids[seat.index()].iter().any(|q| decl.follows(*tile, q)) {
            return Err("history contradicts an observed void".into());
        }
        let led = prefix.first().map(|d| decl.led_context(*d));
        if *seat == viewer {
            if !legal_plays(decl, remaining, led).contains(*tile) {
                return Err("own historical play is illegal".into());
            }
            remaining.remove(*tile);
        } else if original.contains(*tile) {
            return Err("another seat played an own-hand tile".into());
        }
        if let Some(q) = led {
            if !decl.follows(*tile, q) {
                voids[seat.index()].insert(q);
            }
        }
        counts[seat.index()] += 1;
        played.insert(*tile);
        prefix.push(*tile);
        if prefix.len() == 4 {
            let trick = Trick::new(leader, prefix.clone().try_into().expect("four plays"))
                .map_err(|e| format!("{e:?}"))?;
            leader = trick.winner(decl);
            banked[leader.team().index()] += trick.points();
            prior = played;
            prefix.clear();
        }
    }
    if viewer != leader.plus(prefix.len()) || remaining.is_empty() {
        return Err("not the viewer's turn".into());
    }
    let (root, position) = driven_root(&DrivenState {
        decl,
        bid: 30,
        declaring_team: bidder.team(),
        viewer_hand: remaining,
        leader,
        trick_plays: &prefix,
        banked,
        prior_played: prior,
        voids,
    })
    .map_err(|e| e.to_string())?;
    if root.count() == 0 {
        return Err("history has no legal hidden completion".into());
    }
    let frame =
        Frame::new(root.kernel().clone(), leader, prefix, played).map_err(|e| e.to_string())?;
    Ok(ExerciseRoot {
        root,
        position,
        frame,
    })
}

struct OfferCount(PredicateSpec);
impl Predicate for OfferCount {
    fn spec(&self) -> &PredicateSpec {
        &self.0
    }
    fn evaluate(
        &self,
        ctx: PredicateContext<'_>,
        args: &[Value],
        _: &mut Budget,
    ) -> scheme::Result<Option<bool>> {
        let frame = ctx.frame();
        let [Value::Domino(tile)] = args else {
            return Err(scheme::Error("expected domino".into()));
        };
        if frame.next_actor() != Some(frame.kernel().viewer()) || frame.prefix().len() < 2 {
            return Ok(None);
        }
        let decl = frame.kernel().decl();
        let q = frame.led_context().expect("nonempty trick");
        let best = frame
            .prefix()
            .iter()
            .map(|d| decl.trick_key(*d, q))
            .max()
            .expect("nonempty trick");
        Ok(Some(
            tile.count() > 0
                && frame.current_winner() == Some(frame.kernel().viewer().plus(2))
                && legal_plays(decl, frame.kernel().viewer_hand(), Some(q)).contains(*tile)
                && decl.trick_key(*tile, q) < best,
        ))
    }
}
pub fn registry() -> Registry {
    let mut r = Registry::standard();
    r.register(OfferCount(PredicateSpec {
        name: "offer-count-to-partner".into(),
        version: "gym-v1".into(),
        parameters: vec![Sort::Domino],
        horizon_plies: 1,
        access: Access::Viewer,
    }))
    .expect("unique registered extension");
    r
}

/// The fixed teammate/opponent field. Each actor gets only its own hand and
/// public record. L1 here is a deterministic 40/8 procedure with a declared
/// field seed schedule, not a promise to reproduce a live wrapper's random tape.
pub struct GymField {
    partner: Seat,
    low: Level0Field,
    partner_worlds: usize,
    cache: Mutex<HashMap<FieldStateKey, Domino>>,
    id: String,
}
impl GymField {
    pub fn new(viewer: Seat, partner_worlds: u64) -> Self {
        assert!((1..=640).contains(&partner_worlds));
        let low = Level0Field::new(8);
        let id = format!("gym-field-v1/partner={}/l1-fixed-{partner_worlds}-8/inner=voidless/opponents={}/seed=420600-state-v1/tie=lowest/fallback=none", viewer.plus(2), low.id());
        Self {
            partner: viewer.plus(2),
            low,
            partner_worlds: partner_worlds as usize,
            cache: Mutex::new(HashMap::new()),
            id,
        }
    }
}
impl SlicePolicy for GymField {
    fn id(&self) -> &str {
        &self.id
    }
    fn choose(
        &self,
        decl: Decl,
        hand: DominoSet,
        legal: DominoSet,
        record: &PublicRecord<'_>,
    ) -> Domino {
        if record.leader.plus(record.trick_plays.len()) == self.partner {
            if legal.len() == 1 {
                return legal.iter().next().expect("forced move");
            }
            let key = FieldStateKey::from_public(hand, record);
            if let Some(tile) = self.cache.lock().expect("field cache").get(&key) {
                return *tile;
            }
            let frame = continuation_frame(decl, record.root, record.history);
            let cfg = Config {
                inner_belief: InnerBelief::Voidless,
                selection: Rule::Fixed,
                modeled_selection: Rule::Fixed,
                profile: FieldProfile::Baseline,
                n_outer: self.partner_worlds,
                n0: 8,
                n1: 2,
                seed: mix(420600 ^ key.digest64()),
                deadline: Deadline::after(Duration::from_secs(NO_DEADLINE_SECS)),
            };
            // Same evaluator as the playable L1/L2. Only this declared
            // field's deterministic state seed differs from the wrapper.
            let evaluated = partnership::evaluate(
                decl,
                t1_frame_bid(record.root.bid, record.root.declaring_team),
                frame.seat,
                mask_of(hand),
                mask_of(legal),
                &frame.key,
                frame.sizes(),
                frame.voids,
                frame.trick_start_played,
                frame.boundary_hand_size,
                &cfg,
            )
            .expect("positive-support field state");
            let tile = Domino::from_index(evaluated.best() as usize).expect("legal choice");
            if let Some(previous) = self.cache.lock().expect("field cache").insert(key, tile) {
                assert_eq!(previous, tile);
            }
            tile
        } else {
            self.low.choose(decl, hand, legal, record)
        }
    }
}

struct RootAction<'a> {
    tile: Domino,
    continuation: &'a ExtractedPolicy,
}
impl SlicePolicy for RootAction<'_> {
    fn id(&self) -> &str {
        self.continuation.id()
    }
    fn choose(
        &self,
        decl: Decl,
        hand: DominoSet,
        legal: DominoSet,
        record: &PublicRecord<'_>,
    ) -> Domino {
        if record.history.is_empty() {
            self.tile
        } else {
            self.continuation.choose(decl, hand, legal, record)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Trace {
    pub hands: [Vec<usize>; 4],
    pub plays: Vec<usize>,
    pub banked: [u32; 2],
    pub partner_count: u32,
    pub success: bool,
}

/// Direct full-hand replay, separate from the factorized value recursion. Kept
/// complete even after make/set settles so count-capture explanations can be checked.
fn replay(
    ex: &ExerciseRoot,
    world: &World,
    focal: &dyn SlicePolicy,
    field: &dyn SlicePolicy,
) -> Trace {
    let pos = &ex.position;
    let viewer = ex.root.kernel().viewer();
    let mut hands = world.hands();
    let mut leader = pos.leader;
    let mut plays = pos.trick_plays.clone();
    let mut banked = pos.banked;
    let mut history = Vec::new();
    let mut flat = Vec::new();
    let mut partner_count = 0;
    while hands.iter().any(|h| !h.is_empty()) {
        let seat = leader.plus(plays.len());
        let hand = hands[seat.index()];
        let legal = legal_plays(
            pos.decl,
            hand,
            plays.first().map(|d| pos.decl.led_context(*d)),
        );
        let record = PublicRecord {
            leader,
            trick_plays: &plays,
            banked,
            root: pos,
            history: &history,
        };
        let policy = if seat == viewer { focal } else { field };
        let tile = policy.choose(pos.decl, hand, legal, &record);
        assert!(legal.contains(tile), "a frozen policy chooses legally");
        hands[seat.index()].remove(tile);
        flat.extend([seat.index(), tile.index()]);
        plays.push(tile);
        history.push(tile);
        if plays.len() == 4 {
            let trick = Trick::new(leader, plays.clone().try_into().expect("four tiles"))
                .expect("unique plays");
            leader = trick.winner(pos.decl);
            banked[leader.team().index()] += trick.points();
            if leader == viewer.plus(2) {
                partner_count += trick.points() - 1;
            }
            plays.clear();
        }
    }
    assert!(plays.is_empty());
    assert_eq!(banked.iter().sum::<u32>(), 42);
    let made = banked[pos.declaring_team.index()] >= 30;
    Trace {
        hands: world.hands().map(|h| h.iter().map(Domino::index).collect()),
        plays: flat,
        banked,
        partner_count,
        success: made == (viewer.team() == pos.declaring_team),
    }
}

#[derive(Clone, Debug)]
pub struct Action {
    pub tile: usize,
    pub success_mass: u128,
    pub bins: [u128; 43],
    pub policy_id: String,
    pub policy_states: usize,
    pub traces: Vec<Trace>,
}
pub struct Assessment {
    pub field_id: String,
    pub worlds: u128,
    pub offers: Vec<usize>,
    pub best: Vec<usize>,
    pub actions: Vec<Action>,
    pub scheme_identity: String,
}

pub fn assess(
    ex: &ExerciseRoot,
    field: &dyn SlicePolicy,
    max_worlds: u128,
) -> Result<Assessment, String> {
    let worlds = ex.root.count();
    if worlds > max_worlds {
        return Err(format!("world cap: {worlds} > {max_worlds}"));
    }
    let query = OFFER_QUERY
        .parse::<Fix>()
        .map_err(|e| e.to_string())?
        .compile(&registry())
        .map_err(|e| e.to_string())?;
    let measure = scheme::Belief::uniform(
        ex.frame.clone(),
        usize::try_from(max_worlds).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let summary = query
        .summarize(&measure, &mut Budget::new(10_000_000))
        .map_err(|e| e.to_string())?;
    // The query is public: every candidate's presence is zero or one. Never
    // silently condition the pupil's belief on a hidden matching pattern.
    let mut offers = Vec::new();
    for answer in summary.answer_presence_mass.keys() {
        let p = summary.presence_probability(answer);
        assert!(p.is_zero() || p.is_one());
        let [Value::Domino(tile)] = answer.0.as_slice() else {
            return Err("offer query output must be one domino".into());
        };
        offers.push(tile.index());
    }
    let oracle = SupportOracle;
    let belief = FactorBelief::uniform_root(&ex.root, &ex.position, field);
    let legal = legal_plays(
        ex.position.decl,
        ex.root.kernel().viewer_hand(),
        ex.frame.led_context(),
    );
    let mut actions = Vec::new();
    for tile in legal.iter() {
        let child = belief.focal_play(tile);
        let (success_mass, continuation) = extract_success_policy(
            &oracle,
            &child,
            &ExtractionSource::FullLegal,
            field,
            &mut ResponseStats::default(),
        );
        let focal = RootAction {
            tile,
            continuation: &continuation,
        };
        let profile = viewer_score_profile(
            &oracle,
            &belief,
            &focal,
            field,
            &mut RecursionStats::default(),
        );
        let mass = if ex.root.kernel().viewer().team() == ex.position.declaring_team {
            profile.tail(30)
        } else {
            profile.total() - profile.tail(30)
        };
        assert_eq!(
            mass, success_mass,
            "independent fixed-policy repricing matches extraction"
        );
        assert_eq!(profile.total(), worlds);
        let traces: Vec<_> = ex
            .root
            .worlds()
            .map(|w| replay(ex, &w, &focal, field))
            .collect();
        let mut bins = [0u128; 43];
        for trace in &traces {
            bins[trace.banked[ex.position.declaring_team.index()] as usize] += 1;
        }
        assert_eq!(
            bins, profile.bins,
            "direct complete traces rederive score law"
        );
        assert_eq!(
            traces.iter().filter(|t| t.success).count() as u128,
            success_mass
        );
        actions.push(Action {
            tile: tile.index(),
            success_mass,
            bins,
            policy_id: continuation.id().into(),
            policy_states: continuation.states(),
            traces,
        });
    }
    let optimum = actions
        .iter()
        .map(|a| a.success_mass)
        .max()
        .ok_or("no legal action")?;
    let best = actions
        .iter()
        .filter(|a| a.success_mass == optimum)
        .map(|a| a.tile)
        .collect();
    Ok(Assessment {
        field_id: field.id().into(),
        worlds,
        offers,
        best,
        actions,
        scheme_identity: query.identity(),
    })
}
