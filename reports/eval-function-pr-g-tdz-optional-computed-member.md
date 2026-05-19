# eval/function PR G report: TDZ optional computed member

Run ID: `eval-function-pr-g-tdz-optional-computed-member`
Date: 2026-05-19

## Slice

Closed the next focused descriptor-v2 TDZ scanner gap for optional computed member references.

## Change

- Extended the lowered direct-eval TDZ candidate scanner to treat `later?.["value"]` as a TDZ reference to `later`.
- Mirrored the same optional computed member classifier in the focused Node shim.
- Added `direct-eval-dynamic-tdz-optional-computed-member-node-shim.ts` and registered it in the fixture catalog.
- Added focused Node-shim and host-deny coverage for the fixture.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-tdz-computed-member-target cargo test -p ts2wasm-cli --test node_shim_host dynamic_direct_eval_tdz_optional_computed_member_reference_is_catchable_reference_error -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-tdz-computed-member-target cargo test -p ts2wasm-cli --test m11_host_deny host_deny_rejects_dynamic_direct_eval_tdz_template_expression_host_lane -- --nocapture`
- PASS: `cargo fmt --all --check`

## Remaining

This remains focused TDZ descriptor coverage. Full PR G still needs a parser-backed TDZ model and production mutation-ledger/activation contract beyond the Node shim.
