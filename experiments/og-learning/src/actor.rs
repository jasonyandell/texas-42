//! The rational relational softmax actor (parent §2), in the exact
//! multiplicative-weights parameterization the intake companion proposed:
//! weight(a) = prod_j r_j^{x_j(I,a)} over legal a, with r_j positive
//! rationals. theta_j = ln r_j is never materialized; every probability is
//! exact-rational. One shared coefficient vector serves both partner seats;
//! each invocation sees only its own information.

use num_rational::BigRational;
use num_traits::One;
use walt::rules::{Domino, DominoSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RationalActor {
    /// One positive rational per dictionary clause, in dictionary order.
    pub weights: Vec<BigRational>,
    pub dictionary_version: String,
}

impl RationalActor {
    pub fn uniform(n_clauses: usize, dictionary_version: &str) -> Self {
        RationalActor {
            weights: vec![BigRational::one(); n_clauses],
            dictionary_version: dictionary_version.to_string(),
        }
    }

    /// Canonical identity text: version + every coefficient, reduced.
    pub fn canonical_text(&self) -> String {
        let coeffs: Vec<String> = self
            .weights
            .iter()
            .map(|r| format!("{}/{}", r.numer(), r.denom()))
            .collect();
        format!(
            "og-rational-actor-v1|{}|[{}]",
            self.dictionary_version,
            coeffs.join(",")
        )
    }

    /// 64-bit content digest of the canonical text (record identity only).
    pub fn digest(&self) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for b in self.canonical_text().bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x0000_0100_0000_01B3);
        }
        h
    }

    /// Unnormalized action weights over the legal set, in `legal.iter()`
    /// order. Every weight is positive, so the softmax is well defined.
    pub fn action_weights(
        &self,
        legal: DominoSet,
        candidate_sets: &[DominoSet],
    ) -> (Vec<Domino>, Vec<BigRational>) {
        assert_eq!(candidate_sets.len(), self.weights.len());
        let actions: Vec<Domino> = legal.iter().collect();
        let one = BigRational::one();
        let weights = actions
            .iter()
            .map(|a| {
                let mut w = BigRational::one();
                for (r, set) in self.weights.iter().zip(candidate_sets) {
                    if r != &one && set.contains(*a) {
                        w *= r;
                    }
                }
                w
            })
            .collect();
        (actions, weights)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;

    fn rat(n: i64, d: i64) -> BigRational {
        BigRational::new(BigInt::from(n), BigInt::from(d))
    }

    #[test]
    fn uniform_actor_weights_every_legal_action_equally() {
        // Law: all-ones coefficients give the uniform law over legal.
        let actor = RationalActor::uniform(3, "test-v0");
        let legal: DominoSet = DominoSet::FULL.iter().take(4).collect();
        let sets = vec![DominoSet::EMPTY, legal, DominoSet::FULL];
        let (actions, weights) = actor.action_weights(legal, &sets);
        assert_eq!(actions.len(), 4);
        assert!(weights.iter().all(|w| w == &BigRational::one()));
    }

    #[test]
    fn a_weighted_clause_multiplies_exactly_its_members() {
        // PINNED strictness witness: only members of the clause's set move.
        let mut actor = RationalActor::uniform(2, "test-v0");
        actor.weights[0] = rat(3, 2);
        let legal: DominoSet = DominoSet::FULL.iter().take(3).collect();
        let member: DominoSet = legal.iter().take(1).collect();
        let sets = vec![member, DominoSet::EMPTY];
        let (actions, weights) = actor.action_weights(legal, &sets);
        assert_eq!(weights[0], rat(3, 2));
        assert_eq!(weights[1], BigRational::one());
        assert_eq!(weights[2], BigRational::one());
        assert!(member.contains(actions[0]));
    }

    #[test]
    fn digest_tracks_content() {
        let a = RationalActor::uniform(14, "v");
        let mut b = a.clone();
        assert_eq!(a.digest(), b.digest());
        b.weights[3] = rat(17, 16);
        assert_ne!(a.digest(), b.digest());
    }
}
