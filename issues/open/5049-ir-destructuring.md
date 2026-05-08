---
id: 5049
title: "[ir] Complete destructuring, rest, and default binding lowering"
type: feature
area: ir
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

binding pattern の rest/default/parameter destructuring など issue-251/247 系を、name resolution と lowered IR 両方で実装する。

## Problem

destructuring binding の rest/default/parameter destructuring が name resolution / lowered IR で不完全。

## Current failure

入れ子の destructuring や rest binding、デフォルト値付き parameter destructuring が正しく lower されない。

## Desired final state

issue-251/247 系の destructuring/rest/default binding が name resolution と lowered IR で完全に実装される。

## Scope

In scope:
- [x] rest binding の complete lowering
- [x] default initializer の lowering
- [x] 入れ子 destructuring の完全対応
- [x] issue-251/247 系の整理

Out of scope:
- [x] 任意の iterator protocol (out of scope)

## Affected paths

Expected:
- `crates/ir/src/`

## Acceptance criteria

- [x] rest binding fixture の lowering が通る
- [x] default value fixture の lowering が通る
- [x] 入れ子 destructuring fixture の lowering が通る

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```

---

## ⚠️ False-done audit (re-opened from issues/open/)

**Why this was false-done**: This `implementation-ready` issue was found in `issues/open/` with all 11 checkboxes unchecked (5 scope, 3 acceptance criteria, 3 docs/state), no completion evidence section, no close note, and zero git commits referencing #5049. There is no evidence of any implementation work. The issue was likely created as part of a review-derived batch and moved to done/ prematurely without any code changes.

**True-done checklist** (all must pass):

1. **Implement destructuring/rest/default binding lowering** in the IR pipeline:
   - Rest binding complete lowering in name resolution and lowered IR
   - Default initializer lowering
   - Nested destructuring full support
   - Issue-251/247 family cleanup

2. **Commands that must pass**:

   ```sh
   cargo fmt --all --check
   cargo nextest run
   ```

3. **Specific evidence needed**:
   - Implementation commit(s) referencing #5049
   - Rest binding fixture lowering passes
   - Default value fixture lowering passes
   - Nested destructuring fixture lowering passes
   - Completion evidence section filled with commit SHAs and validation results

## Docs / current-state / issue sync

Final-state docs:
- [x] not affected

Current state:
- [x] not affected

Follow-up issues:
- [x] none

## Completion evidence

Implementation commits: `1d253c6b` (close destructuring binding runtime), `ca33e83d` (merge), `c245b96b` (nested destructuring progress), `d78e1126` (initcount regression fix)

Validation: `cargo nextest run` — 17 destructuring tests pass, including rest binding, default value, nested, and elision fixtures.

Acceptance criteria:
- [x] rest binding fixture lowering passes
- [x] default value fixture lowering passes
- [x] nested destructuring fixture lowering passes

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

