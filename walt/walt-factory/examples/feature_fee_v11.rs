//! walt feature-fee audition, THE CORRECTED RE-RUN — freeze 52 v1.1, F2 only,
//! with the shared-theta fit of FF-A15(iii) in the same pass. EXPLORATORY TIER
//! THROUGHOUT, without exception.
//!
//! Rulings: FF-A1..FF-A16 (`walt/CENSUS-RULINGS.md`, 2026-08-14), with
//! Proposition FF-blind, Lemma FF-min, Proposition FF-oracle and Proposition
//! FF-degen; **freeze 52 v1.1** (FF-A15(i)). This is the build commissioned at
//! FF-A15(ii) and FF-A15(iii).
//!
//! WHY IT EXISTS. The first FF run was adjudicated with NO DEVIATION FOUND
//! against its contract, but the adjudicator found two defects in its own
//! spec. FF-A11: FF-A4's no-outstanding-trump fallback was attached to all
//! three features, yet only F0 and F1 reference the boss-trump holder
//! `h(omega)` — so it silently voided F2 at every h2 state and at h0's 758
//! following states, six of twelve cells, including the one that mattered.
//! FF-A12: F2's definition was ambiguous at following states, where a tile
//! already on the table may already beat `b`.
//!
//! WHAT CHANGES HERE, and nothing else does:
//!   * freeze 52 v1.1's DOMAIN CLAUSE — the fallback is scoped to features
//!     that reference `h(omega)` (F0 and F1 only), and every feature carries
//!     its own domain clause. F2's domain is the whole fiber.
//!   * F2 AMENDED — `phi = 1` iff some opponent YET TO PLAY at `I` holds a
//!     tile that would win over `b` AND over every tile already on the table.
//!     At leading states this is identical to the frozen reading, so h0's
//!     leading measurement stands; that invariance is ASSERTED under (FF-R5)
//!     against the filed value rather than taken on trust.
//!   * F2 ONLY is reported. F0 is still run FIRST AND BLOCKING as the null
//!     control, because the control is what makes the new numbers
//!     trustworthy. F1 is settled at FF-A13 and is not computed.
//!   * THE SHARED-THETA FIT of FF-A15(iii) over h0's leading part, whose
//!     ratio to the oracle-theta capture is the number the programme wants.
//!
//! Emission goes to a NEW results file: the adjudicated artifact carrying the
//! voided measurements stays on disk unaltered, per LD-A11(ii)'s "the
//! corrected text stays visible and the error is not erased".
//!
//! No floats. Regenerate:
//! `cargo run --release -p walt-factory --example feature_fee_v11`

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

use walt_core::receipt::{locate_verify_player, parse_file, Receipt, ReceiptHand};
use walt_core::replay::state_before_trick;
use walt_core::{
    legal_plays, Context, ContextSet, Decl, Domino, DominoSet, Pip, Seat, Team, Trick,
};
use walt_geom::{q, qi, Q};
use walt_kernel::{Hidden, Kernel, HIDDEN_SEATS};

// -- freezes and declared constants -----------------------------------------

/// Freeze 44(b) v2, unchanged by FF-A6(h): B walk-steps per (coordinate,
/// action) for the walk-based evaluator. On exhaustion, `None` and NO PARTIAL
/// FOLD — no partial capture and no partial residual.
const B_WALK: u64 = 10_000_000_000;

/// Freeze 44(b) v2's partition-state cap, asserted against the rung-one
/// frontier partition; the assertion is contentful.
const P_MAX: u64 = 192_000_000;

const N4_TRICK: usize = 4;
const N4_GRADE: usize = 4;
const N4_FIBER: u128 = 34_650;

/// Freeze 52(g): the freeze-set digest travelling on every record. No cache is
/// read or written, so freeze 41/DS-A30's discard rule is vacuous here and is
/// stated rather than exercised.
const FF_DIGEST: &str = "FF-v1.1|freezes-7-23-26-37d-38v1.1-44v2-45-50v1.1-52v1.3|contract=R-A11-full-record|field=uniform-legal-F4|belief=uniform-fiber-freeze7";

/// `12^12`. The deep solve's fixed scale.
const SCALE: i64 = 8_916_100_448_256;

/// `12^6`. The common denominator of every pre-frontier arrival weight.
const DEN_MU: i128 = 2_985_984;

/// The maximum record length this probe encodes.
const REC_MAX: usize = 16;

/// The three frozen features of FF-A4, in the frozen order. F0 runs FIRST and
/// BLOCKING (FF-A7(i)).
/// FF-A15(ii) reports **F2 only**: F0 and F1 are settled — F0 by theorem
/// (Proposition FF-blind) and F1 by FF-A13 — but **F0 is still re-run FIRST
/// AND BLOCKING purely as the null control, because the control is what makes
/// the new numbers trustworthy**. F1 is not computed at all.
const N_FEAT: usize = 2;
const FEAT_NAME: [&str; N_FEAT] = [
    "F0 boss_owner",
    "F2 b_is_beatable (AMENDED, freeze 52 v1.1)",
];
const FEAT_KIND: [&str; N_FEAT] = [
    "NULL CONTROL, action-blind, re-run first and blocking — Proposition FF-blind predicts oracle-theta capture EXACTLY 0 and Proposition FF-degen predicts ZERO breakpoints",
    "THE MEASUREMENT THIS PASS EXISTS FOR, action-conditioned, from the existing rule algebra, domain = the whole fiber",
];

// -- the frozen source tables (SEP-A14(ii), FT-A28(i)) ----------------------

/// One filed root-action row: `(hi, lo, Q^H as (num, den), U as (num, den),
/// revealed walk-steps)`, the two values in the COUNT convention.
type FiledRow = (u8, u8, (i128, i128), (i128, i128), u64);

/// One filed carrier coordinate: `(corpus hand id, declaration pip, the four
/// root-action rows in ascending domino index)`.
type FiledCoord = (usize, u8, [FiledRow; 4]);

/// FREEZE 52(a): the carrier, enumerated with NO GENERATING RULE (FT-A23: a
/// freeze is a constant, not a rule). ARM 1 — h0, pip 3, hand `[00 21 32 53]`,
/// the single unit `a = 00`. ARM 2, attempted after arm 1 completes with a
/// declared stop — h2, pip 5, hand `[21 33 53 54]`, units `a = 53` then
/// `a = 54`. h6, h9 and h12 are OUT OF SCOPE (FF-A5).
///
/// FF-A5's factual correction, carried here: **h0 has ONE freeze-50 unit, not
/// two** — it is the untied coordinate with a single binding pair, competitor
/// `a = 00` against `a* = 53`.
///
/// Quoted from `walt-factory/results/separation_n4_2026-08-14.txt`,
/// exploratory tier; carried as a frozen table and NEVER re-parsed at run time.
const FF_FILED: [FiledCoord; 2] = [
    (
        0,
        3,
        [
            (
                0,
                0,
                (301_653_329, 89_812_800),
                (7_580_063, 2_138_400),
                539_583_224,
            ),
            (2, 1, (164_419, 49_896), (164_419, 49_896), 772_577_200),
            (
                3,
                2,
                (83_974_837, 29_937_600),
                (16_266_721, 5_702_400),
                1_118_641_032,
            ),
            (5, 3, (33_701, 9_900), (1_592_399, 453_600), 1_296_923_400),
        ],
    ),
    (
        2,
        5,
        [
            (2, 1, (9_448, 2_835), (9_448, 2_835), 781_356_124),
            (
                3,
                3,
                (911_507, 249_480),
                (6_106_181, 1_663_200),
                543_418_716,
            ),
            (5, 3, (85_117, 23_100), (58_639, 15_840), 1_297_073_736),
            (5, 4, (85_117, 23_100), (58_639, 15_840), 1_297_073_736),
        ],
    ),
];

/// One unit's rung-one filed record for (FF-R5): `(corpus hand id, a hi, a lo,
/// frontier states, (state, world) arrivals, states with delta_I = 0, states
/// with delta_I > 0, Delta^(1), U^(1), U^(0), walk-steps, the (FT-R7c)
/// frontier digest)`. The three rationals are count convention.
///
/// Quoted from `walt-factory/results/fusion_tax_2026-08-14.txt` and, for the
/// two h2 digests, from `SR_FIRST` in `second_rung.rs` as transcribed and
/// ASSERTED there across two processes. Exploratory tier; never re-parsed.
///
/// The digest is a SHA-256 over the canonical serialisation of the
/// `(record, delta_I)` pairs in freeze-38(d) order — one line per state,
/// `<record>|<delta_I as num/den, count convention>` and a newline. This probe
/// regenerates a rung-one frontier, so FT-A28(iv)'s (FT-R7c) applies: the h2
/// digests are ASSERTED against the transcribed values, and h0's slot is empty
/// so this run EMITS AND FILES it with (FT-R7a)'s corrected scope line.
type FfFirstUnit = (
    usize,
    u8,
    u8,
    u64,
    u64,
    u64,
    u64,
    (i128, i128),
    (i128, i128),
    (i128, i128),
    u64,
    &'static str,
);
const FF_FIRST: [FfFirstUnit; 3] = [
    (
        0,
        0,
        0,
        16_136,
        536_520,
        14_804,
        1_332,
        (19_863_799, 179_625_600),
        (616_861_493, 179_625_600),
        (7_580_063, 2_138_400),
        539_583_224,
        "679775f42213a11e0270092332bd306db31b4a6de8e042fe441e3e4dfe4bc1ab",
    ),
    (
        2,
        5,
        3,
        330,
        554_400,
        114,
        216,
        (145, 22_176),
        (102_437, 27_720),
        (58_639, 15_840),
        1_297_073_736,
        "bcd7e915db4cb39e092bd4597e8b9ceb911d2b632caa0a3bb9b385a6b1471106",
    ),
    (
        2,
        5,
        4,
        330,
        554_400,
        114,
        216,
        (145, 22_176),
        (102_437, 27_720),
        (58_639, 15_840),
        1_297_073_736,
        "de460262778878a72cb84cc4959f12a04f1652df4b1b7a846f3ea00a64d244fa",
    ),
];

// -- small helpers -----------------------------------------------------------

fn tile(d: Domino) -> String {
    format!("{}{}", d.hi().value(), d.lo().value())
}

fn tiles_str(s: DominoSet) -> String {
    s.iter().map(tile).collect::<Vec<_>>().join(" ")
}

fn record_str(r: &[Domino]) -> String {
    r.iter().map(|d| tile(*d)).collect::<Vec<_>>().join(" ")
}

fn qs(x: Q) -> String {
    format!("{}/{}", x.numer(), x.denom())
}

fn bqs(x: &BigRational) -> String {
    format!("{}/{}", x.numer(), x.denom())
}

fn yn(b: bool) -> &'static str {
    // SR-A27(iv), binding on the next emitter: a column read by eye must not
    // change case with its value.
    if b {
        "yes"
    } else {
        "no"
    }
}

/// The freeze-26 bridge at the reporting boundary, as a function of the
/// coordinate's DECLARED grade (N4-A11): `count = (diff + grade)/2`.
fn to_count(diff: Q, grade: usize) -> Q {
    (diff + qi(i128::try_from(grade).expect("grade fits"))) * q(1, 2)
}

/// Corollary SR-conv, the DIFFERENCE half: a tax is a difference of two
/// quantities at a common state, so it is exactly TWICE its count value.
fn tax_to_count(diff_tax: Q) -> Q {
    diff_tax * q(1, 2)
}

fn big(n: i128) -> BigInt {
    BigInt::from(n)
}

/// An exact parts-per-million bracket on a nonnegative rational, by integer
/// arithmetic only (P-A19: no float anywhere). PRESENTATION ONLY — it enters
/// no proof, no receipt and no comparison; the exact rational beside it is the
/// value.
fn ppm_bracket(x: &BigRational) -> (BigInt, BigInt) {
    let scaled = x * BigRational::from_integer(BigInt::from(1_000_000));
    let floor = scaled.floor().to_integer();
    let ceil = scaled.ceil().to_integer();
    (floor, ceil)
}

// -- SHA-256 (FIPS 180-4), exact integer arithmetic, no dependency ----------
// Carried forward from `second_rung.rs` with SR-A33's repair already in it:
// `update` must not clobber the buffered length across calls, and `finish`
// computes its pad length rather than searching for it.

const SHA_K: [u32; 64] = [
    0x428a_2f98,
    0x7137_4491,
    0xb5c0_fbcf,
    0xe9b5_dba5,
    0x3956_c25b,
    0x59f1_11f1,
    0x923f_82a4,
    0xab1c_5ed5,
    0xd807_aa98,
    0x1283_5b01,
    0x2431_85be,
    0x550c_7dc3,
    0x72be_5d74,
    0x80de_b1fe,
    0x9bdc_06a7,
    0xc19b_f174,
    0xe49b_69c1,
    0xefbe_4786,
    0x0fc1_9dc6,
    0x240c_a1cc,
    0x2de9_2c6f,
    0x4a74_84aa,
    0x5cb0_a9dc,
    0x76f9_88da,
    0x983e_5152,
    0xa831_c66d,
    0xb003_27c8,
    0xbf59_7fc7,
    0xc6e0_0bf3,
    0xd5a7_9147,
    0x06ca_6351,
    0x1429_2967,
    0x27b7_0a85,
    0x2e1b_2138,
    0x4d2c_6dfc,
    0x5338_0d13,
    0x650a_7354,
    0x766a_0abb,
    0x81c2_c92e,
    0x9272_2c85,
    0xa2bf_e8a1,
    0xa81a_664b,
    0xc24b_8b70,
    0xc76c_51a3,
    0xd192_e819,
    0xd699_0624,
    0xf40e_3585,
    0x106a_a070,
    0x19a4_c116,
    0x1e37_6c08,
    0x2748_774c,
    0x34b0_bcb5,
    0x391c_0cb3,
    0x4ed8_aa4a,
    0x5b9c_ca4f,
    0x682e_6ff3,
    0x748f_82ee,
    0x78a5_636f,
    0x84c8_7814,
    0x8cc7_0208,
    0x90be_fffa,
    0xa450_6ceb,
    0xbef9_a3f7,
    0xc671_78f2,
];

struct Sha256 {
    h: [u32; 8],
    buf: [u8; 64],
    buf_len: usize,
    total: u64,
}

impl Sha256 {
    fn new() -> Sha256 {
        Sha256 {
            h: [
                0x6a09_e667,
                0xbb67_ae85,
                0x3c6e_f372,
                0xa54f_f53a,
                0x510e_527f,
                0x9b05_688c,
                0x1f83_d9ab,
                0x5be0_cd19,
            ],
            buf: [0u8; 64],
            buf_len: 0,
            total: 0,
        }
    }

