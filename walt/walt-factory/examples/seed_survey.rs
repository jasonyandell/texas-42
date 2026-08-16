//! walt SS seed survey — FREEZE 54, a hundred fresh grade-4 coordinates
//! reached by a declared arithmetic map. EXPLORATORY TIER THROUGHOUT, without
//! exception, cited by nothing above this tier.
//!
//! Commissioned at SS-A1..SS-A9 (`walt/CENSUS-RULINGS.md`, 2026-08-15), with
//! **freeze 54** fixed at SS-A4, under freezes 7/23, 26, 37(d), 38 v1.1, 44
//! v2, 45, 46/49, 50 v1.1, 52 through v1.4, 53. Beneath them the FF, FT, SR
//! and FC chapters entire.
//!
//! WHAT THIS IS (SS-A1). The first carrier in this branch that is **not
//! selected by outcome.** Every previous n = 4 carrier was chosen by negative
//! binding margin. These hundred coordinates are selected by a declared
//! arithmetic map from the natural numbers and by nothing else, and every
//! legal root action is a unit, so neither the coordinate nor the action is
//! chosen by result.
//!
//! WHAT THIS IS NOT (SS-A1). **Not a fee measurement** — tonight measures
//! whether tie multiplicity tracks separation structure across fresh
//! coordinates; whether it tracks fee capture needs fees, which is a later
//! run. Not a distribution over 42: a hundred deals under one declared map is
//! a **carrier**, and P-A21 binds — nothing measured at grade 4 is quoted for
//! trick 1 or for the opening.
//!
//! THE ONE HAZARD, and it guards the headline statistic (SS-A6(vi)). The tie
//! census depends on the COMPLETE per-world clairvoyant argmax set at every
//! arrival. It is accumulated BY AN EQUALITY TEST ACROSS ALL CANDIDATES and
//! NO INDEX IS TRACKED; `max_by_key` returning one index IS the defect
//! (FC-A11(ii)). (SS-R6) catches a collapse loudly, because collapsing to
//! singletons makes the `delta_I = 0` states report empty argmax
//! intersections almost everywhere.
//!
//! No floats. Regenerate:
//! `cargo run --release -p walt-factory --example seed_survey`
//! Smoke (a declared sub-range, filed as PARTIAL and never as the survey):
//! `cargo run --release -p walt-factory --example seed_survey -- seeds=0..1`

use std::cell::Cell;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Signed;
use walt_core::{
    legal_plays, Context, ContextSet, Decl, Domino, DominoSet, Pip, Seat, Team, Trick,
};
use walt_geom::{q, qi, Q};
use walt_kernel::{Hidden, Kernel, HIDDEN_SEATS};
use walt_strat::{information_prices, Direction, InfoPartition};

// -- freeze 54(a): the generator's constants, all asserted ------------------

/// SS-A2(i): the spreading multiplier. The least prime at or above `D/phi`,
/// the golden-ratio multiplier — the standard low-discrepancy choice, frozen
/// at adjudication BEFORE the build and never chosen by a result. A successor
/// changing it is running a different survey (SS-A2(iii)).
const A_SPREAD: u128 = 292_032_399_099_041;

/// SS-A2(i): the deal-index space `C(28,7)*C(21,7)*C(14,7)`.
const D_DEALS: u128 = 472_518_347_558_400;

/// Freeze 54(a): seeds `0..=99` inclusive.
const SEED_LO: usize = 0;
const SEED_HI: usize = 99;

/// SS-A4(e)/DS-A36: the checkpoint block, in seeds.
const BLOCK: usize = 10;

// -- the standing machinery (freezes 44 v2, 45), unchanged ------------------

/// Freeze 44(b) v2, unchanged by SS-A5(i): `B` walk-steps per (coordinate,
/// action) per evaluator, charge-then-descend. On exhaustion `None` and NO
/// PARTIAL FOLD of any kind — no partial tax, no partial census, no partial
/// solve. No new constant is fixed here.
const B_WALK: u64 = 10_000_000_000;

/// Freeze 44(e) v2's partition-state threshold. SS-A4(c): an over-threshold
/// count bars the PRIMAL-WITNESS pipeline only and is recorded NOT PRICED on
/// that route, never attempted. It does not bar (M1)-(M6).
const P_MAX: u64 = 192_000_000;

const N4_GRADE: usize = 4;
const N4_FIBER: u128 = 34_650;

/// `12^12`. The deep solve's fixed scale.
const SCALE: i64 = 8_916_100_448_256;

/// `12^6`. The common denominator of every pre-frontier arrival weight.
const DEN_MU: i128 = 2_985_984;

/// Freeze 54(f): the freeze-set digest travelling on every record.
const SS_DIGEST: &str = "SS-v1.0|freezes-7-23-26-37d-38v1.1-44v2-45-46-49-50v1.1-52v1.4-53-54|contract=R-A11-full-record|field=uniform-legal-F4|belief=uniform-fiber-freeze7";

/// DS-A30/freeze 41: the checkpoint record format identity. A record whose
/// digest differs from the running freeze set is CORRUPT, not stale, and the
/// whole cache is discarded — never partially reused.
const SS_CKPT_DIGEST: &str =
    "SS-ckpt-v1|freeze-54|A=292032399099041|D=472518347558400|block=10|fields=v1";

/// SS-A4(b)(M6), freezes 46/49: the four frozen rule arms.
const ARM_NAMES: [&str; 5] = [
    "",
    "P1 least-tile",
    "P2 greatest-tile",
    "P3 beat-if-able",
    "P4 trump-hoard",
];

/// SS-A4(b)(M2): the sentence that travels with every verdict cell, verbatim.
const SEPARATION_SENSE: &str = "SEPARATION SENSE (SS-A4(b)(M2), printed with every verdict cell): walt's separation sense — an exact separation of one action from every competitor — and never D3's sense.";

// -- run-owner declarations (provenance, never a receipt: SS-A5(ii)) --------

/// The run owner's declared per-unit wall-clock stop, in seconds, and the
/// declared per-block memory budget, in GiB. PROVENANCE, never receipts
/// (N4-A13, SEP-A19(b)), and therefore RUN INPUTS rather than constants of the
/// probe — the separation_probe precedent, whose regenerate line already reads
/// `M_MAX_GIB=.. T_PASS_H=.. M_BUDGET_GIB=.. N4_W=..`. SS-A5(ii) gives these to
/// the run owner to declare; SS-A5(i) forbids fixing any NEW constant here, and
/// neither of these is one. The values below are the defaults declared for this
/// run and are overridden by `SS_T_PASS_SECS` / `SS_M_BUDGET_GIB`; whichever
/// value was in force is printed in the artifact.
const T_PASS_SECS_DEFAULT: u64 = 600;
const M_BUDGET_GIB_DEFAULT: u64 = 40;

