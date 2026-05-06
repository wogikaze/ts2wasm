---
id: 5242
title: "W2: declare JS semantic core workstream complete"
type: docs
area: docs
class: design-ready
priority: P1
depends_on: [5240, 5241]
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

After child issues 5240 (docs audit) and 5241 (number model sentinels) are
complete, verify and declare W2 (JS Semantic Core) complete. Update
`docs/11-shared-definitions.md` gate status, run full Gate B validation,
and update `current-state.md`.

## Problem

W2 (JS semantic core) is defined in `docs/11-shared-definitions.md` as:
"truthiness、`===`、`+`、number/string semantics、operator 優先度、関数呼び出し、
`undefined`/`null` の JS 意味論"

Most W2 items are already implemented and passing Node/iwasm differential tests.
The remaining gaps are:
- Stale docs entries (issue 5240)
- NaN/Infinity/-0 sentinel values (issue 5241)

After both are resolved, W2 can be formally declared complete and Gate B
conditions can be assessed.

Problem: No formal W2 completion gate — items are done but not verified as a
coherent workstream.

## Desired final state

- `docs/11-shared-definitions.md` W2 workstream updated with completion note
- `docs/05-compatibility-and-semantics.md` updated to reflect W2 completion
- `current-state.md` updated
- Gate B acceptance criteria verified with command output

## Scope

In scope:

- [ ] Run full Gate B validation and record results
- [ ] Update `docs/11-shared-definitions.md` W2 row with completion note
- [ ] Update `docs/05-compatibility-and-semantics.md` W2 coverage
- [ ] Update `current-state.md`
- [ ] Move 5240, 5241, 5242 to issues/done/

Out of scope:

- W3 (Data model) or W4 (Control/module/class) work
- Runtime implementation changes

## Affected paths

Expected:

- `docs/11-shared-definitions.md`
- `docs/05-compatibility-and-semantics.md`
- `current-state.md`

Do not touch:

- `crates/`
- `fixtures/`

## Acceptance criteria

- [ ] Gate B verification command output is recorded:
  ```
  curated fixture set 全件で Node.js との stdout 差分がゼロ
  ```
- [ ] Every W2-scope operator has at least one Node differential fixture
- [ ] W2 completion is noted in the workstreams table
- [ ] All three issues (5240, 5241, 5242) moved to done/

## Validation

```sh
cargo fmt --all --check
cargo nextest run
# Gate B: curated fixture set
cargo nextest run -p ts2wasm-cli --test m2_node_diff
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/11-shared-definitions.md` (W2 completion note)
- [ ] updated: `docs/05-compatibility-and-semantics.md`

Current state:

- [ ] not affected
- [ ] updated: `current-state.md`

Follow-up issues:

- [ ] none
- [ ] created/updated: none (W2 declares done; W3/W4 follow as separate workstreams)

## Notes

Gate B conditions:
> curated fixture セット全件で Node.js との stdout 差分がゼロ。
> differential test が CI で運用されている

This should be verified before declaring W2 complete. If any W2-scope fixture
still fails differential, create a blocking issue before resolving this one.
