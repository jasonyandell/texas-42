# Rules Profile — Straight Points-and-Marks Texas 42

[Home](Home.md) · owns: the normative rules profile · Source: `docs/10_RULES.md`
(**byte-identical in both packages**; statements are **Adopted rule / Clarification**
unless labeled otherwise).

## Setup
- One double-six set (28 dominoes), four seats clockwise, opposite seats partnered
  (R-SET-01, R-SEAT-01, R-TEAM-01).
- Configuration: match target `T` (customary 7) and max mark bid `m_max`; no value is
  mathematically privileged (R-CONFIG-01/02).
- Deal: 28 dominoes randomized, seven per seat, no boneyard (R-DEAL-01). **Mathematical
  chance rule**: uniform over ordered deals; each redeal independent (R-DEAL-02/03).

## Auction (one round)
- Left of shaker acts first; each seat acts exactly once: pass or a bid exceeding the
  current high bid (R-AUC-01/02/06).
- Bids: points `P(30)…P(41)` then marks `M(1)…M(m_max)`, ordered
  `P(30)<…<P(41)<M(1)<M(2)<…` (R-AUC-03/04/05).
- First mark bid ≤ 2 marks; each later mark overcall is exactly +1 (R-AUC-07/08).
- All-pass ⇒ redeal with next shaker, no marks (R-AUC-11).
- **Clarification R-AUC-12 [Theorem — proved, Math §4.3]**: the reachable mark ceiling
  is `min(m_max, 5)` — longest chain `M(2),M(3),M(4),M(5)`; every cap ≥5 induces the
  same legal auction tree. Terminal-history counts for caps 1..7:
  2380, 3060, 3196, 3213, 3214, 3214, 3214 [Theorem — exhaustive finite verification].
  *Tier line.* The **mark ceiling** (CFG-03) is a corpus THEOREM — proved *and* a
  proof-assistant kernel theorem: PA-B03 `mark_le_ceiling` / `mark_five_reachable`
  (`lean/Texas42/Auction.lean:207`, `:216`; kernel-proved 2026-07-29). The **auction
  census** (AUC-05A) is a corpus THEOREM — finite verification (`verify_foundation.py`,
  line "auction terminal histories for caps 1..7"; re-run 2026-09-12 on this machine,
  identical), reproduced in Rust by rob's receipt `r_obj_auction_census` — conformance
  evidence, never a status change ([verification](verification.md)); its kernel
  reflection PA-B04 (priority 3) is **open**, so the seven integers remain visibly
  external to the kernel.

## Declaration and contract
- Winner publicly declares one of **nine** options: pips 0–6 trump, doubles trump,
  no-trump/follow-me; legality never depends on hand content (R-DECL-01).
- `P(n)`: threshold `n`, stake 1 mark. `M(m)`: threshold 42, stake `m` (R-CONTRACT-01/02).
- Bidder leads trick one (R-LEAD-01).

## Suits, tricks, scoring
- Pip trump `p`: every domino containing `p` is trump (called out of its natural suits).
  Doubles trump: the seven doubles form the trump suit. No-trump: nothing is trump
  (R-SUIT-01/02/03).
- A called domino leads the called suit; an uncalled domino leads its higher pip
  (R-PLAY-04/05). Follow the led *effective* suit if possible, else anything
  (R-FOLLOW-01/02).
- Highest trump wins, else highest follower; off-suit discards can't win
  (R-WIN-01/02/03). In a natural suit the double is highest, mixed ordered by pip sum;
  in pip trump the trump double is highest; doubles trump ranks 6-6 … 0-0
  (R-RANK-01/02/03).
- Count: 5-5 and 6-4 worth 10; 5-0, 4-1, 3-2 worth 5 (total 35); each trick +1;
  hand total exactly 42 (R-SCORE-01..04).
- **Clarification R-SETTLE-02A [Theorem — proved, Math §4.5]**: in full play,
  `P_D = 42` ⇔ the declaring partnership wins all seven tricks, so the 42-threshold and
  the traditional "take every trick" wording define the same mark-contract success event.

## Match
- Marks accumulate; first partnership at ≥ `T` wins; shaker advances clockwise each
  attempt (R-MATCH-01..03). Ignoring all-passes a match lasts ≤ `2T−1` contracted hands
  [Theorem — proved, Math §5.8]; repeated all-passes are unbounded without an added
  assumption [Boundary, Math §4.3].

## Information
- Each player privately observes their own hand each attempt (including abandoned
  ones); the public stream is bids/declaration/plays with actors; players have perfect
  recall match-globally (R-INFO-01..03). Derived facts (winner, score, settlement…) are
  deterministic functions of the base stream — materializing them must not create a
  second source of truth (R-INFO-02A). No side channels (R-COMM-01).

