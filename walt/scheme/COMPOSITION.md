# Donor composition

The implemented composition experiment works on one immutable root, one fixed
deterministic field, and an ordered finite stream of physical worlds. Its
objective is the number of sampled worlds in which the declaring team makes its
contract.

For each singleton donor world, `donor` traverses every legal focal action at
every reachable focal public history. It records every action having at least
one successful continuation, rather than selecting one arbitrary optimizer.
The `ActionPool` unions these successful alternatives by public action history
across donors. A donor traversal may stop when the score has already decided
the Boolean outcome because no later action can change that outcome.

`compose` then performs a shared exact search on the accumulated sample. At a
focal history it restricts search to the legal actions in the donor union. If
the union is absent, empty, or has no action legal in the current state, it
searches the full legal set. Hidden seats continue to use the same declared
deterministic field. The result is one information-consistent table policy,
repriced by independent replay.

With a complete successful-action pool from every world in the accumulated
sample, this restriction preserves the full solver's optimal training make
count for the fixed field and Boolean objective. Inductively, whenever the full
joint optimum has positive value at a focal node, its chosen action succeeds in
at least one world in that node and a complete singleton traversal places that
action in the union. If every action has zero value, the successful union is
empty and composition falls back to all legal actions. Deterministic hidden
branches preserve the same claim between focal nodes. This is why the
experiment expects composition and fresh full search to agree on training
quality; it measures reuse cost, not a strength gain.

The guarantee does not apply to a partial donor pool. A nonempty union built
from only part of the stream can exclude an action needed by the full joint
optimum, so restricted composition may then lose training makes. Budget refusal
does not publish a partial donor update. Completed unrestricted search memo
entries may survive refusal as exact resumable work, while restricted
composition memo entries are discarded before another grammar or unrestricted
solve can use them.

The emitted policies are exact tables over public histories seen through this
root's search tree. Held-out replay tests whether those tables happen to work on
other sampled worlds from the same root. No learned structural generalization
is claimed across starting hands or whole deals. The broader Scheme policy
language can express relational guards and rigid roles, but this composer does
not infer those rules from its tables. Timing and memory improvements are
experimental measurements; parity of the finite training optimum alone does
not establish either one.
