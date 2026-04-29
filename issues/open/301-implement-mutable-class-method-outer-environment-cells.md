---
id: 301
title: "Implement mutable class-method outer environment cells"
type: feature
area: frontend/ir/runtime
class: implementation-ready
priority: P2
depends_on: []
blocks: ["289", "292"]
created: 2026-04-29
updated: 2026-04-29
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

- [ ] Define the mutable environment cell representation for class-method outer locals.
- [ ] Lower outer local reads and writes through the shared cell when a class method mutates that local.
- [ ] Preserve existing immutable hidden-parameter class-method captures.
- [ ] Add a Node/iwasm fixture for `callCount`-style mutation.

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

- [ ] A focused fixture equivalent to the reduced `callCount` class method mutation prints `1` under Node and iwasm.
- [ ] The issue-289 mutable capture diagnostic is removed or narrowed to forms still outside the implemented cell representation.
- [ ] Existing immutable class-method capture fixture remains Node/iwasm matching.
- [ ] Reference coverage for `language/expressions/class/dstr/meth-ary-name-iter-val.js` no longer stops on `callCount` environment mutation when earlier prerequisites are satisfied.

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

- [ ] not affected
- [ ] updated: `docs/...`

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none
- [ ] created/updated: `issues/open/...`

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

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
