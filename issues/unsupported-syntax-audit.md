# UnsupportedSyntax Audit — I-20260517-AW9JQT

**Target**: 11489 test262 cases classified as `DiagCode::UnsupportedSyntax`
**Scope**: `crates/frontend/` and `crates/ir/` only (no `crates/backend-wasm/`)
**Total production usages**: ~349 across 42 source files

---

## 1. Category Summary

### 1.1 Lexer-level rejections (frontend/lexer)

These are syntax errors during tokenization. Most are edge-case invalid syntax, not test262-relevant features.

| Subcategory | File | Lines | Count |
|---|---|---|---|
| Unicode identifier escapes | `lexer_identifiers.rs` | 11,39,126,143,155,168,181,195,206,219,233 | 11 |
| Unicode string escapes | `lexer_strings.rs` | 168,179,195,221,233,246,260 | 7 |
| Legacy octal escapes (strict mode) | `lexer_strings.rs` | 39,113,138,147 | 4 |
| Invalid/unsupported escape sequences | `lexer_strings.rs` | 5,53,82,95 | 4 |
| BigInt literal restrictions | `lexer_numbers.rs` | 35,46,405,417,437,527 | 6 |
| Invalid numeric literals | `lexer_numbers.rs` | 97,206,219,230,293,326,339,352 | 8 |
| Private identifier `#x` validation | `lexer_identifiers.rs` | 233 | 1 |
| Triple-slash directives | `lexer.rs` | 1292 | 1 |
| Merge conflict markers | `lexer.rs` | 1383 | 1 |
| Triple-slash reference types | `type_reference_directive.rs` | 28 | 1 |

**Total lexer-level**: ~44

### 1.2 Parser-level rejections (frontend/parser)

These reject valid JS/TS syntax that the parser understands structurally but cannot yet lower.

| Subcategory | File | Lines | Count |
|---|---|---|---|
| **Binding pattern restrictions** | `binding_patterns.rs` | 28,46,55,72,81,106,131,191,206,276,285,313,382,392 | 14 |
| **Destructuring restrictions** | `expressions_destructure.rs` | 146,205,229,276,302,312 | 6 |
| Template interpolation | `helpers.rs` | 286,321,392 | 3 |
| Template escape sequences | `helpers.rs` | 487,495,506 | 3 |
| Class parsing errors | `statements_class.rs` | 11,151,203 | 3 |
| TS interface/type/namespace parse errors | `statements_ts.rs` | 134,189,314,682,794,930 | 6 |
| TS type annotation unterminated | `tokens.rs` | 349,500,581,606 | 4 |
| Compound assignment targets | `expressions_main.rs` | 270,405 | 2 |
| JSX syntax | `expressions_main.rs` | 96 | 1 |
| `await` outside async | `expressions_main.rs` | 1291 | 1 |
| `??` mixing with `&&`/`||` | `expressions_main.rs` | 724 | 1 |
| With statement | `expressions_main.rs` | 2191 | 1 |
| TS type expected | `expressions_main.rs` | 1801 | 1 |
| Unterminated TS annotation | `expressions_main.rs` | 1852 | 1 |
| Template literal catch-all | `expressions_main.rs` | 2648 | 1 |
| Static in constructor body | `statements_general.rs` | 38 | 1 |
| Module export forms (unsupported) | `statements_general.rs` | 279,823 | 2 |
| LHS assignment validation | `statements_general.rs` | 905,998 | 2 |
| Semicolon/let/const errors | `statements_general.rs` | 1081,1108 | 2 |
| Const initializer required | `statements_general.rs` | 1134,1141,1163,1170 | 4 |
| Assignment operator expected | `statements_general.rs` | 1228 | 1 |
| `for await` outside async | `statements_general.rs` | 1697 | 1 |
| `for` loop syntax | `statements_general.rs` | 1854 | 1 |
| Switch/case syntax | `statements_general.rs` | 1962 | 1 |
| Try/catch/finally syntax | `statements_general.rs` | 2011 | 1 |
| Unterminated block | `statements_general.rs` | 2063,2095 | 2 |

**Total parser-level**: ~64

### 1.3 IR-level resolver rejections

These are the most significant category for test262 coverage — valid syntax that hits ResolvedExpr lowering boundaries.

