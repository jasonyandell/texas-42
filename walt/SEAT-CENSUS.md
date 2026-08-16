# The seat-level census, part one: three alphabets, composed never
# (design for adjudication)

Status: DESIGN, awaiting walt-math rulings (S-Q1..S-Q6) in
`walt/CENSUS-RULINGS.md`. One-author rule unchanged. Standing rulings inherit:
F1–F7, r3 Q1–Q5, Y1–Y3, P-A1..P-A21, X-A1..X-A19, E-A1..E-A21, Lemmas V, X
and E. Tier: exploratory; this is a CENSUS — counts only, no values, no
composition.

## The bar this finally addresses

Jason, 2026-08-10: show or disprove that the number of situations facing the
trick-1 leader is reasonably small — order 10^5 — against the ~399M-world
fiber behind the seat. The raw seat space at the first play is
C(28,7) = 1,184,040 hands. Every measurement so far has been world-level;
this is the first seat-side quotient.

## Jason's framing (2026-08-11): the flat stack

Pull the nesting out to the side: instead of the tree of nestings, a stack of
LAYER ALPHABETS plus TRANSITION alphabets between adjacent layers — count
each, compose nothing. The nesting is irreplaceable for values (the
contradiction is localized there and deliberately out of scope); for counting
and dividing the work, layers-and-interfaces is a lawful presentation, and P1
(one grade-free machine at every level) plus the a1 result (the complete
last-trick alphabet is exactly 64) are the reasons to expect the interfaces
to have "one set shape." The first play is the cleanest possible layer: the
declaring seat leads trick 1 (focal = leader, P-A4 consistent), and NOTHING
has been observed — the unseen pool is exactly the complement of the hand, so
the hand alone determines the entire structural situation facing the seat.

Part one measures the top of the stack and its first interface:

  COUNT 1 — the hand-form alphabet: hands of 7 up to the seat-side
            structural quotient. The top layer.
  COUNT 2 — the first-trick interface alphabet: the shapes of trick 1 as
            seen from the seat, from a given hand class to the landing. "The
            size of the first trick, given that we are just trying to get to
            the second rock and no further."
  COUNT 3 — the landing alphabet: post-trick-1 seat states up to the
            quotient. Where observation accretion begins.

Both outcomes are results (F7): small alphabets = the factoring is real at
the top; an exploding landing alphabet = we learn exactly which interface
breaks flat representation.

## Design questions

S-Q1 (the seat-side hand form — the new declared object). Proposal: the
canonical form of the two-block structure (mine = 7 tiles | pool = 21 tiles)
under tile relabeling with the trump structure distinguished, carrying the
relations the remaining game reads from the seat's side: trump membership,
within-hand and hand-vs-pool beats relations per context, the led-context
map on my tiles, double flags. Questions: (a) exactly which relations must
the form carry — is there a seat-side analogue of Lemma E ("equal hand forms
⇒ a relabeling carrying one seat-facing situation to the other, preserving
everything any future count-free question reads"), and can it be stated and
proved at adjudication time (Lemma S)? (b) may the form fold the seven
pip-trump declarations together (the world-level form merges across trumps
when no encoded relation differs — same here?); (c) is the world-level
`canonicalize` machinery reusable (a two-block partition is a degenerate
world), or is a new implementation required — and if new, E-A5's one-code-
path discipline needs restating for the seat side.

S-Q2 (the first-trick interface alphabet). From a hand class, the declaring
seat leads: 7 lead choices; three replies from the pool; an outcome. Proposed
shape of one interface element: (the lead tile's role in the hand form; the
three replies' classifications against the led context and their relations
to each other, to the lead, and to the surviving hand — the a1 anatomy,
seat-facing; the count-free outcome; the identity of the four dead tiles AS
ROLES in the form, since the landing hand's form depends on what died).
Questions: is this the lawful quotient of "trick shape from the seat," and
what must it carry so that (hand form, interface element) DETERMINES the
landing form? That determination — flat-stack composability of counts, not
of values — is the property Jason's "landing on the next rock" needs; if it
cannot hold with a bounded alphabet, that is the finding.

S-Q3 (the landing alphabet). Post-trick-1 seat state: 6 tiles, 4 dead, voids
observable from the three replies (a slough shows a void — F2's observation
tokens), next leader as an offset from focal. The quotient: same form
machinery over (mine = 6 | pool = 18) PLUS the observation structure.
Question: what is the lawful minimal observation content the landing form
must carry (voids per hidden offset? the dead tiles' roles? the trick's
outcome?) — and is the landing form's domain "reachable landings only"
(enumerated via COUNT 2) or "all (6|18) states with declared observation
structure" (a superset, closed-form)? Both may be counted; which is the
claim?

S-Q4 (what counting licenses — the scope fence). Counts only: no values, no
policy claims, no belief claims, no composition of interfaces into
evaluations. The flat stack never replaces the nested object for values
(deliberately out of scope, stated in the results file). Confirm the fence
sentence.

S-Q5 (enumeration and freezes). COUNT 1 is exact and cheap: enumerate all
C(28,7) = 1,184,040 hands (× declarations unless S-Q1b folds them), canonical
form each, count distinct. COUNT 2/3: enumerate per hand class from a
representative (the pool is determined), with declared stops if needed —
never silent. New freezes: the seat-side form definition and encoding; the
interface-element encoding; the enumeration order. Numbering continues after
17.

S-Q6 (results discipline). One file
`results/seat_census_2026-08-11.txt`; inherited boilerplate (P-A20 lineage);
the three counts with integers first; per-declaration splits if not folded;
anatomy sections in the a1 style; the bar stated and answered honestly
(COUNT 1 vs the 10^5 bar is the headline); both-outcomes framing.
