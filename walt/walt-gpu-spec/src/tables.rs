//! Canonical semantic table generation for the first GPU ABI.
//!
//! The table generator calls `walt-core` for every rule result.  The GPU is a
//! consumer of the resulting fixed-width byte stream; it does not reproduce
//! declaration, follow, rank, or winner logic itself.

use walt_core::{Context, Decl, Domino, Tier};

use crate::FIELD_SCALE;

pub const TABLE_FORMAT_VERSION: u16 = 2;
pub const TABLE_MAGIC: [u8; 8] = *b"W42GPU01";
pub const TABLE_HEADER_BYTES: usize = 18;
pub const MAX_LEGAL_ACTIONS: usize = 7;
pub const MAX_CHOOSE_N: usize = 21;

const DECL_COUNT: usize = Decl::COUNT;
const CONTEXT_COUNT: usize = Context::COUNT;
const DOMINO_COUNT: usize = Domino::COUNT;
const CHOOSE_DIM: usize = MAX_CHOOSE_N + 1;

/// Semantic tables held in ordinary Rust memory.
///
/// This structure is not a raw GPU ABI.  `canonical_bytes` is the sole
/// serialized contract: it writes each scalar field in a declared order and
/// byte order, so compiler padding and host container layout are irrelevant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemanticTables {
    context_masks: [[u32; CONTEXT_COUNT]; DECL_COUNT],
    lead_context_bits: [[u8; DOMINO_COUNT]; DECL_COUNT],
    ranks: [[u8; DOMINO_COUNT]; DECL_COUNT],
    trick_keys: [[[u16; DOMINO_COUNT]; CONTEXT_COUNT]; DECL_COUNT],
    beats_masks: [[[u32; DOMINO_COUNT]; CONTEXT_COUNT]; DECL_COUNT],
    count_values: [u8; DOMINO_COUNT],
    // Index zero represents one legal action.  Zero legal actions are not a
    // chance node and must never have a denominator-clearing multiplier.
    small_scales: [u16; MAX_LEGAL_ACTIONS],
    choose: [[u32; CHOOSE_DIM]; CHOOSE_DIM],
}

impl SemanticTables {
    /// Generates every semantic entry through the existing Rust authority.
    pub fn from_walt_core() -> SemanticTables {
        let mut context_masks = [[0u32; CONTEXT_COUNT]; DECL_COUNT];
        let mut lead_context_bits = [[0u8; DOMINO_COUNT]; DECL_COUNT];
        let mut ranks = [[0u8; DOMINO_COUNT]; DECL_COUNT];
        let mut trick_keys = [[[0u16; DOMINO_COUNT]; CONTEXT_COUNT]; DECL_COUNT];
        let mut beats_masks = [[[0u32; DOMINO_COUNT]; CONTEXT_COUNT]; DECL_COUNT];

        for (decl_index, decl) in Decl::ALL.into_iter().enumerate() {
            for (context_index, context) in Context::ALL.into_iter().enumerate() {
                context_masks[decl_index][context_index] = decl.effective_incidence(context).bits();
            }
            for (domino_index, domino) in Domino::ALL.into_iter().enumerate() {
                lead_context_bits[decl_index][domino_index] =
                    1u8 << decl.led_context(domino).index();
                ranks[decl_index][domino_index] = decl.rank(domino).value();
            }
            for (context_index, context) in Context::ALL.into_iter().enumerate() {
                for (domino_index, domino) in Domino::ALL.into_iter().enumerate() {
                    trick_keys[decl_index][context_index][domino_index] =
                        encode_trick_key(decl.trick_key(domino, context));
                    beats_masks[decl_index][context_index][domino_index] =
                        decl.beats(context, domino).bits();
                }
            }
        }

        let mut count_values = [0u8; DOMINO_COUNT];
        for (domino_index, domino) in Domino::ALL.into_iter().enumerate() {
            count_values[domino_index] =
                u8::try_from(domino.count()).expect("count decoration fits in u8");
        }

        let mut small_scales = [0u16; MAX_LEGAL_ACTIONS];
        for legal_actions in 1..=MAX_LEGAL_ACTIONS {
            let legal_actions = u16::try_from(legal_actions).expect("small legal-set bound");
            let scale = u16::try_from(FIELD_SCALE).expect("field scale fits in u16");
            assert_eq!(
                scale % legal_actions,
                0,
                "field scale must clear denominator"
            );
            small_scales[usize::from(legal_actions - 1)] = scale / legal_actions;
        }

        let mut choose = [[0u32; CHOOSE_DIM]; CHOOSE_DIM];
        for (n, row) in choose.iter_mut().enumerate() {
            for (k, value) in row.iter_mut().enumerate() {
                *value = choose_u32(
                    u8::try_from(n).expect("small choose n"),
                    u8::try_from(k).expect("small choose k"),
                );
            }
        }

        SemanticTables {
            context_masks,
            lead_context_bits,
            ranks,
            trick_keys,
            beats_masks,
            count_values,
            small_scales,
            choose,
        }
    }

    pub fn context_mask(&self, decl: Decl, context: Context) -> u32 {
        self.context_masks[decl_index(decl)][context.index()]
    }

    pub fn lead_context_bits(&self, decl: Decl, domino: Domino) -> u8 {
        self.lead_context_bits[decl_index(decl)][domino.index()]
    }

    pub fn rank(&self, decl: Decl, domino: Domino) -> u8 {
        self.ranks[decl_index(decl)][domino.index()]
    }

