//! walt fusion-tax probe — Experiment 15.1 of the FT family: the EXACT
//! first-layer fusion tax over the freeze-50 carrier. EXPLORATORY TIER
//! THROUGHOUT, without exception.
//!
//! Rulings: FT-A1..FT-A27 (`walt/CENSUS-RULINGS.md`, 2026-08-14), with
//! Lemma FT-arrive (the frontier arrival law is policy-independent),
//! Lemma FT-trunc (the ladder truncates one rung early), Corollary FT-grade4
//! (at grade 4 the ladder has exactly two rungs), Proposition FT-flat,
//! Proposition FT-tie, Lemma FT-post, Corollary FT-conv and Lemma FT-mix
//! (heterogeneous upper witnesses compose). Freezes:
//! 38 v1 (the gluing cut — cut language, canonical family, cut ordering,
//! stop rule, arithmetic and reporting), 44 v2 (walk-step unit and budgeted
//! walk contract), 45 (the n = 4 coordinate identity), 50 v1.1 (this probe's
//! carrier and its emission layout). Mathematics under DS-A17's citation
//! rule: Lemma E3 with (C1)-(C4), Lemma E4, Corollary E4.1, Theorem E6.4
//! (member-not-set), Theorem E6.5 (cuts, exposed-face stop rule).
//!
//! SECOND EMISSION (freeze 50 v1.1(c), FT-A24). The emission is cut by
//! CONTENT and not by size: the named results file carries every frontier
//! row with `delta_I > 0` — the support of the tax, which is the whole
//! content of Experiment 15.2 — with the complete (FT-R5) material, every
//! receipt, every summary, every pair verdict and every header and fence;
//! the FULL frontier table including the zero rows goes to the companion
//! file, which is REGENERABLE and NOT COMMITTED. Four accounting integers
//! per unit make the omission auditable. The split is produced HERE, by the
//! probe's own emitter in a re-run — never by post-processing emitted text,
//! which is not a machine-readable interface (SEP-A14(ii)'s principle).
//! (FT-R7) is the determinism receipt that re-run buys.
//!
//! What it computes, per carrier coordinate and per binding competitor
//! action `a` (FT-A18(i)): the frontier data `mu_I(omega)` and
//! `q_I(omega,b)`, the local taxes `delta_I`, the layer tax
//! `Delta_a^(1) = sum_I delta_I`, `U_a^(1) = U_a^C - Delta_a^(1)`, the
//! decision question `L_{a*} >= U_a^(1)`, and — free by Corollary FT-grade4
//! — the residual `Delta_a^(2) = U_a^(1) - Q^H(a)`.
//!
//! Two independently written computations of `U^(1)` are carried (FT-R4):
//! PATH A is world-major — one revealed walk per world, recording the
//! per-action child values at the focal seat's FIRST decision below the
//! root, folded into a record-keyed frontier table; PATH B is the
//! glue-one-then-reveal walker — a pooled bag carried to the frontier,
//! lawful there (`max_b` OUTSIDE the world sum), world-informed below it.
//! They share the rule algebra and nothing else.
//!
//! Arithmetic (freeze 38(f)): exact integers and rationals only, no float
//! anywhere (P-A19). The deep per-world solve runs on i64 values scaled by
//! `SCALE = 12^12`; every field average divides exactly because a legal set
//! at these grades has size at most 4, every such size divides 12, and at
//! most 12 field plies remain below the root action — the exactness is
//! asserted at every field node, never assumed. The frontier accumulation
//! runs on i128 over the common denominator `DEN_MU = 12^6`, which every
//! pre-frontier arrival denominator divides.
//!
//! Filed `U` and `Q^H` enter as a FROZEN SOURCE TABLE in this source
//! (FT-A18(v), the SEP-A14(ii) pattern), quoted from
//! `separation_n4_2026-08-14.txt`, exploratory tier — never re-parsed from
//! results text at run time.
//!
//! No floats. Regenerate:
//! `cargo run --release -p walt-factory --example fusion_tax`

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
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

/// Freeze 44(e) v2: B walk-steps per (coordinate, action) for each
/// walk-based evaluator. PATH A and PATH B are two evaluators and carry one
/// budget each; charge-then-descend at `bag.len()` on entry; on exhaustion
/// `None` and NO PARTIAL FOLD of any kind is retained — which here means
/// there is no partial tax and no partial `U^(1)` (FT-A4(iii), FT-A18(iii)).
const B_WALK: u64 = 10_000_000_000;

/// Freeze 44(e) v2: the partition-state cap. The frontier partition is a
/// depth-one object and is asserted against it; the assertion is contentful.
const P_MAX: u64 = 192_000_000;

const N4_TRICK: usize = 4;
const N4_GRADE: usize = 4;
const N4_FIBER: u128 = 34_650;

/// Freeze 50 v1.1(c)(vi), FT-A24(vi): the declared cap, so a future run
/// cannot smuggle a truncation. If one unit's `delta_I > 0` support ever
/// exceeds this many rows, the named file carries the top rows by descending
/// `delta_I`, ties by ascending record order, plus the residual tail's exact
/// count and exact summed `delta` — a DECLARED truncation printed in place,
/// never a silent one, with the sum assertion restated over the full support.
const ROW_CAP: usize = 20_000;

/// The named file's header carries the companion's SHA-256 (FT-A24(v)), which
/// is only known after the companion is written. The header is assembled
/// first, so it carries this marker and the marker is substituted once, at
/// write time, by the real line.
const COMPANION_LINE_PLACEHOLDER: &str = "@@COMPANION@@";

/// Freeze 50(g): the freeze-set digest travelling on every record. No cache
/// is read or written by this probe, so freeze 41/DS-A30's discard rule is
/// vacuous here and is stated rather than exercised.
const FT_DIGEST: &str =
    "FT-v1|freezes-7-23-26-37d-38v1-44v2-45-50|contract=R-A11-full-record|field=uniform-legal-F4|belief=uniform-fiber-freeze7";

/// `12^12`. The deep solve's fixed scale: a node with `j` field plies below
/// it has a value whose denominator divides `12^j`, and at most 12 field
/// plies remain below a grade-4 root action.
const SCALE: i64 = 8_916_100_448_256;

/// `12^6`. The common denominator of every pre-frontier arrival weight: at
/// most 6 field plies separate the root action from the focal seat's next
/// decision (three to finish the root trick, at most three in the next).
const DEN_MU: i128 = 2_985_984;

// -- the frozen source table (FT-A18(v), the SEP-A14(ii) pattern) ------------

/// One filed root-action row: `(hi, lo, Q^H as (num, den), U as (num, den),
/// revealed walk-steps)`, all in the COUNT convention for the two values.
type FiledRow = (u8, u8, (i128, i128), (i128, i128), u64);

/// One filed carrier coordinate: `(corpus hand id, declaration pip, the four
/// root-action rows in ascending domino index, whether the filed run
/// EXHIBITED a primal witness at this coordinate)`.
type FiledCoord = (usize, u8, [FiledRow; 4], bool);

/// FREEZE 50(a): the carrier is the five negative-margin n = 4 coordinates,
/// in the canonical order the ruling enumerates. Quoted from
/// `walt-factory/results/separation_n4_2026-08-14.txt`, exploratory tier;
/// carried as a frozen table in this source and NEVER re-parsed from results
/// text at run time (FT-A18(v), the SEP-A14(ii) pattern). The `l_exhibited`
/// flag is quoted from the same file's per-coordinate `a*` lines: four
/// coordinates carry a filed `R2` primal receipt `L = Q^H`; h9 is NOT PRICED
/// (517,562,322 partition states against P_max v2) and exhibits no primal
/// witness at all, so `L_{a*} = Q^H(a*)` there is Corollary E4.1(2)'s
/// CEILING and not a receipted witness. That distinction is printed in place
/// on every h9 row (FT-A18(iv): h9's NOT PRICED label stands verbatim).
const FT_FILED: [FiledCoord; 5] = [
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
        true,
    ),
    (
        6,
        4,
        [
            (
                1,
                1,
                (535_997_311, 239_500_800),
                (1_090_848_503, 479_001_600),
                521_846_350,
            ),
            (
                4,
                0,
                (541_161_923, 239_500_800),
                (544_949_941, 239_500_800),
                327_214_878,
            ),
            (
                4,
                3,
                (256_988_827, 119_750_400),
                (5_881_639, 2_721_600),
                339_888_954,
            ),
            (
                5,
                3,
                (220_757, 110_880),
                (122_035_561, 59_875_200),
                666_469_784,
            ),
        ],
        true,
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
    (
        12,
        0,
        [
            (
                2,
                0,
                (2_360_209, 855_360),
                (41_850_301, 14_968_800),
                786_443_140,
            ),
            (
                3,
                0,
                (2_360_209, 855_360),
                (41_850_301, 14_968_800),
                786_443_140,
            ),
            (
                4,
                0,
                (2_360_209, 855_360),
                (41_850_301, 14_968_800),
                786_443_140,
            ),
            (6, 5, (128_344, 51_975), (128_344, 51_975), 1_307_478_624),
        ],
        true,
    ),
];

/// Proposition FT-tie's table, recomputed at adjudication time from the same
/// filed rows: `(required shave U_a - L_{a*}, the competitor's own fusion gap
/// U_a - Q^H(a), the fraction required)`, count convention, one entry per
/// carrier coordinate in freeze-50(a) order. Quoted from
/// `walt/CENSUS-RULINGS.md`, Proposition FT-tie, exploratory tier — carried
/// here as a CROSS-CHECK EXPECTATION which the probe recomputes from the
/// frozen source table and asserts, never as an input to any value.
type TieRow = ((i128, i128), (i128, i128), (i128, i128));
const FT_TIE_QUOTED: [TieRow; 5] = [
    (
        (300_647, 2_138_400),
        (16_709_317, 89_812_800),
        (12_627_174, 16_709_317),
    ),
    (
        (8_524_657, 479_001_600),
        (6_284_627, 159_667_200),
        (8_524_657, 18_853_881),
    ),
    ((9_557, 554_400), (9_557, 554_400), (1, 1)),
    ((2_116_837, 8_870_400), (2_116_837, 8_870_400), (1, 1)),
    ((364_429, 9_979_200), (364_429, 9_979_200), (1, 1)),
];

/// One unit's first-emission record: `(corpus hand id, a hi, a lo, frontier
/// states, (state, world) arrivals, states with delta_I = 0, states with
/// delta_I > 0, Delta^(1), U^(1), Delta^(2), walk-steps charged by EACH
/// path)`. The three rationals are count convention, as `(num, den)`.
type FirstUnit = (
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
);

/// (FT-R7) frozen source — THE FIRST EMISSION. Quoted from
/// `walt/CENSUS-RULINGS.md`'s closing note "the probe returned" and
/// FT-A24(ii)'s frontier row census, which walt-math re-verified
/// independently of the run at adjudication time; exploratory tier. Carried
/// as a frozen table in this source and NEVER re-parsed from the first
/// emission's results text, which is not a machine-readable interface
/// (SEP-A14(ii)'s principle, which is also why the split is emitted here
/// rather than post-processed).
///
/// What this half of (FT-R7) tests and what it does not: it compares the
/// present process against a PRIOR PROCESS on every per-unit summary value,
/// every receipt status and the frontier row census. It does not reach the
/// individual `delta_I` values, because those exist in the first emission
/// only as results text. The in-run second pass supplies that half at full
/// strength; both halves are printed with their scope named.
const FT_FIRST: [FirstUnit; 9] = [
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
        (387_281, 5_132_160),
        539_583_224,
    ),
    (
        6,
        1,
        1,
        53_570,
        1_868_040,
        49_529,
        4_041,
        (611_579, 21_772_800),
        (10_260_893, 4_561_920),
        (5_399_143, 479_001_600),
        521_846_350,
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
        (1_483, 138_600),
        1_297_073_736,
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
    ),
    (
        12,
        2,
        0,
        69_512,
        2_311_560,
        68_098,
        1_414,
        (34_519, 1_995_840),
        (83_182_817, 29_937_600),
        (95_917, 4_989_600),
        786_443_140,
    ),
    (
        12,
        3,
        0,
        69_512,
        2_311_560,
        68_098,
        1_414,
        (34_519, 1_995_840),
        (83_182_817, 29_937_600),
        (95_917, 4_989_600),
        786_443_140,
    ),
    (
        12,
        4,
        0,
        69_512,
        2_311_560,
        68_098,
        1_414,
        (34_519, 1_995_840),
        (83_182_817, 29_937_600),
        (95_917, 4_989_600),
        786_443_140,
    ),
];

