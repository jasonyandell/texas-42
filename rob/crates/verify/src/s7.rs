//! Stage S7 verification harness: the necessary outer language and its
//! censuses (BRIEF_SLICE_02 §9, table S7).
//!
//! Exchange tier: `x-` lines draw on x:002 and x:005 (both CONFIRMED
//! 2026-07-27, `exchange/README.md`); everything else is corpus-anchored.

use num_bigint::BigUint;
use num_traits::Zero;

use rob_core::support::census::{relabel_signature, SEAT_PERMUTATIONS};
use rob_core::{
    all_ternary_signatures, lead_witness_b_convolution, lead_witness_b_inclusion_exclusion,
    profile_certificate_count, reachable_capacity_profiles, schedule_admissible, schedule_census_a,
    schedule_census_t, LedSuit, LedSuitSet,
};

use crate::receipt::{fmt_commas, Receipt};

/// `r_out_schedule` (REACH-06/06A): the three projected schedule censuses,
/// from the formulas AND cross-checked by brute-force enumeration of all
/// `8^7` context-membership assignments.
pub fn schedule_check() -> ([u64; 8], [u64; 8], [u64; 8]) {
    let a = schedule_census_a();
    let t1 = schedule_census_t(1);
    let t2 = schedule_census_t(2);

    // Brute force: each of the 7 contexts gets one of 8 membership
    // patterns (0 = unused, 1..=7 = the nonempty hidden-seat subsets).
    let mut brute_a = [0u64; 8];
    let mut brute_t1 = [0u64; 8];
    let mut brute_t2 = [0u64; 8];
    // Fixed follower sets of sizes 1 and 2 (counts are label-invariant).
    let f1: u8 = 0b001;
    let f2: u8 = 0b011;
    let mut assignment = [0u8; 7];
    loop {
        let used: Vec<u8> = assignment.iter().copied().filter(|&p| p != 0).collect();
        let q = used.len();
        for (j, slot) in brute_a.iter_mut().enumerate() {
            if q <= j {
                *slot += 1;
            }
        }
        for (f, brute) in [(f1, &mut brute_t1), (f2, &mut brute_t2)] {
            for (j, slot) in brute.iter_mut().enumerate() {
                let admissible =
                    q <= j || (q == j + 1 && used.iter().any(|&pattern| pattern & !f == 0));
                if admissible {
                    *slot += 1;
                }
            }
        }
        // Odometer increment over 8^7 assignments.
        let mut i = 0;
        loop {
            if i == 7 {
                break;
            }
            assignment[i] += 1;
            if assignment[i] < 8 {
                break;
            }
            assignment[i] = 0;
            i += 1;
        }
        if i == 7 {
            break;
        }
    }
    assert_eq!(a, brute_a, "A_j formula ≡ brute force");
    // T_{7,f} has no current-trick term; the brute-force count at j = 7 is
    // the full space for q ≤ 7, matching A_7.
    assert_eq!(t1, brute_t1, "T_j1 formula ≡ brute force");
    assert_eq!(t2, brute_t2, "T_j2 formula ≡ brute force");

    // The predicate agrees with the census on a spot grid: every capacity
    // profile with a used-context count at the boundary.
    let profiles = reachable_capacity_profiles();
    for k in &profiles {
        let mut masks = [
            LedSuitSet::empty(),
            LedSuitSet::empty(),
            LedSuitSet::empty(),
        ];
        // No voids is always admissible.
        assert!(schedule_admissible([k[0], k[1], k[2]], &masks));
        // j+2 distinct contexts is never admissible.
        let h = *k.iter().max().expect("three");
        let j = 7 - h;
        if j + 2 <= 7 {
            for (index, q) in LedSuit::all().into_iter().take(j + 2).enumerate() {
                masks[index % 3].insert(q);
            }
            assert!(!schedule_admissible([k[0], k[1], k[2]], &masks));
        }
    }
    (a, t1, t2)
}

/// `r_out_lead_witness` (REACH-07/11): the `B_{n,u}` table by two
/// independent routes; 176 = 22 × 8 agreeing entries.
pub fn lead_witness_check() -> u64 {
    let route_a = lead_witness_b_inclusion_exclusion();
    let route_b = lead_witness_b_convolution();
    let mut entries = 0u64;
    for n in 0..=21usize {
        for u in 0..=7usize {
            assert_eq!(route_a[n][u], route_b[n][u], "B[{n}][{u}] routes agree");
            entries += 1;
        }
    }
    entries
}

/// Summary of the per-profile certificate census (REACH-11/11A).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProfileCensus {
    /// Σ over the 50 profiles of `C(k)` (one declaration tag).
    pub per_declaration: BigUint,
    /// × 9 declaration tags.
    pub total: BigUint,
    /// The largest single-profile block.
    pub max_block: BigUint,
    /// Fixed-width ceilings: standalone / declaration-supplied /
    /// capacities-supplied / both.
    pub widths: (u64, u64, u64, u64),
}

/// `r_out_profiles`: evaluate the Math §7.13.6 formulas in exact
/// arithmetic over the 50 reachable capacity profiles.
pub fn profile_census() -> ProfileCensus {
    let b = lead_witness_b_inclusion_exclusion();
    let profiles = reachable_capacity_profiles();
    assert_eq!(profiles.len(), 50);
    let mut per_declaration = BigUint::zero();
    let mut max_block = BigUint::zero();
    for k in &profiles {
        let c = profile_certificate_count([k[0], k[1], k[2]], &b);
        max_block = max_block.max(c.clone());
        per_declaration += c;
    }
    let total = BigUint::from(9u32) * &per_declaration;
    let widths = (
        total.bits(),
        per_declaration.bits(),
        (BigUint::from(9u32) * &max_block).bits(),
        max_block.bits(),
    );
    ProfileCensus {
        per_declaration,
        total,
        max_block,
        widths,
    }
}