    fn block(&mut self, chunk: &[u8]) {
        let mut w = [0u32; 64];
        for (i, word) in chunk.chunks_exact(4).enumerate() {
            w[i] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let mut v = self.h;
        for i in 0..64 {
            let s1 = v[4].rotate_right(6) ^ v[4].rotate_right(11) ^ v[4].rotate_right(25);
            let ch = (v[4] & v[5]) ^ ((!v[4]) & v[6]);
            let t1 = v[7]
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(SHA_K[i])
                .wrapping_add(w[i]);
            let s0 = v[0].rotate_right(2) ^ v[0].rotate_right(13) ^ v[0].rotate_right(22);
            let maj = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let t2 = s0.wrapping_add(maj);
            v[7] = v[6];
            v[6] = v[5];
            v[5] = v[4];
            v[4] = v[3].wrapping_add(t1);
            v[3] = v[2];
            v[2] = v[1];
            v[1] = v[0];
            v[0] = t1.wrapping_add(t2);
        }
        for (slot, add) in self.h.iter_mut().zip(v) {
            *slot = slot.wrapping_add(add);
        }
    }

    fn update(&mut self, data: &[u8]) {
        self.total = self.total.wrapping_add(data.len() as u64);
        let mut rest = data;
        if self.buf_len > 0 {
            let take = (64 - self.buf_len).min(rest.len());
            self.buf[self.buf_len..self.buf_len + take].copy_from_slice(&rest[..take]);
            self.buf_len += take;
            rest = &rest[take..];
            if self.buf_len < 64 {
                // `take` consumed all of `data`; returning here is
                // load-bearing (SR-A33), because falling through would
                // overwrite `buf_len` with the empty remainder below and
                // silently discard the buffered bytes.
                return;
            }
            let chunk = self.buf;
            self.block(&chunk);
            self.buf_len = 0;
        }
        let mut it = rest.chunks_exact(64);
        for chunk in it.by_ref() {
            self.block(chunk);
        }
        let tail = it.remainder();
        self.buf[..tail.len()].copy_from_slice(tail);
        self.buf_len = tail.len();
    }

    fn finish(mut self) -> String {
        let bitlen = self.total.wrapping_mul(8);
        self.update(&[0x80]);
        let zeros = [0u8; 64];
        let pad = (56 + 64 - self.buf_len) % 64;
        self.update(&zeros[..pad]);
        assert_eq!(
            self.buf_len, 56,
            "SHA-256 padding must leave exactly the 8-byte length field"
        );
        let mut chunk = self.buf;
        chunk[56..64].copy_from_slice(&bitlen.to_be_bytes());
        self.block(&chunk);
        let mut out = String::with_capacity(64);
        for word in self.h {
            let _ = write!(out, "{word:08x}");
        }
        out
    }
}

fn sha256_hex(data: &[u8]) -> String {
    let mut s = Sha256::new();
    s.update(data);
    s.finish()
}

/// The FIPS 180-4 known-answer self-check, THE FIRST STATEMENT OF `main` and
/// run before any number exists. SR-A33(iii)'s standing discipline: a receipt
/// whose assertion is an equality of digests carries a second, silent
/// obligation — that the digest function is anchored to published vectors
/// covering the code path actually used, INCLUDING THE STREAMING PATH if the
/// receipt streams. A digest primitive that has never been checked against a
/// published vector is not a receipt of anything.
fn sha256_self_check() {
    assert_eq!(
        sha256_hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        "SHA-256 known-answer self-check failed (FIPS 180-4, \"abc\")"
    );
    assert_eq!(
        sha256_hex(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "SHA-256 known-answer self-check failed (FIPS 180-4, empty string)"
    );
    assert_eq!(
        sha256_hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
        "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
        "SHA-256 known-answer self-check failed (FIPS 180-4, two-block message)"
    );
    let million: Vec<u8> = vec![b'a'; 1_000_000];
    assert_eq!(
        sha256_hex(&million),
        "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0",
        "SHA-256 known-answer self-check failed (FIPS 180-4, one million 'a')"
    );
    let msg = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
    let mut s = Sha256::new();
    for byte in msg {
        s.update(&[*byte]);
    }
    assert_eq!(
        s.finish(),
        sha256_hex(msg),
        "SHA-256 streaming self-check failed: byte-at-a-time differs from one-shot"
    );
    let mut s = Sha256::new();
    let mut at = 0usize;
    for step in [1usize, 7, 63, 64, 65, 3, 100, 200] {
        let end = (at + step).min(million.len());
        s.update(&million[at..end]);
        at = end;
    }
    s.update(&million[at..]);
    assert_eq!(
        s.finish(),
        sha256_hex(&million),
        "SHA-256 streaming self-check failed: irregular chunking differs from one-shot"
    );
}

fn out_dir(name: &str) -> PathBuf {
    let a = PathBuf::from(format!("walt-factory/{name}"));
    if a.exists() {
        return a;
    }
    let b = PathBuf::from(name);
    if b.exists() {
        b
    } else {
        a
    }
}

// -- the packed observation record ------------------------------------------

/// An observation record packed into a `u128` as fixed 5-bit fields, most
/// significant first, plus a length. The derived `Ord` is freeze 36(b)'s
/// lexicographic order over the canonical ascending domino index, with a
/// proper prefix ordering BEFORE its extensions — verified against
/// `Vec<Domino>`'s own ordering before any record is keyed.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
struct Rec {
    packed: u128,
    len: u8,
}

impl Rec {
    fn push(self, d: Domino) -> Rec {
        let n = self.len as usize;
        assert!(
            n < REC_MAX,
            "stop-and-report: an observation record exceeded {REC_MAX} plays"
        );
        Rec {
            packed: self.packed | ((d.index() as u128) << (5 * (REC_MAX - 1 - n))),
            len: self.len + 1,
        }
    }

    fn at(self, i: usize) -> Domino {
        assert!(i < self.len as usize, "record index in range");
        let raw = ((self.packed >> (5 * (REC_MAX - 1 - i))) & 31) as usize;
        Domino::from_index(raw).expect("a packed field is a domino index")
    }

    fn tiles(self) -> Vec<Domino> {
        (0..self.len as usize).map(|i| self.at(i)).collect()
    }

    fn text(self) -> String {
        record_str(&self.tiles())
    }
}

fn rec_order_self_check() {
    let alphabet: Vec<Domino> = [0usize, 1, 2, 5, 27]
        .iter()
        .map(|i| Domino::from_index(*i).expect("domino"))
        .collect();
    let mut pairs: Vec<(Vec<Domino>, Rec)> = Vec::new();
    let push = |v: Vec<Domino>, pairs: &mut Vec<(Vec<Domino>, Rec)>| {
        let mut r = Rec::default();
        for d in &v {
            r = r.push(*d);
        }
        assert_eq!(r.tiles(), v, "record pack/unpack round-trip");
        pairs.push((v, r));
    };
    push(Vec::new(), &mut pairs);
    for a in &alphabet {
        push(vec![*a], &mut pairs);
        for b in &alphabet {
            push(vec![*a, *b], &mut pairs);
            for c in &alphabet {
                push(vec![*a, *b, *c], &mut pairs);
            }
        }
    }
    let mut by_vec = pairs.clone();
    by_vec.sort_by(|x, y| x.0.cmp(&y.0));
    let mut by_rec = pairs;
    by_rec.sort_by_key(|x| x.1);
    assert_eq!(
        by_vec.iter().map(|p| p.0.clone()).collect::<Vec<_>>(),
        by_rec.iter().map(|p| p.0.clone()).collect::<Vec<_>>(),
        "stop-and-report: the packed record ordering is not freeze 36(b)'s lexicographic order"
    );
}

/// Freeze 45's void-free capacity kernel at an n = 4 receipt-corpus
/// coordinate. The leader offset from focal is asserted 0 in place.
fn n4_void_free_kernel(hand: &ReceiptHand) -> Kernel {
    let (hands, leader) = state_before_trick(hand, N4_TRICK).expect("the receipt replays");
    let focal = hand.bidder;
    assert_eq!(
        leader, focal,
        "freeze 45: the leader offset from focal is 0"
    );
    let mut hidden = [Hidden {
        seat: focal,
        capacity: 0,
        voids: ContextSet::EMPTY,
    }; HIDDEN_SEATS];
    let mut pool = DominoSet::EMPTY;
    for (slot, k) in hidden.iter_mut().zip(1..=3) {
        let seat = focal.plus(k);
        *slot = Hidden {
            seat,
            capacity: hands[seat.index()].len(),
            voids: ContextSet::EMPTY,
        };
        pool = pool.union(hands[seat.index()]);
    }
    Kernel::new(hand.decl, focal, hands[focal.index()], pool, hidden)
        .expect("the void-free capacity kernel is well formed")
}

// -- the rule algebra: one node, one walker -----------------------------------

#[derive(Clone, Copy)]
struct Node {
    hands: [DominoSet; Seat::COUNT],
    leader: Seat,
    tiles: [Domino; 4],
    k: usize,
}

impl Node {
    fn child(self, seat: Seat, d: Domino) -> Node {
        let mut hands = self.hands;
        hands[seat.index()].remove(d);
        let mut tiles = self.tiles;
        tiles[self.k] = d;
        Node {
            hands,
            leader: self.leader,
            tiles,
            k: self.k + 1,
        }
    }

    fn seat(self) -> Seat {
        self.leader.plus(self.k)
    }
}

#[derive(Clone, Copy)]
struct Ctx {
    decl: Decl,
    focal: Seat,
    team: Team,
}

impl Ctx {
    fn led(self, node: Node) -> Option<Context> {
        (node.k > 0).then(|| self.decl.led_context(node.tiles[0]))
    }

    fn legal_at(self, node: Node, seat: Seat) -> DominoSet {
        legal_plays(self.decl, node.hands[seat.index()], self.led(node))
    }
}

// -- the three frozen features of FF-A4 --------------------------------------

/// FF-A4's "outstanding trump" at a frontier state: the highest-ranking trump
/// under the declaration among tiles **not yet played in the record and not in
/// the focal seat's hand at `I`** — i.e. held by a field seat. The set of such
/// tiles is a function of the RECORD ALONE (the record names every played tile
/// and the focal hand is fixed), so the identity of the outstanding trump is a
/// property of `I`; only the seat holding it varies across `X_I`. That
/// invariance is asserted at every arrival rather than assumed.
///
/// Ranking is the rule algebra's own `Decl::rank`, which is the order inside
/// the called tier: the double is top and the rest descend by pip sum. No
/// ranking is re-implemented here.
fn outstanding_trump(ctx: Ctx, node: Node) -> Option<Domino> {
    let mut field = DominoSet::EMPTY;
    for k in 1..=3 {
        field = field.union(node.hands[ctx.focal.plus(k).index()]);
    }
    let trumps = ctx.decl.called_set().intersection(field);
    trumps.iter().max_by_key(|d| ctx.decl.rank(*d))
}

/// The seat holding `boss` in this world; it is a field seat by construction.
fn boss_holder(ctx: Ctx, node: Node, boss: Domino) -> Seat {
    for k in 1..=3 {
        let s = ctx.focal.plus(k);
        if node.hands[s.index()].contains(boss) {
            return s;
        }
    }
    panic!("stop-and-report: the outstanding trump is held by no field seat");
}

/// The two features of this pass at one `(omega, b)`: index 0 is F0, the null
/// control, and index 1 is F2 under its AMENDED definition. F1 is not computed
/// — it is settled at FF-A13 and FF-A15(ii) reports F2 only.
///
/// FREEZE 52 v1.1's DOMAIN CLAUSE (FF-A15(i)), which is the whole point of
/// this pass: the no-outstanding-trump fallback is scoped to features that
/// REFERENCE `h(omega)` — F0 and F1 only. **F2 never mentions `h(omega)` and
/// is perfectly well defined with no trump outstanding**; indeed that is when
/// it is most interesting, because control then turns entirely on suit rank.
/// Each feature therefore carries its OWN domain clause, below, and no blanket
/// clause spans the family.
fn features_at(ctx: Ctx, node: Node, boss: Option<Domino>, b: Domino) -> [bool; N_FEAT] {
    // (F0) NULL CONTROL — `boss_owner`. DOMAIN: requires `h(omega)`, so it is
    // 0 when no trump is outstanding. 1 if the holder is an opponent of the
    // focal seat, 0 if a partner. ACTION-BLIND by construction: `b` is not
    // read. Proposition FF-blind then gives capture exactly 0, and
    // Proposition FF-degen gives zero breakpoints — two independent
    // predictions the harness confirms at once.
    let f0 = match boss {
        None => false,
        Some(boss) => boss_holder(ctx, node, boss).team() != ctx.team,
    };

    // (F2) CONTROL SIBLING — `b_is_beatable`, AMENDED per FF-A12 and freeze
    // 52 v1.1. DOMAIN: the whole fiber. It does not reference `h(omega)` and
    // is defined whether or not a trump is outstanding.
    //
    // `phi(omega,b) = 1` iff some opponent **who has not yet played at `I`**
    // holds a tile that, if played, would win the trick over `b` AND over
    // every tile already on the table.
    //
    // The frozen text was ambiguous at FOLLOWING states, where a tile already
    // on the table may already beat `b` — it did not say whether the current
    // winner counts. It does now: the candidate must beat the CURRENT BEST of
    // `{tiles already played} + {b}`, so a `b` that is already losing to the
    // table is not "beatable" by a further opponent unless that opponent can
    // top the table too.
    //
    // AT LEADING STATES THIS IS IDENTICAL TO THE FROZEN READING (FF-A15(i)):
    // nothing is on the table, so the current best is `b` itself, and the
    // focal seat leading means all three field seats are still to play, so
    // "yet to play" excludes nobody. h0's leading measurement is therefore
    // untouched and stands, which is ASSERTED rather than assumed: the
    // leading-part capture is compared against the filed value under (FF-R5).
    //
    // `Decl::beats` IS the rule algebra's BEATS relation and `Decl::trick_key`
    // its ordering; both are called, never reproduced.
    let led_after_b = ctx.led(node).unwrap_or_else(|| ctx.decl.led_context(b));
    let mut best_on_table = b;
    for i in 0..node.k {
        if ctx.decl.trick_key(node.tiles[i], led_after_b)
            > ctx.decl.trick_key(best_on_table, led_after_b)
        {
            best_on_table = node.tiles[i];
        }
    }
    let beat_set = ctx.decl.beats(led_after_b, best_on_table);
    let mut f2 = false;
    // Seats yet to play at `I` are those after the focal seat in this trick:
    // offsets k+1..3 from the leader. The focal seat sits at offset `node.k`.
    for pos in (node.k + 1)..4 {
        let s = node.leader.plus(pos);
        if s.team() != ctx.team && !node.hands[s.index()].intersection(beat_set).is_empty() {
            f2 = true;
        }
    }

    [f0, f2]
}

// -- the rung-one frontier pass, recording per-world rows --------------------

/// One `(state, world)` arrival, carrying everything the sweep needs. The
/// per-world `q_I(omega, b)` are exact world-informed continuation values, not
/// bounds, and none is ever carried out of a frontier (FT-A12(iii)).
#[derive(Clone, Copy)]
struct WorldRow {
    /// `DEN_MU / den`, the arrival weight (Lemma FT-arrive's product form).
    w: u32,
    /// `q_I(omega, b_j)` for each action of `legal` in ascending domino index.
    q: [i64; 4],
    /// Bit `j` of `phi[f]` is feature `f` at action `b_j`.
    phi: [u8; N_FEAT],
}

struct FrontierState {
    legal: DominoSet,
    prefix: i64,
    /// Whether the focal seat is LEADING at `I` (no led context established).
    leading: bool,
    /// The outstanding trump at `I`, a function of the record; asserted
    /// constant across `X_I`.
    boss: Option<Domino>,
    n_worlds: u64,
    acc_p: i128,
    acc_m: i128,
    acc_q: Vec<i128>,
    rows: Vec<WorldRow>,
}

#[derive(Clone, Copy)]
struct Arrival {
    den: u64,
    prefix: i64,
    seen_focal: bool,
}

struct Recorder<'a> {
    ctx: Ctx,
    states: &'a mut BTreeMap<Rec, FrontierState>,
}

impl Recorder<'_> {
    fn walk(&mut self, node: Node, arr: Arrival, rec: Rec, budget: &mut u64) -> Option<i64> {
        if *budget == 0 {
            return None;
        }
        *budget -= 1;
        if node.k == 4 {
            let trick =
                Trick::new(node.leader, node.tiles).expect("distinct tiles by construction");
            let winner = trick.winner(self.ctx.decl);
            let inc = if winner.team() == self.ctx.team {
                SCALE
            } else {
                -SCALE
            };
            if node.hands.iter().all(|h| h.is_empty()) {
                assert!(
                    arr.seen_focal,
                    "stop-and-report: T_a = 0 fails — a positive-mass world reached the end of the hand with no further focal decision"
                );
                return Some(inc);
            }
            let below = Arrival {
                prefix: if arr.seen_focal {
                    arr.prefix
                } else {
                    arr.prefix + inc
                },
                ..arr
            };
            let next = Node {
                hands: node.hands,
                leader: winner,
                tiles: [Domino::ALL[0]; 4],
                k: 0,
            };
            return Some(inc + self.walk(next, below, rec, budget)?);
        }
        let seat = node.seat();
        let legal = self.ctx.legal_at(node, seat);
        if seat == self.ctx.focal {
            if arr.seen_focal {
                let mut best = i64::MIN;
                for d in legal.iter() {
                    let v = self.walk(node.child(seat, d), arr, rec.push(d), budget)?;
                    if v > best {
                        best = v;
                    }
                }
                return Some(best);
            }
            return self.at_frontier(node, arr, rec, legal, budget);
        }
        let n = i64::try_from(legal.len()).expect("a legal set size fits i64");
        let below = Arrival {
            den: if arr.seen_focal {
                arr.den
            } else {
                arr.den * u64::try_from(legal.len()).expect("a legal set size fits u64")
            },
            ..arr
        };
        let mut sum: i64 = 0;
        for d in legal.iter() {
            sum += self.walk(node.child(seat, d), below, rec.push(d), budget)?;
        }
        let avg = sum / n;
        assert_eq!(
            avg * n,
            sum,
            "freeze 38(f): the scaled field average is exact — 12^12 carries every legal-set size"
        );
        Some(avg)
    }

