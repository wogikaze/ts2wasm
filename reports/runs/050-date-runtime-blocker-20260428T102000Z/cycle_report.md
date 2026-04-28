# Cycle Report: Issue 050 Date Runtime

Date: 2026-04-28
Run ID: `050-date-runtime-blocker-20260428T102000Z`
Branch: `agent/050-date-runtime-20260428T102000Z`
Outcome: `BLOCKED`

## Scope Checked

Assigned issue: `issues/open/050-implement-date.md`

Allowed implementation files do not include `crates/ir/src/**`. Date constructor recognition, static builtin recognition, and method lowering currently live there.

## Evidence

`new Date(0)` reproduction:

```text
command: cargo run -q -p ts2wasm-cli -- build /tmp/ts2wasm-date-ZvvJxH.ts -o /tmp/ts2wasm-date-test.wasm
result: exit 1
stderr: error: [UnsupportedSyntax] issue-207: instanceof right-hand side must be a supported class constructor `Date`
```

`Date.now()` reproduction:

```text
command: cargo run -q -p ts2wasm-cli -- build /tmp/ts2wasm-date-now-jjxJat.ts -o /tmp/ts2wasm-date-now-test.wasm
result: exit 1
stderr: error: [UnresolvedName] unresolved name: `Date`
```

## Decision

Do not implement unaudited host time access in backend/runtime. Keep issue 050 open and report a blocker because safe completion requires:

- `crates/ir` lowering changes outside the assignment's allowed files.
- A Date time capability policy before `Date.now()` or zero-argument `new Date()` can be implemented.

## Validation

Report-only blocker validation:

```text
command: cargo fmt --all --check
result: pass

command: scripts/manager update-issue-index --check
result: pass (issues/index.md OK, up to date)

command: scripts/manager check-issue-health
result: pass (issues/index.md queue OK; check_issue_health: OK)

command: scripts/manager check-repo-smoke
result: pass (shell syntax, issue index, and issue health checks passed)
```

`cargo nextest run` was not run because the issue was not closed as DONE and no implementation code changed.
