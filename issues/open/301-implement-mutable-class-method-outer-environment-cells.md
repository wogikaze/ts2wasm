---
id: 301
title: "Implement mutable class-method outer environment cells"
type: feature
area: frontend/ir/runtime
class: done
priority: P2
depends_on: []
blocks: ["289", "292"]
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
status: done
---

## Summary

Class methods can now read immutable lexical outer locals through hidden capture
parameters, but mutation of captured outer locals remains unsupported.

Problem: `callCount = callCount + 1` inside a class method needs a shared
mutable environment cell rather than a by-value hidden parameter.

## Current failure

Reduced source:

```js
var callCount = 0;
class C {
  method() {
    callCount = callCount + 1;
  }
}
new C().method();
console.log(callCount);
```

Current result:

```text
error: [UnsupportedSyntax] issue-289: class method `method` mutates outer local `callCount`; mutable class-method lexical captures require heap environment cell support
```

## Desired final state

Class methods that mutate a supported outer local update the same binding
observed by code outside the method. The reduced source above prints `1` and
does not rely on stale by-value capture semantics.

## Scope

In scope:

- [x] Define the mutable environment cell representation for class-method outer locals.
- [x] Lower outer local reads and writes through the shared cell when a class method mutates that local.
- [x] Preserve existing immutable hidden-parameter class-method captures.
- [x] Add a Node/iwasm fixture for `callCount`-style mutation.

Out of scope:

- `eval` or `with` environment semantics.
- Escaped class expressions whose lexical environment is no longer available unless the cell representation supports them safely.
- Iterator/destructuring semantics beyond the environment mutation needed by issue 289 and issue 292.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`

Do not touch:

- unrelated BigInt runtime work
- unrelated web-ui/report artifacts

## Acceptance criteria

- [x] A focused fixture equivalent to the reduced `callCount` class method mutation prints `1` under Node and iwasm.
- [x] The issue-289 mutable capture diagnostic is removed or narrowed to forms still outside the implemented cell representation.
- [x] Existing immutable class-method capture fixture remains Node/iwasm matching.
- [x] Reference coverage for `language/expressions/class/dstr/meth-ary-name-iter-val.js` no longer stops on `callCount` environment mutation when earlier prerequisites are satisfied.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(class_method) or test(this_receiver_method_fixtures_match_node_output_under_iwasm)'
cargo nextest run
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/expressions/class/dstr/meth-ary-name-iter-val.js
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected
- [x] updated: n/a

Current state:

- [x] not affected
- [x] updated: n/a

Follow-up issues:

- [x] none
- [x] created/updated: n/a

## Notes

Issue 289 implemented immutable class-method capture by appending hidden
parameters at known call sites. Reusing that by-value path for mutable captures
would silently leave the outer binding unchanged, so this issue must introduce a
shared cell or an equivalently correct mutable environment representation.

### Progress note: 2026-04-29 child worker

Implemented a narrow mutable class-method outer-local environment cell path for
the reduced numeric `callCount = callCount + 1` case. Mutably captured class
method locals now lower to `EnvCellNew` / `EnvCellGet` / `EnvCellSet`, while
existing immutable hidden-parameter captures remain on the by-value path.

Evidence:

- `cargo fmt --all --check` passed.
- `cargo nextest run -p ts2wasm-cli -E 'test(class_method) or test(this_receiver_method_fixtures_match_node_output_under_iwasm)'` passed 5 tests.
- Focused Node/iwasm diff for `fixtures/core-semantics/class-method-mutable-outer-capture.ts` matched and printed `1`.
- `mise run update-issue-index -- --check` passed.
- `mise run check issues` passed after restoring the ignored local reference-coverage result artifact.

Remaining before close:

- Prove or harden GC marking for heap values stored inside environment cells.
- Run broader validation, including full `cargo nextest run`, before moving this
  issue to `done`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `f4ac5ec7` (`agent/301-class-env-close-20260429T193520Z`)

Validation result:

```text
date: 2026-04-29

command: cargo fmt --all --check
result: pass

command: cargo nextest run -p ts2wasm-cli -E 'test(class_method) or test(this_receiver_method_fixtures_match_node_output_under_iwasm)'
result: pass, 5 tests passed

command: cargo test -p ts2wasm-backend-wasm env_cells_are_tagged_array_payloads_for_gc_tracing -- --nocapture
result: pass, proves env cells are emitted as tagged one-slot array payloads and traced by existing array GC marking

command: focused Node/iwasm fixture diff for fixtures/core-semantics/class-method-mutable-outer-capture.ts
result: pass, Node stdout `1`, iwasm stdout `1`, diff empty; saved under reports/runs/20260429T192041Z-301-class-env-close/

command: mise run reference-coverage -- test262 --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/expressions/class/dstr/meth-ary-name-iter-val.js --detail
result: unsupported remains `UnsupportedSyntax: class`; the previous callCount mutable environment-cell diagnostic is no longer the stopping point

command: cargo nextest run --no-fail-fast
result: 581 passed, 2 unrelated failures, 4 skipped. Unrelated failures: `ts2wasm-backend-wasm tests::alloc_heap_emits_gc_header_and_trigger_contract` expects stale memory max `(memory ... 2 16)` while current Layout::MEMORY_MAX_PAGES is 185; `ts2wasm-cli::m2_node_diff function_arguments_fixture_matches_node_output_under_iwasm` fails on `fixtures/core-semantics/arguments-object-property-call.ts`, which has no class-method or EnvCell lowering.
```

Remaining risks:

- Full-suite baseline still has the two unrelated failures listed above; issue 301 scoped validation passes.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/301-implement-mutable-class-method-outer-environment-cells.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
