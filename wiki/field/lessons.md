# Lessons — transferable craft from first contact

[Field Home](Home.md) · owns: the methods that made the encounters fair, cheap,
and interpretable — worth keeping regardless of any particular number.
Field-measurement tier; cites [first-contact](first-contact.md).

## Protocol craft

- **Delete the auction to compare players that don't bid.** The "dropped-30"
  protocol (every hand forced to 30, declaration = best pip trump of the forced
  hand) makes contracts exogenous and deterministic from the deal. Neither
  player is asked to do something it lacks; what remains is pure play.
- **Mirror everything.** Because the contract is deal-deterministic, every deal
  can be played twice with teams swapped. Deal luck cancels *exactly*; the
  discordant pairs (same cards, opposite outcome) are the entire skill signal,
  and McNemar's test sizes it. 1,152 games gave 6,028 usable pairs; the paired
  make-rate CI was ±1.1pp where the unpaired game-level CI was several times
  wider.
- **Score marks per hand, binary.** Make-or-set per hand (making 29 = making 0)
  removes points-margin noise from the headline while raw points remain in the
  artifacts for secondary reads. Defense needs no separate tally: with exactly
  one bidding team per hand and mirrored deal sets, defense is the complement of
  the opponent's offense on the same deals.
- **Localize before diagnosing.** The mid-hand takeover rig — one shared
  deterministic heuristic prefix, freeze the position, play it out both ways —
  turned "why did rob lose?" from speculation into a bisection. The takeover
  trick is a knob; walking it earlier finds where parity breaks. The whole
  instrument was ~200 lines over existing pieces and runs 256 positions in
  seconds-to-minutes.

## The bridge as instrument

- **A dependency-free seam is worth more than an integration.** The
  `rob_bridge` line protocol (plain integers, full history per request, no build
  dependency in either direction) let rob be seated in a foreign harness in one
  session — and because requests are stateless, mid-hand pickup came for free,
  which is what made the localization eval possible the same day it was
  conceived.
- **Make every decision a conformance check.** The bridge replies with rob's
  independently derived trick leader and team points; the harness asserts them
  against its own engine every single decision. ~180k+ decisions, zero
  divergences, at zero marginal cost — continuous cross-validation of two
  independent rules implementations, riding along under the eval.

## Interpretation discipline

- **"Exact" is exact *given a model*.** An information-set best response is
  conditioned on an opponent/evaluation model; when it loses, the model lost,
  not the solve. The encounters graded rob's components separately: the solve
  matched the champion; the opening stand-in did not. State which part a number
  measures, always.
- **Small probes have signs, not sizes.** The 24-game probe said +0.46 for rob;
  1,152 games said −0.38 against. Neither the magnitude *nor the sign* of a
  small-n paired result should be trusted — that is what the mirrored-pairs
  machinery at scale is for.
- **Dead heat ≠ optimal.** Parity against a strong baseline bounds the gap
  *between* the players, not either player's distance from optimal play. Both
  could be leaving the same value on the table.
