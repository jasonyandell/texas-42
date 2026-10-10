# Excel AI evaluator / selector boundary recovery

Jeb read the user-authorized source conversation in Chrome, read-only, on 2026-10-04:
https://claude.ai/chat/58a73a5e-c5d2-4355-84a6-9f8234207aad
Title: Building an AI in Excel. UI message numbering: 1–82. No message was posted.

## What Jason's architecture means

Message 17 (Jason) describes Walt as a sense that tries worlds and reports what tends to happen, with a tiny distillation speeding search to the next level. Message 18 proposes search at level k with the previous network standing in for the others and a cheap argmax; its specified output is a per-tile vector matching the sense interface. Message 23 (Jason) identifies the inner sampled pmake computation as the potential replacement inside the next level. This is evaluator replacement, followed by ordinary legal selection, not a one-hot policy classifier.

Message 24 explicitly distinguishes V0 (everyone uniform after the root) from L1 (others uniform, own future optimized), and L2 (best response to L1). It warns that even if two lower evaluators rank moves similarly their pmake values are different: if the upper computation consumes only the chosen move, selection substitution is the relevant contract; if it consumes values for weighting, tie-breaking or plunge logic, those values need the appropriate semantics. Message 26 repeats: use the surrogate's choice, not an incorrectly interpreted percentage. These are historical explanatory statements, not new measurements or proofs of equivalence.

Message 64's still-open alternative is offline, across-hand supervision on the whole candidate vector on common dice, rather than a single game's outcome. The same message's stronger claims about automatically obtaining Bayes values from hidden-outcome labels and its hardware/time estimates are historical claims and are not accepted as established results. Message 40 corrected the own-future strategy-fusion issue with an information-set grouped fold. Therefore earlier exact-tape/per-world optimization claims cannot be imported as lawful teacher guarantees.

## Actual recovered code

Paths below are relative to this isolated worktree.

`experiments/astra-sol-20261004/phase2/incoming/drive/nofusion_sc.py`:

```python
def choose(side_bid, vals, legal):
    return max(legal, key=lambda t: (vals[t], -t)) if side_bid else min(legal, key=lambda t: (vals[t], t))
# expectimax(...): root candidates are deliberately not folded together
vals = {tt: float(val[t == tt].sum() / D) for tt in legal}
return vals, choose(side_bid, vals, legal), legal
```

`expectimax` supplies raw declarer-makes probabilities per legal root tile. The selector maximizes for declarers, minimizes for defenders, and chooses the lowest tile ID on exact ties. Equivalently a seat-oriented success vector uses argmax on both teams. Root evaluation still calls choose in its return tuple; this is a verified separable helper and full-vector return, not a claim that the historical API is a modern pure evaluator object. The wrappers `tail_nofusion` and `tape_nofusion` return `(values, choice)`.

The taped wrapper samples one compatible-deal bundle and one `(worlds, 28)` tape. All root siblings share that bundle/tape. The grouped fold selects one continuation per shared public-history information set instead of a separate own action per hidden world. Its historical sampler and key representation have known audit issues; production teacher work must use the audited repaired versions and legality checks, not blindly reuse this file.

`experiments/astra-sol-20261004/phase2/incoming/drive/exact_tape.py` also returns `(vals, choice)`, but its per-world own-future extrema fuse information. Its V0 method instead makes all future seats random. Neither distinction is erased by exposing a common vector interface.

`experiments/astra-sol-20261004/phase2/incoming/drive/walt.c` uses a tile-returning Policy function pointer. `value` computes a scalar for a candidate's continuation; `walt_policy` loops legal candidates, selects inline and returns only a tile, discarding the vector. `inner_L0` configures eight worlds and uniform assumed play; outer `walt` calls that inner policy. The standalone port's fixed declaration and outer count differ from the pinned phone evaluator. The C port is not evidence of the newer vector-returning API.

## Later design and remaining source gap

Message 72 recommends the ply-synchronous own-choice frontier batched across inner problems, with shared history-group reductions. It distinguishes flat restricted plans from the full own-choice tree, and references `batched.py`, `l2.py`, `exact_tape.py` and `walt42x.py`. The uploaded flat/batched artifacts and later sampler repairs are preserved under `experiments/adversarial-20261004/incoming/`; the explicit `l2.py` outer source is not in the committed incoming packet.

Messages 30 and 52 mention shared-container files `rungs42.py`, `tables.py`, `levers.py`, `tourney.py` and `l2ab.py`. They do not supply a new evaluator/selector signature in the message body. These files are absent from the recovered committed artifact inventory; the chat itself reports accidental deletion of rungs42.py. Message 52 explicitly points a Claude Code session to walt.c and nofusion_sc.py as actual artifacts. No accessible link to a distinct evolved API was established. Do not fabricate a newer interface from the hint UI or infer that the missing files use one.

Thus the architectural intent and one real Python boundary are recovered. The exact newer implementation, if Jason means a different session/version, remains unverified. The existing hint mapping is evidence for outer display values only; it does not decide which inner computation to replace.

## Reconciliation required before experimental execution

Name the lower rung, its assumed continuation policy, own-future optimization, sample budget, lawfulness and target orientation. State whether the upper rung consumes only selected actions or numerical values. Preserve the whole legal candidate vector, ties, public/own-hand inputs, common sampled worlds/tapes and whole-deal independent test. Keep selection as the existing deterministic legal argmax/argmin adapter. Choose direct Q, centered advantages, or baseline-plus-advantage only after the consumer contract is explicit. Test nested replacement by disagreement and action gaps on inner calls plus paired outer regret; small average vector error alone cannot prove safe conditional substitution. No new labels, training, builds, games or benchmarks are authorized by this recovery note.
