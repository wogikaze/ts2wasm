# Cycle Report: issue 215

Run ID: `20260428-issue-215-math-random-policy`
Branch: `agent/215-math-random-policy-20260428T010000Z`
Implementation commit: `2cbf0af`

## Outcome

DONE. `Math.random()` no longer emits a deterministic placeholder helper. The backend now imports `wasi_snapshot_preview1.random_get`, records `wasi.random: true` with audit reasons in the capability manifest, and the host-deny path continues to accept it as standalone WASI rather than a Node host import.

The current runtime number model is still tagged integer based. The docs and current state explicitly record that full ECMAScript fractional double parity belongs to the broader number representation model, not to the randomness capability policy.

## Acceptance Evidence

- Fixed placeholder removal: backend WAT tests assert `Math.random` emits `random_get` and no `$random_counter`.
- Capability manifest: backend and CLI tests assert `wasi.random: true`, `standalone: true`, and `node_host.required: false` for `fixtures/builtins-and-io/math-random.ts`.
- Host-deny / manifest validation: CLI host-deny manifest test passes, and shared manifest validation now rejects `wasi.random` without an audit reason.
- Docs/current-state/issues sync: `docs/language-reference/javascript-features.md`, `docs/05-compatibility-and-semantics.md`, `current-state.md`, and issue 215 were updated; `issues/index.md` was regenerated.

## Commands

- `cargo fmt --all --check`: pass
- `cargo nextest run -E 'test(math) | test(manifest) | test(host)'`: pass, 15 passed / 232 skipped
- `scripts/manager check-manifest-imports`: pass for `fixtures/basics-hello/hello.ts`
- Direct build of `fixtures/builtins-and-io/math-random.ts` with `--emit-manifest`: pass; manifest has `standalone=true`, `wasi.random=true`, `node_host.required=false`, and wasm includes `random_get`
- `cargo nextest run`: pass, 243 passed / 4 skipped
- `scripts/manager update-issue-index`: pass
- `scripts/manager check-issue-index`: pass
- `scripts/manager check-issue-health`: pass
- `scripts/manager check-agent-state`: pass
- `scripts/manager check-repo-smoke`: pass

## Follow-up

No follow-up issue was created for issue 215. Fractional JS number parity remains part of the existing broader number-model work.
