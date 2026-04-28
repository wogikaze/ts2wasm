# Cycle Report: 062 Function Constructor Diagnostics

Run id: `062-function-constructor-diagnostics-20260428T050359Z`
Branch: `agent/062-function-constructor-diagnostics-20260428T050359Z`
Issue: `062`
Outcome: `PROGRESS`

## Scope Completed

- Added an issue-linked `UnsupportedSyntax` diagnostic for unshadowed dynamic `Function(...)` constructor calls.
- Added the same diagnostic for unshadowed dynamic `new Function(...)` constructor usage.
- Kept dynamic code evaluation unimplemented.
- Added resolver regression coverage and CLI build-failure coverage for both spellings.
- Added diagnostic fixtures under `fixtures/core-semantics/`.
- Updated the open issue with progress evidence and kept it open.

## Remaining Scope

- Full function feature syntax and semantics remain incomplete.
- Reference-test diagnostic reduction was not remeasured in this slice.
- Issue 062 acceptance criteria are not fully satisfied, so the issue was not moved to done.

## Validation

```text
cargo fmt --all --check
result: pass

cargo test -p ts2wasm-ir name_resolver_tests::tests::rejects_global
result: pass; 2 tests passed

cargo test -p ts2wasm-ir name_resolver_tests::tests::allows_shadowed_function_identifier_call
result: pass; 1 test passed

cargo test -p ts2wasm-cli --test m2_node_diff function_constructor
result: pass; 2 tests passed

cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/function-constructor-call-unsupported.ts -o /tmp/ts2wasm-function-constructor-call.wasm
result: expected failure; [UnsupportedSyntax] issue-062 diagnostic emitted at 89..109

cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/new-function-constructor-unsupported.ts -o /tmp/ts2wasm-new-function-constructor.wasm
result: expected failure; [UnsupportedSyntax] issue-062 diagnostic emitted at 93..117

scripts/manager check-issue-health
result: pass

scripts/manager check-agent-state
result: pass

cargo nextest run
result: pass; 330 tests passed, 4 skipped
```

## Commit

```text
29b553ac7f174b968fd313ead87f6943e3f3c50d issue-062: diagnose Function constructor usage
```

## Notes

The assignment report directory `reports/agents/062-function-constructor-diagnostics-20260428T050359Z/` was present as untracked input and was not staged.