    fn at_frontier(
        &mut self,
        node: Node,
        arr: Arrival,
        rec: Rec,
        legal: DominoSet,
        budget: &mut u64,
    ) -> Option<i64> {
        let seat = self.ctx.focal;
        assert!(
            legal.len() < N4_GRADE,
            "|A(I)| is bounded by the grade - 1 tiles in hand at the frontier"
        );
        let below = Arrival {
            seen_focal: true,
            ..arr
        };
        let mut child = [0i64; 4];
        let mut best = i64::MIN;
        for (j, d) in legal.iter().enumerate() {
            let v = self.walk(node.child(seat, d), below, rec.push(d), budget)?;
            child[j] = v;
            if v > best {
                best = v;
            }
        }
        let leading = self.ctx.led(node).is_none();
        let boss = outstanding_trump(self.ctx, node);
        let mut phi = [0u8; N_FEAT];
        for (j, d) in legal.iter().enumerate() {
            let f = features_at(self.ctx, node, boss, d);
            for (fi, on) in f.iter().enumerate() {
                if *on {
                    phi[fi] |= 1u8 << j;
                }
            }
        }
        let w = DEN_MU / i128::from(arr.den);
        assert_eq!(
            w * i128::from(arr.den),
            DEN_MU,
            "freeze 38(f): DEN_MU = 12^6 carries every pre-frontier arrival denominator"
        );
        let entry = self.states.entry(rec).or_insert_with(|| FrontierState {
            legal,
            prefix: arr.prefix,
            leading,
            boss,
            n_worlds: 0,
            acc_p: 0,
            acc_m: 0,
            acc_q: vec![0; legal.len()],
            rows: Vec::new(),
        });
        assert_eq!(
            entry.legal, legal,
            "(FT-A7(ii)) stop-and-report: A(I) is not common across X_I"
        );
        assert_eq!(
            entry.prefix, arr.prefix,
            "stop-and-report: the pre-frontier increment is not a function of the record"
        );
        assert_eq!(
            entry.leading, leading,
            "stop-and-report: the leading/following position is not a function of the record"
        );
        // FF-A4's invariance, asserted rather than assumed: the outstanding
        // trump is determined by the record, because the record names every
        // played tile and the focal hand is fixed at I. Only its HOLDER varies
        // across X_I, and that variation is the whole content of F0 and F1.
        assert_eq!(
            entry.boss, boss,
            "(FF-A4) stop-and-report: the outstanding trump is not a function of the record at [{}]",
            rec.text()
        );
        entry.n_worlds += 1;
        entry.acc_p += w;
        entry.acc_m += w * i128::from(arr.prefix + best);
        let mut qrow = [0i64; 4];
        for (j, _) in legal.iter().enumerate() {
            let qv = arr.prefix + child[j];
            entry.acc_q[j] += w * i128::from(qv);
            qrow[j] = qv;
        }
        entry.rows.push(WorldRow {
            w: u32::try_from(w).expect("an arrival weight fits u32"),
            q: qrow,
            phi,
        });
        Some(best)
    }
}

struct FrontierPass {
    states: BTreeMap<Rec, FrontierState>,
    world_fold: i128,
    steps: u64,
    residual: u64,
}

fn frontier_pass(
    ctx: Ctx,
    worlds: &[[DominoSet; Seat::COUNT]],
    root: Domino,
) -> Option<FrontierPass> {
    let mut states: BTreeMap<Rec, FrontierState> = BTreeMap::new();
    let mut budget = B_WALK;
    let mut world_fold: i128 = 0;
    for hands in worlds {
        let mut hands = *hands;
        assert!(
            hands[ctx.focal.index()].contains(root),
            "a root action is a focal tile"
        );
        hands[ctx.focal.index()].remove(root);
        let mut tiles = [Domino::ALL[0]; 4];
        tiles[0] = root;
        let node = Node {
            hands,
            leader: ctx.focal,
            tiles,
            k: 1,
        };
        let arr = Arrival {
            den: 1,
            prefix: 0,
            seen_focal: false,
        };
        let mut r = Recorder {
            ctx,
            states: &mut states,
        };
        world_fold += i128::from(r.walk(node, arr, Rec::default().push(root), &mut budget)?);
    }
    Some(FrontierPass {
        states,
        world_fold,
        steps: B_WALK - budget,
        residual: budget,
    })
}

// -- Lemma FF-min: exact minimisation over the breakpoints -------------------

/// Compare `n1/d1` with `n2/d2` exactly, in i128, with no float. Denominators
/// are normalised positive first; the cross products are bounded far inside
/// i128 at this carrier and the multiplications are checked.
fn cmp_frac(n1: i128, d1: i128, n2: i128, d2: i128) -> std::cmp::Ordering {
    let (n1, d1) = if d1 < 0 { (-n1, -d1) } else { (n1, d1) };
    let (n2, d2) = if d2 < 0 { (-n2, -d2) } else { (n2, d2) };
    assert!(d1 > 0 && d2 > 0, "a breakpoint denominator is nonzero");
    let a = n1
        .checked_mul(d2)
        .expect("freeze 52(d) stop-and-report: breakpoint comparison overflowed");
    let b = n2
        .checked_mul(d1)
        .expect("freeze 52(d) stop-and-report: breakpoint comparison overflowed");
    a.cmp(&b)
}

/// One breakpoint crossing, as an event on the swept `(intercept, slope)`.
struct Event {
    /// The breakpoint `sigma = num/den`, with `den > 0`.
    num: i128,
    den: i128,
    /// `w * (q_new - q_old)` and `w * (P_old - P_new)`.
    d_t: i128,
    d_s: i128,
}

/// What the sweep returns at one `(state, feature)`.
struct Sweep {
    n_break: usize,
    /// `sigma* = num/den` with `den > 0`; `(0, 1)` when there are no
    /// breakpoints (freeze 52(e)).
    s_num: i128,
    s_den: i128,
    /// `theta* = p_num * sigma*`, raw (SCALE-scaled differential) units.
    theta_raw: BigRational,
    /// `delta_I^{theta*}`, unnormalised, as an exact rational.
    resid: BigRational,
    /// `c_I(b)` numerators over `p_num`, one per action.
    c_num: Vec<i128>,
    /// `G_I(theta*) * s_den` and `s_den`, kept for (FF-R4).
    g_scaled: i128,
}

/// The exact minimisation of Lemma FF-min at one state and one feature.
///
/// Freeze 52(d) prescribes: enumerate the breakpoints, evaluate `G_I` at each,
/// take the least. That is what happens here; the evaluation is made `O(1)`
/// per breakpoint by sorting the breakpoints and sweeping an exactly
/// maintained `(T, S)` with `G_I(sigma) = T + S*sigma`, which is a cheaper way
/// to perform the specified evaluation and not a different algorithm.
/// (FF-R4) re-evaluates the winner by direct summation with no incremental
/// state.
///
/// Freeze 52(e)'s tie rule: `theta*` is the SMALLEST breakpoint attaining the
/// minimum, in ascending rational order — the sweep visits breakpoints in
/// ascending order and updates only on a STRICT decrease, so the first
/// minimiser is the one kept. With no breakpoints, `theta* = 0`.
fn sweep_state(st: &FrontierState, f: usize) -> Sweep {
    let n_act = st.legal.len();
    let p_num = st.acc_p;
    assert!(p_num > 0, "a frontier state has positive mass");

    // Per-action centres, per FF-A2 and freeze 52(c). A single per-state
    // centre is exactly what is barred here.
    let mut c_num = vec![0i128; n_act];
    for row in &st.rows {
        for (j, c) in c_num.iter_mut().enumerate() {
            if row.phi[f] & (1u8 << j) != 0 {
                *c += i128::from(row.w);
            }
        }
    }
    let pat = |row: &WorldRow, j: usize| -> i128 {
        let phi = i128::from(u8::from(row.phi[f] & (1u8 << j) != 0));
        p_num * phi - c_num[j]
    };

    let mut events: Vec<Event> = Vec::new();
    let mut t_acc: i128 = 0;
    let mut s_acc: i128 = 0;
    for row in &st.rows {
        let w = i128::from(row.w);
        // The lines L_j(sigma) = q_j - sigma * P_j, sorted by P descending,
        // i.e. by slope ascending, which is the order in which they can become
        // active left to right on the upper envelope.
        let mut lines: Vec<(i128, i128)> = (0..n_act)
            .map(|j| (pat(row, j), i128::from(row.q[j])))
            .collect();
        lines.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));
        lines.dedup_by_key(|l| l.0);
        // The upper envelope, by the standard monotone chain. With
        // |A(I)| <= 3 the stack never exceeds three entries and there are at
        // most two breakpoints per world, exactly as FF-A9(iii) says.
        let mut hull: Vec<(i128, i128)> = Vec::with_capacity(lines.len());
        for c in lines {
            while hull.len() >= 2 {
                let a = hull[hull.len() - 2];
                let b = hull[hull.len() - 1];
                // b is unnecessary iff x(a,c) <= x(a,b).
                let ord = cmp_frac(c.1 - a.1, c.0 - a.0, b.1 - a.1, b.0 - a.0);
                if ord == std::cmp::Ordering::Greater {
                    break;
                }
                hull.pop();
            }
            hull.push(c);
        }
        // As sigma -> -infinity the active line is the one with the largest P,
        // which is hull[0].
        let (p0, q0) = hull[0];
        t_acc += w * q0;
        s_acc -= w * p0;
        for pair in hull.windows(2) {
            let (pu, qu) = pair[0];
            let (pv, qv) = pair[1];
            let den = pv - pu;
            assert!(den != 0, "consecutive envelope lines have distinct slopes");
            let (num, den) = if den < 0 {
                (-(qv - qu), -den)
            } else {
                (qv - qu, den)
            };
            events.push(Event {
                num,
                den,
                d_t: w * (qv - qu),
                d_s: w * (pu - pv),
            });
        }
    }
    events.sort_by(|a, b| cmp_frac(a.num, a.den, b.num, b.den));
    let n_break = {
        let mut n = 0usize;
        let mut i = 0usize;
        while i < events.len() {
            let mut j = i + 1;
            while j < events.len()
                && cmp_frac(events[i].num, events[i].den, events[j].num, events[j].den)
                    == std::cmp::Ordering::Equal
            {
                j += 1;
            }
            n += 1;
            i = j;
        }
        n
    };

    let g_at = |t: i128, s: i128, n: i128, d: i128| -> i128 {
        let a = t
            .checked_mul(d)
            .expect("freeze 52(d) stop-and-report: G evaluation overflowed");
        let b = s
            .checked_mul(n)
            .expect("freeze 52(d) stop-and-report: G evaluation overflowed");
        a.checked_add(b)
            .expect("freeze 52(d) stop-and-report: G evaluation overflowed")
    };

    let best_q = *st.acc_q.iter().max().expect("A(I) is nonempty");
    let (mut s_num, mut s_den) = (0i128, 1i128);
    let mut g_scaled = t_acc; // G(0) * 1 on the piece in force, set below
    let mut best: Option<BigRational> = None;
    if events.is_empty() {
        // Freeze 52(e): with no breakpoints, theta* = 0. The envelope is a
        // single line per world, so its active action is optimal at every
        // sigma and in particular at 0, whence G(0) = t_acc = sum_omega
        // w * m(omega) and the residual is delta_I exactly.
        g_scaled = t_acc;
    } else {
        let mut i = 0usize;
        while i < events.len() {
            let (n, d) = (events[i].num, events[i].den);
            let g = g_at(t_acc, s_acc, n, d);
            let val = BigRational::new(big(g), big(d));
            if best.as_ref().is_none_or(|b| val < *b) {
                best = Some(val);
                s_num = n;
                s_den = d;
                g_scaled = g;
            }
            while i < events.len()
                && cmp_frac(events[i].num, events[i].den, n, d) == std::cmp::Ordering::Equal
            {
                t_acc += events[i].d_t;
                s_acc += events[i].d_s;
                i += 1;
            }
        }
    }

    let resid =
        BigRational::new(big(g_scaled), big(s_den)) - BigRational::from_integer(big(best_q));
    let theta_raw = BigRational::new(big(p_num) * big(s_num), big(s_den));
    Sweep {
        n_break,
        s_num,
        s_den,
        theta_raw,
        resid,
        c_num,
        g_scaled,
    }
}

