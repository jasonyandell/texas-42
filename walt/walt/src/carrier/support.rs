use crate::kernel::{Hidden, Kernel};
use crate::rules::{ContextSet, Domino, DominoSet, Seat, Trick};

use crate::carrier::profile::{support_digest, Digest};
use crate::carrier::{
    CarrierError, CarrierFacts, HIDDEN_SEAT_ORDER, PUBLIC_PREFIX_BYTES, PUBLIC_PREFIX_PAIR_COUNT,
    SUPPORT_COUNT, SUPPORT_RECORD_BYTES, SUPPORT_STREAM_DIGEST, VIEWER, VOID_FREE_PARENT_COUNT,
};

/// One canonical 16-byte support row: four little-endian hand masks in
/// absolute seat order S0, S1, S2, S3.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SupportRecord {
    hands: [DominoSet; Seat::COUNT],
}

impl SupportRecord {
    pub const fn hands(self) -> [DominoSet; Seat::COUNT] {
        self.hands
    }

    pub const fn hand(self, seat: Seat) -> DominoSet {
        self.hands[seat.index()]
    }

    pub fn to_bytes(self) -> [u8; SUPPORT_RECORD_BYTES] {
        let mut bytes = [0u8; SUPPORT_RECORD_BYTES];
        for (seat, hand) in self.hands.into_iter().enumerate() {
            let offset = seat * 4;
            bytes[offset..offset + 4].copy_from_slice(&hand.bits().to_le_bytes());
        }
        bytes
    }

    fn freeze_order_key(self) -> [u8; 12] {
        let mut key = [0u8; 12];
        let mut offset = 0usize;
        for seat in HIDDEN_SEAT_ORDER {
            for domino in self.hand(seat) {
                key[offset] =
                    u8::try_from(domino.index()).expect("a domino index always fits in one byte");
                offset += 1;
            }
        }
        debug_assert_eq!(offset, key.len());
        key
    }
}

/// The exact numeric-order support stream. The serialized records, not an
/// auxiliary index, are its sole stored authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CarrierSupport {
    records: Box<[SupportRecord]>,
}

impl CarrierSupport {
    fn new(records: Vec<SupportRecord>) -> Result<Self, CarrierError> {
        if records.len() != SUPPORT_COUNT {
            return Err(CarrierError::SupportCount {
                expected: SUPPORT_COUNT,
                actual: records.len(),
            });
        }
        Ok(Self {
            records: records.into_boxed_slice(),
        })
    }

    /// Parses, validates, and independently regenerates the exact frozen
    /// 1,200-record payload before admitting it.
    pub fn from_payload(payload: &[u8], facts: &CarrierFacts) -> Result<Self, CarrierError> {
        let expected_bytes = SUPPORT_COUNT * SUPPORT_RECORD_BYTES;
        if payload.len() != expected_bytes {
            return Err(CarrierError::SupportPayloadLength {
                expected: expected_bytes,
                actual: payload.len(),
            });
        }

        let mut records = Vec::with_capacity(SUPPORT_COUNT);
        for (ordinal, record_bytes) in payload.chunks_exact(SUPPORT_RECORD_BYTES).enumerate() {
            let mut hands = [DominoSet::EMPTY; Seat::COUNT];
            for (seat, hand) in hands.iter_mut().enumerate() {
                let offset = seat * 4;
                let bits =
                    u32::from_le_bytes(record_bytes[offset..offset + 4].try_into().map_err(
                        |_| CarrierError::InvalidSupport {
                            ordinal,
                            field: "four-byte hand mask",
                        },
                    )?);
                *hand = DominoSet::from_bits(bits).ok_or(CarrierError::InvalidSupport {
                    ordinal,
                    field: "low-28-bit hand mask",
                })?;
            }
            records.push(SupportRecord { hands });
        }

        let parsed = Self::new(records)?;
        parsed.validate(facts)?;
        let primary = primary_filtered_support(facts)?;
        let independent = independent_constrained_support(facts)?;
        primary.require_identical(&independent)?;
        parsed.require_identical(&primary)?;
        if parsed.digest() != SUPPORT_STREAM_DIGEST {
            return Err(CarrierError::FrozenFact("support stream digest"));
        }
        Ok(parsed)
    }

