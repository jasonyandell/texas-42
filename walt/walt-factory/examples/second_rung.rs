//! walt second-rung probe — the SR family of SR-A22: the EXACT depth-two
//! layer over freeze 51's carrier. EXPLORATORY TIER THROUGHOUT, without
//! exception.
//!
//! Rulings: SR-A1..SR-A26 (`walt/CENSUS-RULINGS.md`, 2026-08-14), with
//! Lemma SR-coord (the free product at the first frontier and the unique
//! parent at the second), Lemma SR-forced (gluing a forced decision is free),
//! Proposition SR-sep, Proposition SR-post, Corollary SR-conv, Proposition
//! SR-degen and Proposition SR-taut; on the whole FT chapter beneath it
//! (FT-A1..FT-A29, Lemma FT-arrive, Lemma FT-trunc, Corollary FT-grade4,
//! Proposition FT-flat, Lemma FT-post). Freezes: 38 v1.1(d) (the rung-two cut
//! order, SR-A21), 44 v2 (walk-step unit and budgeted walk contract), 45 (the
//! n = 4 coordinate identity), 51 (this probe's carrier, SR-A22(iii)).
//!
//! What it computes, per carrier unit — a (coordinate, binding competitor
//! action `a`) pair of freeze 50 — is SR-A22(i)'s list: the second-frontier
//! data `mu_{I,b,J}(omega)` and `q_{I,b,J}(omega,c)`; per `(I,b,J)` the
//! aggregates `C_{I,b,J}`, every `A_{I,b,J,c}` and `delta_{I,b,J}`; per
//! `(I,b)` the branch values `F^(1)_{I,b}`, `F^(2)_{I,b}`, the slack
//! `s_{I,b}` and the downstream tax `d_{I,b}`; per `I` the complete rung-one
//! optimal face `B*_I`, the complete argmin set of `s + d`, the local tax
//! `Delta_I^(2) = min_b (s_{I,b} + d_{I,b})` and the ESCAPE flag; and per unit
//! `U^(2) = sum_I max_b F^(2)_{I,b}` and `Delta^(2) = sum_I Delta_I^(2)`.
//!
//! The four engine changes SR-A22(ii) names, every one ASSERTED and never
//! assumed: (1) the arrival denominator keeps accumulating THROUGH the second
//! frontier instead of freezing at the first; (2) the banked prefix likewise
//! carries the between-frontier increment; (3) `seen_focal` is a DEPTH
//! COUNTER, not a bool; (4) the depth-two common denominator is `12^12`,
//! which is `SCALE` — `DEN_MU = 12^6` carries only the pre-frontier-1
//! denominators.
//!
//! Two independently written computations of `U^(2)` are carried (SR-R6):
//! PATH A2 is world-major — one revealed walk per world, recording BOTH the
//! first-frontier table (which reproduces rung one, SR-R7) and the
//! second-frontier table; PATH B2 is the GLUE-TWO-THEN-REVEAL walker — a
//! pooled bag lawful at the first frontier and lawful again at the second
//! (`max` outside the world sum at BOTH), world-informed below. They share
//! the rule algebra and nothing else.
//!
//! Arithmetic (freeze 38(f), Corollary SR-conv): exact integers and rationals
//! only, no float anywhere (P-A19). Every reported column is in the COUNT
//! convention. A difference at a common state (`s`, `d`, `delta`, `Delta`)
//! scales purely and is exactly twice its count value; a `p`-weighted VALUE
//! (`F^(1)`, `F^(2)`, `M_I`, `C`, `A`) takes the additive `c*p` of Corollary
//! SR-conv and is bridged as `(x_diff + grade*p)/2`. Mixing the two is the
//! failure mode most likely to produce a near-miss that looks like a
//! discovery, so the two bridges are separate functions here.
//!
//! Filed values enter as FROZEN SOURCE TABLES in this source (SR-A22(v), the
//! SEP-A14(ii) pattern), quoted from `separation_n4_2026-08-14.txt` and
//! `fusion_tax_2026-08-14.txt`, exploratory tier — never re-parsed from
//! results text at run time.
//!
//! No floats. Regenerate:
//! `cargo run --release -p walt-factory --example second_rung`

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Instant;

use walt_core::receipt::{locate_verify_player, parse_file, Receipt, ReceiptHand};
use walt_core::replay::state_before_trick;
use walt_core::{
    legal_plays, Context, ContextSet, Decl, Domino, DominoSet, Pip, Seat, Team, Trick,
};
use walt_geom::{q, qi, Q};
use walt_kernel::{Hidden, Kernel, HIDDEN_SEATS};
use walt_strat::{information_prices, Direction};

// -- freezes and declared constants -----------------------------------------

/// Freeze 44(b) v2, unchanged by SR-A22(iv): B walk-steps per (coordinate,
/// action) for EACH walk-based evaluator. PATH A2 and PATH B2 are two
/// evaluators and carry one budget each; charge-then-descend at `bag.len()`
/// on entry; on exhaustion `None` and NO PARTIAL FOLD of any kind is
/// retained — which here means no partial `s`, no partial `d`, no partial
/// `Delta^(2)` and no partial `U^(2)`.
const B_WALK: u64 = 10_000_000_000;

/// Freeze 44(b) v2's partition-state cap. `sum_{I,b} |I_2(I,b)|` is asserted
/// against it BEFORE the aggregate pass, and the assertion is contentful.
const P_MAX: u64 = 192_000_000;

const N4_TRICK: usize = 4;
const N4_GRADE: usize = 4;
const N4_FIBER: u128 = 34_650;

/// SR-A22(iii)(d): the committed file prints the branch rows entire at h2
/// (at most `330 * 3 = 990` per unit). A cap is DECLARED so a future run
/// cannot smuggle a truncation; if it ever fires the file carries the top
/// rows by descending `s + d`, ties by ascending emission order, plus the
/// residual tail's exact count — a declared truncation, never a silent one.
const ROW_CAP: usize = 20_000;

/// The named file's header carries the companion's SHA-256, which is only
/// known after the companion is written. The header is assembled first, so it
/// carries this marker and the marker is substituted once, at write time.
const COMPANION_LINE_PLACEHOLDER: &str = "@@COMPANION@@";

/// Freeze 51(h): the freeze-set digest travelling on every record. No cache
/// is read or written by this probe, so freeze 41/DS-A30's discard rule is
/// vacuous here and is stated rather than exercised.
const SR_DIGEST: &str = "SR-v1|freezes-7-23-26-37d-38v1.1d-44v2-45-50v1.1-51|contract=R-A11-full-record|field=uniform-legal-F4|belief=uniform-fiber-freeze7";

/// `12^12`. The deep solve's fixed scale: a node with `j` field plies below
/// it has a value whose denominator divides `12^j`, and at most 12 field
/// plies remain below a grade-4 root action.
const SCALE: i64 = 8_916_100_448_256;

/// `12^6`. The common denominator of every PRE-FRONTIER-1 arrival weight: at
/// most 6 field plies separate the root action from the focal seat's next
/// decision. This is SR-A22(ii)(4)'s point — it covers rung one and nothing
/// below it.
const DEN_MU: i128 = 2_985_984;

/// `12^12`, the DEPTH-TWO common denominator of SR-A22(ii)(4), which is
/// `SCALE`. At most six field plies precede the first frontier and at most
/// six more precede the second, so every depth-two arrival denominator
/// divides it. Asserted equal to `SCALE` and asserted to divide exactly at
/// every second-frontier arrival — never assumed.
const DEN2: i128 = 8_916_100_448_256;

/// The maximum record length this probe encodes: a grade-4 coordinate has
/// four tricks below the root action's trick start, hence at most 16 plays.
const REC_MAX: usize = 16;

// -- the frozen source tables (SR-A22(v), the SEP-A14(ii) pattern) -----------

/// One filed root-action row: `(hi, lo, Q^H as (num, den), U as (num, den),
/// revealed walk-steps)`, the two values in the COUNT convention.
type FiledRow = (u8, u8, (i128, i128), (i128, i128), u64);

/// One filed carrier coordinate: `(corpus hand id, declaration pip, the four
/// root-action rows in ascending domino index, whether the filed run
/// EXHIBITED a primal witness at this coordinate)`.
type FiledCoord = (usize, u8, [FiledRow; 4], bool);

/// FREEZE 51(a): the carrier is ARM 1 = h2 (pip 5, hand `[21 33 53 54]`),
/// both freeze-50 units, competitor `a = 53` then `a = 54`; ARM 2 = h9
/// (pip 4, hand `[30 41 54 61]`), units `a = 41` then `a = 54`, attempted
/// after arm 1 completes with a declared stop. h0, h6 and h12 are OUT OF
/// SCOPE for this build. Enumerated with NO GENERATING RULE (FT-A23: a freeze
/// is a constant, not a rule).
///
/// Quoted from `walt-factory/results/separation_n4_2026-08-14.txt`,
/// exploratory tier; carried as a frozen table in this source and NEVER
/// re-parsed from results text at run time. The `l_exhibited` flag is quoted
/// from the same file's per-coordinate `a*` lines: h2 carries a filed `R2`
/// primal receipt `L = Q^H`; h9 is NOT PRICED (517,562,322 partition states
/// against P_max v2) and exhibits no primal witness at all, so
/// `L_{a*} = Q^H(a*)` there is Corollary E4.1(2)'s CEILING and not a
/// receipted witness. That distinction is printed in place on every h9 row
/// (FT-A18(iv), RW-A3(iii): h9's NOT PRICED label stands verbatim).
const SR_FILED: [FiledCoord; 2] = [
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
        true,
    ),
    (
        9,
        4,
        [
            (
                3,
                0,
                (56_497_319, 19_958_400),
                (16_378_763, 5_702_400),
                833_630_064,
            ),
            (
                4,
                1,
                (28_422_259, 8_870_400),
                (545_341, 158_400),
                7_253_759_970,
            ),
            (
                5,
                4,
                (28_422_259, 8_870_400),
                (545_341, 158_400),
                7_253_759_970,
            ),
            (
                6,
                1,
                (31_039_087, 9_979_200),
                (31_207_009, 9_979_200),
                870_337_998,
            ),
        ],
        false,
    ),
];

/// One unit's rung-one filed record — the `FT_FIRST` table of (FT-R7a),
/// EXTENDED per SR-A22(v): `(corpus hand id, a hi, a lo, frontier states,
/// (state, world) arrivals, states with delta_I = 0, states with
/// delta_I > 0, Delta^(1), U^(1), Delta^(2), walk-steps charged by each FT
/// path, the (FT-R7c) frontier digest)`. The three rationals are count
/// convention, as `(num, den)`.
///
/// Quoted from `walt/CENSUS-RULINGS.md` (FT-A24(ii), the FT closing note, and
/// SR-A22(v)'s h2 reference list) and re-checked against the filed rows of
/// `walt-factory/results/fusion_tax_2026-08-14.txt`; exploratory tier.
/// Carried as a frozen table in this source and NEVER re-parsed from results
/// text (SEP-A14(ii)).
///
/// The last field is (FT-R7c)'s per-unit SHA-256 over the canonical
/// serialisation of the `(record, delta_I)` pairs in freeze-38(d) order.
/// FT-A28(iv) made it BINDING on the next run that regenerates a frontier and
/// no artifact discharges it; this is that run. An empty string means the
/// digest is OWED and is emitted here for transcription; a non-empty string
/// is asserted.
type SrFirstUnit = (
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
const SR_FIRST: [SrFirstUnit; 4] = [
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
        (1_483, 138_600),
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
        (1_483, 138_600),
        1_297_073_736,
        "de460262778878a72cb84cc4959f12a04f1652df4b1b7a846f3ea00a64d244fa",
    ),
    (
        9,
        4,
        1,
        1_320,
        2_217_600,
        24,
        1_296,
        (227_251, 3_326_400),
        (1_122_491, 332_640),
        (4_532_503, 26_611_200),
        7_253_759_970,
        "0d059121368ceeea7209166b150675a8b92d66b30da49209a0d33859c037514e",
    ),
    (
        9,
        5,
        4,
        1_320,
        2_217_600,
        24,
        1_296,
        (227_251, 3_326_400),
        (1_122_491, 332_640),
        (4_532_503, 26_611_200),
        7_253_759_970,
        "b196c789c227eee0e6e957cb720ee84c3e5db86dace719a6c5398c2bc820242c",
    ),
];

/// (SR-R9) frozen source at the declared grade-3 coordinate: the S6a filed
/// `Q^H` rows at base index 0, count convention, quoted from
/// `predictive_rank_2026-08-12.txt`, S6a, exploratory tier.
const G3_IDX0_QH: [(u8, u8, i128, i128); 3] =
    [(0, 0, 53, 21), (1, 0, 355, 168), (1, 1, 16_319, 6_720)];

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

/// The freeze-26 bridge at the reporting boundary, as a function of the
/// coordinate's DECLARED grade (N4-A11): `count = (diff + grade)/2`. No grade
/// literal appears in bridge code.
fn to_count(diff: Q, grade: usize) -> Q {
    (diff + qi(i128::try_from(grade).expect("grade fits"))) * q(1, 2)
}

/// Corollary SR-conv, the DIFFERENCE half: `s`, `d`, `delta`, `Delta_I^(2)`
/// and `Delta^(2)` are differences of two quantities at a common state, so
/// the additive `c*p_I` cancels exactly and a differential tax is exactly
/// TWICE its count-convention value.
fn tax_to_count(diff_tax: Q) -> Q {
    diff_tax * q(1, 2)
}

/// Corollary SR-conv, the VALUE half: `F^(1)_{I,b}`, `F^(2)_{I,b}`, `M_I`,
/// `C_{I,b,J}` and `A_{I,b,J,c}` are sums against masses totalling `p`, so an
/// affine reconvention adds `c*p` and scales the rest. The inverse of freeze
/// 26's bridge is therefore `(x_diff + grade*p)/2` and NOT `to_count`.
/// Quoting one of these columns through the difference bridge, or a tax
/// through this one, is VOID.
fn fval_to_count(diff: Q, grade: usize, p: Q) -> Q {
    (diff + qi(i128::try_from(grade).expect("grade fits")) * p) * q(1, 2)
}

// -- SHA-256 (FIPS 180-4), exact integer arithmetic, no dependency ----------

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

/// A streaming SHA-256 state. Streaming rather than one-shot so a per-unit
/// digest over a large canonical serialisation never materialises the whole
/// byte string; pure `u32` wrapping integer arithmetic (P-A19).
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
                // The residue did not fill a block, and `take` consumed all
                // of `data`, so there is nothing further to do. Returning
                // here is load-bearing: falling through would overwrite
                // `buf_len` with the (empty) remainder below and silently
                // discard the buffered bytes.
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
        // The padding length is computed, never searched for: a loop that
        // feeds zeros until the residue reaches 56 spins forever if `update`
        // mishandles its buffer, and a hang is a far worse failure than an
        // assertion.
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

/// The FIPS 180-4 known-answer self-check, run before any digest is reported.
/// A digest primitive that has never been checked against a published vector
/// is not a receipt of anything. The third vector crosses a block boundary,
/// which is exactly what the streaming path adds over the one-shot one.
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
    // A many-block known-answer vector, so the streaming path is anchored to
    // a published digest and not merely to this file's own one-shot path.
    let million: Vec<u8> = vec![b'a'; 1_000_000];
    assert_eq!(
        sha256_hex(&million),
        "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0",
        "SHA-256 known-answer self-check failed (FIPS 180-4, one million 'a')"
    );
    // The streaming path, fed in irregular chunks and one byte at a time,
    // must agree with the one-shot path on the same message. The first
    // version of this file buffered incorrectly across `update` calls, and
    // this is the check that has to catch it.
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