## Exclusions [Boundary]
Nello, plunge, splash/crash, sevens, exposed-hand rules, renege adjudication, etc. are
**outside the formal object** and structurally so — they can change declaration
semantics, active-player count, and the shape of exact support; no theorem transfers
automatically (00_THESIS §3; Rules §12).

## Mechanization status (proof-assistant kernel tier)

Rows are **v0.7** `65_MECHANIZATION_LEDGER.md` `PA-` rows (priority in parentheses);
"proved" = a declaration under `lean/Texas42/` checked by the Lean kernel over at most
`propext`/`Classical.choice`/`Quot.sound`, with no `sorry`, `native_decide` or local
axiom (grep re-verified 2026-09-12), as of commit d190b26 (2026-08-02; all 42
priority-0 rows closed). Row-to-declaration map: [lean-row-index](lean-row-index.md).
A kernel theorem never promotes a corpus status, and a rob receipt named here is
conformance evidence, never a status change ([Home](Home.md)).

| Rule / claim on this page | Ledger row (priority) | Kernel status (d190b26) | Declaration (`lean/Texas42/`) |
|---|---|---|---|
| R-SET-01, R-DEAL-01: one double-six set (28), four seats, seven each | PA-A01, PA-A02 (0); PA-B05 (1) | proved (finite types; `Fintype.card Domino = 28`); `Deal` defined; the deal cardinalities `28!/(7!)⁴` and `21!/(7!)³` **open** (PA-B05 prove half) | `Basic.lean:60` `card_domino`; `Deal.lean` `Deal`, `biUnion_eq_univ` |
| R-AUC-01..08: one round, one action per seat, ordered bids, mark entry ≤2, +1 overcalls | PA-B01, PA-B02 (0) | proved: decidable `legalBid`, deterministic `step`, `LegalAuction` derivations | `Auction.lean` |
| R-AUC-12 / CFG-03: mark ceiling `min(m_max,5)`, chain to `M(5)` | PA-B03 (1) | **proved** | `Auction.lean:207` `mark_le_ceiling`, `:216` `mark_five_reachable` |
| AUC-05A census 2380 … 3214 | PA-B04 (3, REFLECT) | **open** — external finite receipt (ingest verifier; rob `r_obj_auction_census`) | — |
| R-DECL-01: nine declarations, legality hand-independent | PA-A05 (0) | proved (`card_declaration = 9`) | `Trick.lean:38` |
| R-SUIT / R-PLAY-04/05 / R-FOLLOW / R-WIN / R-RANK: effective suits, led context, follow, unique winner | PA-A05..A11 (0); PA-B08 (0) | proved (winner from key injectivity, not enumeration); legal set lead/follow/slough characterization | `Trick.lean:319` `existsUnique_winner`; `Play.lean:193–221` `legalSet_*` |
| R-SCORE-01..04: 35 count, one point per trick, hand total 42 | PA-A04, PA-B09, PA-B10 (0) | proved: `total_countPoints = 35`; invariant-preserving step; at any terminal state seven tricks and `score 0 + score 1 = 42` | `Basic.lean:77`; `Play.lean:359` `inv_step`, `:559` `terminal_scores` |
| R-CONTRACT-01/02, settlement K-01..K-03 | PA-B06 (0); PA-B13 (1) | contract, threshold, stake, `Makes`, `award` **defined**; deterministic-settlement theorem **open** | `Play.lean` `Contract` |
| R-SETTLE-02A / K-04 sweep equivalence (`P_D = 42` ⇔ seven tricks) | no row of its own (PA-B13 territory) | not stated as a named kernel theorem; rob `r_obj_conservation` "201 hands; P_D=42 iff seven-trick sweep" is conformance | — |
| R-MATCH-01..03; MATCH-01 (≤ `2T−1` contracted hands); MATCH-02 / AUC-06 (all-pass unbounded) | none — "full match almost-sure termination" is a **deferred module** of the ledger | not mechanized | — |
| R-INFO-01..03 perfect recall; R-INFO-02A derived facts are not a second source of truth | PA-C01 (0); PA-F05 (0) | proved: deal-local perfect-recall record; the mechanical projection is **not injective** on it | `Information.lean` `DealLocalInfo`, `:47` `mech_not_injective` |
| R-DEAL-02/03 uniform chance rule and its posterior | PA-E04 (1) | **open** (physics-only posterior under uniform chance) | — |
| Exclusions (nello, plunge, splash, sevens, …) | "special contracts" is a **deferred module** | not mechanized, by design | — |
