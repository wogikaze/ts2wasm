---
id: 1126
title: "Implement Chainedcallswithtypeparameterconstrainedtoothertypeparameter"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5221]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1126.

## Summary

Triage chainedCallsWithTypeParameterConstrainedToOtherTypeParameter across 2 failing reference test cases and fold the current blocker into an implementation-ready issue.

## Problem

Reference test results showed 2 cases failing in directory `chainedCallsWithTypeParameterConstrainedToOtherTypeParameter` with diagnostics: parser-syntax. Fresh triage shows both paths parse and build AST successfully, then lowering stops at the existing `issue-211` call-expression method receiver boundary.

Problem: chainedCallsWithTypeParameterConstrainedToOtherTypeParameter has 2 reference failures whose actionable blocker is now tracked by `issues/open/5221-support-bitwise-and-xor-binary-lowering.md`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/chainedCallsWithTypeParameterConstrainedToOtherTypeParameter.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/chainedCallsWithTypeParameterConstrainedToOtherTypeParameter.ts --detail
```

## Desired final state

This generated bucket is superseded by `issues/open/5221-support-bitwise-and-xor-binary-lowering.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split the current observable blocker into issue 5221
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this issue and issue 5221

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Issue 5221 contains exact `mise run reference-triage -- ...` commands
- [x] Issue 5221 includes failing paths, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5221 acceptance names the exact reference paths and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/chainedCallsWithTypeParameterConstrainedToOtherTypeParameter.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/chainedCallsWithTypeParameterConstrainedToOtherTypeParameter.ts
```

Not run:

- `cargo fmt --all --check`; issue triage only, no Rust code changed
- `cargo nextest run`; issue triage only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5221-support-bitwise-and-xor-binary-lowering.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/chainedCallsWithTypeParameterConstrainedToOtherTypeParameter.ts`
- `reference/typescript/tests/cases/compiler/chainedCallsWithTypeParameterConstrainedToOtherTypeParameter2.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-06:

- command: `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/chainedCallsWithTypeParameterConstrainedToOtherTypeParameter.ts`
- issue class: `triage-needed`
- feature label: `method-call`
- diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- message: `issue-211: method then requires an identifier receiver at 323..408`
- child issue: `issues/open/5221-support-bitwise-and-xor-binary-lowering.md`

Source context:

```text
20 | // Ok to go down the chain, but error to try to climb back up
21 | (new Chain(new A)).then(a => new B).then(b => new C).then(c => new B).then(b => new A);
```

Generated on 2026-05-06:

- command: `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/chainedCallsWithTypeParameterConstrainedToOtherTypeParameter2.ts`
- issue class: `triage-needed`
- feature label: `method-call`
- diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- message: `issue-211: method then requires an identifier receiver at 257..298`
- child issue: `issues/open/5221-support-bitwise-and-xor-binary-lowering.md`

Source context:

```text
 8 |         // Ok to go down the chain, but error to climb up the chain
 9 |         (new Chain(t)).then(tt => s).then(ss => t);
10 |
11 |         // But error to try to climb up the chain
12 |         (new Chain(s)).then(ss => t);
```

Compiler evidence:

```text
tokens: ok
AST: nested Call(Member(Call(...), property="then"), args=[ArrowFn ...]) chains
resolved/lowered: issue-211 method `then` requires an identifier receiver
TypeScript oracle: emits type diagnostics; no parser blocker
```

## Completion evidence

Closed as a generated triage bucket. The actionable chained method receiver
blocker is tracked by child issue 5221.

Commits:

- this fold commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/chainedCallsWithTypeParameterConstrainedToOtherTypeParameter.ts
result: fail with issue-211 method `then` call-expression receiver diagnostic; split to issue 5221
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/chainedCallsWithTypeParameterConstrainedToOtherTypeParameter2.ts
result: fail with issue-211 method `then` call-expression receiver diagnostic; split to issue 5221
date: 2026-05-06
```

Remaining risks:

- After issue 5217 is implemented, these paths may expose TypeScript type diagnostics or class/generic semantics outside this generated bucket.