    pub const fn len(&self) -> usize {
        self.records.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub const fn records(&self) -> &[SupportRecord] {
        &self.records
    }

    pub fn payload(&self) -> Vec<u8> {
        let mut payload = Vec::with_capacity(self.records.len() * SUPPORT_RECORD_BYTES);
        for record in &self.records {
            payload.extend_from_slice(&record.to_bytes());
        }
        payload
    }

    pub fn digest(&self) -> Digest {
        support_digest(&self.payload()).expect("the frozen support width fits in u64")
    }

    pub(crate) fn require_identical(&self, other: &Self) -> Result<(), CarrierError> {
        let common = self.records.len().min(other.records.len());
        for ordinal in 0..common {
            if self.records[ordinal].to_bytes() != other.records[ordinal].to_bytes() {
                return Err(CarrierError::SupportMismatch { ordinal });
            }
        }
        if self.records.len() != other.records.len() {
            return Err(CarrierError::SupportMismatch { ordinal: common });
        }
        Ok(())
    }

    pub(crate) fn validate(&self, facts: &CarrierFacts) -> Result<(), CarrierError> {
        if self.records.len() != SUPPORT_COUNT {
            return Err(CarrierError::SupportCount {
                expected: SUPPORT_COUNT,
                actual: self.records.len(),
            });
        }

        let mut previous_key = None;
        for (ordinal, record) in self.records.iter().copied().enumerate() {
            validate_record(record, facts, ordinal)?;
            let key = record.freeze_order_key();
            if previous_key.is_some_and(|previous| previous >= key) {
                return Err(CarrierError::InvalidSupport {
                    ordinal,
                    field: "strict freeze order",
                });
            }
            previous_key = Some(key);
        }
        Ok(())
    }
}

/// Independent constrained construction. This does not call `Kernel`, its DP,
/// or `FiberIter`: it scans 12-bit subsets, applies independently replayed
/// void evidence, and sorts the admitted assignments into freeze order.
#[derive(Debug)]
pub struct ConstrainedSupportIter {
    inner: std::vec::IntoIter<SupportRecord>,
}

impl ConstrainedSupportIter {
    pub fn new(facts: &CarrierFacts) -> Result<Self, CarrierError> {
        let independent_voids = independent_public_prefix_voids(facts)?;
        if independent_voids != facts.voids {
            return Err(CarrierError::FrozenFact(
                "independent public-prefix void sets",
            ));
        }

        let pool_tiles: Vec<Domino> = facts.hidden_pool.iter().collect();
        if pool_tiles.len() != 12 {
            return Err(CarrierError::FrozenFact(
                "independent hidden-pool cardinality",
            ));
        }

        let mut records = Vec::with_capacity(SUPPORT_COUNT);
        let pattern_limit = 1u16 << pool_tiles.len();
        for first_pattern in 0u16..pattern_limit {
            if first_pattern.count_ones() != 4 {
                continue;
            }
            let s2 = subset_from_pattern(&pool_tiles, first_pattern);
            if !respects_voids(facts, Seat::S2, s2, &independent_voids) {
                continue;
            }

            for second_pattern in 0u16..pattern_limit {
                if second_pattern.count_ones() != 4 || first_pattern & second_pattern != 0 {
                    continue;
                }
                let s3 = subset_from_pattern(&pool_tiles, second_pattern);
                if !respects_voids(facts, Seat::S3, s3, &independent_voids) {
                    continue;
                }

                let assigned_pattern = first_pattern | second_pattern;
                let remainder_pattern = (pattern_limit - 1) ^ assigned_pattern;
                let s0 = subset_from_pattern(&pool_tiles, remainder_pattern);
                if s0.len() != 4 || !respects_voids(facts, Seat::S0, s0, &independent_voids) {
                    continue;
                }

                let mut hands = [DominoSet::EMPTY; Seat::COUNT];
                hands[Seat::S0.index()] = s0;
                hands[Seat::S1.index()] = facts.viewer_hand;
                hands[Seat::S2.index()] = s2;
                hands[Seat::S3.index()] = s3;
                records.push(SupportRecord { hands });
            }
        }

        records.sort_unstable_by_key(|record| record.freeze_order_key());
        if records.len() != SUPPORT_COUNT {
            return Err(CarrierError::SupportCount {
                expected: SUPPORT_COUNT,
                actual: records.len(),
            });
        }
        Ok(Self {
            inner: records.into_iter(),
        })
    }
}

impl Iterator for ConstrainedSupportIter {
    type Item = SupportRecord;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl ExactSizeIterator for ConstrainedSupportIter {}

pub(crate) fn primary_filtered_support(
    facts: &CarrierFacts,
) -> Result<CarrierSupport, CarrierError> {
    let hidden = HIDDEN_SEAT_ORDER.map(|seat| Hidden {
        seat,
        capacity: 4,
        voids: ContextSet::EMPTY,
    });
    let kernel = Kernel::new(
        facts.declaration,
        VIEWER,
        facts.viewer_hand,
        facts.hidden_pool,
        hidden,
    )
    .map_err(|_| CarrierError::FrozenFact("void-free parent kernel"))?;

    if kernel.unconstrained_count() != VOID_FREE_PARENT_COUNT as u128
        || kernel.count() != VOID_FREE_PARENT_COUNT as u128
    {
        return Err(CarrierError::ParentCount {
            expected: VOID_FREE_PARENT_COUNT,
            actual: usize::try_from(kernel.count()).unwrap_or(usize::MAX),
        });
    }

    let mut parent_count = 0usize;
    let mut records = Vec::with_capacity(SUPPORT_COUNT);
    for world in kernel.worlds() {
        parent_count = parent_count
            .checked_add(1)
            .ok_or(CarrierError::LengthOverflow("void-free parent count"))?;
        let record = SupportRecord {
            hands: world.hands(),
        };
        if hidden_hands_respect_voids(record, facts, &facts.voids) {
            records.push(record);
        }
    }
    if parent_count != VOID_FREE_PARENT_COUNT {
        return Err(CarrierError::ParentCount {
            expected: VOID_FREE_PARENT_COUNT,
            actual: parent_count,
        });
    }

    CarrierSupport::new(records)
}

pub(crate) fn independent_constrained_support(
    facts: &CarrierFacts,
) -> Result<CarrierSupport, CarrierError> {
    let support = CarrierSupport::new(ConstrainedSupportIter::new(facts)?.collect())?;
    support.validate(facts)?;
    Ok(support)
}

fn independent_public_prefix_voids(
    facts: &CarrierFacts,
) -> Result<[ContextSet; Seat::COUNT], CarrierError> {
    let mut voids = [ContextSet::EMPTY; Seat::COUNT];
    let mut seen = DominoSet::EMPTY;
    let mut leader = facts.bidder;
    let mut banked = [0u32; 2];

    for trick_ordinal in 0..3 {
        let byte_offset = trick_ordinal * 8;
        let mut dominoes = [Domino::ALL[0]; 4];
        for position in 0..4 {
            let actor = usize::from(PUBLIC_PREFIX_BYTES[byte_offset + position * 2]);
            let actor = Seat::from_index(actor)
                .ok_or(CarrierError::FrozenFact("independent prefix actor"))?;
            if actor != leader.plus(position) {
                return Err(CarrierError::FrozenFact("independent prefix actor order"));
            }

            let tile_index = usize::from(PUBLIC_PREFIX_BYTES[byte_offset + position * 2 + 1]);
            let domino = Domino::from_index(tile_index)
                .ok_or(CarrierError::FrozenFact("independent prefix tile"))?;
            if !seen.insert(domino) {
                return Err(CarrierError::FrozenFact("independent prefix repeated tile"));
            }
            dominoes[position] = domino;
        }

        let led = facts.declaration.led_context(dominoes[0]);
        for position in 1..4 {
            if !facts.declaration.follows(dominoes[position], led) {
                voids[leader.plus(position).index()].insert(led);
            }
        }

        let trick = Trick::new(leader, dominoes)
            .map_err(|_| CarrierError::FrozenFact("independent prefix trick"))?;
        let winner = trick.winner(facts.declaration);
        banked[winner.team().index()] = banked[winner.team().index()]
            .checked_add(trick.points())
            .ok_or(CarrierError::FrozenFact(
            "independent prefix banked-point overflow",
        ))?;
        leader = winner;
    }

    if PUBLIC_PREFIX_PAIR_COUNT != 12
        || leader != facts.next_leader
        || banked != facts.banked_points
    {
        return Err(CarrierError::FrozenFact(
            "independent public-prefix outcome",
        ));
    }
    Ok(voids)
}

fn subset_from_pattern(pool_tiles: &[Domino], pattern: u16) -> DominoSet {
    pool_tiles
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(bit, domino)| (pattern & (1u16 << bit) != 0).then_some(domino))
        .collect()
}

fn hidden_hands_respect_voids(
    record: SupportRecord,
    facts: &CarrierFacts,
    voids: &[ContextSet; Seat::COUNT],
) -> bool {
    HIDDEN_SEAT_ORDER
        .into_iter()
        .all(|seat| respects_voids(facts, seat, record.hand(seat), voids))
}

fn respects_voids(
    facts: &CarrierFacts,
    seat: Seat,
    hand: DominoSet,
    voids: &[ContextSet; Seat::COUNT],
) -> bool {
    voids[seat.index()]
        .iter()
        .all(|context| hand.is_disjoint(facts.declaration.effective_incidence(context)))
}

fn validate_record(
    record: SupportRecord,
    facts: &CarrierFacts,
    ordinal: usize,
) -> Result<(), CarrierError> {
    let mut union = DominoSet::EMPTY;
    for seat in Seat::ALL {
        let hand = record.hand(seat);
        if hand.len() != 4 {
            return Err(CarrierError::InvalidSupport {
                ordinal,
                field: "four tiles per seat",
            });
        }
        if !union.is_disjoint(hand) {
            return Err(CarrierError::InvalidSupport {
                ordinal,
                field: "pairwise-disjoint hands",
            });
        }
        union = union.union(hand);
    }
    if record.hand(VIEWER) != facts.viewer_hand {
        return Err(CarrierError::InvalidSupport {
            ordinal,
            field: "fixed viewer hand",
        });
    }
    if union != facts.live_tiles() {
        return Err(CarrierError::InvalidSupport {
            ordinal,
            field: "exact live-set partition",
        });
    }
    if !hidden_hands_respect_voids(record, facts, &facts.voids) {
        return Err(CarrierError::InvalidSupport {
            ordinal,
            field: "historical void feasibility",
        });
    }
    Ok(())
}
