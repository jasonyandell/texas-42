//! Stage S10 (stretch) verification harness: the x:001 reachable-support
//! floor (BRIEF_SLICE_02 §9, table S10). Exchange tier throughout (x:001,
//! CONFIRMED 2026-07-27); re-implemented from the inbox/001 prose proof and
//! JSON tables, never from its Python.

use rob_core::support::floor::{
    admissible_groups, mark_fragment_unions, module_winner_cases, star_declarations,
    ClosureBuilder, FragmentContext, FragmentKind, UpwardClosure,
};
use rob_core::{algebra_for, all_ids, LedSuit, PIPS};

use crate::receipt::{fmt_commas, Receipt};

const FULL: u32 = (1 << 28) - 1;

fn called_mask(declaration: rob_core::Declaration) -> u32 {
    let algebra = algebra_for(declaration);
    all_ids()
        .filter(|&d| algebra.called().contains(d))
        .map(|d| 1u32 << d.index())
        .sum()
}

/// One family row: a witness language cut at one size under one |X| range.
struct Row {
    kind: FragmentKind,
    modules: usize,
    size: u32,
    x_max: u32,
    categories: u64,
    expected_per_category: u64,
}

/// `x-r_flo_modules` (x:001 steps 3 and 5): the 3,808 module/winner cases.
pub fn modules_check() -> u64 {
    module_winner_cases()
}

/// The no-void family (x:001 step 11): the four REACH-12 rows, the |T| = 11
/// closed form, and the six module-coverage rows with their labeling
/// multiplicities. Returns the family total.
pub fn no_void_family() -> u64 {
    let binomial = |n: u64, k: u64| -> u64 {
        let mut r = 1u64;
        for i in 0..k {
            r = r * (n - i) / (i + 1);
        }
        r
    };
    // REACH-12 rows (corpus-anchored in S4) + the |T| = 11 row.
    let mut total = binomial(28, 7)
        + 3 * binomial(28, 8)
        + 3 * binomial(28, 9)
        + binomial(28, 10)
        + 3 * binomial(28, 11);
    assert_eq!(total, 44_352_165 + 64_422_540);

    // Module-coverage rows: (pattern, [(size, labelings, expected)]).
    type CoverageRow<'a> = (&'a [usize], &'a [(u32, u64, u64)]);
    let coverage: [CoverageRow<'_>; 6] = [
        (&[4, 2], &[(12, 2, 30_402_400)]),
        (&[4, 3], &[(12, 1, 30_294_577)]),
        (&[4, 4], &[(13, 1, 34_115_923), (14, 3, 39_546_166)]),
        (&[4, 4, 2], &[(15, 2, 37_400_509)]),
        (&[4, 4, 3], &[(15, 1, 37_241_110), (16, 1, 30_419_732)]),
        (&[4, 4, 4], &[(17, 3, 21_408_593)]),
    ];
    for (pattern, cuts) in coverage {
        let unions = rob_core::support::floor::witness_unions(pattern);
        let closure = UpwardClosure::new(&unions);
        for &(size, labelings, expected) in cuts {
            let count = closure.count(size, 0, &mut |_| true);
            assert_eq!(count, expected, "no-void coverage at size {size}");
            total += labelings * count;
        }
    }
    assert_eq!(total, 559_316_142);
    total
}