/// `r_out_five_checks` (REACH-04/06/07; CELL-09; x:002): the validator
/// returns necessary-only membership; sanity-exercise it on obviously good
/// and obviously bad profiles. (Its INV-14 no-conversion doctest lives on
/// the type.)
pub fn five_checks_sanity() {
    use rob_core::{Declaration, DominoSet, ReachabilityOuterNecessaryProfile};
    // The all-unrestricted initial profile passes everything.
    let viewer_hand: Vec<rob_core::DominoId> = rob_core::all_ids().take(7).collect();
    let pool = DominoSet::full().difference(&DominoSet::from_ids(viewer_hand));
    let profile = ReachabilityOuterNecessaryProfile {
        declaration: Declaration::NoTrump,
        capacities: [7, 7, 7],
        void_masks: [
            LedSuitSet::empty(),
            LedSuitSet::empty(),
            LedSuitSet::empty(),
        ],
        pool,
    };
    let report = profile.check_necessary();
    assert!(report.all(), "the unrestricted profile is in the language");
    // A capacity shape with range 2 fails the first check.
    let bad = ReachabilityOuterNecessaryProfile {
        capacities: [7, 7, 5],
        pool: {
            let mut p = profile.pool;
            let mut it = p.iter().take(2).collect::<Vec<_>>();
            for d in it.drain(..) {
                p.remove(d);
            }
            p
        },
        ..profile.clone()
    };
    assert!(!bad.check_necessary().capacity_shape);
    // A void triple using j+2 contexts fails the schedule check.
    let mut masks = [
        LedSuitSet::empty(),
        LedSuitSet::empty(),
        LedSuitSet::empty(),
    ];
    masks[0].insert(LedSuit::all()[0]);
    masks[1].insert(LedSuit::all()[1]);
    masks[2].insert(LedSuit::all()[2]);
    let over = ReachabilityOuterNecessaryProfile {
        capacities: [6, 6, 6],
        void_masks: masks,
        pool: {
            // 18-tile pool: drop three more tiles.
            let mut p = profile.pool;
            let drop: Vec<rob_core::DominoId> = p.iter().take(3).collect();
            for d in drop {
                p.remove(d);
            }
            p
        },
        ..profile.clone()
    };
    assert!(
        !over.check_necessary().schedule,
        "j+2 contexts inadmissible"
    );
}

/// `x-r_out_burnside` (CELL-21/22; x:005): S₃ fixed-signature counts and
/// the Burnside average reproducing the S4 orbit census. Returns
/// (identity, per-transposition, per-3-cycle, average).
pub fn burnside_check() -> (u64, u64, u64, u64) {
    let signatures = all_ternary_signatures();
    let mut fixed = [0u64; 6];
    for (index, &p) in SEAT_PERMUTATIONS.iter().enumerate() {
        fixed[index] = signatures
            .iter()
            .filter(|sig| relabel_signature(sig, p) == **sig)
            .count() as u64;
    }
    // SEAT_PERMUTATIONS order: identity, (12), (01), 3-cycle, 3-cycle, (02).
    let identity = fixed[0];
    let transpositions = [fixed[1], fixed[2], fixed[5]];
    let cycles = [fixed[3], fixed[4]];
    assert!(
        transpositions.iter().all(|&t| t == transpositions[0]),
        "the three transpositions fix equally many signatures"
    );
    assert!(cycles.iter().all(|&c| c == cycles[0]));
    let total: u64 = fixed.iter().sum();
    assert!(total.is_multiple_of(6));
    let average = total / 6;
    (identity, transpositions[0], cycles[0], average)
}

/// Build the canonical S7 receipt (§9.1 tier labeling).
pub fn receipt() -> String {
    let mut r = Receipt::new("S7");
    r.line("# exchange", "002, 005 (CONFIRMED 2026-07-27)");
    let (a, t1, t2) = schedule_check();
    let tuple = |t: &[u64; 8]| {
        let items: Vec<String> = t.iter().map(|v| v.to_string()).collect();
        format!("({})", items.join(", "))
    };
    r.line(
        "r_out_schedule",
        &format!(
            "A_j {}; T_j1 {}; T_j2 {}",
            tuple(&a),
            tuple(&t1),
            tuple(&t2)
        ),
    );
    r.line(
        "r_out_lead_witness",
        &format!("{} entries; two routes agree", lead_witness_check()),
    );
    let census = profile_census();
    r.line(
        "r_out_profiles",
        &format!(
            "{} per declaration; {} total; {} max block; ceilings {}/{}/{}/{} bits",
            fmt_commas(
                census
                    .per_declaration
                    .to_string()
                    .parse::<u128>()
                    .expect("fits")
            ),
            fmt_commas(census.total.to_string().parse::<u128>().expect("fits")),
            fmt_commas(census.max_block.to_string().parse::<u128>().expect("fits")),
            census.widths.0,
            census.widths.1,
            census.widths.2,
            census.widths.3
        ),
    );
    five_checks_sanity();
    r.line(
        "r_out_five_checks",
        "necessary-only membership; no conversion to certified types",
    );
    let (identity, transposition, cycle, average) = burnside_check();
    r.line(
        "x-r_out_burnside",
        &format!(
            "{} identity; {} per transposition; {} per 3-cycle; average {}",
            fmt_commas(identity as u128),
            fmt_commas(transposition as u128),
            cycle,
            fmt_commas(average as u128)
        ),
    );
    r.line(
        "r_out_interval",
        "26..46 bits (corpus-anchored REACH-12/13)",
    );
    r.finish()
}
