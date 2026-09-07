# Player families and names

Use the **family** to say whose thinking is modeled, then name any departure
from its default search or belief. This terminology applies to the sampling-stack
player in this experiment; it does not rename the separate proof-state player.

## Everyday names

| Say | Runnable preset | What it does |
|---|---|---|
| **L1 default** | `l1-default` | Models all three other seats at L0. Fixed search, legacy inner beliefs. Same behavior/settings as `l1-fixed`. |
| **L2 Partner default** | `l2-partner-default` | Models the partner at L1, opponents at L0. Fixed search at the real root and inside modeled minds. |
| **L2 Partner with voids** | `l2-partner-voids` | Same as L2 Partner default, with counted sampling respecting public voids inside modeled minds. |

In the current partnership conversation, **L2 means L2 Partner** unless stated
otherwise. Write the full family name in durable reports. **L2 All** means all
three other seats are modeled at L1 (`--mode all-l1`); it is a separate family
and is not included in the default-partner battery.

"Default" means the existing **fixed-search settings**, not whichever policy
has the highest observed score. These names do not alter `player.py` defaults.
Historical presets and campaign identities are preserved.

## The independent settings

| Category | Choices | Meaning |
|---|---|---|
| Family | L1 / L2 Partner / L2 All | Which other seats receive modeled L1 thinking; the rest are L0. |
| Root search | Fixed / Refine / Race | Fixed takes one sample bundle. Refine takes fresh larger bundles when best values tie. Race compares blocks, can eliminate candidates, then refines tied survivors. These procedures can choose different moves. |
| Modeled search | Fixed / Refine / Race | The same selection choices applied inside modeled L1 minds. Inactive for L1 itself; L0 stays fixed. |
| Inner belief | Legacy / Voids | `voidless` ignores public void constraints in inner samples; `voids-counted` respects them through the existing exact uniform sampler. Outer samples already respect voids in both cases. |
| Sample budget | Root / L0 / L1 | The separate sample counts at the real root and within modeled minds. |
| Execution allowance | Up to 14 seconds/move | The external wrapper's deadline, including fallback preparation and cleanup. |

The default sample counts are **40 / 8 / 2**. L1's modeled-L1 count is inactive.
An L2 Partner's modeled L1 uses a two-world base bundle, not the real L1's
forty-world budget. Sharing a decision procedure does not make those two
approximations identical.

The belief choices use different deterministic inner sample streams. Comparing
them measures the implemented strategies as a whole. Public-void support is
not the same as learning a behavioral posterior from all observed choices.

## Fallback is an outcome of execution

A fallback is **not** a search family, refinement step, or a stronger player.
The wrapper first prepares a cheap, completed fixed L1 reserve (8/2 samples),
then attempts the requested search. If that search cannot finish, it returns
the labeled `l1-fallback`; if even the reserve failed, it returns the labeled
`legal-fallback`. Forced moves have their own route. Requested search can be
expensive even when the eventual fallback is cheap. The selected belief
strategy also applies to the native reserve.

Modeled minds never choose based on a timeout: incomplete modeled computation
aborts the parent attempt. Only the outer real-player wrapper can fall back.
Measured strength and time include that complete executed policy.

## Archived phone and advanced presets

**Phone** is the separately preserved WASM artifact using its original
race/refinement procedure. **L1 Race** (`l1-race`) is the native counterpart
whose completed decisions matched the archived phone in the foundation checks.
It is not an alias of **L1 default**. Native `l1-fixed` is the historical alias
of L1 default; names beginning `partner-` belong to the L2 Partner family.

`partner-race` uses Race at both root and modeled L1; `partner-race-fixedmind`
uses Race only at the root. A `-small` suffix changes sample budgets. Use the
expanded catalog for every coordinate, rather than inferring it from a name.
Existing advanced names stay available for reproducibility, not as a preferred
set of choices or a promise that they meet the latency gate.

```sh
python3 experiments/partnership/match.py players
python3 experiments/partnership/match.py players --all
```

`players.json` supplies named presets. Initialization validates and expands
every default into each immutable match manifest; later catalog edits cannot
change a running or saved experiment.

## Current comparisons

The [default-partner protocol](campaigns/default-partner-battery/PROTOCOL.md)
compares L2 Partner default against L1 default, then L2 Partner with voids
against L2 Partner default, on the same 100 fresh deals at bid 30. Both use
Fixed root/model search throughout. Its live progress is in
[level status](campaigns/default-partner-battery/01-level/STATUS.md) and
[void status](campaigns/default-partner-battery/02-voids/STATUS.md).