#### 1.3.1 Builtin/global constructors and functions

| Feature | File | Lines | Count |
|---|---|---|---|
| **Proxy** constructor | `call/constructor.rs` | 110,121 | 2 |
| **Reflect** API | `call/constructor.rs` | 121 | 1 |
| **Date(string)** parsing | `call/constructor.rs` | 204 | 1 |
| **new Array(length)** restrictions | `call/constructor.rs` | 228 | 1 |
| **new Class(iterable)** unsupported | `call/constructor.rs` | 294 | 1 |
| **new Promise()** without executor | `call/constructor.rs` | 337 | 1 |
| **DataView** constructor | `call/constructor.rs` | 442,451 | 2 |
| **Intl** constructors | `call/constructor.rs` | 284,365,377 | 3 |
| **String()** call | `semantic.rs` | 1413 | 1 |
| **require()** unsupported | `builtin_resolver.rs` | 2096 | 1 |
| **process.\*** unsupported | `builtin_resolver_host.rs` | 236,252,670 | 3 |
| **console.\*** unsupported | `builtin_resolver_host.rs` | 219,236,670 | 3 |
| **Bun.file().text()** | `builtin_resolver_host.rs` | 343 | 1 |
| **RegExp** constructor flags | `program_builtins.rs` | 1034 | 1 |
| **RegExp.prototype.compile** | `program_builtins.rs` | 1394 | 1 |
| **JSON.stringify** space restrictions | `program_builtins.rs` | 614 | 1 |

#### 1.3.2 Builtin method restrictions (most impactful for test262)

| Feature | File | Lines | Count |
|---|---|---|---|
| **Array methods** (from, of, map, reduce, sort, etc.) | `resolver/array.rs` | 107-2327 | 18 |
| **String methods** unsupported in milestone | `call/method.rs` | 2391 | 1 |
| **String.prototype.matchAll** | `call/spread.rs` | 294,315,374,391 | 4 |
| **Function.prototype.apply** restrictions | `call/method.rs` | 792,844,852,3822 | 4 |
| **Function.prototype.bind** restrictions | `call/user.rs` | 714 | 1 |
| **JSON.parse** reviver restrictions | `call/method.rs` | 2767,2775,2801 | 3 |
| **Array.prototype.push** multi-arg | `call/method.rs` | 2801 | 1 |
| **Object.groupBy** | `resolver/array.rs` | 1017,1033,1041 | 3 |
| **Map.groupBy** | `resolver/array.rs` | 1219,1234,1242 | 3 |
| **TypedArray.from** | `call/method.rs` | 1176 | 1 |
| **Intl.\*** methods (NumberFormat, DateTimeFormat, DurationFormat, ListFormat) | `call/method.rs` | 4718,4752,4785,4893,4955,4976,4997 | 7 |
| **Set.prototype.forEach** callback restrictions | `call/callback.rs` | 25,38,59,76,94,108 | 6 |
| **Map.prototype.forEach** callback restrictions | `call/callback.rs` | 239,252,273,290,308,322 | 6 |
| **Array.prototype.map** with user callbacks | `resolver/mod.rs` | 1947 | 1 |
| **Array.prototype.sort** restrictions | `resolver/mod.rs` | 1947 | 1 |

#### 1.3.3 Super/this/class restrictions

| Feature | File | Lines | Count |
|---|---|---|---|
| Super property access | `expr/property.rs` | 663,687,707,729 | 4 |
| Super property assignment | `expr/assignment.rs` | 578,590,615,627 | 4 |
| Super method call | `call/method.rs` | 4017,4050,4063,4077 | 4 |
| Super constructor call | `call/user.rs` | 352,365,378,391 | 4 |
| Super property in control flow | `expr/control.rs` | 164 | 1 |
| `this.method(...)` requires class context | `call/method.rs` | 3378,3387 | 2 |
| Private field access restrictions | `expr/property.rs` | 912,926,943,951,968,996 | 6 |
| Private field assignment restrictions | `expr/assignment.rs` | 462,485,502,516,544 | 5 |
| Private method call restrictions | `call/method.rs` | 953,965,977 | 3 |
| Private field backing storage | `resolver/mod.rs` | 1849 | 1 |
| Private field logical assignment | `builtin_resolver.rs` | 2247 | 1 |
| Private field optional chaining | `builtin_resolver.rs` | 2361 | 1 |
| Private class features (general) | `builtin_resolver_class_features.rs` | 35-637 | 7 |
| Class inheritance restrictions | `builtin_resolver.rs` | 1512,1636 | 2 |
| Class prototype restrictions | `builtin_resolver.rs` | 2521 | 1 |