/// An observation record — the plays since the kernel decision point with the
/// root action first (freeze 26's observation contract, freeze 36(b)) — packed
/// into a `u128` as fixed 5-bit fields, most significant first, plus a length.
/// The record IS the information state; this is only its encoding.
///
/// The derived `Ord` compares `packed` then `len`, which is exactly freeze
/// 36(b)'s lexicographic order over the canonical ascending domino index, with
/// a proper prefix ordering BEFORE its extensions. That claim is not assumed:
/// `rec_order_self_check` verifies it against `Vec<Domino>`'s own ordering
/// over an enumerated family before any record is keyed.
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
            "stop-and-report: an observation record exceeded {REC_MAX} plays, which a grade-4 coordinate cannot produce"
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

    fn prefix(self, n: usize) -> Rec {
        assert!(n <= self.len as usize, "prefix length in range");
        let keep = 5 * (REC_MAX - n);
        let mask = if keep >= 128 {
            0
        } else {
            !((1u128 << keep) - 1)
        };
        Rec {
            packed: self.packed & mask,
            len: u8::try_from(n).expect("length fits"),
        }
    }

    fn tiles(self) -> Vec<Domino> {
        (0..self.len as usize).map(|i| self.at(i)).collect()
    }

    fn text(self) -> String {
        record_str(&self.tiles())
    }
}

/// The encoding's ordering claim, checked against `Vec<Domino>`'s own
/// ordering over every record of length 0..=3 drawn from a five-domino
/// alphabet that includes index 0 — the case a naive packing gets wrong,
/// since a proper prefix and an extension by domino `00` pack identically in
/// the value field and are separated only by the length.
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

// -- kernels -----------------------------------------------------------------

fn binom(n: u128, k: u128) -> u128 {
    if k > n {
        return 0;
    }
    let mut out: u128 = 1;
    for i in 0..k.min(n - k) {
        out = out * (n - i) / (i + 1);
    }
    out
}

fn unrank_comb(n: usize, k: usize, mut rank: u128) -> Vec<usize> {
    let mut out = Vec::with_capacity(k);
    let (mut x, mut k, mut n_left) = (0usize, k, n);
    while k > 0 {
        let c = binom((n_left - 1) as u128, (k - 1) as u128);
        if rank < c {
            out.push(x);
            k -= 1;
        } else {
            rank -= c;
        }
        x += 1;
        n_left -= 1;
    }
    out
}

/// The S6a-style reduced-grade coordinate at base index 0 (freezes 22-25's
/// unranking), used only by (SR-R9).
fn reduced_kernel(grade: usize) -> Kernel {
    let live_idx = unrank_comb(28, 4 * grade, 0);
    let hand_pos: std::collections::BTreeSet<usize> =
        unrank_comb(4 * grade, grade, 0).into_iter().collect();
    let (mut pool, mut hand) = (DominoSet::EMPTY, DominoSet::EMPTY);
    for (pos, di) in live_idx.iter().enumerate() {
        let d = Domino::from_index(*di).expect("domino");
        if hand_pos.contains(&pos) {
            hand.insert(d);
        } else {
            pool.insert(d);
        }
    }
    let hidden = [Seat::S1, Seat::S2, Seat::S3].map(|s| Hidden {
        seat: s,
        capacity: grade,
        voids: ContextSet::EMPTY,
    });
    Kernel::new(
        Decl::PipTrump(Pip::new(0).expect("pip")),
        Seat::S0,
        hand,
        pool,
        hidden,
    )
    .expect("the reduced-grade kernel is well formed")
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

// -- the rule algebra: one node, three walkers --------------------------------

/// A node of the observation tree in flight. `k` is the seat offset inside
/// the current trick, `k == 4` being the resolution node — exactly the node
/// structure of `walt_strat::info::walk`, so the walk-step charge of this
/// probe is comparable with the filed revealed subtotals.
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

/// The declaration, the focal seat and its team: everything the rule algebra
/// reads. A derived view of the kernel, never an authority over the fiber.
#[derive(Clone, Copy)]
struct Ctx {
    decl: Decl,
    focal: Seat,
    team: Team,
    /// The coordinate's DECLARED grade. The focal seat holds `grade` tiles and
    /// leads the root trick (freeze 45), so exactly `grade - 1` focal
    /// decisions follow the root action. No grade literal travels without
    /// this field (N4-A11).
    grade: usize,
}

impl Ctx {
    fn led(self, node: Node) -> Option<Context> {
        (node.k > 0).then(|| self.decl.led_context(node.tiles[0]))
    }

    fn legal_at(self, node: Node, seat: Seat) -> DominoSet {
        legal_plays(self.decl, node.hands[seat.index()], self.led(node))
    }

    /// The world-informed (revealed) continuation value of `node`, in the
    /// trick DIFFERENTIAL convention scaled by `SCALE`: the focal seat
    /// maximizes with the world known, the field is uniform over its own
    /// world-relative legal set. Budgeted (freeze 44(a)-(b)): one walk-step
    /// per node entry, charged before any child call; `None` on exhaustion
    /// with no partial fold retained.
    fn rev(self, node: Node, budget: &mut u64) -> Option<i64> {
        if *budget == 0 {
            return None;
        }
        *budget -= 1;
        if node.k == 4 {
            let trick =
                Trick::new(node.leader, node.tiles).expect("distinct tiles by construction");
            let winner = trick.winner(self.decl);
            let inc = if winner.team() == self.team {
                SCALE
            } else {
                -SCALE
            };
            if node.hands.iter().all(|h| h.is_empty()) {
                return Some(inc);
            }
            let next = Node {
                hands: node.hands,
                leader: winner,
                tiles: [Domino::ALL[0]; 4],
                k: 0,
            };
            return Some(inc + self.rev(next, budget)?);
        }
        let seat = node.seat();
        let legal = self.legal_at(node, seat);
        if seat == self.focal {
            let mut best = i64::MIN;
            for d in legal.iter() {
                let v = self.rev(node.child(seat, d), budget)?;
                if v > best {
                    best = v;
                }
            }
            assert!(best > i64::MIN, "a legal focal move exists at every node");
            return Some(best);
        }
        let n = i64::try_from(legal.len()).expect("a legal set size fits i64");
        let mut sum: i64 = 0;
        for d in legal.iter() {
            sum += self.rev(node.child(seat, d), budget)?;
        }
        let avg = sum / n;
        assert_eq!(
            avg * n,
            sum,
            "freeze 38(f): the scaled field average is exact — 12^12 carries every legal-set size"
        );
        Some(avg)
    }
}

// -- the independent replay: a record, read back as a position --------------

/// What replaying a record from the kernel decision point recovers. Computed
/// from the RECORD ALONE plus (decl, focal, the focal seat's root hand) — it
/// shares no bookkeeping with the walk, which is what makes (SR-R3) and the
/// action-set half of (SR-R8) contentful rather than circular.
struct Replay {
    /// The focal seat's remaining tiles after its plays inside the record.
    focal_hand: DominoSet,
    /// The led context of the incomplete trick, `None` at a trick boundary.
    led: Option<Context>,
    /// The seat to move after the record.
    seat_to_move: Seat,
    /// The positions, inside the record, of the focal seat's own plays.
    focal_plays: Vec<usize>,
    /// The scaled increment banked by the tricks the record COMPLETES.
    prefix: i64,
}

/// Lemma SR-coord(b), instantiated: the declaration is fixed and the record's
/// first entry is the root action (freeze 36(b)), so replaying the record
/// determines the seat to move at every ply — leaders are `Trick::winner`
/// applied to the record's completed tricks and the focal seat leads the root
/// trick at leader offset 0 (freeze 45). Hence the positions of the focal
/// seat's own plays inside the record are determined BY THE RECORD.
fn replay(ctx: Ctx, root_hand: DominoSet, rec: Rec) -> Replay {
    let mut leader = ctx.focal;
    let mut tiles = [Domino::ALL[0]; 4];
    let mut k = 0usize;
    let mut hand = root_hand;
    let mut focal_plays = Vec::new();
    let mut prefix: i64 = 0;
    for i in 0..rec.len as usize {
        let d = rec.at(i);
        let seat = leader.plus(k);
        if seat == ctx.focal {
            focal_plays.push(i);
            assert!(
                hand.remove(d),
                "stop-and-report: a record's focal play is not in the focal hand"
            );
        }
        tiles[k] = d;
        k += 1;
        if k == 4 {
            let trick = Trick::new(leader, tiles).expect("a record's trick has distinct tiles");
            let winner = trick.winner(ctx.decl);
            prefix += if winner.team() == ctx.team {
                SCALE
            } else {
                -SCALE
            };
            leader = winner;
            tiles = [Domino::ALL[0]; 4];
            k = 0;
        }
    }
    Replay {
        focal_hand: hand,
        led: (k > 0).then(|| ctx.decl.led_context(tiles[0])),
        seat_to_move: leader.plus(k),
        focal_plays,
        prefix,
    }
}

// -- PATH A2: the world-major two-frontier table ------------------------------

/// One FIRST-frontier information state's accumulated exact data — the rung
/// one object, reproduced here so (SR-R7) can compare it against the frozen
/// `FT_FIRST` table.
struct F1State {
    /// `A(I)`, asserted constant across the latent worlds of `I` (FT-A7(ii)).
    legal: DominoSet,
    /// The scaled increment banked before the first frontier. A function of
    /// the record alone, asserted equal on every arrival AND asserted against
    /// the independent replay.
    prefix: i64,
    /// `|X_I|`: latent worlds arriving here.
    n_worlds: u64,
    /// `sum_omega DEN_MU/den`, i.e. `p_I * DEN_MU * |X|`.
    acc_p: i128,
    /// `sum_omega (DEN_MU/den) * m_I(omega)`, scaled by `SCALE`.
    acc_m: i128,
    /// `sum_omega (DEN_MU/den) * q_I(omega,b)` per action of `legal` in
    /// ascending domino index (freeze 38 v1.1(d)), scaled by `SCALE`. This is
    /// exactly `F^(1)_{I,b}` unnormalised — SR-A22(ii)'s "the slack column is
    /// already computed and merely unprinted".
    acc_q: Vec<i128>,
    /// The running intersection of the COMPLETE per-world argmax sets.
    inter: DominoSet,
}

/// One SECOND-frontier information state's accumulated exact data. At grade 4
/// the focal seat holds two tiles here, so `|A(J)| <= 2` — asserted, and the
/// accumulators are sized by it (SR-A24(f): this is arithmetic, before any
/// measurement).
struct F2State {
    /// `|I|`, so `I = J[..parent_len]` and `b = J[parent_len]`. Carried by the
    /// walk and asserted against the independent replay (SR-R3).
    parent_len: u8,
    /// `A(J)`, asserted constant across `X_J`.
    legal: DominoSet,
    /// The scaled increment banked before the second frontier — the
    /// between-frontier increment is inside it (SR-A22(ii)(2)).
    prefix: i64,
    /// `|X_J|`.
    n_worlds: u64,
    /// `sum_omega DEN2/den`, i.e. `p_{I,b,J} * DEN2 * |X|`.
    acc_p: i128,
    /// `C_{I,b,J} = sum_omega mu_{I,b,J}(omega) m_{I,b,J}(omega)`,
    /// unnormalised over `DEN2 * SCALE * |X|`.
    acc_m: i128,
    /// `A_{I,b,J,c}` per action of `legal` in ascending domino index.
    acc_q: [i128; 2],
    /// The running intersection of the COMPLETE per-world argmax sets. Its
    /// emptiness is Corollary 5.2's criterion, computed from COMPLETE argmax
    /// sets and never from freeze 26's least-index tie rule (freeze 38(e),
    /// FT-A8, and §6.3's stronger rule).
    inter: DominoSet,
    /// The greedy strictly-shrinking chain of worlds: at most `|A(J)| <= 2`
    /// of them, and the search space of the minimal fusion core.
    chain: [(u32, DominoSet); 2],
    chain_len: u8,
}

impl F2State {
    fn observe(&mut self, wi: u32, mask: DominoSet) {
        let next = self.inter.intersection(mask);
        if next != self.inter {
            let n = self.chain_len as usize;
            assert!(
                n < 2,
                "SR-A24(f): |A(J)| <= 2 bounds the strictly shrinking chain at 2"
            );
            self.chain[n] = (wi, mask);
            self.chain_len += 1;
            self.inter = next;
        }
    }
}

/// The pre-frontier data a PATH A2 descent carries. SR-A22(ii)'s four changes
/// live in this struct and in how `walk` updates it.
#[derive(Clone, Copy)]
struct Arrival {
    /// The product of the field legal-set sizes so far (Lemma FT-arrive's
    /// product form). CHANGE (1): it accumulates while `depth < 2`, i.e.
    /// THROUGH the second frontier, and freezes only below it.
    den: u64,
    /// The same product frozen at the FIRST frontier, kept so the
    /// between-frontier increment is auditable in place.
    den1: u64,
    /// The increment banked so far. CHANGE (2): it likewise carries the
    /// between-frontier increment.
    prefix: i64,
    /// The banked increment frozen at the first frontier.
    prefix1: i64,
    /// CHANGE (3): a DEPTH COUNTER, not a bool — the number of focal
    /// decisions taken since the root action.
    depth: u8,
    /// The length of `I`'s record, valid once `depth >= 1`.
    i_len: u8,
}

struct Recorder<'a> {
    ctx: Ctx,
    wi: u32,
    f1: &'a mut BTreeMap<Rec, F1State>,
    f2: &'a mut BTreeMap<Rec, F2State>,
}

