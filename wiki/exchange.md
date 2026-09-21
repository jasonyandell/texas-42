[Home](Home.md) · owns: the Claude ↔ ChatGPT 5.6 Pro exchange as a whole — the courier mechanism, the evidentiary tier it produces, the quota protocol, every numbered dispatch 001–024 and what came of it, the reading discipline for an exchange result, the incidents and costs, and the ledger-drift errata · Sources: `exchange/README.md` (the operational ledger of record), `exchange/outbox/`, `exchange/inbox/`, `exchange/adjudication/{workflow.js, programs/, witnesses/}`, `automation/submit.mjs`, `.claude/skills/pro-exchange/SKILL.md`, [claim-ledger](claim-ledger.md) (caveats verbatim), `walt/CENSUS-RULINGS.md` (the walt-tier rulings), [walt-math-intakes](walt-math-intakes.md) (the side channel), `git log -p -- exchange/submission_count.txt`. Repository state as of 2026-09-07 (c00717d1); fresh measurements dated 2026-09-13 (an earlier pass on 2026-09-12 gave the same PASS counts and outputs).

# The adversary in the loop — the ChatGPT 5.6 Pro exchange

**Plain statement.** From 2026-07-27 the project has used a second model, ChatGPT 5.6 Pro, as an adversary and later as a colleague. It has no API, so every exchange is a courier run: a self-contained prompt is pasted into the web app, the reply is harvested verbatim, any program in the reply is executed unmodified, and the proof is attacked by three referees before a verdict is written. Twenty-four numbered dispatches exist. Eleven produced results at the **exchange-adjudicated** tier — the flagship open problem OPEN-11 went from a corpus-proved 26–46-bit interval to **[36,45] bits**, its no-void stratum was closed exactly at **624,892,870**, and OPEN-01 collapsed. The nine dispatches 016–024 (five threads: 016, 017, the unanswered 018, the 019–023 panel batch, 024) fed the walt program inside its exploratory fence, where Pro's mathematics has since arrived mostly by side channel rather than by numbered dispatch.

**Precise object.** An exchange result is a triple (dispatch prompt, harvested response, adjudication record) whose verdict is one of CONFIRMED / PARTIAL / REFUTED / UNVERIFIABLE / CONTRACT_VIOLATION, cited as `x:NNN`. The tier it can reach is tier 3 of [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred): below corpus statuses and the Lean kernel, above rob's conformance receipts. Nothing an external model asserts is ever imported as an axiom (TRUST-01, v0.7).

**Where it lives.** `exchange/README.md` is the ledger of record and stays operational; this page is the book's account. Prompts are `exchange/outbox/NNN-<slug>.md`, replies `exchange/inbox/NNN-<slug>.md`, Pro's programs `exchange/adjudication/programs/NNN.py`, extracted witnesses `exchange/adjudication/witnesses/`, and the adjudication workflow `exchange/adjudication/workflow.js`. Section 11 says how to re-run every receipt.

## 1. The mechanism

### 1.1 The courier loop

