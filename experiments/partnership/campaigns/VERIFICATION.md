# Campaign runner verification

2026-09-06. Exploratory engineering checks, not game-theory proofs.

- Four focused campaign tests passed: Boolean-only ranking and bounded paired
  seed outcome; fixed-hand/different-world generation across ten groups;
  atomic all-three-arm completion and source-identity rejection; early stopping
  arithmetic and exclusion of clustered worlds from the random-panel test.
- Existing focused player checks passed again, including seven Rust tests,
  252 native/independent-referee state comparisons, private input validation,
  actual child timeout, malformed output, and fallback rejection. Captured in
  `../runs/016-campaign-checks/`.
- `calibration-t4/caps/slice-01` requested a one-second cooperative pause. All
  three games saved exactly one opening decision, no seed result was committed,
  and the process group finished in 4.654 seconds after current moves finished.
- `calibration-t4/caps/slice-02` resumed those games and completed all three.
  Every saved opening decision (including timing and evaluation details) was
  byte-equivalent as parsed JSON to `pause-proof.json`; it was not recomputed.
- Comparing calibration seed 420601 at four threads with the primary campaign
  at six threads gave identical moves and routes in all three games, with zero
  fallbacks. Summed decision times, four vs six threads: phone 10.775 vs 11.021
  seconds; candidate declaring 14.441 vs 13.651; candidate defending 11.601 vs
  11.328. This is one calibration deal, not a broad speedup claim. Six threads
  were retained for the main panel. This seed uses a twos contract selected by
  the campaign heuristic; the earlier report's fixed sixes fixture differs.

The first main slice committed 13 seeds / 39 games in 270.093 seconds, stayed
inside its 295-second external watchdog, and exited with no detached workers.
The next slice skipped those completed seeds and resumed the pending work.

`calibration-hard-stop/caps/killed` forcibly killed a running seed after two
seconds. No seed result was committed. Restart completed all three games with
exactly the same move/route sequences as the primary campaign's calibration
seed. The original packet watchdog reported `timed_out`, child signal 9, and
return code 125 because its repeated cleanup attempt raised macOS EPERM. A
subsequent process-group inventory confirmed no surviving process in group
80113. The receipt preserves that warning; the original packet is unchanged.
This validates recovery and absence of survivors for this trial, not a blanket
claim that every watchdog cleanup returns without an OS warning.

At the roughly ten-minute review, the primary panel had completed 24 seeds /
72 games. Its 21 fresh seeds yielded two paired wins, four losses, and 36 ties.
That mixed evidence did not justify the contemplated no-observed-gain pause;
the main panel continued with its original automatic stop rules. Periodic
foreground slices are owned by the thread heartbeat automation
`texas-42-make-set-evaluation`, every five minutes, until completion or STOP.
