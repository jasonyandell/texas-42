use walt_core::receipt;
use walt_core::replay::{replay_hand, state_before_trick, voids_before_trick};
use walt_core::{
    legal_plays, Context, ContextSet, Decl, Domino, DominoSet, Pip, Seat, Team, Trick,
};

use crate::{
    CarrierError, CARRIER_HAND_ID, CARRIER_TRICK, FUTURE_FIELD_MOVES, HAND_TOTAL_POINTS,
    HIDDEN_POOL_MASK, HIDDEN_SEAT_ORDER, LEGAL_ROOT_MASK, P30_INITIAL_T0_ALLOWANCE,
    P30_REMAINING_T0_ALLOWANCE, PUBLIC_PREFIX_BYTES_LEN, PUBLIC_PREFIX_PAIR_COUNT,
    UNBANKED_POINTS_BEFORE_TRICK4, VIEWER,
};

/// The twelve exact `(actor, domino-index)` pairs before trick four.
pub const PUBLIC_PREFIX_BYTES: [u8; PUBLIC_PREFIX_BYTES_LEN] = [
    1, 17, 2, 24, 3, 16, 0, 26, 0, 12, 1, 19, 2, 14, 3, 8, 1, 2, 2, 5, 3, 1, 0, 22,
];

/// Replayed semantic facts at the sole M3 carrier cut.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CarrierFacts {
    pub declaration: Decl,
    pub shaker: Seat,
    pub bidder: Seat,
    pub declaring_team: Team,
    pub viewer: Seat,
    pub next_leader: Seat,
    pub contract_points: u32,
    pub viewer_hand: DominoSet,
    pub hidden_pool: DominoSet,
    pub voids: [ContextSet; Seat::COUNT],
    pub banked_points: [u32; 2],
}

impl CarrierFacts {
    pub const fn live_tiles(&self) -> DominoSet {
        self.viewer_hand.union(self.hidden_pool)
    }

    pub const fn unbanked_points(&self) -> u32 {
        HAND_TOTAL_POINTS - self.banked_points[0] - self.banked_points[1]
    }

    pub const fn p30_initial_t0_allowance(&self) -> u32 {
        HAND_TOTAL_POINTS - self.contract_points
    }

    pub const fn p30_remaining_t0_allowance(&self) -> u32 {
        self.p30_initial_t0_allowance() - self.banked_points[Team::T0.index()]
    }

    pub const fn future_field_moves(&self) -> u32 {
        ((walt_core::replay::TRICKS_PER_HAND + 1 - CARRIER_TRICK) * 3) as u32
    }
}

