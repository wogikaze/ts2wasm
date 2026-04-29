---
id: 250
title: "Design BigInt runtime value support"
type: feature
area: runtime/semantics
class: design-ready
priority: P2
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Define the runtime and IR contract for ECMAScript BigInt values after issue 244 introduced frontend BigInt literal classification.

Problem: BigInt literals now parse as explicit AST nodes, but runtime representation, operations, equality, and builtin behavior remain intentionally unsupported.

## Current failure

Programs that reach runtime-facing phases with a BigInt literal report an issue-linked unsupported diagnostic instead of producing a BigInt value.

Representative command:

```sh
tmp=/tmp/ts2wasm-250-bigint-runtime.ts
printf 'let x = 1n;\nconsole.log(x);\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-250-bigint-runtime.wasm
```

## Desired final state

The compiler has an accepted BigInt value representation and phase contract. A later implementation slice can lower supported BigInt literals and operations without mixing parser classification, IR contracts, runtime ABI, and backend emission in one change.

## Scope

In scope:

- [ ] Decide whether BigInt values use a heap object representation, tagged immediate representation, or hybrid representation.
- [ ] Define the IR/runtime boundary for BigInt literals and basic operations.
- [ ] Define diagnostics for unsupported BigInt operations that remain out of the first implementation slice.
- [ ] Split implementation-ready child issues for literal values, arithmetic/comparison, equality/coercion boundaries, and builtin/string conversion behavior as needed.

Out of scope:

- Parser tokenization or AST classification; completed by issue 244.
- Numeric separator syntax; tracked by issue 243.
- Implementing the full BigInt runtime in this design issue unless the issue is explicitly split or reclassified.

## Affected paths

Expected:

- `docs/`
- `issues/`
- `crates/ir/src/`
- `crates/runtime-abi/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- unrelated parser syntax work
- unrelated runtime builtins

## Acceptance criteria

- [ ] BigInt value representation and ABI boundary are documented.
- [ ] Unsupported and supported BigInt operation boundaries are issue-linked and phase-specific.
- [ ] Implementation-ready child issues exist for the first runtime slices.
- [ ] Docs/current-state/issues are synchronized with the chosen boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
mise run update-issue-index
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo nextest run
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] update runtime/ABI docs with the accepted BigInt contract

Current state:

- [ ] update `current-state.md` if implementation status changes

Follow-up issues:

- [ ] create implementation-ready child issues after the runtime design boundary is accepted

## Notes

Issue 244 intentionally stops at parser classification and stable unsupported diagnostics. This issue owns the runtime design gap exposed by that parser work.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
