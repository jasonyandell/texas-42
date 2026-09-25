# What a double lead exposes

Exploratory tactical diagnostic, using only Ruby's own hand and public plays.
This complements the sampling panels; it does not identify the optimal move.

Jason's objection puts a real burden on the double-lead explanation. Leading
6–6 cannot catch the declarer on that trick. If the declarer has a double, they
receive a safe play and may dispose of a dangerous high double. If they have no
doubles, they may discard any mixed tile. Merely calling this "drawing doubles"
does not establish a benefit over attacking a mixed suit immediately.

Any advantage must come from the subsequent position or information revealed by
the three plays. The solver can value that information through its continuation
search. This is a possible mechanism, not a demonstrated explanation of the
original decision. Small-sample optimization may also overvalue such branches.

There are no publicly revealed voids at either inspected root. Under the
engine's uniform distribution over mechanically feasible allocations, exact
combinatorics therefore give:

| Before Ruby's lead | Unseen tiles | Unseen doubles | Declarer's tiles | Declarer has no doubles |
|---|---:|---:|---:|---:|
| 6–6 (ply 6) | 17 | 5 | 5 | C(12,5)/C(17,5) = 198/1547 = 12.80% |
| 4–4 (ply 9) | 15 | 3 | 4 | C(12,4)/C(15,4) = 33/91 = 36.26% |

The corresponding support contains 4,900,896 and 450,450 complete hidden-hand
allocations, including the inactive player's seven tiles. Every possible
declarer hand of the required size has the same number of completions for the
other two hidden hands, which justifies the combinations above.

4–4 has a different immediate possibility from 6–6: the declarer can hold 5–5
as their only double, in which case 4–4 forces an immediate set. Its exact
frequency under the same model is 495/6188 (8.00%) at the first position and
44/273 (16.12%) at the second. This does not make 4–4 best overall; it is one
mechanical distinction between the two double leads. The actual 5–5 was in the
inactive hand, which Ruby could not know.

These percentages describe the engine's uniform legal-deal model. They are not
probabilities conditioned on why a person bid Nel-O or chose earlier plays.
They were not computed from the actual concealed hands in the shared replay.

Reproduction: double-exposure.py imports the independent Python rules,
validates both public/own-hand requests and the absence of voids, then computes
integer allocation counts and exact rational probabilities. Output is in
double-exposure.json; exposure-run/run.json records bounded completion.