pub(crate) fn primary_replay_facts(bytes: &[u8]) -> Result<CarrierFacts, CarrierError> {
    let text = core::str::from_utf8(bytes).map_err(|_| CarrierError::ReceiptUtf8)?;
    let parsed =
        receipt::parse(text).map_err(|error| CarrierError::ReceiptParse(error.to_string()))?;
    let hand = parsed
        .hands
        .iter()
        .find(|hand| hand.id == CARRIER_HAND_ID)
        .ok_or(CarrierError::MissingHand(CARRIER_HAND_ID))?;

    replay_hand(hand).map_err(|error| CarrierError::Replay(error.to_string()))?;

    let p5 = Decl::PipTrump(Pip::new(5).expect("five is a canonical pip"));
    require(hand.shaker == Seat::S0, "shaker")?;
    require(hand.bidder == Seat::S1, "bidder")?;
    require(hand.bid_points == 30, "contract points")?;
    require(hand.decl == p5, "declaration")?;
    require(hand.declaring_team == Team::T1, "declaring team")?;
    require(hand.tricks.len() == 7, "complete hand trick count")?;

    let mut prefix = [0u8; PUBLIC_PREFIX_BYTES_LEN];
    let mut prefix_at = 0usize;
    let mut banked = [0u32; 2];
    for (trick_index, trick) in hand.tricks.iter().take(CARRIER_TRICK - 1).enumerate() {
        require(trick.number == trick_index + 1, "prefix trick number")?;
        for (seat, domino) in trick.plays {
            prefix[prefix_at] = u8::try_from(seat.index())
                .map_err(|_| CarrierError::FrozenFact("prefix actor width"))?;
            prefix[prefix_at + 1] = u8::try_from(domino.index())
                .map_err(|_| CarrierError::FrozenFact("prefix domino width"))?;
            prefix_at += 2;
        }
        banked[trick.winner.team().index()] = banked[trick.winner.team().index()]
            .checked_add(trick.points)
            .ok_or(CarrierError::FrozenFact("prefix point overflow"))?;
    }
    require(
        prefix_at == PUBLIC_PREFIX_PAIR_COUNT * 2,
        "public-prefix width",
    )?;
    require(prefix == PUBLIC_PREFIX_BYTES, "public-prefix bytes")?;

    let expected_winners = [Seat::S0, Seat::S1, Seat::S1];
    let expected_points = [1u32, 6, 1];
    for (index, trick) in hand.tricks.iter().take(3).enumerate() {
        require(trick.winner == expected_winners[index], "prefix winner")?;
        require(trick.points == expected_points[index], "prefix points")?;
    }
    require(banked == [1, 7], "banked points")?;

    let (hands, next_leader) = state_before_trick(hand, CARRIER_TRICK)
        .map_err(|error| CarrierError::Replay(error.to_string()))?;
    let viewer_hand = DominoSet::from_bits(LEGAL_ROOT_MASK)
        .ok_or(CarrierError::FrozenFact("viewer-hand mask"))?;
    require(next_leader == Seat::S1, "next leader")?;
    require(hands[VIEWER.index()] == viewer_hand, "viewer hand")?;

    let hidden_pool = Seat::ALL
        .into_iter()
        .filter(|seat| *seat != VIEWER)
        .fold(DominoSet::EMPTY, |pool, seat| {
            pool.union(hands[seat.index()])
        });
    require(hidden_pool.bits() == HIDDEN_POOL_MASK, "hidden-pool mask")?;
    require(viewer_hand.len() == 4, "viewer hand size")?;
    require(hidden_pool.len() == 12, "hidden pool size")?;
    require(
        viewer_hand.is_disjoint(hidden_pool),
        "live-set disjointness",
    )?;

    let voids = voids_before_trick(hand, CARRIER_TRICK);
    let natural_1 = Context::Natural(Pip::new(1).expect("one is a canonical pip"));
    let natural_4 = Context::Natural(Pip::new(4).expect("four is a canonical pip"));
    require(voids[Seat::S0.index()].is_empty(), "S0 void set")?;
    require(
        voids[Seat::S1.index()] == ContextSet::single(natural_4),
        "S1 void set",
    )?;
    require(
        voids[Seat::S2.index()]
            == ContextSet::single(Context::Called).union(ContextSet::single(natural_1)),
        "S2 void set",
    )?;
    require(
        voids[Seat::S3.index()] == ContextSet::single(natural_4),
        "S3 void set",
    )?;

    let facts = CarrierFacts {
        declaration: p5,
        shaker: hand.shaker,
        bidder: hand.bidder,
        declaring_team: hand.declaring_team,
        viewer: VIEWER,
        next_leader,
        contract_points: hand.bid_points,
        viewer_hand,
        hidden_pool,
        voids,
        banked_points: banked,
    };
    require(
        facts.unbanked_points() == UNBANKED_POINTS_BEFORE_TRICK4,
        "unbanked points",
    )?;
    require(
        facts.p30_initial_t0_allowance() == P30_INITIAL_T0_ALLOWANCE,
        "P30 initial T0 allowance",
    )?;
    require(
        facts.p30_remaining_t0_allowance() == P30_REMAINING_T0_ALLOWANCE,
        "P30 remaining T0 allowance",
    )?;
    require(
        facts.future_field_moves() == FUTURE_FIELD_MOVES,
        "future field moves",
    )?;
    independent_replay_check(bytes, &facts, hands)?;
    Ok(facts)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct IndependentTrick {
    number: usize,
    plays: [(Seat, Domino); 4],
    recorded_winner: Seat,
    recorded_points: u32,
}

/// A deliberately separate, receipt-module-free reconstruction of the h8
/// coordinate. It parses only the hand-eight header and seven physical play
/// rows, derives the deal from those physical tiles, and replays the prefix
/// against the shared game rules. No `receipt`, `replay_hand`, `deal`,
/// `state_before_trick`, or `voids_before_trick` code is called here.
fn independent_replay_check(
    bytes: &[u8],
    facts: &CarrierFacts,
    primary_hands: [DominoSet; Seat::COUNT],
) -> Result<(), CarrierError> {
    let text = core::str::from_utf8(bytes).map_err(|_| CarrierError::ReceiptUtf8)?;
    let mut header = None;
    let mut tricks = Vec::with_capacity(7);
    let mut in_hand_eight = false;

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if let Some(rest) = line.strip_prefix("hand 8: ") {
            if header.is_some() {
                return Err(CarrierError::FrozenFact(
                    "independent duplicate hand-eight header",
                ));
            }
            header = Some(parse_independent_header(rest)?);
            in_hand_eight = true;
            continue;
        }
        if in_hand_eight && line.starts_with("hand ") {
            break;
        }
        if in_hand_eight {
            if let Some(rest) = line.strip_prefix("trick ") {
                tricks.push(parse_independent_trick(rest)?);
            }
        }
    }

    let (shaker, bidder, contract_points, declaration) = header.ok_or(CarrierError::FrozenFact(
        "independent missing hand-eight header",
    ))?;
    require(shaker == facts.shaker, "independent shaker")?;
    require(bidder == facts.bidder, "independent bidder")?;
    require(
        contract_points == facts.contract_points,
        "independent contract points",
    )?;
    require(declaration == facts.declaration, "independent declaration")?;
    require(tricks.len() == 7, "independent complete trick count")?;

    let mut deal = [DominoSet::EMPTY; Seat::COUNT];
    for trick in &tricks {
        for (seat, domino) in trick.plays {
            if !deal[seat.index()].insert(domino) {
                return Err(CarrierError::FrozenFact("independent repeated dealt tile"));
            }
        }
    }
    let mut complete_deal = DominoSet::EMPTY;
    for seat in Seat::ALL {
        require(
            deal[seat.index()].len() == 7,
            "independent initial hand size",
        )?;
        require(
            complete_deal.is_disjoint(deal[seat.index()]),
            "independent disjoint deal",
        )?;
        complete_deal = complete_deal.union(deal[seat.index()]);
    }
    require(
        complete_deal == DominoSet::FULL,
        "independent complete deal",
    )?;

    let mut current_hands = deal;
    let mut leader = bidder;
    let mut voids = [ContextSet::EMPTY; Seat::COUNT];
    let mut banked = [0u32; 2];
    let mut prefix = [0u8; PUBLIC_PREFIX_BYTES_LEN];
    let expected_winners = [Seat::S0, Seat::S1, Seat::S1];
    let expected_points = [1u32, 6, 1];

    for (trick_ordinal, recorded) in tricks.iter().take(3).copied().enumerate() {
        require(
            recorded.number == trick_ordinal + 1,
            "independent prefix trick number",
        )?;
        let led = declaration.led_context(recorded.plays[0].1);
        let mut dominoes = [Domino::ALL[0]; 4];
        for (position, (actor, domino)) in recorded.plays.into_iter().enumerate() {
            require(
                actor == leader.plus(position),
                "independent prefix actor order",
            )?;
            let held = current_hands[actor.index()];
            require(held.contains(domino), "independent prefix physical holding")?;
            let legal = legal_plays(declaration, held, (position != 0).then_some(led));
            require(legal.contains(domino), "independent prefix follow legality")?;
            if position != 0 && !declaration.follows(domino, led) {
                voids[actor.index()].insert(led);
            }
            require(
                current_hands[actor.index()].remove(domino),
                "independent prefix hand removal",
            )?;
            dominoes[position] = domino;

            let prefix_offset = trick_ordinal * 8 + position * 2;
            prefix[prefix_offset] = u8::try_from(actor.index())
                .map_err(|_| CarrierError::FrozenFact("independent actor width"))?;
            prefix[prefix_offset + 1] = u8::try_from(domino.index())
                .map_err(|_| CarrierError::FrozenFact("independent tile width"))?;
        }

        let trick = Trick::new(leader, dominoes)
            .map_err(|_| CarrierError::FrozenFact("independent repeated trick tile"))?;
        let winner = trick.winner(declaration);
        let points = trick.points();
        require(
            winner == recorded.recorded_winner,
            "independent recorded winner",
        )?;
        require(
            points == recorded.recorded_points,
            "independent recorded points",
        )?;
        require(
            winner == expected_winners[trick_ordinal],
            "independent frozen winner",
        )?;
        require(
            points == expected_points[trick_ordinal],
            "independent frozen points",
        )?;
        banked[winner.team().index()] =
            banked[winner.team().index()]
                .checked_add(points)
                .ok_or(CarrierError::FrozenFact(
                    "independent banked-point overflow",
                ))?;
        leader = winner;
    }

    require(prefix == PUBLIC_PREFIX_BYTES, "independent prefix bytes")?;
    require(current_hands == primary_hands, "independent current hands")?;
    require(
        current_hands[VIEWER.index()] == facts.viewer_hand,
        "independent viewer hand",
    )?;
    let independent_pool = HIDDEN_SEAT_ORDER
        .into_iter()
        .fold(DominoSet::EMPTY, |pool, seat| {
            pool.union(current_hands[seat.index()])
        });
    require(
        independent_pool == facts.hidden_pool,
        "independent hidden pool",
    )?;
    require(leader == facts.next_leader, "independent next leader")?;
    require(voids == facts.voids, "independent void sets")?;
    require(banked == facts.banked_points, "independent banked points")?;
    Ok(())
}

