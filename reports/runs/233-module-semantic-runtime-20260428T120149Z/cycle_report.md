# Cycle Report: 233-module-semantic-runtime-20260428T120149Z

## Outcome

DONE: issue 233 closed.

## Scope

- Fixed `LoweredStmt::Export` emission so `$module_exports_set` receives arguments in catalog order: key pointer, key length, value.
- Added Node/iwasm semantic coverage for static named ES module import/export execution:
  - direct import: `static-entry.ts`
  - alias import: `static-entry-alias.ts`
  - importer lexical shadowing: `static-entry-shadow.ts`
  - repeated imports from one source module: `static-entry-repeated.ts`
- Kept broader module semantics out of scope: live bindings, default/namespace/dynamic imports, package resolution, and broader module body execution.
- Moved issue 233 to done, regenerated `issues/index.md`, and updated umbrella issue 055 paths required by issue-health.

## Acceptance Evidence

- Simple named export/import programs build to WASM:
  `static-entry.ts`, alias, shadow, and repeated fixtures all build.
- Imported values are read from resolved source module:
  `static-entry-shadow.ts` compares Node static ESM output with iwasm and prints `1`, not importer local `99`.
- Module initialization runs once per module for repeated imports:
  existing compiler graph coverage validates once-only initialization steps, and `static-entry-repeated.ts` Node/iwasm differential prints `2`.
- Runtime link plan includes module helpers only for module IR:
  `ts2wasm-backend-wasm` link-plan tests pass, including helper inclusion/exclusion coverage.
- Existing CommonJS module-cache fixtures still build:
  `cargo nextest run -p ts2wasm-cli module` passed.

## Validation

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-ir: PASS (18 tests)
cargo nextest run -p ts2wasm-backend-wasm: PASS (19 tests)
cargo nextest run -p ts2wasm-compiler: PASS (39 tests)
cargo nextest run -p ts2wasm-cli module: PASS (17 tests, 220 skipped)
cargo nextest run -p ts2wasm-cli static_named_module_import_fixtures_match_node_output_under_iwasm: PASS (1 test, 236 skipped)
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-233-semantic-entry.wasm: PASS
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-alias.ts -o /tmp/ts2wasm-233-semantic-alias.wasm: PASS
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-shadow.ts -o /tmp/ts2wasm-233-semantic-shadow.wasm: PASS
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-repeated.ts -o /tmp/ts2wasm-233-semantic-repeated.wasm: PASS
iwasm /tmp/ts2wasm-233-semantic-entry.wasm && iwasm /tmp/ts2wasm-233-semantic-alias.wasm && iwasm /tmp/ts2wasm-233-semantic-shadow.wasm && iwasm /tmp/ts2wasm-233-semantic-repeated.wasm: PASS (stdout 1, 1, 1, 2)
scripts/manager check-issue-index: PASS
scripts/manager check-issue-health: PASS
scripts/manager check-agent-state: PASS
scripts/manager check-repo-smoke: PASS
cargo nextest run: PASS (382 tests, 4 skipped)
```

## Reporting

Discord reporting was attempted after close artifacts were written. See `reporting_error.log` if reporting was deferred.
