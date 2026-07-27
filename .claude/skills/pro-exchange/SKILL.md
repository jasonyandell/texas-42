---
name: pro-exchange
description: Run the ChatGPT 5.6 Pro research exchange — submit dispatches from exchange/outbox/ via browser automation, poll conversations, harvest responses to exchange/inbox/. Trigger on "send this to pro", "send to buddy/chatgpt", "dispatch to 5.6 pro", "harvest responses", "check the pro conversations", or any exchange/ courier work.
---

# ChatGPT 5.6 Pro exchange

ChatGPT 5.6 Pro has no API (app/web only; codex CLI lacks the Pro tier). The
exchange runs through browser automation against Jason's logged-in account.
Everything lives in `exchange/` (protocol + ledger) and `automation/` (harness).

## Budget — read first

**Hard cap: 10 submissions total, ever, across all sessions.**
`exchange/submission_count.txt` holds the live count — **read that file, never
trust a number quoted here**; any count in this doc is a stale snapshot.
A send counts once it is visually confirmed submitted. Increment the count and
add the ledger row in `exchange/README.md` in the same commit, immediately
after confirmation. If a send attempt errors ambiguously, **open the
conversation in the UI and check before deciding whether it counted — never
risk a double-spend by retrying blind.**

## Courier protocol

- Dispatch: `exchange/outbox/NNN-slug.md` — YAML frontmatter (`number`, `slug`,
  `channel: new-chat | continuation`, optional `conversation_url:`,
  `attachments:` list of repo-relative paths, `deliverable:` one line) then the
  body, pasted verbatim. The file is final only when the empty marker
  `exchange/outbox/NNN-slug.ready` exists.
- **Channel targeting (three modes):**
  - `channel: new-chat` — fresh conversation at https://chatgpt.com/.
  - `channel: continuation` with **no** `conversation_url` — posts into the
    standing conversation
    https://chatgpt.com/c/6a64ccec-2328-83ea-b0d1-917f487297a2 (the general
    follow-up channel).
  - `channel: continuation` **with** `conversation_url:` — posts the follow-up
    into that *specific* prior conversation, so the model still has its own
    machinery from the earlier dispatch in context (e.g. sending 006 back into
    the 001 conversation, where its floor construction is still live). Prefer
    this for targeted follow-ups; the standing channel is only for generic
    continuations. Always keep the body self-contained anyway — the continuation
    context may be unavailable or truncated, and if the targeted conversation is
    locked you fall back to a new chat (note the channel change in the ledger).
- Prompt discipline (see exchange/README.md): every prompt is an *adversarial
  task with a machine-checkable deliverable* — explicit witnesses, programs
  that re-derive numbers, exact fractions. Never "review this".
- Response lands in `exchange/inbox/NNN-slug.md` with a metadata header
  (conversation URL, submitted/harvested timestamps, extraction method).
- Adjudication: inbox files are adjudicated via
  `exchange/adjudication/workflow.js` (witnesses re-run, programs executed,
  proofs step-checked) before anything enters the wiki.

## Harness usage (automation/)

```sh
automation/launch-chrome.sh          # start CDP Chrome (idempotent); --fresh re-copies cookies
node automation/check-login.mjs      # screenshot + login state
node automation/rehearse.mjs         # dry-run capability proof — NEVER sends
node automation/submit.mjs exchange/outbox/NNN-slug.md   # full submission
node automation/poll.mjs   exchange/outbox/NNN-slug.submitted.json  # exit 0 done / 2 pending
node automation/harvest.mjs exchange/outbox/NNN-slug.submitted.json # write inbox file
automation/poll-loop.sh              # LEGACY background loop — see WEDGE LESSON below
```

**WEDGE LESSON (learned the hard way).** `poll-loop.sh` (and any pattern that
exits the watcher process on completion and *expects the harness to re-invoke
you* to harvest) is a trap: the completion notification was delayed for HOURS in
practice, so the response sat un-harvested and the whole exchange wedged.
**Detection and harvest must happen in the SAME live process / same turn.** Do
not split "notice it finished" from "grab the text" across a process exit — the
moment you detect completion, harvest immediately in that same live turn before
doing anything that could end the turn.

