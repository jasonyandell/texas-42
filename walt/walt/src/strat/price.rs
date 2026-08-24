//! Information prices (v0.4 §10.5) and the on-ray zero-information reading of
//! §10.6: exact PWL differences of the treatment values over lambda.
//!
//! Every price is nonnegative by the information-polytope chain
//! `P^H ⊆ P^C ⊆ P^F` (§10.4); `information_prices` asserts that, and the
//! exact decomposition `G_total = G_cont + G_root` (§10.5), on every result
//! it returns, so a violation is a loud arithmetic bug, never a wrong number.

use crate::geom::Envelope;
use crate::kernel::Kernel;
use crate::rules::{Domino, Team};

use crate::strat::direction::Direction;
use crate::strat::hidden::hidden_root_values;
use crate::strat::revealed::{revealed_summary, RevealedStop, RevealedSummary};

/// `V` from the root values: the upper envelope over actions.
pub fn upper_value(roots: &[(Domino, Envelope)]) -> Envelope {
    Envelope::max_of(roots.iter().map(|(_, e)| e.clone()))
}

/// §10.6 read along the ray: revelation is worthless at every lambda iff the
/// price envelope vanishes identically. (The exposed-witness form of the same
/// criterion lives finite-first on feature sets: `crate::geom::zero_information_at`.)
pub fn zero_information(price: &Envelope) -> bool {
    *price == Envelope::zero()
}

/// The three treatment values and every §10.5 price for one kernel, focal
/// team, and valuation direction.
pub struct InfoPrices {
    /// `Q^H(a)` per root, ascending domino order.
    pub q_h: Vec<(Domino, Envelope)>,
    /// `Q^C(a)` per root, same order.
    pub q_c: Vec<(Domino, Envelope)>,
    pub v_h: Envelope,
    pub v_c: Envelope,
    pub v_f: Envelope,
    /// `G_a^cont = Q_a^C - Q_a^H` per root, same order.
    pub g_cont_by_root: Vec<(Domino, Envelope)>,
    /// `G^cont = V^C - V^H`.
    pub g_cont: Envelope,
    /// `G^root = V^F - V^C`.
    pub g_root: Envelope,
    /// `G^total = V^F - V^H`.
    pub g_total: Envelope,
    /// Per-action residual budgets of the H walks (freeze 44; for
    /// non-binding assertions).
    pub h_residuals: Vec<u64>,
    /// Per-action walk-step subtotals of the revealed pass (SEP-A19(b)'s
    /// typing class).
    pub revealed_action_steps: Vec<(Domino, u64)>,
}

/// Runs H, C, and F and forms every §10.5 price. Budgeted (freeze 44):
/// `per_action_budget` for each H walk, `revealed_budget` whole-call for the
/// revealed pass; `None` on any exhaustion, the assertions firing only on a
/// complete result.
pub fn information_prices(
    kernel: &Kernel,
    focal: Team,
    dir: &Direction,
    per_action_budget: u64,
    revealed_budget: &mut u64,
    revealed_stop: &mut Option<RevealedStop>,
) -> Option<InfoPrices> {
    let (q_h, h_residuals) = hidden_root_values(kernel, focal, dir, per_action_budget)?;
    let RevealedSummary {
        q_c,
        v_f,
        action_steps: revealed_action_steps,
    } = revealed_summary(kernel, focal, dir, revealed_budget, revealed_stop)?;
    let g_cont_by_root: Vec<(Domino, Envelope)> = q_h
        .iter()
        .zip(&q_c)
        .map(|((a, h), (b, c))| {
            assert_eq!(a, b, "H and C share the root action order");
            let g = c.sub(h);
            assert!(g.is_nonnegative(), "P_a^H ⊆ P_a^C (§10.4) at root {a:?}");
            (*a, g)
        })
        .collect();
    let v_h = upper_value(&q_h);
    let v_c = upper_value(&q_c);
    let g_cont = v_c.sub(&v_h);
    let g_root = v_f.sub(&v_c);
    let g_total = v_f.sub(&v_h);
    assert!(g_cont.is_nonnegative(), "V^C >= V^H (§10.4)");
    assert!(g_root.is_nonnegative(), "V^F >= V^C (§10.4)");
    assert_eq!(
        g_total,
        g_cont.add(&g_root),
        "the §10.5 exact decomposition"
    );
    Some(InfoPrices {
        q_h,
        q_c,
        v_h,
        v_c,
        v_f,
        g_cont_by_root,
        g_cont,
        g_root,
        g_total,
        h_residuals,
        revealed_action_steps,
    })
}
