# A playable Mac table and a bounded override comparison

2026-09-13. Exploratory. The next milestone is human play with exact original
move evidence, flagged hands, and repeatable model-relative gym comparisons.

Before changing the live override, compare three simple rules on saved complete
count-offer keys: any positive observed gain (current), at least two net paired
saves, and at least three net paired saves. A complete census may accept any
positive gain in every rule. The baseline wins ties and every unresolved result.
These are heuristic thresholds, not confidence guarantees.

Use the 143 development roots to select minimum mean exact regret; ties prefer
the lower threshold. Report useful corrections and harmful changes separately.
Apply the selected rule to the 25 already-seen follow-up roots as regression
checks. Also compare all three rules over 32 reproducible fresh random subsets
of at most 64 worlds per root, using the saved all-world action vectors. These
subsets measure sensitivity on known positions; they are not new source deals
or an untouched strength test. Do not tune further on these results.

Keep the existing player and review presets intact. Add the selected optional
rule to the unified wrapper. Live play remains straight 42, bid 30, on the Mac.
The first bidder rotates with the deal; declaration is chosen from own/public
information. A real human partner is outside the frozen L1 continuation model;
flagged play is precisely an opportunity to find that model's limitations.

Plunge supplies the table. A localhost bridge calls the existing bounded native
player and atomically saves its original request, response, configuration and
implementation hashes. It accepts only own/public information for decisions.
A separate finished-hand flag endpoint validates the full replay and creates a
gym input. Full dealt hands belong only to that examiner record. No hidden hands
are supplied to any live or simulated chooser.

A flag records a selected play, optional alternative and note, original decision
receipt when available, and a replay link. It can be compared under named future
players with an explicit full-support cap. Preserve unknown/out-of-scope results
and partial progress, including cases too early for a full census.

Verify a complete hand through the real UI, native decision conformance,
reload/resume, flag export/import, and one complete comparison. Every experiment
or test workload uses a watchdog of at most 295 seconds. Long-lived local web
services are explicitly controlled development processes, not experiment jobs.
