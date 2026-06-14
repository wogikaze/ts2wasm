# test262 Coverage Results (2026-05-29)

## Summary
- Total tests: 53,469
- Build pass: 7,696 (14.39% coverage)
- Semantic pass: 7,696
- Differential pass: 6,100
- Fail: 12,810
- Unsupported: 31,892
- Blocked: 1,071
- Runtime error: 12,795
- Negative compile pass: 1,596
- Mismatch: 832

## Major Categories

### BackendI/O (10,335)
Backend I/O operations not supported - likely involves Node.js host APIs, file system, network access, or other environment-specific features.

### UnsupportedSyntax (10,930)
1,298 from parser, 4303 from lexer, 1,521 from name-resolver, 2,798 from lowering

### UnsupportedRegExp (285)
Regular expression features not implemented

### UnsupportedModule (1,033)
Module resolution and import/export features not supported

### UnsupportedBuiltin (580)
Built-in object methods not implemented

### Feature-Resolution (3,051)
Feature detection and resolution issues

### Compiler Invariant (348)
Cases that fail due to invariant violations in the compiler

### Syntax Errors (2,276)
Parser/lexer syntax errors

### UnresolvedName (1,882)
Name resolution issues

### UnresolvedFunction (1,169)
Function name resolution issues

### UnsupportedEval (247)
Eval-related features not supported

### UnsupportedDate (24)
Date-related features not supported

### UnsupportedRuntimeSubset (808)
Runtime features not supported

### Test262Metadata (611)
Test metadata parsing issues

### DuplicateFunction (3)
Duplicate function definitions
- 2 from builtin-resolver
- 1 from name-resolver
- 10 from semantic-validator
- 1 from ast-validator

### DuplicateLocal (2)
Duplicate local variable declarations
- 54 from ast-validator
- 55 from name-resolver

### DuplicateParameter (51)
Duplicate parameter declarations
- 51 from lowering

### InvariantViolation (348)
Backend invariant violations
- 340 from backend
- 2 from lowering

### Negative-Parse-SyntaxError (832)
Expected negative syntax error cases producing different results

### Negative-Compile (208)
Expected compile failures not occurring

### RuntimeError (87)
Runtime errors (assertion failures, etc.)
- 188 Test262AssertionFailure
- 87 RuntimeError

## Key Files to Reference

### Parser/Syntax Issues
- `UnsupportedSyntax/parser: 1,298`
- `SyntaxError/parser: 2,195`
- `SyntaxError/lexer: 78`

### Name Resolution
- `UnresolvedName: 1,882` (total combined)
- `UnresolvedName/name-resolver: 1,148`
- `UnresolvedName/lowering: 617`

### Module Resolution
- `UnsupportedModule: 1,033`
- `UnsupportedModule/module-resolver: 3`
- `UnsupportedModule/parser: 468`
- `UnsupportedModule/name-resolver: 75`

### RegExp
- `UnsupportedRegExp: 285`
- `UnsupportedRegExp/lowering: 217`
- `regexp-literal: 285`

### Builtin APIs
- `UnsupportedBuiltin: 580`
- `UnsupportedBuiltin/builtin-resolver: 131`
- `UnsupportedBuiltin/backend: 34`
- `builtin-api: 580`

### Runtime
- `UnsupportedRuntimeSubset: 808`
- `runtime: 87`
- `semantic-unimplemented: 188`
- `negative-runtime-unverified: 8`

## Issues to Create

Based on the test262 coverage results, the following categories need dedicated issues:

### Parser & Lexer (Total: 3,531)
- `parser-syntax-error`: 2,195 cases
- `lexer-syntax-error`: 530 + 78 cases
- `parser-syntax`: 1,298 cases (UnsupportedSyntax)

### Name Resolution (Total: 3,043)
- `UnresolvedName`: 1,882 cases
  - `name-resolver`: 1,148 cases
  - `lowering`: 617 cases
- `DuplicateFunction`: 3 cases
  - `builtin-resolver`: 2 cases
  - `name-resolver`: 1 case
- `DuplicateLocal`: 2 cases
  - `ast-validator`: 54 cases
  - `name-resolver`: 55 cases
- `DuplicateParameter`: 51 cases (from lowering)

### Module Resolution (Total: 1,547)
- `UnsupportedModule`: 1,033 cases
  - `parser`: 468 cases
  - `name-resolver`: 75 cases
  - `builtin-resolver`: 131 cases

### RegExp (Total: 285)
- `UnsupportedRegExp`: 285 cases
  - `lowering`: 217 cases

### Builtin APIs (Total: 580)
- `UnsupportedBuiltin`: 580 cases
  - `builtin-resolver`: 131 cases
  - `backend`: 34 cases
  - `parser`: 412 cases
  - `lowering`: 412 cases

### Feature Resolution (Total: 3,051)
- `feature-resolution`: 3,051 cases

### Runtime (Total: 895)
- `UnsupportedRuntimeSubset`: 808 cases
  - `builtin-resolver`: 95 cases
  - `lowering`: 337 cases
  - `name-resolver`: 376 cases
- `runtime`: 87 cases
- `semantic-unimplemented`: 188 cases
- `negative-runtime-unverified`: 8 cases

### Compiler Invariants (Total: 348)
- `InvariantViolation`: 348 cases
  - `backend`: 340 cases
  - `lowering`: 2 cases

### Backend I/O (Total: 10,335)
- `BackendIo`: 10,335 cases

### Syntax Errors (Total: 2,276)
- `SyntaxError`: 2,276 cases
  - `parser`: 2,195 cases
  - `lexer`: 78 cases

### Date (Total: 24)
- `UnsupportedDate`: 24 cases
  - `lowering`: 24 cases

### Eval (Total: 247)
- `UnsupportedEval`: 247 cases
  - `builtin-resolver`: 1 case
  - `lowering`: 103 cases

### Test262 Metadata (Total: 611)
- `UnsupportedTest262Metadata`: 611 cases

### Negative Tests
- `negative-parse-syntaxerror`: 832 cases
- `ExpectedNegativeCompile`: 208 cases
- `ExpectedNegativeSyntax`: 832 cases

## Discord Report Generation

To generate the discord report, run:
```python
discord-report.py --summary-file issues/TEST262-COVERAGE-2026-05-29.md
```