#### 1.3.4 Control flow / statement restrictions

| Feature | File | Lines | Count |
|---|---|---|---|
| `throw` statement | `semantic.rs` | 1413 | 1 |
| `try`/`catch` | `semantic.rs` | 1413 | 1 |
| `for(;;)` C-style loops | `semantic.rs` | 1413 | 1 |
| `switch` statement | `semantic.rs` | 1413 | 1 |
| `do`/`while` | `semantic.rs` | 1413 | 1 |
| `break` | `semantic.rs` | 1413 | 1 |
| `continue` | `semantic.rs` | 1413 | 1 |
| `for-in` | `semantic.rs` | 1413 | 1 |
| `for-of` | `semantic.rs` | 1413 | 1 |
| Labeled statements | `semantic.rs` | 1413 | 1 |
| Block statements | `semantic.rs` | 1413 | 1 |
| Nested function declarations | `semantic.rs` | 1413 | 1 |
| Ternary expression | `semantic.rs` | 1413 | 1 |
| Assignment expression (as expr) | `semantic.rs` | 1413 | 1 |
| `typeof` operator | `semantic.rs` | 1413 | 1 |
| Dynamic function calls | `semantic.rs` | 1413 | 1 |
| Spread in non-call context | `expr/control.rs` | 216 | 1 |
| Optional chaining | `call/optional.rs` | 17,64 | 2 |
| `yield*` delegation | `resolver/mod.rs` | 411, `expr/mod.rs` 52 | 2 |

#### 1.3.5 BigInt-specific restrictions

| Feature | File | Lines | Count |
|---|---|---|---|
| Invalid BigInt literals | `builtin_resolver_bigint.rs` | 6,30 | 2 |
| BigInt(string) restrictions | `builtin_resolver_bigint.rs` | 411,422,432 | 3 |
| BigInt dynamic arithmetic limits | `builtin_resolver_bigint_ops.rs` | 24,34,44,56,67 | 5 |
| BigInt comparison limits | `builtin_resolver_bigint.rs` | 1546,1556,1566 | 3 |
| BigInt operator boundaries | `builtin_resolver.rs` | 2066 | 1 |

#### 1.3.6 Function call restrictions

| Feature | File | Lines | Count |
|---|---|---|---|
| Non-identifier call expression | `call/user.rs` | 160,175 | 2 |
| Callable interface check | `call/user.rs` | 479 | 1 |
| Constructor without new | `call/user.rs` | 579 | 1 |
| Direct call receiver binding | `call/user.rs` | 682 | 1 |
| Function metadata (.length, .name) | `call/user.rs` | 1417,1433 | 2 |
| Nested function mutable capture | `resolver/array.rs` | 556 | 1 |
| `arguments` in non-arrow | `expr/control.rs` | 177 | 1 |
| Direct eval IIFE | `resolver/mod.rs` | 373 | 1 |
| Rest parameter spread call | `call/user.rs` | 813,823 | 2 |
| Unary delete restrictions | `expr/unary.rs` | 92,107 | 2 |

#### 1.3.7 Assignment restrictions

| Feature | File | Lines | Count |
|---|---|---|---|
| Assignment to eval/arguments | `expr/assignment.rs` | 25 | 1 |
| Increment/decrement update | `builtin_resolver.rs` | 2667 | 1 |
| `++`/`--` prefix/postfix | `expr/assignment.rs` | 544-627 | 8 |

#### 1.3.8 Misc / catch-all

| Feature | File | Lines | Count |
|---|---|---|---|
| Binary operator not supported | `program.rs` | 5269 | 1 |
| Unary operator not supported | `program.rs` | 5295,5303 | 2 |
| Rest parameter binding patterns | `program.rs` | 4986 | 1 |
| Callback lowering failures | `call/callback.rs` | 25-322 | 12 |
| `issue-051` general unsupported | `program_builtins.rs` | 1385 | 1 |

