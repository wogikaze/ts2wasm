# Copilot Instructions for ts2wasm

## Commit Constraints

**Every agent turn that modifies files must follow these rules before finishing.**

### Commit granularity

- Commit in small logical work units. Do not collapse an entire assistant turn into one commit.
- One PR = one concern. Documentation changes, constant extraction, module splits, and IR introductions are separate concerns and must be separate commits (or PRs).
- Preferred commit boundaries for the current refactor sequence:
  - `runtime: extract ValueTag and layout constants` (PR2)
  - `runtime: replace magic numbers in WatEmitter with named constants` (PR2)
  - `emitter: split string intern into separate module` (PR3)
  - `emitter: split runtime builder from WatEmitter` (PR3)
  - `emitter: split emit_stmt and emit_expr into own modules` (PR3)
  - `ir: introduce LoweredIR types` (PR4)
  - `backend: lower AST through LoweredIR before emit` (PR4)

### Commit message format

- Short imperative subject line, e.g. `runtime: extract ValueTag constants`
- No period at the end
- Body (optional): motivation, what changed, what was explicitly not changed

### What must NOT be in a single commit

- Behaviour change + refactor together
- Documentation update + code change together (unless the doc change is the sole point of the commit)
- Multiple unrelated module splits

### Before each commit

1. Run `cargo test` — all tests must pass
2. Run `cargo fmt --all --check` — no formatting drift
3. Confirm no new `format!`-assembled WAT strings or raw opcode bytes were added to production backend code (see AGENTS.md § Compiler Code Rules)

## Compiler Code Rules (summary)

Full rules in `AGENTS.md` § Compiler Code Rules and `docs/04-compiler-architecture-and-runtime.md` § Compiler Code Rules.

Key prohibitions:

- Backend must only read `LoweredIR` — never `Ast`, `Stmt`, or `Expr`
- No `format!`-assembled WAT in production backend
- No raw opcode bytes (`bytes.push(0x41)`) in new code
- No magic numbers — use `ValueTag`, `Layout`, `MemoryAddr`, `RuntimeConst`
- No `$while_exit` / `$while_loop` string labels in backend — use structured IR
- No runtime logic embedded as string literals in emit functions

## PR Sequence (current work)

| PR | Title | Status |
|---|---|---|
| PR2 | Value layout separation — extract `ValueTag` / `MemoryAddr` constants | not started |
| PR3 | WatEmitter split — string intern / runtime builder / stmt / expr into own modules | not started |
| PR4 | LoweredIR introduction — backend stops reading `Stmt`/`Expr` directly | not started |
| PR5 | Verification gate — `wasm-tools validate` + snapshot CI | not started |
