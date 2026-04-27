# Cycle Report: issue 217

Status: DONE

Implementation commit: `1ef8f90`

Summary:

- Added runtime ABI constants for the 16-byte hidden GC header, header field offsets, kind/flag bits, and allocation/occupancy trigger thresholds.
- Changed `$alloc_heap` to allocate header + aligned payload while returning the existing payload pointer ABI to callers.
- Added allocation pressure accounting and `$gc_collect_stub` trigger before the OOM check.
- Added emitted-WAT contract coverage for the header/accounting path and runtime ABI layout tests.
- Moved issue 217 to `issues/done/` and regenerated `issues/index.md`.

Validation:

- `cargo fmt --all --check`: pass
- `cargo nextest run -p ts2wasm-runtime-abi`: pass, 8 tests
- `cargo nextest run -p ts2wasm-backend-wasm`: pass, 5 tests
- `scripts/manager update-issue-index --check`: pass
- `scripts/manager check-issue-health`: pass
- `scripts/manager check-agent-state`: pass
- `scripts/manager check-repo-smoke`: pass
- `cargo nextest run`: pass, 219 tests passed and 4 skipped

Notes:

- `$alloc_heap(size)` remains one-parameter in this slice, so the header kind field is emitted as `unknown`. This preserves all current call sites and leaves precise typed allocation metadata for a later typed allocator entrypoint.
- Webhook reporting was deferred because no webhook environment configuration was present.
