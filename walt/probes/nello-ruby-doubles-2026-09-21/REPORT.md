# Ruby's Nel-O double leads

Exploratory investigation of the shared Plunge hand on 2026-09-21 (local date).

Replay: https://plunge-pr-4.texas42.workers.dev/#r=v1l063525043332220656253413011106160555440320066645144423121.PPPM1Dn206242416443663311442265316330

The doubles did not surrender a forced set in the revealed deal. They were not
needed to obtain that set: either 2–1 or 3–1 would have set the declarer immediately
at either of Ruby's two double leads. The engine's fresh hidden-information
comparisons favor doubles at the first position; its second-position preference
changes with the sample. This is evidence about this hand, not general validation
of Nel-O playing strength.

## Actual hand

Declarer: You (seat 0). Earl: seat 1. Gran: inactive seat 2. Ruby: seat 3.

| Trick | Plays in order | Winner |
|---|---|---|
| 1 | You 2–0, Earl 6–2, Ruby 4–2 | Earl |
| 2 | Earl 4–1, Ruby 6–4, You 4–3 | Ruby |
| 3 | Ruby 6–6, You 3–3, Earl 1–1 | Ruby |
| 4 | Ruby 4–4, You 2–2, Earl 6–5 | Ruby |
| 5 | Ruby 3–1, You 6–3, Earl 3–0 | You: set |

Before trick 3, your remaining tiles were 2–2, 3–3, 5–0, 5–2, 6–3.
Under doubles-own-suit rules, neither double follows a mixed two or three lead.
Thus 2–1 forces your 5–2, with Earl void in twos; 3–1 forces your 6–3, which
beats either of Earl's available threes (3–0 and 5–3). These immediate catches
remain available before trick 4.

The independent Python rules implementation decoded and validated the entire
record. A separate exhaustive, memoized full-information minimax over this
specific deal (4,799 states) found a forced set after every legal Ruby lead at
both positions. This diagnostic gives all players all hands and optimizes their
continuations; it is not the information or policy available to Ruby in the game.
See actual.py, actual.json, and actual-run/run.json.

## Fresh engine comparisons

Ran the preview's local, checksum-verified WASM artifact with seeds 1–8 for each
position. Each 160-world request completed its 40-world checkpoint and 160-world
comparison with eight inner worlds. All comparisons completed; no deadline
fallback was used. Output is retained in panel.json and summary.json.

Mean estimated **eventual set** probability across eight 160-world comparisons:

| Lead | Before 6–6 | Before 4–4 |
|---|---:|---:|
| 2–1 | 73.5% (941/1280) | 64.1% (41/64) |
| 3–1 | 80.2% (1027/1280) | 71.4% (457/640) |
| 4–4 | 83.1% (133/160) | 71.4% (457/640) |
| 5–1 | 79.5% (509/640) | 70.2% (449/640) |
| 6–6 | 84.6% (1083/1280) | — |

At the first position, the 160-world choices were 6–6 in 5/8 runs and 4–4 in
3/8. At the second, choices were 3–1 in 4/8, 4–4 in 2/8, and 5–1 in 2/8.
These are model estimates over sampled hidden deals, not calibrated real-world
probabilities, and the small seed panel does not establish optimal play.

The live browser's separate fresh 160-world recheck scored the first lead:
6–6 138/160, 4–4 137/160, 3–1 134/160, 5–1 128/160, 2–1 122/160.
Its second-lead scores were 3–1 111/160, 4–4 109/160, 5–1 108/160,
2–1 105/160. The replay UI explicitly reported that no original estimate was
available. Plain hand links carry the deal and plays, not the original decision
seed, receipt, scores, or timing. Therefore these results cannot identify the
exact historical comparison or whether it used a fallback.

## Why a winning double can have value

The source objective is eventual contract success: the declarer's first trick
is an absorbing failure; surviving all seven tricks is success. Defenders
minimize declarer success. There is no reward for taking defensive tricks,
points, or setting one trick earlier. Source: walt/walt/src/solver/contract.rs
(terminal), solver/selection.rs (fixed selection), and solver/mod.rs (best_of).

A high double can retain the lead and force other players to spend doubles
instead of discarding dangerous mixed tiles; it can also reveal who is void in
doubles. This is a possible tactical benefit, not a recovered explanation trace
for the original decision. A void declarer can instead discard a dangerous tile,
so such leads are not universally good. Also, Ruby cannot know 4–4 is a certain
winner: 5–5 remains unseen and is actually in Gran's inactive hand.

On this deal, both double leads preserved the winning continuation, and the
eventual 3–1 caught 6–3. There is no demonstrated suit-rule or objective reversal
in this investigation. The shaky ranking at the second position and any broader
response-model weakness remain separate from a proven implementation defect.

## Provenance and reproduction

- Texas 42 worktree HEAD: e4c549e2fd533c942f39125bf3134ac707c4e638.
- Plunge preview worktree HEAD: ce5995fbffa84e1712cc4e63d57bc400fc3258b9.
- WASM source commit from manifest: dba963feb112c0e11865181c51df6e02c556be5e.
- WASM SHA-256: fe22d2d24c33e98327799e2e08972481b4e1ec4e48f40256b22b32d0ccd76398.
- positions.json retains the shared code, public/own-hand requests and actual
  remaining hands. Only the public/own-hand requests were passed to WASM.
- panel.mjs evaluates the frozen artifact; actual.py uses the independent Python
  rules for a full-information diagnostic. They use this session's local paths.
- run/run.json and actual-run/run.json retain watchdog commands and completion.
- No game source or deployment was changed.
