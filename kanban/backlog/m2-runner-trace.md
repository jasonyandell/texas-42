id: [[m2-runner-trace]]
opened: 2026-08-24

## What

walt-m2-runner's LIVE status is inferred, not confirmed (census flag
3): zero doc mentions; ci/check_m2_metal.sh exists but nobody traced
whether it still runs in an active gate. Trace it; either wire it into
the documented CI story or archive the crate with the Metal pair's
receipts intact.

## Done when

m2-runner's gate status is documented where the GPU branch lives, or
the crate is archived with a ledger entry.

## Links

[[walt-unification]], walt/UNIFICATION-CENSUS.md
