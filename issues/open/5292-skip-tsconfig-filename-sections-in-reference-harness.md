---
id: 5292
title: "Skip tsconfig @Filename sections in reference harness"
type: feature
area: compiler/multi-section
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Handle TypeScript reference tests whose `// @Filename:` sections include a
`tsconfig.json` config file before executable `.js` / `.ts` sections.

## Problem

`commonJsIsolatedModules.ts` starts with a virtual `tsconfig.json` section:

```ts
// @Filename: tsconfig.json
{
  "compilerOptions": {
    "allowJs": true,
    "outDir": "foo",
    "isolatedModules": true,
  }
}

// @Filename: index.js
module.exports = {}
var x = 1
```

`split_file_name_sections` finds both sections, but `build_multi_section_file`
currently sends every section through `lower_source_as_module_body`. The JSON
config body is therefore tokenized and parsed as TypeScript/JavaScript source.
Focused coverage reports `UnsupportedSyntax`, and the AST/resolved dump fails
when parsing resumes at the `index.js` `module` token after the JSON block.

Problem: reference-style `tsconfig.json` virtual sections are treated as module
bodies instead of config metadata or non-code sections.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonJsIsolatedModules.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonJsIsolatedModules.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
coverage: build_pass=0, unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
ast/resolved: unsupported expression ... RightBrace ... at 173..179
```

## Desired final state

The multi-section compiler path recognizes `tsconfig.json` sections as reference
test config metadata and does not parse them as executable module bodies. The
representative fixture should advance past the JSON config section; any later
CommonJS `module.exports` limitation must be reported as its own blocker.

## Scope

In scope:

- [ ] In `build_multi_section_file`, do not pass a `tsconfig.json` section to
      `lower_source_as_module_body`.
- [ ] Add one focused regression fixture with `tsconfig.json` followed by
      `index.js`.

Out of scope:

- Full TypeScript `tsconfig.json` semantics.
- JSON module import / `resolveJsonModule` support.
- CommonJS `module.exports` lowering.
- Import resolution between virtual sections; tracked by issue 5229.

## Affected paths

Expected:

- `crates/compiler/src/lib.rs`
- focused compiler tests or fixtures

Do not touch:

- `crates/backend-wasm/`
- package resolution
- broad CommonJS runtime support

## Acceptance criteria

- [ ] `commonJsIsolatedModules.ts` no longer reports `UnsupportedSyntax` from parsing the `tsconfig.json` body as source.
- [ ] A focused compiler test proves a `// @Filename: tsconfig.json` section is skipped before a `.js` section.
- [ ] If `module.exports` remains unsupported after the skip, the diagnostic points at the `index.js` section and does not mention the JSON boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler -E 'test(filename) or test(multi) or test(tsconfig)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonJsIsolatedModules.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonJsIsolatedModules.ts --detail --no-dashboard-data
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from generated bucket
`issues/done/1385-implement-commonJsIsolatedModules.md`.
Also owns the `tsconfig.json` subset of
`issues/done/3380-implement-moduleResolutionWithExtensions-import-export.md`:
fresh triage for `moduleResolutionWithExtensions_withPaths.ts` stops in the
virtual `/tsconfig.json` section at the first JSON property colon before path
mapping or extension resolution diagnostics become actionable.

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

- Skipping the config section may reveal a later CommonJS `module.exports`
  lowering blocker in the same reference path.
