---
id: 5224
title: "Handle package.json virtual sections in multi-file references"
type: feature
area: compiler/reference
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

`APISample_linter.ts` and `APISample_transform.ts` both contain a TypeScript
test-harness virtual file section named `node_modules/typescript/package.json`.
The compiler currently parses that JSON section as TypeScript and stops at the
first object-property colon before it reaches the actual sample source.

## Problem

The multi-file reference splitter treats every `// @filename:` section as
TypeScript source. JSON virtual files are reference harness metadata for module
resolution, not TypeScript statements, so parsing them as source produces a
misleading parser failure.

Problem: `node_modules/typescript/package.json` virtual sections in APISample references report `expected Semicolon, got Some(Colon)` before the real TypeScript section can be triaged.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/APISample_linter.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/APISample_transform.ts
```

Current diagnostic for both files:

```text
UnsupportedSyntax: expected Semicolon, got Some(Colon) at 216..217
```

Representative virtual section:

```ts
// @filename: node_modules/typescript/package.json
{
    "name": "typescript",
    "types": "/.ts/typescript.d.ts"
}
```

TypeScript oracle parses the virtual JSON section as a harness file and reports
TS1005/TS2695 diagnostics for the JSON section instead of a compiler crash or
unclassified parser boundary.

## Desired final state

The compiler recognizes non-TypeScript virtual sections such as `.json` in
multi-file references and does not parse them as TypeScript program bodies.
The APISample representatives should advance past the `package.json` colon
parser boundary to the next import/export or API-sample diagnostic.

## Scope

In scope:

- [ ] Detect `.json` virtual sections produced by `// @filename:` references.
- [ ] Preserve enough section metadata for later module-resolution work, or emit a focused unsupported diagnostic that names the JSON section.
- [ ] Ensure `.ts`, `.tsx`, `.d.ts`, `.js`, and `.jsx` virtual sections keep the existing TypeScript parsing path.
- [ ] Add focused coverage using `node_modules/typescript/package.json` plus a following TypeScript section.
- [ ] Re-run `APISample_linter.ts` and `APISample_transform.ts` triage and record the next narrower blocker.

Out of scope:

- Full package.json-driven module resolution.
- Implementing the TypeScript compiler API samples.
- Broad import/export module loading owned by issue 543 and issue 432.

## Affected paths

Expected:

- `crates/compiler/src/lib.rs`
- `crates/cli/tests/official_corpora.rs`
- `fixtures/` or inline compiler tests

Do not touch:

- `crates/backend-wasm/src/`
- TypeScript compiler API runtime implementation

## Acceptance criteria

- [ ] `APISample_linter.ts` no longer reports `expected Semicolon, got Some(Colon)` for `node_modules/typescript/package.json`.
- [ ] `APISample_transform.ts` no longer reports `expected Semicolon, got Some(Colon)` for `node_modules/typescript/package.json`.
- [ ] Focused coverage proves JSON virtual sections are not parsed as TypeScript source.
- [ ] Follow-up work is represented if triage advances to import/export or TypeScript compiler API runtime gaps.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli --test official_corpora
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/APISample_linter.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/APISample_transform.ts
```

Impacted commands:

```sh
mise run check issue-readiness -- --fail-ready-below 80
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

Split while closing issue 5139. Existing issue 543 owns the broader APISample
import/export bucket, but the current observable blocker for both remaining
parser leftovers is earlier: JSON virtual sections are treated as TypeScript.

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
