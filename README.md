# TS transpiler to WASM

このリポジトリの README は入口に限定し、詳細設計は `docs/` に分割する。元 README の情報は削除せず、各ドキュメントへ移動する。未移動・改変確認用として、原文を `docs/99-original-plan.md` に保持する。

## Project position

TypeScript / JavaScript の既存資産を、Node.js に処理を丸投げせず、可能な限り WebAssembly 実行環境へ持ち込む compiler/runtime プロジェクトである。AssemblyScript のような TypeScript 風の別言語ではなく、QuickJS/Javy のような JS engine 同梱にも寄せすぎない。生成 WASM、WASM 側 runtime、必要最小限の host shim を分離する。

## Documentation map

| Document | Role |
|---|---|
| `docs/00-docs-list.md` | docs 全体の分割方針、情報保持ルール、各 doc の責務 |
| `docs/01-project-definition.md` | 概要、目標、非目標、禁止事項、まとめ |
| `docs/02-execution-model-and-targets.md` | 三層実行モデル、WASI/iwasm/Node host/将来 backend、出力形式、CLI |
| `docs/03-api-and-host-capability.md` | API 対応、WASI-compatible Node idioms、host shim trimming、capability manifest |
| `docs/04-compiler-architecture-and-runtime.md` | frontend/semantic/lowering/backend、値表現、memory management、runtime ABI |
| `docs/05-compatibility-and-semantics.md` | TypeScript 構文・型、JavaScript 意味論、module/npm 対応境界 |
| `docs/06-testing-and-coverage.md` | テスト分類、coverage state、differential testing、skip policy |
| `docs/07-performance-and-optimization.md` | performance goal、optimization levels、optimization strategy、benchmark state |
| `docs/08-roadmap-and-success.md` | 実装ロードマップ、成功条件、canonical milestone への参照 |
| `docs/09-security-and-capability-model.md` | host capability、WASI preopen、Node host security、manifest 監査 |
| `docs/10-related-projects.md` | QuickJS、AssemblyScript、Emscripten、Javy、tsc などとの比較 |
| `docs/11-shared-definitions.md` | milestone、test status schema、capability manifest、optimization mode、benchmark policy |
| `docs/12-current-implementation-status.md` | 現在の実装事実、未実装範囲、検証状況 |
| `docs/13-coding-standard.md` | Rust コード規約。panic 禁止、Diagnostic、Span、IR variant 追加、backend WAT 直書き禁止、RuntimeFn catalog |
| `docs/14-ir-contracts.md` | AST / HIR / MIR / Wasm IR の責務と不変条件。validate_* の仕様 |
| `docs/15-runtime-abi.md` | RawValue tagged encoding、heap layout、RuntimeFn catalog、host import ABI |
| `docs/16-coverage-matrix.md` | reference test coverage dashboard。test262/TypeScript/tests を分母にした進捗可視化 |
| `docs/99-original-plan.md` | 元 PLAN 原文。情報欠落チェック用 |

## First implementation slice

```text
Long-term target:
  TypeScript / JavaScript assets を可能な限り WASM 実行へ移す。

M0 implementation scope (現在):
  single-file JS subset compiler。
  TypeScript type syntax、object semantics、module resolution、npm package compatibility は対象外。
  数値は i32 small-int tagged encoding のみ。

M0 pipeline:
  single file TS/JS
  → JS semantic IR
  → runtime ABI call
  → WASI wasm
  → iwasm execution
  → Node differential test
  → capability manifest

M1+ expansion:
  compatibility level ごとに syntax / semantics / runtime / host capability を追加する。
  詳細は docs/08-roadmap-and-success.md を参照。
```

## Current source preservation rule

- 元 README の主要セクションは、上記 docs のいずれかに移動する。
- docs 化により情報密度を落とさない。
- 改善案は `追加設計` として区別し、原文由来の設計と混ぜて不可視化しない。
- 原文は `docs/99-original-plan.md` に保持する。
