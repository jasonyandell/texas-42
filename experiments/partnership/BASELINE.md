# Partnership launch baseline freeze

Observed 2026-09-06. **EXPLORATORY**: implementation identities, legal fixtures,
and reproducible instrument inputs; no playing-strength result is established by
this document. The packet is the experiment brief, while repository sources and
the inspected local client establish what actually exists.

**Later calibration, 2026-09-06:** [50 fresh paired deals / 150 games](campaigns/native-l1-vs-phone-620600-649/CALIBRATION.md)
completed in 202.12 seconds. Native fixed-sample L1 recorded 8 favorable
contract flips / 15 unfavorable / 77 ties against this phone reference;
equivalence was not established. Native L1 is a useful separately named
internal reference, with the phone retained as an external strength anchor.
The original artifact identification below remains unchanged.

## Actual local phone client

The available client is `/Users/jason/code/plunge`, inspected clean at commit
`122ea7a59362c15850e7b54e76ce4353ddb9d07a`. This identifies the local checkout;
it does not independently establish the version currently deployed on a phone.

| Coordinate | Frozen local behavior and evidence |
| --- | --- |
| Play | `n=40`, `n0=8`, `race=true`: `src/ai/walt/requests.ts:35-38,139-157`; default tuning installed by `src/ai/walt/index.ts:69`. No production tuning overrides were found. |
| Play seed | FNV-1a 32-bit hash of `walt/${handNumber}/${shaker}/${marks[0]}-${marks[1]}`: `requests.ts:108-113`, `src/engine/rng.ts:17-25`. This uses public state, not an undisclosed deal. |
| Playing input | Own seven originally dealt tiles reconstructed from own remaining hand and own public plays; full public chronological actor/tile record; contract, declaration, actor, bidder. `requests.ts:115-157`; `src/ai/observation.ts:1-19`. |
| Auction | `requestFor` returns null for bidding, so the action goes through `hard`; `hardAction` delegates bidding to `mediumBid`: `index.ts:171-177`, `src/ai/hard.ts:177-185`. `mediumBid` is the heuristic ladder at `src/ai/medium.ts:110` onward, including partner-high handling. |
| Declaration | Walt declaration handler, with the same 40/8 tuning: `index.ts:178-180`, `requests.ts:186-195`. |
| Unused bidder knob | `WALT_THETA=[19,20]` remains in a tested request builder but is not the live auction: `requests.ts:40-46`, `index.ts:171-177`. |
| Review | Full evaluator, `race=false`, `n=40` or “look closer” `n=160`, `n0=8`; seed FNV-1a of `explain/${handNumber}/${shaker}`: `src/ai/walt/explain.ts:29-33,103-115`. Review values are fresh model estimates with different seed/mode from played decisions. |
| Delivery | Synchronous WASM in a Web Worker, response cache keyed by exact request, legality and rules-conformance checks; miss/error/out-of-scope falls back to hard: `index.ts:19-37,167-168`. |

Local phone artifacts (SHA-256):

```
af0200af8dc99d5a95ac898cded12861ca4a36c38b336dbbd6a33a515c071d59  /Users/jason/code/plunge/src/ai/walt/walt.wasm
f05aeca0e7f680878fdf601410e6d15ac5911ee8935094fc0fa233888901d606  /Users/jason/code/plunge/src/ai/walt/walt.ts
```

Both local artifacts are preserved byte-for-byte under `reference/phone/`, with
`MANIFEST.sha256` and `PROVENANCE.md`. `phone.mjs` loads that exact wrapper and
binary for a single JSON request, selecting only own-hand and public fields;
the parent process owns timeouts and timing. Node >= 23.6 is required for
the original TypeScript wrapper.

The local WASM file was added by Plunge commit
`1810da2060bb8354130fd0315efb5b5d04816392` (2026-08-22); `git log` shows no
later binary update. A subsequent historical-object search identified its
byte-for-byte match in this repository at
`9a056f20461fbe951544e4145ee49b726e0f6852` (2026-08-19). The source anchor and
hashes are in [PROVENANCE.md](reference/phone/PROVENANCE.md).
**Current native L1 is still a separate decision procedure.** Preserve or load
the exact WASM for a literal phone-artifact comparison, or label a current
native 40/8 race run as a proxy and report its parity checks separately.

## Current repository implementation

The current worktree's `walt/walt-wasm/pkg/walt.wasm` SHA-256 is
`d7f61f222cd0a03175584a1e5e3a1fef73675cb0860d5cd5d878d0a6c85c5e2a`;
its `walt.ts` is
`9af4114dad28a70c369878e544670ce0130ec0b3edf9d11d5665efa531b4ac68`.
Both differ from the local phone artifacts. Source anchors below are in the
launch worktree before the partnership candidate change.

* `walt/walt-wasm/src/api.rs:194-198` defaults to 40/8, seed
  `0xB7E151628AED2A6B`, **race off**, and a 120-second native budget. The
  actual phone play setting explicitly turns race on.
