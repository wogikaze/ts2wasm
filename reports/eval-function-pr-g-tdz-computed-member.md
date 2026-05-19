# eval/function PR G report: TDZ computed member

Run ID: `eval-function-pr-g-tdz-computed-member`
Date: 2026-05-19

## Slice

Added focused coverage for dynamic direct eval TDZ base references through computed member access.

## Change

- Added `direct-eval-dynamic-tdz-computed-member-node-shim.ts`.
- Registered the fixture in `fixtures/catalog.yaml`.
- Added focused Node-shim coverage proving `eval("later['value']")` reports a catchable `ReferenceError` before `later` is initialized.
- Extended the existing host-deny TDZ group to include the computed-member fixture.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-tdz-computed-member-target cargo test -p ts2wasm-cli --test node_shim_host dynamic_direct_eval_tdz_computed_member_reference_is_catchable_reference_error -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-tdz-computed-member-target cargo test -p ts2wasm-cli --test m11_host_deny host_deny_rejects_dynamic_direct_eval_tdz_template_expression_host_lane -- --nocapture`
- PASS: `cargo fmt --all --check`

## Remaining

This is focused descriptor-v2 coverage. Full PR G still needs parser-backed TDZ modeling and production mutation-ledger/activation semantics beyond the Node shim.
