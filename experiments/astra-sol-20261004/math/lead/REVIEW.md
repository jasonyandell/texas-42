# Lead's independent challenge of Sol's result

I read `../sol/ArgmaxAmplification.lean` and its exact Fraction implementation,
checked the payoff orientation and information assumptions, and independently
ran the Lean file. The independent receipt is
`../../results/lead-recheck-sol/` (pass, 0.148 s).

The potentially misleading interpretation would be that optimal values are
discontinuous under uniformly small errors at one unchanged belief. The example
does not establish that. Instead it changes the chosen constant policy near a
tie, then evaluates that policy from a different seat's information set, where
the hidden type is known. This is a lawful information difference: neither
field policy consults the type, and the host knows its own private information.
For the arbitrary-small-error family, n=m>0, so the conditioned-on type has
positive prior mass. All reported fractions have the same positive denominator.

The Lean conclusion is exactly an integer-numerator and denominator statement;
it does not silently import an external numerical calculation. A small
lower-mind regret therefore cannot, by itself, justify substituting the field
inside an outer response. My disagreement result gives sufficient additional
conditions, but constructing those uniform bounds cheaply remains open.

I also challenged the demand-tree claim: an abstract binary tree with unique
keys is not an information-theoretic lower bound for fixed finite Texas 42.
Sol's final statement correctly preserves this restriction. A fixed finite
state universe can allow an O(k*M) formulation with an enormous M. A practical
linear-cost claim must control M, local work and dependency-edge counts;
memoization alone does not do so. The RPS cycle is a known counterexample to
monotone strength, not a new result and not a direct Texas 42 witness.

Both proof files use only standard Lean axioms; neither contains admitted
results. This review does not turn either finite abstraction into a verified
implementation theorem for the complete Walt program.
