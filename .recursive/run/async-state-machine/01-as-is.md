# Phase 1: AS-IS Analysis

## Source Requirement Inventory

Three TRACKING.yaml items are in scope:

| ID | Title | Status | Acceptance |
|---|---|---|---|
| 170 | Implement state-machine transformation for iwasm/CoreWasm target | active | `semantic_diff_async_return` |
| 171 | Define and implement runtime polling ABI (task_poll/task_result) for iwasm | open | `semantic_diff_await_sequence` |
| 172 | Implement JS Promise wrapper for wasm+js target using state-machine | open | `semantic_diff_async_exception` |

## Current async/await Pipeline

### 1. Lexer

- `async` token present (`TokenKind::Async`, `Token::Async`)
- `await` token present (`TokenKind::Await`, `Token::Await`)

### 2. Parser

- `async_function_statement()` at `statements_general.rs:1534`: parses `async function` and creates `Stmt::Function` but **drops the async flag** — the produced AST node is identical to a regular function.
- `Expr::Await` at `ast.rs:378`: parsed when `self.in_async_fn == true` (set during async function body parsing). Contains `{ expr: Box<Expr>, span: Span }`.

Key: `Stmt::Function` in `ast.rs:249` has NO `is_async` field. The async-ness is lost after parsing.

### 3. Resolver (name_resolver.rs → builtin_resolved.rs)

- `ResolvedStmt::Function` at `builtin_resolved.rs:36` has: `name`, `params`, `body`, `is_generator`, `is_ambient` — **NO `is_async`**.
- `ResolvedExpr::Await` at `builtin_resolved.rs:130` exists: `{ expr: Box<ResolvedExpr> }`.

### 4. Lowering (resolver.rs → lowered/types.rs)

- `LoweredFunction` at `types.rs:78` has: `id`, `params`, `uses_receiver`, `min_required_params`, `rest_param_index`, `locals`, `body`, `recursion_depth` — **NO `is_async`**.
- `ResolvedExpr::Await` maps to `LoweredExpr::PromiseGetValue` in `resolver_expr.rs:24`.

### 5. Backend emission (emitter.rs, expr_emit.rs)

- `PromiseGetValue` is a pass-through: it emits the promise expression but **does not poll or suspend**.
- `emit_functions()` at `emitter.rs:1397`: emits all functions the same way — no state machine.
- No frame lifting, no br_table resume points, no STATUS_PENDING return.

## Existing Test Fixtures

| Fixture | Purpose | Current status |
|---|---|---|
| `basic-async-return.ts` | `async function return_value(): Promise<number> { return 42; }` with await | build_smoke passes |
| `await-sequence.ts` | Two sequential awaits | build_smoke passes |
| `async-exception.ts` | try/catch around await | build_smoke passes |

All 3 semantic_diff tests are `#[ignore]`.

## RuntimeFn Catalog

No async-related RuntimeFn variants exist yet. No task_poll, task_result, or task_drop.

## Summary of Gaps

1. **AST**: Stmt::Function has no `is_async` field → can't distinguish async from sync
2. **Resolved**: ResolvedStmt::Function has no `is_async` field
3. **Lowered**: LoweredFunction has no `is_async` field
4. **Backend**: No state-machine transformation (frame lifting, br_table resume, STATUS_PENDING)
5. **Runtime**: No task_poll/task_result/task_drop RuntimeFn variants
6. **No JS Promise wrapper**: No queueMicrotask-driven polling for wasm+js target
