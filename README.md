# ts2wasm

`ts2wasm` は TypeScript / JavaScript を WebAssembly へ変換する Rust workspace である。現在の実装は、既存 JS/TS をできるだけ保ったまま WASI Preview 1 で動かし、必要な Node 互換 host API は capability manifest で監査可能にすることを目指す。

## 現在の位置づけ

- 実装済み target: `wasm32-wasi-p1` (`wasm32-wasi`) と `wasm32-wasi-p1+node-shim` (`wasm32-wasi+node-host`)。
- 予約 target: `wasm32-wasi-gc`, `wasm32-component`。target parser は存在するが backend は未実装として拒否する。
- Compiler pipeline は legacy `LoweredProgram` emission が主経路。`--experimental-hir-mir` と `--experimental-hir-mir-compat-fallback` は HIR/MIR 移行検証用。
- Runtime catalog は `RuntimeFn`、依存関係、host import、capability、runtime strings、link plan を一元管理する。
- Fixture catalog は 36 ディレクトリを管理し、status は pass=29, unknown=7。

## Quick start

```bash
# toolchain check / smoke
python scripts/manager.py check

# build a fixture
cargo run -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/hello.wasm

# inspect pipeline output
cargo run -p ts2wasm-cli -- dump --ast fixtures/basics-hello/hello.ts
cargo run -p ts2wasm-cli -- dump --wat fixtures/basics-hello/hello.ts

# standard gates
python scripts/manager.py gate-fast
python scripts/manager.py nextest
```

`mise` が入っている場合は `mise run check`、`mise run gate-fast`、`mise run nextest` を同じ入口として使える。

## CLI

```bash
ts2wasm build <input.ts> -o <output.wasm> [--emit-manifest <manifest.json>] [--target <target>]
ts2wasm check <input.ts>
ts2wasm dump [--tokens|--ast|--resolved|--tir|--optimize|-O <0..3>|--lowered|--wat|--unparse] <input.ts>
ts2wasm server
```

主な build flags:

- `--emit-manifest` / `--emit-capabilities`: capability manifest JSON を出力する。
- `--host-deny`: Node host import を必要とする program を拒否する。
- `--explain-unsupported`: unsupported diagnostic の tracking/実装先を詳しく表示する。
- `--experimental-hir-mir`: HIR → MIR → emission を strict に使う。
- `--experimental-hir-mir-compat-fallback`: HIR/MIR が未対応なら legacy emission に戻る。
- `--target`: `wasm32-wasi`, `wasm32-wasi-p1`, `wasm32-wasi+node-host`, `wasm32-wasi-p1+node-shim` など。

## Architecture map

```text
source text
  -> lexer / parser / AST validation
  -> module graph + static import binding rewrite
  -> name resolution
  -> builtin resolution
  -> static literal eval expansion
  -> semantic validation
  -> legacy LoweredProgram or experimental HIR/MIR
  -> runtime gate + link plan + capability manifest
  -> wasm binary emission + ABI metadata custom section
```

Workspace crates:

| Crate | Responsibility |
|---|---|
| `source` | spans/source helpers |
| `diagnostic` | `Diagnostic`, `DiagCode`, phase/source errors |
| `syntax` | canonical AST and token types |
| `frontend` | lexer/parser/type-erasure/TypeScript oracle |
| `resolve` | binding and name resolution |
| `semantics` | builtin domain identities and semantic helpers |
| `ir` | resolved AST, HIR, MIR, LoweredProgram, optimizer |
| `runtime-abi` | tagged value and linear-memory ABI |
| `runtime-catalog` | runtime function registry and link plans |
| `backend-core` | backend-independent Wasm IR shell |
| `backend-wasm` | wasm binary emission, WAT debug emission, and runtime templates |
| `compiler` | end-to-end pipeline and server |
| `cli` | command-line interface |
| `shared` | compatibility re-exports and manifest types |

Next architecture crates:

| Crate | Responsibility |
|---|---|
| `runtime-core` | JS engine substrate: values, heap, shapes, realms, environments, frames, GC, and baseline VM containers |
| `semantic-ir` | ECMAScript semantic floor: CFG blocks, references, completions, abrupt edges, iterator/env/property operations |
| `spec-kernel` | `SpecOp` and ECMAScript internal method dispatch; new specification operations go here, not to `RuntimeFn` |
| `backend-correctness` | slow-correct `semantic-ir -> SpecOp -> wasm/runtime call` exit path |
| `opt-mir` | optional fast path IR with guards, deopt, slow-path calls, and `FrameState` |

## Documentation map

Start from `docs/INDEX.md`. The root docs are intentionally routers; detailed design lives in task-focused files.

| Need | Read |
|---|---|
| Project identity and non-goals | `docs/01-project-definition.md` |
| Execution target and CLI contract | `docs/02-execution-model-and-targets.md` |
| Capability manifest / host imports | `docs/03-api-and-host-capability.md` |
| Compiler/runtime architecture | `docs/04-compiler-architecture-and-runtime.md` |
| Compatibility and supported semantics | `docs/05-compatibility-and-semantics.md` |
| Tests and coverage | `docs/06-testing-and-coverage.md` |
| IR contract | `docs/13-ir-contracts.md` |
| Runtime ABI | `docs/14-runtime-abi.md` |
| Current status | `docs/current-state.md` |

## Development notes

- Do not edit `issues/` unless the task is issue workflow itself.
- Do not hand-edit generated coverage artifacts without also documenting the generator command.
- Code changes that alter architecture, ABI, target semantics, or capability policy must update the matching docs.
- Historical `plans/`, `.agents/plans/`, `reports/`, and `.recursive/run/` files are retained as archive/context. Canonical current docs live in `docs/`.
