# Scheme dynamics

`walt::scheme` exposes finite, extensional transforms for one fully typed
public play. `ObservedPlay` names the actor and physical domino; `PlayClass`
also says whether it is a lead, a follow in a named context, or a slough from a
named context. The transform rejects an actor, context, classification, holder,
or follow-legality mismatch.

`step_frame` rebuilds the inherited kernel and public residue. A hidden play
forces the holder, removes the tile, decrements that chair's capacity, and, for
a slough, records the observed void before the kernel's exact matching fiber is
recomputed. A viewer play removes the tile from the known hand. The fourth play
closes the trick with the rules engine and installs its winner as next leader.
`step_world` is the corresponding partial deterministic map on a concrete
world, and `play_is_legal` recognizes its domain.

`step_belief` performs the finite update in the required order:

1. discard worlds outside the typed legal domain;
2. multiply each remaining weight by the caller's exact likelihood in `[0,1]`;
3. push each physical world through `step_world`;
4. merge equal successors and report the observation's prior probability.

As with every `Belief`, stored weights may be an unnormalized exact measure;
probability accessors divide by its total mass.

`unit_likelihood` is suitable for a forced action, where no discretionary
likelihood ratio remains, and for a viewer intervention randomized
independently of hidden deal. An ordinary hidden choice needs a declared policy
likelihood.

The current `Belief` is only a marginal over physical remaining hands. If two
states with the same hands carry different policy memory, private observations,
or other field state, they must remain distinct in a caller-owned augmented
state until after likelihood and latent transition. Passing a likelihood over
the already-merged physical marginal cannot recover that information.

Output bindings are rigid identities. `transport_answers` carries domino,
chair, and context values unchanged; it does not assert that the same values
still satisfy a predicate. `compare_answers` compares transported prior answers
with a fresh successor evaluation and returns the persistent, extinct, and born
sets separately. `anchor_back` computes a finite predecessor preimage against
explicit later `(world, answer)` pairs. It is analyst hindsight filtering and
does not add later information to the player's earlier policy.

All transforms return a complete value or an error. They consume the explicit
`Budget`; a refusal never returns a partial frame, belief, answer comparison, or
anchor. World caps are also refusals. These APIs execute the finite semantics of
§6; they do not compile a Scheme `step`, compress support, or claim a compact
dynamic representation.
