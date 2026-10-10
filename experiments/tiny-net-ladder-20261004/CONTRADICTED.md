# Failed / corrected / inconclusive record

Exploratory. Original accepted/contradicted studies remain unchanged.

- The sandbox MLX probe failed because Metal was inaccessible. Its receipt is
  `results/hardware-probe`; the authorized host probe succeeded, and real training
  used that GPU. This is a permission/environment distinction, not absent hardware.
- First evaluator invocation failed on relative-vs-absolute model path bookkeeping
  (`results/validation0-log`). Fixed output is `validation0-final`; no metric or
  training target was changed by that path repair.
- `results/test1-log` ran before its reference campaign had completed, found no
  test rows and failed. It produced no accepted result. Completed reference test
  invocation is `test1-log-complete`; all605test roots are present.
- Sol found that the earlier `validation1` gate assessed N0 whenever N0 was among
  the compared models. The source now uses the first named model; authoritative
  `validation1-final` names N1 first. Earlier raw output is preserved. Both passed
  the survival arithmetic, but the earlier gate cannot be cited as N1's gate.
- The useful T0 distillation claim passes whole-deal heldout comparison with random.
  N0 superiority over the linear control, N1 improvement over N0, and competitive
  phone strength are **not established**. The two-deal phone smoke is unfavorable.
- No true-policy posterior, automatic belief correctness, improvement theorem,
  linear cost ladder or production readiness follows from the regression fit.
