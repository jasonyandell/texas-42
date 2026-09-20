# CPU speedups deployed to the phone — 2026-09-20

**Deployed and verified; exploratory engineering evidence.**
[Play Plunge](https://plunge.jasonyandell.workers.dev/).
App `65f8f68b3d7d460a62abcfcc1ed00bc201331c6c` ships shared source
`1dfd0e22f5fe2a2ee266f166493442547ad33834`. The manifest and build identities are in
[the release receipt](receipts/cpu-live-release-v1/release.json).

Texas 42 main was fast-forwarded through the CPU integration and prior Kiln work;
the CPU and partnership worktrees follow that main. Main was merged into the
separate GPU response-ladder branch, preserving its experimental implementation.
The original `walt-gran` checkout and its untracked experiments were untouched.

Plunge now imports `walt-player` with explicit portable `cpu-speedups`, no default
features and no native parallelism. The importer builds in a fresh target
location, verifies committed source before/after compilation, clears inherited
native compiler flags, and records its feature/profile/compiler/source identity.
Its corrected source digest includes embedded Scheme inputs. Thin LTO and one
codegen unit are explicit; debug information is stripped. Binary and manifest
verification guards the existing two-import browser ABI. Four importer guard
tests run in both app workflows. Rollback is the focused Plunge release commit.

Validation passed: 120 optimized Rust tests (one pre-existing receipt-emission
test ignored), five player tests, 40 reference tests, exhaustive completed-trick
table validation, native/WASM play and auction parity, forced deadlines and
completed-stage retention, 112 exact live-wrapper comparisons on four complete
reference trajectories, 201 app tests, typecheck/build and deployment dry-run.
Both GitHub workflows succeeded. Production version and served WASM SHA were
verified from Chrome after deployment.

The hosted browser completed a hand, preserved original scores, ran Think Deeper,
saved a question locally with upload disabled, reloaded and advanced to the next
hand without JavaScript errors or horizontal overflow. The observed opening took
265 ms; its later deeper inspection took 209 ms. These are Mac Chrome timings,
not Pixel measurements or a controlled cross-device benchmark. A 4x-throttled
renderer check covered deeper solving, a forced deadline, worker termination
with no late messages, and a pre-bid-book shared link. Its WASM linear memory grew
from about 6.7 MB to 22.3 MB for that one deeper comparison; this is not whole
browser memory.

The final artifact is 6,265,946 raw bytes, about 403 KB under local gzip. Production
responded with zstd encoding. Service-worker timing fields do not provide a
trustworthy compressed-byte count here. Offline reload, an actual cached
160-world solve, and empirical catalogue bidding all passed. See
[offline verification](receipts/cpu-live-release-v1/hosted-offline.json).

The bid catalogue and bidding logic are byte-identical to the previous release.
Opening160/ordinary40, the partner budget, and deadlines retain their settings.
Faster execution can finish more stages/review work under an unchanged deadline;
completed fixed comparisons matching does not imply every timed move is identical.
The catalogue continues to describe its original measured player. No new policy,
strength claim, or higher-bid calibration is inferred from these speedups.

Full raw logs, browser receipts/screenshots, final and prior WASM artifacts live
outside the worktrees at
`/Users/jason/data/texas-42/releases/cpu-speedup-65f8f68/`, with file hashes.
Compact source-side evidence is [checked in here](receipts/cpu-live-release-v1/).
The original [integration plan](CPU-RELEASE-PLAN.md) remains for the audit trail.