/// (FT-R8) cross-check expectation: FT-A25(iv)'s three surpluses at h6, in
/// competitor order 11, 43, 53, count convention. Quoted from
/// `walt/CENSUS-RULINGS.md`, FT-A25(iv), exploratory tier — recomputed in-run
/// from the frozen source table and asserted, never an input to any value.
const H6_SURPLUS_QUOTED: [(i128, i128); 3] = [
    (4_930_081, 479_001_600),
    (23_577_691, 239_500_800),
    (53_019_679, 239_500_800),
];

/// (FT-R6) frozen source at the declared grade-3 coordinate: the S6a filed
/// `Q^H` rows at base index 0, count convention, quoted from
/// `predictive_rank_2026-08-12.txt`, S6a, exploratory tier (the SEP-A14(ii)
/// pattern, as `separation_probe.rs` carries them).
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

fn to_diff(count: Q, grade: usize) -> Q {
    count * qi(2) - qi(i128::try_from(grade).expect("grade fits"))
}

/// Corollary FT-conv, in the one direction freeze 38(f) mandates: an
/// internally-computed DIFFERENTIAL tax is exactly twice its count-convention
/// value. A tax quoted in one convention against a margin in the other is
/// void, so this function is the only route a tax takes to a report.
fn tax_to_count(diff_tax: Q) -> Q {
    diff_tax * q(1, 2)
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

/// The SHA-256 digest of `data`, lowercase hex. Pure `u32` wrapping integer
/// arithmetic (P-A19: no float anywhere near it). Used only to carry the
/// uncommitted companion file's identity in the committed named file's
/// header, per FT-A24(v).
fn sha256_hex(data: &[u8]) -> String {
    let mut h: [u32; 8] = [
        0x6a09_e667,
        0xbb67_ae85,
        0x3c6e_f372,
        0xa54f_f53a,
        0x510e_527f,
        0x9b05_688c,
        0x1f83_d9ab,
        0x5be0_cd19,
    ];
    let bitlen = (data.len() as u64).wrapping_mul(8);
    let mut msg = data.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bitlen.to_be_bytes());
    for chunk in msg.chunks_exact(64) {
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
        let mut v = h;
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
        for (slot, add) in h.iter_mut().zip(v) {
            *slot = slot.wrapping_add(add);
        }
    }
    let mut out = String::with_capacity(64);
    for word in h {
        let _ = write!(out, "{word:08x}");
    }
    out
}

/// The FIPS 180-4 known-answer self-check, run before any digest is reported.
/// A digest primitive that has never been checked against a published vector
/// is not a receipt of anything.
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
/// unranking), used only by (FT-R6).
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

// -- the rule algebra: one node, two walkers ---------------------------------

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
    /// per node entry — the single-world bag's `bag.len()` — charged before
    /// any child call; `None` on exhaustion with no partial fold retained.
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

// -- PATH A: the world-major frontier table ----------------------------------

/// One frontier information state's accumulated exact data. The record — the
/// plays since the kernel decision point with the root action first (freeze
/// 26's observation contract, freeze 36(b)) — is the map key and IS the
/// information state; two records of different length are two different
/// states, which is why the frontier states are mutually exclusive and the
/// local taxes add (FT-A7(iii)-(iv)).
struct FrontierState {
    /// `A(I)`, asserted constant across the latent worlds of `I` (FT-A7(ii):
    /// an engine fact — `legal_plays` reads only `(decl, hand, led)`).
    legal: DominoSet,
    /// The scaled increment banked before the frontier. A function of the
    /// record alone (the tricks it completes are determined by it), asserted
    /// equal on every arrival.
    prefix: i64,
    /// `|X_I|`: latent worlds arriving here. Each world reaches a given
    /// record at most once — the record fixes the field plays — so this is
    /// both the arrival count and the world count.
    n_worlds: u64,
    /// `sum_omega DEN_MU/den`, i.e. `p_I * DEN_MU * |X|`.
    acc_p: i128,
    /// `sum_omega (DEN_MU/den) * m_I(omega)`, scaled by `SCALE`.
    acc_m: i128,
    /// `sum_omega (DEN_MU/den) * q_I(omega,b)` per action of `legal` in
    /// ascending domino index (freeze 38(d)), scaled by `SCALE`.
    acc_q: Vec<i128>,
    /// The running intersection of the complete per-world argmax sets. Its
    /// emptiness is Corollary 6.4's criterion, computed from COMPLETE argmax
    /// sets and never from freeze 26's least-index tie rule (FT-A8).
    inter: DominoSet,
    /// The greedy strictly-shrinking chain of worlds: at most `|A(I)|` of
    /// them, and the search space of the minimal fusion core.
    chain: Vec<(u32, DominoSet)>,
}

impl FrontierState {
    fn new(legal: DominoSet, prefix: i64) -> FrontierState {
        FrontierState {
            legal,
            prefix,
            n_worlds: 0,
            acc_p: 0,
            acc_m: 0,
            acc_q: vec![0; legal.len()],
            inter: legal,
            chain: Vec::new(),
        }
    }

    fn observe(&mut self, wi: u32, mask: DominoSet) {
        let next = self.inter.intersection(mask);
        if next != self.inter {
            self.chain.push((wi, mask));
            self.inter = next;
        }
    }

    /// `max_b sum_omega mu_I(omega) q_I(omega,b)`, unnormalized.
    fn best_q(&self) -> i128 {
        *self.acc_q.iter().max().expect("A(I) is nonempty")
    }

    /// The COMPLETE argmax set of `sum_omega mu_I(omega) q_I(omega,b)`
    /// (freeze 38(e): tie sets are complete, never least-index broken).
    fn argmax_set(&self) -> DominoSet {
        let best = self.best_q();
        let mut out = DominoSet::EMPTY;
        for (j, d) in self.legal.iter().enumerate() {
            if self.acc_q[j] == best {
                out.insert(d);
            }
        }
        out
    }

    /// A MINIMAL fusion core: a subset of `X_I` whose complete argmax sets
    /// have empty intersection while every proper subset's is nonempty
    /// (Proposition 8.2 bounds it by `|A(I)|`, at most 3 at the grade-4
    /// frontier). `None` when the state has a common optimum.
    fn minimal_core(&self) -> Option<Vec<(u32, DominoSet)>> {
        if !self.inter.is_empty() {
            return None;
        }
        let n = self.chain.len();
        assert!(n <= self.legal.len(), "Proposition 8.2: a core is small");
        let mut best: Option<Vec<(u32, DominoSet)>> = None;
        for bits in 1u32..(1u32 << n) {
            let mut inter = self.legal;
            let mut sub = Vec::new();
            for (j, entry) in self.chain.iter().enumerate() {
                if bits & (1u32 << j) != 0 {
                    inter = inter.intersection(entry.1);
                    sub.push(*entry);
                }
            }
            if inter.is_empty() && best.as_ref().is_none_or(|b| sub.len() < b.len()) {
                best = Some(sub);
            }
        }
        let core = best.expect("the whole chain already has an empty intersection");
        // (FT-R5): empty at the core, nonempty at every proper subset.
        let mut inter = self.legal;
        for (_, m) in &core {
            inter = inter.intersection(*m);
        }
        assert!(
            inter.is_empty(),
            "(FT-R5) stop-and-report: the reported fusion core has a common optimum"
        );
        for skip in 0..core.len() {
            let mut sub = self.legal;
            for (j, (_, m)) in core.iter().enumerate() {
                if j != skip {
                    sub = sub.intersection(*m);
                }
            }
            assert!(
                !sub.is_empty(),
                "(FT-R5) stop-and-report: the reported fusion core is not minimal"
            );
        }
        Some(core)
    }
}

/// The pre-frontier data a PATH A descent carries: the arrival denominator
/// (the product of the field legal-set sizes so far — Lemma FT-arrive's
/// product form), the increment banked so far, and whether the focal seat has
/// already acted below the root.
#[derive(Clone, Copy)]
struct Arrival {
    den: u64,
    prefix: i64,
    seen_focal: bool,
}

struct Recorder<'a> {
    ctx: Ctx,
    wi: u32,
    states: &'a mut BTreeMap<Vec<Domino>, FrontierState>,
}

impl Recorder<'_> {
    /// The same revealed walk as `Ctx::rev`, recording at the focal seat's
    /// FIRST decision below the root the complete per-action child values —
    /// which are `q_I(omega,b)` less the record's banked increment. Returns
    /// the subtree value so the whole descent still folds the per-world
    /// revealed value, which the caller cross-checks against the frontier
    /// accumulation.
    fn walk(
        &mut self,
        node: Node,
        arr: Arrival,
        obs: &mut Vec<Domino>,
        budget: &mut u64,
    ) -> Option<i64> {
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
                    "(FT-R2) stop-and-report: T_a = 0 fails — a positive-mass world reached the end of the hand with no further focal decision"
                );
                return Some(inc);
            }
            let next = Node {
                hands: node.hands,
                leader: winner,
                tiles: [Domino::ALL[0]; 4],
                k: 0,
            };
            let below = Arrival {
                prefix: if arr.seen_focal {
                    arr.prefix
                } else {
                    arr.prefix + inc
                },
                ..arr
            };
            return Some(inc + self.walk(next, below, obs, budget)?);
        }
        let seat = node.seat();
        let legal = self.ctx.legal_at(node, seat);
        if seat == self.ctx.focal {
            if arr.seen_focal {
                let mut best = i64::MIN;
                for d in legal.iter() {
                    obs.push(d);
                    let v = self.walk(node.child(seat, d), arr, obs, budget);
                    obs.pop();
                    let v = v?;
                    if v > best {
                        best = v;
                    }
                }
                return Some(best);
            }
            // The frontier: the focal seat's next decision after the root.
            let below = Arrival {
                seen_focal: true,
                ..arr
            };
            assert!(
                legal.len() <= 4,
                "Proposition 8.2, sharpened: |A(I)| is bounded by the tiles in hand at that decision"
            );
            let mut child = [0i64; 4];
            let mut best = i64::MIN;
            for (j, d) in legal.iter().enumerate() {
                obs.push(d);
                let v = self.walk(node.child(seat, d), below, obs, budget);
                obs.pop();
                let v = v?;
                child[j] = v;
                if v > best {
                    best = v;
                }
            }
            let entry = match self.states.get_mut(obs.as_slice()) {
                Some(e) => e,
                None => self
                    .states
                    .entry(obs.clone())
                    .or_insert_with(|| FrontierState::new(legal, arr.prefix)),
            };
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
                "freeze 38(f): DEN_MU = 12^6 carries every pre-frontier arrival denominator"
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
            return Some(best);
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
            obs.push(d);
            let v = self.walk(node.child(seat, d), below, obs, budget);
            obs.pop();
            sum += v?;
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

/// PATH A's result at one (coordinate, root action).
struct PathA {
    states: BTreeMap<Vec<Domino>, FrontierState>,
    /// `sum_omega V_revealed(omega, a)` scaled by `SCALE` — the per-world
    /// fold, kept for the arithmetic identity against the frontier table.
    world_fold: i128,
    steps: u64,
    residual: u64,
}

fn path_a(ctx: Ctx, worlds: &[[DominoSet; Seat::COUNT]], root: Domino) -> Option<PathA> {
    let mut states: BTreeMap<Vec<Domino>, FrontierState> = BTreeMap::new();
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
        let mut obs = Vec::with_capacity(16);
        obs.push(root);
        let mut rec = Recorder {
            ctx,
            wi: u32::try_from(wi).expect("fiber index fits u32"),
            states: &mut states,
        };
        let v = rec.walk(node, arr, &mut obs, &mut budget)?;
        assert_eq!(obs.len(), 1, "the record stack unwinds to the root action");
        world_fold += i128::from(v);
    }
    Some(PathA {
        states,
        world_fold,
        steps: B_WALK - budget,
        residual: budget,
    })
}

// -- PATH B: the glue-one-then-reveal walker ---------------------------------

