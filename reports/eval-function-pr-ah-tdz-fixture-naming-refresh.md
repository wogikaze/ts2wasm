# eval/function PR A/H report: TDZ fixture naming refresh

Run ID: `eval-function-pr-ah-tdz-fixture-naming-refresh`
Date: 2026-05-19

## Slice

Refreshed the plan/docs guard for the dynamic direct eval TDZ boundary that moved from unsupported to supported host-lane coverage.

## Change

- Updated `plans/eval-new-function-implementation-plan.md` so Phase 0 no longer names the stale `direct-eval-dynamic-tdz-conflict-unsupported.ts` fixture.
- Documented that `direct-eval-dynamic-tdz-conflict-node-shim.ts` is now the supported dynamic direct eval descriptor-v2 fixture.
- Added a fixture comment explaining that descriptor v2 marks the later `value` binding as TDZ and the Node host lane returns a catchable `ReferenceError` rather than a build-time `UnsupportedEval`.
- Added an issue note tying the docs refresh to the existing PR G descriptor migration.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-tdz-fixture-refresh-target cargo test -p ts2wasm-cli --test node_shim_host dynamic_direct_eval_tdz_env_descriptor_conflict_is_catchable_reference_error -- --nocapture`
- PASS: `python scripts/issue-lint.py`
- PASS: scoped `git diff --check`

## Remaining

This is a PR A/H tracking cleanup slice. It does not expand descriptor-v2 TDZ coverage beyond the focused supported forms already guarded by the node-shim fixtures.
