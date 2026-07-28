//! Inspector trace emission: a versioned, deterministic, NON-NORMATIVE
//! JSON record of a full self-play match, state by state and perspective by
//! perspective.
//!
//! Honesty rules: every displayed value is exact (integer totals, exact
//! fiber counts and marginal counts as decimal strings, reduced rational
//! averages) — the viewer only displays. Perspective masking is derived
//! HERE, from each seat's own `MechanicalState`: the per-seat `views`
//! records contain exactly what that seat knows; the omniscient `truth` and
//! `deal` fields are separate. A viewer bug can therefore not leak hidden
//! information out of a seat view, because it was never emitted into one.

use num_bigint::BigUint;
use num_traits::Zero;

use rob_core::{
    all_ids, assignment_count, derive_rule_cells, domino_from_id, natural_incidence, AuctionAction,
    BidValue, Declaration, DominoSet, LedSuit, MechanicalState, RulesConfig, Seat, HIDDEN_SEATS,
};

use crate::match_driver::{fmt_declaration, fmt_domino, run_match};
use crate::observer::{DecisionContext, HandEndContext, HandStartContext, MatchObserver};
use crate::player::{MonteCarloPlayer, UtilityLens};

/// Public state shared by all four perspectives (before the play).
#[derive(Clone, Debug)]
pub struct TracePublic {
    /// Current trick leader.
    pub leader: u8,
    /// Current partial trick as `(actor, domino)`.
    pub trick: Vec<(u8, String)>,
    /// Banked points by partnership.
    pub points: [u32; 2],
    /// Actor-attributed played sets.
    pub played: [Vec<String>; 4],
    /// Derived public void contexts per seat (`"0".."6"`, `"C"`).
    pub voids: [Vec<String>; 4],
}

/// One seat's exact private view: precisely what that seat knows, emitted
/// from that seat's own mechanical state (the masking source of truth).
#[derive(Clone, Debug)]
pub struct TraceView {
    /// The seat's own remaining hand.
    pub own: Vec<String>,
    /// The unseen pool from this seat's perspective.
    pub pool: Vec<String>,
    /// The three hidden seats in clockwise order from this seat.
    pub hidden: [u8; 3],
    /// Hidden-seat capacities.
    pub caps: [usize; 3],
    /// Locally allowed possible-set sizes per hidden seat.
    pub psizes: [usize; 3],
    /// Exact fiber cardinality (decimal string).
    pub fiber: String,
    /// Exact holder-marginal counts per pool tile: `(tile, [count at each
    /// hidden seat])`; probability is count / fiber.
    pub marginals: Vec<(String, [String; 3])>,
}

/// One root decision state.
#[derive(Clone, Debug)]
pub struct TraceDecision {
    /// Play index in the hand, `0..28`.
    pub play: u64,
    /// Trick number `1..=7`.
    pub trick: u32,
    /// Position within the trick `0..=3`.
    pub pos: u8,
    /// The acting seat.
    pub actor: u8,
    /// Shared public state before the play.
    pub public: TracePublic,
    /// The four per-seat private views before the play.
    pub views: [TraceView; 4],
    /// Omniscient remaining hands before the play (truth; never part of a
    /// seat view).
    pub truth: [Vec<String>; 4],
    /// The actor's legal set (canonical order).
    pub legal: Vec<String>,
    /// Per-action integer utility totals over the common sampled worlds.
    pub totals: Vec<i64>,
    /// Worlds sampled (0 when forced).
    pub worlds: usize,
    /// Reduced exact rational averages `total/worlds`, aligned with
    /// `legal`; empty when forced.
    pub avgs: Vec<String>,
    /// The chosen action.
    pub chosen: String,
    /// Whether the action was forced (single legal play).
    pub forced: bool,
    /// rob's capped contingency-book projection when the actor is rob
    /// (`book::plan_book_projection` JSON), `None` for baseline deciders.
    /// Display data only (INV-P1).
    pub plan: Option<String>,
    /// Leader after the play.
    pub after_leader: u8,
    /// Points after the play.
    pub after_points: [u32; 2],
    /// `(winner, points)` when this play completed a trick.
    pub trick_complete: Option<(u8, u8)>,
}