/// One pooled particle of the glue walk: a fiber world, its arrival
/// denominator, and its remaining hands. The weight the field assigns is
/// `1/den` (Lemma FT-arrive's product form) and the belief factor `1/|X|` is
/// applied once at the end.
#[derive(Clone, Copy)]
struct Pooled {
    wi: u32,
    den: u64,
    hands: [DominoSet; Seat::COUNT],
}

struct Glue {
    ctx: Ctx,
}

impl Glue {
    /// The C^(1) value below `node`: LAWFUL at the focal seat's first
    /// decision below the root — one common action per information state,
    /// `max_b` taken OUTSIDE the world sum, which is the whole content of
    /// first-layer gluing — and world-informed below it. Returns
    /// `sum_omega (DEN_MU/den) * (prefix + q)` scaled by `SCALE`, i.e.
    /// `U^(1)` less the belief factor `1/|X|`.
    ///
    /// Budgeted (freeze 44(a)-(b)): the charge at entry is the pooled bag's
    /// `bag.len()`, taken before any child call; the revealed continuations
    /// below the frontier charge one step per node.
    fn walk(
        &self,
        support: &[Pooled],
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        prefix: i64,
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
                "(FT-R2) stop-and-report: T_a = 0 fails — the hand ended before the focal seat acted again"
            );
            return self.walk(
                support,
                winner,
                [Domino::ALL[0]; 4],
                0,
                prefix + inc,
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
                    let w = DEN_MU / i128::from(p.den);
                    assert_eq!(
                        w * i128::from(p.den),
                        DEN_MU,
                        "freeze 38(f): DEN_MU = 12^6 carries every pre-frontier arrival denominator"
                    );
                    acc += w * i128::from(prefix + v);
                }
                best = Some(best.map_or(acc, |b: i128| b.max(acc)));
            }
            return Some(best.expect("A(I) is nonempty"));
        }
        let mut by_tile: BTreeMap<usize, Vec<Pooled>> = BTreeMap::new();
        for p in support {
            let legal = legal_plays(self.ctx.decl, p.hands[seat.index()], led);
            let n = u64::try_from(legal.len()).expect("a legal set size fits u64");
            for d in legal.iter() {
                let mut hands = p.hands;
                hands[seat.index()].remove(d);
                by_tile.entry(d.index()).or_default().push(Pooled {
                    wi: p.wi,
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
            sum += self.walk(&sup, leader, t, k + 1, prefix, budget)?;
        }
        Some(sum)
    }
}

struct PathB {
    total: i128,
    steps: u64,
    residual: u64,
}

fn path_b(ctx: Ctx, worlds: &[[DominoSet; Seat::COUNT]], root: Domino) -> Option<PathB> {
    let mut support: Vec<Pooled> = Vec::with_capacity(worlds.len());
    for (wi, hands) in worlds.iter().enumerate() {
        let mut hands = *hands;
        hands[ctx.focal.index()].remove(root);
        support.push(Pooled {
            wi: u32::try_from(wi).expect("fiber index fits u32"),
            den: 1,
            hands,
        });
    }
    let mut tiles = [Domino::ALL[0]; 4];
    tiles[0] = root;
    let mut budget = B_WALK;
    let glue = Glue { ctx };
    let total = glue.walk(&support, ctx.focal, tiles, 1, 0, &mut budget)?;
    Some(PathB {
        total,
        steps: B_WALK - budget,
        residual: budget,
    })
}

// -- one unit: a (coordinate, binding competitor action) --------------------

/// Everything one unit produces. Every field is an exact rational or an
/// exact integer; nothing here is a bound, an estimate or a sample.
struct UnitResult {
    /// EVERY frontier row in freeze-50(c) order (ascending observation
    /// record) — the companion file's content (FT-A24(v)).
    all_rows: Vec<String>,
    /// The rows with `delta_I > 0` — the support of the tax, and the named
    /// file's content (FT-A24(iii)). Held in freeze-50(c) order unless the
    /// declared `ROW_CAP` truncation fires, which it announces in place.
    printed_rows: Vec<String>,
    /// (FT-A24(iv)) the four accounting integers: printed, suppressed split
    /// into forced (`|A(I)| = 1`) and unforced-with-a-common-optimum, and the
    /// exact sum of the printed `delta_I` in the count convention.
    n_printed: u64,
    n_suppressed_forced: u64,
    n_suppressed_common: u64,
    printed_delta_sum: Q,
    /// The declared truncation's residual tail, `None` when the cap did not
    /// fire: `(exact count, exact summed delta in the count convention)`.
    tail: Option<(u64, Q)>,
    /// The exact `delta_I` values of the printed rows in emission order, in
    /// the count convention — (FT-R7)'s in-run comparison material.
    printed_deltas: Vec<Q>,
    n_states: u64,
    n_arrivals: u64,
    mass: Q,
    u0_diff: Q,
    delta1_diff: Q,
    u1_a_diff: Q,
    u1_b_diff: Q,
    zero_tax_states: u64,
    core_states: u64,
    sample: Vec<String>,
    steps_a: u64,
    steps_b: u64,
    residual_a: u64,
    residual_b: u64,
}

/// The declared deterministic (FT-R5) sample: the first ten frontier states
/// in freeze-50(c) order. The assertions themselves are made at EVERY state,
/// which is strictly stronger than the declared sample; the sample is what is
/// printed.
const R5_SAMPLE: usize = 10;

#[allow(clippy::too_many_lines)]
fn run_unit(kernel: &Kernel, grade: usize, root: Domino, u_filed_diff: Q) -> Option<UnitResult> {
    assert_eq!(
        kernel.viewer_hand().len(),
        grade,
        "the declared grade is the coordinate's grade (N4-A11: no grade literal travels without this check)"
    );
    let worlds: Vec<[DominoSet; Seat::COUNT]> = kernel.worlds().map(|w| w.hands()).collect();
    let n_worlds = i128::try_from(worlds.len()).expect("fiber size fits i128");
    let ctx = Ctx {
        decl: kernel.decl(),
        focal: kernel.viewer(),
        team: kernel.viewer().team(),
    };

    let a = path_a(ctx, &worlds, root)?;
    let b = path_b(ctx, &worlds, root)?;

    let n_states = u64::try_from(a.states.len()).expect("state count fits u64");
    assert!(
        n_states <= P_MAX,
        "freeze 44(e) v2: the depth-one frontier partition exceeds P_max v2"
    );

    // The normalizing denominator: the belief factor 1/|X|, the arrival
    // common denominator DEN_MU, and the deep solve's SCALE.
    let norm = DEN_MU * i128::from(SCALE) * n_worlds;

    let mut sum_p: i128 = 0;
    let mut sum_m: i128 = 0;
    let mut sum_best: i128 = 0;
    let mut n_arrivals: u64 = 0;
    let mut zero_tax_states: u64 = 0;
    let mut core_states: u64 = 0;
    let mut all_rows: Vec<String> = Vec::new();
    let mut positive: Vec<(i128, String, Q)> = Vec::new();
    let mut n_suppressed_forced: u64 = 0;
    let mut n_suppressed_common: u64 = 0;
    let mut sample: Vec<String> = Vec::new();

    for (record, st) in &a.states {
        let best_q = st.best_q();
        let dtax = st.acc_m - best_q;
        assert!(
            dtax >= 0,
            "stop-and-report: a local tax is negative (avg-of-max is below max-of-avg)"
        );
        sum_p += st.acc_p;
        sum_m += st.acc_m;
        sum_best += best_q;
        n_arrivals += st.n_worlds;

        // Corollary 6.4, both directions, at EVERY state (FT-A8: computed
        // from COMPLETE argmax sets, never from a tie-broken optimiser).
        let common = !st.inter.is_empty();
        assert_eq!(
            dtax == 0,
            common,
            "(FT-R5) stop-and-report: Corollary 6.4 fails at record [{}]",
            record_str(record)
        );
        let core = st.minimal_core();
        if dtax == 0 {
            zero_tax_states += 1;
            // (FT-A24(iv)) the suppressed split, and it is Corollary 6.4's
            // actual content: a FORCED state pays nothing trivially, while an
            // unforced state paying nothing is Theorem E6.5(G2)'s exposed-face
            // criterion doing real work.
            if st.legal.len() == 1 {
                n_suppressed_forced += 1;
            } else {
                n_suppressed_common += 1;
            }
        } else {
            core_states += 1;
        }

        let p_i = Q::new(st.acc_p, DEN_MU * n_worlds);
        let delta_count = tax_to_count(Q::new(dtax, norm));
        let core_txt = match &core {
            None => String::new(),
            Some(c) => {
                let ws: Vec<String> = c
                    .iter()
                    .map(|(wi, m)| format!("{wi}:{{{}}}", tiles_str(*m)))
                    .collect();
                format!(
                    "  minimal fusion core (fiber indices, freeze 7/23) = [{}]",
                    ws.join(" ")
                )
            }
        };
        let row = format!(
            "    I=[{}]  p_I = {}  |X_I| = {}  |A(I)| = {}  delta_I = {} (count)  argmax{{sum_omega mu q}} = {{{}}}{}",
            record_str(record),
            qs(p_i),
            st.n_worlds,
            st.legal.len(),
            qs(delta_count),
            tiles_str(st.argmax_set()),
            core_txt
        );
        if all_rows.len() < R5_SAMPLE {
            let verdict = if dtax == 0 {
                format!(
                    "delta_I = 0 and the complete argmax sets intersect at {{{}}} — Corollary 6.4 HELD",
                    tiles_str(st.inter)
                )
            } else {
                let c = core.as_ref().expect("a positive tax has a core");
                format!(
                    "delta_I > 0, empty argmax intersection, minimal fusion core of size {} <= |A(I)| = {} — Corollary 6.4 HELD (every proper subset intersects, asserted)",
                    c.len(),
                    st.legal.len()
                )
            };
            sample.push(format!("    I=[{}]: {verdict}", record_str(record)));
        }
        if dtax > 0 {
            positive.push((dtax, row.clone(), delta_count));
        }
        all_rows.push(row);
    }

    // (FT-A24(iv)) and (FT-A24(vi)): the printed support, the declared cap,
    // and the four accounting integers. The cap orders by DESCENDING delta_I
    // with ties by ASCENDING record order — `positive` is already in
    // ascending record order and the sort is stable, so the tie rule holds by
    // construction.
    let full_support = u64::try_from(positive.len()).expect("support fits u64");
    let full_sum: i128 = positive.iter().map(|(d, _, _)| *d).sum();
    assert_eq!(
        full_sum,
        sum_m - sum_best,
        "(FT-A24(iv)(1)) stop-and-report: the summed delta_I over the positive support is not Delta^(1)"
    );
    let mut tail: Option<(u64, Q)> = None;
    if positive.len() > ROW_CAP {
        // Stable sort, so the freeze-38(d) tie rule holds by construction:
        // `positive` is already in ascending record order.
        positive.sort_by_key(|r| std::cmp::Reverse(r.0));
        let cut: i128 = positive[ROW_CAP..].iter().map(|(d, _, _)| *d).sum();
        tail = Some((
            u64::try_from(positive.len() - ROW_CAP).expect("tail fits u64"),
            tax_to_count(Q::new(cut, norm)),
        ));
        positive.truncate(ROW_CAP);
    }
    let n_printed = u64::try_from(positive.len()).expect("printed fits u64");
    let printed_rows: Vec<String> = positive.iter().map(|(_, r, _)| r.clone()).collect();
    let printed_deltas: Vec<Q> = positive.iter().map(|(_, _, d)| *d).collect();
    let printed_delta_sum = tax_to_count(Q::new(
        positive.iter().map(|(d, _, _)| *d).sum::<i128>(),
        norm,
    ));
    let tail_count = tail.map_or(0, |(c, _)| c);
    let tail_sum = tail.map_or(qi(0), |(_, s)| s);
    // (FT-A24(iv)(1)), restated over the FULL support whenever the declared
    // cap fires, so the sum still reconciles.
    assert_eq!(
        printed_delta_sum + tail_sum,
        tax_to_count(Q::new(full_sum, norm)),
        "(FT-A24(iv)(1)) stop-and-report: printed + tail delta_I != Delta^(1)"
    );
    // (FT-A24(iv)(2)).
    assert_eq!(
        n_printed + tail_count + zero_tax_states,
        n_states,
        "(FT-A24(iv)(2)) stop-and-report: printed + tail + suppressed != the frontier state count"
    );
    assert_eq!(
        n_suppressed_forced + n_suppressed_common,
        zero_tax_states,
        "(FT-A24(iv)) stop-and-report: the suppressed split does not total the suppressed count"
    );
    assert_eq!(
        full_support, core_states,
        "stop-and-report: the positive support and the fusion-core state count disagree"
    );

    // (FT-R2) the mass receipt: sum_I sum_omega mu_I(omega) = 1 exactly.
    let mass = Q::new(sum_p, DEN_MU * n_worlds);
    // The arithmetic identity between the frontier accumulation and the
    // per-world fold: an arithmetic remark on the denominator bookkeeping,
    // NOT a receipt (it cannot fail unless the arrival denominators are
    // mis-tracked, which is exactly what it guards).
    assert_eq!(
        sum_m,
        DEN_MU * a.world_fold,
        "stop-and-report: the frontier accumulation and the per-world revealed fold disagree"
    );

    let u0_diff = Q::new(sum_m, norm);
    let delta1_diff = Q::new(sum_m - sum_best, norm);
    // FT-A18(i)/FT-R4 path 1: U^(1) = U_a^C - sum_I delta_I, with U_a^C the
    // frozen filed value (which (FT-R1) has tied to the frontier table).
    let u1_a_diff = u_filed_diff - delta1_diff;
    // FT-R4 path 2: the glue-one-then-reveal walker, written independently.
    let u1_b_diff = Q::new(b.total, norm);

    Some(UnitResult {
        all_rows,
        printed_rows,
        n_printed,
        n_suppressed_forced,
        n_suppressed_common,
        printed_delta_sum,
        tail,
        printed_deltas,
        n_states,
        n_arrivals,
        mass,
        u0_diff,
        delta1_diff,
        u1_a_diff,
        u1_b_diff,
        zero_tax_states,
        core_states,
        sample,
        steps_a: a.steps,
        steps_b: b.steps,
        residual_a: a.residual,
        residual_b: b.residual,
    })
}

// -- (FT-R6) the reduced-grade cross-check ----------------------------------

/// (FT-R6): at a declared grade-2 and a declared grade-3 coordinate, compute
/// `U^(1)` by the probe and assert `U^(0) >= U^(1) >= Q^H` with all three
/// exact, and assert `U^(1) = Q^H` wherever Lemma FT-trunc predicts the
/// ladder has already collapsed. At grade `g` the focal seat holds `g` tiles
/// and leads the root trick, so `N = g - 1` focal decisions remain and
/// `U^(N-1) = Q^H`: at grade 2 that is `U^(0) = Q^H` and the first tax is
/// identically zero; at grade 3 it is `U^(1) = Q^H`, so the whole fusion gap
/// is first-order. The engine's own `Q^H` and `Q^C` are recomputed here and
/// asserted against the probe's `U^(1)` and `U^(0)`, which tests the LEMMA
/// and not merely the code (FT-A19(vi)).
#[allow(clippy::too_many_lines)]
fn reduced_grade_check(out: &mut String, grade: usize) {
    let kernel = reduced_kernel(grade);
    let n_worlds = kernel.count();
    let expect_worlds: u128 = match grade {
        2 => 90,
        3 => 1680,
        _ => panic!("(FT-R6) is declared at grades 2 and 3 only"),
    };
    assert_eq!(
        n_worlds, expect_worlds,
        "(FT-R6) coordinate identity: the full void-free fiber"
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
        "  (FT-R6) grade-{grade} coordinate: base index 0, decl = PipTrump({}), hand = [{}], pool = [{}], leader offset from focal = 0, |X| = {n_worlds}, enumeration = freeze 7/23, digest {FT_DIGEST}",
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
    .expect("freeze-44 budgets are non-binding at the reduced grades");

    if grade == 3 {
        for (i, (hi, lo, n, d)) in G3_IDX0_QH.iter().enumerate() {
            let (a, env) = &prices.q_h[i];
            assert_eq!(
                *a,
                Domino::new(Pip::new(*hi).expect("pip"), Pip::new(*lo).expect("pip")),
                "(FT-R6) filed S6a action identity"
            );
            assert_eq!(
                to_count(env.eval(qi(0)), grade),
                q(*n, *d),
                "(FT-R6) stop-and-report: the recomputed Q^H differs from the filed S6a row"
            );
        }
        let _ = writeln!(
            out,
            "    filed S6a cross-check (frozen source, quoted from predictive_rank_2026-08-12.txt, S6a, exploratory tier): the three Q^H rows reproduce exactly — HELD"
        );
    }

    for (i, a) in kernel.viewer_hand().iter().enumerate() {
        let qh_diff = prices.q_h[i].1.eval(qi(0));
        let u_diff = prices.q_c[i].1.eval(qi(0));
        assert_eq!(prices.q_h[i].0, a, "H action order");
        assert_eq!(prices.q_c[i].0, a, "C action order");
        let r = run_unit(&kernel, grade, a, u_diff).expect("reduced-grade budgets are non-binding");
        assert_eq!(
            r.mass,
            qi(1),
            "(FT-R2) stop-and-report: the frontier arrival mass is not 1"
        );
        assert_eq!(
            r.u0_diff, u_diff,
            "(FT-R1) stop-and-report: the frontier decomposition does not reconstruct U at the reduced grade"
        );
        assert_eq!(
            r.u1_a_diff, r.u1_b_diff,
            "(FT-R4) stop-and-report: the two paths disagree at the reduced grade"
        );
        assert!(
            r.u0_diff >= r.u1_a_diff,
            "(FT-R6) Proposition 5.2: U^(0) >= U^(1)"
        );
        assert!(
            r.u1_a_diff >= qh_diff,
            "(FT-R3)/(FT-R6) stop-and-report: U^(1) < Q^H"
        );
        // Lemma FT-trunc: N = grade - 1 focal decisions remain, and
        // U^(N-1) = Q^H. At grade 2, N = 1 and already U^(0) = Q^H; at
        // grade 3, N = 2 and U^(1) = Q^H.
        if grade == 2 {
            assert_eq!(
                r.u0_diff, qh_diff,
                "(FT-R6) stop-and-report: Lemma FT-trunc predicts U^(0) = Q^H at grade 2"
            );
            assert_eq!(
                r.delta1_diff,
                qi(0),
                "(FT-R6) stop-and-report: the first tax is identically zero at grade 2"
            );
        }
        assert_eq!(
            r.u1_a_diff, qh_diff,
            "(FT-R6) stop-and-report: Lemma FT-trunc predicts U^(N-1) = Q^H, i.e. U^(1) = Q^H at grades 2 and 3"
        );
        let _ = writeln!(
            out,
            "    root {}: U^(0) = {}  U^(1) = {}  Q^H = {}  Delta^(1) = {} (count)  |I| = {}  mass = 1 — U^(0) >= U^(1) >= Q^H HELD; Lemma FT-trunc's U^(N-1) = Q^H HELD (N = {}); (FT-R1) HELD against the engine's own C value; (FT-R4) HELD",
            tile(a),
            qs(to_count(r.u0_diff, grade)),
            qs(to_count(r.u1_a_diff, grade)),
            qs(to_count(qh_diff, grade)),
            qs(tax_to_count(r.delta1_diff)),
            r.n_states,
            grade - 1
        );
    }
}

// -- the carrier's units -----------------------------------------------------

/// One (coordinate, binding competitor action) unit of the run.
#[derive(Clone, Copy)]
struct UnitKey {
    coord: usize,
    action: Domino,
}

fn filed_actions(rows: &[FiledRow; 4]) -> Vec<Domino> {
    rows.iter()
        .map(|(hi, lo, _, _, _)| {
            Domino::new(Pip::new(*hi).expect("pip"), Pip::new(*lo).expect("pip"))
        })
        .collect()
}

fn filed_qh(rows: &[FiledRow; 4]) -> Vec<Q> {
    rows.iter().map(|(_, _, (n, d), _, _)| q(*n, *d)).collect()
}

fn filed_u(rows: &[FiledRow; 4]) -> Vec<Q> {
    rows.iter().map(|(_, _, _, (n, d), _)| q(*n, *d)).collect()
}

/// The H-argmax set (freeze 50(b): `a*` ranges over the filed H-argmax set)
/// and, per `a*`, the competitors whose filed margin `L_{a*} - U_a` is
/// negative — the binding pairs, in ascending domino index on each side.
fn binding_pairs(rows: &[FiledRow; 4]) -> Vec<(Domino, Domino)> {
    let actions = filed_actions(rows);
    let qh = filed_qh(rows);
    let u = filed_u(rows);
    let vh = qh.iter().copied().max().expect("nonempty");
    let mut out = Vec::new();
    for (i, astar) in actions.iter().enumerate() {
        if qh[i] != vh {
            continue;
        }
        for (j, a) in actions.iter().enumerate() {
            if i == j {
                continue;
            }
            if qh[i] - u[j] < qi(0) {
                out.push((*astar, *a));
            }
        }
    }
    out
}

// -- main --------------------------------------------------------------------

#[allow(clippy::too_many_lines)]
fn main() {
    let t0 = Instant::now();
    sha256_self_check();
    let mut out = String::new();

    // ---- header: both FT-A20 outcomes pre-declared, before any delta_I ----
    let _ = writeln!(
        out,
        "walt fusion-tax probe — Experiment 15.1: the exact first-layer fusion tax over the freeze-50 carrier — EXPLORATORY TIER"
    );
    let _ = writeln!(
        out,
        "SECOND EMISSION (freeze 50 v1.1(c), FT-A24): cut by CONTENT, not by size. This file carries every frontier row with delta_I > 0 — the support of the tax, the whole content of Experiment 15.2 — with the complete (FT-R5) material, every receipt, every summary, every pair verdict and every header and fence. The full frontier table including the zero rows is the COMPANION named below: regenerable, and NOT COMMITTED. Four accounting integers per unit make the omission auditable. The split was produced by this probe's own emitter in a re-run, NEVER by post-processing emitted text (SEP-A14(ii)'s principle: results text is not a machine-readable interface); (FT-R7) is the determinism receipt that re-run buys."
    );
    let _ = writeln!(
        out,
        "rulings: FT-A1..FT-A27, freezes 38 v1 / 44 v2 / 45 / 50 v1.1 (walt/CENSUS-RULINGS.md 2026-08-14); Lemma FT-arrive, Lemma FT-trunc, Corollary FT-grade4, Proposition FT-flat, Proposition FT-tie, Lemma FT-post, Corollary FT-conv, Lemma FT-mix; mathematics under DS-A17: Lemma E3 + (C1)-(C4), Lemma E4, Corollary E4.1, Theorem E6.4, Theorem E6.5"
    );
    let _ = writeln!(
        out,
        "regenerate (both files, deterministically, from the repository alone): cargo run --release -p walt-factory --example fusion_tax"
    );
    let _ = writeln!(out, "freeze-set digest (freeze 50(g)): {FT_DIGEST}");
    let _ = writeln!(out, "{COMPANION_LINE_PLACEHOLDER}");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "BOTH OUTCOMES PRE-DECLARED (FT-A20, printed before any delta_I exists; F7):"
    );
    let _ = writeln!(
        out,
        "  (a) U_a^(1) <= L_{{a*}} at any binding pair -> the first exact gluing cut in the branch closes a pair that Corollary E4.1(3) proved no candidate could ever close. The verdict is filed with Theorem E6.4's MEMBER-NOT-SET caveat verbatim, with the freeze-38 cut sentence of FT-A10(ii), and — at the three tied coordinates — with Proposition FT-tie's note that closure there means the fusion gap was ENTIRELY FIRST-ORDER, an exact statement about this coordinate and nothing wider."
    );
    let _ = writeln!(
        out,
        "  (b) U_a^(1) > L_{{a*}} everywhere -> the filed object is the exact pair (Delta_a^(1), Delta_a^(2)) per binding pair, which by Corollary FT-grade4 is the COMPLETE layer decomposition of the fusion gap at grade 4. That answers the received note's §18.4 at this carrier and specifies exactly what a second-layer or feature-penalty cut must beat. This is a RESULT under F7, filed as one, not a null."
    );
    let _ = writeln!(
        out,
        "  (c) a budget stop -> declared stop, no partial fold retained (freeze 44(b)), no partial tax reported, printed as a stop and never as a finding (R-A18)."
    );
    let _ = writeln!(
        out,
        "  (d) (FT-R1) or (FT-R4) disagreeing -> the most informative outcome available: Theorem 6.1's hypotheses or the implementation are wrong, nothing is claimed, and the disagreeing exact rationals are printed (F7, NO-RESCUE)."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "TIER: exploratory throughout, without exception (FT-A1). Nothing here is promoted, nothing is quotable in a brief, a dispatch, FINDINGS.md or any claim-tier page except by brief amendment adding it to a verifier receipt; an external note is never imported as an axiom (TRUST-01). No number of the received note enters as evidence: every value below is an exact rational of this engine."
    );
    let _ = writeln!(
        out,
        "FENCE (R-A2, P-A1, mandatory in this header per FT-A22(i)): no object produced by this probe is an identity-bearing witness of anything; reachability is a proof-irrelevant proposition; the carrier is the void-free capacity fiber whose members are FEASIBLE and never reachable."
    );
    let _ = writeln!(
        out,
        "REAL-DEAL FENCE (N4-A8, verbatim, travelling with every carrier coordinate): the hands and pools come from rob's receipt corpus, THE BELIEF DOES NOT. The voids the play record had already revealed are deliberately discarded (P-A2's void-free carrier), and support is not belief in any case. No row here is a statement about correct play in that deal, about reachability, or about any belief other than the declared one; the void-filtered column licenses nothing and is not reproduced."
    );
    let _ = writeln!(
        out,
        "NOT CLAIMED, printed in place (FT-A22(ii)): nothing about points or marks (the valuation is the count-free trick differential; E-A2's boundary, and a count re-entry voids every form-keyed record wholesale); nothing about bidding; nothing about how real opponents play; no growth law and no quantity measured at grade 4 quoted for trick 1 or for the opening (P-A21); no cost, timing, runtime or tractability claim read off any traversal observable (SEP-A19(b), N4-A16). Wall-clock below is provenance only."
    );
    let _ = writeln!(
        out,
        "CUT TYPING (FT-A10(ii), binding, printed per cut below): this cut identifies action variables at one information state; the fiber and every world's mass are untouched. Freeze 38(c): validity is discharged for the whole family at once — a lawful policy chooses one action per information state and therefore satisfies every block identification within that state."
    );
    let _ = writeln!(
        out,
        "CUT ORDERING (freeze 38(d), declared before the run and never chosen by result): layers ascending, k = 1 only here (Corollary FT-grade4 makes k = 2 vacuous, U^(2) = Q^H); within a layer, frontier information states in ascending observation-record order (freeze 36(b)'s lexicographic order over the canonical ascending domino index); within a state, actions in ascending domino index. No block merges are used in this run (freeze 38(b)(2) is not exercised)."
    );
    let _ = writeln!(
        out,
        "STOP RULE (freeze 38(e)): Theorem E6.5(G2)'s exposed-face criterion, instantiated as Corollary 6.4's zero-tax test computed from COMPLETE argmax sets. A single tie-broken optimiser is not evidence of a conflict, and freeze 26's least-domino-index tie rule is NOT used here (FT-A8)."
    );
    let _ = writeln!(
        out,
        "CONVENTION (freeze 38(f), Corollary FT-conv): every evaluator here runs in the trick DIFFERENTIAL; every reported value and EVERY TAX COLUMN below is in the COUNT convention, reached by the exact inverse of freeze 26's bridge Q_diff = 2*Q_count - grade. A differential tax is exactly twice its count-convention value; a tax quoted in one convention against a margin in the other is VOID. Every delta_I, Delta^(1) and Delta^(2) column below is labelled (count)."
    );
    let _ = writeln!(
        out,
        "SLOT TYPING (FT-A12(iii), binding): no row of this probe names a per-world BOUND. The q_I(omega,b) values are exact world-informed continuation values, not bounds, and none is ever carried out of a frontier and installed as a root primal witness L (Non-theorem E4')."
    );
    let _ = writeln!(
        out,
        "COMPOSITION FORM (Lemma FT-post, binding): this probe pastes NO residual witness. Every continuation value is evaluated INSIDE THE SAME WALK under the carried arrival weights — form (i) of Lemma FT-post. Form (ii) is not used. The frontier posterior nu_I is NOT uniform on X_I and is never treated as such: the arrival weights are the field's per-world legal-set-size products (Lemma FT-arrive)."
    );
    let _ = writeln!(
        out,
        "BUDGET HONESTY (freeze 44(b)-(e) v2): B = {B_WALK} walk-steps per (coordinate, action) for EACH walk-based evaluator — PATH A and PATH B are two evaluators and carry one budget each; the charge is bag.len() at entry, taken before any child call; on exhaustion None and NO PARTIAL FOLD of any kind is retained, which here means there is no partial tax and no partial U^(1) (FT-A4(iii)). P_max v2 = {P_MAX} states is asserted against the depth-one frontier partition, and the assertion is contentful. Residuals are printed per unit."
    );
    let _ = writeln!(
        out,
        "T_a (FT-A7(vi)): the terminal-frontier mass is ASSERTED zero at every n = 4 coordinate — the focal seat holds four tiles, plays one at the root, and therefore always acts again. The assertion is contentful: it fails if the root is ever the focal seat's last tile."
    );
    let _ = writeln!(
        out,
        "LADDER (Corollary FT-grade4): at grade 4 the focal seat holds four tiles and leads the root trick, so N = 3, C^(0) superset C^(1) superset C^(2) = H, and U_a^C - Q^H(a) = Delta_a^(1) + Delta_a^(2). One computation of U^(1) determines the whole decomposition; the received note's Experiment 15.4 is VACUOUS at this carrier and is not commissioned, and its open question §18.4 is answered exactly and for free."
    );
    let _ = writeln!(
        out,
        "CARRIER ORDER — RULED AT FT-A23; FREEZE 50 v1.1(a). Freeze 50(a) as first written carried an explicit five-element list AND a trailing sort rule, and the rule does not generate the list (the enumerated order is h0, h6, h2, h9, h12, which is neither pip-ascending nor hand-lexicographic nor the corpus-hand-id order). The first emission reported the conflict and implemented the enumeration; FT-A23 rules that THE ENUMERATION GOVERNS — EC-A8's principle, a freeze is a constant, not a rule — and the sort clause is STRUCK, not repaired. FREEZE 50 v1.1(a): the carrier is pip 3 [00 21 32 53] (h0); pip 4 [11 40 43 53] (h6); pip 5 [21 33 53 54] (h2); pip 4 [30 41 54 61] (h9); pip 0 [20 30 40 65] (h12), WITH NO GENERATING RULE. The emitted order stands and no re-emission is owed on this account: nothing numeric crosses coordinates, and no budget stop occurred. It would NOT have been merely presentational had a stop occurred, since which units complete is a function of the order — which is the whole reason a carrier order is frozen."
    );
    let _ = writeln!(
        out,
        "h9 TYPING (FT-A18(iv), RW-A3(iii)): h9 remains NOT PRICED on the exact route — its extraction map measured 517,562,322 states against P_max v2 = 192,000,000, so the primal pipeline never ran there and NO PRIMAL WITNESS IS EXHIBITED at that coordinate. The FT probe needs no extraction map: it needs the depth-one frontier and the revealed continuations below it. At h9 alone, L_{{a*}} = Q^H(a*) is Corollary E4.1(2)'s CEILING, not a receipted primal witness, and every h9 row says so in place. The NOT PRICED label stands verbatim and is not weakened by this run."
    );
    let _ = writeln!(out);

    // ---- (FT-R6), first and blocking ----------------------------------------
    let _ = writeln!(
        out,
        "== (FT-R6) THE REDUCED-GRADE CROSS-CHECK — BLOCKING, run before any carrier number exists =="
    );
    let _ = writeln!(
        out,
        "  It tests the LEMMA and not just the code (FT-A19(vi)): a nonzero Delta^(1) at grade 2 falsifies Lemma FT-trunc or the implementation, and either is stop-and-report. At grade g the focal seat holds g tiles and leads, so N = g - 1 and U^(N-1) = Q^H. The engine's own Q^H and Q^C (walt-strat's H and C operators) are recomputed here and asserted against this probe's U^(1) and U^(0) — an independent-evaluator comparison the grade-4 carrier cannot make."
    );
    reduced_grade_check(&mut out, 2);
    reduced_grade_check(&mut out, 3);
    let _ = writeln!(out, "  (FT-R6) HELD at both declared reduced grades.");
    let _ = writeln!(out);

    // ---- Proposition FT-tie: recomputed from the frozen source table -------
    let _ = writeln!(
        out,
        "== PROPOSITION FT-tie — the fence on the reading, recomputed here from the frozen source table before any tax exists =="
    );
    let _ = writeln!(
        out,
        "  Where the binding competitor is TIED with a* in Q^H, the pair closes only if the relaxation is EXACT at a: L_{{a*}} >= U_a^(k) <=> U_a^(k) = Q^H(a), i.e. sum_{{j>k}} Delta_a^(j) = 0. At those coordinates the experiment's question is not \"does the first layer shave enough\" but \"is the fusion gap entirely first-order\" — a strictly binary question whose negative answer is as informative as its positive one (F7)."
    );
    for (ci, (hand_id, pip, rows, _)) in FT_FILED.iter().enumerate() {
        let actions = filed_actions(rows);
        let qh = filed_qh(rows);
        let u = filed_u(rows);
        let pairs = binding_pairs(rows);
        let (astar, a) = pairs[0];
        let ia = actions.iter().position(|x| *x == a).expect("competitor");
        let is = actions.iter().position(|x| *x == astar).expect("a*");
        let shave = u[ia] - qh[is];
        let own_gap = u[ia] - qh[ia];
        let frac = shave / own_gap;
        let (qs_shave, qs_gap, qs_frac) = FT_TIE_QUOTED[ci];
        assert_eq!(
            shave,
            q(qs_shave.0, qs_shave.1),
            "stop-and-report: the recomputed required shave differs from Proposition FT-tie's quoted value"
        );
        assert_eq!(
            own_gap,
            q(qs_gap.0, qs_gap.1),
            "stop-and-report: the recomputed competitor gap differs from Proposition FT-tie's quoted value"
        );
        assert_eq!(
            frac,
            q(qs_frac.0, qs_frac.1),
            "stop-and-report: the recomputed fraction differs from Proposition FT-tie's quoted value"
        );
        let tied = qh[ia] == qh[is];
        let _ = writeln!(
            out,
            "  h{hand_id} (pip {pip}): a* = {} / binding a = {}  required shave U_a - L_{{a*}} = {}  competitor's own gap U_a - Q^H(a) = {}  fraction required = {}{} — recomputed from the frozen source table and asserted equal to Proposition FT-tie's quoted entry",
            tile(astar),
            tile(a),
            qs(shave),
            qs(own_gap),
            qs(frac),
            if tied { "  [TIED: the fraction is 1 exactly — all or nothing]" } else { "" }
        );
    }
    let _ = writeln!(out);

    // ---- the carrier pass ---------------------------------------------------
    let receipt: Receipt = {
        let path =
            locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
        parse_file(&path).expect("the receipt parses")
    };

    let mut units: Vec<UnitKey> = Vec::new();
    for (ci, (_, _, rows, _)) in FT_FILED.iter().enumerate() {
        let mut comps: Vec<Domino> = binding_pairs(rows).into_iter().map(|(_, a)| a).collect();
        comps.sort_by_key(|d| d.index());
        comps.dedup();
        for a in comps {
            units.push(UnitKey {
                coord: ci,
                action: a,
            });
        }
    }
    let n_units = units.len();
    let _ = writeln!(
        out,
        "== THE CARRIER — freeze 50(a)-(b): five coordinates, {} distinct (coordinate, binding competitor) units, {} binding pairs ==",
        n_units,
        FT_FILED
            .iter()
            .map(|(_, _, rows, _)| binding_pairs(rows).len())
            .sum::<usize>()
    );
    let _ = writeln!(
        out,
        "  W = {n_units} threads (recorded, never frozen; DS-A34/N4-A17(e)): one process, one thread per unit. Every unit's content is a function of (kernel, freeze-44 budgets) alone and is byte-identical at any W; results are assembled in canonical unit order, never completion order (DS-A36)."
    );

    let texts: Mutex<BTreeMap<usize, UnitText>> = Mutex::new(BTreeMap::new());
    let next = AtomicUsize::new(0);
    let receipt_ref = &receipt;
    let texts_ref = &texts;
    let next_ref = &next;
    let units_ref = &units;
    std::thread::scope(|scope| {
        for _ in 0..n_units {
            scope.spawn(move || loop {
                let ui = next_ref.fetch_add(1, Ordering::SeqCst);
                if ui >= units_ref.len() {
                    break;
                }
                let key = units_ref[ui];
                let t = Instant::now();
                let text = render_unit(receipt_ref, key);
                // stdout-only progress; the results file carries no timing row.
                eprintln!(
                    "  [stderr only] unit {ui} (coord h{}, a = {}) complete in {} ms",
                    FT_FILED[key.coord].0,
                    tile(key.action),
                    t.elapsed().as_millis()
                );
                texts_ref.lock().expect("lock").insert(ui, text);
            });
        }
    });
    let texts = texts.into_inner().expect("lock");
    // Assembled in canonical unit order, never completion order (DS-A36).
    let mut companion = String::new();
    let _ = writeln!(
        companion,
        "walt fusion-tax probe — COMPANION frontier table (freeze 50 v1.1(c), FT-A24(v)) — EXPLORATORY TIER"
    );
    let _ = writeln!(
        companion,
        "This file is REGENERABLE and NOT COMMITTED. It is a deterministic function of committed inputs and contains no row that carries a claim; its rows with delta_I = 0 contribute exactly zero to Delta^(1), cannot appear in Experiment 15.2's ranking, and have no fusion core. The named results file carries the positive support, every receipt, every summary and every verdict, and accounts for every row omitted here with four integers per unit."
    );
    let _ = writeln!(
        companion,
        "regenerate: cargo run --release -p walt-factory --example fusion_tax"
    );
    let _ = writeln!(companion, "freeze-set digest (freeze 50(g)): {FT_DIGEST}");
    for ui in 0..n_units {
        let t = texts.get(&ui).expect("every unit rendered");
        let _ = write!(out, "{}", t.named);
        let _ = writeln!(companion);
        for row in &t.companion {
            let _ = writeln!(companion, "{row}");
        }
    }

    // ---- (FT-R8): the single-roof composition receipt at h6 ----------------
    let u1_11_count = {
        let h6 = FT_FILED
            .iter()
            .position(|(h, ..)| *h == 6)
            .expect("h6 is in the carrier");
        let ui = units
            .iter()
            .position(|k| {
                k.coord == h6
                    && k.action == Domino::new(Pip::new(1).expect("pip"), Pip::new(1).expect("pip"))
            })
            .expect("h6's competitor 11 is a unit of this run");
        // The upper witness the composition imports from THIS run, taken from
        // the unit's own summary rather than re-derived: U^(1) at h6, a = 11.
        texts
            .get(&ui)
            .expect("unit rendered")
            .u1_count
            .expect("a completed unit carries U^(1)")
    };
    let _ = write!(out, "{}", render_h6_composition(&receipt, u1_11_count));

    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "total wall-clock (provenance only, never a dividend; DS-A31/DS-A36): {} ms",
        t0.elapsed().as_millis()
    );
    let _ = writeln!(out, "run complete: yes");

    // The companion is written FIRST, so the named file's header can carry
    // its SHA-256 (FT-A24(v)).
    let companion_path = out_dir("results").join("fusion_tax_frontier_2026-08-14.txt");
    std::fs::write(&companion_path, &companion).expect("write companion");
    let digest = sha256_hex(companion.as_bytes());
    let companion_line = format!(
        "companion (FT-A24(v), REGENERABLE and NOT COMMITTED — a deterministic function of committed inputs, carrying the full frontier table including every delta_I = 0 row): results/fusion_tax_frontier_2026-08-14.txt  SHA-256 {digest}  {} bytes, {} lines. \"Reproducible from the repository alone\" is satisfied by the deterministic regeneration command above plus this digest.",
        companion.len(),
        companion.lines().count()
    );
    assert!(
        out.contains(COMPANION_LINE_PLACEHOLDER),
        "the header carries exactly one companion marker"
    );
    let out = out.replace(COMPANION_LINE_PLACEHOLDER, &companion_line);

    let results = out_dir("results").join("fusion_tax_2026-08-14.txt");
    std::fs::write(&results, &out).expect("write results");
    print!("{}", &out[..out.len().min(6000)]);
    println!("results: {}", results.display());
    println!("companion: {} ({digest})", companion_path.display());
}

