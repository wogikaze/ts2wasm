# Issues Index

This file is the human entrypoint for the issue queue.

Issue files are the source of truth for work items. The generated section below may be replaced by a script or pasted manually from a generated report.

## Reading rules

- Start with `Ready queue`.
- Check `Blocked queue` only after ready work is exhausted.
- Do not use `done/` as current project truth.
- For docs work, verify whether the issue updates final-state docs, `current-state.md` (repo root), or follow-up issues.
- For implementation work, verify acceptance criteria and validation commands before starting.

## Ready queue

<!-- generated:ready:start -->
| ID | Title | Type | Area | Class | Priority | Depends on | Summary |
|---:|---|---|---|---|---|---|---|
| 003 | Verify manifest against emitted WAT imports | test | wasi/tests | implementation-ready | P0 | 002 | A manifest is only useful as a gate if it matches emitted WAT/wasm imports. The current project needs a test that cro... |
| 004 | Reclassify compile-only compatibility tests | test | tests/coverage | implementation-ready | P0 | none | Tests such as class/module/Node API compile-only checks can make compatibility look more advanced than it is. Build s... |
| 005 | Add fine-grained unsupported feature breakdown | infra | scripts/coverage | implementation-ready | P0 | none | `UnsupportedSyntax:423` is not actionable. The project needs feature-level breakdown such as class, import, regexp li... |
| 011 | Enable `RUSTFLAGS=-D warnings` for nextest / harness (warning-clean tree) | infra | tests | implementation-ready | P2 | none | Enable `RUSTFLAGS=-D warnings` for nextest / harness (warning-clean tree) |
| 013 | Implement heap OOM check | feature | runtime/memory | implementation-ready | P0 | none | `$alloc_heap` does not check `memory.size` before allocation. Large allocations can cause undefined behavior or memor... |
| 014 | Implement dynamic property key support | feature | runtime/semantics | implementation-ready | P1 | 012 | Dynamic property keys (e.g., `obj[variable]`) are not implemented. Currently diagnosed as `unsupported-dynamic-proper... |
| 015 | Implement object literal string key support | feature | parser/semantics | implementation-ready | P1 | none | Object literal with string literal keys `{"x": v}` is not implemented. Currently only identifier keys `{x: v}` are su... |
| 018 | Implement UTF-8 string support | feature | runtime/semantics | implementation-ready | P1 | none | Non-ASCII string literals are intentionally unsupported. UTF-8 support is incomplete. docs/04 specifies UTF-8 decode/... |
| 026 | Migrate backend module to backend-wasm crate | refactor | runtime | implementation-ready | P1 | 024, 025 | Migrate backend module to backend-wasm crate |
| 029 | Implement typeof operator | feature | runtime/semantics | implementation-ready | P1 |  | Implement typeof operator |
| 030 | Implement instanceof operator | feature | runtime/semantics | implementation-ready | P1 |  | Implement instanceof operator |
| 031 | Implement in operator | feature | runtime/semantics | implementation-ready | P1 |  | Implement in operator |
| 032 | Implement delete operator | feature | runtime/semantics | implementation-ready | P1 |  | Implement delete operator |
| 033 | Implement switch statement | feature | frontend/semantics | implementation-ready | P1 |  | Implement switch statement |
| 034 | Implement while and do-while loops | feature | frontend/semantics | implementation-ready | P1 |  | Implement while and do-while loops |
| 035 | Implement break and continue statements | feature | frontend/semantics | implementation-ready | P1 |  | Implement break and continue statements |
| 036 | Implement arrow function | feature | frontend/semantics | implementation-ready | P1 |  | Implement arrow function |
| 037 | Implement this binding | feature | runtime/semantics | implementation-ready | P1 |  | Implement this binding |
| 038 | Implement rest parameters | feature | frontend/semantics | implementation-ready | P1 |  | Implement rest parameters |
| 039 | Implement spread arguments | feature | frontend/semantics | implementation-ready | P1 |  | Implement spread arguments |
| 040 | Implement default parameters | feature | frontend/semantics | implementation-ready | P1 |  | Implement default parameters |
| 041 | Implement template literals | feature | frontend/semantics | implementation-ready | P1 |  | Implement template literals |
| 042 | Implement string methods | feature | runtime/builtins | implementation-ready | P1 |  | Implement string methods |
| 043 | Implement string indexing | feature | runtime/semantics | implementation-ready | P1 |  | Implement string indexing |
| 044 | Implement String.fromCharCode and charCodeAt | feature | runtime/builtins | implementation-ready | P1 |  | Implement String.fromCharCode and charCodeAt |
| 045 | Implement class declaration and expression | feature | frontend/semantics | implementation-ready | P1 |  | Implement class declaration and expression |
| 048 | Implement prototype chain | feature | runtime/semantics | implementation-ready | P1 |  | Implement prototype chain |
| 049 | Implement Map and Set | feature | runtime/builtins | implementation-ready | P1 |  | Implement Map and Set |
| 050 | Implement Date | feature | runtime/builtins | implementation-ready | P1 |  | Implement Date |
| 051 | Implement RegExp | feature | runtime/builtins | implementation-ready | P1 |  | Implement RegExp |
| 052 | Implement JSON | feature | runtime/builtins | implementation-ready | P1 |  | Implement JSON |
| 053 | Implement Math | feature | runtime/builtins | implementation-ready | P1 |  | Implement Math |
| 054 | Implement Error types | feature | runtime/builtins | implementation-ready | P1 |  | Implement Error types |
| 055 | Implement import and export | feature | frontend/semantics | implementation-ready | P1 |  | Implement import and export |
<!-- generated:ready:end -->

## Blocked queue

<!-- generated:blocked:start -->
| ID | Title | Type | Area | Blocker | Summary |
|---:|---|---|---|---|---|
| 006 | Remove stale milestone and transitional docs | cleanup | docs | 002,003 | Several docs appear to mix stale milestone notes, transitional manifest schema, and current imple... |
| 007 | Harden reference coverage prerequisites | infra | scripts/reference | 005 | Reference coverage scripts depend on external reference repositories. If those repositories are m... |
| 008 | Introduce typed WAT writer skeleton | refactor | backend | 003 | Large raw WAT string generation in runtime builder code is a major maintainability risk. A full r... |
| 009 | Select first coverage-improvement feature slice | spike | frontend/ir/runtime | 005 | After coverage breakdown exists, the next implementation should be chosen by data. The goal is to... |
| 010 | Extract frontend module from crates/cli | refactor | frontend | 003,004 | `crates/cli/src/lib.rs` appears to mix lexer, parser, AST, span, validation, and build pipeline c... |
| 016 | Implement prototype and method call support | feature | runtime/semantics | 014 | Prototype chain lookup and method calls are not implemented. Currently diagnosed as `unsupported-... |
| 017 | Design and implement GC strategy | feature | runtime/memory | 013 | Current runtime has no GC. Long-running programs and programs with closure escape will leak memor... |
| 017a | Design GC strategy | feature | runtime/memory | 013 | Current runtime has no GC. Long-running programs and programs with closure escape will leak memor... |
| 017b | Implement GC strategy | feature | runtime/memory | 017a | GC strategy is designed in 017a but not implemented. Runtime needs actual GC to prevent memory le... |
| 019 | Integrate TypeScript parser/checker | feature | frontend | 010 | TypeScript parser/checker integration is not implemented. Current parser is minimal. docs/04 spec... |
| 019a | Integrate TypeScript compiler API for type checking | feature | frontend | 010 | TypeScript parser/checker integration is not implemented. Current parser is minimal. docs/04 spec... |
| 019b | Extract type information for optimization hints | feature | frontend | 019a | TypeScript compiler API is integrated in 019a but type information is not yet extracted for optim... |
| 020 | Implement generic JavaScript semantic IR | feature | ir/semantics | 019 | Generic JavaScript semantic IR is not implemented. Current IR is minimal and tied to specific low... |
| 020a | Design JavaScript semantic IR | feature | ir/semantics | 019 | Generic JavaScript semantic IR is not designed. Current IR is minimal and tied to specific loweri... |
| 020b | Implement IR lowering from TypeScript AST | feature | ir/semantics | 020a | IR design is complete in 020a but IR lowering from TypeScript AST is not implemented. |
| 020c | Add IR validation passes and document contracts | feature | ir/semantics | 020b | IR lowering is implemented in 020b but validation passes and contract documentation are missing. |
| 021 | Implement full wasm backend | feature | backend | 008, 020 | Full wasm backend is not implemented. Current implementation is WAT-centric. docs/04 specifies in... |
| 022 | Expand test262 differential coverage | feature | tests/coverage | 005 | test262 full differential operation is incomplete. Current coverage uses sample/ramp approach. do... |
| 023 | Implement host-deny and auditable E2E manifest | feature | security/capability | 002, 003 | host-deny / capability manifest "auditable E2E" is planned but not implemented. docs/06 specifies... |
| 046 | Implement extends inheritance | feature | runtime/semantics | 045 | Implement extends inheritance |
| 047 | Implement super keyword | feature | runtime/semantics | 045, 046 | Implement super keyword |
<!-- generated:blocked:end -->

