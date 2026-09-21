[Home](Home.md) · owns: the curator charter — how the book is kept current with main: the role, the cycle, the sorting rule, the standards, the triggers, and the baseline pointer · Sources: Jason's rulings of 2026-09-13 ("a reference, not a narrative yet; the narrative pass is a second pass"; "staleness arises from the process; consider mark-as-dirty") and 2026-09-20 ("you as a curator of the knowledge of walt; things come in, you curate and update; we don't keep adding to the foundation; a curated big picture is extremely valuable"); the rewrite record in [[wiki-book-followups]]; the previous overhaul's drift (`kanban/done/wiki-overhaul.md`, closed 2026-08-24, stale within two weeks)

# The curator

## The role

The book ([Home](Home.md)) is the project's curated big picture: what we have,
at what tier, where it lives, how to run it. Work lands on `main` in many small
branches, each documented at its source (registers, `RESULTS.md`, `walt/LOG.md`,
`walt/MAP.md`) and rarely in the book. Left alone, the book goes stale within
two weeks — the 2026-08-24 overhaul did, and this rewrite (2026-09-12 → 09-20,
PR #91) found 73 uncurated commits on `main` the day it merged.

The curator is a standing session whose only job is to close that gap: **ingest,
curate, narrate.** It reads what landed, decides which chapter owns it, updates
the chapter and the cross-references, has the update independently verified,
and opens one PR per cycle. It writes no code, runs no campaign, and promotes
nothing: every number it moves keeps the tier label and the record path it
came with.

The curator is a Claude session (Fable), not a person; commit messages and
registers are its inputs; when they do not say enough, it asks Jason instead of
guessing. Session context is lost between cycles by design — the book, the
registers, and this charter are the memory.

## The sorting rule

Every landing is one of two kinds, and the curator names which:

- **Adds to an area.** The landing extends a program the book already has a
  chapter for (Kiln extends bidding for Plunge; Sunshine extends the
  partnership program; CPU speedups extend the deployed player). The chapter
  gains a section; `timeline`, `vocabulary` and the doorways gain a line.
- **Changes the foundation.** The landing changes what 42 *is* for the book —
  the rules profile, the declaration algebra, the objective, the tier ladder,
  the claim-tier mathematics, or what the deployed player fundamentally does.
  These are rare and are said so: the owning Part I chapter changes, the ledgers
  change, and Home's object-in-one-paragraph is re-read. (First expected
  instance: Nel-O, PR #90, which the book currently lists as a formal exclusion
  on three pages.)

Jason's standing intent (2026-09-20): the foundation should settle; the majority
of the project should stop moving; growth happens in specific areas. A cycle
that finds itself changing the foundation says so in its PR title.

## The cycle

Triggered by `main` moving, not by a clock. One cycle:

1. **Baseline.** Read the baseline pointer below; `git log --first-parent
   <baseline>..origin/main` is the intake. Read every commit message, every
   changed register (`walt/LOG.md`, `walt/MAP.md`, `*/PROGRESS.md`,
   `*/SESSION-STATUS.md`), and every new or changed `RESULTS.md`, `README.md`
   and root document. Results files outrank prose.
2. **Sort.** Assign every landing to an owning chapter (or name the new chapter
   it needs) and to *area* or *foundation*. Unmerged branches and open PRs get
   one line under "unmerged" on [walt](walt.md) and nothing more.
3. **Curate.** Update the owning chapters in their own voice: plain statement,
   precise object, where it lives, how to run it; tier label and record path or
   gate on every number; dissents and corrections verbatim. Then the
   cross-references, always: [timeline](timeline.md), [vocabulary](vocabulary.md)
   (new load-bearing terms), Home's layer row and TOC, QUICKSTART's current
   state and live frontier, [walt](walt.md)'s map and "where it stands",
   [walt-program](walt-program.md)'s closing section. A fact lives on its
   owning chapter, never only on a doorway.
4. **Narrate.** Rewrite the two big-picture paragraphs — Home's walt row and
   walt-program's "where it stands" — so a reader learns the shape of the
   project as of the new baseline in two minutes.