/// Hand settlement summary.
#[derive(Clone, Debug)]
pub struct TraceResult {
    /// The declaring partnership.
    pub declaring_team: u8,
    /// Its final points.
    pub declaring_points: u32,
    /// Whether the contract was made.
    pub made: bool,
    /// The stake in marks.
    pub stake: u32,
    /// The partnership awarded the stake.
    pub award_team: u8,
    /// Marks after settlement.
    pub marks_after: [u32; 2],
}

/// One traced contracted hand.
#[derive(Clone, Debug)]
pub struct TraceHand {
    /// Deal-attempt index.
    pub index: u64,
    /// Shaker seat.
    pub shaker: u8,
    /// Winning bidder.
    pub bidder: u8,
    /// Declaration display.
    pub declaration: String,
    /// The declaration's trump (called-and-powered) tiles, canonical order —
    /// exactly the public trump suit for this hand, so the viewer marks trump
    /// tiles without recomputing any game rule (Math §3.2 called set `κ_δ`).
    pub trump: Vec<String>,
    /// The actor-attributed auction actions.
    pub auction: Vec<(u8, String)>,
    /// The omniscient dealt hands (truth).
    pub deal: [Vec<String>; 4],
    /// The 28 decision states.
    pub decisions: Vec<TraceDecision>,
    /// Settlement (present once the hand ends).
    pub result: Option<TraceResult>,
}

/// A complete inspector trace.
#[derive(Clone, Debug)]
pub struct TraceDocument {
    /// Rules configuration `(max_mark_bid, match_target)`.
    pub config: (u32, u32),
    /// Player parameters `(worlds_per_decision, lens, seed)`.
    pub player: (usize, String, u64),
    /// Match seed.
    pub match_seed: u64,
    /// All traced hands.
    pub hands: Vec<TraceHand>,
    /// Final marks.
    pub final_marks: [u32; 2],
}

fn fmt_set(set: &DominoSet) -> Vec<String> {
    set.iter().map(fmt_domino).collect()
}

/// The declaration's trump (called-and-powered) tile set, mirroring the
/// called set `κ_δ` built by the core suit tables (Math §3.2): every domino
/// bearing the trump pip, the seven doubles, or nothing for no-trump. Public
/// information — the viewer only marks these tiles, it never derives them.
/// Public for the rob trace builder (P5).
pub fn trump_set(declaration: Declaration) -> DominoSet {
    match declaration {
        Declaration::PipTrump(pip) => natural_incidence(pip),
        Declaration::DoublesTrump => {
            DominoSet::from_ids(all_ids().filter(|&id| domino_from_id(id).is_double()))
        }
        Declaration::NoTrump => DominoSet::empty(),
    }
}

fn fmt_context(q: LedSuit) -> String {
    match q {
        LedSuit::Natural(p) => p.value().to_string(),
        LedSuit::Called => "C".to_string(),
    }
}

/// Auction-action display (public for the rob trace builder, P5).
pub fn fmt_action(action: AuctionAction) -> String {
    match action {
        AuctionAction::Pass => "pass".to_string(),
        AuctionAction::Bid(BidValue::Point(n)) => format!("P({})", n.value()),
        AuctionAction::Bid(BidValue::Mark(m)) => format!("M({})", m.value()),
    }
}

