# Local browser smoke record

Exploratory engineering check on the Mac's Codex in-app browser, 2026-09-20.
Local Vite instance: `http://127.0.0.1:5189/`. This is not a Pixel measurement.
Imported shared player: source `dba963feb112c0e11865181c51df6e02c556be5e`,
WASM SHA-256 `fe22d2d24c33e98327799e2e08972481b4e1ec4e48f40256b22b32d0ccd76398`.
Plunge includes upstream `aff966d` saved-hint integration.

The UI accepted an ordinary one-mark bid after Ruby passed; Earl and Gran then
passed. The human chose Nel-O from the declaration sheet. Gran displayed seven
face-down dominoes and “sitting this one out” throughout.

Observed plays:

1. Human 0-0, Earl 6-6, Ruby 2-2: Earl wins.
2. Earl 5-1, Ruby 2-0, human 5-5: Earl wins. The double is a legal slough on
   the mixed five lead, not a trump. A page reload/resume before the human play
   retained this partial trick and the Nel-O contract.
3. Earl 6-0, Ruby 6-2, human 6-3: human wins; immediate “Set — the bidder won
   a trick. 1 mark their way.” The hand ends after three tricks, with real
   displayed points Us 1 / Them 12.

Before the human's second play the browser completed a 40-world hint for 5-5:
26/40 makes. “Why this hint?” saved it in the local notebook with “Trick 2 ·
before play 3 of the trick,” original choice scores and only the human's hand.
The local question service is absent: the UI truthfully reports that the copy
is saved on this device and awaiting a connection. No production question was
posted.

Finished-hand inspection opened Earl's 6-0 as physical play 7. Its original
receipt showed 28/40 sets. “Think deeper” completed 160 worlds, showed 110/160
sets, retained the original panel above, and reported 0.39 seconds. “Share this
hand” displayed “Link copied!”; portable replay and receipt round trips are
separately checked against all pinned fixtures in `tests/nello.test.ts`.

Two display/validation remnants were found and corrected during the check:
hint prose assumed four plays, and the upstream notebook integration initially
retained a four-play lead index. Regression coverage now includes every legal
alternative and saved human hints throughout all fixture histories.
