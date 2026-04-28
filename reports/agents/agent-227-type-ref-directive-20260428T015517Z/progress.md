# Issue 227 progress report

Status: PROGRESS.

Implemented a narrow TypeScript triple-slash `reference types` directive preflight. The compiler now emits a precise `issue-227` `UnsupportedSyntax` diagnostic for unresolved `/// <reference types="..."/>` directives before parser fallback diagnostics can hide the root cause. The preflight suppresses that directive diagnostic for the two issue-note interactions covered in this slice: immediately preceding `// @ts-ignore` and virtual TypeScript-Go inputs with `"skipLibCheck": true`.

Regression evidence:

- `fixtures/typescript-directives/reference-types-missing.ts`
- `fixtures/typescript-directives/reference-types-ts-ignore.ts`
- `fixtures/typescript-directives/reference-types-skip-lib-check.ts`
- `crates/cli/tests/type_reference_directives.rs`
- frontend unit tests in `crates/frontend/src/type_reference_directive.rs`

Reference coverage evidence:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage tsgo --path-filter processingDiagnostic --detail
result: pass
unsupported_features=import-export:2,typescript-directive:1
processingDiagnostic.ts: UnsupportedSyntax: typescript-directive
processingDiagnosticSkipLibCheck.ts: UnsupportedSyntax: import-export
processingDiagnosticTsIgnore.ts: UnsupportedSyntax: import-export
```

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage tsgo --limit 120
result: pass
unsupported_features=import-export:22,parser-syntax:17,declaration-emit:16,module-resolution:10,jsx:8,class:7,type-system:7,decorator:4,enum:3,object-literal:3,type-assertion:3,destructuring:2,jsdoc:2,type-alias:2,class-accessor:1,module-system-amd:1,name-resolution:1,scope-analysis:1,typescript-directive:1
```

Validation:

```text
cargo fmt --all --check: pass
cargo nextest run type_reference: pass; 4 passed
cargo nextest run -p ts2wasm-cli --test type_reference_directives: pass; 3 passed
cargo nextest run: pass; 279 passed, 4 skipped
scripts/manager check-issue-health: pass
scripts/manager check-agent-state: pass
scripts/manager check-repo-smoke: pass
```

Remaining risk:

Full package manager / type package resolution is still not implemented. The issue remains open because this child assignment requested a minimal diagnostic and regression slice, not full type package resolution.