impl Recorder<'_> {
    /// The revealed walk, recording at the focal seat's FIRST decision below
    /// the root the complete per-action child values (`q_I(omega,b)` less the
    /// record's banked increment) and at its SECOND decision the complete
    /// per-action child values again (`q_{I,b,J}(omega,c)`). Returns the
    /// subtree value, so the whole descent still folds the per-world revealed
    /// value.
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
                // (SR-R2), and CHANGE (3) put to work: the focal seat makes
                // exactly `grade - 1` decisions below the root in EVERY
                // positive-mass world. `depth == grade - 1` at the terminal is
                // simultaneously `T_0 = 0` (a second frontier is always
                // reached) and `Theta_{I,b} = 0` (no early termination after
                // `b`). Both are contentful at grade 4 and both fail loudly if
                // a world ever ends before the focal seat has acted twice.
                assert_eq!(
                    usize::from(arr.depth),
                    self.ctx.grade - 1,
                    "(SR-R2) stop-and-report: T_0 = 0 or Theta_(I,b) = 0 fails — a positive-mass world reached the end of the hand after {} focal decisions, not {}",
                    arr.depth,
                    self.ctx.grade - 1
                );
                return Some(inc);
            }
            let below = Arrival {
                prefix: if arr.depth >= 2 {
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
            return match arr.depth {
                0 => self.at_frontier_one(node, arr, rec, legal, budget),
                1 => self.at_frontier_two(node, arr, rec, legal, budget),
                _ => {
                    let below = Arrival {
                        depth: arr.depth + 1,
                        ..arr
                    };
                    let mut best = i64::MIN;
                    for d in legal.iter() {
                        let v = self.walk(node.child(seat, d), below, rec.push(d), budget)?;
                        if v > best {
                            best = v;
                        }
                    }
                    assert!(best > i64::MIN, "a legal focal move exists at every node");
                    Some(best)
                }
            };
        }
        let n = i64::try_from(legal.len()).expect("a legal set size fits i64");
        assert!(
            legal.len() <= N4_GRADE,
            "freeze 38(f): a field legal set at these grades has size at most 4, which is why 12^12 divides every arrival denominator"
        );
        let below = Arrival {
            // CHANGE (1): accumulate while `depth < 2`, i.e. through the
            // second frontier, instead of freezing at the first.
            den: if arr.depth >= 2 {
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

    /// The FIRST frontier: the focal seat's next decision after the root.
    fn at_frontier_one(
        &mut self,
        node: Node,
        arr: Arrival,
        rec: Rec,
        legal: DominoSet,
        budget: &mut u64,
    ) -> Option<i64> {
        let seat = self.ctx.focal;
        assert!(
            legal.len() < self.ctx.grade,
            "|A(I)| is bounded by the grade - 1 tiles in hand at the first frontier"
        );
        let below = Arrival {
            depth: 1,
            i_len: rec.len,
            den1: arr.den,
            prefix1: arr.prefix,
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
        let entry = self.f1.entry(rec).or_insert_with(|| F1State {
            legal,
            prefix: arr.prefix,
            n_worlds: 0,
            acc_p: 0,
            acc_m: 0,
            acc_q: vec![0; legal.len()],
            inter: legal,
        });
        assert_eq!(
            entry.legal, legal,
            "(FT-A7(ii)) stop-and-report: A(I) is not common across X_I"
        );
        assert_eq!(
            entry.prefix, arr.prefix,
            "stop-and-report: the pre-frontier increment is not a function of the record"
        );
        let w = DEN_MU / i128::from(arr.den);
        assert_eq!(
            w * i128::from(arr.den),
            DEN_MU,
            "SR-A22(ii)(4): DEN_MU = 12^6 carries every PRE-FRONTIER-1 arrival denominator"
        );
        entry.n_worlds += 1;
        entry.acc_p += w;
        entry.acc_m += w * i128::from(arr.prefix + best);
        let mut mask = DominoSet::EMPTY;
        for (j, d) in legal.iter().enumerate() {
            entry.acc_q[j] += w * i128::from(arr.prefix + child[j]);
            if child[j] == best {
                mask.insert(d);
            }
        }
        entry.inter = entry.inter.intersection(mask);
        Some(best)
    }

    /// The SECOND frontier, per freeze 51(c): the focal seat's next decision
    /// after `b`, FORCED OR NOT. A forced `J` is a frontier state with
    /// `|A(J)| = 1` and `delta_{I,b,J} = 0`, and it is COUNTED, NOT SKIPPED.
    fn at_frontier_two(
        &mut self,
        node: Node,
        arr: Arrival,
        rec: Rec,
        legal: DominoSet,
        budget: &mut u64,
    ) -> Option<i64> {
        let seat = self.ctx.focal;
        assert!(
            legal.len() <= self.ctx.grade - 2 && !legal.is_empty(),
            "SR-A24(f): at the second frontier the focal seat holds grade-2 tiles, so |A(J)| <= 2 at grade 4 — by arithmetic, before any measurement"
        );
        let below = Arrival { depth: 2, ..arr };
        let mut child = [0i64; 2];
        let mut best = i64::MIN;
        for (j, d) in legal.iter().enumerate() {
            let v = self.walk(node.child(seat, d), below, rec.push(d), budget)?;
            child[j] = v;
            if v > best {
                best = v;
            }
        }
        // CHANGE (2), asserted rather than assumed: exactly ONE trick
        // completes between the focal seat's decision at the first frontier
        // and its decision at the second, so the between-frontier increment
        // is exactly one trick's worth. The same holds between the root
        // action and the first frontier.
        assert_eq!(
            arr.prefix1.abs(),
            SCALE,
            "SR-A22(ii)(2) stop-and-report: the pre-frontier-1 increment is not exactly one trick"
        );
        assert_eq!(
            (arr.prefix - arr.prefix1).abs(),
            SCALE,
            "SR-A22(ii)(2) stop-and-report: the between-frontier increment did not accumulate through the second frontier"
        );
        // CHANGE (1), asserted rather than assumed: the arrival denominator
        // strictly extends the one frozen at the first frontier.
        assert!(
            arr.den.is_multiple_of(arr.den1) && arr.den >= arr.den1,
            "SR-A22(ii)(1) stop-and-report: the arrival denominator did not accumulate through the second frontier"
        );
        let entry = self.f2.entry(rec).or_insert_with(|| F2State {
            parent_len: arr.i_len,
            legal,
            prefix: arr.prefix,
            n_worlds: 0,
            acc_p: 0,
            acc_m: 0,
            acc_q: [0i128; 2],
            inter: legal,
            chain: [(0u32, DominoSet::EMPTY); 2],
            chain_len: 0,
        });
        assert_eq!(
            entry.legal, legal,
            "(FT-A7(ii)) stop-and-report: A(J) is not common across X_J"
        );
        assert_eq!(
            entry.prefix, arr.prefix,
            "stop-and-report: the pre-frontier-2 increment is not a function of the record"
        );
        // Lemma SR-coord(b) as an in-run assertion: a second-frontier record
        // determines `(I,b)` uniquely, so two arrivals at one `J` may never
        // disagree about their parent. This IS the disjointness half of
        // (SR-R3) — `I_2(I,b)` and `I_2(I',b')` are disjoint for distinct
        // branches.
        assert_eq!(
            entry.parent_len, arr.i_len,
            "(SR-R3) stop-and-report: one second-frontier record carries two different parents — Lemma SR-coord(b) fails in this engine"
        );
        // CHANGE (4), asserted rather than assumed.
        let w = DEN2 / i128::from(arr.den);
        assert_eq!(
            w * i128::from(arr.den),
            DEN2,
            "SR-A22(ii)(4): the depth-two common denominator 12^12 = SCALE carries every second-frontier arrival denominator"
        );
        entry.n_worlds += 1;
        entry.acc_p += w;
        entry.acc_m += w * i128::from(arr.prefix + best);
        let mut mask = DominoSet::EMPTY;
        for (j, d) in legal.iter().enumerate() {
            entry.acc_q[j] += w * i128::from(arr.prefix + child[j]);
            if child[j] == best {
                mask.insert(d);
            }
        }
        entry.observe(self.wi, mask);
        Some(best)
    }
}

/// PATH A2's result at one (coordinate, root action).
struct PathA2 {
    f1: BTreeMap<Rec, F1State>,
    f2: BTreeMap<Rec, F2State>,
    /// `sum_omega V_revealed(omega, a)` scaled by `SCALE`.
    world_fold: i128,
    steps: u64,
    residual: u64,
}

fn path_a2(ctx: Ctx, worlds: &[[DominoSet; Seat::COUNT]], root: Domino) -> Option<PathA2> {
    let mut f1: BTreeMap<Rec, F1State> = BTreeMap::new();
    let mut f2: BTreeMap<Rec, F2State> = BTreeMap::new();
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
            den1: 1,
            prefix: 0,
            prefix1: 0,
            depth: 0,
            i_len: 0,
        };
        let rec = Rec::default().push(root);
        let mut r = Recorder {
            ctx,
            wi: u32::try_from(wi).expect("fiber index fits u32"),
            f1: &mut f1,
            f2: &mut f2,
        };
        world_fold += i128::from(r.walk(node, arr, rec, &mut budget)?);
    }
    Some(PathA2 {
        f1,
        f2,
        world_fold,
        steps: B_WALK - budget,
        residual: budget,
    })
}

// -- PATH B2: the glue-two-then-reveal walker (SR-R6) ------------------------

/// One pooled particle of the glue-two walk: a fiber world, its arrival
/// denominator, and its remaining hands. The weight the field assigns is
/// `1/den` (Lemma FT-arrive's product form, carried through the SECOND
/// frontier here) and the belief factor `1/|X|` is applied once at the end.
#[derive(Clone, Copy)]
struct Pooled {
    den: u64,
    hands: [DominoSet; Seat::COUNT],
}

struct Glue2 {
    ctx: Ctx,
}

impl Glue2 {
    /// The `C^(2)` value below a pooled node: LAWFUL at the focal seat's
    /// first decision below the root (one common `b` per information state,
    /// `max_b` OUTSIDE the world sum), LAWFUL AGAIN at its second decision
    /// (one common `c` per second-frontier information state, `max_c` outside
    /// the world sum), and world-informed below that. Returns
    /// `sum (DEN2/den) * (prefix + q)` scaled by `SCALE`, i.e. `U^(2)` less
    /// the belief factor `1/|X|`.
    ///
    /// Written against the same rule algebra and NOTHING ELSE: it shares no
    /// map, no accumulator, no arrival struct and no recursion with PATH A2.
    /// This is (FT-R4) one rung up, and it is the receipt that would catch a
    /// wrong `max`/`sum` order.
    ///
    /// Budgeted (freeze 44(a)-(b)): the charge at entry is the pooled bag's
    /// `bag.len()`, taken before any child call; the revealed continuations
    /// below the second frontier charge one step per node.
    #[allow(clippy::too_many_arguments)]
    fn walk(
        &self,
        support: &[Pooled],
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        prefix: i64,
        depth: u8,
        budget: &mut u64,
    ) -> Option<i128> {
        let cost = u64::try_from(support.len()).expect("a pooled bag fits u64");
        if *budget < cost {
            return None;
        }
        *budget -= cost;
        if k == 4 {
            let trick = Trick::new(leader, tiles).expect("distinct tiles by construction");
            let winner = trick.winner(self.ctx.decl);
            let inc = if winner.team() == self.ctx.team {
                SCALE
            } else {
                -SCALE
            };
            assert!(
                !support[0].hands.iter().all(|h| h.is_empty()),
                "(SR-R2) stop-and-report: the hand ended before the focal seat had acted twice"
            );
            return self.walk(
                support,
                winner,
                [Domino::ALL[0]; 4],
                0,
                prefix + inc,
                depth,
                budget,
            );
        }
        let seat = leader.plus(k);
        let led: Option<Context> = (k > 0).then(|| self.ctx.decl.led_context(tiles[0]));
        if seat == self.ctx.focal {
            let hand = support[0].hands[seat.index()];
            for p in support {
                assert_eq!(
                    p.hands[seat.index()],
                    hand,
                    "the focal hand is observable, hence constant on the pooled bag"
                );
            }
            let legal = legal_plays(self.ctx.decl, hand, led);
            if depth == 0 {
                // The FIRST frontier: one common `b`, `max_b` outside the
                // world sum, and the bag carried on lawfully below it.
                let mut best: Option<i128> = None;
                for d in legal.iter() {
                    let mut sup: Vec<Pooled> = Vec::with_capacity(support.len());
                    for p in support {
                        let mut hands = p.hands;
                        hands[seat.index()].remove(d);
                        sup.push(Pooled { den: p.den, hands });
                    }
                    let mut t = tiles;
                    t[k] = d;
                    let v = self.walk(&sup, leader, t, k + 1, prefix, 1, budget)?;
                    best = Some(best.map_or(v, |b: i128| b.max(v)));
                }
                return Some(best.expect("A(I) is nonempty"));
            }
            // The SECOND frontier: one common `c`, `max_c` outside the world
            // sum, world-informed below.
            let mut best: Option<i128> = None;
            for d in legal.iter() {
                let mut acc: i128 = 0;
                for p in support {
                    let mut hands = p.hands;
                    hands[seat.index()].remove(d);
                    let mut t = tiles;
                    t[k] = d;
                    let node = Node {
                        hands,
                        leader,
                        tiles: t,
                        k: k + 1,
                    };
                    let v = self.ctx.rev(node, budget)?;
                    let w = DEN2 / i128::from(p.den);
                    assert_eq!(
                        w * i128::from(p.den),
                        DEN2,
                        "SR-A22(ii)(4): 12^12 carries every second-frontier arrival denominator"
                    );
                    acc += w * i128::from(prefix + v);
                }
                best = Some(best.map_or(acc, |b: i128| b.max(acc)));
            }
            return Some(best.expect("A(J) is nonempty"));
        }
        let mut by_tile: BTreeMap<usize, Vec<Pooled>> = BTreeMap::new();
        for p in support {
            let legal = legal_plays(self.ctx.decl, p.hands[seat.index()], led);
            let n = u64::try_from(legal.len()).expect("a legal set size fits u64");
            for d in legal.iter() {
                let mut hands = p.hands;
                hands[seat.index()].remove(d);
                by_tile.entry(d.index()).or_default().push(Pooled {
                    den: p.den * n,
                    hands,
                });
            }
        }
        let mut sum: i128 = 0;
        for (ti, sup) in by_tile {
            let d = Domino::from_index(ti).expect("tile index");
            let mut t = tiles;
            t[k] = d;
            sum += self.walk(&sup, leader, t, k + 1, prefix, depth, budget)?;
        }
        Some(sum)
    }
}

struct PathB2 {
    total: i128,
    steps: u64,
    residual: u64,
}

fn path_b2(ctx: Ctx, worlds: &[[DominoSet; Seat::COUNT]], root: Domino) -> Option<PathB2> {
    let mut support: Vec<Pooled> = Vec::with_capacity(worlds.len());
    for hands in worlds {
        let mut hands = *hands;
        hands[ctx.focal.index()].remove(root);
        support.push(Pooled { den: 1, hands });
    }
    let mut tiles = [Domino::ALL[0]; 4];
    tiles[0] = root;
    let mut budget = B_WALK;
    let glue = Glue2 { ctx };
    let total = glue.walk(&support, ctx.focal, tiles, 1, 0, 0, &mut budget)?;
    Some(PathB2 {
        total,
        steps: B_WALK - budget,
        residual: budget,
    })
}

// -- one unit: a (coordinate, binding competitor action) --------------------

/// One `(I,b)` branch's depth-two aggregate. Every field is an exact integer
/// over a declared common denominator; nothing here is a bound, an estimate
/// or a sample.
struct Branch {
    /// The index of `b` inside `A(I)` in ascending domino index — the column
    /// of `F^(1)_{I,b}` inside `F1State::acc_q`.
    slot: usize,
    /// `sum_J C_{I,b,J}`, over `DEN2 * SCALE * |X|`.
    sum_c: i128,
    /// `sum_J max_c A_{I,b,J,c}`, over `DEN2 * SCALE * |X|`.
    sum_max: i128,
    /// `sum_J p_{I,b,J}`, over `DEN2 * |X|`.
    sum_p: i128,
    n_j: u64,
    n_j_pos: u64,
    n_j_forced: u64,
}

/// The declared deterministic (SR-R8) sample: the first ten `(I,b,J)` triples
/// in freeze-38 v1.1(d) order. The assertions themselves are made at EVERY
/// triple, which is strictly stronger than the declared sample; the sample is
/// what is printed.
const R8_SAMPLE: usize = 10;

/// Everything one unit produces.
struct SrUnit {
    // -- rung one, for (SR-R7) --
    n_states1: u64,
    n_arrivals1: u64,
    zero1: u64,
    pos1: u64,
    mass: Q,
    u0_diff: Q,
    u1_diff: Q,
    delta1_diff: Q,
    /// (FT-R7c): SHA-256 over the canonical serialisation of the
    /// `(record, delta_I)` pairs in freeze-38(d) order.
    frontier_digest: String,
    // -- rung two --
    n_states2: u64,
    n_arrivals2: u64,
    n_branches: u64,
    sum_legal1: u64,
    n_j_pos: u64,
    n_j_forced: u64,
    n_escape: u64,
    u2_diff: Q,
    delta2_diff: Q,
    u2_glue_diff: Q,
    /// The `(I,b)` branch rows and the per-`I` state rows of SR-A22(iii)(d),
    /// in freeze-38 v1.1(d) order; the committed file's content.
    branch_rows: Vec<String>,
    state_rows: Vec<String>,
    /// `|A(I)|` per state row, in the same order: how many branch rows belong
    /// to each state row when the declared cap has not fired.
    branch_counts: Vec<usize>,
    printed_branch: u64,
    branch_tail: Option<u64>,
    /// The per-`(I,b,J)` rows; the companion's content — REGENERABLE and NOT
    /// COMMITTED, carried into the committed header only by its digest. Held
    /// only by the emitting pass: the (SR-R10) second pass compares the
    /// STREAMING DIGEST of the same row sequence, which is at least as strong
    /// a comparison and does not hold a second copy of a table this large.
    companion: Vec<String>,
    companion_digest: String,
    r8_sample: Vec<String>,
    /// The complete escape census: every `I` whose `argmin_b(s+d)` is
    /// disjoint from `B*_I`, with both complete sets printed.
    escape_rows: Vec<String>,
    steps_a: u64,
    residual_a: u64,
    steps_b: u64,
    residual_b: u64,
}

#[allow(clippy::too_many_lines)]
fn run_unit(kernel: &Kernel, grade: usize, root: Domino, keep_rows: bool) -> Option<SrUnit> {
    assert_eq!(
        kernel.viewer_hand().len(),
        grade,
        "the declared grade is the coordinate's grade (N4-A11: no grade literal travels without this check)"
    );
    assert_eq!(
        DEN2,
        i128::from(SCALE),
        "SR-A22(ii)(4): the depth-two common denominator is 12^12, which is SCALE"
    );
    assert_eq!(
        DEN_MU * DEN_MU,
        DEN2,
        "SR-A22(ii)(4): 12^6 squared is 12^12 — the rung-one table rescales into the rung-two denominator by exactly DEN_MU"
    );
    let root_hand = kernel.viewer_hand();
    let worlds: Vec<[DominoSet; Seat::COUNT]> = kernel.worlds().map(|w| w.hands()).collect();
    let n_worlds = i128::try_from(worlds.len()).expect("fiber size fits i128");
    let ctx = Ctx {
        decl: kernel.decl(),
        focal: kernel.viewer(),
        team: kernel.viewer().team(),
        grade,
    };

    let a = path_a2(ctx, &worlds, root)?;
    let b = path_b2(ctx, &worlds, root)?;

    let n_states1 = u64::try_from(a.f1.len()).expect("state count fits u64");
    let n_states2 = u64::try_from(a.f2.len()).expect("state count fits u64");
    // Freeze 44(b) v2, SR-A22(iv): the second-frontier partition count is
    // asserted against P_max v2 BEFORE the aggregate pass, and the assertion
    // is contentful. Every J has a unique parent (Lemma SR-coord(b)), so the
    // map's size IS `sum_{I,b} |I_2(I,b)|`.
    assert!(
        n_states2 <= P_MAX,
        "freeze 44(b) v2: sum_{{I,b}} |I_2(I,b)| = {n_states2} exceeds P_max v2 = {P_MAX}"
    );
    assert!(
        n_states1 <= P_MAX,
        "freeze 44(b) v2: the depth-one frontier partition exceeds P_max v2"
    );

    let norm1 = DEN_MU * i128::from(SCALE) * n_worlds;
    let norm2 = DEN2 * i128::from(SCALE) * n_worlds;
    let pnorm1 = DEN_MU * n_worlds;
    let pnorm2 = DEN2 * n_worlds;

    // ---- rung one, reproduced for (SR-R7) ---------------------------------
    let mut sum_p1: i128 = 0;
    let mut sum_m1: i128 = 0;
    let mut sum_best1: i128 = 0;
    let mut n_arrivals1: u64 = 0;
    let mut zero1: u64 = 0;
    let mut pos1: u64 = 0;
    let mut digest = Sha256::new();
    // Every `(I, b)` branch, initialised from the FIRST-frontier table so a
    // branch with no recorded second frontier would be caught by the `n_j`
    // assertion below rather than silently absent.
    let mut branches: BTreeMap<(Rec, Domino), Branch> = BTreeMap::new();
    for (rec, st) in &a.f1 {
        // Lemma SR-coord instantiated at the FIRST frontier, from the record
        // alone: the seat to move, the remaining focal hand, the led context
        // and the banked increment are all functions of the record.
        let rp = replay(ctx, root_hand, *rec);
        assert_eq!(
            rp.seat_to_move, ctx.focal,
            "(SR-R3) stop-and-report: a first-frontier record does not replay to a focal decision"
        );
        assert_eq!(
            rp.focal_plays,
            vec![0usize],
            "(SR-R3) stop-and-report: a first-frontier record contains focal plays other than the root action"
        );
        assert_eq!(
            rp.prefix, st.prefix,
            "(SR-R3) stop-and-report: the record's replayed increment differs from the walk's banked prefix at I"
        );
        assert_eq!(
            legal_plays(ctx.decl, rp.focal_hand, rp.led),
            st.legal,
            "(SR-R8) stop-and-report: |A(I)| disagrees with legal_plays at the reconstructed position"
        );
        let best = *st.acc_q.iter().max().expect("A(I) is nonempty");
        let dtax = st.acc_m - best;
        assert!(
            dtax >= 0,
            "stop-and-report: a rung-one local tax is negative (avg-of-max is below max-of-avg)"
        );
        assert_eq!(
            dtax == 0,
            !st.inter.is_empty(),
            "(SR-R7) stop-and-report: Corollary 5.2 fails at rung one, record [{}]",
            rec.text()
        );
        if dtax == 0 {
            zero1 += 1;
        } else {
            pos1 += 1;
        }
        sum_p1 += st.acc_p;
        sum_m1 += st.acc_m;
        sum_best1 += best;
        n_arrivals1 += st.n_worlds;
        // (FT-R7c)'s canonical serialisation, in freeze-38(d) order:
        // `<record>|<delta_I as num/den, count convention>` and a newline.
        digest.update(rec.text().as_bytes());
        digest.update(b"|");
        digest.update(qs(tax_to_count(Q::new(dtax, norm1))).as_bytes());
        digest.update(b"\n");
        for (slot, d) in st.legal.iter().enumerate() {
            branches.insert(
                (*rec, d),
                Branch {
                    slot,
                    sum_c: 0,
                    sum_max: 0,
                    sum_p: 0,
                    n_j: 0,
                    n_j_pos: 0,
                    n_j_forced: 0,
                },
            );
        }
    }
    let mass = Q::new(sum_p1, pnorm1);
    assert_eq!(
        sum_m1,
        DEN_MU * a.world_fold,
        "stop-and-report: the frontier accumulation and the per-world revealed fold disagree"
    );
    let u0_diff = Q::new(sum_m1, norm1);
    let u1_diff = Q::new(sum_best1, norm1);
    let delta1_diff = Q::new(sum_m1 - sum_best1, norm1);
    let frontier_digest = digest.finish();

    // ---- rung two: the per-(I,b,J) pass ------------------------------------
    let mut n_arrivals2: u64 = 0;
    let mut companion: Vec<String> = Vec::with_capacity(if keep_rows { a.f2.len() } else { 0 });
    let mut cdigest = Sha256::new();
    let mut r8_sample: Vec<String> = Vec::new();
    for (rec, st) in &a.f2 {
        let parent_len = usize::from(st.parent_len);
        // (SR-R3), the parent receipt: replaying J's record recovers (I,b)
        // exactly — Lemma SR-coord(b) instantiated, computed from the record
        // and never from the walk's carried attribution.
        let rp = replay(ctx, root_hand, *rec);
        assert_eq!(
            rp.seat_to_move, ctx.focal,
            "(SR-R3) stop-and-report: a second-frontier record does not replay to a focal decision"
        );
        assert_eq!(
            rp.focal_plays,
            vec![0usize, parent_len],
            "(SR-R3) stop-and-report: replaying [{}] does not put the focal seat's second play at position {parent_len} — the carried parent attribution and the record disagree",
            rec.text()
        );
        assert_eq!(
            rp.prefix, st.prefix,
            "(SR-R3) stop-and-report: the record's replayed increment differs from the walk's banked prefix at J"
        );
        assert_eq!(
            legal_plays(ctx.decl, rp.focal_hand, rp.led),
            st.legal,
            "(SR-R8) stop-and-report: |A(J)| disagrees with legal_plays at the reconstructed position"
        );
        let i_rec = rec.prefix(parent_len);
        let b_tile = rec.at(parent_len);
        let best = st.acc_q[..st.legal.len()]
            .iter()
            .copied()
            .max()
            .expect("A(J) is nonempty");
        let dtax = st.acc_m - best;
        assert!(
            dtax >= 0,
            "stop-and-report: a second-frontier local tax is negative"
        );
        // Corollary 5.2, both directions, at EVERY (I,b,J) — strictly
        // stronger than (SR-R8)'s declared sample; from COMPLETE argmax sets
        // and never from freeze 26's least-index tie rule.
        let common = !st.inter.is_empty();
        assert_eq!(
            dtax == 0,
            common,
            "(SR-R8) stop-and-report: Corollary 5.2 fails at record [{}]",
            rec.text()
        );
        let entry = branches
            .get_mut(&(i_rec, b_tile))
            .expect("(SR-R3) every J's parent branch exists in the first-frontier table");
        entry.sum_c += st.acc_m;
        entry.sum_max += best;
        entry.sum_p += st.acc_p;
        entry.n_j += 1;
        n_arrivals2 += st.n_worlds;
        if dtax > 0 {
            entry.n_j_pos += 1;
        }
        if st.legal.len() == 1 {
            entry.n_j_forced += 1;
        }

        let p_j = Q::new(st.acc_p, pnorm2);
        let c_count = fval_to_count(Q::new(st.acc_m, norm2), grade, p_j);
        let delta_count = tax_to_count(Q::new(dtax, norm2));
        let mut argmax = DominoSet::EMPTY;
        let mut acols = String::new();
        for (j, d) in st.legal.iter().enumerate() {
            if st.acc_q[j] == best {
                argmax.insert(d);
            }
            let _ = write!(
                acols,
                "  A[c={}] = {}",
                tile(d),
                qs(fval_to_count(Q::new(st.acc_q[j], norm2), grade, p_j))
            );
        }
        let core_txt = if dtax > 0 {
            assert_eq!(
                st.legal.len(),
                2,
                "SR-A24(f): a positive delta at the second frontier forces |A(J)| = 2"
            );
            assert_eq!(
                st.chain_len, 2,
                "SR-A24(f): every positive-delta minimal core has size exactly 2, by arithmetic"
            );
            let c0 = st.chain[0];
            let c1 = st.chain[1];
            assert!(
                c0.1.intersection(c1.1).is_empty() && !c0.1.is_empty() && !c1.1.is_empty(),
                "(SR-R8) stop-and-report: the reported minimal fusion core is not minimal or has a common optimum"
            );
            format!(
                "  minimal fusion core (fiber indices, freeze 7/23) = [{}:{{{}}} {}:{{{}}}] size 2",
                c0.0,
                tiles_str(c0.1),
                c1.0,
                tiles_str(c1.1)
            )
        } else {
            String::new()
        };
        let row = format!(
            "    J=[{}]  parent I=[{}] b={}  p_J = {}  |X_J| = {}  |A(J)| = {}  C = {} (count){}  delta = {} (count)  argmax_c = {{{}}}{}",
            rec.text(),
            i_rec.text(),
            tile(b_tile),
            qs(p_j),
            st.n_worlds,
            st.legal.len(),
            qs(c_count),
            acols,
            qs(delta_count),
            tiles_str(argmax),
            core_txt
        );
        cdigest.update(row.as_bytes());
        cdigest.update(b"\n");
        if r8_sample.len() < R8_SAMPLE {
            let verdict = if dtax == 0 {
                format!(
                    "delta = 0 and the complete argmax_c sets intersect at {{{}}} — Corollary 5.2 HELD; |A(I)| and |A(J)| asserted against legal_plays at the reconstructed position",
                    tiles_str(st.inter)
                )
            } else {
                "delta > 0, empty argmax_c intersection, minimal fusion core of size exactly 2 — Corollary 5.2 HELD. SR-A24(f), A PRIORI: at grade 4 the focal seat holds two tiles at the second frontier, so |A(J)| <= 2 and every positive-delta minimal core has size exactly 2 BY ARITHMETIC, before any measurement. This row measures nothing about core size and the received note's open ledger row is UNMEASURABLE at this carrier".to_string()
            };
            r8_sample.push(format!(
                "    (I=[{}], b={}, J=[{}]): {verdict}",
                i_rec.text(),
                tile(b_tile),
                rec.text()
            ));
        }
        if keep_rows {
            companion.push(row);
        }
    }
    let companion_digest = cdigest.finish();

    // ---- rung two: the per-(I,b) and per-I pass ---------------------------
    let mut branch_rows: Vec<String> = Vec::new();
    let mut state_rows: Vec<String> = Vec::new();
    let mut branch_counts: Vec<usize> = Vec::new();
    let mut escape_rows: Vec<String> = Vec::new();
    let mut sum_u2: i128 = 0;
    let mut sum_delta2: i128 = 0;
    let mut sum_legal1: u64 = 0;
    let mut n_j_pos: u64 = 0;
    let mut n_j_forced: u64 = 0;
    let mut n_escape: u64 = 0;
    let mut n_branches: u64 = 0;
    let mut branch_sortkey: Vec<i128> = Vec::new();
    for (rec, st) in &a.f1 {
        let p_i = Q::new(st.acc_p, pnorm1);
        sum_legal1 += u64::try_from(st.legal.len()).expect("fits");
        // `F^(1)_{I,b}` rescaled from the rung-one denominator into the
        // rung-two one: `norm2 = norm1 * DEN_MU`, asserted above.
        let f1: Vec<i128> = st.acc_q.iter().map(|x| x * DEN_MU).collect();
        let m_i = *f1.iter().max().expect("A(I) is nonempty");
        let mut f2: Vec<i128> = Vec::with_capacity(st.legal.len());
        let mut sd: Vec<i128> = Vec::with_capacity(st.legal.len());
        for (slot, d) in st.legal.iter().enumerate() {
            let br = branches
                .get(&(*rec, d))
                .expect("every (I,b) branch was initialised from the first-frontier table");
            assert_eq!(br.slot, slot, "branch slot bookkeeping");
            assert!(
                br.n_j > 0,
                "(SR-R2) stop-and-report: branch (I=[{}], b={}) reached no second frontier — Theta_(I,b) is not zero",
                rec.text(),
                tile(d)
            );
            // (SR-R1) THE BRANCH RECONSTRUCTION RECEIPT. The left side is the
            // depth-two table's `Theta + sum_J C_{I,b,J}` with `Theta = 0`;
            // the right side is the rung-one branch value
            // `sum_omega mu_I(omega) q_I(omega,b)` from `FrontierState::acc_q`,
            // computed by a different pass over different intermediate
            // quantities. It fails on any error in second-frontier detection,
            // in the between-frontier arrival weights, in the J keying, or in
            // the parent attribution.
            assert_eq!(
                br.sum_c, f1[slot],
                "(SR-R1) stop-and-report: Theta + sum_J C_(I,b,J) != F^(1)_(I,b) at record [{}], b = {}. Nothing is claimed; this is a bug in the probe or in Theorem 4.1's hypotheses, never a finding about the game (R-A18, NO-RESCUE)",
                rec.text(),
                tile(d)
            );
            // (SR-R2) THE MASS RECEIPT, per branch: `sum_J p_{I,b,J} +
            // p^term_{I,b} = p_I` with `p^term = 0`. It is the only check
            // that the J list is exhaustive; a dropped J lowers C, A, F^(1)
            // and F^(2) together and understates Delta^(2) with every
            // algebraic assertion still green.
            assert_eq!(
                br.sum_p,
                st.acc_p * DEN_MU,
                "(SR-R2) stop-and-report: sum_J p_(I,b,J) + p^term != p_I at record [{}], b = {}",
                rec.text(),
                tile(d)
            );
            let f2b = br.sum_max;
            let s = m_i - f1[slot];
            let dd = br.sum_c - br.sum_max;
            f2.push(f2b);
            sd.push(s + dd);
            n_j_pos += br.n_j_pos;
            n_j_forced += br.n_j_forced;
            n_branches += 1;
            // ARITHMETIC REMARKS, named in place per Proposition SR-taut —
            // they cannot fail and are never counted among receipts HELD.
            assert!(s >= 0 && dd >= 0, "arithmetic remark: s, d >= 0");
        }
        let best_f2 = *f2.iter().max().expect("A(I) is nonempty");
        let min_sd = *sd.iter().min().expect("A(I) is nonempty");
        // ARITHMETIC REMARK (Proposition SR-taut (2)): it cannot fail.
        assert_eq!(
            m_i - best_f2,
            min_sd,
            "arithmetic remark: max_b F^(1) - max_b F^(2) = min_b (s + d)"
        );
        sum_u2 += best_f2;
        sum_delta2 += min_sd;

        let mut bstar = DominoSet::EMPTY;
        let mut argmin = DominoSet::EMPTY;
        for (slot, d) in st.legal.iter().enumerate() {
            if f1[slot] == m_i {
                bstar.insert(d);
            }
            if sd[slot] == min_sd {
                argmin.insert(d);
            }
        }
        // Corollary 5.2 at rung one, the containment direction: an action
        // optimal in EVERY world maximises the weighted sum, so the running
        // argmax intersection sits inside the complete optimal face. Both are
        // computed from COMPLETE sets; neither is tie-broken.
        assert_eq!(
            bstar,
            st.inter.union(bstar),
            "stop-and-report: the rung-one argmax intersection is not contained in the complete optimal face B*_I"
        );
        let escape = bstar.intersection(argmin).is_empty();
        if escape {
            n_escape += 1;
        }
        state_rows.push(format!(
            "  STATE I=[{}]  p_I = {}  |A(I)| = {}  M_I = {} (count)  B*_I = {{{}}}  argmin_b(s+d) = {{{}}}  Delta_I^(2) = {} (count)  ESCAPE = {}",
            rec.text(),
            qs(p_i),
            st.legal.len(),
            qs(fval_to_count(Q::new(m_i, norm2), grade, p_i)),
            tiles_str(bstar),
            tiles_str(argmin),
            qs(tax_to_count(Q::new(min_sd, norm2))),
            if escape { "YES" } else { "no" }
        ));
        if escape {
            escape_rows.push(format!(
                "    ESCAPE at I=[{}]: B*_I = {{{}}} (complete), argmin_b(s+d) = {{{}}} (complete), disjoint. Delta_I^(2) = {} (count) is attained OFF the rung-one optimal face — the naive min_{{b in B*_I}} d_(I,b) = {} (count) would have OVERSTATED the true local tax, which is §6.3's inequality direction and is why a lower witness must cover EVERY first action.",
                rec.text(),
                tiles_str(bstar),
                tiles_str(argmin),
                qs(tax_to_count(Q::new(min_sd, norm2))),
                qs(tax_to_count(Q::new(
                    st.legal
                        .iter()
                        .enumerate()
                        .filter(|(slot, _)| f1[*slot] == m_i)
                        .map(|(slot, _)| sd[slot])
                        .min()
                        .expect("B*_I is nonempty"),
                    norm2
                )))
            ));
        }
        branch_counts.push(st.legal.len());
        for (slot, d) in st.legal.iter().enumerate() {
            let br = branches.get(&(*rec, d)).expect("branch");
            let s = m_i - f1[slot];
            let dd = br.sum_c - br.sum_max;
            branch_sortkey.push(s + dd);
            branch_rows.push(format!(
                "    I=[{}]  p_I = {}  |A(I)| = {}  b = {}  F1 = {} (count)  s = {} (count)  d = {} (count)  |I_2(I,b)| = {}  #{{J: delta>0}} = {}  {}{}",
                rec.text(),
                qs(p_i),
                st.legal.len(),
                tile(d),
                qs(fval_to_count(Q::new(f1[slot], norm2), grade, p_i)),
                qs(tax_to_count(Q::new(s, norm2))),
                qs(tax_to_count(Q::new(dd, norm2))),
                br.n_j,
                br.n_j_pos,
                if bstar.contains(d) { "[b in B*_I]" } else { "[b not in B*_I]" },
                if argmin.contains(d) { " [b in argmin(s+d)]" } else { "" }
            ));
        }
    }

    // SR-A22(iii)(d)'s declared cap. `branch_rows` is already in freeze-38
    // v1.1(d) order and the sort is stable, so the tie rule holds by
    // construction. A truncation is DECLARED, never silent.
    let full_branch = u64::try_from(branch_rows.len()).expect("fits");
    let mut branch_tail: Option<u64> = None;
    if branch_rows.len() > ROW_CAP {
        let mut zipped: Vec<(i128, String)> =
            branch_sortkey.iter().copied().zip(branch_rows).collect();
        zipped.sort_by_key(|r| std::cmp::Reverse(r.0));
        branch_tail = Some(u64::try_from(zipped.len() - ROW_CAP).expect("tail fits u64"));
        zipped.truncate(ROW_CAP);
        branch_rows = zipped.into_iter().map(|(_, r)| r).collect();
    }
    let printed_branch = u64::try_from(branch_rows.len()).expect("fits");
    assert_eq!(
        printed_branch + branch_tail.unwrap_or(0),
        full_branch,
        "the declared truncation accounts for every branch row"
    );

    let u2_diff = Q::new(sum_u2, norm2);
    let delta2_diff = Q::new(sum_delta2, norm2);
    let u2_glue_diff = Q::new(b.total, norm2);

    Some(SrUnit {
        n_states1,
        n_arrivals1,
        zero1,
        pos1,
        mass,
        u0_diff,
        u1_diff,
        delta1_diff,
        frontier_digest,
        n_states2,
        n_arrivals2,
        n_branches,
        sum_legal1,
        n_j_pos,
        n_j_forced,
        n_escape,
        u2_diff,
        delta2_diff,
        u2_glue_diff,
        branch_rows,
        state_rows,
        branch_counts,
        printed_branch,
        branch_tail,
        companion,
        companion_digest,
        r8_sample,
        escape_rows,
        steps_a: a.steps,
        residual_a: a.residual,
        steps_b: b.steps,
        residual_b: b.residual,
    })
}

// -- (SR-R9) the reduced-grade cross-check ----------------------------------

/// (SR-R9), BLOCKING and run before any carrier number exists. At a declared
/// grade-3 coordinate the focal seat has two decisions after the root and the
/// second is forced, so the SECOND FRONTIER IS ENTIRELY FORCED: every
/// `|A(J)| = 1`, every `delta_{I,b,J} = 0`, every `F^(2)_{I,b} = F^(1)_{I,b}`,
/// `Delta^(2) = 0` and `U^(2) = U^(1) = Q^H` — the last against the engine's
/// OWN `H` operator, an independent evaluator the grade-4 carrier cannot
/// consult.
///
/// It tests the LEMMAS and not just the code: a nonzero grade-3 `Delta^(2)`
/// falsifies Lemma FT-trunc, Lemma SR-forced or the implementation, and
/// either is stop-and-report. It is the only check in the build that
/// exercises the frontier-2 detector against a case whose answer is known BY
/// PROOF rather than by a filed number, and freeze 51(c)'s
/// forced-J-is-counted convention is exactly what it exercises.
#[allow(clippy::too_many_lines)]
fn reduced_grade_check(out: &mut String) {
    let grade = 3usize;
    let kernel = reduced_kernel(grade);
    let n_worlds = kernel.count();
    assert_eq!(
        n_worlds, 1680,
        "(SR-R9) coordinate identity: the full void-free fiber at grade 3, base index 0"
    );
    assert_eq!(
        kernel.viewer(),
        Seat::S0,
        "the viewer is the declaring leader"
    );
    assert_eq!(kernel.viewer_hand().len(), grade, "grade identity");
    let Decl::PipTrump(p) = kernel.decl() else {
        panic!("pip-trump only")
    };
    let _ = writeln!(
        out,
        "  (SR-R9) grade-{grade} coordinate: base index 0, decl = PipTrump({}), hand = [{}], pool = [{}], leader offset from focal = 0, |X| = {n_worlds}, enumeration = freeze 7/23, digest {SR_DIGEST}",
        p.value(),
        tiles_str(kernel.viewer_hand()),
        tiles_str(kernel.pool())
    );

    let dir = Direction::trick_diff();
    let mut revealed_budget = 4 * B_WALK;
    let mut revealed_stop = None;
    let prices = information_prices(
        &kernel,
        Seat::S0.team(),
        &dir,
        B_WALK,
        &mut revealed_budget,
        &mut revealed_stop,
    )
    .expect("freeze-44 budgets are non-binding at the reduced grade");

    for (i, (hi, lo, n, d)) in G3_IDX0_QH.iter().enumerate() {
        let (act, env) = &prices.q_h[i];
        assert_eq!(
            *act,
            Domino::new(Pip::new(*hi).expect("pip"), Pip::new(*lo).expect("pip")),
            "(SR-R9) filed S6a action identity"
        );
        assert_eq!(
            to_count(env.eval(qi(0)), grade),
            q(*n, *d),
            "(SR-R9) stop-and-report: the recomputed Q^H differs from the filed S6a row"
        );
    }
    let _ = writeln!(
        out,
        "    filed S6a cross-check (frozen source, quoted from predictive_rank_2026-08-12.txt, S6a, exploratory tier): the three Q^H rows reproduce exactly — HELD"
    );

    for (i, act) in kernel.viewer_hand().iter().enumerate() {
        let qh_diff = prices.q_h[i].1.eval(qi(0));
        let uc_diff = prices.q_c[i].1.eval(qi(0));
        assert_eq!(prices.q_h[i].0, act, "H action order");
        assert_eq!(prices.q_c[i].0, act, "C action order");
        let r =
            run_unit(&kernel, grade, act, false).expect("reduced-grade budgets are non-binding");
        assert_eq!(
            r.mass,
            qi(1),
            "(SR-R9) stop-and-report: the frontier arrival mass is not 1"
        );
        assert_eq!(
            r.u0_diff, uc_diff,
            "(SR-R9) stop-and-report: the frontier decomposition does not reconstruct the engine's own U^C at the reduced grade"
        );
        // The second frontier is ENTIRELY FORCED at grade 3.
        assert_eq!(
            r.n_j_forced, r.n_states2,
            "(SR-R9) stop-and-report: a grade-3 second-frontier state is not forced — |A(J)| = 1 fails, and freeze 51(c) counts forced J rather than skipping them"
        );
        assert_eq!(
            r.n_j_pos, 0,
            "(SR-R9) stop-and-report: a grade-3 second-frontier state carries a positive delta, so F^(2) != F^(1) — Lemma SR-forced or the implementation is wrong"
        );
        assert_eq!(
            r.delta2_diff,
            qi(0),
            "(SR-R9) stop-and-report: Delta^(2) is not zero at grade 3 — Lemma FT-trunc, Lemma SR-forced or the implementation is wrong (F7, NO-RESCUE)"
        );
        assert_eq!(
            r.u2_diff, r.u1_diff,
            "(SR-R9) stop-and-report: U^(2) != U^(1) at grade 3"
        );
        assert_eq!(
            r.u2_diff, qh_diff,
            "(SR-R9) stop-and-report: U^(2) != Q^H at grade 3, against the engine's own H operator"
        );
        assert_eq!(
            r.u2_glue_diff, r.u2_diff,
            "(SR-R6)/(SR-R9) stop-and-report: the glue-two-then-reveal walker and the depth-two table disagree at grade 3"
        );
        assert_eq!(
            r.n_escape, 0,
            "(SR-R9) stop-and-report: an escape action at grade 3, where every s + d equals s and the argmin is the optimal face"
        );
        let _ = writeln!(
            out,
            "    root {}: |I_1| = {}  sum_(I,b) |I_2(I,b)| = {}  ALL {} second-frontier states FORCED (|A(J)| = 1, delta = 0, counted not skipped — freeze 51(c))  U^(1) = {}  U^(2) = {}  Q^H = {}  Delta^(2) = {} (count) — U^(2) = U^(1) = Q^H HELD against the engine's own H operator; (SR-R6) HELD; ESCAPE count 0",
            tile(act),
            r.n_states1,
            r.n_states2,
            r.n_j_forced,
            qs(to_count(r.u1_diff, grade)),
            qs(to_count(r.u2_diff, grade)),
            qs(to_count(qh_diff, grade)),
            qs(tax_to_count(r.delta2_diff))
        );
    }
}

// -- the carrier's units -----------------------------------------------------

/// One (coordinate, binding competitor action) unit of the run, and its arm.
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

fn filed_u(rows: &[FiledRow; 4]) -> Vec<Q> {
    rows.iter().map(|(_, _, _, (n, d), _)| q(*n, *d)).collect()
}

/// FREEZE 51(a), enumerated with NO GENERATING RULE (FT-A23): arm 1 is h2,
/// competitor 53 then competitor 54; arm 2 is h9, competitor 41 then
/// competitor 54, attempted after arm 1 completes.
fn carrier_units() -> Vec<UnitKey> {
    vec![
        UnitKey {
            coord: 0,
            action: dom(5, 3),
            arm: 1,
        },
        UnitKey {
            coord: 0,
            action: dom(5, 4),
            arm: 1,
        },
        UnitKey {
            coord: 1,
            action: dom(4, 1),
            arm: 2,
        },
        UnitKey {
            coord: 1,
            action: dom(5, 4),
            arm: 2,
        },
    ]
}

/// One unit's two emissions: the named committed file's text, and the
/// companion's per-`(I,b,J)` rows.
struct UnitText {
    named: String,
    companion: Vec<String>,
    stopped: bool,
}

#[allow(clippy::too_many_lines)]
fn render_unit(receipt: &Receipt, key: UnitKey) -> UnitText {
    let (hand_id, filed_pip, rows, l_exhibited) = &SR_FILED[key.coord];
    let hand = &receipt.hands[*hand_id];
    let kernel = n4_void_free_kernel(hand);

    // Freeze 45's coordinate identity, ASSERTED FIRST and REBUILT IN-RUN: an
    // equality of values at coordinates not shown to be the same coordinate
    // is not a cross-check.
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
    let qh_count = filed_qh(rows);
    let u_count = filed_u(rows);
    let ia = actions
        .iter()
        .position(|x| *x == key.action)
        .expect("the unit's action is a root action");
    // Freeze 50(b): `a*` ranges over the filed H-argmax set, and a binding
    // PAIR is `(a*, a)` with `a != a*` — so the unit's own action is never
    // its own `a*`, even when the pair is tied and both sit in the H-argmax
    // set, which is exactly the case at both carrier coordinates.
    let vh = qh_count.iter().copied().max().expect("nonempty");
    let astar_idx = (0..actions.len())
        .find(|i| qh_count[*i] == vh && *i != ia)
        .expect("a binding pair has an H-argmax competitor distinct from a");
    let l_astar = qh_count[astar_idx];
    assert!(
        l_astar - u_count[ia] < qi(0),
        "freeze 50(b): every carrier unit is a BINDING competitor — its filed margin L_(a*) - U_a is negative"
    );

    let first = SR_FIRST
        .iter()
        .find(|(h, hi, lo, ..)| *h == *hand_id && dom(*hi, *lo) == key.action)
        .expect("every carrier unit has a rung-one filed record");
    let (_, _, _, f_states, f_arrivals, f_zero, f_pos, f_d1, f_u1, f_d2, f_steps, f_digest) =
        *first;

    let mut out = String::new();
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== SR unit (freeze 51(a), ARM {}): coord h{hand_id} grade={N4_GRADE} pip={} hand=[{}] pool=[{}] leader-offset=0 |X|={N4_FIBER} enumeration=freeze-7/23 || competitor a = {} ==",
        key.arm,
        p.value(),
        tiles_str(kernel.viewer_hand()),
        tiles_str(kernel.pool()),
        tile(key.action)
    );
    let _ = writeln!(
        out,
        "  provenance only: corpus hand id {hand_id}, trick {N4_TRICK} (never identity components, freeze 45); freeze-set digest {SR_DIGEST}"
    );
    let _ = writeln!(
        out,
        "  frozen source (SR-A22(v), the SEP-A14(ii) pattern; quoted from separation_n4_2026-08-14.txt and fusion_tax_2026-08-14.txt, exploratory tier, NEVER re-parsed at run time): Q^H(a) = {}  U_a = {}  U^(1) = {}  Delta^(1) = {}  Delta^(2) = {}  |I_1| = {f_states}  arrivals = {f_arrivals}  census (zero/positive) = {f_zero}/{f_pos}  (count convention)",
        qs(qh_count[ia]),
        qs(u_count[ia]),
        qs(q(f_u1.0, f_u1.1)),
        qs(q(f_d1.0, f_d1.1)),
        qs(q(f_d2.0, f_d2.1))
    );
    let _ = writeln!(
        out,
        "  CUT (freeze 38(b)(1), layer k = 2; SR-A21(i): the depth-two cut is ALREADY INSIDE freeze 38 v1 and needs no new authority, and freeze 38 v2 is NOT opened): the one-block partition at every focal frontier of depth <= 2, singleton below. THIS CUT IDENTIFIES ACTION VARIABLES AT ONE INFORMATION STATE; THE FIBER AND EVERY WORLD'S MASS ARE UNTOUCHED (FT-A10(ii), verbatim and binding). Validity is discharged by freeze 38(c)'s single argument for the whole family."
    );

    let Some(r) = run_unit(&kernel, N4_GRADE, key.action, true) else {
        let _ = writeln!(
            out,
            "  DECLARED STOP (freeze 44(b) v2, SR-A24(e)): a walk budget of {B_WALK} was exhausted. NO PARTIAL FOLD is retained, so there is no partial s, no partial d, no partial Delta^(2) and no partial U^(2); this is a stop and is never a finding (R-A18). Arm 2 stopping while arm 1 completes is an ordinary outcome, not a failure."
        );
        return UnitText {
            named: out,
            companion: Vec::new(),
            stopped: true,
        };
    };

    // ---- (SR-R10) determinism: the in-run second pass ----------------------
    // The second pass keeps no companion rows: its comparison is the
    // STREAMING DIGEST over the identical row sequence, which is at least as
    // strong as string equality and does not hold a second copy of a table
    // with one row per (I,b,J).
    let r2 = run_unit(&kernel, N4_GRADE, key.action, false)
        .expect("the second pass runs under the same non-binding budgets as the first");
    assert_eq!(
        (
            r.u0_diff,
            r.u1_diff,
            r.delta1_diff,
            r.u2_diff,
            r.delta2_diff,
            r.u2_glue_diff,
            r.mass
        ),
        (
            r2.u0_diff,
            r2.u1_diff,
            r2.delta1_diff,
            r2.u2_diff,
            r2.delta2_diff,
            r2.u2_glue_diff,
            r2.mass
        ),
        "(SR-R10) stop-and-report: two passes of one unit disagree on a summary value"
    );
    assert_eq!(
        (
            r.n_states1,
            r.n_arrivals1,
            r.zero1,
            r.pos1,
            r.n_states2,
            r.n_arrivals2
        ),
        (
            r2.n_states1,
            r2.n_arrivals1,
            r2.zero1,
            r2.pos1,
            r2.n_states2,
            r2.n_arrivals2
        ),
        "(SR-R10) stop-and-report: two passes of one unit disagree on a census integer"
    );
    assert_eq!(
        (
            r.n_branches,
            r.sum_legal1,
            r.n_j_pos,
            r.n_j_forced,
            r.n_escape,
            r.steps_a,
            r.steps_b
        ),
        (
            r2.n_branches,
            r2.sum_legal1,
            r2.n_j_pos,
            r2.n_j_forced,
            r2.n_escape,
            r2.steps_a,
            r2.steps_b
        ),
        "(SR-R10) stop-and-report: two passes of one unit disagree on an accounting integer"
    );
    assert_eq!(
        (
            &r.branch_rows,
            &r.state_rows,
            &r.escape_rows,
            &r.r8_sample,
            &r.branch_counts
        ),
        (
            &r2.branch_rows,
            &r2.state_rows,
            &r2.escape_rows,
            &r2.r8_sample,
            &r2.branch_counts
        ),
        "(SR-R10) stop-and-report: two passes of one unit disagree on a printed row"
    );
    assert_eq!(
        (&r.companion_digest, &r.frontier_digest),
        (&r2.companion_digest, &r2.frontier_digest),
        "(SR-R10) stop-and-report: two passes of one unit disagree on the companion or frontier digest"
    );
    drop(r2);

    // ---- the count-convention columns -------------------------------------
    let u0_c = to_count(r.u0_diff, N4_GRADE);
    let u1_c = to_count(r.u1_diff, N4_GRADE);
    let u2_c = to_count(r.u2_diff, N4_GRADE);
    let d1_c = tax_to_count(r.delta1_diff);
    let d2_c = tax_to_count(r.delta2_diff);

    // ---- (SR-R7) the rung-one invariance receipt ---------------------------
    assert_eq!(
        (r.n_states1, r.n_arrivals1, r.zero1, r.pos1),
        (f_states, f_arrivals, f_zero, f_pos),
        "(SR-R7) stop-and-report: this probe's rung-one frontier census differs from the frozen FT_FIRST record"
    );
    assert_eq!(
        (d1_c, u1_c),
        (q(f_d1.0, f_d1.1), q(f_u1.0, f_u1.1)),
        "(SR-R7) stop-and-report: this probe's rung-one Delta^(1) or U^(1) differs from the frozen FT_FIRST record"
    );
    assert_eq!(
        u0_c, u_count[ia],
        "(SR-R7) stop-and-report: the rung-one reconstruction does not return the frozen filed U_a"
    );
    assert_eq!(
        r.mass,
        qi(1),
        "(SR-R2) stop-and-report: sum_I p_I != 1 — a field branch was dropped or double-counted"
    );

    // ---- (SR-R4) the ladder receipt ---------------------------------------
    assert_eq!(
        u2_c,
        qh_count[ia],
        "(SR-R4) stop-and-report: U^(2) = T_0 + sum_I max_b F^(2)_(I,b) != the frozen filed Q^H(a). Nothing is claimed and the disagreeing exact rationals are {} and {} (count); this tests Lemma FT-trunc, Corollary FT-grade4, Theorem 4.1's U^(2) formula and the whole depth-two construction at once (F7, NO-RESCUE)",
        qs(u2_c),
        qs(qh_count[ia])
    );
    // ---- (SR-R5) the interchange receipt ----------------------------------
    assert_eq!(
        d2_c,
        q(f_d2.0, f_d2.1),
        "(SR-R5) stop-and-report: sum_I min_b [s_(I,b) + d_(I,b)] != the frozen filed Delta^(2). The disagreeing exact rationals are {} and {} (count) (F7, NO-RESCUE)",
        qs(d2_c),
        qs(q(f_d2.0, f_d2.1))
    );
    // ---- (SR-R6) the two-path receipt -------------------------------------
    assert_eq!(
        r.u2_diff,
        r.u2_glue_diff,
        "(SR-R6) stop-and-report: the table-derived U^(2) and the independently written glue-two-then-reveal walker disagree; the disagreeing exact rationals are {} and {} (differential)",
        qs(r.u2_diff),
        qs(r.u2_glue_diff)
    );

    // ---- the accounting integers (SR-A22(iii)(d), FT-A24(iv)) --------------
    let _ = writeln!(
        out,
        "  ACCOUNTING (SR-A22(iii)(d); the companion's omission is auditable from these integers alone): |I_1| = {}  sum_I |A(I)| = {}  sum_(I,b) |I_2(I,b)| = {}  #{{(I,b,J): delta > 0}} = {}  #{{(I,b,J): |A(J)| = 1}} = {}  #{{I: ESCAPE}} = {}  (state, world) arrivals: rung one {} / rung two {} ; P_max v2 admission {} <= {} — ASSERTED BEFORE the aggregate pass ; sum_I Delta_I^(2) = {} (count) = the reported Delta^(2) — ASSERTED.",
        r.n_states1,
        r.sum_legal1,
        r.n_states2,
        r.n_j_pos,
        r.n_j_forced,
        r.n_escape,
        r.n_arrivals1,
        r.n_arrivals2,
        r.n_states2,
        P_MAX,
        qs(d2_c)
    );
    assert_eq!(
        r.n_branches, r.sum_legal1,
        "every (I,b) branch is present exactly once"
    );

    match r.branch_tail {
        None => {
            let _ = writeln!(
                out,
                "  DECLARED CAP (SR-A22(iii)(d)): the cap is {ROW_CAP} branch rows per unit; this unit has {} and the cap DID NOT FIRE — every (I,b) branch row and every (I) state row is printed below, in freeze-38 v1.1(d) order. Had it fired, the file would carry the top {ROW_CAP} by descending s + d with ties by ascending emission order, plus the residual tail's exact count, printed in place — a declared truncation, never a silent one.",
                r.printed_branch
            );
        }
        Some(t) => {
            let _ = writeln!(
                out,
                "  DECLARED TRUNCATION (SR-A22(iii)(d)): {} branch rows, above the declared cap of {ROW_CAP}. Printed: the top {ROW_CAP} by descending s + d, ties by ascending emission order. RESIDUAL TAIL: exactly {t} rows, omitted here and present in full in the companion's parent columns.",
                r.printed_branch + t
            );
        }
    }
    let _ = writeln!(
        out,
        "  ROWS (freeze 38 v1.1(d) order, exhibited: first states I in ascending record order; within I, first actions b in ascending domino index; within (I,b), second states J in ascending record order; within J, second actions c in ascending domino index). COUNT CONVENTION on every column, with the two bridges kept separate (Corollary SR-conv): F1, M_I, C and A are p-weighted VALUES and are bridged as (x_diff + grade*p)/2; s, d, delta and Delta are DIFFERENCES at a common state and are bridged as x_diff/2, a differential tax being exactly twice its count value."
    );
    if r.branch_tail.is_none() {
        // Each state row is followed by its own branch rows, both in
        // freeze-38 v1.1(d) order.
        let mut start = 0usize;
        for (srow, n) in r.state_rows.iter().zip(&r.branch_counts) {
            let _ = writeln!(out, "{srow}");
            for brow in &r.branch_rows[start..start + n] {
                let _ = writeln!(out, "{brow}");
            }
            start += n;
        }
        assert_eq!(
            start,
            r.branch_rows.len(),
            "every branch row belongs to exactly one state row"
        );
    } else {
        // The declared cap reordered the branch rows, so the state rows and
        // the capped branch block are printed as two blocks.
        for srow in &r.state_rows {
            let _ = writeln!(out, "{srow}");
        }
        for brow in &r.branch_rows {
            let _ = writeln!(out, "{brow}");
        }
    }

    // ---- the escape census (SR-A24(c)/(d)) ---------------------------------
    if r.n_escape == 0 {
        let _ = writeln!(
            out,
            "  ESCAPE CENSUS — SR-A24(d): ESCAPE ACTIONS ABSENT at every one of the {} first states of this unit. argmin_b (s_(I,b) + d_(I,b)) meets B*_I everywhere, so the naive min_(b in B*_I) d_(I,b) would have coincided with the truth HERE. THIS IS A RESULT TOO, AND IT IS NOT A LICENCE: §6.3's inequality is one-directional and a coincidence at this coordinate licenses nothing about another. Filed under F7 with P-A21 and the FT-A26(iii) selection fence attached.",
            r.n_states1
        );
    } else {
        let _ = writeln!(
            out,
            "  ESCAPE CENSUS — SR-A24(c): ESCAPE ACTIONS PRESENT at {} of the {} first states of this unit — the first measured instance of policy adjustment in the branch. §6.3's warning is NOT hypothetical at our scale, and EVERY FUTURE RUNG-TWO LOWER WITNESS MUST COVER EVERY FIRST ACTION, not the rung-one optimal face. A result, filed as one, SCOPED TO THIS COORDINATE AND NOTHING WIDER. Every escaping state is printed entire, with both complete sets:",
            r.n_escape,
            r.n_states1
        );
        for row in &r.escape_rows {
            let _ = writeln!(out, "{row}");
        }
    }

    // ---- the receipts, one by one -----------------------------------------
    let _ = writeln!(
        out,
        "  (SR-R1) BRANCH RECONSTRUCTION — HELD at every one of the {} (I,b) branches: Theta_(I,b) + sum_J C_(I,b,J) = F^(1)_(I,b), the right-hand side being the rung-one branch value sum_omega mu_I(omega) q_I(omega,b) computed by the rung-one path. The two sides come from different passes over different intermediate quantities; it fails on any error in second-frontier detection, in the between-frontier arrival weights, in the J keying or in the parent attribution.",
        r.n_branches
    );
    let _ = writeln!(
        out,
        "  (SR-R2) MASS — HELD: sum_J p_(I,b,J) + p^term_(I,b) = p_I at every one of the {} branches, sum_I p_I = 1 exactly, and p^term_(I,b) = 0 and Theta_(I,b) = 0 asserted by the focal seat having a further decision in EVERY positive-mass world after b (the depth counter reaches grade - 1 = {} at every terminal). It is the only check that the J list is exhaustive: a dropped J lowers C, A, F^(1) and F^(2) together and understates Delta^(2) with every algebraic assertion still green.",
        r.n_branches,
        N4_GRADE - 1
    );
    let _ = writeln!(
        out,
        "  (SR-R3) PARENT — HELD at every one of the {} second-frontier states: replaying J's record recovers (I,b) exactly (Lemma SR-coord(b) instantiated — the focal seat's second play sits at a position determined BY THE RECORD, computed by an independent replay that shares no bookkeeping with the walk), the replayed banked increment equals the walk's, and I_2(I,b) and I_2(I',b') are disjoint for distinct branches (asserted at every arrival: one J may never carry two parents). It fails if any coordinate coarsening has crept in.",
        r.n_states2
    );
    let _ = writeln!(
        out,
        "  (SR-R4) LADDER — HELD: U^(2) = T_0 + sum_I max_b F^(2)_(I,b) = {} (count) = the frozen filed Q^H(a) exactly. The strongest single check in the build: it tests Lemma FT-trunc, Corollary FT-grade4, Theorem 4.1's U^(2) formula and the entire depth-two construction at once, against an exact solve produced by a different evaluator on a different day.",
        qs(u2_c)
    );
    let _ = writeln!(
        out,
        "  (SR-R5) INTERCHANGE — HELD: sum_I min_b [s_(I,b) + d_(I,b)] = {} (count) = the frozen filed Delta^(2) exactly. This is Theorem 6.2 instantiated against a value produced by an entirely different route — U^(1) - Q^H from two independently filed columns.",
        qs(d2_c)
    );
    let _ = writeln!(
        out,
        "    ARITHMETIC REMARKS, NOT RECEIPTS (Proposition SR-taut, named in place per SR-R5 and NEVER counted among receipts HELD — they cannot fail): delta_(I,b,J) >= 0 ; d_(I,b) >= 0 ; s_(I,b) >= 0 ; F^(1)_(I,b) - F^(2)_(I,b) = sum_J delta_(I,b,J) ; max_b F^(1)_(I,b) - max_b F^(2)_(I,b) = min_b (s + d). All five are identities in this probe's own recomputed quantities and hold for every input whatsoever, valid or not. \"By construction is not a receipt\" (PG-A8), and \"the algebra checks out\" is not one either."
    );
    let _ = writeln!(
        out,
        "  (SR-R6) TWO-PATH — HELD: U^(2) computed from the depth-two table and computed directly by the independently written GLUE-TWO-THEN-REVEAL walker — a pooled bag lawful at the first frontier and lawful again at the second (max outside the world sum at BOTH), world-informed below — agree exactly at {} (count), differential {}. The two computations share only the rule algebra: no map, no accumulator, no arrival struct and no recursion. This is (FT-R4) one rung up and it is the receipt that would catch a wrong max/sum order.",
        qs(u2_c),
        qs(r.u2_diff)
    );
    let digest_line = if f_digest.is_empty() {
        assert!(
            r.frontier_digest.len() == 64,
            "(FT-R7c) a digest is 64 hex characters"
        );
        format!(
            "the frozen slot is EMPTY, so this run EMITS the digest for transcription and asserts nothing against it: {}. FT-A28(iv) made (FT-R7c) binding on the next run that regenerates a frontier; this is that run, the digest now exists as a committed artifact, and the ASSERTION half is discharged on the next run that carries it transcribed",
            r.frontier_digest
        )
    } else {
        assert_eq!(
            r.frontier_digest, f_digest,
            "(SR-R7)/(FT-R7c) stop-and-report: the frontier digest differs from the frozen transcribed value"
        );
        format!(
            "ASSERTED EQUAL to the frozen transcribed value {} — a comparison against a PRIOR PROCESS, which is what closes the across-process residual FT-A28(iii) named",
            r.frontier_digest
        )
    };
    let _ = writeln!(
        out,
        "  (SR-R7) RUNG-ONE INVARIANCE — HELD against the extended FT_FIRST frozen table (quoted from CENSUS-RULINGS.md and re-checked against fusion_tax_2026-08-14.txt; never re-parsed from results text): |I_1| = {}, arrivals {}, census {}/{} (zero/positive), Delta^(1) = {}, U^(1) = {}, and U^(0) = {} = the frozen filed U_a, all reproduce exactly by a probe that computes them as a by-product of a DEPTH-TWO traversal. (FT-R7c) THE FRONTIER DIGEST: SHA-256 over the canonical serialisation of the (record, delta_I) pairs in freeze-38(d) order — one line per state, `<record>|<delta_I as num/den, count>` and a newline — {}.",
        r.n_states1,
        r.n_arrivals1,
        r.zero1,
        r.pos1,
        qs(d1_c),
        qs(u1_c),
        qs(u0_c),
        digest_line
    );
    let _ = writeln!(
        out,
        "    (FT-R7a)'s CORRECTED SCOPE LINE, adopted verbatim: \"reaches sum_I delta_I and |supp delta_I| per unit across executions; does not reach individual delta_I.\" The digest above is what extends that reach to the individual delta_I, and it does so only once a later run carries it transcribed."
    );
    let _ = writeln!(
        out,
        "  (SR-R8) COMPLETE-FACE AT RUNG TWO — HELD. Corollary 5.2 is asserted BOTH WAYS at EVERY one of the {} (I,b,J) triples of this unit (strictly stronger than the declared sample), from COMPLETE argmax_c sets and never from freeze 26's least-index tie rule (freeze 38(e), FT-A8, and §6.3's stronger rule that the complete optimal face is not sufficient either). |A(I)| and |A(J)| are asserted against legal_plays at the INDEPENDENTLY RECONSTRUCTED position at every state, since §6.3 makes the completeness of the b range load-bearing. The declared deterministic sample — the first {R8_SAMPLE} (I,b,J) triples in freeze-38 v1.1(d) order — is printed:",
        r.n_states2
    );
    for s in &r.r8_sample {
        let _ = writeln!(out, "{s}");
    }
    let _ = writeln!(
        out,
        "  (SR-R9) — HELD at the declared grade-3 coordinate, in this file's BLOCKING block above, run before any carrier number existed."
    );
    let _ = writeln!(
        out,
        "  (SR-R10) DETERMINISM — HELD: a full in-run second pass with fresh maps, fresh accumulators and fresh budgets recomputed this unit entire; every summary value, every accounting integer, every one of the {} printed branch rows, every one of the {} state rows, every escape row, the (SR-R8) sample, the companion digest and the frontier digest were asserted identical. It fails on uninitialised state, on accumulator reuse and on any accidental dependence on iteration order.",
        r.printed_branch,
        r.state_rows.len()
    );

    // ---- the summary -------------------------------------------------------
    let _ = writeln!(
        out,
        "  SUMMARY (COUNT convention on every column; SR-A22(i)'s per-unit objects): T_0 = 0 (asserted)  Theta_(I,b) = 0 (asserted)  U^(0) = {}  U^(1) = {}  U^(2) = sum_I max_b F^(2)_(I,b) = {}  Delta^(1) = {}  Delta^(2) = sum_I Delta_I^(2) = {}  ESCAPE states {} of {}.",
        qs(u0_c),
        qs(u1_c),
        qs(u2_c),
        qs(d1_c),
        qs(d2_c),
        r.n_escape,
        r.n_states1
    );
    let _ = writeln!(
        out,
        "  PAIR TYPING (SR-A24(g), SETTLED A PRIORI — NO CLOSURE VERDICT IS REPORTED FOR THIS BUILD): a* = {} with L_(a*) = Q^H(a*) = {}, competitor a = {} with Q^H(a) = {}. By Proposition SR-degen, L_(a*) >= U_a^(2) holds at EVERY binding pair of this carrier UNCONDITIONALLY — a* is H-optimal so Q^H(a*) >= Q^H(a), and U_a^(2) = Q^H(a) by Corollary FT-grade4 — with equality exactly at the tied pairs, which both carrier coordinates are. NO GRADE-4 EXPERIMENT CAN TEST WHETHER THE SECOND RUNG CLOSES A BINDING PAIR. What this build reports is the identity, the decomposition and the escape census, and nothing else. {}",
        tile(actions[astar_idx]),
        qs(l_astar),
        tile(key.action),
        qs(qh_count[ia]),
        if *l_exhibited {
            "L_(a*) = Q^H(a*) is the filed primal witness (Corollary E4.1(2); the filed run's R2 receipt HELD at this coordinate)."
        } else {
            "L_(a*) = Q^H(a*) is Corollary E4.1(2)'s CEILING ONLY — this coordinate is NOT PRICED and NO PRIMAL WITNESS IS EXHIBITED (FT-A18(iv), RW-A3(iii)); the NOT PRICED label stands verbatim and is not weakened by this run."
        }
    );
    let _ = writeln!(
        out,
        "  SR-A24(f), SETTLED A PRIORI and reportable only as such — RUNG-TWO FUSION CORES: at grade 4 the focal seat holds two tiles at the second frontier, so |A(J)| <= 2; argmax sets are then nonempty subsets of a two-element set and an empty intersection forces one world with {{c_1}} and one with {{c_2}}. EVERY POSITIVE-delta MINIMAL CORE THEREFORE HAS SIZE EXACTLY 2, BY ARITHMETIC, BEFORE ANY MEASUREMENT. The received note's open ledger row \"second-rung fusion cores remain binary\" is UNMEASURABLE at this carrier and this run may NOT be reported as answering it; it is open only at grade >= 5, where |A(J)| >= 3 becomes possible. The {} positive-delta triples of this unit all carry a size-2 core and that is a restatement of the arithmetic, not a measurement.",
        r.n_j_pos
    );
    let _ = writeln!(
        out,
        "  COMPOSITION FORM (Lemma FT-post, binding, printed in place per SR-A2(iv)): this probe pastes NO residual witness. Every continuation value is evaluated INSIDE THE SAME WALK under the carried arrival weights — FORM (i) of Lemma FT-post. Form (ii) is not used and form (1) of the received note's §1.4 — \"evaluated under the actual posterior\" — is not adopted, because the posterior here is EXHIBITED as the walk's arrival weights, which makes it form (i) and it is called that. The frontier posteriors nu_I and nu_(I,b,J) are NOT uniform and are never treated as such: constant p_I and constant |X_I| do not make nu_I uniform (Lemma FT-post, SR-A18(vi))."
    );
    let _ = writeln!(
        out,
        "  SLOT TYPING (FT-A12(iii), SR-A22(vi), binding): no row of this unit names a per-world BOUND. The q_(I,b,J)(omega,c) are exact world-informed continuation values, not bounds, and none is ever carried out of a frontier and installed as a root primal witness (Non-theorem E4'). Every argmax and argmin printed is a COMPLETE set; a tie-broken optimiser appears nowhere."
    );
    let _ = writeln!(
        out,
        "  walk-step observables (SEP-A19(b) class, per-traversal named — never an information value, a decision width, a cost claim or a DS-A2 term; PROVENANCE ONLY): PATH A2 (world-major revealed walks recording BOTH frontiers) {} charged, residual {} ; PATH B2 (glue-two-then-reveal, pooled to the second frontier) {} charged, residual {}. Same-traversal comparison against the filed FT PATH A subtotal {f_steps} for this (coordinate, action): {}. This is a traversal-shape observable only; it constrains no value, it is NOT a receipt, and (SR-R1)/(SR-R4)/(SR-R5)/(SR-R6) are the value checks.",
        r.steps_a,
        r.residual_a,
        r.steps_b,
        r.residual_b,
        if r.steps_a == f_steps {
            "EQUAL"
        } else {
            "DIFFERS"
        }
    );

    let mut companion = Vec::with_capacity(r.companion.len() + 1);
    companion.push(format!(
        "== COMPANION per-(I,b,J) table (SR-A22(iii)(d)): coord h{hand_id} pip={} hand=[{}] competitor a = {} || {} rows, freeze-38 v1.1(d) order, SHA-256 of these rows {} ==",
        p.value(),
        tiles_str(kernel.viewer_hand()),
        tile(key.action),
        r.companion.len(),
        r.companion_digest
    ));
    companion.extend(r.companion.iter().cloned());
    UnitText {
        named: out,
        companion,
        stopped: false,
    }
}