---

## 2. Top 5 Patterns Fixable in Frontend/IR Only

These patterns only require changes in `crates/frontend/` or `crates/ir/` — no WAT runtime changes needed.

### #1: Rest/binding pattern restrictions (`issue-247`)
- **File**: `crates/frontend/src/parser/binding_patterns.rs`
- **Lines**: 28, 46, 55, 72, 81, 106, 131, 191, 206, 276, 285, 313, 382, 392
- **Lines**: `crates/frontend/src/parser/statements_general.rs` L1134, L1141, L1163, L1170
- **Description**: Many binding pattern forms (parameter properties, `this` parameters, rest patterns, literal object keys, etc.) are rejected at parse time. Some of these are strict TypeScript forms that likely don't affect test262 (which is JS-only).
- **Fixability**: Medium — many are TS-only diagnostics; JS-relevant ones like binding patterns with initializers or rest patterns would benefit test262. These emit `issue-247` which may already be partially test262-covered.

### #2: Destructuring assignment restrictions (`issue-252`)
- **File**: `crates/frontend/src/parser/expressions_destructure.rs`
- **Lines**: 146, 205, 229, 276, 302, 312
- **Description**: Array/object destructuring assignment patterns are rejected at parse/IR level.
- **Fixability**: High — destructuring is a core JS feature with high test262 coverage. Lowering destructuring in the IR resolver would directly reduce the count. No backend-wasm changes needed if lowered to existing IR patterns.

### #3: Private class field/method restrictions (`issue-255`)
- **Files**:
  - `crates/ir/src/lowered/resolver/expr/property.rs` — L912, L926, L943, L951, L968, L996
  - `crates/ir/src/lowered/resolver/expr/assignment.rs` — L462, L485, L502, L516, L544
  - `crates/ir/src/lowered/resolver/call/method.rs` — L953, L965, L977
  - `crates/ir/src/builtin_resolver.rs` — L2247, L2361
  - `crates/ir/src/builtin_resolver_class_features.rs` — L35-L637
- **Description**: Private class members (`#field`, `#method`) have many restrictions in the resolver — static private fields, private getters/setters, private method extraction, logical assignment, optional chaining. Some restrictions are just about fixing the resolver logic to allow more patterns.
- **Fixability**: Medium-High — the IR resolver already has private field lowering machinery (it compiles `#x` to `PrivateFieldGet`/`PrivateFieldSet`). Many restrictions are conservative guards that could be relaxed. No backend changes needed.

### #4: Super property/method restrictions
- **Files**:
  - `crates/ir/src/lowered/resolver/expr/property.rs` — L663, L687, L707, L729
  - `crates/ir/src/lowered/resolver/expr/assignment.rs` — L578, L590, L615, L627
  - `crates/ir/src/lowered/resolver/call/method.rs` — L4017, L4050, L4063, L4077
  - `crates/ir/src/lowered/resolver/call/user.rs` — L352, L365, L378, L391
  - `crates/ir/src/lowered/resolver/expr/control.rs` — L164
- **Description**: `super` keyword usage in computed property access, method calls, constructor calls, and property assignments has many context-dependent restrictions (requires class context, requires extends, etc.).
- **Fixability**: Medium — the IR resolver already handles basic `super` patterns; these restrictions guard edge cases. Some could be relaxed in the resolver alone if the underlying IR can represent them.

### #5: Array.from / TypedArray.from restrictions (`issue-313`, `issue-1176`)
- **File**: `crates/ir/src/lowered/resolver/array.rs` — L269, L305
- **File**: `crates/ir/src/lowered/resolver/call/method.rs` — L1176
- **Description**: `Array.from(source, mapFn, thisArg)` has argument-count and callback-type restrictions. `TypedArray.from` with mapFn/thisArg is outright rejected.
- **Fixability**: High — these are purely IR resolver guards. The runtime for iteration already exists. Only need to wire the callback dispatch in the resolver.

---

## 3. Raw Usage Counts by File (excl. tests)

