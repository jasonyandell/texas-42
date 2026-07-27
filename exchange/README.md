# Exchange protocol: Claude ↔ ChatGPT 5.6 Pro

ChatGPT 5.6 Pro has no API. This directory is the courier channel.

## Loop (automated via `buddy`)

1. A dispatch author writes a self-contained prompt to `outbox/NNN-<slug>.md`.
   Each prompt inlines every definition it needs — 5.6 Pro sees nothing
   outside the pasted text. No "see the spec" references. When the file is
   complete, the author creates an empty marker `outbox/NNN-<slug>.ready`.
2. The `buddy` automation agent watches `outbox/`, submits each ready dispatch
   to ChatGPT 5.6 Pro via browser automation (new chat at https://chatgpt.com/,
   model picker set to Intelligence → **Pro**), records the conversation URL,
   and polls until the response completes (expect 1–2 h, timeout 3 h).
3. buddy harvests the full response to `inbox/NNN-<slug>.md` with a metadata
   header (conversation URL, submitted/harvested timestamps), and updates the
   ledger below.
4. Claude adjudicates every inbox file against the foundation before anything
   enters the wiki: witnesses re-run, programs executed, proofs step-checked.

Dispatch file format: YAML frontmatter (`number`, `slug`,
`channel: new-chat | continuation`, `deliverable:` one line), then the body.
buddy pastes the body only, verbatim. `channel: continuation` means post into
the standing conversation
https://chatgpt.com/c/6a64ccec-2328-83ea-b0d1-917f487297a2 instead of a new chat.

## Hard budget

**Maximum 10 submissions total, ever, across all sessions** — tracked in
`submission_count.txt` (one line, the integer count; buddy increments it
before each send). A send only counts once visually confirmed submitted;
a harness failure before the message leaves the composer does not count,
but buddy must verify in the UI before retrying.

## Prompt discipline

Every outbox prompt must be an *adversarial task with a checkable deliverable*,
never "review this":

- "Construct a counterexample to X, or prove no counterexample exists."
- "Here is a claimed proof of Y. Find the first incorrect step, or certify
  each step."
- "Compute Z exactly and provide a certificate Claude can verify mechanically."

Deliverables must be machine-checkable where possible: explicit witnesses,
Python that re-derives the number, Lean statements, exact fractions — so the
answer can be verified here without trusting the model.

## Status ledger

| # | slug | dispatched | response | adjudicated | outcome |
|---|------|-----------|----------|-------------|---------|
| 001 | reachable-support-cardinality | 2026-07-27T05:07:18Z [conv](https://chatgpt.com/c/6a66e786-2ac0-83ea-ade0-dff707fae5e6) | pending | — | — |