/// The called-suit one-void family (x:001 steps 6, 8–12): per-category
/// upward-closure counts × category multiplicities × the eight star
/// declarations (verified per declaration via the K8 symmetry — the count
/// is identical for every star, checked on two).
pub fn called_family() -> u64 {
    let rows: [Row; 17] = [
        // T=12: one module.
        row(FragmentKind::Pair, 1, 12, 7, 3, 23_966_810),
        row(FragmentKind::M1, 1, 12, 7, 1, 19_940_291),
        row(FragmentKind::M2, 1, 12, 7, 1, 23_966_810),
        // T=13.
        row(FragmentKind::M1, 1, 13, 7, 3, 29_359_456),
        row(FragmentKind::M2, 1, 13, 5, 3, 31_177_027),
        // T=14 (winning void trick then lead: the extra lead is free).
        row(FragmentKind::M1, 1, 14, 7, 6, 34_711_089),
        row(FragmentKind::M2, 1, 14, 4, 3, 29_851_710),
        // T=15: two modules.
        row(FragmentKind::Pair, 2, 15, 7, 3, 26_574_471),
        row(FragmentKind::M1, 2, 15, 7, 1, 20_479_830),
        row(FragmentKind::M2, 2, 15, 5, 1, 25_760_616),
        // T=16.
        row(FragmentKind::M1, 2, 16, 7, 3, 21_582_309),
        row(FragmentKind::M2, 2, 16, 4, 3, 22_693_062),
        // T=17.
        row(FragmentKind::M1, 2, 17, 7, 6, 17_212_461),
        row(FragmentKind::M2, 2, 17, 3, 3, 13_118_238),
        // T=18/19: three modules.
        row(FragmentKind::Pair, 3, 18, 7, 3, 8_905_344),
        row(FragmentKind::M1, 3, 19, 5, 3, 4_114_740),
        row(FragmentKind::M2, 3, 19, 3, 3, 4_233_495),
    ];
    // The K8 vertex transport (x:001 steps 2/12) makes every star's count
    // equal; verify explicitly on zeros trump and doubles trump, then
    // multiply by 8.
    let mut per_declaration_total = 0u64;
    for (probe, declaration) in [
        (true, star_declarations()[0]),
        (false, star_declarations()[7]),
    ] {
        let algebra = algebra_for(declaration);
        let s_mask = called_mask(declaration);
        let context = FragmentContext {
            follow: s_mask,
            fiber: s_mask,
            outsiders: !s_mask & FULL,
            forbidden: 0,
        };
        // Group rows by (kind, modules) to reuse closures.
        let mut sum = 0u64;
        for (kind, modules) in [
            (FragmentKind::Pair, 1),
            (FragmentKind::M1, 1),
            (FragmentKind::M2, 1),
            (FragmentKind::Pair, 2),
            (FragmentKind::M1, 2),
            (FragmentKind::M2, 2),
            (FragmentKind::Pair, 3),
            (FragmentKind::M1, 3),
            (FragmentKind::M2, 3),
        ] {
            let relevant: Vec<&Row> = rows
                .iter()
                .filter(|r| r.kind == kind && r.modules == modules)
                .collect();
            if relevant.is_empty() {
                continue;
            }
            let mut builder = ClosureBuilder::new();
            mark_fragment_unions(&algebra, modules, kind, &context, &mut builder);
            let closure = builder.finish();
            for r in relevant {
                let count = closure.count(r.size, 0, &mut |t| {
                    let x = (s_mask & !t).count_ones();
                    (2..=r.x_max).contains(&x)
                });
                assert_eq!(
                    count, r.expected_per_category,
                    "called {kind:?} m{modules} @{}",
                    r.size
                );
                sum += r.categories * count;
            }
        }
        if probe {
            per_declaration_total = sum;
        } else {
            assert_eq!(sum, per_declaration_total, "K8 transport symmetry");
        }
    }
    let total = 8 * per_declaration_total;
    assert_eq!(total, 8_387_350_664);
    total
}

