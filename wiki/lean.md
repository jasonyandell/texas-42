# lean — the Kernel Mechanization

[Home](Home.md) · owns: the `lean/` library as an artifact — its module layout,
the `decide`-and-kernel discipline, how to build it, and how to extend it ·
Sources: [lean/README.md](../lean/README.md), [lean/PROOFS.md](../lean/PROOFS.md);
**v0.7** `60_PROOF_ASSISTANT_HANDOFF.md` + `65_MECHANIZATION_LEDGER.md`; **rec**
`60_PROOF_ASSISTANT_KERNEL.md`. Related:
[proof-assistant-plan](proof-assistant-plan.md),
[lean-row-index](lean-row-index.md), [rob](rob.md).

[`lean/`](../lean/README.md) is a Lean 4 library, depending on mathlib4, that
proves the Texas 42 foundations *inside a proof kernel*. Where every other layer
of this repository produces evidence, this one produces theorems: a Lean
declaration that builds is checked by a small trusted kernel against a fixed set
of axioms, and no amount of prose, testing, or external agreement substitutes
for that.

Two pages divide the topic. [proof-assistant-plan](proof-assistant-plan.md) owns
the *plan* — the trust boundary, the K0–K15 dependency spine, the mechanization
priorities, the acceptance standard, and the scoreboard. This page owns the
*artifact* — what is in the directory, what discipline governs code that goes
there, and how to build and extend it. [lean-row-index](lean-row-index.md) owns
the row-by-row map from ledger obligations to Lean declarations.

**Status (2026-08-02): all 42 priority-0 rows of the mechanization ledger are
kernel-proved** — no `sorry`, no `native_decide`, standard axioms only. The
first-release target of the ledger is closed. The scoreboard and what comes next
are owned by [proof-assistant-plan](proof-assistant-plan.md).

## Where Lean sits in the evidence hierarchy

