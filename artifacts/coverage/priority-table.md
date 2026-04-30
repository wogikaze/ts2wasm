# Coverage Priority Table

Generated from current coverage data (2026-04-30)

## Overview

- **test262**: 18,000/53,445 executed (33.7%), build coverage 0.39%, semantic coverage 0.28%
- **tsc**: 200/6,419 executed (3.1%), build coverage 0.26%, semantic coverage 0.16%
- **tsgo**: 120/166 executed (72.3%), build coverage 5.42%, semantic coverage 4.22%

## Priority Table

### Tier 1: Critical Foundation (High Impact, High Dependency)

| Priority | Feature | Count | Rationale | Dependencies |
|---|---|---|---|---|
| 1 | name-resolution | 7,609 | Prerequisite for most features; largest blocker | None |
| 2 | builtin-api | 3,190 | Core runtime APIs; essential for practical code | name-resolution |
| 3 | array-builtin | 1,826 | Most frequently used data structure | builtin-api |
| 4 | object-builtin | 1,721 | Core object operations | builtin-api |

### Tier 2: Language Features (High Impact, Medium Dependency)

| Priority | Feature | Count | Rationale | Dependencies |
|---|---|---|---|---|
| 5 | regexp-literal | 972 | String matching/parsing is common | builtin-api |
| 6 | string-builtin | 745 | Text processing is fundamental | builtin-api |
| 7 | date | 398 | Date/time operations are common | builtin-api |
| 8 | function | 332 | Core language construct | name-resolution |
| 9 | function-resolution | 285 | Function call resolution | name-resolution |
| 10 | eval | 195 | Dynamic code execution | name-resolution, builtin-api |

### Tier 3: Syntax & Parsing (Medium Impact, Low Dependency)

| Priority | Feature | Count | Rationale | Dependencies |
|---|---|---|---|---|
| 11 | negative-parse-syntaxerror | 199 | Error handling for invalid syntax | parser-syntax |
| 12 | parser-syntax | 106 | General syntax parsing | None |
| 13 | duplicate-local | 66 | Variable shadowing detection | name-resolution |
| 14 | arity | 9 | Function parameter validation | function-resolution |

### Tier 4: Advanced Features (Lower Impact, High Complexity)

| Priority | Feature | Count | Rationale | Dependencies |
|---|---|---|---|---|
| 15 | annexb-ishtmldda | 13 | Annex B legacy features | builtin-api |
| 16 | legacy-global-builtin | 8 | Legacy global properties | builtin-api |
| 17 | rest-parameter | 5 | ES6 rest parameters | function |
| 18 | class | 2 | ES6 classes | object-builtin, function |
| 19 | object-literal | 2 | Enhanced object literals | object-builtin |
| 20 | arguments-object | 1 | Legacy arguments object | function |
| 21 | async-iteration | 1 | Async iteration protocol | async/await, iterators |
| 22 | switch | 1 | Switch statement | control-flow |

## Unsupported Syntax Breakdown

| Syntax Type | Count | Priority | Notes |
|---|---|---|---|
| UnsupportedSyntax | 9,578 | Varies | Generic unsupported syntax marker |
| UnresolvedName | 7,609 | Tier 1 | Name resolution failure |
| UnresolvedFunction | 285 | Tier 2 | Function resolution failure |
| ExpectedNegativeSyntaxError | 199 | Tier 3 | Intentional parse errors |
| DuplicateLocal | 66 | Tier 3 | Variable shadowing |
| ArityMismatch | 9 | Tier 4 | Parameter count mismatch |

## Suite-Specific Priorities

### test262

- Focus on Tier 1-2 features first (name-resolution, builtin-api, array/object/string builtins)
- These cover ~80% of unsupported cases

### tsc

- Current: 200/6,419 executed (3.1%)
- Top unsupported features:
  - parser-syntax: 47
  - ambient-declaration: 30
  - type-alias: 23
  - import-export: 21
- Priority: TypeScript-specific features (type system, modules)

### tsgo

- Current: 120/166 executed (72.3%)
- Highest coverage but still low absolute numbers
- Top unsupported features:
  - import-export: 20
  - declaration-emit: 16
  - parser-syntax: 16
- Priority: Go-specific TypeScript features

## Recommended Implementation Order

1. **Phase 1**: name-resolution (7,609 cases)
   - Foundation for almost everything else
   - Enables function-resolution, eval, etc.

2. **Phase 2**: builtin-api (3,190 cases)
   - Core runtime APIs (console, Math, etc.)
   - Enables array-builtin, object-builtin, string-builtin

3. **Phase 3**: array-builtin + object-builtin (3,547 cases combined)
   - Most commonly used data structures
   - High impact on practical code

4. **Phase 4**: string-builtin + regexp-literal (1,717 cases combined)
   - Text processing and pattern matching
   - Essential for data manipulation

5. **Phase 5**: function + function-resolution (617 cases combined)
   - Core language construct
   - Enables more complex code patterns

## Next Steps

1. Run `mise run reference-coverage -- test262 --limit 50000` to expand coverage
2. Generate issues from expanded coverage: `mise run reference-coverage -- test262 --limit 50000 --detail | mise run gen-issues-from-coverage -- --suite test262`
3. Update issue index: `mise run update-issue-index`
4. Create implementation issues for Tier 1 features first
