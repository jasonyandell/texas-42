//! walt FC fee-correlation diagnostic — FREEZE 53, the quantitative form of
//! Proposition FF-corr measured on the carrier we already have. EXPLORATORY
//! TIER THROUGHOUT, without exception.
//!
//! Commissioned at FC-A1..FC-A11 (`walt/CENSUS-RULINGS.md`, 2026-08-14), with
//! Proposition FC-drop and Corollary FC-null delivered there; **freeze 53**
//! (FC-A5) under **freeze 52 v1.4** (the per-cell screen with the null-control
//! exemption). Beneath them the FF, FT and SR chapters entire.
//!
//! WHAT IT MEASURES, and it is a DIAGNOSTIC and not a sweep. At every swept
//! frontier state of freeze 53's three units, and for each of four features:
//! the one-sided slopes `s^+ = G'(0^+)` and `s^- = G'(0^-)` of Lemma FF-min's
//! `G_I`, the CLAIRVOYANT-ARGMAX CARDINALITY PROFILE, the nearest breakpoint
//! `t_0` on the descending side, and Proposition FC-drop's bound `|s| * t_0`.
//! Nothing is minimised. `s^±` needs no breakpoints at all; `t_0` needs them
//! enumerated but not searched over.
//!
//! THE SHARPEST HAZARD IN THIS BUILD, and it is why (FC-R7) exists.
//! `s^±` are taken over the COMPLETE clairvoyant argmax set
//! `argmax_b q_I(omega,b)` at every world, never a tie-broken representative.
//! A tie-broken argmax collapses `s^-` and `s^+` into one number, which turns
//! Proposition FC-drop(a)'s STRADDLE test into a POINT test — and the straddle
//! is exactly what separates FC-A7's outcome (c), genuine orthogonality, from
//! outcome (d), a tie-driven zero. Freeze 38(e)'s complete-face rule binds
//! here harder than anywhere it has bound before, and (FC-R7) catches a
//! collapse against faces filed by a DIFFERENT PROGRAM on a different day.
//!
//! No floats. Regenerate:
//! `cargo run --release -p walt-factory --example fc_correlation`

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
use walt_factory::fc_cores::FT_CORES;
use walt_factory::fc_kappa::V11_KAPPA;
use walt_geom::{q, qi, Q};
use walt_kernel::{Hidden, Kernel, HIDDEN_SEATS};

// -- freezes and declared constants -----------------------------------------

/// Freeze 44(b) v2, unchanged by FC-A5(h): B walk-steps per (coordinate,
/// action). On exhaustion, `None` and NO PARTIAL FOLD.
const B_WALK: u64 = 10_000_000_000;

/// Freeze 44(b) v2's partition-state cap, asserted against the rung-one
/// frontier partition; the assertion is contentful.
const P_MAX: u64 = 192_000_000;

const N4_TRICK: usize = 4;
const N4_GRADE: usize = 4;
const N4_FIBER: u128 = 34_650;

/// Freeze 53(g): the freeze-set digest travelling on every record. No cache is
/// read or written, so freeze 41/DS-A30's discard rule is vacuous here and is
/// stated rather than exercised.
const FC_DIGEST: &str = "FC-v1.0|freezes-7-23-26-37d-38v1.1-44v2-45-50v1.1-52v1.4-53|contract=R-A11-full-record|field=uniform-legal-F4|belief=uniform-fiber-freeze7";

/// `12^12`. The deep solve's fixed scale.
const SCALE: i64 = 8_916_100_448_256;

/// `12^6`. The common denominator of every pre-frontier arrival weight.
const DEN_MU: i128 = 2_985_984;

/// The maximum record length this probe encodes.
const REC_MAX: usize = 16;

/// FREEZE 53(b): the four features, in the frozen order. F0 runs FIRST AND
/// BLOCKING (FC-A6(i)).
const N_FEAT: usize = 4;
const FEAT_NAME: [&str; N_FEAT] = [
    "F0 boss_owner",
    "F1 boss_can_follow_b",
    "F1g boss_b_suit_count (the GRADED form of F1, new at freeze 53(b))",
    "F2 b_is_beatable (AMENDED, freeze 52 v1.1)",
];
const FEAT_KIND: [&str; N_FEAT] = [
    "NULL CONTROL, action-blind, run first and blocking, EXEMPT from freeze 52 v1.4's screen — Corollary FC-null fixes s^+ = s^- = 0 EXACTLY, by theorem and not by a filed number",
    "REFUTED AS A BINARY at FF-A13 and re-measured here only as the parent of F1g; DIAGNOSTIC-ONLY, no capture number, no frozen kappa_I and therefore NO COMPARISON",
    "DIAGNOSTIC-ONLY per FC-A4: s^± decides the graded form with no sweep, and NO CAPTURE NUMBER IS COMPUTED OR REPORTED FOR IT; the FC-drop bound is emitted and is a different object (FC-A10(v))",
    "THE MEASUREMENT THIS CHAPTER EXISTS FOR, action-conditioned, domain = the whole fiber; its exact zero capture at 432 h2 states is the unexplained fact the diagnostic is aimed at",
];
/// Freeze 52 v1.1's DOMAIN CLAUSE, carried to freeze 53(b): which features
/// reference `h(omega)` and are therefore 0 where no trump is outstanding.
const FEAT_BOSS_KEYED: [bool; N_FEAT] = [true, true, true, false];
/// FC-A10(i): the frozen table's feature index, `None` where NO frozen
/// `kappa_I` exists. The v1.1 run swept F0 and F2 only.
const FEAT_KAPPA: [Option<u8>; N_FEAT] = [Some(0), None, None, Some(1)];
/// FREEZE 52 v1.4 (FF-A33(iii)): the null control is exempt from the screen in
/// all cases, because its job is to test the harness rather than the feature.
const FEAT_SCREEN_EXEMPT: [bool; N_FEAT] = [true, false, false, false];

// -- the frozen source tables (SEP-A14(ii), FT-A28(i)) ----------------------

/// One filed root-action row: `(hi, lo, Q^H as (num, den), U as (num, den),
/// revealed walk-steps)`, the two values in the COUNT convention.
type FiledRow = (u8, u8, (i128, i128), (i128, i128), u64);

/// One filed carrier coordinate: `(corpus hand id, declaration pip, the four
/// root-action rows in ascending domino index)`.
type FiledCoord = (usize, u8, [FiledRow; 4]);