## Done queue

<!-- generated:done:start -->
| ID | Title | Type | Area | Completed evidence |
|---:|---|---|---|---|
| 000 | Short imperative title | feature | bug | refactor | docs | test | infra | cleanup | spike | frontend | ir | runtime | abi | wasi | cli | fixtures | scripts | docs | tests | coverage | reference | see `issues/done/000-sample-issue.md` |
| 001 | Fix issue infrastructure and current-state path references | infra | issues/docs | see `issues/done/001-fix-issue-infrastructure-and-current-state-path-references.md` |
| 002 | Emit canonical capability manifest schema | feature | abi/wasi | see `issues/done/002-emit-canonical-capability-manifest-schema.md` |
| 012 | Fix computed property semantics bug | bug | runtime/semantics | see `issues/done/012-fix-computed-property-semantics-bug.md` |
| 024 | Migrate runtime module to runtime-abi crate | refactor | abi | see `issues/done/024-migrate-runtime-module-to-runtime-abi-crate.md` |
| 025 | Migrate ir module to ir crate | refactor | ir | see `issues/done/025-migrate-ir-module-to-ir-crate.md` |
| 027 | Migrate frontend code to frontend crate | refactor | frontend | see `issues/done/027-migrate-frontend-code-to-frontend-crate.md` |
<!-- generated:done:end -->

## Index generation contract

Run `scripts/manager update-issue-index` after adding, closing, or moving issues. CI and agents should run `scripts/manager update-issue-index --check` and `scripts/manager check-issue-index`.

A future generator replaces only the regions between the `<!-- generated:*:start -->` / `<!-- generated:*:end -->` markers.

Do not put hand-written policy text inside generated regions.

## Manual update checklist

When adding, completing, or blocking an issue:

- [ ] issue file is in the correct directory
- [ ] frontmatter is updated
- [ ] dependencies are reflected by re-running `scripts/manager update-issue-index`
- [ ] done issue has completion evidence
- [ ] follow-up work is represented as a separate open issue
- [ ] final-state docs do not contain future TODOs
- [ ] current implementation gaps are in `current-state.md` (repo root)