/// One unit's two emissions (freeze 50 v1.1(c)): the named committed file's
/// text, and the companion's full frontier table.
struct UnitText {
    named: String,
    companion: Vec<String>,
    /// `U_a^(1)` in the count convention, `None` on a declared stop. The
    /// (FT-R8) composition imports this unit's own summary value rather than
    /// re-deriving it, so the composed verdict and the pair row cannot drift.
    u1_count: Option<Q>,
}

#[allow(clippy::too_many_lines)]
fn render_unit(receipt: &Receipt, key: UnitKey) -> UnitText {
    let (hand_id, filed_pip, rows, l_exhibited) = &FT_FILED[key.coord];
    let hand = &receipt.hands[*hand_id];
    let kernel = n4_void_free_kernel(hand);

    // Freeze 45's coordinate identity, ASSERTED FIRST (FT-A18(v)): an
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
    let u_filed_diff = to_diff(u_count[ia], N4_GRADE);
    let qh_filed_diff = to_diff(qh_count[ia], N4_GRADE);

    let mut out = String::new();
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== FT unit: coord h{hand_id} grade={N4_GRADE} pip={} hand=[{}] pool=[{}] leader-offset=0 |X|={N4_FIBER} enumeration=freeze-7/23 || competitor a = {} ==",
        p.value(),
        tiles_str(kernel.viewer_hand()),
        tiles_str(kernel.pool()),
        tile(key.action)
    );
    let _ = writeln!(
        out,
        "  provenance only: corpus hand id {hand_id}, trick {N4_TRICK} (never identity components, freeze 45); freeze-set digest {FT_DIGEST}"
    );
    let _ = writeln!(
        out,
        "  frozen source (FT-A18(v), the SEP-A14(ii) pattern; quoted from separation_n4_2026-08-14.txt, exploratory tier, never re-parsed at run time): Q^H(a) = {}  U_a = {}  (count convention)",
        qs(qh_count[ia]),
        qs(u_count[ia])
    );
    let _ = writeln!(
        out,
        "  CUT (freeze 38(b)(1), layer k = 1): the one-block partition at the first focal frontier, singleton below. THIS CUT IDENTIFIES ACTION VARIABLES AT ONE INFORMATION STATE; THE FIBER AND EVERY WORLD'S MASS ARE UNTOUCHED (FT-A10(ii), verbatim and binding). Validity is discharged by freeze 38(c)'s single argument for the whole family."
    );

    let Some(r) = run_unit(&kernel, N4_GRADE, key.action, u_filed_diff) else {
        let _ = writeln!(
            out,
            "  DECLARED STOP (freeze 44(b), FT-A20(c)): a walk budget of {B_WALK} was exhausted. No partial fold is retained, so there is no partial tax and no partial U^(1); this is a stop and is never a finding (R-A18)."
        );
        return UnitText {
            named: out,
            companion: Vec::new(),
            u1_count: None,
        };
    };

    // ---- (FT-R7) half two: the in-run second pass --------------------------
    // A wholly fresh computation of the same unit — new frontier map, new
    // accumulators, new budgets, both paths — asserted identical on every
    // summary value and on EVERY printed delta_I. This is the half that
    // reaches the individual taxes, which the first emission holds only as
    // results text. It fails on uninitialised state, on accumulator reuse and
    // on any accidental dependence on iteration order.
    let r2 = run_unit(&kernel, N4_GRADE, key.action, u_filed_diff)
        .expect("the second pass runs under the same non-binding budgets as the first");
    assert_eq!(
        (r.u0_diff, r.delta1_diff, r.u1_a_diff, r.u1_b_diff, r.mass),
        (
            r2.u0_diff,
            r2.delta1_diff,
            r2.u1_a_diff,
            r2.u1_b_diff,
            r2.mass
        ),
        "(FT-R7) stop-and-report: two passes of one unit disagree on a summary value"
    );
    assert_eq!(
        (
            r.n_states,
            r.n_arrivals,
            r.n_printed,
            r.n_suppressed_forced,
            r.n_suppressed_common,
            r.steps_a,
            r.steps_b
        ),
        (
            r2.n_states,
            r2.n_arrivals,
            r2.n_printed,
            r2.n_suppressed_forced,
            r2.n_suppressed_common,
            r2.steps_a,
            r2.steps_b
        ),
        "(FT-R7) stop-and-report: two passes of one unit disagree on an accounting integer"
    );
    assert_eq!(
        r.printed_deltas, r2.printed_deltas,
        "(FT-R7) stop-and-report: two passes of one unit disagree on a printed delta_I"
    );
    assert_eq!(
        r.printed_rows, r2.printed_rows,
        "(FT-R7) stop-and-report: two passes of one unit disagree on a printed row"
    );

    // ---- (FT-R7) half one: against the FIRST EMISSION ----------------------
    // The frozen record of the prior process (FT_FIRST), quoted from the
    // rulings, never re-parsed from the first emission's results text.
    let first = FT_FIRST
        .iter()
        .find(|(h, hi, lo, ..)| {
            *h == *hand_id
                && Domino::new(Pip::new(*hi).expect("pip"), Pip::new(*lo).expect("pip"))
                    == key.action
        })
        .expect("every unit has a first-emission record");
    let (_, _, _, f_states, f_arrivals, f_zero, f_pos, f_d1, f_u1, f_d2, f_steps) = *first;

    // ---- the six receipts --------------------------------------------------
    // (FT-R1) the reconstruction receipt — the one that earns the section.
    assert_eq!(
        r.u0_diff, u_filed_diff,
        "(FT-R1) stop-and-report: T_a + sum_I sum_omega mu_I(omega) m_I(omega) != the frozen filed U_a. Nothing is claimed; this is a bug in the probe or in Theorem 6.1's hypotheses, never a finding about the game (R-A18, NO-RESCUE)."
    );
    // (FT-R2) the mass receipt.
    assert_eq!(
        r.mass,
        qi(1),
        "(FT-R2) stop-and-report: sum_I sum_omega mu_I(omega) != 1 — a field branch was dropped or double-counted"
    );
    // (FT-R4) the two-path receipt.
    assert_eq!(
        r.u1_a_diff,
        r.u1_b_diff,
        "(FT-R4) stop-and-report: the frontier-table U^(1) and the glue-one-then-reveal walker disagree (FT-A20(d)); the disagreeing exact rationals are {} and {} (differential)",
        qs(r.u1_a_diff),
        qs(r.u1_b_diff)
    );
    // (FT-R3) the sandwich receipt.
    assert!(
        r.u1_a_diff >= qh_filed_diff,
        "(FT-R3) stop-and-report: U_a^(1) < Q^H(a) — the glue was applied at the wrong node or in the wrong direction"
    );

    let u0_c = to_count(r.u0_diff, N4_GRADE);
    let u1_c = to_count(r.u1_a_diff, N4_GRADE);
    let d1_c = tax_to_count(r.delta1_diff);
    let d2_c = u1_c - qh_count[ia];

    let _ = writeln!(
        out,
        "  FRONTIER (freeze 50(c)): {} information states, {} (state, world) arrivals, P_max v2 admission {} <= {} — ASSERTED. Records are the plays since the kernel decision point with the root action first (freeze 26's observation contract, freeze 36(b)); two records of different length are two different states, which is why these states are mutually exclusive and the local taxes add exactly (FT-A7(iii)-(iv)).",
        r.n_states, r.n_arrivals, r.n_states, P_MAX
    );
    let _ = writeln!(
        out,
        "  states with delta_I = 0 (a common optimum exists): {} ; states with delta_I > 0 (a fusion core exists): {}",
        r.zero_tax_states, r.core_states
    );
    // ---- (FT-A24(iv)) the four accounting integers, per unit ---------------
    let _ = writeln!(
        out,
        "  EMISSION ACCOUNTING (freeze 50 v1.1(c), FT-A24(iv); the omission below is auditable from these four integers alone): PRINTED here (delta_I > 0, the support of the tax) = {} ; SUPPRESSED to the companion (delta_I = 0) = {}, split into FORCED |A(I)| = 1 = {} (where the zero is trivial) and UNFORCED WITH A COMMON OPTIMUM = {} (where the zero is Theorem E6.5(G2)'s exposed-face criterion doing real work) ; printed + suppressed = {} = the frontier state count asserted on the P_max line above — ASSERTED ; sum of the printed delta_I = {} = Delta^(1) exactly — ASSERTED.",
        r.n_printed,
        r.zero_tax_states,
        r.n_suppressed_forced,
        r.n_suppressed_common,
        r.n_printed + r.tail.map_or(0, |(c, _)| c) + r.zero_tax_states,
        qs(r.printed_delta_sum)
    );
    match r.tail {
        None => {
            let _ = writeln!(
                out,
                "  DECLARED CAP (FT-A24(vi)): the cap is {ROW_CAP} rows of positive support per unit; this unit's support is {} and the cap DID NOT FIRE — every row of the support is printed below, in freeze-50(c) order. Had it fired, the file would carry the top {ROW_CAP} by descending delta_I with ties by ascending record order, plus the residual tail's exact count and exact summed delta, printed in place — a declared truncation, never a silent one.",
                r.n_printed
            );
        }
        Some((c, s)) => {
            let _ = writeln!(
                out,
                "  DECLARED TRUNCATION (FT-A24(vi)): the support is {} rows, above the declared cap of {ROW_CAP}. Printed: the top {ROW_CAP} by descending delta_I, ties by ascending record order. RESIDUAL TAIL: exactly {c} rows carrying exactly {} summed delta (count convention); printed sum + tail sum = Delta^(1) — ASSERTED over the full support.",
                r.n_printed + c,
                qs(s)
            );
        }
    }
    for row in &r.printed_rows {
        let _ = writeln!(out, "{row}");
    }
    let _ = writeln!(
        out,
        "  (FT-R5) THE OPTIMAL-FACE RECEIPT — HELD. Corollary 6.4 is asserted BOTH WAYS at EVERY frontier state of this unit (strictly stronger than the declared sample), from COMPLETE argmax sets and never from freeze 26's least-index tie rule (FT-A8). The declared deterministic sample — the first {R5_SAMPLE} frontier states in freeze-50(c) order — is printed:"
    );
    for s in &r.sample {
        let _ = writeln!(out, "{s}");
    }

    // ---- the per-coordinate summary (freeze 50(d)) --------------------------
    let _ = writeln!(
        out,
        "  (FT-R1) RECONSTRUCTION — HELD: T_a + sum_I sum_omega mu_I(omega) m_I(omega) = {} (count) = the frozen filed U_a exactly.",
        qs(u0_c)
    );
    let _ = writeln!(
        out,
        "  (FT-R2) MASS — HELD: sum_I sum_omega mu_I(omega) = 1 exactly, and T_a = 0 asserted by the focal seat having a further decision in every positive-mass world."
    );
    let _ = writeln!(
        out,
        "  (FT-R3) SANDWICH — HELD: U_a^(1) = {} >= Q^H(a) = {} (count). Note in place (FT-A19(iii)): delta_I >= 0 and Delta^(1) >= 0 are ARITHMETIC REMARKS, not receipts — they cannot fail given the formulas — except as computed by the two independent paths of (FT-R4).",
        qs(u1_c),
        qs(qh_count[ia])
    );
    let _ = writeln!(
        out,
        "  (FT-R4) TWO-PATH — HELD: U_a^(1) computed as U_a^C - sum_I delta_I from the frontier table and computed directly by the glue-one-then-reveal walker agree exactly at {} (differential {}). The two computations are independently written and share only the rule algebra.",
        qs(u1_c),
        qs(r.u1_b_diff)
    );
    let _ = writeln!(
        out,
        "  (FT-R6) — HELD at the declared grade-2 and grade-3 coordinates, in this file's blocking block above."
    );
    // ---- (FT-R7) the re-emission determinism receipt -----------------------
    assert_eq!(
        (r.n_states, r.n_arrivals, r.zero_tax_states, r.core_states),
        (f_states, f_arrivals, f_zero, f_pos),
        "(FT-R7) stop-and-report: this run's frontier census differs from the first emission's"
    );
    assert_eq!(
        (d1_c, u1_c, d2_c),
        (q(f_d1.0, f_d1.1), q(f_u1.0, f_u1.1), q(f_d2.0, f_d2.1)),
        "(FT-R7) stop-and-report: this run's summary values differ from the first emission's"
    );
    assert_eq!(
        (r.steps_a, r.steps_b),
        (f_steps, f_steps),
        "(FT-R7) stop-and-report: this run's walk-step charges differ from the first emission's"
    );
    let _ = writeln!(
        out,
        "  (FT-R7) RE-EMISSION DETERMINISM — HELD, in two halves with their scopes named. HALF ONE, against the FIRST EMISSION (a prior process; the frozen record quoted from CENSUS-RULINGS.md's closing note and FT-A24(ii), never re-parsed from results text): the frontier census {}/{} (zero/positive of {} states), the {} arrivals, Delta^(1) = {}, U^(1) = {}, Delta^(2) = {} and both walk-step charges {} all reproduce exactly. It does NOT reach the individual delta_I, which the first emission holds only as text. HALF TWO, the IN-RUN SECOND PASS (a wholly fresh computation of this unit — new frontier map, new accumulators, new budgets, both paths): every summary value, every accounting integer, and EVERY ONE of the {} printed delta_I and printed rows asserted identical. All six receipt statuses are HELD in both passes. Disagreement in either half is stop-and-report, never a re-emission (F7, NO-RESCUE).",
        r.zero_tax_states,
        r.core_states,
        r.n_states,
        r.n_arrivals,
        qs(d1_c),
        qs(u1_c),
        qs(d2_c),
        r.steps_a,
        r.n_printed
    );
    let _ = writeln!(
        out,
        "  SUMMARY (freeze 50(d), COUNT convention on every column including every tax): T_a = 0 (asserted)  Delta_a^(1) = {}  U_a^(1) = {}  Delta_a^(2) = U_a^(1) - Q^H(a) = {}  [Corollary FT-grade4: (Delta^(1), Delta^(2)) is the COMPLETE layer decomposition of this pair's fusion gap; U^(2) = Q^H, so Experiment 15.4 is vacuous here]",
        qs(d1_c),
        qs(u1_c),
        qs(d2_c)
    );
    assert_eq!(
        d1_c + d2_c,
        u_count[ia] - qh_count[ia],
        "stop-and-report: Delta^(1) + Delta^(2) != U_a - Q^H(a) (Corollary FT-grade4)"
    );
    let _ = writeln!(
        out,
        "  walk-step observables (SEP-A19(b) class, per-traversal named — never an information value, a decision width, a cost claim or a DS-A2 term): PATH A (world-major revealed walks recording the frontier) {} charged, residual {} ; PATH B (glue-one-then-reveal, pooled to the frontier) {} charged, residual {}. Same-traversal comparison: PATH A's charge against the filed revealed subtotal {} for this (coordinate, action) — {}.",
        r.steps_a,
        r.residual_a,
        r.steps_b,
        r.residual_b,
        rows[ia].4,
        if r.steps_a == rows[ia].4 { "EQUAL" } else { "DIFFERS (a traversal-shape observable only; it constrains no value, and (FT-R1) is the value check)" }
    );

    // ---- the per-pair verdicts (FT-A20) ------------------------------------
    let pairs: Vec<(Domino, Domino)> = binding_pairs(rows)
        .into_iter()
        .filter(|(_, a)| *a == key.action)
        .collect();
    for (astar, a) in pairs {
        let is = actions.iter().position(|x| *x == astar).expect("a*");
        let l = qh_count[is];
        let tied = qh_count[ia] == qh_count[is];
        let l_note = if *l_exhibited {
            "L_{a*} = Q^H(a*) is the filed primal witness (Corollary E4.1(2); the filed run's R2 receipt HELD at this coordinate)"
        } else {
            "L_{a*} = Q^H(a*) is Corollary E4.1(2)'s CEILING ONLY — this coordinate is NOT PRICED and NO PRIMAL WITNESS IS EXHIBITED (FT-A18(iv))"
        };
        let closed = u1_c <= l;
        let _ = writeln!(
            out,
            "  PAIR (a* = {}, a = {}): L_{{a*}} = {}  U_a = {}  U_a^(1) = {}  ->  {}",
            tile(astar),
            tile(a),
            qs(l),
            qs(u_count[ia]),
            qs(u1_c),
            if closed {
                "FT-A20(a) CLOSED"
            } else {
                "FT-A20(b) NOT CLOSED"
            }
        );
        let _ = writeln!(out, "    {l_note}.");
        if closed {
            // (FT-A25(v)): a STRICT comparison says something stronger and
            // different, and MEMBER-NOT-SET under-claims on it. The boilerplate
            // stays mandatory wherever any comparison is non-strict.
            let surplus = l - u1_c;
            if surplus > qi(0) {
                let _ = writeln!(
                    out,
                    "    FT-A20(a), STRICT (FT-A25(v)): the first exact gluing cut in the branch closes a pair that Corollary E4.1(3) proved no candidate could ever close. The comparison is STRICT, with exact surplus L_{{a*}} - U_a^(1) = {}. A strict pair comparison says more than membership: Q^H(a) <= U_a^(1) < L_{{a*}} <= Q^H(a*) gives Q^H({}) < Q^H({}), so COMPETITOR {} IS EXCLUDED FROM Opt^H. Theorem E6.4's MEMBER-NOT-SET caveat is not wrong here, it UNDER-CLAIMS, and is therefore not printed on this row; it stays mandatory wherever any comparison is non-strict. Cut typing, verbatim (FT-A10(ii)): this cut identifies action variables at one information state; the fiber and every world's mass are untouched.",
                    qs(surplus),
                    tile(a),
                    tile(astar),
                    tile(a)
                );
            } else {
                let _ = writeln!(
                    out,
                    "    FT-A20(a), NON-STRICT: the first exact gluing cut in the branch closes a pair that Corollary E4.1(3) proved no candidate could ever close, with L_{{a*}} = U_a^(1) exactly. MEMBER-NOT-SET (Theorem E6.4, verbatim, in the statement and not the commentary): non-strict separation certifies MEMBERSHIP of a* in Opt^H and NEVER uniqueness, and never the set. Cut typing, verbatim (FT-A10(ii)): this cut identifies action variables at one information state; the fiber and every world's mass are untouched."
                );
            }
            if tied {
                let _ = writeln!(
                    out,
                    "    Proposition FT-tie, TIED pair: Q^H(a) = Q^H(a*), so closure means U_a^(1) = Q^H(a) exactly — the fusion gap at this pair was ENTIRELY FIRST-ORDER (sum_{{j>1}} Delta^(j) = 0). An exact statement about this coordinate and nothing wider."
                );
            }
        } else {
            let short = u1_c - l;
            let _ = writeln!(
                out,
                "    FT-A20(b), a RESULT under F7 and not a null: the first layer does not close this pair. Exact shortfall U_a^(1) - L_{{a*}} = {}. Required shave was U_a - L_{{a*}} = {}; achieved shave Delta_a^(1) = {}; fraction of the competitor's own fusion gap achieved = {} against the {} required. The filed object is the exact pair (Delta_a^(1), Delta_a^(2)) = ({}, {}), which by Corollary FT-grade4 is the COMPLETE layer decomposition of the fusion gap at grade 4: it answers the received note's §18.4 at this pair and specifies exactly what a second-layer or feature-penalty cut must beat.",
                qs(short),
                qs(u_count[ia] - l),
                qs(d1_c),
                qs(d1_c / (u_count[ia] - qh_count[ia])),
                qs((u_count[ia] - l) / (u_count[ia] - qh_count[ia])),
                qs(d1_c),
                qs(d2_c)
            );
            if tied {
                let _ = writeln!(
                    out,
                    "    Proposition FT-tie, TIED pair: Q^H(a) = Q^H(a*), so the all-or-nothing threshold was NOT met — the fusion gap at this pair is NOT entirely first-order, and Delta^(2) = {} is exactly the part the first layer cannot reach. The negative answer is as informative as the positive one (F7).",
                    qs(d2_c)
                );
            }
        }
        let _ = writeln!(
            out,
            "    Proposition FT-flat travels with this row (FT-A11(ii)): a frontier-action-INDEPENDENT upper feature B_I(omega,b) = B_I(omega) returns a bound at least U_a^C and therefore never improves on the witness already filed. A witness must be conditioned on the decision it is trying to price."
        );
    }

    let mut companion = Vec::with_capacity(r.all_rows.len() + 2);
    companion.push(format!(
        "== COMPANION frontier table (freeze 50 v1.1(c), FT-A24(v)): coord h{hand_id} pip={} hand=[{}] competitor a = {} || {} rows, ALL delta_I, freeze-50(c) order ==",
        p.value(),
        tiles_str(kernel.viewer_hand()),
        tile(key.action),
        r.all_rows.len()
    ));
    companion.extend(r.all_rows.iter().cloned());
    UnitText {
        named: out,
        companion,
        u1_count: Some(u1_c),
    }
}

