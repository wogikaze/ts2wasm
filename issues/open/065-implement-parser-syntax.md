---
id: 065
title: "Implement parser syntax extensions (dup)"
type: feature
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-05-05
completed: 2026-04-29
status: done
---

## Summary

Implement parser-syntax feature to handle 52 failing test cases in reference tests.

Problem: This duplicates the parser syntax epic in issue 059 and should not compete as a separate Ready item.

Queue design note:

- Do not select this issue directly.
- Superseded by issue 059. Useful affected-test evidence was copied into issue 059 on 2026-04-29.

## Problem

Reference test results show 52 cases fail with parser-syntax diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

parser-syntax feature is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

Original in-scope implementation work, now superseded by issue 059:

- Add required syntax to lexer/parser
- Implement semantics for parser-syntax feature
- Add fixtures for parser-syntax feature behavior
- Update diagnostics appropriately

Out of scope:

- Related features (separate issues)

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Original acceptance criteria

- parser-syntax feature passes for basic cases
- Related diagnostics reduced in reference tests
- Regression test added for parser-syntax feature
- Docs updated if semantics change

These implementation criteria were not completed in issue 065 because the issue was
closed as a duplicate of issue 059. Parser syntax implementation work remains
tracked there.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 104
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- not affected

Current state:

- not updated by this superseded close

Follow-up issues:

- issue 059 remains the canonical parser syntax epic

## Notes

## Affected test files

- `reference/test262/test/annexB/built-ins/String/prototype/big/B.2.3.3.js`
- `reference/test262/test/annexB/built-ins/String/prototype/blink/B.2.3.4.js`
- `reference/test262/test/annexB/built-ins/String/prototype/bold/B.2.3.5.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fixed/B.2.3.6.js`
- `reference/test262/test/annexB/built-ins/String/prototype/italics/B.2.3.9.js`
- `reference/test262/test/annexB/built-ins/String/prototype/small/B.2.3.11.js`
- `reference/test262/test/annexB/built-ins/String/prototype/strike/B.2.3.12.js`
- `reference/test262/test/annexB/built-ins/String/prototype/sub/B.2.3.13.js`
- `reference/test262/test/annexB/built-ins/String/prototype/substr/start-and-length-as-numbers.js`
- `reference/test262/test/annexB/built-ins/String/prototype/substr/start-negative.js`
- ... and 42 more files

## Completion evidence

Closed as superseded by issue 059. No parser implementation was performed in this cleanup.

Commits:

- closing commit on branch `agent/061a-065a-issue-dedupe-20260428T233550Z` (hash recorded in cycle report)

Validation result:

```text
command: mise run update-issue-index; mise run update-issue-index -- --check; mise run check issues; mise run check issue-index
result: index update/check passed; issue-health commands returned nonzero only for unrelated pre-existing missing reports in issues 052 and 228
date: 2026-04-29
```

Remaining risks:

- Parser syntax behavior remains tracked by issue 059 and its child slices.

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: superseded`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/065-implement-parser-syntax.md` before this move
- `issues/open/065-implement-parser-syntax.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.

## False-done audit correction

Date: 2026-05-05

Classification: truly-done duplicate/superseded tracking issue.

Audit result: returned to `issues/done/` after review feedback. The title contains `(dup)`, which marks this issue as a duplicate/superseded tracker; duplicate closure issues must not remain in `issues/open/` unless the duplicate mapping itself is invalid.

Evidence files:
- `issues/open/065-implement-parser-syntax.md` after this correction
- `issues/index.md` after regeneration