fn parse_independent_header(rest: &str) -> Result<(Seat, Seat, u32, Decl), CarrierError> {
    let mut shaker = None;
    let mut bidder = None;
    let mut contract_points = None;
    let mut declaration = None;
    for field in rest.split(", ") {
        if let Some(value) = field.strip_prefix("shaker ") {
            shaker = parse_independent_seat(value);
        } else if let Some(value) = field.strip_prefix("bidder ") {
            bidder = parse_independent_seat(value);
        } else if let Some(value) = field
            .strip_prefix("bid P(")
            .and_then(|value| value.strip_suffix(')'))
        {
            contract_points = value.parse::<u32>().ok();
        } else if let Some(value) = field.strip_prefix("declaration P") {
            declaration = value
                .parse::<u8>()
                .ok()
                .and_then(Pip::new)
                .map(Decl::PipTrump);
        } else {
            return Err(CarrierError::FrozenFact("independent hand-header field"));
        }
    }
    Ok((
        shaker.ok_or(CarrierError::FrozenFact("independent header shaker"))?,
        bidder.ok_or(CarrierError::FrozenFact("independent header bidder"))?,
        contract_points.ok_or(CarrierError::FrozenFact("independent header bid"))?,
        declaration.ok_or(CarrierError::FrozenFact("independent header declaration"))?,
    ))
}

