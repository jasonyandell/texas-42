# ChatGPT 5.6 Pro browser-automation harness

Small composable scripts that drive a real, logged-in Chrome via CDP to submit
`exchange/outbox/` dispatches to ChatGPT 5.6 Pro and harvest responses into
`exchange/inbox/`. See `exchange/README.md` for the dispatch protocol and the
quota rules — dispatches are authorized by Jason in batches, each batch's quota
agreed up front; there is no lifetime cap.

## How it connects

`launch-chrome.sh` copies the minimal login state (Local State, Default/Cookies,
Preferences, Login Data, Web Data) from Jason's real Chrome profile into
`~/Library/Application Support/Google/Chrome-buddy`, then launches a second
Chrome instance there with `--remote-debugging-port=9222`. macOS Chrome >=136
refuses CDP on the default profile dir, and this leaves the user's own Chrome
untouched. Cookies decrypt because the Keychain "Chrome Safe Storage" key is
per-user, not per-profile-dir. Scripts attach with
`playwright-core`'s `connectOverCDP`.

Re-run with `--fresh` to re-copy cookies if the session ever goes stale.

## Scripts

| script | purpose |
|---|---|
| `launch-chrome.sh` | start (or reuse) the CDP Chrome on the copied profile |
| `check-login.mjs` | open chatgpt.com, screenshot, report login state |
| `rehearse.mjs` | dry-run capability proof — never sends |
| `submit.mjs <outbox/NNN-slug.md>` | full submission: parse frontmatter, clear draft, verify **Pro** model, attach files, clipboard-paste body, verify fidelity, send, confirm, increment `exchange/submission_count.txt`, write `NNN-slug.submitted.json` |
| `poll.mjs <NNN-slug.submitted.json>` | exit 0 if response complete, 2 if pending |
| `harvest.mjs <NNN-slug.submitted.json>` | save final assistant message to `exchange/inbox/NNN-slug.md` (copy-button markdown preferred, innerText fallback) + screenshot |
| `lib.mjs` | shared CDP/page helpers |

## Selector notes (chatgpt.com, July 2026)

- Composer: `#prompt-textarea` (ProseMirror contenteditable). Drafts persist
  across page loads — always clear before composing (`clearComposer`).
- File upload: hidden `input#upload-files` (accepts any type); `#upload-photos`
  / `#upload-camera` are image-only. `setInputFiles` on `#upload-files` works
  without opening the "+" menu.
- Upload completion: send button (`[data-testid="send-button"]`) is disabled
  while any attachment uploads; complete = filename visible in form innerText
  AND send enabled.
- Model picker: composer button showing e.g. "Pro"; menu items via
  `[role="menuitem"]`. `ensureProModel` hard-fails the submission if the picker
  does not read "Pro" after selection.
- Messages: `[data-message-author-role="assistant"|"user"]`. Completed turns
  render `[data-testid="copy-turn-action-button"]`; its click puts the turn's
  markdown source on the clipboard (needs `clipboard-read` permission granted
  via CDP).
- Clipboard paste (`Meta+V` after `navigator.clipboard.writeText`) preserves
  the body verbatim and is fast; note it clobbers the system clipboard.

## Logs

Everything appends to `logs/harness.log`; screenshots land in `logs/*.png`
(`NNN-...-pre-send`, `NNN-...-sent`, `NNN-final`).