// -- (FT-R8): the single-roof composition receipt at h6 ---------------------

/// (FT-R8), FT-A25(iii): wherever a composed verdict is filed, ONE BLOCK
/// carries the freeze-45 coordinate identity rebuilt in-run, the single `L`
/// with its provenance and its S6h `R2` status, one row per competitor with
/// its upper witness's SOURCE and the comparison's STRICTNESS, the (C-iv)
/// freeze-subset assertion, and the composed verdict.
///
/// The dangerous failure mode, and the reason this needed a receipt rather
/// than a paragraph: a composed verdict is a UNIVERSALLY QUANTIFIED claim
/// over competitors, and a competitor silently absent from an imported table
/// would make a false verdict look complete. So the competitor row set is
/// asserted equal to `legal_plays` at the REBUILT KERNEL minus `a*` — never
/// against a row count in a filed file.
#[allow(clippy::too_many_lines)]
fn render_h6_composition(receipt: &Receipt, u1_11_count: Q) -> String {
    let ci = FT_FILED
        .iter()
        .position(|(h, ..)| *h == 6)
        .expect("h6 is in the carrier");
    let (hand_id, filed_pip, rows, l_exhibited) = &FT_FILED[ci];
    let hand = &receipt.hands[*hand_id];
    let kernel = n4_void_free_kernel(hand);
    let Decl::PipTrump(p) = kernel.decl() else {
        panic!("pip-trump only")
    };
    let actions: Vec<Domino> = kernel.viewer_hand().iter().collect();
    let qh_count = filed_qh(rows);
    let u_count = filed_u(rows);
    let astar = Domino::new(Pip::new(4).expect("pip"), Pip::new(0).expect("pip"));
    let is = actions.iter().position(|x| *x == astar).expect("a* = 40");

    let mut out = String::new();
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== (FT-R8) THE SINGLE-ROOF COMPOSITION RECEIPT — h6 (FT-A25(iii)); EXPLORATORY TIER =="
    );
    // (C-i) same coordinate: the freeze-45 identity, rebuilt in-run.
    assert_eq!(
        p.value(),
        *filed_pip,
        "(C-i) freeze 45: declaration identity"
    );
    assert_eq!(kernel.count(), N4_FIBER, "(C-i) freeze 45: |X| = 34,650");
    assert_eq!(
        kernel.viewer_hand().len(),
        N4_GRADE,
        "(C-i) freeze 45: grade 4"
    );
    assert_eq!(
        actions,
        filed_actions(rows),
        "(C-i) freeze 45: the filed root-action list is this coordinate's hand"
    );
    let _ = writeln!(
        out,
        "  (C-i) SAME COORDINATE — freeze-45 identity rebuilt in-run and asserted: grade={N4_GRADE} pip={} hand=[{}] pool=[{}] leader-offset=0 |X|={N4_FIBER} enumeration=freeze-7/23. Both halves of the composition assert this same identity; provenance only: corpus hand id {hand_id}, trick {N4_TRICK}.",
        p.value(),
        tiles_str(kernel.viewer_hand()),
        tiles_str(kernel.pool())
    );

    // (C-ii) same L: one number, one provenance.
    let l = qh_count[is];
    assert!(
        *l_exhibited,
        "(C-ii) stop-and-report: composition requires a receipted primal witness at this coordinate"
    );
    let _ = writeln!(
        out,
        "  (C-ii) SAME L — one number, one provenance: L_{{40}} = Q^H(40) = {}, receipted in S6h by its R2 primal receipt (HELD: the H-optimal policy re-priced by the fixed-policy evaluator, so L = Q^H by Corollary E4.1(2)), and quoted by FT from the frozen source table under FT-A18(v). Both halves of the composition use this one number.",
        qs(l)
    );

    // (C-iii) same (C1)-(C4).
    let _ = writeln!(
        out,
        "  (C-iii) SAME (C1)-(C4) — same fixed field, same belief, same world set, same utility and count contract: both halves cite freeze 26 (observation contract, authority, bridge) and freeze 37(d) (belief uniform over the full enumerated fiber) UNCHANGED, with no decimation anywhere inside any L, U or tax. The valuation is the count-free trick differential in both, bridged to the count convention at the reporting boundary only."
    );

    // (C-iv) the freeze-subset assertion — printed, never assumed.
    let _ = writeln!(
        out,
        "  (C-iv) FREEZE-SUBSET ASSERTION (printed, never assumed; FT-A25(ii)). The two freeze-set digests DIFFER BY CONSTRUCTION, because this session added freezes 38 v1 and 50. DS-A30 does NOT bite: it governs a STORED RECORD of computed state, for which a digest mismatch is corruption and the cache is discarded entire. A number imported as a FROZEN SOURCE TABLE WITH PROVENANCE is a different object, and SEP-A14(ii) is the governing precedent — it imported S6a values computed under an older freeze set on exactly this basis. What is required instead, and is asserted here: the freezes the imported numbers depend on — 7/23 enumeration order, 26 authority and bridge and tie rule, 37 the upper witness, 44 v2 budgets, 45 coordinate identity — are IDENTICAL in both runs; and the freezes this run adds — 38 v1 (cut language and ordering) and 50 (a carrier) — touch NO OBJECT the imported numbers depend on: neither changes the kernel, the belief, the field, the valuation or the enumeration order."
    );

    // The competitor rows, and the assertion that earns the receipt.
    let legal_at_root = legal_plays(kernel.decl(), kernel.viewer_hand(), None);
    let expected = legal_at_root.difference(DominoSet::single(astar));
    let mut row_set = DominoSet::EMPTY;
    let mut all_strict = true;
    let mut rows_out: Vec<String> = Vec::new();
    for (j, a) in actions.iter().enumerate() {
        if *a == astar {
            continue;
        }
        row_set.insert(*a);
        let (w, source) = if *a == Domino::new(Pip::new(1).expect("pip"), Pip::new(1).expect("pip"))
        {
            (
                u1_11_count,
                "FT U^(1) — this run's first-layer gluing cut (freeze 38(b)(1))",
            )
        } else {
            (
                u_count[j],
                "S6h U^C — treatment C, frozen source table quoted from separation_n4_2026-08-14.txt",
            )
        };
        let surplus = l - w;
        let strict = surplus > qi(0);
        all_strict = all_strict && strict;
        assert!(
            surplus >= qi(0),
            "(FT-R8) stop-and-report: competitor {} is not covered by L",
            tile(*a)
        );
        assert_eq!(
            surplus,
            q(
                H6_SURPLUS_QUOTED[rows_out.len()].0,
                H6_SURPLUS_QUOTED[rows_out.len()].1
            ),
            "(FT-R8) stop-and-report: the recomputed surplus differs from FT-A25(iv)'s quoted value"
        );
        rows_out.push(format!(
            "    competitor {}: upper witness W = {}  source = {}  |  L - W = {} > 0, {} — {}",
            tile(*a),
            qs(w),
            source,
            qs(surplus),
            if strict { "STRICT" } else { "NON-STRICT" },
            if strict {
                "so Q^H(this competitor) < Q^H(40) and it is EXCLUDED from Opt^H"
            } else {
                "membership only; MEMBER-NOT-SET applies"
            }
        ));
    }
    // THE assertion: the competitor row set is the legal action set at the
    // root minus a*, computed from the REBUILT KERNEL and never from a row
    // count in a filed file.
    assert_eq!(
        row_set, expected,
        "(FT-R8) stop-and-report: the competitor row set is not legal_plays at the rebuilt kernel minus a* — a composed verdict is a universally quantified claim over competitors, and a silently absent competitor would make a false verdict look complete"
    );
    let _ = writeln!(
        out,
        "  COMPETITOR ROW SET — ASSERTED EQUAL to legal_plays(decl, focal hand, led = None) at the REBUILT KERNEL minus a* = 40, i.e. {{{}}} \\ {{40}} = {{{}}}, computed from the kernel and NEVER from a row count in a filed file. This is the clause that earns the receipt: a composed verdict is a UNIVERSALLY QUANTIFIED claim over competitors, and a competitor silently absent from an imported table would make a false verdict look complete.",
        tiles_str(legal_at_root),
        tiles_str(row_set)
    );
    for row in &rows_out {
        let _ = writeln!(out, "{row}");
    }

    // The composed verdict.
    let _ = writeln!(
        out,
        "  COMPOSITION — Lemma FT-mix (heterogeneous upper witnesses compose; delivered at CENSUS-RULINGS.md 2026-08-14): for each competitor a there is SOME valid upper witness W_a >= Q^H(a) — they need not come from one relaxation, one evaluator, one traversal or one run — and L_{{a*}} >= W_a for every a, hence a* in Opt^H; if every inequality is strict, Opt^H = {{a*}}. Here two relaxations are mixed: competitor 11 by the first-layer gluing cut, competitors 43 and 53 by treatment C. Theorem E6.4's proof never required one relaxation; Lemma FT-mix records the generalisation explicitly so this is licensed by a stated result and never read as an extension of E6.4 past what E6.4 proves."
    );
    assert!(
        all_strict,
        "(FT-R8) stop-and-report: a non-strict comparison bars the uniqueness verdict"
    );
    let _ = writeln!(
        out,
        "  VERDICT (EXPLORATORY TIER, as is everything in this file): all three comparisons are STRICT and {{11, 43, 53}} is the COMPLETE competitor set, so by Lemma FT-mix  Opt^H(h6) = {{40}}  — UNIQUENESS, not merely membership. Theorem E6.4's MEMBER-NOT-SET caveat is not printed here because no comparison is non-strict; it remains mandatory wherever one is."
    );
    let _ = writeln!(
        out,
        "  MANDATORY SENTENCE (FT-A25(vi), verbatim, and it travels with this verdict into any wiki text derived from it): this coordinate's optimal set was already determined by the filed Q^H column; what this verdict demonstrates is that the two-sided proof architecture now closes here, and that the lever was a gluing cut and never a better candidate — which is exactly what Corollary E4.1(3) proved was the only lever available."
    );
    let _ = writeln!(
        out,
        "  Reading that sentence forces (FT-A25(vi)): all four of h6's Q^H values are filed and 40 is strictly the largest, so Opt^H(h6) = {{40}} was determined before this probe ran. The composed verdict is a RESULT ABOUT THE MACHINERY, NOT A DISCOVERY ABOUT THE COORDINATE. The closure COULD have failed — it did at ten of twelve pairs — so the run is a genuine test; the CONCLUSION could not have come out otherwise, so it is not evidence about the game."
    );
    let _ = writeln!(
        out,
        "  NO COMPOSED VERDICT AT h9 (FT-A25(vii), ratified): h9 has no R2 primal receipt — its extraction map exceeded P_max v2 and the primal pipeline never ran — so L_{{a*}} = Q^H(a*) there is Corollary E4.1(2)'s CEILING, not a receipted primal witness. (C-ii) fails at h9 and composition is BARRED until a primal witness is receipted there. No composed verdict is filed at any other carrier coordinate either."
    );
    let _ = writeln!(
        out,
        "  NOT CLAIMED (FT-A27(iii)): the h6 closure is not a claim that gluing is cheap, general or scalable. One pair closed of twelve, at the untied coordinate with the smaller required fraction, at grade 4, on a five-coordinate carrier SELECTED BY NEGATIVE MARGIN — and no cost, timing or tractability claim is read off any traversal observable (SEP-A19(b), N4-A16)."
    );
    out
}