    pub fn trick_key_code(&self, decl: Decl, context: Context, domino: Domino) -> u16 {
        self.trick_keys[decl_index(decl)][context.index()][domino.index()]
    }

    pub fn beats_mask(&self, decl: Decl, context: Context, domino: Domino) -> u32 {
        self.beats_masks[decl_index(decl)][context.index()][domino.index()]
    }

    pub fn count_value(&self, domino: Domino) -> u8 {
        self.count_values[domino.index()]
    }

    pub fn small_scale(&self, legal_actions: u8) -> Option<u16> {
        legal_actions
            .checked_sub(1)
            .and_then(|index| self.small_scales.get(usize::from(index)))
            .copied()
    }

    pub fn choose(&self, n: u8, k: u8) -> Option<u32> {
        self.choose
            .get(usize::from(n))
            .and_then(|row| row.get(usize::from(k)))
            .copied()
    }

    /// The exact size of `canonical_bytes` for this format version.
    pub const fn canonical_byte_len() -> usize {
        TABLE_HEADER_BYTES
            + DECL_COUNT * CONTEXT_COUNT * core::mem::size_of::<u32>()
            + DECL_COUNT * DOMINO_COUNT * core::mem::size_of::<u8>()
            + DECL_COUNT * DOMINO_COUNT * core::mem::size_of::<u8>()
            + DECL_COUNT * CONTEXT_COUNT * DOMINO_COUNT * core::mem::size_of::<u16>()
            + DECL_COUNT * CONTEXT_COUNT * DOMINO_COUNT * core::mem::size_of::<u32>()
            + DOMINO_COUNT * core::mem::size_of::<u8>()
            + MAX_LEGAL_ACTIONS * core::mem::size_of::<u16>()
            + CHOOSE_DIM * CHOOSE_DIM * core::mem::size_of::<u32>()
    }

    /// Serializes fixed-width scalar tables in this exact order:
    ///
    /// 1. context masks `[declaration][context]` as little-endian `u32`;
    /// 2. lead-context bits `[declaration][domino]` as `u8`;
    /// 3. ranks `[declaration][domino]` as `u8`;
    /// 4. trick keys `[declaration][context][domino]` as little-endian `u16`;
    /// 5. beat masks in the same three-dimensional order as little-endian `u32`;
    /// 6. count values `[domino]` as `u8`;
    /// 7. small denominator-clearing scales `[1..=7]`, packed at index
    ///    `legal_actions - 1`, as little-endian `u16`;
    /// 8. choose values `[n][k]`, `0..=21`, as little-endian `u32`.
    ///
    /// The header is `magic`, format version, three cardinalities, one zero
    /// reserved byte, then payload byte length.  No Rust enum, boolean,
    /// pointer-sized integer, padding byte, or host endianness enters this
    /// serialized ABI.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut payload = Vec::with_capacity(Self::canonical_byte_len() - TABLE_HEADER_BYTES);

        for rows in &self.context_masks {
            for value in rows {
                push_u32(&mut payload, *value);
            }
        }
        for rows in &self.lead_context_bits {
            payload.extend_from_slice(rows);
        }
        for rows in &self.ranks {
            payload.extend_from_slice(rows);
        }
        for decl_rows in &self.trick_keys {
            for context_row in decl_rows {
                for value in context_row {
                    push_u16(&mut payload, *value);
                }
            }
        }
        for decl_rows in &self.beats_masks {
            for context_row in decl_rows {
                for value in context_row {
                    push_u32(&mut payload, *value);
                }
            }
        }
        payload.extend_from_slice(&self.count_values);
        for value in self.small_scales {
            push_u16(&mut payload, value);
        }
        for row in &self.choose {
            for value in row {
                push_u32(&mut payload, *value);
            }
        }

        assert_eq!(
            payload.len(),
            Self::canonical_byte_len() - TABLE_HEADER_BYTES,
            "canonical semantic table payload length drift"
        );

        let mut bytes = Vec::with_capacity(Self::canonical_byte_len());
        bytes.extend_from_slice(&TABLE_MAGIC);
        push_u16(&mut bytes, TABLE_FORMAT_VERSION);
        bytes.push(u8::try_from(DECL_COUNT).expect("fixed declaration count"));
        bytes.push(u8::try_from(CONTEXT_COUNT).expect("fixed context count"));
        bytes.push(u8::try_from(DOMINO_COUNT).expect("fixed domino count"));
        bytes.push(0);
        push_u32(
            &mut bytes,
            u32::try_from(payload.len()).expect("canonical payload length fits in u32"),
        );
        bytes.extend_from_slice(&payload);
        bytes
    }
}

fn decl_index(decl: Decl) -> usize {
    Decl::ALL
        .into_iter()
        .position(|candidate| candidate == decl)
        .expect("a declaration comes from Decl::ALL")
}

fn encode_trick_key(key: walt_core::TrickKey) -> u16 {
    let tier = match key.tier {
        Tier::Slough => 0u16,
        Tier::Follows => 1u16,
        Tier::Called => 2u16,
    };
    (tier << 8) | u16::from(key.rank.value())
}

fn choose_u32(n: u8, k: u8) -> u32 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut out = 1u64;
    for step in 0..k {
        out = out * u64::from(n - step) / u64::from(step + 1);
    }
    u32::try_from(out).expect("choose table value fits in u32 through n=21")
}

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}