// -- FF-A15(iii): the SHARED-theta fit over a declared part ------------------

/// What the pooled fit returns.
struct SharedFit {
    /// The number of DISTINCT pooled breakpoints — the union of the per-state
    /// breakpoint sets, as Lemma FF-min applies verbatim to the pooled sum.
    n_break: usize,
    /// The single shared `theta*`, raw (SCALE-scaled differential) units.
    theta_raw: BigRational,
    /// `sum_I delta_I^{theta*}` over the part, under ONE shared theta.
    resid: BigRational,
    /// The part's `sum_I delta_I`, for the capture denominator.
    part_delta: i128,
}

/// FF-A15(iii). The object is `min_theta sum_I delta_I^theta` over a declared
/// part with **one** theta, exactly minimised. The sum of convex
/// piecewise-linear functions is convex piecewise-linear, so Lemma FF-min
/// applies verbatim to the pooled objective with the UNION of the per-state
/// breakpoints; freeze 52(e)'s tie rule is applied verbatim to the pooled
/// function (the smallest pooled breakpoint attaining the minimum).
///
/// The per-state sweep is parameterised by `sigma_I = theta / p_num_I`, which
/// differs from state to state, so the pooled fit cannot reuse those integers
/// directly: each state's envelope is rebuilt and its breakpoints are mapped
/// into the COMMON theta by `theta = p_num_I * sigma_I`. The pooled slope and
/// intercept are then exact rationals, accumulated in BigRational so that no
/// comparison or accumulation can overflow.
fn shared_fit(part: &[(&FrontierState, i128)], f: usize) -> SharedFit {
    let mut t_acc = BigRational::zero();
    let mut s_acc = BigRational::zero();
    let mut sum_best = BigInt::from(0);
    let mut part_delta: i128 = 0;
    // (theta, delta_T, delta_S)
    let mut events: Vec<(BigRational, BigInt, BigRational)> = Vec::new();

    for (st, dtax) in part {
        part_delta += *dtax;
        let n_act = st.legal.len();
        let p_num = st.acc_p;
        sum_best += big(*st.acc_q.iter().max().expect("A(I) is nonempty"));
        let mut c_num = vec![0i128; n_act];
        for row in &st.rows {
            for (j, c) in c_num.iter_mut().enumerate() {
                if row.phi[f] & (1u8 << j) != 0 {
                    *c += i128::from(row.w);
                }
            }
        }
        let pn = BigRational::from_integer(big(p_num));
        for row in &st.rows {
            let w = i128::from(row.w);
            let mut lines: Vec<(i128, i128)> = (0..n_act)
                .map(|j| {
                    let phi = i128::from(u8::from(row.phi[f] & (1u8 << j) != 0));
                    (p_num * phi - c_num[j], i128::from(row.q[j]))
                })
                .collect();
            lines.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));
            lines.dedup_by_key(|l| l.0);
            let mut hull: Vec<(i128, i128)> = Vec::with_capacity(lines.len());
            for c in lines {
                while hull.len() >= 2 {
                    let a = hull[hull.len() - 2];
                    let b = hull[hull.len() - 1];
                    let ord = cmp_frac(c.1 - a.1, c.0 - a.0, b.1 - a.1, b.0 - a.0);
                    if ord == std::cmp::Ordering::Greater {
                        break;
                    }
                    hull.pop();
                }
                hull.push(c);
            }
            let (p0, q0) = hull[0];
            t_acc += BigRational::from_integer(big(w) * big(q0));
            s_acc -= BigRational::from_integer(big(w) * big(p0)) / pn.clone();
            for pair in hull.windows(2) {
                let (pu, qu) = pair[0];
                let (pv, qv) = pair[1];
                let den = pv - pu;
                let (num, den) = if den < 0 {
                    (-(qv - qu), -den)
                } else {
                    (qv - qu, den)
                };
                // sigma = num/den maps to the common theta = p_num * sigma.
                let theta = BigRational::new(big(p_num) * big(num), big(den));
                events.push((
                    theta,
                    big(w) * big(qv - qu),
                    BigRational::from_integer(big(w) * big(pu - pv)) / pn.clone(),
                ));
            }
        }
    }
    events.sort_by(|a, b| a.0.cmp(&b.0));

    let mut n_break = 0usize;
    let mut best: Option<BigRational> = None;
    let mut theta_star = BigRational::zero();
    let mut g_star = t_acc.clone();
    let mut i = 0usize;
    while i < events.len() {
        let theta = events[i].0.clone();
        n_break += 1;
        let g = t_acc.clone() + s_acc.clone() * theta.clone();
        if best.as_ref().is_none_or(|b| g < *b) {
            best = Some(g.clone());
            theta_star = theta.clone();
            g_star = g;
        }
        while i < events.len() && events[i].0 == theta {
            t_acc += BigRational::from_integer(events[i].1.clone());
            s_acc += events[i].2.clone();
            i += 1;
        }
    }
    SharedFit {
        n_break,
        theta_raw: theta_star,
        resid: g_star - BigRational::from_integer(sum_best),
        part_delta,
    }
}

// -- one unit ----------------------------------------------------------------

/// One feature's audition at one unit. Every rational is exact.
struct FeatOut {
    resid_sum: BigRational,
    capture: BigRational,
    n_all: u64,
    n_some: u64,
    n_none: u64,
    n_break_total: u64,
    lead_delta: i128,
    lead_resid: BigRational,
    n_lead: u64,
    foll_delta: i128,
    foll_resid: BigRational,
    n_foll: u64,
    rows: Vec<String>,
    /// `(leading, theta*_raw)` per swept state, in freeze-38(d) record order —
    /// FF-A15(iii)'s "per-state distribution of theta*, already emitted and
    /// costing nothing to summarise".
    theta_raws: Vec<(bool, BigRational)>,
}

struct FfUnit {
    n_states: u64,
    n_arrivals: u64,
    zero: u64,
    pos: u64,
    mass: Q,
    u0_diff: Q,
    u1_diff: Q,
    delta1_diff: Q,
    frontier_digest: String,
    n_no_boss_pos: u64,
    n_no_boss_all: u64,
    n_lead_pos: u64,
    n_foll_pos: u64,
    feats: Vec<FeatOut>,
    /// FF-A15(iii)'s pooled fit, computed only where the ruling commissions it.
    shared: Option<SharedFit>,
    steps: u64,
    residual: u64,
}

#[allow(clippy::too_many_lines)]
fn run_unit(kernel: &Kernel, root: Domino, do_shared: bool) -> Option<FfUnit> {
    assert_eq!(
        kernel.viewer_hand().len(),
        N4_GRADE,
        "the declared grade is the coordinate's grade (N4-A11)"
    );
    let worlds: Vec<[DominoSet; Seat::COUNT]> = kernel.worlds().map(|w| w.hands()).collect();
    let n_worlds = i128::try_from(worlds.len()).expect("fiber size fits i128");
    let ctx = Ctx {
        decl: kernel.decl(),
        focal: kernel.viewer(),
        team: kernel.viewer().team(),
    };
    let pass = frontier_pass(ctx, &worlds, root)?;
    let n_states = u64::try_from(pass.states.len()).expect("state count fits u64");
    assert!(
        n_states <= P_MAX,
        "freeze 44(b) v2: the rung-one frontier partition exceeds P_max v2"
    );

    let norm1 = DEN_MU * i128::from(SCALE) * n_worlds;
    let pnorm1 = DEN_MU * n_worlds;

    let mut sum_p: i128 = 0;
    let mut sum_m: i128 = 0;
    let mut sum_best: i128 = 0;
    let mut n_arrivals: u64 = 0;
    let mut zero: u64 = 0;
    let mut pos: u64 = 0;
    let mut n_no_boss_pos: u64 = 0;
    let mut n_no_boss_all: u64 = 0;
    let mut n_lead_pos: u64 = 0;
    let mut n_foll_pos: u64 = 0;
    let mut digest = Sha256::new();
    let mut positive: Vec<(Rec, i128)> = Vec::new();
    for (rec, st) in &pass.states {
        let best = *st.acc_q.iter().max().expect("A(I) is nonempty");
        let dtax = st.acc_m - best;
        assert!(
            dtax >= 0,
            "stop-and-report: a local tax is negative (avg-of-max is below max-of-avg)"
        );
        sum_p += st.acc_p;
        sum_m += st.acc_m;
        sum_best += best;
        n_arrivals += st.n_worlds;
        if st.boss.is_none() {
            n_no_boss_all += 1;
        }
        if dtax == 0 {
            zero += 1;
        } else {
            pos += 1;
            positive.push((*rec, dtax));
            if st.boss.is_none() {
                n_no_boss_pos += 1;
            }
            if st.leading {
                n_lead_pos += 1;
            } else {
                n_foll_pos += 1;
            }
        }
        // (FT-R7c)'s canonical serialisation, byte-identical to the one
        // `second_rung.rs` used, so the h2 digests are a genuine cross-probe
        // comparison against a prior process rather than a self-comparison.
        digest.update(rec.text().as_bytes());
        digest.update(b"|");
        digest.update(qs(tax_to_count(Q::new(dtax, norm1))).as_bytes());
        digest.update(b"\n");
    }
    assert_eq!(
        sum_m,
        DEN_MU * pass.world_fold,
        "stop-and-report: the frontier accumulation and the per-world revealed fold disagree"
    );
    let mass = Q::new(sum_p, pnorm1);
    let delta1_unnorm = sum_m - sum_best;

    let mut feats: Vec<FeatOut> = Vec::new();
    // The feature loop is sequential and F0 is FIRST in the frozen FF-A4
    // order, which is what makes (FF-R1) blocking: its assertions fire before
    // any F1 or F2 sweep is run.
    #[allow(clippy::needless_range_loop)]
    for f in 0..N_FEAT {
        let mut out = FeatOut {
            resid_sum: BigRational::zero(),
            capture: BigRational::zero(),
            n_all: 0,
            n_some: 0,
            n_none: 0,
            n_break_total: 0,
            lead_delta: 0,
            lead_resid: BigRational::zero(),
            n_lead: 0,
            foll_delta: 0,
            foll_resid: BigRational::zero(),
            n_foll: 0,
            rows: Vec::new(),
            theta_raws: Vec::new(),
        };
        for (rec, dtax) in &positive {
            let st = pass.states.get(rec).expect("a positive state exists");
            let sw = sweep_state(st, f);

            // (FF-R2) THE CENTRING RECEIPT, at EVERY b and not only the
            // argmax b: sum_omega mu_I(omega) lambda_I(omega,b) = 0 at the
            // reported theta*. With lambda = sigma * P this is sum_omega
            // w * P_I(omega,b) = 0, which is exactly Theorem 12.1's
            // hypothesis and exactly what the per-state centring of FF-A2
            // fails. By construction is not a receipt (PG-A8).
            for (j, _) in st.legal.iter().enumerate() {
                let mut acc: i128 = 0;
                for row in &st.rows {
                    let phi = i128::from(u8::from(row.phi[f] & (1u8 << j) != 0));
                    acc += i128::from(row.w) * (st.acc_p * phi - sw.c_num[j]);
                }
                assert_eq!(
                    acc, 0,
                    "(FF-R2) stop-and-report: the fee is not centred at action index {j} of state [{}] for feature {}",
                    rec.text(),
                    FEAT_NAME[f]
                );
            }

            // (FF-R3) THE BOUND RECEIPT: 0 <= delta_I^{theta*} <= delta_I,
            // with delta_I taken from the FRONTIER PASS. Contentful by Lemma
            // FF-min(b),(d) — the swept minimum and delta_I are different
            // computations, so a sweep bug shows up here. This is NOT a
            // Proposition SR-taut identity, and that distinction is printed
            // beside it.
            assert!(
                !sw.resid.is_negative(),
                "(FF-R3) stop-and-report: delta_I^theta* < 0 at [{}] for feature {}",
                rec.text(),
                FEAT_NAME[f]
            );
            assert!(
                sw.resid <= BigRational::from_integer(big(*dtax)),
                "(FF-R3) stop-and-report: delta_I^theta* > delta_I at [{}] for feature {}",
                rec.text(),
                FEAT_NAME[f]
            );

            // (FF-R4) THE DIRECT-EVALUATION RECEIPT: recompute G_I(theta*) by
            // direct summation over worlds with NO incremental state, and
            // assert equality with the swept value. Two independently written
            // paths sharing only the inputs.
            let mut direct: i128 = 0;
            for row in &st.rows {
                let mut best = i128::MIN;
                for (j, _) in st.legal.iter().enumerate() {
                    let phi = i128::from(u8::from(row.phi[f] & (1u8 << j) != 0));
                    let p = st.acc_p * phi - sw.c_num[j];
                    let v = i128::from(row.q[j])
                        .checked_mul(sw.s_den)
                        .expect("freeze 52(d) stop-and-report: direct evaluation overflowed")
                        - sw.s_num
                            .checked_mul(p)
                            .expect("freeze 52(d) stop-and-report: direct evaluation overflowed");
                    if v > best {
                        best = v;
                    }
                }
                direct = direct
                    .checked_add(
                        i128::from(row.w)
                            .checked_mul(best)
                            .expect("freeze 52(d) stop-and-report: direct evaluation overflowed"),
                    )
                    .expect("freeze 52(d) stop-and-report: direct evaluation overflowed");
            }
            assert_eq!(
                direct, sw.g_scaled,
                "(FF-R4) stop-and-report: the direct summation and the sweep disagree on G_I(theta*) at [{}] for feature {}",
                rec.text(),
                FEAT_NAME[f]
            );

            let captured = BigRational::from_integer(big(*dtax)) - sw.resid.clone();
            let verdict = if sw.resid.is_zero() {
                out.n_all += 1;
                "all"
            } else if captured.is_zero() {
                out.n_none += 1;
                "none"
            } else {
                out.n_some += 1;
                "some"
            };
            out.theta_raws.push((st.leading, sw.theta_raw.clone()));
            out.resid_sum += sw.resid.clone();
            out.n_break_total += u64::try_from(sw.n_break).expect("fits");
            if st.leading {
                out.n_lead += 1;
                out.lead_delta += *dtax;
                out.lead_resid += sw.resid.clone();
            } else {
                out.n_foll += 1;
                out.foll_delta += *dtax;
                out.foll_resid += sw.resid.clone();
            }

            let denom2 = BigRational::from_integer(big(2) * big(norm1));
            let mut centres = String::new();
            for (j, d) in st.legal.iter().enumerate() {
                let _ = write!(
                    centres,
                    " {}:{}",
                    tile(d),
                    bqs(&BigRational::new(big(sw.c_num[j]), big(st.acc_p)))
                );
            }
            out.rows.push(format!(
                "    I=[{}]  lead = {}  p_I = {}  |X_I| = {}  |A(I)| = {}  boss = {}  c_I(b):{}  breakpoints = {}  sigma* = {}/{}  theta* = {} (count)  delta_I = {} (count)  delta_I^theta* = {} (count)  captured = {} (count)  verdict = {}",
                rec.text(),
                yn(st.leading),
                qs(Q::new(st.acc_p, pnorm1)),
                st.n_worlds,
                st.legal.len(),
                st.boss.map_or_else(|| "none".to_string(), tile),
                centres,
                sw.n_break,
                sw.s_num,
                sw.s_den,
                bqs(&(sw.theta_raw.clone()
                    / BigRational::from_integer(big(2) * big(i128::from(SCALE))))),
                qs(tax_to_count(Q::new(*dtax, norm1))),
                bqs(&(sw.resid.clone() / denom2.clone())),
                bqs(&(captured / denom2)),
                verdict
            ));

            // (FF-R1) THE NULL-CONTROL RECEIPT, per state. F0 is action-blind,
            // so Proposition FF-blind gives delta_I^{theta*} = delta_I exactly
            // and freeze 52(e) gives theta* = 0. This is the only check in the
            // build whose answer is known BY PROOF rather than by a filed
            // rational — the (SR-R9) role.
            if f == 0 {
                assert_eq!(
                    sw.n_break, 0,
                    "(FF-R1) stop-and-report: the action-blind null control produced a breakpoint at [{}] — an action-blind fee has no crossings",
                    rec.text()
                );
                assert!(
                    sw.theta_raw.is_zero(),
                    "(FF-R1) stop-and-report: the null control's theta* is not 0 at [{}], so freeze 52(e)'s tie rule is wrong",
                    rec.text()
                );
                assert_eq!(
                    sw.resid,
                    BigRational::from_integer(big(*dtax)),
                    "(FF-R1) stop-and-report: the null control captured a nonzero amount at [{}] — Proposition FF-blind says exactly zero, so the centring, the sweep, the accumulation or the arithmetic is wrong",
                    rec.text()
                );
            }
        }
        let d1 = BigRational::from_integer(big(delta1_unnorm));
        out.capture = if d1.is_zero() {
            BigRational::zero()
        } else {
            (d1 - out.resid_sum.clone()) / BigRational::from_integer(big(delta1_unnorm))
        };
        // (FF-R1), per unit: the null control's oracle-theta capture is
        // exactly 0. BLOCKING — this assertion fires before any F1 or F2
        // sweep is run, because the feature loop is sequential and F0 is
        // first in the frozen FF-A4 order.
        if f == 0 {
            assert!(
                out.capture.is_zero(),
                "(FF-R1) stop-and-report: the null control's oracle-theta capture is {} and Proposition FF-blind says exactly 0. No F1 or F2 number is reported, emitted or discussed (FF-A8(a))",
                bqs(&out.capture)
            );
            assert_eq!(
                out.n_none, pos,
                "(FF-R1) stop-and-report: the null control captured something at some state"
            );
        }
        feats.push(out);
    }

    // FF-A15(iii), COMMISSIONED: the shared-theta fit for F2 on the LEADING
    // part. It is run only where the ruling commissions it — h0's leading
    // states — and nowhere else.
    let shared = if do_shared {
        let part: Vec<(&FrontierState, i128)> = positive
            .iter()
            .filter_map(|(rec, d)| {
                let st = pass.states.get(rec).expect("a positive state exists");
                st.leading.then_some((st, *d))
            })
            .collect();
        Some(shared_fit(&part, 1))
    } else {
        None
    };

    Some(FfUnit {
        n_states,
        n_arrivals,
        zero,
        pos,
        mass,
        u0_diff: Q::new(sum_m, norm1),
        u1_diff: Q::new(sum_best, norm1),
        delta1_diff: Q::new(delta1_unnorm, norm1),
        frontier_digest: digest.finish(),
        n_no_boss_pos,
        n_no_boss_all,
        n_lead_pos,
        n_foll_pos,
        feats,
        shared,
        steps: pass.steps,
        residual: pass.residual,
    })
}

