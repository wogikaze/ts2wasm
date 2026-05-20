# Testing and Coverage

## Test layers

| Layer | Purpose | Examples |
|---|---|---|
| Unit tests | crate-local invariants and small behavior | `crates/runtime-abi/tests`, `crates/runtime-catalog/tests` |
| Snapshot tests | AST/HIR/MIR/lowered/manifest/link-plan stability | `crates/frontend/tests/parser_snapshot.rs`, `crates/ir/tests/*snapshot*` |
| Differential tests | Node.js vs generated wasm output | `crates/cli/tests/m2_node_diff.rs`, catalog fixtures |
| Negative tests | expected diagnostics for invalid or unsupported input | `fixtures/negative`, parser/diagnostic tests |
| Reference coverage | larger suite execution and classification | `python scripts/manager.py reference-coverage ...` |
| Dashboard data | visualization of coverage/result history | `python scripts/manager.py coverage-dashboard-data` |

## Default commands

```bash
python scripts/manager.py check
python scripts/manager.py gate-fast
python scripts/manager.py nextest
python scripts/manager.py reference-coverage test262 --jsonl
python scripts/manager.py update-coverage-matrix -- --check
```

`gate` runs fmt, architecture checks, coverage matrix check, and nextest. Use focused tests first, then broader gates.

## Fixture catalog

`fixtures/catalog.yaml` is the fixture directory source of truth. Each directory records category, status, expected behavior, fixture names, and host import policy. Do not infer support from file existence alone.

## Feature matrix

`fixtures/feature-matrix.yaml` groups feature tags by fixture directory. `docs/26-semantic-feature-matrix.md` summarizes it for humans and agents.

## Reference coverage matrix

`artifacts/coverage/reference-coverage-matrix.md` is generated. It contains per-suite denominator, executed count, build/semantic coverage, unsupported reason tables, and evidence commands. Update with the manager script rather than manual edits.

## Evidence standard

A feature/change is done when the final report names:

- changed files,
- focused test command and result,
- broader gate command and result or why it was skipped,
- remaining unsupported/fail cases if any.