* Current API play streams are `SplitMix64(seed ^ mix(original_hand_mask)
  ^ record_hash(key))` (`api.rs:245`). `record_hash` incorporates played
  mask, current leader and current-trick tile order (`solver/mod.rs:194-200`).
  Modeled-mind streams use `INNER_SEED=0x243F6A8885A308D3`, the level tag,
  acting seat, own hand, and record hash (`solver/mod.rs:120,797-803`).
* Full L1 is a best response to `Field::Level(0)` using `n_inner=[n0]`
  (`solver/mod.rs:1248-1265`). Candidate values are exact rationals over
  sampled worlds, not full-fiber exact game values. Equal best estimates
  trigger fresh 4x then 16x sample refinement (`mod.rs:1278-1305`). Remaining
  ties retain the first ascending tile (`mod.rs:1410-1426`).
* Race mode supplies a `2*n` race cap and `n` refinement base
  (`api.rs:245-269`). The race uses blocks of 8 and can call full refinement
  on the tied survivors (`solver/mod.rs:1876-1955`). Nominal `n=40` does not
  mean only 40 worlds of total work; exact-equal ties also trigger refinement.
* Native deadlines expire; the WASM deadline never does
  (`solver/mod.rs:67-103`). Record measured completed timing and distinguish
  a process timeout from a solver answer. A native run that hits a deadline
  is not byte-equivalent evidence for the no-clock WASM path.
* Current L2 defaults are `n=8,n1=4,n0=2`, with `Field::Level(1)` at all
  modeled seats and `n_inner=[n0,n1]`, the same outer seed formula, and the
  same fresh 4x/16x tie refinement (`walt/walt2-wasm/src/api.rs:166-251,280-283,319`).
  Its bid/declare handlers remain L1, pinned by
  `walt/walt2-wasm/tests/full_hand.rs:199`.
* The current API bidder uses theta 11/16 and walks the best declaration
  upward while its sampled make estimate clears that threshold
  (`walt/walt-wasm/src/api.rs:419-475`). This differs from the phone's
  heuristic auction. Freeze contracts for a focused play-only comparison;
  do not describe it as an auction-strength comparison.
* `walt_bridge` is also a different baseline: its line protocol is always
  P(30), declaration defaults to pip trumps, and its default outer budget
  is 50 (`walt/walt/src/bin/walt_bridge.rs:22-36,957-960`).

## Frozen fixtures and scope

`fixtures.json` uses triangular tile ids `high*(high+1)/2+low`, seats
0/2 against 1/3, and flat chronological `(actor,tile)` pairs. A root carries
only its acting seat's original hand plus public state. A full deal is separate
referee input and must never be passed into a decision request.

G1 is the screenshot-transcribed, rules-validated failed hand (bid 30, sixes,
25-17), source `walt/probes/gran/g1.receipt.txt:12-32`. Its complete deal supports
four trick-1 roots (bidder, both defenders, partner) and a later partner root.
Shaker, auction, prior marks and original seed were not recovered. The common
development decision seed **420600 is newly frozen experiment provenance**,
not the historical phone seed. Do not derive a supposed historical seed from
the fixture's placeholder shaker/mark values.

G2/G3 source `walt/probes/gran/g2g3.receipt.txt:11-46` is a six-trick prefix.
Gran's S2 hand is completely known: 6-4, 5-0, 3-2, 3-3, 4-3, 5-5, 5-2.
The three residual tiles 4-1, 4-4, 5-3 can be assigned to S0/S1/S3 in six
ways. These roots are valid information sets but not a recovered complete deal.
G2's trick-1 6-4 is forced (it is S2's only six): useful legality sanity, not
evidence of a changed strategic choice. G3's trick-4 root has four legal options
and is the reported saturation anchor. These are development anchors because
the project has already studied them.

Fresh deal seeds **420601, 420602, 420603, 420604** are reserved now as held-out
inputs. No generated outcomes were inspected during this freeze. The driven
harness owns and must report its deterministic deal algorithm and the separate
decision-seed schedule; the integer seed alone does not define a deal.

The Gran status is newer than stale “blocked on missing seed” prose/tests:
`kanban/backlog/gran-anchor-reconstruction.md:52-105` records G1 completion and
G2/G3 prefix validation. No missing hidden tiles are invented here.

## Focused existing checks

No builds or experiments were run in this orientation step. Commands below
are from `walt/` and can be selected for the relevant change:

```
cargo test --release -p walt-wasm --test full_hand full_hand_all_walt_raced -- --exact
cargo test --release -p walt2-wasm --test full_hand auction_matches_walt1 -- --exact
cargo run --release -p walt --bin granrun -- validate probes/gran/g1.receipt.txt
cargo run --release -p walt --bin granrun -- validate-partial probes/gran/g2g3.receipt.txt
```

`walt/walt-wasm/tests/full_hand.rs:1-7` is a small 6/2 lawfulness,
conformance and determinism harness, with race coverage at line 268.
`walt/walt-wasm/smoke.mjs` additionally checks its actual packaged WASM against
a frozen native full-hand trace; it is hardwired to the current package, not
the local phone package. The local phone has a separate tiny game driver at
`/Users/jason/code/plunge/tests/walt-play.test.ts:47-66` (4-world test tuning).
Neither tiny conformance harness is evidence of playing strength.
