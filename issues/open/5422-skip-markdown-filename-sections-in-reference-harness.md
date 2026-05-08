---
id: 5422
title: "Skip markdown @Filename sections in reference harness"
type: feature
area: compiler/multi-section
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Handle TypeScript reference tests whose `// @Filename:` sections include
markdown/non-code files before executable `.ts` sections.

## Problem

`moduleResolution_noLeadingDot.ts` contains:

```ts
// @Filename: /node_modules/@types/.svn/README.md
This is a test.

// @Filename: /a.ts
true;
```

Problem: the multi-section compiler path tokenizes and parses the README body
as TypeScript source, then reports `UnsupportedSyntax: expected Semicolon, got
Some(Ident("is"))`.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_noLeadingDot.ts
```

Current evidence:

```text
tokens: ok; README text tokenizes as identifiers `This is a test`
ast: UnsupportedSyntax expected Semicolon, got Some(Ident("is")) at 106..108
TypeScript oracle: raw-source diagnostics for README text, then parses `true;`
```

## Desired final state

The reference multi-section compiler path recognizes `.md` virtual sections as
non-code fixture files and does not parse them as executable module bodies.
`moduleResolution_noLeadingDot.ts` should advance past the README section; any
later module-resolution behavior must be reported as a narrower blocker.

## Scope

In scope:

- [ ] Skip `.md` / markdown virtual sections in the multi-section builder.
- [ ] Add focused regression coverage with a README.md section followed by a code section.
- [ ] Re-triage `moduleResolution_noLeadingDot.ts` and record the next diagnostic.

Out of scope:

- package.json handling, covered by issue 5402.
- tsconfig.json handling, covered by issue 5292.
- Full module-resolution implementation for `@types` package folders.
- Markdown parsing or markdown module imports.

## Affected paths

Expected:

- `crates/compiler/src/lib.rs`
- focused compiler tests or fixtures

Do not touch:

- backend/runtime emit
- broad package resolution implementation

## Acceptance criteria

- [ ] `moduleResolution_noLeadingDot.ts` no longer reports `expected Semicolon, got Some(Ident("is"))` from parsing README text.
- [ ] A focused compiler test proves a `// @Filename: README.md` section is skipped before a code section.
- [ ] If module resolution remains unsupported after the skip, the diagnostic points at the later module-resolution boundary and not the markdown body.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler -E 'test(filename) or test(multi) or test(markdown)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_noLeadingDot.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolution_noLeadingDot.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
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

Split from `issues/open/3371-implement-moduleResolution-module-resolution.md`.

## Completion evidence

Fill only when moving to `done`.
