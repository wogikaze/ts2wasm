---
id: 047
title: "Implement super keyword (dup)"
type: feature
area: runtime/semantics
class: done
priority: P1
depends_on: [045, 046]
blocks: []
created: 2026-04-26
updated: 2026-04-28
completed: 2026-04-28
---

## Summary

Implement `super` keyword for accessing parent class members.

## Problem

The `super` keyword is not implemented. It is essential for calling parent constructors and methods.

## Desired final state

`super()` calls parent constructor, `super.method()` calls parent method.

## Scope

In scope:

- [x] Add super to lexer/parser
- [x] Implement super constructor call
- [x] Implement super method call
- [x] Add fixtures for super behavior

Out of scope:

- none

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/backend-wasm/src/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] super parses correctly
- [x] super() calls parent constructor
- [x] super.method() calls parent method
- [x] Fixtures cover super behavior
- [x] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/super-test.ts -o /tmp/test.wasm
iwasm /tmp/test.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Requires class and extends implementation.

## Completion evidence

Commits:

- Close commit recorded by branch HEAD / parent event.

Validation result:

```text
command: node fixtures/classes-and-inheritance/class-super.ts
result: pass; stdout `9`, proving the fixture's Node reference behavior for `super(...)`
date: 2026-04-28

command: cargo run -q -p ts2wasm-cli -- build fixtures/classes-and-inheritance/class-super.ts -o /tmp/ts2wasm-047-class-super.wasm && iwasm /tmp/ts2wasm-047-class-super.wasm
result: pass; iwasm stdout `9`, matching Node for parent constructor dispatch through `super(...)`
date: 2026-04-28

command: node fixtures/classes-and-inheritance/class-super-method.ts
result: pass; stdout `4`, proving the fixture's Node reference behavior for `super.method(...)`
date: 2026-04-28

command: cargo run -q -p ts2wasm-cli -- build fixtures/classes-and-inheritance/class-super-method.ts -o /tmp/ts2wasm-047-class-super-method.wasm && iwasm /tmp/ts2wasm-047-class-super-method.wasm
result: pass; iwasm stdout `4`, matching Node for parent method dispatch through `super.method(...)`
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli class_super_fixtures_match_node_output_under_iwasm
result: pass; 1 passed, covering Node/iwasm differential for both super fixtures
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli super
result: pass; 3 passed, including super build-smoke tests and Node/iwasm differential coverage
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli class
result: pass; 15 passed, including class super semantic coverage
date: 2026-04-28

command: cargo fmt --all --check
result: pass
date: 2026-04-28

command: mise run check-agent-state
result: pass
date: 2026-04-28

command: mise run update-issue-index -- --check
result: pass; issues/index.md up to date after moving 047 to done
date: 2026-04-28

command: mise run check-issue-index
result: pass
date: 2026-04-28

command: mise run check-issue-health
result: pass
date: 2026-04-28

command: mise run check-repo-smoke
result: pass
date: 2026-04-28

command: cargo nextest run
result: pass; 255 passed, 4 skipped
date: 2026-04-28
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/047-implement-super-keyword.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