// -- the carrier's units -----------------------------------------------------

#[derive(Clone, Copy)]
struct UnitKey {
    coord: usize,
    action: Domino,
    arm: usize,
}

fn dom(hi: u8, lo: u8) -> Domino {
    Domino::new(Pip::new(hi).expect("pip"), Pip::new(lo).expect("pip"))
}

fn filed_actions(rows: &[FiledRow; 4]) -> Vec<Domino> {
    rows.iter().map(|(hi, lo, _, _, _)| dom(*hi, *lo)).collect()
}

fn filed_qh(rows: &[FiledRow; 4]) -> Vec<Q> {
    rows.iter().map(|(_, _, (n, d), _, _)| q(*n, *d)).collect()
}

/// FREEZE 52(a), enumerated with NO GENERATING RULE.
fn carrier_units() -> Vec<UnitKey> {
    vec![
        UnitKey {
            coord: 0,
            action: dom(0, 0),
            arm: 1,
        },
        UnitKey {
            coord: 1,
            action: dom(5, 3),
            arm: 2,
        },
        UnitKey {
            coord: 1,
            action: dom(5, 4),
            arm: 2,
        },
    ]
}

struct UnitText {
    named: String,
    stopped: bool,
}

#[allow(clippy::too_many_lines)]
fn render_unit(receipt: &Receipt, key: UnitKey) -> UnitText {
    let (hand_id, filed_pip, rows) = &FF_FILED[key.coord];
    let hand = &receipt.hands[*hand_id];
    let kernel = n4_void_free_kernel(hand);

    let Decl::PipTrump(p) = kernel.decl() else {
        panic!("pip-trump only")
    };
    assert_eq!(p.value(), *filed_pip, "freeze 45: declaration identity");
    assert_eq!(kernel.count(), N4_FIBER, "freeze 45: |X| = 34,650");
    assert_eq!(
        kernel.viewer_hand().len(),
        N4_GRADE,
        "freeze 45: grade 4 identity"
    );
    let actions: Vec<Domino> = kernel.viewer_hand().iter().collect();
    assert_eq!(
        actions,
        filed_actions(rows),
        "freeze 45: the filed root-action list is this coordinate's hand"
    );
    let qh = filed_qh(rows);
    let ia = actions
        .iter()
        .position(|x| *x == key.action)
        .expect("the unit's action is a root action");
    let vh = qh.iter().copied().max().expect("nonempty");
    let astar_idx = (0..actions.len())
        .find(|i| qh[*i] == vh && *i != ia)
        .expect("a binding pair has an H-argmax competitor distinct from a");

    let first = FF_FIRST
        .iter()
        .find(|(h, hi, lo, ..)| *h == *hand_id && dom(*hi, *lo) == key.action)
        .expect("every carrier unit has a rung-one filed record");
    let (_, _, _, f_states, f_arr, f_zero, f_pos, f_d1, f_u1, f_u0, f_steps, f_digest) = *first;

    let mut out = String::new();
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== FF unit (freeze 52(a), ARM {}): coord h{hand_id} grade={N4_GRADE} pip={} hand=[{}] pool=[{}] leader-offset=0 |X|={N4_FIBER} enumeration=freeze-7/23 || competitor a = {} ==",
        key.arm,
        p.value(),
        tiles_str(kernel.viewer_hand()),
        tiles_str(kernel.pool()),
        tile(key.action)
    );
    let _ = writeln!(
        out,
        "  provenance only: corpus hand id {hand_id}, trick {N4_TRICK} (never identity components, freeze 45); freeze-set digest {FF_DIGEST}"
    );
    let _ = writeln!(
        out,
        "  frozen source (SEP-A14(ii), FT-A28(i); quoted from separation_n4_2026-08-14.txt and fusion_tax_2026-08-14.txt, exploratory tier, NEVER re-parsed at run time): Q^H(a) = {}  a* = {} with Q^H(a*) = {}  U^(0) = {}  U^(1) = {}  Delta^(1) = {}  |I_1| = {f_states}  arrivals = {f_arr}  census (zero/positive) = {f_zero}/{f_pos}  (count convention)",
        qs(qh[ia]),
        tile(actions[astar_idx]),
        qs(qh[astar_idx]),
        qs(q(f_u0.0, f_u0.1)),
        qs(q(f_u1.0, f_u1.1)),
        qs(q(f_d1.0, f_d1.1))
    );

    let Some(r) = run_unit(&kernel, key.action, key.coord == 0) else {
        let _ = writeln!(
            out,
            "  DECLARED STOP (freeze 44(b) v2, FF-A8(e)): a walk budget of {B_WALK} was exhausted. NO PARTIAL FOLD is retained, so there is no partial capture and no partial residual; this is a stop and is never a finding (R-A18)."
        );
        return UnitText {
            named: out,
            stopped: true,
        };
    };

    // ---- (FF-R6) determinism: the in-run second pass ----------------------
    let r2 = run_unit(&kernel, key.action, key.coord == 0)
        .expect("the second pass runs under the same budgets");
    assert_eq!(
        (
            r.n_states,
            r.n_arrivals,
            r.zero,
            r.pos,
            r.n_no_boss_pos,
            r.n_lead_pos,
            r.n_foll_pos,
            r.steps
        ),
        (
            r2.n_states,
            r2.n_arrivals,
            r2.zero,
            r2.pos,
            r2.n_no_boss_pos,
            r2.n_lead_pos,
            r2.n_foll_pos,
            r2.steps
        ),
        "(FF-R6) stop-and-report: two passes of one unit disagree on an accounting integer"
    );
    assert_eq!(
        (
            r.u0_diff,
            r.u1_diff,
            r.delta1_diff,
            r.mass,
            &r.frontier_digest
        ),
        (
            r2.u0_diff,
            r2.u1_diff,
            r2.delta1_diff,
            r2.mass,
            &r2.frontier_digest
        ),
        "(FF-R6) stop-and-report: two passes of one unit disagree on a summary value"
    );
    #[allow(clippy::needless_range_loop)]
    for f in 0..N_FEAT {
        assert_eq!(
            (
                &r.feats[f].resid_sum,
                &r.feats[f].capture,
                &r.feats[f].rows,
                r.feats[f].n_all,
                r.feats[f].n_some,
                r.feats[f].n_none
            ),
            (
                &r2.feats[f].resid_sum,
                &r2.feats[f].capture,
                &r2.feats[f].rows,
                r2.feats[f].n_all,
                r2.feats[f].n_some,
                r2.feats[f].n_none
            ),
            "(FF-R6) stop-and-report: two passes disagree on feature {}",
            FEAT_NAME[f]
        );
    }
    for f in 0..N_FEAT {
        assert_eq!(
            r.feats[f].theta_raws, r2.feats[f].theta_raws,
            "(FF-R6) stop-and-report: two passes disagree on a per-state theta*"
        );
    }
    match (&r.shared, &r2.shared) {
        (None, None) => {}
        (Some(a), Some(b)) => assert!(
            a.theta_raw == b.theta_raw && a.resid == b.resid && a.n_break == b.n_break,
            "(FF-R6) stop-and-report: two passes disagree on the shared-theta fit"
        ),
        _ => panic!("(FF-R6) stop-and-report: the shared fit ran in only one pass"),
    }
    drop(r2);

    // ---- (FF-R5) the rung-one invariance receipt --------------------------
    let d1_c = tax_to_count(r.delta1_diff);
    let u1_c = to_count(r.u1_diff, N4_GRADE);
    let u0_c = to_count(r.u0_diff, N4_GRADE);
    assert_eq!(
        (r.n_states, r.n_arrivals, r.zero, r.pos),
        (f_states, f_arr, f_zero, f_pos),
        "(FF-R5) stop-and-report: this probe's rung-one frontier census differs from the frozen table"
    );
    assert_eq!(
        (d1_c, u1_c, u0_c),
        (
            q(f_d1.0, f_d1.1),
            q(f_u1.0, f_u1.1),
            q(f_u0.0, f_u0.1)
        ),
        "(FF-R5) stop-and-report: this probe's Delta^(1), U^(1) or U^(0) differs from the frozen table"
    );
    assert_eq!(
        r.mass,
        qi(1),
        "stop-and-report: sum_I p_I != 1 — a field branch was dropped or double-counted"
    );
    let digest_line = if f_digest.is_empty() {
        assert_eq!(r.frontier_digest.len(), 64, "a digest is 64 hex characters");
        format!(
            "no transcribed value exists at this coordinate, so this run EMITS AND FILES it and asserts nothing against it: {}",
            r.frontier_digest
        )
    } else {
        assert_eq!(
            r.frontier_digest, f_digest,
            "(FF-R5)/(FT-R7c) stop-and-report: the frontier digest differs from the transcribed value"
        );
        format!(
            "ASSERTED EQUAL to the value transcribed from {}, {} — a comparison against a PRIOR PROCESS and not a self-comparison",
            if key.coord == 0 {
                "feature_fee_2026-08-14.txt, the first FF run, which filed this coordinate's digest for the first time"
            } else {
                "SR_FIRST in second_rung.rs, which makes it a CROSS-PROBE comparison against a different program on a different day, the canonical serialisation being byte-identical"
            },
            r.frontier_digest
        )
    };

    let _ = writeln!(
        out,
        "  (FF-R5) RUNG-ONE INVARIANCE — HELD: |I_1| = {}, arrivals {}, census {}/{} (zero/positive), Delta^(1) = {}, U^(1) = {}, U^(0) = {}, sum_I p_I = 1 exactly. (FT-R7c) THE FRONTIER DIGEST — SHA-256 over the canonical serialisation of the (record, delta_I) pairs in freeze-38(d) order, one line per state as `<record>|<delta_I as num/den, count>` — {}.",
        r.n_states,
        r.n_arrivals,
        r.zero,
        r.pos,
        qs(d1_c),
        qs(u1_c),
        qs(u0_c),
        digest_line
    );
    let _ = writeln!(
        out,
        "    (FT-R7a)'s CORRECTED SCOPE LINE, adopted verbatim: \"reaches sum_I delta_I and |supp delta_I| per unit across executions; does not reach individual delta_I.\" The digest is what extends that reach to the individual delta_I, and only once a later run carries it transcribed."
    );
    // FF-A15(i)'s claim, ASSERTED rather than assumed: at LEADING states the
    // amended F2 is identical to the frozen reading, so h0's leading
    // measurement is unaffected and stands. The frozen value is transcribed
    // from feature_fee_2026-08-14.txt and re-derived by walt-math-11 at
    // FF-A14(i); this is the FT-A28(i) pattern and it is reported under
    // (FF-R5) as a frozen-table comparison.
    let mut amend_line = String::from(
        "no filed leading-part value exists at this coordinate, so nothing is asserted here",
    );
    if key.coord == 0 {
        let fo = &r.feats[1];
        let part = BigRational::from_integer(big(fo.lead_delta));
        let lead_cap = (part.clone() - fo.lead_resid.clone()) / part;
        let filed = BigRational::new(big(2_841_944_614), big(3_716_765_745));
        assert_eq!(
            lead_cap, filed,
            "(FF-R5)/FF-A15(i) stop-and-report: the AMENDED F2 changed h0's leading-part oracle-theta capture, which FF-A15(i) says is impossible because the amendment is identical to the frozen reading at leading states. Filed {} against recomputed {}",
            bqs(&filed),
            bqs(&lead_cap)
        );
        amend_line = format!(
            "ASSERTED EQUAL to the filed {} — so the amendment of FF-A12/FF-A15(i) provably did NOT disturb h0's leading measurement, which is the claim FF-A15(i) makes and which is here checked rather than taken on trust",
            bqs(&filed)
        );
    }
    let _ = writeln!(
        out,
        "  (FF-R5) AMENDMENT INVARIANCE at the leading part (FF-A15(i), the FT-A28(i) frozen-value pattern): {amend_line}."
    );
    let _ = writeln!(
        out,
        "  (FF-R6) DETERMINISM — HELD: a full in-run second pass with fresh maps, accumulators and budgets recomputed this unit entire; every accounting integer, every summary value, the frontier digest and every printed row of all three features asserted identical."
    );
    let _ = writeln!(
        out,
        "  STATE CENSUS: {} frontier states, {} with delta_I > 0 (the swept support; states with delta_I = 0 are counted and skipped per freeze 52(b), where Lemma FF-min(d) gives delta_I^theta* = 0 and there is nothing to capture). Of the swept states, LEADING {} and FOLLOWING {}. States with NO OUTSTANDING TRUMP: {} of the swept support, {} of all {} states — at those every feature is 0 for every b (FF-A4) and Proposition FF-blind makes the capture zero there.",
        r.n_states, r.pos, r.n_lead_pos, r.n_foll_pos, r.n_no_boss_pos, r.n_no_boss_all, r.n_states
    );

    #[allow(clippy::needless_range_loop)]
    for f in 0..N_FEAT {
        let fo = &r.feats[f];
        let denom2 = BigRational::from_integer(big(2) * big(DEN_MU * i128::from(SCALE) * 34_650));
        let resid_c = fo.resid_sum.clone() / denom2.clone();
        let (ppm_lo, ppm_hi) = ppm_bracket(&fo.capture);
        let _ = writeln!(out);
        let _ = writeln!(out, "  -- FEATURE {} ({}) --", FEAT_NAME[f], FEAT_KIND[f]);
        if f == 0 {
            let _ = writeln!(
                out,
                "    (FF-R1) THE NULL-CONTROL RECEIPT — HELD, BLOCKING, asserted at every one of the {} swept states BEFORE any F1 or F2 number existed: delta_I^theta* = delta_I exactly, theta* = 0 by freeze 52(e), zero breakpoints, and the per-unit oracle-theta capture = 0 EXACTLY. This is the only check in the build whose answer is known BY PROOF (Proposition FF-blind) rather than by a filed rational — the (SR-R9) role. It tests the per-action centring, the breakpoint enumeration, the sweep, the exact accumulation and the tie rule at once.",
                r.pos
            );
        }
        let _ = writeln!(
            out,
            "    ORACLE-THETA CAPTURE OVER THE {} SWEPT STATES OF THIS UNIT (all states with delta_I > 0; the denominator is this unit's Delta^(1)) = {} (exact rational; convention-free, being a ratio of two taxes) = between {} and {} parts per million (PRESENTATION ONLY, exact integer floor/ceil, enters no proof and no comparison).",
            r.pos,
            bqs(&fo.capture),
            ppm_lo,
            ppm_hi
        );
        let _ = writeln!(
            out,
            "    sum_I delta_I^theta* OVER THE {} SWEPT STATES = {} (count)  against  Delta^(1) OVER THE SAME {} SWEPT STATES = {} (count).  Breakpoints enumerated across those {} states: {} — and by Proposition FF-degen a zero capture with a POSITIVE breakpoint count is a MEASUREMENT (the feature was priced and failed), while a zero capture with ZERO breakpoints is a TAUTOLOGY (the feature had no content here).",
            r.pos,
            bqs(&resid_c),
            r.pos,
            qs(d1_c),
            r.pos,
            fo.n_break_total
        );
        let _ = writeln!(
            out,
            "    THREE-WAY CENSUS (freeze 52(f)) over the {} swept states: ALL captured (delta_I^theta* = 0) = {} ; SOME captured (0 < delta_I^theta* < delta_I) = {} ; NONE captured (delta_I^theta* = delta_I) = {}.",
            r.pos, fo.n_all, fo.n_some, fo.n_none
        );
        let lead_cap = if fo.lead_delta == 0 {
            "n/a (no leading states in the support)".to_string()
        } else {
            bqs(
                &((BigRational::from_integer(big(fo.lead_delta)) - fo.lead_resid.clone())
                    / BigRational::from_integer(big(fo.lead_delta))),
            )
        };
        let foll_cap = if fo.foll_delta == 0 {
            "n/a (no following states in the support)".to_string()
        } else {
            bqs(
                &((BigRational::from_integer(big(fo.foll_delta)) - fo.foll_resid.clone())
                    / BigRational::from_integer(big(fo.foll_delta))),
            )
        };
        let _ = writeln!(
            out,
            "    LEADING/FOLLOWING SPLIT (FF-A4, reported separately because F1's motivating reading is the leading one): LEADING {} states, sum delta_I = {} (count), oracle-theta capture on that part = {} ; FOLLOWING {} states, sum delta_I = {} (count), oracle-theta capture on that part = {}. FF-A8(d): a sharp difference here is a scope fact about F1's DEFINITION, not a fact about 42, and is read against h2 being wholly leading and h0 mixed.",
            fo.n_lead,
            qs(tax_to_count(Q::new(fo.lead_delta, DEN_MU * i128::from(SCALE) * 34_650))),
            lead_cap,
            fo.n_foll,
            qs(tax_to_count(Q::new(fo.foll_delta, DEN_MU * i128::from(SCALE) * 34_650))),
            foll_cap
        );
        let _ = writeln!(
            out,
            "    (FF-R2) CENTRING — HELD at every one of the {} swept states and at EVERY action b, not only the argmax b: sum_omega mu_I(omega) lambda_I(omega,b) = 0 exactly at the reported theta*. This is Theorem 12.1's hypothesis and precisely what the per-state centring of FF-A2 fails; by construction is not a receipt (PG-A8).",
            r.pos
        );
        let _ = writeln!(
            out,
            "    (FF-R3) BOUND — HELD at every swept state: 0 <= delta_I^theta* <= delta_I, with delta_I taken FROM THE FRONTIER PASS. It is a receipt ONLY because it compares against the frontier pass's delta_I — the swept minimum and delta_I are different computations, so a sweep bug shows up here. It is NOT a Proposition SR-taut identity."
        );
        let _ = writeln!(
            out,
            "    (FF-R4) DIRECT EVALUATION — HELD at every swept state: G_I(theta*) recomputed by direct summation over worlds with NO incremental state agrees exactly with the swept value. Two independently written paths sharing only the inputs."
        );
        let _ = writeln!(
            out,
            "    ARITHMETIC REMARKS, NOT RECEIPTS (Proposition SR-taut, FF-A7(vii); they cannot fail and are excluded from every HELD count): delta_I^theta* >= 0 AS RE-DERIVED FROM THIS PROBE'S OWN G AND GLUED VALUE; capture <= 1; sum_I (delta_I - delta_I^theta*) = Delta^(1) - sum_I delta_I^theta*; and the convexity of G_I."
        );
        let _ = writeln!(
            out,
            "    PER-STATE ROWS (every swept state, freeze 52(b): c_I(b) for every b, the breakpoint count, theta*, delta_I^theta*, and the captured amount; freeze-38(d) record order; COUNT convention on every tax column, a tax being exactly half its differential value):"
        );
        for row in &fo.rows {
            let _ = writeln!(out, "{row}");
        }
    }

    // ---- FF-A15(iii): the shared-theta fit, and the ratio the programme wants
    if let Some(sf) = &r.shared {
        let fo = &r.feats[1];
        let part = BigRational::from_integer(big(sf.part_delta));
        assert_eq!(
            sf.part_delta, fo.lead_delta,
            "the shared fit and the oracle sweep disagree on the part's summed delta_I"
        );
        let shared_cap = (part.clone() - sf.resid.clone()) / part.clone();
        let oracle_cap = (part.clone() - fo.lead_resid.clone()) / part.clone();
        // Lemma FF-min applied to the pooled objective, and Proposition
        // FF-oracle: R_free <= R_shared, hence capture_free >= capture_shared.
        // Contentful — the two are different computations over the same part.
        assert!(
            fo.lead_resid <= sf.resid,
            "stop-and-report: the shared-theta residual is below the per-state oracle residual, contradicting Proposition FF-oracle"
        );
        assert!(
            !sf.resid.is_negative() && sf.resid <= part,
            "stop-and-report: the shared-theta residual is outside [0, sum_I delta_I]"
        );
        let ratio = if oracle_cap.is_zero() {
            BigRational::zero()
        } else {
            shared_cap.clone() / oracle_cap.clone()
        };
        let (s_lo, s_hi) = ppm_bracket(&shared_cap);
        let (r_lo, r_hi) = ppm_bracket(&ratio);
        let mut distinct: Vec<BigRational> = fo
            .theta_raws
            .iter()
            .filter(|(lead, _)| *lead)
            .map(|(_, t)| t.clone())
            .collect();
        let n_part = distinct.len();
        let n_at_shared = distinct.iter().filter(|t| **t == sf.theta_raw).count();
        let n_zero = distinct.iter().filter(|t| t.is_zero()).count();
        distinct.sort();
        let lo = distinct.first().cloned().unwrap_or_else(BigRational::zero);
        let hi = distinct.last().cloned().unwrap_or_else(BigRational::zero);
        distinct.dedup();
        let scale2 = BigRational::from_integer(big(2) * big(i128::from(SCALE)));
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "  == FF-A15(iii): THE SHARED-THETA FIT FOR F2 ON THE LEADING PART — COMMISSIONED, AND THE RATIO IS THE NUMBER THE PROGRAMME WANTS =="
        );
        let _ = writeln!(
            out,
            "    THE OBJECT: min_theta sum_I delta_I^theta over the {} leading states with ONE theta, exactly minimised. The sum of convex piecewise-linear functions is convex piecewise-linear, so Lemma FF-min applies VERBATIM to the pooled objective with the UNION of the per-state breakpoints; freeze 52(e)'s tie rule is applied verbatim to the pooled function (the smallest pooled breakpoint attaining the minimum). Pooled distinct breakpoints: {}. No grid, no float; the pooled slope and intercept are exact rationals, because the per-state parameterisation sigma_I = theta/p_num_I differs from state to state and the common theta must be carried exactly.",
            n_part, sf.n_break
        );
        let _ = writeln!(
            out,
            "    SHARED theta* OVER THE {} LEADING STATES OF h0 = {} (count convention).",
            n_part,
            bqs(&(sf.theta_raw.clone() / scale2.clone()))
        );
        let _ = writeln!(
            out,
            "    SHARED-THETA CAPTURE OVER h0'S {} LEADING STATES (one theta pooled across exactly those {} states; the denominator is sum_I delta_I over the same {}) = {} (exact rational) = between {} and {} parts per million (PRESENTATION ONLY).",
            n_part,
            n_part,
            n_part,
            bqs(&shared_cap),
            s_lo,
            s_hi
        );
        let _ = writeln!(
            out,
            "    ORACLE-THETA CAPTURE OVER THE SAME {} LEADING STATES OF h0 = {} (exact rational), and sum_I delta_I^theta* over those {} states is {} shared against {} oracle, over sum_I delta_I = {} on the same {} (all count convention).",
            n_part,
            bqs(&oracle_cap),
            n_part,
            bqs(&(sf.resid.clone() / BigRational::from_integer(big(2) * big(DEN_MU * i128::from(SCALE) * 34_650)))),
            bqs(&(fo.lead_resid.clone() / BigRational::from_integer(big(2) * big(DEN_MU * i128::from(SCALE) * 34_650)))),
            qs(tax_to_count(Q::new(sf.part_delta, DEN_MU * i128::from(SCALE) * 34_650))),
            n_part
        );
        let _ = writeln!(
            out,
            "    ==> SHARED / ORACLE RATIO, BOTH TAKEN OVER h0'S {} LEADING STATES AND OVER NO OTHER SET = {} (exact rational) = between {} and {} parts per million (PRESENTATION ONLY). This is the quantity FF-A15(iii) names as the one the whole programme actually wants: how much of the per-state oracle's capture survives collapsing {} free rationals to ONE. It is a ratio of two captures over the SAME {} states and is meaningless against any other state set.",
            n_part,
            bqs(&ratio),
            r_lo,
            r_hi,
            n_part,
            n_part
        );
        let _ = writeln!(
            out,
            "    PER-STATE theta* DISTRIBUTION OVER THE SAME {} LEADING STATES OF h0 (FF-A15(iii); summarised from rows already emitted): {} states, {} DISTINCT values of theta*, {} of them exactly 0, and {} states whose own optimal theta* equals the shared theta*. Range in the count convention: min {} , max {}.",
            n_part,
            n_part,
            distinct.len(),
            n_zero,
            n_at_shared,
            bqs(&(lo / scale2.clone())),
            bqs(&(hi / scale2))
        );
        let _ = writeln!(
            out,
            "    WHAT THIS DOES AND DOES NOT LICENSE (Proposition FF-oracle, FF-A8(c)): the shared fit is the ONE follow-on FF-A8(c) permits and it is not a licence for anything further. R_free <= R_shared is asserted in-run, so the ratio is at most 1 by theorem and its value is a measurement of how much structure survives pooling. NO SENTENCE HERE SAYS F2 \"WORKS\"."
        );
    }

    let _ = writeln!(
        out,
        "  walk-step observables (SEP-A19(b) class, PROVENANCE ONLY — never a cost, timing or tractability claim): the rung-one frontier pass charged {} walk-steps, residual {}; the filed FT PATH A subtotal for this (coordinate, action) is {f_steps} — {}. The sweep itself is negligible beside the traversal and is not separately budgeted.",
        r.steps,
        r.residual,
        if r.steps == f_steps { "EQUAL" } else { "DIFFERS" }
    );
    let _ = writeln!(
        out,
        "  SETTLED A PRIORI (FF-A8(f)): by Proposition SR-degen no closure verdict exists at grade 4 for this coordinate, and A SHAVE MEASURED HERE CHANGES NO VERDICT. The entire value of this unit is as a SCREENING MEASUREMENT for trick 1, where FT-A21's three obligations are the binding constraint. Any sentence implying a grade-4 verdict moved is void on its face."
    );

    UnitText {
        named: out,
        stopped: false,
    }
}