/// The natural-suit one-void family (x:001 steps 7, 13): generated under
/// zeros trump for each natural context (fiber sizes 1..6), each count
/// occurring for exactly seven ordered (t, q) contexts.
pub fn natural_family() -> u64 {
    // Rows: (kind, modules, size, x_max, categories, expected counts by
    // fiber size 1..6).
    type NaturalRow = (FragmentKind, usize, u32, u32, u64, [u64; 6]);
    let rows: [NaturalRow; 9] = [
        (
            FragmentKind::Pair,
            1,
            12,
            6,
            3,
            [
                5_760_594, 9_210_738, 11_360_129, 12_660_550, 13_404_689, 13_812_747,
            ],
        ),
        (
            FragmentKind::M1,
            1,
            12,
            6,
            1,
            [
                5_205_424, 8_170_230, 9_851_930, 10_700_937, 11_015_340, 11_015_340,
            ],
        ),
        (
            FragmentKind::M2,
            1,
            12,
            6,
            1,
            [
                5_760_594, 9_210_738, 11_360_129, 12_660_550, 13_404_689, 13_812_747,
            ],
        ),
        (
            FragmentKind::M1,
            1,
            13,
            6,
            3,
            [
                7_491_473, 11_679_933, 13_902_336, 14_925_464, 15_273_873, 15_273_873,
            ],
        ),
        (
            FragmentKind::M2,
            1,
            13,
            5,
            3,
            [
                7_908_238, 12_476_855, 15_067_893, 16_446_391, 17_134_605, 17_458_845,
            ],
        ),
        (
            FragmentKind::M1,
            2,
            15,
            6,
            1,
            [
                5_711_043, 8_584_043, 10_057_926, 10_705_653, 10_917_124, 10_917_124,
            ],
        ),
        (
            FragmentKind::M2,
            2,
            15,
            5,
            1,
            [
                6_175_761, 9_384_409, 11_150_343, 12_046_854, 12_471_751, 12_662_139,
            ],
        ),
        (
            FragmentKind::M1,
            2,
            16,
            6,
            3,
            [
                5_238_902, 7_878_214, 9_083_898, 9_529_061, 9_652_578, 9_652_578,
            ],
        ),
        (
            FragmentKind::M2,
            2,
            16,
            4,
            3,
            [
                5_318_483, 8_010_800, 9_254_244, 9_723_064, 9_860_356, 9_869_049,
            ],
        ),
    ];
    let declaration = star_declarations()[0]; // zeros trump
    let algebra = algebra_for(declaration);
    let mut total = 0u64;
    #[allow(clippy::needless_range_loop)] // q_pip is the context pip label
    for q_pip in 1..=6usize {
        let q = LedSuit::Natural(PIPS[q_pip]);
        let fiber_size = q_pip; // under zeros trump, context q has fiber size q
        let follow: u32 = all_ids()
            .filter(|&d| algebra.follows(d, q))
            .map(|d| 1u32 << d.index())
            .sum();
        let fiber: u32 = algebra
            .lead_fiber(q)
            .iter()
            .map(|d| 1u32 << d.index())
            .sum();
        assert_eq!(fiber.count_ones() as usize, fiber_size);
        // The omitted edge e = q:0 must remain unseen.
        let e_mask: u32 =
            1 << rob_core::domino_id(rob_core::Domino::new(PIPS[q_pip], PIPS[0])).index();
        let outsiders = !follow & !e_mask & FULL;
        let context = FragmentContext {
            follow,
            fiber,
            outsiders,
            forbidden: e_mask,
        };
        for (kind, modules) in [
            (FragmentKind::Pair, 1),
            (FragmentKind::M1, 1),
            (FragmentKind::M2, 1),
            (FragmentKind::M1, 2),
            (FragmentKind::M2, 2),
        ] {
            let relevant: Vec<_> = rows
                .iter()
                .filter(|r| r.0 == kind && r.1 == modules)
                .collect();
            if relevant.is_empty() {
                continue;
            }
            let mut builder = ClosureBuilder::new();
            mark_fragment_unions(&algebra, modules, kind, &context, &mut builder);
            let closure = builder.finish();
            for &(_, _, size, x_max, categories, expected) in relevant {
                let count = closure.count(size, e_mask, &mut |t| {
                    let x = (follow & !t).count_ones();
                    (2..=x_max).contains(&x)
                });
                assert_eq!(
                    count,
                    expected[fiber_size - 1],
                    "natural {kind:?} m{modules} @{size} fiber {fiber_size}"
                );
                // Seven ordered (t, q) contexts share each fiber size.
                total += categories * 7 * count;
            }
        }
    }
    assert_eq!(total, 8_721_399_239);
    total
}

fn row(
    kind: FragmentKind,
    modules: usize,
    size: u32,
    x_max: u32,
    categories: u64,
    expected_per_category: u64,
) -> Row {
    Row {
        kind,
        modules,
        size,
        x_max,
        categories,
        expected_per_category,
    }
}

