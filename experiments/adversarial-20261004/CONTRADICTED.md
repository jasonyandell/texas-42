# Contradicted and corrected source assertions — retained history

This is separate from [accepted findings](ACCEPTED.md). Sources are preserved,
not overwritten. Only actual sourced assertions or precisely stated
interpretations are recorded; absence of proof is not contradiction.

## C1

**Assertion:** the hard-coded bidder is irrelevant to historical legality in
[engine42.py:178](../astra-sol-20261004/phase2/incoming/drive/engine42.py:178).
The subsequent [AUDIT-RESPONSE item 2](incoming/drive-0815/AUDIT-RESPONSE.md:12)
says this sampler was unaffected; [MATH-TEAM-STATUS](incoming/drive-0815/MATH-TEAM-STATUS.md:12)
attributes our avoided bug to `batched.py` v1 instead.

**Counterexample:** [full legal deal, history and accepted illegal world](sol/receipts/bidder-counterexample-numpy/stdout.log).
True bidder 1, trump 2, 16 plays, score even 7 / odd 17. Fake bidder 0 ends after
one trick at odd 16. The accepted hidden hand revokes at zero-based ply 7:
seat 2 plays tile 0 despite holding tile 24, the required follower of lead 6.
Both ZIP and current Drive versions reproduce it.
[Direct independent replay](results/lead-bidder-counterexample/stdout.log).

**Correction:** the defect is in `consistent_worlds`, not the separate `replay`
function. Prevent historical replay from stopping at the fake contract
threshold. [Repair](sol/repaired_sampler.py), [complete finite support test](sol/receipts/sampler-repair/stdout.log).
The new `batched.py` v1 fallback is a separate defect, removed in its v2.
No production Walt or playing-strength claim is contradicted by this witness.

**Later acknowledgment:** [AUDIT-RESPONSE-2](incoming/drive-final/AUDIT-RESPONSE-2.md)
explicitly supersedes item 2 and acknowledges the separate replay defect.
The revised source [engine42.v2.py](incoming/drive-final/engine42.v2.py)
introduces `no_stop=True` for historical replay. The earlier assertions remain
here as history, not accepted current findings. Its prose paraphrases our
fixture inaccurately as holding tiles 2 and 4; the saved executable fixture
and exact required follower 24 above remain authoritative.
The [independent v2 audit](sol/receipts/final-packet-fixed/stdout.log) confirms
the repair on that witness, exhaustive finite support and 384 sampled outputs.

## C2

**Assertion/interpretation removed:** the recovered creation script's
validation is held-out-game validation. The script's actual
[row permutation](../astra-sol-20261004/phase2/incoming/rollout_net.py:81)
and [Drive summary](../astra-sol-20261004/phase2/incoming/drive/TINY-MODEL.md)
must be read with their precise scope: “held-out” rows do not mean held-out games.

**Evidence:** [exact split regeneration](sol/receipts/teacher-data-split/stdout.log)
finds all 20,000 validation rows overlap training at game identity. This does
not contradict that rows themselves were held out, establish accuracy
inflation, or invalidate lawful uniform-rollout labels. Later neural versions
are unverified and are not silently equated to this creation script.

## C3

**Retraction already accepted by the proposer:** the one-line sum-before-choice
swap is insufficient for the sampled-m branch of
[l1_orders](../astra-sol-20261004/phase2/incoming/drive/exact_tape.py:70), because
each world draws a different ordering list. Column index alone is not a shared
policy. [AUDIT-RESPONSE item 1](incoming/drive-0815/AUDIT-RESPONSE.md:5)
explicitly retracts that suggestion.

**Correction retained:** draw one canonical deduplicated plan list, tile it
across worlds, reuse each world's tape across plans, sum by actual plan before
optimizing. [flatplan.v2.py](incoming/drive-0815/flatplan.v2.py) and our
[shared_orders.py](shared_orders.py) implement that structural repair.
Their shared-ordering policy class is deliberately restricted. Our concrete
class-gap fixtures quantify that known limitation; they are **not** a
contradiction of the proposer's qualified claim.

## Not placed in contradiction history

The stochastic nofusion recurrence lacks explicit prefix reach weighting, but
our 653 legal test roots found no discrepancy. Its actual Texas42 impact,
claimed historical match strength, later neural training, full conversation
contents and higher-k linearity remain unverified. Original phase-3 one-bid
and thirteen-bid measurements both reproduce; workload differences are not
contradictions. Flatplan v2 gaps use a fused reference and different positions;
they are not directly comparable to our lawful-grouped-reference regret.
Flatplan v3 now distinguishes these objectives; its numerical table remains
unverified rather than contradicted.
