# exchange/drafts/ — adversary-panel dispatch DRAFTS

**STATUS UPDATE 2026-08-24: the five panel briefs were hand-ferried by
Jason as an authorized batch of five (quota cleared by his delivery) and
now live in `../outbox/` as dispatches 019–023. The consolidated
response is `../inbox/019-023-response-panel-and-cancellation-v0.1.md`,
adjudicated same day. This directory is empty pending future drafts; the
rules below govern anything drafted here.**

**STATUS (for any future draft): DRAFT — NOT DISPATCHED until Jason's explicit go.**

Nothing in this directory has been sent, scheduled, or queued. These files are
**unsent and unauthorized until Jason's explicit go**, per the batch protocol in
[`../README.md`](../README.md):

- Dispatches are authorized by Jason **in batches, each batch's quota agreed up
  front** (monthly pacing, cleared per batch). There is no lifetime cap and no
  fixed total. **Never submit without Jason's explicit go for the batch.**
- No file here has a dispatch number, and none may claim one until it is moved
  to `outbox/` under an authorized batch. No `.ready` markers exist or may be
  created here; the automation never reads this directory.
- `submission_count.txt` is untouched by anything in this directory.
- On return, every response is adjudicated before anything touches the wiki:
  **witnesses re-run, programs executed, proofs step-checked.**

## Provenance

These drafts are the adversary-panel briefs anticipated by ruling **CE-A8** in
`walt/CENSUS-RULINGS.md` (2026-08-24): "No adversary panel is convened now; one
may be convened later on Jason's word under the batch protocol, with CE-T1..T5
and O21/O24/O26 as the natural briefs." The mathematics under review is
**walt-tier exploratory** material (`walt/math/calculated_evidence_v0.1.md` and
`walt/math/targeted_level2_field_stability_v0.1.md`, with their intake
companions). The briefs seek independent adversarial review of exploratory
claims; nothing about drafting or dispatching them promotes any claim to a
higher evidentiary tier.

## Index

| draft | attacks | primary sources |
|---|---|---|
| [`panel-ce-evidence-process.md`](panel-ce-evidence-process.md) | CE-T1/T2/T3: the finite-sum and closed-form evidence identities and the anytime-valid supermartingale claims | CE parent §3–§4, intake §1, CE-A1 |
| [`panel-ce-bounded-mean.md`](panel-ce-bounded-mean.md) | CE-T4/T5: bounded-mean betting processes, λ-range validity, mixture construction; the §10.1 sign-majority defect | CE parent §10, intake §1, CE-A5/V7 |
| [`panel-ce-risk-ledger-escalation.md`](panel-ce-risk-ledger-escalation.md) | O21 + O24: all-pairs and telescoping risk allocation, safe elimination, and the sample-to-enumeration switch bookkeeping | CE parent §5–§6, §11; O21/O24 rows in `walt/SCENARIO-PLAYER.md`; CE-A8 items (2)(4) |
| [`panel-ce-execution-order.md`](panel-ce-execution-order.md) | O26: execution-order invariance of evidence, plus the CE-A8 item (1) predictable-sequence / conditional-null question stated precisely | CE parent §17, intake §5 item 1, CE-A8 item (1), O26 row |
| [`panel-l2-coupling-theorems.md`](panel-l2-coupling-theorems.md) (optional fifth) | L2-T1..T5: coupling and field-stability theorems — the sup/optimization-lock step of L2-T2 and the L2-T4 bar argument | `walt/math/targeted_level2_field_stability_v0.1.md` §3–§5, §13; L2-A1 |

## Dispatch mechanics when (and only when) authorized

1. Jason authorizes a batch and its quota.
2. A draft moves to `outbox/NNN-<slug>.md` with the next number, frontmatter
   completed (`number`, `channel`, `status: cleared by Jason <date>`), and the
   DRAFT header block removed.
3. Only then may a `.ready` marker be created, and only for the automated path
   within the batch ceiling.

Until step 1 happens, this directory is inert text.
