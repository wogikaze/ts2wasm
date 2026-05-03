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
- [ ] rest binding の complete lowering
- [ ] default initializer の lowering
- [ ] 入れ子 destructuring の完全対応
- [ ] issue-251/247 系の整理

Out of scope:
- [ ] 任意の iterator protocol

## Affected paths

Expected:
- `crates/ir/src/`

## Acceptance criteria

- [ ] rest binding fixture の lowering が通る
- [ ] default value fixture の lowering が通る
- [ ] 入れ子 destructuring fixture の lowering が通る

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```

---

## ⚠️ False-done audit (re-opened from issues/done/)

**Why this was false-done**: This `implementation-ready` issue was found in `issues/done/` with all 11 checkboxes unchecked (5 scope, 3 acceptance criteria, 3 docs/state), no completion evidence section, no close note, and zero git commits referencing #5049. There is no evidence of any implementation work. The issue was likely created as part of a review-derived batch and moved to done/ prematurely without any code changes.

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
- [ ] not affected

Current state:
- [ ] not affected

Follow-up issues:
- [ ] none
