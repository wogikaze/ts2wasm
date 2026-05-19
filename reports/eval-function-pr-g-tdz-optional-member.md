# eval/function PR G report: TDZ optional member

Run ID: `eval-function-pr-g-tdz-optional-member`
Date: 2026-05-19

## Slice

Closed a dynamic direct eval TDZ scanner gap for optional member references.

## Change

- Extended the lowered direct-eval TDZ candidate scanner to treat `later?.value` like `later.value`.
- Mirrored the optional-member classifier in the focused Node shim so descriptor-v2 TDZ entries are interpreted consistently.
- Added `direct-eval-dynamic-tdz-optional-member-node-shim.ts` and registered it in the fixture catalog.
- Added focused Node-shim and host-deny coverage for the new fixture.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-tdz-optional-member-target cargo test -p ts2wasm-cli --test node_shim_host dynamic_direct_eval_tdz_optional_member_reference_is_catchable_reference_error -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-tdz-optional-member-target cargo test -p ts2wasm-cli --test m11_host_deny host_deny_rejects_dynamic_direct_eval_tdz_template_expression_host_lane -- --nocapture`
- PASS: `cargo fmt --all --check`

## Remaining

This is focused TDZ descriptor coverage. Full PR G still needs the broader descriptor v2 activation model, TDZ coverage beyond focused source forms, and mutation-ledger integration outside the Node shim.