1. **Author** a dispatch to `outbox/NNN-<slug>.md`: YAML frontmatter (`number`, `slug`, `channel: new-chat | continuation`, optional `conversation_url`, `attachments`, one-line `deliverable`), then a body that inlines every definition it needs. Pro sees nothing but the pasted body and the attached ingest documents (`10_RULES.md`, `20_MATHEMATICAL_FOUNDATION.md`, sometimes one ingest verifier). An empty `NNN-<slug>.ready` marker signals completion.
2. **Submit** — the automated path (`automation/submit.mjs`, a CDP-driven Chrome on Jason's logged-in account): ensure the Pro model, attach files, clipboard-paste the body, verify fidelity, send, write `NNN-<slug>.submitted.json` (created with an exclusive `wx` write *before* the send — the double-send guard, §8.1), increment the tally. Or the hand-ferried path: Jason pastes the outbox body into the app himself and uploads the reply file. Every dispatch since 016 (2026-08-14) has been hand-ferried. The automated path's supporting scripts, all under `automation/` (documented in `automation/README.md`): `launch-chrome.sh` copies the login state into a `Chrome-buddy` profile and starts Chrome with `--remote-debugging-port=9222`; `check-login.mjs` screenshots the login state; `rehearse.mjs` is a never-sends capability dry run; `lib.mjs` holds the shared CDP helpers (`ensureProModel`, `pasteIntoComposer`, `attachFile`, `parseDispatch`, `chipStem`); the `probe-*.mjs` files are read-only selector probes whose notes date from July 2026. None of them sends anything; only `submit.mjs` does, and only under §3's rules.
3. **Harvest** — `automation/harvest.mjs` / `watch-harvest.mjs` save the final assistant message to `inbox/NNN-<slug>.md` with a metadata header (conversation URL, `submitted-at`, `harvested-at`, `extraction: copy-button`), preferring the copy-button markdown over `innerText`. Hand-ferried replies open with a `<!-- HARVEST METADATA -->` block marked `status: UNADJUDICATED`.
4. **Adjudicate** before anything touches the wiki: witnesses re-run, programs executed, proofs step-checked (§1.3).

### 1.2 Prompt discipline

Every prompt is an *adversarial task with a checkable deliverable*, never "review this" (`exchange/README.md` § Prompt discipline). The three shapes:

- "Construct a counterexample to X, or prove no counterexample exists."
- "Here is a claimed proof of Y. Find the first incorrect step, or certify each step."
- "Compute Z exactly and provide a receipt Claude can verify mechanically."

Deliverables are machine-checkable where possible — explicit witnesses, stdlib-only Python that re-derives the number, Lean statements, exact fractions — so the answer is verified here without trusting the model. Responses are asked for a `FINAL ANSWER:` line, numbered proof steps with `[USES:]` labels, and a `MACHINE-CHECKABLE ARTIFACTS` block; missing any of these is a contract violation the extractor records.

### 1.3 Adjudication: four phases, three lenses, one rule

`exchange/adjudication/workflow.js` (`adjudicate-pro-responses`) runs per inbox file:

| Phase | What happens |
|---|---|
| Extract | Every `FINAL ANSWER` line verbatim; the program saved **exactly as given** to `programs/<n>.py`; JSON witnesses to `witnesses/<n>.json`; count of numbered steps, unlabeled steps, contract violations (missing FINAL ANSWER, non-stdlib imports, file I/O or network, scientific-notation integers). No judgement of correctness. |
| Execute | `cd exchange/adjudication && timeout 2700 python3 programs/<n>.py` — a 45-minute hard cap although the dispatch allows 6 h; capture every PASS/FAIL line; check specifically that the corpus anchors the program claims to reproduce (e.g. 44,352,165) actually printed PASS. The program is never modified. |
| Verify | Three adversarial referee lenses, each an Opus agent at default effort, each told to "default to FLAWED/UNVERIFIABLE unless the material survives your genuine best attack": **proof-chain** (walk every numbered step and its `[USES:]` label; find the first gap, circularity, or scope error — finite receipts cited as general theorems, deduplication semantics silently changed, canonicalization merging or missing states), **program-vs-claim** (does the executed program compute what the FINAL ANSWER asserts, or does it assume the answer, hardcode tables, weaken checks, verify a subtly different object — wrong dedup, gauge, slice), **corpus-consistency** (every integer, definition and claim ID against `ingest/` and the wiki; run ingest verifiers if needed; catch any silent redefinition of support NF, reachability, outer profile). Each returns SOUND / FLAWED / UNVERIFIABLE with a confidence and a "what survives regardless" line. |
| Verdict | One xhigh agent per response (the model policy: xhigh at most once per question). |

The verdict rule, verbatim from the workflow: *"CONFIRMED needs the program green on its own claims AND no referee finding a real flaw; honest partials score PARTIAL; a single confirmed flaw in the load-bearing chain is REFUTED for the headline but record salvage; execution TIMEOUT with sound proof chain is UNVERIFIABLE-leaning-PARTIAL."* The verdict also lists the concrete wiki edits it justifies, and only those.

Two consequences of the rule matter for reading the ledger. First, **a referee marked UNVERIFIABLE who found no defect does not block CONFIRMED** — that is how REACH-20 was confirmed at 2/3 SOUND + 1 UNVERIFIABLE (§4). [Home](Home.md)'s one-line tier definition says "3/3 adversarial referees SOUND", which is stricter than the rule actually applied; [claim-ledger](claim-ledger.md) carries the rule and the dissent, and the dissent must travel with the result. Second, **a referee marked FLAWED who confirmed a flaw outside the load-bearing chain yields PARTIAL, not REFUTED** — that is x:009 (§5).

The workflow was run 2026-07-27 as `wf_775fe0ec` for 001–005 (thirty agents: five responses × one extractor, one executor, three referees, one verdict), then per response for 006/007/008 the same day, and 2026-08-01 in two rounds for 009/012 and 010. Two constants in the file are stale: `REPO` is hard-coded to the main checkout, and the verdict prompt still mentions a "5-shot reserve" from the fixed-cap era retired 2026-08-01 (§3).

### 1.4 Three registers

| Register | Dispatches | Adjudicator | Highest tier reachable |
|---|---|---|---|
| Automated adversarial dispatch with a machine-checkable deliverable | 001–012 (and the Lean thread 011/013/015, which iterated without a panel by the 2026-08-01 policy) | `workflow.js` panel | exchange-adjudicated CONFIRMED (tier 3) |
| Hand-ferried adversarial panel on walt mathematics | 019–023, 024 | walt-math same-day intake, rulings filed as `PANEL-A*` / `TRIPLE-A*` in `walt/CENSUS-RULINGS.md` | EXPLORATORY (walt fence) — never the CONFIRMED pipeline |
| Colleague correspondence ("the x:014 register": no deliverable contract) | 014, 016, 017, 018 | 016/017 walt-math intake (`FT-A*`, `SR-A*`); 014 none; 018 no reply | EXPLORATORY; 014 UNADJUDICATED |
| Informal capture outside the protocol | `exchange/informal/2026-08-03-…` | none (a `.REVIEW.md` reading memo inherits the capture's tier) | UNADJUDICATED, at or under [ideas](ideas.md) |

The walt-side register is the larger channel since 2026-08-24: seven Pro parents (CE, L2, CBS, APS, MB, SC, FH) arrived by side channel, were adjudicated same-day by walt-math, and are indexed only on [walt-math-intakes](walt-math-intakes.md) — the courier ledger has no rows for them (§6.5).

## 2. The tier it produces, and TRUST-01

**Exchange-adjudicated CONFIRMED** (tier 3): the response's verification program executed green on its own claims here, unmodified, and no referee demonstrated a real flaw. It is not "Theorem — proved" in the corpus sense and not a kernel proof. In the ladder of [Home](Home.md): corpus statuses > proof-assistant kernel > **exchange-adjudicated CONFIRMED** > rob conformance receipts > (below all of these) everything exploratory.

What the tier means in practice:

- **A result at this tier can narrow an open problem but never closes a corpus row.** OPEN-11's interval is *corpus-proved* [26,46] and *exchange-adjudicated* [36,45]; both endpoints are always stated with their tier ([reachability](reachability.md), [open-problems](open-problems.md)).
- **TRUST-01 (v0.7): external `PASS` is never imported as an axiom.** A program that printed `ALL_PASS` here is evidence about the statement its program checks, at module granularity, under the caveats the referees recorded — nothing more. The Lean thread (§5.5) is the one place an exchange produced kernel artifacts, and those files prove their own statements, not C1.
- **rob receipts sit *below* this tier, not above it.** rob's `x-` prefixed receipt lines (S7–S10: `x-r_unr_002_*`, `x-r_tra_corpus_commutation`, `x-r_out_burnside`, `x-r_flo_*`) reproduce x:002, x:004, x:005 and the x:001 family totals in Rust — independent cross-language evidence, never a status change ([verification](verification.md)). REACH-18 (x:006, the floor's second family), REACH-19 (x:007, the ceiling) and REACH-20 (x:008, the no-void stratum) — the results the [36,45] box actually rests on — have **no rob backing yet**: rob reproduces x:001's family totals, not x:006's; 007 and 008 are the named rob slice-03 targets ([rob-slices](rob-slices.md)), and slice 03 has not begun (`rob/BRIEF_SLICE_02.md` §13 closes with "Do not begin slice 03").
- **The walt-tier registers produce no tier-3 results at all.** FT/SR/PANEL/TRIPLE rulings are real adjudications (repairs filed, claims confirmed or rejected) inside walt's exploratory fence; they can never become CONFIRMED by this pipeline and are cited by nothing above the fence.

## 3. The quota protocol

There is **no lifetime cap and no fixed total**. Dispatches are authorized by Jason **in batches, each batch's quota agreed up front** — monthly pacing, cleared per batch. Never submit without Jason's explicit go for the batch being sent in.

Two numbers, different kinds of thing:

| Number | Where | Meaning | Value at c00717d1 |
|---|---|---|---|
| the tally | `exchange/submission_count.txt` | running count of dispatches **ever sent** — incremented once per confirmed send (automated or hand-ferried); never a ceiling, never reset | **24** |
| `HARD_CAP` | `automation/submit.mjs` | the **current batch's ceiling for the automated path only**; `submit.mjs` refuses at `count >= HARD_CAP` | **17** |

`count >= HARD_CAP` means the automated path is closed pending a new authorized batch — never that the channel is spent. The count legitimately exceeds the cap because hand-ferried dispatches are counted but never pass through the automation; it has exceeded it since 2026-08-14.

History of the framing: the channel opened 2026-07-27 with a fixed budget of ten (commit 4b9f3344 "courier protocol, ledger, and submission budget"; the early commit messages say "count 5/10", "7/10"). Jason retired the fixed lifetime cap on 2026-08-01 as wrong framing and cleared up to eight dispatches for that day — `HARD_CAP` became 17 = 9 already sent + 8 (commit 392db685) and has not changed since. The docs were swept 2026-08-03 (10122fb1) and the batch protocol was made the only framing everywhere on 2026-08-24 (fb280840, PR #12; kanban card `exchange-quota-reframe`, closed). Residue: `automation/finish-001.mjs` keeps a dead `count >= 10` guard, comment-flagged; the `submit.mjs` header and the `pro-exchange` skill still quote "count 18 > cap 17" as their example (the skill self-declares any quoted number a stale snapshot).

Operational rules that came out of incidents (§8): a send counts only once visually confirmed in the UI; in the same breath as any send, re-read the tally and scan the target conversation for an identical turn; one operator with browser/submit authority at a time, even mid-handoff.

## 4. The foundation batch (001–008, 2026-07-27)

All eight dispatches were automated, all eight were adjudicated CONFIRMED the same day, and together they are one story about [reachability](reachability.md) — the exact cardinality of the strictly-Straight-reachable support image `R_Str^m` (OPEN-11) and its neighbours. Dispatches 001–005 were the five external questions Q1–Q5 of [FINDINGS](FINDINGS.md) §8; 006, 007 and 008 were same-day continuations into the 001 and 002 conversations.

### 4.1 The narrative

- **The floor moves 26 → 35 → 36 bits.** x:001 (REACH-17) built what its response calls a "rigorously disjoint family" of **17,668,066,045** reachable supports — more than 2³⁴ = 17,179,869,184 — split 559,316,142 no-void / 8,387,350,664 called-suit void / 8,721,399,239 natural-suit void. x:006 (REACH-18) added a structurally disjoint two-void-context family of **19,245,318,365**; combined floor **36,913,384,410 > 2³⁵** (margin 2,553,646,042), so the standalone interval became [36,46].
- **The ceiling moves 46 → 45 bits.** x:007 (REACH-19) computed the filtered tagged outer census exactly: **33,297,009,347,414 ∈ (2⁴⁴, 2⁴⁵)** — the first ceiling movement since the corpus's REACH-11 — giving **[36,45]**. A referee-proved fallback survives even without the temporal follower rule: the capacity-bound-only census 33,737,166,807,767 is still below 2⁴⁵.
- **The no-void stratum closes exactly.** x:008 (REACH-20): the no-void reachable slice is *saturated* — every pool of the right size under each of the 50 range-≤1 capacity profiles has a legal Straight realization with no hidden-seat void — so its census is exactly **624,892,870 = Σ over the 50 profiles of C(28, Σk)**. Corollary: x:001's "exactly 559,316,142" was the count of its regular-module grammar family, a proper subfamily (undercount 65,576,728 — D17 in [discrepancies](discrepancies.md)); the derived combined floor becomes 36,978,961,138 (labelled derived-from-REACH-20, not separately adjudicated); the interval stays [36,45]. Void-context strata remain open.
- **The outer language is not tight.** x:002: a witness — NT, capacities (6,6,6), V₁ = {6}, an 18-tile pool — passes all four outer necessary checks (capacity shape, schedule, lead witness, Hall) yet is unreachable under every declaration: 450 generators, 3 static matches, 425,520 traces, 0 realizers. A fifth necessary condition was named: the **follower-supply obstruction** (a realizing trick would need two distinct public six-followers while only `6:6` lies outside the pool). RESOLVED negative in [open-problems](open-problems.md).
- **OPEN-01 collapses.** x:003: the reduced viewer kernel K = (δ, H_m, N, τ, α_U) is *strictly finer* than the future-equivalence quotient for the support-aware P30 declaring-points contract — two reachable kernels differing only in fold ordinal (r = 7 vs r = 6; NT, viewer 0, bidder 3) are output-equivalent, by the dead-cut lemma. Product statistics 204 pairs / 22,848 alphabet checks / 1,604 legal edges / 1,280 diagonal closures ([reduced-viewer-kernel](reduced-viewer-kernel.md)).
- **Nine declaration tags fold to three.** x:004: the unscored pip-trump transport commutes with legal-prefix generation, `f_{t,u}(R_t) = R_u`, so the tagged reachable census is `7·r_pip + |R_DT| + |R_NT|` ([declaration-algebra](declaration-algebra.md)). The Step-15 quotient corollary was conditional on a one-line cocycle lemma `f_{u,v}∘f_{t,u} = f_{t,v}`; that gap was closed **in-house** the same day by `programs/004-cocycle.py` (Claude-authored, the only non-Pro program in the directory) over all 343 ordered pip-trump triples.
- **Every load-bearing integer is reproduced twice.** x:005: all 19 census integers of the corpus — N_det 8,102,258,940,222,814; N_bin 11,495,078,055,913,018,482; N_ter 1,830,955,704,129,296,418,354,864; total 1,830,967,207,309,611,271,596,161 (81 bits); outer totals 7,124,838,074,989 and 64,123,542,674,901; max C(k) 839,220,930,919; floor 44,352,165; the signature chain 136,514 / 23,842 / 1,667,666 / 114 / 296,721 / 21,686 / 2,121 / 35 / 279,048 / 103 — by two computation routes each, plus a new Burnside decomposition 136,514 / 2,156 / 35 ⇒ 23,842. Their status moved from single-source verifier receipts to *independently reproduced* ([verification](verification.md)).

### 4.2 The table

Recorded runtime is the adjudication run's; "re-run" is `timeout 60 python3 -B NNN.py` on a copy of the program, measured 2026-09-13 on this machine (008's recorded 71.8 s exceeds the 60 s budget and was not re-run). Line counts are `wc -l`. [verification](verification.md) owns the receipts and carries the PASS/FAIL line counts of this pass and the previous one.

| x | Question | FINAL ANSWER (verbatim) | Program | Recorded run | Re-run 2026-09-13 | Referees | Result |
|---|---|---|---|---|---|---|---|
| 001 | exact `|R_Str^m|` or a tighter interval than 26–46 bits | `INTERVAL [35,46] bits` | `001.py`, 1,789 lines | ALL_PASS 15.9 s | `PASS headline INTERVAL [35,46] bits`, 15.5 s | 3/3 SOUND | **REACH-17** |
| 002 | witness passing all four outer checks yet unreachable, or a sufficiency proof | `COUNTEREXAMPLE` | `002.py`, 965 lines | 16/16 PASS 0.9 s | `PASS overall counterexample_verified generators=450 traces=425520`, 0.91 s | 3/3 SOUND | **outer language NOT tight**; fifth condition |
| 003 | two future-equivalent distinct reduced kernels, or a Myhill–Nerode proof | `COLLAPSE` | `003.py`, 907 lines | ALL_PASS 0.43 s (8/8) | `PASS OPEN-01_COLLAPSE`, 0.38 s | 3/3 SOUND | **OPEN-01 RESOLVED (collapse)** |
| 004 | transport bijects reachable images (9 → 3), or a counterexample | `TRUE (bijection proved)` | `004.py`, 673 lines; `004-cocycle.py`, 146 lines (in-house) | ALL_PASS 4.6 s; cocycle ALL_PASS | `PASS ALL`, 4.28 s; `ALL_PASS 343 ordered triples`, 0.02 s | 3/3 SOUND | **transport theorem** |
| 005 | independent re-derivation of every census integer | `ALL REPRODUCED` | `005.py`, 699 lines | 19/19 PASS ~13 s | `PASS max_matrix_orbits_per_signature 103`, 12.9 s | 3/3 SOUND | **19 integers independently reproduced**; Burnside supplement |
| 006 | exact `|R|` or a slice, else an interval strictly inside [35,46] | `INTERVAL [36,46] bits` | `006.py`, 1,053 lines | 16/16 PASS 17.3 s | `PASS headline INTERVAL [36,46] bits`, 16.9 s | 3/3 SOUND | **REACH-18** |
| 007 | filtered outer census and a proved ceiling below 46 | `FILTERED_TAGGED_OUTER = 33297009347414` / `CEILING = 45 bits` / `INTERVAL [36,45] bits` | `007.py`, 975 lines | 17/17 PASS 44.1 s | `PASS overall FILTERED_TAGGED_OUTER=33297009347414 CEILING=45 INTERVAL=[36,45]`, 36.4 s | 3/3 SOUND | **REACH-19** |
| 008 | exact no-void slice with a two-sided completeness proof | `NO_VOID_SLICE = 624892870` | `008.py`, 1,217 lines | ALL_PASS 38/38, 71.8 s | not re-run (budget) | **2/3 SOUND-high + 1 UNVERIFIABLE-medium** | **REACH-20** |

### 4.3 The caveats, verbatim

These travel with the results; a citation of any REACH-17..20 number without its caveat is wrong. Source: [claim-ledger](claim-ledger.md) rows.

- **REACH-17 (x:001).** "Verification-tier caveat: reachability/disjointness of the counted family are prose trace-templates closed by referee adversarial replay, not end-to-end machine replay. Machine-hardened fallback tiers: ≥2³³ i.e. [34,46] without the four winning-void-trick rows; no-void family alone ≥30 bits." REACH-17 is now a component of the REACH-18 combined floor, not superseded. rob's S10 `verify_floor` reproduces the family totals (`x-r_flo_families`, `x-r_flo_total`) — conformance evidence.
- **REACH-18 (x:006).** "Same verification-tier caveat as REACH-17: 3,114 template representatives machine-replayed; within-class generalization and disjointness-from-001 close via prose argument + referee adversarial replay, not end-to-end machine replay of all ~19B members. Fallback tiers: the new family alone > 2³⁴ ⇒ ≥35 bits independent of 001; disjointness from the 001 no-void subfamily unconditional; … no single sub-block reaches ≥36 without the full family. Exact census and full declaration classes explicitly still open." Not reproduced by rob.
- **REACH-19 (x:007).** Filters: the licensed (6,6,6)-singleton fifth condition, an unconditional context-capacity supply bound (pure set arithmetic, Hall-independent), and the temporal follower rule (complete finite enumeration of trick-prefix cases); necessity stress-tested on ~986k machine-generated legal prefixes (116k in-program + 870k referee, fresh seeds), zero over-rejections. "Caveats: temporal-rule necessity is finite-enumeration + smoke-tested, not end-to-end machine-proved; 7× pip multiplicity licensed by the transport theorem (pip-0 and DT recomputed equal in-program, not all seven)." Not reproduced by rob (slice-03 target).
- **REACH-20 (x:008).** The non-unanimous panel: "2/3 SOUND-high + 1 UNVERIFIABLE-medium that found 'no computational error… nothing in the receipt is wrong' — dissent recorded here, not presented as 3/3." The heaviest verification-tier caveat of the batch: "coverage side machine-exact (per-phase meet-in-the-middle counts, covered+missing = C(28,|T|); all 5,430 exceptional pools realized and replayed end-to-end; 1,030 strided replays across all 50 profiles; fixed-hand j≤2 brute-force matches), but the stitching lemma (step 6) is machine-verified at module granularity (3,808 module-winner assignments + strong-triple checks), the j=1 block (64,422,540 pools) rests on the checked K₈-star pigeonhole + 63 strided samples, and the no-overcount direction rests on corpus-proved CELL-14 + Math §7.13.1." The proof-chain referee adversarially closed the stitching question and dissolved the §7.13.5 objection (REACH-10's witness is one-void, outside this slice). Not reproduced by rob (slice-03 target). The `QUICKSTART.md` trap list carries this dissent for a reason.
- **x:002.** Three independent referee re-verifications: a 1,276,560-trace single-layer enumeration, a 301,860-state recursive DFS with max-flow feasibility, and a corpus ID/integer cross-check; `witnesses/002.json` byte-identical to the inbox. rob's S8 `verify_unreachable` reproduces the witness and the follower-supply check (`x-r_unr_002_*`).
- **x:003.** A SHA provenance blemish (the response cited a verifier SHA that matches no retrievable artifact) is non-load-bearing: the inline program is the artifact of record and passes with the claimed statistics (D16). `programs/003.py` is also a reusable synchronized-product bisimulation checker, teeth-tested via forged perturbations. The x:003 witness is the required regression for rob slice 03.
- **x:004.** Mechanical scope is family certification per contract — a single auction shape, 224 pseudo-random traces; universality is carried by the prose induction, not the run ([verification](verification.md)). Four injected mutations (broken order preservation, unmapped exclusions, untransported deal, unmapped void contexts) were all caught. Artifact of record: inline `programs/004.py` (SHA-256 13420aa7…); the external sandbox SHA is a dead link (D16). rob's S9 `verify_transport` reproduces the commutation on 588 hands (16,464 transitions, 17,052 NF equalities).
- **x:005.** Referee-side foreign methods: a max-flow validator over all 343 triples reproducing the 136,514 criterion, an exact-rational EGF for N_det, a brute-forced ternary validity criterion over 16,712 structural cases (0 mismatches), the {1..7} lead-fiber multiset re-derived from the raw 28 dominoes for all 9 declarations. rob's S7 `verify_outer` reproduces the Burnside supplement (`x-r_out_burnside`).

## 5. The constellation batch (009–015, 2026-08-01)

Authorized by Jason on 2026-08-01 (up to eight dispatches that day; the fixed cap retired the same morning). The batch attacked the constellation direction of [idea-retrograde-rank](idea-retrograde-rank.md) — the declaration-blind relational key on the living tiles — and opened a Lean thread. It produced the first PARTIAL, the first refutation, the first honest refusal, and the first informal capture.

### 5.1 C1, the first PARTIAL (x:009)

**Claim.** Suffix minimax factors through the declaration-free constellation key at all depths with all nine declarations pooled — "C1". Response: `FINAL ANSWER: TRUE (C1 proved)`; program 8/8 ALL_PASS 16.3 s (re-run 2026-09-13: `PASS all-checks`, 14.0 s); panel **2/3 SOUND-high + 1 FLAWED-high**.

**Verdict: PARTIAL, split by sub-result.** (i) The C1 proof chain survived all three referees step by step — an adversarially step-checked proof at the external tier, *not* a kernel proof. (ii) The FLAWED referee's confirmed flaw was in the response's *corroboration artifacts*, not the proof chain: deliverables (c)/(d) were vacuous — the k ≥ 2 "cross-declaration" evidence was an undisclosed 2↔3 pip-transport monoculture (independent nontrivial k2 agreements = 5; zero DT/NT positions solved), and deliverable (b)'s class count was a non-invariant selection artifact.

**What must never be quoted from 009's program:** `classes=19329` (retired by x:010 — it is neither the ordered-opponent 31,197 nor the swap-pooled 15,680), `multi_groups=9495` / `cross_declaration_groups=9495` / `pairs=5000` as k ≥ 2 evidence. Genuine anchors: 2,211,300 k = 1 positions / 14 outcomes match the corpus; the corpus's 15,680 was reproduced exactly by two referees under opponent-swap pooling. k ≥ 2 cross-declaration evidence is owned by rob's `constellation_k2_probe.rs` (817,896 checks, zero divergences), not by this response.

### 5.2 The counterweight: backward commutation REFUTED (x:009, secondary)

The same response refuted backward commutation for the pooled key with a zeros-trump/doubles-trump witness: predecessor trick 2:1, 2:2, 3:1, 3:0; exhaustive exclusion `fixed_partial_maps=4 full_embeddings=0 legal_embeddings=0`; two referee brute-force routes disjoint from the response's backtracker (5,953,536 and 372,096 enumerations) both returned 0. This sub-result is **exchange-adjudicated CONFIRMED**. Scope: the embeddability/feasibility sense only — **feasible ≠ reachable**, no REACH-* impact. Consequence on the idea page: a retrograde walk must either keep the declaration in its backward key or compute predecessor sets per realization, never per pooled representative.

### 5.3 R1: realizable = reachable at k = 1 (x:010)

Every realizable last-trick constellation class is legal-play reachable via a forward-replayed full-hand witness; the realizable-but-unreachable gap is zero, so reachability filtering of the k = 1 retrograde seed table is a no-op at class granularity. `FINAL ANSWER: R1 TRUE`; 31,830 PASS / 0 FAIL ~19 s (re-run 2026-09-13: `PASS R1 every realizable class has a replayed legal full-hand witness`, 17.5 s); 3/3 SOUND-high; all 31,197 witnesses independently re-replayed through the corpus ingest verifier by a referee, 0 failures. **CONFIRMED.**

**The convention note.** 31,197 is the dispatch-literal ordered-opponent count; rob's frozen number is the swap-pooled 15,680, of which 163 classes are reflection-fixed — **31,197 = 2·15,680 − 163**. Convert before diffing against `constellation_k1_census.rs`. Scope caveats verbatim: REACHABLE here means legal-play reachable (follow obligations + winner-leads; no contract/bid consistency) — a different predicate from the reachable-support image, no REACH-* impact; outcome-constancy is quotable only from the adjudication re-run (all 4,422,600 oriented positions, 0 collisions, 14 outcomes) or rob's `fine_collisions == 0`, never from the response's tautological per-class receipt; 0 NT and 0 δ = 3 witnesses in the exhaustive loop (NT realizes 19,069 of 31,197 classes), so per-declaration reachability rests on the 600-case sample over all 216 = 9 × 24 declaration × hold cells.

### 5.4 The carrier staircase (x:012)

Exact S₇ orbit counts of j-edge subgraphs of K₇-with-loops: **a₄ = 37, b₄ = 486, b₈ = 126,657**, role-decorated count-labelled 4-carriers **4,767**; the full pure row a₀..a₂₈ is palindromic with Σa = 79,264 (a = 1, 2, 5, 14, 37, 98, 252, 585, 1239, 2396, 4135, 6340, 8630, 10381, 11034, …); the count-labelled row is not palindromic (b₁ = 5 vs b₂₇ = 22) with Σb = 47,940,826. 14/14 PASS 18.95 s (re-run 2026-09-13: `a4=37 b4=486 b8=126657`, 17.4 s); 3/3 SOUND-high; b₈ triply confirmed by referee-independent routes including a C program (`witnesses/012/ref012.c`); the corpus-frozen 486 / 4,767 match rob's instrument exactly. **CONFIRMED.**

Caveats verbatim: b is computed by a fibered stabilizer-Burnside, a disclosed substitute for the displayed but infeasible conjugacy-class cycle index (correctness proved by the response's Step-4 theorem and independently reproved); the `ROLE_LOCAL_OK` conjunct is a literal `True`; two PASS lines are tautological; the response's own direct anchors are only j ∈ {0..5, 27, 28} (mid-layers closed by referee reruns); the OEIS attribution is unverified (403). **The counts are the rule-free carrier skeleton — feasibility, not reachability** — a strictly poorer object than the standings-bearing carrier of the idea page; the vocabulary split is recorded there.

### 5.5 The Lean thread (011 → 013 → 015): an honest refusal, then two green stages

Dispatch 011 asked for a single lake-buildable Lean 4 file (mathlib v4.33.0-rc1) formalizing suffix positions, minimax value, constellations and C1, with `unique_winner`, the k = 1 base case and hereditariness sorry-free. After 6 h 48 min (past the 3 h harvest window, "still generating, not resubmitted" — commit 9d997c95) Pro replied with 2,217 bytes (`inbox/011`):

> I can't honestly produce what you've requested. … I cannot truthfully certify compilation. … Because your evaluator awards **zero** for a non-compiling artifact or an inaccurate compilation claim, I won't fabricate either.

The contract worked as designed. The reply also **caught a specification error in the dispatch**: the foundation deliberately defines tier-zero trick keys as tied and proves a unique *maximum winner*, not global key injectivity — so the mandatory theorem had to be unique winner, not key injectivity. The correction was accepted and the staged build Pro proposed was adopted (Jason's 2026-08-01 loose-but-directed iteration policy: Pro writes, the project lake-builds under the pin and returns the log; no panel).

- **013, Stage 1 GREEN** (submitted 14:28Z into the 011 conversation, harvested 15:10Z): a 278-line core; `lake build` clean under the pin after two mechanical local fixes (`set_option`-in chaining → section scope); `unique_winner` kernel-checked with zero sorries, including a 56,448-case `decide` and two worked tricks. Landed as `lean/Texas42/ConstellationCore.lean` (commit 1d36ddb8).
- **015, Stage 2 GREEN after local repair** (submitted 15:15Z, harvested 15:58Z): suffix positions, mid-trick legality, a fuel-indexed exact minimax over the 4k remaining plays, `value_k1_forced`; the kernel *evaluates* minimax in two k = 1 examples (−11 and 16). The local fixes were made on the project side, not by Pro: the reserved-keyword rename `prefix → pending`, a restructured `step_remaining` proof, a `LinearOrder Domino` lift, a `SuffixPos` namespace. Landed as `lean/Texas42/ConstellationSuffix.lean` (commit f0de9333).

What the two files are and are not ([lean](lean.md)): sorry-free kernel artifacts, part of the default build (so the axiom-hygiene claims cover them), but **self-contained** (they re-derive their own domino and declaration algebra rather than importing `Basic.lean`/`Trick.lean`), **unreconciled**, not on the priority-0 scoreboard, and carrying **no claim-ledger row**. They differ methodologically from the main spine one level below the headline: the constellation core discharges key injectivity by `decide` over every declaration/context/tile pair where `Trick.lean` proves it by the shared-pip argument. **C1 itself is not stated in Lean** — `grep` finds no factorization statement in either file — so the claim-ledger's "Lean mechanization pending (dispatch 011)" on the x:009 row should read: the core and the minimax are mechanized, C1 is not, and no dispatch is queued for it.

### 5.6 The informal take (x:014) and the informal capture of 2026-08-03

014 was sent in the loose-but-directed mode with no deliverable contract ("its take, its improvisation"). The 26,286-byte reply — "you found the intrinsic geometry of the game": constellation as intrinsic relational type, realization as embedding, backward failure as non-surjective restriction of embedding spaces, salience as a filtration, beliefs as measures on the realization bundle, trump's monotone disappearance law — is **UNADJUDICATED, an exploratory capture only**, numbered because it went through the courier. Two days later the `informal/` convention was created for threads that never had a number: `exchange/informal/2026-08-03-domino-constellations-theory.md` (a 5,180-line ChatGPT share thread) with a single-pass `.REVIEW.md` memo that records 32 dangling citation markers unresolvable from the capture and whose every "re-derivation" is one arithmetic re-run, not a receipt. Both sit at or under [ideas](ideas.md); nothing in them is quotable as a result.

## 6. Pro as colleague — the walt threads (016–024) and the side channel

**Fence, stated once for the whole section:** every statement below is at walt's EXPLORATORY tier — below every tier of [Home](Home.md), quotable by nothing above the fence, and not promoted by having been adjudicated. The adjudications here are walt-math's same-day intakes, filed as A-ruling families in `walt/CENSUS-RULINGS.md`; none went through `workflow.js`, and none can become exchange-adjudicated CONFIRMED. The objective throughout is pmake (ruled 2026-08-17); "level 2" is a best response to a named σ1, never equilibrium.

### 6.1 The register change (2026-08-14)

On 2026-08-14 Jason cleared dispatch 016 with "ship it please" and ferried it himself — the courier automation watches the main checkout, not the worktree the walt program lived in — pasting the outbox body into the app and uploading Pro's reply. The dispatch was written in the x:014 colleague register (no deliverable contract; "formal adversarial dispatches to follow once the direction firms up"), and every dispatch since has been hand-ferried. Each reply was harvested with a `<!-- HARVEST METADATA -->` header marked `status: UNADJUDICATED` and the caveat that the note's self-classification labels "are Pro's, not ours, until confirmed"; walt-math then adjudicated the same day.

### 6.2 First-rung nonanticipativity taxes (x:016 → FT-A1..A29)

Pro's note "Decision-Sparse Exact Solving: Nonanticipativity Taxes and a Compositional Plan Calculus v0.1" (`inbox/016`, 44,099 bytes) answered the cheap-upper-witness handoff. walt-math confirmed its first-layer mathematics — the fusion-gap identity, the binary tax formula, fusion cores ≤ |A(I)|, the one-stage penalty dual — and filed four repairs: Lemma FT-arrive names a silent hypothesis, Lemma FT-trunc shortens the ladder by one rung, Proposition FT-flat replaces §10.2 (an action-blind upper feature can never shave), Lemma FT-post defuses the uniform-posterior trap. Its Experiment 15.1 was built and run the same day as the S6k `fusion_tax` probe: first gluing-cut closure at h6, eleven of twelve pairs *not* closed with tied shortfall = Δ² exactly — a probe record (`walt/probes/factory-results/fusion_tax_2026-08-14.txt`), quotable only inside the fence. Owner: [walt-decision-sparse](walt-decision-sparse.md).

### 6.3 Second-rung gluing (x:017 → SR-A1..A37)

Pro's note "Second-Rung Gluing: Policy-Dependent Occupancies, the Slack–Tax Interchange Law, and Exact Martingale Penalties v0.1" (`inbox/017`) was ACCEPTED IN LARGE PART: the slack–tax interchange law Δ² = Σ_I min_b(s + d) CONFIRMED; the multistage martingale dual CONFIRMED (discharging the validity half of FT-A13(iv)). Repairs: an unnamed free-product hypothesis (Lemma SR-coord); a wrong justification on §6.1 (Proposition SR-sep); the §12.1 verifier proved vacuous (Proposition SR-taut — REJECTED as a receipt); §1.4's silent weakening of Lemma FT-post declined. The SR depth-two probe ran the same day (`second_rung_2026-08-14.txt`): ten receipts HELD at all four units (h2, h9), Δ² reconstructed exactly at both, escape actions PRESENT (36/330 at h2, 498/1320 at h9 — the first measured policy adjustment). The ruling range is **SR-A1..A37** — `walt/CENSUS-RULINGS.md` contains SR-A37 (three corrections to the carried-obligation list); `exchange/README.md` said A36 until this rewrite, and [claim-ledger](claim-ledger.md) still does.

### 6.4 The unanswered letter (x:018)

Dispatch 018 (`outbox/018-fee-correlation-update.md`, 19,260 bytes) is correspondence, not an adversarial problem set: it reports the FT/SR/FF/FC arc back to Pro (the h6 gluing-cut closure; the exact convex breakpoint solve; the shared-θ figure 7,095,382,833/7,104,861,535 of oracle; the exact-zero coordinate; Proposition FC-width) and asks what object carries the lower-witness burden when the fee route is structurally unavailable — a covering / fractional-covering dual over the core hypergraph with the fee as the rank-one case — naming the conditional-moment gap blocking trick 1 ([walt-math-open-questions](walt-math-open-questions.md) item 11). **No reply is in the inbox as of 2026-09-07.** Whether it was ever delivered is not established from the repository record: the tally shows two increments for the three hand-ferried dispatches 016–018 (§8.4), and the ledger row says "staged … awaiting Pro's reply". Only Jason can settle it.

### 6.5 The adversary panel on calculated evidence and level 2 (x:019–023 → PANEL-A1..A8)

The register returned to adversarial on 2026-08-24, when Jason hand-ferried five self-contained briefs as one authorized batch (drafted earlier the same day in `exchange/drafts/`, see `exchange/drafts/README.md`): 019 the CE-T1/T2/T3 evidence process, 020 the CE-T4/T5 bounded mean, 021 the O21/O24 risk ledger and exact escalation, 022 the O26 execution-order invariance, 023 the L2-T1..T5 coupling theorems. They attack the two side-channel parents — calculated evidence (CE-A1..A8) and targeted level-2 field stability (L2-A1..A7) — that never had courier rows. One consolidated response came back the same day (`inbox/019-023-response-panel-and-cancellation-v0.1.md`, SHA-256 `a3f468aa…`, pinned; re-verified 2026-09-13) with an exact-rational companion `verify_walt_panel_response_v0_1.py` (36/36, `ALL CHECKS PASS` in 0.74 s on 2026-09-13 from a copy — session evidence, scratch tier, never a receipt).

Verdicts as the panel wrote them: CE-T1/T2/T3 CERTIFIED; CE-T4/T5 CERTIFIED with one sentence narrowed (the unrestricted bounded-rational class is not sign-safe, but it contains sign-safe subclasses); O21/O24 MIXED; O26 UNDER-SPECIFIED, REPAIRABLE; L2-T1..T5 CERTIFIED with the coupling definition repaired. "Certified" is the panel's word for its own step-check; it promotes nothing (PANEL-A1). Three concrete corrections:

1. **Claim D of the risk ledger is false as written.** If a newly opened edge may reuse retrospectively selected historical evidence, three e-values each with mass 1/8 at the value 8 give false-cross probability 1 − (7/8)³ = **169/512 > 1/4 = δ_dec**. Binding repair (PANEL-A3): edge risk is never assigned retrospectively — future-only opening or preallocation before any evidence enters.
2. **W1–W6 under-specify liveness inside an overshot batch.** The repaired semantics W7–W11 (predictable activation, canonical per-index liveness replay, speculative isolation, deterministic same-index crossing with a typed inconsistency, complete pause state) make the execution-order invariance theorem true; the conditional-null question resolves positively under predictable activation and i.i.d. future worlds (PANEL-A5).
3. **The committed field lift was wrong by arithmetic.** For `receipt-h8-t4`: reveal c⁺ = 30/1200, c⁻ = 26/1200 (net 4/1200); retain c⁺ = 45/1200, c⁻ = 72/1200 (net −27/1200); Λ = 4/1200 − (−27/1200) = **31/1200**, where the prose said 41/1200. Re-verified from the 2,400 raw records and corrected at both committed sites (PANEL-A8); the component counts were always right.

Also adopted: the unsolicited Part VI — the strict hierarchy |net value correction| ≤ terminal outcome-change mass ≤ field-exposure mass ("small net correction is not the same as irrelevance"), pairwise (B, H, q, g) masses, a dominance theorem (H = 0 ∧ B > 0 is strict dominance, never cancellation), directional R± bounds — for slice 3 (PANEL-A7/A8). The two believed-by-construction conformance claims were carded and audited the next day: `walt/audits/panel_response_conformance.md` and eight gates in `walt/walt/tests/solver_panel_conformance.rs`, all CONFORMS, with one recorded judgement call (W10's typed `InconsistentEvidence` is not implemented; its trigger is structurally unreachable in the shipped common-stream design). Thread labels per Jason's framing: CE = depth of looking, L2 = choice of model.

### 6.6 Three deferred producers, built the same night (x:024 → TRIPLE-A1..A7)

Dispatch 024 (drafted and hand-ferried 2026-08-24; the count reached 24) stated the project's own candidate solutions to three slice-3 deferrals for attack. Pro's response "Three Deferred Producers" (`inbox/024-…`, SHA-256 `337296a7…`, pinned; re-verified 2026-09-13) with `verify_deferred_producers_triple_v0_1.py` (13/13, `ALL CHECKS PASS` in 6.6 s on 2026-09-13 from a copy; scratch tier) was hand-delivered 2026-08-25 and adjudicated the same day:

- **Part 1, CONSTRUCTION** — a class-size-free anytime upper confidence sequence for a finite maximum of means: one fixed true maximizer suffices, so no |Π_a| Bonferroni split (Theorem M1); endpoint monotonicity collapses the family to the single empirical-optimum count S*_n, which the shipped split-reach count already computes (Corollary M2). Exhaustive sweep of all 256 two-policy Boolean tables × 256 length-4 streams (65,536 evaluations), worst finite-horizon undercoverage 11/128 < δ = 1/4; in the worked example at δ = 1/4 the e-process value E^>_{2,2}(1/4) = 1/3 + 1/2 + 3/10 = 17/15 < 4 = 1/δ, so the grid point 3/4 is not rejected, and the standing specimen is R = 1/2 ≤ E3 = 3/4 < E2 = 1 (TRIPLE-A2). The dispatch's branch-mixture upper route was retired (wrong orientation).
- **Part 2, CONDITION** — there is no canonical weakest *local* exchange predicate; the **Hazard-Exclusion Invariant** (initial coverage, forward closure, terminal safety) is sound (H1) and semantically complete (H2) and becomes the single dominance-bound authority; a one-round trump-extraction witness ships as the first incomplete producer with an explicit non-coverage specimen.
- **Part 3, ALPHABET** — six mutually exclusive first-split motifs (LeadContextFork, ImmediateControlFork, CountCommitmentFork, TrumpCommitmentFork, SuitShapeFork, StrengthCommitmentFork) plus Other; `RevealResponse` refused as undecidable pending suffix enrichment.

All three producers were built with gates the same night (slices 4a/4b/4c = PRs #45/#46/#44, all merged 2026-08-25, the last of them `cbce1ae3`): `walt/walt/tests/solver_e3_upper.rs`, `solver_hazard_witness.rs`, `solver_fieldswap_motifs.rs`, with instrument records at `walt/probes/hazard_witness/` and `walt/probes/fieldswap_motifs/`. Thread label [L2] throughout; Part 1 consumes the CE evidence engine through the sanctioned one-directional crossing (L2 consumes CE, never the reverse). A later side-channel note (CBS) recognized its Theorem 5.1 as x:024's M1/M2 over pmake.

### 6.7 The side channel: seven parents the courier ledger never saw

Since 2026-08-24 the bulk of Pro's walt contribution has arrived outside the numbered exchange: Jason hand-delivers a note, it is frozen verbatim under `walt/math/` with a SHA-256 pin, walt-math adjudicates it the same day, and the rulings are appended to `walt/CENSUS-RULINGS.md`. "Not an automation dispatch — the courier ledger is untouched" (walt-math-intakes §6). They are indexed **only** on [walt-math-intakes](walt-math-intakes.md); a reader of `exchange/README.md` alone cannot discover them, which is why the README now points there.

| Parent (verbatim, `walt/math/`) | Rulings | Date | One line |
|---|---|---|---|
| `calculated_evidence_v0.1.md` | CE-A1..A8 | 2026-08-24 | anytime-valid adaptive settlement; the §22 build program |
| `targeted_level2_field_stability_v0.1.md` | L2-A1..A7 | 2026-08-24 | first-disagreement localization L2-T1..T5; exposure bounds |
| `counted_belief_sandwich_v0.1.md` (the filename is the parent's; the objects are root interval and survivor set — CBS-A3) | CBS-A1..A9 | 2026-08-30 | root intervals and survivor sets over policy regions and factorized belief |
| `anytime_proof_state_score_v0.1.md` | APS-A1..A9 | 2026-08-31 | append-only proof state; certified regret Γ = U* − B_exec; Phases 0–8 |
| `model_belief_base_player_v0.1.md` | MB-A1..A8 | 2026-09-01 | the field model as hidden state Ξ = Ω × Θ |
| `salvation_complex_v0.1.md` | SC-A1..A8 | 2026-09-01 | 1 − Q as a minimum belief-mass transversal; doom = singleton cuts |
| `focal_horizon_sandwich_v0.1.md` (parent title; the object is the focal-horizon hierarchy — FH-A2) | FH-A1..A11 | 2026-09-04 | one refinement hierarchy indexed by focal decisions, L_k ≤ Q ≤ U_k |

Every ruling range above was checked against `walt/CENSUS-RULINGS.md` on 2026-09-13 (the highest identifier present in each family matches). After FH: `walt/briefs/FH-RESPONSE-TO-PRO.md` (2026-09-04) is a draft letter for Jason's hand-ferry — "not a courier dispatch — no number, the exchange ledger is untouched" — and the two Pro packet notes of 2026-09-05 (`TEXAS42-UNIFIED-REVIEW-v0.1`, `TEXAS42-IMPROVISATION-v0.1`) live under `experiments/partnership/packet/` and belong to [walt-partnership-program](walt-partnership-program.md). The eras these parents opened are [walt-calculated-evidence](walt-calculated-evidence.md), [walt-counted-belief-era](walt-counted-belief-era.md) and [walt-focal-horizon-era](walt-focal-horizon-era.md).

## 7. How to read an exchange result

1. **Cite by number: `x:NNN`.** The number resolves to `exchange/README.md`'s row, the inbox file, and (for 001–012) `programs/NNN.py`. Results 016–024 resolve to A-ruling families in `walt/CENSUS-RULINGS.md`, not to the CONFIRMED pipeline.
2. **Carry the caveat block verbatim.** Every CONFIRMED row in [claim-ledger](claim-ledger.md) has one; §4.3 and §5 reproduce them. A number without its caveat is a different, stronger claim than the one adjudicated.
3. **Keep the two tiers' endpoints distinct.** OPEN-11 is corpus-proved [26,46] and exchange-adjudicated [36,45]. Never write "[36,45]" without the tier, and never write "proved" for it.
4. **Distinguish referee-replayed from end-to-end machine-replayed.** REACH-17/18/20 are counts whose membership is closed by referee adversarial replay of templates, modules or samples — "machine-exact at module granularity" — not by replaying every member. REACH-19's referee-proved fallback census is the case where a number survives discarding an entire apparatus (the temporal rule); say so when citing it.
5. **Never present REACH-20 as 3/3.** It is 2/3 SOUND + 1 UNVERIFIABLE that found no defect. The same goes for x:009's 2/3 + 1 FLAWED (the flaw outside the chain).
6. **A green receipt is evidence, never a status change.** A re-run today (§4.2) confirms the program still prints what it printed; it does not move a result up a tier. rob's `x-` lines are the same kind of thing one tier lower.
7. **Walt-tier intake ≠ CONFIRMED.** FT/SR/PANEL/TRIPLE (and the side-channel CE/L2/CBS/APS/MB/SC/FH) are exploratory adjudications; a walt probe number (h6 closure, 11/12, 36/330, 31/1200) is quotable only through the gate or record path that pins it — say which, or label it "probe record, not gate-pinned".
8. **Feasible ≠ reachable; realizable ≠ reachable-support.** x:009's refutation and x:012's staircase are about embeddability of the rule-free skeleton; x:010's R1 is about legal-play reachability of last-trick classes. None touches REACH-*.
9. **Informal is informal.** x:014 and `exchange/informal/` are captures, not results, whatever their numbers or their reading memos say.
10. **Results files outrank prose.** Where a ledger sentence and a program's PASS line or a rulings file disagree (SR-A36 vs SR-A37; 41/1200 vs 31/1200), the receipt wins and the disagreement is recorded.

## 8. Incidents and costs

### 8.1 The 006 double-send (2026-07-27)

Dispatch 006 was sent twice, about 28 seconds apart, by two operator agents during an ownership handoff race: the outgoing operator submitted before its stand-down arrived, and the incoming operator sent without re-reading the count or re-scanning the target conversation. Both turns were verified in the 001 conversation DOM; Pro answered once (the duplicate is content-harmless), but one dispatch unit was spent for nothing — the tally was corrected 5 → 7 → 6 → 7 across three commits (b3b57e51, d41e8193, 783aff66) and the duplicate is counted as sent. Rules adopted: the **pre-send guard** (in the same breath as any send, re-read `submission_count.txt` and scan the conversation for an identical turn; abort on either signal — a coordinator's stand-down order is not a substitute, because orders and sends cross in flight) and the **single-operator hard rule** (never two agents with browser/submit authority alive at once, even mid-handoff). `submit.mjs` was made self-guarding the same afternoon (68fa85a6): the `.submitted.json` marker is created with an exclusive `wx` write before the composer send, and the count is incremented exactly once after a confirmed server-uuid send.

### 8.2 Harvest wedges and false negatives

- The original poll-then-harvest pair could die silently; after the 006 postmortem `watch-harvest.mjs` (8ad216ef) became a single-tab watcher that detects completion in the live DOM and harvests in the same process, writing an `inbox/NNN-<slug>.FAILED.md` marker on failure.
- 010's watcher was killed by SIGTERM at 08:31:33Z on 2026-08-01; the response was harvested manually at 08:43:32Z. The FAILED marker still sits in `inbox/` although its own text says to delete it after a manual harvest — housekeeping residue.
- The post-send verifier returned a false-negative `bodyOk` on 008 and again on 015; in both cases the turn was confirmed present exactly once by a DOM check before the send was counted (the "008 precedent").
- 011 ran past the 3 h harvest window ("still generating, not resubmitted") and was harvested at 6 h 48 min.
- Harvests made in a worktree stranded artifacts until they were landed in the main checkout (0f83478b, which also set the tally to 16); the same checkout mismatch is why 016 onward were hand-ferried.

### 8.3 Latency

Submit-to-harvest intervals from the inbox metadata headers (the harvest time bounds the generation time from above; 001–005 were harvested in one sweep at ~13:02Z, so their true generation latency is not recorded):

| x | 001 | 002 | 003 | 004 | 005 | 006 | 007 | 008 | 009 | 010 | 011 | 012 | 013 | 014 | 015 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| interval | 7h55 | 7h52 | 7h53 | 7h53 | 7h53 | 2h47 | 0h46 | 1h46 | 0h44 | 1h05* | 6h48 | 0h55 | 0h42 | 0h31 | 0h42 |

\* manual harvest after the SIGTERM. Hand-ferried replies (016–024) carry a date, not a timestamp; 019–023 and 024's response were returned the same day or the next. The README's planning figure is "expect 1–2 h, timeout 3 h"; the observed spread is 31 min to about 7 h. This is the reason Pro is not a tool for iterating quickly, and part of why the 2026-08-01 iteration policy — refine in-conversation, convene no adversary panel until a result is finalizable ([lean](lean.md)) — and later the hand-ferried colleague register displaced the automated loop.

### 8.4 The cost of adjudication

The foundation batch's first five responses took a thirty-agent workflow (`wf_775fe0ec`) — per response one extractor, one executor, three referees at default effort and one xhigh verdict — then 006, 007 and 008 the same day (per-response panels whose run statistics and referee tallies are in the ledger rows), then two rounds on 2026-08-01. Program runtimes were small (0.4 s to 71.8 s); the referees' independent re-verifications were the expensive part (a 1,276,560-trace enumeration for x:002; 870k referee-generated prefixes for x:007; all 31,197 witnesses re-replayed through the ingest verifier for x:010; a C program for b₈ in x:012). The walt-tier intakes cost one walt-math session each, same day.

### 8.5 The tally arithmetic

`submission_count.txt` reads 24 and there are 24 dispatch numbers, but the two agree **by cancellation**. From `git log -p`:

| Commit | Δ | Event |
|---|---|---|
| 8ea2ff24 | 0 → 1 | 001 sent |
| 8d3651ef | 1 → 5 | 002–005 sent |
| b3b57e51 / d41e8193 / 783aff66 | 5 → 7 → 6 → 7 | 006 sent, double-sent, corrected: the duplicate counts |
| 0727f9db | 7 → 9 | 007, 008 sent |
| 0f83478b | 9 → 16 | 009–015 landed from the worktree (seven dispatches) |
| 0f4acf55 | 16 → 17 | 017 staged (016's clearing commit 6dcd7fdf did not bump) |
| 3394ade0 | 17 → 18 | 018 staged |
| 43437fe3 | 18 → 23 | 019–023 hand-ferried |
| d8a2d70e | 23 → 24 | 024 hand-ferried |

Sixteen automated sends (fifteen dispatches plus the 006 duplicate), then two increments for the three hand-ferried dispatches 016–018, then six more. "Dispatches ever sent" is therefore **25 if 018 reached Pro and 24 if it did not**; either way one entry among 016–018 is uncounted or unsent, and the record does not say which. Settling it is Jason's call (§6.4).

## 9. The complete dispatch table, 001–024

Conversation identifiers are the leading eight characters of the `chatgpt.com/c/…` URL in `exchange/README.md`. "Tier entered" is the highest tier the result reached; walt-tier rows are EXPLORATORY by definition.

| # | Date (UTC) | Topic | Ferry | Adjudication — status, location | Tier entered | Claim IDs / rulings |
|---|---|---|---|---|---|---|
| 001 | 2026-07-27 05:07 | reachable-support cardinality | automated, new chat 6a66e786 | CONFIRMED (ALL_PASS 15.9 s; 3/3 SOUND) — `workflow.js` wf_775fe0ec; claim-ledger | exchange-adjudicated | REACH-17 |
| 002 | 2026-07-27 05:09 | outer-language tightness | automated, new chat 6a66e7f0 | CONFIRMED (16/16; 3/3) — claim-ledger; open-problems | exchange-adjudicated | outer language NOT tight; fifth condition |
| 003 | 2026-07-27 05:09 | kernel vs future quotient | automated, new chat 6a66e808 | CONFIRMED (ALL_PASS; 3/3) — claim-ledger; open-problems; D16 | exchange-adjudicated | OPEN-01 RESOLVED (collapse) |
| 004 | 2026-07-27 05:09 | transport–reachability commutation | automated, new chat 6a66e821 | CONFIRMED (ALL_PASS; 3/3; cocycle closed in-house) — claim-ledger | exchange-adjudicated | transport theorem `f_{t,u}(R_t) = R_u` |
| 005 | 2026-07-27 05:10 | census-integer audit | automated, new chat 6a66e834 | CONFIRMED (19/19; 3/3) — claim-ledger; verification | exchange-adjudicated | 19 integers independently reproduced; Burnside 136,514/2,156/35 |
| 006 | 2026-07-27 14:53 | exact reachable census (continuation of 001) | automated, continuation 6a66e786; **double-sent** | CONFIRMED (16/16; 3/3) — claim-ledger | exchange-adjudicated | REACH-18 |
| 007 | 2026-07-27 18:15 | fifth-condition ceiling (continuation of 002) | automated, continuation 6a66e7f0 | CONFIRMED (17/17; 3/3) — claim-ledger | exchange-adjudicated | REACH-19 |
| 008 | 2026-07-27 18:15 | no-void exact census (continuation of 001/006) | automated, continuation 6a66e786 | CONFIRMED (38/38; **2/3 SOUND + 1 UNVERIFIABLE-no-defect**) — claim-ledger; D17 | exchange-adjudicated | REACH-20 |
| 009 | 2026-08-01 07:33 | constellation suffix factorization (C1) | automated, new chat 6a6da164 | **PARTIAL** (8/8; 2/3 SOUND + 1 FLAWED in corroboration artifacts) — claim-ledger; FINDINGS §4 | C1 proof: external step-checked; backward refutation: exchange-adjudicated CONFIRMED | C1; backward commutation REFUTED |
| 010 | 2026-08-01 07:38 | constellation realizability = reachability (R1) | automated, new chat 6a6da260 | CONFIRMED (31,830/0; 3/3) — claim-ledger | exchange-adjudicated | R1; 31,197 = 2·15,680 − 163 |
| 011 | 2026-08-01 07:38 | constellation Lean formalization | automated, new chat 6a6da27e | honest refusal + spec correction (unique winner, not key injectivity); no panel — README row | process record (no result) | — |
| 012 | 2026-08-01 07:39 | carrier staircase (Burnside) | automated, new chat 6a6da294 | CONFIRMED (14/14; 3/3) — claim-ledger | exchange-adjudicated | a₄ = 37, b₄ = 486, b₈ = 126,657, 4,767 |
| 013 | 2026-08-01 14:28 | Lean Stage 1 (continuation of 011) | automated, continuation 6a6da27e | STAGE 1 GREEN; no panel — README; lean.md | Lean kernel for its own statements; self-contained, unreconciled, no ledger row | `ConstellationCore.lean` (1d36ddb8) |
| 014 | 2026-08-01 14:49 | constellation informal take | automated, new chat 6a6e077a | INFORMAL, unadjudicated — claim-ledger informal table | UNADJUDICATED capture | — |
| 015 | 2026-08-01 15:15 | Lean Stage 2 (continuation of 011/013) | automated, continuation 6a6da27e | STAGE 2 GREEN after local repair; no panel — README; lean.md | Lean kernel for its own statements; C1 not stated | `ConstellationSuffix.lean` (f0de9333) |
| 016 | 2026-08-14 | cheap-upper-witness handoff (colleague register) | hand-ferried by Jason | walt-math same day — `walt/CENSUS-RULINGS.md` | EXPLORATORY | FT-A1..A29 |
| 017 | 2026-08-14 | second-rung gluing handoff | hand-ferried by Jason | walt-math same day, ACCEPTED IN LARGE PART — `walt/CENSUS-RULINGS.md` | EXPLORATORY | SR-A1..A37 |
| 018 | 2026-08-14 | fee-correlation update (correspondence) | hand-ferried by Jason (delivery not established from the record) | no reply as of 2026-09-07 — README; walt-math-open-questions item 11 | none | — |
| 019 | 2026-08-24 | panel: CE evidence process | hand-ferried batch of five | one consolidated response, walt-math same day — PANEL-A1..A8; intake `walt/math/response_walt_panel_and_cancellation_v0.1_intake.md` | EXPLORATORY | PANEL-A1, A2 |
| 020 | 2026-08-24 | panel: CE bounded mean | hand-ferried batch of five | as above | EXPLORATORY | PANEL-A1, A2 |
| 021 | 2026-08-24 | panel: risk ledger + escalation | hand-ferried batch of five | as above — Claim D counterexampled 169/512 | EXPLORATORY | PANEL-A3, A4 |
| 022 | 2026-08-24 | panel: execution order | hand-ferried batch of five | as above — W7–W11 adopted | EXPLORATORY | PANEL-A5 |
| 023 | 2026-08-24 | panel: L2 coupling theorems | hand-ferried batch of five | as above — τ coupling repair; Part VI adopted; 41/1200 → 31/1200 | EXPLORATORY | PANEL-A6, A7, A8 |
| 024 | 2026-08-24 | deferred-producers triple | hand-ferried single dispatch; response hand-delivered 2026-08-25 | walt-math 2026-08-25 — TRIPLE-A1..A7; intake `walt/math/response_deferred_producers_triple_v0.1_intake.md`; producers built same night (PRs #44/#45/#46) | EXPLORATORY | TRIPLE-A1..A7 |

## 10. Ledger-drift errata

What this rewrite cleared (files owned by this chapter):

| Item | Was | Now |
|---|---|---|
| `exchange/README.md` row 017 | "SR-A1..A36" | "SR-A1..A37" — the rulings file contains SR-A37 |
| `exchange/README.md` row 018 | "awaiting Pro's reply", undated | dated note: no inbox reply as of 2026-09-07; delivery not established from the record |
| `exchange/README.md` status ledger order | 006 filed after 008; 011 after 015 | rows in numeric order, content unchanged |
| `exchange/README.md` informal-captures section | no pointer to the side channel | pointer to [walt-math-intakes](walt-math-intakes.md) for hand-delivered walt notes |
| `exchange/drafts/README.md` index | listed five drafts that had moved | provenance note pointing at outbox 019–023 |

Drift found at c00717d1 in pages owned by other chapters (reported, not edited here). The book rewrite runs page by page, so most of these have since been cleared in the working tree — status as re-checked 2026-09-13:

- `QUICKSTART.md`: said "dispatches 001–018 (count 18; 016–018 hand-ferried)" — the count is 24. Cleared in the working tree (no "count 18" remains).
- [Home](Home.md) exchange row opened "Dispatches 001–018:" then described 019–024, and its tier-3 sentence "3/3 adversarial referees SOUND" was stricter than the applied rule (REACH-20 does not satisfy it literally). Both cleared in the working tree.
- [reachability](reachability.md) said "In flight: dispatch 007 … dispatch 008" although both were adjudicated 2026-07-27 and are described as REACH-19/20 on the same page. Cleared in the working tree 2026-09-12.
- [claim-ledger](claim-ledger.md) at c00717d1: x:017 said SR-A36; the x:009 row said "Lean mechanization pending (dispatch 011)"; the x:004 row did not say `004-cocycle.py` is in-house; the Lean thread had no row. All four cleared in the working tree 2026-09-12 (it now carries a Lean-thread row: kernel tier for exactly the statements the two files make, C1 not mechanized).
- [FINDINGS](FINDINGS.md) §4 addendum cited "claim-ledger rows 9–10" for three constellation rows. Cleared ("rows 9–11").
- [verification](verification.md) "Exchange-adjudicated program runs" enumerated only 003/004/005 although it owns every receipt. Cleared — it now tabulates every program, its recorded run and its re-run.
- [open-problems](open-problems.md) did not note that REACH-18/19/20 have no rob backing. Cleared (the OPEN-11 evidentiary note).
- [idea-retrograde-rank](idea-retrograde-rank.md) said C1 is "pending Lean" (dispatch 011); the working tree now annotates it as still pending with C1 stated in neither Lean file.
- `automation/submit.mjs` header and the `pro-exchange` skill quote "count 18 > cap 17"; `workflow.js` hard-codes `REPO` to the main checkout and mentions a "5-shot reserve"; `inbox/010-….FAILED.md` and `outbox/017-….md.ready` are residue. All still present at c00717d1 (outside the wiki; not book pages).

## 11. Running the receipts

All programs are stdlib-only Python 3; use `-B` so no `__pycache__` is created. Everything below was run 2026-09-13 on this machine (and 2026-09-12 before that) with the wall times shown in §4.2 and §5.

```
cd exchange/adjudication
timeout 60 python3 -B programs/002.py          # ~1 s   → PASS overall counterexample_verified …
timeout 60 python3 -B programs/003.py          # ~0.4 s → PASS OPEN-01_COLLAPSE
timeout 60 python3 -B programs/004.py          # ~4 s   → PASS ALL
timeout 60 python3 -B programs/004-cocycle.py  # ~0.02 s→ ALL_PASS 343 ordered triples …
timeout 60 python3 -B programs/005.py          # ~13 s  → 19 PASS lines
timeout 60 python3 -B programs/001.py          # ~15 s  → PASS headline INTERVAL [35,46] bits
timeout 60 python3 -B programs/006.py          # ~17 s  → PASS headline INTERVAL [36,46] bits
timeout 60 python3 -B programs/007.py          # ~36 s  → … CEILING=45 INTERVAL=[36,45]
timeout 120 python3 -B programs/008.py         # ~72 s  → PASS … NO_VOID_SLICE (not re-run in this pass)
timeout 60 python3 -B programs/009.py          # ~14 s  → PASS all-checks
timeout 60 python3 -B programs/010.py          # ~17 s  → PASS R1 …
timeout 60 python3 -B programs/012.py          # ~17 s  → a4=37 b4=486 b8=126657

cd ../inbox
shasum -a 256 -c 019-023-response-panel-and-cancellation-v0.1.sha256     # OK (response + verifier)
shasum -a 256 024-response-deferred-producers-triple-v0.1.md verify_deferred_producers_triple_v0_1.py   # 337296a7… / a4d23d50…
timeout 60 python3 -B verify_walt_panel_response_v0_1.py      # ~0.7 s → ALL CHECKS PASS (36/36; scratch tier)
timeout 60 python3 -B verify_deferred_producers_triple_v0_1.py # ~7 s   → ALL CHECKS PASS (13/13; scratch tier)
```

Sending a new dispatch is never part of reading this page: it requires Jason's explicit go for an authorized batch, the pre-send guard, and a single operator (`.claude/skills/pro-exchange/SKILL.md`). Adjudicating a new numbered response is `workflow.js` with `args.files` pointing at the inbox paths, run from the main checkout it hard-codes.