fn parse_independent_trick(rest: &str) -> Result<IndependentTrick, CarrierError> {
    let (number, tail) = rest
        .split_once(": ")
        .ok_or(CarrierError::FrozenFact("independent trick separator"))?;
    let number = number
        .parse::<usize>()
        .map_err(|_| CarrierError::FrozenFact("independent trick number"))?;
    let (plays_text, result_text) = tail
        .split_once(" -> ")
        .ok_or(CarrierError::FrozenFact("independent outcome separator"))?;
    let mut plays = [(Seat::S0, Domino::ALL[0]); 4];
    let mut play_count = 0usize;
    for (slot, token) in plays.iter_mut().zip(plays_text.split_whitespace()) {
        *slot = parse_independent_play(token)?;
        play_count += 1;
    }
    require(
        play_count == 4 && plays_text.split_whitespace().count() == 4,
        "independent trick play count",
    )?;

    let mut result = result_text.split_whitespace();
    let recorded_winner = result
        .next()
        .and_then(parse_independent_seat)
        .ok_or(CarrierError::FrozenFact("independent outcome winner"))?;
    let recorded_points = result
        .next()
        .and_then(|points| points.strip_prefix('+'))
        .and_then(|points| points.parse::<u32>().ok())
        .ok_or(CarrierError::FrozenFact("independent outcome points"))?;
    require(result.next().is_none(), "independent outcome width")?;
    Ok(IndependentTrick {
        number,
        plays,
        recorded_winner,
        recorded_points,
    })
}

fn parse_independent_play(token: &str) -> Result<(Seat, Domino), CarrierError> {
    let (seat, tile) = token
        .split_once(':')
        .ok_or(CarrierError::FrozenFact("independent play separator"))?;
    let seat =
        parse_independent_seat(seat).ok_or(CarrierError::FrozenFact("independent play seat"))?;
    let (high, low) = tile
        .split_once('-')
        .ok_or(CarrierError::FrozenFact("independent tile separator"))?;
    let high = high
        .parse::<u8>()
        .ok()
        .and_then(Pip::new)
        .ok_or(CarrierError::FrozenFact("independent tile high pip"))?;
    let low = low
        .parse::<u8>()
        .ok()
        .and_then(Pip::new)
        .ok_or(CarrierError::FrozenFact("independent tile low pip"))?;
    require(
        high.value() >= low.value(),
        "independent canonical tile orientation",
    )?;
    Ok((seat, Domino::new(high, low)))
}

fn parse_independent_seat(token: &str) -> Option<Seat> {
    let index = token.strip_prefix('S')?.parse::<usize>().ok()?;
    Seat::from_index(index)
}

fn require(condition: bool, field: &'static str) -> Result<(), CarrierError> {
    if condition {
        Ok(())
    } else {
        Err(CarrierError::FrozenFact(field))
    }
}