/// One-context marginal-holder verification (x:001 step 5): build every
/// abstract one-context cell system over the labeled capacity triples and
/// void-seat memberships the family tables use, across the step-5 valid
/// exclusion sizes, and verify the described marginal holder sets through
/// rob's own reduction machinery. Returns the number of profiles verified.
///
/// rob-frozen note (final-report item): rob enumerates the full principled
/// space — 369 profiles, a strict superset of the 216 the x:001 program
/// tabled; every profile satisfies the step-5 description.
pub fn one_context_profiles() -> u64 {
    use rob_core::AbstractCells;
    let mut profiles = 0u64;
    // Labeled capacity triples used by the tables.
    let labeled: Vec<[usize; 3]> = {
        let orbits: [[usize; 3]; 7] = [
            [5, 5, 6],
            [5, 5, 5],
            [4, 5, 5],
            [4, 4, 5],
            [4, 4, 4],
            [3, 3, 4],
            [3, 3, 3],
        ];
        let mut out = Vec::new();
        for orbit in orbits {
            let mut perms: Vec<[usize; 3]> = Vec::new();
            let indices = [
                [0, 1, 2],
                [0, 2, 1],
                [1, 0, 2],
                [1, 2, 0],
                [2, 0, 1],
                [2, 1, 0],
            ];
            for p in indices {
                let k = [orbit[p[0]], orbit[p[1]], orbit[p[2]]];
                if !perms.contains(&k) {
                    perms.push(k);
                }
            }
            out.extend(perms);
        }
        out
    };
    // Void-seat memberships: three singletons and three pairs.
    let memberships: [&[usize]; 6] = [&[0], &[1], &[2], &[0, 1], &[0, 2], &[1, 2]];
    for k in &labeled {
        let n: usize = k.iter().sum(); // |U| = Σ capacities
        for membership in memberships {
            #[allow(clippy::needless_range_loop)] // x_size is a size, not an index
            for x_size in 2..=6usize {
                // Step-5 tabled ranges: singleton M needs |N| ≥ k_r + 1;
                // pair M forces X to the third seat with |X| ≤ k_u.
                let valid = if membership.len() == 1 {
                    let r = membership[0];
                    n - x_size > k[r]
                } else {
                    let u = (0..3)
                        .find(|s| !membership.contains(s))
                        .expect("third seat");
                    x_size <= k[u]
                };
                if !valid || x_size >= n {
                    continue;
                }
                // Build the abstract one-context system: X = first x tiles,
                // N = rest; P_s = N for s ∈ M, U otherwise.
                let possible: [Vec<bool>; 3] = core::array::from_fn(|s| {
                    (0..n)
                        .map(|t| !(membership.contains(&s) && t < x_size))
                        .collect()
                });
                let cells = AbstractCells::new(n, possible, *k).expect("one-context system");
                if !cells.is_feasible() {
                    continue;
                }
                let marginal = rob_core::marginal_by_projection(&cells);
                for t in 0..n {
                    for s in 0..3 {
                        let expected = if t < x_size {
                            // X-tiles: excluded exactly at the void seats;
                            // possible at every non-void seat (in the pair
                            // case that is the single forced third seat).
                            !membership.contains(&s)
                        } else {
                            // N-tiles: possible everywhere, except (pair
                            // case) the equality k_u = |X| pins the third
                            // seat entirely to X.
                            if membership.len() == 2 && !membership.contains(&s) {
                                k[s] > x_size
                            } else {
                                true
                            }
                        };
                        assert_eq!(
                            marginal[s][t], expected,
                            "step-5 marginal description at k={k:?} M={membership:?} x={x_size} tile {t} seat {s}"
                        );
                    }
                }
                profiles += 1;
            }
        }
    }
    profiles
}

/// Build the canonical S10 receipt (§9.1 tier labeling).
pub fn receipt() -> String {
    let mut r = Receipt::new("S10");
    r.line("# exchange", "001 (CONFIRMED 2026-07-27)");
    let modules = modules_check();
    let profiles = one_context_profiles();
    r.line(
        "x-r_flo_modules",
        &format!(
            "{} declaration/group/winner cases; {} one-context profiles verified (superset of x:001's 216 tabled)",
            fmt_commas(modules as u128),
            profiles
        ),
    );
    let no_void = no_void_family();
    let called = called_family();
    let natural = natural_family();
    r.line(
        "x-r_flo_families",
        &format!(
            "{} no-void; {} called-suit void; {} natural-suit void",
            fmt_commas(no_void as u128),
            fmt_commas(called as u128),
            fmt_commas(natural as u128)
        ),
    );
    let total = no_void + called + natural;
    assert_eq!(total, 17_668_066_045);
    assert!(total > 1u64 << 34, "floor exceeds 2^34");
    r.line(
        "x-r_flo_total",
        &format!(
            "{} > 2^34; floor 35 bits; interval [35,46]",
            fmt_commas(total as u128)
        ),
    );
    r.finish()
}

/// Sanity anchor for the group census (docs).
pub fn group_census() -> (usize, usize, usize) {
    let algebra = algebra_for(star_declarations()[0]);
    (
        admissible_groups(&algebra, 2).len(),
        admissible_groups(&algebra, 3).len(),
        admissible_groups(&algebra, 4).len(),
    )
}
