# Outer bundle control on frozen v4 lower rungs

Prepared after completed v4 CPU20 development outcomes (4/7/61, candidate
40.519ms mean partnership time versus current Walt 85.982ms), before any
larger-outer-bundle outcomes.

Keep frozen v4 C0/C1 artifacts, compiled focal tail, CPU backend, plans=1,
horizon=7, work=2,000,000 and 20ms move allowance. Change only outer sample
count from 40 to 80 in a complete 72-pair development panel, with the same
compiled-v1-dev001 namespace and six-thread unweakened current Walt anchor.
Use output compiled-h2h-v4-outer80-cpu20. Complete all 144 games regardless
of early outcomes, preserve failures/fallbacks, and independently replay.
Use 55-second cooperative cycles with the existing 60-second watchdog.

The larger bundle changes the finite sampled optimization problem. With the
same deadline the planner may complete a smaller fraction of it; neither a
larger sample nor a smaller reported local gap guarantees stronger play.
Report actual partnership mean and p95, root certifications, interrupted
decisions, full vectors and reserve use alongside paired outcomes. Native
GPU conformance is already established, but this quality control is CPU only.

The earlier 40-world panel is a completed development control, not a fresh
confirmation sample. No fresh final holdout begins here. Any later outer
count or time allowance requires a separately declared control.