fn t_pass_secs() -> u64 {
    std::env::var("SS_T_PASS_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(T_PASS_SECS_DEFAULT)
}

fn m_budget_gib() -> u64 {
    std::env::var("SS_M_BUDGET_GIB")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(M_BUDGET_GIB_DEFAULT)
}

/// Amendment (run-owner provenance, 2026-08-15): every FAT artifact — the
/// gitignored companion and the checkpoint blocks — is written OUT OF TREE, so
/// a worktree can be deleted without losing run data. The thin committed
/// summary stays in the repository and carries this path plus the pinned
/// SHA-256, byte and line counts, which are the only link between them.
const FAT_ROOT: &str = "/Users/jason/data/texas-42/seed-survey";

/// The header is assembled before the companion's digest exists; the marker is
/// substituted once, at write time (the freeze 50 v1.1(c) pattern).
const COMPANION_LINE_PLACEHOLDER: &str = "@@COMPANION@@";

// -- small helpers ----------------------------------------------------------

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

fn parse_q(s: &str) -> Q {
    let (n, d) = s.split_once('/').expect("a checkpoint rational is n/d");
    Q::new(
        n.parse::<i128>().expect("numerator"),
        d.parse::<i128>().expect("denominator"),
    )
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

/// An exact rational in arbitrary precision. SS-A10(ii)'s PRIMARY statistic is
/// an unweighted mean of up to 400 exact fractions whose denominators are
/// arrival counts of order 10^6; their common denominator is unbounded and
/// overflows `Q = Ratio<i128>` long before 400 terms. The mean is therefore
/// accumulated here, exactly, with no float anywhere (P-A19).
fn br(n: u128, d: u128) -> BigRational {
    assert!(
        d > 0,
        "stop-and-report: an exact fraction with zero denominator"
    );
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

fn brs(x: &BigRational) -> String {
    format!("{}/{}", x.numer(), x.denom())
}

/// An exact parts-per-million bracket on a nonnegative rational, by integer
/// arithmetic only (P-A19: no float anywhere). PRESENTATION ONLY — it enters no
/// proof, no receipt and no comparison.
fn ppm(x: &BigRational) -> String {
    let scaled = x * BigRational::from_integer(BigInt::from(1_000_000));
    let (lo, hi) = (scaled.floor().to_integer(), scaled.ceil().to_integer());
    if lo == hi {
        format!("{lo} ppm exactly")
    } else {
        format!("between {lo} and {hi} ppm")
    }
}

/// The unweighted mean of exact fractions — SS-A10(ii)'s primary convention.
/// `None` when the unit set is empty, so an empty cell never prints a mean.
fn mean_of(xs: &[BigRational]) -> Option<BigRational> {
    if xs.is_empty() {
        return None;
    }
    let mut acc = BigRational::from_integer(BigInt::from(0));
    for x in xs {
        acc += x;
    }
    Some(acc / BigRational::from_integer(BigInt::from(xs.len())))
}

/// An order statistic of a sorted integer slice, by the declared rule: the
/// lower median at even length, so every quantile is an EXACT observed value
/// and no averaging of two observations is ever performed.
fn order_stat(sorted: &[u64], num: usize, den: usize) -> u64 {
    assert!(
        !sorted.is_empty(),
        "an order statistic needs a nonempty set"
    );
    let idx = (sorted.len() * num) / den;
    sorted[idx.min(sorted.len() - 1)]
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

/// The existing combinadic unranking, reused rather than re-derived — the
/// FC-A11(ii) rule, restated at SS-A2(iv): mirror the receipted path.
/// Transcribed unchanged from `rule_economy_n4.rs` / `fusion_tax.rs`.
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

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// -- SHA-256 (FIPS 180-4), exact integer arithmetic, no dependency ----------
// Carried forward from fc_correlation.rs with SR-A33's repair already in it:
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

    fn block(&mut self, b: &[u8]) {
        let mut w = [0u32; 64];
        for (i, c) in b.chunks_exact(4).enumerate().take(16) {
            w[i] = u32::from_be_bytes([c[0], c[1], c[2], c[3]]);
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
        for (i, x) in v.iter().enumerate() {
            self.h[i] = self.h[i].wrapping_add(*x);
        }
    }

    fn update(&mut self, mut data: &[u8]) {
        self.total += data.len() as u64;
        if self.buf_len > 0 {
            let take = (64 - self.buf_len).min(data.len());
            self.buf[self.buf_len..self.buf_len + take].copy_from_slice(&data[..take]);
            self.buf_len += take;
            data = &data[take..];
            if self.buf_len == 64 {
                let b = self.buf;
                self.block(&b);
                self.buf_len = 0;
            }
        }
        while data.len() >= 64 {
            let (b, rest) = data.split_at(64);
            self.block(b);
            data = rest;
        }
        if !data.is_empty() {
            self.buf[..data.len()].copy_from_slice(data);
            self.buf_len = data.len();
        }
    }

    fn finish(mut self) -> String {
        let bits = self.total * 8;
        let rem = (self.total % 64) as usize;
        let pad = if rem < 56 { 56 - rem } else { 120 - rem };
        let mut tail = vec![0u8; pad + 8];
        tail[0] = 0x80;
        tail[pad..].copy_from_slice(&bits.to_be_bytes());
        let saved = self.total;
        self.update(&tail);
        self.total = saved;
        let mut out = String::with_capacity(64);
        for x in self.h {
            let _ = write!(out, "{x:08x}");
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

/// Process resident size in KiB, for the M_budget provenance line. Provenance
/// only; it gates nothing and is never a dividend.
fn rss_kb() -> u64 {
    std::process::Command::new("ps")
        .args(["-o", "rss=", "-p", &std::process::id().to_string()])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|t| t.trim().parse().ok())
        .unwrap_or(0)
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

// == FREEZE 54(a): THE GENERATOR ============================================

/// One seed's generated coordinate. Everything here is a function of the seed
/// alone; (SS-R3) recomputes it a second time from the seed with fresh state
/// and asserts the identity tuple byte-identical.
struct Coord {
    seed: usize,
    index: u128,
    decl: Decl,
    /// The four dealt seven-tile hands, by seat.
    deal: [DominoSet; Seat::COUNT],
    /// The twelve tiles of the three dumb-policy tricks, in play order.
    played: Vec<Domino>,
    /// The three trick winners, in order.
    winners: [Seat; 3],
    /// The focal seat: the winner of trick 3, on lead at the coordinate, which
    /// makes the leader offset 0 automatically (freeze 45).
    focal: Seat,
    /// The four hands remaining at the coordinate, by seat.
    rest: [DominoSet; Seat::COUNT],
}

/// SS-A2(i): the declared spreading map. The seed does NOT index the deal
/// directly.
fn spread_index(seed: usize) -> u128 {
    (u128::try_from(seed).expect("a seed fits u128") * A_SPREAD) % D_DEALS
}

/// SS-A2(iv): the standard mixed radix — `index` split by division into
/// `(r0, r1, r2)` over the radices `C(21,7)*C(14,7)`, `C(14,7)`, `1`, then
/// three combinadic unrankings giving seats 0, 1, 2 their hands and seat 3 the
/// remainder.
fn deal_from_index(index: u128) -> [DominoSet; Seat::COUNT] {
    let c21 = binom(21, 7);
    let c14 = binom(14, 7);
    assert_eq!(c21 * c14, 399_072_960, "SS-A2's blocking constant");
    assert_eq!(binom(28, 7) * c21 * c14, D_DEALS, "SS-A2(i): D");
    let r0 = index / (c21 * c14);
    let rem = index % (c21 * c14);
    let r1 = rem / c14;
    let r2 = rem % c14;

    let mut pool: Vec<usize> = (0..Domino::COUNT).collect();
    let mut hands = [DominoSet::EMPTY; Seat::COUNT];
    for (s, rank) in [r0, r1, r2].into_iter().enumerate() {
        let picks = unrank_comb(pool.len(), 7, rank);
        let mut h = DominoSet::EMPTY;
        for p in picks.iter().rev() {
            let idx = pool.remove(*p);
            assert!(
                h.insert(Domino::from_index(idx).expect("a domino index")),
                "an unranking names each tile once"
            );
        }
        hands[s] = h;
    }
    let mut last = DominoSet::EMPTY;
    for idx in &pool {
        assert!(
            last.insert(Domino::from_index(*idx).expect("a domino index")),
            "the remainder names each tile once"
        );
    }
    hands[3] = last;
    hands
}

/// SS-A2(v): the frozen dumb policy of freeze 26 — least legal domino index —
/// plays three complete tricks, twelve tiles, leaving four in every hand. Seat
/// 0 leads trick 1.
fn generate(seed: usize) -> Coord {
    let index = spread_index(seed);
    let pip = u8::try_from(seed % 7).expect("a pip fits u8");
    let decl = Decl::PipTrump(Pip::new(pip).expect("pips 0..6"));
    let deal = deal_from_index(index);

    let mut hands = deal;
    let mut leader = Seat::S0;
    let mut played: Vec<Domino> = Vec::with_capacity(12);
    let mut winners = [Seat::S0; 3];
    for (t, slot) in winners.iter_mut().enumerate() {
        let mut tiles = [Domino::ALL[0]; 4];
        for k in 0..4usize {
            let seat = leader.plus(k);
            let led: Option<Context> = (k > 0).then(|| decl.led_context(tiles[0]));
            let legal = legal_plays(decl, hands[seat.index()], led);
            let choice = legal
                .iter()
                .min_by_key(|d| d.index())
                .expect("freeze 26: a live seat has a legal move");
            hands[seat.index()].remove(choice);
            tiles[k] = choice;
            played.push(choice);
        }
        let trick = Trick::new(leader, tiles).expect("distinct tiles by construction");
        let winner = trick.winner(decl);
        *slot = winner;
        leader = winner;
        assert_eq!(played.len(), 4 * (t + 1), "four tiles per trick");
    }
    Coord {
        seed,
        index,
        decl,
        deal,
        played,
        winners,
        focal: leader,
        rest: hands,
    }
}

/// The freeze-45 identity tuple, as one line. (SS-R3) asserts this string
/// byte-identical across two independent generations from the seed alone;
/// SS-A3's canonical coordinate key is its declaration/hand/pool fields, so
/// collisions are computable from the committed summary alone by anyone.
fn identity_line(c: &Coord) -> String {
    let Decl::PipTrump(p) = c.decl else {
        panic!("stop-and-report: freeze 54(a) declares PipTrump only")
    };
    let pool = pool_of(c);
    format!(
        "seed={} index={} decl=PipTrump({}) deal0=[{}] deal1=[{}] deal2=[{}] deal3=[{}] played=[{}] winners=[{} {} {}] focal=S{} key-hand=[{}] key-pool=[{}]",
        c.seed,
        c.index,
        p.value(),
        tiles_str(c.deal[0]),
        tiles_str(c.deal[1]),
        tiles_str(c.deal[2]),
        tiles_str(c.deal[3]),
        record_str(&c.played),
        c.winners[0].index(),
        c.winners[1].index(),
        c.winners[2].index(),
        c.focal.index(),
        tiles_str(c.rest[c.focal.index()]),
        tiles_str(pool),
    )
}

/// SS-A3's CANONICAL COORDINATE KEY: declaration, focal hand, pool, as
/// ascending domino-index tile lists in freeze-45 form — and NOTHING
/// seed-specific, so collisions are computable from the committed summary alone
/// by anyone. Derived from the identity line so the two cannot drift.
fn coord_key(identity: &str) -> String {
    let decl = identity
        .split(' ')
        .find(|t| t.starts_with("decl="))
        .expect("the identity line carries a declaration");
    let tail = &identity[identity
        .find(" key-hand=")
        .expect("the identity line carries a focal hand")..];
    format!("{decl}{tail}")
}

fn pool_of(c: &Coord) -> DominoSet {
    let mut pool = DominoSet::EMPTY;
    for k in 1..=3usize {
        pool = pool.union(c.rest[c.focal.plus(k).index()]);
    }
    pool
}

/// (SS-R1) GENERATOR SOUNDNESS — BLOCKING, before any unit runs. Contentful,
/// and it is the check that catches an unranking error, which is otherwise
/// invisible because a wrong deal is still a well-formed deal. The playout is
/// re-validated by an INDEPENDENT replay from the deal and the recorded twelve
/// tiles — legality is not read off the policy that produced it.
fn r1_generator_soundness(c: &Coord) {
    // The four hands partition all 28: disjoint, seven each, union complete.
    let mut union = DominoSet::EMPTY;
    for (i, h) in c.deal.iter().enumerate() {
        assert_eq!(h.len(), 7, "(SS-R1) seat {i} was not dealt seven tiles");
        assert!(
            union.is_disjoint(*h),
            "(SS-R1) stop-and-report: dealt hands overlap at seat {i}"
        );
        union = union.union(*h);
    }
    assert_eq!(
        union,
        DominoSet::FULL,
        "(SS-R1) stop-and-report: the four hands do not partition all 28 dominoes"
    );
    // Exactly twelve tiles played.
    assert_eq!(
        c.played.len(),
        12,
        "(SS-R1) stop-and-report: the playout is not twelve tiles"
    );
    // The independent replay: every tile legal AT THE MOMENT it was played.
    let mut hands = c.deal;
    let mut leader = Seat::S0;
    for t in 0..3usize {
        let mut tiles = [Domino::ALL[0]; 4];
        for k in 0..4usize {
            let seat = leader.plus(k);
            let led: Option<Context> = (k > 0).then(|| c.decl.led_context(tiles[0]));
            let legal = legal_plays(c.decl, hands[seat.index()], led);
            let d = c.played[4 * t + k];
            assert!(
                legal.contains(d),
                "(SS-R1) stop-and-report: played tile [{}] was ILLEGAL at trick {} ply {} for seat S{}",
                tile(d),
                t + 1,
                k,
                seat.index()
            );
            hands[seat.index()].remove(d);
            tiles[k] = d;
        }
        let trick = Trick::new(leader, tiles).expect("distinct tiles by construction");
        let winner = trick.winner(c.decl);
        assert_eq!(
            winner,
            c.winners[t],
            "(SS-R1) stop-and-report: the replayed trick-{} winner differs from the generator's",
            t + 1
        );
        leader = winner;
    }
    // Four remain in every hand, and the replay reproduces the coordinate.
    for (i, h) in hands.iter().enumerate() {
        assert_eq!(
            h.len(),
            N4_GRADE,
            "(SS-R1) stop-and-report: seat {i} does not hold four tiles at the coordinate"
        );
        assert_eq!(
            *h, c.rest[i],
            "(SS-R1) stop-and-report: the replay's residual hand differs at seat {i}"
        );
    }
    // The trick-3 winner is the focal seat.
    assert_eq!(
        leader, c.focal,
        "(SS-R1) stop-and-report: the focal seat is not the trick-3 winner"
    );
}

/// (SS-R4) COORDINATE IDENTITY: freeze 45's form at every coordinate, the
/// kernel rebuilt in-run, `|X| = 34,650` against `kernel.count()`, `|A| = 4`
/// asserted at the root. The void-free capacity kernel with hidden slots in
/// offset order 1,2,3 from focal, mirroring the receipted path.
fn kernel_of(c: &Coord) -> Kernel {
    let focal = c.focal;
    let mut hidden = [Hidden {
        seat: focal,
        capacity: 0,
        voids: ContextSet::EMPTY,
    }; HIDDEN_SEATS];
    let mut pool = DominoSet::EMPTY;
    for (slot, k) in hidden.iter_mut().zip(1..=3usize) {
        let seat = focal.plus(k);
        *slot = Hidden {
            seat,
            capacity: c.rest[seat.index()].len(),
            voids: ContextSet::EMPTY,
        };
        pool = pool.union(c.rest[seat.index()]);
    }
    Kernel::new(c.decl, focal, c.rest[focal.index()], pool, hidden)
        .expect("(SS-R4) the void-free capacity kernel is well formed")
}

// == the wall-clock stop (SS-A5(ii)): provenance, never a receipt ===========

/// A declared per-unit wall-clock deadline. It TERMINATES a unit and files it
/// as a DECLARED STOP; it NEVER truncates a value — inside the two walks this
/// probe owns it returns `None` on the freeze-44 no-partial-fold path, so
/// every accumulator is discarded entire, exactly as budget exhaustion does.
///
/// SCOPE, printed in the artifact and named rather than assumed. `T_pass`
/// governs THE UNIT'S OWN PASSES — PATH A, PATH B, the four rule walks and the
/// count-only pass — where the check is a GENUINE INTERRUPT on the freeze-44
/// no-partial-fold path. The seed's SHARED COORDINATE SOLVE (the H and revealed
/// passes of `walt-strat`, which produce (M1) and (M3) for all four of that
/// seed's units) carries the freeze-44 walk-step budget and has no wall-clock
/// interrupt available to it; its elapsed time is TIMED AND RECORDED as
/// provenance and is not a stop, because a timer that fires only AFTER a
/// completed solve could do nothing but DISCARD A CORRECT VALUE — which is the
/// one thing SS-A5(ii) forbids a wall-clock stop from doing.
struct Deadline {
    at: Option<Instant>,
    ticks: Cell<u64>,
}

impl Deadline {
    fn new(start: Instant, spent: std::time::Duration) -> Deadline {
        let budget = std::time::Duration::from_secs(t_pass_secs());
        Deadline {
            at: budget.checked_sub(spent).map(|left| start + left),
            ticks: Cell::new(0),
        }
    }

    /// True when the unit is past its declared wall.
    fn expired(&self) -> bool {
        match self.at {
            None => true,
            Some(at) => Instant::now() > at,
        }
    }

    /// The in-walk check, sampled so it costs nothing measurable.
    fn hit(&self) -> bool {
        let t = self.ticks.get().wrapping_add(1);
        self.ticks.set(t);
        if t & 0x000F_FFFF != 0 {
            return false;
        }
        self.expired()
    }
}

// == the rule algebra: one node, one walker (mirrored from fusion_tax) ======

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

    /// The world-informed revealed continuation below a node.
    fn rev(self, node: Node, budget: &mut u64, dl: &Deadline) -> Option<i64> {
        if *budget == 0 || dl.hit() {
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
            return Some(inc + self.rev(next, budget, dl)?);
        }
        let seat = node.seat();
        let legal = self.legal_at(node, seat);
        if seat == self.focal {
            let mut best = i64::MIN;
            for d in legal.iter() {
                let v = self.rev(node.child(seat, d), budget, dl)?;
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
            sum += self.rev(node.child(seat, d), budget, dl)?;
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

// == PATH A: the depth-one frontier census (M4, M5) ========================

/// One frontier information state's accumulated exact data. The record — the
/// plays since the kernel decision point with the root action first (freeze
/// 26's observation contract, freeze 36(b)) — is the map key and IS the
/// information state.
///
/// NO PER-`(state, world)` ROWS ARE RETAINED (SS-A8's deliberate omission):
/// the tie census and the argmax intersection are folded in place.
struct FrontierState {
    /// `A(I)`, asserted constant across the latent worlds of `I`.
    legal: DominoSet,
    /// The scaled increment banked before the frontier; a function of the
    /// record alone, asserted equal on every arrival.
    prefix: i64,
    /// `|X_I|`: latent worlds arriving here.
    n_worlds: u64,
    /// `sum_omega DEN_MU/den`.
    acc_p: i128,
    /// `sum_omega (DEN_MU/den) * m_I(omega)`, scaled by `SCALE`.
    acc_m: i128,
    /// `sum_omega (DEN_MU/den) * q_I(omega,b)` per action of `legal`.
    acc_q: Vec<i128>,
    /// The running intersection of the COMPLETE per-world clairvoyant argmax
    /// sets. Its emptiness is Corollary 5.2's criterion and (SS-R6)'s subject.
    inter: DominoSet,
    /// THE HEADLINE STATISTIC's numerator at this state: arrivals whose
    /// COMPLETE clairvoyant argmax set is NON-SINGLETON.
    nonsingleton: u64,
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
            nonsingleton: 0,
        }
    }

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
}

/// THE COMPLETE clairvoyant argmax face `argmax_b q_I(omega,b)` at ONE world.
///
/// SS-A6(vi) and FC-A11(ii) are binding on this function and it is the one
/// hazard of the build: the set is accumulated BY AN EQUALITY TEST ACROSS ALL
/// CANDIDATES and NO INDEX IS TRACKED. The natural Rust idiom —
/// `iter().enumerate().max_by_key(..)` — IS the defect, because it returns ONE
/// index; a collapsed face would make the tie multiplicity zero everywhere and
/// would make the `delta_I = 0` states report EMPTY intersections almost
/// everywhere, which is what (SS-R6) catches loudly.
fn argmax_face(legal: DominoSet, child: &[i64]) -> DominoSet {
    let mut best = i64::MIN;
    for (j, _) in legal.iter().enumerate() {
        if child[j] > best {
            best = child[j];
        }
    }
    let mut out = DominoSet::EMPTY;
    for (j, d) in legal.iter().enumerate() {
        if child[j] == best {
            out.insert(d);
        }
    }
    assert!(
        !out.is_empty(),
        "stop-and-report: an argmax face is empty, which A(I) nonempty forbids"
    );
    out
}

#[derive(Clone, Copy)]
struct Arrival {
    den: u64,
    prefix: i64,
    seen_focal: bool,
}

struct Recorder<'a> {
    ctx: Ctx,
    states: &'a mut BTreeMap<Vec<Domino>, FrontierState>,
}

impl Recorder<'_> {
    fn walk(
        &mut self,
        node: Node,
        arr: Arrival,
        obs: &mut Vec<Domino>,
        budget: &mut u64,
        dl: &Deadline,
    ) -> Option<i64> {
        if *budget == 0 || dl.hit() {
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
            return Some(inc + self.walk(next, below, obs, budget, dl)?);
        }
        let seat = node.seat();
        let legal = self.ctx.legal_at(node, seat);
        if seat == self.ctx.focal {
            if arr.seen_focal {
                let mut best = i64::MIN;
                for d in legal.iter() {
                    obs.push(d);
                    let v = self.walk(node.child(seat, d), arr, obs, budget, dl);
                    obs.pop();
                    let v = v?;
                    if v > best {
                        best = v;
                    }
                }
                return Some(best);
            }
            return self.at_frontier(node, arr, obs, legal, budget, dl);
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
            let v = self.walk(node.child(seat, d), below, obs, budget, dl);
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

    fn at_frontier(
        &mut self,
        node: Node,
        arr: Arrival,
        obs: &mut Vec<Domino>,
        legal: DominoSet,
        budget: &mut u64,
        dl: &Deadline,
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
            obs.push(d);
            let v = self.walk(node.child(seat, d), below, obs, budget, dl);
            obs.pop();
            let v = v?;
            child[j] = v;
            if v > best {
                best = v;
            }
        }
        let w = DEN_MU / i128::from(arr.den);
        assert_eq!(
            w * i128::from(arr.den),
            DEN_MU,
            "freeze 38(f): DEN_MU = 12^6 carries every pre-frontier arrival denominator"
        );
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
        entry.n_worlds += 1;
        entry.acc_p += w;
        entry.acc_m += w * i128::from(arr.prefix + best);
        for (j, _) in legal.iter().enumerate() {
            entry.acc_q[j] += w * i128::from(arr.prefix + child[j]);
        }
        // THE HAZARD, guarded: the COMPLETE face, by equality across all
        // candidates, no index tracked (SS-A6(vi), FC-A11(ii)).
        let face = argmax_face(legal, &child);
        if face.len() > 1 {
            entry.nonsingleton += 1;
        }
        entry.inter = entry.inter.intersection(face);
        Some(best)
    }
}

struct PathA {
    states: BTreeMap<Vec<Domino>, FrontierState>,
    world_fold: i128,
    steps: u64,
    residual: u64,
}

fn path_a(
    ctx: Ctx,
    worlds: &[[DominoSet; Seat::COUNT]],
    root: Domino,
    dl: &Deadline,
) -> Option<PathA> {
    let mut states: BTreeMap<Vec<Domino>, FrontierState> = BTreeMap::new();
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
        let mut obs = Vec::with_capacity(16);
        obs.push(root);
        let mut rec = Recorder {
            ctx,
            states: &mut states,
        };
        let v = rec.walk(node, arr, &mut obs, &mut budget, dl)?;
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

// == PATH B: glue one, then reveal — U^(1), hence an INDEPENDENT Delta^(2) ==

#[derive(Clone, Copy)]
struct Pooled {
    den: u64,
    hands: [DominoSet; Seat::COUNT],
}

struct Glue {
    ctx: Ctx,
}

impl Glue {
    /// The `C^(1)` value below `node`: LAWFUL at the focal seat's first
    /// decision below the root — one common action per information state,
    /// `max_b` taken OUTSIDE the world sum — and world-informed below it.
    #[allow(clippy::too_many_arguments)]
    fn walk(
        &self,
        support: &[Pooled],
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        prefix: i64,
        budget: &mut u64,
        dl: &Deadline,
    ) -> Option<i128> {
        let cost = u64::try_from(support.len()).expect("a pooled bag fits u64");
        if *budget < cost || dl.hit() {
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
                "stop-and-report: T_a = 0 fails — the hand ended before the focal seat acted again"
            );
            return self.walk(
                support,
                winner,
                [Domino::ALL[0]; 4],
                0,
                prefix + inc,
                budget,
                dl,
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
                    let v = self.ctx.rev(node, budget, dl)?;
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
            sum += self.walk(&sup, leader, t, k + 1, prefix, budget, dl)?;
        }
        Some(sum)
    }
}

struct PathB {
    total: i128,
    steps: u64,
    residual: u64,
}

fn path_b(
    ctx: Ctx,
    worlds: &[[DominoSet; Seat::COUNT]],
    root: Domino,
    dl: &Deadline,
) -> Option<PathB> {
    let mut support: Vec<Pooled> = Vec::with_capacity(worlds.len());
    for hands in worlds {
        let mut hands = *hands;
        hands[ctx.focal.index()].remove(root);
        support.push(Pooled { den: 1, hands });
    }
    let mut tiles = [Domino::ALL[0]; 4];
    tiles[0] = root;
    let mut budget = B_WALK;
    let glue = Glue { ctx };
    let total = glue.walk(&support, ctx.focal, tiles, 1, 0, &mut budget, dl)?;
    Some(PathB {
        total,
        steps: B_WALK - budget,
        residual: budget,
    })
}

// == freezes 46/49: the four frozen rule arms ==============================

/// The trick context a record implies at a focal decision: the tiles already
/// on the table this trick and how many. Transcribed from `rule_economy_n4.rs`
/// (FC-A11(ii): mirror the receipted path).
fn trick_context(decl: Decl, viewer: Seat, record: &[Domino]) -> ([Domino; 4], usize) {
    // The record starts at the root action, with the viewer on lead.
    let mut leader = viewer;
    let mut tiles = [Domino::ALL[0]; 4];
    let mut k = 0usize;
    for d in record {
        tiles[k] = *d;
        k += 1;
        if k == 4 {
            let trick = Trick::new(leader, tiles).expect("distinct tiles by construction");
            leader = trick.winner(decl);
            tiles = [Domino::ALL[0]; 4];
            k = 0;
        }
    }
    (tiles, k)
}

fn rule_choice(
    arm: usize,
    decl: Decl,
    viewer: Seat,
    record: &[Domino],
    legal: DominoSet,
) -> Domino {
    let least = || {
        legal
            .iter()
            .min_by_key(|d| d.index())
            .expect("a live state has a legal move")
    };
    match arm {
        1 => least(),
        2 => legal
            .iter()
            .max_by_key(|d| d.index())
            .expect("a live state has a legal move"),
        3 => {
            let (tiles, k) = trick_context(decl, viewer, record);
            if k == 0 {
                return least();
            }
            let led = decl.led_context(tiles[0]);
            legal
                .iter()
                .filter(|c| (0..k).all(|i| decl.beats(led, tiles[i]).contains(*c)))
                .min_by_key(|d| d.index())
                .unwrap_or_else(least)
        }
        4 => {
            let Decl::PipTrump(p) = decl else {
                panic!("freeze 54(a) declares PipTrump only")
            };
            legal
                .iter()
                .filter(|d| !d.has(p))
                .min_by_key(|d| d.index())
                .unwrap_or_else(least)
        }
        _ => unreachable!("arms are P1..P4"),
    }
}

// == one unit ==============================================================

/// Everything one unit produces. Every value is an exact rational or an exact
/// integer; nothing here is a bound, an estimate or a sample.
struct UnitOut {
    action: Domino,
    /// `ok` or a DECLARED STOP string. A stopped unit contributes to no
    /// aggregate (SS-A5(ii)).
    status: String,
    // (M1)-(M3), count convention.
    qh: Q,
    uc: Q,
    gap: Q,
    margin: Q,
    argmax_h: String,
    verdict: String,
    // (M4).
    n_states: u64,
    n_arrivals: u64,
    a_dist: [u64; 4],
    nonsingleton: u64,
    // (M5), count convention.
    delta1: Q,
    delta2: Q,
    tax_support: u64,
    zero_forced: u64,
    zero_common: u64,
    u1_b: Q,
    mass: Q,
    // (M6), count convention.
    rules: [Q; 5],
    // (M7).
    part_count: u64,
    part_digest: u128,
    priced: bool,
    // walk-step observables (SEP-A19(b)'s class; never a cost claim).
    steps_a: u64,
    steps_b: u64,
    residual_a: u64,
    residual_b: u64,
    count_residual: u64,
    h_residual: u64,
    revealed_steps: u64,
    /// The companion's frontier rows for this unit, in freeze-50(c) order.
    rows: Vec<String>,
}

fn stopped(action: Domino, what: &str) -> UnitOut {
    UnitOut {
        action,
        status: what.to_owned(),
        qh: qi(0),
        uc: qi(0),
        gap: qi(0),
        margin: qi(0),
        argmax_h: String::new(),
        verdict: "DECLARED-STOP".to_owned(),
        n_states: 0,
        n_arrivals: 0,
        a_dist: [0; 4],
        nonsingleton: 0,
        delta1: qi(0),
        delta2: qi(0),
        tax_support: 0,
        zero_forced: 0,
        zero_common: 0,
        u1_b: qi(0),
        mass: qi(0),
        rules: [qi(0); 5],
        part_count: 0,
        part_digest: 0,
        priced: false,
        steps_a: 0,
        steps_b: 0,
        residual_a: 0,
        residual_b: 0,
        count_residual: 0,
        h_residual: 0,
        revealed_steps: 0,
        rows: Vec::new(),
    }
}

/// The coordinate-level solve (M1, M3), shared by the seed's four units and
/// charged to each of them for the wall-clock declaration.
struct CoordSolve {
    actions: Vec<Domino>,
    qh_diff: Vec<Q>,
    uc_diff: Vec<Q>,
    h_residuals: Vec<u64>,
    revealed_steps: Vec<u64>,
}

fn coord_solve(kernel: &Kernel, dir: &Direction) -> Option<CoordSolve> {
    let mut revealed_budget = 4 * B_WALK;
    let mut revealed_stop = None;
    let prices = information_prices(
        kernel,
        kernel.viewer().team(),
        dir,
        B_WALK,
        &mut revealed_budget,
        &mut revealed_stop,
    )?;
    let actions: Vec<Domino> = kernel.viewer_hand().iter().collect();
    for (i, a) in actions.iter().enumerate() {
        assert_eq!(prices.q_h[i].0, *a, "H action order is ascending domino");
        assert_eq!(prices.q_c[i].0, *a, "C action order is ascending domino");
    }
    Some(CoordSolve {
        qh_diff: prices.q_h.iter().map(|(_, e)| e.eval(qi(0))).collect(),
        uc_diff: prices.q_c.iter().map(|(_, e)| e.eval(qi(0))).collect(),
        h_residuals: prices.h_residuals.clone(),
        revealed_steps: prices
            .revealed_action_steps
            .iter()
            .map(|(_, s)| *s)
            .collect(),
        actions,
    })
}

#[allow(clippy::too_many_lines)]
fn run_unit(c: &Coord, kernel: &Kernel, dir: &Direction, solve: &CoordSolve, ai: usize) -> UnitOut {
    let root = solve.actions[ai];
    let start = Instant::now();
    // T_pass governs the unit's OWN passes; the seed's shared coordinate solve
    // is timed and recorded rather than stopped (see `Deadline`).
    let dl = Deadline::new(start, std::time::Duration::ZERO);
    // SS-A6(v): PATH B, the independently written glue-one-then-reveal walker,
    // runs on the DECLARED SAMPLE — the first unit of every block — where it
    // strengthens (SS-R5) to an independently computed Delta^(2). Everywhere
    // else Delta^(2) is Corollary FT-grade4's, and (SS-R5)'s content is carried
    // by the frontier table's own reconstruction of U^C, which is asserted at
    // EVERY unit. The sample is a function of the seed and the action index
    // alone, so it is deterministic and checkpointable.
    let path_b_sample = c.seed.is_multiple_of(BLOCK) && ai == 0;

    let worlds: Vec<[DominoSet; Seat::COUNT]> = kernel.worlds().map(|w| w.hands()).collect();
    let n_worlds = i128::try_from(worlds.len()).expect("fiber size fits i128");
    let ctx = Ctx {
        decl: kernel.decl(),
        focal: kernel.viewer(),
        team: kernel.viewer().team(),
    };

    // (M2) the complete H-argmax set and the exact margin against the best
    // competitor. SS-A4(b)(M2): walt's separation sense, never D3's.
    let qh: Vec<Q> = solve
        .qh_diff
        .iter()
        .map(|d| to_count(*d, N4_GRADE))
        .collect();
    let uc: Vec<Q> = solve
        .uc_diff
        .iter()
        .map(|d| to_count(*d, N4_GRADE))
        .collect();
    let vh = qh.iter().copied().max().expect("A is nonempty");
    let argmax_h: Vec<Domino> = solve
        .actions
        .iter()
        .zip(&qh)
        .filter(|(_, v)| **v == vh)
        .map(|(a, _)| *a)
        .collect();
    let best_competitor = qh
        .iter()
        .enumerate()
        .filter(|(j, _)| *j != ai)
        .map(|(_, v)| *v)
        .max()
        .expect("three competitors");
    let margin = qh[ai] - best_competitor;
    let verdict = if qh[ai] < vh {
        "DOMINATED"
    } else if argmax_h.len() == 1 {
        "UNIQUE-OPTIMAL"
    } else {
        "TIED-OPTIMAL"
    };
    let gap = uc[ai] - qh[ai];
    assert!(
        gap >= qi(0),
        "arithmetic remark (SS-A6(x), cannot fail): the fusion gap is nonnegative"
    );

    // (M4)/(M5) PATH A: the depth-one frontier census and Delta^(1).
    let Some(a) = path_a(ctx, &worlds, root, &dl) else {
        return stopped(
            root,
            if dl.expired() {
                "stop:wall-clock(path-A)"
            } else {
                "stop:budget(path-A)"
            },
        );
    };
    let n_states = u64::try_from(a.states.len()).expect("state count fits u64");

    let norm = DEN_MU * i128::from(SCALE) * n_worlds;
    let mut sum_p: i128 = 0;
    let mut sum_m: i128 = 0;
    let mut sum_best: i128 = 0;
    let mut n_arrivals: u64 = 0;
    let mut nonsingleton: u64 = 0;
    let mut a_dist = [0u64; 4];
    let mut tax_support: u64 = 0;
    let mut zero_forced: u64 = 0;
    let mut zero_common: u64 = 0;
    let mut rows: Vec<String> = Vec::with_capacity(a.states.len());

    for (record, st) in &a.states {
        let best_q = st.best_q();
        let dtax = st.acc_m - best_q;
        assert!(
            dtax >= 0,
            "arithmetic remark (SS-A6(x)): a local tax is nonnegative"
        );
        sum_p += st.acc_p;
        sum_m += st.acc_m;
        sum_best += best_q;
        n_arrivals += st.n_worlds;
        nonsingleton += st.nonsingleton;
        a_dist[st.legal.len()] += 1;

        // (SS-R6) THE COMPLETE-FACE RECEIPT: Corollary 5.2 BOTH WAYS at every
        // frontier state. Where delta_I = 0 the complete per-world argmax sets
        // INTERSECT; where delta_I > 0 they do NOT. A collapsed face is caught
        // here loudly.
        let common = !st.inter.is_empty();
        assert_eq!(
            dtax == 0,
            common,
            "(SS-R6) stop-and-report: Corollary 5.2 fails at seed {} root [{}] record [{}] — delta_I = {} but the complete argmax intersection is {{{}}}",
            c.seed,
            tile(root),
            record_str(record),
            dtax,
            tiles_str(st.inter)
        );
        if dtax == 0 {
            if st.legal.len() == 1 {
                zero_forced += 1;
            } else {
                zero_common += 1;
            }
        } else {
            tax_support += 1;
        }

        let p_i = Q::new(st.acc_p, DEN_MU * n_worlds);
        let delta_count = tax_to_count(Q::new(dtax, norm));
        rows.push(format!(
            "    seed={} a=[{}] I=[{}]  p_I = {}  |X_I| = {}  |A(I)| = {}  delta_I = {} (count)  argmax{{sum mu q}} = {{{}}}  complete-face intersection = {}  non-singleton clairvoyant argmax arrivals = {}/{}",
            c.seed,
            tile(root),
            record_str(record),
            qs(p_i),
            st.n_worlds,
            st.legal.len(),
            qs(delta_count),
            tiles_str(st.argmax_set()),
            if common {
                format!("{{{}}}", tiles_str(st.inter))
            } else {
                "EMPTY".to_owned()
            },
            st.nonsingleton,
            st.n_worlds
        ));
    }
    assert_eq!(
        u64::try_from(rows.len()).expect("rows fit u64"),
        n_states,
        "SS-A4(d) accounting: the companion carries one row per frontier state"
    );
    assert_eq!(
        tax_support + zero_forced + zero_common,
        n_states,
        "SS-A4(d) accounting: support + forced + common != |I_1|"
    );
    assert_eq!(
        a_dist.iter().sum::<u64>(),
        n_states,
        "SS-A4(d) accounting: the |A(I)| distribution does not total |I_1|"
    );
    assert_eq!(
        a_dist[0], 0,
        "stop-and-report: a frontier state with an empty legal set"
    );

    // Arithmetic remarks on the denominator bookkeeping (SS-A6(x) class).
    let mass = Q::new(sum_p, DEN_MU * n_worlds);
    assert_eq!(
        mass,
        qi(1),
        "stop-and-report: the frontier arrival mass is not 1"
    );
    assert_eq!(
        sum_m,
        DEN_MU * a.world_fold,
        "stop-and-report: the frontier accumulation and the per-world revealed fold disagree"
    );

    let u0_diff = Q::new(sum_m, norm);
    let delta1_diff = Q::new(sum_m - sum_best, norm);
    // The frontier table's own reconstruction of U^C, tied to the revealed
    // solve. This is what makes (SS-R5) contentful rather than definitional.
    assert_eq!(
        to_count(u0_diff, N4_GRADE),
        uc[ai],
        "(SS-R5) stop-and-report: the frontier table's U^(0) differs from the revealed solve's U^C at seed {} root [{}]",
        c.seed,
        tile(root)
    );

    // (M5) Delta^(1) from the frontier table; Delta^(2) by Corollary FT-grade4.
    let delta1 = tax_to_count(delta1_diff);
    let delta2 = (uc[ai] - qh[ai]) - delta1;
    assert!(
        delta1 >= qi(0) && delta2 >= qi(0),
        "arithmetic remark (SS-A6(x)): the two rung taxes are nonnegative"
    );
    // (SS-R5) THE LADDER RECEIPT: U^C - Q^H = Delta^(1) + Delta^(2), with
    // Delta^(1) summed from the frontier table and U^C, Q^H from their own
    // solves. Its CONTENT is the assertion above that the frontier table's own
    // U^(0) equals the revealed solve's U^C — three quantities from three
    // passes, tied by Corollary FT-grade4, failing on any error in the frontier
    // decomposition. On the declared sample PATH B strengthens it further by
    // computing U^(1), hence Delta^(2), from a pass of its own.
    assert_eq!(
        uc[ai] - qh[ai],
        delta1 + delta2,
        "(SS-R5) stop-and-report: U^C - Q^H != Delta^(1) + Delta^(2) at seed {} root [{}]",
        c.seed,
        tile(root)
    );
    let mut u1_b = uc[ai] - delta1;
    let mut steps_b = 0u64;
    let mut residual_b = 0u64;
    if path_b_sample {
        let Some(b) = path_b(ctx, &worlds, root, &dl) else {
            return stopped(
                root,
                if dl.expired() {
                    "stop:wall-clock(path-B)"
                } else {
                    "stop:budget(path-B)"
                },
            );
        };
        u1_b = to_count(Q::new(b.total, norm), N4_GRADE);
        steps_b = b.steps;
        residual_b = b.residual;
        assert_eq!(
            u1_b - qh[ai],
            delta2,
            "(SS-R5, strengthened on the declared sample) stop-and-report: PATH B's Delta^(2) differs from Corollary FT-grade4's at seed {} root [{}]",
            c.seed,
            tile(root)
        );
        assert_eq!(
            u1_b,
            uc[ai] - delta1,
            "(SS-R5, strengthened on the declared sample) stop-and-report: PATH B's U^(1) differs from U^C - Delta^(1)"
        );
    }

    // (M6) the four frozen rule arms and each one's gap to Q^H(b).
    let mut rules = [qi(0); 5];
    for (arm, slot) in rules.iter_mut().enumerate().skip(1) {
        if dl.expired() {
            return stopped(root, "stop:wall-clock(rules)");
        }
        let mut budget = B_WALK;
        let decl = kernel.decl();
        let viewer = kernel.viewer();
        let priced = walt_strat::policy_value_by_rule(
            kernel,
            kernel.viewer().team(),
            dir,
            root,
            &mut |record, legal| rule_choice(arm, decl, viewer, record, legal),
            &mut budget,
        );
        let Some((line, _)) = priced else {
            return stopped(root, "stop:budget(rule-walk)");
        };
        let l_count = to_count(line.eval(qi(0)), N4_GRADE);
        // (SS-R7) THE RULE BAR: a lawful rule policy cannot beat the lawful
        // optimum. A failure means the rule was evaluated against the wrong
        // field, belief or convention.
        assert!(
            l_count <= qh[ai],
            "(SS-R7) stop-and-report: {} prices {} > Q^H = {} at seed {} root [{}]",
            ARM_NAMES[arm],
            qs(l_count),
            qs(qh[ai]),
            c.seed,
            tile(root)
        );
        *slot = l_count;
    }

    // (M7) the count-only partition pass: exact state count and FNV-128
    // streaming digest at O(1) memory, its own budget, its own declared stop.
    if dl.expired() {
        return stopped(root, "stop:wall-clock(count-only)");
    }
    let mut cb = B_WALK;
    let Some((part_count, part_digest)) = InfoPartition::count_digest(kernel, root, &mut cb) else {
        return stopped(root, "stop:budget(count-only)");
    };
    // SS-A4(c): P_max gates the PRIMAL-WITNESS pipeline ONLY. Over-threshold
    // is recorded NOT PRICED on that route and never attempted; it does not
    // bar (M1)-(M6), which are all above and all complete.
    let priced = part_count <= P_MAX;

    UnitOut {
        action: root,
        status: "ok".to_owned(),
        qh: qh[ai],
        uc: uc[ai],
        gap,
        margin,
        argmax_h: argmax_h
            .iter()
            .map(|d| tile(*d))
            .collect::<Vec<_>>()
            .join(" "),
        verdict: verdict.to_owned(),
        n_states,
        n_arrivals,
        a_dist,
        nonsingleton,
        delta1,
        delta2,
        tax_support,
        zero_forced,
        zero_common,
        u1_b,
        mass,
        rules,
        part_count,
        part_digest,
        priced,
        steps_a: a.steps,
        steps_b,
        residual_a: a.residual,
        residual_b,
        count_residual: cb,
        h_residual: solve.h_residuals[ai],
        revealed_steps: solve.revealed_steps[ai],
        rows,
    }
}

// == the per-seed pass =====================================================

struct SeedOut {
    seed: usize,
    identity: String,
    units: Vec<UnitOut>,
    /// Provenance only, never a receipt and never a dividend.
    wall_ms: u128,
    /// The seed's SHARED coordinate solve (H + revealed), timed and recorded
    /// rather than stopped; and each unit's own passes. Provenance only.
    solve_ms: u128,
    unit_ms: Vec<u128>,
}

/// `only_first` runs the seed's FIRST UNIT alone — the (SS-R8) declared
/// sample's exact object, so the second pass re-runs a unit rather than a seed.
/// Every other stage (generator, (SS-R1), (SS-R3), (SS-R4), the coordinate
/// solve) is re-run in full and with fresh state either way.
fn run_seed(seed: usize) -> SeedOut {
    run_seed_inner(seed, false)
}

fn run_seed_inner(seed: usize, only_first: bool) -> SeedOut {
    let t0 = Instant::now();
    let c = generate(seed);
    r1_generator_soundness(&c);

    // (SS-R3) GENERATOR DETERMINISM: recompute the coordinate a second time
    // from the seed alone with fresh state and assert the freeze-45 identity
    // tuple byte-identical. Contentful across the whole survey, and cheap.
    let identity = identity_line(&c);
    let again = generate(seed);
    assert_eq!(
        identity,
        identity_line(&again),
        "(SS-R3) stop-and-report: the generator is not a function of the seed at seed {seed}"
    );

    // (SS-R4) COORDINATE IDENTITY.
    let kernel = kernel_of(&c);
    let rebuilt = kernel_of(&again);
    assert_eq!(
        kernel.viewer(),
        rebuilt.viewer(),
        "(SS-R4) the rebuilt kernel's viewer differs"
    );
    assert_eq!(
        kernel.viewer_hand(),
        rebuilt.viewer_hand(),
        "(SS-R4) the rebuilt kernel's focal hand differs"
    );
    assert_eq!(
        kernel.pool(),
        rebuilt.pool(),
        "(SS-R4) the rebuilt kernel's pool differs"
    );
    assert_eq!(
        kernel.count(),
        N4_FIBER,
        "(SS-R4) stop-and-report: |X| != 34,650 at seed {seed}"
    );
    assert_eq!(
        kernel.viewer_hand().len(),
        N4_GRADE,
        "(SS-R4) the declared grade is the coordinate's grade (N4-A11)"
    );
    // SS-A3: |A| = 4 exactly, ASSERTED IN-RUN and not assumed. The focal seat
    // leads at the coordinate, so its legal set is its whole hand; this is
    // contentful and fails if the coordinate is malformed.
    let root_legal = legal_plays(c.decl, kernel.viewer_hand(), None);
    assert_eq!(
        root_legal,
        kernel.viewer_hand(),
        "(SS-R4) stop-and-report: the focal seat is not on lead at seed {seed}"
    );
    assert_eq!(
        root_legal.len(),
        4,
        "(SS-R4)/SS-A3 stop-and-report: |A| != 4 at seed {seed}"
    );

    let dir = Direction::trick_diff();
    let t_solve = Instant::now();
    eprintln!("[stage] seed {seed}: coordinate solve (H + revealed) begins");
    let solve = coord_solve(&kernel, &dir);
    let solve_ms = t_solve.elapsed().as_millis();
    eprintln!("[stage] seed {seed}: coordinate solve done in {solve_ms} ms");
    let mut units = Vec::with_capacity(4);
    let mut unit_ms = Vec::with_capacity(4);
    match solve {
        None => {
            for a in root_legal.iter() {
                units.push(stopped(a, "stop:budget(coordinate-solve)"));
                unit_ms.push(0);
            }
        }
        Some(solve) => {
            let n = if only_first { 1 } else { solve.actions.len() };
            for ai in 0..n {
                let t_u = Instant::now();
                units.push(run_unit(&c, &kernel, &dir, &solve, ai));
                let ms = t_u.elapsed().as_millis();
                eprintln!("[stage] seed {seed}: unit {ai} own passes done in {ms} ms");
                unit_ms.push(ms);
            }
        }
    }
    SeedOut {
        seed,
        identity,
        units,
        wall_ms: t0.elapsed().as_millis(),
        solve_ms,
        unit_ms,
    }
}

// == checkpoints (SS-A4(e), DS-A36) ========================================
//
// The checkpoint block IS the fat data: it carries every companion row, so the
// companion is assembled by concatenating blocks in canonical order and no row
// is stored twice. Blocks live OUT OF TREE under FAT_ROOT.

fn fat_dir(sub: &str) -> PathBuf {
    PathBuf::from(FAT_ROOT).join(sub)
}

/// The checkpoint directory for THIS run. The canonical survey writes to
/// `ckpt/`; any partial range writes to its own directory, so a smoke pass can
/// exercise the write/resume path without ever polluting the survey's blocks.
static CKPT_DIR: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();

fn ckpt_dir() -> &'static PathBuf {
    CKPT_DIR
        .get()
        .expect("the checkpoint directory is set in main")
}

/// The seeds of `block` that THIS run covers. A block is durable when every one
/// of them is complete (SS-A4(e)); for the canonical full range that is all
/// `BLOCK` of them, and a partial range's blocks are written to their own
/// directory and are never loadable by the survey.
fn block_seeds_in(block: usize, lo: usize, hi: usize) -> Vec<usize> {
    (block * BLOCK..(block + 1) * BLOCK)
        .filter(|s| *s >= lo && *s <= hi)
        .collect()
}

fn block_path(block: usize) -> PathBuf {
    ckpt_dir().join(format!("block_{block:02}.txt"))
}

fn encode_unit(u: &UnitOut, w: &mut String) {
    let _ = writeln!(w, "unit={}", tile(u.action));
    let _ = writeln!(w, "status={}", u.status);
    let _ = writeln!(w, "qh={}", qs(u.qh));
    let _ = writeln!(w, "uc={}", qs(u.uc));
    let _ = writeln!(w, "gap={}", qs(u.gap));
    let _ = writeln!(w, "margin={}", qs(u.margin));
    let _ = writeln!(w, "argmaxh={}", u.argmax_h);
    let _ = writeln!(w, "verdict={}", u.verdict);
    let _ = writeln!(w, "n_states={}", u.n_states);
    let _ = writeln!(w, "n_arrivals={}", u.n_arrivals);
    let _ = writeln!(
        w,
        "a_dist={} {} {} {}",
        u.a_dist[0], u.a_dist[1], u.a_dist[2], u.a_dist[3]
    );
    let _ = writeln!(w, "nonsingleton={}", u.nonsingleton);
    let _ = writeln!(w, "delta1={}", qs(u.delta1));
    let _ = writeln!(w, "delta2={}", qs(u.delta2));
    let _ = writeln!(w, "tax_support={}", u.tax_support);
    let _ = writeln!(w, "zero_forced={}", u.zero_forced);
    let _ = writeln!(w, "zero_common={}", u.zero_common);
    let _ = writeln!(w, "u1b={}", qs(u.u1_b));
    let _ = writeln!(w, "mass={}", qs(u.mass));
    for arm in 1..=4usize {
        let _ = writeln!(w, "rule{arm}={}", qs(u.rules[arm]));
    }
    let _ = writeln!(w, "part_count={}", u.part_count);
    let _ = writeln!(w, "part_digest={:032x}", u.part_digest);
    let _ = writeln!(w, "priced={}", u8::from(u.priced));
    let _ = writeln!(w, "steps_a={}", u.steps_a);
    let _ = writeln!(w, "steps_b={}", u.steps_b);
    let _ = writeln!(w, "residual_a={}", u.residual_a);
    let _ = writeln!(w, "residual_b={}", u.residual_b);
    let _ = writeln!(w, "count_residual={}", u.count_residual);
    let _ = writeln!(w, "h_residual={}", u.h_residual);
    let _ = writeln!(w, "revealed_steps={}", u.revealed_steps);
    let _ = writeln!(w, "rows={}", u.rows.len());
    for r in &u.rows {
        let _ = writeln!(w, "{r}");
    }
}

fn encode_block(seeds: &[&SeedOut]) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "digest={SS_CKPT_DIGEST}");
    for so in seeds {
        let _ = writeln!(s, "seed={}", so.seed);
        let _ = writeln!(s, "identity={}", so.identity);
        let _ = writeln!(s, "wall_ms={}", so.wall_ms);
        let _ = writeln!(s, "solve_ms={}", so.solve_ms);
        let _ = writeln!(
            s,
            "unit_ms={}",
            so.unit_ms
                .iter()
                .map(u128::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        );
        let _ = writeln!(s, "units={}", so.units.len());
        for u in &so.units {
            encode_unit(u, &mut s);
        }
    }
    let _ = writeln!(s, "complete=yes");
    s
}

fn save_block(block: usize, seeds: &[&SeedOut]) {
    let dir = ckpt_dir();
    std::fs::create_dir_all(dir).expect("checkpoint dir");
    let body = encode_block(seeds);
    // Write-then-rename so a kill never leaves a torn checkpoint.
    let tmp = dir.join(format!("block_{block:02}.tmp"));
    std::fs::write(&tmp, &body).expect("checkpoint write");
    std::fs::rename(&tmp, block_path(block)).expect("checkpoint rename");
}

struct Lines<'a> {
    at: usize,
    v: Vec<&'a str>,
}

impl<'a> Lines<'a> {
    fn next_kv(&mut self) -> Option<(&'a str, &'a str)> {
        let l = self.v.get(self.at)?;
        self.at += 1;
        l.split_once('=')
    }

    fn expect(&mut self, key: &str) -> &'a str {
        let (k, v) = self.next_kv().expect("a checkpoint key");
        assert_eq!(k, key, "checkpoint field order: expected {key}");
        v
    }
}

fn decode_unit(l: &mut Lines<'_>) -> UnitOut {
    let a = l.expect("unit");
    let b = a.as_bytes();
    let action = Domino::new(
        Pip::new(b[0] - b'0').expect("pip"),
        Pip::new(b[1] - b'0').expect("pip"),
    );
    let status = l.expect("status").to_owned();
    let qh = parse_q(l.expect("qh"));
    let uc = parse_q(l.expect("uc"));
    let gap = parse_q(l.expect("gap"));
    let margin = parse_q(l.expect("margin"));
    let argmax_h = l.expect("argmaxh").to_owned();
    let verdict = l.expect("verdict").to_owned();
    let n_states = l.expect("n_states").parse().expect("u64");
    let n_arrivals = l.expect("n_arrivals").parse().expect("u64");
    let mut a_dist = [0u64; 4];
    for (slot, t) in a_dist.iter_mut().zip(l.expect("a_dist").split(' ')) {
        *slot = t.parse().expect("u64");
    }
    let nonsingleton = l.expect("nonsingleton").parse().expect("u64");
    let delta1 = parse_q(l.expect("delta1"));
    let delta2 = parse_q(l.expect("delta2"));
    let tax_support = l.expect("tax_support").parse().expect("u64");
    let zero_forced = l.expect("zero_forced").parse().expect("u64");
    let zero_common = l.expect("zero_common").parse().expect("u64");
    let u1_b = parse_q(l.expect("u1b"));
    let mass = parse_q(l.expect("mass"));
    let mut rules = [qi(0); 5];
    for (arm, slot) in rules.iter_mut().enumerate().skip(1) {
        *slot = parse_q(l.expect(&format!("rule{arm}")));
    }
    let part_count = l.expect("part_count").parse().expect("u64");
    let part_digest = u128::from_str_radix(l.expect("part_digest"), 16).expect("hex digest");
    let priced = l.expect("priced") == "1";
    let steps_a = l.expect("steps_a").parse().expect("u64");
    let steps_b = l.expect("steps_b").parse().expect("u64");
    let residual_a = l.expect("residual_a").parse().expect("u64");
    let residual_b = l.expect("residual_b").parse().expect("u64");
    let count_residual = l.expect("count_residual").parse().expect("u64");
    let h_residual = l.expect("h_residual").parse().expect("u64");
    let revealed_steps = l.expect("revealed_steps").parse().expect("u64");
    let nrows: usize = l.expect("rows").parse().expect("usize");
    let mut rows = Vec::with_capacity(nrows);
    for _ in 0..nrows {
        rows.push(l.v[l.at].to_owned());
        l.at += 1;
    }
    UnitOut {
        action,
        status,
        qh,
        uc,
        gap,
        margin,
        argmax_h,
        verdict,
        n_states,
        n_arrivals,
        a_dist,
        nonsingleton,
        delta1,
        delta2,
        tax_support,
        zero_forced,
        zero_common,
        u1_b,
        mass,
        rules,
        part_count,
        part_digest,
        priced,
        steps_a,
        steps_b,
        residual_a,
        residual_b,
        count_residual,
        h_residual,
        revealed_steps,
        rows,
    }
}

fn load_block(block: usize) -> Option<Vec<SeedOut>> {
    let s = std::fs::read_to_string(block_path(block)).ok()?;
    if !s.lines().any(|l| l == "complete=yes")
        || !s.lines().any(|l| l == format!("digest={SS_CKPT_DIGEST}"))
    {
        return None;
    }
    let v: Vec<&str> = s.lines().collect();
    let mut l = Lines { at: 0, v };
    assert_eq!(l.expect("digest"), SS_CKPT_DIGEST, "checkpoint digest");
    let mut out = Vec::new();
    while l.at < l.v.len() && l.v[l.at] != "complete=yes" {
        let seed = l.expect("seed").parse().expect("usize");
        let identity = l.expect("identity").to_owned();
        let wall_ms = l.expect("wall_ms").parse().expect("u128");
        let solve_ms = l.expect("solve_ms").parse().expect("u128");
        let unit_ms: Vec<u128> = l
            .expect("unit_ms")
            .split(' ')
            .filter(|t| !t.is_empty())
            .map(|t| t.parse().expect("u128"))
            .collect();
        let n: usize = l.expect("units").parse().expect("usize");
        let mut units = Vec::with_capacity(n);
        for _ in 0..n {
            units.push(decode_unit(&mut l));
        }
        out.push(SeedOut {
            seed,
            identity,
            units,
            wall_ms,
            solve_ms,
            unit_ms,
        });
    }
    Some(out)
}

/// DS-A30(ii): a record whose digest differs from the running freeze set is
/// CORRUPT, not stale — the cache is discarded ENTIRE, never partially reused.
/// Stray `.tmp` files (a kill mid-write) are removed.
fn validate_cache() {
    let dir = ckpt_dir();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut corrupt = false;
    for e in entries.flatten() {
        let p = e.path();
        if p.extension().is_some_and(|x| x == "tmp") {
            let _ = std::fs::remove_file(&p);
            continue;
        }
        let ok = std::fs::read_to_string(&p)
            .map(|s| s.lines().any(|l| l == format!("digest={SS_CKPT_DIGEST}")))
            .unwrap_or(false);
        if !ok {
            corrupt = true;
        }
    }
    if corrupt {
        eprintln!("[cache] digest mismatch: discarding the checkpoint cache entire (DS-A30)");
        let _ = std::fs::remove_dir_all(dir);
    }
}

// == emission ==============================================================

/// One thin committed row per unit. SS-A4(d): the accounting integers are on
/// the row, so the companion's omission is auditable from the committed file
/// alone.
fn thin_row(so: &SeedOut, u: &UnitOut) -> String {
    if u.status != "ok" {
        return format!(
            "unit seed={:03} a=[{}] DECLARED STOP ({}) — contributes to no aggregate (SS-A5(ii)); no value is truncated and none is reported",
            so.seed,
            tile(u.action),
            u.status
        );
    }
    format!(
        "unit seed={:03} a=[{}] verdict={} Q^H={} margin={} Opt^H={{{}}} U^C={} gap={} D1={} D2={} U^(1)={} |I_1|={} arrivals={} |A(I)|[1,2,3]={},{},{} forced={} tie-nonsingleton={}/{} tax-support={} zero[forced,common]={},{} rows={} P1={} P2={} P3={} P4={} rulegaps={},{},{},{} partition={} fnv128={:032x} path-B={} {} steps[A,B]={},{} residual[A,B]={},{} count-residual={} H-residual={} revealed-steps={}",
        so.seed,
        tile(u.action),
        u.verdict,
        qs(u.qh),
        qs(u.margin),
        u.argmax_h,
        qs(u.uc),
        qs(u.gap),
        qs(u.delta1),
        qs(u.delta2),
        qs(u.u1_b),
        u.n_states,
        u.n_arrivals,
        u.a_dist[1],
        u.a_dist[2],
        u.a_dist[3],
        u.a_dist[1],
        u.nonsingleton,
        u.n_arrivals,
        u.tax_support,
        u.zero_forced,
        u.zero_common,
        u.rows.len(),
        qs(u.rules[1]),
        qs(u.rules[2]),
        qs(u.rules[3]),
        qs(u.rules[4]),
        qs(u.qh - u.rules[1]),
        qs(u.qh - u.rules[2]),
        qs(u.qh - u.rules[3]),
        qs(u.qh - u.rules[4]),
        u.part_count,
        u.part_digest,
        if u.steps_b > 0 {
            "SAMPLED (independent U^(1); SS-R5 strengthened here)"
        } else {
            "not-sampled (Delta^(2) by Corollary FT-grade4; SS-R5 carried by U^(0) = U^C)"
        },
        if u.priced {
            "PRIMAL-WITNESS ROUTE ADMISSIBLE (N <= P_max v2)"
        } else {
            "NOT PRICED on the primal-witness route (N > P_max v2 = 192,000,000; never attempted; the solve, tie census and taxes on this row are unaffected — SS-A4(c))"
        },
        u.steps_a,
        u.steps_b,
        u.residual_a,
        u.residual_b,
        u.count_residual,
        u.h_residual,
        u.revealed_steps
    )
}

#[allow(clippy::too_many_lines)]
fn header(out: &mut String, lo: usize, hi: usize, partial: bool, workers: usize) {
    let _ = writeln!(
        out,
        "walt SS SEED SURVEY — freeze 54, a hundred fresh grade-4 coordinates by a declared arithmetic map"
    );
    let _ = writeln!(
        out,
        "rulings SS-A1..SS-A9 (walt/CENSUS-RULINGS.md, 2026-08-15); freeze 54 at SS-A4; under freezes 7/23, 26, 37(d), 38 v1.1, 44 v2, 45, 46/49, 50 v1.1, 52 v1.4, 53"
    );
    let _ = writeln!(
        out,
        "regenerate (thin file and companion, deterministically, from the repository alone): cargo run --release -p walt-factory --example seed_survey"
    );
    let _ = writeln!(out, "freeze-set digest (freeze 54(f)): {SS_DIGEST}");
    let _ = writeln!(out, "{COMPANION_LINE_PLACEHOLDER}");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "TIER: exploratory throughout, without exception (SS-A1). Cited by nothing above this tier. Quotable as a result only by brief amendment adding it to a verifier receipt; an external PASS is never imported as an axiom (TRUST-01). Every value below is an exact rational or an exact integer of this engine — no float exists anywhere in this probe (P-A19)."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "WHAT THIS IS (SS-A1): the FIRST carrier in this branch NOT SELECTED BY OUTCOME. Every previous n = 4 carrier was chosen by negative binding margin. These coordinates are selected by a declared arithmetic map from the natural numbers and by nothing else, and EVERY legal root action is a unit, so neither the coordinate nor the action is chosen by result. That is the survey's whole methodological point and it is worth more than any single number it produces."
    );
    let _ = writeln!(
        out,
        "WHAT THIS IS NOT (SS-A1): NOT A FEE MEASUREMENT. This pass measures whether tie multiplicity tracks separation structure across fresh coordinates; whether it tracks FEE CAPTURE needs fees, which is a later run. NO SENTENCE HERE SAYS THE SURVEY TESTED FEE VIABILITY. It is also not a distribution over 42: a hundred deals under one declared map is a CARRIER, and P-A21 binds — NOTHING MEASURED AT GRADE 4 IS QUOTED FOR TRICK 1 OR FOR THE OPENING."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "THE GENERATOR (freeze 54(a), every constant asserted). index(n) = (n * A) mod D with A = {A_SPREAD} and D = {D_DEALS}. SS-A2(ii): A is the least prime at or above D/phi, the golden-ratio multiplier — the standard low-discrepancy choice — FROZEN BEFORE THE BUILD AND NEVER CHOSEN BY A RESULT; D's prime factorisation contains only primes <= 23, so every prime greater than 28 is automatically coprime to D and A is a bijection on the index space. SS-A2(iii): any fixed spreading map is arbitrary and that is not a defect; choosing one after seeing results would be, and a successor changing A is running a DIFFERENT SURVEY and files it as one."
    );
    let _ = writeln!(
        out,
        "  THE DEFECT THIS REPAIRS, recorded because it would have voided the survey (SS-A2): the design as first written said \"seed n -> deal n by the canonical enumeration\". The first seat's hand does not change until index C(21,7)*C(14,7) = 399,072,960, so seeds 0..99 would have produced ONE HUNDRED DEALS SHARING AN IDENTICAL FIRST HAND — a survey of one deal sampled a hundred times."
    );
    let _ = writeln!(
        out,
        "  The unranking is the standard mixed radix (SS-A2(iv)): index split by division into (r0, r1, r2) over the radices C(21,7)*C(14,7), C(14,7), 1, then three combinadic unrankings giving seats 0, 1, 2 their hands and seat 3 the remainder. It REUSES the existing unrank_comb rather than a fresh implementation (FC-A11(ii): mirror the receipted path, do not re-derive it)."
    );
    let _ = writeln!(
        out,
        "  The rest, fixed as constants (SS-A2(v)): declaration PipTrump(n mod 7); seat 0 leads trick 1; the frozen dumb policy of freeze 26 — least legal domino index — plays THREE COMPLETE TRICKS, twelve tiles, leaving four in every hand; THE FOCAL SEAT IS THE WINNER OF TRICK 3, which is the seat on lead at the coordinate and makes the leader offset 0 automatically, matching freeze 45 rather than restating it."
    );
    let _ = writeln!(
        out,
        "  UNITS (SS-A3): the focal seat leads, so its legal set is its whole hand and |A| = 4 exactly at every coordinate — ASSERTED IN-RUN, NOT ASSUMED, which is contentful and fails if the coordinate is malformed. Four units per seed, no pair selection anywhere. COLLISIONS ARE FILED AS-IS, NEVER DEDUPLICATED: collision frequency is data about the generator, and deduplication would silently make the unit count depend on the results. Each seed emits its canonical coordinate key below, so collisions are computable after the fact by anyone, FROM THIS COMMITTED FILE ALONE."
    );
    let _ = writeln!(out);
    let _ = writeln!(out, "{SEPARATION_SENSE}");
    let _ = writeln!(
        out,
        "  UNIQUE-OPTIMAL: b is the sole member of the complete H-argmax set, and margin = Q^H(b) - max_{{a != b}} Q^H(a) > 0. TIED-OPTIMAL: b is in the complete H-argmax set and that set is not a singleton, so margin = 0. DOMINATED: b is not in the complete H-argmax set, so margin < 0. Every argmax set printed here is COMPLETE — accumulated by equality across all candidates — and never least-index broken (freeze 38(e))."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "BUDGETS AND STOPS (SS-A5). (i) Freeze 44(b) v2 binds UNCHANGED: B = {B_WALK} walk-steps per (coordinate, action) per evaluator, charge-then-descend, Option return, and ON EXHAUSTION NO PARTIAL FOLD OF ANY KIND — no partial tax, no partial census, no partial solve. NO NEW CONSTANT IS FIXED HERE. P_max v2 = {P_MAX}."
    );
    let _ = writeln!(
        out,
        "  (ii) WALL-CLOCK IS THE RUN OWNER'S TO DECLARE AND IS PROVENANCE, NEVER A RECEIPT (N4-A13, SEP-A19(b)). Declared and IN FORCE for this run: T_pass = {} seconds per unit; M_budget = {} GiB per block. These are RUN INPUTS, not constants of the probe (the separation_probe precedent), so SS-A5(i)'s bar on fixing a new constant is untouched. A wall-clock stop TERMINATES a unit and files it as a DECLARED STOP; IT NEVER TRUNCATES A VALUE. A stopped unit contributes no number to any census, ratio or aggregate, and every aggregate below NAMES THE UNIT SET IT RANGES OVER (FF-A18 as generalised) so a stop cannot silently shrink a denominator.",
        t_pass_secs(),
        m_budget_gib()
    );
    let _ = writeln!(
        out,
        "  WALL-CLOCK ACCOUNTING, named rather than assumed, in TWO STAGES under the ONE declared T_pass. STAGE 1, the seed's SHARED COORDINATE SOLVE (the walt-strat H and revealed passes, which produce (M1) and (M3) for all four of that seed's units): TIMED AND RECORDED, NOT STOPPED. It carries the freeze-44 walk-step budget and has no wall-clock interrupt available to it, so a timer there could fire only AFTER a completed solve and could do nothing but DISCARD A CORRECT VALUE — which is precisely what SS-A5(ii) forbids a wall-clock stop from doing. Its elapsed time is printed per seed in the provenance block. STAGE 2, each unit's OWN passes (PATH A, PATH B on the declared sample, the four rule walks, the count-only pass): governed by T_pass, checked at stage boundaries AND INSIDE PATH A AND PATH B, where the check returns on the freeze-44 no-partial-fold path so every accumulator is discarded entire — a GENUINE interrupt, not a truncation. The count-only pass and the rule walks are walt-strat evaluators: the check precedes them, and one that has begun runs to its walk-step budget."
    );
    let _ = writeln!(
        out,
        "  (iii) HEAVY TAILS BECOME VERDICTS: a unit that stops is a filed outcome under F7, printed as a stop and NEVER as a finding (R-A18), and the count of stops with their seeds is itself a recorded measurement."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "ALL OUTCOMES PRE-DECLARED (SS-A7, before any number exists; F7 binds):"
    );
    let _ = writeln!(
        out,
        "  (a) (SS-R1) or (SS-R2) fails -> stop-and-report BEFORE any unit runs; the generator is wrong and no coordinate is trustworthy."
    );
    let _ = writeln!(
        out,
        "  (b) Tie multiplicity TRACKS separation structure — units with high multiplicity systematically show smaller margins, more ties in Opt^H, or larger Delta^(2) share -> the screening statistic generalises off its home carrier and becomes the selection variable for fee work rather than a two-coordinate observation."
    );
    let _ = writeln!(
        out,
        "  (c) It DOES NOT track them -> Proposition FC-width's statistic is CARRIER-LOCAL, which is a result and a sharp one: the h2/h0 contrast would have been driven by something co-varying with multiplicity rather than by multiplicity, and the fee programme loses its cheap screen. THIS IS THE MORE INFORMATIVE OUTCOME AND IT IS NOT THE ONE WE EXPECT, which is exactly why it is written down before any number exists."
    );
    let _ = writeln!(
        out,
        "  (d) The relation is present but weak or non-monotone -> reported as measured with NO MECHANISM CLAIMED."
    );
    let _ = writeln!(
        out,
        "  (e) TAX SPARSITY OFF-CARRIER — the fraction of frontier states with delta_I > 0 at unselected units, against the 4.49% measured at five margin-selected ones -> the first out-of-carrier reading of that number, filed either way, AND NEVER QUOTED FOR TRICK 1 (P-A21)."
    );
    let _ = writeln!(
        out,
        "  (f) Stops -> declared, counted with their seeds, no partial anything. (g) Collisions -> counted and filed; a high collision rate is a fact about the dumb policy's funnelling and is reported as one, NOT as a defect."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "FENCES, verbatim (SS-A9): the R-A2/P-A1 fence; THE N4-A8 REAL-DEAL FENCE IN ITS AMENDED FORM — these hands do not come from rob's receipt corpus at all but from a declared arithmetic map, so they are FEASIBLE CONSTRUCTIONS AND NOT DEALS ANYONE PLAYED, and no row here is a statement about correct play in any hand; P-A21, no grade-4 quantity quoted for trick 1 or the opening; Proposition SR-degen, no verdict at grade 4 turns on any relaxation here; and SR-A25(vii)'s implementation-versus-corpus risk undiminished, with T1-A12's check still owed. NOT CLAIMED: nothing about points or marks (the valuation is the count-free trick differential; E-A2's boundary, and a count re-entry voids every form-keyed record wholesale); nothing about bidding; nothing about how real opponents play; no cost, timing, runtime or tractability claim read off any traversal observable (SEP-A19(b), N4-A16). Wall-clock below is provenance only."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "NAMED AS NON-RECEIPTS and printed as arithmetic remarks (SS-A6(x), Proposition SR-taut — they cannot fail): delta_I >= 0; Delta^(1) >= 0; the fusion gap >= 0; the tie fraction lying in [0,1]; and, ADDED BY SS-A11(iii), the ladder identity U^C - Q^H = Delta^(1) + Delta^(2) itself, which with Delta^(2) := gap - Delta^(1) is an identity in this probe's own recomputed quantities. It is still computed and still printed on every unit row; it is simply never counted among the receipts HELD. The adjudicator's original justification for (SS-R5) is left visible rather than rewritten (LD-A11(ii)), and the receipt's real content is the U^(0) = U^C comparison named above."
    );
    let _ = writeln!(out);
    if partial {
        let _ = writeln!(
            out,
            "*** PARTIAL RANGE — THIS IS NOT THE SURVEY. Declared seed set: {lo}..={hi}. Freeze 54(a) fixes the survey at seeds {SEED_LO}..={SEED_HI}; a run over any narrower set is a SMOKE PASS of the instrument and no aggregate below is the survey's aggregate. Every scope sentence names this seed set explicitly (SS-R9). ***"
        );
        let _ = writeln!(out);
    }
    let _ = writeln!(
        out,
        "run shape: seeds {lo}..={hi} ({} seeds, {} units), W = {workers} worker threads over seeds (seeds are independent; every emitted value is a pure function of its seed and the assembly below is in CANONICAL UNIT ORDER, never completion order — SS-A4(e), DS-A36).",
        hi - lo + 1,
        4 * (hi - lo + 1)
    );
}

// == SS-A10: the aggregation of the tie statistic ==========================

/// One COMPLETED unit's aggregation inputs. Every field is exact; there is no
/// sampling error anywhere in this survey (SS-A10(iii)), so a weighting choice
/// selects a population rather than a reliability.
struct UnitStat {
    verdict: String,
    /// Non-singleton complete-argmax arrivals over total arrivals, exact over
    /// this unit's COMPLETE arrival set.
    tie: BigRational,
    nonsingleton: u64,
    arrivals: u64,
    states: u64,
}

impl UnitStat {
    /// SS-A10(v)'s grouping: the separation-structure side of the comparison —
    /// units in `Opt^H` against DOMINATED units.
    fn in_opt(&self) -> bool {
        self.verdict != "DOMINATED"
    }
}

fn cell_lines(out: &mut String, label: &str, us: &[&UnitStat], lo: usize, hi: usize) {
    if us.is_empty() {
        let _ = writeln!(
            out,
            "  {label}: 0 units of seeds {lo}..={hi}; no figure printed."
        );
        return;
    }
    let fracs: Vec<BigRational> = us.iter().map(|u| u.tie.clone()).collect();
    let mean = mean_of(&fracs).expect("nonempty");
    let ns: u128 = us.iter().map(|u| u128::from(u.nonsingleton)).sum();
    let ar: u128 = us.iter().map(|u| u128::from(u.arrivals)).sum();
    let pooled = br(ns, ar);
    let mut st: Vec<u64> = us.iter().map(|u| u.states).collect();
    st.sort_unstable();
    let st_sum: u128 = st.iter().map(|x| u128::from(*x)).sum();
    let st_mean = br(st_sum, us.len() as u128);
    let _ = writeln!(
        out,
        "  {label}: {} completed unit(s) of seeds {lo}..={hi}. TIE MULTIPLICITY — per-unit mean (PRIMARY, SS-A10(ii)) {} = {}; arrival-pooled {} = {} (over {ns} of {ar} arrivals). FRONTIER SIZE, reported SIDE BY SIDE per SS-A10(v) over the SAME unit set — per-unit mean |I_1| {} (integer part {}); min {} / median {} / max {}.",
        us.len(),
        brs(&mean),
        ppm(&mean),
        brs(&pooled),
        ppm(&pooled),
        brs(&st_mean),
        st_mean.to_integer(),
        st[0],
        order_stat(&st, 1, 2),
        st[st.len() - 1]
    );
}

/// The contrast SS-A7(b) is about: the per-unit-mean tie fraction of units in
/// `Opt^H` less that of DOMINATED units. `None` when either group is absent.
fn contrast(us: &[&UnitStat]) -> Option<BigRational> {
    let a: Vec<BigRational> = us
        .iter()
        .filter(|u| u.in_opt())
        .map(|u| u.tie.clone())
        .collect();
    let b: Vec<BigRational> = us
        .iter()
        .filter(|u| !u.in_opt())
        .map(|u| u.tie.clone())
        .collect();
    Some(mean_of(&a)? - mean_of(&b)?)
}

fn sign_word(x: &BigRational) -> &'static str {
    match x.numer().sign() {
        num_bigint::Sign::Plus => "positive",
        num_bigint::Sign::Minus => "negative",
        num_bigint::Sign::NoSign => "zero",
    }
}

#[allow(clippy::too_many_lines)]
fn emit_aggregation(out: &mut String, us: &[UnitStat], unit_set: &str, lo: usize, hi: usize) {
    let all: Vec<&UnitStat> = us.iter().collect();
    let fracs: Vec<BigRational> = us.iter().map(|u| u.tie.clone()).collect();
    let tot_ns: u128 = us.iter().map(|u| u128::from(u.nonsingleton)).sum();
    let tot_arr: u128 = us.iter().map(|u| u128::from(u.arrivals)).sum();
    let tot_states: u128 = us.iter().map(|u| u128::from(u.states)).sum();

    let _ = writeln!(
        out,
        "AGGREGATION CONVENTION (SS-A10, ruled BEFORE any survey total existed). BOTH conventions are emitted and THE PER-UNIT MEAN IS PRIMARY. The reason is what the statistic is FOR, not which way any coordinate points: SS-A7(b) asks whether multiplicity tracks SEPARATION STRUCTURE, separation structure is carried by the UNIT — the verdict cell is per-unit, and a fee, if one is ever built, is built at a unit — so the unit is the observational unit and units weigh equally. Arrival-pooling answers a different question, the chance that a randomly drawn arrival of the pooled survey is non-singleton, and it weights units by frontier size, which is not what SS-A7(b) is about. The usual reason to weight by n does not apply here: THERE IS NO SAMPLING ERROR ANYWHERE IN THIS SURVEY — every per-unit fraction is an exact rational over that unit's COMPLETE arrival set — so a weighting choice selects a population, never a reliability (SS-A10(iii))."
    );
    let _ = writeln!(
        out,
        "BINDING (SS-A10(iv)): an SS-A7(b) association is reportable ONLY IF IT HOLDS UNDER BOTH CONVENTIONS. Where they disagree, THE DISAGREEMENT IS THE FINDING and is reported as such, never resolved by preferring one — an association that appears under unit weighting and vanishes under arrival weighting is telling you that FRONTIER SIZE, not tie multiplicity, is the thing associated, and that is a real and different result."
    );
    let _ = writeln!(out);

    if us.is_empty() {
        let _ = writeln!(
            out,
            "TIE MULTIPLICITY: no completed unit in this run; no figure printed."
        );
        return;
    }
    let mean = mean_of(&fracs).expect("nonempty");
    let pooled = br(tot_ns, tot_arr);
    let _ = writeln!(
        out,
        "TIE MULTIPLICITY, over {unit_set}, the {tot_arr} (state, world) arrivals they produced at the {tot_states} depth-one frontier states, and seeds {lo}..={hi}:"
    );
    let _ = writeln!(
        out,
        "  PRIMARY — per-unit mean of the exact per-unit fractions: {} = {} (unweighted, over {} units).",
        brs(&mean),
        ppm(&mean),
        us.len()
    );
    let _ = writeln!(
        out,
        "  SECONDARY — arrival-pooled: {} = {} ({tot_ns} of {tot_arr} arrivals). (Arithmetic remark, SS-A6(x): both fractions lie in [0,1] and cannot fail.)",
        brs(&pooled),
        ppm(&pooled)
    );
    let _ = writeln!(out);

    let _ = writeln!(
        out,
        "BY VERDICT CELL — the SS-A7(b)/(c)/(d) comparison, reported as measured with NO MECHANISM CLAIMED, each cell carrying BOTH conventions and the frontier-size association SIDE BY SIDE over the same unit set (SS-A10(v)). {SEPARATION_SENSE}"
    );
    let mut cells: BTreeMap<&str, Vec<&UnitStat>> = BTreeMap::new();
    for u in us {
        cells.entry(u.verdict.as_str()).or_default().push(u);
    }
    for (label, v) in &cells {
        cell_lines(out, label, v, lo, hi);
    }
    let _ = writeln!(out);

    // ---- the |I_1| stratification: the actual separability test -----------
    let mut st: Vec<u64> = us.iter().map(|u| u.states).collect();
    st.sort_unstable();
    let cuts = [
        order_stat(&st, 1, 4),
        order_stat(&st, 2, 4),
        order_stat(&st, 3, 4),
    ];
    let _ = writeln!(
        out,
        "THE CONFOUND CHECK (SS-A10(v)), the clause that decides whether this survey may attribute anything to tie multiplicity at all. Units are stratified by the SURVEY'S OWN |I_1| quartiles — order statistics of the observed |I_1| values, so every cut point is an EXACT OBSERVED VALUE and no two observations are ever averaged. Cut points over {unit_set}: {} / {} / {} (min {}, max {}). A verdict-cell contrast that survives INSIDE a stratum is separated from frontier size; one that exists only ACROSS strata is not.",
        cuts[0],
        cuts[1],
        cuts[2],
        st[0],
        st[st.len() - 1]
    );
    let strat = |u: &UnitStat| -> usize {
        if u.states <= cuts[0] {
            0
        } else if u.states <= cuts[1] {
            1
        } else if u.states <= cuts[2] {
            2
        } else {
            3
        }
    };
    let names = ["Q1 (smallest |I_1|)", "Q2", "Q3", "Q4 (largest |I_1|)"];
    let overall = contrast(&all);
    let mut mixed = 0usize;
    let mut agree = 0usize;
    // SS-A14(ii): the weakest link. A within-stratum contrast can rest on a
    // single unit, so the smallest per-cell n that any part of the separability
    // sentence rests on is tracked and named. No threshold and no test — a
    // threshold would be the invented inference this design refuses.
    let mut weakest: Option<usize> = None;
    for (k, name) in names.iter().enumerate() {
        let bucket: Vec<&UnitStat> = us.iter().filter(|u| strat(u) == k).collect();
        if bucket.is_empty() {
            let _ = writeln!(
                out,
                "  {name}: empty — |I_1| ties collapse this stratum, which is itself a fact about the carrier."
            );
            continue;
        }
        let n_opt = bucket.iter().filter(|u| u.in_opt()).count();
        let n_dom = bucket.len() - n_opt;
        let c = contrast(&bucket);
        let verdict_txt = match (&c, &overall) {
            (Some(c), Some(o)) => {
                mixed += 1;
                if sign_word(c) == sign_word(o) && sign_word(c) != "zero" {
                    agree += 1;
                }
                let link = n_opt.min(n_dom);
                weakest = Some(weakest.map_or(link, |w: usize| w.min(link)));
                format!(
                    "within-stratum contrast (per-unit-mean tie of Opt^H units LESS that of DOMINATED units) = {} = {} in magnitude, {} — the overall contrast is {}; THIS CONTRAST RESTS ON n = {n_opt} Opt^H unit(s) against n = {n_dom} DOMINATED unit(s), smaller cell {link} (SS-A14(ii))",
                    brs(c),
                    ppm(&c.abs()),
                    sign_word(c),
                    sign_word(o)
                )
            }
            _ => "no within-stratum contrast exists — this stratum holds units of only one group, so it can separate nothing".to_owned(),
        };
        let bucket_mean = mean_of(&bucket.iter().map(|u| u.tie.clone()).collect::<Vec<_>>())
            .map_or_else(|| "n/a".to_owned(), |m| brs(&m));
        let _ = writeln!(
            out,
            "  {name}: {} unit(s) ({n_opt} in Opt^H, {n_dom} DOMINATED); per-unit-mean tie fraction {bucket_mean}; {verdict_txt}.",
            bucket.len()
        );
    }

    // ---- the exact 2x2 collinearity count ---------------------------------
    let mut sorted_tie = fracs.clone();
    sorted_tie.sort();
    let med_tie = sorted_tie[sorted_tie.len() / 2].clone();
    let med_st = order_stat(&st, 1, 2);
    let (mut hh, mut hl, mut lh, mut ll) = (0u64, 0u64, 0u64, 0u64);
    for u in us {
        match (u.tie > med_tie, u.states > med_st) {
            (true, true) => hh += 1,
            (true, false) => hl += 1,
            (false, true) => lh += 1,
            (false, false) => ll += 1,
        }
    }
    // SS-A14(iii): all four cells, each labelled with its half-plane. The
    // DIRECTION is the informative part — seed 5 hinted at ANTI-correlation
    // (small frontier, high tie fraction) — and a bare concordance integer
    // would read a strong opposite-sign association as a weak one.
    // The marginals, printed so a DEGENERATE split is visible on sight: with
    // heavy ties in |I_1| a median can coincide with the maximum, throwing
    // every unit into one half-plane and making the four cells uninformative.
    // Printing the margins is the same discipline as printing the n.
    let (m_tie_hi, m_st_hi) = (hh + hl, hh + lh);
    let degenerate =
        m_tie_hi == 0 || m_st_hi == 0 || m_tie_hi == us.len() as u64 || m_st_hi == us.len() as u64;
    let lean = if degenerate {
        "is DEGENERATE — one median coincides with an extreme of its own distribution, so every unit falls in a single half-plane and the four cells carry no direction at all"
    } else if hh + ll > hl + lh {
        "leans CONCORDANT (large frontiers go with high tie fractions)"
    } else if hl + lh > hh + ll {
        "leans ANTI-CORRELATED (SMALL frontiers go with HIGH tie fractions), which is the direction seed 5 hinted at"
    } else {
        "is exactly balanced between the two directions"
    };
    let _ = writeln!(
        out,
        "  COLLINEARITY, exact counts over {unit_set}, all four cells with their half-planes (SS-A14(iii)). Units are split at the survey's own medians — tie fraction {}, |I_1| {}. [high tie, high |I_1|] = {hh}; [high tie, LOW |I_1|] = {hl}; [low tie, high |I_1|] = {lh}; [low tie, low |I_1|] = {ll}. Concordant (both high or both low) {} of {}; discordant {} of {}. MARGINALS, printed so a degenerate split is visible on sight: {m_tie_hi} of {} units lie strictly above the tie median and {m_st_hi} of {} strictly above the |I_1| median. The table {lean}. Integers only; this counts co-movement, IS NOT AN INFERENTIAL TEST, and claims no mechanism.",
        brs(&med_tie),
        med_st,
        hh + ll,
        us.len(),
        hl + lh,
        us.len(),
        us.len(),
        us.len()
    );
    // SS-A14(vi), the free companion: the MEDIAN of the per-unit fractions
    // needs only comparisons — no common denominator and no BigRational
    // arithmetic — and is robust to a handful of extreme units driving a mean.
    let _ = writeln!(
        out,
        "  MEDIAN of the per-unit fractions (SS-A14(vi), comparisons only, no common denominator): {} = {}, against the primary per-unit MEAN {} = {}. If these disagree materially the disagreement is worth a sentence in the reading, on SS-A10(iv)'s principle: a summary that changes with the summarising choice is telling you about the distribution rather than about the variable.",
        brs(&med_tie),
        ppm(&med_tie),
        brs(&mean),
        ppm(&mean)
    );

    // ---- the separability sentence, by the rule stated here ---------------
    let answer = if overall.is_none() {
        "NOT APPLICABLE — this unit set does not contain both groups, so no contrast exists at all."
    } else if mixed == 0 {
        "CANNOT ATTRIBUTE — no |I_1| stratum holds both groups, so tie multiplicity and frontier size are PERFECTLY CONFOUNDED at this carrier. The survey must credit neither variable, and per SS-A10(v) it says so rather than crediting the one we came in believing."
    } else if agree == mixed {
        "SEPARATES — every stratum holding both groups shows a contrast of the same nonzero sign as the overall contrast, so the association survives conditioning on frontier size."
    } else {
        "DOES NOT SEPARATE CLEANLY — the within-stratum contrasts do not all carry the overall sign, so this survey cannot attribute the association to tie multiplicity rather than to frontier size."
    };
    let _ = writeln!(
        out,
        "  DECLARED RULE, applied mechanically and stated before its answer is read: if NO |I_1| stratum contains units from both groups (Opt^H and DOMINATED), the two explanators are PERFECTLY CONFOUNDED at this carrier and the survey CANNOT ATTRIBUTE. Otherwise the within-stratum contrasts printed above stand, and the survey SEPARATES them only if every stratum holding both groups shows a contrast of the SAME NONZERO SIGN as the overall contrast; anything else is DOES NOT SEPARATE CLEANLY."
    );
    let weak_txt = weakest.map_or_else(
        || "no within-stratum contrast exists, so this sentence rests on no cell at all".to_owned(),
        |w| format!("THE WEAKEST LINK: the smallest per-cell n that any part of this sentence rests on is {w} unit(s) (SS-A14(ii)). A contrast carried by one unit and a contrast carried by a hundred read identically without this number, which is why it is printed"),
    );
    let _ = writeln!(
        out,
        "  DOES THIS SURVEY SEPARATE TIE MULTIPLICITY FROM FRONTIER SIZE? {answer} ({mixed} stratum/strata held both groups; {agree} of those agreed in sign with the overall contrast.) {weak_txt}."
    );
    let _ = writeln!(
        out,
        "  CARRIER MEMBERSHIP, said in place so no successor re-litigates it (SS-A14(iv)): SEED 5's FOUR UNITS ARE IN THIS CARRIER, in every total, every stratum and every verdict cell above, exactly like the units of every other seed. SS-A13(i) excluded the pre-run seed-5 OBSERVATION from appearing as evidence — ONE SENTENCE, NOT FOUR UNITS. The carrier is defined by the freeze-54 generating rule, and removing a seed because it was looked at early would be precisely the selection-by-result that SS-A1 says this survey exists to avoid."
    );
    let _ = writeln!(out);
}

// == main ==================================================================

#[allow(clippy::too_many_lines)]
fn main() {
    // The digest function is anchored before any number exists (SR-A33(iii)).
    sha256_self_check();

    let args: Vec<String> = std::env::args().collect();
    let mut lo = SEED_LO;
    let mut hi = SEED_HI;
    let mut workers = std::thread::available_parallelism()
        .map(std::num::NonZero::get)
        .unwrap_or(1)
        .min(16);
    for a in &args {
        if let Some(r) = a.strip_prefix("seeds=") {
            let (x, y) = r.split_once("..").expect("seeds=A..B, inclusive");
            lo = x.parse().expect("seed lo");
            hi = y.parse().expect("seed hi");
        }
        if let Some(w) = a.strip_prefix("workers=") {
            workers = w.parse().expect("workers=N");
        }
    }
    assert!(
        lo <= hi && hi <= SEED_HI,
        "freeze 54(a) fixes the survey at seeds {SEED_LO}..={SEED_HI}"
    );
    let partial = lo != SEED_LO || hi != SEED_HI;
    let seeds: Vec<usize> = (lo..=hi).collect();

    // ---- (SS-R2) THE SPREADING RECEIPT — BLOCKING, before any unit runs ---
    // Contentful: it fails on a mistyped A, and a mistyped A is exactly the
    // defect SS-A2 exists to prevent recurring silently.
    let g = gcd(A_SPREAD, D_DEALS);
    assert_eq!(
        g, 1,
        "(SS-R2) stop-and-report: gcd(A, D) = {g} != 1 — A is not a bijection on the index space"
    );
    let mut idx: BTreeMap<u128, usize> = BTreeMap::new();
    for n in SEED_LO..=SEED_HI {
        let i = spread_index(n);
        assert!(
            idx.insert(i, n).is_none(),
            "(SS-R2) stop-and-report: seeds collide on deal index {i}"
        );
    }
    assert_eq!(
        idx.len(),
        SEED_HI - SEED_LO + 1,
        "(SS-R2) the 100 deal indices are pairwise distinct"
    );
    let r2_line = format!(
        "(SS-R2) THE SPREADING RECEIPT — BLOCKING — HELD, over the whole frozen seed set {SEED_LO}..={SEED_HI} (100 seeds) and not merely this run's range: gcd(A, D) = 1 with A = {A_SPREAD}, D = {D_DEALS}; the 100 deal indices are PAIRWISE DISTINCT (min {}, max {}). Contentful: it fails on a mistyped A.",
        idx.keys().next().expect("nonempty"),
        idx.keys().next_back().expect("nonempty")
    );

    // ---- (SS-R1) GENERATOR SOUNDNESS — BLOCKING, before any unit runs -----
    // Over the WHOLE frozen seed set, not this run's range: a generator defect
    // is a defect of the survey, not of a sub-range.
    let mut first_hand_ranks: std::collections::BTreeSet<u128> = std::collections::BTreeSet::new();
    for n in SEED_LO..=SEED_HI {
        let c = generate(n);
        r1_generator_soundness(&c);
        first_hand_ranks.insert(spread_index(n) / (binom(21, 7) * binom(14, 7)));
    }
    let r1_line = format!(
        "(SS-R1) GENERATOR SOUNDNESS — BLOCKING — HELD at every one of the 100 frozen seeds {SEED_LO}..={SEED_HI} (not merely this run's range): the four hands partition all 28 dominoes (disjoint, seven each, union complete); every playout tile was LEGAL AT THE MOMENT IT WAS PLAYED, checked by an INDEPENDENT replay from the deal and the recorded twelve tiles rather than read off the policy that produced them; exactly twelve tiles played; exactly four remain in every hand; and the trick-3 winner is the focal seat. Contentful, and it is the check that catches an unranking error, which is otherwise invisible because a wrong deal is still a well-formed deal. Corroboration of SS-A2's repair, over the 100 frozen seeds: {} DISTINCT first-hand ranks of the 1,184,040 possible, spanning {} to {} — against the ONE value the unrepaired design would have given.",
        first_hand_ranks.len(),
        first_hand_ranks.iter().next().expect("nonempty"),
        first_hand_ranks.iter().next_back().expect("nonempty")
    );

    // ---- checkpoints: load complete blocks, compute the rest --------------
    CKPT_DIR
        .set(if partial {
            fat_dir(&format!("ckpt_smoke_seeds{lo}-{hi}"))
        } else {
            fat_dir("ckpt")
        })
        .expect("the checkpoint directory is set once");
    validate_cache();
    let blocks: Vec<usize> = {
        let mut b: Vec<usize> = seeds.iter().map(|s| s / BLOCK).collect();
        b.dedup();
        b
    };
    let mut loaded: BTreeMap<usize, SeedOut> = BTreeMap::new();
    let mut loaded_blocks: Vec<usize> = Vec::new();
    for b in &blocks {
        let want = block_seeds_in(*b, lo, hi);
        if let Some(v) = load_block(*b) {
            if v.iter().map(|s| s.seed).eq(want.iter().copied()) {
                loaded_blocks.push(*b);
                for so in v {
                    loaded.insert(so.seed, so);
                }
            }
        }
    }
    // DS-A36: a resumed run must validate byte-identity of the re-derived
    // generator outputs before quoting anything from a checkpoint.
    let mut resume_line =
        String::from("resume-validation: n/a (fresh run, no checkpoint block was loaded)");
    if !loaded.is_empty() {
        for (seed, so) in &loaded {
            let fresh = generate(*seed);
            r1_generator_soundness(&fresh);
            assert_eq!(
                identity_line(&fresh),
                so.identity,
                "DS-A36 stop-and-report: the re-derived generator output at seed {seed} is not byte-identical to its checkpoint — checkpointing defect, and the cache is not an authority"
            );
        }
        resume_line = format!(
            "resume-validation: PASS — {} seed(s) loaded from {} complete checkpoint block(s) {:?}; every loaded seed's generator output RE-DERIVED FROM THE SEED ALONE and asserted BYTE-IDENTICAL to its checkpoint (DS-A36; the cache is never an authority, DS-A30(iii)).",
            loaded.len(),
            loaded_blocks.len(),
            loaded_blocks
        );
    }

    let pending: Vec<usize> = seeds
        .iter()
        .copied()
        .filter(|s| !loaded.contains_key(s))
        .collect();
    // The declared worker count, kept before the solve phase narrows it: a
    // fully-resumed run has nothing pending and would otherwise drop to one
    // thread, which must not decide how (SS-R8)'s sample is scheduled.
    let w_declared = workers;
    let workers = workers.min(pending.len().max(1));
    let done: Mutex<BTreeMap<usize, SeedOut>> = Mutex::new(BTreeMap::new());
    let t_run = Instant::now();
    if !pending.is_empty() {
        let next = AtomicUsize::new(0);
        let next_ref = &next;
        let pending_ref = &pending;
        let done_ref = &done;
        let loaded_ref = &loaded;
        std::thread::scope(|scope| {
            for _ in 0..workers {
                scope.spawn(move || loop {
                    let i = next_ref.fetch_add(1, Ordering::Relaxed);
                    if i >= pending_ref.len() {
                        break;
                    }
                    let seed = pending_ref[i];
                    eprintln!("[progress] seed {seed} ({}/{})", i + 1, pending_ref.len());
                    let so = run_seed(seed);
                    let b = seed / BLOCK;
                    let mut d = done_ref.lock().expect("done map");
                    d.insert(seed, so);
                    // SS-A4(e): a block is durable WHEN COMPLETE. The morning
                    // has whatever completed and nothing partial.
                    let want = block_seeds_in(b, lo, hi);
                    if want
                        .iter()
                        .all(|s| d.contains_key(s) || loaded_ref.contains_key(s))
                    {
                        let refs: Vec<&SeedOut> = want
                            .iter()
                            .map(|s| {
                                d.get(s)
                                    .or_else(|| loaded_ref.get(s))
                                    .expect("a complete block")
                            })
                            .collect();
                        save_block(b, &refs);
                        eprintln!("[checkpoint] block {b:02} durable ({} seeds)", refs.len());
                    }
                });
            }
        });
    }
    let run_ms = t_run.elapsed().as_millis();
    let computed = done.into_inner().expect("done map");
    let mut all: BTreeMap<usize, SeedOut> = loaded;
    for (k, v) in computed {
        all.insert(k, v);
    }

    // ---- (SS-R8) DETERMINISM SAMPLE ---------------------------------------
    // A full in-run second pass with fresh maps, accumulators and budgets on a
    // DECLARED sample — THE FIRST UNIT OF EVERY BLOCK — every printed figure
    // asserted identical. Declared rather than universal because at 400 units
    // a universal second pass doubles the night; (SS-R3) covers the generator
    // at every seed regardless.
    let mut sample_seeds: Vec<usize> = blocks
        .iter()
        .filter_map(|b| seeds.iter().copied().find(|s| s / BLOCK == *b))
        .collect();
    sample_seeds.dedup();
    // The sample's units are INDEPENDENT — each re-solves its own coordinate
    // from the seed alone and asserts equality against its own checkpointed
    // row — so the loop is scheduled across workers. This is a SCHEDULE change
    // and not a measured-object change: which seeds are in the sample and how
    // many were re-run are deterministic functions of the seed set, and DS-A36
    // REQUIRES every emitted value to be independent of worker count, so
    // running the sample in parallel exercises that requirement rather than
    // straining it. Sequentially the tail is the SUM of the sample's solves;
    // in parallel it is their MAX. A panic in any worker is a stop-and-report
    // and propagates out of the scope, exactly as in the sequential form.
    let r8_counter = AtomicUsize::new(0);
    {
        let next = AtomicUsize::new(0);
        let next_ref = &next;
        let sample_ref = &sample_seeds;
        let all_ref = &all;
        let r8_ref = &r8_counter;
        let w = w_declared.min(sample_seeds.len().max(1));
        std::thread::scope(|scope| {
            for _ in 0..w {
                scope.spawn(move || loop {
                    let i = next_ref.fetch_add(1, Ordering::Relaxed);
                    if i >= sample_ref.len() {
                        break;
                    }
                    let s = sample_ref[i];
                    let first = &all_ref[&s].units[0];
                    if first.status != "ok" {
                        continue;
                    }
                    eprintln!("[SS-R8] declared-sample second pass at seed {s}");
                    let second = run_seed_inner(s, true);
                    assert_eq!(
                        thin_row(&all_ref[&s], first),
                        thin_row(&second, &second.units[0]),
                        "(SS-R8) stop-and-report: the second pass differs at seed {s}, first unit"
                    );
                    assert_eq!(
                        first.rows,
                        second.units[0].rows,
                        "(SS-R8) stop-and-report: the second pass's companion rows differ at seed {s}, first unit"
                    );
                    r8_ref.fetch_add(1, Ordering::Relaxed);
                });
            }
        });
    }
    let r8_units = r8_counter.into_inner();
    let r8_line = format!(
        "(SS-R8) DETERMINISM SAMPLE — HELD on the declared sample (the FIRST UNIT OF EVERY BLOCK present in this run: seeds {sample_seeds:?}, {r8_units} unit(s) re-run; stopped units are skipped and named): a full in-run SECOND PASS with fresh maps, accumulators and budgets, every printed figure and every companion row asserted IDENTICAL. Declared rather than universal because at 400 units a universal second pass doubles the night; (SS-R3) covers the generator at EVERY seed regardless. SCOPE, stated rather than implied (SS-A12(ii)): the sample's object is ONE UNIT — that coordinate's solve plus that unit's own passes, re-run from the seed alone — and it reaches accumulator reuse and iteration-order dependence within a unit's own path; per-unit content is a function of (kernel, budgets) alone and (SS-R3) covers the generator at every seed regardless. The sample was scheduled across {} worker thread(s); its units are independent and DS-A36 requires every emitted value to be independent of worker count, so the schedule is not a term in anything asserted here.",
        w_declared.min(sample_seeds.len().max(1))
    );

    // ---- assembly, in canonical unit order --------------------------------
    let mut out = String::new();
    header(&mut out, lo, hi, partial, workers);
    let _ = writeln!(out);
    let _ = writeln!(out, "== BLOCKING RECEIPTS (SS-A6(i)-(ii)) ==");
    let _ = writeln!(out, "{r1_line}");
    let _ = writeln!(out, "{r2_line}");
    let _ = writeln!(out);

    // The companion's body is accumulated ONCE, in canonical unit order. At 400
    // units the frontier rows are the bulk of this process's memory, so they are
    // never held in a second Vec: the bytes below are exactly the bytes written.
    let mut companion_body = String::new();
    let mut ok_units = 0u64;
    let mut stop_units: Vec<String> = Vec::new();
    let mut tot_arrivals: u128 = 0;
    let mut tot_nonsingleton: u128 = 0;
    let mut tot_states: u128 = 0;
    let mut tot_support: u128 = 0;
    let mut tot_forced: u128 = 0;
    let mut not_priced: Vec<String> = Vec::new();
    // SS-A10: the per-unit inputs to BOTH aggregation conventions. Every
    // COMPLETED unit contributes exactly one entry — including seed 5's four,
    // per SS-A14(iv) — and a DECLARED STOP contributes none.
    let mut ustats: Vec<UnitStat> = Vec::new();
    let mut by_verdict: BTreeMap<String, (u64, u128, u128, u128, u128)> = BTreeMap::new();
    let mut coord_keys: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let mut r6_positive = 0u64;
    let mut all_rows = 0u64;
    let mut path_b_units = 0u64;
    let mut stage_rows: Vec<String> = Vec::new();

    let _ = writeln!(
        out,
        "== THE SURVEY, in canonical unit order (seed ascending, then root action in ascending domino index) =="
    );
    for s in &seeds {
        let so = &all[s];
        let _ = writeln!(out, "coordinate {}", so.identity);
        stage_rows.push(format!(
            "  seed {:03}: shared coordinate solve (STAGE 1, timed and recorded, never stopped) {} ms; per-unit own passes (STAGE 2, governed by T_pass) {:?} ms; seed total {} ms",
            so.seed, so.solve_ms, so.unit_ms, so.wall_ms
        ));
        coord_keys
            .entry(coord_key(&so.identity))
            .or_default()
            .push(*s);
        for u in &so.units {
            let _ = writeln!(out, "  {}", thin_row(so, u));
            if u.status != "ok" {
                stop_units.push(format!(
                    "seed {} a=[{}] {}",
                    so.seed,
                    tile(u.action),
                    u.status
                ));
                continue;
            }
            ok_units += 1;
            tot_arrivals += u128::from(u.n_arrivals);
            tot_nonsingleton += u128::from(u.nonsingleton);
            tot_states += u128::from(u.n_states);
            tot_support += u128::from(u.tax_support);
            tot_forced += u128::from(u.a_dist[1]);
            all_rows += u64::try_from(u.rows.len()).expect("rows fit u64");
            if u.nonsingleton > 0 {
                r6_positive += 1;
            }
            if u.steps_b > 0 {
                path_b_units += 1;
            }
            if !u.priced {
                not_priced.push(format!(
                    "seed {} a=[{}] partition {} > P_max v2",
                    so.seed,
                    tile(u.action),
                    u.part_count
                ));
            }
            ustats.push(UnitStat {
                verdict: u.verdict.clone(),
                tie: br(u128::from(u.nonsingleton), u128::from(u.n_arrivals)),
                nonsingleton: u.nonsingleton,
                arrivals: u.n_arrivals,
                states: u.n_states,
            });
            let e = by_verdict
                .entry(u.verdict.clone())
                .or_insert((0, 0, 0, 0, 0));
            e.0 += 1;
            e.1 += u128::from(u.nonsingleton);
            e.2 += u128::from(u.n_arrivals);
            e.3 += u128::from(u.tax_support);
            e.4 += u128::from(u.n_states);
            for r in &u.rows {
                let _ = writeln!(companion_body, "{r}");
            }
        }
    }
    let _ = writeln!(out);

    // ---- the run-level receipts -------------------------------------------
    let _ = writeln!(out, "== RECEIPTS (SS-A6) ==");
    let unit_set = format!(
        "the {ok_units} COMPLETED units of the {} units of seeds {lo}..={hi} ({} DECLARED STOPS, named below and contributing to no aggregate)",
        4 * (hi - lo + 1),
        stop_units.len()
    );
    let _ = writeln!(
        out,
        "(SS-R3) GENERATOR DETERMINISM — HELD at every seed of {lo}..={hi}: each coordinate recomputed a SECOND TIME from the seed alone with fresh state and the freeze-45 identity tuple asserted BYTE-IDENTICAL. Contentful across the whole survey, and cheap."
    );
    let _ = writeln!(
        out,
        "(SS-R4) COORDINATE IDENTITY — HELD at every coordinate of seeds {lo}..={hi}: freeze 45's form (void-free capacity kernel, hidden slots in offset order 1,2,3 from focal, leader offset 0); |X| = {N4_FIBER} asserted against kernel.count(); the kernel REBUILT IN-RUN from an independently regenerated coordinate and asserted equal on viewer, focal hand and pool; and |A| = 4 asserted at the root against legal_plays with no led context."
    );
    let _ = writeln!(
        out,
        "(SS-R5) THE LADDER RECEIPT, IN ITS AMENDED FORM (SS-A11, which ratified this placement and corrected SS-A6(v)'s original justification) — HELD over {unit_set}. WHAT IS ASSERTED AND WHERE THE CONTENT LIVES: at EVERY unit the frontier table's OWN reconstruction U^(0) — folded from the depth-one arrival weights and the per-world revealed continuations of PATH A — is asserted EXACTLY EQUAL to the revealed solve's U^C. That is a comparison against something the checker did not produce, which is Proposition SR-taut's test, and it fails on any error in the frontier decomposition. STRENGTHENED ON THE DECLARED SAMPLE (the first unit of every block, {} unit(s) here): the independently written glue-one-then-reveal walker (PATH B) computes U^(1), hence Delta^(2), FROM A PASS OF ITS OWN, and both U^(1) = U^C - Delta^(1) and PATH B's Delta^(2) = Corollary FT-grade4's are asserted. PATH B is sampled rather than universal because it costs a second whole-fiber traversal per unit; the sample is a function of the seed and the action index alone, declared before any result and never chosen by one. NOT COUNTED AMONG THE RECEIPTS HELD, per SS-A11(iii): the identity U^C - Q^H = Delta^(1) + Delta^(2) itself, which with Delta^(2) := gap - Delta^(1) is an identity in the probe's own recomputed quantities and CANNOT FAIL — it is retained and printed below among SS-A6(x)'s arithmetic remarks, where it belongs.",
        path_b_units
    );
    let _ = writeln!(
        out,
        "(SS-R6) THE COMPLETE-FACE RECEIPT — HELD at EVERY ONE of the {tot_states} frontier states of {unit_set}: Corollary 5.2 asserted BOTH WAYS — where delta_I = 0 the complete per-world clairvoyant argmax sets INTERSECT, and where delta_I > 0 they do NOT. The construction accumulates a SET BY EQUALITY ACROSS ALL CANDIDATES AND TRACKS NO INDEX; max_by_key returning one index IS the defect (FC-A11(ii)), and a collapsed face is caught here loudly because collapsing to singletons makes the delta_I = 0 states report EMPTY intersections almost everywhere."
    );
    let _ = writeln!(
        out,
        "(SS-R6) THE RUN-LEVEL NON-NULL PAIRING (FC-A26(iv)'s discipline, required BY DESIGN rather than by luck) — {}: {r6_positive} of the {ok_units} completed units report a POSITIVE non-singleton clairvoyant argmax arrival count; the survey total is {tot_nonsingleton} of {tot_arrivals} arrivals. A stuck-at-singleton implementation would report a tie multiplicity of ZERO EVERYWHERE and would satisfy every other check.",
        if r6_positive > 0 { "HELD" } else { "FAILED" }
    );
    assert!(
        r6_positive > 0 || ok_units == 0,
        "(SS-R6) stop-and-report: the run-level non-null pairing FAILED — the non-singleton arrival count is zero at every completed unit, which is what a stuck-at-singleton argmax would produce"
    );
    let _ = writeln!(
        out,
        "(SS-R7) THE RULE BAR — HELD for all four frozen rules of freezes 46/49 at every one of {unit_set}: policy_value_by_rule <= Q^H(b). Contentful: a lawful rule policy cannot beat the lawful optimum, so a failure would mean the rule was evaluated against the wrong field, belief or convention."
    );
    let _ = writeln!(out, "{r8_line}");
    let _ = writeln!(
        out,
        "(SS-R9) SCOPE — every figure in this file names EVERY dimension it ranges over in the same sentence: its unit set, its state set and its seed set. A scope derived from an adjective is not a scope (FF-A18 as generalised at FC-A10(iv), FC-A14(ii)). The unit set of every aggregate below is: {unit_set}, over seeds {lo}..={hi}; the state set is the {tot_states} depth-one frontier states those units produced."
    );
    let _ = writeln!(out);

    // ---- the measurements -------------------------------------------------
    let _ = writeln!(out, "== AGGREGATES (SS-A7(b)-(g); every one scoped) ==");
    emit_aggregation(&mut out, &ustats, &unit_set, lo, hi);
    let _ = writeln!(
        out,
        "TAX SPARSITY OFF-CARRIER (SS-A7(e)), over the same unit, state and seed sets: {tot_support}/{tot_states} depth-one frontier states have delta_I > 0. This is the first out-of-carrier reading of the number whose margin-selected value was 4.49% at five units; it is filed either way and IS NEVER QUOTED FOR TRICK 1 OR FOR THE OPENING (P-A21)."
    );
    let _ = writeln!(
        out,
        "DECISION-DEADNESS AT THE FRONTIER (SS-A8), over the same unit, state and seed sets: {tot_forced}/{tot_states} frontier states are FORCED (|A(I)| = 1). Typing (J-A1): forced is its own column — no decision — and is never presented as a deadness count."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "TAX SUPPORT BY VERDICT CELL — the same cells as the tie tables above, carrying the delta_I > 0 counts. {SEPARATION_SENSE}"
    );
    for (v, (n, ns, ar, sup, st)) in &by_verdict {
        let _ = writeln!(
            out,
            "  {v}: {n} completed unit(s) of seeds {lo}..={hi}; frontier states with delta_I > 0 {sup}/{st}; non-singleton clairvoyant argmax arrivals {ns}/{ar} (arrival-pooled; the PRIMARY per-unit-mean form of this cell is in the tie tables above, SS-A10(ii))."
        );
    }
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "COLLISIONS (SS-A3, SS-A7(g)), over the {} coordinates of seeds {lo}..={hi}: {} DISTINCT canonical coordinate keys (declaration, focal hand, pool). Filed as-is and NEVER deduplicated — collision frequency is data about the generator and about the dumb policy's funnelling, and deduplication would silently make the unit count depend on the results.",
        seeds.len(),
        coord_keys.len()
    );
    for (k, v) in coord_keys.iter().filter(|(_, v)| v.len() > 1) {
        let _ = writeln!(out, "  COLLIDING seeds {v:?} share the coordinate key: {k}");
    }
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "DECLARED STOPS (SS-A5(iii), SS-A7(f)), over the {} units of seeds {lo}..={hi}: {}. A stop is a FILED OUTCOME under F7, printed as a stop and never as a finding (R-A18); the count of stops with their seeds is itself a recorded measurement.",
        4 * (hi - lo + 1),
        if stop_units.is_empty() {
            "none".to_owned()
        } else {
            format!("{} — {stop_units:?}", stop_units.len())
        }
    );
    let _ = writeln!(
        out,
        "NOT PRICED ON THE PRIMAL-WITNESS ROUTE (SS-A4(c)), over {unit_set}: {}. An over-threshold count bars the primal-witness pipeline ONLY and that route was NEVER ATTEMPTED; it does not bar (M1)-(M6), and every such unit above still carries its solve, its tie census and its taxes.",
        if not_priced.is_empty() {
            "none".to_owned()
        } else {
            format!("{} — {not_priced:?}", not_priced.len())
        }
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "ACCOUNTING (SS-A4(d)): every completed unit's row carries |I_1|, the |A(I)| distribution, the tax support, the zero-tax split (forced / common) and its companion row count, so THE COMPANION'S OMISSION IS AUDITABLE FROM THIS COMMITTED FILE ALONE. Companion rows over {unit_set}: {all_rows}, one per depth-one frontier state, and {tot_states} = {all_rows} is asserted in-run."
    );
    assert_eq!(
        u128::from(all_rows),
        tot_states,
        "SS-A4(d) accounting: the companion row total is not the frontier state total"
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "OMITTED BY DESIGN (SS-A8): NO per-(state, world) rows are retained. At four hundred units that is hundreds of millions of rows for a fee pass we have not designed. The aggregates above are chosen so a LATER fee pass can SELECT ITS COORDINATES without re-solving — which is the honest scope: the survey is a SELECTION INSTRUMENT FOR FEE WORK, NOT A FEE MEASUREMENT."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== PROVENANCE (never a receipt, never a dividend; N4-A13, DS-A31/DS-A36) =="
    );
    let _ = writeln!(out, "{resume_line}");
    let _ = writeln!(
        out,
        "BINARY PROVENANCE (run-owner requirement, 2026-08-15): the coordinates in this file were SOLVED by the binary built from commit {}, and this file was ASSEMBLED by the binary built from commit {}. Where those differ, every unit value above was LOADED FROM A CHECKPOINT BLOCK written by the solving binary, with each loaded seed's generator output re-derived from the seed alone and asserted byte-identical (DS-A36); the assembling binary changed PRESENTATION ONLY — no coordinate was re-solved and no measured value was recomputed by it, save the (SS-R8) declared sample, which is re-run in full by design. Both hashes are declared at run time (SS_SOLVE_COMMIT / SS_ASSEMBLY_COMMIT) and are provenance, never a receipt.",
        std::env::var("SS_SOLVE_COMMIT").unwrap_or_else(|_| "unset (not declared)".to_owned()),
        std::env::var("SS_ASSEMBLY_COMMIT").unwrap_or_else(|_| "unset (not declared)".to_owned())
    );
    let rss_kb = rss_kb();
    let _ = writeln!(
        out,
        "PER-STAGE WALL TIMES (provenance only, never a receipt, never a dividend; CONTENDED at W > 1, so no figure here forms a ratio and none is quoted):"
    );
    for r in &stage_rows {
        let _ = writeln!(out, "{r}");
    }
    let _ = writeln!(
        out,
        "run-owner declarations in force: T_pass = {} s per unit (SS_T_PASS_SECS); M_budget = {} GiB per block (SS_M_BUDGET_GIB); W = {workers} worker threads. Wall-clock this invocation: {run_ms} ms (excludes checkpointed prior invocations). Process resident size at write time: {rss_kb} KiB against M_budget = {} GiB. A resumed run inherits counts and receipts freely and inherits NO quotable timing at all.",
        t_pass_secs(),
        m_budget_gib(),
        m_budget_gib()
    );
    let _ = writeln!(
        out,
        "checkpoint blocks (SS-A4(e), DS-A36): blocks of {BLOCK} seeds, durable ONLY WHEN COMPLETE, written write-then-rename so a kill never leaves a torn record; the morning has whatever completed and nothing partial. Assembly is in CANONICAL UNIT ORDER, never completion order. Checkpoint format digest: {SS_CKPT_DIGEST} — a record whose digest differs from the running freeze set is CORRUPT, not stale, and the cache is discarded ENTIRE (DS-A30(ii))."
    );
    let _ = writeln!(
        out,
        "FAT DATA LOCATION (run-owner amendment, 2026-08-15): every fat artifact — the companion and the checkpoint blocks — lives OUT OF TREE under {FAT_ROOT}, so a worktree can be deleted without losing run data. The pinned SHA-256, byte and line counts in the companion line above are the ONLY link between this committed file and that out-of-tree data; the checkpoint blocks under {FAT_ROOT}/ckpt carry the companion rows themselves, so no row is stored twice."
    );

    // ---- write the companion, then pin it into the header -----------------
    std::fs::create_dir_all(fat_dir("")).expect("fat data dir");
    let suffix = if partial {
        format!("_smoke_seeds{lo}-{hi}")
    } else {
        String::new()
    };
    let companion_path = fat_dir("").join(format!("seed_survey_companion_2026-08-15{suffix}.txt"));
    let mut ctext = String::new();
    let _ = writeln!(
        ctext,
        "walt SS seed survey — COMPANION (freeze 54(d), the freeze 50 v1.1(c) content cut). Gitignored by construction: this file lives OUT OF TREE under {FAT_ROOT} and is a deterministic function of committed inputs, regenerated by the probe. It carries NO row that carries a claim. The committed summary at walt-factory/results/ pins its SHA-256, byte and line counts and accounts for every omitted row."
    );
    let _ = writeln!(ctext, "freeze-set digest: {SS_DIGEST}");
    let _ = writeln!(
        ctext,
        "scope (SS-R9): one row per depth-one frontier state, over {unit_set}, seeds {lo}..={hi}."
    );
    for so in seeds.iter().map(|s| &all[s]) {
        let _ = writeln!(ctext, "coordinate {}", so.identity);
    }
    // Head and body are written and hashed as two chunks through the STREAMING
    // digest path, which the FIPS 180-4 self-check at the top of main covers
    // explicitly — no third copy of the rows is ever materialised.
    let mut sha = Sha256::new();
    sha.update(ctext.as_bytes());
    sha.update(companion_body.as_bytes());
    let digest = sha.finish();
    let bytes = ctext.len() + companion_body.len();
    let lines = ctext.lines().count() + companion_body.lines().count();
    {
        use std::io::Write as _;
        let f = std::fs::File::create(&companion_path).expect("create companion");
        let mut w = std::io::BufWriter::new(f);
        w.write_all(ctext.as_bytes()).expect("write companion head");
        w.write_all(companion_body.as_bytes())
            .expect("write companion body");
        w.flush().expect("flush companion");
    }
    let companion_line = format!(
        "companion (freeze 54(d), gitignored BY LOCATION — out of tree): {} — SHA-256 {digest}, {bytes} bytes, {lines} lines. It carries the per-seed deal, the twelve played tiles and every per-frontier row. Regenerated by this probe; no row in it carries a claim, and the accounting integers on every unit row above make its omission auditable.",
        companion_path.display()
    );
    assert!(
        out.contains(COMPANION_LINE_PLACEHOLDER),
        "the header carries exactly one companion marker"
    );
    let out = out.replace(COMPANION_LINE_PLACEHOLDER, &companion_line);

    let results = out_dir("results").join(format!("seed_survey_2026-08-15{suffix}.txt"));
    std::fs::write(&results, &out).expect("write results");
    println!("{}", &out[..out.len().min(4000)]);
    println!("results: {}", results.display());
    println!("companion: {} ({digest})", companion_path.display());
}
