# Fixed opening hand, varying hidden deal

Exploratory follow-up requested after the 100-deal campaign. This panel fixes
trick-one positions: ten newly generated opening hands, ten hidden completions
per hand, and three matched lineups per completion (300 games total).

World IDs are 520600–520699. Opening-hand seeds are 520600–520609. Each group
holds the opening player's seven tiles fixed, rotates its seat by group,
chooses pip trump from that hand alone using the existing heuristic, and fixes
the bid at 30. The remaining 21 dominoes are independently reshuffled among
the other three seats for each completion. Partner and opponent hands vary
together; this does not isolate a partner-only causal effect.

The private input at the opening is identical within a group and lineup:
own hand, empty public record, seat/bidder, trump, bid, and policy seed 420600.
The actual completion is known only to the referee. Completed opening choices
must agree across worlds. Deadline fallbacks are recorded separately, because
wall timing can change the executed policy even for identical information.
Later public trajectories may differ and induce different lawful choices.

The outcomes are declaring make/set at 30, plus the same two paired role
comparisons as the original campaign. Points, opening model scores, and
overbid amounts are diagnostics only. A sampled score against a modeled field
is not a calibrated probability against the executed field.

The experimental units are ten opening-hand clusters, not 100 independent
opening positions. The independent-deal sequential stopping rule is disabled.
Technical/fallback safeguards remain in effect. No claim about a fixed
midgame public history or a realistic auction is made by this trick-one panel.

The unchanged player runs in the ten-game shared pool with six native threads
per game, atomic per-move checkpoints, ordered result commits, and bounded
foreground slices. Independent replay verifies completed games. The report
also verifies fixed hands/contracts, distinct hidden completions and partner
hands, and agreement of completed opening calculations.