// -- main --------------------------------------------------------------------

#[allow(clippy::too_many_lines)]
fn header(out: &mut String) {
    let _ = writeln!(
        out,
        "walt second-rung probe — the SR family of SR-A22: the EXACT depth-two layer over freeze 51's carrier — EXPLORATORY TIER"
    );
    let _ = writeln!(
        out,
        "rulings: SR-A1..SR-A26, freezes 38 v1.1(d) / 44 v2 / 45 / 50 v1.1 / 51 (walt/CENSUS-RULINGS.md 2026-08-14); Lemma SR-coord, Lemma SR-forced, Proposition SR-sep, Proposition SR-post, Corollary SR-conv, Proposition SR-degen, Proposition SR-taut; beneath them the whole FT chapter (Lemma FT-arrive, Lemma FT-trunc, Corollary FT-grade4, Proposition FT-flat, Lemma FT-post); mathematics under DS-A17: Lemma E3 + (C1)-(C4), Lemma E4, Corollary E4.1, Theorem E6.4, Theorem E6.5"
    );
    let _ = writeln!(
        out,
        "regenerate (both files, deterministically, from the repository alone): cargo run --release -p walt-factory --example second_rung"
    );
    let _ = writeln!(out, "freeze-set digest (freeze 51(h)): {SR_DIGEST}");
    let _ = writeln!(out, "{COMPANION_LINE_PLACEHOLDER}");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "TIER: exploratory throughout, without exception (SR-A1). Nothing here is promoted, nothing is quotable in a brief, a dispatch, FINDINGS.md or any claim-tier page except by brief amendment adding it to a verifier receipt; an external note is never imported as an axiom (TRUST-01). NO NUMBER OF THE RECEIVED NOTE ENTERS AS EVIDENCE: every value below is an exact rational of this engine, and the ten rationals of inbox 017's §13 table are OURS."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== PROPOSITION SR-degen, PRINTED IN THE HEADER BEFORE ANY DEPTH-TWO NUMBER EXISTS (SR-A18, SR-A24(g)) =="
    );
    let _ = writeln!(
        out,
        "  Let a* be a root action whose primal witness attains the lawful ceiling, L_(a*) = Q^H(a*) (Corollary E4.1(2)), let a != a* be any competitor, and work at an n = 4 coordinate so that U_a^(2) = Q^H(a) (Corollary FT-grade4). Then L_(a*) >= U_a^(2) ALWAYS, and the inequality is strict exactly when the pair is untied, with exact surplus Q^H(a*) - Q^H(a). Proof: a* is H-optimal, so Q^H(a*) >= Q^H(a); substitute both filed identities."
    );
    let _ = writeln!(
        out,
        "  WHAT IT FORCES ON THE READING: NO GRADE-4 EXPERIMENT CAN TEST WHETHER THE SECOND RUNG CLOSES A BINDING PAIR — the answer is fixed by two already-filed columns and by Lemma FT-trunc. Both carrier coordinates are TIED, so closure holds with EQUALITY at every pair here, unconditionally. NO CLOSURE VERDICT IS REPORTED FOR THIS BUILD. What the build reports is the identity, the decomposition and the escape census. This is Proposition FT-tie's job one rung up and it earns its keep the same way: as a fence on the reading, filed before the run."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== ALL OUTCOMES PRE-DECLARED (SR-A24, printed before any depth-two number exists; F7 binds — both answers to every open gate are results) =="
    );
    let _ = writeln!(
        out,
        "  (a) (SR-R4) AND (SR-R5) HOLD -> Theorem 6.2 and Theorem 4.1 are instantiated exactly at this carrier against two independently produced filed columns, and the (s, d) decomposition of the fusion gap exists as an artifact for the first time. This is the result the build is for, and it is A RESULT ABOUT THE PROOF MACHINERY, NOT A DISCOVERY ABOUT 42 — the exact value column already knows h2's answer."
    );
    let _ = writeln!(
        out,
        "  (b) (SR-R4) OR (SR-R5) FAILS -> the most informative outcome available and pre-declared as such: either Theorem 4.1's hypotheses fail in this engine, or Theorem 6.2 does, or the implementation is wrong. Nothing is claimed, the disagreeing exact rationals are printed, and no patch is attempted (F7, NO-RESCUE)."
    );
    let _ = writeln!(
        out,
        "  (c) ESCAPE ACTIONS PRESENT — at some I, argmin_b (s_(I,b) + d_(I,b)) is disjoint from B*_I -> the first measured instance of policy adjustment in the branch. §6.3's warning is then not hypothetical at our scale, and every future rung-two lower witness must cover EVERY first action, not the optimal face. A result, filed as one, scoped to this coordinate and nothing wider."
    );
    let _ = writeln!(
        out,
        "  (d) ESCAPE ACTIONS ABSENT at every I of the carrier -> the minimum is attained on the rung-one optimal face throughout, so the naive min_(b in B*_I) d_(I,b) would have coincided with the truth here. THIS IS A RESULT TOO, AND IT IS NOT A LICENCE: §6.3's inequality is one-directional and a coincidence at two coordinates licenses nothing about a third. Filed under F7 with P-A21 and the FT-A26(iii) selection fence attached."
    );
    let _ = writeln!(
        out,
        "  (e) A BUDGET STOP, or P_max exceeded at the second frontier -> declared stop, no partial fold retained (freeze 44(b) v2), no partial s, d or Delta^(2) reported, printed as a stop and never as a finding (R-A18). ARM 2 (h9) STOPPING WHILE ARM 1 (h2) COMPLETES IS AN ORDINARY OUTCOME, NOT A FAILURE."
    );
    let _ = writeln!(
        out,
        "  (f) SETTLED A PRIORI, and reportable only as such — RUNG-TWO FUSION CORES. At grade 4 the focal seat holds two tiles at the second frontier, so |A(J)| <= 2; argmax sets are then nonempty subsets of a two-element set, and an empty intersection forces one world with {{c_1}} and one with {{c_2}}. EVERY POSITIVE-delta MINIMAL CORE THEREFORE HAS SIZE EXACTLY 2, BY ARITHMETIC, BEFORE ANY MEASUREMENT. The received note's open ledger row \"second-rung fusion cores remain binary\" is UNMEASURABLE AT THIS CARRIER and this run may not be reported as answering it. It is open only at grade >= 5. This is FT-A26(ii)'s lesson caught BEFORE the run instead of after it."
    );
    let _ = writeln!(
        out,
        "  (g) SETTLED A PRIORI — PAIR CLOSURE. Proposition SR-degen above. No closure verdict is reported for this build."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "FENCE (R-A2, P-A1, mandatory in this header per SR-A25(i)): no object produced by this probe is an identity-bearing witness of anything; reachability is a proof-irrelevant proposition; the carrier is the void-free capacity fiber whose members are FEASIBLE and never reachable."
    );
    let _ = writeln!(
        out,
        "REAL-DEAL FENCE (N4-A8, verbatim, travelling with every carrier coordinate): the hands and pools come from rob's receipt corpus, THE BELIEF DOES NOT. The voids the play record had already revealed are deliberately discarded (P-A2's void-free carrier), and support is not belief in any case. No row here is a statement about correct play in that deal, about reachability, or about any belief other than the declared one; the void-filtered column licenses nothing and is not reproduced."
    );
    let _ = writeln!(
        out,
        "SELECTION FENCE (SR-A25(iii), FT-A26(iii), P-A21, restated because the received note omits it and because it binds every escape-action and sparsity number this build produces): five coordinates chosen by NEGATIVE BINDING MARGIN are a CARRIER, NOT A SAMPLE, and the selection criterion is correlated with the quantity being described. TWO of the five are in scope here, and NEITHER THE ESCAPE RATE NOR THE RUNG-TWO TAX DENSITY MAY BE READ AS A DISTRIBUTION over coordinates or over hands."
    );
    let _ = writeln!(
        out,
        "NOT CLAIMED, printed in place (SR-A25(ii)): nothing about points or marks (the valuation is the count-free trick differential; E-A2's boundary, and a count re-entry voids every form-keyed record wholesale); nothing about bidding; nothing about how real opponents play; NO DISTRIBUTION MEASURED AT GRADE 4 IS QUOTED FOR TRICK 1 OR FOR THE OPENING (P-A21); and no cost, timing or tractability claim is read off any traversal observable (SEP-A19(b), N4-A16) — walk-step and wall-clock columns are PROVENANCE."
    );
    let _ = writeln!(
        out,
        "WHAT IS NOT COMMISSIONED (SR-A22(vii)): no penalty family, no feature moment, no regret-event minorant, no b-uniform event, no deeper rung, no trick-1 object, and no Python verifier (SR-A17(iii)). Freeze 38(g)'s exclusions stand and nothing here fixes freeze 38 v2."
    );
    let _ = writeln!(
        out,
        "THE LOAD-BEARING RISK, NAMED SO IT IS WATCHED (SR-A25(vii)): Lemma SR-coord — the hypothesis that makes Theorem 4.1 true at all — was discharged by reading the implementation and freeze 26's contract at adjudication time. If the implementation and the rules corpus disagree, the mathematics is still correct and its application here is wrong, and NO RECEIPT INSIDE THIS FILE CAN DETECT IT, because every receipt is computed by the same implementation. (SR-R9) is the partial guard. The corpus check of T1-A12 and LD-A10(ii) is still owed before any of this leaves walt."
    );
    let _ = writeln!(
        out,
        "CUT ORDERING — FREEZE 38 v1.1(d) (SR-A21(ii), the induced total order EXHIBITED rather than described, declared before the run and never chosen by result): first states I in ascending record order; within I, first actions b in ascending domino index; within (I,b), second states J in ascending record order; within J, second actions c in ascending domino index. Second-frontier records are frontier information states of layer 2 and are ordered by the same freeze-36(b) lexicographic rule, and J's record strictly extends I's (Lemma SR-coord(b)), so this is a well-defined total order on (I,b,J,c). NO BLOCK MERGES ARE USED — freeze 38(b)(2) is scoped to the first frontier and is not exercised."
    );
    let _ = writeln!(
        out,
        "FRONTIER-2 CONVENTION — FREEZE 51(c), fixed by Lemma SR-forced: the second frontier is the focal seat's NEXT DECISION AFTER b, FORCED OR NOT. A forced J is a frontier state with |A(J)| = 1 and delta_(I,b,J) = 0, and it is COUNTED, NOT SKIPPED — matching rung one. Consistency with rung one is the reason, and the alternative is not wrong, it is merely a different object; two objects with one name is how a chapter goes bad. Theta_(I,b) and the early-terminal mass p^term_(I,b) are ASSERTED ZERO at grade 4 and the assertion is contentful (SR-R2)."
    );
    let _ = writeln!(
        out,
        "STOP RULE (freeze 38(e), and §6.3's STRICTLY STRONGER rule on top of it): the zero-tax test is Corollary 5.2, computed from COMPLETE argmax sets; freeze 26's least-domino-index tie rule is NOT used anywhere in this file. FT-A8 bars a TIE-BROKEN optimiser in favour of the complete optimal face; §6.3 says THE COMPLETE OPTIMAL FACE IS ALSO NOT ENOUGH AT RUNG TWO, because an escape action outside it may carry the minimum. BOTH BIND, AND THEY ARE DIFFERENT RULES."
    );
    let _ = writeln!(
        out,
        "CONVENTION (freeze 38(f), Corollary SR-conv): every evaluator here runs in the trick DIFFERENTIAL; EVERY REPORTED COLUMN is in the COUNT convention, reached by the exact inverse of freeze 26's bridge Q_diff = 2*Q_count - grade. THE TWO BRIDGES ARE DIFFERENT AND ARE KEPT SEPARATE. A DIFFERENCE at a common state (s, d, delta, Delta_I^(2), Delta^(2), Delta^(1)) loses the additive term exactly and IS EXACTLY TWICE ITS COUNT VALUE. A p-WEIGHTED VALUE (F^(1), F^(2), M_I, C, A) maps as x -> alpha*x + c*p and is bridged as (x_diff + grade*p)/2. A RUNG-TWO TAX QUOTED IN ONE CONVENTION AGAINST A RUNG-ONE TAX, A MARGIN OR A FILED Delta^(2) IN THE OTHER IS VOID, and this is the failure mode most likely to produce a near-miss that looks like a discovery."
    );
    let _ = writeln!(
        out,
        "BELIEF AND FIELD ARE NOT RE-DECLARED (freeze 51(f)): freeze 26 and freeze 37(d), cited unchanged, uniform over the full enumerated fiber, NO DECIMATION anywhere inside any L, U, s, d or delta ((C2)). NO LIBRARY ENTRY IS WRITTEN AT ANY COORDINATE (freeze 45). The freeze-set digest travels on every record; a digest mismatch is corruption and the cache is discarded entire (freeze 41, DS-A30) — no cache is read or written here, so that rule is stated rather than exercised."
    );
    let _ = writeln!(
        out,
        "BUDGET HONESTY (freeze 44(b) v2, unchanged by SR-A22(iv) — no new constant is fixed): B = {B_WALK} walk-steps per (coordinate, action) for EACH walk-based evaluator; PATH A2 and PATH B2 are two evaluators and carry one budget each; the charge is bag.len() at entry, taken before any child call; on exhaustion None and NO PARTIAL FOLD of any kind is retained, which here means no partial s, no partial d, no partial Delta^(2) and no partial U^(2). sum_(I,b) |I_2(I,b)| is asserted against P_max v2 = {P_MAX} BEFORE the aggregate pass and the assertion is contentful. Residuals are printed per unit."
    );
    let _ = writeln!(
        out,
        "THE FOUR ENGINE CHANGES (SR-A22(ii)), EVERY ONE ASSERTED IN-RUN AND NEVER ASSUMED: (1) the arrival denominator accumulates THROUGH the second frontier instead of freezing at the first — asserted by den_2 being a multiple of den_1 at every second-frontier arrival; (2) the banked prefix likewise carries the between-frontier increment — asserted by the pre-frontier-1 increment being exactly one trick and the between-frontier increment being exactly one more; (3) seen_focal is a DEPTH COUNTER, not a bool — asserted by the counter reaching grade - 1 at every terminal, which is simultaneously T_0 = 0 and Theta_(I,b) = 0; (4) the depth-two common denominator is 12^12 = SCALE, DEN_MU = 12^6 covering only pre-frontier-1 — asserted equal to SCALE, asserted to be DEN_MU squared, and asserted to divide exactly at every arrival."
    );
    let _ = writeln!(
        out,
        "WHAT THE MACHINERY ALREADY HAD (SR-A22(ii)): the rung-one walk already descends into EVERY action at the first frontier, so the depth-two probe adds RECORDING, NOT SEARCH; and FrontierState::acc_q[j] IS F^(1)_(I,b_j), so the entire slack column s_(I,b) of Theorem 6.2 was already inside the rung-one pass and had only to be printed."
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

    // ---- (SR-R9), first and BLOCKING ---------------------------------------
    let _ = writeln!(
        out,
        "== (SR-R9) THE REDUCED-GRADE CROSS-CHECK — BLOCKING, run before any carrier number exists =="
    );
    let _ = writeln!(
        out,
        "  It tests the LEMMAS and not just the code. At a grade-3 coordinate the focal seat has two decisions after the root and the second is FORCED, so the second frontier is ENTIRELY FORCED: every |A(J)| = 1, every delta_(I,b,J) = 0, every F^(2)_(I,b) = F^(1)_(I,b), Delta^(2) = 0 and U^(2) = U^(1) = Q^H — the last against the ENGINE'S OWN H OPERATOR, an independent evaluator the grade-4 carrier cannot consult. A nonzero grade-3 Delta^(2) falsifies Lemma FT-trunc, Lemma SR-forced or the implementation, and either is stop-and-report. It is the only check in the build that exercises the frontier-2 detector against a case whose answer is known BY PROOF rather than by a filed number, and freeze 51(c)'s counted-forced-J convention is exactly what it exercises."
    );
    reduced_grade_check(&mut out);
    let _ = writeln!(
        out,
        "  (SR-R9) HELD at the declared grade-3 coordinate. The carrier pass follows."
    );
    let _ = writeln!(out);

    // ---- the carrier pass, arm by arm --------------------------------------
    let receipt: Receipt = {
        let path =
            locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
        parse_file(&path).expect("the receipt parses")
    };

    let units = carrier_units();
    let _ = writeln!(
        out,
        "== THE CARRIER — FREEZE 51(a), enumerated with NO GENERATING RULE (FT-A23: a freeze is a constant, not a rule) =="
    );
    let _ = writeln!(
        out,
        "  ARM 1, MANDATORY: coordinate h2, pip 5, hand [21 33 53 54], BOTH freeze-50 units — competitor a = 53 then competitor a = 54, in that order. ARM 2, ATTEMPTED AFTER ARM 1 COMPLETES, WITH A DECLARED STOP: coordinate h9, pip 4, hand [30 41 54 61], units a = 41 then a = 54. h0, h6 and h12 are OUT OF SCOPE for this build."
    );
    let _ = writeln!(
        out,
        "  WHY h2 FIRST AND h9 SECOND (freeze 51(b)): h2 is the smallest first frontier in the carrier (330 states, 554,400 arrivals) and is the coordinate the received note itself nominates; h9 is the second smallest (1,320 states, 2,217,600 arrivals), carries the branch's largest exact negative, and is the coordinate the exact primal route CANNOT PRICE — so a rung-two U^(2) there is a second independent check on a Q^H that has been computed once. h9's NOT PRICED label stands verbatim and is not weakened by this run."
    );
    let _ = writeln!(
        out,
        "  Coordinate identity is asserted first in freeze 45's form at every unit — grade, declaration, hand and pool as canonical ascending domino-index tile lists, leader offset 0, |X| = 34,650 against kernel.count(), freeze-7/23 enumeration order, THE KERNEL REBUILT IN-RUN and asserted equal. W = 2 threads per arm (recorded, never frozen; DS-A34/N4-A17(e)): every unit's content is a function of (kernel, freeze-44 budgets) alone and is byte-identical at any W; results are assembled in canonical unit order, never completion order (DS-A36)."
    );

    let texts: Mutex<BTreeMap<usize, UnitText>> = Mutex::new(BTreeMap::new());
    let mut arm1_completed = true;
    for arm in [1usize, 2usize] {
        if arm == 2 && !arm1_completed {
            let _ = writeln!(
                out,
                "\n== ARM 2 NOT ATTEMPTED (freeze 51(a), SR-A24(e)): arm 1 did not complete, and arm 2 is attempted only after arm 1 completes. This is a declared stop and never a finding (R-A18). =="
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
                        SR_FILED[units_ref[ui].coord].0,
                        tile(units_ref[ui].action),
                        t.elapsed().as_millis()
                    );
                    texts_ref.lock().expect("lock").insert(ui, text);
                });
            }
        });
        if arm == 1 {
            let guard = texts.lock().expect("lock");
            arm1_completed = idx.iter().all(|i| !guard[i].stopped);
        }
    }
    let texts = texts.into_inner().expect("lock");

    let mut companion = String::new();
    let _ = writeln!(
        companion,
        "walt second-rung probe — COMPANION per-(I,b,J) table (SR-A22(iii)(d)) — EXPLORATORY TIER"
    );
    let _ = writeln!(
        companion,
        "This file is REGENERABLE and NOT COMMITTED. It is a deterministic function of committed inputs and contains no row that carries a claim; the committed results file carries its SHA-256, its byte and line counts, and per-unit accounting integers that make every row omitted from it auditable."
    );
    let _ = writeln!(
        companion,
        "regenerate: cargo run --release -p walt-factory --example second_rung"
    );
    let _ = writeln!(companion, "freeze-set digest (freeze 51(h)): {SR_DIGEST}");
    let _ = writeln!(
        companion,
        "columns per (I,b,J): J's record, its parent (I,b), p_(I,b,J), |X_J|, |A(J)|, C_(I,b,J), EVERY A_(I,b,J,c), delta_(I,b,J), the COMPLETE argmax_c set, and — where delta > 0 — the minimal fusion core. COUNT convention throughout, with Corollary SR-conv's two bridges kept separate."
    );
    for ui in 0..units.len() {
        let Some(t) = texts.get(&ui) else { continue };
        let _ = write!(out, "{}", t.named);
        let _ = writeln!(companion);
        for row in &t.companion {
            let _ = writeln!(companion, "{row}");
        }
    }

    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== WHAT THIS BUILD OWES SR-A25(v), LISTED SO IT IS NOT LOST AND NOT DISCHARGEABLE HERE =="
    );
    let _ = writeln!(
        out,
        "  Freeze 51 (SR-A22(iii)) and freeze 38 v1.1(d) (SR-A21(ii)) are NEW and are not in the wiki's freeze register; the claim-ledger, FINDINGS and open-problems cross-references for this adjudication are likewise owed to that page's owner. DS-A28(ii) remains CARRIED, and the errata §9 queue now also carries Lemma SR-coord, Lemma SR-forced, Proposition SR-sep, Proposition SR-post, Corollary SR-conv, Proposition SR-degen and Proposition SR-taut. Until that amendment, walt/CENSUS-RULINGS.md is their only authority."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "total wall-clock (provenance only, never a dividend; DS-A31/DS-A36): {} ms",
        t0.elapsed().as_millis()
    );
    let _ = writeln!(out, "run complete: yes");

    // The companion is written FIRST, so the named file's header can carry
    // its SHA-256, byte count and line count.
    let companion_path = out_dir("results").join("second_rung_frontier_2026-08-14.txt");
    std::fs::write(&companion_path, &companion).expect("write companion");
    let digest = sha256_hex(companion.as_bytes());
    let companion_line = format!(
        "companion (SR-A22(iii)(d), REGENERABLE and NOT COMMITTED — a deterministic function of committed inputs, carrying one row per (I,b,J)): results/second_rung_frontier_2026-08-14.txt  SHA-256 {digest}  {} bytes, {} lines. \"Reproducible from the repository alone\" is satisfied by the deterministic regeneration command above plus this digest.",
        companion.len(),
        companion.lines().count()
    );
    assert!(
        out.contains(COMPANION_LINE_PLACEHOLDER),
        "the header carries exactly one companion marker"
    );
    let out = out.replace(COMPANION_LINE_PLACEHOLDER, &companion_line);

    let results = out_dir("results").join("second_rung_2026-08-14.txt");
    std::fs::write(&results, &out).expect("write results");
    print!("{}", &out[..out.len().min(4000)]);
    println!("results: {}", results.display());
    println!("companion: {} ({digest})", companion_path.display());
}
