# Progress Report: issue 054 Error types

Child id: agent-054-error-types-20260428T010400Z
Branch: agent/054-error-types-20260428T010400Z
Date: 2026-04-28

## Outcome

PROGRESS

Implemented the first Error-object slice: `new Error("...")`, `new TypeError("...")`, `new ReferenceError("...")`, and `new SyntaxError("...")` now build and expose `.message` through the existing object/property runtime.

## Reproduction

Before changes, each requested constructor form failed during build:

```text
new Error("msg") -> issue-207: instanceof right-hand side must be a supported class constructor `Error`
new TypeError("msg") -> issue-207: instanceof right-hand side must be a supported class constructor `TypeError`
new ReferenceError("msg") -> issue-207: instanceof right-hand side must be a supported class constructor `ReferenceError`
new SyntaxError("msg") -> issue-207: instanceof right-hand side must be a supported class constructor `SyntaxError`
```

## Implemented

- Lower builtin Error constructors to plain heap objects with a `message` property.
- Use an empty string for `new Error().message`.
- Added Node/iwasm differential coverage in `fixtures/builtins-and-io/error-message.ts`.

## Remaining

- `Error.prototype.stack` is not implemented and likely needs a design decision about stack capture/source locations.
- Error prototype identity and `instanceof Error` are not implemented by this slice.
- Non-string constructor message coercion is not covered by this slice.

## Validation

All validation commands below passed on 2026-04-28:

- `cargo fmt --all --check`
- `cargo nextest run -E 'test(error)'`
- `cargo nextest run -p ts2wasm-cli error`
- `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/error-message.ts -o <tmp>/error-message.wasm && iwasm <tmp>/error-message.wasm`
- `cargo nextest run`
- `scripts/manager check-agent-state`
- `scripts/manager check-issue-health`
- `scripts/manager check-repo-smoke`
- `scripts/manager clippy` exited 0 with pre-existing warning-only output outside this slice.

Webhook reporting was deferred: `scripts/manager discord-report --run-id 054-error-types-20260428T010400Z` failed twice because `DISCORD_WEBHOOK_URL` is not configured. Deferred payload and error evidence are saved under `reports/runs/054-error-types-20260428T010400Z/`.
