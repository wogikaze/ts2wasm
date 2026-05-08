---
id: 5405
title: "Bind require fs local method calls"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Track `const fs = require("fs")` as a builtin module local so later
`fs.readFileSync(...)` calls use the existing `require("fs").readFileSync`
builtin path instead of generic issue-211 method receiver lowering.

## Problem

Problem: `ambientRequireFunction.ts` currently reaches issue-211 unknown
receiver class for `fs.readFileSync("/a/b/c")`.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ambientRequireFunction.ts
```

```text
source: const fs = require("fs"); const text = fs.readFileSync("/a/b/c");
coverage: unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=ambient-declaration:1
triage: issue-211 unknown receiver class for method `readFileSync`
tokens/ast: ok through declare require, declare module "fs", require local, and fs.readFileSync call
```

## Desired final state

The builtin resolver recognizes `fs` as a local alias for `require("fs")` and
routes `fs.readFileSync(...)` to the builtin module call path, or advances to a
narrower source-spanned arity/path diagnostic for the specific call arguments.

## Scope

In scope:

- [ ] Record `const fs = require("fs")` as a builtin module local.
- [ ] Resolve `fs.readFileSync(...)` through existing `fs` builtin method handling.
- [ ] Re-run `ambientRequireFunction.ts` and record any next blocker.

Out of scope:

- General CommonJS module loading.
- Package resolution for non-builtin modules.
- Full Node `fs` compatibility beyond existing builtin method policy.

## Affected paths

Expected: `crates/ir/src/builtin_resolver.rs`,
`crates/ir/src/builtin_resolver_host.rs`, focused IR/CLI tests.

## Acceptance criteria

- [ ] `ambientRequireFunction.ts` no longer reports issue-211 for `readFileSync`.
- [ ] A focused regression covers `const fs = require("fs"); fs.readFileSync("/a/b/c");`.
- [ ] Direct `require("fs").readFileSync(...)` behavior remains unchanged.

## Validation

Required:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(builtin) or test(require) or test(read_file)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ambientRequireFunction.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/ambientRequireFunction.ts --detail --no-dashboard-data
```

## Docs / current-state / issue sync

Final-state docs: not affected.
Current state: not affected.
Follow-up issues: none.

## Notes

Split from `issues/open/622-implement-ambientRequireFunction.md` on
2026-05-08.

## Completion evidence

Fill only when implemented.
