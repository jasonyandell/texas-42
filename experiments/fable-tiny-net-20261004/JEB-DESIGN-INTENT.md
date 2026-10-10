# Jason's design clarification, received after test freeze

Jeb received this from the coordinating parent task while this run was in progress:

> Jason's dream is input position, output chances like Walt sees, not just one action, since ties are prevalent.

This is a request to clarify the existing interface and retain the full legal-action chance vector and ties. **Do not modify frozen models, test, data, or selection to chase already seen results.** Record the distinction candidly in the report and propose any future absolute-chance/calibration work as a separate future experiment.

Source inspection: raw862 models emit 28 per-tile scores; the lawful feature scorer emits one shared-network score for every legal candidate, making the full action vector. Targets use all legal Q values, not argmax-only/class labels. But current targets are `4 * (Q(a) - mean_legal Q)`, with legally centered predictions and masked vector MSE. The absolute state baseline is discarded. There is no sigmoid/calibration, and a softmax over hypothetical action outcomes would not be the desired chance vector. Centering preserves pairwise action gaps and target ties; the runtime decision/evaluation still chooses legal argmax, with ascending physical tile ID for exact ties.

For a future implementation aligned with Jason's goal, retain/output each action's acting-partnership win chance under an explicitly specified teacher/belief/continuation. Possible routes are direct per-action Q regression with calibrated outputs or a state baseline head plus legal advantages. Preserve equal/near-equal actions and uncertainty rather than only a chosen class. The current controls diagnose distillation; they do not yet supply absolute calibrated chances or prove gameplay strength.
