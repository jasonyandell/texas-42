id: [[exchange-quota-reframe]]
opened: 2026-08-24

## What

Kill the fixed-number exchange budget framing in the main docs
(CLAUDE.md exchange section, exchange/ docs): replace with the
clear-quota-up-front protocol — dispatches authorized by Jason in
batches, each batch's ceiling agreed before it opens, monthly pacing,
never a lifetime cap. Standing task from Jason, pre-dates this board.

## Done when

No doc states a fixed lifetime number; the batch protocol is the only
description; submission_count.txt semantics documented.

## Links

exchange/, CLAUDE.md

## Closed 2026-08-24

Merged to main via PR #12 (re-issue of stacked #10; commits 1b6ab53 +
fb28084). Batch protocol is the only framing everywhere live; the
pro-exchange skill doc's "10 total, ever" cap killed; tally vs batch
ceiling documented at README, CLAUDE.md, and beside HARD_CAP.
Verified: non-comment diff on both .mjs files empty; nothing
submitted. Residue: finish-001.mjs keeps its dead count>=10 guard,
comment-flagged — deletion is a separate small call.