The kernel is the **second** tier on
[Home](Home.md#evidentiary-tiers--never-promoted-never-blurred), stated there as:

> 2. **Proof-assistant kernel** — the target tier; external `PASS` is never
>    imported as an axiom (TRUST-01). First theorems landed in `lean/`.

Above it sit only the corpus statuses — the ingest packages' own labels — which
are ground truth by definition. Below it sit exchange-adjudicated CONFIRMED
results and then [rob](rob.md)'s conformance receipts.

That ordering has a consequence people find counterintuitive, so it is worth
stating plainly: **Lean does not rank below rob because rob is more thoroughly
tested, and it does not outrank rob by being newer.** The tiers order *kinds of
evidence*, not amounts of it. A kernel proof is a mechanically checked
derivation from stated axioms; a receipt is a report that a program agreed with
an expected string. The former can be wrong only if the definitions do not say
what we think they say; the latter can be wrong in many more ways.

Which is exactly why the discipline below fixates on definitions.

## The trust boundary, in practice

The formal statement is owned by [proof-assistant-plan](proof-assistant-plan.md)
(TRUST-01). Its working consequence for anyone writing Lean here is a single
prohibition, stated verbatim in [lean/PROOFS.md](../lean/PROOFS.md):

> **Never import an external `PASS` as an axiom** (TRUST-01). External
> receipts (Python verifiers, rob) are evidence for *us*, never premises
> for the kernel.

So a finite fact that rob has verified — a census count, an enumeration, a
witness — does not arrive in Lean by citation. It enters only by direct proof, by
a proved-sound internal decision procedure plus kernel evaluation, or by proved
reflection. If a number is worth having in the kernel, it gets recomputed *by the
kernel*.

## The `decide`-and-kernel discipline

This is the part of `lean/` most worth reading carefully, because it is a real
methodological position and not a style guide.

### The five hard rules

From [lean/PROOFS.md](../lean/PROOFS.md), in its own words:

1. **No `sorry` on the main branch.** In-progress statements live on PR branches
   only. A slice merges when it builds sorry-free.
2. **No `native_decide`, no new axioms.** Every theorem must close over the
   standard axioms only (`propext`, `Classical.choice`, `Quot.sound`). Verify
   with `#print axioms` in a scratch file before committing — this is the
   receipt; record it in the commit message.
3. **Never import an external `PASS` as an axiom** (TRUST-01).
4. **Definitions are the trust surface.** The kernel checks proofs; nothing
   checks that a definition matches the ingest spec except review. Every
   definition carries a docstring citing its ingest source (`Math §x.y`, ledger
   row `PA-xNN`). When two packages disagree, the wiki's resolution
   (`wiki/discrepancies.md`) governs; note it in the docstring.
5. **Statement fidelity over proof convenience.** Do not weaken a statement to
   make it provable without flagging it loudly. If a ledger row can only be
   proved in a weaker form, that is a *finding* — surface it, don't bury it.

Rule 4 is the one that deserves emphasis. A mechanized proof transfers all the
risk to the statement: once the kernel accepts it, the only remaining way to be
wrong is to have proved something other than what the specification meant. That
is why every definition in this library carries a citation to the ingest section
it formalizes, and why the reconciliation in [discrepancies](discrepancies.md)
governs when the two packages disagree. The proofs check themselves; the
definitions are checked by human reading, and by nothing else.

### Why `decide` is allowed and `native_decide` is not

Both tactics discharge a decidable proposition by computing it. The difference is
*who computes*.

`decide` reduces the proposition inside the Lean kernel itself. The kernel is the
small, audited program whose correctness the whole edifice already rests on, so a
`decide` proof adds no new trusted component — it is kernel evaluation, which is
precisely what the trust boundary permits. `native_decide` instead compiles the
proposition to machine code and runs it, then asks the kernel to believe the
result. That imports the Lean compiler, the runtime, and the C toolchain into the
trusted base. The `#print axioms` output tells the difference: a `native_decide`
proof depends on an extra axiom, and this library has none.

The practical rule from the playbook is therefore permissive rather than
grudging:

> Kernel `decide` is a legitimate proof (it *is* kernel evaluation — allowed by
> the trust boundary, unlike `native_decide`). Use it freely for finite-arithmetic
> facts: cardinalities, census counts, witnesses, per-domino case sweeps.

### Where structure is required anyway

Permission to compute is not permission to stop thinking, and the library draws
that line explicitly. The unique trick winner is derived from key injectivity —
a structural argument about shared pips — with `decide` nowhere in the chain,
because the spine demands the *reason* and not merely the fact. The
737,100-case exhaustive agreement with an independent prose resolver stays a
separate, deliberately deferred reflection target rather than being conflated
with the theorem. Likewise, pip transports are proved by the specification's
analytic forcing argument rather than by sweeping permutation space, which does
not kernel-reduce in reasonable time in any case.

### `decide +kernel`, and the cost model

Plain `decide` evaluates the proposition in the *elaborator*, not the kernel —
which is orders of magnitude slower, builds enormous terms, and shares nothing.
`decide +kernel` produces the same proof term with the same axioms, evaluated
where it should be. The measured difference on this library's heaviest
obligation, the 90-world witness with its 180 rollouts and 90 replay checks, is
33 seconds versus an out-of-memory kill after 16 hours.

The playbook's rule is therefore to use `decide +kernel` for anything that
evaluates game machinery. Read it as a rule for new work rather than as a
description of the library: all 35 of its `decide +kernel` uses are in
`Witness.lean`, and the constellation modules evaluate their own game machinery —
a 56,448-case key sweep, and two exact suffix minimax values — under plain
`decide`.

A related trap, recorded because it cost two days: never put a heavy `decide`
inside `refine ⟨…⟩`. Filling a `refine`-created metavariable makes unification
symbolically evaluate the proposition. State the components as top-level lemmas
and assemble them with `exact`.

### The idiom collection

[lean/PROOFS.md](../lean/PROOFS.md) carries roughly twenty idioms discovered
while building this library — how to index a concrete family by `Fin n` rather
than by a membership subtype so the kernel does not stall; why exact rationals
must stay outside `decide` (rational normalisation runs a well-founded `gcd` that
will not kernel-reduce, so heavy moments are computed in integers and lifted
analytically); how to amortise repeated kernel evaluation through verified
tables; why a silent long run should be treated as a hang until proven otherwise.
They are working knowledge, not decoration: most of them exist because their
absence produced a multi-hour failure once. Read that file before starting a new
slice.

## Layout

Sixteen modules under `lean/Texas42/`, about 6,300 lines, all imported by the
root module `lean/Texas42.lean` in dependency order. Only `Basic.lean` and
`ConstellationCore.lean` import mathlib wholesale; everything else imports within
the project.

| Module | Proves |
|---|---|
| `Basic.lean` | The finite algebra: dominoes as canonical high/low pairs, the 28-tile cardinality, the count-point total of 35, and the natural incidence covering. |
| `Trick.lean` | The nine declarations and eight led contexts, called and powered sets, the follow relation, rank, tier, the lexicographic trick key, key injectivity, and the **unique trick winner** — plus contextual BEATS and threat monotonicity. |
| `Transport.lean` | Pip transports: a pip permutation preserves every count label exactly when it is the identity or the 2↔3 swap, and that swap transports contextual game order between the two declarations it exchanges. |
| `Auction.lean` | The Straight auction: bid value embedding, decidable legality, the deterministic transition, and the structural reachable mark ceiling. |
| `Deal.lean` | Ordered deal worlds — four labelled seven-tile disjoint hands, with a computable owner map. |
| `Play.lean` | Contract and contracted play: the reduced play state, the legal-play characterization, the trick-resolving transition and its partition invariant, and conservation — seven tricks, 28 plays, 42 points. |
| `Cells.lean` | The public-record machine, the upper-bound-only void update, the derived capacity cells and the fiber, and **the losslessness theorem** with its four-case completeness induction. |
| `Reachability.lean` | The reachability predicate and a certified state whose equality goes through the projection — reachability carried as proof-irrelevant evidence. |
| `Information.lean` | The counterexample separating physical congruence from information-state equality. |
| `Reduction.lean` | The generic capacitated cell kernel: canonical reduction, fiber-preserving and idempotent, with the coarsest-exact-quotient equivalence; the game cells instantiate it. |
| `NormalForm.lean` | The support normal form: the trichotomy, the strict Hall inequality, a generic capacitated Hall lemma by slot expansion, compile and decode with their inverse laws, and the total form that classifies exact fibers. The longest module, at about 1,450 lines. |
| `Belief.lean` | The finite belief layer over exact rationals: Bayes conditioning with its chain rule, policy kernels, the history likelihood, the posterior, and the pushforward whose support lies inside the cell fiber. |
| `Strategic.lean` | Strategic sufficiency: a finite-horizon viewer decision process, the Bayes filter, and the theorem that expected continuation value is a function of the strategic state. |
| `Witness.lean` | The **90-world posterior-flip witness**, internalized whole. |
| `ConstellationCore.lean` | The constellation thread's self-contained core (x:013): its own domino and declaration algebra, a packed trick key, and a unique-winner theorem. |
| `ConstellationSuffix.lean` | The constellation thread's depth-*k* suffix machine and its fuel-indexed exact minimax (x:015). |

[lean/README.md](../lean/README.md) carries the per-declaration breakdown of each
module; [lean-row-index](lean-row-index.md) maps ledger rows to declarations.

### The 90-world witness, internalized whole

`Witness.lean` deserves separate mention because "internalized whole" is a strong
claim and it means something specific. The
[belief-vs-support](belief-vs-support.md) counterexample — two legal histories
with the same mechanical endpoint, the same 90-world fiber and the same posterior
support, yet opposite optimal leads — is not asserted in Lean from an external
computation. Every piece is built and checked inside the kernel: the endpoint
cells are computed from the public record, the fiber is enumerated and proved
equal to the cells, each of the 90 worlds is realized by a rule-compatible
complete deal that replays the prefix legally, both auction histories are proved
legal and distinct with the same result, the two posteriors are exact rationals
with identical full support, and the value columns come from 180 deterministic
rollouts of the committed play machinery. The final theorem is a seven-way
conjunction of those parts. No external receipt is imported.

### The constellation files are not reconciled

`ConstellationCore.lean` and `ConstellationSuffix.lean` came out of the exchange
Lean thread (x:013, x:015) and are **self-contained**: they re-derive their own
domino and declaration algebra rather than importing the main layers, and they
are not yet reconciled with `Basic.lean` and `Trick.lean`. They carry no ledger
row. They are part of the default build target, so the sorry-free and
axiom-hygiene claims cover them, but they are not part of the priority-0
scoreboard.

They also differ from the main spine one level below their headline theorem. Both
derive the unique winner structurally from key injectivity; but the constellation
core discharges *injectivity itself* by `decide` over every declaration, context
and tile pair, where `Trick.lean` proves it by the shared-pip argument. Both
routes are legitimate under the trust boundary — this is a methodological choice
about how much of the theorem should have a reason rather than a computation, and
a reader comparing the two files should know it is unresolved rather than an
oversight.

## Building

```sh
cd lean
lake exe cache get   # fetch prebuilt mathlib artifacts (multi-GB, one-time)
lake build
```

`elan` must be on `PATH`; it lives in `~/.elan/bin`. The toolchain is pinned by
`lean/lean-toolchain` and the mathlib revision by `lean/lakefile.toml`, with the
resolved commit in `lake-manifest.json`. Build artifacts live in `lean/.lake`,
which is gitignored and runs to some 7.4 GB of mathlib artifacts.

Three practical notes:

- **The first build is slow and the cache step is not optional in practice.**
  Without `lake exe cache get`, `lake build` will compile mathlib from source.
- **In a fresh worktree, clone the artifacts instead of refetching.** On APFS,
  `cp -Rc <main>/lean/.lake <worktree>/lean/.lake` is instant and copy-on-write.
  A worktree that has never been built has no `.lake` at all.
- **Never run two `lake build`s in the same directory concurrently.** Once the
  cache is warm, a full incremental build of this library is 20–60 seconds;
  budget for that loop rather than for interactive elaboration.

There is **no CI gate for Lean**. The workflow files under `lean/.github/` came
with the mathlib template and are inert, because GitHub only runs workflows from
a repository root and `lean/` is a subdirectory; they are kept against a possible
future split into a standalone repository. There is no `.github/` at the
repository root at all, and the gate scripts that do exist — `rob/ci/check.sh`
and `walt/ci/check.sh` — are Rust gates that never touch Lean. `lake build` is
run by hand.

## Adding a proof

The per-slice checklist from [lean/PROOFS.md](../lean/PROOFS.md), with the
mechanics filled in.

1. **Read the ingest sections being formalized** — both packages if they overlap.
   [discrepancies](discrepancies.md) resolves conflicts; do not resolve them in
   the Lean file.
2. **Create `lean/Texas42/<Name>.lean`** with the house preamble: the copyright
   block, an import of the nearest layer (not mathlib wholesale unless you truly
   need it), a module docstring naming the Math sections and ledger rows the file
   covers, and a `namespace Texas42`.
3. **Register it** by adding `import Texas42.<Name>` to `lean/Texas42.lean` in
   dependency order. No lakefile edit is needed — the library target picks up any
   module reachable from the root import list.
4. **Land the definitions first, with citations** (hard rule 4), keeping them
   minimal: derived views are *functions*, not structure fields. If you will
   `decide` over a match-defined predicate, write its `Decidable` instances
   per-constructor rather than hoping for derivation through the match.
5. **Prove the row's theorems**, each docstring naming its `PA-` row. Use kernel
   `decide` freely for finite arithmetic and `decide +kernel` for anything that
   evaluates game machinery.
6. **Build clean and check axioms.** `lake build`, then `#print axioms` on every
   new theorem in a scratch file. The expected output is exactly
   `[propext, Classical.choice, Quot.sound]`.
7. **Update the two doc surfaces** — the layout list in
   [lean/README.md](../lean/README.md) and the status in
   [proof-assistant-plan](proof-assistant-plan.md) — and commit with the axiom
   receipt in the message, in the house format: a
   `Receipts (#print axioms, all [propext, Classical.choice, Quot.sound]):`
   block naming every new theorem, followed by `No sorry, no native_decide.`

A proof *counts* only when all of that holds: on main, sorry-free,
`native_decide`-free, closing over exactly the three standard axioms with the
receipt recorded, definitions cited, and the statement not quietly weakened to
make it go through.

### How axiom hygiene is actually enforced

Honestly: **by convention and commit-message receipt, not by machine.** There is
no script, no CI check, and no committed `#print axioms` output. A grep for
`sorry` and `native_decide` across the library returns nothing today, and that
half is cheap for any reader to re-verify; the axiom half cannot be verified
without running Lean. This is a real gap between what the documentation asserts
and what the repository checks, and it is stated here rather than glossed because
the whole point of the tier is mechanical checking.

## What Lean is for

Two roles, and it is worth separating them.

**The independent verification path.** Lean is the route by which a finite claim
stops being "we ran a program and it agreed" and becomes a theorem. It is
independent because it may not read anyone else's answers — not the ingest
Python verifiers, not exchange programs, not [rob](rob.md). When [walt](walt.md)
needs mechanical verification, its standing no-rescue policy routes it here for
the same reason.

**The place where definitions get pinned down.** The subtler benefit is that
mechanizing a section forces every implicit assumption in the prose into the
open. This library has already produced findings of that kind — the completeness
half of the losslessness induction needs a fact the prose leaves implicit, that a
hidden seat's publicly played tile must respect that seat's previously recorded
voids, which the mechanization had to derive rather than assume. Findings like
that are the reason rule 5 insists that a weakened statement is a result to
report, not an embarrassment to hide.
