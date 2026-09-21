id: [[scheme-compact-compiler]]
opened: 2026-09-13

## What

`walt::scheme` (2026-09-06/07) executes typed relational expressions,
one-play finite dynamics and executable policies on the full finite states
supplied to it. `walt/scheme/README.md` names what it deliberately does not
implement: "the still-absent compact descriptor step compiler" — the step
from a Scheme descriptor to a compact transducer whose state is the
descriptor's bindings rather than the extensional world set — "and a
response-preservation/lumpability proof" for it. That compiler was the
original purpose of the Scheme/Fix research (walt S4–S6, archived:
`wiki/walt-scheme-fix.md` §16), where the negative result was that at the
candidate scale the only lumpable skeletons were world-reconstructing
(`wiki/walt-negative-results.md`). Jason's redirection (2026-09-06):
"invented to compress; commissioned here to express."

This card holds the compression half so it is not lost. It is NOT queued:
it opens no work without Jason's word, and it would need a mathematical
parent, which the 2026-09-04 ruling forbids until [[consolidation-slice]]
lands.

## Done when

Either (a) a design note states the descriptor-to-transducer step, the
lumpability obligation it must discharge, and which S4–S6 negative results
constrain it, and Jason schedules it; or (b) the card is closed as
"expression only" with the reason recorded. Nothing on this card is
evidence.

## Links

`walt/scheme/README.md`, `walt/scheme/DYNAMICS.md`,
`walt/scheme/POLICIES.md`, `wiki/walt-scheme-fix.md` §§14, 16,
`wiki/walt-negative-results.md`, [[consolidation-slice]].
