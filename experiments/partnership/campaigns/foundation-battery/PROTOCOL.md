# Foundation battery — predeclared matched comparisons

2026-09-06. EXPLORATORY, fixed bid 30, no auction. Freeze each manifest before
execution. Random panel: seeds 720600–720649, 50 complete deals; all random
matchups use the same physical deals and contracts. Each deal is played twice
with policy teams swapped. A pair score is make(A)-make(B); points never break
ties. No outcome-based early stopping. The pool pauses a campaign on repeated
worker failures or >5% fallbacks for either player after 20 nonforced moves.
These technical stops limit what can be inferred from an incomplete panel.

1. l1-race versus phone: establish the faithful native L1 anchor in full play.
2. l1-race versus l1-fixed: assess the original simplification.
3. l1-refine versus l1-race: compare the two established selection schedules.
4. partner-race versus l1-race: add partner L1 modeling with shared selection.
5. l1-race-voids versus l1-race: add public-void constraints to inner beliefs.
6. partner-race-voids versus l1-race-voids: add partner modeling on that belief.

All native profiles use n=40, n0=8, n1=2 where relevant, with a 14-second
wrapper allowance, six native threads per game, and a ten-game shared pool.
The internal modeled L1 mind uses its own deterministic two-world base budget,
with the same race/refinement rule as the real player's forty-world base.
Phone remains the exact archived WASM at 40/8 with race enabled.

Secondary conditional panel: l1-race-voids versus l1-race on seeds 820600–820649,
five focal opening hands with ten hidden completions each. This is five focal
hand units, not fifty independent opening hands. Publish strength aggregates
only for complete pairs/groups in planned order. Report make/set, latency,
and fallback counts separately. No larger strength campaign is authorized by
this protocol without an explicit recorded extension; tuning after these
results uses a separately identified development or fresh panel.

Execution uses foreground slices beneath the 295-second process-group cap.
After a technical stop, diagnose the limiting component before defining any
new configuration or rerunning; never silently increase budgets or reroll a
completed outcome. The baseline anchor is preselected by its recovered phone
semantics, not by choosing the winner of this battery.