// -- main --------------------------------------------------------------------

#[allow(clippy::too_many_lines)]
fn header(out: &mut String) {
    let _ = writeln!(
        out,
        "walt feature-fee audition, THE CORRECTED RE-RUN — FREEZE 52 v1.1, F2 ONLY, with the shared-theta fit of FF-A15(iii) in the same pass — EXPLORATORY TIER"
    );
    let _ = writeln!(
        out,
        "rulings: FF-A1..FF-A24 (walt/CENSUS-RULINGS.md 2026-08-14) — this build is commissioned at FF-A15(ii) and FF-A15(iii) and runs under FREEZE 52 v1.1 (the domain clause of FF-A15(i)) and FREEZE 52 v1.3 (the sweep/census separation of FF-A23(iv)), with FF-A18's state-set naming rule and FF-A23(ii)'s sweep ruling binding on every figure; Proposition FF-blind, Lemma FF-min, Proposition FF-oracle, Proposition FF-degen; beneath them the FT and SR chapters entire, with Proposition FT-flat, Lemma FT-arrive, Lemma FT-post, Corollary FT-conv, Corollary FT-grade4, Proposition SR-degen and Proposition SR-taut; FT-A13(i)/(iii) and SR-A12(i)/SR-A14(v) on centring; freezes 26, 37(d), 38 v1.1, 44 v2, 45, 50 v1.1"
    );
    let _ = writeln!(
        out,
        "regenerate (deterministically, from the repository alone): cargo run --release -p walt-factory --example feature_fee_v11 — NOTE the _v11 suffix: `--example feature_fee` is the FIRST probe and regenerates the OTHER file, which this one supersedes in part and must not overwrite"
    );
    let _ = writeln!(out, "freeze-set digest (freeze 52(g)): {FF_DIGEST}");
    let _ = writeln!(
        out,
        "emission: this file is committed ENTIRE — there is no companion (FF-A9(iv)); the build's emission is small enough to commit and it should be."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "TIER: exploratory throughout, without exception (FF-A1), below every tier like everything in walt. Nothing here is promoted, nothing is cited by anything above this tier, and nothing is quotable as a result except by brief amendment adding it to a verifier receipt. DS-A1 binds: witness, receipt, necessary outer profile. Both outcomes of every gate are results (F7), WITH PROPOSITION FF-oracle'S ASYMMETRY ATTACHED TO EVERY ONE. A receipt failure is stop-and-report, never a patch (NO-RESCUE)."
    );
    let _ = writeln!(
        out,
        "THE FEATURE IS JASON'S AND ITS PROVENANCE IS TABLE REASONING ABOUT A REAL HAND (FF-A1); that is a perfectly good source of a hypothesis and no kind of evidence for it. The audition exists precisely to price it, and a null result costs the hypothesis nothing but its candidacy."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== PROPOSITION FF-oracle, PRINTED BEFORE ANY NUMBER EXISTS — THE READING IT FORCES =="
    );
    let _ = writeln!(
        out,
        "  Optimising theta INDEPENDENTLY AT EVERY STATE spends one free rational per information state — 1,332 at h0's positive support and 216 at h2's — which is A LOOKUP TABLE, NOT A FEATURE BASIS. Writing R_free = sum_I min_theta delta_I^theta and R_shared = min_theta sum_I delta_I^theta, we have R_free <= R_shared and hence capture_free >= capture_shared, and the same against ANY fee family whose parameter depends on less than the full identity of I. Proof: for every theta, R_free = sum_I min_theta' delta_I^theta' <= sum_I delta_I^theta; minimise the right side over theta."
    );
    let _ = writeln!(
        out,
        "  THEREFORE THE TWO OUTCOMES ARE NOT LOGICALLY SYMMETRIC. A SMALL oracle-theta capture REFUTES THE FEATURE CONCLUSIVELY, because no shared or coarser parameterisation can beat the per-state oracle. A LARGE oracle-theta capture ESTABLISHES NOTHING about a usable fee family and licenses exactly one thing: the next experiment. F7 says both outcomes are results; it does not say they are results of equal strength, and here they are not."
    );
    let _ = writeln!(
        out,
        "  BINDING: the column is named ORACLE-THETA CAPTURE in every artifact and every sentence, never \"capture\" unqualified and never \"the feature's capture\"."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== PROPOSITION FF-blind, PRINTED BEFORE ANY NUMBER EXISTS — WHY F0 IS THE NULL CONTROL =="
    );
    let _ = writeln!(
        out,
        "  Fix a frontier state I with p_I > 0, let psi_I(omega) be any world-feature NOT depending on the action, let c_I = sum_omega mu_I psi_I / p_I and lambda_I(omega,b) = theta(psi_I(omega) - c_I). Then lambda_I is centred for every b, the penalised local value is unchanged, and delta_I^lambda = delta_I EXACTLY, for every theta — THE CAPTURED FRACTION IS IDENTICALLY ZERO. Proof: centring is immediate from the definition of c_I; because lambda does not depend on b it passes through the inner maximum, giving max_b[q - lambda] = m(omega) - lambda(omega); summing against mu_I and using centring once more kills the lambda term."
    );
    let _ = writeln!(
        out,
        "  It is the penalty-side twin of Proposition FT-flat, and with Proposition T1-blind on the primal side the branch now has ONE LESSON PROVED THREE TIMES IN THREE FORMALISMS: A WITNESS, A BOUND OR A FEE MUST BE CONDITIONED ON THE DECISION IT IS TRYING TO PRICE. The scope is equally precise: it constrains phi as a function of b only. It says nothing against a b-dependent but crude feature, and nothing against a feature whose b-dependence is weak."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== ALL OUTCOMES PRE-DECLARED (FF-A8, printed before any number exists; F7 binds) =="
    );
    let _ = writeln!(
        out,
        "  (a) F0's oracle-theta capture is ANYTHING OTHER THAN EXACTLY 0 -> STOP-AND-REPORT. The harness is wrong, and NO F1 OR F2 NUMBER IS REPORTED, EMITTED OR DISCUSSED. This gate is BLOCKING and comes first."
    );
    let _ = writeln!(
        out,
        "  (b) F1's oracle-theta capture is SMALL at both arms -> THE FEATURE IS REFUTED, AND CONCLUSIVELY, because by Proposition FF-oracle no shared or coarser parameterisation can do better than the per-state oracle. THIS IS A RESULT AND THE MOST LIKELY ONE; it costs the hypothesis its candidacy and nothing else, and it is filed as a result under F7, NOT as a null. The same for F2."
    );
    let _ = writeln!(
        out,
        "  (c) F1's or F2's oracle-theta capture is LARGE -> NOTHING IS ESTABLISHED ABOUT A USABLE FEE FAMILY (Proposition FF-oracle), and no claim is made beyond the measurement. It licenses exactly one thing: a follow-on ruling commissioning the shared-theta fit and then the joint two-feature fit, which are DIFFERENT EXPERIMENTS WITH DIFFERENT OBJECTS. NO ARTIFACT OF THIS BUILD MAY SAY A FEATURE \"WORKS\"."
    );
    let _ = writeln!(
        out,
        "  (d) The LEADING/FOLLOWING split differs sharply -> reported as a SCOPE FACT ABOUT F1'S DEFINITION (FF-A4), not as a fact about 42, and read against h2 being wholly leading and h0 mixed."
    );
    let _ = writeln!(
        out,
        "  (e) A budget stop, or arm 2 not reached -> DECLARED STOP, no partial fold, no partial capture, printed as a stop and never as a finding (R-A18, freeze 44(b))."
    );
    let _ = writeln!(
        out,
        "  (f) SETTLED A PRIORI, reportable only as such: by Proposition SR-degen NO CLOSURE VERDICT EXISTS AT GRADE 4 for either coordinate, and A SHAVE MEASURED HERE CHANGES NO VERDICT — h0's binding pair is untied and already closes at rung two, h2's is tied and terminates at equality. THE ENTIRE VALUE OF THIS BUILD IS AS A SCREENING MEASUREMENT FOR TRICK 1, where FT-A21's three obligations are the binding constraint. Any sentence in this artifact implying a grade-4 verdict moved is VOID ON ITS FACE."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "P-A21 — THE FENCE MOST AT RISK HERE AND THEREFORE PRINTED FIRST AND LARGEST (FF-A9(i)): NO QUANTITY MEASURED AT GRADE 4 IS QUOTED FOR TRICK 1 OR FOR THE OPENING. This is a screening experiment AIMED at trick 1 and its numbers are grade-4 numbers; the aim is what makes the fence fragile, not what relaxes it."
    );
    let _ = writeln!(
        out,
        "SELECTION FENCE (SR-A25(iii), FT-A26(iii)), which binds HARDER here than at rung two: THREE UNITS AT TWO COORDINATES CHOSEN BY NEGATIVE BINDING MARGIN ARE A CARRIER, NOT A SAMPLE, and a capture fraction is exactly the kind of number that reads like a rate. It is not a rate."
    );
    let _ = writeln!(
        out,
        "FENCE (R-A2, P-A1): no object produced by this probe is an identity-bearing witness of anything; reachability is a proof-irrelevant proposition; the carrier is the void-free capacity fiber whose members are FEASIBLE and never reachable."
    );
    let _ = writeln!(
        out,
        "REAL-DEAL FENCE (N4-A8, verbatim): the hands and pools come from rob's receipt corpus, THE BELIEF DOES NOT. The voids the play record had already revealed are deliberately discarded, and support is not belief in any case. No row here is a statement about correct play in that deal, about reachability, or about any belief other than the declared one."
    );
    let _ = writeln!(
        out,
        "NOT CLAIMED, printed in place (FF-A9(ii)): nothing about points or marks; nothing about bidding; nothing about how real opponents play; no cost or tractability claim off any traversal observable (SEP-A19(b)); and NOTHING WHATEVER ABOUT WHETHER JASON'S READING OF h0 AT THE TABLE IS CORRECT — the feature is being priced as a fee, and a fee's capture is not a statement about the reasoning that suggested it."
    );
    let _ = writeln!(
        out,
        "THE LOAD-BEARING RISK, undiminished (SR-A25(vii)): if the implementation and the rules corpus disagree, the mathematics is still correct and its application here is wrong, and no receipt inside this file can detect it, because every receipt is computed by the same implementation. (FF-R1) is the partial guard — it is the one quantity whose value is known by proof."
    );
    let _ = writeln!(
        out,
        "THE FEE AND ITS CENTRING (FF-A2, freeze 52(c)): lambda_I(omega,b) = theta * (phi_I(omega,b) - c_I(b)) with c_I(b) = sum_omega mu_I(omega) phi_I(omega,b) / p_I. PER-ACTION CENTRING IS MANDATORY AND RECEIPTED at (FF-R2). A SINGLE PER-STATE CENTRE DOES NOT SATISFY THEOREM 12.1'S HYPOTHESIS when phi depends on b, and a fee that is not centred per action IS NOT A VALID UPPER WITNESS AT ALL — the resulting number would be neither an upper nor a lower bound on anything. This is FT-A13(i)'s \"the centering law is indexed by b\" one rung down, and it is the single most likely way this build could have produced a plausible wrong number."
    );
    let _ = writeln!(
        out,
        "EXACT MINIMISATION (Lemma FF-min, freeze 52(d)): the breakpoints are enumerated, G_I is evaluated at each, and the least is taken. NO GRID, NO SEARCH, NO FLOAT (P-A19; clippy -D float_arithmetic and the no-float grep bind). The evaluation is made O(1) per breakpoint by sorting the breakpoints and sweeping an exactly maintained (intercept, slope) pair — a cheaper way to perform the SPECIFIED evaluation, not a different algorithm — and (FF-R4) re-evaluates the winner by direct summation with no incremental state. Every division that must be exact is asserted to be exact, and every breakpoint and evaluation multiplication is CHECKED: an overflow is stop-and-report, not a wrap."
    );
    let _ = writeln!(
        out,
        "THE TIE RULE (freeze 52(e)), declared before the run and never chosen by result: theta* is the SMALLEST breakpoint attaining the minimum, in ascending rational order; if a state has no breakpoints, theta* = 0. The sweep visits breakpoints in ascending order and updates only on a STRICT decrease, so the first minimiser is the one kept."
    );
    let _ = writeln!(
        out,
        "CONVENTION (freeze 38(f), Corollary SR-conv): every evaluator runs in the trick DIFFERENTIAL; every reported TAX column is in the COUNT convention, a tax being a difference at a common state and therefore exactly HALF its differential value. THE ORACLE-THETA CAPTURE IS A RATIO OF TWO TAXES AND IS THEREFORE CONVENTION-FREE. theta* is a fee coefficient in value units and is reported in the count convention beside its exact internal parameter sigma* = theta*/p_num."
    );
    let _ = writeln!(
        out,
        "BELIEF AND FIELD ARE NOT RE-DECLARED (freeze 52(g)): freeze 26 and 37(d), uniform over the full enumerated fiber, NO DECIMATION inside anything ((C2)). No library entry is written at any coordinate (freeze 45)."
    );
    let _ = writeln!(
        out,
        "RUNTIME (FF-A9(iii)): the 2-5 minute target IS A TARGET, NEVER A RECEIPT (SEP-A19(b)). No outcome of this build turns on it, and if arm 2 does not fit, FF-A8(e) is the declared answer rather than a re-scope mid-run. Wall-clock and walk-step columns are provenance."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== WHAT THIS PASS IS, AND WHY IT EXISTS (FF-A11, FF-A12, FF-A15) =="
    );
    let _ = writeln!(
        out,
        "  THIS IS THE CORRECTED RE-RUN COMMISSIONED AT FF-A15(ii). The first FF run was adjudicated at FF-A10..FF-A16 with NO DEVIATION FOUND against its contract, but walt-math-11 found TWO DEFECTS IN ITS OWN SPEC, and both are the adjudicator's and neither is the build's."
    );
    let _ = writeln!(
        out,
        "  DEFECT ONE (FF-A11): FF-A4 attached the no-outstanding-trump fallback to ALL THREE features, but only F0 and F1 reference the boss-trump holder h(omega). F2 never mentions h(omega) and is perfectly well defined with no trump outstanding — indeed that is when it is MOST interesting, because control then turns entirely on suit rank. The clause therefore set phi == 0 for F2 at every h2 state and at h0's 758 following states, giving zero breakpoints and, by Proposition FF-degen, capture zero AS A TAUTOLOGY. SIX OF THE TWELVE (feature, unit) CELLS WERE VACUOUS BY CONSTRUCTION, and F2 at h2 — the measurement the build should have produced — IS UNMEASURED, NOT ZERO."
    );
    let _ = writeln!(
        out,
        "  DEFECT TWO (FF-A12): F2's frozen definition was AMBIGUOUS AT FOLLOWING STATES, where a tile already on the table may already beat b; it did not say whether the current winner counts. FREEZE 52 v1.1 fixes it explicitly, below. At h0's following states F2 was thus doubly unusable, and the first defect accidentally prevented an ill-defined number from being printed, which is luck and not design."
    );
    let _ = writeln!(
        out,
        "  WHAT THIS PASS MEASURES: F2 ONLY, under the amended definition, at the states the clause voided. F0 IS STILL RE-RUN FIRST AND BLOCKING purely as the null control — its verdict is known by theorem, but THE CONTROL IS WHAT MAKES THE NEW NUMBERS TRUSTWORTHY. F1 is not computed: it is settled at FF-A13, refuted at h0's leading part and inapplicable everywhere else in this carrier."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== SUPERSESSION — WHAT THIS FILE REPLACES IN feature_fee_2026-08-14.txt AND WHAT IT DOES NOT (FF-A23, LD-A11(ii)) =="
    );
    let _ = writeln!(
        out,
        "  TWO RESULTS FILES FOR ONE EXPERIMENT IS A READER TRAP, so this says exactly which is which. The earlier file is NOT overwritten and NOT annotated: FF-A10..FF-A22 cite it by content and by line, and FF-A11's typing of the six vacuous cells is a ruling ABOUT THAT FILE AS IT STANDS, so overwriting it would silently invalidate every citation in the chapter and erase the evidence for the defect it records. CLAUDE.md also bars hand-editing a results file."
    );
    let _ = writeln!(
        out,
        "  SUPERSEDED BY THIS FILE — the six cells FF-A11 typed as vacuous by construction: F2 at BOTH h2 units, and F2's h0 FOLLOWING part. Those were suppressed by FF-A4's blanket clause and are measured here for the first time under freeze 52 v1.1's domain clause and FF-A12's amended definition."
    );
    let _ = writeln!(
        out,
        "  ALSO SUPERSEDED — the earlier file's WHOLE-UNIT F2 figure at h0, 5683889228/7647562615 over its 1,332 swept states. FF-A14(ii) typed that figure as A LOWER BOUND and never as F2's capture, because it averaged the genuine leading measurement against 758 states forced to zero by the clause. This file replaces it with the MEASURED whole-unit value over the same 1,332 swept states, and the bound held: the measured figure is the larger. The earlier figure must no longer be quoted even as a bound, because a measurement of the same quantity over the same state set now exists."
    );
    let _ = writeln!(
        out,
        "  STANDING, UNALTERED AND NOT RE-DERIVED — F0 everywhere (settled by Proposition FF-blind), F1 everywhere (settled at FF-A13: refuted at h0's 574 leading states and inapplicable elsewhere in this carrier), and F2'S h0 LEADING MEASUREMENT, which FF-A15(i) shows the amendment cannot disturb. F2's h0 leading figure is nonetheless RECOMPUTED here and ASSERTED against the filed value, because the re-run passes through it anyway and a free cross-check between two programs is worth taking."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== FF-A18'S BINDING RULE, WHICH GOVERNS EVERY FIGURE BELOW =="
    );
    let _ = writeln!(
        out,
        "  NO COUNT AND NO CAPTURE FIGURE APPEARS ANYWHERE IN THIS FILE WITHOUT THE STATE SET IT RANGES OVER NAMED IN THE SAME SENTENCE. The rule exists because the adjudicator broke it twice — FF-A18 attached a whole-unit rational to a leading-state sentence, and FF-A23's \"all 330\" carried a total-state count into a swept-state instruction — and because SCOPE MISLABELLING IS THE STANDING HAZARD OF A FILE THAT REPORTS THE SAME QUANTITY OVER NESTED STATE SETS. This file reports captures over three nested sets at h0 (1,332 swept; 574 leading; 758 following) and one at h2 (216 swept), and every rational below names which."
    );
    let _ = writeln!(
        out,
        "  FREEZE 52 v1.3 (FF-A23(iv), clarifying v1.2): the FEATURE-DOMAIN CENSUS is emitted over EVERY frontier state, because it is a SCREEN whose purpose is to characterise whether the feature has content at the coordinate at all; the SWEPT CENSUS is over the swept support, because a state with no tax has nothing to price. BOTH are printed and BOTH are labelled with the set they count."
    );
    let _ = writeln!(
        out,
        "  FREEZE 52(b) GOVERNS THE SWEEP (FF-A23(ii)): the positive-delta_I states per unit are swept — 1,332 of h0's 16,136 and 216 of h2's 330 — and the delta_I = 0 states are counted and skipped. FF-A15(ii)'s \"all 330 h2 states\" was corrected in place to \"F2 at every swept h2 state — 216 per unit\". NOTHING NUMERIC TURNS ON THIS (FF-A23(iii)): Delta^(1) over all states equals the sum over swept states, and Lemma FF-min(b)+(d) force 0 <= delta_I^theta* <= delta_I = 0 at every skipped state, so the oracle-theta capture, the shared-theta capture and their ratio are identical under either reading — only the emitted row set and the census counts differ."
    );
    let _ = writeln!(
        out,
        "== PROPOSITION FF-degen, PRINTED BEFORE ANY NUMBER EXISTS — ZERO BREAKPOINTS IS EXACTLY VACUITY, AND IT IS EMITTED =="
    );
    let _ = writeln!(
        out,
        "  At a frontier state I with |A(I)| >= 2, the breakpoint set of Lemma FF-min is empty IF AND ONLY IF Phi_I(omega,b) does not depend on b at any positive-mass omega; and in that case the fee is action-blind on I, so by Proposition FF-blind delta_I^theta = delta_I for every theta and the state's captured amount is exactly 0. Proof: a breakpoint exists iff some positive-mass omega and some b != b' have Phi_I(omega,b) != Phi_I(omega,b'); its absence is exactly the stated constancy in b, whence lambda_I(omega,b) = theta*rho_I(omega) for a b-free rho_I, which is centred, and FF-blind applies."
    );
    let _ = writeln!(
        out,
        "  WHY IT IS PRINTED: freeze 52(b) requires the per-state breakpoint count, so this artifact SEPARATES \"the feature was priced and failed\" from \"the feature had no content here\" MECHANICALLY, without re-running anything. A capture of zero at a state with thousands of breakpoints is a MEASUREMENT; a capture of zero at a state with none is a TAUTOLOGY. Every zero below can therefore be typed, and the per-state breakpoint count is emitted on every row for exactly that reason."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== THE FEATURES OF THIS PASS (FREEZE 52 v1.1; every feature carries ITS OWN domain clause, and no blanket clause spans the family) =="
    );
    let _ = writeln!(
        out,
        "  \"OUTSTANDING TRUMP\" means the highest-ranking trump under the declaration among tiles NOT YET PLAYED IN THE RECORD AND NOT IN THE FOCAL SEAT'S HAND AT I — i.e. held by a field seat in omega; h(omega) is the seat holding it. Its IDENTITY is a function of the record and that invariance is ASSERTED at every arrival; only its HOLDER varies across X_I."
    );
    let _ = writeln!(
        out,
        "  (F0) NULL CONTROL — boss_owner: phi(omega,b) = 1 if h(omega) is an opponent of the focal seat, 0 if a partner. DOMAIN CLAUSE: F0 REFERENCES h(omega), so it is 0 where no trump is outstanding. ACTION-BLIND. PRE-DECLARED EXACT PREDICTIONS, BOTH OF THEM: oracle-theta capture = 0 (Proposition FF-blind) AND zero breakpoints (Proposition FF-degen). Two independent predictions on the same states, which is what makes this control worth re-running."
    );
    let _ = writeln!(
        out,
        "  (F2) THE MEASUREMENT — b_is_beatable, AMENDED at FF-A12 and freeze 52 v1.1: phi(omega,b) = 1 iff some opponent WHO HAS NOT YET PLAYED AT I holds a tile that, if played, would win the trick OVER b AND OVER EVERY TILE ALREADY ON THE TABLE. DOMAIN CLAUSE: F2 does NOT reference h(omega) and its domain is THE WHOLE FIBER, whether or not a trump is outstanding. Computed from Decl::beats and Decl::trick_key — the rule algebra's own BEATS relation and ordering — and NEVER by a re-implementation."
    );
    let _ = writeln!(
        out,
        "  AT LEADING STATES THE AMENDED F2 IS IDENTICAL TO THE FROZEN READING (FF-A15(i)): nothing is on the table, so the current best is b itself, and a leading focal seat means all three field seats are still to play, so \"yet to play\" excludes nobody. h0's leading measurement is therefore untouched and stands — AND THAT IS ASSERTED IN-RUN, not taken on trust, by comparing this pass's h0 leading-part capture against the filed value under (FF-R5)."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== THE THREE PRE-DECLARED h2 READINGS (FF-A15(ii)), PRINTED BEFORE ANY h2 NUMBER EXISTS — THEY ARE THREE DIFFERENT FINDINGS AND MUST NOT BE COLLAPSED =="
    );
    let _ = writeln!(
        out,
        "  (h2-1) A LARGE h2 capture -> a second IN-SCOPE datum for F2, and it STILL LICENSES NOTHING about a shared family (Proposition FF-oracle)."
    );
    let _ = writeln!(
        out,
        "  (h2-2) A SMALL OR ZERO h2 capture WITH A LARGE BREAKPOINT COUNT -> F2 IS REFUTED AT h2 CONCLUSIVELY, and this is THE MORE INTERESTING OUTCOME, because h2 is the pure-suit-rank regime: the focal seat holds the top trumps and control questions there are exactly what F2 was written for."
    );
    let _ = writeln!(
        out,
        "  (h2-3) ZERO BREAKPOINTS AGAIN at h2 -> F2 IS CONSTANT ACROSS THE FIBER THERE. This is a THIRD possibility and must be reported as such rather than as either of the other two; by Proposition FF-degen it is a tautological zero and not a measurement of the feature."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== FF-A14(ii), BINDING ON HOW THE FIRST RUN'S NUMBER MAY BE QUOTED =="
    );
    let _ = writeln!(
        out,
        "  The first run's whole-unit h0 figure 5683889228/7647562615 (about 74.32%) is NOT void but is NOT F2's capture either — IT IS A LOWER BOUND, because it averages the genuine leading measurement against 758 states forced to zero by FF-A11's clause, and removing the clause can only lower each state's residual since theta = 0 remains feasible. IT MAY BE QUOTED ONLY AS A LOWER BOUND AND NEVER AS \"F2's CAPTURE\". The quotable measured number from that run is the leading-part 2841944614/3716765745, about 76.46%."
    );
    let _ = writeln!(
        out,
        "  NOT COMMISSIONED, and each for its own reason (FF-A15(iv)): the JOINT TWO-FEATURE FIT — one live feature is not a joint problem, and pairing a live feature with a refuted one buys at most F1's 0.36%; the GRADED FORM OF F1 — eligible rather than refuted, but a 0.36% binary is a weak prior and it earns a run only on request; and ANY TRICK-1 OBJECT — FT-A21 stands BLOCKED in full. None of the three is built here."
    );
    let _ = writeln!(out);
}

