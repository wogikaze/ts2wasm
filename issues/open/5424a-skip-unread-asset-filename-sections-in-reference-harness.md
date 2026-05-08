---
id: 5424a
title: "Skip unread asset @Filename sections in reference harness"
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
virtual asset or dependency files that are deliberately not read by module
resolution.

## Problem

`moduleResolutionWithExtensions_unexpected.ts` and
`moduleResolutionWithExtensions_unexpected2.ts` contain dependency fixture
files whose content is deliberately invalid:

```ts
// @Filename: /node_modules/foo/foo.js
This file is not read.

// @Filename: /node_modules/foo/package.json
{ "types": "foo.js" }

// @Filename: /a.ts
import "foo";
```

The reference multi-section compiler path currently tokenizes and parses the
unread dependency file body as executable source before the package metadata
or `/a.ts` import section can be triaged.

Problem: fixture-only asset sections such as `.js` or `.css` files that should
not be reached by package resolution are parsed as module bodies, hiding the
real package-field extension and import-resolution behavior.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_unexpected2.ts
```

Observed result:

```text
UnsupportedSyntax: expected Semicolon, got Some(Ident("file")) at 5..9
```

Source context:

```text
// @Filename: /node_modules/foo/foo.js
This file is not read.
```

Compiler evidence:

```text
tokens: Ident("This"), Ident("file"), Ident("is"), Ident("not"), Ident("read"), Dot
ast/resolved: fail before the later package.json body or import "foo";
TypeScript AST top-level includes separate ExpressionStatement nodes for
`This`, `file`, `is`, `not`, and `read.`, then continues to the package.json
Block and ImportDeclaration.
```

The sibling `moduleResolutionWithExtensions_unexpected.ts` shows the same
observed result:

```text
UnsupportedSyntax: expected Semicolon, got Some(Ident("file")) at 5..9
```

Source context:

```text
// @Filename: /node_modules/normalize.css/normalize.css
This file is not read.
```

## Desired final state

The reference multi-section compiler path does not parse virtual dependency
asset sections that are intentionally unread fixtures for package-resolution
tests. The two `moduleResolutionWithExtensions_unexpected*` references should
advance past the `This file is not read.` body; any later package metadata,
package field, or bare import behavior must be reported as a narrower blocker.

## Scope

In scope:

- [ ] Identify virtual `@Filename` sections that are fixture assets rather
      than active root/code sections for the current reference test.
- [ ] Skip the unread `.js`/`.css` fixture bodies in the multi-section builder
      or otherwise prevent them from becoming executable module bodies before
      they are resolved as dependencies.
- [ ] Add focused regression coverage with an unread dependency asset section
      followed by a real code section.
- [ ] Re-triage both `moduleResolutionWithExtensions_unexpected*` references
      and record the next diagnostic.

Out of scope:

- Full package.json semantics, covered by issue 5402.
- tsconfig.json handling, covered by issue 5292.
- Markdown fixture sections, covered by issue 5422.
- General local import resolution between `@Filename` sections, covered by
  issue 5229.
- Package field extension filtering or bare package resolution after the
  unread section is skipped.

## Affected paths

Expected:

- `crates/compiler/src/lib.rs`
- focused compiler tests or fixtures

Do not touch:

- backend/runtime emit
- broad package resolution implementation

## Acceptance criteria

- [ ] `moduleResolutionWithExtensions_unexpected2.ts` no longer reports
      `UnsupportedSyntax` from parsing `/node_modules/foo/foo.js` text
      `This file is not read.`
- [ ] `moduleResolutionWithExtensions_unexpected.ts` no longer reports
      `UnsupportedSyntax` from parsing
      `/node_modules/normalize.css/normalize.css` text
      `This file is not read.`
- [ ] A focused compiler test proves an unread dependency fixture section is
      not parsed before the active code section.
- [ ] If the next blocker is `package.json`, the diagnostic points at the JSON
      metadata body and issue 5402 remains the owner.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler -E 'test(filename) or test(multi) or test(package)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_unexpected2.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_unexpected.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_unexpected --detail --no-dashboard-data
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

Split from `issues/done/3381-implement-moduleResolutionWithExtensions-module-resolution.md`.

Related but not duplicates:

- `issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md`
  covers the later virtual `package.json` metadata bodies in the same
  references.
- `issues/open/5422a-skip-markdown-filename-sections-in-reference-harness.md`
  covers markdown sections.
- `issues/open/5229a-resolve-imports-between-filename-sections.md` covers
  imports that should resolve to reachable virtual code sections.

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
