# Copilot Instructions for ts2wasm

Use `AGENTS.md` as the primary project contract and `docs/INDEX.md` as the documentation map. Keep this file short so GitHub Copilot receives stable routing rather than a stale full manual.

## Do

- Read `README.md` for the project overview before proposing code.
- Route architecture, runtime, ABI, and test questions through `docs/INDEX.md`.
- Prefer existing crates and modules over adding new dependencies.
- Add or update focused tests when behavior changes.

## Never

- Add backend code that consumes frontend AST directly when an IR path is available.
- Introduce raw runtime ABI numbers instead of `ts2wasm-runtime-abi` constants/types.
- Add host imports without capability/link-plan coverage.
- Treat generated coverage artifacts as editable source.

## Common gates

```bash
python scripts/manager.py check
python scripts/manager.py gate-fast
python scripts/manager.py nextest
```