/// FREEZE 53(a): the carrier, enumerated with NO GENERATING RULE (FT-A23: a
/// freeze is a constant, not a rule) — THE SAME THREE UNITS AS FREEZE 52 v1.1,
/// and NO NEW COORDINATE. h0, pip 3, hand `[00 21 32 53]`, unit `a = 00`; h2,
/// pip 5, hand `[21 33 53 54]`, units `a = 53` then `a = 54`.
///
/// Quoted from `walt-factory/results/separation_n4_2026-08-14.txt`,
/// exploratory tier; carried as a frozen table and NEVER re-parsed at run time.
const FC_FILED: [FiledCoord; 2] = [
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

/// One unit's rung-one filed record for (FC-R5): `(corpus hand id, a hi, a lo,
/// frontier states, (state, world) arrivals, states with delta_I = 0, states
/// with delta_I > 0, Delta^(1), U^(1), U^(0), walk-steps, the (FT-R7c)
/// frontier digest)`. The three rationals are count convention.
///
/// Quoted from `fusion_tax_2026-08-14.txt`; h0's digest from
/// `feature_fee_2026-08-14.txt`, the first FF run, which filed it; h2's two
/// from `SR_FIRST` in `second_rung.rs`. Exploratory tier; never re-parsed.
/// UNLIKE the v1.1 run, ALL THREE DIGESTS ARE ASSERTED HERE — h0's slot was
/// empty then and is filed now, so this run has nothing left to file and
/// everything to check (FT-A28(iv)).
type FcFirstUnit = (
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
const FC_FIRST: [FcFirstUnit; 3] = [
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
    // SR-A27(iv): a column read by eye must not change case with its value.
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
/// no proof, no receipt and no comparison.
fn ppm_bracket(x: &BigRational) -> (BigInt, BigInt) {
    let scaled = x * BigRational::from_integer(BigInt::from(1_000_000));
    (scaled.floor().to_integer(), scaled.ceil().to_integer())
}

// -- SHA-256 (FIPS 180-4), exact integer arithmetic, no dependency ----------
// Carried forward with SR-A33's repair already in it: `update` must not
// clobber the buffered length across calls, and `finish` computes its pad
// length rather than searching for it.

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
                // Returning here is load-bearing (SR-A33): falling through
                // would overwrite `buf_len` with the empty remainder below and
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
/// run before any number exists (SR-A33(iii)): a receipt whose assertion is an
/// equality of digests carries a second, silent obligation — that the digest
/// function is anchored to published vectors covering the code path actually
/// used, INCLUDING THE STREAMING PATH.
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
/// lexicographic order over the canonical ascending domino index, verified
/// against `Vec<Domino>`'s own ordering before any record is keyed.
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

// -- the four frozen features of freeze 53(b) --------------------------------

/// The "outstanding trump" at a frontier state: the highest-ranking trump
/// under the declaration among tiles **not yet played in the record and not in
/// the focal seat's hand at `I`** — i.e. held by a field seat. Its identity is
/// a function of the RECORD ALONE and that invariance is asserted at every
/// arrival; only the seat holding it varies across `X_I`. Ranking is the rule
/// algebra's own `Decl::rank` and is never re-implemented.
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

/// The four features of freeze 53(b) at one `(omega, b)`, as SMALL INTEGERS
/// rather than bits: F1g is a COUNT and `phi` is no longer binary.
///
/// FREEZE 52 v1.1's DOMAIN CLAUSE, carried unchanged: the no-outstanding-trump
/// fallback is scoped to the features that REFERENCE `h(omega)` — F0, F1 and
/// F1g. **F2 never mentions `h(omega)`** and is perfectly well defined with no
/// trump outstanding; its domain is the whole fiber.
fn features_at(ctx: Ctx, node: Node, boss: Option<Domino>, b: Domino) -> [u8; N_FEAT] {
    // The boss-keyed block. DOMAIN: requires `h(omega)`, so F0, F1 and F1g are
    // all 0 when no trump is outstanding.
    let (f0, f1, f1g) = match boss {
        None => (0u8, 0u8, 0u8),
        Some(boss) => {
            let h = boss_holder(ctx, node, boss);
            let hand_h = node.hands[h.index()];

            // (F0) NULL CONTROL — `boss_owner`: 1 if the holder is an opponent
            // of the focal seat, 0 if a partner. ACTION-BLIND by construction:
            // `b` is not read. Corollary FC-null then gives s^+ = s^- = 0
            // EXACTLY, which is (FC-R1) and is fixed by theorem.
            let f0 = u8::from(h.team() != ctx.team);

            // (F1) `boss_can_follow_b`: 1 if `h(omega)` holds at least one
            // tile of `b`'s suit under the declaration. `b`'s suit is
            // `Decl::led_context(b)` and membership is the rule algebra's own
            // `Decl::effective_incidence`, never a re-implementation.
            //
            // (F1g) THE GRADED FORM, new at freeze 53(b): the CARDINALITY of
            // the same intersection whose emptiness F1 tests. The
            // by-construction agreement `F1 = 1 <=> F1g >= 1` is asserted in
            // place below rather than left implicit (FC-A10(vi)).
            let suit = ctx.decl.led_context(b);
            let held = hand_h.intersection(ctx.decl.effective_incidence(suit));
            let f1g = u8::try_from(held.len()).expect("a suit intersection is small");
            assert!(
                f1g <= 7,
                "stop-and-report: F1g exceeds a suit's seven tiles at [{}]",
                tile(b)
            );
            let f1 = u8::from(!held.is_empty());
            assert_eq!(
                f1 == 1,
                f1g >= 1,
                "stop-and-report: F1 and F1g disagree, and they are the same intersection tested two ways"
            );
            (f0, f1, f1g)
        }
    };

    // (F2) `b_is_beatable`, AMENDED per FF-A12 and freeze 52 v1.1: 1 iff some
    // opponent **who has not yet played at `I`** holds a tile that, if played,
    // would win the trick over `b` AND over every tile already on the table.
    // At LEADING states this is identical to the frozen reading. `Decl::beats`
    // IS the rule algebra's BEATS relation and `Decl::trick_key` its ordering;
    // both are called, never reproduced.
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
    let mut f2 = 0u8;
    // Seats yet to play at `I` are those after the focal seat in this trick:
    // offsets k+1..3 from the leader. The focal seat sits at offset `node.k`.
    for pos in (node.k + 1)..4 {
        let s = node.leader.plus(pos);
        if s.team() != ctx.team && !node.hands[s.index()].intersection(beat_set).is_empty() {
            f2 = 1;
        }
    }

    [f0, f1, f1g, f2]
}

// -- the rung-one frontier pass, recording per-world rows --------------------

/// One `(state, world)` arrival. The per-world `q_I(omega, b)` are exact
/// world-informed continuation values, not bounds, and none is ever carried
/// out of a frontier (FT-A12(iii)).
#[derive(Clone, Copy)]
struct WorldRow {
    /// `DEN_MU / den`, the arrival weight (Lemma FT-arrive's product form).
    w: u32,
    /// The world's FIBER INDEX in freeze-7/23 enumeration order — the same
    /// index `fusion_tax.rs` filed its cores under, and what (FC-R7) keys on.
    wi: u32,
    /// `q_I(omega, b_j)` for each action of `legal` in ascending domino index.
    q: [i64; 4],
    /// `phi[f][j]`: feature `f` at action `b_j`, as a SMALL INTEGER. F1g is a
    /// count, so the v1.1 probe's per-action BITMASK cannot carry it.
    phi: [[u8; 4]; N_FEAT],
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
    wi: u32,
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
        let mut phi = [[0u8; 4]; N_FEAT];
        for (j, d) in legal.iter().enumerate() {
            let f = features_at(self.ctx, node, boss, d);
            for (fi, v) in f.iter().enumerate() {
                phi[fi][j] = *v;
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
            wi: self.wi,
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
    for (wi, hands) in worlds.iter().enumerate() {
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
            wi: u32::try_from(wi).expect("fiber index fits u32"),
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

// -- the complete clairvoyant argmax face ------------------------------------

/// THE COMPLETE argmax set `argmax_b q_I(omega,b)` at one world, as a
/// `DominoSet`.
///
/// FC-A11(ii) is binding on this function and it is the sharpest hazard in the
/// build: the set is accumulated BY AN EQUALITY TEST ACROSS ALL `j`, mirroring
/// `fusion_tax.rs`'s `if child[j] == best { mask.insert(d) }`, and NO INDEX IS
/// TRACKED. The natural Rust idiom — `iter().enumerate().max_by_key(..)` — is
/// exactly the defect, because it returns ONE index and would collapse `s^-`
/// and `s^+` into a single number, turning Proposition FC-drop(a)'s STRADDLE
/// test into a POINT test. (FC-R7) checks this function against faces filed by
/// a different program.
fn argmax_face(st: &FrontierState, row: &WorldRow) -> DominoSet {
    let mut best = i64::MIN;
    for (j, _) in st.legal.iter().enumerate() {
        if row.q[j] > best {
            best = row.q[j];
        }
    }
    let mut out = DominoSet::EMPTY;
    for (j, d) in st.legal.iter().enumerate() {
        if row.q[j] == best {
            out.insert(d);
        }
    }
    assert!(
        !out.is_empty(),
        "stop-and-report: an argmax face is empty, which A(I) nonempty forbids"
    );
    out
}

/// A filed mask from `fc_cores.rs`, parsed into the rule algebra's own set
/// type. The masks are SOURCE, transcribed at build time; no results text is
/// read at run time (SEP-A14(ii), FT-A28(i)).
fn parse_mask(s: &str) -> DominoSet {
    let mut out = DominoSet::EMPTY;
    for t in s.split(' ') {
        let b = t.as_bytes();
        assert_eq!(b.len(), 2, "a filed mask tile is two pip digits: [{s}]");
        let hi = Pip::new(b[0] - b'0').expect("a filed mask tile's high pip");
        let lo = Pip::new(b[1] - b'0').expect("a filed mask tile's low pip");
        assert!(
            out.insert(Domino::new(hi, lo)),
            "a filed mask names each tile once: [{s}]"
        );
    }
    assert!(!out.is_empty(), "a filed mask is nonempty: [{s}]");
    out
}

// -- the diagnostic at one (state, feature) ----------------------------------

/// Compare `n1/d1` with `n2/d2` exactly, in i128, with no float. Denominators
/// are normalised positive first; the cross products are bounded far inside
/// i128 at this carrier and the multiplications are checked.
fn cmp_frac(n1: i128, d1: i128, n2: i128, d2: i128) -> std::cmp::Ordering {
    let (n1, d1) = if d1 < 0 { (-n1, -d1) } else { (n1, d1) };
    let (n2, d2) = if d2 < 0 { (-n2, -d2) } else { (n2, d2) };
    assert!(d1 > 0 && d2 > 0, "a breakpoint denominator is nonzero");
    let a = n1
        .checked_mul(d2)
        .expect("freeze 53(e) stop-and-report: breakpoint comparison overflowed");
    let b = n2
        .checked_mul(d1)
        .expect("freeze 53(e) stop-and-report: breakpoint comparison overflowed");
    a.cmp(&b)
}

/// What the diagnostic returns at one `(state, feature)`. Every field is an
/// exact rational or an exact integer.
struct Diag {
    /// `s^+` and `s^-`, CONVENTION-FREE: they are mass-weighted feature
    /// quantities, `sum_omega mu_I(omega) * (-Phi_I(omega,b))`, in which no
    /// value enters — `q` is read only to decide WHICH `b` lie in the argmax.
    s_plus: BigRational,
    s_minus: BigRational,
    /// The clairvoyant-argmax CARDINALITY PROFILE (freeze 53(c)) over the
    /// worlds of this state: `card[k]` worlds have an argmax of size `k`.
    card: [u64; 5],
    n_multi: u64,
    /// Distinct breakpoints of `G_I`, in `sigma` and hence in `theta`.
    n_break: usize,
    /// Proposition FC-drop(a)'s zero test: `s^- <= 0 <= s^+`.
    straddle: bool,
    /// `t_0`, the distance from 0 to the nearest breakpoint on the DESCENDING
    /// side, in the COUNT convention. `None` exactly when the straddle holds.
    t0: Option<BigRational>,
    side: &'static str,
    /// Proposition FC-drop(c)'s bound `|s| * t_0`, COUNT convention. Zero when
    /// the straddle holds, where the proposition asserts `kappa_I = 0`.
    bound: BigRational,
}

/// The diagnostic of Proposition FC-drop at one state and one feature.
///
/// The integer parameterisation is the v1.1 probe's, unchanged: with
/// `p_num = acc_p` and `c_num[j] = sum_omega w * phi`, set
/// `P(omega,j) = p_num * phi - c_num[j]`, so that `Phi = P / p_num` and the
/// `sigma`-parameterised line is `q - sigma * P` with `theta = p_num * sigma`.
/// Hence `s^± = sum_omega w * (-P_extreme) / (p_num * DEN_MU * |X|)`, the
/// extremes taken over the COMPLETE argmax set, and a breakpoint in `sigma` is
/// `(q_v - q_u)/(P_v - P_u)`.
fn diagnose(st: &FrontierState, f: usize) -> Diag {
    let n_act = st.legal.len();
    let p_num = st.acc_p;
    assert!(p_num > 0, "a frontier state has positive mass");

    // Per-action centres, per FF-A2 and freeze 52(c). A single per-state
    // centre is exactly what is barred: it does not satisfy Theorem 12.1's
    // hypothesis when phi depends on b.
    let mut c_num = vec![0i128; n_act];
    for row in &st.rows {
        for (j, c) in c_num.iter_mut().enumerate() {
            *c = c
                .checked_add(
                    i128::from(row.w)
                        .checked_mul(i128::from(row.phi[f][j]))
                        .expect("freeze 53(e) stop-and-report: a centre accumulation overflowed"),
                )
                .expect("freeze 53(e) stop-and-report: a centre accumulation overflowed");
        }
    }
    let pat = |row: &WorldRow, j: usize| -> i128 {
        p_num
            .checked_mul(i128::from(row.phi[f][j]))
            .expect("freeze 53(e) stop-and-report: P overflowed")
            - c_num[j]
    };

    // ---- s^+ and s^-, over the COMPLETE clairvoyant argmax set ------------
    let mut s_plus_num: i128 = 0;
    let mut s_minus_num: i128 = 0;
    let mut card = [0u64; 5];
    let mut n_multi = 0u64;
    for row in &st.rows {
        let face = argmax_face(st, row);
        let mut p_lo = i128::MAX;
        let mut p_hi = i128::MIN;
        let mut n_face = 0u64;
        for (j, d) in st.legal.iter().enumerate() {
            if !face.contains(d) {
                continue;
            }
            n_face += 1;
            let p = pat(row, j);
            if p < p_lo {
                p_lo = p;
            }
            if p > p_hi {
                p_hi = p;
            }
        }
        let n_face_sz = usize::try_from(n_face).expect("a face size fits usize");
        assert!(
            n_face >= 1 && n_face_sz <= n_act,
            "stop-and-report: an argmax face has an impossible cardinality"
        );
        card[n_face_sz] += 1;
        if n_face > 1 {
            n_multi += 1;
        }
        let w = i128::from(row.w);
        // max_b (-P) = -min_b P and min_b (-P) = -max_b P, both over the face.
        s_plus_num = s_plus_num
            .checked_add(
                w.checked_mul(-p_lo)
                    .expect("freeze 53(e) stop-and-report: s^+ accumulation overflowed"),
            )
            .expect("freeze 53(e) stop-and-report: s^+ accumulation overflowed");
        s_minus_num = s_minus_num
            .checked_add(
                w.checked_mul(-p_hi)
                    .expect("freeze 53(e) stop-and-report: s^- accumulation overflowed"),
            )
            .expect("freeze 53(e) stop-and-report: s^- accumulation overflowed");
    }
    // An ARITHMETIC REMARK and not a receipt (Proposition SR-taut): it cannot
    // fail, since the face maximum is never below the face minimum.
    assert!(
        s_minus_num <= s_plus_num,
        "stop-and-report: s^- > s^+, which convexity forbids"
    );
    let s_den = BigRational::from_integer(big(p_num) * big(DEN_MU) * BigInt::from(N4_FIBER));
    let s_plus = BigRational::from_integer(big(s_plus_num)) / s_den.clone();
    let s_minus = BigRational::from_integer(big(s_minus_num)) / s_den;
    let straddle = s_minus_num <= 0 && 0 <= s_plus_num;

    // ---- the breakpoints of G_I, per world, by the upper envelope ---------
    let mut breaks: Vec<(i128, i128)> = Vec::new();
    for row in &st.rows {
        // The lines L_j(sigma) = q_j - sigma * P_j, sorted by P descending,
        // i.e. by slope ascending, which is the order in which they can become
        // active left to right on the upper envelope.
        let mut lines: Vec<(i128, i128)> = (0..n_act)
            .map(|j| (pat(row, j), i128::from(row.q[j])))
            .collect();
        lines.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));
        lines.dedup_by_key(|l| l.0);
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
            breaks.push((num, den));
        }
    }
    breaks.sort_by(|a, b| cmp_frac(a.0, a.1, b.0, b.1));
    let n_break = {
        let mut n = 0usize;
        let mut i = 0usize;
        while i < breaks.len() {
            let mut j = i + 1;
            while j < breaks.len()
                && cmp_frac(breaks[i].0, breaks[i].1, breaks[j].0, breaks[j].1)
                    == std::cmp::Ordering::Equal
            {
                j += 1;
            }
            n += 1;
            i = j;
        }
        n
    };

    // ---- t_0 and the drop bound (Proposition FC-drop(b),(c)) -------------
    let mut t0: Option<BigRational> = None;
    let mut side = "n/a (the straddle holds, so Proposition FC-drop(a) gives kappa_I = 0)";
    let mut bound = BigRational::zero();
    if !straddle {
        let want_right = s_plus_num < 0;
        let mut nearest: Option<(i128, i128)> = None;
        for (num, den) in &breaks {
            let ord = cmp_frac(*num, *den, 0, 1);
            let on_side = if want_right {
                ord == std::cmp::Ordering::Greater
            } else {
                ord == std::cmp::Ordering::Less
            };
            if !on_side {
                continue;
            }
            let better = match nearest {
                None => true,
                Some((bn, bd)) => {
                    if want_right {
                        cmp_frac(*num, *den, bn, bd) == std::cmp::Ordering::Less
                    } else {
                        cmp_frac(*num, *den, bn, bd) == std::cmp::Ordering::Greater
                    }
                }
            };
            if better {
                nearest = Some((*num, *den));
            }
        }
        // Proposition FC-drop(b) says the descending side IS populated; its
        // failure is a harness fault, never a finding.
        let (num, den) = nearest.expect(
            "stop-and-report: Proposition FC-drop(b) says a breakpoint lies on the descending side and none was found — G_I would be affine with negative slope and unbounded below, contradicting Lemma FF-min(b)",
        );
        // theta = p_num * sigma, and the COUNT convention halves a value while
        // the SCALE-scaled differential divides by 12^12: t_0 (count) =
        // p_num * |sigma| / (2 * SCALE).
        let t = BigRational::new(
            big(p_num) * big(num),
            big(den) * big(2) * big(i128::from(SCALE)),
        )
        .abs();
        assert!(
            t.is_positive(),
            "stop-and-report: t_0 is not strictly positive, so the nearest breakpoint was taken at 0"
        );
        side = if want_right {
            "right (s^+ < 0)"
        } else {
            "left (s^- > 0)"
        };
        let s = if want_right {
            s_plus.clone()
        } else {
            s_minus.clone()
        };
        bound = s.abs() * t.clone();
        t0 = Some(t);
    }

    Diag {
        s_plus,
        s_minus,
        card,
        n_multi,
        n_break,
        straddle,
        t0,
        side,
        bound,
    }
}

// -- one unit ----------------------------------------------------------------

/// One feature's diagnostic at one unit. Every rational is exact.
struct FeatOut {
    /// Freeze 52 v1.4: the cell was screened as an EMPTY TEST.
    screened: bool,
    /// The feature's domain census, over TWO named sets (freeze 52 v1.3).
    n_domain_all: u64,
    n_domain_swept: u64,
    /// Straddle census over the swept states of this unit.
    n_zero: u64,
    n_pos: u64,
    /// States whose `s^±` are not both exactly 0, and the leading part of that
    /// count — (FC-R2)'s material.
    n_s_nonzero: u64,
    n_s_nonzero_lead: u64,
    n_break_total: u64,
    n_multi_worlds: u64,
    n_worlds_seen: u64,
    bound_sum: BigRational,
    bound_max: BigRational,
    bound_max_rec: String,
    /// Where a frozen `kappa_I` exists: tightness of the bound against it.
    n_tight: u64,
    ratio_max: Option<(BigRational, String)>,
    rows: Vec<String>,
}

struct FcUnit {
    n_states: u64,
    n_arrivals: u64,
    zero: u64,
    pos: u64,
    mass: Q,
    u0_diff: Q,
    u1_diff: Q,
    delta1_diff: Q,
    frontier_digest: String,
    n_boss_pos: u64,
    n_boss_all: u64,
    n_lead_pos: u64,
    n_foll_pos: u64,
    /// (FC-R7): swept states face-checked, filed masks compared, and how many
    /// of those masks carry TWO tiles — the ones a tie-break would break.
    n_faces: u64,
    n_face_masks: u64,
    n_face_multi: u64,
    feats: Vec<FeatOut>,
    steps: u64,
    residual: u64,
}

/// The frozen `kappa_I` of freeze 53(d), keyed for one unit.
fn kappa_table(unit: u8) -> BTreeMap<(u8, String), (i128, i128)> {
    let mut m: BTreeMap<(u8, String), (i128, i128)> = BTreeMap::new();
    for (u, f, rec, num, den) in V11_KAPPA {
        if u == unit {
            assert!(
                m.insert((f, rec.to_string()), (num, den)).is_none(),
                "stop-and-report: the frozen kappa_I table repeats a (feature, record) key"
            );
        }
    }
    m
}

/// (FC-R7)'s filed faces of FC-A11(iii), keyed for one unit.
type FiledCore = (u32, &'static str, u32, &'static str);
fn core_table(unit: u8) -> BTreeMap<String, FiledCore> {
    let mut m: BTreeMap<String, FiledCore> = BTreeMap::new();
    for (u, rec, wa, ma, wb, mb) in FT_CORES {
        if u == unit {
            assert!(
                m.insert(rec.to_string(), (wa, ma, wb, mb)).is_none(),
                "stop-and-report: the filed-core table repeats a record"
            );
        }
    }
    m
}

#[allow(clippy::too_many_lines)]
fn run_unit(kernel: &Kernel, root: Domino, unit: u8) -> Option<FcUnit> {
    assert_eq!(
        kernel.viewer_hand().len(),
        N4_GRADE,
        "the declared grade is the coordinate's grade (N4-A11)"
    );
    let worlds: Vec<[DominoSet; Seat::COUNT]> = kernel.worlds().map(|w| w.hands()).collect();
    let n_worlds = i128::try_from(worlds.len()).expect("fiber size fits i128");
    assert_eq!(
        u128::try_from(n_worlds).expect("fiber size fits u128"),
        N4_FIBER,
        "freeze 45: |X| = 34,650"
    );
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
    let mut n_boss_pos: u64 = 0;
    let mut n_boss_all: u64 = 0;
    let mut n_lead_pos: u64 = 0;
    let mut n_foll_pos: u64 = 0;
    let mut digest = Sha256::new();
    let mut positive: Vec<(Rec, i128)> = Vec::new();

    // (FC-R7) THE FILED-FACE RECEIPT (FC-A11(iii)), run in the census pass —
    // before any feature quantity exists. It produces NO NUMBER, so FC-A6(i)'s
    // ordering (the null control first before any other number) is untouched,
    // and the object every s^± is built from is validated first.
    let mut cores = core_table(unit);
    let mut n_faces: u64 = 0;
    let mut n_face_masks: u64 = 0;
    let mut n_face_multi: u64 = 0;

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
        if st.boss.is_some() {
            n_boss_all += 1;
        }
        if dtax == 0 {
            zero += 1;
        } else {
            pos += 1;
            positive.push((*rec, dtax));
            if st.boss.is_some() {
                n_boss_pos += 1;
            }
            if st.leading {
                n_lead_pos += 1;
            } else {
                n_foll_pos += 1;
            }

            let filed = cores.remove(&rec.text()).expect(
                "(FC-R7) stop-and-report: a swept state has no filed minimal fusion core, so the filed set and this run's swept set disagree",
            );
            for (wi, mask) in [(filed.0, filed.1), (filed.2, filed.3)] {
                let row = st
                    .rows
                    .iter()
                    .find(|r| r.wi == wi)
                    .expect("(FC-R7) stop-and-report: a filed core names a fiber index that does not arrive at this state");
                let face = argmax_face(st, row);
                let want = parse_mask(mask);
                assert_eq!(
                    face,
                    want,
                    "(FC-R7) stop-and-report: the complete argmax face at fiber index {wi} of state [{}] is {{{}}} and fusion_tax_2026-08-14.txt filed {{{}}} — a COLLAPSED FACE, which turns Proposition FC-drop(a)'s straddle test into a point test",
                    rec.text(),
                    tiles_str(face),
                    tiles_str(want)
                );
                n_face_masks += 1;
                if want.len() > 1 {
                    n_face_multi += 1;
                }
            }
            n_faces += 1;
        }
        // (FT-R7c)'s canonical serialisation, byte-identical to the one
        // `second_rung.rs` and `feature_fee_v11.rs` used, so the digests are a
        // genuine cross-probe comparison rather than a self-comparison.
        digest.update(rec.text().as_bytes());
        digest.update(b"|");
        digest.update(qs(tax_to_count(Q::new(dtax, norm1))).as_bytes());
        digest.update(b"\n");
    }
    assert!(
        cores.is_empty(),
        "(FC-R7) stop-and-report: {} filed minimal fusion cores of this unit were never claimed by a swept state",
        cores.len()
    );
    assert_eq!(
        sum_m,
        DEN_MU * pass.world_fold,
        "stop-and-report: the frontier accumulation and the per-world revealed fold disagree"
    );
    let mass = Q::new(sum_p, pnorm1);
    let delta1_unnorm = sum_m - sum_best;

    let kappa = kappa_table(unit);
    let mut feats: Vec<FeatOut> = Vec::new();
    // The feature loop is sequential and F0 is FIRST in freeze 53(b)'s order,
    // which is what makes (FC-R1) blocking: its assertions fire before any
    // F1, F1g or F2 number is computed.
    #[allow(clippy::needless_range_loop)]
    for f in 0..N_FEAT {
        // FREEZE 52 v1.4 (FF-A33(iii)): the screen is per (unit, feature)
        // CELL, and the null control is EXEMPT in all cases.
        let mut n_domain_all: u64 = 0;
        let mut n_domain_swept: u64 = 0;
        for (rec, st) in &pass.states {
            let in_domain = !FEAT_BOSS_KEYED[f] || st.boss.is_some();
            if in_domain {
                n_domain_all += 1;
                if positive.binary_search_by(|p| p.0.cmp(rec)).is_ok() {
                    n_domain_swept += 1;
                }
            }
        }
        let screened = n_domain_swept == 0 && !FEAT_SCREEN_EXEMPT[f];
        let mut out = FeatOut {
            screened,
            n_domain_all,
            n_domain_swept,
            n_zero: 0,
            n_pos: 0,
            n_s_nonzero: 0,
            n_s_nonzero_lead: 0,
            n_break_total: 0,
            n_multi_worlds: 0,
            n_worlds_seen: 0,
            bound_sum: BigRational::zero(),
            bound_max: BigRational::zero(),
            bound_max_rec: String::from("n/a"),
            n_tight: 0,
            ratio_max: None,
            rows: Vec::new(),
        };
        if screened {
            feats.push(out);
            continue;
        }

        for (rec, _) in &positive {
            let st = pass.states.get(rec).expect("a positive state exists");
            let d = diagnose(st, f);

            // (FC-R1) THE NULL-CONTROL RECEIPT, per state. F0 is action-blind,
            // so Corollary FC-null gives s^+ = s^- = 0 EXACTLY. This is the
            // only check in the build whose answer is known BY PROOF rather
            // than by a filed rational — the (SR-R9)/(FF-R1) role.
            if f == 0 {
                assert!(
                    d.s_plus.is_zero() && d.s_minus.is_zero(),
                    "(FC-R1) stop-and-report: the action-blind null control has s^+ = {} and s^- = {} at [{}], and Corollary FC-null says both are EXACTLY 0. No F1, F1g or F2 number is reported, emitted or discussed (FC-A7(a))",
                    bqs(&d.s_plus),
                    bqs(&d.s_minus),
                    rec.text()
                );
            }

            out.n_worlds_seen += st.n_worlds;
            out.n_multi_worlds += d.n_multi;
            out.n_break_total += u64::try_from(d.n_break).expect("fits");
            if d.straddle {
                out.n_zero += 1;
            } else {
                out.n_pos += 1;
            }
            let s_nonzero = !d.s_plus.is_zero() || !d.s_minus.is_zero();
            if s_nonzero {
                out.n_s_nonzero += 1;
                if st.leading {
                    out.n_s_nonzero_lead += 1;
                }
            }
            out.bound_sum += d.bound.clone();
            if d.bound > out.bound_max {
                out.bound_max = d.bound.clone();
                out.bound_max_rec = rec.text();
            }

            // The frozen comparison, where one exists (FC-A10(i)): F0 and F2.
            let mut kappa_cell = String::from(
                "  NO kappa_I COLUMN AND NO COMPARISON: the v1.1 run swept F0 and F2 only, so no frozen captured amount exists for this feature (FC-A10(i)), and the bound beside it is a PROVED LOWER BOUND ON CAPTURE (Proposition FC-drop(c)) and is NEVER a capture",
            );
            if let Some(kf) = FEAT_KAPPA[f] {
                let (kn, kd) = *kappa.get(&(kf, rec.text())).expect(
                    "(FC-R3) stop-and-report: a swept state has no frozen kappa_I, so the frozen table and this run's swept set disagree",
                );
                let k = BigRational::new(big(kn), big(kd));

                // (FC-R3) THE ZERO-CHARACTERISATION RECEIPT: Proposition
                // FC-drop(a) against a value produced by a DIFFERENT PROGRAM
                // in a different run.
                assert_eq!(
                    k.is_zero(),
                    d.straddle,
                    "(FC-R3) stop-and-report: Proposition FC-drop(a) fails at [{}] for feature {} — the frozen kappa_I is {} and the straddle s^- <= 0 <= s^+ is {}",
                    rec.text(),
                    FEAT_NAME[f],
                    bqs(&k),
                    yn(d.straddle)
                );

                // (FC-R4) THE DROP RECEIPT: where the straddle is false,
                // kappa_I >= |s| * t_0 against the frozen kappa_I.
                if !d.straddle {
                    assert!(
                        d.bound <= k,
                        "(FC-R4) stop-and-report: Proposition FC-drop(c) fails at [{}] for feature {} — the bound |s| * t_0 = {} exceeds the frozen kappa_I = {}",
                        rec.text(),
                        FEAT_NAME[f],
                        bqs(&d.bound),
                        bqs(&k)
                    );
                    if d.bound == k {
                        out.n_tight += 1;
                    }
                    let ratio = d.bound.clone() / k.clone();
                    if out.ratio_max.as_ref().is_none_or(|(r, _)| ratio > *r) {
                        out.ratio_max = Some((ratio, rec.text()));
                    }
                }
                kappa_cell = format!(
                    "  frozen kappa_I = {} (count)  bound/kappa_I = {}",
                    bqs(&k),
                    if d.straddle {
                        String::from("n/a (kappa_I = 0 and the bound is 0)")
                    } else {
                        bqs(&(d.bound.clone() / k))
                    }
                );
            }

            out.rows.push(format!(
                "    I=[{}]  lead = {}  |X_I| = {}  |A(I)| = {}  domain = {}  s^+ = {}  s^- = {}  argmax cardinality over the {} worlds of this state: 1:{} 2:{} 3:{} (non-singleton {})  breakpoints = {}  straddle = {}  t_0 = {} (count)  side = {}  proved lower bound on capture (Proposition FC-drop(c)) |s|*t_0 = {} (count){}  verdict = {}",
                rec.text(),
                yn(st.leading),
                st.n_worlds,
                st.legal.len(),
                yn(!FEAT_BOSS_KEYED[f] || st.boss.is_some()),
                bqs(&d.s_plus),
                bqs(&d.s_minus),
                st.n_worlds,
                d.card[1],
                d.card[2],
                d.card[3],
                d.n_multi,
                d.n_break,
                yn(d.straddle),
                d.t0.as_ref().map_or_else(|| String::from("n/a"), bqs),
                d.side,
                bqs(&d.bound),
                kappa_cell,
                if d.straddle { "zero" } else { "positive" }
            ));
        }
        feats.push(out);
    }

    // (FC-R2) THE NON-NULL PAIRING RECEIPT is asserted by the caller, which
    // knows which unit is h0; the count it needs is `n_s_nonzero_lead` at F2.

    Some(FcUnit {
        n_states,
        n_arrivals,
        zero,
        pos,
        mass,
        u0_diff: Q::new(sum_m, norm1),
        u1_diff: Q::new(sum_best, norm1),
        delta1_diff: Q::new(delta1_unnorm, norm1),
        frontier_digest: digest.finish(),
        n_boss_pos,
        n_boss_all,
        n_lead_pos,
        n_foll_pos,
        n_faces,
        n_face_masks,
        n_face_multi,
        feats,
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
    /// The freeze 53(a) index, which is also the frozen tables' unit index.
    unit: u8,
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

/// FREEZE 53(a), enumerated with NO GENERATING RULE — the same three units as
/// freeze 52 v1.1, in the order the frozen tables are indexed by.
fn carrier_units() -> Vec<UnitKey> {
    vec![
        UnitKey {
            coord: 0,
            action: dom(0, 0),
            arm: 1,
            unit: 0,
        },
        UnitKey {
            coord: 1,
            action: dom(5, 3),
            arm: 2,
            unit: 1,
        },
        UnitKey {
            coord: 1,
            action: dom(5, 4),
            arm: 2,
            unit: 2,
        },
    ]
}

struct UnitText {
    named: String,
    stopped: bool,
}

#[allow(clippy::too_many_lines)]
fn render_unit(receipt: &Receipt, key: UnitKey) -> UnitText {
    let (hand_id, filed_pip, rows) = &FC_FILED[key.coord];
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

    let first = FC_FIRST
        .iter()
        .find(|(h, hi, lo, ..)| *h == *hand_id && dom(*hi, *lo) == key.action)
        .expect("every carrier unit has a rung-one filed record");
    let (_, _, _, f_states, f_arr, f_zero, f_pos, f_d1, f_u1, f_u0, f_steps, f_digest) = *first;

    let mut out = String::new();
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== FC unit (freeze 53(a), ARM {}, unit index {}): coord h{hand_id} grade={N4_GRADE} pip={} hand=[{}] pool=[{}] leader-offset=0 |X|={N4_FIBER} enumeration=freeze-7/23 || competitor a = {} ==",
        key.arm,
        key.unit,
        p.value(),
        tiles_str(kernel.viewer_hand()),
        tiles_str(kernel.pool()),
        tile(key.action)
    );
    let _ = writeln!(
        out,
        "  provenance only: corpus hand id {hand_id}, trick {N4_TRICK} (never identity components, freeze 45); freeze-set digest {FC_DIGEST}"
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

    let Some(r) = run_unit(&kernel, key.action, key.unit) else {
        let _ = writeln!(
            out,
            "  DECLARED STOP (freeze 44(b) v2, FC-A7(i)): a walk budget of {B_WALK} was exhausted. NO PARTIAL FOLD is retained, so there is no partial diagnostic; this is a stop and is never a finding (R-A18)."
        );
        return UnitText {
            named: out,
            stopped: true,
        };
    };

    // ---- (FC-R6) determinism: the in-run second pass ----------------------
    let r2 = run_unit(&kernel, key.action, key.unit)
        .expect("the second pass runs under the same budgets");
    assert_eq!(
        (
            r.n_states,
            r.n_arrivals,
            r.zero,
            r.pos,
            r.n_boss_pos,
            r.n_lead_pos,
            r.n_foll_pos,
            r.n_faces,
            r.n_face_masks,
            r.n_face_multi,
            r.steps
        ),
        (
            r2.n_states,
            r2.n_arrivals,
            r2.zero,
            r2.pos,
            r2.n_boss_pos,
            r2.n_lead_pos,
            r2.n_foll_pos,
            r2.n_faces,
            r2.n_face_masks,
            r2.n_face_multi,
            r2.steps
        ),
        "(FC-R6) stop-and-report: two passes of one unit disagree on an accounting integer"
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
        "(FC-R6) stop-and-report: two passes of one unit disagree on a summary value"
    );
    #[allow(clippy::needless_range_loop)]
    for f in 0..N_FEAT {
        assert_eq!(
            (
                r.feats[f].screened,
                r.feats[f].n_domain_all,
                r.feats[f].n_domain_swept,
                r.feats[f].n_zero,
                r.feats[f].n_pos,
                r.feats[f].n_s_nonzero,
                r.feats[f].n_s_nonzero_lead,
                r.feats[f].n_break_total,
                r.feats[f].n_multi_worlds,
                r.feats[f].n_tight
            ),
            (
                r2.feats[f].screened,
                r2.feats[f].n_domain_all,
                r2.feats[f].n_domain_swept,
                r2.feats[f].n_zero,
                r2.feats[f].n_pos,
                r2.feats[f].n_s_nonzero,
                r2.feats[f].n_s_nonzero_lead,
                r2.feats[f].n_break_total,
                r2.feats[f].n_multi_worlds,
                r2.feats[f].n_tight
            ),
            "(FC-R6) stop-and-report: two passes disagree on an accounting integer of feature {}",
            FEAT_NAME[f]
        );
        assert_eq!(
            (
                &r.feats[f].bound_sum,
                &r.feats[f].bound_max,
                &r.feats[f].ratio_max,
                &r.feats[f].rows
            ),
            (
                &r2.feats[f].bound_sum,
                &r2.feats[f].bound_max,
                &r2.feats[f].ratio_max,
                &r2.feats[f].rows
            ),
            "(FC-R6) stop-and-report: two passes disagree on a rational or a printed row of feature {}",
            FEAT_NAME[f]
        );
    }
    drop(r2);

    // ---- (FC-R2) the non-null pairing receipt, BLOCKING -------------------
    // FC-A6(ii): F2 at h0's LEADING part has s^± not both zero at at least one
    // state, and the count is emitted. It exists because FF-A26(iv) said the
    // next pre-declaration must require BY DESIGN what h2 got by luck, and it
    // is contentful in a way a null control cannot be: a diagnostic stuck at
    // zero satisfies (FC-R1) at every state and fails here.
    let r2_line = if key.unit == 0 {
        let f2 = &r.feats[3];
        assert!(
            f2.n_s_nonzero_lead >= 1,
            "(FC-R2) stop-and-report: F2 has s^+ = s^- = 0 at every one of the {} LEADING swept states of h0 a=00. The diagnostic is stuck at zero, which (FC-R1) cannot see, and FC-A7(b) declares this outcome a stop before any other reading is taken",
            r.n_lead_pos
        );
        format!(
            "  (FC-R2) THE NON-NULL PAIRING RECEIPT — HELD, BLOCKING, over the {} LEADING states of the {} SWEPT states of this ONE unit h0 a=00, at the ONE feature F2: s^+ and s^- are NOT BOTH EXACTLY 0 at {} of them, and at least one was required. Contentful in the way a null control cannot be — a diagnostic stuck at zero would satisfy (FC-R1) at every state and fail here, which is exactly the failure mode FF-A26(iv) said the next pre-declaration must catch BY DESIGN rather than by luck.",
            r.n_lead_pos, r.pos, f2.n_s_nonzero_lead
        )
    } else {
        String::from(
            "  (FC-R2) DOES NOT RANGE OVER THIS UNIT: FC-A6(ii) scopes it to F2 at h0 a=00's leading part, and it is asserted there. Nothing is claimed or checked by it here.",
        )
    };
    let _ = writeln!(out, "{r2_line}");

    // ---- (FC-R5) the rung-one invariance receipt --------------------------
    let d1_c = tax_to_count(r.delta1_diff);
    let u1_c = to_count(r.u1_diff, N4_GRADE);
    let u0_c = to_count(r.u0_diff, N4_GRADE);
    assert_eq!(
        (r.n_states, r.n_arrivals, r.zero, r.pos),
        (f_states, f_arr, f_zero, f_pos),
        "(FC-R5) stop-and-report: this probe's rung-one frontier census differs from the frozen table"
    );
    assert_eq!(
        (d1_c, u1_c, u0_c),
        (
            q(f_d1.0, f_d1.1),
            q(f_u1.0, f_u1.1),
            q(f_u0.0, f_u0.1)
        ),
        "(FC-R5) stop-and-report: this probe's Delta^(1), U^(1) or U^(0) differs from the frozen table"
    );
    assert_eq!(
        r.mass,
        qi(1),
        "stop-and-report: sum_I p_I != 1 — a field branch was dropped or double-counted"
    );
    assert_eq!(
        r.frontier_digest, f_digest,
        "(FC-R5)/(FT-R7c) stop-and-report: the frontier digest differs from the transcribed value"
    );

    let _ = writeln!(
        out,
        "  (FC-R5) RUNG-ONE INVARIANCE — HELD over ALL {} frontier states of this ONE unit: |I_1| = {}, arrivals {}, census {}/{} (zero/positive), Delta^(1) = {}, U^(1) = {}, U^(0) = {}, sum_I p_I = 1 exactly, every value against the frozen table. (FT-R7c) THE FRONTIER DIGEST — SHA-256 over the canonical serialisation of the (record, delta_I) pairs in freeze-38(d) order, one line per state as `<record>|<delta_I as num/den, count>` — ASSERTED EQUAL to the transcribed {}, which makes it a comparison against a PRIOR PROCESS and not a self-comparison: {}.",
        r.n_states,
        r.n_states,
        r.n_arrivals,
        r.zero,
        r.pos,
        qs(d1_c),
        qs(u1_c),
        qs(u0_c),
        if key.coord == 0 {
            "value the FIRST FF run filed at this coordinate"
        } else {
            "SR_FIRST value of second_rung.rs, a different program on a different day, the canonical serialisation being byte-identical"
        },
        r.frontier_digest
    );
    let _ = writeln!(
        out,
        "    (FT-R7a)'s CORRECTED SCOPE LINE, adopted verbatim: \"reaches sum_I delta_I and |supp delta_I| per unit across executions; does not reach individual delta_I.\" The digest is what extends that reach to the individual delta_I. ALL THREE UNITS ASSERT A DIGEST IN THIS BUILD — h0's slot was empty at the v1.1 run and was filed by it, so nothing is filed here and everything is checked (FT-A28(iv))."
    );
    let _ = writeln!(
        out,
        "  (FC-R7) THE FILED-FACE RECEIPT (FC-A11(iii)) — HELD at every one of the {} SWEPT STATES of this ONE unit, over the state set that has a filed minimal fusion core, which is exactly the swept set: this probe's own COMPLETE per-world argmax set at each fiber index named in the filed core equals the filed mask EXACTLY. {} masks compared, of which {} carry TWO tiles — those are the ones a tie-broken argmax would break, and a collapse there fails this receipt on the spot. Contentful in the strongest way available: the masks were produced by a DIFFERENT PROGRAM (fusion_tax.rs) on a different day and are receipted at (FT-R5). Also asserted: every filed core of this unit was claimed by exactly one swept state, and no swept state lacked one.",
        r.n_faces, r.n_face_masks, r.n_face_multi
    );
    let _ = writeln!(
        out,
        "    WHAT (FC-R7) DOES NOT REACH, named rather than hidden (FC-A11(iv)): it proves the face construction complete AT THE TWO WORLDS OF EACH FILED CORE, not at every world of every state. A collapse preserving exactly the core worlds and no others would survive it. The derivation is uniform across worlds and accumulates a SET BY EQUALITY ACROSS ALL j, tracking no index, which together with this receipt makes that residual remote. A named residual costs nothing; an unnamed one is how a chapter goes wrong."
    );
    let _ = writeln!(
        out,
        "  (FC-R6) DETERMINISM — HELD: a full in-run second pass with fresh maps, accumulators and budgets recomputed this unit entire; every accounting integer, every summary value, the frontier digest, the face-receipt counts and every printed row of all {N_FEAT} features asserted identical."
    );
    let _ = writeln!(
        out,
        "  STATE CENSUS FOR THIS ONE UNIT: {} frontier states, of which {} carry delta_I > 0 and are the SWEPT SET of freeze 52(b) (states with delta_I = 0 are counted and skipped: Lemma FF-min(d) gives delta_I^theta = 0 there and there is nothing to capture, and Proposition FC-drop(a) is vacuous). Of the {} swept states, LEADING {} and FOLLOWING {}. States WITH AN OUTSTANDING TRUMP — the domain of every boss-keyed feature, which is F0, F1 and F1g: {} of the {} swept states and {} of all {} frontier states.",
        r.n_states, r.pos, r.pos, r.n_lead_pos, r.n_foll_pos, r.n_boss_pos, r.pos, r.n_boss_all, r.n_states
    );

    #[allow(clippy::needless_range_loop)]
    for f in 0..N_FEAT {
        let fo = &r.feats[f];
        let _ = writeln!(out);
        let _ = writeln!(out, "  -- FEATURE {} ({}) --", FEAT_NAME[f], FEAT_KIND[f]);
        let _ = writeln!(
            out,
            "    DOMAIN CENSUS, over TWO NAMED SETS (freeze 52 v1.3): the domain of {} is nonempty at {} of ALL {} frontier states of this unit, and at {} of the {} SWEPT states of this unit.",
            FEAT_NAME[f], fo.n_domain_all, r.n_states, fo.n_domain_swept, r.pos
        );
        if fo.screened {
            let _ = writeln!(
                out,
                "    EMPTY TEST — DECLARED AND NOT MEASURED (FREEZE 52 v1.4, the per-(unit, feature) CELL screen with the null control exempt). The domain of {} is empty at every one of the {} swept states of this unit, so phi is identically 0 for every b at every one of them. Corollary FC-null then fixes s^+ = s^- = 0 there BY THEOREM, and no s^±, t_0 or bound is emitted for this cell: a theorem-fixed zero is not a measurement of the feature and must not be averaged into one (the FF-A11 fault, which typed six cells as vacuous by construction after the fact). NO ROW BELOW RANGES OVER THIS CELL.",
                FEAT_NAME[f], r.pos
            );
            continue;
        }
        let _ = writeln!(
            out,
            "    ARGMAX CARDINALITY PROFILE (freeze 53(c)) over the {} (state, world) ARRIVALS at the {} SWEPT states of this ONE unit: {} arrivals have a NON-SINGLETON clairvoyant argmax. This is the column that separates FC-A7's outcome (c) — genuine orthogonality, a unique argmax everywhere and s^± collapsing to -C_I — from outcome (d), a tie-driven zero in which the subgradient straddles 0 because the argmax is widely non-unique. It is emitted for exactly that reason and not as decoration.",
            fo.n_worlds_seen, r.pos, fo.n_multi_worlds
        );
        let _ = writeln!(
            out,
            "    STRADDLE CENSUS over the {} SWEPT states of this ONE unit at {} alone: s^- <= 0 <= s^+ holds at {} states (Proposition FC-drop(a): kappa_I = 0) and fails at {} (kappa_I > 0, and the bound below is what FC-drop(c) proves). States whose s^± are NOT BOTH EXACTLY 0: {}, of which {} are LEADING states.",
            r.pos, FEAT_NAME[f], fo.n_zero, fo.n_pos, fo.n_s_nonzero, fo.n_s_nonzero_lead
        );
        let _ = writeln!(
            out,
            "    BREAKPOINTS enumerated across the same {} SWEPT states of this ONE unit at {} alone: {}. By Proposition FF-degen a zero correlation with a POSITIVE breakpoint count is a MEASUREMENT (the feature has action content and does not lean on the clairvoyant choice), while zero correlation with ZERO breakpoints is a TAUTOLOGY (the feature had no content here at all).",
            r.pos, FEAT_NAME[f], fo.n_break_total
        );
        let bound_label = if FEAT_KAPPA[f].is_some() {
            "PROVED LOWER BOUND ON CAPTURE (Proposition FC-drop(c))"
        } else {
            "PROVED LOWER BOUND ON CAPTURE (Proposition FC-drop(c)), WHICH IS NOT A CAPTURE NUMBER AND IS NOT COMPARED AGAINST ONE"
        };
        let max_txt = if fo.bound_max.is_zero() {
            String::from(
                "no state of that set has a positive bound, so the largest single-state bound over it is exactly 0",
            )
        } else {
            format!(
                "the LARGEST single-state bound over that same set is {} (count) at I=[{}]",
                bqs(&fo.bound_max),
                fo.bound_max_rec
            )
        };
        let _ = writeln!(
            out,
            "    {} summed over the {} SWEPT states of this ONE unit at {} alone = {} (count convention); {}.",
            bound_label,
            r.pos,
            FEAT_NAME[f],
            bqs(&fo.bound_sum),
            max_txt
        );
        if FEAT_KAPPA[f].is_some() {
            let tight_txt = match &fo.ratio_max {
                None => String::from(
                    "no state of this cell has the straddle false, so no bound-to-kappa_I ratio exists over this set",
                ),
                Some((r_max, rec)) => {
                    let (lo, hi) = ppm_bracket(r_max);
                    format!(
                        "the bound is TIGHT (equal to the frozen kappa_I) at {} of the {} states of this cell where the straddle is false, and the LARGEST ratio bound/kappa_I over that same set is {} = between {} and {} parts per million (PRESENTATION ONLY), attained at I=[{}]",
                        fo.n_tight, fo.n_pos, bqs(r_max), lo, hi, rec
                    )
                }
            };
            let _ = writeln!(
                out,
                "    TIGHTNESS, which is FC-A7(h)'s pre-declared question: {}. A weak bound is a RESULT under F7 and not a null — a screening functional that is valid but loose is exactly what the trick-1 programme must know before relying on one.",
                tight_txt
            );
            let _ = writeln!(
                out,
                "    (FC-R3) ZERO CHARACTERISATION — HELD at every one of the {} SWEPT states of this ONE unit at the ONE feature {}: kappa_I = 0 <=> s^- <= 0 <= s^+, with kappa_I taken from the FROZEN v1.1 table. (FC-R4) THE DROP RECEIPT — HELD at every state of that same set where the straddle is FALSE, {} of them: kappa_I >= |s| * t_0 against the same frozen kappa_I. THESE TWO ARE RECEIPTS ONLY BECAUSE THEY COMPARE AGAINST THE FROZEN v1.1 kappa_I (FC-A6(vii)) — a value produced by a different program in a different run; recomputed from this probe's own quantities they would be arithmetic remarks and could not fail.",
                r.pos, FEAT_NAME[f], fo.n_pos
            );
        } else {
            let _ = writeln!(
                out,
                "    NO (FC-R3) AND NO (FC-R4) AT THIS CELL, and the reason is stated rather than left to inference (FC-A10(i),(v)): the v1.1 run swept F0 and F2 only — F1 was settled at FF-A13 and not computed there, and F1g appears for the first time in this chapter — so NO FROZEN kappa_I EXISTS FOR THIS FEATURE AT ANY STATE, no comparison column exists, and none is manufactured. What IS emitted is the FC-drop bound, which FC-A10(v) rules is a DIFFERENT OBJECT from a capture number: by Proposition FC-drop(c) a positive |s| * t_0 PROVES capture is at least that much, with no minimisation and nothing to compare against. It is self-standing evidence, and it is labelled a proved lower bound on capture in every sentence and never \"capture\"."
            );
        }
        if f == 0 {
            let _ = writeln!(
                out,
                "    (FC-R1) THE NULL-CONTROL RECEIPT — HELD, BLOCKING, asserted at every one of the {} SWEPT states of this ONE unit BEFORE any F1, F1g or F2 number was computed: s^+ = s^- = 0 EXACTLY. Its answer is fixed by COROLLARY FC-null — by theorem, not by a filed number — and it tests the per-action centring, the complete-face construction, the exact accumulation and the sign discipline at once, which is the (SR-R9)/(FF-R1) role.",
                r.pos
            );
        }
        let _ = writeln!(
            out,
            "    ARITHMETIC REMARKS, NOT RECEIPTS (Proposition SR-taut, FC-A6(vii); they cannot fail and are excluded from every HELD count): s^- <= s^+, which is convexity and holds by construction because a face maximum is never below its minimum; t_0 > 0 wherever it exists; |s| * t_0 >= 0; and kappa_I >= 0 as RE-DERIVED FROM THIS PROBE'S OWN QUANTITIES."
        );
        let _ = writeln!(
            out,
            "    PER-STATE ROWS — one row per state of the {} SWEPT states of this ONE unit at the ONE feature {}, in freeze-38(d) record order. COUNT CONVENTION on t_0, on the bound and on kappa_I, a tax being exactly half its differential value; s^+ and s^- are CONVENTION-FREE, being mass-weighted feature quantities in which no value enters — q is read only to decide WHICH b lie in the argmax:",
            r.pos, FEAT_NAME[f]
        );
        for row in &fo.rows {
            let _ = writeln!(out, "{row}");
        }
    }

    let _ = writeln!(
        out,
        "  walk-step observables (SEP-A19(b) class, PROVENANCE ONLY — never a cost, timing or tractability claim): the rung-one frontier pass charged {} walk-steps, residual {}; the filed FT PATH A subtotal for this (coordinate, action) is {f_steps} — {}. The diagnostic itself is negligible beside the traversal and is not separately budgeted.",
        r.steps,
        r.residual,
        if r.steps == f_steps { "EQUAL" } else { "DIFFERS" }
    );
    let _ = writeln!(
        out,
        "  SETTLED A PRIORI (FC-A1): by Proposition SR-degen no closure verdict exists at grade 4 for this coordinate, and NOTHING MEASURED HERE CHANGES ANY VERDICT. This chapter MEASURES AN INSTRUMENT, NOT THE GAME. Any sentence implying a grade-4 verdict moved is void on its face."
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
        "walt FC FEE-CORRELATION DIAGNOSTIC — FREEZE 53, Proposition FC-drop measured at every swept state of three units and four features — EXPLORATORY TIER"
    );
    let _ = writeln!(
        out,
        "freezes this run executes under: FREEZE 53 (the fee-correlation diagnostic) with FREEZE 52 v1.4 (the per-(unit, feature) cell screen, null control exempt), and beneath them freezes 7, 23, 26, 37(d), 38 v1.1, 44(b) v2, 45, 50 v1.1; Proposition FC-drop and Corollary FC-null; Proposition FF-blind, Lemma FF-min, Proposition FF-oracle, Proposition FF-degen; Proposition FT-flat, Lemma FT-arrive, Lemma FT-post, Corollary FT-conv, Corollary FT-grade4, Proposition SR-degen, Proposition SR-taut. THIS LINE NAMES THE FREEZES THE RUN EXECUTED UNDER AND NO RULING WRITTEN AFTER THE RUN, and it will not be amended when this run is adjudicated (the backwards-provenance trap)."
    );
    let _ = writeln!(
        out,
        "regenerate (deterministically, from the repository alone): cargo run --release -p walt-factory --example fc_correlation"
    );
    let _ = writeln!(out, "freeze-set digest (freeze 53(g)): {FC_DIGEST}");
    let _ = writeln!(
        out,
        "emission: this file is committed ENTIRE — there is no companion (FC-A5(f)); the emission is small enough to commit and it should be."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "TIER: exploratory throughout, without exception (FC-A1), below every tier like everything in walt. Nothing here is promoted, nothing is cited by anything above this tier, and nothing is quotable as a result except by brief amendment adding it to a verifier receipt. DS-A1 binds: witness, receipt, necessary outer profile. Both outcomes of every gate are results (F7). A receipt failure is stop-and-report, never a patch (NO-RESCUE)."
    );
    let _ = writeln!(
        out,
        "WHAT THIS CHAPTER IS: it MEASURES AN INSTRUMENT, NOT THE GAME. Every number below is about how a fee behaves at coordinates whose exact answers are ALREADY FILED, and by Proposition SR-degen no grade-4 verdict can move."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== PROPOSITION FC-drop, PRINTED BEFORE ANY NUMBER EXISTS — WHAT IS BEING MEASURED AND WHY IT IS WORTH MEASURING =="
    );
    let _ = writeln!(
        out,
        "  Fix a frontier state I with delta_I > 0. With Lemma FF-min's G_I and Phi_I(omega,b) = phi_I(omega,b) - c_I(b), write s^+ = G_I'(0^+) = sum_omega mu_I(omega) max over b in argmax_b q_I(omega,b) of (-Phi_I(omega,b)), and s^- = G_I'(0^-) with min in place of max; so s^- <= s^+ by convexity. Let the state's CAPTURED AMOUNT be kappa_I = delta_I - min_theta delta_I^theta = G_I(0) - min_theta G_I(theta)."
    );
    let _ = writeln!(
        out,
        "  (a) ZERO TEST: kappa_I = 0 IFF s^- <= 0 <= s^+. When the clairvoyant argmax is UNIQUE at every positive-mass omega, both collapse to -C_I and kappa_I = 0 iff C_I = 0. (b) DESCENDING SIDE IS POPULATED: if s^+ < 0 there is a breakpoint strictly to the right of 0; if s^- > 0, one strictly to the left. (c) THE DROP BOUND: with t_0 the distance from 0 to the nearest breakpoint on the descending side, kappa_I >= |s| * t_0, exact in rationals and requiring NO MINIMISATION — one directional slope and one breakpoint distance."
    );
    let _ = writeln!(
        out,
        "  WHAT IT IS AND WHAT IT IS NOT: a LOWER bound. A large value proves a fee bites; A SMALL VALUE PROVES NOTHING, because the true drop may continue past t_0 across many further pieces. Its content is that CAPTURE IS AT LEAST CORRELATION TIMES REACH — |s| measures how far the feature leans on the clairvoyant choice, t_0 how far the fee can be pushed before that choice starts changing."
    );
    let _ = writeln!(
        out,
        "  HOW t_0 IS TAKEN, stated because it is the one place the bound could be read as stronger than it is: the breakpoints are the set LEMMA FF-min ENUMERATES — the kinks of the per-world upper envelopes, which is the same set the v1.1 sweep evaluated at and the same count it filed. If two worlds' kinks were to CANCEL at one sigma, that sigma is a candidate breakpoint but not a kink of G_I, and t_0 would then be SMALLER than the first true kink. That direction is safe: G_I is still affine on [0, t_0], so G_I(t_0) = G_I(0) - |s| * t_0 holds exactly and the bound stays PROVED, merely weaker. The bound is never made larger by this choice."
    );
    let _ = writeln!(
        out,
        "== COROLLARY FC-null, PRINTED BEFORE ANY NUMBER EXISTS — WHY F0 IS THE NULL CONTROL =="
    );
    let _ = writeln!(
        out,
        "  If phi_I(omega,b) = psi_I(omega) does not depend on the action then Phi_I is b-free, the inner max and min over the active set both return -Phi_I(omega), and s^+ = s^- = -sum_omega mu_I(omega)(psi_I(omega) - c_I) = 0 by the definition of c_I; hence kappa_I = 0, recovering Proposition FF-blind. THE NULL CONTROL'S ANSWER IS THEREFORE FIXED BY THEOREM AND NOT BY A FILED NUMBER, which is what makes (FC-R1) worth running at every unit even where the feature's domain is empty."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== THE SHARPEST HAZARD IN THIS BUILD, DECLARED BEFORE ANY NUMBER EXISTS (freeze 38(e), FC-A10(vi), FC-A11) =="
    );
    let _ = writeln!(
        out,
        "  s^+ and s^- ARE TAKEN OVER THE COMPLETE CLAIRVOYANT ARGMAX SET at every world, never a tie-broken representative. A tie-broken argmax collapses s^- and s^+ into a SINGLE number, which turns FC-drop(a)'s STRADDLE test into a POINT test — and the straddle is precisely what distinguishes outcome (c), genuine orthogonality, from outcome (d), a tie-driven zero. It would not perturb this chapter's headline; it would silently answer its central question with the WRONG ONE OF TWO PRE-DECLARED READINGS, with every other receipt green."
    );
    let _ = writeln!(
        out,
        "  THE CODE SHAPE THAT DEFECT TAKES is the natural Rust idiom, an enumerate().max_by_key() returning ONE index. This probe accumulates a SET BY AN EQUALITY TEST ACROSS ALL j and tracks no index, mirroring fusion_tax.rs's own walk. (FC-R7) checks that construction against faces filed by that different program, and the emitted ARGMAX CARDINALITY PROFILE (freeze 53(c)) is what lets a reader see the (c)-versus-(d) distinction directly rather than trusting it."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== ALL OUTCOMES PRE-DECLARED (FC-A7 with FC-A10(vii), printed before any number exists; F7 binds) =="
    );
    let _ = writeln!(
        out,
        "  (a) F0's s^± is ANYTHING OTHER THAN EXACTLY 0 -> STOP-AND-REPORT. The harness is wrong and NO OTHER NUMBER IS REPORTED. This gate is BLOCKING and comes first."
    );
    let _ = writeln!(
        out,
        "  (b) (FC-R2) FAILS — F2 at h0's leading part has s^± = 0 at every state -> STOP-AND-REPORT, and it is the failure a null control could not have caught, because a diagnostic stuck at zero satisfies (FC-R1) perfectly."
    );
    let _ = writeln!(
        out,
        "  (c) AT h2, s^+ = s^- = 0 EXACTLY AT EVERY STATE -> F2 IS EXACTLY ORTHOGONAL TO THE CLAIRVOYANT CHOICE THERE; the h2 refutation acquires its mechanism, and an exact rational identity at 432 states becomes a structural fact about this coordinate demanding an explanation the branch does not yet have. THIS IS THE OUTCOME THAT WOULD MOST CHANGE WHAT WE DO NEXT."
    );
    let _ = writeln!(
        out,
        "  (d) AT h2, s^- < 0 < s^+ WITH s^± NOT BOTH ZERO -> THE ZERO IS TIE-DRIVEN: the clairvoyant argmax is widely non-unique and the subgradient straddles zero. A mundane explanation, and the h2 refutation becomes a statement about ARGMAX MULTIPLICITY rather than about the feature. The cardinality profile is what tells (c) from (d)."
    );
    let _ = writeln!(
        out,
        "  (e) MIXED ACROSS STATES -> report the split; no single mechanism."
    );
    let _ = writeln!(
        out,
        "  (f) F1g's CORRELATION IS ZERO AT EVERY STATE OF THE CELLS WHERE IT IS MEASURED -> GRADED F1 IS REFUTED by FC-drop(a) with no sweep, and FF-A15(iv)'s open item is discharged."
    );
    let _ = writeln!(
        out,
        "  (g) F1g's BOUND IS POSITIVE SOMEWHERE -> by Proposition FC-drop(c) F1g's capture IS POSITIVE at that state, BY THEOREM, magnitude unknown; a sweep to measure how much is warranted and is commissioned IN ITS OWN LATER RULING, never inherited from this one. THIRD READING (FC-A10(vii)): s^± NONZERO BUT THE BOUND ZERO OR NEGLIGIBLE EVERYWHERE -> CORRELATION WITHOUT REACH, the most interesting of the three for characterising the instrument, since it is the case where a screening functional used alone would mislead."
    );
    let _ = writeln!(
        out,
        "  (h) THE BOUND IS TIGHT NOWHERE -> the drop bound is valid but WEAK, and is reported as an instrument that SCREENS rather than PREDICTS; that is a result under F7 and not a null, because a weak screening bound is exactly what the trick-1 programme must know before relying on one."
    );
    let _ = writeln!(
        out,
        "  (i) A BUDGET STOP -> declared stop, no partial fold, printed as a stop and never as a finding (R-A18, freeze 44(b))."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "P-A21 — THE FENCE MOST AT RISK HERE AND THEREFORE PRINTED FIRST AND LARGEST (FC-A9(i)): NO QUANTITY MEASURED AT GRADE 4 IS QUOTED FOR TRICK 1 OR FOR THE OPENING. It binds HARDEST in this chapter, because a screening functional is FOR trick 1 and will be tempting to quote there; the aim is what makes the fence fragile, not what relaxes it."
    );
    let _ = writeln!(
        out,
        "SELECTION FENCE (SR-A25(iii), FT-A26(iii)): THREE UNITS AT TWO COORDINATES CHOSEN BY NEGATIVE BINDING MARGIN ARE A CARRIER, NOT A SAMPLE. No count, ratio or bound below is a rate, and none of them estimates anything about states outside this carrier."
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
        "THE LOAD-BEARING RISK, undiminished (SR-A25(vii)), with T1-A12's corpus check still owed: if the implementation and the rules corpus disagree, the mathematics is still correct and its application here is wrong, and no receipt inside this file can detect it, because every receipt is computed by the same implementation. (FC-R1) is the partial guard — its value is known by proof — and (FC-R7) is the second, because it compares against a different program's output."
    );
    let _ = writeln!(
        out,
        "NOT CLAIMED, printed in place (FC-A9(ii)): that s^± PREDICTS capture — FC-drop is a LOWER bound and a small value proves nothing; that any mechanism found at h2 generalises anywhere; that any grade-4 verdict moved (Proposition SR-degen forbids it); anything about points, marks, bidding or how real opponents play; no cost or tractability claim off any traversal observable (SEP-A19(b)); and NOTHING WHATEVER ABOUT WHETHER JASON'S READING OF h0 AT THE TABLE IS CORRECT — FF-A9(ii) travels unchanged, and a fee's correlation is not a statement about the reasoning that suggested the feature."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== THE FEE AND ITS CENTRING (FF-A2, freeze 52(c)), UNCHANGED AND STILL THE MOST LIKELY WAY TO PRODUCE A PLAUSIBLE WRONG NUMBER =="
    );
    let _ = writeln!(
        out,
        "  lambda_I(omega,b) = theta * (phi_I(omega,b) - c_I(b)) with c_I(b) = sum_omega mu_I(omega) phi_I(omega,b) / p_I. PER-ACTION CENTRING IS MANDATORY: a single per-state centre does not satisfy Theorem 12.1's hypothesis when phi depends on b, and a fee that is not centred per action is not a valid upper witness at all. Every c_I(b) below is computed per action."
    );
    let _ = writeln!(
        out,
        "CONVENTION (freeze 38(f), Corollary SR-conv), with the TWO BRIDGES KEPT SEPARATE: every evaluator runs in the trick DIFFERENTIAL; a TAX column is in the COUNT convention, a tax being a difference at a common state and therefore exactly HALF its differential value, while a VALUE column crosses by count = (diff + grade)/2. t_0, the FC-drop bound and the frozen kappa_I are TAX columns and are printed in the count convention. s^+ and s^- ARE CONVENTION-FREE: they are mass-weighted feature quantities, sum_omega mu_I(omega) * (-Phi_I(omega,b)), in which no value enters at all — q is read ONLY to decide which b lie in the argmax."
    );
    let _ = writeln!(
        out,
        "EXACT ARITHMETIC (P-A19, freeze 53(e)): exact integers and rationals everywhere, NO FLOAT ANYWHERE (clippy -D clippy::float_arithmetic and a no-float grep bind, and release builds carry overflow-checks). Every divisibility that must be exact is ASSERTED rather than assumed, and every hot multiplication and accumulation is CHECKED: an overflow is stop-and-report, never a wrap."
    );
    let _ = writeln!(
        out,
        "BELIEF AND FIELD ARE NOT RE-DECLARED (freeze 53(g)): freeze 26 and 37(d), uniform over the full enumerated fiber, NO DECIMATION inside anything ((C2)). No library entry is written at any coordinate (freeze 45). The freeze-set digest travels on every record."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== THE FEATURES (FREEZE 53(b), four; every feature carries ITS OWN domain clause and no blanket clause spans the family) =="
    );
    let _ = writeln!(
        out,
        "  \"OUTSTANDING TRUMP\" means the highest-ranking trump under the declaration among tiles NOT YET PLAYED IN THE RECORD AND NOT IN THE FOCAL SEAT'S HAND AT I — i.e. held by a field seat in omega; h(omega) is the seat holding it. Its IDENTITY is a function of the record and that invariance is ASSERTED at every arrival; only its HOLDER varies across X_I."
    );
    let _ = writeln!(
        out,
        "  (F0) NULL CONTROL — boss_owner: phi = 1 if h(omega) is an opponent of the focal seat, 0 if a partner. ACTION-BLIND. DOMAIN CLAUSE: references h(omega), so 0 where no trump is outstanding. RUN FIRST AND BLOCKING and EXEMPT from freeze 52 v1.4's screen in all cases, because its job is to test the harness rather than the feature, and a harness check that skips the states where the feature is empty is precisely a harness check that has not been run where it is cheapest to run."
    );
    let _ = writeln!(
        out,
        "  (F1) boss_can_follow_b: phi = 1 if h(omega) holds at least one tile of b's suit under the declaration, computed as Decl::led_context(b) with membership by Decl::effective_incidence. DOMAIN CLAUSE: references h(omega). REFUTED AS A BINARY at FF-A13; it is re-measured here only as the parent of F1g."
    );
    let _ = writeln!(
        out,
        "  (F1g) THE GRADED FORM, NEW AT FREEZE 53(b): the COUNT of b-suit tiles h(omega) holds — the cardinality of the very intersection whose EMPTINESS F1 tests. The by-construction agreement F1 = 1 <=> F1g >= 1 IS ASSERTED IN PLACE at every (omega, b) rather than left implicit. phi is NOT BINARY here, so this probe carries phi as a small integer per (feature, action) and not as the v1.1 probe's per-action bitmask. DIAGNOSTIC-ONLY per FC-A4: NO CAPTURE NUMBER IS COMPUTED OR REPORTED FOR F1g."
    );
    let _ = writeln!(
        out,
        "  (F2) b_is_beatable, AMENDED at FF-A12 and freeze 52 v1.1: phi = 1 iff some opponent WHO HAS NOT YET PLAYED AT I holds a tile that would win the trick OVER b AND OVER EVERY TILE ALREADY ON THE TABLE. DOMAIN CLAUSE: F2 does NOT reference h(omega) and its domain is THE WHOLE FIBER. Computed from Decl::beats and Decl::trick_key — the rule algebra's own BEATS relation and ordering — and NEVER by a re-implementation."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== WHAT F1 AND F1g EMIT, AND WHY THERE IS NO COMPARISON COLUMN FOR THEM (FC-A10(i),(v)) =="
    );
    let _ = writeln!(
        out,
        "  The v1.1 run swept F0 AND F2 ONLY — 1,332 swept states at h0 a=00 and 216 at each of h2 a=53 and h2 a=54, 1,764 per feature and 3,528 rows in all. F1 was settled at FF-A13 and not computed there; F1g had never been computed before this chapter. NO FROZEN kappa_I EXISTS FOR F1 OR F1g AT ANY STATE, so those cells carry NO kappa_I COLUMN AND NO COMPARISON, and every one of their rows says so in place."
    );
    let _ = writeln!(
        out,
        "  THEY DO EMIT THE FC-drop BOUND, which is a DIFFERENT OBJECT from a capture number: by Proposition FC-drop(c) a positive |s| * t_0 PROVES capture is at least that much, with no minimisation and no kappa_I to compare against — self-standing evidence rather than a measurement awaiting a check. IT IS LABELLED \"PROVED LOWER BOUND ON CAPTURE (Proposition FC-drop(c))\" IN EVERY SENTENCE AND COLUMN AND IS NEVER CALLED \"CAPTURE\"."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== THE RECEIPTS (FC-A6, six, with (FC-R7) added at FC-A11; the non-receipts named) =="
    );
    let _ = writeln!(
        out,
        "  (FC-R1) THE NULL CONTROL — BLOCKING AND FIRST, before any other number: F0's s^+ = s^- = 0 EXACTLY at every swept state and every unit, the answer fixed by Corollary FC-null. (FC-R2) THE NON-NULL PAIRING RECEIPT — BLOCKING: F2 at h0's LEADING part has s^± not both zero at AT LEAST ONE state, and the count is emitted. Contentful, because a diagnostic stuck at zero would satisfy (FC-R1) and fail here, which is exactly the failure mode a null control alone cannot see. (FC-R3) THE ZERO CHARACTERISATION: kappa_I = 0 <=> s^- <= 0 <= s^+ at every swept state, at EVERY FEATURE CARRYING A FROZEN kappa_I — F0 AND F2 — against the frozen v1.1 table. (FC-R4) THE DROP RECEIPT: kappa_I >= |s| * t_0 wherever the straddle is false, same feature set, same frozen table. (FC-R5) RUNG-ONE INVARIANCE and the (FT-R7c) frontier digest at ALL THREE UNITS. (FC-R6) DETERMINISM: an in-run second pass with fresh maps, accumulators and budgets. (FC-R7) THE FILED-FACE RECEIPT: this probe's own COMPLETE per-world argmax set equals the mask fusion_tax_2026-08-14.txt filed, at every fiber index of every filed minimal fusion core, at every swept state of all three units."
    );
    let _ = writeln!(
        out,
        "  NAMED AS NON-RECEIPTS and printed as ARITHMETIC REMARKS (Proposition SR-taut, FC-A6(vii); they cannot fail and are excluded from every HELD count): s^- <= s^+; t_0 > 0; |s| * t_0 >= 0; and kappa_I >= 0 as re-derived from this probe's own quantities. AND THE SENTENCE THAT MATTERS MOST: (FC-R3) AND (FC-R4) ARE RECEIPTS ONLY BECAUSE THEY COMPARE AGAINST THE FROZEN v1.1 kappa_I, a value produced by a different program in a different run; against this probe's own recomputed capture they would be tautologies."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== FF-A18 AS GENERALISED AT FC-A10(iv), WHICH GOVERNS EVERY FIGURE BELOW =="
    );
    let _ = writeln!(
        out,
        "  A RECEIPT'S OR A FIGURE'S SCOPE NAMES EVERY DIMENSION IT RANGES OVER — STATE SET, FEATURE SET, UNIT SET — IN THE SAME SENTENCE, AND A SCOPE DERIVED FROM AN ADJECTIVE RATHER THAN STATED AS A SET IS NOT A SCOPE. The rule was generalised because \"every swept feature\" denoted one set in the v1.1 frame and a different one here, which is the third appearance of that family. This file reports over three nested state sets at h0 (16,136 frontier; 1,332 swept; 574 leading) and two at each h2 unit (330 frontier; 216 swept), over four features, over three units, and every figure below names which of each."
    );
    let _ = writeln!(
        out,
        "  FREEZE 52(b) GOVERNS THE MEASURED SET (FF-A23(ii)): the states with delta_I > 0 are diagnosed — 1,332 of h0's 16,136 and 216 of each h2 unit's 330 — and the delta_I = 0 states are counted and skipped, where Proposition FC-drop(a) is vacuous. FREEZE 52 v1.3: the DOMAIN CENSUS is emitted over EVERY frontier state because it is a SCREEN, and both censuses are labelled with the set they count."
    );
    let _ = writeln!(
        out,
        "  FREEZE 52 v1.4 (FF-A33(iii)): the screen applies PER (unit, feature) CELL and not per unit, and the null control is EXEMPT IN ALL CASES. A cell whose domain is empty at every swept state is declared an EMPTY TEST and is not measured — its s^± would be 0 by Corollary FC-null, a theorem-fixed zero that is not a measurement of the feature and must not be averaged into one."
    );
    let _ = writeln!(
        out,
        "  FF-A32(v), BINDING ON THIS FILE: a results file may restate a reading-rule a ruling has fixed and may NEVER ORIGINATE ONE. Every reading-rule above is quoted from the ruling that fixed it."
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
        "== THE CARRIER — FREEZE 53(a), enumerated with NO GENERATING RULE (FT-A23: a freeze is a constant, not a rule); THE SAME THREE UNITS AS FREEZE 52 v1.1 AND NO NEW COORDINATE =="
    );
    let _ = writeln!(
        out,
        "  ARM 1 — h0, pip 3, hand [00 21 32 53], the SINGLE unit a = 00 (unit index 0). It is the cheapest unit in the carrier and the hand the feature came from."
    );
    let _ = writeln!(
        out,
        "  ARM 2, ATTEMPTED AFTER ARM 1 COMPLETES, WITH A DECLARED STOP — h2, pip 5, hand [21 33 53 54], units a = 53 (index 1) then a = 54 (index 2). h6, h9 and h12 are OUT OF SCOPE (FF-A5). The h2 units are where F2's capture is EXACTLY ZERO at 216 states twice over with 3,126 breakpoints proving the fee genuinely varied, and explaining that zero is what this chapter was commissioned for."
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
                "\n== ARM 2 NOT ATTEMPTED (freeze 53(a), FC-A7(i)): arm 1 did not complete, and arm 2 is attempted only after arm 1 completes. This is a declared stop and never a finding (R-A18). =="
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
                        FC_FILED[units_ref[ui].coord].0,
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
        "== WHAT THIS BUILD DOES NOT SAY, restated at the foot because a correlation travels badly =="
    );
    let _ = writeln!(
        out,
        "  Every bound above is a LOWER bound on capture (Proposition FC-drop(c)): a large value proves a fee bites and A SMALL VALUE PROVES NOTHING. s^± does not predict capture. No mechanism found at h2 is claimed to generalise. No grade-4 verdict moved and none could (Proposition SR-degen). Nothing measured here is quoted for trick 1 or for the opening (P-A21), which binds hardest here precisely because a screening functional is FOR trick 1. Three units at two coordinates chosen by negative binding margin are a CARRIER, NOT A SAMPLE. And nothing here says anything whatever about whether Jason's reading of h0 at the table is correct."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "total wall-clock (provenance only, never a dividend and never a receipt; SEP-A19(b), DS-A31/DS-A36): {} ms",
        t0.elapsed().as_millis()
    );
    let _ = writeln!(out, "run complete: yes");

    let results = out_dir("results").join("fc_correlation_2026-08-14.txt");
    std::fs::write(&results, &out).expect("write results");
    print!("{}", &out[..out.len().min(3000)]);
    println!("results: {}", results.display());
}