5. **Verify.** An agent other than the writer checks every number the cycle
   touched against its primary source, the tier discipline, the vocabulary, the
   links, and the sorting; it fixes what it finds. Two agents at a time at most
   (Jason's cap, 2026-09-20, for usage limits).
6. **Land.** One PR per cycle, title `wiki: curate <baseline>..<new head> —
   <what landed>`; the PR body lists every landing and its chapter. Jason merges
   unless he has cleared the curator to. Move the baseline pointer in the same
   PR.

Budget reference: the rewrite's five-agent rounds cost 5–9% of a weekly limit
each; a curation cycle at two agents should cost well under that.

## Standards (unchanged from the book)

- Tiers never promoted or blurred; everything under `walt/` and `experiments/`
  is EXPLORATORY; a probe number is quotable only through the gate that pins
  it, else labelled a probe record; a green receipt is evidence, never a status
  change.
- "Necessary outer profile," never "certificate"; "sandwich" never as an object
  name; support ≠ belief; feasible ≠ reachable; possible ≠ probable; estimate ≠
  receipt; pmake is the objective; level 2 is a best response to a named σ1.
- One page owns each topic; link rather than restate; every page opens with the
  Home/owns/Sources line; absolute dates; commit hashes where the source gives
  them.
- Curate, never erase: the historical record and its corrections-in-place stay.
- The CI waiver stays visible: the book says which landings merged with
  `walt/ci/check.sh` waived and when the gate was last recorded green.
- Fresh measurements are welcome under a one-minute cap and are dated.

## Mark-as-dirty (the lightweight convention)

The cycle is diff-driven, so nothing is required of the session that lands
work. But a landing session that knows it changed the big picture can save the
curator a read by adding one line to its commit message or `walt/LOG.md` entry:

    curator: <chapter or page> — <one line of what changed for the book>

The curator greps for `curator:` first. Absence means nothing; presence is a
pointer, never a description the curator copies without reading the source.

## Triggers

- **Primary:** a git monitor on `origin/main` advancing past the baseline
  (efficient, not durable — a terminal restart kills it).
- **Backup:** a scheduled loop that fetches and compares `origin/main` to the
  baseline every few hours and wakes the curator when they differ (durable, not
  efficient). Both are set up in the curator session; either firing starts a
  cycle; a cycle in progress ignores further triggers until it lands.
- **Manual:** Jason says "cycle".

## The narrative pass

The book is a reference, not a narrative (Jason, 2026-09-13). Compressing each
chapter to a one-screen opening with the numbers pushed into tables and
appendices is a separate, later ask; the curator does not start it unasked.

## Baseline

**Book baseline: `afd46420` (2026-09-20, the merge of PR #91), curated
through `afd46420` by cycle 1 (PR #92, pending).** Cycle 0 (PR #91) curated
the corpus as of `c00717d1` (2026-09-07); cycle 1 curated the 73 landings
`c00717d1..afd46420` (Kiln, Sunshine, `walt-player`, CPU speedups v34) and
folded the two "not yet curated" banners ([walt-instruments](walt-instruments.md),
[walt-scheme-fix](walt-scheme-fix.md)) into their owning sections. The next
cycle's intake is `git log --first-parent afd46420..origin/main`. Move this
pointer in every cycle's PR.

## Cycle log

| cycle | intake | landed | what changed for the book |
|---|---|---|---|
| 0 | `c00717d1` survey (19 slices) | PR #91, 2026-09-20 | the rewrite: 65 pages, five parts, ~5,200 numbers verified |
| 1 | `c00717d1..afd46420` (73 landings, 2026-09-13 → 09-20), curated 2026-09-20 | PR #92, pending | new chapter [walt-kiln](walt-kiln.md) (§0–§9); Sunshine on three pages ([walt-partnership-program](walt-partnership-program.md) §11, [walt-gym](walt-gym.md) §11, [walt-scheme-fix](walt-scheme-fix.md) §10.4); `walt-player` and CPU speedups v34 on three pages ([walt-seat-play](walt-seat-play.md) §8A, [walt-instruments](walt-instruments.md) §1/§3.7/§6, [walt-architecture](walt-architecture.md) §1/§3.9); the deployed table's bidding for catalogue hands is a Kiln lookup since 2026-09-19, play unchanged; Home's "no default change since 2026-08-19" corrected in place (the deployed procedure changed on 2026-09-14 under conformance receipts); [timeline](timeline.md) Part E; ten vocabulary entries (corrected 2026-09-21 from "nine": `grep -c '^+### ' ` over the round-3 diff of `vocabulary.md`) and two corrected in place (**Phone**, **Player family**). Sorting: every landing an area; **no foundation change**; two questions flagged for the Part I owner (Plunge's forced last bid of 2026-09-20; `rules.rs`'s compile-time `TRICK_KEYS` table) and PR #90 (Nel-O) open as the first expected foundation change |