#[allow(clippy::too_many_lines)]
fn main() {
    let t0 = Instant::now();
    sha256_self_check();
    rec_order_self_check();
    let mut out = String::new();
    header(&mut out);

    let receipt: Receipt = {
        let path =
            locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
        parse_file(&path).expect("the receipt parses")
    };

    let units = carrier_units();
    let _ = writeln!(
        out,
        "== THE CARRIER — FREEZE 52(a), enumerated with NO GENERATING RULE (FT-A23: a freeze is a constant, not a rule) =="
    );
    let _ = writeln!(
        out,
        "  ARM 1 — h0, pip 3, hand [00 21 32 53], the SINGLE unit a = 00. FF-A5's factual correction is carried here: H0 HAS ONE FREEZE-50 UNIT, NOT TWO — it is the untied coordinate with a single binding pair, competitor a = 00 against a* = 53. Arm 1 is both the cheapest unit in the carrier and the hand the feature came from."
    );
    let _ = writeln!(
        out,
        "  ARM 2, ATTEMPTED AFTER ARM 1 COMPLETES, WITH A DECLARED STOP — h2, pip 5, hand [21 33 53 54], units a = 53 then a = 54. h6, h9 and h12 are OUT OF SCOPE (FF-A5): h9 for cost, h6 and h12 because 53,570 and 69,512 frontier states buy nothing an audition needs that 330 and 16,136 do not."
    );
    let _ = writeln!(
        out,
        "  Coordinate identity is asserted in freeze 45's form at every unit — grade, declaration, hand and pool as canonical ascending domino-index tile lists, leader offset 0, |X| = 34,650 against kernel.count(), freeze-7/23 enumeration order, THE KERNEL REBUILT IN-RUN and asserted equal. Results are assembled in canonical unit order, never completion order (DS-A36)."
    );

    let texts: Mutex<BTreeMap<usize, UnitText>> = Mutex::new(BTreeMap::new());
    let mut arm1_ok = true;
    for arm in [1usize, 2usize] {
        if arm == 2 && !arm1_ok {
            let _ = writeln!(
                out,
                "\n== ARM 2 NOT ATTEMPTED (freeze 52(a), FF-A8(e)): arm 1 did not complete, and arm 2 is attempted only after arm 1 completes. This is a declared stop and never a finding (R-A18). =="
            );
            break;
        }
        let idx: Vec<usize> = (0..units.len()).filter(|i| units[*i].arm == arm).collect();
        let receipt_ref = &receipt;
        let texts_ref = &texts;
        let units_ref = &units;
        std::thread::scope(|scope| {
            for ui in &idx {
                let ui = *ui;
                scope.spawn(move || {
                    let t = Instant::now();
                    let text = render_unit(receipt_ref, units_ref[ui]);
                    eprintln!(
                        "  [stderr only] arm {arm} unit {ui} (coord h{}, a = {}) complete in {} ms",
                        FF_FILED[units_ref[ui].coord].0,
                        tile(units_ref[ui].action),
                        t.elapsed().as_millis()
                    );
                    texts_ref.lock().expect("lock").insert(ui, text);
                });
            }
        });
        if arm == 1 {
            let guard = texts.lock().expect("lock");
            arm1_ok = idx.iter().all(|i| !guard[i].stopped);
        }
    }
    let texts = texts.into_inner().expect("lock");
    for ui in 0..units.len() {
        if let Some(t) = texts.get(&ui) {
            let _ = write!(out, "{}", t.named);
        }
    }

    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== WHAT THIS BUILD DOES NOT SAY, restated at the foot because a capture fraction travels badly =="
    );
    let _ = writeln!(
        out,
        "  Every capture above is an ORACLE-THETA capture, spending one free rational per information state. By Proposition FF-oracle it UPPER-BOUNDS what any shared or coarser fee family can achieve, so a small number refutes conclusively and a large number establishes nothing about a usable fee family. No feature here is said to \"work\". Nothing measured here is quoted for trick 1 or for the opening (P-A21). Three units at two coordinates chosen by negative binding margin are a carrier, not a sample."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "total wall-clock (provenance only, never a dividend and never a receipt; SEP-A19(b), DS-A31/DS-A36): {} ms",
        t0.elapsed().as_millis()
    );
    let _ = writeln!(out, "run complete: yes");

    let results = out_dir("results").join("feature_fee_v11_2026-08-14.txt");
    std::fs::write(&results, &out).expect("write results");
    print!("{}", &out[..out.len().min(3000)]);
    println!("results: {}", results.display());
}
