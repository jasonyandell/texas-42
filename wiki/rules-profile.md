# Rules Profile — Straight Points-and-Marks Texas 42

[Home](Home.md) · Source: `docs/10_RULES.md` (**byte-identical in both packages**; all
statements below are **Adopted rule / Clarification** unless labeled otherwise).

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