fn reduced_average(total: i64, worlds: usize) -> String {
    let worlds = worlds as i64;
    debug_assert!(worlds > 0);
    let g = gcd(total.unsigned_abs(), worlds.unsigned_abs());
    let (n, d) = ((total / g as i64), (worlds / g as i64));
    if d == 1 {
        n.to_string()
    } else {
        format!("{n}/{d}")
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a.max(1)
}

/// Build one seat's exact view record from that seat's own mechanical
/// state (the perspective-masking source). Public so the rob trace builder
/// (`rob-verify` P5) reuses the identical masking path.
pub fn view_of(state: &MechanicalState) -> TraceView {
    let cells = derive_rule_cells(state);
    let (abstract_cells, tile_order) = cells.to_abstract();
    let fiber = assignment_count(&abstract_cells);
    let mut marginals = Vec::with_capacity(tile_order.len());
    for (t, &tile) in tile_order.iter().enumerate() {
        let counts: [String; 3] = core::array::from_fn(|s| {
            if abstract_cells.capacity(s) > 0 && abstract_cells.possible(s)[t] {
                let successor = abstract_cells
                    .removal_update(s, t)
                    .expect("allowed tile with capacity");
                assignment_count(&successor).to_string()
            } else {
                BigUint::zero().to_string()
            }
        });
        marginals.push((fmt_domino(tile), counts));
    }
    let hidden = state.hidden_seats();
    TraceView {
        own: fmt_set(state.own_remaining_hand()),
        pool: fmt_set(cells.unseen_pool()),
        hidden: core::array::from_fn(|i| hidden[i].index() as u8),
        caps: core::array::from_fn(|i| cells.capacity(i)),
        psizes: core::array::from_fn(|i| cells.possible(i).len()),
        fiber: fiber.to_string(),
        marginals,
    }
}

/// The trace-building observer.
#[derive(Default)]
struct TraceObserver {
    hands: Vec<TraceHand>,
    final_marks: [u32; 2],
}

impl MatchObserver for TraceObserver {
    fn on_hand_start(&mut self, context: &HandStartContext<'_>) {
        self.hands.push(TraceHand {
            index: context.attempt_index,
            shaker: context.shaker.index() as u8,
            bidder: context.bidder.index() as u8,
            declaration: fmt_declaration(context.declaration),
            trump: fmt_set(&trump_set(context.declaration)),
            auction: context
                .auction_actions
                .iter()
                .map(|&(seat, action)| (seat.index() as u8, fmt_action(action)))
                .collect(),
            deal: core::array::from_fn(|s| fmt_set(context.deal.hand(Seat::ALL[s]))),
            decisions: Vec::with_capacity(28),
            result: None,
        });
    }

    fn on_decision(&mut self, context: &DecisionContext<'_>) {
        let hand = self.hands.last_mut().expect("hand started");
        let reference = &context.viewers[0];
        // Public facts are identical across the four viewer states.
        let public = TracePublic {
            leader: reference.leader().index() as u8,
            trick: reference
                .current_trick()
                .iter()
                .map(|p| (p.actor.index() as u8, fmt_domino(p.domino)))
                .collect(),
            points: reference.hand_points(),
            played: core::array::from_fn(|s| fmt_set(reference.played_by_seat(Seat::ALL[s]))),
            voids: core::array::from_fn(|s| {
                reference
                    .public_voids(Seat::ALL[s])
                    .iter()
                    .map(fmt_context)
                    .collect()
            }),
        };
        let report = context.report;
        let forced = report.worlds == 0;
        let after = context.objective_after.state();
        hand.decisions.push(TraceDecision {
            play: context.play_index,
            trick: (context.play_index / 4) as u32 + 1,
            pos: (context.play_index % 4) as u8,
            actor: context.actor.index() as u8,
            public,
            views: core::array::from_fn(|s| view_of(&context.viewers[s])),
            truth: core::array::from_fn(|s| {
                fmt_set(
                    context
                        .objective_before
                        .state()
                        .remaining_hand(Seat::ALL[s]),
                )
            }),
            legal: report.legal.iter().copied().map(fmt_domino).collect(),
            totals: report.totals.clone(),
            worlds: report.worlds,
            avgs: if forced {
                Vec::new()
            } else {
                report
                    .totals
                    .iter()
                    .map(|&t| reduced_average(t, report.worlds))
                    .collect()
            },
            chosen: fmt_domino(report.chosen),
            forced,
            plan: None,
            after_leader: after.leader().index() as u8,
            after_points: after.hand_points(),
            trick_complete: context
                .trick_result
                .map(|r| (r.winner.index() as u8, r.points)),
        });
    }

    fn on_hand_end(&mut self, context: &HandEndContext<'_>) {
        let hand = self.hands.last_mut().expect("hand started");
        let declaring = Seat::ALL[usize::from(hand.bidder)].team();
        hand.result = Some(TraceResult {
            declaring_team: declaring.index() as u8,
            declaring_points: context.result.final_points[declaring.index()],
            made: context.result.award.made,
            stake: context.result.award.marks,
            award_team: context.result.award.team.index() as u8,
            marks_after: context.marks_after,
        });
    }

    fn on_match_end(&mut self, final_marks: [u32; 2], _hands: u32) {
        self.final_marks = final_marks;
    }
}

/// Trace one deterministic self-play match (same driver as the frozen
/// receipt; the observer is passive).
pub fn trace_match(config: RulesConfig, player: &MonteCarloPlayer, seed: u64) -> TraceDocument {
    let mut observer = TraceObserver::default();
    run_match(config, player, seed, &mut observer);
    TraceDocument {
        config: (config.max_mark_bid(), config.match_target()),
        player: (
            player.worlds_per_decision,
            match player.lens {
                UtilityLens::Points => "Points".to_string(),
                UtilityLens::ContractSuccess => "ContractSuccess".to_string(),
            },
            player.seed,
        ),
        match_seed: seed,
        hands: observer.hands,
        final_marks: observer.final_marks,
    }
}

// ---------------------------------------------------------------------------
// JSON serialization (hand-rolled: deterministic key order, no runtime
// dependency beyond the workspace's num crates).

fn json_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn json_str_array(values: &[String]) -> String {
    let items: Vec<String> = values.iter().map(|v| json_string(v)).collect();
    format!("[{}]", items.join(","))
}

fn json_pairs(pairs: &[(u8, String)]) -> String {
    let items: Vec<String> = pairs
        .iter()
        .map(|(seat, value)| format!("[{},{}]", seat, json_string(value)))
        .collect();
    format!("[{}]", items.join(","))
}

impl TraceView {
    fn to_json(&self) -> String {
        let marginals: Vec<String> = self
            .marginals
            .iter()
            .map(|(tile, counts)| {
                format!(
                    "[{},{},{},{}]",
                    json_string(tile),
                    json_string(&counts[0]),
                    json_string(&counts[1]),
                    json_string(&counts[2])
                )
            })
            .collect();
        format!(
            "{{\"own\":{},\"pool\":{},\"hidden\":[{},{},{}],\"caps\":[{},{},{}],\"psizes\":[{},{},{}],\"fiber\":{},\"marginals\":[{}]}}",
            json_str_array(&self.own),
            json_str_array(&self.pool),
            self.hidden[0], self.hidden[1], self.hidden[2],
            self.caps[0], self.caps[1], self.caps[2],
            self.psizes[0], self.psizes[1], self.psizes[2],
            json_string(&self.fiber),
            marginals.join(",")
        )
    }
}

impl TraceDecision {
    fn to_json(&self) -> String {
        let views: Vec<String> = self.views.iter().map(TraceView::to_json).collect();
        let truth: Vec<String> = self.truth.iter().map(|h| json_str_array(h)).collect();
        let played: Vec<String> = self
            .public
            .played
            .iter()
            .map(|h| json_str_array(h))
            .collect();
        let voids: Vec<String> = self
            .public
            .voids
            .iter()
            .map(|h| json_str_array(h))
            .collect();
        let totals: Vec<String> = self.totals.iter().map(i64::to_string).collect();
        format!(
            "{{\"play\":{},\"trick\":{},\"pos\":{},\"actor\":{},\
             \"public\":{{\"leader\":{},\"trick\":{},\"points\":[{},{}],\"played\":[{}],\"voids\":[{}]}},\
             \"views\":[{}],\"truth\":[{}],\
             \"decision\":{{\"legal\":{},\"totals\":[{}],\"worlds\":{},\"avgs\":{},\"chosen\":{},\"forced\":{}}},\
             \"plan\":{},\
             \"after\":{{\"leader\":{},\"points\":[{},{}],\"trick_complete\":{}}}}}",
            self.play,
            self.trick,
            self.pos,
            self.actor,
            self.public.leader,
            json_pairs(&self.public.trick),
            self.public.points[0],
            self.public.points[1],
            played.join(","),
            voids.join(","),
            views.join(","),
            truth.join(","),
            json_str_array(&self.legal),
            totals.join(","),
            self.worlds,
            json_str_array(&self.avgs),
            json_string(&self.chosen),
            self.forced,
            match &self.plan {
                Some(book) => book.clone(),
                None => "null".to_string(),
            },
            self.after_leader,
            self.after_points[0],
            self.after_points[1],
            match self.trick_complete {
                Some((winner, points)) => format!("{{\"winner\":{winner},\"points\":{points}}}"),
                None => "null".to_string(),
            }
        )
    }
}

impl TraceHand {
    fn to_json(&self) -> String {
        let deal: Vec<String> = self.deal.iter().map(|h| json_str_array(h)).collect();
        let decisions: Vec<String> = self.decisions.iter().map(TraceDecision::to_json).collect();
        let result = match &self.result {
            None => "null".to_string(),
            Some(r) => format!(
                "{{\"declaring_team\":{},\"declaring_points\":{},\"made\":{},\"stake\":{},\"award_team\":{},\"marks_after\":[{},{}]}}",
                r.declaring_team, r.declaring_points, r.made, r.stake, r.award_team,
                r.marks_after[0], r.marks_after[1]
            ),
        };
        format!(
            "{{\"index\":{},\"shaker\":{},\"bidder\":{},\"declaration\":{},\"trump\":{},\"auction\":{},\
             \"auction_note\":{},\"deal\":[{}],\"decisions\":[{}],\"result\":{}}}",
            self.index,
            self.shaker,
            self.bidder,
            json_string(&self.declaration),
            json_str_array(&self.trump),
            json_pairs(&self.auction),
            json_string("placeholder heuristic (left of shaker bids P(30)); not a modeled policy"),
            deal.join(","),
            decisions.join(","),
            result
        )
    }
}

impl TraceDocument {
    /// Serialize the whole trace as deterministic JSON.
    pub fn to_json(&self) -> String {
        let hands: Vec<String> = self.hands.iter().map(TraceHand::to_json).collect();
        format!(
            "{{\"format\":\"rob-inspector-trace\",\"version\":1,\"normative\":false,\
             \"note\":{},\
             \"config\":{{\"max_mark_bid\":{},\"match_target\":{}}},\
             \"player\":{{\"worlds_per_decision\":{},\"lens\":{},\"seed\":{}}},\
             \"match_seed\":{},\"hands\":[{}],\"final_marks\":[{},{}]}}",
            json_string(
                "Deterministic regenerable diagnostic trace; display data only, not a verification receipt."
            ),
            self.config.0,
            self.config.1,
            self.player.0,
            json_string(&self.player.1),
            self.player.2,
            self.match_seed,
            hands.join(","),
            self.final_marks[0],
            self.final_marks[1]
        )
    }
}

/// Sanity helper used by tests: the number of decision records.
pub fn decision_count(document: &TraceDocument) -> usize {
    document.hands.iter().map(|h| h.decisions.len()).sum()
}

const _: () = {
    // HIDDEN_SEATS is the arity of every per-view triple above.
    assert!(HIDDEN_SEATS == 3);
};