| File | Usages | Primary Category |
|---|---|---|
| `crates/ir/src/lowered/resolver/call/method.rs` | 33 | Method dispatch restrictions |
| `crates/ir/src/lowered/resolver/call/user.rs` | 15 | Function call restrictions |
| `crates/ir/src/lowered/resolver/call/callback.rs` | 12 | Callback lowering failures |
| `crates/ir/src/lowered/resolver/call/constructor.rs` | 11 | Constructor restrictions |
| `crates/ir/src/lowered/resolver/expr/property.rs` | 10 | Property access restrictions |
| `crates/ir/src/lowered/resolver/expr/assignment.rs` | 10 | Assignment restrictions |
| `crates/ir/src/lowered/resolver/call/spread.rs` | 9 | Spread call restrictions |
| `crates/ir/src/builtin_resolver_host.rs` | 8 | Host API restrictions |
| `crates/ir/src/builtin_resolver_bigint_ops.rs` | 7 | BigInt arithmetic restrictions |
| `crates/ir/src/builtin_resolver_class_features.rs` | 7 | Private class features |
| `crates/ir/src/lowered/resolver/mod.rs` | 7 | Various IR boundaries |
| `crates/ir/src/builtin_resolver_bigint.rs` | 10 | BigInt literal/conversion restrictions |
| `crates/ir/src/builtin_resolver.rs` | 12 | Builtin class/method restrictions |
| `crates/ir/src/lowered/program_builtins.rs` | 10 | Builtin function restrictions |
| `crates/ir/src/lowered/resolver/array.rs` | 18 | Array method restrictions |
| `crates/ir/src/lowered/resolver/function.rs` | 6 | Function resolver restrictions |
| `crates/ir/src/lowered/resolver/class.rs` | 5 | Class resolver restrictions |
| `crates/ir/src/builtin_resolver_outer.rs` | 2 | Outer builtin fallthrough |
| `crates/ir/src/lowered/program.rs` | 4 | Operator/binding fallthrough |
| `crates/ir/src/lowered/resolver/string.rs` | 1 | String method fallthrough |
| `crates/ir/src/lowered/resolver/expr/binary.rs` | 1 | Binary operator fallthrough |
| `crates/ir/src/lowered/resolver/expr/facts.rs` | 1 | Symbol iterator spread |
| `crates/ir/src/lowered/resolver/expr/unary.rs` | 2 | Unary delete restrictions |
| `crates/ir/src/lowered/resolver/expr/control.rs` | 3 | Spread/arguments restrictions |
| `crates/ir/src/lowered/resolver/expr/binding.rs` | 1 | Binding pattern lowering |
| `crates/ir/src/lowered/resolver/expr/mod.rs` | 2 | Expression lowering catch-all |
| `crates/ir/src/lowered/resolver/call/builtin.rs` | 1 | Builtin method catch-all |
| `crates/ir/src/lowered/resolver/call/optional.rs` | 2 | Optional chaining |
| `crates/ir/src/lowered/resolver/object.rs` | 3 | Object resolver catch-all |

---

## 4. Test Coverage Status (from `semantic.rs`)

The HIR semantic test suite (`crates/ir/src/semantic.rs` L2013-2253) explicitly tests that these language features are rejected:

- Nested function declarations
- `throw` statement
- `try`/`catch`
- `for(;;)` loops
- `switch` statement
- `do`/`while` loops
- `break` / `continue`
- `for-in` / `for-of`
- Labeled statements
- Block statements (bare `{}`)
- Ternary expressions (`? :`)
- Assignment expressions as values
- `String()` call
- Unsupported binary operators
- Dynamic function calls
- `typeof` operator

All of these use a catch-all message: `"not part of the initial HIR slice"` — meaning they are structurally parsed but intentionally excluded from the initial HIR.

---

## 5. Recommendations

1. **Top priority for frontend/IR-only fixes**: Destructuring patterns (#2) and Array.from restrictions (#5) are the most self-contained, with no backend-wasm dependency.

2. **Private class features (#3)** has the largest absolute count of guard restrictions in the IR layer and high test262 relevance (private fields/methods are ES2022). Many guards could be relaxed incrementally.

3. **Super keyword (#4)** restrictions are spread across 5 files and have complex class-context dependencies, but fixing them in the resolver would unlock many test262 class-related tests.

4. **The 11489 test262 case count** likely includes many duplicate reports from the same few features. The ~349 production code sites of `UnsupportedSyntax` each map to roughly 33 test262 cases on average.