- `launch-chrome.sh` copies minimal login state (Local State, Default/Cookies,
  Preferences, Login Data, Web Data) into `~/Library/Application
  Support/Google/Chrome-buddy` and launches a **second** Chrome with
  `--remote-debugging-port=9222`. macOS Chrome >=136 refuses CDP on the default
  profile dir; the copy sidesteps that and leaves Jason's Chrome untouched.
  Cookies decrypt because the Keychain "Chrome Safe Storage" key is per-user.
- `submit.mjs` parses frontmatter, clears any persisted draft, verifies the
  model, uploads attachments, clipboard-pastes the body, verifies fidelity,
  sends, waits for the server-assigned conversation UUID, increments the
  budget, writes `NNN-slug.submitted.json`, then reloads the conversation and
  verifies the sent turn contains the body and every attachment.
- Logs append to `automation/logs/harness.log`; screenshots (`NNN-*-pre-send`,
  `-sent`, `-verified`, `NNN-final`) land in `automation/logs/`.
- Pro responses take 1–2 h (deep research; one finished in ~25 min). 3 h
  timeout per dispatch from submission; on timeout note it in the ledger — do
  **not** resubmit.
- Run submissions before harvests in the same session; scripts are independent
  and safe to rerun (poll/harvest are read-only).
- Poll/harvest target a specific conversation via its
  `exchange/outbox/NNN-slug.submitted.json` (written by submit.mjs; carries
  `conversationUrl` + `baselineAssistantCount`). Always pass that meta file —
  never guess URLs or rely on sidebar ordering.

## Waiting protocol (post-wedge, 2026-07-27)

Jason's instruction, verbatim: "run those questions through one at a time,
staying on the single webpage and just checking it rather than reloading. if
nothing changes, a reload once per 10 minutes as a backstop. easier to manage
and less chance of wedge and lighter on potential limits."

Concretely:

- Watch **one pending conversation at a time** in ONE open tab, ordered by
  likely completion. Check completion by observing the live DOM in place (the
  page streams; no navigation needed), reloading that tab at most once per
  10 min as a backstop if nothing changes. When it completes, harvest it **in
  that same live process** (see WEDGE LESSON above), then move to the next
  conversation.

## UI gotchas (all bitten once; selectors in automation/README.md)

- **Model picker**: composer button opens the "Intelligence" menu (Instant
  5.5 / Medium / High / Extra High / **Pro**). Must read "Pro" per chat before
  every send; `ensureProModel` hard-fails otherwise. Screenshot-verify.
- **Long pastes collapse** into a "Pasted text" chip. The harness clicks "Show
  in text field" so the body goes inline — otherwise fidelity checks fail and
  the message would send as an attachment-like chip.
- **Duplicate uploads get renamed** server-side: `10_RULES.md` becomes
  `10_RULES(3).md`. Match attachments by extension-less stem (`chipStem`),
  never exact filename.
- **Upload completion**: the send button is disabled while any attachment
  uploads; complete = every stem visible in form text AND send enabled. Never
  send before this.
- **Placeholder URLs**: right after send the URL is `/c/WEB:<id>` (client-side
  temp). Wait for the real UUID (`/c/xxxxxxxx-xxxx-...`) before recording; if
  missed, recover the URL from the sidebar (conversation gets an auto title).
- **Drafts persist** across page loads (text only, not attachments) — always
  clear the composer before composing.
- **Completion detection**: a completed turn renders
  `[data-testid="copy-turn-action-button"]`; while generating there's a stop
  button. Harvest prefers the copy button (real markdown via clipboard) over
  innerText.

## Recovery and blockers

- Ambiguous send: check the conversation UI (or sidebar recents) for the sent
  turn before any retry. The budget increments on confirmed send even if
  post-send verification then fails.
- Login wall or captcha: **stop and surface to Jason** — do not attempt to
  clear it. If the session goes stale, `launch-chrome.sh --fresh` re-copies
  cookies from the real profile (Jason's Chrome must have a live session).
- Fallbacks if CDP breaks: "ChatGPT Atlas.app" (OpenAI Chromium, natively
  logged in — CDP flag untested) and macOS UI scripting (osascript/System
  Events + screencapture; `cliclick` 5.1 IS installed at
  /opt/homebrew/bin/cliclick but needs an Accessibility grant — System
  Settings → Privacy & Security → Accessibility for the terminal app — an
  interactive step only Jason can do). Neither fallback has been needed.
- Multi-agent repo: commit with explicit pathspecs only
  (`git add <paths> && git commit -- <paths>`), never `-A` or bare commit.
