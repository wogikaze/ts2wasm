# Issues Index

This file is the human entrypoint for the issue queue.

Issue files are the source of truth for work items. The generated section below may be replaced by a script or pasted manually from a generated report.

## Summary

<!-- generated:summary:start -->
| Area | Total | Open | Resolved |
|---|---:|---:|---:|
| abi | 7 | 0 | 7 |
| backend | 13 | 1 | 12 |
| backend-wasm | 2 | 1 | 1 |
| cli | 15 | 0 | 15 |
| compiler | 25 | 13 | 12 |
| coverage | 42 | 0 | 42 |
| docs | 2 | 0 | 2 |
| frontend | 4597 | 3313 | 1284 |
| harness | 1 | 0 | 1 |
| ir | 84 | 61 | 23 |
| issues | 4 | 0 | 4 |
| parser | 1 | 0 | 1 |
| reference | 215 | 144 | 71 |
| runtime | 263 | 82 | 181 |
| scripts | 3 | 1 | 2 |
| security | 1 | 0 | 1 |
| tests | 6 | 0 | 6 |
| wasi | 1 | 0 | 1 |
| total | 5282 | 3616 | 1666 |
<!-- generated:summary:end -->

## Reading rules

- Start with `Ready queue`.
- Check `Dependency graph` for implementation order and parent-child relationships between meta issues.
- Check `Blocked queue` only after ready work is exhausted.
- Do not use `done/` as current project truth.
- For docs work, verify whether the issue updates final-state docs, `current-state.md` (repo root), or follow-up issues.
- For implementation work, verify acceptance criteria and validation commands before starting.

## Dependency graph

<!-- generated:dep-graph:start -->
### Meta issue dependency tree

Direct child counts are derived from issue-file `depends_on` links. A meta issue can be `done` as a classification/design umbrella while implementation child issues remain open.

```
5000 (Meta: TypeScript Compiler Parser Syntax Coverage) [done/design] ch:446 open:354 done:92
├── 5002 (Meta: TypeScript Compiler Type System Coverage) [done/done] ch:223 open:203 done:20 (also ← 5005)
├── 5003 (Meta: TypeScript Compiler Declaration Emit Coverage) [done/done] ch:100 open:89 done:11 (also ← 5001)
5001 (Meta: TypeScript Compiler Semantic Analysis Coverage) [done/done] ch:556 open:296 done:260
5004 (Meta: Runtime Builtins Coverage (test262) (audit reopened #5004)) [done/done] ch:24 open:3 done:21
5005 (Meta: TypeScript Compiler Name Resolution Coverage) [done/done] ch:397 open:336 done:61
├── 5006 (Meta: TypeScript Compiler Scope Analysis Coverage) [done/done] ch:28 open:20 done:8
├── 5007 (Meta: TypeScript Compiler Module Resolution Coverage (audit reopened #5007)) [done/done] ch:20 open:12 done:8
```

### Multi-parent notes

- **5002** (Meta: TypeScript Compiler Type System Coverage) also depends on **5005** - shown under primary parent **5000** in tree above
- **5003** (Meta: TypeScript Compiler Declaration Emit Coverage) also depends on **5001** - shown under primary parent **5000** in tree above

### Meta issue overview

| Order | ID | Title | State | Class | Area | Priority | Depends on | Direct children | Open children | Done children |
|-----:|---:|------|-------|-------|------|--------:|-----------:|----------------:|--------------:|--------------:|
| 1 | 5000 | Meta: TypeScript Compiler Parser Syntax Coverage | done | design | frontend/syntax | P1 | - | 446 | 354 | 92 |
| 2 | 5001 | Meta: TypeScript Compiler Semantic Analysis Coverage | done | done | frontend/semantics | P1 | - | 556 | 296 | 260 |
| 3 | 5004 | Meta: Runtime Builtins Coverage (test262) (audit reopened #5004) | done | done | runtime/builtins | P1 | - | 24 | 3 | 21 |
| 4 | 5005 | Meta: TypeScript Compiler Name Resolution Coverage | done | done | frontend/resolver | P1 | - | 397 | 336 | 61 |
| 5 | 5002 | Meta: TypeScript Compiler Type System Coverage | done | done | frontend/semantics | P1 | 5000, 5005 | 223 | 203 | 20 |
| 6 | 5003 | Meta: TypeScript Compiler Declaration Emit Coverage | done | done | frontend/syntax | P2 | 5000, 5001 | 100 | 89 | 11 |
| 7 | 5006 | Meta: TypeScript Compiler Scope Analysis Coverage | done | done | frontend/resolver | P2 | 5005 | 28 | 20 | 8 |
| 8 | 5007 | Meta: TypeScript Compiler Module Resolution Coverage (audit reopened #5007) | done | done | frontend/resolver | P2 | 5005 | 20 | 12 | 8 |

### Topological order

| Order | ID | Title | State | Class | Priority | Level | Depends on |
|-----:|---:|------|-------|-------|--------:|------:|-----------:|
| 1 | 5000 | Meta: TypeScript Compiler Parser Syntax Coverage | done | design | P1 | 0 | - |
| 2 | 5001 | Meta: TypeScript Compiler Semantic Analysis Coverage | done | done | P1 | 0 | - |
| 3 | 5004 | Meta: Runtime Builtins Coverage (test262) (audit reopened #5004) | done | done | P1 | 0 | - |
| 4 | 5005 | Meta: TypeScript Compiler Name Resolution Coverage | done | done | P1 | 0 | - |
| 5 | 5002 | Meta: TypeScript Compiler Type System Coverage | done | done | P1 | 1 | 5000, 5005 |
| 6 | 5003 | Meta: TypeScript Compiler Declaration Emit Coverage | done | done | P2 | 1 | 5000, 5001 |
| 7 | 5006 | Meta: TypeScript Compiler Scope Analysis Coverage | done | done | P2 | 1 | 5005 |
| 8 | 5007 | Meta: TypeScript Compiler Module Resolution Coverage (audit reopened #5007) | done | done | P2 | 1 | 5005 |
<!-- generated:dep-graph:end -->

## Ready queue

<!-- generated:ready:start -->
| ID | Title | Type | Area | Class | Priority | Depends on | Summary |
|---:|---|---|---|---|---|---|---|
| 353 | Implement iterator protocol integration for spread operator | feature | runtime/semantics | unstarted | P2 |  | General iterator protocol is not implemented for spread operator. |
| 711 | Report TS1108 for top-level return statements | feature | compiler/diagnostics | implementation-ready | P2 |  | top-level `return` currently reports unsupported `InvalidTopLevelReturn` instead of a TS1108-style diagnostic. |
| 1999 | Report symbol WeakSet.add diagnostics | feature | frontend/resolver | implementation-ready | P1 |  | weak collection symbol negative tests currently fail with |
| 5132 | Add ABC451 non-top array separation attribution | infra | runtime/memory | implementation-ready | P1 | 5131 | issue 365 proves the dominant array-growth pressure is |
| 5137 | Split remaining Date API scope | cleanup | runtime/builtins | design-ready | P1 |  | Date issue 050 currently stays open after its named child issues 240 and 241 are done, but non-literal constructor in... |
| 5138 | Split Reflect.construct isConstructor reference window | spike | runtime/builtins | design-ready | P1 |  | `reference/test262/test/annexB/built-ins/String/prototype/anchor/not-a-constructor.js` currently fails with `Unresolv... |
| 5139 | Split APISample parser leftovers | cleanup | frontend/syntax | design-ready | P1 |  | issue 070 identified `APISample_linter.ts` and `APISample_transform.ts` as remaining parser/frontend leftovers, but t... |
| 5143 | Implement unary void operator lowering | feature | ir/lowered | implementation-ready | P1 |  | unary `void` currently fails with `UnsupportedSyntax` in lowering even though TypeScript accepts the reference case w... |
| 5144 | Support entry-module export function declarations | feature | ir/compiler | implementation-ready | P1 |  | entry-module `export function` declarations currently fail with `UnsupportedModule`, even though nearby static export... |
| 5145 | Parse await as an identifier call outside async contexts | feature | frontend/syntax | implementation-ready | P2 |  | `await(...)` in a sync function currently fails with `UnsupportedRuntimeSubset` instead of preserving the TypeScript ... |
| 5146 | Report for-await context errors before async runtime diagnostics | feature | frontend/syntax | implementation-ready | P2 |  | `for await...of` outside async/top-level-allowed contexts currently reports the broad async runtime unsupported messa... |
| 5147 | Report await expression context errors before runtime diagnostics | feature | frontend/syntax | implementation-ready | P2 |  | `await <literal>` in a non-async function currently fails with `UnsupportedRuntimeSubset` instead of a TypeScript-ali... |
| 5149 | Parse trailing comma in typed class method parameters | feature | frontend/syntax | implementation-ready | P1 |  | class method parameter lists with a trailing comma currently fail with `UnsupportedSyntax`, even though TypeScript ac... |
| 5151 | Parse ASI after multiline const initializer | feature | frontend/syntax | implementation-ready | P1 |  | semicolonless `const` declarations with multi-line call/object initializers currently fail before the next expression... |
| 5152 | Support class constructor outer callback captures | feature | ir | implementation-ready | P1 |  | class constructors cannot currently call outer callback locals when nested arrow callbacks capture constructor `this`. |
| 5153 | Erase union types in as assertions | feature | frontend/syntax | implementation-ready | P1 |  | `expr as A \| B` is currently parsed as runtime bitwise-or instead of erasing the whole union type annotation. |
| 5154 | Parse angle-bracket type assertion statements | feature | frontend/syntax | implementation-ready | P1 |  | top-level `<T>expr;` type assertion statements currently fail parsing instead of erasing the type assertion. |
| 5155 | Fix exception_pending runtime link for top-level statement checks | bug | backend-wasm | implementation-ready | P1 |  | `reference/typescript/tests/cases/compiler/baseTypeAfterDerivedType.ts` reaches backend emission, then `wat2wasm` fai... |
| 5156 | Parse generic type arguments in class heritage clauses | feature | frontend/syntax | implementation-ready | P1 |  | `reference/typescript/tests/cases/compiler/baseTypeOrderChecking.ts` fails parsing on `class Class4<T> extends Class3... |
| 5157 | Report set accessor rest parameter diagnostics | feature | frontend/syntax | implementation-ready | P1 |  | `reference/typescript/tests/cases/compiler/accessorWithRestParam.ts` currently reaches backend emission and fails wit... |
| 5158 | Report interface private member clashes | feature | frontend/resolver | implementation-ready | P1 |  | `reference/typescript/tests/cases/compiler/baseTypePrivateMemberClash.ts` currently reports `BackendIo` instead of a ... |
| 5159 | Recover colon type annotations after expression statements | feature | frontend/syntax | implementation-ready | P1 |  | constructor-body statements of the form `this.<name>: any;` currently fail as parser-unsupported before the reference... |
| 5160 | Lower plain ternary conditional expressions | feature | frontend/semantics | implementation-ready | P2 |  | `reference/typescript/tests/cases/compiler/bestChoiceType.ts` currently stops in builtin resolution on `let y = x ? x... |
| 5161 | Model ambient value declarations for name resolution | feature | frontend/resolver | implementation-ready | P1 |  | `reference/typescript/tests/cases/compiler/bestCommonTypeWithContextualTyping.ts` currently reports `UnresolvedName` ... |
| 5162 | Allow compatible var redeclarations | feature | frontend/syntax | implementation-ready | P2 |  | `reference/typescript/tests/cases/compiler/duplicateLocalVariable3.ts` reports `DuplicateLocal` for `var x = 1; var x... |
| 5163 | Lower nested call expression callees | feature | frontend/semantics | implementation-ready | P2 |  | `reference/typescript/tests/cases/compiler/betterErrorForAccidentalCall.ts` currently reports `UnsupportedSyntax: onl... |
| 5164 | Parse exponentiation compound assignment | feature | frontend/syntax | implementation-ready | P1 |  | the BigInt target reference cases currently report `UnsupportedSyntax: expected Semicolon, got Some(PowerEqual)` for ... |
| 5165 | Support typed array subarray builtins | feature | ir/builtin-resolver | implementation-ready | P1 |  | Support typed array subarray builtins |
| 5166 | Parse string-literal module specifier aliases | feature | frontend/syntax | implementation-ready | P1 |  | `reference/typescript/tests/cases/compiler/bigintArbirtraryIdentifier.ts` currently reports `UnsupportedSyntax: expec... |
| 5167 | Support global Symbol builtin call | feature | ir/builtin-resolver | implementation-ready | P1 |  | `reference/typescript/tests/cases/compiler/bigintIndex.ts` currently reports `UnresolvedFunction: unresolved function... |
| 5168 | Report BigInt property-name diagnostics | feature | frontend/syntax | implementation-ready | P1 |  | `reference/typescript/tests/cases/compiler/bigintPropertyName.ts` currently reports `UnsupportedSyntax: expected iden... |
| 5169 | Parse ASI after expression statement | feature | frontend/syntax | implementation-ready | P1 |  | the BigInt lib reference cases report `UnsupportedSyntax: expected Semicolon, got Some(Let)` after: |
| 5170 | Support bitwise OR binary lowering | feature | ir/lowering | implementation-ready | P1 |  | ordinary number/null/undefined bitwise OR expressions parse successfully but cannot be lowered. |
| 5171 | Accept unsigned 32-bit hex literals | feature | frontend/lexer | implementation-ready | P1 |  | non-decimal number literals above `i32::MAX` are rejected during lexing, so parser and lowering triage cannot reach t... |
| 5172 | Report unresolved implements in erased namespace | feature | frontend/semantics | implementation-ready | P1 |  | erased namespace declarations can hide unresolved class `implements` clauses and produce a false build pass. |
| 5173 | Avoid stack overflow on deep binary expressions | feature | ir/builtin-resolver | implementation-ready | P1 |  | recursive expression folding in builtin resolution cannot process the deep binary-expression stress references. |
| 5174 | Ignore empty binding pattern synthetic names | feature | frontend/name-resolution | implementation-ready | P1 |  | empty binding patterns do not declare a local binding, but the compiler currently registers their display text as if ... |
| 5175 | Support export let declarations | feature | frontend/module-syntax | implementation-ready | P1 |  | the parser has an `export const <ident> = ...` slice, but `export let` declarations still stop at the generic variabl... |
| 5176 | Report ambient var lib redeclaration diagnostics | feature | frontend/semantics | implementation-ready | P1 |  | ambient `declare var` declarations can conflict with lib globals, but ts2wasm currently erases the declaration and re... |
| 5177 | Report strict-null diagnostics in erased namespace methods | feature | frontend/semantics | implementation-ready | P1 |  | erased namespace class method bodies can hide typed local declaration diagnostics and produce a false build pass. |
| 5178 | Parse bitwise compound assignment operators | feature | frontend/syntax | implementation-ready | P1 |  | bitwise compound assignment operators `^=`, `&=`, and `\|=` fail in parser/frontend syntax before semantic diagnostic... |
| 5179 | Report implicit this before closure runtime guard | feature | frontend/semantics | implementation-ready | P1 |  | a TypeScript implicit-`this` diagnostic is hidden by the nested-function closure runtime-subset guard. |
| 5180 | Parse computed property object binding patterns | feature | frontend/syntax | implementation-ready | P1 |  | computed property names in object binding patterns are parser-unsupported, blocking `TS2448` used-before-definition c... |
| 5181 | Support prefix update expressions in call arguments | feature | frontend/semantics | implementation-ready | P1 |  | identifier prefix update expressions in call arguments are parser-accepted but resolver/lowering-unsupported. |
| 5182 | Parse comma-separated for update expressions | feature | frontend/syntax | implementation-ready | P1 |  | comma-separated for-loop update expressions are parser-unsupported, blocking the block-scoped loop reassignment refer... |
| 5183 | Report typed getter null return diagnostics | feature | frontend/semantics | implementation-ready | P1 |  | typed class getter return annotations are erased before return-expression diagnostics are checked. |
| 5184 | Parse const enum declarations | feature | frontend/syntax | implementation-ready | P1 |  | `const enum` declarations are parser-unsupported and are misclassified as malformed `const` variable declarations. |
| 5185 | Source-span unresolved class method function calls | feature | frontend/resolver | implementation-ready | P1 |  | unresolved function calls from class method bodies lose source-span information and should report the out-of-scope ca... |
| 5186 | Parse export assignment expressions | feature | frontend/syntax | implementation-ready | P1 |  | `export = expr` is treated as an unsupported module boundary before the exported expression can be represented in the... |
| 5187 | Lower namespace-only multi-section files | feature | compiler | implementation-ready | P1 |  | multi-section TypeScript reference files that contain namespace declarations but no static imports/exports are reduce... |
| 5188 | Report block-scoped function call arity diagnostics | feature | frontend/semantics | implementation-ready | P1 |  | user-defined function calls that resolve to block-scoped same-name declarations can build even when TypeScript report... |
| 5189 | Parse ASI after class expression variable initializer | feature | frontend/syntax | implementation-ready | P1 |  | semicolonless variable declarations whose initializer is an anonymous class expression do not accept ASI before the n... |
| 5190 | Skip implements in ambient class declarations | feature | frontend/syntax | implementation-ready | P1 |  | ambient class declaration parsing does not skip `implements` heritage clauses. |
| 5191 | Parse leading decimal numeric literals | feature | frontend/syntax | implementation-ready | P1 |  | `builtinIterator.ts` stops at a generic `unsupported expression` parser diagnostic before it can reach the intended `... |
| 5192 | Support first-class class constructor values | feature | ir/runtime | implementation-ready | P1 |  | class constructor bindings used as expression values still fail with `issue-5011`, blocking reference cases that pass... |
| 5193 | Parse ASI after ambient variable declarations | feature | frontend/syntax | implementation-ready | P1 |  | declaration-only ambient variables followed by a newline and another statement can still report `issue-400` instead o... |
| 5194 | Report empty call type arguments | feature | frontend/syntax | implementation-ready | P1 |  | `Foo<a,,b>();` currently reports `expected Semicolon, got Some(Comma)` instead of a source-spanned missing type-argum... |
| 5195 | Support callable interface-typed local calls | feature | ir/lowering | implementation-ready | P1 |  | callable interface-typed locals currently lower to `Undefined` values and calls to them stop with `issue-211`. |
| 5196 | Support callable conditional-typed parameter calls | feature | ir/lowering | implementation-ready | P1 |  | callable parameters typed through conditional type aliases currently fall into the generic `issue-211` function-value... |
| 5197 | Report class called without new | feature | frontend/resolver | implementation-ready | P1 |  | direct calls to class constructors without `new` currently report generic `issue-5011` class-value unsupported diagno... |
| 5198 | Support class method overload signatures for element access calls | feature | frontend/resolver | implementation-ready | P1 |  | class method overload signatures are currently treated as duplicate method definitions. |
| 5199 | Report function overload list class merge diagnostics | feature | frontend/resolver | implementation-ready | P1 |  | top-level bodyless function overload declarations are currently handled as duplicate concrete function implementations. |
| 5200 | Validate top-level function overload implementations | feature | frontend/resolver | implementation-ready | P1 |  | top-level function overload implementation groups are currently classified as duplicate concrete functions. |
| 5201 | Parse object type literal call signatures | feature | frontend/parser | implementation-ready | P1 |  | TypeScript object type literals with call-signature members are not parsed as complete type annotations. |
| 5202 | Parse member call explicit type arguments | feature | frontend/parser | implementation-ready | P1 |  | explicit type arguments after member call callees are not parsed or erased. |
| 5203 | Report indexed new type-only callee diagnostics | feature | frontend/resolver | implementation-ready | P1 |  | indexed `new` callees that start with type-only identifiers fall into the generic issue-062 class-name requirement. |
| 5204 | Resolve lexical super property captures in super call arguments | feature | frontend/resolver | implementation-ready | P1 |  | lexical `super` property access inside arrow arguments to `super(...)` is not resolved against the derived instance c... |
| 5205 | Report incompatible var redeclaration type diagnostics | feature | frontend/resolver | implementation-ready | P2 |  | incompatible same-scope `var` redeclarations can build successfully instead of reporting a TS2403-style diagnostic. |
| 5206 | Hoist loop-body var declarations for post-loop reads | feature | frontend/resolver | implementation-ready | P2 | 5006 | loop-body `var v` is not registered in the enclosing function var |
| 5207 | Parse do-while ASI before following for | feature | frontend/syntax | implementation-ready | P2 | 5000 | capturedLetConstInLoop parser tests currently fail because the parser |
| 5208 | Parse arrow body destructuring assignments | feature | frontend/syntax | implementation-ready | P2 | 5000 | the parser treats `[i]` in an arrow body as a complete expression and |
| 5209 | Parse computed object literal property expressions | feature | frontend/syntax | implementation-ready | P2 | 5000 | the parser expects a simple dotted/key form inside computed object |
| 5210 | Parse do-while ASI before block end or expression | feature | frontend/syntax | implementation-ready | P2 | 5000 | capturedLetConstInLoop parser tests still reject no-semicolon |
| 5211 | Parse do-while ASI before labeled statement | feature | frontend/syntax | implementation-ready | P2 | 5000 | capturedLetConstInLoop parser tests reject a no-semicolon |
| 5212 | Parse function expression statements in nested blocks | feature | frontend/syntax | implementation-ready | P2 | 5000 | capturedLetConstInLoop parser tests reject `(function() { return x })` |
| 5213 | Parse generator function expressions in parameter initializers | feature | frontend/syntax | implementation-ready | P2 | 5000 | capturedParametersInInitializers parser coverage rejects |
| 5214 | Parse computed class members in class expression initializers | feature | frontend/syntax | implementation-ready | P2 | 5000 | capturedParametersInInitializers parser coverage rejects `get [x]()` in |
| 5215 | Support loop-local arrow calls from arrow closures | feature | ir/lowering | implementation-ready | P2 | 5001 | capturedVarInLoop parses and resolves, but lowering rejects |
| 5216 | Accept large decimal exponent number literals | feature | frontend/lexer | implementation-ready | P2 |  | `castExpressionParentheses.ts` fails before token output because |
| 5217 | Support method calls on call expression receivers | feature | ir/lowering | implementation-ready | P2 | 5001 | `castFunctionExpressionShouldBeParenthesized.ts` parses successfully, |
| 5218 | Support nested function closures capturing this | feature | ir/runtime | implementation-ready | P2 |  | `castTest.ts` parses and erases its angle-bracket type assertions, but |
| 5219 | Report catch clause initializer diagnostics | feature | frontend/syntax | implementation-ready | P1 |  | `catchClauseWithInitializer1.ts` tokenizes successfully, but the parser |
| 5220 | Hoist for-initializer var declarations for sibling loop reads | feature | frontend/resolver | implementation-ready | P1 |  | `cf.ts` parses control-flow syntax successfully, but name resolution |
| 5221 | Support chained .then calls on call-expression receivers | feature | ir/lowering | implementation-ready | P1 |  | both `chainedCallsWithTypeParameterConstrainedToOtherTypeParameter` |
| 5222 | Support interface-typed method calls on erased locals | feature | ir/lowering | implementation-ready | P1 |  | Support interface-typed method calls on erased locals |
| 5223 | Parse computed properties after object spread | feature | frontend/parser | implementation-ready | P1 |  | Parse computed properties after object spread |
| 5224 | Parse destructuring assignment call arguments | feature | frontend/parser | implementation-ready | P1 |  | Parse destructuring assignment call arguments |
| 5225 | Support qualified class heritage names | feature | ir/resolver | implementation-ready | P1 |  | Support qualified class heritage names |
| 5226 | Allow ambient function overload declarations | feature | frontend/resolver | implementation-ready | P1 |  | Allow ambient function overload declarations |
| 5227 | Honor @ts-ignore for JavaScript call diagnostics | feature | frontend/diagnostics | implementation-ready | P1 |  | the representative reports issue-211 at byte span `130..133` for an |
| 5228 | Parse simple computed object literal keys | feature | frontend/parser | implementation-ready | P1 |  | `{ [n]: 1 }` reports `UnsupportedSyntax: expected Dot, got Some(RightBracket)` at `190..191`. |
| 5229 | Resolve imports between @Filename sections | feature | compiler/module-graph | implementation-ready | P1 |  | `import b from "./b"` in a `// @Filename: c.js` section reports `issue-232: missing local module ./b`. |
| 5230 | Tokenize JSX elements before RegExp fallback | feature | frontend/lexer | implementation-ready | P1 |  | `</div>` in a JSX element reports `UnsupportedRegExp: unterminated RegExp literal`. |
| 5231 | Parse export as namespace declarations | feature | frontend/parser | implementation-ready | P1 |  | `export as namespace THREE;` reports `UnsupportedModule: issue-055: unsupported static export`. |
| 5232 | Support entry-module export class declarations | feature | ir/compiler | implementation-ready | P1 |  | entry-module `export class Foo {}` currently reports `UnsupportedModule: issue-5005: entry module ... uses a declarat... |
| 5233 | Report super call in non-derived class | feature | ir/lowering | implementation-ready | P1 |  | `super()` in a class without `extends` reports `UnsupportedSyntax: super(...) used in class without extends` instead ... |
| 5234 | Track array-typed parameters for callback methods | feature | frontend/semantics | implementation-ready | P1 |  | class method parameter `x: Array<string>` reports `UnsupportedSyntax: issue-211: unknown receiver class for method fo... |
| 5235 | Erase type predicate object return types | feature | frontend/parser | implementation-ready | P1 |  | `function f(x: any): x is { a: string; a: string; } { return true; }` reports `InvalidTopLevelReturn` because the ret... |
| 5236 | Support nested function rest parameters in closure lowering | feature | ir | implementation-ready | P1 |  | nested function expressions with rest parameters report |
| 5237 | Parse this-property computed object literal keys | feature | frontend/parser | implementation-ready | P1 |  | `{ [this.a]: "" }` reports `UnsupportedSyntax: expected identifier, |
| 5238 | Preserve template interpolation expression spans | feature | frontend/parser | implementation-ready | P1 |  | `` `${a}` `` reports `UnresolvedName: unresolved name: a at 0..1` |
| 5239 | Bind nested class declarations in function scopes | bug | ir | implementation-ready | P1 |  | a nested class declaration inside a function body is not bound as a local value/type name for later statements in tha... |
| 5240 | Parse async arrow function expressions | feature | frontend/syntax | implementation-ready | P1 |  | `async () => { ... }` and `(async () => { ... })` are rejected as `unsupported expression: Async` before AST construc... |
| 5241 | Parse spread arguments in new expressions | bug | frontend/syntax | implementation-ready | P1 |  | spread arguments in constructor calls are rejected as raw `DotDotDot` parser errors before class/type-system behavior... |
| 5242 | Parse direct generic call type arguments for callable consts | feature | frontend/parser | implementation-ready | P1 |  | the existing generic-call erasure path is too narrow for callable |
| 5243 | Erase type arguments on ambient generic function calls | feature | frontend/parser | implementation-ready | P1 |  | the existing direct generic-call erasure guard covers simple generic |
| 5244 | Support namespace-merged function static properties | feature | frontend/semantics | implementation-ready | P1 |  | TypeScript namespace merging can attach exported namespace members as |
| 5245 | Parse interface construct signatures | feature | frontend/parser | implementation-ready | P1 |  | interface construct signatures are parsed as if they were runtime |
| 5246 | Report static declarations inside constructor bodies | feature | frontend/parser | implementation-ready | P1 |  | invalid `static` declarations in constructor bodies report generic unsupported expression instead of a spanned parser... |
| 5247 | Fix JS noEmit class constructor FuncId invariant | feature | ir/compiler | implementation-ready | P1 |  | `classAttributeInferenceTemplateJS.ts` triage reports `InvariantViolation: ClassDecl constructor FuncId 0 is out of r... |
| 5248 | Lower class expressions | feature | ir/compiler | implementation-ready | P1 |  | `classBlockScoping.ts` reports `UnsupportedSyntax: issue-313: class expression lowering not yet implemented`. |
| 5249 | Scope block-local class declarations | feature | frontend/resolver | implementation-ready | P1 |  | `classDeclarationBlockScoping1.ts` reports `DuplicateLocal` for an inner block-local `class C {}` that TypeScript acc... |
| 5250 | Parse class declarations in nested block statements | feature | frontend/parser | implementation-ready | P1 |  | `classDeclarationBlockScoping2.ts` reports `UnsupportedSyntax: expected Comma, got Some(Ident("C"))` at a nested `{ c... |
| 5251 | Parse computed class member names in class declarations | feature | frontend/parser | implementation-ready | P1 |  | `classDeclarationShouldBeOutOfScopeInComputedNames.ts` stops during |
| 5252 | Support call-expression class heritage | feature | ir/resolver | implementation-ready | P1 |  | `classDeclaredBeforeClassFactory.ts` parses successfully but name |
| 5253 | Report class expression decorator boundary | feature | frontend/lexer | implementation-ready | P1 |  | `classExpressionWithDecorator1.ts` stops in tokenization at |
| 5254 | Parse ASI between static class fields | feature | frontend/parser | implementation-ready | P1 |  | `classExpressionWithStaticProperties2.ts` parses `static b` as the |
| 5255 | Resolve super property accesses | feature | frontend/resolver | implementation-ready | P1 |  | `classExtendingAny.ts` parses class declarations, `extends Err`, and |
| 5256 | Report non-constructor class heritage expressions | feature | ir/resolver | implementation-ready | P1 |  | `classExtendsInterface_not.ts` parses `class C extends "".bogus {}`, |
| 5257 | Parse object type literal construct signatures | feature | frontend/parser | implementation-ready | P1 |  | `classExtendsInterfaceInExpression.ts` fails before class heritage can |
| 5258 | Report super calls in class extends null constructors | feature | ir/resolver | implementation-ready | P1 |  | `classExtendsNull.ts` parses `extends null` and a constructor |
| 5259 | Report super property access in class extends null | feature | ir/resolver | implementation-ready | P1 |  | `classExtendsNull3.ts` parses two classes with `extends null` and |
| 5260 | Report class heritage trailing comma | feature | frontend/parser | implementation-ready | P1 |  | `classHeritageWithTrailingSeparator.ts` tokenizes the class heritage |
| 5261 | Report class-typed missing instance method calls | feature | ir/lowering | implementation-ready | P1 |  | Report class-typed missing instance method calls |
| 5262 | Resolve import-equals aliases in class implements clauses | feature | frontend/resolver | implementation-ready | P1 |  | Resolve import-equals aliases in class implements clauses |
| 5263 | Report primitive implements clauses on class expressions | feature | frontend/parser | implementation-ready | P1 |  | Report primitive implements clauses on class expressions |
| 5264 | Parse typed const declarations before initializers | feature | frontend/parser | implementation-ready | P1 |  | Parse typed const declarations before initializers |
| 5265 | Report missing class member identifier after modifier | feature | frontend/parser | implementation-ready | P1 |  | Report missing class member identifier after modifier |
| 5266 | Support class constructor new of later class binding | feature | ir | implementation-ready | P1 |  | `reference/typescript/tests/cases/compiler/classOrderBug.ts` reports |
| 5267 | Parse string literal class member names | feature | frontend | implementation-ready | P1 |  | `reference/typescript/tests/cases/compiler/classStaticPropertyAccess.ts` |
| 5268 | Support derived constructor parameter properties after super | feature | ir | implementation-ready | P1 |  | `reference/typescript/tests/cases/compiler/classUpdateTests.ts` reports |
| 5269 | Parse optional class property declarations | feature | frontend | implementation-ready | P1 |  | `reference/typescript/tests/cases/compiler/classUsedBeforeInitializedVariables.ts` |
| 5270 | Parse modified class accessor declarations | feature | frontend | implementation-ready | P1 |  | `reference/typescript/tests/cases/compiler/classdecl.ts` reports |
| 5271 | Parse modified static class fields | feature | frontend | implementation-ready | P1 |  | `reference/typescript/tests/cases/compiler/cloduleStaticMembers.ts` |
| 5272 | Support generic return interface method receivers | feature | ir/lowering | implementation-ready | P1 |  | Support generic return interface method receivers |
| 5273 | Parse nested zero-argument arrow returns | feature | frontend | implementation-ready | P1 |  | Parse nested zero-argument arrow returns |
| 5274 | Parse general comma expressions | feature | frontend | implementation-ready | P1 |  | Parse general comma expressions |
| 5275 | Parse modified static class methods | feature | frontend | implementation-ready | P1 |  | Parse modified static class methods |
| 5276 | Report class declaration decorator boundary | feature | frontend/lexer | implementation-ready | P1 |  | `commentOnDecoratedClassDeclaration.ts` stops in tokenization at the |
| 5277 | Parse export enum declarations to enum boundary | feature | frontend/module-syntax | implementation-ready | P1 |  | `commentOnExportEnumDeclaration.ts` stops at `export` before the enum |
| 5278 | Parse trailing comma in function parameters with comments | feature | frontend/syntax | implementation-ready | P1 |  | `commentOnParameter3.ts` fails after parsing `a`, `b`, and a trailing |
| 5279 | Report function-typed local call definite assignment | feature | ir/lowering | implementation-ready | P2 |  | `commentOnParenthesizedExpressionOpenParen1.ts` parses successfully, |
| 5280 | Validate commented top-level function overloads | feature | frontend/resolver | implementation-ready | P1 |  | `commentOnSignature1.ts` parses successfully, but `validate_ast` |
| 5281 | Resolve arrow rest parameter bindings | feature | ir/name-resolution | implementation-ready | P1 |  | arrow rest parameters are not made visible under their identifier name |
| 5282 | Parse labeled empty statements | feature | frontend/syntax | implementation-ready | P2 |  | `Input: ;` currently reports `UnsupportedSyntax: unsupported expression: ... Semicolon`, even though TypeScript accep... |
| 5283 | Support entry-module export var declarations | feature | frontend/module-syntax | implementation-ready | P1 |  | `export var b: number;` currently stops at the generic unsupported variable export boundary before the declaration ca... |
| 5284 | Bind plain enum declarations before member access | feature | frontend/syntax | implementation-ready | P1 |  | `enum Colors { Cornflower, FancyPink }` does not create a frontend |
| 5285 | Support export var initializer declarations | feature | frontend/module-syntax | implementation-ready | P1 |  | `export var newVar = new extMod.m1.m2.c();` currently cannot be |
| 5286 | Preserve class constructor parameters for new arity | feature | ir | implementation-ready | P1 |  | `new c2(10)` reports `ArityMismatch` because the lowered constructor |
| 5287 | Bind namespace declarations for qualified value access | feature | frontend/name-resolution | implementation-ready | P1 |  | a same-file non-ambient namespace declaration is erased before it |
| 5288 | Parse typed modified static class fields | feature | frontend/parser | implementation-ready | P1 |  | `reference/typescript/tests/cases/compiler/commentsOnStaticMembers.ts` |
| 5289 | Validate commentsOverloads top-level functions | feature | frontend/resolver | implementation-ready | P1 |  | `reference/typescript/tests/cases/compiler/commentsOverloads.ts` |
| 5290 | Parse private static generic class methods | feature | frontend/parser | implementation-ready | P1 |  | `reference/typescript/tests/cases/compiler/commentsTypeParameters.ts` |
| 5291 | Report malformed export type declarations | feature | frontend/parser | implementation-ready | P1 |  | malformed `export type` declarations are not diagnosed or recovered |
| 5292 | Skip tsconfig @Filename sections in reference harness | feature | compiler/multi-section | implementation-ready | P1 |  | reference-style `tsconfig.json` virtual sections are treated as module |
| 5293 | Handle recursive generic self-heritage class lowering | bug | frontend/semantics | implementation-ready | P1 |  | recursive generic self-heritage classes fail with an opaque |
| 5294 | Resolve sibling namespaces in nested namespace scopes | feature | frontend/name-resolution | implementation-ready | P1 |  | nested namespace resolution does not predeclare or look up sibling |
| 5295 | Resolve import-equals require to virtual node_modules class export | feature | frontend/module-resolution | implementation-ready | P1 |  | bare `require("myModule")` aliases do not bind to a virtual |
| 5296 | Parse double-dot numeric literal property access | feature | frontend/parser | implementation-ready | P1 |  | the parser treats the first dot as member access and rejects the |
| 5297 | Lower computed object binding aliases | feature | frontend/semantics | implementation-ready | P1 |  | object binding aliases can only use identifier keys, so computed keys |
| 5298 | Parse for-of array binding pattern heads | feature | frontend/syntax | implementation-ready | P1 |  | array binding patterns in `for-of` declaration heads are parsed as |
| 5299 | Lower computed object binding parameters | feature | frontend/semantics | implementation-ready | P1 |  | computed object binding aliases in parameters parse, but name |
| 5300 | Report assignment to class binding diagnostics | feature | frontend/resolver | implementation-ready | P1 |  | assignment to a class binding currently parses, but name resolution |
| 5301 | Report literal reference comparison diagnostics | feature | frontend/semantics | implementation-ready | P1 |  | `conditionalEqualityOnLiteralObjects.ts` builds successfully even |
| 5302 | Parse fractional number literals in expressions | feature | frontend/syntax | implementation-ready | P1 |  | `conditionalTypeAssignabilityWhenDeferred.ts` currently fails before |
| 5303 | Parse trailing comma in typed function parameters | feature | frontend/syntax | implementation-ready | P1 |  | typed function declaration parameter lists with a trailing comma |
| 5304 | Parse generic arrow functions with typed parameters | feature | frontend/syntax | implementation-ready | P1 |  | generic arrow functions with typed parameters currently fail with |
| 5305 | Report merge conflict marker diagnostics | feature | frontend/syntax | implementation-ready | P1 |  | merge conflict marker source currently reports generic parser syntax |
| 5306 | Report export assignment with other exports | bug | frontend/syntax | implementation-ready | P1 |  | `ExportAssignment8.ts` stops at generic issue-055 instead of reporting the specific `export =` plus other exports rule. |
| 5307 | Report var/function duplicate identifier diagnostics | bug | frontend/resolver | implementation-ready | P2 |  | var/function declaration collisions report generic `DuplicateLocal` |
| 5308 | Parse ASI after instance class field initializers | feature | frontend/parser | implementation-ready | P1 |  | `conflictingTypeParameterSymbolTransfer.ts` reports `expected property name, got Equal` after `foo = this.t` instead ... |
| 5309 | Skip generic type arguments in type annotations | feature | frontend/parser | implementation-ready | P1 |  | function parameter annotations such as `x: Record<'a', string>` fail with `expected Comma, got Some(Greater)`. |
| 5310 | Parse nested block statements with variable declarations | feature | frontend/parser | implementation-ready | P1 |  | a nested block containing `var y = 0;` reports `expected Comma, got Some(Ident("y"))` instead of parsing as a block s... |
| 5311 | Parse namespace property += assignment | feature | frontend/syntax | implementation-ready | P1 |  | `M.x += 2` in `constDeclarations-access3.ts` fails with |
| 5312 | Parse export abstract class declarations | feature | frontend/syntax | implementation-ready | P1 |  | `export abstract class ConvenientObservable<T, TChange> ...` in |
| 5313 | Report non-exported namespace member in qualified heritage | feature | frontend/name-resolution | implementation-ready | P1 |  | `classExtendingQualifiedName.ts` now builds successfully, but |
| 5314 | Report non-constructor local class heritage | feature | frontend/name-resolution | implementation-ready | P1 |  | `classExtendsClauseClassMergedWithModuleNotReferingConstructor.ts` |
| 5315 | Report class extends interface diagnostics | feature | frontend/name-resolution | implementation-ready | P1 |  | `classExtendsInterface.ts` now build-passes, but TypeScript reports |
| 5316 | Report class implements interface private member mismatch | feature | frontend/resolver | implementation-ready | P1 |  | `classExtendsInterfaceThatExtendsClassWithPrivates1.ts` now |
| 5317 | Report multiple class heritage bases | feature | frontend/parser | implementation-ready | P1 |  | `classExtendsMultipleBaseClasses.ts` currently stops in the parser |
| 5318 | Support class expression class heritage | feature | ir/resolver | implementation-ready | P1 |  | `classFieldSuperAccessible.ts` parses to AST successfully, including |
| 5319 | Support super property reads in class static blocks | feature | ir/runtime | implementation-ready | P1 |  | `classFieldSuperAccessibleJs1.ts` parses successfully, but resolver |
| 5320 | Support class prototype method call dispatch | feature | ir/runtime | implementation-ready | P1 |  | `classFieldSuperAccessibleJs2.ts` parses and lowers through ordinary |
| 5321 | Report super class field access diagnostic | feature | ir/resolver | implementation-ready | P1 |  | `classFieldSuperNotAccessible.ts` currently reaches lowering and |
| 5322 | Support callable class auto-accessor fields | feature | ir/runtime | implementation-ready | P1 |  | `classFieldSuperNotAccessibleJs.ts` parses successfully, but lowering |
| 5323 | Report missing constructor parameter list | feature | frontend/parser | implementation-ready | P1 |  | `classFieldsBrokenConstructorEmitNoCrash1.ts` currently reports a |
| 5324 | Support dependency-module export class declarations | feature | ir/compiler | implementation-ready | P1 |  | dependency-module `export class Test1 { ... }` currently reports |
| 5325 | Fix multifile class constructor FuncId invariant | feature | ir/compiler | implementation-ready | P1 |  | a class with a constructor in the second virtual file can produce a |
| 5326 | Parse anonymous default class export | feature | frontend/module-syntax | implementation-ready | P1 |  | this single anonymous default class form is still treated as an |
| 5327 | Report class method overload wrong implementation name | feature | frontend/resolver | implementation-ready | P1 |  | `classWithOverloadImplementationOfWrongName2.ts` reports `DuplicateFunction: duplicate method definition: C.foo` inst... |
| 5328 | Share script globals across @Filename sections for class namespace merge | feature | compiler/name-resolution | implementation-ready | P1 |  | cross-section global script declarations are not shared, so the second |
| 5329 | Report class namespace duplicate member diagnostics | feature | frontend/semantics | implementation-ready | P1 |  | class/namespace duplicate member names currently produce a false build |
| 5330 | Report namespace before class merge diagnostic | feature | frontend/semantics | implementation-ready | P1 |  | prior instantiated namespace/class merge ordering currently produces a |
| 5331 | Report class namespace static side inheritance diagnostic | feature | frontend/semantics | implementation-ready | P1 |  | namespace-augmented static-side inheritance compatibility currently |
| 5332 | Parse interface call signatures | feature | frontend/parser | implementation-ready | P1 |  | interface call-signature members are not parsed as erasable TypeScript |
| 5333 | Report strict mode arguments binding diagnostics | feature | frontend/semantics | implementation-ready | P1 |  | strict-mode `arguments` bindings currently build-pass silently instead |
| 5334 | Parse class constructor overload signatures | feature | frontend/parser | implementation-ready | P1 |  | class constructor overload signatures are not represented separately |
| 5335 | Validate nested function overload implementations | feature | frontend/resolver | implementation-ready | P1 |  | nested function overload implementation groups are classified as |
| 5336 | Parse object type literal signatures with rest parameters | feature | frontend/parser | implementation-ready | P1 |  | object type literal signature members with rest parameters are not |
| 5337 | Parse rest parameter constructor overload signatures | feature | frontend/parser | implementation-ready | P1 |  | bodyless constructor overload signatures with rest parameters are |
| 5338 | Support rest constructor outer local captures | feature | ir/lowering | implementation-ready | P1 |  | constructor rest parameters and hidden lexical-capture parameters |
| 5339 | Preserve var after object type declaration | feature | frontend/syntax | implementation-ready | P1 |  | a `var name: { ... }` object type annotation consumes the next |
| 5340 | Preserve function after object type declaration | feature | frontend/syntax | implementation-ready | P1 |  | `var name: { method(param: Type); }` is not terminated before a following `function` declaration. |
| 5341 | Resolve lexical super captures in method arrows | feature | frontend/resolver | implementation-ready | P1 |  | lexical `super.foo()` inside an arrow in a derived class method is not resolved against the method's derived instance... |
| 5342 | Preserve class after object type declaration | feature | frontend/syntax | implementation-ready | P1 |  | `var name: { method(param: Type); }` is not terminated before a |
| 5343 | Track array-typed erased locals for callback methods | feature | frontend/semantics | implementation-ready | P1 |  | declaration-only array locals such as `var s: string[];` lose their |
| 5344 | Resolve ambient var assignment targets | feature | frontend/resolver | implementation-ready | P1 |  | declaration-only ambient variable assignment targets are not |
| 5345 | Parse generic ambient const type annotations | feature | frontend/parser | implementation-ready | P1 |  | nested generic ambient const annotations are not erased as a complete |
| 5346 | Parse CommonJS export assignment statements | feature | frontend/parser | implementation-ready | P1 |  | Parse CommonJS export assignment statements |
| 5347 | Align class var redeclaration diagnostics | feature | frontend/resolver | implementation-ready | P1 |  | Align class var redeclaration diagnostics |
| 5348 | Resolve const declarations before use diagnostics | bug | frontend/resolver | implementation-ready | P1 |  | lexical const bindings are not registered early enough for |
| 5349 | Parse multiplicative compound assignment operators | feature | frontend/syntax | implementation-ready | P1 |  | Parse multiplicative compound assignment operators |
| 5350 | Report missing const initializer diagnostics | feature | frontend/parser | implementation-ready | P1 |  | Report missing const initializer diagnostics |
| 5351 | Accept large decimal integer number literals | feature | frontend/lexer | implementation-ready | P1 |  | Accept large decimal integer number literals |
| 5352 | Parse export namespace declarations | feature | frontend/module-syntax | implementation-ready | P1 |  | `export namespace Name { ... }` is treated as an unsupported static |
| 5353 | Parse extended Unicode string escapes | feature | frontend/syntax | implementation-ready | P1 |  | string literal lexing rejects valid ECMAScript extended Unicode code |
| 5354 | Report invalid const class members | feature | frontend/semantics | implementation-ready | P1 |  | invalid `const` class members in class expressions are silently |
| 5355 | Report invalid constructor parameter modifiers | bug | frontend/parser | implementation-ready | P1 |  | current failure is `expected Comma, got Some(Static)` for |
| 5356 | Report uninitialized generic class fields | bug | frontend/semantics | implementation-ready | P1 |  | current failure is a false build pass for `class D<T, U> { x: T; y: U }`, where TypeScript reports uninitialized prop... |
| 5357 | Avoid eval diagnostic for qualified Function constructors | bug | ir/resolver | implementation-ready | P1 |  | current failure is a false eval diagnostic for qualified `new M.Function(...)` because resolver `Expr::New` collapses... |
| 5358 | Report constructor bodies in ambient class declarations | feature | frontend/parser | implementation-ready | P1 |  | implementation bodies inside ambient class declarations are not |
| 5359 | Report multiple constructor implementation diagnostics | feature | frontend/diagnostics | implementation-ready | P1 |  | invalid multiple constructor implementations are not reported with |
| 5360 | Report class field initializer constructor-scope captures | feature | frontend/semantics | implementation-ready | P2 |  | class field initializer semantic validation currently does not reject |
| 5361 | Report invalid constructor return value diagnostics | feature | frontend/semantics | implementation-ready | P2 |  | constructor return value semantic validation currently does not reject |
| 5362 | Report strict-mode static constructor parameter name | bug | frontend/parser | implementation-ready | P1 |  | the compiler currently reports an unsupported parser failure, |
| 5363 | Support class constructor outer local return captures | feature | ir | implementation-ready | P1 |  | class constructor bodies cannot currently resolve and lower direct |
| 5364 | Report unterminated string literal at raw newline | bug | frontend/lexer | implementation-ready | P1 |  | Report unterminated string literal at raw newline |
| 5365 | Parse readonly private field type annotations | feature | frontend/parser | implementation-ready | P1 |  | `constructorWithParameterPropertiesAndPrivateFields.es2015.ts` currently reports `UnsupportedSyntax: expected propert... |
| 5366 | Restore call argument scope after typed arrow callbacks | bug | frontend/resolver | implementation-ready | P1 |  | `contextSensitiveReturnTypeInference.ts` currently reports `UnresolvedName: unresolved name: DEPS` for a call argumen... |
| 5367 | Parse named default class export | feature | frontend/module-syntax | implementation-ready | P1 |  | a named default class export is still treated as an unsupported module |
| 5368 | Isolate exported bindings across @filename sections | feature | compiler/multi-section | implementation-ready | P1 |  | external-module `@filename` sections are not isolated during name |
| 5369 | Parse call-expression type arguments in class heritage | feature | frontend/parser | implementation-ready | P1 |  | the class heritage parser expects the class body after `Tag("Foo")` |
| 5370 | Bind ambient namespace declarations for qualified value access | feature | frontend/resolver | implementation-ready | P1 |  | ambient namespace declarations are not visible as namespace values for same-file qualified value access. |
| 5371 | Parse generic function type annotations | feature | frontend/parser | implementation-ready | P1 |  | generic function type annotations in variable declarations are not |
| 5372 | Parse ambient function ASI with constructor types | feature | frontend/parser | implementation-ready | P1 |  | ambient function declaration erasure still requires a terminator for |
| 5373 | Lower complex default binding initializers | feature | ir/lowering | implementation-ready | P2 |  | complex default binding initializers in object binding parameters are |
| 5374 | Support callable ambient const local calls | feature | ir/lowering | implementation-ready | P1 |  | callable ambient const locals with generic call signatures currently |
| 5375 | Support callable ambient interface local calls | feature | ir/lowering | implementation-ready | P1 |  | ambient locals typed by callable interfaces currently fall into the |
| 5376 | Support ambient generic factory local calls | feature | ir/lowering | implementation-ready | P1 |  | ambient generic factory locals currently fall into the generic |
| 5377 | Support callable ambient interface local calls with key remap | feature | ir/lowering | implementation-ready | P1 |  | callable ambient interface locals after mapped-type key remapping fall |
| 5378 | Report mixed ambient function overload diagnostics | feature | frontend/resolver | implementation-ready | P1 |  | mixed ambient/non-ambient top-level function overload groups report a |
| 5379 | Lower array binding object default initializers | feature | ir/lowering | implementation-ready | P2 |  | array binding patterns with object-literal default initializers are not |
| 5380 | Report array literal index-signature element mismatch | feature | frontend/semantics | implementation-ready | P1 |  | array literals assigned to numeric-index-signature interfaces can skip the invalid element diagnostic and fall throug... |
| 5381 | Parse untyped arrow ternary branches | feature | frontend/syntax | implementation-ready | P1 |  | untyped arrow functions used as ternary conditional branches are rejected during AST construction before contextual t... |
| 5382 | Parse typed arrow ternary branches | feature | frontend/syntax | implementation-ready | P1 |  | typed arrow functions used as ternary conditional branches are rejected during AST construction before contextual typ... |
| 5383 | Classify number parameter toFixed calls | feature | ir/lowering | implementation-ready | P1 |  | `x.toFixed()` inside an arrow callback with `x: number` falls through to `issue-211: unknown receiver class for metho... |
| 5384 | Resolve ambient function value references | feature | frontend/resolver | implementation-ready | P1 |  | `contextualTypingReturnStatementWithReturnTypeAnnotation.ts` currently |
| 5385 | Parse arrow body assignment expressions | feature | frontend/syntax | implementation-ready | P1 | 5000 | `contextualTypingTwoInstancesOfSameTypeParameter.ts` currently fails |
| 5386 | Bind DOM setTimeout global | feature | frontend/resolver | implementation-ready | P1 |  | `contextuallyTypeArgumentsKeyword.ts` requests `// @lib: es2017, dom` |
| 5387 | Parse generator function expressions in const initializers | feature | frontend/syntax | implementation-ready | P1 | 5000 | `contextuallyTypeGeneratorReturnTypeFromUnion.ts` currently stops at |
| 5388 | Support discriminated union parameter method calls | feature | ir/lowering | implementation-ready | P1 |  | Support discriminated union parameter method calls |
| 5389 | Support nested function default parameters in closure lowering | feature | ir/lowering | implementation-ready | P1 |  | Support nested function default parameters in closure lowering |
| 5390 | Report abstract property constructor access diagnostics | feature | ir/lowering | implementation-ready | P1 |  | abstract property accesses inside the declaring class constructor |
| 5391 | Report unqualified static member suggestion diagnostics | feature | frontend/resolver | implementation-ready | P1 |  | `foo` inside an instance method currently reports generic |
| 5392 | Report unqualified instance member name diagnostics | feature | frontend/resolver | implementation-ready | P1 |  | `foo` inside a static method currently reports generic |
| 5393 | Report get accessor accidental call diagnostics | feature | ir/lowering | implementation-ready | P1 |  | Report get accessor accidental call diagnostics |
| 5394 | Fix object accessor FuncId invariant | feature | ir/compiler | implementation-ready | P1 |  | Fix object accessor FuncId invariant |
| 5395 | Report getter return mismatch with setter annotation | feature | frontend/semantics | implementation-ready | P1 |  | `accessors_spec_section-4.5_error-cases.ts` build-passes even though |
| 5396 | Report setter body mismatch with getter annotation | feature | frontend/semantics | implementation-ready | P1 |  | `accessors_spec_section-4.5_error-cases.ts` build-passes even though |
| 5397 | Report missing namespace alias member diagnostics | feature | frontend/semantics | implementation-ready | P1 |  | `aliasBug.ts` now build-passes even though TypeScript reports TS2694 |
| 5400 | Parse exported import-equals declarations | feature | frontend/parser | implementation-ready | P1 |  | exported import-equals declarations stop at generic issue-055 static export before the parser can preserve the alias ... |
| 5401 | Parse export default interface declarations | feature | frontend/parser | implementation-ready | P1 |  | `export default interface zzz { ... }` stops with `expected Semicolon` before the frontend can represent the interfac... |
| 5402 | Skip package.json @Filename sections in reference harness | feature | compiler/multi-section | implementation-ready | P1 |  | reference-style `package.json` virtual sections are treated as module bodies instead of package metadata or non-code ... |
| 5403 | Support type-only default exports of local interfaces | feature | frontend/name-resolution | implementation-ready | P1 |  | the resolver treats `export default Color;` as a value export |
| 5405 | Bind require fs local method calls | feature | frontend/resolver | implementation-ready | P1 |  | `ambientRequireFunction.ts` currently reaches issue-211 unknown |
| 5406 | Report ambient enum nonconstant initializers | feature | frontend/semantics | implementation-ready | P2 |  | `ambientEnum1.ts` currently records a ts2wasm `BuildPass`, but |
| 5407 | Report ambient getter implementation bodies | feature | frontend/parser | implementation-ready | P1 |  | ambient class getter implementation bodies are currently erased |
| 5408 | Parse bare global augmentation blocks | bug | frontend/syntax | implementation-ready | P1 |  | bare TypeScript global augmentation syntax falls through to runtime |
| 5409 | Report non-exported namespace member type annotations | bug | frontend/semantics | implementation-ready | P1 |  | non-exported namespace members in qualified type annotations are erased |
| 5410 | Report namespace-as-base-type diagnostics | feature | frontend/semantics | implementation-ready | P1 |  | namespace-as-base-type misuse currently produces a false build pass |
| 5411 | Report TS2709 for namespace variable annotation | feature | frontend/semantics | implementation-ready | P1 |  | `var a: A;` silently build-passes when `A` is a namespace. |
| 5412 | Report TS2451 for duplicate const filename sections | feature | frontend/semantics | implementation-ready | P2 |  | duplicate `const a` across the representative `.cts`/`.mts` reference |
| 5413 | Report nested namespace wrong-context diagnostic | bug | frontend/syntax | implementation-ready | P1 |  | the frontend accepts a nested namespace declaration in a statement |
| 5414 | Classify non-builtin require result method calls | feature | ir/lowering | implementation-ready | P1 |  | the compiler does not distinguish a non-builtin CommonJS require result |
| 5415 | Support identifier update expressions in value positions | feature | ir/lowering | implementation-ready | P1 |  | identifier update expressions are parser-accepted in value positions, |
| 5416 | Report invalid block after member expression statement | bug | frontend/parser | implementation-ready | P1 |  | the parser should reject or recover from a block immediately following |
| 5417 | Parse default keyword in named import specifiers | feature | frontend/module-syntax | implementation-ready | P1 |  | the parser should accept contextual `default` as the imported binding |
| 5418 | Parse dynamic import call expressions | feature | frontend/module-syntax | implementation-ready | P1 |  | dynamic import calls such as `const foo = import("./b");` are rejected |
| 5419 | Parse parenthesized self-closing JSX expressions | feature | frontend/jsx | implementation-ready | P1 |  | self-closing JSX expressions in expression position are not parsed or |
| 5420 | Parse import attributes with clauses | feature | frontend/module-syntax | implementation-ready | P1 |  | import attribute `with { type: "json" }` clauses are not accepted or |
| 5421 | Resolve classic moduleResolution bare imports to virtual sections | feature | compiler/module-graph | implementation-ready | P1 |  | module graph validation reports `issue-232: unsupported non-local |
| 5422 | Skip markdown @Filename sections in reference harness | feature | compiler/multi-section | implementation-ready | P1 |  | the multi-section compiler path tokenizes and parses the README body |
| 5423 | Parse declaration-file exported const declarations | feature | frontend/parser | implementation-ready | P1 |  | Parse declaration-file exported const declarations |
| 5424 | Skip unread asset @Filename sections in reference harness | feature | compiler/multi-section | implementation-ready | P1 |  | fixture-only asset sections such as `.js` or `.css` files that should |
| 5425 | Resolve local require between @Filename sections | feature | compiler/module-graph | implementation-ready | P1 |  | local CommonJS require calls can create dangling `ModuleLoad` |
| 5426 | Resolve @symlink filename aliases for local imports | feature | compiler/module-graph | implementation-ready | P1 |  | `@symlink` aliases are not registered as virtual module paths for |
| 5427 | Resolve reference types to virtual @types packages | feature | frontend/semantics | implementation-ready | P1 | 227 | Resolve reference types to virtual @types packages |
| 5429 | Bind DOM self.cancelAnimationFrame global | feature | frontend/resolver | implementation-ready | P1 |  | DOM `self.cancelAnimationFrame(0)` currently fails with generic |
| 5430 | Parse exported import-equals require declarations | feature | frontend/parser | implementation-ready | P1 |  | `export import Math = require("./Math/Math")` currently reports a |
| 5431 | Erase object type literal function return annotations | feature | frontend/parser | implementation-ready | P1 |  | Erase object type literal function return annotations |
| 5432 | Support class namespace merged static members | feature | frontend/semantics | implementation-ready | P1 |  | `multiModuleClodule1.ts` reports `UnresolvedName` for class/namespace merged value access because exported namespace ... |
| 5433 | Report duplicate static class member modifiers | feature | frontend/parser | implementation-ready | P1 |  | duplicate `static` class member modifiers are parsed as a method/field boundary error instead of a source-spanned dup... |
| 5434 | Report duplicate ambient module export assignments | bug | frontend/semantics | implementation-ready | P1 |  | duplicate `export =` declarations inside an ambient module are hidden by ambient erasure and produce a false build-pass. |
| 5435 | Report export declarations inside namespaces | bug | frontend/semantics | implementation-ready | P1 |  | namespace-body export declarations are erased before diagnostics, causing a false build-pass. |
| 5436 | Report mixed exported and local namespace var merges | feature | frontend/semantics | implementation-ready | P1 |  | `multivar.ts` now build-passes, but TypeScript reports TS2395 because |
| 5437 | Report typed class method null return diagnostics | feature | frontend/semantics | implementation-ready | P1 |  | `mutuallyRecursiveGenericBaseTypes2.ts` now build-passes, but |
| 5438 | Support named exports of local interfaces | feature | frontend/name-resolution | implementation-ready | P1 |  | named exports of local type-only interface declarations fail with `UnsupportedSyntax` unknown local binding. |
| 5439 | Report namespace/value duplicate identifiers | bug | frontend/resolver | implementation-ready | P1 |  | namespace declarations that collide with `var` bindings currently build-pass silently instead of reporting duplicate ... |
| 5440 | Support initialized function expression local calls | feature | ir/lowering | implementation-ready | P1 |  | initialized function-expression locals fall into the generic issue-211 function-valued local call boundary. |
| 5441 | Report namespaced union literal assignment diagnostic | feature | frontend/semantics | implementation-ready | P1 |  | erased namespace type aliases and union annotations hide the invalid |
| 5442 | Report mixed default function namespace merge diagnostic | feature | frontend/semantics | implementation-ready | P1 |  | ambient module declaration bodies are erased before declaration export |
| 5443 | Report duplicate type alias identifiers | feature | frontend/semantics | implementation-ready | P1 |  | erased type aliases hide duplicate type-only declarations, so the |
| 5444 | Resolve const arrow predicate calls in switch true clauses | feature | frontend/resolver | implementation-ready | P1 |  | const arrow predicate bindings are visible as bindings, but direct |
| 5445 | Parse braced switch case clause statements | feature | frontend/parser | implementation-ready | P1 |  | switch case clause parsing does not correctly dispatch a braced block |
| 5446 | Parse empty statements in switch case bodies | feature | frontend/parser | implementation-ready | P1 |  | ordinary empty statements in switch case bodies are not accepted as |
| 5447 | Support instanceof callable prototype RHS | feature | ir/runtime | implementation-ready | P1 |  | `instanceof` RHS resolution only accepts the currently supported class |
<!-- generated:ready:end -->

## Blocked queue

<!-- generated:blocked:start -->
| ID | Title | Type | Area | Blocker | Summary |
|---:|---|---|---|---|---|
| 021 | Implement full wasm backend | feature | backend | class: blocked | Implement full wasm backend |
| 050 | Implement Date | feature | runtime/builtins | class: blocked | Implement Date |
| 052 | Implement JSON | feature | runtime/builtins | class: blocked | Implement JSON |
| 052d | Implement broader JSON.stringify replacer semantics (audit reopened #052d) | feature | runtime/builtins | class: blocked | Implement broader JSON.stringify replacer semantics (audit reopened #052d) |
| 059 | Implement parser syntax extensions for TypeScript and advanced JS (audit reopened #059) | feature | frontend | class: blocked | Implement parser syntax extensions for TypeScript and advanced JS (audit reopened #059) |
| 064 | Implement name resolution (triaged - superseded by test262 metadata issues) | spike | frontend/resolver | class: blocked | Implement name resolution (triaged - superseded by test262 metadata issues) |
| 168 | Implement Ambiguousoverload | spike | frontend/syntax | class: blocked | Implement Ambiguousoverload |
| 194 | Implement Argumentsaspropertyname | spike | frontend/semantics | class: blocked | Implement Argumentsaspropertyname |
| 274 | Implement spread operator (audit reopened #274) | meta | frontend/semantics | class: blocked | Implement spread operator (audit reopened #274) |
| 294 | Support ABC451 D original submission without source rewrite | feature | frontend/runtime | class: blocked | Support ABC451 D original submission without source rewrite |
| 300 | Support ABC451 large integer number boundary | feature | runtime | class: blocked | Support ABC451 large integer number boundary |
| 308 | Implement ABC451 depth-9 GC cadence policy | feature | runtime/memory | class: blocked | Implement ABC451 depth-9 GC cadence policy |
| 309 | Reduce ABC451 depth-9 live allocation shape | feature | runtime/memory | class: blocked | Reduce ABC451 depth-9 live allocation shape |
| 312 | Triage test262 blocked P0 window | spike | reference | class: triage-needed | Triage test262 blocked P0 window |
| 313 | Implement array-builtin support | feature | runtime/builtins | class: triage-needed | Implement array-builtin support |
| 335 | Implement full Math.pow number semantics | feature | runtime/builtins | class: triage-needed | Implement full Math.pow number semantics |
| 342 | Implement Object builtin method coverage (1,721 test262 cases) | feature | runtime/builtins | class: triage-needed | Implement Object builtin method coverage (1,721 test262 cases) |
| 343 | Implement DuplicateLocal diagnostic detection (66 test262 cases) | feature | frontend/resolver | class: blocked | Implement DuplicateLocal diagnostic detection (66 test262 cases) |
| 345 | Implement TypeScript type alias coverage for tsc suite (23 cases) | feature | frontend/syntax | class: blocked | Implement TypeScript type alias coverage for tsc suite (23 cases) |
| 346 | Implement TypeScript declaration emit coverage for tsgo suite (16 cases) | feature | frontend/syntax | class: triage-needed | Implement TypeScript declaration emit coverage for tsgo suite (16 cases) |
| 357 | Fix ABC451 depth-8 iwasm timeout | bug | runtime/memory | class: blocked | Fix ABC451 depth-8 iwasm timeout |
| 363 | Reduce ABC451 allocation and sweep volume after bulk copy narrowing | bug | runtime/memory | class: blocked | Reduce ABC451 allocation and sweep volume after bulk copy narrowing |
| 365 | Reduce ABC451 array-growth allocation and copy pressure | bug | runtime/memory | class: blocked | Reduce ABC451 array-growth allocation and copy pressure |
| 411 | Implement annexb-ishtmldda support | spike | frontend/syntax | class: triage-needed | Implement annexb-ishtmldda support |
| 416 | Implement async/await support | spike | frontend/syntax | class: triage-needed | Implement async/await support |
| 417 | Implement async-iteration support | spike | frontend/syntax | class: triage-needed | Implement async-iteration support |
| 418 | Implement break/continue | spike | frontend/syntax | class: blocked | Implement break/continue |
| 419 | Implement built-in API support | spike | runtime/builtins | class: triage-needed | Implement built-in API support |
| 420 | Implement call expression support | spike | frontend/syntax | class: blocked | Implement call expression support |
| 421 | Implement class syntax | spike | frontend/syntax | class: triage-needed | Implement class syntax |
| 422 | Implement class-accessor support | spike | frontend/syntax | class: triage-needed | Implement class-accessor support |
| 423 | Implement Date object support (dup) | spike | runtime/builtins | class: blocked | Implement Date object support (dup) |
| 424 | Implement declaration-emit support | spike | frontend/syntax | class: triage-needed | Implement declaration-emit support |
| 425 | Implement destructuring | spike | frontend/syntax | class: triage-needed | Implement destructuring |
| 426 | Implement duplicate-function support | spike | reference/triage | class: triage-needed | Implement duplicate-function support |
| 428 | Implement enum support | spike | frontend/syntax | class: triage-needed | Implement enum support |
| 429 | Implement eval support | spike | reference/triage | class: blocked | Implement eval support |
| 431 | Implement function resolution | spike | frontend/resolver | class: triage-needed | Implement function resolution |
| 432 | Implement import/export module syntax | spike | frontend/syntax | class: triage-needed | Implement import/export module syntax |
| 434 | Implement loop constructs | spike | frontend/syntax | class: blocked | Implement loop constructs |
| 435 | Implement method call support | spike | frontend/syntax | class: blocked | Implement method call support |
| 436 | Implement module-resolution support | spike | frontend/syntax | class: triage-needed | Implement module-resolution support |
| 437 | Implement name resolution | spike | frontend/resolver | class: blocked | Implement name resolution |
| 438 | Implement negative-parse-syntaxerror support | spike | reference/triage | class: triage-needed | Implement negative-parse-syntaxerror support |
| 439 | Implement new expression | spike | frontend/syntax | class: blocked | Implement new expression |
| 440 | Implement object-builtin support | spike | frontend/syntax | class: triage-needed | Implement object-builtin support |
| 441 | Implement object literal enhancements | spike | frontend/syntax | class: triage-needed | Implement object literal enhancements |
| 442 | Implement parser syntax extensions | spike | frontend/syntax | class: triage-needed | Implement parser syntax extensions |
| 443 | Implement property access support | spike | frontend/syntax | class: blocked | Implement property access support |
| 445 | Implement runtime-subset support | spike | reference/triage | class: triage-needed | Implement runtime-subset support |
| 446 | Implement scope-analysis support | spike | frontend/syntax | class: triage-needed | Implement scope-analysis support |
| 449 | Implement super keyword | spike | frontend/syntax | class: triage-needed | Implement super keyword |
| 450 | Implement template literals | spike | frontend/syntax | class: triage-needed | Implement template literals |
| 451 | Implement try-catch-finally | spike | frontend/syntax | class: blocked | Implement try-catch-finally |
| 452 | Implement type-alias support | spike | frontend/syntax | class: blocked | Implement type-alias support |
| 453 | Implement type-system support | spike | frontend/semantics | class: blocked | Implement type-system support |
| 454 | Investigate and classify unknown-unsupported cases | spike | frontend/syntax | class: triage-needed | Investigate and classify unknown-unsupported cases |
| 541 | Implement Apilibcheck | spike | frontend/syntax | class: blocked | Implement Apilibcheck |
| 542 | Implement Apisample Arrow Function | spike | frontend/syntax | class: blocked | Implement Apisample Arrow Function |
| 543 | Implement Apisample Import Export | spike | frontend/syntax | class: blocked | Implement Apisample Import Export |
| 544 | Implement Apisample Jsdoc | spike | frontend/syntax | class: blocked | Implement Apisample Jsdoc |
| 554 | Implement Abstractclassinlocalscope | spike | frontend/resolver | class: blocked | Implement Abstractclassinlocalscope |
| 555 | Implement Abstractclassinlocalscopeisabstract | spike | frontend/resolver | class: blocked | Implement Abstractclassinlocalscopeisabstract |
| 556 | Implement Abstractclassunioninstantiation | spike | frontend/resolver | class: blocked | Implement Abstractclassunioninstantiation |
| 597 | Implement Allowjsclassthistypecrash | spike | reference/triage | class: triage-needed | Implement Allowjsclassthistypecrash |
| 625 | Implement Ambiguouscallswherereturntypesagree | spike | frontend/syntax | class: triage-needed | Implement Ambiguouscallswherereturntypesagree |
| 626 | Implement Ambiguousgenericassertion | spike | frontend/syntax | class: blocked | Implement Ambiguousgenericassertion |
| 627 | Implement Ambiguousoverloadresolution | spike | frontend/resolver | class: blocked | Implement Ambiguousoverloadresolution |
| 628 | Implement Amddeclarationemitnoextradeclare | spike | frontend/syntax | class: blocked | Implement Amddeclarationemitnoextradeclare |
| 629 | Implement Amddependencycomment | spike | frontend/syntax | class: blocked | Implement Amddependencycomment |
| 630 | Implement Amddependencycommentname | spike | frontend/syntax | class: blocked | Implement Amddependencycommentname |
| 631 | Implement Amdlikeinputdeclarationemit | spike | frontend/syntax | class: blocked | Implement Amdlikeinputdeclarationemit |
| 632 | Implement Amdmodulebundlenoduplicatedeclarationemitcomments | spike | frontend/syntax | class: blocked | Implement Amdmodulebundlenoduplicatedeclarationemitcomments |
| 633 | Implement Amdmoduleconstenumusage | spike | frontend/syntax | class: blocked | Implement Amdmoduleconstenumusage |
| 634 | Implement Amdmodulename | spike | frontend/syntax | class: blocked | Implement Amdmodulename |
| 635 | Implement Anonclassdeclarationemitisanon | spike | frontend/syntax | class: blocked | Implement Anonclassdeclarationemitisanon |
| 636 | Implement Anonterface | spike | frontend/syntax | class: triage-needed | Implement Anonterface |
| 637 | Implement Anonymousclassdeclarationdoesntprintwithreadonly | spike | frontend/syntax | class: blocked | Implement Anonymousclassdeclarationdoesntprintwithreadonly |
| 640 | Implement Anyandunknownhavefalsycomponents | spike | frontend/resolver | class: blocked | Implement Anyandunknownhavefalsycomponents |
| 643 | Implement Anyidenticaltoitself | spike | frontend/syntax | class: triage-needed | Implement Anyidenticaltoitself |
| 644 | Implement Anyinferenceanonymousfunctions | spike | frontend/semantics | class: blocked | Implement Anyinferenceanonymousfunctions |
| 645 | Implement Argsinscope | spike | frontend/resolver | class: blocked | Implement Argsinscope |
| 646 | Implement Arguments | spike | frontend/syntax | class: blocked | Implement Arguments |
| 647 | Implement Argumentsaspropertyname Arguments Object | spike | frontend/syntax | class: blocked | Implement Argumentsaspropertyname Arguments Object |
| 648 | Implement Argumentsaspropertyname Name Resolution | spike | frontend/resolver | class: blocked | Implement Argumentsaspropertyname Name Resolution |
| 649 | Implement Argumentsbindstofunctionscopeargumentlist | spike | frontend/resolver | class: blocked | Implement Argumentsbindstofunctionscopeargumentlist |
| 650 | Implement Argumentsobjectcreatesrestforjs | spike | frontend/syntax | class: blocked | Implement Argumentsobjectcreatesrestforjs |
| 651 | Implement Argumentsobjectiterator | spike | frontend/syntax | class: blocked | Implement Argumentsobjectiterator |
| 652 | Implement Argumentspropertynameinjsmode | spike | frontend/syntax | class: blocked | Implement Argumentspropertynameinjsmode |
| 653 | Implement Argumentsreferenceinconstructor Arguments Object | spike | frontend/syntax | class: blocked | Implement Argumentsreferenceinconstructor Arguments Object |
| 654 | Implement Argumentsreferenceinconstructor Name Resolution | spike | frontend/resolver | class: blocked | Implement Argumentsreferenceinconstructor Name Resolution |
| 655 | Implement Argumentsreferenceinfunction | spike | frontend/syntax | class: blocked | Implement Argumentsreferenceinfunction |
| 656 | Implement Argumentsreferenceinmethod Arguments Object | spike | frontend/syntax | class: blocked | Implement Argumentsreferenceinmethod Arguments Object |
| 657 | Implement Argumentsreferenceinmethod Name Resolution | spike | frontend/resolver | class: blocked | Implement Argumentsreferenceinmethod Name Resolution |
| 658 | Implement Argumentsreferenceinobjectliteral | spike | frontend/syntax | class: blocked | Implement Argumentsreferenceinobjectliteral |
| 659 | Implement Argumentsusedinclassfieldinitializerorstaticinitializationblock | spike | frontend/syntax | class: blocked | Implement Argumentsusedinclassfieldinitializerorstaticinitializationblock |
| 660 | Implement Argumentsusedinobjectliteralproperty | spike | frontend/syntax | class: blocked | Implement Argumentsusedinobjectliteralproperty |
| 662 | Implement Arrayassignmenttest Import Export | spike | frontend/syntax | class: blocked | Implement Arrayassignmenttest Import Export |
| 663 | Implement Arrayassignmenttest Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Arrayassignmenttest Parser Syntax |
| 664 | Implement Arrayaugment | spike | reference/triage | class: triage-needed | Implement Arrayaugment |
| 665 | Implement Arraybestcommontypes | spike | frontend/syntax | class: blocked | Implement Arraybestcommontypes |
| 666 | Implement Arraybindingpatternomittedexpressions | spike | frontend/syntax | class: blocked | Implement Arraybindingpatternomittedexpressions |
| 667 | Implement Arraybufferisviewnarrowstype | spike | frontend/resolver | class: blocked | Implement Arraybufferisviewnarrowstype |
| 668 | Implement Arraycast | spike | frontend/syntax | class: triage-needed | Implement Arraycast |
| 670 | Implement Arrayconcatmap (audit reopened #670) | spike | frontend/syntax | class: blocked | Implement Arrayconcatmap (audit reopened #670) |
| 671 | Implement Arrayconstructors | spike | frontend/syntax | class: blocked | Implement Arrayconstructors |
| 672 | Implement Arraydestructuringinswitch | spike | frontend/syntax | class: blocked | Implement Arraydestructuringinswitch |
| 673 | Implement Arrayevery | spike | frontend/syntax | class: blocked | Implement Arrayevery |
| 674 | Implement Arrayfakeflatnocrashinferencedeclarations | spike | runtime/builtins | class: triage-needed | Implement Arrayfakeflatnocrashinferencedeclarations |
| 675 | Implement Arrayfilter (audit reopened #675) | spike | runtime/builtins | class: triage-needed | Implement Arrayfilter (audit reopened #675) |
| 676 | Implement Arrayfind (audit reopened #676) | spike | frontend/syntax | class: triage-needed | Implement Arrayfind (audit reopened #676) |
| 677 | Implement Arrayflatmap (audit reopened #677) | spike | frontend/syntax | class: blocked | Implement Arrayflatmap (audit reopened #677) |
| 678 | Implement Arrayflatnocrashinference (audit reopened #678) | spike | frontend/semantics | class: blocked | Implement Arrayflatnocrashinference (audit reopened #678) |
| 679 | Implement Arrayflatnocrashinferencedeclarations (audit reopened #679) | spike | frontend/semantics | class: blocked | Implement Arrayflatnocrashinferencedeclarations (audit reopened #679) |
| 680 | Implement Arrayfrom | spike | runtime/builtins | class: triage-needed | Implement Arrayfrom |
| 681 | Implement Arrayfromasync | spike | reference/triage | class: triage-needed | Implement Arrayfromasync |
| 682 | Implement Arrayindexwitharrayfails | spike | frontend/resolver | class: blocked | Implement Arrayindexwitharrayfails |
| 683 | Implement Arrayiterationlibes | spike | frontend/resolver | class: blocked | Implement Arrayiterationlibes |
| 684 | Implement Arrayliteralandarrayconstructorequivalence | spike | frontend/resolver | class: blocked | Implement Arrayliteralandarrayconstructorequivalence |
| 685 | Implement Arrayliteralcomments | spike | frontend/syntax | class: triage-needed | Implement Arrayliteralcomments |
| 686 | Implement Arrayliteralcontextualtype | spike | frontend/syntax | class: blocked | Implement Arrayliteralcontextualtype |
| 687 | Implement Arrayliteraltypeinference | spike | frontend/semantics | class: blocked | Implement Arrayliteraltypeinference |
| 688 | Implement Arrayofexportedclass | spike | frontend/syntax | class: blocked | Implement Arrayofexportedclass |
| 689 | Implement Arrayofsubtypeisassignabletoreadonlyarray | spike | frontend/syntax | class: blocked | Implement Arrayofsubtypeisassignabletoreadonlyarray |
| 690 | Implement Arrayreferencewithouttypeargs | spike | frontend/syntax | class: blocked | Implement Arrayreferencewithouttypeargs |
| 691 | Implement Arraysigchecking | spike | frontend/syntax | class: blocked | Implement Arraysigchecking |
| 692 | Implement Arrayslice (audit reopened #692) | spike | frontend/syntax | class: blocked | Implement Arrayslice (audit reopened #692) |
| 693 | Implement Arraytolocalestringes Name Resolution | spike | frontend/resolver | class: blocked | Implement Arraytolocalestringes Name Resolution |
| 694 | Implement Arraytolocalestringes Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Arraytolocalestringes Unknown Unsupported |
| 695 | Implement Arraytypeinsignatureofinterfaceandclass | spike | frontend/syntax | class: blocked | Implement Arraytypeinsignatureofinterfaceandclass |
| 696 | Implement Arrayconcat (audit reopened #696) | spike | runtime/builtins | class: triage-needed | Implement Arrayconcat (audit reopened #696) |
| 697 | Implement Arrowfunctioninconstructorargument | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctioninconstructorargument |
| 698 | Implement Arrowfunctioninexpressionstatement | spike | frontend/syntax | class: blocked | Implement Arrowfunctioninexpressionstatement |
| 699 | Implement Arrowfunctionmissingcurlywithsemicolon | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctionmissingcurlywithsemicolon |
| 700 | Implement Arrowfunctionparsingdoesnotconfuseparenthesizedobjectforarrowhead | spike | frontend/syntax | class: blocked | Implement Arrowfunctionparsingdoesnotconfuseparenthesizedobjectforarrowhead |
| 701 | Implement Arrowfunctionparsinggenericinobject | spike | frontend/semantics | class: blocked | Implement Arrowfunctionparsinggenericinobject |
| 702 | Implement Arrowfunctionwithobjectliteralbody | spike | frontend/syntax | class: blocked | Implement Arrowfunctionwithobjectliteralbody |
| 703 | Implement Arrowfunctionsmissingtokens | spike | frontend/syntax | class: blocked | Implement Arrowfunctionsmissingtokens |
| 704 | Implement Asiabstract | spike | frontend/syntax | class: triage-needed | Implement Asiabstract |
| 705 | Implement Asiambientfunctiondeclaration | spike | frontend/syntax | class: triage-needed | Implement Asiambientfunctiondeclaration |
| 706 | Implement Asiarith | spike | frontend/syntax | class: triage-needed | Implement Asiarith |
| 709 | Implement Asiines | spike | frontend/syntax | class: triage-needed | Implement Asiines |
| 710 | Implement Asipublicprivateprotected | spike | frontend/syntax | class: blocked | Implement Asipublicprivateprotected |
| 712 | Implement Assertinwrapsometypeparameter | spike | frontend/syntax | class: blocked | Implement Assertinwrapsometypeparameter |
| 713 | Implement Assertionfunctionwildcardimport | spike | frontend/syntax | class: blocked | Implement Assertionfunctionwildcardimport |
| 714 | Implement Assertionfunctionscannarrowbydiscriminant | spike | frontend/syntax | class: blocked | Implement Assertionfunctionscannarrowbydiscriminant |
| 715 | Implement Assign | spike | frontend/syntax | class: blocked | Implement Assign |
| 716 | Implement Assigntoenum | spike | frontend/syntax | class: triage-needed | Implement Assigntoenum |
| 717 | Implement Assigntoexistingclass | spike | frontend/syntax | class: blocked | Implement Assigntoexistingclass |
| 718 | Implement Assigntofn | spike | frontend/syntax | class: blocked | Implement Assigntofn |
| 719 | Implement Assigntoinvalidlhs | spike | frontend/syntax | class: triage-needed | Implement Assigntoinvalidlhs |
| 720 | Implement Assigntomodule | spike | frontend/syntax | class: blocked | Implement Assigntomodule |
| 721 | Implement Assigntoobjecttypewithprototypeproperty | spike | frontend/resolver | class: blocked | Implement Assigntoobjecttypewithprototypeproperty |
| 722 | Implement Assigntoprototype | spike | frontend/resolver | class: blocked | Implement Assigntoprototype |
| 723 | Implement Assigningfromobjecttoanythingelse | spike | frontend/resolver | class: blocked | Implement Assigningfromobjecttoanythingelse |
| 724 | Implement Assigningfunctiontotupleissueserror | spike | frontend/resolver | class: blocked | Implement Assigningfunctiontotupleissueserror |
| 725 | Implement Assignmentcompat | spike | frontend/resolver | class: blocked | Implement Assignmentcompat |
| 726 | Implement Assignmentcompatbug | spike | frontend/syntax | class: blocked | Implement Assignmentcompatbug |
| 727 | Implement Assignmentcompatforenums | spike | frontend/syntax | class: blocked | Implement Assignmentcompatforenums |
| 728 | Implement Assignmentcompatfunctionswithoptionalargs | spike | frontend/syntax | class: blocked | Implement Assignmentcompatfunctionswithoptionalargs |
| 729 | Implement Assignmentcompatinterfacewithstringindexsignature | spike | frontend/syntax | class: blocked | Implement Assignmentcompatinterfacewithstringindexsignature |
| 730 | Implement Assignmentcompatonnew | spike | frontend/resolver | class: blocked | Implement Assignmentcompatonnew |
| 731 | Implement Assignmentcompatwithoverloads | spike | frontend/syntax | class: blocked | Implement Assignmentcompatwithoverloads |
| 732 | Implement Assignmentcompatability Import Export | spike | frontend/syntax | class: blocked | Implement Assignmentcompatability Import Export |
| 733 | Implement Assignmentcompatability Name Resolution | spike | frontend/resolver | class: blocked | Implement Assignmentcompatability Name Resolution |
| 734 | Implement Assignmentcompatability Parser Syntax | spike | frontend/syntax | class: blocked | Implement Assignmentcompatability Parser Syntax |
| 735 | Implement Assignmentindexedtoprimitives | spike | frontend/syntax | class: triage-needed | Implement Assignmentindexedtoprimitives |
| 736 | Implement Assignmentnestedinliterals | spike | reference/triage | class: triage-needed | Implement Assignmentnestedinliterals |
| 737 | Implement Assignmentnonobjecttypeconstraints | spike | frontend/syntax | class: blocked | Implement Assignmentnonobjecttypeconstraints |
| 738 | Implement Assignmentrestelementwitherrorsourcetype | spike | frontend/resolver | class: blocked | Implement Assignmentrestelementwitherrorsourcetype |
| 739 | Implement Assignmentstricterconstraints | spike | frontend/syntax | class: blocked | Implement Assignmentstricterconstraints |
| 740 | Implement Assignmenttoanyarrayrestparameters | spike | frontend/syntax | class: blocked | Implement Assignmenttoanyarrayrestparameters |
| 741 | Implement Assignmenttoconditionalbrandedstringtemplateormapping | spike | frontend/semantics | class: blocked | Implement Assignmenttoconditionalbrandedstringtemplateormapping |
| 742 | Implement Assignmenttoexpandingarraytype | spike | frontend/syntax | class: triage-needed | Implement Assignmenttoexpandingarraytype |
| 743 | Implement Assignmenttofunction | spike | frontend/syntax | class: blocked | Implement Assignmenttofunction |
| 744 | Implement Assignmenttoinstantiationexpression | spike | frontend/syntax | class: triage-needed | Implement Assignmenttoinstantiationexpression |
| 745 | Implement Assignmenttoobjectandfunction | spike | frontend/syntax | class: blocked | Implement Assignmenttoobjectandfunction |
| 746 | Implement Assignmenttoparenthesizedexpression | spike | frontend/syntax | class: triage-needed | Implement Assignmenttoparenthesizedexpression |
| 747 | Implement Assignmenttoreferencetypes | spike | frontend/syntax | class: blocked | Implement Assignmenttoreferencetypes |
| 748 | Implement Asyncarrowinclasses | spike | runtime/builtins | class: triage-needed | Implement Asyncarrowinclasses |
| 749 | Implement Asyncawaitwithcapturedblockscopevar | spike | reference/triage | class: triage-needed | Implement Asyncawaitwithcapturedblockscopevar |
| 750 | Implement Asyncfunctioncontextuallytypedreturns | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctioncontextuallytypedreturns |
| 751 | Implement Asyncfunctionnoreturntype | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctionnoreturntype |
| 752 | Implement Asyncfunctionreturnexpressionerrorspans | spike | reference/triage | class: triage-needed | Implement Asyncfunctionreturnexpressionerrorspans |
| 753 | Implement Asyncfunctionreturntype Parser Syntax | spike | runtime/builtins | class: triage-needed | Implement Asyncfunctionreturntype Parser Syntax |
| 754 | Implement Asyncfunctionreturntype Runtime Subset | spike | reference/triage | class: triage-needed | Implement Asyncfunctionreturntype Runtime Subset |
| 755 | Implement Asyncfunctiontempvariablescoping | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctiontempvariablescoping |
| 756 | Implement Asyncfunctionwithforstatementnoinitializer | spike | reference/triage | class: triage-needed | Implement Asyncfunctionwithforstatementnoinitializer |
| 757 | Implement Asyncfunctionsacrossfiles | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctionsacrossfiles |
| 758 | Implement Asyncfunctionsandstrictnullchecks | spike | frontend/syntax | class: blocked | Implement Asyncfunctionsandstrictnullchecks |
| 759 | Implement Asynciife | spike | frontend/syntax | class: triage-needed | Implement Asynciife |
| 760 | Implement Asyncimportnestedyield | spike | reference/triage | class: triage-needed | Implement Asyncimportnestedyield |
| 761 | Implement Asynciteratorextraparameters | spike | runtime/builtins | class: triage-needed | Implement Asynciteratorextraparameters |
| 1541 | Implement Contextuallytypedjsxattribute | spike | frontend/syntax | class: blocked | Implement Contextuallytypedjsxattribute |
| 1542 | Implement Contextuallytypedoptionalproperty | spike | frontend/resolver | class: blocked | Implement Contextuallytypedoptionalproperty |
| 1544 | Implement Contextuallytypedparameterswithinitializers Arrow Function | spike | frontend/syntax | class: blocked | Implement Contextuallytypedparameterswithinitializers Arrow Function |
| 1545 | Implement Contextuallytypedparameterswithinitializers Import Export | spike | frontend/syntax | class: blocked | Implement Contextuallytypedparameterswithinitializers Import Export |
| 1550 | Implement Continueiniterationstatement | spike | frontend/resolver | class: blocked | Implement Continueiniterationstatement |
| 1551 | Implement Continueinloopswithcapturedblockscopedbindings | spike | frontend/resolver | class: blocked | Implement Continueinloopswithcapturedblockscopedbindings |
| 1552 | Implement Continuenotiniterationstatement Arrow Function | spike | frontend/syntax | class: blocked | Implement Continuenotiniterationstatement Arrow Function |
| 1553 | Implement Continuenotiniterationstatement Break Continue | spike | frontend/syntax | class: blocked | Implement Continuenotiniterationstatement Break Continue |
| 1554 | Implement Continuetarget | spike | frontend/syntax | class: blocked | Implement Continuetarget |
| 1555 | Implement Contravariantinferenceandtypeguard | spike | frontend/semantics | class: blocked | Implement Contravariantinferenceandtypeguard |
| 1556 | Implement Contravariantonlyinferencewithannotatedoptionalparameter | spike | frontend/resolver | class: blocked | Implement Contravariantonlyinferencewithannotatedoptionalparameter |
| 1557 | Implement Contravarianttypealiasinference | spike | frontend/resolver | class: blocked | Implement Contravarianttypealiasinference |
| 1558 | Implement Controlflowaliaseddiscriminants | spike | frontend/syntax | class: blocked | Implement Controlflowaliaseddiscriminants |
| 1559 | Implement Controlflowanalysisonbarethiskeyword | spike | frontend/syntax | class: blocked | Implement Controlflowanalysisonbarethiskeyword |
| 1560 | Implement Controlflowarrayerrors | spike | frontend/resolver | class: blocked | Implement Controlflowarrayerrors |
| 1561 | Implement Controlflowarrays | spike | frontend/syntax | class: blocked | Implement Controlflowarrays |
| 1562 | Implement Controlflowautoaccessor | spike | frontend/syntax | class: blocked | Implement Controlflowautoaccessor |
| 1563 | Implement Controlflowbreakcontinuewithlabel | spike | frontend/syntax | class: blocked | Implement Controlflowbreakcontinuewithlabel |
| 1564 | Implement Controlflowcaching | spike | frontend/syntax | class: blocked | Implement Controlflowcaching |
| 1565 | Implement Controlflowcommaexpressionassertionmultiple | spike | frontend/syntax | class: blocked | Implement Controlflowcommaexpressionassertionmultiple |
| 1566 | Implement Controlflowcommaexpressionassertionwithinternary | spike | frontend/syntax | class: blocked | Implement Controlflowcommaexpressionassertionwithinternary |
| 1567 | Implement Controlflowcommaexpressionfunctioncall | spike | frontend/syntax | class: blocked | Implement Controlflowcommaexpressionfunctioncall |
| 1568 | Implement Controlflowdestructuringloop | spike | frontend/syntax | class: blocked | Implement Controlflowdestructuringloop |
| 1569 | Implement Controlflowdestructuringvariablesintrycatch | spike | frontend/resolver | class: blocked | Implement Controlflowdestructuringvariablesintrycatch |
| 1570 | Implement Controlflowfavorassertedtypethroughtypepredicate | spike | frontend/syntax | class: blocked | Implement Controlflowfavorassertedtypethroughtypepredicate |
| 1571 | Implement Controlflowforcatchandfinally | spike | reference/triage | class: triage-needed | Implement Controlflowforcatchandfinally |
| 1572 | Implement Controlflowforcompoundassignmenttothismember | spike | frontend/syntax | class: blocked | Implement Controlflowforcompoundassignmenttothismember |
| 1573 | Implement Controlflowforfunctionlike | spike | frontend/syntax | class: blocked | Implement Controlflowforfunctionlike |
| 1574 | Implement Controlflowforindexsignatures | spike | frontend/syntax | class: blocked | Implement Controlflowforindexsignatures |
| 1575 | Implement Controlflowforstatementcontinueintoincrementor | spike | frontend/syntax | class: blocked | Implement Controlflowforstatementcontinueintoincrementor |
| 1576 | Implement Controlflowfunctionlikecircular | spike | frontend/syntax | class: triage-needed | Implement Controlflowfunctionlikecircular |
| 1577 | Implement Controlflowinitializeddestructuringvariables | spike | reference/triage | class: triage-needed | Implement Controlflowinitializeddestructuringvariables |
| 1578 | Implement Controlflowinstanceof | spike | frontend/resolver | class: blocked | Implement Controlflowinstanceof |
| 1579 | Implement Controlflowinstanceofwithsymbolhasinstance | spike | frontend/syntax | class: blocked | Implement Controlflowinstanceofwithsymbolhasinstance |
| 1580 | Implement Controlflowjavascript | spike | frontend/syntax | class: blocked | Implement Controlflowjavascript |
| 1581 | Implement Controlflowloopanalysis | spike | frontend/resolver | class: blocked | Implement Controlflowloopanalysis |
| 1582 | Implement Controlflowmanyconsecutiveconditionsnotimeout | spike | frontend/syntax | class: blocked | Implement Controlflowmanyconsecutiveconditionsnotimeout |
| 1583 | Implement Controlflownoimplicitany | spike | frontend/syntax | class: blocked | Implement Controlflownoimplicitany |
| 1584 | Implement Controlflownulltypeandliteral | spike | frontend/syntax | class: blocked | Implement Controlflownulltypeandliteral |
| 1585 | Implement Controlflowoutervariable | spike | frontend/syntax | class: blocked | Implement Controlflowoutervariable |
| 1586 | Implement Controlflowpropertydeclarations | spike | frontend/syntax | class: blocked | Implement Controlflowpropertydeclarations |
| 1587 | Implement Controlflowpropertyinitializer | spike | frontend/syntax | class: blocked | Implement Controlflowpropertyinitializer |
| 1588 | Implement Controlflowselfreferentialloop | spike | frontend/syntax | class: blocked | Implement Controlflowselfreferentialloop |
| 1589 | Implement Controlflowunioncontainingtypeparameter | spike | frontend/syntax | class: blocked | Implement Controlflowunioncontainingtypeparameter |
| 1590 | Implement Controlflowwithincompletetypes | spike | frontend/resolver | class: blocked | Implement Controlflowwithincompletetypes |
| 1591 | Implement Convertclassexpressiontofunctionfromobjectproperty | spike | frontend/syntax | class: blocked | Implement Convertclassexpressiontofunctionfromobjectproperty |
| 1592 | Implement Convertkeywordsyes | spike | frontend/syntax | class: triage-needed | Implement Convertkeywordsyes |
| 1593 | Implement Copyrightwithnewline | spike | frontend/syntax | class: blocked | Implement Copyrightwithnewline |
| 1594 | Implement Copyrightwithoutnewline | spike | frontend/syntax | class: blocked | Implement Copyrightwithoutnewline |
| 1595 | Implement Correctorderofpromisemethod | spike | reference/triage | class: triage-needed | Implement Correctorderofpromisemethod |
| 1596 | Implement Correlatedunions | spike | frontend/syntax | class: blocked | Implement Correlatedunions |
| 1597 | Implement Corrupted | spike | frontend/syntax | class: triage-needed | Implement Corrupted |
| 1598 | Implement Covariance | spike | frontend/syntax | class: blocked | Implement Covariance |
| 1599 | Implement Crashdeclareglobaltypeofexport | spike | frontend/syntax | class: blocked | Implement Crashdeclareglobaltypeofexport |
| 1600 | Implement Crashinemittokenwithcomment | spike | frontend/syntax | class: triage-needed | Implement Crashinemittokenwithcomment |
| 1601 | Implement Crashingettextofcomputedpropertyname | spike | frontend/syntax | class: triage-needed | Implement Crashingettextofcomputedpropertyname |
| 1602 | Implement Crashinresolveinterface | spike | frontend/resolver | class: blocked | Implement Crashinresolveinterface |
| 1603 | Implement Crashinyieldstarinasyncfunction | spike | frontend/syntax | class: triage-needed | Implement Crashinyieldstarinasyncfunction |
| 1604 | Implement Crashinresolvereturnstatement | spike | frontend/syntax | class: triage-needed | Implement Crashinresolvereturnstatement |
| 1605 | Implement Crashinsourcepropertyisrelatabletotargetproperty | spike | frontend/syntax | class: triage-needed | Implement Crashinsourcepropertyisrelatabletotargetproperty |
| 1606 | Implement Crashintypecheckinvocationexpression | spike | frontend/syntax | class: blocked | Implement Crashintypecheckinvocationexpression |
| 1607 | Implement Crashintypecheckobjectcreationexpression | spike | frontend/syntax | class: blocked | Implement Crashintypecheckobjectcreationexpression |
| 1608 | Implement Crashonmethodsignatures | spike | frontend/syntax | class: triage-needed | Implement Crashonmethodsignatures |
| 1609 | Implement Crashregressiontest | spike | frontend/syntax | class: blocked | Implement Crashregressiontest |
| 1610 | Implement Createarray | spike | frontend/syntax | class: triage-needed | Implement Createarray |
| 1611 | Implement Crossfileoverloadmodifierconsistency | spike | frontend/syntax | class: blocked | Implement Crossfileoverloadmodifierconsistency |
| 1612 | Implement Ctsfileinesnexthelpers | spike | frontend/syntax | class: blocked | Implement Ctsfileinesnexthelpers |
| 1613 | Implement Customasynciterator | spike | runtime/builtins | class: triage-needed | Implement Customasynciterator |
| 1614 | Implement Customeventdetail | spike | frontend/syntax | class: blocked | Implement Customeventdetail |
| 1615 | Implement Cyclicmoduleimport | spike | frontend/syntax | class: blocked | Implement Cyclicmoduleimport |
| 1616 | Implement Dataviewconstructor | spike | frontend/resolver | class: blocked | Implement Dataviewconstructor |
| 1617 | Implement Debugger | spike | frontend/resolver | class: blocked | Implement Debugger |
| 1618 | Implement Debuggeremit | spike | frontend/syntax | class: triage-needed | Implement Debuggeremit |
| 1619 | Implement Declfileaccessors | spike | frontend/syntax | class: blocked | Implement Declfileaccessors |
| 1620 | Implement Declfilealiasusebeforedeclaration | spike | frontend/syntax | class: blocked | Implement Declfilealiasusebeforedeclaration |
| 1621 | Implement Declfileambientexternalmodulewithsingleexportedmodule | spike | frontend/syntax | class: blocked | Implement Declfileambientexternalmodulewithsingleexportedmodule |
| 1622 | Implement Declfileclassextendsnull | spike | frontend/syntax | class: triage-needed | Implement Declfileclassextendsnull |
| 1623 | Implement Declfileclasswithindexsignature | spike | frontend/syntax | class: triage-needed | Implement Declfileclasswithindexsignature |
| 1624 | Implement Declfileclasswithstaticmethodreturningconstructor | spike | frontend/syntax | class: blocked | Implement Declfileclasswithstaticmethodreturningconstructor |
| 1625 | Implement Declfileconstructors | spike | frontend/syntax | class: blocked | Implement Declfileconstructors |
| 1626 | Implement Declfileemitdeclarationonly | spike | frontend/syntax | class: triage-needed | Implement Declfileemitdeclarationonly |
| 1627 | Implement Declfileenumusedasvalue | spike | frontend/syntax | class: triage-needed | Implement Declfileenumusedasvalue |
| 1628 | Implement Declfileenums | spike | frontend/syntax | class: triage-needed | Implement Declfileenums |
| 1629 | Implement Declfileexportassignmentimportinternalmodule | spike | frontend/syntax | class: blocked | Implement Declfileexportassignmentimportinternalmodule |
| 1630 | Implement Declfileexportassignmentofgenericinterface | spike | frontend/syntax | class: blocked | Implement Declfileexportassignmentofgenericinterface |
| 1631 | Implement Declfileexportimportchain | spike | frontend/syntax | class: blocked | Implement Declfileexportimportchain |
| 1632 | Implement Declfileforclasswithmultiplebaseclasses | spike | frontend/syntax | class: blocked | Implement Declfileforclasswithmultiplebaseclasses |
| 1633 | Implement Declfileforclasswithprivateoverloadedfunction | spike | frontend/syntax | class: blocked | Implement Declfileforclasswithprivateoverloadedfunction |
| 1634 | Implement Declfileforexportedimport | spike | frontend/syntax | class: blocked | Implement Declfileforexportedimport |
| 1635 | Implement Declfileforfunctiontypeastypeparameter | spike | frontend/syntax | class: blocked | Implement Declfileforfunctiontypeastypeparameter |
| 1636 | Implement Declfilefortypeparameters | spike | frontend/syntax | class: blocked | Implement Declfilefortypeparameters |
| 1637 | Implement Declfilefunctions | spike | frontend/syntax | class: blocked | Implement Declfilefunctions |
| 1638 | Implement Declfilegenericclasswithgenericextendedclass | spike | frontend/semantics | class: blocked | Implement Declfilegenericclasswithgenericextendedclass |
| 1639 | Implement Declfilegenerictype | spike | frontend/syntax | class: blocked | Implement Declfilegenerictype |
| 1640 | Implement Declfileimportchaininexportassignment | spike | frontend/syntax | class: blocked | Implement Declfileimportchaininexportassignment |
| 1641 | Implement Declfileimportmodulewithexportassignment | spike | frontend/syntax | class: blocked | Implement Declfileimportmodulewithexportassignment |
| 1642 | Implement Declfileimportedtypeuseintypeargposition | spike | frontend/syntax | class: blocked | Implement Declfileimportedtypeuseintypeargposition |
| 1643 | Implement Declfileinternalaliases | spike | frontend/syntax | class: blocked | Implement Declfileinternalaliases |
| 1644 | Implement Declfilemethods | spike | frontend/syntax | class: blocked | Implement Declfilemethods |
| 1645 | Implement Declfilemoduleassignmentinobjectliteralproperty | spike | frontend/syntax | class: blocked | Implement Declfilemoduleassignmentinobjectliteralproperty |
| 1646 | Implement Declfilemodulecontinuation | spike | frontend/syntax | class: blocked | Implement Declfilemodulecontinuation |
| 1647 | Implement Declfilemodulewithpropertyoftypemodule | spike | frontend/syntax | class: blocked | Implement Declfilemodulewithpropertyoftypemodule |
| 1648 | Implement Declfileobjectliteralwithaccessors | spike | frontend/syntax | class: blocked | Implement Declfileobjectliteralwithaccessors |
| 1649 | Implement Declfileobjectliteralwithonlygetter | spike | frontend/syntax | class: blocked | Implement Declfileobjectliteralwithonlygetter |
| 1650 | Implement Declfileobjectliteralwithonlysetter | spike | frontend/syntax | class: blocked | Implement Declfileobjectliteralwithonlysetter |
| 1651 | Implement Declfileprivatemethodoverloads | spike | frontend/syntax | class: blocked | Implement Declfileprivatemethodoverloads |
| 1652 | Implement Declfileprivatestatic | spike | frontend/syntax | class: blocked | Implement Declfileprivatestatic |
| 1653 | Implement Declfilerestparametersoffunctionandfunctiontype | spike | frontend/syntax | class: triage-needed | Implement Declfilerestparametersoffunctionandfunctiontype |
| 1654 | Implement Declfiletypeannotationarraytype | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationarraytype |
| 1655 | Implement Declfiletypeannotationparentype | spike | frontend/syntax | class: triage-needed | Implement Declfiletypeannotationparentype |
| 1656 | Implement Declfiletypeannotationtupletype | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationtupletype |
| 1657 | Implement Declfiletypeannotationtypealias | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationtypealias |
| 1658 | Implement Declfiletypeannotationtypeliteral | spike | frontend/syntax | class: triage-needed | Implement Declfiletypeannotationtypeliteral |
| 1659 | Implement Declfiletypeannotationtypequery | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationtypequery |
| 1660 | Implement Declfiletypeannotationtypereference | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationtypereference |
| 1661 | Implement Declfiletypeannotationuniontype | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationuniontype |
| 1662 | Implement Declfiletypeannotationvisibilityerroraccessors | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationvisibilityerroraccessors |
| 1663 | Implement Declfiletypeannotationvisibilityerrorparameteroffunction | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationvisibilityerrorparameteroffunction |
| 1664 | Implement Declfiletypeannotationvisibilityerrorreturntypeoffunction | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationvisibilityerrorreturntypeoffunction |
| 1665 | Implement Declfiletypeannotationvisibilityerrortypealias | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationvisibilityerrortypealias |
| 1666 | Implement Declfiletypeannotationvisibilityerrortypeliteral | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationvisibilityerrortypeliteral |
| 1667 | Implement Declfiletypeannotationvisibilityerrorvariabledeclaration | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationvisibilityerrorvariabledeclaration |
| 1668 | Implement Declfiletypeofclass | spike | frontend/syntax | class: triage-needed | Implement Declfiletypeofclass |
| 1669 | Implement Declfiletypeofenum | spike | frontend/syntax | class: triage-needed | Implement Declfiletypeofenum |
| 1670 | Implement Declfiletypeoffunction | spike | frontend/syntax | class: triage-needed | Implement Declfiletypeoffunction |
| 1671 | Implement Declfiletypeofinanonymoustype | spike | frontend/syntax | class: blocked | Implement Declfiletypeofinanonymoustype |
| 1672 | Implement Declfiletypeofmodule | spike | frontend/syntax | class: blocked | Implement Declfiletypeofmodule |
| 1673 | Implement Declfilewithclassnameconflictingwithclassreferredbyextendsclause | spike | frontend/syntax | class: blocked | Implement Declfilewithclassnameconflictingwithclassreferredbyextendsclause |
| 1674 | Implement Declfilewitherrorsininputdeclarationfile | spike | frontend/syntax | class: blocked | Implement Declfilewitherrorsininputdeclarationfile |
| 1675 | Implement Declfilewitherrorsininputdeclarationfilewithout | spike | frontend/syntax | class: blocked | Implement Declfilewitherrorsininputdeclarationfilewithout |
| 1676 | Implement Declfilewithextendsclausethathasitscontainernameconflict | spike | frontend/syntax | class: blocked | Implement Declfilewithextendsclausethathasitscontainernameconflict |
| 1677 | Implement Declfilewithinternalmodulenameconflictsinextendsclause | spike | frontend/syntax | class: blocked | Implement Declfilewithinternalmodulenameconflictsinextendsclause |
| 1678 | Implement Declinput Import Export | spike | frontend/syntax | class: blocked | Implement Declinput Import Export |
| 1679 | Implement Declinput Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Declinput Parser Syntax |
| 1680 | Implement Declarationassertionnodenotreusedwhentypenotequivalent | spike | frontend/syntax | class: blocked | Implement Declarationassertionnodenotreusedwhentypenotequivalent |
| 1681 | Implement Declarationemitaliasexportstar | spike | frontend/syntax | class: blocked | Implement Declarationemitaliasexportstar |
| 1682 | Implement Declarationemitaliasfromindirectfile | spike | frontend/syntax | class: blocked | Implement Declarationemitaliasfromindirectfile |
| 1683 | Implement Declarationemitaliasinlineing | spike | frontend/syntax | class: blocked | Implement Declarationemitaliasinlineing |
| 1684 | Implement Declarationemitamdmoduledefault | spike | frontend/syntax | class: blocked | Implement Declarationemitamdmoduledefault |
| 1685 | Implement Declarationemitamdmodulenamedirective | spike | frontend/syntax | class: blocked | Implement Declarationemitamdmodulenamedirective |
| 1686 | Implement Declarationemitanycomputedpropertyinclass | spike | frontend/syntax | class: blocked | Implement Declarationemitanycomputedpropertyinclass |
| 1687 | Implement Declarationemitarrowfunctionnorenaming | spike | frontend/syntax | class: blocked | Implement Declarationemitarrowfunctionnorenaming |
| 1688 | Implement Declarationemitbindingpatternwithreservedword | spike | frontend/syntax | class: blocked | Implement Declarationemitbindingpatternwithreservedword |
| 1689 | Implement Declarationemitbindingpatterns | spike | frontend/syntax | class: blocked | Implement Declarationemitbindingpatterns |
| 1690 | Implement Declarationemitbindingpatternsfunctionexpr | spike | frontend/syntax | class: blocked | Implement Declarationemitbindingpatternsfunctionexpr |
| 1691 | Implement Declarationemitbindingpatternsunused | spike | frontend/syntax | class: blocked | Implement Declarationemitbindingpatternsunused |
| 1692 | Implement Declarationemitbundlewithambientreferences | spike | frontend/syntax | class: blocked | Implement Declarationemitbundlewithambientreferences |
| 1693 | Implement Declarationemitbundlerconditions | spike | frontend/syntax | class: blocked | Implement Declarationemitbundlerconditions |
| 1694 | Implement Declarationemitcastreusestypenode Declaration Emit | spike | frontend/syntax | class: blocked | Implement Declarationemitcastreusestypenode Declaration Emit |
| 1695 | Implement Declarationemitcastreusestypenode Import Export | spike | frontend/syntax | class: blocked | Implement Declarationemitcastreusestypenode Import Export |
| 1696 | Implement Declarationemitclassaccessorsjs | spike | frontend/syntax | class: blocked | Implement Declarationemitclassaccessorsjs |
| 1697 | Implement Declarationemitclassinherritsany | spike | frontend/syntax | class: blocked | Implement Declarationemitclassinherritsany |
| 1698 | Implement Declarationemitclassmembernameconflict Import Export | spike | frontend/syntax | class: blocked | Implement Declarationemitclassmembernameconflict Import Export |
| 1699 | Implement Declarationemitclassmembernameconflict Parser Syntax | spike | frontend/syntax | class: blocked | Implement Declarationemitclassmembernameconflict Parser Syntax |
| 1700 | Implement Declarationemitclassmemberwithcomputedpropertyname | spike | frontend/syntax | class: triage-needed | Implement Declarationemitclassmemberwithcomputedpropertyname |
| 1701 | Implement Declarationemitclassmixinlocalclassdeclaration | spike | frontend/syntax | class: blocked | Implement Declarationemitclassmixinlocalclassdeclaration |
| 1702 | Implement Declarationemitclassprivateconstructor | spike | frontend/syntax | class: blocked | Implement Declarationemitclassprivateconstructor |
| 1703 | Implement Declarationemitclasssetaccessorparamnameinjs | spike | frontend/syntax | class: blocked | Implement Declarationemitclasssetaccessorparamnameinjs |
| 1704 | Implement Declarationemitcommonjsmodulereferencedtype | spike | frontend/syntax | class: blocked | Implement Declarationemitcommonjsmodulereferencedtype |
| 1705 | Implement Declarationemitcommonsourcedirectorydoesnotcontainallfiles | spike | frontend/syntax | class: blocked | Implement Declarationemitcommonsourcedirectorydoesnotcontainallfiles |
| 1706 | Implement Declarationemitcomputednamecausesimporttobepainted | spike | frontend/syntax | class: blocked | Implement Declarationemitcomputednamecausesimporttobepainted |
| 1707 | Implement Declarationemitcomputednameconstenumalias | spike | frontend/syntax | class: triage-needed | Implement Declarationemitcomputednameconstenumalias |
| 1708 | Implement Declarationemitcomputednamewithquestiontoken | spike | frontend/syntax | class: triage-needed | Implement Declarationemitcomputednamewithquestiontoken |
| 1709 | Implement Declarationemitcomputednamesinaccessible | spike | frontend/syntax | class: blocked | Implement Declarationemitcomputednamesinaccessible |
| 1710 | Implement Declarationemitcomputedpropertyname | spike | frontend/syntax | class: blocked | Implement Declarationemitcomputedpropertyname |
| 1711 | Implement Declarationemitcomputedpropertynameenum Declaration Emit | spike | frontend/syntax | class: blocked | Implement Declarationemitcomputedpropertynameenum Declaration Emit |
| 1712 | Implement Declarationemitcomputedpropertynameenum Import Export | spike | frontend/syntax | class: blocked | Implement Declarationemitcomputedpropertynameenum Import Export |
| 1713 | Implement Declarationemitcomputedpropertynamesymbol | spike | frontend/syntax | class: blocked | Implement Declarationemitcomputedpropertynamesymbol |
| 1714 | Implement Declarationemitconstantnowidening | spike | frontend/syntax | class: blocked | Implement Declarationemitconstantnowidening |
| 1715 | Implement Declarationemitcrossfilecopiedgeneratedimporttype | spike | frontend/syntax | class: blocked | Implement Declarationemitcrossfilecopiedgeneratedimporttype |
| 1716 | Implement Declarationemitcrossfileimporttypeofambientmodule | spike | frontend/syntax | class: blocked | Implement Declarationemitcrossfileimporttypeofambientmodule |
| 1717 | Implement Declarationemitdefaultexport Declaration Emit | spike | frontend/syntax | class: blocked | Implement Declarationemitdefaultexport Declaration Emit |
| 1718 | Implement Declarationemitdefaultexport Import Export | spike | frontend/syntax | class: blocked | Implement Declarationemitdefaultexport Import Export |
| 1719 | Implement Declarationemitdefaultexportwithstaticassignment | spike | frontend/syntax | class: blocked | Implement Declarationemitdefaultexportwithstaticassignment |
| 1720 | Implement Declarationemitdefaultexportwithtempvarname | spike | frontend/syntax | class: blocked | Implement Declarationemitdefaultexportwithtempvarname |
| 1721 | Implement Declarationemitdefaultexportwithtempvarnamewithbundling | spike | frontend/syntax | class: blocked | Implement Declarationemitdefaultexportwithtempvarnamewithbundling |
| 1722 | Implement Declarationemitdestructuring | spike | reference/triage | class: triage-needed | Implement Declarationemitdestructuring |
| 1723 | Implement Declarationemitdestructuringarraypattern Import Export | spike | frontend/syntax | class: blocked | Implement Declarationemitdestructuringarraypattern Import Export |
| 1724 | Implement Declarationemitdestructuringarraypattern Runtime Subset | spike | reference/triage | class: triage-needed | Implement Declarationemitdestructuringarraypattern Runtime Subset |
| 1725 | Implement Declarationemitdestructuringobjectliteralpattern | spike | frontend/syntax | class: blocked | Implement Declarationemitdestructuringobjectliteralpattern |
| 1726 | Implement Declarationemitdestructuringoptionalbindingparametersinoverloads | spike | frontend/syntax | class: blocked | Implement Declarationemitdestructuringoptionalbindingparametersinoverloads |
| 1727 | Implement Declarationemitdestructuringparameterproperties | spike | frontend/syntax | class: blocked | Implement Declarationemitdestructuringparameterproperties |
| 1728 | Implement Declarationemitdestructuringprivacyerror | spike | frontend/syntax | class: blocked | Implement Declarationemitdestructuringprivacyerror |
| 1729 | Implement Declarationemitdistributiveconditionalwithinfer | spike | frontend/syntax | class: blocked | Implement Declarationemitdistributiveconditionalwithinfer |
| 1730 | Implement Declarationemitdoesnotusereexportednamespaceaslocal | spike | frontend/syntax | class: blocked | Implement Declarationemitdoesnotusereexportednamespaceaslocal |
| 1731 | Implement Declarationemitduplicateparameterdestructuring | spike | frontend/syntax | class: blocked | Implement Declarationemitduplicateparameterdestructuring |
| 1732 | Implement Declarationemitenumreadonlyproperty | spike | frontend/syntax | class: blocked | Implement Declarationemitenumreadonlyproperty |
| 1733 | Implement Declarationemitenumreferenceviaimportequals | spike | frontend/syntax | class: blocked | Implement Declarationemitenumreferenceviaimportequals |
| 1734 | Implement Declarationemitexactoptionalpropertytypesnodenotreused | spike | frontend/syntax | class: blocked | Implement Declarationemitexactoptionalpropertytypesnodenotreused |
| 1735 | Implement Declarationemitexpandopropertyprivatename | spike | frontend/syntax | class: blocked | Implement Declarationemitexpandopropertyprivatename |
| 1736 | Implement Declarationemitexpandowithgenericconstraint | spike | frontend/syntax | class: blocked | Implement Declarationemitexpandowithgenericconstraint |
| 1737 | Implement Declarationemitexportaliasvisibiilitymarking | spike | frontend/syntax | class: blocked | Implement Declarationemitexportaliasvisibiilitymarking |
| 1738 | Implement Declarationemitexportassignednamespacenotripleslashtypesreference | spike | frontend/syntax | class: blocked | Implement Declarationemitexportassignednamespacenotripleslashtypesreference |
| 1739 | Implement Declarationemitexportassignment | spike | frontend/syntax | class: blocked | Implement Declarationemitexportassignment |
| 1740 | Implement Declarationemitexportdeclaration | spike | frontend/syntax | class: blocked | Implement Declarationemitexportdeclaration |
| 1741 | Implement Declarationemitexpressioninextends Declaration Emit | spike | frontend/syntax | class: blocked | Implement Declarationemitexpressioninextends Declaration Emit |
| 1742 | Implement Declarationemitexpressioninextends Import Export | spike | frontend/syntax | class: blocked | Implement Declarationemitexpressioninextends Import Export |
| 1743 | Implement Declarationemitexpressioninextends Name Resolution | spike | frontend/resolver | class: blocked | Implement Declarationemitexpressioninextends Name Resolution |
| 1744 | Implement Declarationemitexpressionwithnonlocalprivateuniquesymbol | spike | frontend/syntax | class: blocked | Implement Declarationemitexpressionwithnonlocalprivateuniquesymbol |
| 1745 | Implement Declarationemitfirsttypeargumentgenericfunctiontype | spike | frontend/syntax | class: blocked | Implement Declarationemitfirsttypeargumentgenericfunctiontype |
| 1746 | Implement Declarationemitfordefaultexportclassextendingexpression | spike | frontend/syntax | class: blocked | Implement Declarationemitfordefaultexportclassextendingexpression |
| 1747 | Implement Declarationemitforglobalishspecifiersymlink | spike | frontend/syntax | class: blocked | Implement Declarationemitforglobalishspecifiersymlink |
| 1748 | Implement Declarationemitformoduleimportingmoduleaugmentationretainsimport | spike | frontend/syntax | class: blocked | Implement Declarationemitformoduleimportingmoduleaugmentationretainsimport |
| 1749 | Implement Declarationemitfortypeswhichneedimporttypes | spike | frontend/syntax | class: blocked | Implement Declarationemitfortypeswhichneedimporttypes |
| 1750 | Implement Declarationemitfunctionduplicatenamespace | spike | frontend/resolver | class: blocked | Implement Declarationemitfunctionduplicatenamespace |
| 1751 | Implement Declarationemitfunctionkeywordprop | spike | frontend/syntax | class: blocked | Implement Declarationemitfunctionkeywordprop |
| 1752 | Implement Declarationemitgenerictypeparamerserialization | spike | frontend/syntax | class: blocked | Implement Declarationemitgenerictypeparamerserialization |
| 1753 | Implement Declarationemitglobalthispreserved | spike | frontend/syntax | class: blocked | Implement Declarationemitglobalthispreserved |
| 1754 | Implement Declarationemithastypesrefonnamespaceuse | spike | frontend/syntax | class: blocked | Implement Declarationemithastypesrefonnamespaceuse |
| 1755 | Implement Declarationemithigherorderretainedgenerics | spike | frontend/syntax | class: blocked | Implement Declarationemithigherorderretainedgenerics |
| 1756 | Implement Declarationemitimportinexportassignmentmodule | spike | frontend/syntax | class: blocked | Implement Declarationemitimportinexportassignmentmodule |
| 1757 | Implement Declarationemitindextypearray | spike | frontend/syntax | class: blocked | Implement Declarationemitindextypearray |
| 1758 | Implement Declarationemitinferreddefaultexporttype Declaration Emit | spike | frontend/syntax | class: blocked | Implement Declarationemitinferreddefaultexporttype Declaration Emit |
| 1759 | Implement Declarationemitinferreddefaultexporttype Import Export | spike | frontend/syntax | class: blocked | Implement Declarationemitinferreddefaultexporttype Import Export |
| 1760 | Implement Declarationemitinferredtypealias Declaration Emit | spike | frontend/syntax | class: blocked | Implement Declarationemitinferredtypealias Declaration Emit |
| 1761 | Implement Declarationemitinferredtypealias Import Export | spike | frontend/syntax | class: blocked | Implement Declarationemitinferredtypealias Import Export |
| 1762 | Implement Declarationemitinferredtypealias Name Resolution | spike | frontend/resolver | class: blocked | Implement Declarationemitinferredtypealias Name Resolution |
| 1763 | Implement Declarationemitinferredundefinedpropfromfunctioninarray | spike | frontend/syntax | class: blocked | Implement Declarationemitinferredundefinedpropfromfunctioninarray |
| 1764 | Implement Declarationemitinlineddistributiveconditional | spike | frontend/syntax | class: blocked | Implement Declarationemitinlineddistributiveconditional |
| 1765 | Implement Declarationemitinvalidexport | spike | frontend/syntax | class: blocked | Implement Declarationemitinvalidexport |
| 1766 | Implement Declarationemitisolateddeclarationerrornotemittedfornonemittedfile | spike | frontend/syntax | class: blocked | Implement Declarationemitisolateddeclarationerrornotemittedfornonemittedfile |
| 1767 | Implement Declarationemitjsreexportdefault | spike | frontend/syntax | class: blocked | Implement Declarationemitjsreexportdefault |
| 1768 | Implement Declarationemitkeyworddestructuring | spike | frontend/syntax | class: blocked | Implement Declarationemitkeyworddestructuring |
| 1769 | Implement Declarationemitlateboundassignments Declaration Emit | spike | frontend/syntax | class: blocked | Implement Declarationemitlateboundassignments Declaration Emit |
| 1770 | Implement Declarationemitlateboundassignments Import Export | spike | frontend/syntax | class: blocked | Implement Declarationemitlateboundassignments Import Export |
| 1771 | Implement Declarationemitlateboundjsassignments | spike | frontend/syntax | class: blocked | Implement Declarationemitlateboundjsassignments |
| 1772 | Implement Declarationemitlocalclassdeclarationmixin | spike | frontend/syntax | class: blocked | Implement Declarationemitlocalclassdeclarationmixin |
| 1773 | Implement Declarationemitlocalclasshasrequireddeclare | spike | frontend/syntax | class: blocked | Implement Declarationemitlocalclasshasrequireddeclare |
| 1774 | Implement Declarationemitmappedtypedistributivitypreservesconstraints | spike | frontend/syntax | class: blocked | Implement Declarationemitmappedtypedistributivitypreservesconstraints |
| 1775 | Implement Declarationemitmappedtypepreservestypeparameterconstraint | spike | frontend/syntax | class: blocked | Implement Declarationemitmappedtypepreservestypeparameterconstraint |
| 1776 | Implement Declarationemitmappedtypepropertyfromnumericstringkey | spike | frontend/syntax | class: blocked | Implement Declarationemitmappedtypepropertyfromnumericstringkey |
| 1777 | Implement Declarationemitmappedtypetemplatetypeofsymbol | spike | reference/triage | class: triage-needed | Implement Declarationemitmappedtypetemplatetypeofsymbol |
| 1778 | Implement Declarationemitmergedaliaswithconst | spike | frontend/syntax | class: blocked | Implement Declarationemitmergedaliaswithconst |
| 1779 | Implement Declarationemitmethoddeclaration | spike | frontend/syntax | class: blocked | Implement Declarationemitmethoddeclaration |
| 1780 | Implement Declarationemitmixinprivateprotected | spike | frontend/syntax | class: blocked | Implement Declarationemitmixinprivateprotected |
| 1781 | Implement Declarationemitmodulewithscopemarker | spike | frontend/syntax | class: blocked | Implement Declarationemitmodulewithscopemarker |
| 1782 | Implement Declarationemitmonorepobaseurl | spike | frontend/syntax | class: blocked | Implement Declarationemitmonorepobaseurl |
| 1783 | Implement Declarationemitmultiplecomputednamessamedomain | spike | frontend/syntax | class: blocked | Implement Declarationemitmultiplecomputednamessamedomain |
| 1784 | Implement Declarationemitnameconflicts | spike | frontend/syntax | class: blocked | Implement Declarationemitnameconflicts |
| 1785 | Implement Declarationemitnameconflictswithalias | spike | frontend/syntax | class: blocked | Implement Declarationemitnameconflictswithalias |
| 1786 | Implement Declarationemitnamespacemergedwithinterfacenestedfunction | spike | frontend/syntax | class: blocked | Implement Declarationemitnamespacemergedwithinterfacenestedfunction |
| 1787 | Implement Declarationemitnestedanonymousmappedtype | spike | frontend/syntax | class: blocked | Implement Declarationemitnestedanonymousmappedtype |
| 1788 | Implement Declarationemitnestedbindingpattern | spike | frontend/syntax | class: blocked | Implement Declarationemitnestedbindingpattern |
| 1789 | Implement Declarationemitnoinvalidcommentreuse | spike | frontend/syntax | class: blocked | Implement Declarationemitnoinvalidcommentreuse |
| 1790 | Implement Declarationemitnononrequiredparens | spike | frontend/syntax | class: blocked | Implement Declarationemitnononrequiredparens |
| 1791 | Implement Declarationemitnonexportedbindingpattern | spike | frontend/syntax | class: blocked | Implement Declarationemitnonexportedbindingpattern |
| 1792 | Implement Declarationemitobjectassigneddefaultexport | spike | frontend/syntax | class: blocked | Implement Declarationemitobjectassigneddefaultexport |
| 1793 | Implement Declarationemitobjectliteralaccessors | spike | frontend/syntax | class: blocked | Implement Declarationemitobjectliteralaccessors |
| 1794 | Implement Declarationemitobjectliteralaccessorsjs | spike | frontend/syntax | class: blocked | Implement Declarationemitobjectliteralaccessorsjs |
| 1795 | Implement Declarationemitoffuncspace | spike | frontend/syntax | class: blocked | Implement Declarationemitoffuncspace |
| 1796 | Implement Declarationemitoftypeofaliasedexport | spike | frontend/syntax | class: blocked | Implement Declarationemitoftypeofaliasedexport |
| 1797 | Implement Declarationemitoptionalmappedtypepropertynostrictnullchecks Declaration Emit | spike | frontend/syntax | class: blocked | Implement Declarationemitoptionalmappedtypepropertynostrictnullchecks Declaration Emit |
| 1798 | Implement Declarationemitoptionalmappedtypepropertynostrictnullchecks Import Export | spike | frontend/syntax | class: blocked | Implement Declarationemitoptionalmappedtypepropertynostrictnullchecks Import Export |
| 1799 | Implement Declarationemitoptionalmethod | spike | frontend/syntax | class: blocked | Implement Declarationemitoptionalmethod |
| 1800 | Implement Declarationemitoutfilebundlepaths | spike | frontend/syntax | class: blocked | Implement Declarationemitoutfilebundlepaths |
| 1801 | Implement Declarationemitoverloadedprivateinference | spike | frontend/syntax | class: blocked | Implement Declarationemitoverloadedprivateinference |
| 1802 | Implement Declarationemitparameterproperty | spike | frontend/syntax | class: blocked | Implement Declarationemitparameterproperty |
| 1803 | Implement Declarationemitpartialnodereusetypeof | spike | frontend/syntax | class: blocked | Implement Declarationemitpartialnodereusetypeof |
| 1804 | Implement Declarationemitpartialnodereusetypereferences | spike | frontend/syntax | class: blocked | Implement Declarationemitpartialnodereusetypereferences |
| 1805 | Implement Declarationemitpartialreusecomputedproperty | spike | frontend/syntax | class: blocked | Implement Declarationemitpartialreusecomputedproperty |
| 1806 | Implement Declarationemitpathmappingmonorepo Declaration Emit | spike | frontend/syntax | class: blocked | Implement Declarationemitpathmappingmonorepo Declaration Emit |
| 1807 | Implement Declarationemitpathmappingmonorepo Import Export | spike | frontend/syntax | class: blocked | Implement Declarationemitpathmappingmonorepo Import Export |
| 1808 | Implement Declarationemitpreferspathkindbasedonbundling | spike | frontend/syntax | class: blocked | Implement Declarationemitpreferspathkindbasedonbundling |
| 1809 | Implement Declarationemitpreservereferencedimports | spike | frontend/syntax | class: blocked | Implement Declarationemitpreservereferencedimports |
| 1810 | Implement Declarationemitprivateasync | spike | frontend/syntax | class: blocked | Implement Declarationemitprivateasync |
| 1811 | Implement Declarationemitprivatenamecauseserror | spike | frontend/syntax | class: blocked | Implement Declarationemitprivatenamecauseserror |
| 1812 | Implement Declarationemitprivatepromiselikeinterface | spike | frontend/syntax | class: blocked | Implement Declarationemitprivatepromiselikeinterface |
| 1813 | Implement Declarationemitprivatereadonlyliterals | spike | frontend/syntax | class: blocked | Implement Declarationemitprivatereadonlyliterals |
| 1814 | Implement Declarationemitprivatesymbolcausesvardeclarationemit | spike | frontend/syntax | class: blocked | Implement Declarationemitprivatesymbolcausesvardeclarationemit |
| 1815 | Implement Declarationemitprivatesymbolcausesvardeclarationtobeemitted | spike | frontend/syntax | class: blocked | Implement Declarationemitprivatesymbolcausesvardeclarationtobeemitted |
| 1816 | Implement Declarationemitpromise | spike | frontend/syntax | class: blocked | Implement Declarationemitpromise |
| 1817 | Implement Declarationemitpropertynumericstringkey | spike | frontend/syntax | class: blocked | Implement Declarationemitpropertynumericstringkey |
| 1818 | Implement Declarationemitprotectedmembers | spike | frontend/syntax | class: blocked | Implement Declarationemitprotectedmembers |
| 1819 | Implement Declarationemitqualifiedaliastypeargument | spike | frontend/syntax | class: blocked | Implement Declarationemitqualifiedaliastypeargument |
| 1820 | Implement Declarationemitreadonlycomputedproperty | spike | frontend/syntax | class: blocked | Implement Declarationemitreadonlycomputedproperty |
| 1821 | Implement Declarationemitrecursiveconditionalaliaspreserved | spike | frontend/syntax | class: blocked | Implement Declarationemitrecursiveconditionalaliaspreserved |
| 1822 | Implement Declarationemitredundanttripleslashmoduleaugmentation | spike | frontend/syntax | class: blocked | Implement Declarationemitredundanttripleslashmoduleaugmentation |
| 1823 | Implement Declarationemitreexportedsymlinkreference | spike | frontend/syntax | class: blocked | Implement Declarationemitreexportedsymlinkreference |
| 1824 | Implement Declarationemitrelativemoduleerror | spike | frontend/syntax | class: blocked | Implement Declarationemitrelativemoduleerror |
| 1825 | Implement Declarationemitresolvetypesifnotreusable | spike | frontend/syntax | class: triage-needed | Implement Declarationemitresolvetypesifnotreusable |
| 1826 | Implement Declarationemitretainedannotationretainsimportinoutput | spike | frontend/syntax | class: blocked | Implement Declarationemitretainedannotationretainsimportinoutput |
| 1827 | Implement Declarationemitretainsjsdocycomments | spike | frontend/syntax | class: blocked | Implement Declarationemitretainsjsdocycomments |
| 1828 | Implement Declarationemitreuseslambdaparameternodes | spike | frontend/syntax | class: blocked | Implement Declarationemitreuseslambdaparameternodes |
| 1829 | Implement Declarationemitscopeconsistency Declaration Emit | spike | frontend/syntax | class: blocked | Implement Declarationemitscopeconsistency Declaration Emit |
| 1830 | Implement Declarationemitscopeconsistency Import Export | spike | frontend/syntax | class: blocked | Implement Declarationemitscopeconsistency Import Export |
| 1831 | Implement Declarationemitshadowing | spike | frontend/syntax | class: blocked | Implement Declarationemitshadowing |
| 1832 | Implement Declarationemitshadowinginfernotrenamed | spike | frontend/syntax | class: blocked | Implement Declarationemitshadowinginfernotrenamed |
| 1833 | Implement Declarationemitsimplecomputednames | spike | frontend/syntax | class: blocked | Implement Declarationemitsimplecomputednames |
| 1834 | Implement Declarationemitspreadstringlykeyedenum | spike | frontend/syntax | class: blocked | Implement Declarationemitspreadstringlykeyedenum |
| 1835 | Implement Declarationemitstringenumusedinnonlocalspread | spike | frontend/syntax | class: blocked | Implement Declarationemitstringenumusedinnonlocalspread |
| 1836 | Implement Declarationemitsymlinkpaths | spike | frontend/syntax | class: blocked | Implement Declarationemitsymlinkpaths |
| 1837 | Implement Declarationemittodeclarationdirwithcompositeoption | spike | frontend/syntax | class: blocked | Implement Declarationemittodeclarationdirwithcompositeoption |
| 1838 | Implement Declarationemittodeclarationdirwithdeclarationoption | spike | frontend/syntax | class: blocked | Implement Declarationemittodeclarationdirwithdeclarationoption |
| 1839 | Implement Declarationemittodeclarationdirwithoutcompositeanddeclarationoptions | spike | frontend/syntax | class: blocked | Implement Declarationemittodeclarationdirwithoutcompositeanddeclarationoptions |
| 1840 | Implement Declarationemittoplevelnodefromcrossfile | spike | frontend/syntax | class: blocked | Implement Declarationemittoplevelnodefromcrossfile |
| 1841 | Implement Declarationemittransitiveimportofhtmldeclarationitem | spike | frontend/syntax | class: blocked | Implement Declarationemittransitiveimportofhtmldeclarationitem |
| 1842 | Implement Declarationemittripleslashreferenceambientmodule | spike | frontend/syntax | class: blocked | Implement Declarationemittripleslashreferenceambientmodule |
| 1843 | Implement Declarationemittuplerestsignatureleadingvariadic | spike | frontend/syntax | class: blocked | Implement Declarationemittuplerestsignatureleadingvariadic |
| 1844 | Implement Declarationemittypealiaswithtypeparameters | spike | frontend/syntax | class: blocked | Implement Declarationemittypealiaswithtypeparameters |
| 1845 | Implement Declarationemittypeparammergedwithprivate | spike | frontend/syntax | class: blocked | Implement Declarationemittypeparammergedwithprivate |
| 1846 | Implement Declarationemittypeparameternameinouterscope | spike | frontend/syntax | class: blocked | Implement Declarationemittypeparameternameinouterscope |
| 1847 | Implement Declarationemittypeparameternamereusedinoverloads | spike | frontend/syntax | class: blocked | Implement Declarationemittypeparameternamereusedinoverloads |
| 1848 | Implement Declarationemittypeparameternameshadowedinternally | spike | frontend/syntax | class: blocked | Implement Declarationemittypeparameternameshadowedinternally |
| 1849 | Implement Declarationemittypeofdefaultexport | spike | frontend/syntax | class: blocked | Implement Declarationemittypeofdefaultexport |
| 1850 | Implement Declarationemittypeofrest | spike | frontend/syntax | class: blocked | Implement Declarationemittypeofrest |
| 1851 | Implement Declarationemittypeofthisinclass | spike | frontend/syntax | class: blocked | Implement Declarationemittypeofthisinclass |
| 1852 | Implement Declarationemitunknownimport | spike | frontend/syntax | class: blocked | Implement Declarationemitunknownimport |
| 1853 | Implement Declarationemitunnessesarytypereferencenotadded | spike | frontend/syntax | class: blocked | Implement Declarationemitunnessesarytypereferencenotadded |
| 1854 | Implement Declarationemitunsafeimportsymbolname | spike | frontend/syntax | class: blocked | Implement Declarationemitunsafeimportsymbolname |
| 1855 | Implement Declarationemitusingalternativecontainingmodules | spike | frontend/syntax | class: blocked | Implement Declarationemitusingalternativecontainingmodules |
| 1856 | Implement Declarationemitusingtypealias Declaration Emit | spike | frontend/syntax | class: blocked | Implement Declarationemitusingtypealias Declaration Emit |
| 1857 | Implement Declarationemitusingtypealias Import Export | spike | frontend/syntax | class: blocked | Implement Declarationemitusingtypealias Import Export |
| 1858 | Implement Declarationemitvarinelidedblock | spike | frontend/syntax | class: blocked | Implement Declarationemitvarinelidedblock |
| 1859 | Implement Declarationemitwithcomposite | spike | frontend/syntax | class: blocked | Implement Declarationemitwithcomposite |
| 1860 | Implement Declarationemitwithdefaultascomputedname | spike | frontend/syntax | class: blocked | Implement Declarationemitwithdefaultascomputedname |
| 1861 | Implement Declarationemitwithinvalidpackagejsontypings | spike | frontend/syntax | class: blocked | Implement Declarationemitwithinvalidpackagejsontypings |
| 1862 | Implement Declarationfilenocrashonextraexportmodifier | spike | frontend/syntax | class: blocked | Implement Declarationfilenocrashonextraexportmodifier |
| 1863 | Implement Declarationfilesgeneratingtypereferences | spike | frontend/syntax | class: triage-needed | Implement Declarationfilesgeneratingtypereferences |
| 1864 | Implement Declarationfileswithtypereferences | spike | frontend/syntax | class: triage-needed | Implement Declarationfileswithtypereferences |
| 1865 | Implement Declarationfunctiontypenonlocalshouldnotbeanerror | spike | frontend/syntax | class: blocked | Implement Declarationfunctiontypenonlocalshouldnotbeanerror |
| 1866 | Implement Declarationimporttypealiasinferredandemittable | spike | frontend/syntax | class: blocked | Implement Declarationimporttypealiasinferredandemittable |
| 1867 | Implement Declarationmaps | spike | frontend/syntax | class: blocked | Implement Declarationmaps |
| 1868 | Implement Declarationmapsmultifile | spike | frontend/syntax | class: blocked | Implement Declarationmapsmultifile |
| 1869 | Implement Declarationmapsoutfile | spike | frontend/syntax | class: blocked | Implement Declarationmapsoutfile |
| 1870 | Implement Declarationmapswithoutdeclaration | spike | frontend/syntax | class: blocked | Implement Declarationmapswithoutdeclaration |
| 1871 | Implement Declarationmerging Import Export | spike | frontend/syntax | class: blocked | Implement Declarationmerging Import Export |
| 1872 | Implement Declarationmerging Parser Syntax | spike | frontend/syntax | class: blocked | Implement Declarationmerging Parser Syntax |
| 1873 | Implement Declarationnodanglinggenerics | spike | frontend/semantics | class: blocked | Implement Declarationnodanglinggenerics |
| 1874 | Implement Declarationquotedmembers | spike | frontend/syntax | class: triage-needed | Implement Declarationquotedmembers |
| 1875 | Implement Declarationtypechecknousebeforereferencecheck | spike | frontend/syntax | class: blocked | Implement Declarationtypechecknousebeforereferencecheck |
| 1876 | Implement Declarationsforfileshadowingglobalnoerror | spike | frontend/syntax | class: blocked | Implement Declarationsforfileshadowingglobalnoerror |
| 1877 | Implement Declarationsforindirecttypealiasreference | spike | frontend/syntax | class: blocked | Implement Declarationsforindirecttypealiasreference |
| 1878 | Implement Declarationsforinferredtypefromotherfile | spike | frontend/syntax | class: blocked | Implement Declarationsforinferredtypefromotherfile |
| 1879 | Implement Declarationsindirectgeneratedaliasreference | spike | frontend/syntax | class: blocked | Implement Declarationsindirectgeneratedaliasreference |
| 1880 | Implement Declarationswithrecursiveinternaltypesproduceuniquetypeparams | spike | frontend/syntax | class: triage-needed | Implement Declarationswithrecursiveinternaltypesproduceuniquetypeparams |
| 1881 | Implement Declarealreadyseen | spike | frontend/syntax | class: blocked | Implement Declarealreadyseen |
| 1882 | Implement Declareclassinterfaceimplementation | spike | frontend/syntax | class: blocked | Implement Declareclassinterfaceimplementation |
| 1883 | Implement Declaredottedextend | spike | frontend/syntax | class: blocked | Implement Declaredottedextend |
| 1884 | Implement Declaredottedmodulename | spike | frontend/syntax | class: blocked | Implement Declaredottedmodulename |
| 1885 | Implement Declareexternalmodulewithexportassignedfundule | spike | frontend/syntax | class: blocked | Implement Declareexternalmodulewithexportassignedfundule |
| 1886 | Implement Declarefileexportassignment | spike | frontend/syntax | class: blocked | Implement Declarefileexportassignment |
| 1887 | Implement Declarefileexportassignmentwithvarfromvariablestatement | spike | frontend/syntax | class: blocked | Implement Declarefileexportassignmentwithvarfromvariablestatement |
| 1888 | Implement Declareidentifierasbeginningofstatementexpression | spike | frontend/syntax | class: triage-needed | Implement Declareidentifierasbeginningofstatementexpression |
| 1889 | Implement Declaremodifieronimport | spike | frontend/syntax | class: triage-needed | Implement Declaremodifieronimport |
| 1890 | Implement Declaredexternalmodule | spike | frontend/syntax | class: blocked | Implement Declaredexternalmodule |
| 1891 | Implement Declaredexternalmodulewithexportassignment | spike | frontend/syntax | class: blocked | Implement Declaredexternalmodulewithexportassignment |
| 1892 | Implement Decoratorinjsfile | spike | frontend/syntax | class: blocked | Implement Decoratorinjsfile |
| 1893 | Implement Decoratormetadataconditionaltype | spike | frontend/syntax | class: blocked | Implement Decoratormetadataconditionaltype |
| 1894 | Implement Decoratormetadataelidedimport | spike | frontend/syntax | class: blocked | Implement Decoratormetadataelidedimport |
| 1895 | Implement Decoratormetadataelidedimportondeclare | spike | frontend/syntax | class: blocked | Implement Decoratormetadataelidedimportondeclare |
| 1896 | Implement Decoratormetadataformethodwithnoreturntypeannotation | spike | frontend/syntax | class: blocked | Implement Decoratormetadataformethodwithnoreturntypeannotation |
| 1897 | Implement Decoratormetadatagenerictypevariable | spike | frontend/syntax | class: blocked | Implement Decoratormetadatagenerictypevariable |
| 1898 | Implement Decoratormetadatagenerictypevariabledefault | spike | frontend/syntax | class: blocked | Implement Decoratormetadatagenerictypevariabledefault |
| 1899 | Implement Decoratormetadatagenerictypevariableinscope | spike | frontend/syntax | class: blocked | Implement Decoratormetadatagenerictypevariableinscope |
| 1900 | Implement Decoratormetadatanolibisolatedmodulestypes | spike | frontend/syntax | class: blocked | Implement Decoratormetadatanolibisolatedmodulestypes |
| 1901 | Implement Decoratormetadatanostrictnull | spike | frontend/syntax | class: blocked | Implement Decoratormetadatanostrictnull |
| 1902 | Implement Decoratormetadataoninferredtype | spike | frontend/syntax | class: blocked | Implement Decoratormetadataoninferredtype |
| 1903 | Implement Decoratormetadatapromise | spike | frontend/syntax | class: blocked | Implement Decoratormetadatapromise |
| 1904 | Implement Decoratormetadatarestparameterwithimportedtype | spike | frontend/syntax | class: blocked | Implement Decoratormetadatarestparameterwithimportedtype |
| 1905 | Implement Decoratormetadatatypeonlyexport | spike | frontend/syntax | class: blocked | Implement Decoratormetadatatypeonlyexport |
| 1906 | Implement Decoratormetadatatypeonlyimport | spike | frontend/syntax | class: blocked | Implement Decoratormetadatatypeonlyimport |
| 1907 | Implement Decoratormetadatawithconstructortype | spike | frontend/syntax | class: blocked | Implement Decoratormetadatawithconstructortype |
| 1908 | Implement Decoratormetadatawithimportdeclarationnamecollision | spike | frontend/syntax | class: blocked | Implement Decoratormetadatawithimportdeclarationnamecollision |
| 1909 | Implement Decoratorreferenceonotherproperty | spike | frontend/syntax | class: blocked | Implement Decoratorreferenceonotherproperty |
| 1910 | Implement Decoratorreferences | spike | frontend/syntax | class: blocked | Implement Decoratorreferences |
| 1911 | Implement Decoratorusedbeforedeclaration | spike | frontend/syntax | class: blocked | Implement Decoratorusedbeforedeclaration |
| 1912 | Implement Decoratorwithnegativeliteraltypenocrash | spike | frontend/syntax | class: blocked | Implement Decoratorwithnegativeliteraltypenocrash |
| 1913 | Implement Decoratorwithunderscoremethod | spike | frontend/syntax | class: blocked | Implement Decoratorwithunderscoremethod |
| 1914 | Implement Decoratorsoncomputedproperties | spike | frontend/syntax | class: blocked | Implement Decoratorsoncomputedproperties |
| 1915 | Implement Decrementandincrementoperators | spike | frontend/syntax | class: triage-needed | Implement Decrementandincrementoperators |
| 1916 | Implement Deduplicateimportsinsystem | spike | frontend/syntax | class: blocked | Implement Deduplicateimportsinsystem |
| 1917 | Implement Deepcomparisons | spike | frontend/syntax | class: triage-needed | Implement Deepcomparisons |
| 1918 | Implement Deepelaborationsintoarrowexpressions | spike | frontend/syntax | class: triage-needed | Implement Deepelaborationsintoarrowexpressions |
| 1919 | Implement Deepexcesspropertycheckingwhentargetisintersection | spike | frontend/syntax | class: triage-needed | Implement Deepexcesspropertycheckingwhentargetisintersection |
| 1920 | Implement Deepkeysindexing | spike | frontend/syntax | class: triage-needed | Implement Deepkeysindexing |
| 1921 | Implement Deeplydependentlargearraymutation | spike | frontend/syntax | class: triage-needed | Implement Deeplydependentlargearraymutation |
| 1922 | Implement Deeplynestedassignabilityerrorscombined | spike | runtime/builtins | class: triage-needed | Implement Deeplynestedassignabilityerrorscombined |
| 1923 | Implement Deeplynestedassignabilityissue | spike | frontend/syntax | class: triage-needed | Implement Deeplynestedassignabilityissue |
| 1924 | Implement Deeplynestedcheck | spike | frontend/syntax | class: triage-needed | Implement Deeplynestedcheck |
| 1925 | Implement Deeplynestedconstraints | spike | frontend/syntax | class: blocked | Implement Deeplynestedconstraints |
| 1926 | Implement Deeplynestedmappedtypes | spike | frontend/syntax | class: blocked | Implement Deeplynestedmappedtypes |
| 1927 | Implement Deeplynestedtemplateliteralintersection | spike | frontend/syntax | class: blocked | Implement Deeplynestedtemplateliteralintersection |
| 1928 | Implement Defaultargsinfunctionexpressions | spike | frontend/syntax | class: blocked | Implement Defaultargsinfunctionexpressions |
| 1929 | Implement Defaultargsinoverloads | spike | frontend/syntax | class: blocked | Implement Defaultargsinoverloads |
| 1930 | Implement Defaultdeclarationemitdefaultimport | spike | frontend/syntax | class: blocked | Implement Defaultdeclarationemitdefaultimport |
| 1931 | Implement Defaultdeclarationemitnamedcorrectly | spike | frontend/syntax | class: blocked | Implement Defaultdeclarationemitnamedcorrectly |
| 1932 | Implement Defaultdeclarationemitshadowednamedcorrectly | spike | frontend/syntax | class: blocked | Implement Defaultdeclarationemitshadowednamedcorrectly |
| 1933 | Implement Defaultindexprops | spike | frontend/syntax | class: triage-needed | Implement Defaultindexprops |
| 1934 | Implement Defaultisnotvisibleinlocalscope | spike | frontend/syntax | class: blocked | Implement Defaultisnotvisibleinlocalscope |
| 1935 | Implement Defaultkeywordwithoutexport | spike | frontend/syntax | class: blocked | Implement Defaultkeywordwithoutexport |
| 1936 | Implement Defaultnamedexportwithtype | spike | frontend/syntax | class: blocked | Implement Defaultnamedexportwithtype |
| 1937 | Implement Defaultparameteraddsundefinedwithstrictnullchecks | spike | frontend/resolver | class: blocked | Implement Defaultparameteraddsundefinedwithstrictnullchecks |
| 1938 | Implement Defaultpropsemptycurlybecomesanyforjs | spike | frontend/syntax | class: blocked | Implement Defaultpropsemptycurlybecomesanyforjs |
| 1939 | Implement Defaultvalueinconstructoroverload | spike | frontend/syntax | class: blocked | Implement Defaultvalueinconstructoroverload |
| 1940 | Implement Defaultvalueinfunctionoverload | spike | frontend/syntax | class: blocked | Implement Defaultvalueinfunctionoverload |
| 1941 | Implement Defaultvalueinfunctiontypes | spike | frontend/syntax | class: triage-needed | Implement Defaultvalueinfunctiontypes |
| 1942 | Implement Deferredconditionaltypes | spike | frontend/semantics | class: blocked | Implement Deferredconditionaltypes |
| 1943 | Implement Deferredlookuptyperesolution | spike | frontend/resolver | class: blocked | Implement Deferredlookuptyperesolution |
| 1944 | Implement Definevariables | spike | frontend/syntax | class: blocked | Implement Definevariables |
| 1945 | Implement Definiteassignmentofdestructuredvariable | spike | frontend/syntax | class: blocked | Implement Definiteassignmentofdestructuredvariable |
| 1946 | Implement Definiteassignmentwitherrorstillstripped | spike | frontend/syntax | class: blocked | Implement Definiteassignmentwitherrorstillstripped |
| 1947 | Implement Deleteexpressionmustbeoptional | spike | frontend/syntax | class: triage-needed | Implement Deleteexpressionmustbeoptional |
| 1948 | Implement Deletereadonlyinstrictnullchecks | spike | frontend/resolver | class: blocked | Implement Deletereadonlyinstrictnullchecks |
| 1949 | Implement Dependencyviaimportalias | spike | frontend/syntax | class: blocked | Implement Dependencyviaimportalias |
| 1950 | Implement Derivedclassconstructorwithexplicitreturns | spike | frontend/syntax | class: blocked | Implement Derivedclassconstructorwithexplicitreturns |
| 1951 | Implement Derivedclassoverridesprivatefunction | spike | frontend/syntax | class: blocked | Implement Derivedclassoverridesprivatefunction |
| 1952 | Implement Derivedclasses | spike | frontend/syntax | class: blocked | Implement Derivedclasses |
| 1953 | Implement Derivedinterfacecallsignature | spike | frontend/resolver | class: blocked | Implement Derivedinterfacecallsignature |
| 1954 | Implement Derivedtypecallingbaseimplwithoptionalparams | spike | frontend/syntax | class: triage-needed | Implement Derivedtypecallingbaseimplwithoptionalparams |
| 1955 | Implement Destructionassignmenterror | spike | runtime/builtins | class: triage-needed | Implement Destructionassignmenterror |
| 1956 | Implement Destructurecatchclause | spike | frontend/syntax | class: blocked | Implement Destructurecatchclause |
| 1957 | Implement Destructurecomputedproperty | spike | frontend/syntax | class: blocked | Implement Destructurecomputedproperty |
| 1958 | Implement Destructureofvariablesameasshorthand | spike | reference/triage | class: triage-needed | Implement Destructureofvariablesameasshorthand |
| 1959 | Implement Destructuretuplewithvariableelement | spike | frontend/syntax | class: blocked | Implement Destructuretuplewithvariableelement |
| 1960 | Implement Destructureddeclarationemit | spike | frontend/syntax | class: blocked | Implement Destructureddeclarationemit |
| 1961 | Implement Destructuredlateboundnamehascorrecttypes | spike | frontend/syntax | class: blocked | Implement Destructuredlateboundnamehascorrecttypes |
| 1962 | Implement Destructuredmaappedtypeisnotimplicitlyany | spike | frontend/syntax | class: blocked | Implement Destructuredmaappedtypeisnotimplicitlyany |
| 1963 | Implement Destructuringassignment | spike | frontend/syntax | class: blocked | Implement Destructuringassignment |
| 1964 | Implement Destructuringassignmentwithdefault | spike | frontend/resolver | class: blocked | Implement Destructuringassignmentwithdefault |
| 1965 | Implement Destructuringassignmentwithexportedname | spike | frontend/syntax | class: blocked | Implement Destructuringassignmentwithexportedname |
| 1966 | Implement Destructuringassignmentwithstrictnullchecks | spike | frontend/resolver | class: blocked | Implement Destructuringassignmentwithstrictnullchecks |
| 1967 | Implement Destructuringcontrolflownocrash | spike | frontend/syntax | class: blocked | Implement Destructuringcontrolflownocrash |
| 1968 | Implement Destructuringfromunionspread | spike | frontend/syntax | class: blocked | Implement Destructuringfromunionspread |
| 1969 | Implement Destructuringinvariabledeclarations Destructuring | spike | frontend/syntax | class: blocked | Implement Destructuringinvariabledeclarations Destructuring |
| 1970 | Implement Destructuringinvariabledeclarations Import Export | spike | frontend/syntax | class: blocked | Implement Destructuringinvariabledeclarations Import Export |
| 1971 | Implement Destructuringinitializercontextualtypefromcontext | spike | frontend/syntax | class: blocked | Implement Destructuringinitializercontextualtypefromcontext |
| 1972 | Implement Destructuringpropertyassignmentnameisnotassignmenttarget | spike | frontend/resolver | class: blocked | Implement Destructuringpropertyassignmentnameisnotassignmenttarget |
| 1973 | Implement Destructuringtempoccursafterprologue | spike | frontend/syntax | class: blocked | Implement Destructuringtempoccursafterprologue |
| 1974 | Implement Destructuringtuple | spike | frontend/resolver | class: blocked | Implement Destructuringtuple |
| 1975 | Implement Destructuringunspreadableintorest | spike | frontend/syntax | class: blocked | Implement Destructuringunspreadableintorest |
| 1976 | Implement Destructuringwithgenericparameter | spike | frontend/syntax | class: blocked | Implement Destructuringwithgenericparameter |
| 1977 | Implement Destructuringwithnewexpression | spike | frontend/syntax | class: blocked | Implement Destructuringwithnewexpression |
| 1978 | Implement Detachedcommentatstartofconstructor | spike | frontend/syntax | class: triage-needed | Implement Detachedcommentatstartofconstructor |
| 1979 | Implement Didyoumeanelaborationsforexpressionswhichcouldbecalled | spike | frontend/syntax | class: triage-needed | Implement Didyoumeanelaborationsforexpressionswhichcouldbecalled |
| 1980 | Implement Didyoumeansuggestionerrors | spike | frontend/syntax | class: blocked | Implement Didyoumeansuggestionerrors |
| 1981 | Implement Differenttypeswithsamename | spike | frontend/syntax | class: blocked | Implement Differenttypeswithsamename |
| 1982 | Implement Disallowedblockscopedinpresenceofparseerrors | spike | frontend/resolver | class: blocked | Implement Disallowedblockscopedinpresenceofparseerrors |
| 1983 | Implement Discriminableunionwithintersectedmembers | spike | frontend/syntax | class: triage-needed | Implement Discriminableunionwithintersectedmembers |
| 1984 | Implement Discriminantnarrowingcouldbecircular | spike | frontend/syntax | class: triage-needed | Implement Discriminantnarrowingcouldbecircular |
| 1985 | Implement Discriminantorderindependence | spike | frontend/resolver | class: blocked | Implement Discriminantorderindependence |
| 1986 | Implement Discriminantpropertycheck | spike | frontend/syntax | class: triage-needed | Implement Discriminantpropertycheck |
| 1987 | Implement Discriminantpropertyinference | spike | frontend/resolver | class: blocked | Implement Discriminantpropertyinference |
| 1988 | Implement Discriminantusingevaluatabletemplateexpression | spike | frontend/syntax | class: blocked | Implement Discriminantusingevaluatabletemplateexpression |
| 1989 | Implement Discriminantsandnullorundefined | spike | frontend/resolver | class: blocked | Implement Discriminantsandnullorundefined |
| 1990 | Implement Discriminantsandprimitives | spike | frontend/syntax | class: triage-needed | Implement Discriminantsandprimitives |
| 1991 | Implement Discriminatewithdivergentaccessors | spike | frontend/resolver | class: blocked | Implement Discriminatewithdivergentaccessors |
| 1992 | Implement Discriminatewithmissingproperty | spike | frontend/resolver | class: blocked | Implement Discriminatewithmissingproperty |
| 1993 | Implement Discriminatewithoptionalproperty Import Export | spike | frontend/syntax | class: blocked | Implement Discriminatewithoptionalproperty Import Export |
| 1994 | Implement Discriminatewithoptionalproperty Name Resolution | spike | frontend/resolver | class: blocked | Implement Discriminatewithoptionalproperty Name Resolution |
| 1995 | Implement Discriminatewithoptionalproperty Parser Syntax | spike | frontend/syntax | class: blocked | Implement Discriminatewithoptionalproperty Parser Syntax |
| 1996 | Implement Discriminatedunionerrormessage | spike | frontend/syntax | class: blocked | Implement Discriminatedunionerrormessage |
| 1997 | Implement Discriminatedunionwithindexsignature | spike | frontend/syntax | class: blocked | Implement Discriminatedunionwithindexsignature |
| 1998 | Implement Discriminatingunionwithunionpropertyagainstundefinedwithoutstrictnullchecks | spike | frontend/syntax | class: blocked | Implement Discriminatingunionwithunionpropertyagainstundefinedwithoutstrictnullchecks |
| 2000 | Implement Divergentaccessors | spike | frontend/syntax | class: blocked | Implement Divergentaccessors |
| 2001 | Implement Divergentaccessorstypes Class Accessor | spike | frontend/syntax | class: blocked | Implement Divergentaccessorstypes Class Accessor |
| 2002 | Implement Divergentaccessorstypes Name Resolution | spike | frontend/resolver | class: blocked | Implement Divergentaccessorstypes Name Resolution |
| 2003 | Implement Divergentaccessorstypes Parser Syntax | spike | frontend/syntax | class: blocked | Implement Divergentaccessorstypes Parser Syntax |
| 2004 | Implement Divergentaccessorsvisibility | spike | frontend/syntax | class: blocked | Implement Divergentaccessorsvisibility |
| 2005 | Implement Divideandconquerintersections | spike | frontend/syntax | class: blocked | Implement Divideandconquerintersections |
| 2006 | Implement Donotelaborateassignabilitytotypeparameters | spike | reference/triage | class: triage-needed | Implement Donotelaborateassignabilitytotypeparameters |
| 2007 | Implement Donotemitdetachedcommentsatstartoflambdafunction | spike | frontend/syntax | class: triage-needed | Implement Donotemitdetachedcommentsatstartoflambdafunction |
| 2008 | Implement Donotemitpinnedcommentnotontopoffile | spike | reference/triage | class: triage-needed | Implement Donotemitpinnedcommentnotontopoffile |
| 2009 | Implement Donotemitpinnedcommentonnotemittednode | spike | frontend/syntax | class: triage-needed | Implement Donotemitpinnedcommentonnotemittednode |
| 2010 | Implement Donotemitpinnedcommentonnotemittednodets | spike | frontend/syntax | class: triage-needed | Implement Donotemitpinnedcommentonnotemittednodets |
| 2011 | Implement Donotinferunrelatedtypes | spike | frontend/resolver | class: blocked | Implement Donotinferunrelatedtypes |
| 2012 | Implement Doyouneedtochangeyourtargetlibraryes Import Export | spike | frontend/syntax | class: blocked | Implement Doyouneedtochangeyourtargetlibraryes Import Export |
| 2013 | Implement Doyouneedtochangeyourtargetlibraryes Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Doyouneedtochangeyourtargetlibraryes Parser Syntax |
| 2014 | Implement Doyouneedtochangeyourtargetlibraryes Try Catch | spike | frontend/syntax | class: blocked | Implement Doyouneedtochangeyourtargetlibraryes Try Catch |
| 2015 | Implement Doesnotnarrowunionofconstructorswithinstanceof | spike | frontend/syntax | class: triage-needed | Implement Doesnotnarrowunionofconstructorswithinstanceof |
| 2016 | Implement Dottedmodulename | spike | frontend/syntax | class: blocked | Implement Dottedmodulename |
| 2017 | Implement Dottednamesinsystem | spike | frontend/syntax | class: blocked | Implement Dottednamesinsystem |
| 2018 | Implement Doublemixinconditionaltypebaseclassworks | spike | frontend/semantics | class: blocked | Implement Doublemixinconditionaltypebaseclassworks |
| 2019 | Implement Doubleunderscoreenumemit | spike | frontend/syntax | class: triage-needed | Implement Doubleunderscoreenumemit |
| 2020 | Implement Doubleunderscoreexportstarconflict | spike | frontend/syntax | class: blocked | Implement Doubleunderscoreexportstarconflict |
| 2021 | Implement Doubleunderscorereactnamespace | spike | frontend/syntax | class: triage-needed | Implement Doubleunderscorereactnamespace |
| 2022 | Implement Downleveliterationdeprecated | spike | frontend/resolver | class: blocked | Implement Downleveliterationdeprecated |
| 2023 | Implement Downlevelletconst Arrow Function | spike | frontend/syntax | class: blocked | Implement Downlevelletconst Arrow Function |
| 2024 | Implement Downlevelletconst Import Export | spike | frontend/syntax | class: blocked | Implement Downlevelletconst Import Export |
| 2025 | Implement Downlevelletconst Name Resolution | spike | frontend/resolver | class: blocked | Implement Downlevelletconst Name Resolution |
| 2026 | Implement Downlevelletconst Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Downlevelletconst Parser Syntax |
| 2027 | Implement Downlevelletconst Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Downlevelletconst Unknown Unsupported |
| 2028 | Implement Dtsemittripleslashavoidunnecessaryresolutionmode | spike | frontend/syntax | class: triage-needed | Implement Dtsemittripleslashavoidunnecessaryresolutionmode |
| 2029 | Implement Duplicateanonymousinners | spike | frontend/syntax | class: blocked | Implement Duplicateanonymousinners |
| 2030 | Implement Duplicateanonymousmoduleclasses | spike | frontend/syntax | class: blocked | Implement Duplicateanonymousmoduleclasses |
| 2031 | Implement Duplicateclasselements | spike | frontend/syntax | class: blocked | Implement Duplicateclasselements |
| 2032 | Implement Duplicateconstructoroverloadsignature | spike | frontend/syntax | class: blocked | Implement Duplicateconstructoroverloadsignature |
| 2033 | Implement Duplicatedefaultexport | spike | frontend/syntax | class: blocked | Implement Duplicatedefaultexport |
| 2034 | Implement Duplicateerrorassignability | spike | frontend/syntax | class: blocked | Implement Duplicateerrorassignability |
| 2035 | Implement Duplicateerrorclassexpression | spike | frontend/syntax | class: blocked | Implement Duplicateerrorclassexpression |
| 2036 | Implement Duplicateerrornamenotfound | spike | frontend/syntax | class: blocked | Implement Duplicateerrornamenotfound |
| 2037 | Implement Duplicateidentifierbindingelementinparameterdeclaration | spike | reference/triage | class: triage-needed | Implement Duplicateidentifierbindingelementinparameterdeclaration |
| 2038 | Implement Duplicateidentifiercomputedname | spike | frontend/syntax | class: blocked | Implement Duplicateidentifiercomputedname |
| 2039 | Implement Duplicateidentifierdifferentmodifiers | spike | frontend/syntax | class: blocked | Implement Duplicateidentifierdifferentmodifiers |
| 2040 | Implement Duplicateidentifierdifferentspelling | spike | frontend/syntax | class: blocked | Implement Duplicateidentifierdifferentspelling |
| 2041 | Implement Duplicateidentifierenum | spike | frontend/syntax | class: blocked | Implement Duplicateidentifierenum |
| 2042 | Implement Duplicateidentifierincatchblock | spike | reference/triage | class: triage-needed | Implement Duplicateidentifierincatchblock |
| 2043 | Implement Duplicateidentifierrelatedspans Duplicate Function | spike | reference/triage | class: triage-needed | Implement Duplicateidentifierrelatedspans Duplicate Function |
| 2044 | Implement Duplicateidentifierrelatedspans Import Export | spike | frontend/syntax | class: blocked | Implement Duplicateidentifierrelatedspans Import Export |
| 2045 | Implement Duplicateidentifierrelatedspans Parser Syntax | spike | frontend/syntax | class: blocked | Implement Duplicateidentifierrelatedspans Parser Syntax |
| 2046 | Implement Duplicateidentifiershouldnotshortcircuitbasetypebinding | spike | frontend/syntax | class: blocked | Implement Duplicateidentifiershouldnotshortcircuitbasetypebinding |
| 2047 | Implement Duplicateidentifiersacrosscontainerboundaries | spike | frontend/syntax | class: blocked | Implement Duplicateidentifiersacrosscontainerboundaries |
| 2048 | Implement Duplicateidentifiersacrossfileboundaries | spike | frontend/syntax | class: blocked | Implement Duplicateidentifiersacrossfileboundaries |
| 2049 | Implement Duplicatelabel | spike | frontend/syntax | class: triage-needed | Implement Duplicatelabel |
| 2051 | Implement Duplicatelocalvariable Import Export | spike | frontend/syntax | class: blocked | Implement Duplicatelocalvariable Import Export |
| 2052 | Implement Duplicatelocalvariable Parser Syntax | spike | frontend/syntax | class: blocked | Implement Duplicatelocalvariable Parser Syntax |
| 2053 | Implement Duplicateobjectliteralproperty Import Export | spike | frontend/syntax | class: blocked | Implement Duplicateobjectliteralproperty Import Export |
| 2054 | Implement Duplicateobjectliteralproperty Object Literal | spike | frontend/syntax | class: blocked | Implement Duplicateobjectliteralproperty Object Literal |
| 2055 | Implement Duplicateobjectliteralproperty Parser Syntax | spike | frontend/syntax | class: blocked | Implement Duplicateobjectliteralproperty Parser Syntax |
| 2056 | Implement Duplicateoverloadintypeaugmentation | spike | frontend/syntax | class: blocked | Implement Duplicateoverloadintypeaugmentation |
| 2057 | Implement Duplicatepackage Import Export | spike | frontend/syntax | class: blocked | Implement Duplicatepackage Import Export |
| 2058 | Implement Duplicatepackage Module Resolution | spike | frontend/syntax | class: blocked | Implement Duplicatepackage Module Resolution |
| 2059 | Implement Duplicatepackage Parser Syntax | spike | frontend/syntax | class: blocked | Implement Duplicatepackage Parser Syntax |
| 2060 | Implement Duplicatepropertiesinstrictmode | spike | frontend/syntax | class: blocked | Implement Duplicatepropertiesinstrictmode |
| 2061 | Implement Duplicatesymbolsexportmatching | spike | frontend/syntax | class: blocked | Implement Duplicatesymbolsexportmatching |
| 2062 | Implement Duplicatetypeparameters | spike | frontend/syntax | class: blocked | Implement Duplicatetypeparameters |
| 2063 | Implement Duplicatevarandimport | spike | frontend/syntax | class: blocked | Implement Duplicatevarandimport |
| 2064 | Implement Duplicatevariabledeclaration | spike | frontend/syntax | class: blocked | Implement Duplicatevariabledeclaration |
| 2065 | Implement Duplicatevariablesbyscope | spike | frontend/syntax | class: blocked | Implement Duplicatevariablesbyscope |
| 2066 | Implement Duplicatevariableswithany | spike | frontend/syntax | class: blocked | Implement Duplicatevariableswithany |
| 2067 | Implement Duplicatevarsacrossfileboundaries | spike | frontend/syntax | class: blocked | Implement Duplicatevarsacrossfileboundaries |
| 2068 | Implement Dynamicimportevaluatespecifier | spike | frontend/syntax | class: blocked | Implement Dynamicimportevaluatespecifier |
| 2069 | Implement Dynamicimportindefaultexportexpression | spike | frontend/syntax | class: blocked | Implement Dynamicimportindefaultexportexpression |
| 2070 | Implement Dynamicimporttrailingcomma | spike | frontend/syntax | class: blocked | Implement Dynamicimporttrailingcomma |
| 2071 | Implement Dynamicimportwithnestedthis | spike | frontend/syntax | class: blocked | Implement Dynamicimportwithnestedthis |
| 2072 | Implement Dynamicimportsdeclaration | spike | frontend/syntax | class: blocked | Implement Dynamicimportsdeclaration |
| 2073 | Implement Dynamicmoduletypecheckerror | spike | frontend/syntax | class: blocked | Implement Dynamicmoduletypecheckerror |
| 2074 | Implement Dynamicnames | spike | frontend/syntax | class: triage-needed | Implement Dynamicnames |
| 2075 | Implement Dynamicnameserrors | spike | frontend/syntax | class: blocked | Implement Dynamicnameserrors |
| 2076 | Implement Dynamicrequire | spike | frontend/syntax | class: triage-needed | Implement Dynamicrequire |
| 2077 | Implement Elaboratederrors | spike | runtime/builtins | class: triage-needed | Implement Elaboratederrors |
| 2078 | Implement Elaboratederrorsonnullabletargets | spike | frontend/resolver | class: blocked | Implement Elaboratederrorsonnullabletargets |
| 2079 | Implement Elaborationforpossiblycallabletypestillreferencesargumentattoplevel | spike | frontend/resolver | class: blocked | Implement Elaborationforpossiblycallabletypestillreferencesargumentattoplevel |
| 2080 | Implement Elidedembeddedstatementsreplacedwithsemicolon | spike | frontend/syntax | class: triage-needed | Implement Elidedembeddedstatementsreplacedwithsemicolon |
| 2081 | Implement Elidedjsimport | spike | frontend/syntax | class: blocked | Implement Elidedjsimport |
| 2082 | Implement Elidingimportnames | spike | frontend/syntax | class: blocked | Implement Elidingimportnames |
| 2083 | Implement Emitaccessexpressionofcastedobjectliteralexpressioninarrowfunctiones | spike | frontend/syntax | class: blocked | Implement Emitaccessexpressionofcastedobjectliteralexpressioninarrowfunctiones |
| 2084 | Implement Emitbundlewithprologuedirectives | spike | frontend/syntax | class: blocked | Implement Emitbundlewithprologuedirectives |
| 2085 | Implement Emitbundlewithshebang | spike | frontend/syntax | class: triage-needed | Implement Emitbundlewithshebang |
| 2086 | Implement Emitbundlewithshebangandprologuedirectives | spike | frontend/syntax | class: triage-needed | Implement Emitbundlewithshebangandprologuedirectives |
| 2087 | Implement Emitcapturingthisintupledestructuring | spike | frontend/syntax | class: blocked | Implement Emitcapturingthisintupledestructuring |
| 2088 | Implement Emitclassexpressionindeclarationfile | spike | frontend/syntax | class: blocked | Implement Emitclassexpressionindeclarationfile |
| 2089 | Implement Emitclassmergedwithconstnamespacenotelided | spike | frontend/syntax | class: blocked | Implement Emitclassmergedwithconstnamespacenotelided |
| 2090 | Implement Emitdecoratormetadata Decorator | spike | frontend/syntax | class: blocked | Implement Emitdecoratormetadata Decorator |
| 2091 | Implement Emitdecoratormetadata Import Export | spike | frontend/syntax | class: blocked | Implement Emitdecoratormetadata Import Export |
| 2092 | Implement Emithelperswithlocalcollisions | spike | frontend/syntax | class: blocked | Implement Emithelperswithlocalcollisions |
| 2093 | Implement Emitmemberaccessexpression | spike | frontend/syntax | class: blocked | Implement Emitmemberaccessexpression |
| 2094 | Implement Emitmethodcallednew | spike | frontend/syntax | class: triage-needed | Implement Emitmethodcallednew |
| 2095 | Implement Emitonelinevariabledeclarationremovecommentsfalse | spike | frontend/syntax | class: blocked | Implement Emitonelinevariabledeclarationremovecommentsfalse |
| 2096 | Implement Emitskipsthiswithrestparameter | spike | frontend/syntax | class: blocked | Implement Emitskipsthiswithrestparameter |
| 2097 | Implement Emitsupercallbeforeemitparameterpropertydeclaration | spike | frontend/syntax | class: blocked | Implement Emitsupercallbeforeemitparameterpropertydeclaration |
| 2098 | Implement Emitsupercallbeforeemitpropertydeclaration | spike | frontend/syntax | class: triage-needed | Implement Emitsupercallbeforeemitpropertydeclaration |
| 2099 | Implement Emitsupercallbeforeemitpropertydeclarationandparameterpropertydeclaration | spike | frontend/syntax | class: blocked | Implement Emitsupercallbeforeemitpropertydeclarationandparameterpropertydeclaration |
| 2100 | Implement Emitthisinobjectliteralgetter | spike | frontend/syntax | class: blocked | Implement Emitthisinobjectliteralgetter |
| 2101 | Implement Emitthisinsupermethodcall | spike | frontend/syntax | class: blocked | Implement Emitthisinsupermethodcall |
| 2102 | Implement Emittopoffiletripleslashcommentonnotemittednodeifremovecommentsisfalse | spike | frontend/syntax | class: triage-needed | Implement Emittopoffiletripleslashcommentonnotemittednodeifremovecommentsisfalse |
| 2103 | Implement Emptyanonymousobjectnarrowing | spike | frontend/syntax | class: blocked | Implement Emptyanonymousobjectnarrowing |
| 2104 | Implement Emptyargumentslistcomment | spike | frontend/resolver | class: blocked | Implement Emptyargumentslistcomment |
| 2105 | Implement Emptyarraydestructuringexpressionvisitedbytransformer | spike | frontend/resolver | class: blocked | Implement Emptyarraydestructuringexpressionvisitedbytransformer |
| 2106 | Implement Emptydeclarationemitismodule | spike | frontend/syntax | class: blocked | Implement Emptydeclarationemitismodule |
| 2107 | Implement Emptyenum | spike | frontend/syntax | class: triage-needed | Implement Emptyenum |
| 2108 | Implement Emptygenericparamlist | spike | frontend/semantics | class: blocked | Implement Emptygenericparamlist |
| 2109 | Implement Emptyindexer | spike | frontend/syntax | class: blocked | Implement Emptyindexer |
| 2110 | Implement Emptymemberaccess | spike | frontend/syntax | class: triage-needed | Implement Emptymemberaccess |
| 2111 | Implement Emptymodulename | spike | frontend/syntax | class: blocked | Implement Emptymodulename |
| 2112 | Implement Emptyobjectnotsubtypeofindexsignaturecontainingobject | spike | frontend/syntax | class: blocked | Implement Emptyobjectnotsubtypeofindexsignaturecontainingobject |
| 2113 | Implement Emptyoptionalbindingpatternindeclarationsignature | spike | frontend/syntax | class: triage-needed | Implement Emptyoptionalbindingpatternindeclarationsignature |
| 2114 | Implement Emptythenwarning | spike | frontend/syntax | class: triage-needed | Implement Emptythenwarning |
| 2115 | Implement Emptytypeargumentlist | spike | frontend/syntax | class: triage-needed | Implement Emptytypeargumentlist |
| 2116 | Implement Emptytypeargumentlistwithnew | spike | frontend/syntax | class: blocked | Implement Emptytypeargumentlistwithnew |
| 2117 | Implement Ensurenocrashexportassignmentdefineproperrtypotentialmerge | spike | frontend/syntax | class: blocked | Implement Ensurenocrashexportassignmentdefineproperrtypotentialmerge |
| 2118 | Implement Enumassignmentcompat Import Export | spike | frontend/syntax | class: blocked | Implement Enumassignmentcompat Import Export |
| 2119 | Implement Enumassignmentcompat Parser Syntax | spike | frontend/syntax | class: blocked | Implement Enumassignmentcompat Parser Syntax |
| 2120 | Implement Enumbasics Import Export | spike | frontend/syntax | class: blocked | Implement Enumbasics Import Export |
| 2121 | Implement Enumbasics Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Enumbasics Parser Syntax |
| 2122 | Implement Enumcodegennewlines | spike | frontend/syntax | class: triage-needed | Implement Enumcodegennewlines |
| 2123 | Implement Enumconflictswithglobalidentifier | spike | frontend/syntax | class: blocked | Implement Enumconflictswithglobalidentifier |
| 2124 | Implement Enumdecl | spike | frontend/syntax | class: blocked | Implement Enumdecl |
| 2125 | Implement Enumdeclarationemitinitializerhasimport | spike | frontend/syntax | class: blocked | Implement Enumdeclarationemitinitializerhasimport |
| 2126 | Implement Enumfromexternalmodule | spike | frontend/syntax | class: blocked | Implement Enumfromexternalmodule |
| 2127 | Implement Enumgenerictypeclash | spike | frontend/syntax | class: blocked | Implement Enumgenerictypeclash |
| 2128 | Implement Enumidentifierliterals | spike | frontend/syntax | class: blocked | Implement Enumidentifierliterals |
| 2129 | Implement Enumindexer | spike | frontend/syntax | class: triage-needed | Implement Enumindexer |
| 2130 | Implement Enuminitializerswithexponents | spike | frontend/syntax | class: blocked | Implement Enuminitializerswithexponents |
| 2131 | Implement Enumkeysquotedasobjectpropertiesindeclarationemit | spike | frontend/syntax | class: blocked | Implement Enumkeysquotedasobjectpropertiesindeclarationemit |
| 2132 | Implement Enumliteralassignabletoenuminsideunion | spike | frontend/syntax | class: blocked | Implement Enumliteralassignabletoenuminsideunion |
| 2133 | Implement Enumliteralunionnotwidened | spike | frontend/syntax | class: triage-needed | Implement Enumliteralunionnotwidened |
| 2134 | Implement Enumliteralssubtypereduction | spike | frontend/syntax | class: triage-needed | Implement Enumliteralssubtypereduction |
| 2135 | Implement Enummapbackintoitself | spike | frontend/syntax | class: triage-needed | Implement Enummapbackintoitself |
| 2136 | Implement Enummembernamenonidentifier | spike | frontend/syntax | class: blocked | Implement Enummembernamenonidentifier |
| 2137 | Implement Enummemberreduction | spike | frontend/syntax | class: triage-needed | Implement Enummemberreduction |
| 2138 | Implement Enummemberresolution | spike | frontend/syntax | class: triage-needed | Implement Enummemberresolution |
| 2139 | Implement Enumnegativeliteral | spike | frontend/syntax | class: triage-needed | Implement Enumnegativeliteral |
| 2140 | Implement Enumnoinitializerfollowsnonliteralinitializer | spike | frontend/syntax | class: triage-needed | Implement Enumnoinitializerfollowsnonliteralinitializer |
| 2141 | Implement Enumnumbering | spike | frontend/syntax | class: triage-needed | Implement Enumnumbering |
| 2142 | Implement Enumoperations | spike | frontend/syntax | class: triage-needed | Implement Enumoperations |
| 2143 | Implement Enumpropertyaccess | spike | frontend/syntax | class: triage-needed | Implement Enumpropertyaccess |
| 2144 | Implement Enumpropertyaccessbeforeinitalisation | spike | frontend/syntax | class: triage-needed | Implement Enumpropertyaccessbeforeinitalisation |
| 2145 | Implement Enumusedbeforedeclaration | spike | frontend/syntax | class: triage-needed | Implement Enumusedbeforedeclaration |
| 2146 | Implement Enumwithbigint | spike | runtime/builtins | class: triage-needed | Implement Enumwithbigint |
| 2147 | Implement Enumwithcomputedmember | spike | frontend/syntax | class: triage-needed | Implement Enumwithcomputedmember |
| 2148 | Implement Enumwithexport | spike | frontend/syntax | class: blocked | Implement Enumwithexport |
| 2149 | Implement Enumwithinfinityproperty | spike | frontend/syntax | class: triage-needed | Implement Enumwithinfinityproperty |
| 2150 | Implement Enumwithnanproperty | spike | frontend/syntax | class: triage-needed | Implement Enumwithnanproperty |
| 2151 | Implement Enumwithnegativeinfinityproperty | spike | frontend/syntax | class: triage-needed | Implement Enumwithnegativeinfinityproperty |
| 2152 | Implement Enumwithnonliteralstringinitializer | spike | frontend/syntax | class: triage-needed | Implement Enumwithnonliteralstringinitializer |
| 2153 | Implement Enumwithparenthesizedinitializer | spike | frontend/syntax | class: triage-needed | Implement Enumwithparenthesizedinitializer |
| 2154 | Implement Enumwithprimitivename | spike | frontend/syntax | class: triage-needed | Implement Enumwithprimitivename |
| 2155 | Implement Enumwithquotedelementname | spike | frontend/syntax | class: triage-needed | Implement Enumwithquotedelementname |
| 2156 | Implement Enumwithunicodeescape | spike | frontend/syntax | class: triage-needed | Implement Enumwithunicodeescape |
| 2157 | Implement Enumwithoutinitializeraftercomputedmember | spike | frontend/syntax | class: triage-needed | Implement Enumwithoutinitializeraftercomputedmember |
| 2158 | Implement Enumswithmultipledeclarations Import Export | spike | frontend/syntax | class: blocked | Implement Enumswithmultipledeclarations Import Export |
| 2159 | Implement Enumswithmultipledeclarations Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Enumswithmultipledeclarations Parser Syntax |
| 2160 | Implement Erasablesyntaxonly Import Export | spike | frontend/syntax | class: blocked | Implement Erasablesyntaxonly Import Export |
| 2161 | Implement Erasablesyntaxonly Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Erasablesyntaxonly Unknown Unsupported |
| 2162 | Implement Erasablesyntaxonlydeclaration | spike | frontend/syntax | class: blocked | Implement Erasablesyntaxonlydeclaration |
| 2163 | Implement Errorcause | spike | frontend/resolver | class: blocked | Implement Errorcause |
| 2164 | Implement Errorconstructorsubtypes | spike | runtime/builtins | class: triage-needed | Implement Errorconstructorsubtypes |
| 2165 | Implement Errorelaboration | spike | runtime/builtins | class: triage-needed | Implement Errorelaboration |
| 2166 | Implement Errorforbarespecifierwithimplicitmoduleresolutionnone | spike | frontend/syntax | class: blocked | Implement Errorforbarespecifierwithimplicitmoduleresolutionnone |
| 2167 | Implement Errorforconflictingexportequalsvalue | spike | frontend/syntax | class: blocked | Implement Errorforconflictingexportequalsvalue |
| 2168 | Implement Errorforusingpropertyoftypeastype | spike | frontend/syntax | class: blocked | Implement Errorforusingpropertyoftypeastype |
| 2169 | Implement Errorforwardreferenceforwadingconstructor | spike | reference/triage | class: triage-needed | Implement Errorforwardreferenceforwadingconstructor |
| 2170 | Implement Errorhandlingininstanceof | spike | frontend/resolver | class: blocked | Implement Errorhandlingininstanceof |
| 2171 | Implement Errorinunnamedclassexpression | spike | frontend/syntax | class: blocked | Implement Errorinunnamedclassexpression |
| 2172 | Implement Errorinfoforrelatedindextypesnoconstraintelaboration | spike | frontend/syntax | class: blocked | Implement Errorinfoforrelatedindextypesnoconstraintelaboration |
| 2173 | Implement Errormessageonintersectionswithdiscriminants | spike | frontend/resolver | class: blocked | Implement Errormessageonintersectionswithdiscriminants |
| 2174 | Implement Errormessageonobjectliteraltype | spike | frontend/resolver | class: blocked | Implement Errormessageonobjectliteraltype |
| 2175 | Implement Errormessagesintersectiontypes | spike | frontend/resolver | class: blocked | Implement Errormessagesintersectiontypes |
| 2176 | Implement Erroronenumreferenceincondition | spike | runtime/builtins | class: triage-needed | Implement Erroronenumreferenceincondition |
| 2177 | Implement Errorrecoveryinclassdeclaration | spike | runtime/builtins | class: triage-needed | Implement Errorrecoveryinclassdeclaration |
| 2178 | Implement Errorrecoverywithdotfollowedbynamespacekeyword | spike | frontend/syntax | class: blocked | Implement Errorrecoverywithdotfollowedbynamespacekeyword |
| 2179 | Implement Errorsupression | spike | frontend/resolver | class: blocked | Implement Errorsupression |
| 2180 | Implement Errorwithsamenametype | spike | runtime/builtins | class: triage-needed | Implement Errorwithsamenametype |
| 2181 | Implement Errorwithtruncatedtype | spike | frontend/resolver | class: blocked | Implement Errorwithtruncatedtype |
| 2182 | Implement Errorsforcallandassignmentaresimilar | spike | runtime/builtins | class: triage-needed | Implement Errorsforcallandassignmentaresimilar |
| 2183 | Implement Errorsingenerictypereference | spike | frontend/semantics | class: blocked | Implement Errorsingenerictypereference |
| 2184 | Implement Errorsonimportedsymbol | spike | frontend/syntax | class: blocked | Implement Errorsonimportedsymbol |
| 2185 | Implement Errorsonunionsofoverlappingobjects | spike | frontend/syntax | class: blocked | Implement Errorsonunionsofoverlappingobjects |
| 2186 | Implement Errorswithinvokablesinunions | spike | frontend/syntax | class: blocked | Implement Errorswithinvokablesinunions |
| 2187 | Implement Es Destructuring | spike | frontend/syntax | class: blocked | Implement Es Destructuring |
| 2188 | Implement Es Duplicate Local | spike | reference/triage | class: triage-needed | Implement Es Duplicate Local |
| 2189 | Implement Es Import Export | spike | frontend/syntax | class: blocked | Implement Es Import Export |
| 2190 | Implement Es Module System Amd | spike | frontend/syntax | class: blocked | Implement Es Module System Amd |
| 2191 | Implement Es Object Literal | spike | frontend/syntax | class: blocked | Implement Es Object Literal |
| 2192 | Implement Es Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Es Parser Syntax |
| 2193 | Implement Es Runtime Subset | spike | reference/triage | class: triage-needed | Implement Es Runtime Subset |
| 2194 | Implement Es Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Es Unknown Unsupported |
| 2195 | Implement Esdecoratorsclassfieldscrash | spike | frontend/syntax | class: blocked | Implement Esdecoratorsclassfieldscrash |
| 2196 | Implement Esmoduleinterop | spike | frontend/syntax | class: blocked | Implement Esmoduleinterop |
| 2197 | Implement Esmoduleinteropdefaultimports | spike | frontend/syntax | class: blocked | Implement Esmoduleinteropdefaultimports |
| 2198 | Implement Esmoduleinteropdefaultmembermustbesyntacticallydefaultexport | spike | frontend/syntax | class: blocked | Implement Esmoduleinteropdefaultmembermustbesyntacticallydefaultexport |
| 2199 | Implement Esmoduleinteropenablessyntheticdefaultimports | spike | frontend/syntax | class: blocked | Implement Esmoduleinteropenablessyntheticdefaultimports |
| 2200 | Implement Esmoduleinteropimportcall | spike | frontend/syntax | class: blocked | Implement Esmoduleinteropimportcall |
| 2201 | Implement Esmoduleinteropimportdefaultwhenallnamedaredefaultalias | spike | frontend/syntax | class: blocked | Implement Esmoduleinteropimportdefaultwhenallnamedaredefaultalias |
| 2202 | Implement Esmoduleinteropimportnamespace | spike | frontend/syntax | class: blocked | Implement Esmoduleinteropimportnamespace |
| 2203 | Implement Esmoduleinteropimporttslibhasimport | spike | frontend/syntax | class: blocked | Implement Esmoduleinteropimporttslibhasimport |
| 2204 | Implement Esmoduleinteropnameddefaultimports | spike | frontend/syntax | class: blocked | Implement Esmoduleinteropnameddefaultimports |
| 2205 | Implement Esmoduleinteropprettyerrorrelatedinformation | spike | frontend/syntax | class: blocked | Implement Esmoduleinteropprettyerrorrelatedinformation |
| 2206 | Implement Esmoduleinteroptslibhelpers | spike | frontend/syntax | class: blocked | Implement Esmoduleinteroptslibhelpers |
| 2207 | Implement Esmoduleinteropusesexportstarwhendefaultplusnames | spike | frontend/syntax | class: blocked | Implement Esmoduleinteropusesexportstarwhendefaultplusnames |
| 2208 | Implement Esmoduleinteropwithexportstar | spike | frontend/syntax | class: blocked | Implement Esmoduleinteropwithexportstar |
| 2209 | Implement Esmoduleintersectioncrash | spike | frontend/syntax | class: blocked | Implement Esmoduleintersectioncrash |
| 2210 | Implement Esnextweakrefs | spike | frontend/syntax | class: blocked | Implement Esnextweakrefs |
| 2211 | Implement Escapedidentifiers | spike | frontend/syntax | class: blocked | Implement Escapedidentifiers |
| 2212 | Implement Esmmodedeclarationfilewithexportassignment | spike | frontend/syntax | class: blocked | Implement Esmmodedeclarationfilewithexportassignment |
| 2213 | Implement Esmnosynthesizeddefault | spike | frontend/syntax | class: blocked | Implement Esmnosynthesizeddefault |
| 2214 | Implement Evalafter | spike | runtime/builtins | class: triage-needed | Implement Evalafter |
| 2215 | Implement Evalorargumentsindeclarationfunctions | spike | runtime/builtins | class: triage-needed | Implement Evalorargumentsindeclarationfunctions |
| 2216 | Implement Evolvingarraytypeinassert | spike | frontend/syntax | class: blocked | Implement Evolvingarraytypeinassert |
| 2217 | Implement Exactoptionalpropertytypesidentical | spike | frontend/syntax | class: blocked | Implement Exactoptionalpropertytypesidentical |
| 2218 | Implement Exactspellingsuggestion | spike | frontend/syntax | class: triage-needed | Implement Exactspellingsuggestion |
| 2219 | Implement Excesspropertiesinoverloads | spike | frontend/resolver | class: blocked | Implement Excesspropertiesinoverloads |
| 2220 | Implement Excesspropertycheckintersectionwithindexsignature | spike | frontend/syntax | class: triage-needed | Implement Excesspropertycheckintersectionwithindexsignature |
| 2221 | Implement Excesspropertycheckintersectionwithrecursivetype | spike | frontend/syntax | class: blocked | Implement Excesspropertycheckintersectionwithrecursivetype |
| 2222 | Implement Excesspropertycheckwithemptyobject | spike | frontend/resolver | class: blocked | Implement Excesspropertycheckwithemptyobject |
| 2223 | Implement Excesspropertycheckwithmultiplediscriminants | spike | frontend/syntax | class: blocked | Implement Excesspropertycheckwithmultiplediscriminants |
| 2224 | Implement Excesspropertycheckwithnestedarrayintersection | spike | frontend/syntax | class: blocked | Implement Excesspropertycheckwithnestedarrayintersection |
| 2225 | Implement Excesspropertycheckwithspread | spike | frontend/resolver | class: blocked | Implement Excesspropertycheckwithspread |
| 2226 | Implement Excesspropertycheckwithunions | spike | frontend/syntax | class: blocked | Implement Excesspropertycheckwithunions |
| 2227 | Implement Excesspropertycheckingintersectionwithconditional | spike | frontend/semantics | class: blocked | Implement Excesspropertycheckingintersectionwithconditional |
| 2228 | Implement Excesspropertycheckswithnestedintersections | spike | frontend/syntax | class: blocked | Implement Excesspropertycheckswithnestedintersections |
| 2229 | Implement Excesspropertyerrorforfunctiontypes | spike | frontend/syntax | class: blocked | Implement Excesspropertyerrorforfunctiontypes |
| 2230 | Implement Excessivestackdepthflatarray | spike | reference/triage | class: triage-needed | Implement Excessivestackdepthflatarray |
| 2231 | Implement Excessivelylargetuplespread | spike | frontend/syntax | class: blocked | Implement Excessivelylargetuplespread |
| 2232 | Implement Exhaustiveswitchcheckcircularity | spike | frontend/syntax | class: blocked | Implement Exhaustiveswitchcheckcircularity |
| 2233 | Implement Exhaustiveswitchwithwideningliteraltypes | spike | frontend/syntax | class: blocked | Implement Exhaustiveswitchwithwideningliteraltypes |
| 2234 | Implement Expandofunctionblockshadowing | spike | frontend/syntax | class: blocked | Implement Expandofunctionblockshadowing |
| 2235 | Implement Expandofunctioncontextualtypesjsdocints | spike | frontend/syntax | class: blocked | Implement Expandofunctioncontextualtypesjsdocints |
| 2236 | Implement Expandofunctioncontextualtypesjs | spike | frontend/syntax | class: blocked | Implement Expandofunctioncontextualtypesjs |
| 2237 | Implement Expandofunctioncontextualtypesnovalue | spike | frontend/syntax | class: blocked | Implement Expandofunctioncontextualtypesnovalue |
| 2238 | Implement Expandofunctionexpressionswithdynamicnames | spike | frontend/syntax | class: triage-needed | Implement Expandofunctionexpressionswithdynamicnames |
| 2239 | Implement Expandofunctionnestedassigments | spike | frontend/syntax | class: triage-needed | Implement Expandofunctionnestedassigments |
| 2240 | Implement Expandofunctionnestedassigmentsdeclared | spike | frontend/syntax | class: blocked | Implement Expandofunctionnestedassigmentsdeclared |
| 2241 | Implement Expandofunctionnullishproperty | spike | frontend/syntax | class: blocked | Implement Expandofunctionnullishproperty |
| 2242 | Implement Expandofunctionsymbolproperty | spike | frontend/syntax | class: blocked | Implement Expandofunctionsymbolproperty |
| 2243 | Implement Expandofunctionsymbolpropertyjs | spike | frontend/syntax | class: blocked | Implement Expandofunctionsymbolpropertyjs |
| 2244 | Implement Experimentaldecoratormetadataunresolvedtypeobjectinemit | spike | frontend/syntax | class: blocked | Implement Experimentaldecoratormetadataunresolvedtypeobjectinemit |
| 2245 | Implement Exportalreadyseen | spike | frontend/syntax | class: blocked | Implement Exportalreadyseen |
| 2246 | Implement Exportarraybindingpattern | spike | frontend/syntax | class: blocked | Implement Exportarraybindingpattern |
| 2247 | Implement Exportasnamespace | spike | frontend/syntax | class: blocked | Implement Exportasnamespace |
| 2248 | Implement Exportasnamespaceconflict | spike | frontend/syntax | class: blocked | Implement Exportasnamespaceconflict |
| 2249 | Implement Exportassignclassandmodule | spike | frontend/syntax | class: blocked | Implement Exportassignclassandmodule |
| 2250 | Implement Exportassignvalueandtype | spike | frontend/syntax | class: blocked | Implement Exportassignvalueandtype |
| 2251 | Implement Exportassignednamespaceisvisibleindeclarationemit | spike | frontend/syntax | class: blocked | Implement Exportassignednamespaceisvisibleindeclarationemit |
| 2252 | Implement Exportassignedtypeastypeannotation | spike | frontend/syntax | class: blocked | Implement Exportassignedtypeastypeannotation |
| 2253 | Implement Exportassignmentclass | spike | frontend/syntax | class: blocked | Implement Exportassignmentclass |
| 2254 | Implement Exportassignmentenum | spike | frontend/syntax | class: blocked | Implement Exportassignmentenum |
| 2255 | Implement Exportassignmenterror | spike | frontend/syntax | class: blocked | Implement Exportassignmenterror |
| 2256 | Implement Exportassignmentexpressionisexpressionnode | spike | frontend/syntax | class: blocked | Implement Exportassignmentexpressionisexpressionnode |
| 2257 | Implement Exportassignmentfunction | spike | frontend/syntax | class: blocked | Implement Exportassignmentfunction |
| 2258 | Implement Exportassignmentimportmergenocrash | spike | frontend/syntax | class: blocked | Implement Exportassignmentimportmergenocrash |
| 2259 | Implement Exportassignmentinterface | spike | frontend/syntax | class: blocked | Implement Exportassignmentinterface |
| 2260 | Implement Exportassignmentinternalmodule | spike | frontend/syntax | class: blocked | Implement Exportassignmentinternalmodule |
| 2261 | Implement Exportassignmentmembersvisibleinaugmentation | spike | frontend/syntax | class: blocked | Implement Exportassignmentmembersvisibleinaugmentation |
| 2262 | Implement Exportassignmentofdeclaredexternalmodule | spike | frontend/syntax | class: blocked | Implement Exportassignmentofdeclaredexternalmodule |
| 2263 | Implement Exportassignmentofgenerictype | spike | frontend/syntax | class: blocked | Implement Exportassignmentofgenerictype |
| 2264 | Implement Exportassignmentvariable | spike | frontend/syntax | class: blocked | Implement Exportassignmentvariable |
| 2265 | Implement Exportassignmentwithdeclareandexportmodifiers | spike | frontend/syntax | class: blocked | Implement Exportassignmentwithdeclareandexportmodifiers |
| 2266 | Implement Exportassignmentwithdeclaremodifier | spike | frontend/syntax | class: blocked | Implement Exportassignmentwithdeclaremodifier |
| 2267 | Implement Exportassignmentwithexportmodifier | spike | frontend/syntax | class: blocked | Implement Exportassignmentwithexportmodifier |
| 2268 | Implement Exportassignmentwithexports | spike | frontend/syntax | class: blocked | Implement Exportassignmentwithexports |
| 2269 | Implement Exportassignmentwithimportstatementprivacyerror | spike | frontend/syntax | class: blocked | Implement Exportassignmentwithimportstatementprivacyerror |
| 2270 | Implement Exportassignmentwithprivacyerror | spike | frontend/syntax | class: blocked | Implement Exportassignmentwithprivacyerror |
| 2271 | Implement Exportassignmentwithoutallowsyntheticdefaultimportserror | spike | frontend/syntax | class: blocked | Implement Exportassignmentwithoutallowsyntheticdefaultimportserror |
| 2272 | Implement Exportassignmentwithoutidentifier | spike | frontend/syntax | class: blocked | Implement Exportassignmentwithoutidentifier |
| 2273 | Implement Exportclassextendingintersection | spike | frontend/syntax | class: blocked | Implement Exportclassextendingintersection |
| 2274 | Implement Exportclasswithoutname | spike | frontend/syntax | class: blocked | Implement Exportclasswithoutname |
| 2275 | Implement Exportdeclarationformoduleorenumwithmemberofsamename | spike | frontend/syntax | class: blocked | Implement Exportdeclarationformoduleorenumwithmemberofsamename |
| 2276 | Implement Exportdeclarationininternalmodule | spike | frontend/syntax | class: blocked | Implement Exportdeclarationininternalmodule |
| 2277 | Implement Exportdeclarationwithmodulespecifiernameonnextline | spike | frontend/syntax | class: blocked | Implement Exportdeclarationwithmodulespecifiernameonnextline |
| 2278 | Implement Exportdeclarationsinambientnamespaces | spike | frontend/syntax | class: blocked | Implement Exportdeclarationsinambientnamespaces |
| 2279 | Implement Exportdefaultabstractclass | spike | frontend/syntax | class: blocked | Implement Exportdefaultabstractclass |
| 2280 | Implement Exportdefaultalias | spike | frontend/syntax | class: blocked | Implement Exportdefaultalias |
| 2281 | Implement Exportdefaultasyncfunction | spike | frontend/syntax | class: blocked | Implement Exportdefaultasyncfunction |
| 2282 | Implement Exportdefaultclassandvalue | spike | frontend/syntax | class: blocked | Implement Exportdefaultclassandvalue |
| 2283 | Implement Exportdefaultclassinnamespace | spike | frontend/syntax | class: blocked | Implement Exportdefaultclassinnamespace |
| 2284 | Implement Exportdefaultduplicatecrash | spike | frontend/syntax | class: blocked | Implement Exportdefaultduplicatecrash |
| 2285 | Implement Exportdefaultfornoninstantiatedmodule | spike | frontend/syntax | class: blocked | Implement Exportdefaultfornoninstantiatedmodule |
| 2286 | Implement Exportdefaultfunctioninnamespace | spike | frontend/syntax | class: blocked | Implement Exportdefaultfunctioninnamespace |
| 2287 | Implement Exportdefaultimportedtype | spike | frontend/syntax | class: blocked | Implement Exportdefaultimportedtype |
| 2288 | Implement Exportdefaultinterface | spike | frontend/syntax | class: blocked | Implement Exportdefaultinterface |
| 2289 | Implement Exportdefaultinterfaceandfunctionoverloads | spike | frontend/syntax | class: blocked | Implement Exportdefaultinterfaceandfunctionoverloads |
| 2290 | Implement Exportdefaultinterfaceandtwofunctions | spike | frontend/syntax | class: blocked | Implement Exportdefaultinterfaceandtwofunctions |
| 2291 | Implement Exportdefaultinterfaceandvalue | spike | frontend/syntax | class: blocked | Implement Exportdefaultinterfaceandvalue |
| 2292 | Implement Exportdefaultinterfaceclassandfunctionoverloads | spike | frontend/syntax | class: blocked | Implement Exportdefaultinterfaceclassandfunctionoverloads |
| 2293 | Implement Exportdefaultinterfaceclassandvalue | spike | frontend/syntax | class: blocked | Implement Exportdefaultinterfaceclassandvalue |
| 2294 | Implement Exportdefaultmarksidentifierasused | spike | frontend/syntax | class: blocked | Implement Exportdefaultmarksidentifierasused |
| 2295 | Implement Exportdefaultmissingname | spike | frontend/syntax | class: blocked | Implement Exportdefaultmissingname |
| 2296 | Implement Exportdefaultparenthesize | spike | frontend/syntax | class: blocked | Implement Exportdefaultparenthesize |
| 2297 | Implement Exportdefaultparenthesizees | spike | frontend/syntax | class: blocked | Implement Exportdefaultparenthesizees |
| 2298 | Implement Exportdefaultproperty | spike | frontend/syntax | class: blocked | Implement Exportdefaultproperty |
| 2299 | Implement Exportdefaultqualifiednamenoerror | spike | frontend/syntax | class: blocked | Implement Exportdefaultqualifiednamenoerror |
| 2300 | Implement Exportdefaultstripsfreshness | spike | frontend/syntax | class: blocked | Implement Exportdefaultstripsfreshness |
| 2301 | Implement Exportdefaulttypeandclass | spike | frontend/syntax | class: blocked | Implement Exportdefaulttypeandclass |
| 2302 | Implement Exportdefaulttypeandfunctionoverloads | spike | frontend/syntax | class: blocked | Implement Exportdefaulttypeandfunctionoverloads |
| 2303 | Implement Exportdefaulttypeclassandvalue | spike | frontend/syntax | class: blocked | Implement Exportdefaulttypeclassandvalue |
| 2304 | Implement Exportdefaultvariable | spike | frontend/syntax | class: blocked | Implement Exportdefaultvariable |
| 2305 | Implement Exportdefaultwithjsdoc | spike | frontend/syntax | class: blocked | Implement Exportdefaultwithjsdoc |
| 2306 | Implement Exportemptyarraybindingpattern | spike | frontend/syntax | class: blocked | Implement Exportemptyarraybindingpattern |
| 2307 | Implement Exportemptyobjectbindingpattern | spike | frontend/syntax | class: blocked | Implement Exportemptyobjectbindingpattern |
| 2308 | Implement Exportequalcallable | spike | frontend/syntax | class: blocked | Implement Exportequalcallable |
| 2309 | Implement Exportequalerrortype | spike | frontend/syntax | class: blocked | Implement Exportequalerrortype |
| 2310 | Implement Exportequalmembermissing | spike | frontend/syntax | class: blocked | Implement Exportequalmembermissing |
| 2311 | Implement Exportequalnamespaces | spike | frontend/syntax | class: blocked | Implement Exportequalnamespaces |
| 2312 | Implement Exportequalsamd | spike | frontend/syntax | class: blocked | Implement Exportequalsamd |
| 2313 | Implement Exportequalsclassnoredeclarationerror | spike | frontend/syntax | class: blocked | Implement Exportequalsclassnoredeclarationerror |
| 2314 | Implement Exportequalsclassredeclarationerror | spike | frontend/syntax | class: blocked | Implement Exportequalsclassredeclarationerror |
| 2315 | Implement Exportequalscommonjs | spike | frontend/syntax | class: blocked | Implement Exportequalscommonjs |
| 2316 | Implement Exportequalsdefaultproperty | spike | frontend/syntax | class: blocked | Implement Exportequalsdefaultproperty |
| 2317 | Implement Exportequalsofmodule | spike | frontend/syntax | class: blocked | Implement Exportequalsofmodule |
| 2318 | Implement Exportequalsproperty | spike | frontend/syntax | class: blocked | Implement Exportequalsproperty |
| 2319 | Implement Exportequalsumd | spike | frontend/syntax | class: blocked | Implement Exportequalsumd |
| 2320 | Implement Exportimport | spike | frontend/syntax | class: blocked | Implement Exportimport |
| 2321 | Implement Exportimportandclodule | spike | frontend/syntax | class: blocked | Implement Exportimportandclodule |
| 2322 | Implement Exportimportcansubstituteconstenumforvalue | spike | frontend/syntax | class: blocked | Implement Exportimportcansubstituteconstenumforvalue |
| 2323 | Implement Exportimportmultiplefiles | spike | frontend/syntax | class: blocked | Implement Exportimportmultiplefiles |
| 2324 | Implement Exportimportnoninstantiatedmodule | spike | frontend/syntax | class: blocked | Implement Exportimportnoninstantiatedmodule |
| 2325 | Implement Exportinfunction | spike | frontend/syntax | class: blocked | Implement Exportinfunction |
| 2326 | Implement Exportinterfaceclassandvalue | spike | frontend/syntax | class: blocked | Implement Exportinterfaceclassandvalue |
| 2327 | Implement Exportinterfaceclassandvaluewithduplicatesinimportlist | spike | frontend/syntax | class: blocked | Implement Exportinterfaceclassandvaluewithduplicatesinimportlist |
| 2328 | Implement Exportnamespacedeclarationretainsvisibility | spike | frontend/syntax | class: blocked | Implement Exportnamespacedeclarationretainsvisibility |
| 2329 | Implement Exportobjectrest | spike | frontend/syntax | class: blocked | Implement Exportobjectrest |
| 2330 | Implement Exportprivatetype | spike | frontend/syntax | class: blocked | Implement Exportprivatetype |
| 2331 | Implement Exportredeclarationtypealiases | spike | frontend/syntax | class: blocked | Implement Exportredeclarationtypealiases |
| 2332 | Implement Exportsamenamefuncvar | spike | frontend/syntax | class: blocked | Implement Exportsamenamefuncvar |
| 2333 | Implement Exportspecifierandexportedmemberdeclaration | spike | frontend/syntax | class: blocked | Implement Exportspecifierandexportedmemberdeclaration |
| 2334 | Implement Exportspecifierandlocalmemberdeclaration | spike | frontend/syntax | class: blocked | Implement Exportspecifierandlocalmemberdeclaration |
| 2335 | Implement Exportspecifierforaglobal | spike | frontend/syntax | class: blocked | Implement Exportspecifierforaglobal |
| 2336 | Implement Exportspecifierreferencingouterdeclaration | spike | frontend/syntax | class: blocked | Implement Exportspecifierreferencingouterdeclaration |
| 2337 | Implement Exportstarforvalues | spike | frontend/syntax | class: blocked | Implement Exportstarforvalues |
| 2338 | Implement Exportstarforvaluesinsystem | spike | frontend/syntax | class: blocked | Implement Exportstarforvaluesinsystem |
| 2339 | Implement Exportstarfromemptymodule | spike | frontend/syntax | class: blocked | Implement Exportstarfromemptymodule |
| 2340 | Implement Exportstarnotelided | spike | frontend/syntax | class: blocked | Implement Exportstarnotelided |
| 2341 | Implement Exporttostring | spike | frontend/syntax | class: blocked | Implement Exporttostring |
| 2342 | Implement Exportvisibility | spike | frontend/syntax | class: blocked | Implement Exportvisibility |
| 2343 | Implement Exportedblockscopeddeclarations | spike | frontend/syntax | class: blocked | Implement Exportedblockscopeddeclarations |
| 2344 | Implement Exportedinterfaceinaccessibleincallbackinmodule | spike | frontend/syntax | class: blocked | Implement Exportedinterfaceinaccessibleincallbackinmodule |
| 2345 | Implement Exportedvariable | spike | frontend/syntax | class: blocked | Implement Exportedvariable |
| 2346 | Implement Exportingcontainingvisibletype | spike | frontend/syntax | class: blocked | Implement Exportingcontainingvisibletype |
| 2347 | Implement Exportsinambientmodules | spike | frontend/syntax | class: blocked | Implement Exportsinambientmodules |
| 2348 | Implement Expr | spike | frontend/syntax | class: triage-needed | Implement Expr |
| 2349 | Implement Expressiontypenodeshoulderror | spike | frontend/syntax | class: triage-needed | Implement Expressiontypenodeshoulderror |
| 2350 | Implement Expressionwithjsdoctypearguments | spike | frontend/syntax | class: blocked | Implement Expressionwithjsdoctypearguments |
| 2351 | Implement Expressionsforbiddeninparameterinitializers | spike | frontend/syntax | class: blocked | Implement Expressionsforbiddeninparameterinitializers |
| 2352 | Implement Extbaseclass | spike | frontend/syntax | class: blocked | Implement Extbaseclass |
| 2353 | Implement Extendandimplementthesamebasetype | spike | frontend/syntax | class: triage-needed | Implement Extendandimplementthesamebasetype |
| 2354 | Implement Extendarray | spike | frontend/syntax | class: blocked | Implement Extendarray |
| 2355 | Implement Extendconstructsignatureininterface | spike | frontend/syntax | class: blocked | Implement Extendconstructsignatureininterface |
| 2356 | Implement Extendfromany | spike | frontend/syntax | class: triage-needed | Implement Extendfromany |
| 2357 | Implement Extendgenericarray | spike | frontend/semantics | class: blocked | Implement Extendgenericarray |
| 2358 | Implement Extendglobalthis Import Export | spike | frontend/syntax | class: blocked | Implement Extendglobalthis Import Export |
| 2359 | Implement Extendglobalthis Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Extendglobalthis Parser Syntax |
| 2360 | Implement Extendnonclasssymbol Class | spike | frontend/syntax | class: blocked | Implement Extendnonclasssymbol Class |
| 2361 | Implement Extendnonclasssymbol Name Resolution | spike | frontend/resolver | class: blocked | Implement Extendnonclasssymbol Name Resolution |
| 2362 | Implement Extendprivateconstructorclass | spike | frontend/syntax | class: blocked | Implement Extendprivateconstructorclass |
| 2363 | Implement Extendedinterfacegenerictype | spike | frontend/semantics | class: blocked | Implement Extendedinterfacegenerictype |
| 2364 | Implement Extendedunicodeplaneidentifiers | spike | frontend/syntax | class: triage-needed | Implement Extendedunicodeplaneidentifiers |
| 2365 | Implement Extendedunicodeplaneidentifiersjsdoc | spike | frontend/syntax | class: blocked | Implement Extendedunicodeplaneidentifiersjsdoc |
| 2366 | Implement Extendingclassfromaliasandusageinindexer | spike | frontend/syntax | class: blocked | Implement Extendingclassfromaliasandusageinindexer |
| 2367 | Implement Extendingcollectionswithcheckjs | spike | frontend/syntax | class: blocked | Implement Extendingcollectionswithcheckjs |
| 2368 | Implement Extendsclausealreadyseen | spike | frontend/syntax | class: triage-needed | Implement Extendsclausealreadyseen |
| 2369 | Implement Extendsuntypedmodule | spike | frontend/syntax | class: blocked | Implement Extendsuntypedmodule |
| 2370 | Implement Extension | spike | frontend/syntax | class: blocked | Implement Extension |
| 2371 | Implement Externmodule | spike | frontend/syntax | class: blocked | Implement Externmodule |
| 2372 | Implement Externmoduleclobber | spike | frontend/syntax | class: blocked | Implement Externmoduleclobber |
| 2373 | Implement Externsemantics | spike | frontend/syntax | class: triage-needed | Implement Externsemantics |
| 2374 | Implement Externsyntax | spike | frontend/syntax | class: blocked | Implement Externsyntax |
| 2375 | Implement Externalmoduleassigntovar | spike | frontend/syntax | class: blocked | Implement Externalmoduleassigntovar |
| 2376 | Implement Externalmoduleexportinggenericclass | spike | frontend/syntax | class: blocked | Implement Externalmoduleexportinggenericclass |
| 2377 | Implement Externalmoduleimmutablebindings | spike | frontend/syntax | class: blocked | Implement Externalmoduleimmutablebindings |
| 2378 | Implement Externalmodulequalification | spike | frontend/syntax | class: blocked | Implement Externalmodulequalification |
| 2379 | Implement Externalmodulereferencedoubleunderscore | spike | frontend/syntax | class: blocked | Implement Externalmodulereferencedoubleunderscore |
| 2380 | Implement Externalmodulereferenceofimportdeclarationwithexportmodifier | spike | frontend/syntax | class: blocked | Implement Externalmodulereferenceofimportdeclarationwithexportmodifier |
| 2381 | Implement Externalmodulerefernceresolutionorderinimportdeclaration | spike | frontend/syntax | class: blocked | Implement Externalmodulerefernceresolutionorderinimportdeclaration |
| 2382 | Implement Externalmoduleresolution | spike | frontend/syntax | class: blocked | Implement Externalmoduleresolution |
| 2383 | Implement Externalmodulewithoutcompilerflag | spike | frontend/syntax | class: blocked | Implement Externalmodulewithoutcompilerflag |
| 2384 | Implement Extractinferenceimprovement | spike | frontend/semantics | class: blocked | Implement Extractinferenceimprovement |
| 2385 | Implement Fakeinfinity | spike | frontend/syntax | class: triage-needed | Implement Fakeinfinity |
| 2386 | Implement Fallfromlastcase | spike | frontend/resolver | class: blocked | Implement Fallfromlastcase |
| 2387 | Implement Fallbacktobindingpatternfortypeinference | spike | frontend/resolver | class: blocked | Implement Fallbacktobindingpatternfortypeinference |
| 2388 | Implement Fatarrowself | spike | frontend/syntax | class: blocked | Implement Fatarrowself |
| 2389 | Implement Fatarrowfunctionastype | spike | frontend/syntax | class: triage-needed | Implement Fatarrowfunctionastype |
| 2390 | Implement Fatarrowfunctions | spike | frontend/syntax | class: blocked | Implement Fatarrowfunctions |
| 2391 | Implement Fatarrowfunctionserrors | spike | runtime/builtins | class: triage-needed | Implement Fatarrowfunctionserrors |
| 2392 | Implement Fatarrowfunctionsinfunctionparameterdefaults | spike | frontend/syntax | class: blocked | Implement Fatarrowfunctionsinfunctionparameterdefaults |
| 2393 | Implement Fatarrowfunctionsinfunctions | spike | frontend/resolver | class: blocked | Implement Fatarrowfunctionsinfunctions |
| 2394 | Implement Fatarrowfunctionsoptionalargs | spike | frontend/syntax | class: blocked | Implement Fatarrowfunctionsoptionalargs |
| 2395 | Implement Fatarrowfunctionsoptionalargserrors | spike | frontend/syntax | class: blocked | Implement Fatarrowfunctionsoptionalargserrors |
| 2396 | Implement Fieldandgetterwithsamename | spike | frontend/syntax | class: blocked | Implement Fieldandgetterwithsamename |
| 2397 | Implement Filereferenceswithnoextensions | spike | frontend/resolver | class: blocked | Implement Filereferenceswithnoextensions |
| 2398 | Implement Filewithnextline | spike | reference/triage | class: triage-needed | Implement Filewithnextline |
| 2399 | Implement Filesemittingintosameoutputwithoutoption | spike | frontend/syntax | class: blocked | Implement Filesemittingintosameoutputwithoutoption |
| 2400 | Implement Fillinmissingtypeargsonconstructcalls | spike | frontend/syntax | class: blocked | Implement Fillinmissingtypeargsonconstructcalls |
| 2401 | Implement Fillinmissingtypeargsonjsconstructcalls | spike | frontend/syntax | class: blocked | Implement Fillinmissingtypeargsonjsconstructcalls |
| 2402 | Implement Findlast | spike | frontend/resolver | class: blocked | Implement Findlast |
| 2403 | Implement Firstmatchregexpmatcharray | spike | runtime/builtins | class: triage-needed | Implement Firstmatchregexpmatcharray |
| 2404 | Implement Fixcrashaliaslookupfordefauledimport | spike | frontend/syntax | class: blocked | Implement Fixcrashaliaslookupfordefauledimport |
| 2405 | Implement Fixingtypeparametersrepeatedly Duplicate Local | spike | reference/triage | class: triage-needed | Implement Fixingtypeparametersrepeatedly Duplicate Local |
| 2406 | Implement Fixingtypeparametersrepeatedly Name Resolution | spike | frontend/resolver | class: blocked | Implement Fixingtypeparametersrepeatedly Name Resolution |
| 2407 | Implement Flatarraynoexcessivestackdepth | spike | frontend/syntax | class: blocked | Implement Flatarraynoexcessivestackdepth |
| 2408 | Implement Flowafterfinally | spike | frontend/syntax | class: blocked | Implement Flowafterfinally |
| 2409 | Implement Flowcontroltypeguardthenswitch | spike | frontend/syntax | class: blocked | Implement Flowcontroltypeguardthenswitch |
| 2410 | Implement For | spike | frontend/syntax | class: triage-needed | Implement For |
| 2411 | Implement Forawaitforintersection | spike | reference/triage | class: triage-needed | Implement Forawaitforintersection |
| 2412 | Implement Forawaitforunion | spike | reference/triage | class: triage-needed | Implement Forawaitforunion |
| 2413 | Implement Forin | spike | frontend/syntax | class: triage-needed | Implement Forin |
| 2414 | Implement Forinmodule | spike | frontend/syntax | class: blocked | Implement Forinmodule |
| 2415 | Implement Forinstatement Duplicate Local | spike | reference/triage | class: triage-needed | Implement Forinstatement Duplicate Local |
| 2416 | Implement Forinstatement Name Resolution | spike | frontend/resolver | class: blocked | Implement Forinstatement Name Resolution |
| 2417 | Implement Forinstatement Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Forinstatement Parser Syntax |
| 2418 | Implement Forloopendingmultilinecomments | spike | frontend/syntax | class: blocked | Implement Forloopendingmultilinecomments |
| 2419 | Implement Forloopwithdestructuringdoesnotelidefollowingstatement | spike | frontend/syntax | class: blocked | Implement Forloopwithdestructuringdoesnotelidefollowingstatement |
| 2420 | Implement Forofstringconstituents | spike | frontend/syntax | class: triage-needed | Implement Forofstringconstituents |
| 2421 | Implement Foroftransformsexpression | spike | reference/triage | class: triage-needed | Implement Foroftransformsexpression |
| 2422 | Implement Forstatementinnercomments | spike | frontend/syntax | class: triage-needed | Implement Forstatementinnercomments |
| 2423 | Implement Formattopartsfractionalsecond | spike | frontend/resolver | class: blocked | Implement Formattopartsfractionalsecond |
| 2424 | Implement Forwarddeclaredcommontypes | spike | frontend/resolver | class: blocked | Implement Forwarddeclaredcommontypes |
| 2425 | Implement Forwardrefinclassproperties | spike | frontend/syntax | class: triage-needed | Implement Forwardrefinclassproperties |
| 2426 | Implement Forwardrefinenum | spike | frontend/syntax | class: triage-needed | Implement Forwardrefinenum |
| 2427 | Implement Forwardrefintypedeclaration | spike | frontend/syntax | class: triage-needed | Implement Forwardrefintypedeclaration |
| 2428 | Implement Freshliteralinference | spike | frontend/resolver | class: blocked | Implement Freshliteralinference |
| 2429 | Implement Freshliteraltypesinintersections | spike | frontend/resolver | class: blocked | Implement Freshliteraltypesinintersections |
| 2430 | Implement Funclodule | spike | frontend/syntax | class: blocked | Implement Funclodule |
| 2431 | Implement Funcdecl | spike | frontend/syntax | class: blocked | Implement Funcdecl |
| 2432 | Implement Functionandimportnameconflict | spike | frontend/syntax | class: blocked | Implement Functionandimportnameconflict |
| 2433 | Implement Functionandinterfacewithseparateerrors | spike | runtime/builtins | class: triage-needed | Implement Functionandinterfacewithseparateerrors |
| 2434 | Implement Functionandpropertynameconflict | spike | frontend/syntax | class: blocked | Implement Functionandpropertynameconflict |
| 2435 | Implement Functionargshadowing | spike | reference/triage | class: triage-needed | Implement Functionargshadowing |
| 2436 | Implement Functionassignabilitywitharraylike | spike | frontend/resolver | class: blocked | Implement Functionassignabilitywitharraylike |
| 2437 | Implement Functionassignment | spike | frontend/syntax | class: blocked | Implement Functionassignment |
| 2438 | Implement Functioncall Arity | spike | reference/triage | class: triage-needed | Implement Functioncall Arity |
| 2439 | Implement Functioncall Duplicate Local | spike | reference/triage | class: triage-needed | Implement Functioncall Duplicate Local |
| 2440 | Implement Functioncall Import Export | spike | frontend/syntax | class: blocked | Implement Functioncall Import Export |
| 2441 | Implement Functioncall Name Resolution | spike | frontend/resolver | class: blocked | Implement Functioncall Name Resolution |
| 2442 | Implement Functioncallonconstrainedtypevariable | spike | frontend/syntax | class: blocked | Implement Functioncallonconstrainedtypevariable |
| 2443 | Implement Functiondeclarationwithresolutionoftypenamedarguments | spike | frontend/syntax | class: blocked | Implement Functiondeclarationwithresolutionoftypenamedarguments |
| 2444 | Implement Functiondeclarationwithresolutionoftypeofsamename | spike | frontend/syntax | class: blocked | Implement Functiondeclarationwithresolutionoftypeofsamename |
| 2445 | Implement Functionexpressioninwithblock | spike | frontend/syntax | class: triage-needed | Implement Functionexpressioninwithblock |
| 2446 | Implement Functionexpressionnames | spike | frontend/syntax | class: triage-needed | Implement Functionexpressionnames |
| 2447 | Implement Functionexpressionshadowedbyparams | spike | reference/triage | class: triage-needed | Implement Functionexpressionshadowedbyparams |
| 2448 | Implement Functionexpressionwithresolutionoftypenamedarguments | spike | frontend/syntax | class: blocked | Implement Functionexpressionwithresolutionoftypenamedarguments |
| 2449 | Implement Functionexpressionwithresolutionoftypeofsamename | spike | frontend/syntax | class: blocked | Implement Functionexpressionwithresolutionoftypeofsamename |
| 2450 | Implement Functioninifstatementinmodule | spike | frontend/syntax | class: blocked | Implement Functioninifstatementinmodule |
| 2451 | Implement Functionlikeinparameterinitializer | spike | frontend/syntax | class: blocked | Implement Functionlikeinparameterinitializer |
| 2452 | Implement Functionmergedwithmodule | spike | frontend/syntax | class: blocked | Implement Functionmergedwithmodule |
| 2453 | Implement Functionoverloadambiguity | spike | frontend/syntax | class: blocked | Implement Functionoverloadambiguity |
| 2454 | Implement Functionoverloadimplementationofwrongname | spike | frontend/syntax | class: blocked | Implement Functionoverloadimplementationofwrongname |
| 2455 | Implement Functionoverloads Name Resolution | spike | frontend/resolver | class: blocked | Implement Functionoverloads Name Resolution |
| 2456 | Implement Functionoverloads Parser Syntax | spike | frontend/syntax | class: blocked | Implement Functionoverloads Parser Syntax |
| 2457 | Implement Functionoverloads Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Functionoverloads Unknown Unsupported |
| 2458 | Implement Functionoverloadsoutoforder | spike | frontend/syntax | class: blocked | Implement Functionoverloadsoutoforder |
| 2459 | Implement Functionoverloadsrecursivegenericreturntype | spike | frontend/semantics | class: blocked | Implement Functionoverloadsrecursivegenericreturntype |
| 2460 | Implement Functionparameteraritymismatch | spike | frontend/resolver | class: blocked | Implement Functionparameteraritymismatch |
| 2461 | Implement Functionreturningitself | spike | frontend/resolver | class: blocked | Implement Functionreturningitself |
| 2462 | Implement Functionsignatureassignmentcompat | spike | frontend/resolver | class: blocked | Implement Functionsignatureassignmentcompat |
| 2463 | Implement Functionsubtypingofvarargs | spike | frontend/syntax | class: blocked | Implement Functionsubtypingofvarargs |
| 2464 | Implement Functiontofunctionwithproperror | spike | runtime/builtins | class: triage-needed | Implement Functiontofunctionwithproperror |
| 2465 | Implement Functiontype | spike | reference/triage | class: triage-needed | Implement Functiontype |
| 2466 | Implement Functiontypeargumentarityerrors | spike | frontend/syntax | class: blocked | Implement Functiontypeargumentarityerrors |
| 2467 | Implement Functiontypeargumentarrayassignment | spike | frontend/syntax | class: blocked | Implement Functiontypeargumentarrayassignment |
| 2468 | Implement Functiontypeargumentassignmentcompat | spike | frontend/resolver | class: blocked | Implement Functiontypeargumentassignmentcompat |
| 2469 | Implement Functiontypeslackingreturntypes | spike | frontend/syntax | class: triage-needed | Implement Functiontypeslackingreturntypes |
| 2470 | Implement Functionwithdefaultparameterwithnostatements | spike | frontend/resolver | class: blocked | Implement Functionwithdefaultparameterwithnostatements |
| 2471 | Implement Functionwithsamenameasfield | spike | frontend/syntax | class: triage-needed | Implement Functionwithsamenameasfield |
| 2472 | Implement Functionsinclassexpressions | spike | frontend/syntax | class: triage-needed | Implement Functionsinclassexpressions |
| 2473 | Implement Functionsmissingreturnstatementsandexpressions | spike | frontend/syntax | class: triage-needed | Implement Functionsmissingreturnstatementsandexpressions |
| 2474 | Implement Functionsmissingreturnstatementsandexpressionsstrictnullchecks | spike | frontend/syntax | class: triage-needed | Implement Functionsmissingreturnstatementsandexpressionsstrictnullchecks |
| 2475 | Implement Functionswithimplicitreturntypeassignabletoundefined | spike | frontend/syntax | class: triage-needed | Implement Functionswithimplicitreturntypeassignabletoundefined |
| 2476 | Implement Functionswithmodifiersinblocks | spike | frontend/syntax | class: triage-needed | Implement Functionswithmodifiersinblocks |
| 2477 | Implement Funduleexportedclassisusedbeforedeclaration | spike | frontend/syntax | class: blocked | Implement Funduleexportedclassisusedbeforedeclaration |
| 2478 | Implement Funduleoffunctionwithoutreturntypeannotation | spike | frontend/syntax | class: blocked | Implement Funduleoffunctionwithoutreturntypeannotation |
| 2479 | Implement Fundulesplitacrossfiles | spike | frontend/syntax | class: blocked | Implement Fundulesplitacrossfiles |
| 2480 | Implement Funduleusedacrossfileboundary | spike | frontend/syntax | class: blocked | Implement Funduleusedacrossfileboundary |
| 2481 | Implement Fuzzy | spike | frontend/syntax | class: blocked | Implement Fuzzy |
| 2482 | Implement Generativerecursionwithtypeof | spike | frontend/syntax | class: triage-needed | Implement Generativerecursionwithtypeof |
| 2483 | Implement Generatores Import Export | spike | frontend/syntax | class: blocked | Implement Generatores Import Export |
| 2484 | Implement Generatores Parser Syntax | spike | runtime/builtins | class: triage-needed | Implement Generatores Parser Syntax |
| 2485 | Implement Generatortransformfinallabel | spike | reference/triage | class: triage-needed | Implement Generatortransformfinallabel |
| 2486 | Implement Genericandnongenericoverload | spike | frontend/resolver | class: blocked | Implement Genericandnongenericoverload |
| 2487 | Implement Genericargumentcallsigassignmentcompat | spike | frontend/syntax | class: blocked | Implement Genericargumentcallsigassignmentcompat |
| 2488 | Implement Genericarrayassignment | spike | frontend/resolver | class: blocked | Implement Genericarrayassignment |
| 2489 | Implement Genericarrayassignmentcompaterrors | spike | frontend/resolver | class: blocked | Implement Genericarrayassignmentcompaterrors |
| 2490 | Implement Genericarrayextenstions | spike | frontend/semantics | class: blocked | Implement Genericarrayextenstions |
| 2491 | Implement Genericarraywithouttypeannotation (audit reopened #2491) | spike | frontend/semantics | class: blocked | Implement Genericarraywithouttypeannotation (audit reopened #2491) |
| 2492 | Implement Genericassignmentcompatoffunctionsignatures | spike | frontend/semantics | class: blocked | Implement Genericassignmentcompatoffunctionsignatures |
| 2493 | Implement Genericassignmentcompatwithinterfaces | spike | frontend/semantics | class: blocked | Implement Genericassignmentcompatwithinterfaces |
| 2494 | Implement Genericbaseclassliteralproperty | spike | frontend/semantics | class: blocked | Implement Genericbaseclassliteralproperty |
| 2495 | Implement Genericcallatyieldexpressioningenericcall | spike | frontend/semantics | class: blocked | Implement Genericcallatyieldexpressioningenericcall |
| 2496 | Implement Genericcallinferenceconditionaltype Name Resolution | spike | frontend/resolver | class: blocked | Implement Genericcallinferenceconditionaltype Name Resolution |
| 2497 | Implement Genericcallinferenceconditionaltype Type System | spike | frontend/semantics | class: blocked | Implement Genericcallinferenceconditionaltype Type System |
| 2498 | Implement Genericcallinferenceinconditionaltypes | spike | frontend/semantics | class: blocked | Implement Genericcallinferenceinconditionaltypes |
| 2499 | Implement Genericcallinferenceusingthistypenoinvalidcachereuseaftermappedtypeapplication | spike | frontend/syntax | class: blocked | Implement Genericcallinferenceusingthistypenoinvalidcachereuseaftermappedtypeapplication |
| 2500 | Implement Genericcallinferencewithgenericlocalfunction | spike | frontend/semantics | class: blocked | Implement Genericcallinferencewithgenericlocalfunction |
| 2501 | Implement Genericcallonmemberreturningclosedoverobject | spike | frontend/semantics | class: blocked | Implement Genericcallonmemberreturningclosedoverobject |
| 2502 | Implement Genericcallspecializedtotypearg | spike | frontend/semantics | class: blocked | Implement Genericcallspecializedtotypearg |
| 2503 | Implement Genericcallwithinownbodycasttypeparameteridentity | spike | frontend/semantics | class: blocked | Implement Genericcallwithinownbodycasttypeparameteridentity |
| 2504 | Implement Genericcallwithoutargs | spike | frontend/semantics | class: blocked | Implement Genericcallwithoutargs |
| 2505 | Implement Genericcallbackinvokedinsideitscontainingfunction | spike | frontend/resolver | class: blocked | Implement Genericcallbackinvokedinsideitscontainingfunction |
| 2506 | Implement Genericcallbacksandclasshierarchy | spike | frontend/syntax | class: blocked | Implement Genericcallbacksandclasshierarchy |
| 2507 | Implement Genericcapturingfunctionnarrowing | spike | frontend/resolver | class: blocked | Implement Genericcapturingfunctionnarrowing |
| 2508 | Implement Genericchainedcalls | spike | frontend/semantics | class: blocked | Implement Genericchainedcalls |
| 2509 | Implement Genericclassimplementinggenericinterfacefromanothermodule | spike | frontend/syntax | class: blocked | Implement Genericclassimplementinggenericinterfacefromanothermodule |
| 2510 | Implement Genericclassinheritsconstructorfromnongenericclass | spike | frontend/semantics | class: blocked | Implement Genericclassinheritsconstructorfromnongenericclass |
| 2511 | Implement Genericclasspropertyinheritancespecialization | spike | frontend/syntax | class: blocked | Implement Genericclasspropertyinheritancespecialization |
| 2512 | Implement Genericclassstaticmethod | spike | frontend/semantics | class: blocked | Implement Genericclassstaticmethod |
| 2513 | Implement Genericclasswithstaticfactory | spike | frontend/syntax | class: blocked | Implement Genericclasswithstaticfactory |
| 2514 | Implement Genericclasswithstaticsusingtypearguments | spike | frontend/semantics | class: blocked | Implement Genericclasswithstaticsusingtypearguments |
| 2515 | Implement Genericclasses | spike | frontend/semantics | class: blocked | Implement Genericclasses |
| 2516 | Implement Genericclassesinmodule | spike | frontend/syntax | class: blocked | Implement Genericclassesinmodule |
| 2517 | Implement Genericclassesredeclaration | spike | frontend/syntax | class: blocked | Implement Genericclassesredeclaration |
| 2518 | Implement Genericcloduleinmodule | spike | frontend/syntax | class: blocked | Implement Genericcloduleinmodule |
| 2519 | Implement Genericclonereturntypes | spike | frontend/semantics | class: blocked | Implement Genericclonereturntypes |
| 2520 | Implement Genericcombinators | spike | frontend/syntax | class: blocked | Implement Genericcombinators |
| 2521 | Implement Genericconditionalconstrainedtounknownnotassignabletoconcreteobject | spike | frontend/resolver | class: blocked | Implement Genericconditionalconstrainedtounknownnotassignabletoconcreteobject |
| 2522 | Implement Genericconstraint | spike | frontend/semantics | class: blocked | Implement Genericconstraint |
| 2523 | Implement Genericconstraintdeclaration | spike | frontend/semantics | class: blocked | Implement Genericconstraintdeclaration |
| 2524 | Implement Genericconstraintonextendedbuiltintypes | spike | frontend/syntax | class: blocked | Implement Genericconstraintonextendedbuiltintypes |
| 2525 | Implement Genericconstraintsatisfaction | spike | frontend/syntax | class: blocked | Implement Genericconstraintsatisfaction |
| 2526 | Implement Genericconstructinvocationwithnotypearg | spike | frontend/resolver | class: blocked | Implement Genericconstructinvocationwithnotypearg |
| 2527 | Implement Genericconstructsignatureininterface | spike | frontend/semantics | class: blocked | Implement Genericconstructsignatureininterface |
| 2528 | Implement Genericconstructorfunction | spike | frontend/semantics | class: blocked | Implement Genericconstructorfunction |
| 2529 | Implement Genericcontextualtypingspecialization | spike | frontend/semantics | class: blocked | Implement Genericcontextualtypingspecialization |
| 2530 | Implement Genericdefaults | spike | frontend/semantics | class: blocked | Implement Genericdefaults |
| 2531 | Implement Genericdefaultserrors | spike | frontend/semantics | class: blocked | Implement Genericdefaultserrors |
| 2532 | Implement Genericdefaultsjs | spike | frontend/semantics | class: blocked | Implement Genericdefaultsjs |
| 2533 | Implement Genericderivedtypewithspecializedbase | spike | frontend/semantics | class: blocked | Implement Genericderivedtypewithspecializedbase |
| 2534 | Implement Genericfunctioncallsignaturereturntypemismatch | spike | frontend/resolver | class: blocked | Implement Genericfunctioncallsignaturereturntypemismatch |
| 2535 | Implement Genericfunctionhasfreshtypeargs | spike | frontend/semantics | class: blocked | Implement Genericfunctionhasfreshtypeargs |
| 2536 | Implement Genericfunctioninference | spike | frontend/semantics | class: blocked | Implement Genericfunctioninference |
| 2537 | Implement Genericfunctionspecializations | spike | frontend/semantics | class: blocked | Implement Genericfunctionspecializations |
| 2538 | Implement Genericfunctiontypedargumentsarefixed | spike | frontend/resolver | class: blocked | Implement Genericfunctiontypedargumentsarefixed |
| 2539 | Implement Genericfunctions | spike | frontend/resolver | class: blocked | Implement Genericfunctions |
| 2540 | Implement Genericfunctionsandconditionalinference | spike | frontend/semantics | class: blocked | Implement Genericfunctionsandconditionalinference |
| 2541 | Implement Genericfunctionsnotcontextsensitive | spike | frontend/semantics | class: blocked | Implement Genericfunctionsnotcontextsensitive |
| 2542 | Implement Genericfunctionswithoptionalparameters Name Resolution | spike | frontend/resolver | class: blocked | Implement Genericfunctionswithoptionalparameters Name Resolution |
| 2543 | Implement Genericfunctionswithoptionalparameters Type System | spike | frontend/semantics | class: blocked | Implement Genericfunctionswithoptionalparameters Type System |
| 2544 | Implement Genericfunduleinmodule | spike | frontend/syntax | class: blocked | Implement Genericfunduleinmodule |
| 2545 | Implement Genericgetter | spike | frontend/semantics | class: blocked | Implement Genericgetter |
| 2546 | Implement Genericimplements | spike | frontend/semantics | class: blocked | Implement Genericimplements |
| 2547 | Implement Genericindexedaccessmethodintersectioncanbeaccessed | spike | frontend/semantics | class: blocked | Implement Genericindexedaccessmethodintersectioncanbeaccessed |
| 2548 | Implement Genericindexedaccessvariancecomparisonresultcorrect | spike | frontend/resolver | class: blocked | Implement Genericindexedaccessvariancecomparisonresultcorrect |
| 2549 | Implement Genericinference | spike | frontend/syntax | class: blocked | Implement Genericinference |
| 2550 | Implement Genericinferencedefaulttypeparameter | spike | frontend/resolver | class: blocked | Implement Genericinferencedefaulttypeparameter |
| 2551 | Implement Genericinheriteddefaultconstructors | spike | frontend/semantics | class: blocked | Implement Genericinheriteddefaultconstructors |
| 2552 | Implement Genericinstanceof | spike | frontend/semantics | class: blocked | Implement Genericinstanceof |
| 2553 | Implement Genericinterfacefunctiontypeparameter | spike | frontend/syntax | class: blocked | Implement Genericinterfacefunctiontypeparameter |
| 2554 | Implement Genericinterfaceimplementation | spike | frontend/semantics | class: blocked | Implement Genericinterfaceimplementation |
| 2555 | Implement Genericinterfacetypecall | spike | frontend/resolver | class: blocked | Implement Genericinterfacetypecall |
| 2556 | Implement Genericinterfaceswithouttypearguments | spike | frontend/semantics | class: blocked | Implement Genericinterfaceswithouttypearguments |
| 2557 | Implement Genericisneveremptyobject | spike | reference/triage | class: triage-needed | Implement Genericisneveremptyobject |
| 2558 | Implement Genericmemberfunction | spike | frontend/syntax | class: blocked | Implement Genericmemberfunction |
| 2559 | Implement Genericmergeddeclarationusingtypeparameter Import Export | spike | frontend/syntax | class: blocked | Implement Genericmergeddeclarationusingtypeparameter Import Export |
| 2560 | Implement Genericmergeddeclarationusingtypeparameter Type System | spike | frontend/semantics | class: blocked | Implement Genericmergeddeclarationusingtypeparameter Type System |
| 2561 | Implement Genericmethodoverspecialization | spike | frontend/resolver | class: blocked | Implement Genericmethodoverspecialization |
| 2562 | Implement Genericnewinterface | spike | frontend/semantics | class: blocked | Implement Genericnewinterface |
| 2563 | Implement Genericobjectcreationwithouttypeargs | spike | frontend/semantics | class: blocked | Implement Genericobjectcreationwithouttypeargs |
| 2564 | Implement Genericobjectlitreturntype | spike | frontend/semantics | class: blocked | Implement Genericobjectlitreturntype |
| 2565 | Implement Genericobjectspreadresultinswitch | spike | frontend/semantics | class: blocked | Implement Genericobjectspreadresultinswitch |
| 2566 | Implement Genericofacloduletype | spike | frontend/semantics | class: blocked | Implement Genericofacloduletype |
| 2567 | Implement Genericoverloadsignatures | spike | frontend/semantics | class: blocked | Implement Genericoverloadsignatures |
| 2568 | Implement Genericparameterassignability | spike | frontend/semantics | class: blocked | Implement Genericparameterassignability |
| 2569 | Implement Genericprototypeproperty | spike | frontend/semantics | class: blocked | Implement Genericprototypeproperty |
| 2570 | Implement Genericrecursiveimplicitconstructorerrors | spike | frontend/syntax | class: blocked | Implement Genericrecursiveimplicitconstructorerrors |
| 2571 | Implement Genericreduce | spike | frontend/semantics | class: blocked | Implement Genericreduce |
| 2572 | Implement Genericreturntypefromgetter | spike | frontend/syntax | class: blocked | Implement Genericreturntypefromgetter |
| 2573 | Implement Genericreversingtypeparameters | spike | frontend/semantics | class: blocked | Implement Genericreversingtypeparameters |
| 2574 | Implement Genericsignatureidentity | spike | reference/triage | class: triage-needed | Implement Genericsignatureidentity |
| 2575 | Implement Genericspecializations | spike | frontend/semantics | class: blocked | Implement Genericspecializations |
| 2576 | Implement Genericstaticanytypefunction | spike | frontend/semantics | class: blocked | Implement Genericstaticanytypefunction |
| 2577 | Implement Generictemplateoverloadresolution | spike | frontend/resolver | class: blocked | Implement Generictemplateoverloadresolution |
| 2578 | Implement Generictuplewithsimplifiableelements | spike | frontend/semantics | class: blocked | Implement Generictuplewithsimplifiableelements |
| 2579 | Implement Generictypeargumentinference | spike | frontend/syntax | class: blocked | Implement Generictypeargumentinference |
| 2580 | Implement Generictypeassertions | spike | frontend/syntax | class: blocked | Implement Generictypeassertions |
| 2581 | Implement Generictypeconstraints | spike | frontend/semantics | class: blocked | Implement Generictypeconstraints |
| 2582 | Implement Generictypeparameterequivalence | spike | frontend/semantics | class: blocked | Implement Generictypeparameterequivalence |
| 2583 | Implement Generictypereferencesrequiretypeargs | spike | frontend/semantics | class: blocked | Implement Generictypereferencesrequiretypeargs |
| 2584 | Implement Generictypeusedwithouttypearguments | spike | frontend/semantics | class: blocked | Implement Generictypeusedwithouttypearguments |
| 2585 | Implement Generictypewithcallablemembers | spike | frontend/semantics | class: blocked | Implement Generictypewithcallablemembers |
| 2586 | Implement Generictypewithmultiplebases | spike | frontend/semantics | class: blocked | Implement Generictypewithmultiplebases |
| 2587 | Implement Generictypewithnongenericbasemismatch | spike | frontend/semantics | class: blocked | Implement Generictypewithnongenericbasemismatch |
| 2588 | Implement Genericunboundedtypeparamassignability | spike | frontend/semantics | class: blocked | Implement Genericunboundedtypeparamassignability |
| 2589 | Implement Genericwithcallsignaturereturningspecialization | spike | frontend/semantics | class: blocked | Implement Genericwithcallsignaturereturningspecialization |
| 2590 | Implement Genericwithcallsignatures | spike | frontend/semantics | class: blocked | Implement Genericwithcallsignatures |
| 2591 | Implement Genericwithindexeroftypeparametertype Import Export | spike | frontend/syntax | class: blocked | Implement Genericwithindexeroftypeparametertype Import Export |
| 2592 | Implement Genericwithindexeroftypeparametertype Type System | spike | frontend/semantics | class: blocked | Implement Genericwithindexeroftypeparametertype Type System |
| 2593 | Implement Genericwithopentypeparameters | spike | frontend/semantics | class: blocked | Implement Genericwithopentypeparameters |
| 2594 | Implement Generics | spike | frontend/semantics | class: blocked | Implement Generics |
| 2595 | Implement Genericsandhigherorderfunctions | spike | frontend/semantics | class: blocked | Implement Genericsandhigherorderfunctions |
| 2596 | Implement Genericswithduplicatetypeparameters | spike | frontend/semantics | class: blocked | Implement Genericswithduplicatetypeparameters |
| 2597 | Implement Genericswithouttypeparameters | spike | frontend/semantics | class: blocked | Implement Genericswithouttypeparameters |
| 2598 | Implement Getaccessorwithimpliedreturntypeandfunctionclassmerge | spike | frontend/syntax | class: blocked | Implement Getaccessorwithimpliedreturntypeandfunctionclassmerge |
| 2599 | Implement Getandsetasmembernames | spike | frontend/syntax | class: triage-needed | Implement Getandsetasmembernames |
| 2600 | Implement Getandsetnotidenticaltype Duplicate Function | spike | reference/triage | class: triage-needed | Implement Getandsetnotidenticaltype Duplicate Function |
| 2601 | Implement Getandsetnotidenticaltype Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Getandsetnotidenticaltype Parser Syntax |
| 2602 | Implement Getparameternameatposition | spike | frontend/resolver | class: blocked | Implement Getparameternameatposition |
| 2603 | Implement Getsetenumerable | spike | frontend/syntax | class: blocked | Implement Getsetenumerable |
| 2604 | Implement Getsetreturntypes | spike | frontend/syntax | class: triage-needed | Implement Getsetreturntypes |
| 2605 | Implement Gettercontrolflowstrictnull | spike | frontend/syntax | class: blocked | Implement Gettercontrolflowstrictnull |
| 2606 | Implement Gettermissingreturnerror | spike | frontend/syntax | class: blocked | Implement Gettermissingreturnerror |
| 2607 | Implement Gettersetternonaccessor | spike | frontend/syntax | class: blocked | Implement Gettersetternonaccessor |
| 2608 | Implement Gettersettersubtypeassignment | spike | frontend/syntax | class: blocked | Implement Gettersettersubtypeassignment |
| 2609 | Implement Getterthatthrowsshouldnotneedreturn | spike | frontend/syntax | class: blocked | Implement Getterthatthrowsshouldnotneedreturn |
| 2610 | Implement Gettersandsetters | spike | frontend/syntax | class: blocked | Implement Gettersandsetters |
| 2611 | Implement Gettersandsettersaccessibility | spike | frontend/syntax | class: blocked | Implement Gettersandsettersaccessibility |
| 2612 | Implement Gettersandsetterserrors | spike | frontend/syntax | class: blocked | Implement Gettersandsetterserrors |
| 2613 | Implement Gettersandsetterstypesagree | spike | frontend/syntax | class: blocked | Implement Gettersandsetterstypesagree |
| 2614 | Implement Giant | spike | frontend/syntax | class: triage-needed | Implement Giant |
| 2615 | Implement Global | spike | frontend/syntax | class: triage-needed | Implement Global |
| 2616 | Implement Globalfunctionaugmentationoverload | spike | frontend/syntax | class: blocked | Implement Globalfunctionaugmentationoverload |
| 2617 | Implement Globaliscontextualkeyword | spike | frontend/syntax | class: blocked | Implement Globaliscontextualkeyword |
| 2618 | Implement Globalthiscapture | spike | frontend/syntax | class: blocked | Implement Globalthiscapture |
| 2619 | Implement Globalthisdeclarationemit | spike | frontend/syntax | class: blocked | Implement Globalthisdeclarationemit |
| 2620 | Implement Grammarambiguities | spike | frontend/syntax | class: triage-needed | Implement Grammarambiguities |
| 2621 | Implement Heterogeneousarrayandoverloads | spike | frontend/syntax | class: blocked | Implement Heterogeneousarrayandoverloads |
| 2622 | Implement Hidingcallsignatures | spike | frontend/syntax | class: blocked | Implement Hidingcallsignatures |
| 2623 | Implement Hidingconstructsignatures | spike | frontend/syntax | class: blocked | Implement Hidingconstructsignatures |
| 2624 | Implement Higherordermappedindexlookupinference | spike | frontend/semantics | class: blocked | Implement Higherordermappedindexlookupinference |
| 2625 | Implement Homomorphicmappedtypewithnonhomomorphicinstantiationspreadable | spike | frontend/resolver | class: blocked | Implement Homomorphicmappedtypewithnonhomomorphicinstantiationspreadable |
| 2626 | Implement Hugedeclarationoutputgetstruncatedwitherror | spike | runtime/builtins | class: triage-needed | Implement Hugedeclarationoutputgetstruncatedwitherror |
| 2627 | Implement I | spike | frontend/resolver | class: blocked | Implement I |
| 2628 | Implement Icomparable | spike | frontend/resolver | class: blocked | Implement Icomparable |
| 2629 | Implement Identicalgenericconditionalswithinferrelated | spike | frontend/semantics | class: blocked | Implement Identicalgenericconditionalswithinferrelated |
| 2630 | Implement Identifierstartafternumericliteral | spike | frontend/syntax | class: triage-needed | Implement Identifierstartafternumericliteral |
| 2631 | Implement Identityanddivergentnormalizedtypes | spike | frontend/syntax | class: triage-needed | Implement Identityanddivergentnormalizedtypes |
| 2632 | Implement Identityforsignatureswithtypeparametersandany | spike | frontend/syntax | class: blocked | Implement Identityforsignatureswithtypeparametersandany |
| 2633 | Implement Identityforsignatureswithtypeparametersswitched | spike | frontend/syntax | class: blocked | Implement Identityforsignatureswithtypeparametersswitched |
| 2634 | Implement Identityrelationnevertypes | spike | frontend/syntax | class: triage-needed | Implement Identityrelationnevertypes |
| 2635 | Implement Ifelsewithstatements | spike | frontend/resolver | class: blocked | Implement Ifelsewithstatements |
| 2636 | Implement Illegalmodifiersonclasselements | spike | frontend/syntax | class: triage-needed | Implement Illegalmodifiersonclasselements |
| 2637 | Implement Illegalsupercallsinconstructor | spike | frontend/syntax | class: blocked | Implement Illegalsupercallsinconstructor |
| 2638 | Implement Implementarrayinterface | spike | frontend/syntax | class: triage-needed | Implement Implementarrayinterface |
| 2639 | Implement Implementclauseprecedingextends | spike | frontend/syntax | class: triage-needed | Implement Implementclauseprecedingextends |
| 2640 | Implement Implementgenericwithmismatchedtypes | spike | frontend/semantics | class: blocked | Implement Implementgenericwithmismatchedtypes |
| 2641 | Implement Implementinterfaceanymemberwithvoid | spike | frontend/syntax | class: triage-needed | Implement Implementinterfaceanymemberwithvoid |
| 2642 | Implement Implementpublicpropertyasprivate | spike | frontend/syntax | class: blocked | Implement Implementpublicpropertyasprivate |
| 2643 | Implement Implementsclausealreadyseen | spike | frontend/syntax | class: triage-needed | Implement Implementsclausealreadyseen |
| 2644 | Implement Implementsinclassexpression | spike | frontend/syntax | class: triage-needed | Implement Implementsinclassexpression |
| 2645 | Implement Implementsincorrectlynoassertion | spike | frontend/syntax | class: blocked | Implement Implementsincorrectlynoassertion |
| 2646 | Implement Implicitanyambients | spike | frontend/syntax | class: blocked | Implement Implicitanyambients |
| 2647 | Implement Implicitanyanyreturningfunction | spike | frontend/syntax | class: triage-needed | Implement Implicitanyanyreturningfunction |
| 2648 | Implement Implicitanycastedvalue | spike | frontend/syntax | class: triage-needed | Implement Implicitanycastedvalue |
| 2649 | Implement Implicitanydeclarefunctionexprwithoutformaltype | spike | frontend/syntax | class: blocked | Implement Implicitanydeclarefunctionexprwithoutformaltype |
| 2650 | Implement Implicitanydeclarememberwithouttype | spike | frontend/syntax | class: blocked | Implement Implicitanydeclarememberwithouttype |
| 2651 | Implement Implicitanydeclarevariableswithouttypeandinit | spike | frontend/resolver | class: blocked | Implement Implicitanydeclarevariableswithouttypeandinit |
| 2652 | Implement Implicitanyfromcircularinference | spike | frontend/semantics | class: blocked | Implement Implicitanyfromcircularinference |
| 2653 | Implement Implicitanyfunctioninvocationwithanyarguements | spike | frontend/syntax | class: triage-needed | Implement Implicitanyfunctioninvocationwithanyarguements |
| 2654 | Implement Implicitanygenerictypeinference | spike | frontend/semantics | class: blocked | Implement Implicitanygenerictypeinference |
| 2655 | Implement Implicitanygenerics | spike | frontend/semantics | class: blocked | Implement Implicitanygenerics |
| 2656 | Implement Implicitanygetandsetaccessorwithanyreturntype | spike | frontend/syntax | class: blocked | Implement Implicitanygetandsetaccessorwithanyreturntype |
| 2657 | Implement Implicitanyinambientdeclaration | spike | frontend/syntax | class: blocked | Implement Implicitanyinambientdeclaration |
| 2658 | Implement Implicitanyincatch | spike | frontend/syntax | class: triage-needed | Implement Implicitanyincatch |
| 2659 | Implement Implicitanynewexprlackconstructorsignature | spike | frontend/syntax | class: blocked | Implement Implicitanynewexprlackconstructorsignature |
| 2660 | Implement Implicitanywidentoany | spike | frontend/syntax | class: triage-needed | Implement Implicitanywidentoany |
| 2661 | Implement Implicitconstparameters | spike | frontend/syntax | class: blocked | Implement Implicitconstparameters |
| 2662 | Implement Implicitindexsignatures | spike | frontend/syntax | class: triage-needed | Implement Implicitindexsignatures |
| 2663 | Implement Impliednodeformatemit | spike | frontend/syntax | class: blocked | Implement Impliednodeformatemit |
| 2664 | Implement Impliednodeformatinterop | spike | frontend/syntax | class: blocked | Implement Impliednodeformatinterop |
| 2665 | Implement Import | spike | frontend/syntax | class: blocked | Implement Import |
| 2666 | Implement Importaliasanexternalmoduleinsideaninternalmodule | spike | frontend/syntax | class: blocked | Implement Importaliasanexternalmoduleinsideaninternalmodule |
| 2667 | Implement Importaliasfromnamespace | spike | frontend/syntax | class: blocked | Implement Importaliasfromnamespace |
| 2668 | Implement Importaliasinmoduleaugmentation | spike | frontend/syntax | class: blocked | Implement Importaliasinmoduleaugmentation |
| 2669 | Implement Importaliaswithdottedname | spike | frontend/syntax | class: blocked | Implement Importaliaswithdottedname |
| 2670 | Implement Importanimport | spike | frontend/syntax | class: blocked | Implement Importanimport |
| 2671 | Implement Importandvariabledeclarationconflict | spike | frontend/syntax | class: blocked | Implement Importandvariabledeclarationconflict |
| 2672 | Implement Importasbaseclass | spike | frontend/syntax | class: blocked | Implement Importasbaseclass |
| 2673 | Implement Importassertionnonstring | spike | frontend/syntax | class: blocked | Implement Importassertionnonstring |
| 2674 | Implement Importassertionsdeprecated | spike | frontend/syntax | class: blocked | Implement Importassertionsdeprecated |
| 2675 | Implement Importassertionsdeprecatedignored | spike | frontend/syntax | class: blocked | Implement Importassertionsdeprecatedignored |
| 2676 | Implement Importdecl | spike | frontend/syntax | class: blocked | Implement Importdecl |
| 2677 | Implement Importdeclfromtypenodeinjssource | spike | frontend/syntax | class: blocked | Implement Importdeclfromtypenodeinjssource |
| 2678 | Implement Importdeclrefereingexternalmodulewithnoresolve | spike | frontend/syntax | class: blocked | Implement Importdeclrefereingexternalmodulewithnoresolve |
| 2679 | Implement Importdecltypes | spike | frontend/syntax | class: blocked | Implement Importdecltypes |
| 2680 | Implement Importdeclwithclassmodifiers | spike | frontend/syntax | class: blocked | Implement Importdeclwithclassmodifiers |
| 2681 | Implement Importdeclwithdeclaremodifier | spike | frontend/syntax | class: blocked | Implement Importdeclwithdeclaremodifier |
| 2682 | Implement Importdeclwithdeclaremodifierinambientcontext | spike | frontend/syntax | class: blocked | Implement Importdeclwithdeclaremodifierinambientcontext |
| 2683 | Implement Importdeclwithexportmodifier | spike | frontend/syntax | class: blocked | Implement Importdeclwithexportmodifier |
| 2684 | Implement Importdeclwithexportmodifierandexportassignment | spike | frontend/syntax | class: blocked | Implement Importdeclwithexportmodifierandexportassignment |
| 2685 | Implement Importdeclwithexportmodifierandexportassignmentinambientcontext | spike | frontend/syntax | class: blocked | Implement Importdeclwithexportmodifierandexportassignmentinambientcontext |
| 2686 | Implement Importdeclwithexportmodifierinambientcontext | spike | frontend/syntax | class: blocked | Implement Importdeclwithexportmodifierinambientcontext |
| 2687 | Implement Importdeclarationinmoduledeclaration | spike | frontend/syntax | class: blocked | Implement Importdeclarationinmoduledeclaration |
| 2688 | Implement Importdeclarationnotcheckedasvaluewhentargetnonvalue | spike | frontend/syntax | class: blocked | Implement Importdeclarationnotcheckedasvaluewhentargetnonvalue |
| 2689 | Implement Importdeclarationusedastypequery | spike | frontend/syntax | class: blocked | Implement Importdeclarationusedastypequery |
| 2690 | Implement Importelisionenum | spike | frontend/syntax | class: blocked | Implement Importelisionenum |
| 2691 | Implement Importelisionexportnonexportanddefault | spike | frontend/syntax | class: blocked | Implement Importelisionexportnonexportanddefault |
| 2692 | Implement Importequalserror | spike | frontend/syntax | class: blocked | Implement Importequalserror |
| 2693 | Implement Importexportinternalcomments | spike | frontend/syntax | class: blocked | Implement Importexportinternalcomments |
| 2694 | Implement Importhelpers | spike | frontend/syntax | class: blocked | Implement Importhelpers |
| 2695 | Implement Importhelpersamd | spike | frontend/syntax | class: blocked | Implement Importhelpersamd |
| 2696 | Implement Importhelpersbundler | spike | frontend/syntax | class: blocked | Implement Importhelpersbundler |
| 2697 | Implement Importhelperscommonjsjavascript | spike | frontend/syntax | class: blocked | Implement Importhelperscommonjsjavascript |
| 2698 | Implement Importhelperses | spike | frontend/syntax | class: blocked | Implement Importhelperses |
| 2699 | Implement Importhelpersinambientcontext | spike | frontend/syntax | class: blocked | Implement Importhelpersinambientcontext |
| 2700 | Implement Importhelpersinisolatedmodules | spike | frontend/syntax | class: blocked | Implement Importhelpersinisolatedmodules |
| 2701 | Implement Importhelpersnoemithelpersexportdefault | spike | frontend/syntax | class: blocked | Implement Importhelpersnoemithelpersexportdefault |
| 2702 | Implement Importhelpersnohelpers | spike | frontend/syntax | class: blocked | Implement Importhelpersnohelpers |
| 2703 | Implement Importhelpersnohelpersforasyncgenerators | spike | frontend/syntax | class: blocked | Implement Importhelpersnohelpersforasyncgenerators |
| 2704 | Implement Importhelpersnohelpersforprivatefields | spike | frontend/syntax | class: blocked | Implement Importhelpersnohelpersforprivatefields |
| 2705 | Implement Importhelpersnomodule | spike | frontend/syntax | class: blocked | Implement Importhelpersnomodule |
| 2706 | Implement Importhelpersoutfile | spike | frontend/syntax | class: blocked | Implement Importhelpersoutfile |
| 2707 | Implement Importhelperssystem | spike | frontend/syntax | class: blocked | Implement Importhelperssystem |
| 2708 | Implement Importhelpersverbatimmodulesyntax | spike | frontend/syntax | class: blocked | Implement Importhelpersverbatimmodulesyntax |
| 2709 | Implement Importhelperswithexportstaras | spike | frontend/syntax | class: blocked | Implement Importhelperswithexportstaras |
| 2710 | Implement Importhelperswithimportorexportdefault | spike | frontend/syntax | class: blocked | Implement Importhelperswithimportorexportdefault |
| 2711 | Implement Importhelperswithimportorexportdefaultnotslib | spike | frontend/syntax | class: blocked | Implement Importhelperswithimportorexportdefaultnotslib |
| 2712 | Implement Importhelperswithimportstaras | spike | frontend/syntax | class: blocked | Implement Importhelperswithimportstaras |
| 2713 | Implement Importhelperswithlocalcollisions | spike | frontend/syntax | class: blocked | Implement Importhelperswithlocalcollisions |
| 2714 | Implement Importintypeposition | spike | frontend/syntax | class: blocked | Implement Importintypeposition |
| 2715 | Implement Importinsidemodule | spike | frontend/syntax | class: blocked | Implement Importinsidemodule |
| 2716 | Implement Importnonexportedmember Import Export | spike | frontend/syntax | class: blocked | Implement Importnonexportedmember Import Export |
| 2717 | Implement Importnonexportedmember Parser Syntax | spike | frontend/syntax | class: blocked | Implement Importnonexportedmember Parser Syntax |
| 2718 | Implement Importnotelidedwhennotfound | spike | frontend/syntax | class: blocked | Implement Importnotelidedwhennotfound |
| 2719 | Implement Importonaliasedidentifiers | spike | frontend/syntax | class: blocked | Implement Importonaliasedidentifiers |
| 2720 | Implement Importpropertyfrommappedtype | spike | frontend/syntax | class: blocked | Implement Importpropertyfrommappedtype |
| 2721 | Implement Importshadowsglobalname | spike | frontend/syntax | class: blocked | Implement Importshadowsglobalname |
| 2722 | Implement Importshouldnotbeelidedindeclarationemit | spike | frontend/syntax | class: blocked | Implement Importshouldnotbeelidedindeclarationemit |
| 2723 | Implement Importtypeassertiondeprecation | spike | frontend/syntax | class: blocked | Implement Importtypeassertiondeprecation |
| 2724 | Implement Importtypeassertiondeprecationignored | spike | frontend/syntax | class: blocked | Implement Importtypeassertiondeprecationignored |
| 2725 | Implement Importtypegenericarrowtypeparenthesized | spike | frontend/syntax | class: blocked | Implement Importtypegenericarrowtypeparenthesized |
| 2726 | Implement Importtyperesolutionjsdoceof | spike | frontend/syntax | class: blocked | Implement Importtyperesolutionjsdoceof |
| 2727 | Implement Importtypetypeofclassstaticlookup | spike | frontend/syntax | class: blocked | Implement Importtypetypeofclassstaticlookup |
| 2728 | Implement Importusedastypewitherrors | spike | frontend/syntax | class: blocked | Implement Importusedastypewitherrors |
| 2729 | Implement Importusedinextendslist | spike | frontend/syntax | class: blocked | Implement Importusedinextendslist |
| 2730 | Implement Importusedingenericimportresolves | spike | frontend/syntax | class: blocked | Implement Importusedingenericimportresolves |
| 2731 | Implement Importwithtrailingslash | spike | frontend/syntax | class: blocked | Implement Importwithtrailingslash |
| 2732 | Implement Importedaliasedconditionaltypeinstantiation | spike | frontend/syntax | class: blocked | Implement Importedaliasedconditionaltypeinstantiation |
| 2733 | Implement Importedaliasesintypepositions | spike | frontend/syntax | class: blocked | Implement Importedaliasesintypepositions |
| 2734 | Implement Importedenummembermergedwithexportedaliasiserror | spike | frontend/syntax | class: blocked | Implement Importedenummembermergedwithexportedaliasiserror |
| 2735 | Implement Importedmoduleaddtoglobal | spike | frontend/syntax | class: blocked | Implement Importedmoduleaddtoglobal |
| 2736 | Implement Importedmoduleclassnameclash | spike | frontend/syntax | class: blocked | Implement Importedmoduleclassnameclash |
| 2737 | Implement Importsinambientmodules | spike | frontend/syntax | class: blocked | Implement Importsinambientmodules |
| 2738 | Implement Indoesnotoperateonprimitivetypes | spike | frontend/syntax | class: triage-needed | Implement Indoesnotoperateonprimitivetypes |
| 2739 | Implement Inkeywordandintersection | spike | frontend/syntax | class: blocked | Implement Inkeywordandintersection |
| 2740 | Implement Inkeywordandunknown | spike | frontend/syntax | class: blocked | Implement Inkeywordandunknown |
| 2741 | Implement Inkeywordnarrowingwithnouncheckedindexedaccess | spike | frontend/syntax | class: blocked | Implement Inkeywordnarrowingwithnouncheckedindexedaccess |
| 2742 | Implement Inkeywordtypeguard | spike | frontend/syntax | class: blocked | Implement Inkeywordtypeguard |
| 2743 | Implement Inoperator | spike | frontend/resolver | class: blocked | Implement Inoperator |
| 2744 | Implement Inoperatorwithfunction | spike | frontend/syntax | class: triage-needed | Implement Inoperatorwithfunction |
| 2745 | Implement Inoperatorwithgeneric | spike | frontend/semantics | class: blocked | Implement Inoperatorwithgeneric |
| 2746 | Implement Incompatibleassignmentofidenticallynamedtypes | spike | frontend/syntax | class: blocked | Implement Incompatibleassignmentofidenticallynamedtypes |
| 2747 | Implement Incompatibleexports | spike | frontend/syntax | class: blocked | Implement Incompatibleexports |
| 2748 | Implement Incompatibletypes | spike | frontend/syntax | class: triage-needed | Implement Incompatibletypes |
| 2749 | Implement Incompletedottedexpressionateof | spike | frontend/syntax | class: triage-needed | Implement Incompletedottedexpressionateof |
| 2750 | Implement Incompleteobjectliteral | spike | frontend/syntax | class: blocked | Implement Incompleteobjectliteral |
| 2751 | Implement Incorrectclassoverloadchain | spike | frontend/syntax | class: blocked | Implement Incorrectclassoverloadchain |
| 2752 | Implement Incorrectnumberoftypeargumentsduringerrorreporting | spike | frontend/syntax | class: blocked | Implement Incorrectnumberoftypeargumentsduringerrorreporting |
| 2753 | Implement Incrementonnullassertion | spike | frontend/syntax | class: blocked | Implement Incrementonnullassertion |
| 2754 | Implement Incrementontypeparameter | spike | frontend/syntax | class: blocked | Implement Incrementontypeparameter |
| 2755 | Implement Indexat | spike | frontend/resolver | class: blocked | Implement Indexat |
| 2756 | Implement Indexintoarraysubclass | spike | frontend/resolver | class: blocked | Implement Indexintoarraysubclass |
| 2757 | Implement Indexintoenum | spike | frontend/syntax | class: blocked | Implement Indexintoenum |
| 2758 | Implement Indexsignatureandmappedtype | spike | frontend/syntax | class: blocked | Implement Indexsignatureandmappedtype |
| 2759 | Implement Indexsignatureinotherfile | spike | frontend/syntax | class: triage-needed | Implement Indexsignatureinotherfile |
| 2760 | Implement Indexsignaturemusthavetypeannotation | spike | frontend/syntax | class: triage-needed | Implement Indexsignaturemusthavetypeannotation |
| 2761 | Implement Indexsignatureoftypeunknownstillrequiresindexsignature | spike | frontend/resolver | class: blocked | Implement Indexsignatureoftypeunknownstillrequiresindexsignature |
| 2762 | Implement Indexsignaturetypecheck | spike | frontend/syntax | class: triage-needed | Implement Indexsignaturetypecheck |
| 2763 | Implement Indexsignaturewithaccessibilitymodifier | spike | frontend/syntax | class: triage-needed | Implement Indexsignaturewithaccessibilitymodifier |
| 2764 | Implement Indexsignaturewithinitializer | spike | frontend/syntax | class: triage-needed | Implement Indexsignaturewithinitializer |
| 2765 | Implement Indexsignaturewithtrailingcomma | spike | frontend/syntax | class: triage-needed | Implement Indexsignaturewithtrailingcomma |
| 2766 | Implement Indexsignaturewithouttypeannotation | spike | frontend/syntax | class: triage-needed | Implement Indexsignaturewithouttypeannotation |
| 2767 | Implement Indexsignaturesinferentialtyping | spike | frontend/semantics | class: blocked | Implement Indexsignaturesinferentialtyping |
| 2768 | Implement Indextypecheck | spike | frontend/syntax | class: triage-needed | Implement Indextypecheck |
| 2769 | Implement Indextypenosubstitutiontemplateliteral | spike | frontend/resolver | class: blocked | Implement Indextypenosubstitutiontemplateliteral |
| 2770 | Implement Indexwithoutparamtype | spike | frontend/syntax | class: triage-needed | Implement Indexwithoutparamtype |
| 2771 | Implement Indexedaccessandnullablenarrowing | spike | frontend/syntax | class: blocked | Implement Indexedaccessandnullablenarrowing |
| 2772 | Implement Indexedaccesscanbehighorder | spike | frontend/resolver | class: blocked | Implement Indexedaccesscanbehighorder |
| 2773 | Implement Indexedaccessconstraints | spike | frontend/syntax | class: blocked | Implement Indexedaccessconstraints |
| 2774 | Implement Indexedaccessimplicitlyany | spike | frontend/resolver | class: blocked | Implement Indexedaccessimplicitlyany |
| 2775 | Implement Indexedaccessnormalization | spike | frontend/resolver | class: blocked | Implement Indexedaccessnormalization |
| 2776 | Implement Indexedaccessprivatememberofgenericconstraint | spike | frontend/semantics | class: blocked | Implement Indexedaccessprivatememberofgenericconstraint |
| 2777 | Implement Indexedaccessrelation | spike | frontend/syntax | class: blocked | Implement Indexedaccessrelation |
| 2778 | Implement Indexedaccesstypeconstraints | spike | frontend/syntax | class: blocked | Implement Indexedaccesstypeconstraints |
| 2779 | Implement Indexedaccesswithfreshobjectliteral | spike | frontend/syntax | class: blocked | Implement Indexedaccesswithfreshobjectliteral |
| 2780 | Implement Indexedaccesswithvariableelement | spike | frontend/syntax | class: blocked | Implement Indexedaccesswithvariableelement |
| 2781 | Implement Indexer Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Indexer Parser Syntax |
| 2782 | Implement Indexer Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Indexer Unknown Unsupported |
| 2783 | Implement Indexera | spike | frontend/syntax | class: triage-needed | Implement Indexera |
| 2784 | Implement Indexerasoptional | spike | frontend/syntax | class: triage-needed | Implement Indexerasoptional |
| 2785 | Implement Indexerconstraints | spike | frontend/syntax | class: blocked | Implement Indexerconstraints |
| 2786 | Implement Indexerreturningtypeparameter | spike | frontend/syntax | class: blocked | Implement Indexerreturningtypeparameter |
| 2787 | Implement Indexersignaturewithrestparam | spike | frontend/syntax | class: blocked | Implement Indexersignaturewithrestparam |
| 2788 | Implement Indexingtypeswithnever | spike | frontend/syntax | class: triage-needed | Implement Indexingtypeswithnever |
| 2789 | Implement Indirectdiscriminantandexcessproperty | spike | frontend/resolver | class: blocked | Implement Indirectdiscriminantandexcessproperty |
| 2790 | Implement Indirectglobalsymbolpartofobjecttype | spike | frontend/syntax | class: blocked | Implement Indirectglobalsymbolpartofobjecttype |
| 2791 | Implement Indirectselfreferencegeneric | spike | frontend/semantics | class: blocked | Implement Indirectselfreferencegeneric |
| 2792 | Implement Indirecttypeparameterreferences | spike | frontend/syntax | class: triage-needed | Implement Indirecttypeparameterreferences |
| 2793 | Implement Indirectuniquesymboldeclarationemit | spike | frontend/syntax | class: blocked | Implement Indirectuniquesymboldeclarationemit |
| 2794 | Implement Inexistentpropertyinsidetostringtype | spike | frontend/syntax | class: triage-needed | Implement Inexistentpropertyinsidetostringtype |
| 2795 | Implement Inferfromannotatedreturn | spike | frontend/semantics | class: blocked | Implement Inferfromannotatedreturn |
| 2796 | Implement Inferfromgenericfunctionreturntypes | spike | frontend/semantics | class: blocked | Implement Inferfromgenericfunctionreturntypes |
| 2797 | Implement Inferfromnestedsameshapetuple | spike | frontend/syntax | class: triage-needed | Implement Inferfromnestedsameshapetuple |
| 2798 | Implement Inferobjecttypefromstringliteraltokeyof | spike | frontend/resolver | class: blocked | Implement Inferobjecttypefromstringliteraltokeyof |
| 2799 | Implement Inferparameterwithmethodcallinitializer | spike | frontend/semantics | class: blocked | Implement Inferparameterwithmethodcallinitializer |
| 2800 | Implement Inferpropertywithcontextsensitivereturnstatement | spike | frontend/resolver | class: blocked | Implement Inferpropertywithcontextsensitivereturnstatement |
| 2801 | Implement Inferrestargumentsmappedtuple | spike | frontend/resolver | class: blocked | Implement Inferrestargumentsmappedtuple |
| 2802 | Implement Infersecondaryparameter | spike | frontend/semantics | class: blocked | Implement Infersecondaryparameter |
| 2803 | Implement Infersetterparamtype | spike | reference/triage | class: triage-needed | Implement Infersetterparamtype |
| 2804 | Implement Inferstringliteralunionforbindingelement | spike | frontend/semantics | class: blocked | Implement Inferstringliteralunionforbindingelement |
| 2805 | Implement Infertuplefrombindingpattern | spike | frontend/resolver | class: blocked | Implement Infertuplefrombindingpattern |
| 2806 | Implement Infertypeconstraintinstantiationcircularity | spike | frontend/semantics | class: blocked | Implement Infertypeconstraintinstantiationcircularity |
| 2807 | Implement Infertypeparameterconstraints | spike | frontend/semantics | class: blocked | Implement Infertypeparameterconstraints |
| 2808 | Implement Infertypepredicates | spike | frontend/semantics | class: blocked | Implement Infertypepredicates |
| 2809 | Implement Inferenceandhkts | spike | frontend/syntax | class: triage-needed | Implement Inferenceandhkts |
| 2810 | Implement Inferencecontextualreturntypeunion Import Export | spike | frontend/syntax | class: blocked | Implement Inferencecontextualreturntypeunion Import Export |
| 2811 | Implement Inferencecontextualreturntypeunion Name Resolution | spike | frontend/resolver | class: blocked | Implement Inferencecontextualreturntypeunion Name Resolution |
| 2812 | Implement Inferencecontextualreturntypeunion Type System | spike | frontend/semantics | class: blocked | Implement Inferencecontextualreturntypeunion Type System |
| 2813 | Implement Inferencedoesnotaddundefinedornull | spike | frontend/semantics | class: blocked | Implement Inferencedoesnotaddundefinedornull |
| 2814 | Implement Inferencedoesntcompareagainstuninstantiatedtypeparameter | spike | frontend/semantics | class: blocked | Implement Inferencedoesntcompareagainstuninstantiatedtypeparameter |
| 2815 | Implement Inferenceerasedsignatures | spike | frontend/semantics | class: blocked | Implement Inferenceerasedsignatures |
| 2816 | Implement Inferenceexactoptionalproperties | spike | frontend/semantics | class: blocked | Implement Inferenceexactoptionalproperties |
| 2817 | Implement Inferencefromgenericclassnocrash | spike | frontend/semantics | class: blocked | Implement Inferencefromgenericclassnocrash |
| 2818 | Implement Inferencefromincompletesource | spike | frontend/syntax | class: triage-needed | Implement Inferencefromincompletesource |
| 2819 | Implement Inferencelimit | spike | frontend/syntax | class: blocked | Implement Inferencelimit |
| 2820 | Implement Inferenceofnullableobjecttypeswithcommonbase | spike | frontend/semantics | class: blocked | Implement Inferenceofnullableobjecttypeswithcommonbase |
| 2821 | Implement Inferenceoptionalproperties | spike | reference/triage | class: triage-needed | Implement Inferenceoptionalproperties |
| 2822 | Implement Inferenceoptionalpropertiesstrict | spike | reference/triage | class: triage-needed | Implement Inferenceoptionalpropertiesstrict |
| 2823 | Implement Inferenceoptionalpropertiestoindexsignatures | spike | frontend/semantics | class: blocked | Implement Inferenceoptionalpropertiestoindexsignatures |
| 2824 | Implement Inferenceouterresultnotincorrectlyinstantiatedwithinnerresult | spike | frontend/syntax | class: blocked | Implement Inferenceouterresultnotincorrectlyinstantiatedwithinnerresult |
| 2825 | Implement Inferenceunionofobjectsmappedcontextualtype | spike | frontend/semantics | class: blocked | Implement Inferenceunionofobjectsmappedcontextualtype |
| 2826 | Implement Inferentialtypingobjectliteralmethod | spike | frontend/resolver | class: blocked | Implement Inferentialtypingobjectliteralmethod |
| 2827 | Implement Inferentialtypingusingapparenttype | spike | frontend/semantics | class: blocked | Implement Inferentialtypingusingapparenttype |
| 2828 | Implement Inferentialtypingwithfunctiontype | spike | frontend/resolver | class: blocked | Implement Inferentialtypingwithfunctiontype |
| 2829 | Implement Inferentialtypingwithfunctiontypenested | spike | frontend/resolver | class: blocked | Implement Inferentialtypingwithfunctiontypenested |
| 2830 | Implement Inferentialtypingwithfunctiontypesyntacticscenarios | spike | frontend/semantics | class: blocked | Implement Inferentialtypingwithfunctiontypesyntacticscenarios |
| 2831 | Implement Inferentialtypingwithfunctiontypezip | spike | frontend/semantics | class: blocked | Implement Inferentialtypingwithfunctiontypezip |
| 2832 | Implement Inferentialtypingwithobjectliteralproperties | spike | frontend/semantics | class: blocked | Implement Inferentialtypingwithobjectliteralproperties |
| 2833 | Implement Inferentiallytypinganemptyarray | spike | frontend/resolver | class: blocked | Implement Inferentiallytypinganemptyarray |
| 2834 | Implement Inferredindexeronnamespaceimport | spike | frontend/syntax | class: blocked | Implement Inferredindexeronnamespaceimport |
| 2835 | Implement Inferrednonidentifiertypesgetquotes | spike | frontend/semantics | class: blocked | Implement Inferrednonidentifiertypesgetquotes |
| 2836 | Implement Inferredresttypefixedonce | spike | reference/triage | class: triage-needed | Implement Inferredresttypefixedonce |
| 2837 | Implement Inferredreturntypeincorrectreuse | spike | frontend/syntax | class: triage-needed | Implement Inferredreturntypeincorrectreuse |
| 2838 | Implement Inferrenceinfiniteloopwithsubtyping | spike | frontend/syntax | class: blocked | Implement Inferrenceinfiniteloopwithsubtyping |
| 2839 | Implement Inferringreturntypefromconstructsignaturegeneric | spike | frontend/semantics | class: blocked | Implement Inferringreturntypefromconstructsignaturegeneric |
| 2840 | Implement Infiniteconstraints | spike | frontend/resolver | class: blocked | Implement Infiniteconstraints |
| 2841 | Implement Infinitelyexpandingbasetypes | spike | frontend/syntax | class: triage-needed | Implement Infinitelyexpandingbasetypes |
| 2842 | Implement Infinitelyexpandingoverloads | spike | frontend/syntax | class: blocked | Implement Infinitelyexpandingoverloads |
| 2843 | Implement Infinitelyexpandingtypeassignability | spike | frontend/syntax | class: triage-needed | Implement Infinitelyexpandingtypeassignability |
| 2844 | Implement Infinitelyexpandingtypesnongenericbase | spike | frontend/semantics | class: blocked | Implement Infinitelyexpandingtypesnongenericbase |
| 2845 | Implement Inheritfromgenerictypeparameter | spike | frontend/semantics | class: blocked | Implement Inheritfromgenerictypeparameter |
| 2846 | Implement Inheritsamenameprivatepropertiesfromdifferentorigins | spike | frontend/syntax | class: blocked | Implement Inheritsamenameprivatepropertiesfromdifferentorigins |
| 2847 | Implement Inheritsamenameprivatepropertiesfromsameorigin | spike | frontend/syntax | class: blocked | Implement Inheritsamenameprivatepropertiesfromsameorigin |
| 2848 | Implement Inheritsamenamepropertieswithdifferentvisibility | spike | frontend/syntax | class: triage-needed | Implement Inheritsamenamepropertieswithdifferentvisibility |
| 2849 | Implement Inheritance | spike | frontend/syntax | class: triage-needed | Implement Inheritance |
| 2850 | Implement Inheritancegrandparentprivatemembercollision | spike | frontend/syntax | class: blocked | Implement Inheritancegrandparentprivatemembercollision |
| 2851 | Implement Inheritancegrandparentprivatemembercollisionwithpublicmember | spike | frontend/syntax | class: blocked | Implement Inheritancegrandparentprivatemembercollisionwithpublicmember |
| 2852 | Implement Inheritancegrandparentpublicmembercollisionwithprivatemember | spike | frontend/syntax | class: blocked | Implement Inheritancegrandparentpublicmembercollisionwithprivatemember |
| 2853 | Implement Inheritancememberaccessoroverridingaccessor | spike | reference/triage | class: triage-needed | Implement Inheritancememberaccessoroverridingaccessor |
| 2854 | Implement Inheritancememberaccessoroverridingmethod | spike | reference/triage | class: triage-needed | Implement Inheritancememberaccessoroverridingmethod |
| 2855 | Implement Inheritancememberaccessoroverridingproperty | spike | reference/triage | class: triage-needed | Implement Inheritancememberaccessoroverridingproperty |
| 2856 | Implement Inheritancememberfuncoverridingaccessor | spike | reference/triage | class: triage-needed | Implement Inheritancememberfuncoverridingaccessor |
| 2857 | Implement Inheritancememberpropertyoverridingaccessor | spike | frontend/syntax | class: blocked | Implement Inheritancememberpropertyoverridingaccessor |
| 2858 | Implement Inheritanceofgenericconstructormethod Import Export | spike | frontend/syntax | class: blocked | Implement Inheritanceofgenericconstructormethod Import Export |
| 2859 | Implement Inheritanceofgenericconstructormethod Type System | spike | frontend/semantics | class: blocked | Implement Inheritanceofgenericconstructormethod Type System |
| 2860 | Implement Inheritancestaticaccessoroverridingaccessor | spike | reference/triage | class: triage-needed | Implement Inheritancestaticaccessoroverridingaccessor |
| 2861 | Implement Inheritancestaticaccessoroverridingmethod | spike | reference/triage | class: triage-needed | Implement Inheritancestaticaccessoroverridingmethod |
| 2862 | Implement Inheritancestaticaccessoroverridingproperty | spike | reference/triage | class: triage-needed | Implement Inheritancestaticaccessoroverridingproperty |
| 2863 | Implement Inheritancestaticfuncoverridingaccessor | spike | reference/triage | class: triage-needed | Implement Inheritancestaticfuncoverridingaccessor |
| 2864 | Implement Inheritancestaticpropertyoverridingaccessor | spike | reference/triage | class: triage-needed | Implement Inheritancestaticpropertyoverridingaccessor |
| 2865 | Implement Inheritedconstructorpropertycontextualtype | spike | frontend/syntax | class: blocked | Implement Inheritedconstructorpropertycontextualtype |
| 2866 | Implement Inheritedconstructorwithrestparams Arity | spike | reference/triage | class: triage-needed | Implement Inheritedconstructorwithrestparams Arity |
| 2867 | Implement Inheritedconstructorwithrestparams Parser Syntax | spike | frontend/syntax | class: blocked | Implement Inheritedconstructorwithrestparams Parser Syntax |
| 2868 | Implement Inheritedgenericcallsignature | spike | frontend/semantics | class: blocked | Implement Inheritedgenericcallsignature |
| 2869 | Implement Inheritedmodulemembersforclodule | spike | frontend/syntax | class: blocked | Implement Inheritedmodulemembersforclodule |
| 2870 | Implement Inheritedoverloadedspecializedsignatures | spike | frontend/syntax | class: blocked | Implement Inheritedoverloadedspecializedsignatures |
| 2871 | Implement Initializepropertieswithrenamedlet | spike | frontend/syntax | class: triage-needed | Implement Initializepropertieswithrenamedlet |
| 2872 | Implement Initializeddestructuringassignmenttypes | spike | reference/triage | class: triage-needed | Implement Initializeddestructuringassignmenttypes |
| 2873 | Implement Initializerwiththispropertyaccess | spike | frontend/syntax | class: triage-needed | Implement Initializerwiththispropertyaccess |
| 2874 | Implement Inlineconditionalhassimilarassignability | spike | frontend/semantics | class: blocked | Implement Inlineconditionalhassimilarassignability |
| 2875 | Implement Inlinemappedtypemodifierdeclarationemit | spike | frontend/syntax | class: blocked | Implement Inlinemappedtypemodifierdeclarationemit |
| 2876 | Implement Inneraliases | spike | frontend/syntax | class: blocked | Implement Inneraliases |
| 2877 | Implement Innerboundlambdaemit | spike | frontend/syntax | class: blocked | Implement Innerboundlambdaemit |
| 2878 | Implement Innerextern | spike | frontend/syntax | class: blocked | Implement Innerextern |
| 2879 | Implement Innerfunc | spike | frontend/syntax | class: blocked | Implement Innerfunc |
| 2880 | Implement Innermodexport | spike | frontend/syntax | class: blocked | Implement Innermodexport |
| 2881 | Implement Inneroverloads | spike | frontend/syntax | class: blocked | Implement Inneroverloads |
| 2882 | Implement Instanceandstaticdeclarations | spike | frontend/syntax | class: triage-needed | Implement Instanceandstaticdeclarations |
| 2883 | Implement Instanceofassignability | spike | frontend/syntax | class: triage-needed | Implement Instanceofassignability |
| 2884 | Implement Instanceofinexternalmodules | spike | frontend/syntax | class: blocked | Implement Instanceofinexternalmodules |
| 2885 | Implement Instancesubtypecheck | spike | frontend/syntax | class: triage-needed | Implement Instancesubtypecheck |
| 2886 | Implement Instanceofnarrowreadonlyarray | spike | frontend/syntax | class: blocked | Implement Instanceofnarrowreadonlyarray |
| 2887 | Implement Instanceofoninstantiationexpression | spike | frontend/syntax | class: triage-needed | Implement Instanceofoninstantiationexpression |
| 2888 | Implement Instanceoftypealiastogenericclass | spike | frontend/syntax | class: blocked | Implement Instanceoftypealiastogenericclass |
| 2889 | Implement Instanceofwithprimitiveunion | spike | frontend/syntax | class: blocked | Implement Instanceofwithprimitiveunion |
| 2890 | Implement Instantiatecontextualtypes | spike | frontend/syntax | class: blocked | Implement Instantiatecontextualtypes |
| 2891 | Implement Instantiatecontextuallytypedgenericthis | spike | frontend/semantics | class: blocked | Implement Instantiatecontextuallytypedgenericthis |
| 2892 | Implement Instantiatecrossfilemerge | spike | frontend/syntax | class: triage-needed | Implement Instantiatecrossfilemerge |
| 2893 | Implement Instantiatedbasetypeconstraints | spike | frontend/syntax | class: blocked | Implement Instantiatedbasetypeconstraints |
| 2894 | Implement Instantiatedtypealiasdisplay | spike | frontend/syntax | class: blocked | Implement Instantiatedtypealiasdisplay |
| 2895 | Implement Instantiationexpressionerrornocrash | spike | frontend/syntax | class: triage-needed | Implement Instantiationexpressionerrornocrash |
| 2896 | Implement Inttypecheck | spike | frontend/syntax | class: triage-needed | Implement Inttypecheck |
| 2897 | Implement Intermixingmodulesinterfaces | spike | frontend/syntax | class: blocked | Implement Intermixingmodulesinterfaces |
| 2898 | Implement Interfaceassignmentcompat | spike | frontend/syntax | class: blocked | Implement Interfaceassignmentcompat |
| 2899 | Implement Interfaceclassmerging | spike | frontend/syntax | class: blocked | Implement Interfaceclassmerging |
| 2900 | Implement Interfacecontextualtype | spike | frontend/syntax | class: blocked | Implement Interfacecontextualtype |
| 2901 | Implement Interfacedeclaration Import Export | spike | frontend/syntax | class: blocked | Implement Interfacedeclaration Import Export |
| 2902 | Implement Interfacedeclaration Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Interfacedeclaration Parser Syntax |
| 2903 | Implement Interfaceextendsclass | spike | frontend/syntax | class: triage-needed | Implement Interfaceextendsclass |
| 2904 | Implement Interfaceextendsclasswithprivate | spike | frontend/syntax | class: blocked | Implement Interfaceextendsclasswithprivate |
| 2905 | Implement Interfaceimplementation | spike | frontend/syntax | class: triage-needed | Implement Interfaceimplementation |
| 2906 | Implement Interfaceinreopenedmodule | spike | frontend/syntax | class: blocked | Implement Interfaceinreopenedmodule |
| 2907 | Implement Interfaceinheritance Method Call | spike | frontend/syntax | class: blocked | Implement Interfaceinheritance Method Call |
| 2908 | Implement Interfaceinheritance Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Interfaceinheritance Parser Syntax |
| 2909 | Implement Interfacemergewithnongenerictypearguments | spike | frontend/syntax | class: blocked | Implement Interfacemergewithnongenerictypearguments |
| 2910 | Implement Interfacemergedunconstrainednoerrorirrespectiveoforder | spike | frontend/syntax | class: blocked | Implement Interfacemergedunconstrainednoerrorirrespectiveoforder |
| 2911 | Implement Interfacenameasidentifier | spike | frontend/syntax | class: blocked | Implement Interfacenameasidentifier |
| 2912 | Implement Interfacenaming | spike | frontend/syntax | class: triage-needed | Implement Interfacenaming |
| 2913 | Implement Interfacepropertieswithsamename | spike | frontend/syntax | class: blocked | Implement Interfacepropertieswithsamename |
| 2914 | Implement Interfacesubtyping | spike | frontend/syntax | class: triage-needed | Implement Interfacesubtyping |
| 2915 | Implement Interfacewithcommaseparators | spike | frontend/syntax | class: triage-needed | Implement Interfacewithcommaseparators |
| 2916 | Implement Interfacewithmultipledeclarations | spike | frontend/syntax | class: triage-needed | Implement Interfacewithmultipledeclarations |
| 2917 | Implement Interfacedecl | spike | frontend/syntax | class: triage-needed | Implement Interfacedecl |
| 2918 | Implement Interfacedeclwithindexererrors | spike | runtime/builtins | class: triage-needed | Implement Interfacedeclwithindexererrors |
| 2919 | Implement Internalaliasclass | spike | frontend/syntax | class: blocked | Implement Internalaliasclass |
| 2920 | Implement Internalaliasclassinsidelocalmodulewithexport | spike | frontend/syntax | class: blocked | Implement Internalaliasclassinsidelocalmodulewithexport |
| 2921 | Implement Internalaliasclassinsidelocalmodulewithoutexport | spike | frontend/syntax | class: blocked | Implement Internalaliasclassinsidelocalmodulewithoutexport |
| 2922 | Implement Internalaliasclassinsidelocalmodulewithoutexportaccesserror | spike | frontend/syntax | class: blocked | Implement Internalaliasclassinsidelocalmodulewithoutexportaccesserror |
| 2923 | Implement Internalaliasclassinsidetoplevelmodulewithexport | spike | frontend/syntax | class: blocked | Implement Internalaliasclassinsidetoplevelmodulewithexport |
| 2924 | Implement Internalaliasclassinsidetoplevelmodulewithoutexport | spike | frontend/syntax | class: blocked | Implement Internalaliasclassinsidetoplevelmodulewithoutexport |
| 2925 | Implement Internalaliasenum | spike | frontend/syntax | class: blocked | Implement Internalaliasenum |
| 2926 | Implement Internalaliasenuminsidelocalmodulewithexport | spike | frontend/syntax | class: blocked | Implement Internalaliasenuminsidelocalmodulewithexport |
| 2927 | Implement Internalaliasenuminsidelocalmodulewithoutexport | spike | frontend/syntax | class: blocked | Implement Internalaliasenuminsidelocalmodulewithoutexport |
| 2928 | Implement Internalaliasenuminsidelocalmodulewithoutexportaccesserror | spike | frontend/syntax | class: blocked | Implement Internalaliasenuminsidelocalmodulewithoutexportaccesserror |
| 2929 | Implement Internalaliasenuminsidetoplevelmodulewithexport | spike | frontend/syntax | class: blocked | Implement Internalaliasenuminsidetoplevelmodulewithexport |
| 2930 | Implement Internalaliasenuminsidetoplevelmodulewithoutexport | spike | frontend/syntax | class: blocked | Implement Internalaliasenuminsidetoplevelmodulewithoutexport |
| 2931 | Implement Internalaliasfunction | spike | frontend/syntax | class: blocked | Implement Internalaliasfunction |
| 2932 | Implement Internalaliasfunctioninsidelocalmodulewithexport | spike | frontend/syntax | class: blocked | Implement Internalaliasfunctioninsidelocalmodulewithexport |
| 2933 | Implement Internalaliasfunctioninsidelocalmodulewithoutexport | spike | frontend/syntax | class: blocked | Implement Internalaliasfunctioninsidelocalmodulewithoutexport |
| 2934 | Implement Internalaliasfunctioninsidelocalmodulewithoutexportaccesserror | spike | frontend/syntax | class: blocked | Implement Internalaliasfunctioninsidelocalmodulewithoutexportaccesserror |
| 2935 | Implement Internalaliasfunctioninsidetoplevelmodulewithexport | spike | frontend/syntax | class: blocked | Implement Internalaliasfunctioninsidetoplevelmodulewithexport |
| 2936 | Implement Internalaliasfunctioninsidetoplevelmodulewithoutexport | spike | frontend/syntax | class: blocked | Implement Internalaliasfunctioninsidetoplevelmodulewithoutexport |
| 2937 | Implement Internalaliasinitializedmodule | spike | frontend/syntax | class: blocked | Implement Internalaliasinitializedmodule |
| 2938 | Implement Internalaliasinitializedmoduleinsidelocalmodulewithexport | spike | frontend/syntax | class: blocked | Implement Internalaliasinitializedmoduleinsidelocalmodulewithexport |
| 2939 | Implement Internalaliasinitializedmoduleinsidelocalmodulewithoutexport | spike | frontend/syntax | class: blocked | Implement Internalaliasinitializedmoduleinsidelocalmodulewithoutexport |
| 2940 | Implement Internalaliasinitializedmoduleinsidelocalmodulewithoutexportaccesserror | spike | frontend/syntax | class: blocked | Implement Internalaliasinitializedmoduleinsidelocalmodulewithoutexportaccesserror |
| 2941 | Implement Internalaliasinitializedmoduleinsidetoplevelmodulewithexport | spike | frontend/syntax | class: blocked | Implement Internalaliasinitializedmoduleinsidetoplevelmodulewithexport |
| 2942 | Implement Internalaliasinitializedmoduleinsidetoplevelmodulewithoutexport | spike | frontend/syntax | class: blocked | Implement Internalaliasinitializedmoduleinsidetoplevelmodulewithoutexport |
| 2943 | Implement Internalaliasinterface | spike | frontend/syntax | class: blocked | Implement Internalaliasinterface |
| 2944 | Implement Internalaliasinterfaceinsidelocalmodulewithexport | spike | frontend/syntax | class: blocked | Implement Internalaliasinterfaceinsidelocalmodulewithexport |
| 2945 | Implement Internalaliasinterfaceinsidelocalmodulewithoutexport | spike | frontend/syntax | class: blocked | Implement Internalaliasinterfaceinsidelocalmodulewithoutexport |
| 2946 | Implement Internalaliasinterfaceinsidelocalmodulewithoutexportaccesserror | spike | frontend/syntax | class: blocked | Implement Internalaliasinterfaceinsidelocalmodulewithoutexportaccesserror |
| 2947 | Implement Internalaliasinterfaceinsidetoplevelmodulewithexport | spike | frontend/syntax | class: blocked | Implement Internalaliasinterfaceinsidetoplevelmodulewithexport |
| 2948 | Implement Internalaliasinterfaceinsidetoplevelmodulewithoutexport | spike | frontend/syntax | class: blocked | Implement Internalaliasinterfaceinsidetoplevelmodulewithoutexport |
| 2949 | Implement Internalaliasuninitializedmodule | spike | frontend/syntax | class: blocked | Implement Internalaliasuninitializedmodule |
| 2950 | Implement Internalaliasuninitializedmoduleinsidelocalmodulewithexport | spike | frontend/syntax | class: blocked | Implement Internalaliasuninitializedmoduleinsidelocalmodulewithexport |
| 2951 | Implement Internalaliasuninitializedmoduleinsidelocalmodulewithoutexport | spike | frontend/syntax | class: blocked | Implement Internalaliasuninitializedmoduleinsidelocalmodulewithoutexport |
| 2952 | Implement Internalaliasuninitializedmoduleinsidelocalmodulewithoutexportaccesserror | spike | frontend/syntax | class: blocked | Implement Internalaliasuninitializedmoduleinsidelocalmodulewithoutexportaccesserror |
| 2953 | Implement Internalaliasuninitializedmoduleinsidetoplevelmodulewithexport | spike | frontend/syntax | class: blocked | Implement Internalaliasuninitializedmoduleinsidetoplevelmodulewithexport |
| 2954 | Implement Internalaliasuninitializedmoduleinsidetoplevelmodulewithoutexport | spike | frontend/syntax | class: blocked | Implement Internalaliasuninitializedmoduleinsidetoplevelmodulewithoutexport |
| 2955 | Implement Internalaliasvar | spike | frontend/syntax | class: blocked | Implement Internalaliasvar |
| 2956 | Implement Internalaliasvarinsidelocalmodulewithexport | spike | frontend/syntax | class: blocked | Implement Internalaliasvarinsidelocalmodulewithexport |
| 2957 | Implement Internalaliasvarinsidelocalmodulewithoutexport | spike | frontend/syntax | class: blocked | Implement Internalaliasvarinsidelocalmodulewithoutexport |
| 2958 | Implement Internalaliasvarinsidelocalmodulewithoutexportaccesserror | spike | frontend/syntax | class: blocked | Implement Internalaliasvarinsidelocalmodulewithoutexportaccesserror |
| 2959 | Implement Internalaliasvarinsidetoplevelmodulewithexport | spike | frontend/syntax | class: blocked | Implement Internalaliasvarinsidetoplevelmodulewithexport |
| 2960 | Implement Internalaliasvarinsidetoplevelmodulewithoutexport | spike | frontend/syntax | class: blocked | Implement Internalaliasvarinsidetoplevelmodulewithoutexport |
| 2961 | Implement Internalaliaswithdottednameemit | spike | frontend/syntax | class: blocked | Implement Internalaliaswithdottednameemit |
| 2962 | Implement Internalimportinstantiatedmodulemergedwithclassnotreferencinginstance | spike | frontend/syntax | class: blocked | Implement Internalimportinstantiatedmodulemergedwithclassnotreferencinginstance |
| 2963 | Implement Internalimportinstantiatedmodulemergedwithclassnotreferencinginstancenoconflict | spike | frontend/syntax | class: blocked | Implement Internalimportinstantiatedmodulemergedwithclassnotreferencinginstancenoconflict |
| 2964 | Implement Internalimportinstantiatedmodulenotreferencinginstance | spike | frontend/syntax | class: blocked | Implement Internalimportinstantiatedmodulenotreferencinginstance |
| 2965 | Implement Internalimportuninstantiatedmodulemergedwithclassnotreferencinginstance | spike | frontend/syntax | class: blocked | Implement Internalimportuninstantiatedmodulemergedwithclassnotreferencinginstance |
| 2966 | Implement Internalimportuninstantiatedmodulemergedwithclassnotreferencinginstancenoconflict | spike | frontend/syntax | class: blocked | Implement Internalimportuninstantiatedmodulemergedwithclassnotreferencinginstancenoconflict |
| 2967 | Implement Internalimportuninstantiatedmodulenotreferencinginstancenoconflict | spike | frontend/syntax | class: blocked | Implement Internalimportuninstantiatedmodulenotreferencinginstancenoconflict |
| 2968 | Implement Intersectionofmixinconstructortypeandnonconstructortype | spike | frontend/resolver | class: blocked | Implement Intersectionofmixinconstructortypeandnonconstructortype |
| 2969 | Implement Intersectionoftypevariablehasapparentsignatures | spike | frontend/resolver | class: blocked | Implement Intersectionoftypevariablehasapparentsignatures |
| 2970 | Implement Intersectionpropertycheck | spike | frontend/resolver | class: blocked | Implement Intersectionpropertycheck |
| 2971 | Implement Intersectionsatisfiesconstraint | spike | frontend/syntax | class: blocked | Implement Intersectionsatisfiesconstraint |
| 2972 | Implement Intersectiontype | spike | frontend/syntax | class: blocked | Implement Intersectiontype |
| 2973 | Implement Intersectiontypeinference | spike | frontend/semantics | class: blocked | Implement Intersectiontypeinference |
| 2974 | Implement Intersectiontypenormalization | spike | frontend/syntax | class: blocked | Implement Intersectiontypenormalization |
| 2975 | Implement Intersectionwithconflictingprivates | spike | frontend/syntax | class: blocked | Implement Intersectionwithconflictingprivates |
| 2976 | Implement Intersectionsandoptionalproperties | spike | frontend/resolver | class: blocked | Implement Intersectionsandoptionalproperties |
| 2977 | Implement Intersectionsandreadonlyproperties | spike | frontend/resolver | class: blocked | Implement Intersectionsandreadonlyproperties |
| 2978 | Implement Intersectionsoflargeunions Import Export | spike | frontend/syntax | class: blocked | Implement Intersectionsoflargeunions Import Export |
| 2979 | Implement Intersectionsoflargeunions Parser Syntax | spike | frontend/syntax | class: blocked | Implement Intersectionsoflargeunions Parser Syntax |
| 2980 | Implement Intrabindingpatternreferences | spike | frontend/resolver | class: blocked | Implement Intrabindingpatternreferences |
| 2981 | Implement Intrinsics | spike | frontend/syntax | class: blocked | Implement Intrinsics |
| 2982 | Implement Invalidcontinueindownlevelasync | spike | reference/triage | class: triage-needed | Implement Invalidcontinueindownlevelasync |
| 2983 | Implement Invalidletinforofandforin | spike | frontend/syntax | class: triage-needed | Implement Invalidletinforofandforin |
| 2984 | Implement Invalidoptionalchainfromnewexpression | spike | frontend/syntax | class: triage-needed | Implement Invalidoptionalchainfromnewexpression |
| 2985 | Implement Invalidsplice | spike | frontend/syntax | class: blocked | Implement Invalidsplice |
| 2986 | Implement Invalidstaticfield | spike | frontend/syntax | class: triage-needed | Implement Invalidstaticfield |
| 2987 | Implement Invalidsymbolintypeparameter | spike | frontend/syntax | class: blocked | Implement Invalidsymbolintypeparameter |
| 2988 | Implement Invalidthisemitincontextualobjectliteral | spike | frontend/syntax | class: blocked | Implement Invalidthisemitincontextualobjectliteral |
| 2989 | Implement Invalidunicodeescapesequance Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Invalidunicodeescapesequance Parser Syntax |
| 2990 | Implement Invalidunicodeescapesequance Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Invalidunicodeescapesequance Unknown Unsupported |
| 2991 | Implement Invariantgenericerrorelaboration | spike | frontend/syntax | class: blocked | Implement Invariantgenericerrorelaboration |
| 2992 | Implement Invokingnongenericmethodwithtypearguments | spike | frontend/semantics | class: blocked | Implement Invokingnongenericmethodwithtypearguments |
| 2993 | Implement Ipromise Class | spike | frontend/syntax | class: blocked | Implement Ipromise Class |
| 2994 | Implement Ipromise Import Export | spike | frontend/syntax | class: blocked | Implement Ipromise Import Export |
| 2995 | Implement Isarray | spike | frontend/resolver | class: blocked | Implement Isarray |
| 2996 | Implement Isdeclarationvisiblenodekinds | spike | frontend/syntax | class: blocked | Implement Isdeclarationvisiblenodekinds |
| 2997 | Implement Isolateddeclarationerrortypes | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationerrortypes |
| 2998 | Implement Isolateddeclarationerrors | spike | runtime/builtins | class: triage-needed | Implement Isolateddeclarationerrors |
| 2999 | Implement Isolateddeclarationerrorsaugmentation | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationerrorsaugmentation |
| 3000 | Implement Isolateddeclarationerrorsclasses | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationerrorsclasses |
| 3001 | Implement Isolateddeclarationerrorsclassesexpressions | spike | runtime/builtins | class: triage-needed | Implement Isolateddeclarationerrorsclassesexpressions |
| 3003 | Implement Isolateddeclarationerrorsenums | spike | runtime/builtins | class: triage-needed | Implement Isolateddeclarationerrorsenums |
| 3004 | Implement Isolateddeclarationerrorsexpandofunctions | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationerrorsexpandofunctions |
| 3005 | Implement Isolateddeclarationerrorsexpressions | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationerrorsexpressions |
| 3006 | Implement Isolateddeclarationerrorsfunctiondeclarations | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationerrorsfunctiondeclarations |
| 3007 | Implement Isolateddeclarationerrorsobjects | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationerrorsobjects |
| 3008 | Implement Isolateddeclarationerrorsreturntypes | spike | runtime/builtins | class: triage-needed | Implement Isolateddeclarationerrorsreturntypes |
| 3009 | Implement Isolateddeclarationlazysymbols | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationlazysymbols |
| 3010 | Implement Isolateddeclarationoutfile | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationoutfile |
| 3011 | Implement Isolateddeclarationsaddundefined | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationsaddundefined |
| 3012 | Implement Isolateddeclarationsallowjs | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationsallowjs |
| 3013 | Implement Isolateddeclarationsliterals | spike | frontend/syntax | class: triage-needed | Implement Isolateddeclarationsliterals |
| 3014 | Implement Isolateddeclarationsrequiresdeclaration | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationsrequiresdeclaration |
| 3015 | Implement Isolateddeclarationsstrictbuiltiniteratorreturn | spike | runtime/builtins | class: triage-needed | Implement Isolateddeclarationsstrictbuiltiniteratorreturn |
| 3016 | Implement Isolatedmodules | spike | frontend/syntax | class: blocked | Implement Isolatedmodules |
| 3017 | Implement Isolatedmodulesambientconstenum | spike | frontend/syntax | class: triage-needed | Implement Isolatedmodulesambientconstenum |
| 3018 | Implement Isolatedmodulesconstenum | spike | frontend/syntax | class: triage-needed | Implement Isolatedmodulesconstenum |
| 3019 | Implement Isolatedmodulesdeclaration | spike | frontend/syntax | class: blocked | Implement Isolatedmodulesdeclaration |
| 3020 | Implement Isolatedmodulesdontelidereexportstar | spike | frontend/syntax | class: blocked | Implement Isolatedmodulesdontelidereexportstar |
| 3021 | Implement Isolatedmoduleses | spike | frontend/syntax | class: blocked | Implement Isolatedmoduleses |
| 3022 | Implement Isolatedmodulesexportdeclarationtype | spike | frontend/syntax | class: blocked | Implement Isolatedmodulesexportdeclarationtype |
| 3023 | Implement Isolatedmodulesexportimportuninstantiatednamespace | spike | frontend/syntax | class: blocked | Implement Isolatedmodulesexportimportuninstantiatednamespace |
| 3024 | Implement Isolatedmodulesglobalnamespacesandenums | spike | frontend/syntax | class: blocked | Implement Isolatedmodulesglobalnamespacesandenums |
| 3025 | Implement Isolatedmodulesimportconstenum | spike | frontend/syntax | class: blocked | Implement Isolatedmodulesimportconstenum |
| 3026 | Implement Isolatedmodulesimportconstenumtypeonly | spike | frontend/syntax | class: blocked | Implement Isolatedmodulesimportconstenumtypeonly |
| 3027 | Implement Isolatedmodulesimportexportelision | spike | frontend/syntax | class: blocked | Implement Isolatedmodulesimportexportelision |
| 3028 | Implement Isolatedmodulesnoemitonerror | spike | frontend/syntax | class: blocked | Implement Isolatedmodulesnoemitonerror |
| 3030 | Implement Isolatedmodulesout | spike | frontend/syntax | class: blocked | Implement Isolatedmodulesout |
| 3031 | Implement Isolatedmodulesplainfile | spike | frontend/resolver | class: blocked | Implement Isolatedmodulesplainfile |
| 3032 | Implement Isolatedmodulesreexportalias | spike | frontend/syntax | class: blocked | Implement Isolatedmodulesreexportalias |
| 3033 | Implement Isolatedmodulesreexporttype | spike | frontend/syntax | class: blocked | Implement Isolatedmodulesreexporttype |
| 3034 | Implement Isolatedmodulesshadowglobaltypenotvalue | spike | frontend/syntax | class: blocked | Implement Isolatedmodulesshadowglobaltypenotvalue |
| 3035 | Implement Isolatedmodulessketchyaliaslocalmerge | spike | frontend/syntax | class: blocked | Implement Isolatedmodulessketchyaliaslocalmerge |
| 3036 | Implement Isolatedmodulessourcemap | spike | frontend/syntax | class: blocked | Implement Isolatedmodulessourcemap |
| 3037 | Implement Isolatedmodulesspecifiedmodule | spike | frontend/syntax | class: blocked | Implement Isolatedmodulesspecifiedmodule |
| 3038 | Implement Isolatedmodulesunspecifiedmodule | spike | frontend/syntax | class: blocked | Implement Isolatedmodulesunspecifiedmodule |
| 3039 | Implement Isolatedmoduleswithdeclarationfile | spike | frontend/syntax | class: blocked | Implement Isolatedmoduleswithdeclarationfile |
| 3040 | Implement Iterabletreturntnext | spike | runtime/builtins | class: triage-needed | Implement Iterabletreturntnext |
| 3041 | Implement Iterablewithneverasunionmember | spike | frontend/resolver | class: blocked | Implement Iterablewithneverasunionmember |
| 3042 | Implement Iteratorextraparameters | spike | runtime/builtins | class: triage-needed | Implement Iteratorextraparameters |
| 3043 | Implement Iteratorsandstrictnullchecks | spike | frontend/syntax | class: blocked | Implement Iteratorsandstrictnullchecks |
| 3044 | Implement Javascriptcommonjsmodule | spike | frontend/syntax | class: blocked | Implement Javascriptcommonjsmodule |
| 3045 | Implement Javascriptdefinepropertyprototypenonconstructor | spike | frontend/resolver | class: blocked | Implement Javascriptdefinepropertyprototypenonconstructor |
| 3046 | Implement Javascriptimportdefaultbadexport | spike | frontend/syntax | class: blocked | Implement Javascriptimportdefaultbadexport |
| 3047 | Implement Javascriptthisassignmentinstaticblock | spike | frontend/syntax | class: blocked | Implement Javascriptthisassignmentinstaticblock |
| 3048 | Implement Jqueryinference | spike | frontend/syntax | class: triage-needed | Implement Jqueryinference |
| 3049 | Implement Jscheckobjectdefinethisnocrash | spike | frontend/resolver | class: blocked | Implement Jscheckobjectdefinethisnocrash |
| 3050 | Implement Jsdeclarationemitdoesnotrenameimport | spike | frontend/syntax | class: blocked | Implement Jsdeclarationemitdoesnotrenameimport |
| 3051 | Implement Jsdeclarationemitexportassignedarray | spike | frontend/syntax | class: blocked | Implement Jsdeclarationemitexportassignedarray |
| 3052 | Implement Jsdeclarationemitexportassignedfunctionwithextratypedefsmembers | spike | frontend/syntax | class: blocked | Implement Jsdeclarationemitexportassignedfunctionwithextratypedefsmembers |
| 3053 | Implement Jsdeclarationemitexportedclasswithextends | spike | frontend/syntax | class: blocked | Implement Jsdeclarationemitexportedclasswithextends |
| 3054 | Implement Jsdeclarationsglobalfileconstfunction | spike | reference/triage | class: triage-needed | Implement Jsdeclarationsglobalfileconstfunction |
| 3055 | Implement Jsdeclarationsglobalfileconstfunctionnamed | spike | frontend/syntax | class: triage-needed | Implement Jsdeclarationsglobalfileconstfunctionnamed |
| 3056 | Implement Jsdeclarationsinheritedtypes | spike | frontend/syntax | class: triage-needed | Implement Jsdeclarationsinheritedtypes |
| 3057 | Implement Jsdeclarationswithdefaultasnamespacelikemerge | spike | frontend/syntax | class: blocked | Implement Jsdeclarationswithdefaultasnamespacelikemerge |
| 3058 | Implement Jsdocdeclarationemitdoesnotusenodemodulespathwithouterror | spike | frontend/syntax | class: blocked | Implement Jsdocdeclarationemitdoesnotusenodemodulespathwithouterror |
| 3059 | Implement Jselementaccessnocontextualtypecrash | spike | frontend/resolver | class: blocked | Implement Jselementaccessnocontextualtypecrash |
| 3060 | Implement Jsemitintersectionproperty | spike | frontend/syntax | class: blocked | Implement Jsemitintersectionproperty |
| 3061 | Implement Jsenumcrossfileexport | spike | frontend/syntax | class: blocked | Implement Jsenumcrossfileexport |
| 3062 | Implement Jsenumtagonobjectfrozen | spike | frontend/syntax | class: blocked | Implement Jsenumtagonobjectfrozen |
| 3063 | Implement Jsexpandoobjectdefineproperty | spike | frontend/syntax | class: blocked | Implement Jsexpandoobjectdefineproperty |
| 3064 | Implement Jsexportassignmentnonmutablelocation | spike | frontend/syntax | class: blocked | Implement Jsexportassignmentnonmutablelocation |
| 3065 | Implement Jsexportmembermergedwithmoduleaugmentation | spike | frontend/syntax | class: blocked | Implement Jsexportmembermergedwithmoduleaugmentation |
| 3066 | Implement Jsextendsimplicitany | spike | frontend/syntax | class: triage-needed | Implement Jsextendsimplicitany |
| 3067 | Implement Jsfileclasspropertyinitalizationinobjectliteral | spike | frontend/syntax | class: blocked | Implement Jsfileclasspropertyinitalizationinobjectliteral |
| 3068 | Implement Jsfileclasspropertytype | spike | frontend/resolver | class: blocked | Implement Jsfileclasspropertytype |
| 3069 | Implement Jsfileclassselfreferencedproperty | spike | frontend/syntax | class: blocked | Implement Jsfileclassselfreferencedproperty |
| 3070 | Implement Jsfilecompilationabstractmodifier | spike | frontend/syntax | class: triage-needed | Implement Jsfilecompilationabstractmodifier |
| 3071 | Implement Jsfilecompilationawaitmodifier | spike | runtime/builtins | class: triage-needed | Implement Jsfilecompilationawaitmodifier |
| 3072 | Implement Jsfilecompilationbinddeepexportsassignment | spike | frontend/resolver | class: blocked | Implement Jsfilecompilationbinddeepexportsassignment |
| 3073 | Implement Jsfilecompilationbinderrors | spike | reference/triage | class: triage-needed | Implement Jsfilecompilationbinderrors |
| 3074 | Implement Jsfilecompilationbindmultipledefaultexports | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationbindmultipledefaultexports |
| 3075 | Implement Jsfilecompilationbindreachabilityerrors | spike | frontend/resolver | class: blocked | Implement Jsfilecompilationbindreachabilityerrors |
| 3076 | Implement Jsfilecompilationbindstrictmodeerrors | spike | runtime/builtins | class: triage-needed | Implement Jsfilecompilationbindstrictmodeerrors |
| 3077 | Implement Jsfilecompilationconstructoroverloadsyntax | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationconstructoroverloadsyntax |
| 3078 | Implement Jsfilecompilationdecoratorsyntax | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationdecoratorsyntax |
| 3079 | Implement Jsfilecompilationduplicatefunctionimplementation | spike | reference/triage | class: triage-needed | Implement Jsfilecompilationduplicatefunctionimplementation |
| 3080 | Implement Jsfilecompilationduplicatefunctionimplementationfileorderreversed | spike | reference/triage | class: triage-needed | Implement Jsfilecompilationduplicatefunctionimplementationfileorderreversed |
| 3081 | Implement Jsfilecompilationduplicatevariable | spike | reference/triage | class: triage-needed | Implement Jsfilecompilationduplicatevariable |
| 3082 | Implement Jsfilecompilationduplicatevariableerrorreported | spike | reference/triage | class: triage-needed | Implement Jsfilecompilationduplicatevariableerrorreported |
| 3083 | Implement Jsfilecompilationenumsyntax | spike | frontend/syntax | class: triage-needed | Implement Jsfilecompilationenumsyntax |
| 3084 | Implement Jsfilecompilationexportassignmentsyntax | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationexportassignmentsyntax |
| 3085 | Implement Jsfilecompilationexternalpackageerror | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationexternalpackageerror |
| 3086 | Implement Jsfilecompilationfunctionoverloadsyntax | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationfunctionoverloadsyntax |
| 3087 | Implement Jsfilecompilationheritageclausesyntaxofclass | spike | frontend/syntax | class: triage-needed | Implement Jsfilecompilationheritageclausesyntaxofclass |
| 3088 | Implement Jsfilecompilationimportequalssyntax | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationimportequalssyntax |
| 3089 | Implement Jsfilecompilationletdeclarationorder | spike | frontend/resolver | class: blocked | Implement Jsfilecompilationletdeclarationorder |
| 3090 | Implement Jsfilecompilationmethodoverloadsyntax | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationmethodoverloadsyntax |
| 3091 | Implement Jsfilecompilationmodulesyntax | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationmodulesyntax |
| 3092 | Implement Jsfilecompilationnonnullassertion | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationnonnullassertion |
| 3093 | Implement Jsfilecompilationoptionalclasselementsyntaxofclass | spike | frontend/syntax | class: triage-needed | Implement Jsfilecompilationoptionalclasselementsyntaxofclass |
| 3094 | Implement Jsfilecompilationpublicmethodsyntaxofclass | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationpublicmethodsyntaxofclass |
| 3095 | Implement Jsfilecompilationrestparamjsdocfunction | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationrestparamjsdocfunction |
| 3096 | Implement Jsfilecompilationshorthandproperty | spike | frontend/syntax | class: triage-needed | Implement Jsfilecompilationshorthandproperty |
| 3097 | Implement Jsfilecompilationtypeargumentsyntaxofcall | spike | reference/triage | class: triage-needed | Implement Jsfilecompilationtypeargumentsyntaxofcall |
| 3098 | Implement Jsfilecompilationtypeassertions | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationtypeassertions |
| 3099 | Implement Jsfilecompilationtypeparametersyntaxofclass | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationtypeparametersyntaxofclass |
| 3100 | Implement Jsfilecompilationtypeparametersyntaxofclassexpression | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationtypeparametersyntaxofclassexpression |
| 3101 | Implement Jsfileesmodulewithenumtag | spike | frontend/resolver | class: blocked | Implement Jsfileesmodulewithenumtag |
| 3102 | Implement Jsfilefunctionoverloads | spike | frontend/syntax | class: blocked | Implement Jsfilefunctionoverloads |
| 3103 | Implement Jsfileimportpreservedwhenused | spike | frontend/syntax | class: blocked | Implement Jsfileimportpreservedwhenused |
| 3104 | Implement Jsfilemethodoverloads Import Export | spike | frontend/syntax | class: blocked | Implement Jsfilemethodoverloads Import Export |
| 3105 | Implement Jsfilemethodoverloads Parser Syntax | spike | frontend/syntax | class: blocked | Implement Jsfilemethodoverloads Parser Syntax |
| 3106 | Implement Jsfilemethodoverloads Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Jsfilemethodoverloads Unknown Unsupported |
| 3107 | Implement Jsfunctionwithprototypenoerrortruncationnocrash | spike | frontend/resolver | class: blocked | Implement Jsfunctionwithprototypenoerrortruncationnocrash |
| 3108 | Implement Jsnoimplicitanynocascadingreferenceerrors | spike | frontend/syntax | class: blocked | Implement Jsnoimplicitanynocascadingreferenceerrors |
| 3109 | Implement Jspropertyassignedaftermethoddeclaration | spike | reference/triage | class: triage-needed | Implement Jspropertyassignedaftermethoddeclaration |
| 3110 | Implement Jsselfreferencingargumentsfunction | spike | frontend/syntax | class: blocked | Implement Jsselfreferencingargumentsfunction |
| 3111 | Implement Jsdocaccessenumtype | spike | frontend/syntax | class: blocked | Implement Jsdocaccessenumtype |
| 3112 | Implement Jsdocarrayobjectpromiseimplicitany | spike | frontend/resolver | class: blocked | Implement Jsdocarrayobjectpromiseimplicitany |
| 3113 | Implement Jsdocarrayobjectpromisenoimplicitany | spike | frontend/resolver | class: blocked | Implement Jsdocarrayobjectpromisenoimplicitany |
| 3114 | Implement Jsdoccallbackandtype | spike | frontend/syntax | class: blocked | Implement Jsdoccallbackandtype |
| 3115 | Implement Jsdocfunctionclasspropertiesdeclaration | spike | frontend/syntax | class: blocked | Implement Jsdocfunctionclasspropertiesdeclaration |
| 3116 | Implement Jsdocimporttypenodenamespace | spike | frontend/syntax | class: blocked | Implement Jsdocimporttypenodenamespace |
| 3117 | Implement Jsdocimporttyperesolution | spike | frontend/syntax | class: blocked | Implement Jsdocimporttyperesolution |
| 3118 | Implement Jsdocintypescript | spike | frontend/syntax | class: blocked | Implement Jsdocintypescript |
| 3119 | Implement Jsdocparamtagonpropertyinitializer | spike | frontend/syntax | class: blocked | Implement Jsdocparamtagonpropertyinitializer |
| 3120 | Implement Jsdocpropertytaginvalid | spike | frontend/syntax | class: blocked | Implement Jsdocpropertytaginvalid |
| 3121 | Implement Jsdocreferenceglobaltypeincommonjs | spike | frontend/syntax | class: blocked | Implement Jsdocreferenceglobaltypeincommonjs |
| 3122 | Implement Jsdoctypecastnotypenocrash | spike | frontend/resolver | class: blocked | Implement Jsdoctypecastnotypenocrash |
| 3123 | Implement Jsdoctypedefnocrash | spike | frontend/syntax | class: blocked | Implement Jsdoctypedefnocrash |
| 3124 | Implement Jsonfileimportcheckscallcorrectlytwice | spike | frontend/syntax | class: blocked | Implement Jsonfileimportcheckscallcorrectlytwice |
| 3125 | Implement Jsxemitwithattributes | spike | reference/triage | class: triage-needed | Implement Jsxemitwithattributes |
| 3126 | Implement Jsxfactoryandreactnamespace | spike | reference/triage | class: triage-needed | Implement Jsxfactoryandreactnamespace |
| 3127 | Implement Jsxfactoryidentifier | spike | reference/triage | class: triage-needed | Implement Jsxfactoryidentifier |
| 3128 | Implement Jsxfactoryidentifierasparameter | spike | frontend/syntax | class: blocked | Implement Jsxfactoryidentifierasparameter |
| 3129 | Implement Jsxfactoryidentifierwithabsentparameter | spike | frontend/syntax | class: blocked | Implement Jsxfactoryidentifierwithabsentparameter |
| 3130 | Implement Jsxfactorymissingerrorinsideaclass | spike | reference/triage | class: triage-needed | Implement Jsxfactorymissingerrorinsideaclass |
| 3132 | Implement Jsxfactoryqualifiedname | spike | reference/triage | class: triage-needed | Implement Jsxfactoryqualifiedname |
| 3133 | Implement Jsxfactoryqualifiednameresolutionerror | spike | frontend/syntax | class: blocked | Implement Jsxfactoryqualifiednameresolutionerror |
| 3134 | Implement Jsxfactoryqualifiednamewithes | spike | reference/triage | class: triage-needed | Implement Jsxfactoryqualifiednamewithes |
| 3135 | Implement Jsxpreservewithjsinput | spike | reference/triage | class: triage-needed | Implement Jsxpreservewithjsinput |
| 3136 | Implement Jsxruntimepragma | spike | reference/triage | class: triage-needed | Implement Jsxruntimepragma |
| 3137 | Implement Jsxspreadtag | spike | reference/triage | class: triage-needed | Implement Jsxspreadtag |
| 3138 | Implement Keepimportsindts | spike | frontend/syntax | class: blocked | Implement Keepimportsindts |
| 3139 | Implement Keyremappingkeyofresult | spike | frontend/semantics | class: blocked | Implement Keyremappingkeyofresult |
| 3140 | Implement Keyofdoesntcontainsymbols | spike | frontend/semantics | class: blocked | Implement Keyofdoesntcontainsymbols |
| 3141 | Implement Keyofgenericextendingclassdoublelayer | spike | frontend/semantics | class: blocked | Implement Keyofgenericextendingclassdoublelayer |
| 3142 | Implement Keyofisliteralcontexualtype | spike | frontend/resolver | class: blocked | Implement Keyofisliteralcontexualtype |
| 3143 | Implement Keyofmoduleobjecthascorrectkeys | spike | frontend/syntax | class: blocked | Implement Keyofmoduleobjecthascorrectkeys |
| 3144 | Implement Keyofobjectwithglobalsymbolincluded | spike | frontend/semantics | class: blocked | Implement Keyofobjectwithglobalsymbolincluded |
| 3145 | Implement Keywordexpressioninternalcomments | spike | frontend/syntax | class: blocked | Implement Keywordexpressioninternalcomments |
| 3146 | Implement Keywordfield | spike | frontend/syntax | class: triage-needed | Implement Keywordfield |
| 3147 | Implement Knockout | spike | frontend/syntax | class: blocked | Implement Knockout |
| 3148 | Implement Lambdaargcrash | spike | frontend/syntax | class: triage-needed | Implement Lambdaargcrash |
| 3149 | Implement Lambdaparamtypes | spike | frontend/resolver | class: blocked | Implement Lambdaparamtypes |
| 3150 | Implement Lambdaparameterwithtupleargshascorrectassignability | spike | frontend/syntax | class: blocked | Implement Lambdaparameterwithtupleargshascorrectassignability |
| 3151 | Implement Lambdapropself | spike | frontend/syntax | class: triage-needed | Implement Lambdapropself |
| 3152 | Implement Largetupletypes | spike | frontend/syntax | class: triage-needed | Implement Largetupletypes |
| 3153 | Implement Lastpropertyinliteralwins | spike | frontend/syntax | class: blocked | Implement Lastpropertyinliteralwins |
| 3154 | Implement Lateboundassignmentcandidatejs | spike | frontend/syntax | class: blocked | Implement Lateboundassignmentcandidatejs |
| 3155 | Implement Lateboundconstrainttypecheckscorrectly | spike | frontend/resolver | class: blocked | Implement Lateboundconstrainttypecheckscorrectly |
| 3156 | Implement Latebounddestructuringimplicitanyerror | spike | frontend/syntax | class: blocked | Implement Latebounddestructuringimplicitanyerror |
| 3157 | Implement Lateboundfunctionmemberassignmentdeclarations | spike | frontend/syntax | class: blocked | Implement Lateboundfunctionmemberassignmentdeclarations |
| 3158 | Implement Lateboundmethodnameassigmentjs | spike | frontend/syntax | class: blocked | Implement Lateboundmethodnameassigmentjs |
| 3159 | Implement Letandvarredeclaration | spike | frontend/syntax | class: triage-needed | Implement Letandvarredeclaration |
| 3160 | Implement Letasidentifier | spike | frontend/syntax | class: triage-needed | Implement Letasidentifier |
| 3161 | Implement Letasidentifierinstrictmode | spike | frontend/syntax | class: triage-needed | Implement Letasidentifierinstrictmode |
| 3162 | Implement Letconstincaseclauses | spike | frontend/syntax | class: triage-needed | Implement Letconstincaseclauses |
| 3163 | Implement Letconstmatchingparameternames | spike | frontend/resolver | class: blocked | Implement Letconstmatchingparameternames |
| 3164 | Implement Letdeclarations Duplicate Local | spike | reference/triage | class: triage-needed | Implement Letdeclarations Duplicate Local |
| 3165 | Implement Letdeclarations Import Export | spike | frontend/syntax | class: blocked | Implement Letdeclarations Import Export |
| 3166 | Implement Letdeclarations Name Resolution | spike | frontend/resolver | class: blocked | Implement Letdeclarations Name Resolution |
| 3167 | Implement Letdeclarations Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Letdeclarations Parser Syntax |
| 3168 | Implement Letdeclarations Scope Analysis | spike | frontend/resolver | class: blocked | Implement Letdeclarations Scope Analysis |
| 3169 | Implement Letinconstdeclarations | spike | frontend/syntax | class: triage-needed | Implement Letinconstdeclarations |
| 3170 | Implement Letinletconstdeclofforofandforin | spike | frontend/syntax | class: triage-needed | Implement Letinletconstdeclofforofandforin |
| 3171 | Implement Letinletdeclarations | spike | frontend/syntax | class: triage-needed | Implement Letinletdeclarations |
| 3172 | Implement Letinvardeclofforin | spike | frontend/syntax | class: triage-needed | Implement Letinvardeclofforin |
| 3173 | Implement Letinvardeclofforof | spike | frontend/syntax | class: triage-needed | Implement Letinvardeclofforof |
| 3174 | Implement Letkeepnamesoftoplevelitems | spike | frontend/syntax | class: blocked | Implement Letkeepnamesoftoplevelitems |
| 3175 | Implement Libmembers | spike | frontend/syntax | class: blocked | Implement Libmembers |
| 3176 | Implement Libtypescriptoverridesimple | spike | runtime/builtins | class: triage-needed | Implement Libtypescriptoverridesimple |
| 3177 | Implement Libtypescriptoverridesimpleconfig | spike | runtime/builtins | class: triage-needed | Implement Libtypescriptoverridesimpleconfig |
| 3178 | Implement Libtypescriptsubfileresolving | spike | runtime/builtins | class: triage-needed | Implement Libtypescriptsubfileresolving |
| 3179 | Implement Libtypescriptsubfileresolvingconfig | spike | runtime/builtins | class: triage-needed | Implement Libtypescriptsubfileresolvingconfig |
| 3180 | Implement Library Method Call | spike | frontend/syntax | class: blocked | Implement Library Method Call |
| 3181 | Implement Library Object Literal | spike | frontend/syntax | class: blocked | Implement Library Object Literal |
| 3182 | Implement Lift | spike | frontend/syntax | class: triage-needed | Implement Lift |
| 3183 | Implement Limitdeepinstantiations | spike | frontend/syntax | class: triage-needed | Implement Limitdeepinstantiations |
| 3184 | Implement Listfailure | spike | frontend/syntax | class: blocked | Implement Listfailure |
| 3185 | Implement Literalfreshnesspropagationonnarrowing | spike | frontend/syntax | class: blocked | Implement Literalfreshnesspropagationonnarrowing |
| 3186 | Implement Literaltypenameassertionnottriggered | spike | frontend/syntax | class: blocked | Implement Literaltypenameassertionnottriggered |
| 3187 | Implement Literalwideningwithcompoundlikeassignments | spike | frontend/syntax | class: triage-needed | Implement Literalwideningwithcompoundlikeassignments |
| 3188 | Implement Literals | spike | frontend/syntax | class: triage-needed | Implement Literals |
| 3189 | Implement Literalsincomputedproperties | spike | frontend/syntax | class: triage-needed | Implement Literalsincomputedproperties |
| 3190 | Implement Localaliasexportassignment | spike | frontend/syntax | class: blocked | Implement Localaliasexportassignment |
| 3191 | Implement Localclassesinloop | spike | frontend/resolver | class: blocked | Implement Localclassesinloop |
| 3192 | Implement Localimportnamevsglobalname | spike | frontend/syntax | class: blocked | Implement Localimportnamevsglobalname |
| 3193 | Implement Localrequirefunction | spike | frontend/syntax | class: blocked | Implement Localrequirefunction |
| 3194 | Implement Localtypeparameterinferencepriority | spike | frontend/semantics | class: blocked | Implement Localtypeparameterinferencepriority |
| 3195 | Implement Logicalnotexpression | spike | frontend/resolver | class: blocked | Implement Logicalnotexpression |
| 3196 | Implement Longobjectinstantiationchain Name Resolution | spike | frontend/resolver | class: blocked | Implement Longobjectinstantiationchain Name Resolution |
| 3197 | Implement Longobjectinstantiationchain Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Longobjectinstantiationchain Parser Syntax |
| 3198 | Implement M | spike | frontend/syntax | class: triage-needed | Implement M |
| 3199 | Implement Manycompilererrorsinthetwofiles | spike | runtime/builtins | class: triage-needed | Implement Manycompilererrorsinthetwofiles |
| 3200 | Implement Manyconstexports | spike | frontend/syntax | class: blocked | Implement Manyconstexports |
| 3201 | Implement Mapconstructor | spike | frontend/syntax | class: triage-needed | Implement Mapconstructor |
| 3202 | Implement Mapconstructoronreadonlytuple | spike | frontend/syntax | class: blocked | Implement Mapconstructoronreadonlytuple |
| 3203 | Implement Mapgroupby | spike | frontend/syntax | class: triage-needed | Implement Mapgroupby |
| 3204 | Implement Mapontupletypes | spike | frontend/syntax | class: blocked | Implement Mapontupletypes |
| 3205 | Implement Mapupsert | spike | frontend/syntax | class: triage-needed | Implement Mapupsert |
| 3206 | Implement Mappedtotoindexsignatureinference | spike | frontend/syntax | class: triage-needed | Implement Mappedtotoindexsignatureinference |
| 3207 | Implement Mappedtypecontextualtypesapplied | spike | frontend/resolver | class: blocked | Implement Mappedtypecontextualtypesapplied |
| 3208 | Implement Mappedtypegenericindexedaccess | spike | frontend/semantics | class: blocked | Implement Mappedtypegenericindexedaccess |
| 3209 | Implement Mappedtypegenericinstantiationpreserveshomomorphism | spike | frontend/semantics | class: blocked | Implement Mappedtypegenericinstantiationpreserveshomomorphism |
| 3210 | Implement Mappedtypegenericinstantiationpreservesinlineform | spike | frontend/semantics | class: blocked | Implement Mappedtypegenericinstantiationpreservesinlineform |
| 3211 | Implement Mappedtypegenericwithknownkeys | spike | frontend/semantics | class: blocked | Implement Mappedtypegenericwithknownkeys |
| 3212 | Implement Mappedtypeindexedaccessconstraint | spike | frontend/syntax | class: triage-needed | Implement Mappedtypeindexedaccessconstraint |
| 3213 | Implement Mappedtypeinferencealiassubstitution | spike | frontend/syntax | class: blocked | Implement Mappedtypeinferencealiassubstitution |
| 3214 | Implement Mappedtypeinferencecircularity | spike | frontend/resolver | class: blocked | Implement Mappedtypeinferencecircularity |
| 3215 | Implement Mappedtypeinferencefromapparenttype | spike | frontend/resolver | class: blocked | Implement Mappedtypeinferencefromapparenttype |
| 3216 | Implement Mappedtypeinferencetomappedtype | spike | frontend/semantics | class: blocked | Implement Mappedtypeinferencetomappedtype |
| 3217 | Implement Mappedtypemultiinference | spike | frontend/semantics | class: blocked | Implement Mappedtypemultiinference |
| 3218 | Implement Mappedtypenestedgenericinstantiation | spike | frontend/resolver | class: blocked | Implement Mappedtypenestedgenericinstantiation |
| 3219 | Implement Mappedtypenotmistakenlyhomomorphic | spike | frontend/syntax | class: blocked | Implement Mappedtypenotmistakenlyhomomorphic |
| 3220 | Implement Mappedtypepartialconstraints | spike | frontend/syntax | class: blocked | Implement Mappedtypepartialconstraints |
| 3221 | Implement Mappedtypepartialnonhomomorphicbaseconstraint | spike | frontend/syntax | class: blocked | Implement Mappedtypepartialnonhomomorphicbaseconstraint |
| 3222 | Implement Mappedtyperecursiveinference Parser Syntax | spike | frontend/syntax | class: blocked | Implement Mappedtyperecursiveinference Parser Syntax |
| 3223 | Implement Mappedtyperecursiveinference Type System | spike | frontend/semantics | class: blocked | Implement Mappedtyperecursiveinference Type System |
| 3224 | Implement Mappedtypetupleconstraintassignability | spike | frontend/syntax | class: blocked | Implement Mappedtypetupleconstraintassignability |
| 3225 | Implement Mappedtypeunionconstraintupletreatedasarraylike | spike | frontend/syntax | class: triage-needed | Implement Mappedtypeunionconstraintupletreatedasarraylike |
| 3226 | Implement Mappedtypeunionconstraintinferences | spike | frontend/syntax | class: blocked | Implement Mappedtypeunionconstraintinferences |
| 3227 | Implement Mappedtypewithasclauseandlateboundproperty Name Resolution | spike | frontend/resolver | class: blocked | Implement Mappedtypewithasclauseandlateboundproperty Name Resolution |
| 3228 | Implement Mappedtypewithasclauseandlateboundproperty Parser Syntax | spike | frontend/syntax | class: blocked | Implement Mappedtypewithasclauseandlateboundproperty Parser Syntax |
| 3229 | Implement Mappedtypewithcombinedtypemappers | spike | frontend/syntax | class: blocked | Implement Mappedtypewithcombinedtypemappers |
| 3230 | Implement Mappedtypewithnameclauseappliedtoarraytype | spike | frontend/resolver | class: blocked | Implement Mappedtypewithnameclauseappliedtoarraytype |
| 3231 | Implement Matchreturntypeinallbranches | spike | frontend/syntax | class: triage-needed | Implement Matchreturntypeinallbranches |
| 3232 | Implement Maxconstraints | spike | frontend/syntax | class: triage-needed | Implement Maxconstraints |
| 3233 | Implement Maxnodemodulejsdepthdefaultstozero | spike | frontend/syntax | class: blocked | Implement Maxnodemodulejsdepthdefaultstozero |
| 3234 | Implement Maximum | spike | frontend/resolver | class: blocked | Implement Maximum |
| 3235 | Implement Memberaccessmustusemoduleinstances | spike | frontend/syntax | class: blocked | Implement Memberaccessmustusemoduleinstances |
| 3236 | Implement Memberoverride | spike | frontend/syntax | class: triage-needed | Implement Memberoverride |
| 3237 | Implement Memberscope | spike | frontend/syntax | class: blocked | Implement Memberscope |
| 3238 | Implement Membervariabledeclarations | spike | frontend/syntax | class: triage-needed | Implement Membervariabledeclarations |
| 3239 | Implement Mergemultipleinterfacesreexported | spike | frontend/syntax | class: blocked | Implement Mergemultipleinterfacesreexported |
| 3240 | Implement Mergesymbolreexportinterface | spike | frontend/syntax | class: blocked | Implement Mergesymbolreexportinterface |
| 3241 | Implement Mergesymbolreexportedtypealiasinstantiation | spike | frontend/syntax | class: blocked | Implement Mergesymbolreexportedtypealiasinstantiation |
| 3242 | Implement Mergesymbolrexportfunction | spike | frontend/syntax | class: blocked | Implement Mergesymbolrexportfunction |
| 3243 | Implement Mergewithimportednamespace | spike | frontend/syntax | class: blocked | Implement Mergewithimportednamespace |
| 3244 | Implement Mergewithimportedtype | spike | frontend/syntax | class: blocked | Implement Mergewithimportedtype |
| 3245 | Implement Mergedclassnamespacerecordcast | spike | frontend/syntax | class: triage-needed | Implement Mergedclassnamespacerecordcast |
| 3246 | Implement Mergedclasswithnamespaceprototype | spike | frontend/syntax | class: blocked | Implement Mergedclasswithnamespaceprototype |
| 3247 | Implement Mergeddeclarationexports | spike | frontend/syntax | class: blocked | Implement Mergeddeclarationexports |
| 3248 | Implement Mergeddeclarations Import Export | spike | frontend/syntax | class: blocked | Implement Mergeddeclarations Import Export |
| 3249 | Implement Mergeddeclarations Parser Syntax | spike | frontend/syntax | class: blocked | Implement Mergeddeclarations Parser Syntax |
| 3250 | Implement Mergedenumdeclarationcodegen | spike | frontend/syntax | class: blocked | Implement Mergedenumdeclarationcodegen |
| 3251 | Implement Mergedinstantiationassignment | spike | frontend/syntax | class: triage-needed | Implement Mergedinstantiationassignment |
| 3252 | Implement Mergedinterfacefrommultiplefiles | spike | frontend/syntax | class: blocked | Implement Mergedinterfacefrommultiplefiles |
| 3253 | Implement Mergedmoduledeclarationcodegen | spike | frontend/syntax | class: blocked | Implement Mergedmoduledeclarationcodegen |
| 3254 | Implement Mergedmoduledeclarationwithsharedexportedvar | spike | frontend/syntax | class: blocked | Implement Mergedmoduledeclarationwithsharedexportedvar |
| 3255 | Implement Metadataimporttype | spike | frontend/syntax | class: blocked | Implement Metadataimporttype |
| 3256 | Implement Metadataofclassfromalias | spike | frontend/syntax | class: blocked | Implement Metadataofclassfromalias |
| 3257 | Implement Metadataofclassfrommodule | spike | frontend/syntax | class: blocked | Implement Metadataofclassfrommodule |
| 3258 | Implement Metadataofeventalias | spike | frontend/syntax | class: blocked | Implement Metadataofeventalias |
| 3259 | Implement Metadataofstringliteral | spike | frontend/syntax | class: triage-needed | Implement Metadataofstringliteral |
| 3260 | Implement Metadataofunion | spike | frontend/syntax | class: triage-needed | Implement Metadataofunion |
| 3261 | Implement Metadataofunionwithnull | spike | frontend/syntax | class: triage-needed | Implement Metadataofunionwithnull |
| 3262 | Implement Metadatareferencedwithinfilteredunion | spike | frontend/syntax | class: triage-needed | Implement Metadatareferencedwithinfilteredunion |
| 3263 | Implement Methodchainerror | spike | frontend/syntax | class: blocked | Implement Methodchainerror |
| 3264 | Implement Methodcontaininglocalfunction | spike | frontend/syntax | class: triage-needed | Implement Methodcontaininglocalfunction |
| 3265 | Implement Methodsignaturedeclarationemit | spike | frontend/syntax | class: blocked | Implement Methodsignaturedeclarationemit |
| 3266 | Implement Mismatchedexplicittypeparameterandargumenttype | spike | frontend/syntax | class: blocked | Implement Mismatchedexplicittypeparameterandargumenttype |
| 3267 | Implement Mismatchedgenericarguments | spike | frontend/semantics | class: blocked | Implement Mismatchedgenericarguments |
| 3268 | Implement Missingargument | spike | frontend/syntax | class: triage-needed | Implement Missingargument |
| 3269 | Implement Missingclosebrace | spike | frontend/syntax | class: triage-needed | Implement Missingclosebrace |
| 3270 | Implement Missingclosebraceinobjectliteral | spike | frontend/syntax | class: blocked | Implement Missingclosebraceinobjectliteral |
| 3271 | Implement Missingclosebracketinarray | spike | frontend/syntax | class: triage-needed | Implement Missingclosebracketinarray |
| 3272 | Implement Missingcloseparenstatements | spike | frontend/syntax | class: triage-needed | Implement Missingcloseparenstatements |
| 3273 | Implement Missingcommaintemplatestringsarray | spike | frontend/syntax | class: blocked | Implement Missingcommaintemplatestringsarray |
| 3274 | Implement Missingdiscriminants | spike | frontend/syntax | class: triage-needed | Implement Missingdiscriminants |
| 3275 | Implement Missingdomelements | spike | frontend/resolver | class: blocked | Implement Missingdomelements |
| 3276 | Implement Missingfunctionimplementation | spike | frontend/syntax | class: blocked | Implement Missingfunctionimplementation |
| 3277 | Implement Missingimportaftermoduleimport | spike | frontend/syntax | class: blocked | Implement Missingimportaftermoduleimport |
| 3278 | Implement Missingmembererrorhasshortpath | spike | frontend/syntax | class: blocked | Implement Missingmembererrorhasshortpath |
| 3279 | Implement Missingpropertiesofclassexpression | spike | frontend/syntax | class: triage-needed | Implement Missingpropertiesofclassexpression |
| 3280 | Implement Missingreturnstatement | spike | frontend/syntax | class: blocked | Implement Missingreturnstatement |
| 3281 | Implement Missingself | spike | frontend/syntax | class: triage-needed | Implement Missingself |
| 3282 | Implement Missingsemicoloninmodulespecifier | spike | frontend/syntax | class: blocked | Implement Missingsemicoloninmodulespecifier |
| 3283 | Implement Missingtypearguments Arguments Object | spike | frontend/syntax | class: blocked | Implement Missingtypearguments Arguments Object |
| 3284 | Implement Missingtypearguments Import Export | spike | frontend/syntax | class: blocked | Implement Missingtypearguments Import Export |
| 3285 | Implement Misspelledjsdoctypedeftags | spike | frontend/resolver | class: blocked | Implement Misspelledjsdoctypedeftags |
| 3286 | Implement Misspellednewmetaproperty | spike | frontend/syntax | class: triage-needed | Implement Misspellednewmetaproperty |
| 3287 | Implement Mixedexports | spike | frontend/syntax | class: blocked | Implement Mixedexports |
| 3288 | Implement Mixedtypeenumcomparison | spike | frontend/syntax | class: blocked | Implement Mixedtypeenumcomparison |
| 3289 | Implement Mixinintersectionisvalidbasetype | spike | frontend/syntax | class: triage-needed | Implement Mixinintersectionisvalidbasetype |
| 3290 | Implement Mixinprivateandprotected | spike | frontend/syntax | class: blocked | Implement Mixinprivateandprotected |
| 3291 | Implement Mixingapparenttypeoverrides | spike | frontend/syntax | class: triage-needed | Implement Mixingapparenttypeoverrides |
| 3292 | Implement Mixingfunctionandambientmodule | spike | frontend/syntax | class: blocked | Implement Mixingfunctionandambientmodule |
| 3293 | Implement Mixingstaticandinstanceoverloads | spike | frontend/syntax | class: blocked | Implement Mixingstaticandinstanceoverloads |
| 3294 | Implement Modfunctioncrash | spike | frontend/syntax | class: blocked | Implement Modfunctioncrash |
| 3295 | Implement Modkeyword | spike | frontend/syntax | class: triage-needed | Implement Modkeyword |
| 3296 | Implement Modifieronparameter | spike | frontend/syntax | class: triage-needed | Implement Modifieronparameter |
| 3297 | Implement Modifierparencast | spike | frontend/syntax | class: blocked | Implement Modifierparencast |
| 3298 | Implement Modifiersinobjectliterals | spike | frontend/syntax | class: blocked | Implement Modifiersinobjectliterals |
| 3299 | Implement Modularizelibrary Name Resolution | spike | frontend/resolver | class: blocked | Implement Modularizelibrary Name Resolution |
| 3300 | Implement Modularizelibrary Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Modularizelibrary Unknown Unsupported |
| 3301 | Implement Module | spike | frontend/syntax | class: blocked | Implement Module |
| 3302 | Implement Modulealiasasfunctionargument | spike | frontend/syntax | class: blocked | Implement Modulealiasasfunctionargument |
| 3303 | Implement Modulealiasinterface | spike | frontend/syntax | class: blocked | Implement Modulealiasinterface |
| 3304 | Implement Moduleandinterfacesharingname | spike | frontend/syntax | class: blocked | Implement Moduleandinterfacesharingname |
| 3445 | Implement Narrowcommaoperatornestedwithinlhs | spike | frontend/syntax | class: blocked | Implement Narrowcommaoperatornestedwithinlhs |
| 3446 | Implement Narrowrefinedconstlikeparameterbindingelementnameininnerscope | spike | frontend/resolver | class: blocked | Implement Narrowrefinedconstlikeparameterbindingelementnameininnerscope |
| 3447 | Implement Narrowswitchoptionalchaincontainmentevolvingarraynocrash | spike | frontend/syntax | class: blocked | Implement Narrowswitchoptionalchaincontainmentevolvingarraynocrash |
| 3448 | Implement Narrowtypebyinstanceof | spike | frontend/syntax | class: blocked | Implement Narrowtypebyinstanceof |
| 3449 | Implement Narrowunknownbytypepredicate | spike | frontend/resolver | class: blocked | Implement Narrowunknownbytypepredicate |
| 3450 | Implement Narrowedconstinmethod | spike | frontend/syntax | class: triage-needed | Implement Narrowedconstinmethod |
| 3451 | Implement Narrowedimports | spike | frontend/syntax | class: blocked | Implement Narrowedimports |
| 3452 | Implement Narrowingassignmentreadonlyrespectsassertion | spike | frontend/syntax | class: triage-needed | Implement Narrowingassignmentreadonlyrespectsassertion |
| 3453 | Implement Narrowingbytypeofinswitch | spike | frontend/syntax | class: blocked | Implement Narrowingbytypeofinswitch |
| 3454 | Implement Narrowingconstrainedtypeparameter | spike | frontend/syntax | class: blocked | Implement Narrowingconstrainedtypeparameter |
| 3455 | Implement Narrowingdestructuring | spike | reference/triage | class: triage-needed | Implement Narrowingdestructuring |
| 3456 | Implement Narrowingincaseclauseaftercaseclausewithreturn | spike | frontend/syntax | class: blocked | Implement Narrowingincaseclauseaftercaseclausewithreturn |
| 3457 | Implement Narrowingmutualsubtypes | spike | frontend/syntax | class: blocked | Implement Narrowingmutualsubtypes |
| 3458 | Implement Narrowingnoinfer | spike | frontend/semantics | class: blocked | Implement Narrowingnoinfer |
| 3459 | Implement Narrowingofdottednames | spike | frontend/syntax | class: blocked | Implement Narrowingofdottednames |
| 3460 | Implement Narrowingofqualifiednames | spike | frontend/syntax | class: blocked | Implement Narrowingofqualifiednames |
| 3461 | Implement Narrowingpastlastassignment | spike | frontend/syntax | class: blocked | Implement Narrowingpastlastassignment |
| 3462 | Implement Narrowingpastlastassignmentinmodule | spike | frontend/syntax | class: blocked | Implement Narrowingpastlastassignmentinmodule |
| 3463 | Implement Narrowingplainjsnocrash | spike | frontend/syntax | class: blocked | Implement Narrowingplainjsnocrash |
| 3464 | Implement Narrowingrestgenericcall | spike | frontend/semantics | class: blocked | Implement Narrowingrestgenericcall |
| 3465 | Implement Narrowingtruthyobject | spike | frontend/syntax | class: blocked | Implement Narrowingtruthyobject |
| 3466 | Implement Narrowingtypeofparenthesized | spike | frontend/resolver | class: blocked | Implement Narrowingtypeofparenthesized |
| 3467 | Implement Narrowingtypeofundefined Name Resolution | spike | frontend/resolver | class: blocked | Implement Narrowingtypeofundefined Name Resolution |
| 3468 | Implement Narrowingtypeofundefined Parser Syntax | spike | frontend/syntax | class: blocked | Implement Narrowingtypeofundefined Parser Syntax |
| 3469 | Implement Narrowinguniontounion | spike | frontend/syntax | class: blocked | Implement Narrowinguniontounion |
| 3470 | Implement Narrowingunionwithbang | spike | frontend/syntax | class: blocked | Implement Narrowingunionwithbang |
| 3471 | Implement Narrowingwithnonnullexpression | spike | frontend/syntax | class: blocked | Implement Narrowingwithnonnullexpression |
| 3472 | Implement Nativetoboxedtypes | spike | frontend/syntax | class: blocked | Implement Nativetoboxedtypes |
| 3473 | Implement Nearbyidenticalgenericlambdasassignable | spike | frontend/semantics | class: blocked | Implement Nearbyidenticalgenericlambdasassignable |
| 3474 | Implement Negativezero | spike | frontend/syntax | class: triage-needed | Implement Negativezero |
| 3475 | Implement Nestedblockscopedbindings | spike | frontend/resolver | class: blocked | Implement Nestedblockscopedbindings |
| 3476 | Implement Nestedcallbackerrornotflattened | spike | frontend/resolver | class: blocked | Implement Nestedcallbackerrornotflattened |
| 3477 | Implement Nestedexcesspropertychecking | spike | frontend/syntax | class: blocked | Implement Nestedexcesspropertychecking |
| 3478 | Implement Nestedfreshliteral | spike | frontend/syntax | class: triage-needed | Implement Nestedfreshliteral |
| 3479 | Implement Nestedgenericspreadinference | spike | frontend/semantics | class: blocked | Implement Nestedgenericspreadinference |
| 3480 | Implement Nestedglobalnamespaceinclass | spike | frontend/syntax | class: triage-needed | Implement Nestedglobalnamespaceinclass |
| 3481 | Implement Nestedindexer | spike | frontend/syntax | class: triage-needed | Implement Nestedindexer |
| 3482 | Implement Nestedloopwithonlyinnerletcaptured | spike | frontend/syntax | class: triage-needed | Implement Nestedloopwithonlyinnerletcaptured |
| 3483 | Implement Nestedloops | spike | frontend/syntax | class: blocked | Implement Nestedloops |
| 3484 | Implement Nestedmoduleprivateaccess | spike | frontend/syntax | class: blocked | Implement Nestedmoduleprivateaccess |
| 3485 | Implement Nestedobjectrest | spike | frontend/syntax | class: blocked | Implement Nestedobjectrest |
| 3486 | Implement Nestedrecursivelambda | spike | frontend/syntax | class: blocked | Implement Nestedrecursivelambda |
| 3487 | Implement Nestedredeclarationines | spike | frontend/syntax | class: blocked | Implement Nestedredeclarationines |
| 3488 | Implement Nestedself | spike | frontend/syntax | class: blocked | Implement Nestedself |
| 3489 | Implement Nestedsupercallemit | spike | frontend/resolver | class: blocked | Implement Nestedsupercallemit |
| 3490 | Implement Nestedthiscontainer | spike | reference/triage | class: triage-needed | Implement Nestedthiscontainer |
| 3491 | Implement Nestedtypevariableinfersliteral | spike | frontend/syntax | class: triage-needed | Implement Nestedtypevariableinfersliteral |
| 3492 | Implement Nestedunaryexpressionhang | spike | frontend/syntax | class: triage-needed | Implement Nestedunaryexpressionhang |
| 3493 | Implement Neverasdiscriminanttype | spike | frontend/syntax | class: blocked | Implement Neverasdiscriminanttype |
| 3494 | Implement Newabstractinstance Name Resolution | spike | frontend/resolver | class: blocked | Implement Newabstractinstance Name Resolution |
| 3495 | Implement Newabstractinstance Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Newabstractinstance Parser Syntax |
| 3496 | Implement Newarrays | spike | frontend/syntax | class: blocked | Implement Newarrays |
| 3497 | Implement Newexpressionwithcast | spike | frontend/syntax | class: triage-needed | Implement Newexpressionwithcast |
| 3498 | Implement Newexpressionwithtypeparameterconstrainedtooutertypeparameter | spike | frontend/syntax | class: blocked | Implement Newexpressionwithtypeparameterconstrainedtooutertypeparameter |
| 3499 | Implement Newfunctionimplicitany | spike | frontend/syntax | class: blocked | Implement Newfunctionimplicitany |
| 3500 | Implement Newlexicalenvironmentforconvertedloop | spike | frontend/syntax | class: triage-needed | Implement Newlexicalenvironmentforconvertedloop |
| 3501 | Implement Newmap | spike | frontend/resolver | class: blocked | Implement Newmap |
| 3502 | Implement Newmissingidentifier | spike | frontend/syntax | class: triage-needed | Implement Newmissingidentifier |
| 3503 | Implement Newnamesinglobalaugmentations | spike | frontend/syntax | class: blocked | Implement Newnamesinglobalaugmentations |
| 3504 | Implement Newnonreferencetype | spike | frontend/resolver | class: blocked | Implement Newnonreferencetype |
| 3505 | Implement Newoninstancesymbol | spike | frontend/syntax | class: blocked | Implement Newoninstancesymbol |
| 3506 | Implement Newoperator | spike | frontend/syntax | class: triage-needed | Implement Newoperator |
| 3507 | Implement Noasconstnamelookup | spike | frontend/syntax | class: blocked | Implement Noasconstnamelookup |
| 3508 | Implement Nobundledemitfromnodemodules | spike | frontend/syntax | class: blocked | Implement Nobundledemitfromnodemodules |
| 3509 | Implement Nocheckdoesnotreporterror | spike | frontend/syntax | class: blocked | Implement Nocheckdoesnotreporterror |
| 3510 | Implement Nochecknoemit | spike | frontend/syntax | class: blocked | Implement Nochecknoemit |
| 3511 | Implement Nocheckrequiresemitdeclarationonly | spike | frontend/syntax | class: blocked | Implement Nocheckrequiresemitdeclarationonly |
| 3512 | Implement Nocirculardefinitiononexportofprivateinmergednamespace | spike | frontend/syntax | class: blocked | Implement Nocirculardefinitiononexportofprivateinmergednamespace |
| 3513 | Implement Nocircularityselfreferentialgetter | spike | frontend/syntax | class: blocked | Implement Nocircularityselfreferentialgetter |
| 3514 | Implement Nocollisionthisexpressionandclassinglobal | spike | frontend/resolver | class: blocked | Implement Nocollisionthisexpressionandclassinglobal |
| 3515 | Implement Nocollisionthisexpressionandlocalvarinaccessors | spike | frontend/syntax | class: blocked | Implement Nocollisionthisexpressionandlocalvarinaccessors |
| 3516 | Implement Nocollisionthisexpressionandlocalvarinconstructor | spike | frontend/syntax | class: triage-needed | Implement Nocollisionthisexpressionandlocalvarinconstructor |
| 3517 | Implement Nocollisionthisexpressionandlocalvarinfunction | spike | frontend/syntax | class: blocked | Implement Nocollisionthisexpressionandlocalvarinfunction |
| 3518 | Implement Nocollisionthisexpressionandlocalvarinlambda | spike | frontend/syntax | class: triage-needed | Implement Nocollisionthisexpressionandlocalvarinlambda |
| 3519 | Implement Nocollisionthisexpressionandlocalvarinmethod | spike | frontend/syntax | class: triage-needed | Implement Nocollisionthisexpressionandlocalvarinmethod |
| 3520 | Implement Nocollisionthisexpressionandlocalvarinproperty | spike | frontend/syntax | class: blocked | Implement Nocollisionthisexpressionandlocalvarinproperty |
| 3521 | Implement Nocollisionthisexpressioninfunctionandvaringlobal | spike | frontend/syntax | class: blocked | Implement Nocollisionthisexpressioninfunctionandvaringlobal |
| 3522 | Implement Noconstraintinreturntype | spike | frontend/syntax | class: blocked | Implement Noconstraintinreturntype |
| 3523 | Implement Nocrashonimportshadowing | spike | reference/triage | class: triage-needed | Implement Nocrashonimportshadowing |
| 3524 | Implement Nocrashonmixin | spike | frontend/syntax | class: blocked | Implement Nocrashonmixin |
| 3525 | Implement Nocrashonnolib | spike | frontend/syntax | class: blocked | Implement Nocrashonnolib |
| 3526 | Implement Nocrashonthistypeusage | spike | frontend/syntax | class: blocked | Implement Nocrashonthistypeusage |
| 3527 | Implement Nocrashumdmergedwithglobalvalue | spike | frontend/syntax | class: blocked | Implement Nocrashumdmergedwithglobalvalue |
| 3528 | Implement Nocrashwithverbatimmodulesyntaxandimportsnotusedasvalues | spike | frontend/syntax | class: blocked | Implement Nocrashwithverbatimmodulesyntaxandimportsnotusedasvalues |
| 3529 | Implement Noemithelpers | spike | frontend/syntax | class: triage-needed | Implement Noemithelpers |
| 3530 | Implement Noerrorusingimportexportmoduleaugmentationindeclarationfile | spike | frontend/syntax | class: blocked | Implement Noerrorusingimportexportmoduleaugmentationindeclarationfile |
| 3531 | Implement Noerrorsincallback | spike | frontend/syntax | class: blocked | Implement Noerrorsincallback |
| 3532 | Implement Noexcessivestackdeptherror | spike | reference/triage | class: triage-needed | Implement Noexcessivestackdeptherror |
| 3533 | Implement Noimplicitanyandprivatememberswithouttypeannotations | spike | frontend/resolver | class: blocked | Implement Noimplicitanyandprivatememberswithouttypeannotations |
| 3534 | Implement Noimplicitanydestructuringinprivatemethod | spike | frontend/syntax | class: blocked | Implement Noimplicitanydestructuringinprivatemethod |
| 3535 | Implement Noimplicitanydestructuringvardeclaration | spike | frontend/syntax | class: blocked | Implement Noimplicitanydestructuringvardeclaration |
| 3536 | Implement Noimplicitanyforin | spike | frontend/syntax | class: blocked | Implement Noimplicitanyforin |
| 3537 | Implement Noimplicitanyformethodparameters | spike | frontend/syntax | class: blocked | Implement Noimplicitanyformethodparameters |
| 3538 | Implement Noimplicitanyfunctionexpressionassignment | spike | frontend/syntax | class: blocked | Implement Noimplicitanyfunctionexpressionassignment |
| 3539 | Implement Noimplicitanyfunctions | spike | frontend/syntax | class: blocked | Implement Noimplicitanyfunctions |
| 3540 | Implement Noimplicitanyincastexpression | spike | frontend/syntax | class: triage-needed | Implement Noimplicitanyincastexpression |
| 3541 | Implement Noimplicitanyincontextuallytypesfunctionparamter | spike | frontend/syntax | class: blocked | Implement Noimplicitanyincontextuallytypesfunctionparamter |
| 3542 | Implement Noimplicitanyindexing | spike | frontend/syntax | class: blocked | Implement Noimplicitanyindexing |
| 3543 | Implement Noimplicitanyindexingsuppressed | spike | frontend/syntax | class: blocked | Implement Noimplicitanyindexingsuppressed |
| 3544 | Implement Noimplicitanyloopcrash | spike | frontend/syntax | class: blocked | Implement Noimplicitanyloopcrash |
| 3545 | Implement Noimplicitanymissinggetaccessor | spike | frontend/syntax | class: blocked | Implement Noimplicitanymissinggetaccessor |
| 3546 | Implement Noimplicitanymissingsetaccessor | spike | frontend/syntax | class: blocked | Implement Noimplicitanymissingsetaccessor |
| 3547 | Implement Noimplicitanymodule | spike | frontend/syntax | class: blocked | Implement Noimplicitanymodule |
| 3548 | Implement Noimplicitanynamelessparameter | spike | frontend/syntax | class: blocked | Implement Noimplicitanynamelessparameter |
| 3549 | Implement Noimplicitanyparametersinambientmodule | spike | frontend/syntax | class: blocked | Implement Noimplicitanyparametersinambientmodule |
| 3550 | Implement Noimplicitanyparametersinclass | spike | frontend/syntax | class: blocked | Implement Noimplicitanyparametersinclass |
| 3551 | Implement Noimplicitanyparametersinmodule | spike | frontend/syntax | class: blocked | Implement Noimplicitanyparametersinmodule |
| 3552 | Implement Noimplicitanystringindexeronobject | spike | frontend/syntax | class: blocked | Implement Noimplicitanystringindexeronobject |
| 3553 | Implement Noimplicitanywithoverloads | spike | frontend/syntax | class: blocked | Implement Noimplicitanywithoverloads |
| 3554 | Implement Noimplicitreturnsexclusions | spike | frontend/syntax | class: triage-needed | Implement Noimplicitreturnsexclusions |
| 3555 | Implement Noimplicitreturnsinasync | spike | reference/triage | class: triage-needed | Implement Noimplicitreturnsinasync |
| 3556 | Implement Noimplicitreturnswithprotectedblocks | spike | frontend/resolver | class: blocked | Implement Noimplicitreturnswithprotectedblocks |
| 3557 | Implement Noimplicitsymboltostring | spike | frontend/syntax | class: triage-needed | Implement Noimplicitsymboltostring |
| 3558 | Implement Noimplicitthisbigthis | spike | reference/triage | class: triage-needed | Implement Noimplicitthisbigthis |
| 3559 | Implement Noimplicitthisfunctions | spike | frontend/syntax | class: triage-needed | Implement Noimplicitthisfunctions |
| 3560 | Implement Noimplicitusestrict | spike | frontend/syntax | class: blocked | Implement Noimplicitusestrict |
| 3561 | Implement Noinfercommonpropertycheck | spike | frontend/resolver | class: blocked | Implement Noinfercommonpropertycheck |
| 3562 | Implement Noinferunionexcesspropertycheck | spike | frontend/resolver | class: blocked | Implement Noinferunionexcesspropertycheck |
| 3563 | Implement Noiterationtypeerrorsincfa | spike | frontend/syntax | class: blocked | Implement Noiterationtypeerrorsincfa |
| 3564 | Implement Noparameterreassignmentiifeannotated | spike | frontend/resolver | class: blocked | Implement Noparameterreassignmentiifeannotated |
| 3565 | Implement Noparameterreassignmentjsiife | spike | frontend/resolver | class: blocked | Implement Noparameterreassignmentjsiife |
| 3566 | Implement Nosubtypereduction | spike | frontend/syntax | class: blocked | Implement Nosubtypereduction |
| 3567 | Implement Nosymbolformergecrash | spike | frontend/syntax | class: blocked | Implement Nosymbolformergecrash |
| 3568 | Implement Notypeargumentonreturntype | spike | frontend/syntax | class: blocked | Implement Notypeargumentonreturntype |
| 3569 | Implement Nouncheckedindexaccess | spike | frontend/syntax | class: triage-needed | Implement Nouncheckedindexaccess |
| 3570 | Implement Nouncheckedindexedaccesscompoundassignments | spike | frontend/syntax | class: blocked | Implement Nouncheckedindexedaccesscompoundassignments |
| 3571 | Implement Nounusedlocals Destructuring | spike | frontend/syntax | class: blocked | Implement Nounusedlocals Destructuring |
| 3572 | Implement Nounusedlocals Import Export | spike | frontend/syntax | class: blocked | Implement Nounusedlocals Import Export |
| 3573 | Implement Nounusedlocals Name Resolution | spike | frontend/resolver | class: blocked | Implement Nounusedlocals Name Resolution |
| 3574 | Implement Nounusedlocals Parser Syntax | spike | frontend/syntax | class: blocked | Implement Nounusedlocals Parser Syntax |
| 3575 | Implement Nousedbeforedefinederrorintypecontext | spike | frontend/syntax | class: triage-needed | Implement Nousedbeforedefinederrorintypecontext |
| 3576 | Implement Nodecolonmoduleresolution | spike | frontend/syntax | class: blocked | Implement Nodecolonmoduleresolution |
| 3577 | Implement Nodemodulereexportfromdottedpath | spike | frontend/syntax | class: blocked | Implement Nodemodulereexportfromdottedpath |
| 3578 | Implement Nodenextcjsnamespaceimportdefault | spike | frontend/syntax | class: blocked | Implement Nodenextcjsnamespaceimportdefault |
| 3579 | Implement Nodenextesmimportsofpackageswithextensionlessmains | spike | frontend/syntax | class: blocked | Implement Nodenextesmimportsofpackageswithextensionlessmains |
| 3580 | Implement Nodenextimportmodeimplicitindexresolution Import Export | spike | frontend/syntax | class: blocked | Implement Nodenextimportmodeimplicitindexresolution Import Export |
| 3581 | Implement Nodenextimportmodeimplicitindexresolution Module Resolution | spike | frontend/syntax | class: blocked | Implement Nodenextimportmodeimplicitindexresolution Module Resolution |
| 3582 | Implement Nodenextpackageimportmaprootdir | spike | frontend/syntax | class: blocked | Implement Nodenextpackageimportmaprootdir |
| 3583 | Implement Nodenextpackageselfnamewithoutdir | spike | frontend/syntax | class: blocked | Implement Nodenextpackageselfnamewithoutdir |
| 3584 | Implement Nodenextpackageselfnamewithoutdirdecldir | spike | frontend/syntax | class: blocked | Implement Nodenextpackageselfnamewithoutdirdecldir |
| 3585 | Implement Nodenextpackageselfnamewithoutdirdecldircomposite | spike | frontend/syntax | class: blocked | Implement Nodenextpackageselfnamewithoutdirdecldircomposite |
| 3586 | Implement Nodenextpackageselfnamewithoutdirdecldircompositenesteddirs | spike | frontend/syntax | class: blocked | Implement Nodenextpackageselfnamewithoutdirdecldircompositenesteddirs |
| 3587 | Implement Nodenextpackageselfnamewithoutdirdecldirnesteddirs | spike | frontend/syntax | class: blocked | Implement Nodenextpackageselfnamewithoutdirdecldirnesteddirs |
| 3588 | Implement Nodenextpackageselfnamewithoutdirdecldirrootdir | spike | frontend/syntax | class: blocked | Implement Nodenextpackageselfnamewithoutdirdecldirrootdir |
| 3589 | Implement Nodenextpackageselfnamewithoutdirrootdir | spike | frontend/syntax | class: blocked | Implement Nodenextpackageselfnamewithoutdirrootdir |
| 3590 | Implement Noderesolution | spike | frontend/syntax | class: blocked | Implement Noderesolution |
| 3591 | Implement Nonexportedelementsofmergedmodules | spike | frontend/syntax | class: blocked | Implement Nonexportedelementsofmergedmodules |
| 3592 | Implement Nongenericclassextendinggenericclasswithany | spike | frontend/semantics | class: blocked | Implement Nongenericclassextendinggenericclasswithany |
| 3593 | Implement Nonidenticaltypeconstraints | spike | frontend/syntax | class: blocked | Implement Nonidenticaltypeconstraints |
| 3594 | Implement Noninferrabletypepropagation Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Noninferrabletypepropagation Parser Syntax |
| 3595 | Implement Noninferrabletypepropagation Type System | spike | frontend/semantics | class: blocked | Implement Noninferrabletypepropagation Type System |
| 3596 | Implement Nonmergedoverloads | spike | frontend/syntax | class: blocked | Implement Nonmergedoverloads |
| 3597 | Implement Nonnullfullinference | spike | frontend/semantics | class: blocked | Implement Nonnullfullinference |
| 3598 | Implement Nonnullmappedtype | spike | frontend/syntax | class: blocked | Implement Nonnullmappedtype |
| 3599 | Implement Nonnullparameterextendingstringassignabletostring | spike | frontend/syntax | class: blocked | Implement Nonnullparameterextendingstringassignabletostring |
| 3600 | Implement Nonnullreferencematching | spike | frontend/syntax | class: blocked | Implement Nonnullreferencematching |
| 3601 | Implement Nonnullablereduction | spike | frontend/syntax | class: blocked | Implement Nonnullablereduction |
| 3602 | Implement Nonnullablereductionnonstrict | spike | frontend/syntax | class: blocked | Implement Nonnullablereductionnonstrict |
| 3603 | Implement Nonnullabletypes | spike | frontend/syntax | class: blocked | Implement Nonnullabletypes |
| 3604 | Implement Nonnullablewithnullablegenericindexedaccessarg | spike | frontend/semantics | class: blocked | Implement Nonnullablewithnullablegenericindexedaccessarg |
| 3605 | Implement Nongenericpartialinstantiationsrelatedinbothdirections | spike | frontend/resolver | class: blocked | Implement Nongenericpartialinstantiationsrelatedinbothdirections |
| 3606 | Implement Nonnullassertionpropegatescontextualtype | spike | frontend/syntax | class: blocked | Implement Nonnullassertionpropegatescontextualtype |
| 3607 | Implement Normalizedintersectiontoocomplex | spike | frontend/resolver | class: blocked | Implement Normalizedintersectiontoocomplex |
| 3608 | Implement Nounusedtypeparameterconstraint | spike | frontend/syntax | class: blocked | Implement Nounusedtypeparameterconstraint |
| 3609 | Implement Nullablefunctionerror | spike | frontend/syntax | class: blocked | Implement Nullablefunctionerror |
| 3610 | Implement Numberassignabletoenuminsideunion | spike | runtime/builtins | class: triage-needed | Implement Numberassignabletoenuminsideunion |
| 3611 | Implement Numberliteralswithleadingzeros | spike | frontend/syntax | class: triage-needed | Implement Numberliteralswithleadingzeros |
| 3612 | Implement Numbervsbigintoperations | spike | runtime/builtins | class: triage-needed | Implement Numbervsbigintoperations |
| 3613 | Implement Numericclassmembers | spike | frontend/syntax | class: triage-needed | Implement Numericclassmembers |
| 3614 | Implement Numericenummappedtype | spike | frontend/syntax | class: blocked | Implement Numericenummappedtype |
| 3615 | Implement Numericindexexpressions | spike | frontend/resolver | class: blocked | Implement Numericindexexpressions |
| 3616 | Implement Numericindexerconstraint Name Resolution | spike | frontend/resolver | class: blocked | Implement Numericindexerconstraint Name Resolution |
| 3617 | Implement Numericindexerconstraint Parser Syntax | spike | frontend/syntax | class: blocked | Implement Numericindexerconstraint Parser Syntax |
| 3618 | Implement Numericindexertyping Name Resolution | spike | frontend/resolver | class: blocked | Implement Numericindexertyping Name Resolution |
| 3619 | Implement Numericindexertyping Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Numericindexertyping Parser Syntax |
| 3620 | Implement Numericliteralswithtrailingdecimalpoints | spike | frontend/syntax | class: triage-needed | Implement Numericliteralswithtrailingdecimalpoints |
| 3621 | Implement Numericmethodname | spike | frontend/syntax | class: triage-needed | Implement Numericmethodname |
| 3622 | Implement Numericunderscoredseparator | spike | frontend/syntax | class: triage-needed | Implement Numericunderscoredseparator |
| 3623 | Implement Objectassignlikenonunionresult | spike | frontend/resolver | class: blocked | Implement Objectassignlikenonunionresult |
| 3624 | Implement Objectbindingpattern | spike | frontend/syntax | class: blocked | Implement Objectbindingpattern |
| 3625 | Implement Objectbindingpatterncontextuallytypesargument | spike | reference/triage | class: triage-needed | Implement Objectbindingpatterncontextuallytypesargument |
| 3626 | Implement Objectcreate Name Resolution | spike | frontend/resolver | class: blocked | Implement Objectcreate Name Resolution |
| 3627 | Implement Objectcreate Object Literal | spike | frontend/syntax | class: blocked | Implement Objectcreate Object Literal |
| 3628 | Implement Objectcreationexpressioninfunctionparameter | spike | frontend/syntax | class: blocked | Implement Objectcreationexpressioninfunctionparameter |
| 3629 | Implement Objectcreationofelementaccessexpression | spike | frontend/syntax | class: blocked | Implement Objectcreationofelementaccessexpression |
| 3630 | Implement Objectfreeze | spike | frontend/resolver | class: blocked | Implement Objectfreeze |
| 3631 | Implement Objectfreezeliteralsdontwiden | spike | frontend/resolver | class: blocked | Implement Objectfreezeliteralsdontwiden |
| 3632 | Implement Objectfromentries | spike | frontend/resolver | class: blocked | Implement Objectfromentries |
| 3633 | Implement Objectgroupby | spike | frontend/syntax | class: blocked | Implement Objectgroupby |
| 3634 | Implement Objectindexer | spike | frontend/syntax | class: blocked | Implement Objectindexer |
| 3635 | Implement Objectinstantiationfromunionspread | spike | frontend/syntax | class: blocked | Implement Objectinstantiationfromunionspread |
| 3636 | Implement Objectlitarraydeclnonew | spike | frontend/syntax | class: blocked | Implement Objectlitarraydeclnonew |
| 3637 | Implement Objectlitgettersetter | spike | frontend/syntax | class: blocked | Implement Objectlitgettersetter |
| 3638 | Implement Objectlitindexercontextualtype | spike | frontend/syntax | class: blocked | Implement Objectlitindexercontextualtype |
| 3639 | Implement Objectlitpropertyscoping | spike | frontend/syntax | class: blocked | Implement Objectlitpropertyscoping |
| 3640 | Implement Objectliteralarrayspecialization | spike | frontend/resolver | class: blocked | Implement Objectliteralarrayspecialization |
| 3641 | Implement Objectliteralcomputednamenodeclarationerror | spike | frontend/syntax | class: blocked | Implement Objectliteralcomputednamenodeclarationerror |
| 3642 | Implement Objectliteraldeclarationgeneration | spike | frontend/syntax | class: blocked | Implement Objectliteraldeclarationgeneration |
| 3643 | Implement Objectliteralenumpropertynames | spike | frontend/syntax | class: triage-needed | Implement Objectliteralenumpropertynames |
| 3644 | Implement Objectliteralexcessproperties | spike | frontend/syntax | class: blocked | Implement Objectliteralexcessproperties |
| 3645 | Implement Objectliteralfreshnesswithspread | spike | frontend/syntax | class: blocked | Implement Objectliteralfreshnesswithspread |
| 3646 | Implement Objectliteralfunctionargcontextualtyping | spike | frontend/syntax | class: blocked | Implement Objectliteralfunctionargcontextualtyping |
| 3647 | Implement Objectliteralindexererrors | spike | frontend/syntax | class: blocked | Implement Objectliteralindexererrors |
| 3648 | Implement Objectliteralindexernoimplicitany | spike | frontend/syntax | class: blocked | Implement Objectliteralindexernoimplicitany |
| 3649 | Implement Objectliteralindexers | spike | frontend/syntax | class: blocked | Implement Objectliteralindexers |
| 3650 | Implement Objectliteralmemberwithmodifiers | spike | frontend/syntax | class: blocked | Implement Objectliteralmemberwithmodifiers |
| 3651 | Implement Objectliteralmemberwithquestionmark | spike | frontend/syntax | class: blocked | Implement Objectliteralmemberwithquestionmark |
| 3652 | Implement Objectliteralmemberwithoutblock | spike | frontend/syntax | class: blocked | Implement Objectliteralmemberwithoutblock |
| 3653 | Implement Objectliteralparameterresolution | spike | frontend/resolver | class: blocked | Implement Objectliteralparameterresolution |
| 3654 | Implement Objectliteralpropertyimplicitlyany | spike | frontend/syntax | class: blocked | Implement Objectliteralpropertyimplicitlyany |
| 3655 | Implement Objectliteralreferencinginternalproperties | spike | frontend/resolver | class: blocked | Implement Objectliteralreferencinginternalproperties |
| 3656 | Implement Objectliteralthiswidenedonuse | spike | reference/triage | class: triage-needed | Implement Objectliteralthiswidenedonuse |
| 3657 | Implement Objectliteralwithgetaccessorinsidefunction | spike | frontend/syntax | class: blocked | Implement Objectliteralwithgetaccessorinsidefunction |
| 3658 | Implement Objectliteralwithnumericpropertyname | spike | frontend/syntax | class: blocked | Implement Objectliteralwithnumericpropertyname |
| 3659 | Implement Objectliteralwithsemicolons | spike | frontend/syntax | class: blocked | Implement Objectliteralwithsemicolons |
| 3660 | Implement Objectliteralsagainstunionsofarrays | spike | frontend/syntax | class: blocked | Implement Objectliteralsagainstunionsofarrays |
| 3661 | Implement Objectmembersontypes | spike | frontend/syntax | class: blocked | Implement Objectmembersontypes |
| 3662 | Implement Objectpropertyasclass | spike | frontend/syntax | class: blocked | Implement Objectpropertyasclass |
| 3663 | Implement Objectrestbindingcontextualinference | spike | frontend/resolver | class: blocked | Implement Objectrestbindingcontextualinference |
| 3664 | Implement Objectrestspread | spike | frontend/syntax | class: blocked | Implement Objectrestspread |
| 3665 | Implement Objecttypewithoptionalproperty | spike | frontend/syntax | class: blocked | Implement Objecttypewithoptionalproperty |
| 3666 | Implement Observableinferencecanbemade | spike | frontend/semantics | class: blocked | Implement Observableinferencecanbemade |
| 3667 | Implement Octalliteralandescapesequence | spike | frontend/syntax | class: blocked | Implement Octalliteralandescapesequence |
| 3668 | Implement Omittypetesterrors | spike | frontend/syntax | class: blocked | Implement Omittypetesterrors |
| 3669 | Implement Omittypetests | spike | frontend/syntax | class: blocked | Implement Omittypetests |
| 3670 | Implement Omittedexpressionforofloop | spike | frontend/syntax | class: triage-needed | Implement Omittedexpressionforofloop |
| 3671 | Implement Operationsavailableonpromisedtype | spike | reference/triage | class: triage-needed | Implement Operationsavailableonpromisedtype |
| 3672 | Implement Operatoraddnullundefined | spike | frontend/syntax | class: triage-needed | Implement Operatoraddnullundefined |
| 3673 | Implement Optionalaccessorsininterface | spike | frontend/resolver | class: blocked | Implement Optionalaccessorsininterface |
| 3674 | Implement Optionalargswithdefaultvalues | spike | frontend/syntax | class: blocked | Implement Optionalargswithdefaultvalues |
| 3675 | Implement Optionalchainwithinstantiationexpression Import Export | spike | frontend/syntax | class: blocked | Implement Optionalchainwithinstantiationexpression Import Export |
| 3676 | Implement Optionalchainwithinstantiationexpression Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Optionalchainwithinstantiationexpression Parser Syntax |
| 3677 | Implement Optionalconstructorarginsuper | spike | frontend/syntax | class: blocked | Implement Optionalconstructorarginsuper |
| 3678 | Implement Optionalfunctionargassignability | spike | reference/triage | class: triage-needed | Implement Optionalfunctionargassignability |
| 3679 | Implement Optionalparamargstest | spike | frontend/syntax | class: blocked | Implement Optionalparamargstest |
| 3680 | Implement Optionalparamassignmentcompat | spike | frontend/resolver | class: blocked | Implement Optionalparamassignmentcompat |
| 3681 | Implement Optionalparaminoverride | spike | frontend/syntax | class: triage-needed | Implement Optionalparaminoverride |
| 3682 | Implement Optionalparamreferencingotherparams | spike | frontend/resolver | class: blocked | Implement Optionalparamreferencingotherparams |
| 3683 | Implement Optionalparamtypecomparison | spike | frontend/resolver | class: blocked | Implement Optionalparamtypecomparison |
| 3684 | Implement Optionalparameterindestructuringwithinitializer | spike | frontend/resolver | class: blocked | Implement Optionalparameterindestructuringwithinitializer |
| 3685 | Implement Optionalparameterretainsnull | spike | reference/triage | class: triage-needed | Implement Optionalparameterretainsnull |
| 3686 | Implement Optionalparamterandvariabledeclaration | spike | reference/triage | class: triage-needed | Implement Optionalparamterandvariabledeclaration |
| 3687 | Implement Optionalpropertiesinclasses | spike | frontend/syntax | class: triage-needed | Implement Optionalpropertiesinclasses |
| 3688 | Implement Optionalpropertiestest | spike | frontend/syntax | class: triage-needed | Implement Optionalpropertiestest |
| 3689 | Implement Optionalsetterparam | spike | frontend/syntax | class: blocked | Implement Optionalsetterparam |
| 3691 | Implement Optionsoutandnomodulegen | spike | frontend/syntax | class: blocked | Implement Optionsoutandnomodulegen |
| 3692 | Implement Ordermattersforsignaturegroupidentity | spike | frontend/resolver | class: blocked | Implement Ordermattersforsignaturegroupidentity |
| 3693 | Implement Out | spike | frontend/syntax | class: triage-needed | Implement Out |
| 3694 | Implement Outmoduleconcatamd | spike | frontend/syntax | class: blocked | Implement Outmoduleconcatamd |
| 3695 | Implement Outmoduleconcatcommonjs | spike | frontend/syntax | class: blocked | Implement Outmoduleconcatcommonjs |
| 3696 | Implement Outmoduleconcatcommonjsdeclarationonly | spike | frontend/syntax | class: blocked | Implement Outmoduleconcatcommonjsdeclarationonly |
| 3697 | Implement Outmoduleconcates | spike | frontend/syntax | class: blocked | Implement Outmoduleconcates |
| 3698 | Implement Outmoduleconcatsystem | spike | frontend/syntax | class: blocked | Implement Outmoduleconcatsystem |
| 3699 | Implement Outmoduleconcatumd | spike | frontend/syntax | class: blocked | Implement Outmoduleconcatumd |
| 3700 | Implement Outmoduleconcatunspecifiedmodulekind | spike | frontend/syntax | class: blocked | Implement Outmoduleconcatunspecifiedmodulekind |
| 3701 | Implement Outmoduleconcatunspecifiedmodulekinddeclarationonly | spike | frontend/syntax | class: blocked | Implement Outmoduleconcatunspecifiedmodulekinddeclarationonly |
| 3702 | Implement Outmoduletripleslashrefs | spike | frontend/syntax | class: blocked | Implement Outmoduletripleslashrefs |
| 3703 | Implement Overeagerreturntypespecialization | spike | frontend/resolver | class: blocked | Implement Overeagerreturntypespecialization |
| 3704 | Implement Overload Import Export | spike | frontend/syntax | class: blocked | Implement Overload Import Export |
| 3705 | Implement Overload Parser Syntax | spike | frontend/syntax | class: blocked | Implement Overload Parser Syntax |
| 3706 | Implement Overloadassignmentcompat | spike | frontend/resolver | class: blocked | Implement Overloadassignmentcompat |
| 3707 | Implement Overloadbindingacrossdeclarationboundaries | spike | reference/triage | class: triage-needed | Implement Overloadbindingacrossdeclarationboundaries |
| 3708 | Implement Overloadcalltest | spike | reference/triage | class: triage-needed | Implement Overloadcalltest |
| 3709 | Implement Overloadconsecutiveness | spike | frontend/syntax | class: blocked | Implement Overloadconsecutiveness |
| 3710 | Implement Overloadcrash | spike | frontend/resolver | class: blocked | Implement Overloadcrash |
| 3711 | Implement Overloadequivalencewithstatics | spike | frontend/syntax | class: blocked | Implement Overloadequivalencewithstatics |
| 3712 | Implement Overloaderrormatchesimplementationelaboaration | spike | frontend/resolver | class: blocked | Implement Overloaderrormatchesimplementationelaboaration |
| 3713 | Implement Overloadgenericfunctionwithrestargs | spike | frontend/semantics | class: blocked | Implement Overloadgenericfunctionwithrestargs |
| 3714 | Implement Overloadmodifiersmustagree | spike | frontend/syntax | class: blocked | Implement Overloadmodifiersmustagree |
| 3715 | Implement Overloadonconstconstraintchecks | spike | frontend/syntax | class: blocked | Implement Overloadonconstconstraintchecks |
| 3716 | Implement Overloadonconstduplicateoverloads | spike | frontend/syntax | class: blocked | Implement Overloadonconstduplicateoverloads |
| 3717 | Implement Overloadonconstinbasewithbadimplementationinderived | spike | frontend/syntax | class: blocked | Implement Overloadonconstinbasewithbadimplementationinderived |
| 3718 | Implement Overloadonconstincallback | spike | frontend/syntax | class: blocked | Implement Overloadonconstincallback |
| 3719 | Implement Overloadonconstinheritance | spike | frontend/syntax | class: blocked | Implement Overloadonconstinheritance |
| 3720 | Implement Overloadonconstnoanyimplementation | spike | frontend/syntax | class: blocked | Implement Overloadonconstnoanyimplementation |
| 3721 | Implement Overloadonconstnononspecializedsignature | spike | frontend/syntax | class: blocked | Implement Overloadonconstnononspecializedsignature |
| 3722 | Implement Overloadonconstnostringimplementation | spike | frontend/syntax | class: blocked | Implement Overloadonconstnostringimplementation |
| 3723 | Implement Overloadongenericclassandnongenericclass | spike | frontend/semantics | class: blocked | Implement Overloadongenericclassandnongenericclass |
| 3724 | Implement Overloadresolutionondefaultconstructor | spike | frontend/syntax | class: blocked | Implement Overloadresolutionondefaultconstructor |
| 3725 | Implement Overloadresolutionovernonctlambdas | spike | frontend/syntax | class: blocked | Implement Overloadresolutionovernonctlambdas |
| 3726 | Implement Overloadresolutionovernonctobjectlit | spike | frontend/syntax | class: blocked | Implement Overloadresolutionovernonctobjectlit |
| 3727 | Implement Overloadresolutionwithany | spike | frontend/syntax | class: blocked | Implement Overloadresolutionwithany |
| 3728 | Implement Overloadreturntypes | spike | frontend/resolver | class: blocked | Implement Overloadreturntypes |
| 3729 | Implement Overloadwithcallbackswithdifferingoptionalityonargs | spike | frontend/syntax | class: blocked | Implement Overloadwithcallbackswithdifferingoptionalityonargs |
| 3730 | Implement Overloadedconstructorfixesinferencesappropriately | spike | frontend/semantics | class: blocked | Implement Overloadedconstructorfixesinferencesappropriately |
| 3731 | Implement Overloadedstaticmethodspecialization | spike | frontend/syntax | class: blocked | Implement Overloadedstaticmethodspecialization |
| 3732 | Implement Overloadingonconstants | spike | frontend/syntax | class: blocked | Implement Overloadingonconstants |
| 3733 | Implement Overloadingonconstantsinimplementation | spike | frontend/syntax | class: blocked | Implement Overloadingonconstantsinimplementation |
| 3734 | Implement Overloadingstaticfunctionsinfunctions | spike | frontend/syntax | class: blocked | Implement Overloadingstaticfunctionsinfunctions |
| 3735 | Implement Overloadresolutionwithconstraintcheckingdeferred | spike | frontend/syntax | class: blocked | Implement Overloadresolutionwithconstraintcheckingdeferred |
| 3736 | Implement Overloadsandtypeargumentarity | spike | frontend/syntax | class: blocked | Implement Overloadsandtypeargumentarity |
| 3737 | Implement Overloadsandtypeargumentarityerrors | spike | frontend/syntax | class: blocked | Implement Overloadsandtypeargumentarityerrors |
| 3738 | Implement Overloadsindifferentcontainersdisagreeonambient | spike | frontend/syntax | class: blocked | Implement Overloadsindifferentcontainersdisagreeonambient |
| 3739 | Implement Overloadswithcomputednames | spike | frontend/syntax | class: blocked | Implement Overloadswithcomputednames |
| 3740 | Implement Overloadswithprovisionalerrors | spike | frontend/resolver | class: blocked | Implement Overloadswithprovisionalerrors |
| 3741 | Implement Overloadswithinclasses | spike | frontend/syntax | class: blocked | Implement Overloadswithinclasses |
| 3742 | Implement Overridebaseintersectionmethod | spike | frontend/syntax | class: triage-needed | Implement Overridebaseintersectionmethod |
| 3743 | Implement Overridingprivatestaticmembers | spike | frontend/syntax | class: blocked | Implement Overridingprivatestaticmembers |
| 3744 | Implement Overshifts | spike | frontend/syntax | class: triage-needed | Implement Overshifts |
| 3745 | Implement Parampropertiesinsignatures | spike | frontend/syntax | class: triage-needed | Implement Parampropertiesinsignatures |
| 3746 | Implement Parameterdecoratorsemitcrash | spike | frontend/syntax | class: blocked | Implement Parameterdecoratorsemitcrash |
| 3747 | Implement Parameterdestructuringobjectliteral | spike | reference/triage | class: triage-needed | Implement Parameterdestructuringobjectliteral |
| 3748 | Implement Parameterinitializerbeforedestructuringemit | spike | reference/triage | class: triage-needed | Implement Parameterinitializerbeforedestructuringemit |
| 3749 | Implement Parameterpropertyinconstructor | spike | frontend/syntax | class: blocked | Implement Parameterpropertyinconstructor |
| 3750 | Implement Parameterpropertyinconstructorwithprologues | spike | frontend/syntax | class: blocked | Implement Parameterpropertyinconstructorwithprologues |
| 3751 | Implement Parameterpropertyoutsideconstructor | spike | frontend/syntax | class: blocked | Implement Parameterpropertyoutsideconstructor |
| 3752 | Implement Parameterreferenceininitializer | spike | frontend/syntax | class: triage-needed | Implement Parameterreferenceininitializer |
| 3753 | Implement Parameterreferencesotherparameter | spike | frontend/syntax | class: triage-needed | Implement Parameterreferencesotherparameter |
| 3754 | Implement Parameterssyntaxerrornocrash Import Export | spike | frontend/syntax | class: blocked | Implement Parameterssyntaxerrornocrash Import Export |
| 3755 | Implement Parameterssyntaxerrornocrash Parser Syntax | spike | runtime/builtins | class: triage-needed | Implement Parameterssyntaxerrornocrash Parser Syntax |
| 3756 | Implement Paramsonlyhaveliteraltypeswhenappropriatelycontextualized | spike | frontend/syntax | class: blocked | Implement Paramsonlyhaveliteraltypeswhenappropriatelycontextualized |
| 3757 | Implement Parenthesisdoesnotblockaliassymbolcreation | spike | frontend/syntax | class: blocked | Implement Parenthesisdoesnotblockaliassymbolcreation |
| 3758 | Implement Parenthesizedasyncarrowfunction | spike | frontend/syntax | class: triage-needed | Implement Parenthesizedasyncarrowfunction |
| 3759 | Implement Parenthesizedjsdoccastatreturnstatement | spike | frontend/syntax | class: blocked | Implement Parenthesizedjsdoccastatreturnstatement |
| 3760 | Implement Parse Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Parse Parser Syntax |
| 3761 | Implement Parse Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Parse Unknown Unsupported |
| 3762 | Implement Parsearrowfunctionwithfunctionreturntype | spike | frontend/syntax | class: triage-needed | Implement Parsearrowfunctionwithfunctionreturntype |
| 3763 | Implement Parseassertentrieserror | spike | runtime/builtins | class: triage-needed | Implement Parseassertentrieserror |
| 3764 | Implement Parsebigint | spike | frontend/syntax | class: triage-needed | Implement Parsebigint |
| 3765 | Implement Parsecommaseparatednewlinenew | spike | frontend/syntax | class: triage-needed | Implement Parsecommaseparatednewlinenew |
| 3766 | Implement Parsecommaseparatednewlinenumber | spike | frontend/syntax | class: blocked | Implement Parsecommaseparatednewlinenumber |
| 3767 | Implement Parsecommaseparatednewlinestring | spike | frontend/syntax | class: triage-needed | Implement Parsecommaseparatednewlinestring |
| 3768 | Implement Parseentitynamewithreservedword | spike | frontend/syntax | class: triage-needed | Implement Parseentitynamewithreservedword |
| 3769 | Implement Parseerrordoublecommaincall | spike | runtime/builtins | class: triage-needed | Implement Parseerrordoublecommaincall |
| 3770 | Implement Parseerrorinheritageclause | spike | runtime/builtins | class: triage-needed | Implement Parseerrorinheritageclause |
| 3771 | Implement Parseerrorincorrectreturntoken | spike | frontend/syntax | class: blocked | Implement Parseerrorincorrectreturntoken |
| 3772 | Implement Parsegenericarrowratherthanleftshift | spike | frontend/semantics | class: blocked | Implement Parsegenericarrowratherthanleftshift |
| 3773 | Implement Parseimportattributeserror | spike | runtime/builtins | class: triage-needed | Implement Parseimportattributeserror |
| 3774 | Implement Parseinvalidnames | spike | frontend/syntax | class: blocked | Implement Parseinvalidnames |
| 3775 | Implement Parseinvalidnullabletypes | spike | frontend/syntax | class: triage-needed | Implement Parseinvalidnullabletypes |
| 3776 | Implement Parsejsxelementinunaryexpressionnocrash Jsx | spike | frontend/syntax | class: blocked | Implement Parsejsxelementinunaryexpressionnocrash Jsx |
| 3777 | Implement Parsejsxelementinunaryexpressionnocrash Regexp Literal | spike | reference/triage | class: triage-needed | Implement Parsejsxelementinunaryexpressionnocrash Regexp Literal |
| 3778 | Implement Parsejsxextends | spike | reference/triage | class: triage-needed | Implement Parsejsxextends |
| 3779 | Implement Parseobjectliteralswithouttypes | spike | frontend/syntax | class: triage-needed | Implement Parseobjectliteralswithouttypes |
| 3780 | Implement Parsetypes | spike | frontend/syntax | class: triage-needed | Implement Parsetypes |
| 3781 | Implement Parseunaryexpressionnotypeassertioninjsx | spike | frontend/syntax | class: blocked | Implement Parseunaryexpressionnotypeassertioninjsx |
| 3782 | Implement Parseunmatchedtypeassertion | spike | frontend/syntax | class: blocked | Implement Parseunmatchedtypeassertion |
| 3783 | Implement Parserconstructordeclaration | spike | reference/triage | class: triage-needed | Implement Parserconstructordeclaration |
| 3784 | Implement Parserisclassmemberstart | spike | frontend/syntax | class: triage-needed | Implement Parserisclassmemberstart |
| 3785 | Implement Parserprivateidentifierinarrayassignment | spike | frontend/syntax | class: blocked | Implement Parserprivateidentifierinarrayassignment |
| 3786 | Implement Parserunparsedtokencrash Import Export | spike | frontend/syntax | class: blocked | Implement Parserunparsedtokencrash Import Export |
| 3787 | Implement Parserunparsedtokencrash Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Parserunparsedtokencrash Parser Syntax |
| 3788 | Implement Parsingclassrecoverswhenhittingunexpectedsemicolon | spike | frontend/syntax | class: triage-needed | Implement Parsingclassrecoverswhenhittingunexpectedsemicolon |
| 3789 | Implement Parsingdeepparenthensizedexpression | spike | frontend/syntax | class: triage-needed | Implement Parsingdeepparenthensizedexpression |
| 3790 | Implement Partialoflargeapiisabletobeworkedwith | spike | frontend/resolver | class: blocked | Implement Partialoflargeapiisabletobeworkedwith |
| 3791 | Implement Partiallyambientclodule | spike | frontend/syntax | class: blocked | Implement Partiallyambientclodule |
| 3792 | Implement Partiallyambientfundule | spike | frontend/syntax | class: blocked | Implement Partiallyambientfundule |
| 3793 | Implement Partiallydiscriminantedunions | spike | frontend/syntax | class: triage-needed | Implement Partiallydiscriminantedunions |
| 3794 | Implement Pathmappingbasedmoduleresolution Import Export | spike | frontend/syntax | class: blocked | Implement Pathmappingbasedmoduleresolution Import Export |
| 3795 | Implement Pathmappingbasedmoduleresolution Module Resolution | spike | frontend/syntax | class: blocked | Implement Pathmappingbasedmoduleresolution Module Resolution |
| 3796 | Implement Pathmappingbasedmoduleresolution Parser Syntax | spike | frontend/syntax | class: blocked | Implement Pathmappingbasedmoduleresolution Parser Syntax |
| 3797 | Implement Pathmappinginheritedbaseurl | spike | frontend/syntax | class: blocked | Implement Pathmappinginheritedbaseurl |
| 3798 | Implement Pathmappingwithoutbaseurl | spike | frontend/syntax | class: blocked | Implement Pathmappingwithoutbaseurl |
| 3799 | Implement Pathsvalidation | spike | frontend/syntax | class: blocked | Implement Pathsvalidation |
| 3800 | Implement Performancecomparisonofstructurallyidenticalinterfaceswithgenericsignatures | spike | frontend/syntax | class: blocked | Implement Performancecomparisonofstructurallyidenticalinterfaceswithgenericsignatures |
| 3801 | Implement Pickoflargeobjectunionworks | spike | frontend/syntax | class: blocked | Implement Pickoflargeobjectunionworks |
| 3802 | Implement Potentiallyunassignedvariableincatch | spike | frontend/syntax | class: triage-needed | Implement Potentiallyunassignedvariableincatch |
| 3803 | Implement Potentiallyuncalleddecorators | spike | frontend/syntax | class: blocked | Implement Potentiallyuncalleddecorators |
| 3804 | Implement Predicatesemantics | spike | frontend/syntax | class: triage-needed | Implement Predicatesemantics |
| 3805 | Implement Prefixincrementasoperandofplusexpression | spike | frontend/syntax | class: triage-needed | Implement Prefixincrementasoperandofplusexpression |
| 3806 | Implement Prefixunaryoperatorsonexportedvariables | spike | frontend/syntax | class: blocked | Implement Prefixunaryoperatorsonexportedvariables |
| 3807 | Implement Preserveconstenums | spike | frontend/syntax | class: blocked | Implement Preserveconstenums |
| 3808 | Implement Preserveunusedimports | spike | frontend/syntax | class: blocked | Implement Preserveunusedimports |
| 3809 | Implement Prespecializedgenericmembers | spike | frontend/syntax | class: blocked | Implement Prespecializedgenericmembers |
| 3810 | Implement Prettycontextnotdebugassertion | spike | frontend/syntax | class: blocked | Implement Prettycontextnotdebugassertion |
| 3811 | Implement Prettyfilewitherrorsandtabs | spike | runtime/builtins | class: triage-needed | Implement Prettyfilewitherrorsandtabs |
| 3812 | Implement Primaryexpressionmods | spike | frontend/syntax | class: blocked | Implement Primaryexpressionmods |
| 3813 | Implement Primitiveconstraints | spike | frontend/syntax | class: blocked | Implement Primitiveconstraints |
| 3814 | Implement Primitivemembers | spike | frontend/syntax | class: triage-needed | Implement Primitivemembers |
| 3815 | Implement Primitivetypeasmodulename | spike | frontend/syntax | class: blocked | Implement Primitivetypeasmodulename |
| 3816 | Implement Primitivetypeassignment | spike | frontend/resolver | class: blocked | Implement Primitivetypeassignment |
| 3817 | Implement Primitiveuniondetection | spike | frontend/resolver | class: blocked | Implement Primitiveuniondetection |
| 3818 | Implement Privacyaccessordeclfile | spike | frontend/syntax | class: blocked | Implement Privacyaccessordeclfile |
| 3819 | Implement Privacycannotnameaccessordeclfile | spike | frontend/syntax | class: blocked | Implement Privacycannotnameaccessordeclfile |
| 3820 | Implement Privacycannotnamevartypedeclfile | spike | frontend/syntax | class: blocked | Implement Privacycannotnamevartypedeclfile |
| 3821 | Implement Privacycheckanonymousfunctionparameter | spike | frontend/syntax | class: blocked | Implement Privacycheckanonymousfunctionparameter |
| 3822 | Implement Privacycheckexportassignmentonexportedgenericinterface | spike | frontend/syntax | class: blocked | Implement Privacycheckexportassignmentonexportedgenericinterface |
| 3823 | Implement Privacycheckexternalmoduleexportassignmentofgenericclass | spike | frontend/syntax | class: blocked | Implement Privacycheckexternalmoduleexportassignmentofgenericclass |
| 3824 | Implement Privacycheckontypeparameterreferenceinconstructorparameter | spike | frontend/syntax | class: blocked | Implement Privacycheckontypeparameterreferenceinconstructorparameter |
| 3825 | Implement Privacychecktypeoffunction | spike | frontend/syntax | class: blocked | Implement Privacychecktypeoffunction |
| 3826 | Implement Privacychecktypeofinvisiblemoduleerror | spike | frontend/syntax | class: blocked | Implement Privacychecktypeofinvisiblemoduleerror |
| 3827 | Implement Privacychecktypeofinvisiblemodulenoerror | spike | frontend/syntax | class: blocked | Implement Privacychecktypeofinvisiblemodulenoerror |
| 3828 | Implement Privacyclass | spike | frontend/syntax | class: blocked | Implement Privacyclass |
| 3829 | Implement Privacyclassextendsclausedeclfile | spike | frontend/syntax | class: blocked | Implement Privacyclassextendsclausedeclfile |
| 3830 | Implement Privacyclassimplementsclausedeclfile | spike | frontend/syntax | class: blocked | Implement Privacyclassimplementsclausedeclfile |
| 3831 | Implement Privacyfunc | spike | frontend/syntax | class: blocked | Implement Privacyfunc |
| 3832 | Implement Privacyfunctioncannotnameparametertypedeclfile | spike | frontend/syntax | class: blocked | Implement Privacyfunctioncannotnameparametertypedeclfile |
| 3833 | Implement Privacyfunctioncannotnamereturntypedeclfile | spike | frontend/syntax | class: blocked | Implement Privacyfunctioncannotnamereturntypedeclfile |
| 3834 | Implement Privacyfunctionparameterdeclfile | spike | frontend/syntax | class: blocked | Implement Privacyfunctionparameterdeclfile |
| 3835 | Implement Privacyfunctionreturntypedeclfile | spike | frontend/syntax | class: blocked | Implement Privacyfunctionreturntypedeclfile |
| 3836 | Implement Privacygetter | spike | frontend/syntax | class: blocked | Implement Privacygetter |
| 3837 | Implement Privacygloclass | spike | frontend/syntax | class: blocked | Implement Privacygloclass |
| 3838 | Implement Privacyglofunc | spike | frontend/syntax | class: blocked | Implement Privacyglofunc |
| 3839 | Implement Privacyglogetter | spike | frontend/syntax | class: blocked | Implement Privacyglogetter |
| 3840 | Implement Privacygloimport | spike | frontend/syntax | class: blocked | Implement Privacygloimport |
| 3841 | Implement Privacygloimportparseerrors | spike | frontend/syntax | class: blocked | Implement Privacygloimportparseerrors |
| 3842 | Implement Privacyglointerface | spike | frontend/syntax | class: blocked | Implement Privacyglointerface |
| 3843 | Implement Privacyglovar | spike | frontend/syntax | class: blocked | Implement Privacyglovar |
| 3844 | Implement Privacyimport | spike | frontend/syntax | class: blocked | Implement Privacyimport |
| 3845 | Implement Privacyimportparseerrors | spike | frontend/syntax | class: blocked | Implement Privacyimportparseerrors |
| 3846 | Implement Privacyinterface | spike | frontend/syntax | class: blocked | Implement Privacyinterface |
| 3847 | Implement Privacyinterfaceextendsclausedeclfile | spike | frontend/syntax | class: blocked | Implement Privacyinterfaceextendsclausedeclfile |
| 3848 | Implement Privacylocalinternalreferenceimportwithexport | spike | frontend/syntax | class: blocked | Implement Privacylocalinternalreferenceimportwithexport |
| 3849 | Implement Privacylocalinternalreferenceimportwithoutexport | spike | frontend/syntax | class: blocked | Implement Privacylocalinternalreferenceimportwithoutexport |
| 3850 | Implement Privacytoplevelambientexternalmoduleimportwithexport | spike | frontend/syntax | class: blocked | Implement Privacytoplevelambientexternalmoduleimportwithexport |
| 3851 | Implement Privacytoplevelambientexternalmoduleimportwithoutexport | spike | frontend/syntax | class: blocked | Implement Privacytoplevelambientexternalmoduleimportwithoutexport |
| 3852 | Implement Privacytoplevelinternalreferenceimportwithexport | spike | frontend/syntax | class: blocked | Implement Privacytoplevelinternalreferenceimportwithexport |
| 3853 | Implement Privacytoplevelinternalreferenceimportwithoutexport | spike | frontend/syntax | class: blocked | Implement Privacytoplevelinternalreferenceimportwithoutexport |
| 3854 | Implement Privacytypeparameteroffunction | spike | frontend/syntax | class: blocked | Implement Privacytypeparameteroffunction |
| 3855 | Implement Privacytypeparameteroffunctiondeclfile | spike | frontend/syntax | class: blocked | Implement Privacytypeparameteroffunctiondeclfile |
| 3856 | Implement Privacytypeparametersofclass | spike | frontend/syntax | class: blocked | Implement Privacytypeparametersofclass |
| 3857 | Implement Privacytypeparametersofclassdeclfile | spike | frontend/syntax | class: blocked | Implement Privacytypeparametersofclassdeclfile |
| 3858 | Implement Privacytypeparametersofinterface | spike | frontend/syntax | class: blocked | Implement Privacytypeparametersofinterface |
| 3859 | Implement Privacytypeparametersofinterfacedeclfile | spike | frontend/syntax | class: blocked | Implement Privacytypeparametersofinterfacedeclfile |
| 3860 | Implement Privacyvar | spike | frontend/syntax | class: blocked | Implement Privacyvar |
| 3861 | Implement Privacyvardeclfile | spike | frontend/syntax | class: blocked | Implement Privacyvardeclfile |
| 3862 | Implement Privateaccessinsubclass | spike | frontend/syntax | class: blocked | Implement Privateaccessinsubclass |
| 3863 | Implement Privatefieldassignabilityfromunknown | spike | frontend/syntax | class: blocked | Implement Privatefieldassignabilityfromunknown |
| 3864 | Implement Privatefieldsinclassexpressiondeclaration | spike | frontend/syntax | class: blocked | Implement Privatefieldsinclassexpressiondeclaration |
| 3865 | Implement Privateinstancevisibility | spike | frontend/syntax | class: blocked | Implement Privateinstancevisibility |
| 3866 | Implement Privateinterfaceproperties | spike | frontend/syntax | class: blocked | Implement Privateinterfaceproperties |
| 3867 | Implement Privatepropertyinunion | spike | frontend/syntax | class: blocked | Implement Privatepropertyinunion |
| 3868 | Implement Privatepropertyusingobjecttype | spike | frontend/syntax | class: blocked | Implement Privatepropertyusingobjecttype |
| 3869 | Implement Privatevisibility | spike | frontend/syntax | class: blocked | Implement Privatevisibility |
| 3870 | Implement Privatevisibles | spike | frontend/syntax | class: blocked | Implement Privatevisibles |
| 3871 | Implement Promiseallonany | spike | reference/triage | class: triage-needed | Implement Promiseallonany |
| 3872 | Implement Promisechaining | spike | runtime/builtins | class: triage-needed | Implement Promisechaining |
| 3873 | Implement Promisedefinitiontest | spike | runtime/builtins | class: triage-needed | Implement Promisedefinitiontest |
| 3874 | Implement Promiseemptytuplenoexception | spike | frontend/syntax | class: blocked | Implement Promiseemptytuplenoexception |
| 3875 | Implement Promiseidentity | spike | runtime/builtins | class: triage-needed | Implement Promiseidentity |
| 3876 | Implement Promiseidentitywithany | spike | runtime/builtins | class: triage-needed | Implement Promiseidentitywithany |
| 3877 | Implement Promiseidentitywithconstraints | spike | frontend/syntax | class: blocked | Implement Promiseidentitywithconstraints |
| 3878 | Implement Promisepermutations | spike | frontend/resolver | class: blocked | Implement Promisepermutations |
| 3879 | Implement Promisetry | spike | runtime/builtins | class: triage-needed | Implement Promisetry |
| 3880 | Implement Promisetype | spike | reference/triage | class: triage-needed | Implement Promisetype |
| 3881 | Implement Promisetypeinference | spike | frontend/semantics | class: blocked | Implement Promisetypeinference |
| 3882 | Implement Promisetypeinferenceunion | spike | frontend/resolver | class: blocked | Implement Promisetypeinferenceunion |
| 3883 | Implement Promisetypestrictnull | spike | reference/triage | class: triage-needed | Implement Promisetypestrictnull |
| 3884 | Implement Promisevoiderrorcallback | spike | frontend/syntax | class: blocked | Implement Promisevoiderrorcallback |
| 3885 | Implement Promisewithresolvers | spike | frontend/syntax | class: triage-needed | Implement Promisewithresolvers |
| 3886 | Implement Promiseswithconstraints | spike | frontend/syntax | class: triage-needed | Implement Promiseswithconstraints |
| 3887 | Implement Proptypevalidatorinference | spike | frontend/semantics | class: blocked | Implement Proptypevalidatorinference |
| 3888 | Implement Propagatenoninferrabletype | spike | frontend/resolver | class: blocked | Implement Propagatenoninferrabletype |
| 3889 | Implement Propagationofpromiseinitialization | spike | frontend/syntax | class: blocked | Implement Propagationofpromiseinitialization |
| 3890 | Implement Properties | spike | frontend/syntax | class: triage-needed | Implement Properties |
| 3891 | Implement Propertiesandindexers | spike | frontend/syntax | class: triage-needed | Implement Propertiesandindexers |
| 3892 | Implement Propertiesandindexersfornumericnames | spike | frontend/syntax | class: triage-needed | Implement Propertiesandindexersfornumericnames |
| 3893 | Implement Propertyaccess Method Call | spike | frontend/syntax | class: blocked | Implement Propertyaccess Method Call |
| 3894 | Implement Propertyaccess Name Resolution | spike | frontend/resolver | class: blocked | Implement Propertyaccess Name Resolution |
| 3895 | Implement Propertyaccessexpressioninnercomments | spike | frontend/syntax | class: triage-needed | Implement Propertyaccessexpressioninnercomments |
| 3896 | Implement Propertyaccessofreadonlyindexsignature | spike | frontend/resolver | class: blocked | Implement Propertyaccessofreadonlyindexsignature |
| 3897 | Implement Propertyaccessonobjectliteral | spike | frontend/syntax | class: blocked | Implement Propertyaccessonobjectliteral |
| 3898 | Implement Propertyaccessibility | spike | frontend/syntax | class: triage-needed | Implement Propertyaccessibility |
| 3899 | Implement Propertyassignment | spike | frontend/syntax | class: triage-needed | Implement Propertyassignment |
| 3900 | Implement Propertyidentitywithprivacymismatch | spike | frontend/syntax | class: blocked | Implement Propertyidentitywithprivacymismatch |
| 3901 | Implement Propertynameswithstringliteral | spike | frontend/syntax | class: blocked | Implement Propertynameswithstringliteral |
| 3902 | Implement Propertyordering | spike | frontend/syntax | class: triage-needed | Implement Propertyordering |
| 3903 | Implement Propertyparameterwithquestionmark | spike | frontend/syntax | class: triage-needed | Implement Propertyparameterwithquestionmark |
| 3904 | Implement Propertysignatures | spike | reference/triage | class: triage-needed | Implement Propertysignatures |
| 3905 | Implement Propertywrappedintry | spike | frontend/syntax | class: triage-needed | Implement Propertywrappedintry |
| 3906 | Implement Protectedaccessthroughcontextualthis | spike | frontend/syntax | class: blocked | Implement Protectedaccessthroughcontextualthis |
| 3907 | Implement Protectedmembers | spike | frontend/syntax | class: blocked | Implement Protectedmembers |
| 3908 | Implement Protectedmembersthisparameter | spike | frontend/syntax | class: blocked | Implement Protectedmembersthisparameter |
| 3909 | Implement Protoasindexinindexexpression | spike | frontend/syntax | class: blocked | Implement Protoasindexinindexexpression |
| 3910 | Implement Protoassignment | spike | reference/triage | class: triage-needed | Implement Protoassignment |
| 3911 | Implement Prototypeinstantiatedwithbaseconstraint | spike | frontend/syntax | class: blocked | Implement Prototypeinstantiatedwithbaseconstraint |
| 3912 | Implement Prototypeonconstructorfunctions | spike | frontend/syntax | class: triage-needed | Implement Prototypeonconstructorfunctions |
| 3913 | Implement Prototypes | spike | frontend/syntax | class: blocked | Implement Prototypes |
| 3914 | Implement Publicgetterprotectedsetterfromthisparameter | spike | frontend/syntax | class: blocked | Implement Publicgetterprotectedsetterfromthisparameter |
| 3915 | Implement Publicmemberimplementedasprivateinderivedclass | spike | frontend/syntax | class: blocked | Implement Publicmemberimplementedasprivateinderivedclass |
| 3916 | Implement Pushtypegettypeofalias | spike | frontend/syntax | class: blocked | Implement Pushtypegettypeofalias |
| 3917 | Implement Qualifiedmodulelocals | spike | frontend/syntax | class: blocked | Implement Qualifiedmodulelocals |
| 3918 | Implement Qualifiedname | spike | frontend/syntax | class: blocked | Implement Qualifiedname |
| 3919 | Implement Qualify | spike | frontend/syntax | class: blocked | Implement Qualify |
| 3920 | Implement Quickintersectioncheckcorrectlycacheserrors | spike | frontend/syntax | class: blocked | Implement Quickintersectioncheckcorrectlycacheserrors |
| 3921 | Implement Quickinfotypeatreturnpositionsinaccurate | spike | frontend/syntax | class: triage-needed | Implement Quickinfotypeatreturnpositionsinaccurate |
| 3922 | Implement Quotedaccessorname | spike | frontend/syntax | class: blocked | Implement Quotedaccessorname |
| 3923 | Implement Quotedfunctionname | spike | frontend/syntax | class: triage-needed | Implement Quotedfunctionname |
| 3924 | Implement Quotedmodulenamemustbeambient | spike | frontend/syntax | class: blocked | Implement Quotedmodulenamemustbeambient |
| 3925 | Implement Quotedpropertyname | spike | frontend/syntax | class: triage-needed | Implement Quotedpropertyname |
| 3926 | Implement Ramdatoolsnoinfinite | spike | frontend/syntax | class: blocked | Implement Ramdatoolsnoinfinite |
| 3927 | Implement Reexportglobaldeclaration Import Export | spike | frontend/syntax | class: blocked | Implement Reexportglobaldeclaration Import Export |
| 3928 | Implement Reexportglobaldeclaration Parser Syntax | spike | frontend/syntax | class: blocked | Implement Reexportglobaldeclaration Parser Syntax |
| 3929 | Implement Reexportundefined | spike | frontend/syntax | class: blocked | Implement Reexportundefined |
| 3930 | Implement Reachabilitychecks Arrow Function | spike | frontend/syntax | class: blocked | Implement Reachabilitychecks Arrow Function |
| 3931 | Implement Reachabilitychecks Import Export | spike | frontend/syntax | class: blocked | Implement Reachabilitychecks Import Export |
| 3932 | Implement Reachabilitychecks Name Resolution | spike | frontend/resolver | class: blocked | Implement Reachabilitychecks Name Resolution |
| 3933 | Implement Reachabilitychecks Parser Syntax | spike | frontend/syntax | class: blocked | Implement Reachabilitychecks Parser Syntax |
| 3934 | Implement Reachabilitychecks Runtime Subset | spike | reference/triage | class: triage-needed | Implement Reachabilitychecks Runtime Subset |
| 3935 | Implement Reachabilitychecks Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Reachabilitychecks Unknown Unsupported |
| 3936 | Implement Reachabilitychecksnocrash | spike | frontend/syntax | class: blocked | Implement Reachabilitychecksnocrash |
| 3937 | Implement Reactimportdropped | spike | frontend/syntax | class: blocked | Implement Reactimportdropped |
| 3938 | Implement Reactreduxlikedeferredinferenceallowsassignment | spike | frontend/semantics | class: blocked | Implement Reactreduxlikedeferredinferenceallowsassignment |
| 3939 | Implement Reacttransitiveimporthasvaliddeclaration | spike | frontend/syntax | class: triage-needed | Implement Reacttransitiveimporthasvaliddeclaration |
| 3940 | Implement Readonlyassignmentinsubclassofclassexpression | spike | frontend/syntax | class: triage-needed | Implement Readonlyassignmentinsubclassofclassexpression |
| 3941 | Implement Readonlyfloat | spike | frontend/resolver | class: blocked | Implement Readonlyfloat |
| 3942 | Implement Readonlyindeclarationfile | spike | frontend/syntax | class: blocked | Implement Readonlyindeclarationfile |
| 3943 | Implement Readonlyinnonpropertyparameters | spike | frontend/syntax | class: blocked | Implement Readonlyinnonpropertyparameters |
| 3944 | Implement Readonlymembers | spike | frontend/syntax | class: blocked | Implement Readonlymembers |
| 3945 | Implement Readonlypropertysubtyperelationdirected | spike | reference/triage | class: triage-needed | Implement Readonlypropertysubtyperelationdirected |
| 3946 | Implement Readonlytupleandarrayelaboration | spike | frontend/syntax | class: blocked | Implement Readonlytupleandarrayelaboration |
| 3947 | Implement Reassignstaticprop | spike | frontend/syntax | class: triage-needed | Implement Reassignstaticprop |
| 3948 | Implement Reboundbaseclasssymbol | spike | frontend/syntax | class: blocked | Implement Reboundbaseclasssymbol |
| 3949 | Implement Reboundidentifieronimportalias | spike | frontend/syntax | class: blocked | Implement Reboundidentifieronimportalias |
| 3950 | Implement Rectype | spike | frontend/syntax | class: blocked | Implement Rectype |
| 3951 | Implement Recur | spike | frontend/syntax | class: blocked | Implement Recur |
| 3952 | Implement Recursivearraynotcircular | spike | frontend/syntax | class: triage-needed | Implement Recursivearraynotcircular |
| 3953 | Implement Recursivebasecheck Import Export | spike | frontend/syntax | class: blocked | Implement Recursivebasecheck Import Export |
| 3954 | Implement Recursivebasecheck Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Recursivebasecheck Parser Syntax |
| 3955 | Implement Recursivebaseconstructorcreation Name Resolution | spike | frontend/resolver | class: blocked | Implement Recursivebaseconstructorcreation Name Resolution |
| 3956 | Implement Recursivebaseconstructorcreation Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Recursivebaseconstructorcreation Parser Syntax |
| 3957 | Implement Recursiveclassbasetype | spike | frontend/syntax | class: triage-needed | Implement Recursiveclassbasetype |
| 3958 | Implement Recursiveclassinstantiationswithdefaultconstructors | spike | frontend/syntax | class: blocked | Implement Recursiveclassinstantiationswithdefaultconstructors |
| 3959 | Implement Recursiveclassreferencetest | spike | frontend/syntax | class: blocked | Implement Recursiveclassreferencetest |
| 3960 | Implement Recursiveclodulereference | spike | frontend/syntax | class: blocked | Implement Recursiveclodulereference |
| 3961 | Implement Recursivecomplicatedclasses | spike | frontend/syntax | class: triage-needed | Implement Recursivecomplicatedclasses |
| 3962 | Implement Recursiveconditionalcrash | spike | frontend/semantics | class: blocked | Implement Recursiveconditionalcrash |
| 3963 | Implement Recursiveconditionalevaluationnoninfinite | spike | frontend/resolver | class: blocked | Implement Recursiveconditionalevaluationnoninfinite |
| 3964 | Implement Recursiveconditionaltypes | spike | frontend/semantics | class: blocked | Implement Recursiveconditionaltypes |
| 3965 | Implement Recursiveexportassignmentandfindaliasedtype | spike | frontend/syntax | class: blocked | Implement Recursiveexportassignmentandfindaliasedtype |
| 3966 | Implement Recursivefieldsetting | spike | frontend/syntax | class: triage-needed | Implement Recursivefieldsetting |
| 3967 | Implement Recursivefunctiontypes | spike | frontend/resolver | class: blocked | Implement Recursivefunctiontypes |
| 3968 | Implement Recursivegenericuniontype | spike | frontend/syntax | class: blocked | Implement Recursivegenericuniontype |
| 3969 | Implement Recursiveidenticalassignment | spike | frontend/syntax | class: triage-needed | Implement Recursiveidenticalassignment |
| 3970 | Implement Recursiveidenticaloverloadresolution | spike | frontend/syntax | class: blocked | Implement Recursiveidenticaloverloadresolution |
| 3971 | Implement Recursiveinference | spike | frontend/semantics | class: blocked | Implement Recursiveinference |
| 3972 | Implement Recursiveinferencebug | spike | frontend/semantics | class: blocked | Implement Recursiveinferencebug |
| 3973 | Implement Recursiveinheritance Name Resolution | spike | frontend/resolver | class: blocked | Implement Recursiveinheritance Name Resolution |
| 3974 | Implement Recursiveinheritance Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Recursiveinheritance Parser Syntax |
| 3975 | Implement Recursiveletconst | spike | frontend/syntax | class: triage-needed | Implement Recursiveletconst |
| 3976 | Implement Recursivemods | spike | frontend/syntax | class: blocked | Implement Recursivemods |
| 3977 | Implement Recursivenamedlambdacall | spike | frontend/resolver | class: blocked | Implement Recursivenamedlambdacall |
| 3978 | Implement Recursivereturns | spike | frontend/syntax | class: triage-needed | Implement Recursivereturns |
| 3979 | Implement Recursivereversemappedtype | spike | frontend/syntax | class: blocked | Implement Recursivereversemappedtype |
| 3980 | Implement Recursivespecializationofsignatures | spike | frontend/syntax | class: triage-needed | Implement Recursivespecializationofsignatures |
| 3981 | Implement Recursivetypealiaswithspreadconditionalreturnnotcircular | spike | frontend/syntax | class: blocked | Implement Recursivetypealiaswithspreadconditionalreturnnotcircular |
| 3982 | Implement Recursivetypecomparison | spike | frontend/syntax | class: blocked | Implement Recursivetypecomparison |
| 3983 | Implement Recursivetypeparameterconstraintreferencelackstypeargs | spike | frontend/syntax | class: blocked | Implement Recursivetypeparameterconstraintreferencelackstypeargs |
| 3984 | Implement Recursivetypeparameterreferenceerror | spike | frontend/syntax | class: blocked | Implement Recursivetypeparameterreferenceerror |
| 3985 | Implement Recursivetyperelations | spike | frontend/syntax | class: blocked | Implement Recursivetyperelations |
| 3986 | Implement Recursivelyspecializedconstructordeclaration | spike | frontend/syntax | class: blocked | Implement Recursivelyspecializedconstructordeclaration |
| 3987 | Implement Redeclarationofvarwithgenerictype | spike | reference/triage | class: triage-needed | Implement Redeclarationofvarwithgenerictype |
| 3988 | Implement Redeclareparameterincatchblock | spike | frontend/syntax | class: blocked | Implement Redeclareparameterincatchblock |
| 3989 | Implement Redefinearray | spike | frontend/resolver | class: blocked | Implement Redefinearray |
| 3990 | Implement Reducibleindexedaccesstypes | spike | frontend/syntax | class: blocked | Implement Reducibleindexedaccesstypes |
| 3991 | Implement Reexportdefaultiscallable | spike | frontend/syntax | class: blocked | Implement Reexportdefaultiscallable |
| 3992 | Implement Reexportmissingdefault | spike | frontend/syntax | class: blocked | Implement Reexportmissingdefault |
| 3993 | Implement Reexportnamealiasedandhoisted | spike | frontend/syntax | class: blocked | Implement Reexportnamealiasedandhoisted |
| 3994 | Implement Reexportwrittencorrectlyindeclaration | spike | frontend/syntax | class: blocked | Implement Reexportwrittencorrectlyindeclaration |
| 3995 | Implement Reexportedmissingalias | spike | frontend/syntax | class: blocked | Implement Reexportedmissingalias |
| 3997 | Implement Referencesatisfiesexpression | spike | frontend/syntax | class: blocked | Implement Referencesatisfiesexpression |
| 3998 | Implement Referencetypespreferedtopathifpossible | spike | frontend/syntax | class: blocked | Implement Referencetypespreferedtopathifpossible |
| 3999 | Implement Regexpwithopenbracketincharclass | spike | reference/triage | class: triage-needed | Implement Regexpwithopenbracketincharclass |
| 4000 | Implement Regexpwithslashincharclass | spike | reference/triage | class: triage-needed | Implement Regexpwithslashincharclass |
| 4001 | Implement Regexmatchall | spike | frontend/syntax | class: blocked | Implement Regexmatchall |
| 4002 | Implement Regexpexecandmatchtypeusages | spike | frontend/syntax | class: blocked | Implement Regexpexecandmatchtypeusages |
| 4003 | Implement Regularexpressioncharacterclassrangeorder | spike | reference/triage | class: triage-needed | Implement Regularexpressioncharacterclassrangeorder |
| 4004 | Implement Regularexpressionextendedunicodeescapes | spike | reference/triage | class: triage-needed | Implement Regularexpressionextendedunicodeescapes |
| 4005 | Implement Regularexpressionscanning | spike | reference/triage | class: triage-needed | Implement Regularexpressionscanning |
| 4006 | Implement Regularexpressionwithnonbmpflags | spike | frontend/syntax | class: triage-needed | Implement Regularexpressionwithnonbmpflags |
| 4007 | Implement Relatedviadiscriminatedtypenoerror Name Resolution | spike | frontend/resolver | class: blocked | Implement Relatedviadiscriminatedtypenoerror Name Resolution |
| 4008 | Implement Relatedviadiscriminatedtypenoerror Parser Syntax | spike | frontend/syntax | class: blocked | Implement Relatedviadiscriminatedtypenoerror Parser Syntax |
| 4009 | Implement Relativenamesinclassicresolution | spike | frontend/syntax | class: blocked | Implement Relativenamesinclassicresolution |
| 4010 | Implement Renamingdestructuredpropertyinfunctiontype | spike | frontend/syntax | class: blocked | Implement Renamingdestructuredpropertyinfunctiontype |
| 4011 | Implement Reorderproperties | spike | frontend/syntax | class: triage-needed | Implement Reorderproperties |
| 4012 | Implement Requireasfunctioninexternalmodule | spike | frontend/syntax | class: blocked | Implement Requireasfunctioninexternalmodule |
| 4013 | Implement Requireemitsemicolon | spike | frontend/syntax | class: blocked | Implement Requireemitsemicolon |
| 4014 | Implement Requireofanemptyfile | spike | frontend/syntax | class: blocked | Implement Requireofanemptyfile |
| 4015 | Implement Requireofjsonfile | spike | frontend/syntax | class: blocked | Implement Requireofjsonfile |
| 4016 | Implement Requireofjsonfileinjsfile | spike | frontend/syntax | class: blocked | Implement Requireofjsonfileinjsfile |
| 4017 | Implement Requireofjsonfilenonrelative | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilenonrelative |
| 4018 | Implement Requireofjsonfilenonrelativewithoutextension | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilenonrelativewithoutextension |
| 4019 | Implement Requireofjsonfilenonrelativewithoutextensionresolvestots | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilenonrelativewithoutextensionresolvestots |
| 4020 | Implement Requireofjsonfiletypes | spike | frontend/syntax | class: blocked | Implement Requireofjsonfiletypes |
| 4021 | Implement Requireofjsonfilewithalwaysstrictwithouterrors | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithalwaysstrictwithouterrors |
| 4022 | Implement Requireofjsonfilewithamd | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithamd |
| 4023 | Implement Requireofjsonfilewithcomputedpropertyname | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithcomputedpropertyname |
| 4024 | Implement Requireofjsonfilewithdeclaration | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithdeclaration |
| 4025 | Implement Requireofjsonfilewithemptyobject | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithemptyobject |
| 4026 | Implement Requireofjsonfilewithemptyobjectwitherrors | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithemptyobjectwitherrors |
| 4027 | Implement Requireofjsonfilewitherrors | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewitherrors |
| 4028 | Implement Requireofjsonfilewithmoduleemitnone | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithmoduleemitnone |
| 4029 | Implement Requireofjsonfilewithmoduleemitundefined | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithmoduleemitundefined |
| 4030 | Implement Requireofjsonfilewithmodulenoderesolutionemitamd | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithmodulenoderesolutionemitamd |
| 4031 | Implement Requireofjsonfilewithmodulenoderesolutionemitamdoutfile | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithmodulenoderesolutionemitamdoutfile |
| 4032 | Implement Requireofjsonfilewithmodulenoderesolutionemites | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithmodulenoderesolutionemites |
| 4033 | Implement Requireofjsonfilewithmodulenoderesolutionemitesnext | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithmodulenoderesolutionemitesnext |
| 4034 | Implement Requireofjsonfilewithmodulenoderesolutionemitnone | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithmodulenoderesolutionemitnone |
| 4035 | Implement Requireofjsonfilewithmodulenoderesolutionemitsystem | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithmodulenoderesolutionemitsystem |
| 4036 | Implement Requireofjsonfilewithmodulenoderesolutionemitumd | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithmodulenoderesolutionemitumd |
| 4037 | Implement Requireofjsonfilewithmodulenoderesolutionemitundefined | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithmodulenoderesolutionemitundefined |
| 4038 | Implement Requireofjsonfilewithnocontent | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithnocontent |
| 4039 | Implement Requireofjsonfilewithsourcemap | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithsourcemap |
| 4040 | Implement Requireofjsonfilewithtraillingcomma | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithtraillingcomma |
| 4041 | Implement Requireofjsonfilewithoutallowjs | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithoutallowjs |
| 4042 | Implement Requireofjsonfilewithoutesmoduleinterop | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithoutesmoduleinterop |
| 4043 | Implement Requireofjsonfilewithoutextension | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithoutextension |
| 4044 | Implement Requireofjsonfilewithoutextensionresolvestots | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithoutextensionresolvestots |
| 4045 | Implement Requireofjsonfilewithoutoutdir | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithoutoutdir |
| 4046 | Implement Requireofjsonfilewithoutresolvejsonmodule | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithoutresolvejsonmodule |
| 4047 | Implement Requireofjsonfilewithoutresolvejsonmoduleandpathmapping | spike | frontend/syntax | class: blocked | Implement Requireofjsonfilewithoutresolvejsonmoduleandpathmapping |
| 4048 | Implement Requiredinitializedparameter | spike | frontend/syntax | class: triage-needed | Implement Requiredinitializedparameter |
| 4049 | Implement Requiredmappedtypemodifiertrumpsvariance | spike | frontend/syntax | class: blocked | Implement Requiredmappedtypemodifiertrumpsvariance |
| 4050 | Implement Reservednameoninterfaceimport | spike | frontend/syntax | class: blocked | Implement Reservednameoninterfaceimport |
| 4051 | Implement Reservednameonmoduleimport | spike | frontend/syntax | class: blocked | Implement Reservednameonmoduleimport |
| 4052 | Implement Reservednameonmoduleimportwithinterface | spike | frontend/syntax | class: blocked | Implement Reservednameonmoduleimportwithinterface |
| 4053 | Implement Reservedwords Import Export | spike | frontend/syntax | class: blocked | Implement Reservedwords Import Export |
| 4054 | Implement Reservedwords Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Reservedwords Parser Syntax |
| 4055 | Implement Resolutioncandidatefrompackagejsonfield Import Export | spike | frontend/syntax | class: blocked | Implement Resolutioncandidatefrompackagejsonfield Import Export |
| 4056 | Implement Resolutioncandidatefrompackagejsonfield Module Resolution | spike | frontend/syntax | class: blocked | Implement Resolutioncandidatefrompackagejsonfield Module Resolution |
| 4057 | Implement Resolvemodulenamewithsameletdeclarationname | spike | frontend/syntax | class: blocked | Implement Resolvemodulenamewithsameletdeclarationname |
| 4058 | Implement Resolvenamewithnamspace | spike | frontend/syntax | class: blocked | Implement Resolvenamewithnamspace |
| 4059 | Implement Resolvingclassdeclarationwheninbasetyperesolution | spike | frontend/syntax | class: blocked | Implement Resolvingclassdeclarationwheninbasetyperesolution |
| 4060 | Implement Restargassignmentcompat | spike | frontend/syntax | class: blocked | Implement Restargassignmentcompat |
| 4061 | Implement Restargmissingname | spike | frontend/syntax | class: triage-needed | Implement Restargmissingname |
| 4062 | Implement Restelementassignable | spike | frontend/syntax | class: triage-needed | Implement Restelementassignable |
| 4063 | Implement Restelementwithnumberpropertyname | spike | reference/triage | class: triage-needed | Implement Restelementwithnumberpropertyname |
| 4064 | Implement Restintersection | spike | reference/triage | class: triage-needed | Implement Restintersection |
| 4065 | Implement Restinvalidargumenttype | spike | frontend/syntax | class: triage-needed | Implement Restinvalidargumenttype |
| 4066 | Implement Restparammodifier | spike | frontend/syntax | class: blocked | Implement Restparammodifier |
| 4067 | Implement Restparamusingmappedtypeoverunionconstraint | spike | frontend/syntax | class: blocked | Implement Restparamusingmappedtypeoverunionconstraint |
| 4068 | Implement Restparameternotlast | spike | frontend/syntax | class: blocked | Implement Restparameternotlast |
| 4069 | Implement Restparametertypeinstantiation | spike | frontend/syntax | class: triage-needed | Implement Restparametertypeinstantiation |
| 4070 | Implement Restparameterwithbindingpattern | spike | frontend/syntax | class: blocked | Implement Restparameterwithbindingpattern |
| 4071 | Implement Restparamswithnonrestparams | spike | reference/triage | class: triage-needed | Implement Restparamswithnonrestparams |
| 4072 | Implement Resttyperetainsmappyness | spike | frontend/syntax | class: triage-needed | Implement Resttyperetainsmappyness |
| 4073 | Implement Restunion | spike | reference/triage | class: triage-needed | Implement Restunion |
| 4074 | Implement Returnconditionalexpressionjsdoccast | spike | frontend/semantics | class: blocked | Implement Returnconditionalexpressionjsdoccast |
| 4075 | Implement Returninconstructor | spike | frontend/syntax | class: triage-needed | Implement Returninconstructor |
| 4076 | Implement Returninfiniteintersection | spike | frontend/syntax | class: triage-needed | Implement Returninfiniteintersection |
| 4077 | Implement Returntypeinferencecontextualparametertypesingenerator | spike | frontend/syntax | class: blocked | Implement Returntypeinferencecontextualparametertypesingenerator |
| 4078 | Implement Returntypeinferencecontextualtypeignoreanyunknown | spike | frontend/resolver | class: blocked | Implement Returntypeinferencecontextualtypeignoreanyunknown |
| 4079 | Implement Returntypeinferencenottoobroad | spike | frontend/syntax | class: blocked | Implement Returntypeinferencenottoobroad |
| 4080 | Implement Returntypeparameter | spike | frontend/resolver | class: blocked | Implement Returntypeparameter |
| 4081 | Implement Returntypeparameterwithmodules | spike | frontend/syntax | class: blocked | Implement Returntypeparameterwithmodules |
| 4082 | Implement Returntypetypearguments | spike | frontend/syntax | class: blocked | Implement Returntypetypearguments |
| 4083 | Implement Reuseinnermodulemember | spike | frontend/syntax | class: blocked | Implement Reuseinnermodulemember |
| 4084 | Implement Reusetypeannotationimporttypeinglobalthistypeargument | spike | frontend/syntax | class: blocked | Implement Reusetypeannotationimporttypeinglobalthistypeargument |
| 4085 | Implement Reverseinferenceincontextualinstantiation | spike | frontend/semantics | class: blocked | Implement Reverseinferenceincontextualinstantiation |
| 4086 | Implement Reversemappedcontravariantinference | spike | frontend/resolver | class: blocked | Implement Reversemappedcontravariantinference |
| 4087 | Implement Reversemappedintersectioninference | spike | frontend/semantics | class: blocked | Implement Reversemappedintersectioninference |
| 4088 | Implement Reversemappedpartiallyinferabletypes | spike | frontend/semantics | class: blocked | Implement Reversemappedpartiallyinferabletypes |
| 4089 | Implement Reversemappedtuplecontext | spike | frontend/resolver | class: blocked | Implement Reversemappedtuplecontext |
| 4090 | Implement Reversemappedtypecontextualtypenotcircular | spike | frontend/resolver | class: blocked | Implement Reversemappedtypecontextualtypenotcircular |
| 4091 | Implement Reversemappedtypecontextualtypesperelementoftupleconstraint | spike | frontend/resolver | class: blocked | Implement Reversemappedtypecontextualtypesperelementoftupleconstraint |
| 4092 | Implement Reversemappedtypedeepdeclarationemit | spike | frontend/syntax | class: blocked | Implement Reversemappedtypedeepdeclarationemit |
| 4093 | Implement Reversemappedtypeinferencesamesource | spike | frontend/semantics | class: blocked | Implement Reversemappedtypeinferencesamesource |
| 4094 | Implement Reversemappedtypeinferencewidening Name Resolution | spike | frontend/resolver | class: blocked | Implement Reversemappedtypeinferencewidening Name Resolution |
| 4095 | Implement Reversemappedtypeinferencewidening Type System | spike | frontend/semantics | class: blocked | Implement Reversemappedtypeinferencewidening Type System |
| 4096 | Implement Reversemappedtypeintersectionconstraint | spike | frontend/syntax | class: triage-needed | Implement Reversemappedtypeintersectionconstraint |
| 4097 | Implement Reversemappedtypelimitedconstraint | spike | frontend/syntax | class: triage-needed | Implement Reversemappedtypelimitedconstraint |
| 4098 | Implement Reversemappedtypeprimitiveconstraintproperty | spike | frontend/resolver | class: blocked | Implement Reversemappedtypeprimitiveconstraintproperty |
| 4099 | Implement Reversemappedunioninference | spike | frontend/semantics | class: blocked | Implement Reversemappedunioninference |
| 4100 | Implement Reversedrecusivetypeinstantiation | spike | frontend/syntax | class: triage-needed | Implement Reversedrecusivetypeinstantiation |
| 4101 | Implement Satisfiesemit | spike | frontend/syntax | class: blocked | Implement Satisfiesemit |
| 4102 | Implement Scopecheckclassproperty | spike | frontend/resolver | class: blocked | Implement Scopecheckclassproperty |
| 4103 | Implement Scopecheckextendedclassinsidepublicmethod | spike | frontend/resolver | class: blocked | Implement Scopecheckextendedclassinsidepublicmethod |
| 4104 | Implement Scopecheckextendedclassinsidestaticmethod | spike | frontend/resolver | class: blocked | Implement Scopecheckextendedclassinsidestaticmethod |
| 4105 | Implement Scopecheckinsidepublicmethod | spike | frontend/resolver | class: blocked | Implement Scopecheckinsidepublicmethod |
| 4106 | Implement Scopecheckinsidestaticmethod | spike | frontend/resolver | class: blocked | Implement Scopecheckinsidestaticmethod |
| 4107 | Implement Scopecheckstaticinitializer | spike | frontend/resolver | class: blocked | Implement Scopecheckstaticinitializer |
| 4108 | Implement Scopetests | spike | frontend/resolver | class: blocked | Implement Scopetests |
| 4109 | Implement Scopingincatchblocks | spike | frontend/resolver | class: blocked | Implement Scopingincatchblocks |
| 4110 | Implement Selfincallback | spike | frontend/syntax | class: triage-needed | Implement Selfincallback |
| 4111 | Implement Selfinlambdas | spike | frontend/syntax | class: blocked | Implement Selfinlambdas |
| 4112 | Implement Selfnameandimportsemitinclusion | spike | frontend/syntax | class: blocked | Implement Selfnameandimportsemitinclusion |
| 4113 | Implement Selfref | spike | frontend/syntax | class: blocked | Implement Selfref |
| 4114 | Implement Selfreference | spike | frontend/resolver | class: blocked | Implement Selfreference |
| 4115 | Implement Selfreferencesinfunctionparameters | spike | frontend/syntax | class: blocked | Implement Selfreferencesinfunctionparameters |
| 4116 | Implement Selfreferentialdefaultnostackoverflow | spike | frontend/syntax | class: blocked | Implement Selfreferentialdefaultnostackoverflow |
| 4117 | Implement Semicolonsinmoduledeclarations | spike | frontend/syntax | class: blocked | Implement Semicolonsinmoduledeclarations |
| 4118 | Implement Separate Import Export | spike | frontend/syntax | class: blocked | Implement Separate Import Export |
| 4119 | Implement Separate Name Resolution | spike | frontend/resolver | class: blocked | Implement Separate Name Resolution |
| 4120 | Implement Setmethods | spike | frontend/syntax | class: triage-needed | Implement Setmethods |
| 4121 | Implement Setterbeforegetter | spike | frontend/syntax | class: blocked | Implement Setterbeforegetter |
| 4122 | Implement Setterwithreturn | spike | frontend/syntax | class: blocked | Implement Setterwithreturn |
| 4123 | Implement Shadowprivatemembers | spike | frontend/syntax | class: blocked | Implement Shadowprivatemembers |
| 4124 | Implement Shadowedfunctionscopedvariablesbyblockscopedones | spike | frontend/resolver | class: blocked | Implement Shadowedfunctionscopedvariablesbyblockscopedones |
| 4125 | Implement Shadowedreservedcompilerdeclarationswithnoemit | spike | frontend/syntax | class: blocked | Implement Shadowedreservedcompilerdeclarationswithnoemit |
| 4126 | Implement Shadowingvialocalvalue | spike | frontend/syntax | class: blocked | Implement Shadowingvialocalvalue |
| 4127 | Implement Shadowingvialocalvalueorbindingelement | spike | reference/triage | class: triage-needed | Implement Shadowingvialocalvalueorbindingelement |
| 4128 | Implement Shebang | spike | frontend/syntax | class: triage-needed | Implement Shebang |
| 4129 | Implement Shebangbeforereferences | spike | frontend/syntax | class: triage-needed | Implement Shebangbeforereferences |
| 4130 | Implement Shebangerror | spike | frontend/syntax | class: triage-needed | Implement Shebangerror |
| 4131 | Implement Shorthand Module System Amd | spike | frontend/syntax | class: blocked | Implement Shorthand Module System Amd |
| 4132 | Implement Shorthand Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Shorthand Parser Syntax |
| 4133 | Implement Shorthandofexportedentity | spike | frontend/syntax | class: blocked | Implement Shorthandofexportedentity |
| 4134 | Implement Shorthandpropertyassignmentines | spike | frontend/syntax | class: blocked | Implement Shorthandpropertyassignmentines |
| 4135 | Implement Shorthandpropertyassignmentsindestructuring | spike | frontend/syntax | class: blocked | Implement Shorthandpropertyassignmentsindestructuring |
| 4136 | Implement Shorthandpropertyundefined | spike | frontend/syntax | class: triage-needed | Implement Shorthandpropertyundefined |
| 4137 | Implement Shouldnotprintnullescapesintooctalliterals | spike | frontend/syntax | class: triage-needed | Implement Shouldnotprintnullescapesintooctalliterals |
| 4138 | Implement Sideeffectimports | spike | frontend/syntax | class: blocked | Implement Sideeffectimports |
| 4139 | Implement Sigantureissubtypeiftheyareidentical | spike | frontend/syntax | class: triage-needed | Implement Sigantureissubtypeiftheyareidentical |
| 4140 | Implement Signaturecombiningrestparameters Arrow Function | spike | frontend/syntax | class: blocked | Implement Signaturecombiningrestparameters Arrow Function |
| 4141 | Implement Signaturecombiningrestparameters Parser Syntax | spike | frontend/syntax | class: blocked | Implement Signaturecombiningrestparameters Parser Syntax |
| 4142 | Implement Signaturecombiningrestparameters Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Signaturecombiningrestparameters Unknown Unsupported |
| 4143 | Implement Signaturelengthmismatchwithoptionalparameters | spike | frontend/resolver | class: blocked | Implement Signaturelengthmismatchwithoptionalparameters |
| 4144 | Implement Signatureoverloadswithcomments | spike | frontend/syntax | class: triage-needed | Implement Signatureoverloadswithcomments |
| 4145 | Implement Signaturesusejsdocforoptionalparameters | spike | frontend/syntax | class: blocked | Implement Signaturesusejsdocforoptionalparameters |
| 4146 | Implement Silentneverpropagation | spike | frontend/syntax | class: blocked | Implement Silentneverpropagation |
| 4147 | Implement Simplerecursionwithbasecase Name Resolution | spike | frontend/resolver | class: blocked | Implement Simplerecursionwithbasecase Name Resolution |
| 4148 | Implement Simplerecursionwithbasecase Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Simplerecursionwithbasecase Parser Syntax |
| 4149 | Implement Simplerecursionwithbasecase Runtime Subset | spike | reference/triage | class: triage-needed | Implement Simplerecursionwithbasecase Runtime Subset |
| 4150 | Implement Simplifyingconditionalwithinteriorconditionalisrelated | spike | frontend/semantics | class: blocked | Implement Simplifyingconditionalwithinteriorconditionalisrelated |
| 4151 | Implement Slashbeforevariabledeclaration | spike | frontend/syntax | class: triage-needed | Implement Slashbeforevariabledeclaration |
| 4152 | Implement Sliceresultcast | spike | frontend/resolver | class: blocked | Implement Sliceresultcast |
| 4153 | Implement Slightlyindirecteddeepobjectliteralelaborations | spike | frontend/syntax | class: blocked | Implement Slightlyindirecteddeepobjectliteralelaborations |
| 4154 | Implement Sourcemap Import Export | spike | frontend/syntax | class: blocked | Implement Sourcemap Import Export |
| 4155 | Implement Sourcemap Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Sourcemap Unknown Unsupported |
| 4156 | Implement Sourcemapforfunctionininternalmodulewithcommentprecedingstatement | spike | frontend/syntax | class: blocked | Implement Sourcemapforfunctionininternalmodulewithcommentprecedingstatement |
| 4157 | Implement Sourcemapsample | spike | frontend/syntax | class: blocked | Implement Sourcemapsample |
| 4158 | Implement Sourcemapvalidationclass | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationclass |
| 4159 | Implement Sourcemapvalidationclasswithdefaultconstructor | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationclasswithdefaultconstructor |
| 4160 | Implement Sourcemapvalidationclasswithdefaultconstructorandcapturedthisstatement | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationclasswithdefaultconstructorandcapturedthisstatement |
| 4161 | Implement Sourcemapvalidationclasswithdefaultconstructorandextendsclause | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationclasswithdefaultconstructorandextendsclause |
| 4162 | Implement Sourcemapvalidationclasses | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationclasses |
| 4163 | Implement Sourcemapvalidationdebugger | spike | frontend/resolver | class: blocked | Implement Sourcemapvalidationdebugger |
| 4164 | Implement Sourcemapvalidationdecorators | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdecorators |
| 4165 | Implement Sourcemapvalidationdestructuringforarraybindingpattern | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringforarraybindingpattern |
| 4166 | Implement Sourcemapvalidationdestructuringforarraybindingpatterndefaultvalues | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringforarraybindingpatterndefaultvalues |
| 4167 | Implement Sourcemapvalidationdestructuringforobjectbindingpattern | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringforobjectbindingpattern |
| 4168 | Implement Sourcemapvalidationdestructuringforobjectbindingpatterndefaultvalues | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringforobjectbindingpatterndefaultvalues |
| 4169 | Implement Sourcemapvalidationdestructuringforofarraybindingpattern | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringforofarraybindingpattern |
| 4170 | Implement Sourcemapvalidationdestructuringforofarraybindingpatterndefaultvalues | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringforofarraybindingpatterndefaultvalues |
| 4171 | Implement Sourcemapvalidationdestructuringforofobjectbindingpattern | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringforofobjectbindingpattern |
| 4172 | Implement Sourcemapvalidationdestructuringforofobjectbindingpatterndefaultvalues | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringforofobjectbindingpatterndefaultvalues |
| 4173 | Implement Sourcemapvalidationdestructuringparameternestedobjectbindingpattern | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringparameternestedobjectbindingpattern |
| 4174 | Implement Sourcemapvalidationdestructuringparameternestedobjectbindingpatterndefaultvalues | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringparameternestedobjectbindingpatterndefaultvalues |
| 4175 | Implement Sourcemapvalidationdestructuringparameterobjectbindingpattern | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringparameterobjectbindingpattern |
| 4176 | Implement Sourcemapvalidationdestructuringparameterobjectbindingpatterndefaultvalues | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringparameterobjectbindingpatterndefaultvalues |
| 4177 | Implement Sourcemapvalidationdestructuringparametertarraybindingpattern | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringparametertarraybindingpattern |
| 4178 | Implement Sourcemapvalidationdestructuringparametertarraybindingpatterndefaultvalues | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringparametertarraybindingpatterndefaultvalues |
| 4179 | Implement Sourcemapvalidationdestructuringvariablestatement | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringvariablestatement |
| 4180 | Implement Sourcemapvalidationdestructuringvariablestatementarraybindingpattern | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringvariablestatementarraybindingpattern |
| 4181 | Implement Sourcemapvalidationdestructuringvariablestatementarraybindingpatterndefaultvalues | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringvariablestatementarraybindingpatterndefaultvalues |
| 4182 | Implement Sourcemapvalidationdestructuringvariablestatementdefaultvalues | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringvariablestatementdefaultvalues |
| 4183 | Implement Sourcemapvalidationdestructuringvariablestatementnestedobjectbindingpattern | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringvariablestatementnestedobjectbindingpattern |
| 4184 | Implement Sourcemapvalidationdestructuringvariablestatementnestedobjectbindingpatternwithdefaultvalues | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationdestructuringvariablestatementnestedobjectbindingpatternwithdefaultv... |
| 4185 | Implement Sourcemapvalidationenums | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationenums |
| 4186 | Implement Sourcemapvalidationexportassignment | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationexportassignment |
| 4187 | Implement Sourcemapvalidationexportassignmentcommonjs | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationexportassignmentcommonjs |
| 4188 | Implement Sourcemapvalidationfor | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationfor |
| 4189 | Implement Sourcemapvalidationforin | spike | frontend/resolver | class: blocked | Implement Sourcemapvalidationforin |
| 4190 | Implement Sourcemapvalidationfunctionexpressions | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationfunctionexpressions |
| 4191 | Implement Sourcemapvalidationfunctions | spike | frontend/resolver | class: blocked | Implement Sourcemapvalidationfunctions |
| 4192 | Implement Sourcemapvalidationimport | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationimport |
| 4193 | Implement Sourcemapvalidationmodule | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationmodule |
| 4194 | Implement Sourcemapvalidationstatements | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationstatements |
| 4195 | Implement Sourcemapvalidationswitch | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationswitch |
| 4196 | Implement Sourcemapvalidationtrycatchfinally | spike | reference/triage | class: triage-needed | Implement Sourcemapvalidationtrycatchfinally |
| 4197 | Implement Sourcemapvalidationwithcomments | spike | frontend/syntax | class: triage-needed | Implement Sourcemapvalidationwithcomments |
| 4198 | Implement Sourcemapwithmultiplefileswithfileendingwithinterface | spike | frontend/syntax | class: blocked | Implement Sourcemapwithmultiplefileswithfileendingwithinterface |
| 4199 | Implement Sourcemapvalidationduplicatenames | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationduplicatenames |
| 4200 | Implement Spacebeforequestionmarkinpropertyassignment | spike | frontend/syntax | class: triage-needed | Implement Spacebeforequestionmarkinpropertyassignment |
| 4201 | Implement Specednostackblown | spike | frontend/syntax | class: blocked | Implement Specednostackblown |
| 4202 | Implement Specializationofexportedclass | spike | frontend/syntax | class: blocked | Implement Specializationofexportedclass |
| 4203 | Implement Specializationsshouldnotaffecteachother | spike | frontend/syntax | class: blocked | Implement Specializationsshouldnotaffecteachother |
| 4204 | Implement Specializedinheritedconstructors | spike | frontend/syntax | class: triage-needed | Implement Specializedinheritedconstructors |
| 4205 | Implement Specializedlambdatypearguments | spike | frontend/syntax | class: blocked | Implement Specializedlambdatypearguments |
| 4206 | Implement Specializedsignatureascallbackparameter | spike | frontend/syntax | class: triage-needed | Implement Specializedsignatureascallbackparameter |
| 4207 | Implement Spellingsuggestionglobal | spike | frontend/syntax | class: triage-needed | Implement Spellingsuggestionglobal |
| 4208 | Implement Spellingsuggestionleadingunderscores | spike | frontend/syntax | class: triage-needed | Implement Spellingsuggestionleadingunderscores |
| 4209 | Implement Spellingsuggestionmodule | spike | frontend/syntax | class: blocked | Implement Spellingsuggestionmodule |
| 4211 | Implement Spreadbooleanrespectsfreshness | spike | frontend/resolver | class: blocked | Implement Spreadbooleanrespectsfreshness |
| 4212 | Implement Spreadexpressioncontainingobjectexpressioncontextualtype | spike | frontend/syntax | class: blocked | Implement Spreadexpressioncontainingobjectexpressioncontextualtype |
| 4213 | Implement Spreadexpressioncontextualtypewithnamespace | spike | frontend/syntax | class: blocked | Implement Spreadexpressioncontextualtypewithnamespace |
| 4214 | Implement Spreadintersection | spike | reference/triage | class: triage-needed | Implement Spreadintersection |
| 4215 | Implement Spreadinvalidargumenttype | spike | frontend/syntax | class: blocked | Implement Spreadinvalidargumenttype |
| 4216 | Implement Spreadobjectnocircular | spike | frontend/syntax | class: blocked | Implement Spreadobjectnocircular |
| 4217 | Implement Spreadobjectpermutations | spike | frontend/resolver | class: blocked | Implement Spreadobjectpermutations |
| 4218 | Implement Spreadobjectwithindexdoesnotaddundefinedtolocalindex | spike | frontend/syntax | class: blocked | Implement Spreadobjectwithindexdoesnotaddundefinedtolocalindex |
| 4219 | Implement Spreadofobjectliteralassignabletoindexsignature | spike | frontend/syntax | class: blocked | Implement Spreadofobjectliteralassignabletoindexsignature |
| 4220 | Implement Spreadofparamsfromgeneratormakesrequiredparams | spike | frontend/syntax | class: blocked | Implement Spreadofparamsfromgeneratormakesrequiredparams |
| 4221 | Implement Spreadparametertupletype | spike | frontend/syntax | class: blocked | Implement Spreadparametertupletype |
| 4222 | Implement Spreadtupleaccessedbytypeparameter | spike | frontend/syntax | class: blocked | Implement Spreadtupleaccessedbytypeparameter |
| 4223 | Implement Spreadunionpropoverride | spike | frontend/syntax | class: blocked | Implement Spreadunionpropoverride |
| 4224 | Implement Spreadsandcontextualtupletypes | spike | frontend/syntax | class: blocked | Implement Spreadsandcontextualtupletypes |
| 4225 | Implement Spycomparisonchecking | spike | frontend/syntax | class: triage-needed | Implement Spycomparisonchecking |
| 4226 | Implement Stabletypeordering | spike | frontend/syntax | class: triage-needed | Implement Stabletypeordering |
| 4227 | Implement Stackdepthlimitcastingtype | spike | frontend/syntax | class: triage-needed | Implement Stackdepthlimitcastingtype |
| 4229 | Implement Staticandmemberfunctions | spike | frontend/syntax | class: triage-needed | Implement Staticandmemberfunctions |
| 4230 | Implement Staticanonymoustypenotreferencingtypeparameter | spike | frontend/syntax | class: blocked | Implement Staticanonymoustypenotreferencingtypeparameter |
| 4231 | Implement Staticasidentifier | spike | frontend/syntax | class: triage-needed | Implement Staticasidentifier |
| 4232 | Implement Staticclassmembererror | spike | runtime/builtins | class: triage-needed | Implement Staticclassmembererror |
| 4233 | Implement Staticclassprops | spike | frontend/syntax | class: triage-needed | Implement Staticclassprops |
| 4234 | Implement Staticfieldwithinterfacecontext | spike | frontend/syntax | class: triage-needed | Implement Staticfieldwithinterfacecontext |
| 4235 | Implement Staticgetter | spike | frontend/syntax | class: blocked | Implement Staticgetter |
| 4236 | Implement Staticgetterandsetter | spike | reference/triage | class: triage-needed | Implement Staticgetterandsetter |
| 4237 | Implement Staticindexsignatureandnormalindexsignature | spike | frontend/syntax | class: triage-needed | Implement Staticindexsignatureandnormalindexsignature |
| 4238 | Implement Staticinheritance | spike | frontend/syntax | class: triage-needed | Implement Staticinheritance |
| 4239 | Implement Staticinitializersandlegacyclassdecorators | spike | frontend/syntax | class: blocked | Implement Staticinitializersandlegacyclassdecorators |
| 4240 | Implement Staticinstanceresolution Import Export | spike | frontend/syntax | class: blocked | Implement Staticinstanceresolution Import Export |
| 4241 | Implement Staticinstanceresolution Module Resolution | spike | frontend/syntax | class: blocked | Implement Staticinstanceresolution Module Resolution |
| 4242 | Implement Staticinstanceresolution Name Resolution | spike | frontend/resolver | class: blocked | Implement Staticinstanceresolution Name Resolution |
| 4243 | Implement Staticinterfaceassignmentcompat | spike | frontend/resolver | class: blocked | Implement Staticinterfaceassignmentcompat |
| 4244 | Implement Staticmemberaccessoffderivedtype | spike | frontend/syntax | class: triage-needed | Implement Staticmemberaccessoffderivedtype |
| 4245 | Implement Staticmemberexportaccess | spike | frontend/syntax | class: blocked | Implement Staticmemberexportaccess |
| 4246 | Implement Staticmemberofclassandpublicmemberofanotherclassassignment | spike | frontend/syntax | class: blocked | Implement Staticmemberofclassandpublicmemberofanotherclassassignment |
| 4247 | Implement Staticmemberwithstringandnumbernames | spike | frontend/syntax | class: triage-needed | Implement Staticmemberwithstringandnumbernames |
| 4248 | Implement Staticmethodreferencingtypeargument | spike | frontend/syntax | class: blocked | Implement Staticmethodreferencingtypeargument |
| 4249 | Implement Staticmethodwithtypeparameterextendsclausedeclfile | spike | frontend/syntax | class: blocked | Implement Staticmethodwithtypeparameterextendsclausedeclfile |
| 4250 | Implement Staticmethodsreferencingclasstypeparameters | spike | frontend/syntax | class: blocked | Implement Staticmethodsreferencingclasstypeparameters |
| 4251 | Implement Staticmismatchbecauseofprototype | spike | frontend/syntax | class: triage-needed | Implement Staticmismatchbecauseofprototype |
| 4253 | Implement Staticmustprecedepublic | spike | frontend/syntax | class: blocked | Implement Staticmustprecedepublic |
| 4254 | Implement Staticoffofinstance | spike | frontend/syntax | class: triage-needed | Implement Staticoffofinstance |
| 4255 | Implement Staticpropsuper | spike | frontend/syntax | class: triage-needed | Implement Staticpropsuper |
| 4256 | Implement Staticprototypeproperty | spike | frontend/syntax | class: triage-needed | Implement Staticprototypeproperty |
| 4257 | Implement Staticprototypepropertyonclass | spike | frontend/syntax | class: triage-needed | Implement Staticprototypepropertyonclass |
| 4258 | Implement Staticvisibility Duplicate Function | spike | reference/triage | class: triage-needed | Implement Staticvisibility Duplicate Function |
| 4259 | Implement Staticvisibility Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Staticvisibility Parser Syntax |
| 4260 | Implement Statics | spike | frontend/syntax | class: blocked | Implement Statics |
| 4261 | Implement Staticsinafunction | spike | frontend/syntax | class: triage-needed | Implement Staticsinafunction |
| 4263 | Implement Staticsnotinscopeinclodule | spike | frontend/resolver | class: blocked | Implement Staticsnotinscopeinclodule |
| 4264 | Implement Stradac | spike | frontend/resolver | class: blocked | Implement Stradac |
| 4265 | Implement Strictfunctiontypes | spike | frontend/resolver | class: blocked | Implement Strictfunctiontypes |
| 4266 | Implement Strictfunctiontypeserrors | spike | runtime/builtins | class: triage-needed | Implement Strictfunctiontypeserrors |
| 4267 | Implement Strictmodeenummembernamereserved | spike | frontend/syntax | class: triage-needed | Implement Strictmodeenummembernamereserved |
| 4268 | Implement Strictmodeinconstructor | spike | frontend/syntax | class: triage-needed | Implement Strictmodeinconstructor |
| 4269 | Implement Strictmodereservedword | spike | frontend/syntax | class: triage-needed | Implement Strictmodereservedword |
| 4270 | Implement Strictmodereservedwordinclassdeclaration | spike | frontend/syntax | class: triage-needed | Implement Strictmodereservedwordinclassdeclaration |
| 4271 | Implement Strictmodereservedwordindestructuring | spike | frontend/syntax | class: blocked | Implement Strictmodereservedwordindestructuring |
| 4272 | Implement Strictmodereservedwordinimportequaldeclaration | spike | frontend/syntax | class: blocked | Implement Strictmodereservedwordinimportequaldeclaration |
| 4273 | Implement Strictmodereservedwordinmoduledeclaration | spike | frontend/syntax | class: blocked | Implement Strictmodereservedwordinmoduledeclaration |
| 4274 | Implement Strictmodeusecontextualkeyword | spike | frontend/syntax | class: triage-needed | Implement Strictmodeusecontextualkeyword |
| 4275 | Implement Strictmodewordinexportdeclaration | spike | frontend/syntax | class: blocked | Implement Strictmodewordinexportdeclaration |
| 4276 | Implement Strictmodewordinimportdeclaration | spike | frontend/syntax | class: blocked | Implement Strictmodewordinimportdeclaration |
| 4277 | Implement Strictnullemptydestructuring | spike | reference/triage | class: triage-needed | Implement Strictnullemptydestructuring |
| 4278 | Implement Strictnulllogicalandor | spike | frontend/syntax | class: triage-needed | Implement Strictnulllogicalandor |
| 4279 | Implement Strictnullnotnullindextypenolib | spike | frontend/syntax | class: triage-needed | Implement Strictnullnotnullindextypenolib |
| 4280 | Implement Strictnullnotnullindextypeshouldwork | spike | frontend/syntax | class: triage-needed | Implement Strictnullnotnullindextypeshouldwork |
| 4281 | Implement Strictoptionalproperties | spike | frontend/syntax | class: triage-needed | Implement Strictoptionalproperties |
| 4282 | Implement Strictsubtypeandnarrowing | spike | frontend/syntax | class: blocked | Implement Strictsubtypeandnarrowing |
| 4283 | Implement Stricttypeofunionnarrowing | spike | frontend/syntax | class: triage-needed | Implement Stricttypeofunionnarrowing |
| 4285 | Implement Stringindexerandconstructor | spike | frontend/syntax | class: triage-needed | Implement Stringindexerandconstructor |
| 4286 | Implement Stringindexerassignments Name Resolution | spike | frontend/resolver | class: blocked | Implement Stringindexerassignments Name Resolution |
| 4287 | Implement Stringindexerassignments Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Stringindexerassignments Parser Syntax |
| 4288 | Implement Stringliteralobjectliteraldeclaration | spike | frontend/syntax | class: blocked | Implement Stringliteralobjectliteraldeclaration |
| 4289 | Implement Stringliteralpropertynamewithlinecontinuation | spike | frontend/syntax | class: triage-needed | Implement Stringliteralpropertynamewithlinecontinuation |
| 4290 | Implement Stringliteralserrors | spike | frontend/syntax | class: triage-needed | Implement Stringliteralserrors |
| 4292 | Implement Stringpropcodegen | spike | frontend/syntax | class: blocked | Implement Stringpropcodegen |
| 4293 | Implement Stringrawtype | spike | frontend/resolver | class: blocked | Implement Stringrawtype |
| 4295 | Implement Stripmembersoptionality | spike | frontend/resolver | class: blocked | Implement Stripmembersoptionality |
| 4296 | Implement Structural | spike | frontend/syntax | class: blocked | Implement Structural |
| 4297 | Implement Structuraltypeindeclarefileformodule | spike | frontend/syntax | class: blocked | Implement Structuraltypeindeclarefileformodule |
| 4298 | Implement Styledcomponentsinstantiaionlimitnotreached | spike | frontend/syntax | class: triage-needed | Implement Styledcomponentsinstantiaionlimitnotreached |
| 4299 | Implement Subsubclasscanaccessprotectedconstructor | spike | frontend/syntax | class: blocked | Implement Subsubclasscanaccessprotectedconstructor |
| 4300 | Implement Subclassthistypeassignable | spike | frontend/syntax | class: blocked | Implement Subclassthistypeassignable |
| 4301 | Implement Subclasswithpolymorphicthisisassignable | spike | frontend/syntax | class: blocked | Implement Subclasswithpolymorphicthisisassignable |
| 4302 | Implement Substitutiontypenomergeofassignabletype | spike | frontend/syntax | class: triage-needed | Implement Substitutiontypenomergeofassignabletype |
| 4303 | Implement Substitutiontypesinindexedaccesstypes | spike | frontend/resolver | class: blocked | Implement Substitutiontypesinindexedaccesstypes |
| 4304 | Implement Subtypereductionunionconstraints | spike | frontend/syntax | class: blocked | Implement Subtypereductionunionconstraints |
| 4305 | Implement Subtypereductionwithanyfunctiontype | spike | frontend/syntax | class: blocked | Implement Subtypereductionwithanyfunctiontype |
| 4306 | Implement Subtyperelationfornever | spike | frontend/syntax | class: triage-needed | Implement Subtyperelationfornever |
| 4307 | Implement Subtypingtransitivity | spike | frontend/syntax | class: triage-needed | Implement Subtypingtransitivity |
| 4308 | Implement Super Import Export | spike | frontend/syntax | class: blocked | Implement Super Import Export |
| 4309 | Implement Super Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Super Parser Syntax |
| 4310 | Implement Superaccess | spike | frontend/syntax | class: triage-needed | Implement Superaccess |
| 4311 | Implement Superaccessinfatarrow | spike | frontend/syntax | class: blocked | Implement Superaccessinfatarrow |
| 4312 | Implement Supercallargsmustmatch | spike | frontend/syntax | class: blocked | Implement Supercallargsmustmatch |
| 4313 | Implement Supercallfromclassthatderivesfromgenerictype | spike | frontend/semantics | class: blocked | Implement Supercallfromclassthatderivesfromgenerictype |
| 4314 | Implement Supercallfromclassthatderivesfromgenerictypebutwithincorrectnumberoftypearguments | spike | frontend/semantics | class: blocked | Implement Supercallfromclassthatderivesfromgenerictypebutwithincorrectnumberoftypearguments |
| 4315 | Implement Supercallfromclassthatderivesfromgenerictypebutwithnotypearguments | spike | frontend/semantics | class: blocked | Implement Supercallfromclassthatderivesfromgenerictypebutwithnotypearguments |
| 4316 | Implement Supercallfromclassthatderivesnongenerictypebutwithtypearguments | spike | frontend/semantics | class: blocked | Implement Supercallfromclassthatderivesnongenerictypebutwithtypearguments |
| 4317 | Implement Supercallfromclassthathasnobasetype | spike | frontend/syntax | class: blocked | Implement Supercallfromclassthathasnobasetype |
| 4318 | Implement Supercallfromfunction | spike | frontend/resolver | class: blocked | Implement Supercallfromfunction |
| 4319 | Implement Supercallinnonstaticmethod | spike | frontend/syntax | class: blocked | Implement Supercallinnonstaticmethod |
| 4320 | Implement Supercallinstaticmethod | spike | frontend/syntax | class: triage-needed | Implement Supercallinstaticmethod |
| 4321 | Implement Supercallinsideclassexpression | spike | frontend/syntax | class: triage-needed | Implement Supercallinsideclassexpression |
| 4322 | Implement Supercallinsideobjectliteralexpression | spike | frontend/syntax | class: blocked | Implement Supercallinsideobjectliteralexpression |
| 4323 | Implement Supercalloutsideconstructor | spike | frontend/syntax | class: triage-needed | Implement Supercalloutsideconstructor |
| 4324 | Implement Supercallwithmissingbaseclass | spike | frontend/syntax | class: blocked | Implement Supercallwithmissingbaseclass |
| 4325 | Implement Supercallsinconstructor | spike | frontend/syntax | class: triage-needed | Implement Supercallsinconstructor |
| 4326 | Implement Superelementaccess | spike | frontend/syntax | class: triage-needed | Implement Superelementaccess |
| 4327 | Implement Supererrors | spike | frontend/syntax | class: triage-needed | Implement Supererrors |
| 4328 | Implement Superhasmethodsfrommergedinterface | spike | frontend/syntax | class: blocked | Implement Superhasmethodsfrommergedinterface |
| 4329 | Implement Superinconstructorparam | spike | frontend/syntax | class: blocked | Implement Superinconstructorparam |
| 4330 | Implement Superinlambdas | spike | frontend/syntax | class: triage-needed | Implement Superinlambdas |
| 4331 | Implement Superinobjectliterals | spike | frontend/syntax | class: blocked | Implement Superinobjectliterals |
| 4332 | Implement Supernewcall | spike | frontend/syntax | class: triage-needed | Implement Supernewcall |
| 4333 | Implement Supernomodifierscrash | spike | frontend/syntax | class: triage-needed | Implement Supernomodifierscrash |
| 4334 | Implement Superpropertyaccess | spike | frontend/syntax | class: triage-needed | Implement Superpropertyaccess |
| 4335 | Implement Superpropertyaccessincomputedpropertiesofnestedtype | spike | frontend/syntax | class: triage-needed | Implement Superpropertyaccessincomputedpropertiesofnestedtype |
| 4336 | Implement Superpropertyaccessinsupercall | spike | frontend/syntax | class: triage-needed | Implement Superpropertyaccessinsupercall |
| 4337 | Implement Superpropertyelementnounusedlexicalthiscapture | spike | frontend/syntax | class: blocked | Implement Superpropertyelementnounusedlexicalthiscapture |
| 4338 | Implement Superwithgenericspecialization | spike | frontend/semantics | class: blocked | Implement Superwithgenericspecialization |
| 4339 | Implement Superwithgenerics | spike | frontend/semantics | class: blocked | Implement Superwithgenerics |
| 4340 | Implement Superwithtypeargument | spike | frontend/syntax | class: blocked | Implement Superwithtypeargument |
| 4341 | Implement Switchassignmentcompat | spike | frontend/resolver | class: blocked | Implement Switchassignmentcompat |
| 4342 | Implement Switchcasenarrowsmatchingclausesevenwhennonmatchingclausesexist | spike | frontend/syntax | class: blocked | Implement Switchcasenarrowsmatchingclausesevenwhennonmatchingclausesexist |
| 4343 | Implement Switchcasesexpressiontypemismatch | spike | frontend/syntax | class: triage-needed | Implement Switchcasesexpressiontypemismatch |
| 4344 | Implement Switchcomparablecompatforbrands | spike | frontend/syntax | class: triage-needed | Implement Switchcomparablecompatforbrands |
| 4345 | Implement Switchfallthroughs | spike | frontend/syntax | class: triage-needed | Implement Switchfallthroughs |
| 4346 | Implement Switchstatementswithmultipledefaults | spike | frontend/syntax | class: triage-needed | Implement Switchstatementswithmultipledefaults |
| 4347 | Implement Symbollinkdeclarationemitmodulenames | spike | frontend/syntax | class: blocked | Implement Symbollinkdeclarationemitmodulenames |
| 4348 | Implement Symbollinkdeclarationemitmodulenamesimportref | spike | frontend/syntax | class: blocked | Implement Symbollinkdeclarationemitmodulenamesimportref |
| 4349 | Implement Symbollinkdeclarationemitmodulenamesrootdir | spike | frontend/syntax | class: blocked | Implement Symbollinkdeclarationemitmodulenamesrootdir |
| 4350 | Implement Symbolmergevalueandimportedtype | spike | frontend/syntax | class: blocked | Implement Symbolmergevalueandimportedtype |
| 4351 | Implement Symbolobservermismatchingpolyfillsworktogether | spike | frontend/syntax | class: triage-needed | Implement Symbolobservermismatchingpolyfillsworktogether |
| 4352 | Implement Symlinkedworkspacedependenciesnodirectlinkgeneratesdeepnonrelativename | spike | frontend/syntax | class: blocked | Implement Symlinkedworkspacedependenciesnodirectlinkgeneratesdeepnonrelativename |
| 4353 | Implement Symlinkedworkspacedependenciesnodirectlinkgeneratesnonrelativename | spike | frontend/syntax | class: blocked | Implement Symlinkedworkspacedependenciesnodirectlinkgeneratesnonrelativename |
| 4354 | Implement Symlinkedworkspacedependenciesnodirectlinkoptionalgeneratesnonrelativename | spike | frontend/syntax | class: blocked | Implement Symlinkedworkspacedependenciesnodirectlinkoptionalgeneratesnonrelativename |
| 4355 | Implement Symlinkedworkspacedependenciesnodirectlinkpeergeneratesnonrelativename | spike | frontend/syntax | class: blocked | Implement Symlinkedworkspacedependenciesnodirectlinkpeergeneratesnonrelativename |
| 4356 | Implement Syntheticdefaultexportswithdynamicimports | spike | frontend/syntax | class: blocked | Implement Syntheticdefaultexportswithdynamicimports |
| 4357 | Implement Systemdefaultexportcommentvalidity | spike | frontend/syntax | class: blocked | Implement Systemdefaultexportcommentvalidity |
| 4358 | Implement Systemdefaultimportcallable | spike | frontend/syntax | class: blocked | Implement Systemdefaultimportcallable |
| 4359 | Implement Systemexportassignment | spike | frontend/syntax | class: blocked | Implement Systemexportassignment |
| 4360 | Implement Systemjsforinnoexception | spike | frontend/syntax | class: triage-needed | Implement Systemjsforinnoexception |
| 4361 | Implement Systemmodule Import Export | spike | frontend/syntax | class: blocked | Implement Systemmodule Import Export |
| 4362 | Implement Systemmodule Module System Amd | spike | frontend/syntax | class: blocked | Implement Systemmodule Module System Amd |
| 4363 | Implement Systemmodule Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Systemmodule Parser Syntax |
| 4364 | Implement Systemmoduleambientdeclarations | spike | frontend/syntax | class: blocked | Implement Systemmoduleambientdeclarations |
| 4365 | Implement Systemmoduleconstenums | spike | frontend/syntax | class: blocked | Implement Systemmoduleconstenums |
| 4366 | Implement Systemmoduleconstenumsseparatecompilation | spike | frontend/syntax | class: blocked | Implement Systemmoduleconstenumsseparatecompilation |
| 4367 | Implement Systemmoduledeclarationmerging | spike | frontend/syntax | class: blocked | Implement Systemmoduledeclarationmerging |
| 4368 | Implement Systemmoduleexportdefault | spike | frontend/syntax | class: blocked | Implement Systemmoduleexportdefault |
| 4369 | Implement Systemmodulenontoplevelmodulemembers | spike | frontend/syntax | class: blocked | Implement Systemmodulenontoplevelmodulemembers |
| 4370 | Implement Systemmoduletargetes | spike | frontend/syntax | class: blocked | Implement Systemmoduletargetes |
| 4371 | Implement Systemmoduletrailingcomments | spike | frontend/syntax | class: blocked | Implement Systemmoduletrailingcomments |
| 4372 | Implement Systemmodulewithsuperclass | spike | frontend/syntax | class: blocked | Implement Systemmodulewithsuperclass |
| 4373 | Implement Systemnamespacealiasemit | spike | frontend/syntax | class: blocked | Implement Systemnamespacealiasemit |
| 4374 | Implement Systemobjectshorthandrename | spike | frontend/syntax | class: blocked | Implement Systemobjectshorthandrename |
| 4375 | Implement Taggedtemplatestringwithsymbolexpression | spike | frontend/syntax | class: triage-needed | Implement Taggedtemplatestringwithsymbolexpression |
| 4376 | Implement Taggedtemplatestringshexadecimalescapes | spike | frontend/syntax | class: blocked | Implement Taggedtemplatestringshexadecimalescapes |
| 4377 | Implement Taggedtemplatestringshexadecimalescapeses | spike | frontend/syntax | class: blocked | Implement Taggedtemplatestringshexadecimalescapeses |
| 4378 | Implement Taggedtemplatestringswithcurriedfunction | spike | frontend/syntax | class: triage-needed | Implement Taggedtemplatestringswithcurriedfunction |
| 4379 | Implement Taggedtemplatestringswithmultilinetemplate | spike | frontend/syntax | class: blocked | Implement Taggedtemplatestringswithmultilinetemplate |
| 4380 | Implement Taggedtemplatestringswithmultilinetemplatees | spike | frontend/syntax | class: blocked | Implement Taggedtemplatestringswithmultilinetemplatees |
| 4381 | Implement Taggedtemplatestringswithunicodeescapes | spike | frontend/syntax | class: blocked | Implement Taggedtemplatestringswithunicodeescapes |
| 4382 | Implement Taggedtemplatestringswithunicodeescapeses | spike | frontend/syntax | class: blocked | Implement Taggedtemplatestringswithunicodeescapeses |
| 4383 | Implement Taggedtemplatestringswithwhitespaceescapes | spike | frontend/syntax | class: blocked | Implement Taggedtemplatestringswithwhitespaceescapes |
| 4384 | Implement Taggedtemplatestringswithwhitespaceescapeses | spike | frontend/syntax | class: blocked | Implement Taggedtemplatestringswithwhitespaceescapeses |
| 4385 | Implement Taggedtemplatewithoutdeclaredhelper | spike | frontend/syntax | class: blocked | Implement Taggedtemplatewithoutdeclaredhelper |
| 4386 | Implement Taggedtemplatesindifferentscopes | spike | frontend/syntax | class: blocked | Implement Taggedtemplatesindifferentscopes |
| 4387 | Implement Taggedtemplatesinmoduleandglobal | spike | frontend/syntax | class: blocked | Implement Taggedtemplatesinmoduleandglobal |
| 4388 | Implement Taggedtemplateswithincompletenosubstitutiontemplate | spike | frontend/syntax | class: blocked | Implement Taggedtemplateswithincompletenosubstitutiontemplate |
| 4389 | Implement Taggedtemplateswithincompletetemplateexpressions | spike | frontend/syntax | class: blocked | Implement Taggedtemplateswithincompletetemplateexpressions |
| 4390 | Implement Targetes | spike | frontend/syntax | class: blocked | Implement Targetes |
| 4391 | Implement Targettypeargs | spike | frontend/syntax | class: blocked | Implement Targettypeargs |
| 4392 | Implement Targettypecalls | spike | frontend/syntax | class: triage-needed | Implement Targettypecalls |
| 4393 | Implement Targettypecasttest | spike | frontend/syntax | class: triage-needed | Implement Targettypecasttest |
| 4394 | Implement Targettypeobjectliteral | spike | frontend/syntax | class: blocked | Implement Targettypeobjectliteral |
| 4395 | Implement Targettypeobjectliteraltoany | spike | frontend/syntax | class: blocked | Implement Targettypeobjectliteraltoany |
| 4396 | Implement Targettypetest | spike | frontend/syntax | class: triage-needed | Implement Targettypetest |
| 4397 | Implement Targettypevoidfunc | spike | frontend/syntax | class: triage-needed | Implement Targettypevoidfunc |
| 4398 | Implement Templateexpressionaspossiblydiscriminantvalue | spike | frontend/syntax | class: triage-needed | Implement Templateexpressionaspossiblydiscriminantvalue |
| 4399 | Implement Templateexpressionnoinlininingofconstantbindingwithinitializer | spike | frontend/syntax | class: triage-needed | Implement Templateexpressionnoinlininingofconstantbindingwithinitializer |
| 4400 | Implement Templateliteralconstantevaluation | spike | frontend/syntax | class: blocked | Implement Templateliteralconstantevaluation |
| 4401 | Implement Templateliteralescapesequence | spike | frontend/syntax | class: blocked | Implement Templateliteralescapesequence |
| 4402 | Implement Templateliteralintersection Name Resolution | spike | frontend/resolver | class: blocked | Implement Templateliteralintersection Name Resolution |
| 4403 | Implement Templateliteralintersection Parser Syntax | spike | frontend/syntax | class: blocked | Implement Templateliteralintersection Parser Syntax |
| 4404 | Implement Templateliteralsanddecoratormetadata | spike | frontend/syntax | class: blocked | Implement Templateliteralsanddecoratormetadata |
| 4405 | Implement Templateliteralsintypes | spike | reference/triage | class: triage-needed | Implement Templateliteralsintypes |
| 4406 | Implement Templatestringsarraytypedefinedines | spike | frontend/syntax | class: blocked | Implement Templatestringsarraytypedefinedines |
| 4407 | Implement Templatestringsarraytypenotdefinedes | spike | frontend/syntax | class: blocked | Implement Templatestringsarraytypenotdefinedes |
| 4408 | Implement Templatestringsarraytyperedefinedines | spike | frontend/syntax | class: blocked | Implement Templatestringsarraytyperedefinedines |
| 4409 | Implement Temporal | spike | frontend/syntax | class: triage-needed | Implement Temporal |
| 4410 | Implement Ternaryexpressionsourcemap | spike | frontend/syntax | class: triage-needed | Implement Ternaryexpressionsourcemap |
| 4411 | Implement Testcontainerlist | spike | frontend/syntax | class: blocked | Implement Testcontainerlist |
| 4412 | Implement This Import Export | spike | frontend/syntax | class: blocked | Implement This Import Export |
| 4413 | Implement This Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement This Parser Syntax |
| 4414 | Implement Thisassignmentinnamespacedeclaration | spike | frontend/syntax | class: blocked | Implement Thisassignmentinnamespacedeclaration |
| 4415 | Implement Thisbinding Import Export | spike | frontend/syntax | class: blocked | Implement Thisbinding Import Export |
| 4416 | Implement Thisbinding Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Thisbinding Parser Syntax |
| 4417 | Implement Thiscapture | spike | frontend/syntax | class: triage-needed | Implement Thiscapture |
| 4418 | Implement Thisconditionalonmethodreturnofgenericinstance | spike | frontend/semantics | class: blocked | Implement Thisconditionalonmethodreturnofgenericinstance |
| 4419 | Implement Thisexpressionincallexpressionwithtypearguments | spike | frontend/syntax | class: blocked | Implement Thisexpressionincallexpressionwithtypearguments |
| 4420 | Implement Thisexpressionofgenericobject | spike | frontend/semantics | class: blocked | Implement Thisexpressionofgenericobject |
| 4421 | Implement Thisinaccessors | spike | reference/triage | class: triage-needed | Implement Thisinaccessors |
| 4422 | Implement Thisinarrowfunctioninstaticinitializer | spike | frontend/syntax | class: blocked | Implement Thisinarrowfunctioninstaticinitializer |
| 4423 | Implement Thisinclassbodystaticesnext | spike | frontend/syntax | class: triage-needed | Implement Thisinclassbodystaticesnext |
| 4424 | Implement Thisinconstructorparameter | spike | frontend/syntax | class: blocked | Implement Thisinconstructorparameter |
| 4425 | Implement Thisinfunctioncalljs | spike | frontend/syntax | class: blocked | Implement Thisinfunctioncalljs |
| 4426 | Implement Thisingenericstaticmembers | spike | frontend/semantics | class: blocked | Implement Thisingenericstaticmembers |
| 4427 | Implement Thisininnerfunctions | spike | frontend/syntax | class: triage-needed | Implement Thisininnerfunctions |
| 4428 | Implement Thisinlambda | spike | frontend/syntax | class: triage-needed | Implement Thisinlambda |
| 4429 | Implement Thisinmodule | spike | frontend/syntax | class: blocked | Implement Thisinmodule |
| 4430 | Implement Thisinmodulefunction | spike | frontend/syntax | class: blocked | Implement Thisinmodulefunction |
| 4431 | Implement Thisinobjectjs | spike | frontend/syntax | class: blocked | Implement Thisinobjectjs |
| 4432 | Implement Thisinouterclassbody | spike | frontend/syntax | class: triage-needed | Implement Thisinouterclassbody |
| 4433 | Implement Thisinpropertybounddeclarations | spike | frontend/syntax | class: triage-needed | Implement Thisinpropertybounddeclarations |
| 4434 | Implement Thisinstaticmethod | spike | frontend/syntax | class: triage-needed | Implement Thisinstaticmethod |
| 4435 | Implement Thisinstatics | spike | frontend/syntax | class: blocked | Implement Thisinstatics |
| 4436 | Implement Thisinsupercall | spike | frontend/syntax | class: triage-needed | Implement Thisinsupercall |
| 4437 | Implement Thisintupletypeparameterconstraints | spike | frontend/resolver | class: blocked | Implement Thisintupletypeparameterconstraints |
| 4438 | Implement Thisintypequery | spike | frontend/syntax | class: blocked | Implement Thisintypequery |
| 4439 | Implement Thisindexonexistingreadonlyfieldisnotnever | spike | frontend/syntax | class: blocked | Implement Thisindexonexistingreadonlyfieldisnotnever |
| 4440 | Implement Thiskeyword | spike | frontend/syntax | class: blocked | Implement Thiskeyword |
| 4441 | Implement Thispredicateinobjectliteral | spike | frontend/syntax | class: blocked | Implement Thispredicateinobjectliteral |
| 4442 | Implement Thisreferencedinfunctioninsidearrowfunction | spike | frontend/resolver | class: blocked | Implement Thisreferencedinfunctioninsidearrowfunction |
| 4443 | Implement Thisshadowingerrorspans | spike | reference/triage | class: triage-needed | Implement Thisshadowingerrorspans |
| 4444 | Implement Thistypeasconstraint | spike | frontend/syntax | class: blocked | Implement Thistypeasconstraint |
| 4445 | Implement Thiswhentypecheckfails | spike | frontend/syntax | class: blocked | Implement Thiswhentypecheckfails |
| 4446 | Implement Thislessfunctionsnotcontextsensitive Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Thislessfunctionsnotcontextsensitive Parser Syntax |
| 4447 | Implement Thislessfunctionsnotcontextsensitive Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Thislessfunctionsnotcontextsensitive Unknown Unsupported |
| 4448 | Implement Throwwithoutnewline Name Resolution | spike | frontend/resolver | class: blocked | Implement Throwwithoutnewline Name Resolution |
| 4449 | Implement Throwwithoutnewline Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Throwwithoutnewline Unknown Unsupported |
| 4450 | Implement Tostringonprimitives | spike | frontend/syntax | class: triage-needed | Implement Tostringonprimitives |
| 4451 | Implement Toofewargumentsingenericfunctiontypedargument | spike | frontend/semantics | class: blocked | Implement Toofewargumentsingenericfunctiontypedargument |
| 4452 | Implement Toomanytypeparameters | spike | frontend/syntax | class: triage-needed | Implement Toomanytypeparameters |
| 4453 | Implement Topfunctiontypenotcallable | spike | frontend/resolver | class: blocked | Implement Topfunctiontypenotcallable |
| 4454 | Implement Toplevel | spike | frontend/syntax | class: triage-needed | Implement Toplevel |
| 4455 | Implement Toplevelblockexpando | spike | frontend/syntax | class: triage-needed | Implement Toplevelblockexpando |
| 4456 | Implement Toplevelexports | spike | frontend/syntax | class: blocked | Implement Toplevelexports |
| 4457 | Implement Toplevellambda Arrow Function | spike | frontend/syntax | class: blocked | Implement Toplevellambda Arrow Function |
| 4458 | Implement Toplevellambda Class | spike | frontend/syntax | class: blocked | Implement Toplevellambda Class |
| 4459 | Implement Toplevellambda Import Export | spike | frontend/syntax | class: blocked | Implement Toplevellambda Import Export |
| 4460 | Implement Trackedsymbolsnocrash | spike | frontend/syntax | class: blocked | Implement Trackedsymbolsnocrash |
| 4461 | Implement Transformnestedgeneratorswithtry | spike | reference/triage | class: triage-needed | Implement Transformnestedgeneratorswithtry |
| 4462 | Implement Transformparenthesizesconditionalsubexpression | spike | frontend/semantics | class: blocked | Implement Transformparenthesizesconditionalsubexpression |
| 4463 | Implement Transformselidenullundefinedtype | spike | frontend/syntax | class: triage-needed | Implement Transformselidenullundefinedtype |
| 4464 | Implement Transitivetypeargumentinference | spike | frontend/semantics | class: blocked | Implement Transitivetypeargumentinference |
| 4465 | Implement Tripleslashincommentnotparsed | spike | frontend/syntax | class: blocked | Implement Tripleslashincommentnotparsed |
| 4466 | Implement Tripleslashtypesreferencewithmissingexports | spike | frontend/syntax | class: triage-needed | Implement Tripleslashtypesreferencewithmissingexports |
| 4467 | Implement Trivialsubtypereductionnostructuralcheck | spike | frontend/syntax | class: blocked | Implement Trivialsubtypereductionnostructuralcheck |
| 4468 | Implement Truthinesscallexpressioncoercion Name Resolution | spike | frontend/resolver | class: blocked | Implement Truthinesscallexpressioncoercion Name Resolution |
| 4469 | Implement Truthinesscallexpressioncoercion Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Truthinesscallexpressioncoercion Parser Syntax |
| 4470 | Implement Truthinesspromisecoercion | spike | runtime/builtins | class: triage-needed | Implement Truthinesspromisecoercion |
| 4471 | Implement Trycatchfinally | spike | reference/triage | class: triage-needed | Implement Trycatchfinally |
| 4472 | Implement Trycatchfinallycontrolflow | spike | frontend/syntax | class: blocked | Implement Trycatchfinallycontrolflow |
| 4473 | Implement Tsconfigmapoptionsarecaseinsensitive | spike | frontend/syntax | class: blocked | Implement Tsconfigmapoptionsarecaseinsensitive |
| 4474 | Implement Tslibmissinghelper | spike | reference/triage | class: triage-needed | Implement Tslibmissinghelper |
| 4475 | Implement Tslibmultiplemissinghelper | spike | reference/triage | class: triage-needed | Implement Tslibmultiplemissinghelper |
| 4476 | Implement Tslibnotfounddifferentmodules | spike | reference/triage | class: triage-needed | Implement Tslibnotfounddifferentmodules |
| 4477 | Implement Tslibreexporthelpers | spike | frontend/syntax | class: blocked | Implement Tslibreexporthelpers |
| 4478 | Implement Tsxdefaultimports | spike | frontend/syntax | class: triage-needed | Implement Tsxdefaultimports |
| 4479 | Implement Tsxfragmentchildrencheck | spike | reference/triage | class: triage-needed | Implement Tsxfragmentchildrencheck |
| 4480 | Implement Tsxresolveexternalmoduleexportstypes | spike | reference/triage | class: triage-needed | Implement Tsxresolveexternalmoduleexportstypes |
| 4481 | Implement Tsxtypeargumentpartialdefinitionstillerrors | spike | frontend/syntax | class: blocked | Implement Tsxtypeargumentpartialdefinitionstillerrors |
| 4482 | Implement Tupletypeinference Name Resolution | spike | frontend/resolver | class: blocked | Implement Tupletypeinference Name Resolution |
| 4483 | Implement Tupletypeinference Type System | spike | frontend/semantics | class: blocked | Implement Tupletypeinference Type System |
| 4484 | Implement Tupletypes | spike | reference/triage | class: triage-needed | Implement Tupletypes |
| 4485 | Implement Twicenestedkeyofindexinference | spike | frontend/resolver | class: blocked | Implement Twicenestedkeyofindexinference |
| 4486 | Implement Typealiasdeclarationemit | spike | frontend/syntax | class: blocked | Implement Typealiasdeclarationemit |
| 4487 | Implement Typealiasdoesntmakemoduleinstantiated | spike | frontend/syntax | class: blocked | Implement Typealiasdoesntmakemoduleinstantiated |
| 4488 | Implement Typealiasexport | spike | frontend/syntax | class: blocked | Implement Typealiasexport |
| 4489 | Implement Typealiasfunctiontypesharedsymbol | spike | frontend/syntax | class: blocked | Implement Typealiasfunctiontypesharedsymbol |
| 4490 | Implement Typearginference Name Resolution | spike | frontend/resolver | class: blocked | Implement Typearginference Name Resolution |
| 4491 | Implement Typearginference Type System | spike | frontend/semantics | class: blocked | Implement Typearginference Type System |
| 4492 | Implement Typeargumentconstraintresolution | spike | frontend/syntax | class: blocked | Implement Typeargumentconstraintresolution |
| 4493 | Implement Typeargumentdefaultusesconstraintoncirculardefault | spike | frontend/syntax | class: blocked | Implement Typeargumentdefaultusesconstraintoncirculardefault |
| 4494 | Implement Typeargumentinferencewithconstraintascommonroot | spike | frontend/resolver | class: blocked | Implement Typeargumentinferencewithconstraintascommonroot |
| 4495 | Implement Typeargumentinferencewithrecursivelyreferencedtypealiastotypeliteral | spike | frontend/syntax | class: blocked | Implement Typeargumentinferencewithrecursivelyreferencedtypealiastotypeliteral |
| 4496 | Implement Typeargumentsonfunctionswithnotypeparameters | spike | frontend/resolver | class: blocked | Implement Typeargumentsonfunctionswithnotypeparameters |
| 4497 | Implement Typeargumentsshoulddisallownongenericoverloads | spike | frontend/resolver | class: blocked | Implement Typeargumentsshoulddisallownongenericoverloads |
| 4498 | Implement Typeassertiontogenericfunctiontype | spike | frontend/syntax | class: blocked | Implement Typeassertiontogenericfunctiontype |
| 4499 | Implement Typeassignabilityerrormessage | spike | frontend/resolver | class: blocked | Implement Typeassignabilityerrormessage |
| 4500 | Implement Typecheckobjectcreationexpressionwithundefinedcallresolutiondata | spike | frontend/syntax | class: blocked | Implement Typecheckobjectcreationexpressionwithundefinedcallresolutiondata |
| 4501 | Implement Typechecktypeargument | spike | frontend/syntax | class: blocked | Implement Typechecktypeargument |
| 4502 | Implement Typecheckinginsidefunctionexpressioninarray | spike | frontend/resolver | class: blocked | Implement Typecheckinginsidefunctionexpressioninarray |
| 4503 | Implement Typecomparisoncaching | spike | frontend/resolver | class: blocked | Implement Typecomparisoncaching |
| 4504 | Implement Typeconstraintswithconstructsignatures | spike | frontend/syntax | class: blocked | Implement Typeconstraintswithconstructsignatures |
| 4505 | Implement Typeguardconstructorclassandnumber | spike | frontend/syntax | class: blocked | Implement Typeguardconstructorclassandnumber |
| 4506 | Implement Typeguardconstructorderivedclass | spike | frontend/syntax | class: blocked | Implement Typeguardconstructorderivedclass |
| 4507 | Implement Typeguardnarrowbymutableuntypedfield | spike | frontend/resolver | class: blocked | Implement Typeguardnarrowbymutableuntypedfield |
| 4508 | Implement Typeguardnarrowbyuntypedfield | spike | frontend/resolver | class: blocked | Implement Typeguardnarrowbyuntypedfield |
| 4509 | Implement Typeguardnarrowsindexedaccessofknownproperty Arrow Function | spike | frontend/syntax | class: blocked | Implement Typeguardnarrowsindexedaccessofknownproperty Arrow Function |
| 4510 | Implement Typeguardnarrowsindexedaccessofknownproperty Break Continue | spike | frontend/syntax | class: blocked | Implement Typeguardnarrowsindexedaccessofknownproperty Break Continue |
| 4511 | Implement Typeguardnarrowsindexedaccessofknownproperty Import Export | spike | frontend/syntax | class: blocked | Implement Typeguardnarrowsindexedaccessofknownproperty Import Export |
| 4512 | Implement Typeguardnarrowsindexedaccessofknownproperty Name Resolution | spike | frontend/resolver | class: blocked | Implement Typeguardnarrowsindexedaccessofknownproperty Name Resolution |
| 4513 | Implement Typeguardnarrowsindexedaccessofknownproperty Parser Syntax | spike | frontend/syntax | class: blocked | Implement Typeguardnarrowsindexedaccessofknownproperty Parser Syntax |
| 4514 | Implement Typeguardoncontainertypenohang | spike | frontend/syntax | class: blocked | Implement Typeguardoncontainertypenohang |
| 4515 | Implement Typeidentityconsidersbrands | spike | frontend/syntax | class: triage-needed | Implement Typeidentityconsidersbrands |
| 4516 | Implement Typeinfer | spike | frontend/semantics | class: blocked | Implement Typeinfer |
| 4517 | Implement Typeinferencecacheinvalidation | spike | frontend/resolver | class: blocked | Implement Typeinferencecacheinvalidation |
| 4518 | Implement Typeinferenceconflictingcandidates | spike | frontend/resolver | class: blocked | Implement Typeinferenceconflictingcandidates |
| 4519 | Implement Typeinferencefixearly | spike | frontend/resolver | class: blocked | Implement Typeinferencefixearly |
| 4520 | Implement Typeinferenceliteralunion | spike | frontend/semantics | class: blocked | Implement Typeinferenceliteralunion |
| 4521 | Implement Typeinferencereturntypecallback | spike | frontend/semantics | class: blocked | Implement Typeinferencereturntypecallback |
| 4522 | Implement Typeinferencetypepredicate Name Resolution | spike | frontend/resolver | class: blocked | Implement Typeinferencetypepredicate Name Resolution |
| 4523 | Implement Typeinferencetypepredicate Type System | spike | frontend/semantics | class: blocked | Implement Typeinferencetypepredicate Type System |
| 4524 | Implement Typeinferencewithtypeannotation | spike | frontend/resolver | class: blocked | Implement Typeinferencewithtypeannotation |
| 4525 | Implement Typeinterfacedeclarationsinblockstatements | spike | frontend/syntax | class: triage-needed | Implement Typeinterfacedeclarationsinblockstatements |
| 4526 | Implement Typeliteralcallback | spike | frontend/resolver | class: blocked | Implement Typeliteralcallback |
| 4527 | Implement Typematch | spike | frontend/syntax | class: triage-needed | Implement Typematch |
| 4528 | Implement Typename | spike | frontend/syntax | class: triage-needed | Implement Typename |
| 4529 | Implement Typenamedundefined | spike | frontend/syntax | class: blocked | Implement Typenamedundefined |
| 4530 | Implement Typeofenumandvarredeclarations | spike | frontend/syntax | class: triage-needed | Implement Typeofenumandvarredeclarations |
| 4531 | Implement Typeofprototype | spike | frontend/syntax | class: triage-needed | Implement Typeofprototype |
| 4532 | Implement Typeofthisinstatics | spike | frontend/syntax | class: blocked | Implement Typeofthisinstatics |
| 4533 | Implement Typeofyieldwithunionincontextualreturntype | spike | runtime/builtins | class: triage-needed | Implement Typeofyieldwithunionincontextualreturntype |
| 4534 | Implement Typeparamextendsothertypeparam | spike | frontend/syntax | class: triage-needed | Implement Typeparamextendsothertypeparam |
| 4535 | Implement Typeparameterandargumentofsamename | spike | frontend/syntax | class: blocked | Implement Typeparameterandargumentofsamename |
| 4536 | Implement Typeparameterargumentequivalence | spike | frontend/syntax | class: blocked | Implement Typeparameterargumentequivalence |
| 4537 | Implement Typeparameterasbaseclass | spike | frontend/syntax | class: blocked | Implement Typeparameterasbaseclass |
| 4538 | Implement Typeparameterassignmentcompat | spike | frontend/syntax | class: blocked | Implement Typeparameterassignmentcompat |
| 4539 | Implement Typeparametercompatibilityaccrossdeclarations | spike | frontend/syntax | class: blocked | Implement Typeparametercompatibilityaccrossdeclarations |
| 4540 | Implement Typeparameterconstrainedtooutertypeparameter | spike | frontend/resolver | class: blocked | Implement Typeparameterconstrainedtooutertypeparameter |
| 4541 | Implement Typeparameterconstraintinstantiation | spike | frontend/syntax | class: blocked | Implement Typeparameterconstraintinstantiation |
| 4542 | Implement Typeparameterdiamond | spike | frontend/syntax | class: blocked | Implement Typeparameterdiamond |
| 4543 | Implement Typeparameterdoesntblockparameterlookup | spike | frontend/syntax | class: blocked | Implement Typeparameterdoesntblockparameterlookup |
| 4544 | Implement Typeparameterequality | spike | reference/triage | class: triage-needed | Implement Typeparameterequality |
| 4545 | Implement Typeparameterexplicitlyextendsany | spike | frontend/syntax | class: blocked | Implement Typeparameterexplicitlyextendsany |
| 4546 | Implement Typeparameterextendingunion | spike | frontend/syntax | class: blocked | Implement Typeparameterextendingunion |
| 4547 | Implement Typeparameterextendsprimitive | spike | frontend/syntax | class: blocked | Implement Typeparameterextendsprimitive |
| 4548 | Implement Typeparameterfixingwithconstraints | spike | frontend/syntax | class: blocked | Implement Typeparameterfixingwithconstraints |
| 4549 | Implement Typeparameterfixingwithcontextsensitivearguments Arguments Object | spike | frontend/syntax | class: blocked | Implement Typeparameterfixingwithcontextsensitivearguments Arguments Object |
| 4550 | Implement Typeparameterfixingwithcontextsensitivearguments Name Resolution | spike | frontend/resolver | class: blocked | Implement Typeparameterfixingwithcontextsensitivearguments Name Resolution |
| 4551 | Implement Typeparameterinconstraint | spike | frontend/syntax | class: blocked | Implement Typeparameterinconstraint |
| 4552 | Implement Typeparameterleak | spike | frontend/resolver | class: blocked | Implement Typeparameterleak |
| 4553 | Implement Typeparameterlistwithtrailingcomma | spike | frontend/syntax | class: blocked | Implement Typeparameterlistwithtrailingcomma |
| 4554 | Implement Typeparameterwithinvalidconstrainttype | spike | frontend/syntax | class: blocked | Implement Typeparameterwithinvalidconstrainttype |
| 4555 | Implement Typeparametersandparametersincomputednames | spike | frontend/syntax | class: blocked | Implement Typeparametersandparametersincomputednames |
| 4556 | Implement Typeparametersinstaticaccessors | spike | frontend/syntax | class: blocked | Implement Typeparametersinstaticaccessors |
| 4557 | Implement Typeparametersinstaticmethods | spike | frontend/syntax | class: blocked | Implement Typeparametersinstaticmethods |
| 4558 | Implement Typeparametersinstaticproperties | spike | frontend/syntax | class: blocked | Implement Typeparametersinstaticproperties |
| 4559 | Implement Typeparametersshouldnotbeequal | spike | frontend/syntax | class: blocked | Implement Typeparametersshouldnotbeequal |
| 4560 | Implement Typepartameterconstraintinstantiatedwithdefaultwhencheckingdefault | spike | frontend/syntax | class: blocked | Implement Typepartameterconstraintinstantiatedwithdefaultwhencheckingdefault |
| 4561 | Implement Typepredicatefreshliteralwidening | spike | frontend/semantics | class: blocked | Implement Typepredicatefreshliteralwidening |
| 4562 | Implement Typepredicateinloop | spike | frontend/syntax | class: blocked | Implement Typepredicateinloop |
| 4563 | Implement Typepredicateinherit | spike | frontend/semantics | class: blocked | Implement Typepredicateinherit |
| 4564 | Implement Typepredicatestructuralmatch | spike | frontend/semantics | class: blocked | Implement Typepredicatestructuralmatch |
| 4565 | Implement Typepredicatetopleveltypeparameter | spike | frontend/semantics | class: blocked | Implement Typepredicatetopleveltypeparameter |
| 4566 | Implement Typepredicatewiththisparameter | spike | frontend/semantics | class: blocked | Implement Typepredicatewiththisparameter |
| 4567 | Implement Typepredicatescannarrowbydiscriminant | spike | frontend/syntax | class: blocked | Implement Typepredicatescannarrowbydiscriminant |
| 4568 | Implement Typepredicatesinunion Name Resolution | spike | frontend/resolver | class: blocked | Implement Typepredicatesinunion Name Resolution |
| 4569 | Implement Typepredicatesinunion Type System | spike | frontend/semantics | class: blocked | Implement Typepredicatesinunion Type System |
| 4570 | Implement Typepredicatesoptionalchaining Name Resolution | spike | frontend/resolver | class: blocked | Implement Typepredicatesoptionalchaining Name Resolution |
| 4571 | Implement Typepredicatesoptionalchaining Type System | spike | frontend/semantics | class: blocked | Implement Typepredicatesoptionalchaining Type System |
| 4572 | Implement Typereferencedirectivescopedpackagecustomtyperoot | spike | frontend/resolver | class: blocked | Implement Typereferencedirectivescopedpackagecustomtyperoot |
| 4573 | Implement Typereferencedirectivewithfailedfromtyperoot | spike | frontend/syntax | class: triage-needed | Implement Typereferencedirectivewithfailedfromtyperoot |
| 4574 | Implement Typereferencedirectivewithtypeasfile | spike | frontend/resolver | class: blocked | Implement Typereferencedirectivewithtypeasfile |
| 4575 | Implement Typereferencedirectives Import Export | spike | frontend/syntax | class: blocked | Implement Typereferencedirectives Import Export |
| 4576 | Implement Typereferencedirectives Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Typereferencedirectives Parser Syntax |
| 4577 | Implement Typeresolution | spike | frontend/syntax | class: blocked | Implement Typeresolution |
| 4578 | Implement Typerootsfrommultiplenodemodulesdirectories | spike | frontend/syntax | class: blocked | Implement Typerootsfrommultiplenodemodulesdirectories |
| 4579 | Implement Typerootsfromnodemodulesinparentdirectory | spike | frontend/syntax | class: blocked | Implement Typerootsfromnodemodulesinparentdirectory |
| 4580 | Implement Typeusedastypeliteralindex | spike | frontend/resolver | class: blocked | Implement Typeusedastypeliteralindex |
| 4581 | Implement Typeusedasvalueerror Import Export | spike | frontend/syntax | class: blocked | Implement Typeusedasvalueerror Import Export |
| 4582 | Implement Typeusedasvalueerror Name Resolution | spike | frontend/resolver | class: blocked | Implement Typeusedasvalueerror Name Resolution |
| 4583 | Implement Typevalueconflict | spike | frontend/syntax | class: blocked | Implement Typevalueconflict |
| 4584 | Implement Typevariableconstraintintersections | spike | frontend/resolver | class: blocked | Implement Typevariableconstraintintersections |
| 4585 | Implement Typevariableconstraintedtoaliasnotassignabletounion | spike | frontend/syntax | class: blocked | Implement Typevariableconstraintedtoaliasnotassignabletounion |
| 4586 | Implement Typevariabletypeguards | spike | frontend/syntax | class: blocked | Implement Typevariabletypeguards |
| 4587 | Implement Typecheckcommaexpression | spike | frontend/syntax | class: triage-needed | Implement Typecheckcommaexpression |
| 4588 | Implement Typecheckifcondition | spike | frontend/resolver | class: blocked | Implement Typecheckifcondition |
| 4589 | Implement Typedarrayconstructoroverloads | spike | frontend/syntax | class: blocked | Implement Typedarrayconstructoroverloads |
| 4590 | Implement Typedarrays Name Resolution | spike | frontend/resolver | class: blocked | Implement Typedarrays Name Resolution |
| 4591 | Implement Typedarrays Parser Syntax | spike | runtime/builtins | class: triage-needed | Implement Typedarrays Parser Syntax |
| 4592 | Implement Typedarrayscrossassignability | spike | frontend/resolver | class: blocked | Implement Typedarrayscrossassignability |
| 4594 | Implement Typedgenericprototypemember | spike | frontend/semantics | class: blocked | Implement Typedgenericprototypemember |
| 4595 | Implement Typeofambientexternalmodules | spike | frontend/syntax | class: blocked | Implement Typeofambientexternalmodules |
| 4596 | Implement Typeofclass | spike | frontend/resolver | class: blocked | Implement Typeofclass |
| 4597 | Implement Typeofenum | spike | frontend/syntax | class: triage-needed | Implement Typeofenum |
| 4598 | Implement Typeofexternalmodules | spike | frontend/syntax | class: blocked | Implement Typeofexternalmodules |
| 4599 | Implement Typeofimportinstantiationexpression | spike | frontend/syntax | class: blocked | Implement Typeofimportinstantiationexpression |
| 4600 | Implement Typeofinternalmodules | spike | frontend/syntax | class: blocked | Implement Typeofinternalmodules |
| 4601 | Implement Typeofobjectinference | spike | frontend/semantics | class: blocked | Implement Typeofobjectinference |
| 4602 | Implement Typeofproperty | spike | frontend/syntax | class: triage-needed | Implement Typeofproperty |
| 4603 | Implement Typeofsimple | spike | frontend/resolver | class: blocked | Implement Typeofsimple |
| 4604 | Implement Typeofstripsfreshness | spike | frontend/resolver | class: blocked | Implement Typeofstripsfreshness |
| 4605 | Implement Typeofthisinmethodsignature | spike | frontend/syntax | class: blocked | Implement Typeofthisinmethodsignature |
| 4606 | Implement Typeofundefined | spike | reference/triage | class: triage-needed | Implement Typeofundefined |
| 4607 | Implement Typeofunknownsymbol | spike | frontend/syntax | class: triage-needed | Implement Typeofunknownsymbol |
| 4608 | Implement Typeofusedbeforeblockscoped | spike | frontend/resolver | class: blocked | Implement Typeofusedbeforeblockscoped |
| 4609 | Implement Umddependencycomment | spike | frontend/syntax | class: blocked | Implement Umddependencycomment |
| 4610 | Implement Umddependencycommentname | spike | frontend/syntax | class: blocked | Implement Umddependencycommentname |
| 4611 | Implement Umdglobalaugmentationnocrash | spike | frontend/syntax | class: triage-needed | Implement Umdglobalaugmentationnocrash |
| 4612 | Implement Umdglobalconflict | spike | frontend/syntax | class: blocked | Implement Umdglobalconflict |
| 4613 | Implement Umdnamedamdmode | spike | frontend/syntax | class: blocked | Implement Umdnamedamdmode |
| 4614 | Implement Umdnamespacemergedwithglobalaugmentationisnotcircular | spike | frontend/syntax | class: triage-needed | Implement Umdnamespacemergedwithglobalaugmentationisnotcircular |
| 4615 | Implement Unaryoperators | spike | frontend/resolver | class: blocked | Implement Unaryoperators |
| 4616 | Implement Unaryoperatorsinstrictmode | spike | reference/triage | class: triage-needed | Implement Unaryoperatorsinstrictmode |
| 4617 | Implement Unaryplus | spike | frontend/syntax | class: triage-needed | Implement Unaryplus |
| 4618 | Implement Uncalledfunctionchecksinconditional Name Resolution | spike | frontend/resolver | class: blocked | Implement Uncalledfunctionchecksinconditional Name Resolution |
| 4619 | Implement Uncalledfunctionchecksinconditional Type System | spike | frontend/semantics | class: blocked | Implement Uncalledfunctionchecksinconditional Type System |
| 4620 | Implement Uncalledfunctionchecksinconditionalperf | spike | frontend/resolver | class: blocked | Implement Uncalledfunctionchecksinconditionalperf |
| 4621 | Implement Uncaughtcompilererror Name Resolution | spike | frontend/resolver | class: blocked | Implement Uncaughtcompilererror Name Resolution |
| 4622 | Implement Uncaughtcompilererror Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Uncaughtcompilererror Unknown Unsupported |
| 4623 | Implement Unclosedexportclause | spike | frontend/syntax | class: blocked | Implement Unclosedexportclause |
| 4624 | Implement Undeclaredbase | spike | frontend/syntax | class: blocked | Implement Undeclaredbase |
| 4625 | Implement Undeclaredmethod | spike | frontend/syntax | class: blocked | Implement Undeclaredmethod |
| 4626 | Implement Undeclaredmoduleerror | spike | frontend/syntax | class: blocked | Implement Undeclaredmoduleerror |
| 4627 | Implement Undeclaredvaremit | spike | frontend/resolver | class: blocked | Implement Undeclaredvaremit |
| 4628 | Implement Undefinedasdiscriminantwithunknown | spike | frontend/syntax | class: triage-needed | Implement Undefinedasdiscriminantwithunknown |
| 4629 | Implement Undefinedassignabletogenericmappedintersection | spike | frontend/semantics | class: blocked | Implement Undefinedassignabletogenericmappedintersection |
| 4630 | Implement Undefinedsymbolreferencedinarrayliteral | spike | frontend/resolver | class: blocked | Implement Undefinedsymbolreferencedinarrayliteral |
| 4631 | Implement Undefinedtypeargument | spike | frontend/syntax | class: blocked | Implement Undefinedtypeargument |
| 4632 | Implement Undefinedtypeassignment Operator | spike | frontend/syntax | class: blocked | Implement Undefinedtypeassignment Operator |
| 4633 | Implement Undefinedtypeassignment Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Undefinedtypeassignment Parser Syntax |
| 4634 | Implement Underscoreescapednameinenum | spike | frontend/syntax | class: triage-needed | Implement Underscoreescapednameinenum |
| 4635 | Implement Underscoremapfirst | spike | frontend/syntax | class: blocked | Implement Underscoremapfirst |
| 4636 | Implement Underscoretest | spike | frontend/syntax | class: blocked | Implement Underscoretest |
| 4637 | Implement Unexpectedstatementblockterminator | spike | frontend/syntax | class: triage-needed | Implement Unexpectedstatementblockterminator |
| 4638 | Implement Unexportedinstanceclassvariables | spike | frontend/syntax | class: blocked | Implement Unexportedinstanceclassvariables |
| 4639 | Implement Unicodeescapesinnames Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Unicodeescapesinnames Parser Syntax |
| 4640 | Implement Unicodeescapesinnames Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Unicodeescapesinnames Unknown Unsupported |
| 4641 | Implement Unicodeidentifiername | spike | frontend/syntax | class: triage-needed | Implement Unicodeidentifiername |
| 4642 | Implement Unicodestringliteral | spike | frontend/syntax | class: triage-needed | Implement Unicodestringliteral |
| 4643 | Implement Unioncallmixedtypeparameterpresence | spike | frontend/resolver | class: blocked | Implement Unioncallmixedtypeparameterpresence |
| 4644 | Implement Unionexcesspropertychecknoapparentproptypemismatcherrors | spike | frontend/resolver | class: blocked | Implement Unionexcesspropertychecknoapparentproptypemismatcherrors |
| 4645 | Implement Unionexcesspropswithpartialmember | spike | frontend/resolver | class: blocked | Implement Unionexcesspropswithpartialmember |
| 4646 | Implement Unionofarraysfiltercall | spike | frontend/syntax | class: triage-needed | Implement Unionofarraysfiltercall |
| 4647 | Implement Unionofclasscalls | spike | frontend/syntax | class: triage-needed | Implement Unionofclasscalls |
| 4648 | Implement Unionofenuminference | spike | frontend/syntax | class: triage-needed | Implement Unionofenuminference |
| 4649 | Implement Unionoffunctionandsignatureiscallable | spike | frontend/syntax | class: triage-needed | Implement Unionoffunctionandsignatureiscallable |
| 4650 | Implement Unionpropertyexistence | spike | frontend/resolver | class: blocked | Implement Unionpropertyexistence |
| 4651 | Implement Unionpropertyofprotectedandintersectionproperty | spike | frontend/syntax | class: blocked | Implement Unionpropertyofprotectedandintersectionproperty |
| 4652 | Implement Unionreductionmutualsubtypes | spike | frontend/syntax | class: triage-needed | Implement Unionreductionmutualsubtypes |
| 4653 | Implement Unionrelationshipcheckpasses | spike | frontend/syntax | class: triage-needed | Implement Unionrelationshipcheckpasses |
| 4654 | Implement Unionsignatureswiththisparameter | spike | frontend/syntax | class: blocked | Implement Unionsignatureswiththisparameter |
| 4655 | Implement Uniontypeerrormessagetyperefs | spike | frontend/resolver | class: blocked | Implement Uniontypeerrormessagetyperefs |
| 4656 | Implement Uniontypeparameterinference | spike | frontend/resolver | class: blocked | Implement Uniontypeparameterinference |
| 4657 | Implement Uniontypewithindexandmethodsignature | spike | frontend/resolver | class: blocked | Implement Uniontypewithindexandmethodsignature |
| 4658 | Implement Uniontypewithrecursivesubtypereduction Name Resolution | spike | frontend/resolver | class: blocked | Implement Uniontypewithrecursivesubtypereduction Name Resolution |
| 4659 | Implement Uniontypewithrecursivesubtypereduction Parser Syntax | spike | frontend/syntax | class: blocked | Implement Uniontypewithrecursivesubtypereduction Parser Syntax |
| 4660 | Implement Unionwithindexsignature | spike | frontend/syntax | class: blocked | Implement Unionwithindexsignature |
| 4661 | Implement Uniquesymbolallowsindexinobjectwithindexsignature | spike | frontend/syntax | class: blocked | Implement Uniquesymbolallowsindexinobjectwithindexsignature |
| 4662 | Implement Uniquesymbolassignmentonglobalaugmentationsuceeds | spike | frontend/syntax | class: triage-needed | Implement Uniquesymbolassignmentonglobalaugmentationsuceeds |
| 4663 | Implement Uniquesymboljs Function Resolution | spike | frontend/resolver | class: blocked | Implement Uniquesymboljs Function Resolution |
| 4664 | Implement Uniquesymboljs Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Uniquesymboljs Parser Syntax |
| 4665 | Implement Uniquesymbolpropertydeclarationemit | spike | frontend/syntax | class: blocked | Implement Uniquesymbolpropertydeclarationemit |
| 4666 | Implement Unknownlikeunionobjectflagsnotpropagated | spike | reference/triage | class: triage-needed | Implement Unknownlikeunionobjectflagsnotpropagated |
| 4667 | Implement Unknownsymbolingenericreturntype | spike | frontend/semantics | class: blocked | Implement Unknownsymbolingenericreturntype |
| 4668 | Implement Unknownsymboloffcontextualtype | spike | frontend/resolver | class: blocked | Implement Unknownsymboloffcontextualtype |
| 4669 | Implement Unknownsymbols Import Export | spike | frontend/syntax | class: blocked | Implement Unknownsymbols Import Export |
| 4670 | Implement Unknownsymbols Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Unknownsymbols Unknown Unsupported |
| 4671 | Implement Unknowntypeargoncall | spike | frontend/syntax | class: blocked | Implement Unknowntypeargoncall |
| 4672 | Implement Unmatchedparameterpositions | spike | frontend/resolver | class: blocked | Implement Unmatchedparameterpositions |
| 4673 | Implement Unmettypeconstraintinjsdocimportcall | spike | frontend/syntax | class: blocked | Implement Unmettypeconstraintinjsdocimportcall |
| 4674 | Implement Unqualifiedcalltoclassstatic | spike | frontend/syntax | class: triage-needed | Implement Unqualifiedcalltoclassstatic |
| 4675 | Implement Unreachabledeclarations | spike | frontend/syntax | class: blocked | Implement Unreachabledeclarations |
| 4676 | Implement Unreachableflowafterfinally | spike | frontend/syntax | class: blocked | Implement Unreachableflowafterfinally |
| 4677 | Implement Unreachablejavascriptchecked | spike | frontend/resolver | class: blocked | Implement Unreachablejavascriptchecked |
| 4678 | Implement Unreachableswitchtypeofany | spike | frontend/syntax | class: blocked | Implement Unreachableswitchtypeofany |
| 4679 | Implement Unreachableswitchtypeofunknown | spike | frontend/syntax | class: blocked | Implement Unreachableswitchtypeofunknown |
| 4680 | Implement Unresolvableselfreferencingawaitedunion | spike | frontend/syntax | class: blocked | Implement Unresolvableselfreferencingawaitedunion |
| 4681 | Implement Unresolvedtypeassertionsymbol | spike | frontend/syntax | class: blocked | Implement Unresolvedtypeassertionsymbol |
| 4682 | Implement Unspecializedconstraints | spike | frontend/syntax | class: blocked | Implement Unspecializedconstraints |
| 4683 | Implement Unterminatedregexatendofsource | spike | reference/triage | class: triage-needed | Implement Unterminatedregexatendofsource |
| 4684 | Implement Unterminatedstringliteralwithbackslash | spike | frontend/syntax | class: triage-needed | Implement Unterminatedstringliteralwithbackslash |
| 4685 | Implement Untypedargumentinlambdaexpression | spike | frontend/resolver | class: blocked | Implement Untypedargumentinlambdaexpression |
| 4686 | Implement Untypedfunctioncallswithtypeparameters | spike | frontend/syntax | class: triage-needed | Implement Untypedfunctioncallswithtypeparameters |
| 4687 | Implement Untypedmoduleimport | spike | frontend/syntax | class: blocked | Implement Untypedmoduleimport |
| 4688 | Implement Unusedclassesinmodule | spike | frontend/syntax | class: blocked | Implement Unusedclassesinmodule |
| 4689 | Implement Unusedclassesinnamespace | spike | frontend/syntax | class: blocked | Implement Unusedclassesinnamespace |
| 4690 | Implement Unuseddestructuring | spike | frontend/resolver | class: blocked | Implement Unuseddestructuring |
| 4691 | Implement Unusedfunctionsinnamespaces | spike | frontend/syntax | class: blocked | Implement Unusedfunctionsinnamespaces |
| 4692 | Implement Unusedgetterinclass | spike | frontend/syntax | class: blocked | Implement Unusedgetterinclass |
| 4693 | Implement Unusedidentifiersconsolidated | spike | frontend/syntax | class: blocked | Implement Unusedidentifiersconsolidated |
| 4694 | Implement Unusedimportdeclaration | spike | frontend/syntax | class: blocked | Implement Unusedimportdeclaration |
| 4695 | Implement Unusedimportwithspread | spike | frontend/syntax | class: blocked | Implement Unusedimportwithspread |
| 4696 | Implement Unusedimports Import Export | spike | frontend/syntax | class: blocked | Implement Unusedimports Import Export |
| 4697 | Implement Unusedimports Regexp Literal | spike | reference/triage | class: triage-needed | Implement Unusedimports Regexp Literal |
| 4698 | Implement Unusedinterfaceinnamespace | spike | frontend/syntax | class: blocked | Implement Unusedinterfaceinnamespace |
| 4699 | Implement Unusedinvalidtypearguments | spike | frontend/syntax | class: blocked | Implement Unusedinvalidtypearguments |
| 4700 | Implement Unusedlocalproperty | spike | frontend/syntax | class: blocked | Implement Unusedlocalproperty |
| 4701 | Implement Unusedlocalsandobjectspread Name Resolution | spike | frontend/resolver | class: blocked | Implement Unusedlocalsandobjectspread Name Resolution |
| 4702 | Implement Unusedlocalsandobjectspread Runtime Subset | spike | reference/triage | class: triage-needed | Implement Unusedlocalsandobjectspread Runtime Subset |
| 4703 | Implement Unusedlocalsandparameters | spike | frontend/syntax | class: blocked | Implement Unusedlocalsandparameters |
| 4704 | Implement Unusedlocalsandparametersdeferred | spike | frontend/syntax | class: blocked | Implement Unusedlocalsandparametersdeferred |
| 4705 | Implement Unusedlocalsandparametersoverloadsignatures | spike | frontend/syntax | class: blocked | Implement Unusedlocalsandparametersoverloadsignatures |
| 4706 | Implement Unusedlocalsandparameterstypealiases | spike | frontend/syntax | class: blocked | Implement Unusedlocalsandparameterstypealiases |
| 4707 | Implement Unusedlocalsinforinorof | spike | frontend/syntax | class: blocked | Implement Unusedlocalsinforinorof |
| 4708 | Implement Unusedlocalsinmethod | spike | frontend/syntax | class: blocked | Implement Unusedlocalsinmethod |
| 4709 | Implement Unusedlocalsonfunctiondeclarationwithinfunctionexpression | spike | frontend/syntax | class: blocked | Implement Unusedlocalsonfunctiondeclarationwithinfunctionexpression |
| 4710 | Implement Unusedlocalsonfunctionexpressionwithinfunctiondeclaration | spike | frontend/syntax | class: blocked | Implement Unusedlocalsonfunctionexpressionwithinfunctiondeclaration |
| 4711 | Implement Unusedlocalsonfunctionexpressionwithinfunctionexpression | spike | frontend/syntax | class: blocked | Implement Unusedlocalsonfunctionexpressionwithinfunctionexpression |
| 4712 | Implement Unusedlocalsstartingwithunderscore | spike | frontend/syntax | class: blocked | Implement Unusedlocalsstartingwithunderscore |
| 4713 | Implement Unusedmoduleinmodule | spike | frontend/syntax | class: blocked | Implement Unusedmoduleinmodule |
| 4714 | Implement Unusedmultipleparameter | spike | frontend/syntax | class: blocked | Implement Unusedmultipleparameter |
| 4715 | Implement Unusedmultipleparameters | spike | frontend/syntax | class: blocked | Implement Unusedmultipleparameters |
| 4716 | Implement Unusednamespaceinmodule | spike | frontend/syntax | class: blocked | Implement Unusednamespaceinmodule |
| 4717 | Implement Unusednamespaceinnamespace | spike | frontend/syntax | class: blocked | Implement Unusednamespaceinnamespace |
| 4718 | Implement Unusedparametersinlambda | spike | frontend/syntax | class: blocked | Implement Unusedparametersinlambda |
| 4719 | Implement Unusedparametersthis | spike | frontend/syntax | class: blocked | Implement Unusedparametersthis |
| 4720 | Implement Unusedprivatemembers | spike | frontend/syntax | class: blocked | Implement Unusedprivatemembers |
| 4721 | Implement Unusedprivatemethodinclass | spike | frontend/syntax | class: blocked | Implement Unusedprivatemethodinclass |
| 4722 | Implement Unusedprivatestaticmembers | spike | frontend/syntax | class: blocked | Implement Unusedprivatestaticmembers |
| 4723 | Implement Unusedprivatevariableinclass | spike | frontend/syntax | class: blocked | Implement Unusedprivatevariableinclass |
| 4724 | Implement Unusedsetterinclass | spike | frontend/syntax | class: blocked | Implement Unusedsetterinclass |
| 4725 | Implement Unusedsingleparameterinfunctionexpression | spike | frontend/syntax | class: blocked | Implement Unusedsingleparameterinfunctionexpression |
| 4726 | Implement Unusedsingleparameterinmethoddeclaration | spike | frontend/syntax | class: blocked | Implement Unusedsingleparameterinmethoddeclaration |
| 4727 | Implement Unusedswitchstatement | spike | frontend/resolver | class: blocked | Implement Unusedswitchstatement |
| 4728 | Implement Unusedtypeparameterinfunction | spike | frontend/syntax | class: blocked | Implement Unusedtypeparameterinfunction |
| 4729 | Implement Unusedtypeparameterinlambda | spike | frontend/syntax | class: blocked | Implement Unusedtypeparameterinlambda |
| 4730 | Implement Unusedtypeparameterinmethod | spike | frontend/syntax | class: blocked | Implement Unusedtypeparameterinmethod |
| 4731 | Implement Unusedtypeparameters | spike | frontend/syntax | class: blocked | Implement Unusedtypeparameters |
| 4732 | Implement Unusedtypeparameterscheckedbynounusedparameters | spike | frontend/syntax | class: blocked | Implement Unusedtypeparameterscheckedbynounusedparameters |
| 4733 | Implement Unusedtypeparametersnotcheckedbynounusedlocals | spike | frontend/syntax | class: blocked | Implement Unusedtypeparametersnotcheckedbynounusedlocals |
| 4734 | Implement Unusedtypeparameterswithunderscore | spike | frontend/syntax | class: blocked | Implement Unusedtypeparameterswithunderscore |
| 4735 | Implement Unusedvariableswithunderscoreinbindingelement | spike | reference/triage | class: triage-needed | Implement Unusedvariableswithunderscoreinbindingelement |
| 4736 | Implement Unusedvariableswithunderscoreinforofloop | spike | frontend/syntax | class: triage-needed | Implement Unusedvariableswithunderscoreinforofloop |
| 4737 | Implement Unusedvariablesinblocks | spike | frontend/syntax | class: blocked | Implement Unusedvariablesinblocks |
| 4738 | Implement Unusedvariablesinmodules | spike | frontend/syntax | class: blocked | Implement Unusedvariablesinmodules |
| 4739 | Implement Unusedvariablesinnamespaces | spike | frontend/syntax | class: blocked | Implement Unusedvariablesinnamespaces |
| 4740 | Implement Unwitnessedtypeparametervariance | spike | frontend/syntax | class: blocked | Implement Unwitnessedtypeparametervariance |
| 4741 | Implement Usebeforedeclaration Decorator | spike | frontend/syntax | class: blocked | Implement Usebeforedeclaration Decorator |
| 4742 | Implement Usebeforedeclaration Destructuring | spike | frontend/syntax | class: blocked | Implement Usebeforedeclaration Destructuring |
| 4743 | Implement Usebeforedeclaration Import Export | spike | frontend/syntax | class: blocked | Implement Usebeforedeclaration Import Export |
| 4744 | Implement Usebeforedeclaration Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Usebeforedeclaration Parser Syntax |
| 4745 | Implement Usebeforedefinitionindeclarationfiles | spike | frontend/syntax | class: blocked | Implement Usebeforedefinitionindeclarationfiles |
| 4746 | Implement Usedefineforclassfieldsflagdefault | spike | frontend/syntax | class: blocked | Implement Usedefineforclassfieldsflagdefault |
| 4747 | Implement Usestrictlikeprologuestring | spike | frontend/syntax | class: triage-needed | Implement Usestrictlikeprologuestring |
| 4748 | Implement Useunknownincatchvariables | spike | frontend/syntax | class: triage-needed | Implement Useunknownincatchvariables |
| 4749 | Implement Usedimportnotelidedinjs | spike | frontend/syntax | class: blocked | Implement Usedimportnotelidedinjs |
| 4750 | Implement Usingmodulewithexportimportinvalueposition | spike | frontend/syntax | class: blocked | Implement Usingmodulewithexportimportinvalueposition |
| 4751 | Implement Validregexp | spike | runtime/builtins | class: triage-needed | Implement Validregexp |
| 4752 | Implement Validuseofthisinsuper | spike | frontend/syntax | class: blocked | Implement Validuseofthisinsuper |
| 4753 | Implement Valueoftypedarray | spike | frontend/resolver | class: blocked | Implement Valueoftypedarray |
| 4754 | Implement Varandfunctionsharename | spike | reference/triage | class: triage-needed | Implement Varandfunctionsharename |
| 4755 | Implement Varargconstructormemberparameter | spike | frontend/syntax | class: blocked | Implement Varargconstructormemberparameter |
| 4756 | Implement Varargparamtypecheck | spike | frontend/syntax | class: blocked | Implement Varargparamtypecheck |
| 4757 | Implement Varargwithnoparamname | spike | frontend/syntax | class: triage-needed | Implement Varargwithnoparamname |
| 4758 | Implement Varargsonconstructortypes | spike | frontend/syntax | class: blocked | Implement Varargsonconstructortypes |
| 4759 | Implement Varasid | spike | frontend/syntax | class: triage-needed | Implement Varasid |
| 4760 | Implement Varblock | spike | frontend/syntax | class: blocked | Implement Varblock |
| 4761 | Implement Varnameconflictswithimportindifferentpartofmodule | spike | frontend/syntax | class: blocked | Implement Varnameconflictswithimportindifferentpartofmodule |
| 4762 | Implement Vararg | spike | frontend/syntax | class: blocked | Implement Vararg |
| 4763 | Implement Vardecl | spike | frontend/syntax | class: triage-needed | Implement Vardecl |
| 4764 | Implement Variabledeclarationdeclarationemituniquesymbolpartialstatement | spike | frontend/syntax | class: blocked | Implement Variabledeclarationdeclarationemituniquesymbolpartialstatement |
| 4765 | Implement Variabledeclarationinnercommentemit | spike | reference/triage | class: triage-needed | Implement Variabledeclarationinnercommentemit |
| 4766 | Implement Variabledeclaratorresolvedduringcontextualtyping | spike | frontend/syntax | class: blocked | Implement Variabledeclaratorresolvedduringcontextualtyping |
| 4767 | Implement Varianceannotationvalidation | spike | frontend/syntax | class: triage-needed | Implement Varianceannotationvalidation |
| 4768 | Implement Variancecantbestrictwhilestructureisnt | spike | frontend/resolver | class: blocked | Implement Variancecantbestrictwhilestructureisnt |
| 4769 | Implement Variancemeasurement | spike | frontend/syntax | class: triage-needed | Implement Variancemeasurement |
| 4770 | Implement Varianceproblingandzeroorderindexsignaturerelationsalign | spike | frontend/syntax | class: triage-needed | Implement Varianceproblingandzeroorderindexsignaturerelationsalign |
| 4771 | Implement Variancepropagation | spike | frontend/syntax | class: triage-needed | Implement Variancepropagation |
| 4772 | Implement Variancereferences | spike | frontend/resolver | class: blocked | Implement Variancereferences |
| 4773 | Implement Variancerepeatedlypropegateswithunreliableflag | spike | frontend/syntax | class: blocked | Implement Variancerepeatedlypropegateswithunreliableflag |
| 4774 | Implement Verbatim | spike | frontend/syntax | class: blocked | Implement Verbatim |
| 4775 | Implement Verbatimmodulesyntaxdefaultvalue | spike | frontend/syntax | class: blocked | Implement Verbatimmodulesyntaxdefaultvalue |
| 4776 | Implement Verbatimmodulesyntaxreactreference | spike | reference/triage | class: triage-needed | Implement Verbatimmodulesyntaxreactreference |
| 4777 | Implement Vissyntax | spike | frontend/syntax | class: blocked | Implement Vissyntax |
| 4778 | Implement Visibilityofcrossmoduletypeusage | spike | frontend/syntax | class: blocked | Implement Visibilityofcrossmoduletypeusage |
| 4779 | Implement Visibilityoftypeparameters | spike | frontend/syntax | class: blocked | Implement Visibilityoftypeparameters |
| 4780 | Implement Voidarraylit | spike | frontend/syntax | class: blocked | Implement Voidarraylit |
| 4781 | Implement Voidasnonambiguousreturntype | spike | frontend/syntax | class: blocked | Implement Voidasnonambiguousreturntype |
| 4782 | Implement Voidasoperator | spike | frontend/syntax | class: blocked | Implement Voidasoperator |
| 4783 | Implement Voidconstructor | spike | frontend/syntax | class: triage-needed | Implement Voidconstructor |
| 4784 | Implement Voidfunctionassignmentcompat | spike | frontend/syntax | class: blocked | Implement Voidfunctionassignmentcompat |
| 4785 | Implement Voidoperator | spike | frontend/syntax | class: blocked | Implement Voidoperator |
| 4786 | Implement Voidreturnindexunioninference | spike | frontend/syntax | class: blocked | Implement Voidreturnindexunioninference |
| 4787 | Implement Voidreturnlambdavalue | spike | frontend/syntax | class: blocked | Implement Voidreturnlambdavalue |
| 4788 | Implement Voidundefinedreduction | spike | frontend/resolver | class: blocked | Implement Voidundefinedreduction |
| 4789 | Implement Vuelikedataandpropsinference | spike | frontend/semantics | class: blocked | Implement Vuelikedataandpropsinference |
| 4790 | Implement Weaktype | spike | frontend/syntax | class: blocked | Implement Weaktype |
| 4791 | Implement Weaktypeandprimitivenarrowing | spike | frontend/syntax | class: blocked | Implement Weaktypeandprimitivenarrowing |
| 4792 | Implement Webworkeriterable | spike | runtime/builtins | class: triage-needed | Implement Webworkeriterable |
| 4793 | Implement Wellknownsymbolexpando | spike | frontend/resolver | class: blocked | Implement Wellknownsymbolexpando |
| 4794 | Implement Widenedtypes | spike | frontend/syntax | class: triage-needed | Implement Widenedtypes |
| 4795 | Implement Wideningwithtopleveltypeparameter | spike | frontend/syntax | class: blocked | Implement Wideningwithtopleveltypeparameter |
| 4796 | Implement Withexportdecl | spike | frontend/syntax | class: blocked | Implement Withexportdecl |
| 4797 | Implement Withimportdecl | spike | frontend/syntax | class: blocked | Implement Withimportdecl |
| 4798 | Implement Withstatement | spike | frontend/syntax | class: triage-needed | Implement Withstatement |
| 4799 | Implement Withstatementerrors | spike | runtime/builtins | class: triage-needed | Implement Withstatementerrors |
| 4800 | Implement Withstatementinternalcomments | spike | frontend/resolver | class: blocked | Implement Withstatementinternalcomments |
| 4801 | Implement Withstatementnestedscope | spike | frontend/resolver | class: blocked | Implement Withstatementnestedscope |
| 4802 | Implement Wrappedincovations | spike | frontend/syntax | class: blocked | Implement Wrappedincovations |
| 4803 | Implement Wrappedrecursivegenerictype | spike | frontend/resolver | class: blocked | Implement Wrappedrecursivegenerictype |
| 4804 | Implement Yieldstarcontextualtype | spike | frontend/syntax | class: blocked | Implement Yieldstarcontextualtype |
| 4805 | Implement Yieldstringliteral | spike | runtime/builtins | class: triage-needed | Implement Yieldstringliteral |
| 4807 | Implement decorator support | spike | frontend/syntax | class: blocked | Implement decorator support |
| 4812 | Implement RegExp literal support (dup) | spike | runtime/builtins | class: blocked | Implement RegExp literal support (dup) |
| 5012 | Implement Date object support | spike | frontend/syntax | class: triage-needed | Implement Date object support |
| 5015 | Implement function support | spike | frontend/syntax | class: triage-needed | Implement function support |
| 5018 | Implement legacy-global-builtin support | spike | frontend/syntax | class: triage-needed | Implement legacy-global-builtin support |
| 5020 | Implement RegExp literal support | spike | frontend/syntax | class: triage-needed | Implement RegExp literal support |
| 5134 | Admit generators and async-functions features through Python test262 harness | spike | scripts | 416 | Admit generators and async-functions features through Python test262 harness |
| 5135 | Fix builtin arity validation for coercion/math globals | bug | ir | class: blocked | Fix builtin arity validation for coercion/math globals |
| 5136 | Fix arity validation for RegExp/String prototype methods | bug | ir | class: blocked | Fix arity validation for RegExp/String prototype methods |
| 5398 | Resolve namespace import-equals alias value access | feature | frontend/name-resolution | 5287 | Resolve namespace import-equals alias value access |
| 5399 | Resolve ambient namespace import alias in declare module | feature | frontend/name-resolution | 5370 | Resolve ambient namespace import alias in declare module |
| 5404 | Bind dotted ambient namespace qualified access | feature | frontend/resolver | 5370 | Bind dotted ambient namespace qualified access |
| 5428 | Resolve symlinked node_modules static re-exports | feature | compiler/module-graph | 5426 | Resolve symlinked node_modules static re-exports |
<!-- generated:blocked:end -->

## Done queue

<!-- generated:done:start -->
| ID | Title | Type | Area | Completed evidence |
|---:|---|---|---|---|
| 000 | Short imperative title | feature | bug | refactor | docs | test | infra | cleanup | spike | frontend | ir | runtime | abi | wasi | cli | fixtures | scripts | docs | tests | coverage | reference | see `issues/done/000-sample-issue.md` |
| 001 | Fix issue infrastructure and current-state path references | infra | issues/docs | see `issues/done/001-fix-issue-infrastructure-and-current-state-path-references.md` |
| 002 | Emit canonical capability manifest schema | feature | abi/wasi | see `issues/done/002-emit-canonical-capability-manifest-schema.md` |
| 003 | Verify manifest against emitted WAT imports (audit reopened #003) | test | wasi/tests | see `issues/done/003-verify-manifest-against-emitted-wat-imports.md` |
| 004 | Reclassify compile-only compatibility tests | test | tests/coverage | see `issues/done/004-reclassify-compile-only-compatibility-tests.md` |
| 005 | Add fine-grained unsupported feature breakdown | infra | scripts/coverage | see `issues/done/005-add-fine-grained-unsupported-feature-breakdown.md` |
| 006 | Remove stale milestone and transitional docs (audit reopened #006) | cleanup | docs | see `issues/done/006-remove-stale-milestone-and-transitional-docs.md` |
| 007 | Harden reference coverage prerequisites (audit reopened #007) | infra | scripts/reference | see `issues/done/007-harden-reference-coverage-prerequisites.md` |
| 008 | Introduce typed WAT writer skeleton | refactor | backend | see `issues/done/008-introduce-typed-wat-writer-skeleton.md` |
| 009 | Select first coverage-improvement feature slice (audit reopened #009) | spike | frontend/ir/runtime | see `issues/done/009-select-first-coverage-improvement-feature-slice.md` |
| 010 | Extract frontend module from crates/cli (audit reopened #010) | refactor | frontend | see `issues/done/010-extract-frontend-module-from-crates-cli.md` |
| 011 | Enable `RUSTFLAGS=-D warnings` for nextest / harness (warning-clean tree) | infra | tests | see `issues/done/011-enable-cargo-deny-warnings-in-ci-and-harnesses.md` |
| 012 | Fix computed property semantics bug | bug | runtime/semantics | see `issues/done/012-fix-computed-property-semantics-bug.md` |
| 013 | Implement heap OOM check (audit reopened #013) | feature | runtime/memory | see `issues/done/013-implement-heap-oom-check.md` |
| 014 | Implement dynamic property key support | feature | runtime/semantics | see `issues/done/014-implement-dynamic-property-key-support.md` |
| 015 | Implement object literal string key support (audit reopened #015) | feature | parser/semantics | see `issues/done/015-implement-object-literal-string-key-support.md` |
| 016 | Implement prototype and method call support | feature | runtime/semantics | see `issues/done/016-implement-prototype-and-method-call-support.md` |
| 017 | Design and implement GC strategy #017 | feature | runtime/memory | see `issues/done/017-design-and-implement-gc-strategy.md` |
| 017a | Design GC strategy | feature | runtime/memory | see `issues/done/017a-design-gc-strategy.md` |
| 017b | Implement GC strategy | feature | runtime/memory | see `issues/done/017b-implement-gc-strategy.md` |
| 018 | Implement UTF-8 string support (audit reopened #018) | feature | runtime/semantics | see `issues/done/018-implement-utf-8-string-support.md` |
| 019 | Integrate TypeScript parser/checker (audit reopened #019) | feature | frontend | see `issues/done/019-integrate-typescript-parser-checker.md` |
| 019a | Integrate TypeScript compiler API for type checking (audit reopened #019a) | feature | frontend | see `issues/done/019a-integrate-typescript-compiler-api.md` |
| 019b | Extract type information for optimization hints (audit reopened #019b) | feature | frontend | see `issues/done/019b-extract-type-information-for-optimization.md` |
| 020 | Implement generic JavaScript semantic IR (audit reopened #020) | feature | ir/semantics | see `issues/done/020-implement-generic-javascript-semantic-ir.md` |
| 020a | Design JavaScript semantic IR (audit reopened #020a) | feature | ir/semantics | see `issues/done/020a-design-javascript-semantic-ir.md` |
| 020b | Implement IR lowering from TypeScript AST (audit reopened #020b) | feature | ir/semantics | see `issues/done/020b-implement-ir-lowering-from-typescript-ast.md` |
| 020c | Add IR validation passes and document contracts (audit reopened #020c) | feature | ir/semantics | see `issues/done/020c-add-ir-validation-passes-and-document-contracts.md` |
| 021a | Implement wasm-encoder hello binary MVP | feature | backend | see `issues/done/021a-implement-wasm-encoder-hello-binary-mvp.md` |
| 022 | Expand test262 differential coverage (audit reopened #022) | feature | tests/coverage | see `issues/done/022-expand-test262-differential-coverage.md` |
| 023 | Implement host-deny and auditable E2E manifest | feature | security/capability | see `issues/done/023-implement-host-deny-and-auditable-e2e-manifest.md` |
| 024 | Migrate runtime module to runtime-abi crate | refactor | abi | see `issues/done/024-migrate-runtime-module-to-runtime-abi-crate.md` |
| 025 | Migrate ir module to ir crate | refactor | ir | see `issues/done/025-migrate-ir-module-to-ir-crate.md` |
| 026 | Migrate backend module to backend-wasm crate | refactor | runtime | see `issues/done/026-migrate-backend-module-to-backend-wasm-crate.md` |
| 027 | Migrate frontend code to frontend crate | refactor | frontend | see `issues/done/027-migrate-frontend-code-to-frontend-crate.md` |
| 028 | Migrate lexer/parser/AST to frontend crate | refactor | frontend | see `issues/done/028-migrate-lexer-parser-ast-to-frontend-crate.md` |
| 029 | Implement typeof operator | feature | runtime/semantics | see `issues/done/029-implement-typeof-operator.md` |
| 030 | Implement instanceof operator | feature | runtime/semantics | see `issues/done/030-implement-instanceof-operator.md` |
| 031 | Implement in operator | feature | runtime/semantics | see `issues/done/031-implement-in-operator.md` |
| 032 | Implement delete operator | feature | runtime/semantics | see `issues/done/032-implement-delete-operator.md` |
| 033 | Implement switch statement | feature | frontend/semantics | see `issues/done/033-implement-switch-statement.md` |
| 034 | Implement while and do-while loops | feature | frontend/semantics | see `issues/done/034-implement-while-do-while-loops.md` |
| 035 | Implement break and continue statements | feature | frontend/semantics | see `issues/done/035-implement-break-continue.md` |
| 036 | Implement arrow function | feature | frontend/semantics | see `issues/done/036-implement-arrow-function.md` |
| 037 | Implement this binding | feature | runtime/semantics | see `issues/done/037-implement-this-binding.md` |
| 038 | Implement rest parameters | feature | frontend/semantics | see `issues/done/038-implement-rest-parameters.md` |
| 039 | Implement spread arguments | feature | frontend/semantics | see `issues/done/039-implement-spread-arguments.md` |
| 040 | Implement default parameters | feature | frontend/semantics | see `issues/done/040-implement-default-parameters.md` |
| 041 | Implement template literals (dup) | feature | frontend/semantics | see `issues/done/041-implement-template-literals.md` |
| 042 | Implement string methods | feature | runtime/builtins | see `issues/done/042-implement-string-methods.md` |
| 043 | Implement string indexing | feature | runtime/semantics | see `issues/done/043-implement-string-indexing.md` |
| 044 | Implement String.fromCharCode and charCodeAt | feature | runtime/builtins | see `issues/done/044-implement-string-from-char-code.md` |
| 045 | Implement class declaration and expression | feature | frontend/semantics | see `issues/done/045-implement-class-syntax.md` |
| 046 | Implement extends inheritance | feature | runtime/semantics | see `issues/done/046-implement-extends-inheritance.md` |
| 047 | Implement super keyword (dup) | feature | runtime/semantics | see `issues/done/047-implement-super-keyword.md` |
| 048 | Implement prototype chain | feature | runtime/semantics | see `issues/done/048-implement-prototype-chain.md` |
| 049 | Implement Map and Set | feature | runtime/builtins | see `issues/done/049-implement-map-set.md` |
| 050a | Document Date deterministic subset and live-time policy gap | docs | runtime/builtins | see `issues/done/050a-document-date-deterministic-subset-and-live-time-policy-gap.md` |
| 051 | Implement RegExp | feature | runtime/builtins | see `issues/done/051-implement-regexp.md` |
| 052a | Close JSON supported subset contract | docs | runtime/builtins | see `issues/done/052a-close-json-supported-subset-contract.md` |
| 052b | Implement JSON non-integer number representation | feature | runtime/builtins | see `issues/done/052b-implement-json-noninteger-number-representation.md` |
| 052c | Implement JSON UTF-16 and surrogate string handling | feature | runtime/builtins | see `issues/done/052c-implement-json-utf16-surrogate-strings.md` |
| 052e | Complete JSON.stringify boxed argument edge cases | feature | runtime/builtins | see `issues/done/052e-complete-json-stringify-boxed-argument-edge-cases.md` |
| 052f | Implement JSON.parse throw-compatible diagnostics | feature | runtime/builtins | see `issues/done/052f-implement-json-parse-throw-compatible-diagnostics.md` |
| 052g | Implement JSON.stringify function replacer callbacks | feature | runtime/builtins | see `issues/done/052g-implement-json-stringify-function-replacer-callbacks.md` |
| 053 | Implement Math | feature | runtime/builtins | see `issues/done/053-implement-math.md` |
| 054 | Implement Error types | feature | runtime/builtins | see `issues/done/054-implement-error-types.md` |
| 055 | Umbrella: implement import and export | feature | frontend/semantics | see `issues/done/055-implement-import-export.md` |
| 056 | Implement name resolution for variables and identifiers | feature | frontend | see `issues/done/056-implement-name-resolution.md` |
| 057 | Implement function resolution for function calls | feature | frontend | see `issues/done/057-implement-function-resolution.md` |
| 058 | Implement equality operators (==, !=, ===, !==) | feature | runtime/semantics | see `issues/done/058-implement-equality-operators.md` |
| 059a | Implement TypeScript satisfies and const assertion erasure | feature | frontend | see `issues/done/059a-implement-typescript-satisfies-and-const-assertion-erasure.md` |
| 060 | Investigate and classify unknown-unsupported diagnostic cases (audit reopened #060) | spike | frontend | see `issues/done/060-investigate-unknown-unsupported-cases.md` |
| 060a | Close unknown-unsupported fixed-window spike | spike | frontend | see `issues/done/060a-close-unknown-unsupported-fixed-window-spike.md` |
| 061 | Implement Date object support (dup) | feature | runtime/builtins | see `issues/done/061-implement-date.md` |
| 061a | Merge Date reference issue into Date epic | cleanup | issues | see `issues/done/061a-merge-date-reference-issue-into-date-epic.md` |
| 062 | Implement function support (dup) | feature | frontend/semantics | see `issues/done/062-implement-function.md` |
| 062a | Split function epic into callable child issues | cleanup | issues | see `issues/done/062a-split-function-epic-into-callable-child-issues.md` |
| 062b | Own dynamic Function constructor diagnostics | feature | frontend/semantics | see `issues/done/062b-dynamic-function-constructor-diagnostics.md` |
| 062c | Implement ordinary function declarations and direct calls | feature | frontend/semantics | see `issues/done/062c-ordinary-function-declarations-and-calls.md` |
| 062d | Implement function this and arguments semantics | feature | frontend/semantics | see `issues/done/062d-function-this-and-arguments.md` |
| 062e | Implement function closures | feature | frontend/semantics | see `issues/done/062e-function-closures.md` |
| 062f | Implement function object metadata | feature | frontend/semantics | see `issues/done/062f-function-object-metadata.md` |
| 062g | Define and implement heap closure object ABI and rooting (audit reopened #062g) | feature | runtime/abi | see `issues/done/062g-heap-closure-object-abi-and-rooting.md` |
| 063 | Implement function resolution (dup) | feature | frontend/resolver | see `issues/done/063-implement-function-resolution.md` |
| 064a | Resolve Date global builtin namespace | feature | frontend | see `issues/done/064a-resolve-date-global-builtin-namespace.md` |
| 065 | Implement parser syntax extensions (dup) | feature | frontend/syntax | see `issues/done/065-implement-parser-syntax.md` |
| 065a | Merge duplicate parser syntax issue into 059 | cleanup | issues | see `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` |
| 066 | Implement RegExp literal support (dup) | spike | runtime/builtins | see `issues/done/066-implement-regexp-literal.md` |
| 067 | Investigate and classify unknown-unsupported cases (dup) | spike | reference/triage | see `issues/done/067-implement-unknown-unsupported.md` |
| 068 | Implement unsupported expression types | spike | frontend/semantics | see `issues/done/068-implement-unsupported-expression.md` |
| 069 | Implement Apilibcheck (dup) | spike | runtime/builtins | see `issues/done/069-implement-APILibCheck.md` |
| 070 | Implement Apisample | spike | runtime/builtins | see `issues/done/070-implement-APISample.md` |
| 071 | Implement Arrowfunctionexpression | spike | frontend/syntax | see `issues/done/071-implement-ArrowFunctionExpression.md` |
| 072 | Implement Classdeclaration (dup) | spike | frontend/syntax | see `issues/done/072-implement-ClassDeclaration.md` |
| 073 | Implement Classdeclarationwithinvalidconstonpropertydeclaration | spike | frontend/syntax | see `issues/done/073-implement-ClassDeclarationWithInvalidConstOnPropertyDeclaration.md` |
| 074 | Implement Declarationerrorsnoemitonerror | spike | frontend/syntax | see `issues/done/074-implement-DeclarationErrorsNoEmitOnError.md` |
| 075 | Implement Exportassignment (dup) | spike | frontend/syntax | see `issues/done/075-implement-ExportAssignment.md` |
| 076 | Implement Functiondeclaration | spike | frontend/syntax | see `issues/done/076-implement-FunctionDeclaration.md` |
| 077 | Implement Interfacedeclaration | spike | frontend/syntax | see `issues/done/077-implement-InterfaceDeclaration.md` |
| 078 | Implement Memberaccessordeclaration | spike | frontend/syntax | see `issues/done/078-implement-MemberAccessorDeclaration.md` |
| 079 | Implement Parameterlist | spike | frontend/syntax | see `issues/done/079-implement-ParameterList.md` |
| 080 | Implement Systemmoduleforstatementnoinitializer | spike | frontend/syntax | see `issues/done/080-implement-SystemModuleForStatementNoInitializer.md` |
| 081 | Implement Transportstream (dup) | spike | reference/triage | see `issues/done/081-implement-TransportStream.md` |
| 082 | Implement Abstractclassinlocalscope (dup) | spike | frontend/syntax | see `issues/done/082-implement-abstractClassInLocalScope.md` |
| 083 | Implement Abstractclassinlocalscopeisabstract (dup) | spike | frontend/syntax | see `issues/done/083-implement-abstractClassInLocalScopeIsAbstract.md` |
| 084 | Implement Abstractclassunioninstantiation (dup) | spike | frontend/syntax | see `issues/done/084-implement-abstractClassUnionInstantiation.md` |
| 085 | Implement Abstractinterfaceidentifiername | spike | frontend/syntax | see `issues/done/085-implement-abstractInterfaceIdentifierName.md` |
| 086 | Implement Abstractpropertybasics (dup) | spike | frontend/syntax | see `issues/done/086-implement-abstractPropertyBasics.md` |
| 087 | Implement Abstractpropertyinconstructor (dup) | spike | frontend/syntax | see `issues/done/087-implement-abstractPropertyInConstructor.md` |
| 088 | Implement Abstractpropertynegative (dup) | spike | frontend/syntax | see `issues/done/088-implement-abstractPropertyNegative.md` |
| 089 | Implement Acceptsymbolasweaktype (dup) | spike | frontend/resolver | see `issues/done/089-implement-acceptSymbolAsWeakType.md` |
| 090 | Implement Acceptablealias (dup) | spike | frontend/syntax | see `issues/done/090-implement-acceptableAlias.md` |
| 091 | Implement Accessinstancememberfromstaticmethod (dup) | spike | frontend/syntax | see `issues/done/091-implement-accessInstanceMemberFromStaticMethod.md` |
| 092 | Implement Accessoverriddenbaseclassmember | spike | frontend/semantics | see `issues/done/092-implement-accessOverriddenBaseClassMember.md` |
| 093 | Implement Accessstaticmemberfrominstancemethod (dup) | spike | frontend/syntax | see `issues/done/093-implement-accessStaticMemberFromInstanceMethod.md` |
| 094 | Implement Accessoraccidentalcalldiagnostic (dup) | spike | frontend/resolver | see `issues/done/094-implement-accessorAccidentalCallDiagnostic.md` |
| 095 | Implement Accessorbodyintypecontext | spike | frontend/syntax | see `issues/done/095-implement-accessorBodyInTypeContext.md` |
| 096 | Implement Accessordeclarationemitjs (dup) | spike | frontend/syntax | see `issues/done/096-implement-accessorDeclarationEmitJs.md` |
| 097 | Implement Accessordeclarationemitvisibilityerrors (dup) | spike | frontend/syntax | see `issues/done/097-implement-accessorDeclarationEmitVisibilityErrors.md` |
| 098 | Implement Accessordeclarationorder | spike | frontend/syntax | see `issues/done/098-implement-accessorDeclarationOrder.md` |
| 099 | Implement Accessorinambientcontextes | spike | frontend/syntax | see `issues/done/099-implement-accessorInAmbientContextES.md` |
| 100 | Implement Accessorinferredreturntypeerrorinreturnstatement (dup) | spike | frontend/syntax | see `issues/done/100-implement-accessorInferredReturnTypeErrorInReturnStatement.md` |
| 101 | Implement Accessorparameteraccessibilitymodifier | spike | frontend/syntax | see `issues/done/101-implement-accessorParameterAccessibilityModifier.md` |
| 102 | Implement Accessorwithinitializer | spike | frontend/syntax | see `issues/done/102-implement-accessorWithInitializer.md` |
| 103 | Implement Accessorwithlineterminator (dup) | spike | frontend/syntax | see `issues/done/103-implement-accessorWithLineTerminator.md` |
| 104 | Implement Accessorwithrestparam | spike | frontend/syntax | see `issues/done/104-implement-accessorWithRestParam.md` |
| 105 | Implement Accessorwithoutbody | spike | frontend/syntax | see `issues/done/105-implement-accessorWithoutBody.md` |
| 106 | Implement Accessors (dup) | spike | frontend/syntax | see `issues/done/106-implement-accessors.md` |
| 107 | Implement Accessorsemit | spike | frontend/syntax | see `issues/done/107-implement-accessorsEmit.md` |
| 108 | Implement Accessorsinambientcontext | spike | frontend/syntax | see `issues/done/108-implement-accessorsInAmbientContext.md` |
| 109 | Implement Addmorecallsignaturestobasesignature (dup) | spike | frontend/resolver | see `issues/done/109-implement-addMoreCallSignaturesToBaseSignature.md` |
| 110 | Implement Addmoreoverloadstobasesignature | spike | frontend/syntax | see `issues/done/110-implement-addMoreOverloadsToBaseSignature.md` |
| 111 | Implement Aliasassignments (dup) | spike | frontend/syntax | see `issues/done/111-implement-aliasAssignments.md` |
| 112 | Implement Aliasbug (dup) | spike | frontend/syntax | see `issues/done/112-implement-aliasBug.md` |
| 113 | Implement Aliasdoesnotduplicatesignatures (dup) | spike | frontend/syntax | see `issues/done/113-implement-aliasDoesNotDuplicateSignatures.md` |
| 114 | Implement Aliaserrors (dup) | spike | frontend/syntax | see `issues/done/114-implement-aliasErrors.md` |
| 115 | Implement Aliasinaccessiblemodule | spike | frontend/syntax | see `issues/done/115-implement-aliasInaccessibleModule.md` |
| 116 | Implement Aliasinstantiationexpressiongenericintersectionnocrash (dup) | spike | frontend/syntax | see `issues/done/116-implement-aliasInstantiationExpressionGenericIntersectionNoCrash.md` |
| 117 | Implement Aliasofgenericfunctionwithrestbehavedsameasunaliased | spike | frontend/syntax | see `issues/done/117-implement-aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.md` |
| 118 | Implement Aliasonmergedmoduleinterface (dup) | spike | frontend/syntax | see `issues/done/118-implement-aliasOnMergedModuleInterface.md` |
| 119 | Implement Aliasusageinaccessorsofclass (dup) | spike | frontend/syntax | see `issues/done/119-implement-aliasUsageInAccessorsOfClass.md` |
| 120 | Implement Aliasusageinarray (dup) | spike | frontend/syntax | see `issues/done/120-implement-aliasUsageInArray.md` |
| 121 | Implement Aliasusageinfunctionexpression (dup) | spike | frontend/syntax | see `issues/done/121-implement-aliasUsageInFunctionExpression.md` |
| 122 | Implement Aliasusageingenericfunction (dup) | spike | frontend/syntax | see `issues/done/122-implement-aliasUsageInGenericFunction.md` |
| 123 | Implement Aliasusageinindexerofclass (dup) | spike | frontend/syntax | see `issues/done/123-implement-aliasUsageInIndexerOfClass.md` |
| 124 | Implement Aliasusageinobjectliteral (dup) | spike | frontend/syntax | see `issues/done/124-implement-aliasUsageInObjectLiteral.md` |
| 125 | Implement Aliasusageinorexpression (dup) | spike | frontend/syntax | see `issues/done/125-implement-aliasUsageInOrExpression.md` |
| 126 | Implement Aliasusageintypeargumentofextendsclause (dup) | spike | frontend/syntax | see `issues/done/126-implement-aliasUsageInTypeArgumentOfExtendsClause.md` |
| 127 | Implement Aliasusageinvarassignment (dup) | spike | frontend/syntax | see `issues/done/127-implement-aliasUsageInVarAssignment.md` |
| 128 | Implement Aliasusedasnamevalue (dup) | spike | frontend/syntax | see `issues/done/128-implement-aliasUsedAsNameValue.md` |
| 129 | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer (dup) | spike | frontend/syntax | see `issues/done/129-implement-aliasWithInterfaceExportAssignmentUsedInVarInitializer.md` |
| 130 | Implement Aliasesinsystemmodule (dup) | spike | frontend/syntax | see `issues/done/130-implement-aliasesInSystemModule.md` |
| 131 | Implement Allowimportclausestomergewithtypes (dup) | spike | frontend/syntax | see `issues/done/131-implement-allowImportClausesToMergeWithTypes.md` |
| 132 | Implement Allowjsclassthistypecrash (dup) | spike | runtime/builtins | see `issues/done/132-implement-allowJsClassThisTypeCrash.md` |
| 133 | Implement Allowjscrossmonorepopackage (dup) | spike | frontend/syntax | see `issues/done/133-implement-allowJsCrossMonorepoPackage.md` |
| 134 | Implement Allowjscheckjstypeparameternocrash (dup) | spike | frontend/syntax | see `issues/done/134-implement-allowJscheckJsTypeParameterNoCrash.md` |
| 135 | Implement Allowsyntheticdefaultimports (dup) | spike | frontend/syntax | see `issues/done/135-implement-allowSyntheticDefaultImports.md` |
| 136 | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration (dup) | spike | frontend/syntax | see `issues/done/136-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md` |
| 137 | Implement Alwaysstrictalreadyusestrict | spike | frontend/syntax | see `issues/done/137-implement-alwaysStrictAlreadyUseStrict.md` |
| 138 | Implement Alwaysstrictmodule | spike | frontend/syntax | see `issues/done/138-implement-alwaysStrictModule.md` |
| 139 | Implement Alwaysstrictnoimplicitusestrict | spike | frontend/syntax | see `issues/done/139-implement-alwaysStrictNoImplicitUseStrict.md` |
| 140 | Implement Ambientclassdeclarationwithextends | spike | frontend/syntax | see `issues/done/140-implement-ambientClassDeclarationWithExtends.md` |
| 141 | Implement Ambientclassdeclaredbeforebase (dup) | spike | frontend/syntax | see `issues/done/141-implement-ambientClassDeclaredBeforeBase.md` |
| 142 | Implement Ambientclassmergesoverloadswithinterface | spike | frontend/syntax | see `issues/done/142-implement-ambientClassMergesOverloadsWithInterface.md` |
| 143 | Implement Ambientclassoverloadforfunction | spike | frontend/syntax | see `issues/done/143-implement-ambientClassOverloadForFunction.md` |
| 144 | Implement Ambientconstliterals (dup) | spike | frontend/syntax | see `issues/done/144-implement-ambientConstLiterals.md` |
| 145 | Implement Ambientenum | spike | frontend/syntax | see `issues/done/145-implement-ambientEnum.md` |
| 146 | Implement Ambientenumelementinitializer (dup) | spike | frontend/syntax | see `issues/done/146-implement-ambientEnumElementInitializer.md` |
| 147 | Implement Ambienterrors | spike | frontend/syntax | see `issues/done/147-implement-ambientErrors.md` |
| 148 | Implement Ambientexportdefaulterrors (dup) | spike | frontend/syntax | see `issues/done/148-implement-ambientExportDefaultErrors.md` |
| 149 | Implement Ambientexternalmoduleinanotherexternalmodule (dup) | spike | frontend/syntax | see `issues/done/149-implement-ambientExternalModuleInAnotherExternalModule.md` |
| 150 | Implement Ambientexternalmodulereopen | spike | frontend/syntax | see `issues/done/150-implement-ambientExternalModuleReopen.md` |
| 151 | Implement Ambientexternalmodulewithinternalimportdeclaration (dup) | spike | frontend/syntax | see `issues/done/151-implement-ambientExternalModuleWithInternalImportDeclaration.md` |
| 152 | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration | spike | frontend/syntax | see `issues/done/152-implement-ambientExternalModuleWithRelativeExternalImportDeclaration.md` |
| 153 | Implement Ambientexternalmodulewithrelativemodulename | spike | frontend/syntax | see `issues/done/153-implement-ambientExternalModuleWithRelativeModuleName.md` |
| 154 | Implement Ambientexternalmodulewithoutinternalimportdeclaration (dup) | spike | frontend/syntax | see `issues/done/154-implement-ambientExternalModuleWithoutInternalImportDeclaration.md` |
| 155 | Implement Ambientfundule | spike | frontend/syntax | see `issues/done/155-implement-ambientFundule.md` |
| 156 | Implement Ambientgetters | spike | frontend/syntax | see `issues/done/156-implement-ambientGetters.md` |
| 157 | Implement Ambientmoduleexports (dup) | spike | frontend/syntax | see `issues/done/157-implement-ambientModuleExports.md` |
| 158 | Implement Ambientmodulewithclassdeclarationwithextends | spike | frontend/syntax | see `issues/done/158-implement-ambientModuleWithClassDeclarationWithExtends.md` |
| 159 | Implement Ambientmodulewithtemplateliterals (dup) | spike | frontend/syntax | see `issues/done/159-implement-ambientModuleWithTemplateLiterals.md` |
| 160 | Implement Ambientmodules (dup) | spike | frontend/syntax | see `issues/done/160-implement-ambientModules.md` |
| 161 | Implement Ambientnamerestrictions | spike | frontend/syntax | see `issues/done/161-implement-ambientNameRestrictions.md` |
| 162 | Implement Ambientpropertydeclarationinjs | spike | frontend/syntax | see `issues/done/162-implement-ambientPropertyDeclarationInJs.md` |
| 163 | Implement Ambientrequirefunction (dup) | spike | frontend/syntax | see `issues/done/163-implement-ambientRequireFunction.md` |
| 164 | Implement Ambientstatement | spike | frontend/syntax | see `issues/done/164-implement-ambientStatement.md` |
| 165 | Implement Ambientwithstatements | spike | frontend/syntax | see `issues/done/165-implement-ambientWithStatements.md` |
| 166 | Implement Ambiguouscallswherereturntypesagree (dup) | spike | frontend/syntax | see `issues/done/166-implement-ambiguousCallsWhereReturnTypesAgree.md` |
| 167 | Implement Ambiguousgenericassertion (dup) | spike | frontend/syntax | see `issues/done/167-implement-ambiguousGenericAssertion.md` |
| 169 | Implement Ambiguousoverloadresolution (dup) | spike | frontend/syntax | see `issues/done/169-implement-ambiguousOverloadResolution.md` |
| 170 | Implement Amddeclarationemitnoextradeclare (dup) | spike | frontend/syntax | see `issues/done/170-implement-amdDeclarationEmitNoExtraDeclare.md` |
| 171 | Implement Amddependencycomment (dup) | spike | frontend/syntax | see `issues/done/171-implement-amdDependencyComment.md` |
| 172 | Implement Amddependencycommentname (dup) | spike | frontend/syntax | see `issues/done/172-implement-amdDependencyCommentName.md` |
| 173 | Implement Amdlikeinputdeclarationemit (dup) | spike | frontend/syntax | see `issues/done/173-implement-amdLikeInputDeclarationEmit.md` |
| 174 | Implement Amdmodulebundlenoduplicatedeclarationemitcomments (dup) | spike | frontend/syntax | see `issues/done/174-implement-amdModuleBundleNoDuplicateDeclarationEmitComments.md` |
| 175 | Implement Amdmoduleconstenumusage (dup) | spike | frontend/syntax | see `issues/done/175-implement-amdModuleConstEnumUsage.md` |
| 176 | Implement Amdmodulename (dup) | spike | frontend/syntax | see `issues/done/176-implement-amdModuleName.md` |
| 177 | Implement Anonclassdeclarationemitisanon (dup) | spike | frontend/syntax | see `issues/done/177-implement-anonClassDeclarationEmitIsAnon.md` |
| 178 | Implement Anonterface (dup) | spike | frontend/syntax | see `issues/done/178-implement-anonterface.md` |
| 179 | Implement Anonymousclassdeclarationdoesntprintwithreadonly (dup) | spike | frontend/syntax | see `issues/done/179-implement-anonymousClassDeclarationDoesntPrintWithReadonly.md` |
| 180 | Implement Anonymousclassexpression | spike | frontend/syntax | see `issues/done/180-implement-anonymousClassExpression.md` |
| 181 | Implement Anonymousmodules | spike | frontend/syntax | see `issues/done/181-implement-anonymousModules.md` |
| 182 | Implement Anyandunknownhavefalsycomponents (dup) | spike | frontend/syntax | see `issues/done/182-implement-anyAndUnknownHaveFalsyComponents.md` |
| 183 | Implement Anyasreturntypefornewoncall | spike | frontend/syntax | see `issues/done/183-implement-anyAsReturnTypeForNewOnCall.md` |
| 184 | Implement Anydeclare | spike | frontend/syntax | see `issues/done/184-implement-anyDeclare.md` |
| 185 | Implement Anyidenticaltoitself (dup) | spike | frontend/syntax | see `issues/done/185-implement-anyIdenticalToItself.md` |
| 186 | Implement Anyindexedaccessarraynoexception | spike | frontend/syntax | see `issues/done/186-implement-anyIndexedAccessArrayNoException.md` |
| 187 | Implement Anyinferenceanonymousfunctions (dup) | spike | frontend/syntax | see `issues/done/187-implement-anyInferenceAnonymousFunctions.md` |
| 188 | Implement Anyisassignabletoobject | spike | frontend/syntax | see `issues/done/188-implement-anyIsAssignableToObject.md` |
| 189 | Implement Anyisassignabletovoid | spike | frontend/syntax | see `issues/done/189-implement-anyIsAssignableToVoid.md` |
| 190 | Implement Anymappedtypeserror | spike | frontend/syntax | see `issues/done/190-implement-anyMappedTypesError.md` |
| 191 | Implement Anyplusany | spike | frontend/syntax | see `issues/done/191-implement-anyPlusAny.md` |
| 192 | Implement Argsinscope (dup) | spike | frontend/syntax | see `issues/done/192-implement-argsInScope.md` |
| 193 | Implement Arguments (dup) | spike | frontend/resolver | see `issues/done/193-implement-arguments.md` |
| 195 | Implement Argumentsbindstofunctionscopeargumentlist (dup) | spike | frontend/resolver | see `issues/done/195-implement-argumentsBindsToFunctionScopeArgumentList.md` |
| 196 | Implement Argumentsobjectcreatesrestforjs (dup) | spike | frontend/resolver | see `issues/done/196-implement-argumentsObjectCreatesRestForJs.md` |
| 197 | Implement Argumentsobjectiterator (dup) | spike | frontend/semantics | see `issues/done/197-implement-argumentsObjectIterator.md` |
| 198 | Implement Argumentspropertynameinjsmode (dup) | spike | frontend/semantics | see `issues/done/198-implement-argumentsPropertyNameInJsMode.md` |
| 199 | Implement Compiler | spike | frontend/syntax | see `issues/done/199-implement-reference-typescript-tests-cases-compiler.md` |
| 200 | Implement parser syntax extensions (dup) | spike | frontend/syntax | see `issues/done/200-implement-parser-syntax.md` |
| 201 | Investigate and classify unknown-unsupported cases (dup) | spike | reference/triage | see `issues/done/201-implement-unknown-unsupported.md` |
| 202 | Implement RegExp literal support | feature | frontend/semantics | see `issues/done/202-implement-regexp-literal-support.md` |
| 203 | Reconcile partial feature semantics and placeholder completions | cleanup | docs/issues | see `issues/done/203-reconcile-partial-feature-semantics.md` |
| 204 | Add typed IR dump command | feature | cli | see `issues/done/204-add-typed-ir-dump.md` |
| 205 | Add optimizer dump command | feature | cli | see `issues/done/205-add-optimizer-dump.md` |
| 206 | Make CLI a thin toolchain wrapper | refactor | cli | see `issues/done/206-make-cli-a-thin-toolchain-wrapper.md` |
| 207 | Complete instanceof prototype-chain semantics | feature | runtime/semantics | see `issues/done/207-complete-instanceof-prototype-chain-semantics.md` |
| 208 | Implement switch fall-through semantics | feature | frontend/semantics | see `issues/done/208-implement-switch-fall-through-semantics.md` |
| 209 | Implement labeled break and continue | feature | frontend/semantics | see `issues/done/209-implement-labeled-break-continue.md` |
| 210 | Implement arrow function closure and lexical this semantics | feature | frontend/semantics | see `issues/done/210-implement-arrow-function-closure-lexical-this.md` |
| 211 | Complete this receiver binding semantics | feature | runtime/semantics | see `issues/done/211-complete-this-receiver-binding-semantics.md` |
| 212 | Implement rest parameter argument collection | feature | frontend/semantics | see `issues/done/212-implement-rest-parameter-argument-collection.md` |
| 213 | Implement template literal interpolation | feature | frontend/semantics | see `issues/done/213-implement-template-literal-interpolation.md` |
| 214 | Replace string method placeholders | feature | runtime/builtins | see `issues/done/214-replace-string-method-placeholders.md` |
| 215 | Define Math.random capability policy | feature | runtime/builtins | see `issues/done/215-define-math-random-capability-policy.md` |
| 216 | Implement abstract equality coercion | feature | runtime/semantics | see `issues/done/216-implement-abstract-equality-coercion.md` |
| 217 | Implement GC heap header and trigger accounting (audit reopened #217) | feature | runtime/memory | see `issues/done/217-implement-gc-heap-header-and-trigger-accounting.md` |
| 218 | Implement GC mark root scanning (audit reopened #218) | feature | runtime/memory | see `issues/done/218-implement-gc-mark-root-scanning.md` |
| 219 | Implement GC sweep reuse and fixtures (audit reopened #219) | feature | runtime/memory | see `issues/done/219-implement-gc-sweep-reuse-and-fixtures.md` |
| 220 | Implement GC top-level local roots for object escape fixtures (audit reopened #220) | feature | runtime/memory | see `issues/done/220-implement-gc-top-level-local-roots-for-object-escape-fixtures.md` |
| 221 | Implement GC call-frame roots for closure escape (audit reopened #221) | feature | runtime/memory | see `issues/done/221-implement-gc-call-frame-roots-for-closure-escape.md` |
| 222 | Investigate GC high-pressure OOB under repeated local-root allocation (audit reopened #222) | bug | runtime/memory | see `issues/done/222-investigate-gc-high-pressure-oob.md` |
| 223 | Add spans to receiver this diagnostics (audit reopened #223) | bug | frontend/diagnostics | see `issues/done/223-add-spans-to-receiver-this-diagnostics.md` |
| 224 | Implement Annex B HTML-like comments | feature | frontend | see `issues/done/224-implement-annexb-html-comments.md` |
| 225 | Implement eval and Annex B function declaration semantics | meta | frontend/semantics | see `issues/done/225-implement-eval-annexb-function-declarations.md` |
| 226 | Implement TypeScript parameter properties | feature | frontend/semantics | see `issues/done/226-implement-parameter-properties.md` |
| 227 | Implement type reference directive resolution | feature | frontend/semantics | see `issues/done/227-implement-type-reference-directive-resolution.md` |
| 228 | Implement logical assignment operators | feature | frontend/semantics | see `issues/done/228-implement-logical-assignment-operators.md` |
| 229 | Implement legacy octal escape handling | feature | frontend | see `issues/done/229-implement-legacy-octal-escape-handling.md` |
| 230 | Implement async iteration and for-await-of | feature | frontend/semantics | see `issues/done/230-implement-async-iteration-for-await-of.md` |
| 231 | Parse static ES module declarations | feature | frontend | see `issues/done/231-parse-static-es-module-declarations.md` |
| 232 | Resolve local relative ES module graph | feature | compiler/frontend | see `issues/done/232-resolve-local-relative-es-module-graph.md` |
| 233 | Emit static ES module bindings | feature | ir/backend | see `issues/done/233-emit-static-es-module-bindings.md` |
| 234 | Cover static ES module execution | test | tests/fixtures | see `issues/done/234-cover-static-es-module-execution.md` |
| 235 | Fix GC root count backend tests | bug | backend/memory | see `issues/done/235-fix-gc-root-count-tests.md` |
| 236 | Complete logical assignment target forms | feature | frontend/semantics | see `issues/done/236-complete-logical-assignment-target-forms.md` |
| 237 | Implement Annex B IsHTMLDDA compatibility | feature | runtime/semantics | see `issues/done/237-implement-annexb-ishtmldda-compatibility.md` |
| 238 | Make strict warning gates pass | infra | tests | see `issues/done/238-make-strict-warning-gates-pass.md` |
| 239 | Design Date live-time capability policy | docs | runtime/builtins | see `issues/done/239-design-date-live-time-capability-policy.md` |
| 240 | Implement Date timezone-aware toString policy | feature | runtime/builtins | see `issues/done/240-implement-date-timezone-aware-to-string-policy.md` |
| 241 | Implement Annex B Date legacy methods | feature | runtime/builtins | see `issues/done/241-implement-annex-b-date-legacy-methods.md` |
| 242 | Implement Date live time with WASI realtime clock | feature | runtime/builtins | see `issues/done/242-implement-date-live-time-wasi-clock.md` |
| 243 | Implement numeric literal separator parser support | feature | frontend/syntax | see `issues/done/243-implement-numeric-literal-separator-parser.md` |
| 244 | Implement BigInt literal parser classification | feature | frontend/syntax | see `issues/done/244-implement-bigint-literal-parser-classification.md` |
| 245 | Implement nullish coalescing frontend support | feature | frontend/semantics | see `issues/done/245-implement-nullish-coalescing-frontend.md` |
| 246 | Implement optional chaining parser support | feature | frontend/syntax | see `issues/done/246-implement-optional-chaining-parser-support.md` |
| 247 | Implement destructuring binding pattern parser support | feature | frontend/syntax | see `issues/done/247-implement-destructuring-binding-pattern-parser.md` |
| 248 | Implement private class element parser support | feature | frontend/syntax | see `issues/done/248-implement-private-class-element-parser.md` |
| 249 | Implement class static block parser support | feature | frontend/syntax | see `issues/done/249-implement-class-static-block-parser.md` |
| 250 | Design BigInt runtime value support | feature | runtime/semantics | see `issues/done/250-design-bigint-runtime-value-support.md` |
| 251 | Implement destructuring binding runtime semantics | feature | frontend/semantics | see `issues/done/251-implement-destructuring-binding-runtime-semantics.md` |
| 252 | Implement destructuring assignment pattern parser support | feature | frontend/syntax | see `issues/done/252-implement-destructuring-assignment-pattern-parser.md` |
| 253 | Implement optional chaining runtime semantics (audit reopened #253) | feature | frontend/semantics | see `issues/done/253-implement-optional-chaining-runtime-semantics.md` |
| 254 | Implement class static block runtime semantics | feature | runtime/semantics | see `issues/done/254-implement-class-static-block-runtime-semantics.md` |
| 255 | Implement private class element runtime semantics (audit reopened #255) | meta | runtime/semantics | see `issues/done/255-implement-private-class-element-runtime-semantics.md` |
| 256 | Lower returned immutable closures to heap closure values | feature | ir | see `issues/done/256-lower-returned-immutable-closures-to-heap-values.md` |
| 257 | Emit heap closure allocation and dispatch | feature | backend | see `issues/done/257-emit-heap-closure-allocation-and-dispatch.md` |
| 258 | Mark heap closure captures and add allocation-pressure fixture | feature | runtime | see `issues/done/258-mark-heap-closure-captures-and-add-allocation-pressure-fixture.md` |
| 259 | Implement BigInt literal runtime values | feature | runtime/semantics | see `issues/done/259-implement-bigint-literal-runtime-values.md` |
| 260 | Implement BigInt arithmetic operators | feature | runtime/semantics | see `issues/done/260-implement-bigint-arithmetic-operators.md` |
| 261 | Implement BigInt equality comparison and coercion boundaries | feature | runtime/semantics | see `issues/done/261-implement-bigint-equality-comparison-coercion.md` |
| 262 | Implement BigInt builtins and string conversion | feature | runtime/builtins | see `issues/done/262-implement-bigint-builtins-and-string-conversion.md` |
| 263 | Implement BigInt dynamic mul/div/rem signed-i64 runtime slice | feature | runtime/semantics | see `issues/done/263-implement-bigint-dynamic-mul-div-rem-signed-i64-slice.md` |
| 264 | Add broad expression fixture coverage (audit reopened #264) | feature | frontend/syntax | see `issues/done/264-implement-broad-expression-fixture-coverage.md` |
| 265 | Add broad statement fixture coverage (audit reopened #265) | feature | frontend/syntax | see `issues/done/265-implement-broad-statement-fixture-coverage.md` |
| 266 | Implement test262 test harness and host-defined functions (audit reopened #266) | feature | tests/harness | see `issues/done/266-implement-test262-harness.md` |
| 267 | Implement interactive web UI for test results | feature | coverage | see `issues/done/267-implement-interactive-web-ui-for-test-results.md` |
| 267a | Implement web UI data generation and script integration | feature | coverage | see `issues/done/267a-web-ui-data-generation-and-script-integration.md` |
| 267b | Implement web UI interactive charts, regression detection, and performance trends | feature | coverage | see `issues/done/267b-web-ui-interactive-charts-regression-and-performance-trends.md` |
| 267c | Implement web UI real-time test run updates | feature | coverage | see `issues/done/267c-web-ui-real-time-test-run-updates.md` |
| 267d | Implement web UI export controls, theme toggle, and usage docs | feature | coverage | see `issues/done/267d-web-ui-export-theme-and-usage-docs.md` |
| 268 | Implement for loop increment operator (audit reopened #268) | feature | frontend/semantics | see `issues/done/268-implement-for-loop-increment-operator.md` |
| 269 | Implement integer-only Math.pow slice | feature | runtime/builtins | see `issues/done/269-implement-math-pow.md` |
| 270 | Implement Array.prototype.map named-callback slice | feature | runtime/builtins | see `issues/done/270-implement-array-prototype-map.md` |
| 271 | Implement Array.prototype.push | feature | runtime/builtins | see `issues/done/271-implement-array-prototype-push.md` |
| 272 | Implement Set | feature | runtime/builtins | see `issues/done/272-implement-set.md` |
| 273 | Implement recursive function calls | feature | runtime/semantics | see `issues/done/273-implement-recursive-function-calls.md` |
| 275 | Implement Set size and clear | feature | runtime/builtins | see `issues/done/275-implement-set-size-clear.md` |
| 276 | Implement Set constructor from supported iterables | feature | runtime/builtins | see `issues/done/276-implement-set-constructor-from-supported-iterables.md` |
| 277 | Implement Set SameValueZero identity | feature | runtime/builtins | see `issues/done/277-implement-set-samevaluezero-identity.md` |
| 278 | Implement Set iteration | feature | runtime/builtins | see `issues/done/278-implement-set-iteration.md` |
| 279 | Implement observable Set constructor add dispatch | feature | runtime/builtins | see `issues/done/279-implement-observable-set-constructor-add-dispatch.md` |
| 280 | Implement dynamic BigInt builtin inputs | feature | runtime/builtins | see `issues/done/280-implement-dynamic-bigint-builtin-inputs.md` |
| 281 | Implement BigInt/Number edge equality and comparison | feature | runtime/semantics | see `issues/done/281-implement-bigint-number-edge-equality-comparison.md` |
| 282 | Implement dynamic BigInt mixed coercion | feature | runtime/semantics | see `issues/done/282-implement-dynamic-bigint-mixed-coercion.md` |
| 283 | Implement assigned arrow recursion | feature | runtime/semantics | see `issues/done/283-implement-assigned-arrow-recursion.md` |
| 284 | Support test262 async flag in reference coverage | test | reference/tests | see `issues/done/284-support-test262-async-flag-runner-coverage.md` |
| 285 | Support test262 module flag in reference coverage | test | reference/tests | see `issues/done/285-support-test262-module-flag-runner-coverage.md` |
| 286 | Classify expected negative SyntaxError tests correctly | bug | reference/tests | see `issues/done/286-classify-negative-syntax-tests-correctly.md` |
| 287 | Fix arguments-object arity mismatch bucket | bug | runtime/semantics | see `issues/done/287-fix-arguments-object-arity-mismatch.md` |
| 288 | Provide test262 assert harness binding | feature | reference/runtime | see `issues/done/288-provide-test262-assert-harness-binding.md` |
| 289 | Resolve callCount binding in class destructuring tests | bug | frontend/ir | see `issues/done/289-resolve-callcount-binding-in-class-destructuring.md` |
| 290 | Fix ASI EOF semicolon parser bucket | bug | frontend | see `issues/done/290-fix-asi-eof-semicolon-parser-bucket.md` |
| 291 | Provide Object global binding for test262 cases | feature | runtime/semantics | see `issues/done/291-provide-object-global-binding-for-test262.md` |
| 292 | Resolve initCount binding in class destructuring defaults | bug | frontend/ir | see `issues/done/292-resolve-initcount-binding-in-class-destructuring.md` |
| 293 | Parse Unicode escaped identifier parts | feature | frontend | see `issues/done/293-parse-unicode-escaped-identifier-parts.md` |
| 295 | Support Array.map arrow callbacks and chained receivers | feature | runtime/builtins | see `issues/done/295-support-array-map-arrow-and-chained-receivers.md` |
| 296 | Support small-int exponentiation operator | feature | runtime/semantics | see `issues/done/296-support-small-int-exponentiation-operator.md` |
| 297 | Track pushed dense array locals for map callbacks | feature | frontend/runtime | see `issues/done/297-track-pushed-dense-array-locals-for-map.md` |
| 298 | Allow reused for-loop local names in separate loop scopes | bug | frontend/ir | see `issues/done/298-allow-reused-for-loop-local-names.md` |
| 299 | Support Array.sort numeric comparator slice | feature | runtime/builtins | see `issues/done/299-support-array-sort-numeric-comparator.md` |
| 301 | Implement mutable class-method outer environment cells | feature | frontend/ir/runtime | see `issues/done/301-implement-mutable-class-method-outer-environment-cells.md` |
| 302 | Implement direct eval block function declaration slice | feature | frontend/semantics | see `issues/done/302-implement-direct-eval-block-function-declaration-slice.md` |
| 303 | Define runtime memory limit policy for large live sets | feature | runtime/memory | see `issues/done/303-define-runtime-memory-limit-policy-for-large-live-sets.md` |
| 304 | Support ABC451 depth-8 live-set after memory policy | feature | runtime/memory | see `issues/done/304-support-abc451-depth8-live-set-after-memory-policy.md` |
| 305 | Support ABC451 depth-9 search budget | feature | runtime/memory | see `issues/done/305-support-abc451-depth9-search-budget.md` |
| 306 | Implement mutable direct eval block-function environments | feature | frontend/ir/runtime | see `issues/done/306-implement-mutable-direct-eval-block-function-environments.md` |
| 307 | Investigate ABC451 depth-9 allocation performance | spike | runtime/performance | see `issues/done/307-investigate-abc451-depth9-allocation-performance.md` |
| 310 | Fix activation-frame root liveness depth-8 regression | feature | runtime/memory | see `issues/done/310-fix-activation-frame-root-liveness-depth8-regression.md` |
| 311 | Fix test262 arguments object index assignment semantics | bug | runtime/semantics | see `issues/done/311-fix-test262-arguments-object-index-assignment.md` |
| 314 | Implement string-builtin support | feature | runtime/builtins | see `issues/done/314-implement-string-builtin.md` |
| 315 | Fix Math.max/min backend-io errors | feature | runtime/builtins | see `issues/done/315-fix-math-max-min-backend-io.md` |
| 316 | Fix Object.keys backend-io error | feature | harness | see `issues/done/316-fix-object-keys-backend-io.md` |
| 333 | Implement BigInt dynamic string exception parity | feature | runtime/builtins | see `issues/done/333-implement-bigint-dynamic-string-exception-parity.md` |
| 334 | Array.prototype.map completion: sparse array, thisArg, and generic call | meta | runtime/builtins | see `issues/done/334-complete-array-map-sparse-thisarg-test262.md` |
| 336 | Implement test262 includes directive processing | feature | cli/reference | see `issues/done/336-implement-test262-includes-directive.md` |
| 337 | Implement test262 features directive and $262 object | feature | cli/reference | see `issues/done/337-implement-test262-features-directive.md` |
| 338 | Sparse array holes handling for Array.prototype.map | feature | runtime/builtins | see `issues/done/338-array-map-sparse-array-holes.md` |
| 339 | Callback thisArg for Array.prototype.map | feature | runtime/builtins | see `issues/done/339-array-map-thisarg.md` |
| 340 | Generic call for Array.prototype.map (static dense receiver slice) | feature | runtime/builtins | see `issues/done/340-array-map-generic-call.md` |
| 341 | Implement core builtin API coverage (3,190 test262 cases) (audit reopened #341) | meta | runtime/builtins | see `issues/done/341-implement-core-builtin-api-coverage.md` |
| 341a | Implement isNaN, parseInt, parseFloat, isFinite global functions | feature | runtime/builtins | see `issues/done/341a-global-number-functions.md` |
| 341b | Implement Number constructor and static methods | feature | runtime/builtins | see `issues/done/341b-number-constructor.md` |
| 341c | Implement Boolean global | feature | runtime/builtins | see `issues/done/341c-boolean-global.md` |
| 341d | Implement globalThis binding | feature | runtime/builtins | see `issues/done/341d-globalthis-binding.md` |
| 341e | Implement encodeURI, decodeURI, escape, unescape (audit reopened #341e) | feature | runtime/builtins | see `issues/done/341e-encode-uri.md` |
| 344 | Implement legacy global builtin bindings (8 test262 cases) (audit reopened #344) | feature | runtime/builtins | see `issues/done/344-implement-legacy-global-builtin-bindings.md` |
| 347 | Parser and resolver support for direct eval and eval-code scope | feature | frontend/semantics | see `issues/done/347-parser-resolver-direct-eval-scope.md` |
| 348 | Lowering block-level function declarations in direct eval code | feature | ir | see `issues/done/348-lowering-eval-block-function-declarations.md` |
| 349 | Runtime helper or shim JavaScript emission for direct eval execution | feature | backend | see `issues/done/349-runtime-shim-direct-eval-execution.md` |
| 350 | Implement derived-class private element initialization | feature | runtime/semantics | see `issues/done/350-derived-class-private-element-init.md` |
| 351 | Implement full private brand storage and brand-checking semantics | feature | runtime/semantics | see `issues/done/351-private-brand-storage-brand-checks.md` |
| 352 | Implement static private field ordering with static blocks | feature | runtime/semantics | see `issues/done/352-static-private-field-static-blocks-order.md` |
| 354 | Implement sparse array spread support | feature | runtime/semantics | see `issues/done/354-sparse-array-spread-support.md` |
| 355 | Implement dynamic object property enumeration spread | feature | runtime/semantics | see `issues/done/355-dynamic-object-enumeration-spread.md` |
| 356 | Fix array-push growth WAT format compile blocker | bug | backend | see `issues/done/356-fix-array-push-growth-wat-format-compile-blocker.md` |
| 358 | Instrument ABC451 depth-8 runtime costs | test | runtime/performance | see `issues/done/358-instrument-abc451-depth8-runtime-costs.md` |
| 359 | Reduce ABC451 free-list scan cost (audit reopened #359) | bug | runtime/memory | see `issues/done/359-reduce-abc451-free-list-scan-cost.md` |
| 360 | Reduce ABC451 sweep and copy pressure after free-list fix | bug | runtime/memory | see `issues/done/360-reduce-abc451-sweep-and-copy-pressure-after-free-list-fix.md` |
| 361 | Reduce ABC451 array copy pressure after GC cadence fix | bug | runtime/memory | see `issues/done/361-reduce-abc451-array-copy-pressure-after-gc-cadence-fix.md` |
| 362 | Drive ABC451 depth-8 under iwasm timeout after copy reductions | bug | runtime/memory | see `issues/done/362-drive-abc451-depth8-under-iwasm-timeout-after-copy-reductions.md` |
| 364 | Add ABC451 allocation and copy attribution diagnostic | test | runtime/performance | see `issues/done/364-add-abc451-allocation-copy-attribution-diagnostic.md` |
| 366 | Add ABC451 ArrayPushGrow miss attribution diagnostic | test | runtime/performance | see `issues/done/366-add-arraypushgrow-miss-attribution-diagnostic.md` |
| 367 | Extract ArrayPushGrow into a runtime helper | refactor | backend/runtime | see `issues/done/367-extract-arraypushgrow-runtime-helper.md` |
| 368 | Implement remaining BigInt mixed runtime coercion edges | feature | runtime/semantics | see `issues/done/368-implement-remaining-bigint-mixed-runtime-coercion-edges.md` |
| 369 | Implement full multi-limb BigInt arithmetic | feature | runtime/semantics | see `issues/done/369-implement-full-multilimb-bigint-arithmetic.md` |
| 370 | Implement BigInt arithmetic RangeError and TypeError parity | feature | runtime/semantics | see `issues/done/370-implement-bigint-arithmetic-exception-parity.md` |
| 371 | Define BigInt bitwise and exponentiation policy | feature | runtime/semantics | see `issues/done/371-define-bigint-bitwise-and-exponentiation-policy.md` |
| 372 | Implement BigInt object ToPrimitive non-BigInt primitive returns | feature | runtime/semantics | see `issues/done/372-implement-bigint-object-toprimitive-non-bigint-primitive-returns.md` |
| 373 | Handle BigInt object ToPrimitive invalid and out-of-range string returns | feature | runtime/semantics | see `issues/done/373-handle-bigint-object-toprimitive-invalid-out-of-range-string-returns.md` |
| 374 | Design broader object ToPrimitive for mixed BigInt comparisons | design | runtime/semantics | see `issues/done/374-design-broader-object-toprimitive-for-bigint-comparisons.md` |
| 375 | Handle non-source-backed out-of-range BigInt/String comparisons | feature | runtime/semantics | see `issues/done/375-handle-non-source-backed-out-of-range-bigint-string-comparisons.md` |
| 376 | Implement dynamic BigInt exponentiation | feature | runtime/semantics | see `issues/done/376-implement-dynamic-bigint-exponentiation.md` |
| 377 | Implement BigInt bitwise NOT/AND/OR/XOR | feature | runtime/semantics | see `issues/done/377-implement-bigint-bitwise-not-and-or-xor.md` |
| 378 | Implement BigInt shift operators and unsigned-right-shift policy | feature | runtime/semantics | see `issues/done/378-implement-bigint-shift-operators.md` |
| 379 | Validate Array.prototype.map thisArg against Test262 | test | reference/tests | see `issues/done/379-validate-array-map-thisarg-test262.md` |
| 380 | BigInt division/remainder by zero RangeError | feature | runtime/semantics | see `issues/done/380-bigint-division-remainder-zero-rangeerror.md` |
| 381 | Mixed Number/BigInt arithmetic TypeError | feature | runtime/semantics | see `issues/done/381-mixed-number-bigint-arithmetic-typeerror.md` |
| 382 | Multi-limb BigInt addition and subtraction | feature | runtime/semantics | see `issues/done/382-multilimb-bigint-add-sub.md` |
| 383 | Multi-limb BigInt multiplication | feature | runtime/semantics | see `issues/done/383-multilimb-bigint-multiplication.md` |
| 384 | Multi-limb BigInt division and remainder | feature | runtime/semantics | see `issues/done/384-multilimb-bigint-division-remainder.md` |
| 385 | Instrument ABC451 depth-8 copy vs GC time | feature | runtime/memory | see `issues/done/385-instrument-abc451-depth8-copy-vs-gc-time.md` |
| 386 | Reduce ABC451 depth-8 array copy pressure | feature | runtime/memory | see `issues/done/386-reduce-abc451-depth8-array-copy-pressure.md` |
| 387 | Implement BigInt bitwise outside signed-i64 helper slice | feature | runtime/semantics | see `issues/done/387-implement-bigint-bitwise-outside-signed-i64-slice.md` |
| 388 | Runtime array-like generic call for Array.prototype.map | feature | runtime/builtins | see `issues/done/388-array-map-generic-runtime-array-like.md` |
| 389 | Unblock Array.map generic Test262 representative with function-expression initializer | bug | frontend/parser | see `issues/done/389-array-map-generic-test262-function-expression-blocker.md` |
| 390 | Allow or rewrite Test262 harness undefined binding name | bug | frontend/parser | see `issues/done/390-test262-harness-undefined-binding-name.md` |
| 391 | Multi-limb BigInt division | feature | runtime/semantics | see `issues/done/391-multilimb-bigint-division.md` |
| 392 | Multi-limb BigInt remainder | feature | runtime/semantics | see `issues/done/392-multilimb-bigint-remainder.md` |
| 393 | Multi-limb BigInt addition | feature | runtime/semantics | see `issues/done/393-multilimb-bigint-addition.md` |
| 394 | Multi-limb BigInt subtraction | feature | runtime/semantics | see `issues/done/394-multilimb-bigint-subtraction.md` |
| 395 | Runtime array-like Array.prototype.map callback dispatch | feature | runtime/builtins | see `issues/done/395-array-map-runtime-array-like-callback-dispatch.md` |
| 396 | Runtime JS exception object throwing substrate | feature | runtime/semantics | see `issues/done/396-runtime-js-exception-object-throw-substrate.md` |
| 397 | BigInt add/sub branch-assigned operands | feature | runtime/semantics | see `issues/done/397-bigint-add-sub-branch-assigned-operands.md` |
| 398 | Track control-flow-assigned BigInt div/rem locals | feature | runtime/semantics | see `issues/done/398-track-control-flow-assigned-bigint-div-rem.md` |
| 399 | Define TypeScript parse, erase, and emit boundary contract | spike | frontend | see `issues/done/399-define-typescript-parse-erase-emit-boundary.md` |
| 400 | Implement ambient declaration erasure and rejection boundary | feature | frontend/syntax | see `issues/done/400-implement-ambient-declaration-erasure-boundary.md` |
| 401 | Implement generator function syntax prerequisite for iterator spread | feature | frontend/syntax | see `issues/done/401-generator-function-syntax-prerequisite-for-iterator-spread.md` |
| 402 | Implement computed Symbol.iterator prerequisite for spread | feature | frontend/syntax | see `issues/done/402-computed-symbol-iterator-prerequisite-for-spread.md` |
| 403 | Define sparse array hole representation contract | feature | runtime/semantics | see `issues/done/403-sparse-array-hole-representation-contract.md` |
| 404 | Support mutable outer local captures in callback functions | feature | ir/runtime | see `issues/done/404-mutable-outer-local-callback-captures.md` |
| 405 | Support Test262 harness instanceof function RHS | feature | frontend/semantics | see `issues/done/405-test262-instanceof-harness-function-rhs.md` |
| 406 | Direct eval Annex B existing binding residuals | feature | frontend/semantics | see `issues/done/406-direct-eval-annexb-existing-binding-residuals.md` |
| 407 | Implement key-preserving Map entry storage for spread iteration | feature | runtime/semantics | see `issues/done/407-map-spread-key-preserving-iterator-storage.md` |
| 408 | Implement tsgo declaration emit: AsConstSatisfies/const generic method cases | feature | frontend/syntax | see file |
| 409 | Implement tsgo declaration emit: package-json exports and subpath reexport cases | feature | frontend/syntax | see `issues/done/409-implement-tsgo-declaration-emit-package-json-subpath.md` |
| 410 | Implement tsgo declaration emit: subpath import declaration emit cases | feature | frontend/syntax | see `issues/done/410-implement-tsgo-declaration-emit-subpath-import-links.md` |
| 412 | Implement arguments-object support | spike | runtime/builtins | see `issues/done/412-implement-arguments-object.md` |
| 413 | Implement arity support | spike | reference/triage | see `issues/done/413-implement-arity.md` |
| 414 | Implement array-builtin support (dup) | spike | frontend/syntax | see `issues/done/414-implement-array-builtin.md` |
| 415 | Implement arrow functions | spike | frontend/syntax | see `issues/done/415-implement-arrow-function.md` |
| 427 | Implement duplicate-local support | spike | reference/triage | see `issues/done/427-implement-duplicate-local.md` |
| 430 | Implement function support (dup) | spike | frontend/syntax | see `issues/done/430-implement-function.md` |
| 433 | Implement legacy-global-builtin support (dup) | spike | frontend/syntax | see `issues/done/433-implement-legacy-global-builtin.md` |
| 444 | Implement RegExp literal support (dup) | spike | runtime/builtins | see `issues/done/444-implement-regexp-literal.md` |
| 447 | Implement spread operator | spike | frontend/syntax | see `issues/done/447-implement-spread.md` |
| 448 | Implement string-builtin support | spike | frontend/syntax | see `issues/done/448-implement-string-builtin.md` |
| 455 | Implement Apilibcheck (dup) | spike | frontend/syntax | see `issues/done/455-implement-APILibCheck.md` |
| 456 | Implement Apisample Arrow Function (dup) | spike | frontend/syntax | see `issues/done/456-implement-APISample-arrow-function.md` |
| 457 | Implement Apisample Import Export (dup) | spike | frontend/syntax | see `issues/done/457-implement-APISample-import-export.md` |
| 458 | Implement Apisample Jsdoc (dup) | spike | frontend/syntax | see `issues/done/458-implement-APISample-jsdoc.md` |
| 459 | Implement Arrowfunctionexpression (audit reopened #459) | spike | frontend/syntax | see `issues/done/459-implement-ArrowFunctionExpression.md` |
| 460 | Implement Classdeclaration (dup) | spike | frontend/syntax | see `issues/done/460-implement-ClassDeclaration.md` |
| 461 | Implement Classdeclarationwithinvalidconstonpropertydeclaration (audit reopened #461) | spike | frontend/syntax | see `issues/done/461-implement-ClassDeclarationWithInvalidConstOnPropertyDeclaration.md` |
| 462 | Implement Exportassignment (dup) | spike | frontend/syntax | see `issues/done/462-implement-ExportAssignment.md` |
| 463 | Implement Functiondeclaration Import Export | spike | frontend/syntax | see `issues/done/463-implement-FunctionDeclaration-import-export.md` |
| 464 | Implement Functiondeclaration Parser Syntax (dup) | spike | frontend/syntax | see `issues/done/464-implement-FunctionDeclaration-parser-syntax.md` |
| 465 | Implement Memberaccessordeclaration (audit reopened #465) | spike | frontend/syntax | see `issues/done/465-implement-MemberAccessorDeclaration.md` |
| 466 | Implement Parameterlist | spike | frontend/syntax | see `issues/done/466-implement-ParameterList.md` |
| 467 | Implement Transportstream (dup) | spike | frontend/syntax | see `issues/done/467-implement-TransportStream.md` |
| 468 | Implement Abstractclassinlocalscope (dup) | spike | frontend/syntax | see `issues/done/468-implement-abstractClassInLocalScope.md` |
| 469 | Implement Abstractclassinlocalscopeisabstract (dup) | spike | frontend/syntax | see `issues/done/469-implement-abstractClassInLocalScopeIsAbstract.md` |
| 470 | Implement Abstractclassunioninstantiation (dup) | spike | frontend/resolver | see `issues/done/470-implement-abstractClassUnionInstantiation.md` |
| 471 | Implement Abstractpropertybasics (dup) | spike | frontend/syntax | see `issues/done/471-implement-abstractPropertyBasics.md` |
| 472 | Implement Abstractpropertyinconstructor (dup) | spike | frontend/syntax | see `issues/done/472-implement-abstractPropertyInConstructor.md` |
| 473 | Implement Abstractpropertynegative (dup) | spike | frontend/syntax | see `issues/done/473-implement-abstractPropertyNegative.md` |
| 474 | Implement Acceptsymbolasweaktype (dup) | spike | frontend/resolver | see `issues/done/474-implement-acceptSymbolAsWeakType.md` |
| 475 | Implement Acceptablealias (dup) | spike | frontend/syntax | see `issues/done/475-implement-acceptableAlias.md` |
| 476 | Implement Accessinstancememberfromstaticmethod (dup) | spike | frontend/resolver | see `issues/done/476-implement-accessInstanceMemberFromStaticMethod.md` |
| 477 | Implement Accessoverriddenbaseclassmember (audit reopened #477) | spike | frontend/syntax | see `issues/done/477-implement-accessOverriddenBaseClassMember.md` |
| 478 | Implement Accessstaticmemberfrominstancemethod (dup) | spike | frontend/resolver | see `issues/done/478-implement-accessStaticMemberFromInstanceMethod.md` |
| 479 | Implement Accessoraccidentalcalldiagnostic (dup) | spike | frontend/syntax | see `issues/done/479-implement-accessorAccidentalCallDiagnostic.md` |
| 480 | Implement Accessordeclarationemitjs (dup) | spike | frontend/syntax | see `issues/done/480-implement-accessorDeclarationEmitJs.md` |
| 481 | Implement Accessordeclarationemitvisibilityerrors (dup) | spike | frontend/syntax | see `issues/done/481-implement-accessorDeclarationEmitVisibilityErrors.md` |
| 482 | Implement Accessordeclarationorder (audit reopened #482) | spike | frontend/syntax | see `issues/done/482-implement-accessorDeclarationOrder.md` |
| 483 | Implement Accessorinambientcontextes (audit reopened #483) | spike | frontend/syntax | see `issues/done/483-implement-accessorInAmbientContextES.md` |
| 484 | Implement Accessorinferredreturntypeerrorinreturnstatement (dup) | spike | frontend/syntax | see `issues/done/484-implement-accessorInferredReturnTypeErrorInReturnStatement.md` |
| 485 | Implement Accessorparameteraccessibilitymodifier (audit reopened #485) | spike | frontend/syntax | see `issues/done/485-implement-accessorParameterAccessibilityModifier.md` |
| 486 | Implement Accessorwithlineterminator (dup) | spike | reference/triage | see `issues/done/486-implement-accessorWithLineTerminator.md` |
| 487 | Implement Accessorwithoutbody (audit reopened #487) | spike | frontend/syntax | see `issues/done/487-implement-accessorWithoutBody.md` |
| 488 | Implement Accessors (dup) | spike | frontend/syntax | see `issues/done/488-implement-accessors.md` |
| 489 | Implement Accessorsinambientcontext (audit reopened #489) | spike | frontend/syntax | see `issues/done/489-implement-accessorsInAmbientContext.md` |
| 490 | Implement Addmorecallsignaturestobasesignature (dup) | spike | frontend/resolver | see `issues/done/490-implement-addMoreCallSignaturesToBaseSignature.md` |
| 491 | Implement Aliasassignments (dup) | spike | frontend/syntax | see `issues/done/491-implement-aliasAssignments.md` |
| 492 | Implement Aliasbug (dup) | spike | frontend/syntax | see `issues/done/492-implement-aliasBug.md` |
| 493 | Implement Aliasdoesnotduplicatesignatures (dup) | spike | frontend/syntax | see `issues/done/493-implement-aliasDoesNotDuplicateSignatures.md` |
| 494 | Implement Aliaserrors (dup) | spike | frontend/syntax | see `issues/done/494-implement-aliasErrors.md` |
| 495 | Implement Aliasinaccessiblemodule (audit reopened #495) | spike | frontend/syntax | see `issues/done/495-implement-aliasInaccessibleModule.md` |
| 496 | Implement Aliasinstantiationexpressiongenericintersectionnocrash (dup) | spike | frontend/syntax | see `issues/done/496-implement-aliasInstantiationExpressionGenericIntersectionNoCrash.md` |
| 497 | Implement Aliasonmergedmoduleinterface (dup) | spike | frontend/syntax | see `issues/done/497-implement-aliasOnMergedModuleInterface.md` |
| 498 | Implement Aliasusageinaccessorsofclass (dup) | spike | frontend/syntax | see `issues/done/498-implement-aliasUsageInAccessorsOfClass.md` |
| 499 | Implement Aliasusageinarray (dup) | spike | frontend/syntax | see `issues/done/499-implement-aliasUsageInArray.md` |
| 500 | Implement Aliasusageinfunctionexpression (dup) | spike | frontend/syntax | see `issues/done/500-implement-aliasUsageInFunctionExpression.md` |
| 501 | Implement Aliasusageingenericfunction (dup) | spike | frontend/syntax | see `issues/done/501-implement-aliasUsageInGenericFunction.md` |
| 502 | Implement Aliasusageinindexerofclass (dup) | spike | frontend/syntax | see `issues/done/502-implement-aliasUsageInIndexerOfClass.md` |
| 503 | Implement Aliasusageinobjectliteral (dup) | spike | frontend/syntax | see `issues/done/503-implement-aliasUsageInObjectLiteral.md` |
| 504 | Implement Aliasusageinorexpression (dup) | spike | frontend/syntax | see `issues/done/504-implement-aliasUsageInOrExpression.md` |
| 505 | Implement Aliasusageintypeargumentofextendsclause (dup) | spike | frontend/syntax | see `issues/done/505-implement-aliasUsageInTypeArgumentOfExtendsClause.md` |
| 506 | Implement Aliasusageinvarassignment (dup) | spike | frontend/syntax | see `issues/done/506-implement-aliasUsageInVarAssignment.md` |
| 507 | Implement Aliasusedasnamevalue (dup) | spike | frontend/syntax | see `issues/done/507-implement-aliasUsedAsNameValue.md` |
| 508 | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer (dup) | spike | frontend/syntax | see `issues/done/508-implement-aliasWithInterfaceExportAssignmentUsedInVarInitializer.md` |
| 509 | Implement Aliasesinsystemmodule (dup) | spike | frontend/syntax | see `issues/done/509-implement-aliasesInSystemModule.md` |
| 510 | Implement Allowimportclausestomergewithtypes (dup) | spike | frontend/syntax | see `issues/done/510-implement-allowImportClausesToMergeWithTypes.md` |
| 511 | Implement Allowjsclassthistypecrash (dup) | spike | reference/triage | see `issues/done/511-implement-allowJsClassThisTypeCrash.md` |
| 512 | Implement Allowjscrossmonorepopackage (dup) | spike | frontend/syntax | see `issues/done/512-implement-allowJsCrossMonorepoPackage.md` |
| 513 | Implement Allowjscheckjstypeparameternocrash (dup) | spike | frontend/syntax | see `issues/done/513-implement-allowJscheckJsTypeParameterNoCrash.md` |
| 514 | Implement Allowsyntheticdefaultimports (dup) | spike | frontend/syntax | see `issues/done/514-implement-allowSyntheticDefaultImports.md` |
| 515 | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration (dup) | spike | frontend/syntax | see `issues/done/515-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md` |
| 516 | Implement Alwaysstrictmodule (audit reopened #516) | spike | frontend/syntax | see `issues/done/516-implement-alwaysStrictModule.md` |
| 517 | Implement Alwaysstrictnoimplicitusestrict (audit reopened #517) | spike | frontend/syntax | see `issues/done/517-implement-alwaysStrictNoImplicitUseStrict.md` |
| 518 | Implement Ambientclassdeclarationwithextends | spike | frontend/syntax | see `issues/done/518-implement-ambientClassDeclarationWithExtends.md` |
| 519 | Implement Ambientclassdeclaredbeforebase (dup) | spike | frontend/syntax | see `issues/done/519-implement-ambientClassDeclaredBeforeBase.md` |
| 520 | Implement Ambientconstliterals (dup) | spike | frontend/syntax | see `issues/done/520-implement-ambientConstLiterals.md` |
| 521 | Implement Ambientenumelementinitializer | spike | frontend/syntax | see `issues/done/521-implement-ambientEnumElementInitializer.md` |
| 522 | Implement Ambienterrors | spike | runtime/builtins | see `issues/done/522-implement-ambientErrors.md` |
| 523 | Implement Ambientexportdefaulterrors (dup) | spike | frontend/syntax | see `issues/done/523-implement-ambientExportDefaultErrors.md` |
| 524 | Implement Ambientexternalmoduleinanotherexternalmodule (dup) | spike | frontend/syntax | see `issues/done/524-implement-ambientExternalModuleInAnotherExternalModule.md` |
| 525 | Implement Ambientexternalmodulereopen (audit reopened #525) | spike | frontend/syntax | see `issues/done/525-implement-ambientExternalModuleReopen.md` |
| 526 | Implement Ambientexternalmodulewithinternalimportdeclaration (dup) | spike | frontend/syntax | see `issues/done/526-implement-ambientExternalModuleWithInternalImportDeclaration.md` |
| 527 | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration (audit reopened #527) | spike | frontend/syntax | see `issues/done/527-implement-ambientExternalModuleWithRelativeExternalImportDeclaration.md` |
| 528 | Implement Ambientexternalmodulewithrelativemodulename (audit reopened #528) | spike | frontend/syntax | see `issues/done/528-implement-ambientExternalModuleWithRelativeModuleName.md` |
| 529 | Implement Ambientexternalmodulewithoutinternalimportdeclaration (dup) | spike | frontend/syntax | see `issues/done/529-implement-ambientExternalModuleWithoutInternalImportDeclaration.md` |
| 530 | Implement Ambientfundule (audit reopened #530) | spike | frontend/syntax | see `issues/done/530-implement-ambientFundule.md` |
| 531 | Implement Ambientmoduleexports (dup) | spike | frontend/syntax | see `issues/done/531-implement-ambientModuleExports.md` |
| 532 | Implement Ambientmodulewithclassdeclarationwithextends (audit reopened #532) | spike | frontend/syntax | see `issues/done/532-implement-ambientModuleWithClassDeclarationWithExtends.md` |
| 533 | Implement Ambientmodulewithtemplateliterals (dup) | spike | frontend/syntax | see `issues/done/533-implement-ambientModuleWithTemplateLiterals.md` |
| 534 | Implement Ambientmodules (dup) | spike | frontend/syntax | see `issues/done/534-implement-ambientModules.md` |
| 535 | Implement Ambientnamerestrictions (audit reopened #535) | spike | frontend/syntax | see `issues/done/535-implement-ambientNameRestrictions.md` |
| 536 | Implement Ambientrequirefunction (dup) | spike | frontend/syntax | see `issues/done/536-implement-ambientRequireFunction.md` |
| 537 | Implement Ambientstatement (audit reopened #537) | spike | frontend/syntax | see `issues/done/537-implement-ambientStatement.md` |
| 538 | Implement Ambientwithstatements (audit reopened #538) | spike | frontend/syntax | see `issues/done/538-implement-ambientWithStatements.md` |
| 539 | Implement Ambiguouscallswherereturntypesagree (dup) | spike | frontend/syntax | see `issues/done/539-implement-ambiguousCallsWhereReturnTypesAgree.md` |
| 540 | Implement Ambiguousgenericassertion (dup) | spike | frontend/syntax | see `issues/done/540-implement-ambiguousGenericAssertion.md` |
| 545 | Implement Arrowfunctionexpression (audit reopened #545) | spike | frontend/syntax | see `issues/done/545-implement-ArrowFunctionExpression.md` |
| 546 | Implement Classdeclaration | spike | frontend/syntax | see `issues/done/546-implement-ClassDeclaration.md` |
| 547 | Implement Classdeclarationwithinvalidconstonpropertydeclaration (audit reopened #547) | spike | frontend/syntax | see `issues/done/547-implement-ClassDeclarationWithInvalidConstOnPropertyDeclaration.md` |
| 548 | Implement Exportassignment | spike | frontend/syntax | see `issues/done/548-implement-ExportAssignment.md` |
| 549 | Implement Functiondeclaration Import Export (audit reopened #549) | spike | frontend/syntax | see `issues/done/549-implement-FunctionDeclaration-import-export.md` |
| 550 | Implement Functiondeclaration Parser Syntax | spike | frontend/syntax | see `issues/done/550-implement-FunctionDeclaration-parser-syntax.md` |
| 551 | Implement Memberaccessordeclaration (audit reopened #551) | spike | frontend/syntax | see `issues/done/551-implement-MemberAccessorDeclaration.md` |
| 552 | Implement Parameterlist | spike | frontend/syntax | see `issues/done/552-implement-ParameterList.md` |
| 553 | Implement Transportstream | spike | frontend/syntax | see `issues/done/553-implement-TransportStream.md` |
| 557 | Implement Abstractpropertybasics | spike | frontend/syntax | see `issues/done/557-implement-abstractPropertyBasics.md` |
| 558 | Implement Abstractpropertyinconstructor | spike | frontend/syntax | see `issues/done/558-implement-abstractPropertyInConstructor.md` |
| 559 | Implement Abstractpropertynegative | spike | frontend/syntax | see `issues/done/559-implement-abstractPropertyNegative.md` |
| 560 | Implement Acceptsymbolasweaktype | spike | frontend/resolver | see `issues/done/560-implement-acceptSymbolAsWeakType.md` |
| 561 | Implement Acceptablealias | spike | frontend/syntax | see `issues/done/561-implement-acceptableAlias.md` |
| 562 | Implement Accessinstancememberfromstaticmethod | spike | frontend/resolver | see `issues/done/562-implement-accessInstanceMemberFromStaticMethod.md` |
| 563 | Implement Accessoverriddenbaseclassmember (audit reopened #563) | spike | frontend/syntax | see `issues/done/563-implement-accessOverriddenBaseClassMember.md` |
| 564 | Implement Accessstaticmemberfrominstancemethod | spike | frontend/resolver | see `issues/done/564-implement-accessStaticMemberFromInstanceMethod.md` |
| 565 | Implement Accessoraccidentalcalldiagnostic | spike | frontend/syntax | see `issues/done/565-implement-accessorAccidentalCallDiagnostic.md` |
| 566 | Implement Accessordeclarationemitjs | spike | frontend/syntax | see `issues/done/566-implement-accessorDeclarationEmitJs.md` |
| 567 | Implement Accessordeclarationemitvisibilityerrors | spike | frontend/syntax | see `issues/done/567-implement-accessorDeclarationEmitVisibilityErrors.md` |
| 568 | Implement Accessordeclarationorder (audit reopened #568) | spike | frontend/syntax | see `issues/done/568-implement-accessorDeclarationOrder.md` |
| 569 | Implement Accessorinambientcontextes (audit reopened #569) | spike | frontend/syntax | see `issues/done/569-implement-accessorInAmbientContextES.md` |
| 570 | Implement Accessorinferredreturntypeerrorinreturnstatement | spike | frontend/syntax | see `issues/done/570-implement-accessorInferredReturnTypeErrorInReturnStatement.md` |
| 571 | Implement Accessorparameteraccessibilitymodifier (audit reopened #571) | spike | frontend/syntax | see `issues/done/571-implement-accessorParameterAccessibilityModifier.md` |
| 572 | Implement Accessorwithlineterminator | spike | reference/triage | see `issues/done/572-implement-accessorWithLineTerminator.md` |
| 573 | Implement Accessorwithoutbody (audit reopened #573) | spike | frontend/syntax | see `issues/done/573-implement-accessorWithoutBody.md` |
| 574 | Implement Accessors | spike | frontend/syntax | see `issues/done/574-implement-accessors.md` |
| 575 | Implement Accessorsinambientcontext (audit reopened #575) | spike | frontend/syntax | see `issues/done/575-implement-accessorsInAmbientContext.md` |
| 576 | Implement Addmorecallsignaturestobasesignature | spike | frontend/syntax | see `issues/done/576-implement-addMoreCallSignaturesToBaseSignature.md` |
| 577 | Implement Aliasassignments | spike | frontend/syntax | see `issues/done/577-implement-aliasAssignments.md` |
| 578 | Implement Aliasbug | spike | frontend/syntax | see `issues/done/578-implement-aliasBug.md` |
| 579 | Implement Aliasdoesnotduplicatesignatures | spike | frontend/syntax | see `issues/done/579-implement-aliasDoesNotDuplicateSignatures.md` |
| 580 | Implement Aliaserrors | spike | frontend/syntax | see `issues/done/580-implement-aliasErrors.md` |
| 581 | Implement Aliasinaccessiblemodule (audit reopened #581) | spike | frontend/syntax | see `issues/done/581-implement-aliasInaccessibleModule.md` |
| 582 | Implement Aliasinstantiationexpressiongenericintersectionnocrash | spike | frontend/syntax | see `issues/done/582-implement-aliasInstantiationExpressionGenericIntersectionNoCrash.md` |
| 583 | Implement Aliasonmergedmoduleinterface | spike | frontend/syntax | see `issues/done/583-implement-aliasOnMergedModuleInterface.md` |
| 584 | Implement Aliasusageinaccessorsofclass | spike | frontend/syntax | see `issues/done/584-implement-aliasUsageInAccessorsOfClass.md` |
| 585 | Implement Aliasusageinarray | spike | frontend/syntax | see `issues/done/585-implement-aliasUsageInArray.md` |
| 586 | Implement Aliasusageinfunctionexpression | spike | frontend/syntax | see `issues/done/586-implement-aliasUsageInFunctionExpression.md` |
| 587 | Implement Aliasusageingenericfunction | spike | frontend/syntax | see `issues/done/587-implement-aliasUsageInGenericFunction.md` |
| 588 | Implement Aliasusageinindexerofclass | spike | frontend/syntax | see `issues/done/588-implement-aliasUsageInIndexerOfClass.md` |
| 589 | Implement Aliasusageinobjectliteral | spike | frontend/syntax | see `issues/done/589-implement-aliasUsageInObjectLiteral.md` |
| 590 | Implement Aliasusageinorexpression | spike | frontend/syntax | see `issues/done/590-implement-aliasUsageInOrExpression.md` |
| 591 | Implement Aliasusageintypeargumentofextendsclause | spike | frontend/syntax | see `issues/done/591-implement-aliasUsageInTypeArgumentOfExtendsClause.md` |
| 592 | Implement Aliasusageinvarassignment | spike | frontend/syntax | see `issues/done/592-implement-aliasUsageInVarAssignment.md` |
| 593 | Implement Aliasusedasnamevalue | spike | frontend/syntax | see `issues/done/593-implement-aliasUsedAsNameValue.md` |
| 594 | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer | spike | frontend/syntax | see `issues/done/594-implement-aliasWithInterfaceExportAssignmentUsedInVarInitializer.md` |
| 595 | Implement Aliasesinsystemmodule | spike | frontend/syntax | see `issues/done/595-implement-aliasesInSystemModule.md` |
| 596 | Implement Allowimportclausestomergewithtypes | spike | frontend/syntax | see `issues/done/596-implement-allowImportClausesToMergeWithTypes.md` |
| 598 | Implement Allowjscrossmonorepopackage | spike | frontend/syntax | see `issues/done/598-implement-allowJsCrossMonorepoPackage.md` |
| 599 | Implement Allowjscheckjstypeparameternocrash | spike | frontend/syntax | see `issues/done/599-implement-allowJscheckJsTypeParameterNoCrash.md` |
| 600 | Implement Allowsyntheticdefaultimports | spike | frontend/syntax | see `issues/done/600-implement-allowSyntheticDefaultImports.md` |
| 601 | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration | spike | frontend/syntax | see `issues/done/601-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md` |
| 602 | Implement Alwaysstrictmodule (audit reopened #602) | spike | frontend/syntax | see `issues/done/602-implement-alwaysStrictModule.md` |
| 603 | Implement Alwaysstrictnoimplicitusestrict (audit reopened #603) | spike | frontend/syntax | see `issues/done/603-implement-alwaysStrictNoImplicitUseStrict.md` |
| 604 | Implement Ambientclassdeclarationwithextends (audit reopened #604) | spike | frontend/syntax | see `issues/done/604-implement-ambientClassDeclarationWithExtends.md` |
| 605 | Implement Ambientclassdeclaredbeforebase | spike | frontend/syntax | see `issues/done/605-implement-ambientClassDeclaredBeforeBase.md` |
| 606 | Implement Ambientconstliterals | spike | frontend/syntax | see `issues/done/606-implement-ambientConstLiterals.md` |
| 607 | Implement Ambientenumelementinitializer (dup) | spike | frontend/syntax | see `issues/done/607-implement-ambientEnumElementInitializer.md` |
| 608 | Implement Ambienterrors | spike | runtime/builtins | see `issues/done/608-implement-ambientErrors.md` |
| 609 | Implement Ambientexportdefaulterrors | spike | frontend/syntax | see `issues/done/609-implement-ambientExportDefaultErrors.md` |
| 610 | Implement Ambientexternalmoduleinanotherexternalmodule | spike | frontend/syntax | see `issues/done/610-implement-ambientExternalModuleInAnotherExternalModule.md` |
| 611 | Implement Ambientexternalmodulereopen (audit reopened #611) | spike | frontend/syntax | see `issues/done/611-implement-ambientExternalModuleReopen.md` |
| 612 | Implement Ambientexternalmodulewithinternalimportdeclaration | spike | frontend/syntax | see `issues/done/612-implement-ambientExternalModuleWithInternalImportDeclaration.md` |
| 613 | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration (audit reopened #613) | spike | frontend/syntax | see `issues/done/613-implement-ambientExternalModuleWithRelativeExternalImportDeclaration.md` |
| 614 | Implement Ambientexternalmodulewithrelativemodulename (audit reopened #614) | spike | frontend/syntax | see `issues/done/614-implement-ambientExternalModuleWithRelativeModuleName.md` |
| 615 | Implement Ambientexternalmodulewithoutinternalimportdeclaration | spike | frontend/syntax | see `issues/done/615-implement-ambientExternalModuleWithoutInternalImportDeclaration.md` |
| 616 | Implement Ambientfundule (audit reopened #616) | spike | frontend/syntax | see `issues/done/616-implement-ambientFundule.md` |
| 617 | Implement Ambientmoduleexports | spike | frontend/syntax | see `issues/done/617-implement-ambientModuleExports.md` |
| 618 | Implement Ambientmodulewithclassdeclarationwithextends (audit reopened #618) | spike | frontend/syntax | see `issues/done/618-implement-ambientModuleWithClassDeclarationWithExtends.md` |
| 619 | Implement Ambientmodulewithtemplateliterals | spike | frontend/syntax | see `issues/done/619-implement-ambientModuleWithTemplateLiterals.md` |
| 620 | Implement Ambientmodules | spike | frontend/syntax | see `issues/done/620-implement-ambientModules.md` |
| 621 | Implement Ambientnamerestrictions (audit reopened #621) | spike | frontend/syntax | see `issues/done/621-implement-ambientNameRestrictions.md` |
| 622 | Implement Ambientrequirefunction | spike | frontend/syntax | see `issues/done/622-implement-ambientRequireFunction.md` |
| 623 | Implement Ambientstatement (audit reopened #623) | spike | frontend/syntax | see `issues/done/623-implement-ambientStatement.md` |
| 624 | Implement Ambientwithstatements (audit reopened #624) | spike | frontend/syntax | see `issues/done/624-implement-ambientWithStatements.md` |
| 638 | Implement Anonymousclassexpression (audit reopened #638) | spike | frontend/syntax | see `issues/done/638-implement-anonymousClassExpression.md` |
| 639 | Implement Anonymousmodules (audit reopened #639) | spike | frontend/syntax | see `issues/done/639-implement-anonymousModules.md` |
| 641 | Implement Anyasreturntypefornewoncall (audit reopened #641) | spike | frontend/syntax | see `issues/done/641-implement-anyAsReturnTypeForNewOnCall.md` |
| 642 | Implement Anydeclare (audit reopened #642) | spike | frontend/syntax | see `issues/done/642-implement-anyDeclare.md` |
| 661 | Implement Arithassigntyping | spike | frontend/syntax | see `issues/done/661-implement-arithAssignTyping.md` |
| 669 | Implement Arrayconcat (dup) | spike | frontend/syntax | see `issues/done/669-implement-arrayConcat.md` |
| 707 | Implement Asibreak | spike | frontend/syntax | see `issues/done/707-implement-asiBreak.md` |
| 708 | Implement Asicontinue | spike | frontend/syntax | see `issues/done/708-implement-asiContinue.md` |
| 762 | Implement Asyncyieldstarcontextualtype | spike | frontend/syntax | see `issues/done/762-implement-asyncYieldStarContextualType.md` |
| 763 | Implement Augmentexportequals | spike | frontend/syntax | see `issues/done/763-implement-augmentExportEquals.md` |
| 764 | Implement Augmentedclasswithprototypepropertyonmodule | spike | frontend/syntax | see `issues/done/764-implement-augmentedClassWithPrototypePropertyOnModule.md` |
| 765 | Implement Augmentedtypesclass | spike | frontend/syntax | see `issues/done/765-implement-augmentedTypesClass.md` |
| 766 | Implement Augmentedtypesenum Import Export | spike | frontend/syntax | see `issues/done/766-implement-augmentedTypesEnum-import-export.md` |
| 767 | Implement Augmentedtypesenum Parser Syntax | spike | frontend/syntax | see `issues/done/767-implement-augmentedTypesEnum-parser-syntax.md` |
| 768 | Implement Augmentedtypesexternalmodule | spike | frontend/syntax | see `issues/done/768-implement-augmentedTypesExternalModule.md` |
| 769 | Implement Augmentedtypesfunction | spike | frontend/resolver | see `issues/done/769-implement-augmentedTypesFunction.md` |
| 770 | Implement Augmentedtypesinterface | spike | frontend/syntax | see `issues/done/770-implement-augmentedTypesInterface.md` |
| 771 | Implement Augmentedtypesmodules | spike | frontend/syntax | see `issues/done/771-implement-augmentedTypesModules.md` |
| 772 | Implement Augmentedtypesvar | spike | frontend/resolver | see `issues/done/772-implement-augmentedTypesVar.md` |
| 773 | Implement Autoasiforstaticsinclassdeclaration | spike | frontend/parser | see `issues/done/773-implement-autoAsiForStaticsInClassDeclaration.md` |
| 774 | Implement Autolift | spike | frontend/syntax | see `issues/done/774-implement-autoLift.md` |
| 775 | Implement Autotypeassignedusingdestructuringfromnevernocrash | spike | frontend/resolver | see `issues/done/775-implement-autoTypeAssignedUsingDestructuringFromNeverNoCrash.md` |
| 776 | Implement Apilibcheck (dup) | spike | frontend/syntax | see `issues/done/776-implement-APILibCheck.md` |
| 777 | Implement Apisample Arrow Function (dup) | spike | frontend/syntax | see `issues/done/777-implement-APISample-arrow-function.md` |
| 778 | Implement Apisample Import Export (dup) | spike | frontend/syntax | see `issues/done/778-implement-APISample-import-export.md` |
| 779 | Implement Apisample Jsdoc (dup) | spike | frontend/syntax | see `issues/done/779-implement-APISample-jsdoc.md` |
| 780 | Implement Arrowfunctionexpression (audit reopened #780) | spike | frontend/syntax | see `issues/done/780-implement-ArrowFunctionExpression.md` |
| 781 | Implement Classdeclaration (dup) | spike | frontend/syntax | see `issues/done/781-implement-ClassDeclaration.md` |
| 782 | Implement Classdeclarationwithinvalidconstonpropertydeclaration (audit reopened #782) | spike | frontend/syntax | see `issues/done/782-implement-ClassDeclarationWithInvalidConstOnPropertyDeclaration.md` |
| 783 | Implement Exportassignment (dup) | spike | frontend/syntax | see `issues/done/783-implement-ExportAssignment.md` |
| 784 | Implement Functiondeclaration Import Export (audit reopened #784) | spike | frontend/syntax | see `issues/done/784-implement-FunctionDeclaration-import-export.md` |
| 785 | Implement Functiondeclaration Parser Syntax (dup) | spike | frontend/syntax | see `issues/done/785-implement-FunctionDeclaration-parser-syntax.md` |
| 786 | Implement Memberaccessordeclaration (audit reopened #786) | spike | frontend/syntax | see `issues/done/786-implement-MemberAccessorDeclaration.md` |
| 787 | Implement Parameterlist | spike | frontend/syntax | see `issues/done/787-implement-ParameterList.md` |
| 788 | Implement Transportstream (dup) | spike | frontend/syntax | see `issues/done/788-implement-TransportStream.md` |
| 789 | Implement Abstractclassinlocalscope (dup) | spike | frontend/syntax | see `issues/done/789-implement-abstractClassInLocalScope.md` |
| 790 | Implement Abstractclassinlocalscopeisabstract (dup) | spike | frontend/syntax | see `issues/done/790-implement-abstractClassInLocalScopeIsAbstract.md` |
| 791 | Implement Abstractclassunioninstantiation (dup) | spike | frontend/resolver | see `issues/done/791-implement-abstractClassUnionInstantiation.md` |
| 792 | Implement Abstractpropertybasics (dup) | spike | frontend/syntax | see `issues/done/792-implement-abstractPropertyBasics.md` |
| 793 | Implement Abstractpropertyinconstructor (dup) | spike | frontend/syntax | see `issues/done/793-implement-abstractPropertyInConstructor.md` |
| 794 | Implement Abstractpropertynegative (dup) | spike | frontend/syntax | see `issues/done/794-implement-abstractPropertyNegative.md` |
| 795 | Implement Acceptsymbolasweaktype (dup) | spike | frontend/resolver | see `issues/done/795-implement-acceptSymbolAsWeakType.md` |
| 796 | Implement Acceptablealias (dup) | spike | frontend/syntax | see `issues/done/796-implement-acceptableAlias.md` |
| 797 | Implement Accessinstancememberfromstaticmethod (dup) | spike | frontend/resolver | see `issues/done/797-implement-accessInstanceMemberFromStaticMethod.md` |
| 798 | Implement Accessoverriddenbaseclassmember (audit reopened #798) | spike | frontend/syntax | see `issues/done/798-implement-accessOverriddenBaseClassMember.md` |
| 799 | Implement Accessstaticmemberfrominstancemethod (dup) | spike | frontend/resolver | see `issues/done/799-implement-accessStaticMemberFromInstanceMethod.md` |
| 800 | Implement Accessoraccidentalcalldiagnostic (dup) | spike | frontend/syntax | see `issues/done/800-implement-accessorAccidentalCallDiagnostic.md` |
| 801 | Implement Accessordeclarationemitjs (dup) | spike | frontend/syntax | see `issues/done/801-implement-accessorDeclarationEmitJs.md` |
| 802 | Implement Accessordeclarationemitvisibilityerrors (dup) | spike | frontend/syntax | see `issues/done/802-implement-accessorDeclarationEmitVisibilityErrors.md` |
| 803 | Implement Accessordeclarationorder (audit reopened #803) | spike | frontend/syntax | see `issues/done/803-implement-accessorDeclarationOrder.md` |
| 804 | Implement Accessorinambientcontextes (audit reopened #804) | spike | frontend/syntax | see `issues/done/804-implement-accessorInAmbientContextES.md` |
| 805 | Implement Accessorinferredreturntypeerrorinreturnstatement (dup) | spike | frontend/syntax | see `issues/done/805-implement-accessorInferredReturnTypeErrorInReturnStatement.md` |
| 806 | Implement Accessorparameteraccessibilitymodifier (audit reopened #806) | spike | frontend/syntax | see `issues/done/806-implement-accessorParameterAccessibilityModifier.md` |
| 807 | Implement Accessorwithlineterminator (dup) | spike | reference/triage | see `issues/done/807-implement-accessorWithLineTerminator.md` |
| 808 | Implement Accessorwithoutbody (audit reopened #808) | spike | frontend/syntax | see `issues/done/808-implement-accessorWithoutBody.md` |
| 809 | Implement Accessors (dup) | spike | frontend/syntax | see `issues/done/809-implement-accessors.md` |
| 810 | Implement Accessorsinambientcontext (audit reopened #810) | spike | frontend/syntax | see `issues/done/810-implement-accessorsInAmbientContext.md` |
| 811 | Implement Addmorecallsignaturestobasesignature (dup) | spike | frontend/syntax | see `issues/done/811-implement-addMoreCallSignaturesToBaseSignature.md` |
| 812 | Implement Aliasassignments (dup) | spike | frontend/syntax | see `issues/done/812-implement-aliasAssignments.md` |
| 813 | Implement Aliasbug (dup) | spike | frontend/syntax | see `issues/done/813-implement-aliasBug.md` |
| 814 | Implement Aliasdoesnotduplicatesignatures (dup) | spike | frontend/syntax | see `issues/done/814-implement-aliasDoesNotDuplicateSignatures.md` |
| 815 | Implement Aliaserrors (dup) | spike | frontend/syntax | see `issues/done/815-implement-aliasErrors.md` |
| 816 | Implement Aliasinaccessiblemodule (audit reopened #816) | spike | frontend/syntax | see `issues/done/816-implement-aliasInaccessibleModule.md` |
| 817 | Implement Aliasinstantiationexpressiongenericintersectionnocrash (dup) | spike | frontend/syntax | see `issues/done/817-implement-aliasInstantiationExpressionGenericIntersectionNoCrash.md` |
| 818 | Implement Aliasonmergedmoduleinterface (dup) | spike | frontend/syntax | see `issues/done/818-implement-aliasOnMergedModuleInterface.md` |
| 819 | Implement Aliasusageinaccessorsofclass (dup) | spike | frontend/syntax | see `issues/done/819-implement-aliasUsageInAccessorsOfClass.md` |
| 820 | Implement Aliasusageinarray (dup) | spike | frontend/syntax | see `issues/done/820-implement-aliasUsageInArray.md` |
| 821 | Implement Aliasusageinfunctionexpression (dup) | spike | frontend/syntax | see `issues/done/821-implement-aliasUsageInFunctionExpression.md` |
| 822 | Implement Aliasusageingenericfunction (dup) | spike | frontend/syntax | see `issues/done/822-implement-aliasUsageInGenericFunction.md` |
| 823 | Implement Aliasusageinindexerofclass (dup) | spike | frontend/syntax | see `issues/done/823-implement-aliasUsageInIndexerOfClass.md` |
| 824 | Implement Aliasusageinobjectliteral (dup) | spike | frontend/syntax | see `issues/done/824-implement-aliasUsageInObjectLiteral.md` |
| 825 | Implement Aliasusageinorexpression (dup) | spike | frontend/syntax | see `issues/done/825-implement-aliasUsageInOrExpression.md` |
| 826 | Implement Aliasusageintypeargumentofextendsclause (dup) | spike | frontend/syntax | see `issues/done/826-implement-aliasUsageInTypeArgumentOfExtendsClause.md` |
| 827 | Implement Aliasusageinvarassignment (dup) | spike | frontend/syntax | see `issues/done/827-implement-aliasUsageInVarAssignment.md` |
| 828 | Implement Aliasusedasnamevalue (dup) | spike | frontend/syntax | see `issues/done/828-implement-aliasUsedAsNameValue.md` |
| 829 | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer (dup) | spike | frontend/syntax | see `issues/done/829-implement-aliasWithInterfaceExportAssignmentUsedInVarInitializer.md` |
| 830 | Implement Aliasesinsystemmodule (dup) | spike | frontend/syntax | see `issues/done/830-implement-aliasesInSystemModule.md` |
| 831 | Implement Allowimportclausestomergewithtypes (dup) | spike | frontend/syntax | see `issues/done/831-implement-allowImportClausesToMergeWithTypes.md` |
| 832 | Implement Allowjsclassthistypecrash (dup) | spike | reference/triage | see `issues/done/832-implement-allowJsClassThisTypeCrash.md` |
| 833 | Implement Allowjscrossmonorepopackage (dup) | spike | frontend/syntax | see `issues/done/833-implement-allowJsCrossMonorepoPackage.md` |
| 834 | Implement Allowjscheckjstypeparameternocrash (dup) | spike | frontend/syntax | see `issues/done/834-implement-allowJscheckJsTypeParameterNoCrash.md` |
| 835 | Implement Allowsyntheticdefaultimports (dup) | spike | frontend/syntax | see `issues/done/835-implement-allowSyntheticDefaultImports.md` |
| 836 | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration (dup) | spike | frontend/syntax | see `issues/done/836-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md` |
| 837 | Implement Alwaysstrictmodule (audit reopened #837) | spike | frontend/syntax | see `issues/done/837-implement-alwaysStrictModule.md` |
| 838 | Implement Alwaysstrictnoimplicitusestrict (audit reopened #838) | spike | frontend/syntax | see `issues/done/838-implement-alwaysStrictNoImplicitUseStrict.md` |
| 839 | Implement Ambientclassdeclarationwithextends (audit reopened #839) | spike | frontend/syntax | see `issues/done/839-implement-ambientClassDeclarationWithExtends.md` |
| 840 | Implement Ambientclassdeclaredbeforebase (dup) | spike | frontend/syntax | see `issues/done/840-implement-ambientClassDeclaredBeforeBase.md` |
| 841 | Implement Ambientconstliterals (dup) | spike | frontend/syntax | see `issues/done/841-implement-ambientConstLiterals.md` |
| 842 | Implement Ambientenumelementinitializer (dup) | spike | frontend/syntax | see `issues/done/842-implement-ambientEnumElementInitializer.md` |
| 843 | Implement Ambienterrors | spike | runtime/builtins | see `issues/done/843-implement-ambientErrors.md` |
| 844 | Implement Ambientexportdefaulterrors (dup) | spike | frontend/syntax | see `issues/done/844-implement-ambientExportDefaultErrors.md` |
| 845 | Implement Ambientexternalmoduleinanotherexternalmodule (dup) | spike | frontend/syntax | see `issues/done/845-implement-ambientExternalModuleInAnotherExternalModule.md` |
| 846 | Implement Ambientexternalmodulereopen (audit reopened #846) | spike | frontend/syntax | see `issues/done/846-implement-ambientExternalModuleReopen.md` |
| 847 | Implement Ambientexternalmodulewithinternalimportdeclaration (dup) | spike | frontend/syntax | see `issues/done/847-implement-ambientExternalModuleWithInternalImportDeclaration.md` |
| 848 | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration (audit reopened #848) | spike | frontend/syntax | see `issues/done/848-implement-ambientExternalModuleWithRelativeExternalImportDeclaration.md` |
| 849 | Implement Ambientexternalmodulewithrelativemodulename (audit reopened #849) | spike | frontend/syntax | see `issues/done/849-implement-ambientExternalModuleWithRelativeModuleName.md` |
| 850 | Implement Ambientexternalmodulewithoutinternalimportdeclaration (dup) | spike | frontend/syntax | see `issues/done/850-implement-ambientExternalModuleWithoutInternalImportDeclaration.md` |
| 851 | Implement Ambientfundule (audit reopened #851) | spike | frontend/syntax | see `issues/done/851-implement-ambientFundule.md` |
| 852 | Implement Ambientmoduleexports (dup) | spike | frontend/syntax | see `issues/done/852-implement-ambientModuleExports.md` |
| 853 | Implement Ambientmodulewithclassdeclarationwithextends (audit reopened #853) | spike | frontend/syntax | see `issues/done/853-implement-ambientModuleWithClassDeclarationWithExtends.md` |
| 854 | Implement Ambientmodulewithtemplateliterals (dup) | spike | frontend/syntax | see `issues/done/854-implement-ambientModuleWithTemplateLiterals.md` |
| 855 | Implement Ambientmodules (dup) | spike | frontend/syntax | see `issues/done/855-implement-ambientModules.md` |
| 856 | Implement Ambientnamerestrictions (audit reopened #856) | spike | frontend/syntax | see `issues/done/856-implement-ambientNameRestrictions.md` |
| 857 | Implement Ambientrequirefunction (dup) | spike | frontend/syntax | see `issues/done/857-implement-ambientRequireFunction.md` |
| 858 | Implement Ambientstatement (audit reopened #858) | spike | frontend/syntax | see `issues/done/858-implement-ambientStatement.md` |
| 859 | Implement Ambientwithstatements (audit reopened #859) | spike | frontend/syntax | see `issues/done/859-implement-ambientWithStatements.md` |
| 860 | Implement Ambiguouscallswherereturntypesagree (dup) | spike | frontend/syntax | see `issues/done/860-implement-ambiguousCallsWhereReturnTypesAgree.md` |
| 861 | Implement Ambiguousgenericassertion (dup) | spike | frontend/syntax | see `issues/done/861-implement-ambiguousGenericAssertion.md` |
| 862 | Implement Ambiguousoverloadresolution (dup) | spike | frontend/resolver | see `issues/done/862-implement-ambiguousOverloadResolution.md` |
| 863 | Implement Amddeclarationemitnoextradeclare (dup) | spike | frontend/syntax | see `issues/done/863-implement-amdDeclarationEmitNoExtraDeclare.md` |
| 864 | Implement Amddependencycomment (dup) | spike | frontend/syntax | see `issues/done/864-implement-amdDependencyComment.md` |
| 865 | Implement Amddependencycommentname (dup) | spike | frontend/syntax | see `issues/done/865-implement-amdDependencyCommentName.md` |
| 866 | Implement Amdlikeinputdeclarationemit (dup) | spike | frontend/syntax | see `issues/done/866-implement-amdLikeInputDeclarationEmit.md` |
| 867 | Implement Amdmodulebundlenoduplicatedeclarationemitcomments (dup) | spike | frontend/syntax | see `issues/done/867-implement-amdModuleBundleNoDuplicateDeclarationEmitComments.md` |
| 868 | Implement Amdmoduleconstenumusage (dup) | spike | frontend/syntax | see `issues/done/868-implement-amdModuleConstEnumUsage.md` |
| 869 | Implement Amdmodulename (dup) | spike | frontend/syntax | see `issues/done/869-implement-amdModuleName.md` |
| 870 | Implement Anonclassdeclarationemitisanon (dup) | spike | frontend/syntax | see `issues/done/870-implement-anonClassDeclarationEmitIsAnon.md` |
| 871 | Implement Anonterface (dup) | spike | frontend/syntax | see `issues/done/871-implement-anonterface.md` |
| 872 | Implement Anonymousclassdeclarationdoesntprintwithreadonly (dup) | spike | frontend/syntax | see `issues/done/872-implement-anonymousClassDeclarationDoesntPrintWithReadonly.md` |
| 873 | Implement Anonymousclassexpression (audit reopened #873) | spike | frontend/syntax | see `issues/done/873-implement-anonymousClassExpression.md` |
| 874 | Implement Anonymousmodules (audit reopened #874) | spike | frontend/syntax | see `issues/done/874-implement-anonymousModules.md` |
| 875 | Implement Anyandunknownhavefalsycomponents (dup) | spike | frontend/resolver | see `issues/done/875-implement-anyAndUnknownHaveFalsyComponents.md` |
| 876 | Implement Anyasreturntypefornewoncall (audit reopened #876) | spike | frontend/syntax | see `issues/done/876-implement-anyAsReturnTypeForNewOnCall.md` |
| 877 | Implement Anydeclare (audit reopened #877) | spike | frontend/syntax | see `issues/done/877-implement-anyDeclare.md` |
| 878 | Implement Anyidenticaltoitself (dup) | spike | frontend/syntax | see `issues/done/878-implement-anyIdenticalToItself.md` |
| 879 | Implement Anyinferenceanonymousfunctions (dup) | spike | frontend/syntax | see `issues/done/879-implement-anyInferenceAnonymousFunctions.md` |
| 880 | Implement Argsinscope (dup) | spike | frontend/syntax | see `issues/done/880-implement-argsInScope.md` |
| 881 | Implement Arguments (dup) | spike | frontend/syntax | see `issues/done/881-implement-arguments.md` |
| 882 | Implement Argumentsaspropertyname Arguments Object (dup) | spike | frontend/syntax | see `issues/done/882-implement-argumentsAsPropertyName-arguments-object.md` |
| 883 | Implement Argumentsaspropertyname Name Resolution (dup) | spike | frontend/resolver | see `issues/done/883-implement-argumentsAsPropertyName-name-resolution.md` |
| 884 | Implement Argumentsbindstofunctionscopeargumentlist (dup) | spike | frontend/resolver | see `issues/done/884-implement-argumentsBindsToFunctionScopeArgumentList.md` |
| 885 | Implement Argumentsobjectcreatesrestforjs (dup) | spike | frontend/syntax | see `issues/done/885-implement-argumentsObjectCreatesRestForJs.md` |
| 886 | Implement Argumentsobjectiterator (dup) | spike | frontend/syntax | see `issues/done/886-implement-argumentsObjectIterator.md` |
| 887 | Implement Argumentspropertynameinjsmode (dup) | spike | frontend/syntax | see `issues/done/887-implement-argumentsPropertyNameInJsMode.md` |
| 888 | Implement Argumentsreferenceinconstructor Arguments Object (dup) | spike | frontend/syntax | see `issues/done/888-implement-argumentsReferenceInConstructor-arguments-object.md` |
| 889 | Implement Argumentsreferenceinconstructor Name Resolution (dup) | spike | frontend/resolver | see `issues/done/889-implement-argumentsReferenceInConstructor-name-resolution.md` |
| 890 | Implement Argumentsreferenceinfunction (dup) | spike | frontend/syntax | see `issues/done/890-implement-argumentsReferenceInFunction.md` |
| 891 | Implement Argumentsreferenceinmethod Arguments Object (dup) | spike | frontend/syntax | see `issues/done/891-implement-argumentsReferenceInMethod-arguments-object.md` |
| 892 | Implement Argumentsreferenceinmethod Name Resolution (dup) | spike | frontend/resolver | see `issues/done/892-implement-argumentsReferenceInMethod-name-resolution.md` |
| 893 | Implement Argumentsreferenceinobjectliteral (dup) | spike | frontend/syntax | see `issues/done/893-implement-argumentsReferenceInObjectLiteral.md` |
| 894 | Implement Argumentsusedinclassfieldinitializerorstaticinitializationblock (dup) | spike | frontend/syntax | see `issues/done/894-implement-argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.md` |
| 895 | Implement Argumentsusedinobjectliteralproperty (dup) | spike | frontend/syntax | see `issues/done/895-implement-argumentsUsedInObjectLiteralProperty.md` |
| 896 | Implement Arithassigntyping (dup) | spike | frontend/syntax | see `issues/done/896-implement-arithAssignTyping.md` |
| 897 | Implement Arrayassignmenttest Import Export (dup) | spike | frontend/syntax | see `issues/done/897-implement-arrayAssignmentTest-import-export.md` |
| 898 | Implement Arrayassignmenttest Parser Syntax (dup) | spike | frontend/syntax | see `issues/done/898-implement-arrayAssignmentTest-parser-syntax.md` |
| 899 | Implement Arrayaugment (dup) | spike | reference/triage | see `issues/done/899-implement-arrayAugment.md` |
| 900 | Implement Arraybestcommontypes (dup) | spike | frontend/syntax | see `issues/done/900-implement-arrayBestCommonTypes.md` |
| 901 | Implement Arraybindingpatternomittedexpressions (dup) | spike | frontend/syntax | see `issues/done/901-implement-arrayBindingPatternOmittedExpressions.md` |
| 902 | Implement Arraybufferisviewnarrowstype (dup) | spike | frontend/resolver | see `issues/done/902-implement-arrayBufferIsViewNarrowsType.md` |
| 903 | Implement Arraycast (dup) | spike | frontend/syntax | see `issues/done/903-implement-arrayCast.md` |
| 904 | Implement Arrayconcat (dup) | spike | frontend/syntax | see `issues/done/904-implement-arrayConcat.md` |
| 905 | Implement Arrayconcatmap (dup) | spike | frontend/syntax | see `issues/done/905-implement-arrayConcatMap.md` |
| 906 | Implement Arrayconstructors (dup) | spike | frontend/syntax | see `issues/done/906-implement-arrayConstructors.md` |
| 907 | Implement Arraydestructuringinswitch (dup) | spike | frontend/syntax | see `issues/done/907-implement-arrayDestructuringInSwitch.md` |
| 908 | Implement Arrayevery (dup) | spike | frontend/syntax | see `issues/done/908-implement-arrayEvery.md` |
| 909 | Implement Arrayfakeflatnocrashinferencedeclarations (dup) | spike | runtime/builtins | see `issues/done/909-implement-arrayFakeFlatNoCrashInferenceDeclarations.md` |
| 910 | Implement Arrayfilter (dup) | spike | runtime/builtins | see `issues/done/910-implement-arrayFilter.md` |
| 911 | Implement Arrayfind (dup) | spike | frontend/syntax | see `issues/done/911-implement-arrayFind.md` |
| 912 | Implement Arrayflatmap (dup) | spike | frontend/syntax | see `issues/done/912-implement-arrayFlatMap.md` |
| 913 | Implement Arrayflatnocrashinference (dup) | spike | frontend/syntax | see `issues/done/913-implement-arrayFlatNoCrashInference.md` |
| 914 | Implement Arrayflatnocrashinferencedeclarations (dup) | spike | frontend/syntax | see `issues/done/914-implement-arrayFlatNoCrashInferenceDeclarations.md` |
| 915 | Implement Arrayfrom (dup) | spike | runtime/builtins | see `issues/done/915-implement-arrayFrom.md` |
| 916 | Implement Arrayfromasync (dup) | spike | reference/triage | see `issues/done/916-implement-arrayFromAsync.md` |
| 917 | Implement Arrayindexwitharrayfails (dup) | spike | frontend/resolver | see `issues/done/917-implement-arrayIndexWithArrayFails.md` |
| 918 | Implement Arrayiterationlibes (dup) | spike | frontend/resolver | see `issues/done/918-implement-arrayIterationLibES.md` |
| 919 | Implement Arrayliteralandarrayconstructorequivalence (dup) | spike | frontend/resolver | see `issues/done/919-implement-arrayLiteralAndArrayConstructorEquivalence.md` |
| 920 | Implement Arrayliteralcomments (dup) | spike | frontend/syntax | see `issues/done/920-implement-arrayLiteralComments.md` |
| 921 | Implement Arrayliteralcontextualtype (dup) | spike | frontend/semantics | see `issues/done/921-implement-arrayLiteralContextualType.md` |
| 922 | Implement Arrayliteraltypeinference (dup) | spike | frontend/syntax | see `issues/done/922-implement-arrayLiteralTypeInference.md` |
| 923 | Implement Arrayofexportedclass (dup) | spike | frontend/syntax | see `issues/done/923-implement-arrayOfExportedClass.md` |
| 924 | Implement Arrayofsubtypeisassignabletoreadonlyarray (dup) | spike | frontend/semantics | see `issues/done/924-implement-arrayOfSubtypeIsAssignableToReadonlyArray.md` |
| 925 | Implement Arrayreferencewithouttypeargs (dup) | spike | frontend/syntax | see `issues/done/925-implement-arrayReferenceWithoutTypeArgs.md` |
| 926 | Implement Arraysigchecking (dup) | spike | frontend/syntax | see `issues/done/926-implement-arraySigChecking.md` |
| 927 | Implement Arrayslice (dup) | spike | frontend/syntax | see `issues/done/927-implement-arraySlice.md` |
| 928 | Implement Arraytolocalestringes Name Resolution (dup) | spike | frontend/resolver | see `issues/done/928-implement-arrayToLocaleStringES-name-resolution.md` |
| 929 | Implement Arraytolocalestringes Unknown Unsupported (dup) | spike | frontend/syntax | see `issues/done/929-implement-arrayToLocaleStringES-unknown-unsupported.md` |
| 930 | Implement Arraytypeinsignatureofinterfaceandclass (dup) | spike | frontend/syntax | see `issues/done/930-implement-arrayTypeInSignatureOfInterfaceAndClass.md` |
| 931 | Implement Arrayconcat (dup) | spike | runtime/builtins | see `issues/done/931-implement-arrayconcat.md` |
| 932 | Implement Arrowfunctioninconstructorargument (dup) | spike | frontend/syntax | see `issues/done/932-implement-arrowFunctionInConstructorArgument.md` |
| 933 | Implement Arrowfunctioninexpressionstatement (dup) | spike | frontend/syntax | see `issues/done/933-implement-arrowFunctionInExpressionStatement.md` |
| 934 | Implement Arrowfunctionmissingcurlywithsemicolon (dup) | spike | frontend/syntax | see `issues/done/934-implement-arrowFunctionMissingCurlyWithSemicolon.md` |
| 935 | Implement Arrowfunctionparsingdoesnotconfuseparenthesizedobjectforarrowhead (dup) | spike | frontend/syntax | see `issues/done/935-implement-arrowFunctionParsingDoesNotConfuseParenthesizedObjectForArrowHead.md` |
| 936 | Implement Arrowfunctionparsinggenericinobject (dup) | spike | frontend/syntax | see `issues/done/936-implement-arrowFunctionParsingGenericInObject.md` |
| 937 | Implement Arrowfunctionwithobjectliteralbody (dup) | spike | frontend/syntax | see `issues/done/937-implement-arrowFunctionWithObjectLiteralBody.md` |
| 938 | Implement Arrowfunctionsmissingtokens (dup) | spike | frontend/syntax | see `issues/done/938-implement-arrowFunctionsMissingTokens.md` |
| 939 | Implement Asiabstract (dup) | spike | frontend/syntax | see `issues/done/939-implement-asiAbstract.md` |
| 940 | Implement Asiambientfunctiondeclaration (dup) | spike | frontend/syntax | see `issues/done/940-implement-asiAmbientFunctionDeclaration.md` |
| 941 | Implement Asiarith (dup) | spike | frontend/syntax | see `issues/done/941-implement-asiArith.md` |
| 942 | Implement Asibreak (dup) | spike | frontend/syntax | see `issues/done/942-implement-asiBreak.md` |
| 943 | Implement Asicontinue (dup) | spike | frontend/syntax | see `issues/done/943-implement-asiContinue.md` |
| 944 | Implement Asiines (dup) | spike | frontend/syntax | see `issues/done/944-implement-asiInES.md` |
| 945 | Implement Asipublicprivateprotected (dup) | spike | frontend/semantics | see `issues/done/945-implement-asiPublicPrivateProtected.md` |
| 946 | Implement Asireturn (dup) | spike | reference/triage | see `issues/done/946-implement-asiReturn.md` |
| 947 | Implement Assertinwrapsometypeparameter (dup) | spike | frontend/semantics | see `issues/done/947-implement-assertInWrapSomeTypeParameter.md` |
| 948 | Implement Assertionfunctionwildcardimport (dup) | spike | frontend/syntax | see `issues/done/948-implement-assertionFunctionWildcardImport.md` |
| 949 | Implement Assertionfunctionscannarrowbydiscriminant (dup) | spike | frontend/semantics | see `issues/done/949-implement-assertionFunctionsCanNarrowByDiscriminant.md` |
| 950 | Implement Assign (dup) | spike | frontend/syntax | see `issues/done/950-implement-assign.md` |
| 951 | Implement Assigntoenum (dup) | spike | frontend/syntax | see `issues/done/951-implement-assignToEnum.md` |
| 952 | Implement Assigntoexistingclass (dup) | spike | frontend/syntax | see `issues/done/952-implement-assignToExistingClass.md` |
| 953 | Implement Assigntofn (dup) | spike | frontend/syntax | see `issues/done/953-implement-assignToFn.md` |
| 954 | Implement Assigntoinvalidlhs (dup) | spike | frontend/syntax | see `issues/done/954-implement-assignToInvalidLHS.md` |
| 955 | Implement Assigntomodule (dup) | spike | frontend/syntax | see `issues/done/955-implement-assignToModule.md` |
| 956 | Implement Assigntoobjecttypewithprototypeproperty (dup) | spike | frontend/resolver | see `issues/done/956-implement-assignToObjectTypeWithPrototypeProperty.md` |
| 957 | Implement Assigntoprototype (dup) | spike | frontend/resolver | see `issues/done/957-implement-assignToPrototype.md` |
| 958 | Implement Assigningfromobjecttoanythingelse (dup) | spike | frontend/resolver | see `issues/done/958-implement-assigningFromObjectToAnythingElse.md` |
| 959 | Implement Assigningfunctiontotupleissueserror (dup) | spike | frontend/resolver | see `issues/done/959-implement-assigningFunctionToTupleIssuesError.md` |
| 960 | Implement Assignmentcompat (dup) | spike | frontend/resolver | see `issues/done/960-implement-assignmentCompat.md` |
| 961 | Implement Assignmentcompatbug (dup) | spike | frontend/semantics | see `issues/done/961-implement-assignmentCompatBug.md` |
| 962 | Implement Assignmentcompatforenums (dup) | spike | frontend/semantics | see `issues/done/962-implement-assignmentCompatForEnums.md` |
| 963 | Implement Assignmentcompatfunctionswithoptionalargs (dup) | spike | frontend/semantics | see `issues/done/963-implement-assignmentCompatFunctionsWithOptionalArgs.md` |
| 964 | Implement Assignmentcompatinterfacewithstringindexsignature (dup) | spike | frontend/semantics | see `issues/done/964-implement-assignmentCompatInterfaceWithStringIndexSignature.md` |
| 965 | Implement Assignmentcompatonnew (dup) | spike | frontend/resolver | see `issues/done/965-implement-assignmentCompatOnNew.md` |
| 966 | Implement Assignmentcompatwithoverloads (dup) | spike | frontend/semantics | see `issues/done/966-implement-assignmentCompatWithOverloads.md` |
| 967 | Implement Assignmentcompatability Import Export (dup) | spike | frontend/syntax | see `issues/done/967-implement-assignmentCompatability-import-export.md` |
| 968 | Implement Assignmentcompatability Name Resolution (dup) | spike | frontend/resolver | see `issues/done/968-implement-assignmentCompatability-name-resolution.md` |
| 969 | Implement Assignmentcompatability Parser Syntax (dup) | spike | frontend/semantics | see `issues/done/969-implement-assignmentCompatability-parser-syntax.md` |
| 970 | Implement Assignmentindexedtoprimitives (dup) | spike | frontend/syntax | see `issues/done/970-implement-assignmentIndexedToPrimitives.md` |
| 971 | Implement Assignmentnestedinliterals (dup) | spike | reference/triage | see `issues/done/971-implement-assignmentNestedInLiterals.md` |
| 972 | Implement Assignmentnonobjecttypeconstraints (dup) | spike | frontend/syntax | see `issues/done/972-implement-assignmentNonObjectTypeConstraints.md` |
| 973 | Implement Assignmentrestelementwitherrorsourcetype (dup) | spike | frontend/resolver | see `issues/done/973-implement-assignmentRestElementWithErrorSourceType.md` |
| 974 | Implement Assignmentstricterconstraints (dup) | spike | frontend/semantics | see `issues/done/974-implement-assignmentStricterConstraints.md` |
| 975 | Implement Assignmenttoanyarrayrestparameters (dup) | spike | frontend/semantics | see `issues/done/975-implement-assignmentToAnyArrayRestParameters.md` |
| 976 | Implement Assignmenttoconditionalbrandedstringtemplateormapping (dup) | spike | frontend/syntax | see `issues/done/976-implement-assignmentToConditionalBrandedStringTemplateOrMapping.md` |
| 977 | Implement Assignmenttoexpandingarraytype (dup) | spike | frontend/syntax | see `issues/done/977-implement-assignmentToExpandingArrayType.md` |
| 978 | Implement Assignmenttofunction (dup) | spike | frontend/syntax | see `issues/done/978-implement-assignmentToFunction.md` |
| 979 | Implement Assignmenttoinstantiationexpression (dup) | spike | frontend/syntax | see `issues/done/979-implement-assignmentToInstantiationExpression.md` |
| 980 | Implement Assignmenttoobjectandfunction (dup) | spike | frontend/syntax | see `issues/done/980-implement-assignmentToObjectAndFunction.md` |
| 981 | Implement Assignmenttoparenthesizedexpression (dup) | spike | frontend/syntax | see `issues/done/981-implement-assignmentToParenthesizedExpression.md` |
| 982 | Implement Assignmenttoreferencetypes (dup) | spike | frontend/syntax | see `issues/done/982-implement-assignmentToReferenceTypes.md` |
| 983 | Implement Asyncarrowinclasses (dup) | spike | runtime/builtins | see `issues/done/983-implement-asyncArrowInClassES.md` |
| 984 | Implement Asyncawaitwithcapturedblockscopevar (dup) | spike | reference/triage | see `issues/done/984-implement-asyncAwaitWithCapturedBlockScopeVar.md` |
| 985 | Implement Asyncfunctioncontextuallytypedreturns (dup) | spike | frontend/syntax | see `issues/done/985-implement-asyncFunctionContextuallyTypedReturns.md` |
| 986 | Implement Asyncfunctionnoreturntype (dup) | spike | frontend/syntax | see `issues/done/986-implement-asyncFunctionNoReturnType.md` |
| 987 | Implement Asyncfunctionreturnexpressionerrorspans (dup) | spike | reference/triage | see `issues/done/987-implement-asyncFunctionReturnExpressionErrorSpans.md` |
| 988 | Implement Asyncfunctionreturntype Parser Syntax (dup) | spike | runtime/builtins | see `issues/done/988-implement-asyncFunctionReturnType-parser-syntax.md` |
| 989 | Implement Asyncfunctionreturntype Runtime Subset (dup) | spike | reference/triage | see `issues/done/989-implement-asyncFunctionReturnType-runtime-subset.md` |
| 990 | Implement Asyncfunctiontempvariablescoping (dup) | spike | frontend/syntax | see `issues/done/990-implement-asyncFunctionTempVariableScoping.md` |
| 991 | Implement Asyncfunctionwithforstatementnoinitializer (dup) | spike | reference/triage | see `issues/done/991-implement-asyncFunctionWithForStatementNoInitializer.md` |
| 992 | Implement Asyncfunctionsacrossfiles (dup) | spike | frontend/syntax | see `issues/done/992-implement-asyncFunctionsAcrossFiles.md` |
| 993 | Implement Asyncfunctionsandstrictnullchecks (dup) | spike | frontend/syntax | see `issues/done/993-implement-asyncFunctionsAndStrictNullChecks.md` |
| 994 | Implement Asynciife (dup) | spike | frontend/syntax | see `issues/done/994-implement-asyncIIFE.md` |
| 995 | Implement Asyncimportnestedyield (dup) | spike | reference/triage | see `issues/done/995-implement-asyncImportNestedYield.md` |
| 996 | Implement Asynciteratorextraparameters (dup) | spike | runtime/builtins | see `issues/done/996-implement-asyncIteratorExtraParameters.md` |
| 997 | Implement Asyncyieldstarcontextualtype (dup) | spike | frontend/semantics | see `issues/done/997-implement-asyncYieldStarContextualType.md` |
| 998 | Implement Augmentexportequals (dup) | spike | frontend/syntax | see `issues/done/998-implement-augmentExportEquals.md` |
| 999 | Implement Augmentedclasswithprototypepropertyonmodule (dup) | spike | frontend/syntax | see `issues/done/999-implement-augmentedClassWithPrototypePropertyOnModule.md` |
| 1000 | Implement Augmentedtypesclass (dup) | spike | frontend/resolver | see `issues/done/1000-implement-augmentedTypesClass.md` |
| 1001 | Implement Augmentedtypesenum Import Export (dup) | spike | frontend/syntax | see `issues/done/1001-implement-augmentedTypesEnum-import-export.md` |
| 1001e | Annex B eval-code function declaration residuals (existing-binding/no-skip/skip-early-err patterns) (audit reopened #1001e) | feature | frontend/semantics | see `issues/done/1001e-eval-annexb-function-existing-binding-residuals.md` |
| 1002 | Implement Augmentedtypesenum Parser Syntax (dup) | spike | frontend/resolver | see `issues/done/1002-implement-augmentedTypesEnum-parser-syntax.md` |
| 1003 | Implement Augmentedtypesexternalmodule (dup) | spike | frontend/syntax | see `issues/done/1003-implement-augmentedTypesExternalModule.md` |
| 1004 | Implement Augmentedtypesfunction (dup) | spike | frontend/resolver | see `issues/done/1004-implement-augmentedTypesFunction.md` |
| 1005 | Implement Augmentedtypesinterface (dup) | spike | frontend/resolver | see `issues/done/1005-implement-augmentedTypesInterface.md` |
| 1006 | Implement Augmentedtypesmodules (dup) | spike | frontend/syntax | see `issues/done/1006-implement-augmentedTypesModules.md` |
| 1007 | Implement Augmentedtypesvar (dup) | spike | frontend/resolver | see `issues/done/1007-implement-augmentedTypesVar.md` |
| 1008 | Implement Autoasiforstaticsinclassdeclaration (dup) | spike | frontend/syntax | see `issues/done/1008-implement-autoAsiForStaticsInClassDeclaration.md` |
| 1009 | Implement Autolift (dup) | spike | frontend/syntax | see `issues/done/1009-implement-autoLift.md` |
| 1010 | Implement Autotypeassignedusingdestructuringfromnevernocrash (dup) | spike | frontend/resolver | see `issues/done/1010-implement-autoTypeAssignedUsingDestructuringFromNeverNoCrash.md` |
| 1011 | Implement Autolift (dup) | spike | frontend/syntax | see `issues/done/1011-implement-autolift.md` |
| 1012 | Implement Autonumberinginenums | spike | frontend/syntax | see `issues/done/1012-implement-autonumberingInEnums.md` |
| 1013 | Implement Avoid | spike | frontend/syntax | see `issues/done/1013-implement-avoid.md` |
| 1014 | Implement Avoidcyclewithvoidexpressionreturnedfromarrow | spike | frontend/syntax | see `issues/done/1014-implement-avoidCycleWithVoidExpressionReturnedFromArrow.md` |
| 1015 | Implement Avoidnarrowingusingconstvariablefrombindingelementwithliteralinitializer | spike | frontend/syntax | see `issues/done/1015-implement-avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer.md` |
| 1016 | Implement Awaitcallexpressioninsyncfunction | spike | reference/triage | see `issues/done/1016-implement-awaitCallExpressionInSyncFunction.md` |
| 1017 | Implement Awaitexpressioninnercommentemit | spike | reference/triage | see `issues/done/1017-implement-awaitExpressionInnerCommentEmit.md` |
| 1018 | Implement Awaitinclassinasyncfunction | spike | reference/triage | see `issues/done/1018-implement-awaitInClassInAsyncFunction.md` |
| 1019 | Implement Awaitinnonasyncfunction | spike | reference/triage | see `issues/done/1019-implement-awaitInNonAsyncFunction.md` |
| 1020 | Implement Awaitliteralvalues | spike | reference/triage | see `issues/done/1020-implement-awaitLiteralValues.md` |
| 1021 | Implement Awaitunionpromise | spike | reference/triage | see `issues/done/1021-implement-awaitUnionPromise.md` |
| 1022 | Implement Awaitedtype | spike | reference/triage | see `issues/done/1022-implement-awaitedType.md` |
| 1023 | Implement Awaitedtypecrash | spike | reference/triage | see `issues/done/1023-implement-awaitedTypeCrash.md` |
| 1024 | Implement Awaitedtypenolib | spike | runtime/builtins | see `issues/done/1024-implement-awaitedTypeNoLib.md` |
| 1025 | Implement Awaitedtypestrictnull | spike | runtime/builtins | see `issues/done/1025-implement-awaitedTypeStrictNull.md` |
| 1026 | Implement Badarrayindex | spike | frontend/syntax | see `issues/done/1026-implement-badArrayIndex.md` |
| 1027 | Implement Badarraysyntax | spike | frontend/syntax | see `issues/done/1027-implement-badArraySyntax.md` |
| 1028 | Implement Badexternalmodulereference | spike | frontend/syntax | see `issues/done/1028-implement-badExternalModuleReference.md` |
| 1029 | Implement Badinferencelowerprioritythangoodinference | spike | frontend/semantics | see `issues/done/1029-implement-badInferenceLowerPriorityThanGoodInference.md` |
| 1030 | Implement Badoverloaderror | spike | frontend/syntax | see `issues/done/1030-implement-badOverloadError.md` |
| 1031 | Implement Badthisbinding | spike | frontend/syntax | see `issues/done/1031-implement-badThisBinding.md` |
| 1032 | Implement Banginmodulename | spike | frontend/syntax | see `issues/done/1032-implement-bangInModuleName.md` |
| 1033 | Implement Basecheck | spike | frontend/resolver | see `issues/done/1033-implement-baseCheck.md` |
| 1034 | Implement Baseclassimprovedmismatcherrors | spike | frontend/syntax | see `issues/done/1034-implement-baseClassImprovedMismatchErrors.md` |
| 1035 | Implement Baseconstraintofdecorator | spike | frontend/syntax | see `issues/done/1035-implement-baseConstraintOfDecorator.md` |
| 1036 | Implement Baseexpressiontypeparameters | spike | frontend/syntax | see `issues/done/1036-implement-baseExpressionTypeParameters.md` |
| 1037 | Implement Baseindexsignatureresolution | spike | frontend/syntax | see `issues/done/1037-implement-baseIndexSignatureResolution.md` |
| 1038 | Implement Basetypeafterderivedtype | spike | backend-wasm | see `issues/done/1038-implement-baseTypeAfterDerivedType.md` |
| 1039 | Implement Basetypeorderchecking | spike | frontend/syntax | see `issues/done/1039-implement-baseTypeOrderChecking.md` |
| 1040 | Implement Basetypeprivatememberclash | spike | frontend/resolver | see `issues/done/1040-implement-baseTypePrivateMemberClash.md` |
| 1041 | Implement Basetypewrappinginstantiationchain | spike | frontend/syntax | see `issues/done/1041-implement-baseTypeWrappingInstantiationChain.md` |
| 1042 | Implement Bases | spike | frontend/syntax | see `issues/done/1042-implement-bases.md` |
| 1043 | Implement Bestchoicetype | spike | frontend/syntax | see `issues/done/1043-implement-bestChoiceType.md` |
| 1044 | Implement Bestcommontypewithcontextualtyping | spike | frontend/resolver | see `issues/done/1044-implement-bestCommonTypeWithContextualTyping.md` |
| 1045 | Implement Bettererrorforaccidentalcall | spike | frontend/syntax | see `issues/done/1045-implement-betterErrorForAccidentalCall.md` |
| 1046 | Implement Bigintwithtargetes | spike | runtime/builtins | see `issues/done/1046-implement-bigIntWithTargetES.md` |
| 1047 | Implement Bigintwithtargetlessthanes | spike | runtime/builtins | see `issues/done/1047-implement-bigIntWithTargetLessThanES.md` |
| 1048 | Implement Bigint | spike | frontend/resolver | see `issues/done/1048-implement-bigint.md` |
| 1049 | Implement Bigintambientminimal | spike | runtime/builtins | see `issues/done/1049-implement-bigintAmbientMinimal.md` |
| 1050 | Implement Bigintarbirtraryidentifier | spike | runtime/builtins | see `issues/done/1050-implement-bigintArbirtraryIdentifier.md` |
| 1051 | Implement Bigintindex | spike | frontend/resolver | see `issues/done/1051-implement-bigintIndex.md` |
| 1052 | Implement Bigintpropertyname | spike | runtime/builtins | see `issues/done/1052-implement-bigintPropertyName.md` |
| 1053 | Implement Bigintwithlib | spike | runtime/builtins | see `issues/done/1053-implement-bigintWithLib.md` |
| 1054 | Implement Bigintwithoutlib | spike | runtime/builtins | see `issues/done/1054-implement-bigintWithoutLib.md` |
| 1055 | Implement Binaryarithmatic | spike | frontend/syntax | see `issues/done/1055-implement-binaryArithmatic.md` |
| 1056 | Implement Binaryarithmeticcontrolflowgraphnottoolarge | spike | frontend/syntax | see `issues/done/1056-implement-binaryArithmeticControlFlowGraphNotTooLarge.md` |
| 1057 | Implement Bind | spike | frontend/syntax | see `issues/done/1057-implement-bind.md` |
| 1058 | Implement Binderbinaryexpressionstress | spike | reference/triage | see `issues/done/1058-implement-binderBinaryExpressionStress.md` |
| 1059 | Implement Binderbinaryexpressionstressjs | spike | reference/triage | see `issues/done/1059-implement-binderBinaryExpressionStressJs.md` |
| 1060 | Implement Bindingpatterncannotbeonlyinferencesource | spike | reference/triage | see `issues/done/1060-implement-bindingPatternCannotBeOnlyInferenceSource.md` |
| 1061 | Implement Bindingpatterncontextualtypedoesnotcausewidening | spike | frontend/resolver | see `issues/done/1061-implement-bindingPatternContextualTypeDoesNotCauseWidening.md` |
| 1062 | Implement Bindingpatterninparameter | spike | frontend/syntax | see `issues/done/1062-implement-bindingPatternInParameter.md` |
| 1063 | Implement Bindingpatternomittedexpressionnesting | spike | frontend/syntax | see `issues/done/1063-implement-bindingPatternOmittedExpressionNesting.md` |
| 1064 | Implement Binopassignmentshouldhavetype | spike | frontend/syntax | see `issues/done/1064-implement-binopAssignmentShouldHaveType.md` |
| 1065 | Implement Bitwisecompoundassignmentoperators | spike | frontend/syntax | see `issues/done/1065-implement-bitwiseCompoundAssignmentOperators.md` |
| 1066 | Implement Blockscopedbindingcapturethisinfunction | spike | reference/triage | see `issues/done/1066-implement-blockScopedBindingCaptureThisInFunction.md` |
| 1067 | Implement Blockscopedbindingusedbeforedef | spike | frontend/resolver | see `issues/done/1067-implement-blockScopedBindingUsedBeforeDef.md` |
| 1068 | Implement Blockscopedbindingsreassignedinloop Name Resolution | spike | frontend/resolver | see `issues/done/1068-implement-blockScopedBindingsReassignedInLoop-name-resolution.md` |
| 1069 | Implement Blockscopedbindingsreassignedinloop Scope Analysis | spike | frontend/resolver | see `issues/done/1069-implement-blockScopedBindingsReassignedInLoop-scope-analysis.md` |
| 1070 | Implement Blockscopedenumvariablesusebeforedef Enum | spike | frontend/syntax | see `issues/done/1070-implement-blockScopedEnumVariablesUseBeforeDef-enum.md` |
| 1071 | Implement Blockscopedenumvariablesusebeforedef Import Export | spike | frontend/syntax | see `issues/done/1071-implement-blockScopedEnumVariablesUseBeforeDef-import-export.md` |
| 1072 | Implement Blockscopedfunctiondeclarationes | spike | frontend/resolver | see `issues/done/1072-implement-blockScopedFunctionDeclarationES.md` |
| 1073 | Implement Blockscopedfunctiondeclarationinstrictclass | spike | frontend/resolver | see `issues/done/1073-implement-blockScopedFunctionDeclarationInStrictClass.md` |
| 1074 | Implement Blockscopedfunctiondeclarationinstrictmodule | spike | frontend/syntax | see `issues/done/1074-implement-blockScopedFunctionDeclarationInStrictModule.md` |
| 1075 | Implement Blockscopedfunctiondeclarationstrictes | spike | frontend/resolver | see `issues/done/1075-implement-blockScopedFunctionDeclarationStrictES.md` |
| 1076 | Implement Blockscopednamespacedifferentfile | spike | frontend/syntax | see `issues/done/1076-implement-blockScopedNamespaceDifferentFile.md` |
| 1077 | Implement Blockscopedsamenamefunctiondeclarationes | spike | reference/triage | see `issues/done/1077-implement-blockScopedSameNameFunctionDeclarationES.md` |
| 1078 | Implement Blockscopedsamenamefunctiondeclarationstrictes | spike | reference/triage | see `issues/done/1078-implement-blockScopedSameNameFunctionDeclarationStrictES.md` |
| 1079 | Implement Blockscopedvariablesusebeforedef | spike | frontend/resolver | see `issues/done/1079-implement-blockScopedVariablesUseBeforeDef.md` |
| 1080 | Implement Bluebirdstaticthis | spike | frontend/syntax | see `issues/done/1080-implement-bluebirdStaticThis.md` |
| 1081 | Implement Booleanassignment | spike | frontend/resolver | see `issues/done/1081-implement-booleanAssignment.md` |
| 1082 | Implement Booleanfilteranyarray | spike | frontend/resolver | see `issues/done/1082-implement-booleanFilterAnyArray.md` |
| 1083 | Implement Breakiniterationorswitchstatement | spike | frontend/resolver | see `issues/done/1083-implement-breakInIterationOrSwitchStatement.md` |
| 1084 | Implement Breaknotiniterationorswitchstatement | spike | frontend/syntax | see `issues/done/1084-implement-breakNotInIterationOrSwitchStatement.md` |
| 1085 | Implement Breaktarget | spike | frontend/syntax | see `issues/done/1085-implement-breakTarget.md` |
| 1086 | Implement Builtiniterator | spike | frontend/syntax | see `issues/done/1086-implement-builtinIterator.md` |
| 1087 | Implement Bundleddtslateexportrenaming | spike | frontend/syntax | see `issues/done/1087-implement-bundledDtsLateExportRenaming.md` |
| 1088 | Implement Cacheresolutions | spike | frontend/syntax | see `issues/done/1088-implement-cacheResolutions.md` |
| 1089 | Implement Cachedcontextualtypes | spike | frontend/syntax | see `issues/done/1089-implement-cachedContextualTypes.md` |
| 1090 | Implement Cachedmoduleresolution | spike | frontend/syntax | see `issues/done/1090-implement-cachedModuleResolution.md` |
| 1091 | Implement Callconstructassignment | spike | frontend/syntax | see `issues/done/1091-implement-callConstructAssignment.md` |
| 1092 | Implement Callexpressionwithmissingtypeargument | spike | frontend/syntax | see `issues/done/1092-implement-callExpressionWithMissingTypeArgument.md` |
| 1093 | Implement Callexpressionwithtypeparameterconstrainedtooutertypeparameter | spike | frontend/syntax | see `issues/done/1093-implement-callExpressionWithTypeParameterConstrainedToOuterTypeParameter.md` |
| 1094 | Implement Callofconditionaltypewithconcretebranches | spike | frontend/semantics | see `issues/done/1094-implement-callOfConditionalTypeWithConcreteBranches.md` |
| 1095 | Implement Callonclass | spike | frontend/resolver | see `issues/done/1095-implement-callOnClass.md` |
| 1096 | Implement Calloninstance | spike | frontend/resolver | see `issues/done/1096-implement-callOnInstance.md` |
| 1097 | Implement Calloverloadviaelementaccessexpression | spike | frontend/syntax | see `issues/done/1097-implement-callOverloadViaElementAccessExpression.md` |
| 1098 | Implement Calloverloads Class | spike | frontend/syntax | see `issues/done/1098-implement-callOverloads-class.md` |
| 1099 | Implement Calloverloads Parser Syntax | spike | frontend/syntax | see `issues/done/1099-implement-callOverloads-parser-syntax.md` |
| 1100 | Implement Callsignaturefunctionoverload | spike | frontend/syntax | see `issues/done/1100-implement-callSignatureFunctionOverload.md` |
| 1101 | Implement Callsignaturesshouldberesolvedbeforespecialization | spike | frontend/syntax | see `issues/done/1101-implement-callSignaturesShouldBeResolvedBeforeSpecialization.md` |
| 1102 | Implement Callbackargsdifferbyoptionality | spike | frontend/syntax | see `issues/done/1102-implement-callbackArgsDifferByOptionality.md` |
| 1103 | Implement Callbacksdontsharetypes | spike | frontend/syntax | see `issues/done/1103-implement-callbacksDontShareTypes.md` |
| 1104 | Implement Cannotinvokenewonerrorexpression | spike | frontend/syntax | see `issues/done/1104-implement-cannotInvokeNewOnErrorExpression.md` |
| 1105 | Implement Cannotinvokenewonindexexpression | spike | frontend/resolver | see `issues/done/1105-implement-cannotInvokeNewOnIndexExpression.md` |
| 1106 | Implement Capturesuperpropertyaccessinsupercall | spike | frontend/syntax | see `issues/done/1106-implement-captureSuperPropertyAccessInSuperCall.md` |
| 1107 | Implement Capturedletconstinloop Arrow Function | spike | frontend/syntax | see `issues/done/1107-implement-capturedLetConstInLoop-arrow-function.md` |
| 1108 | Implement Capturedletconstinloop Duplicate Local | spike | reference/triage | see `issues/done/1108-implement-capturedLetConstInLoop-duplicate-local.md` |
| 1109 | Implement Capturedletconstinloop Import Export | spike | reference/triage | see `issues/done/1109-implement-capturedLetConstInLoop-import-export.md` |
| 1110 | Implement Capturedletconstinloop Name Resolution | spike | frontend/resolver | see `issues/done/1110-implement-capturedLetConstInLoop-name-resolution.md` |
| 1111 | Implement Capturedletconstinloop Parser Syntax | spike | frontend/syntax | see `issues/done/1111-implement-capturedLetConstInLoop-parser-syntax.md` |
| 1112 | Implement Capturedparametersininitializers | spike | frontend/syntax | see `issues/done/1112-implement-capturedParametersInInitializers.md` |
| 1113 | Implement Capturedshorthandpropertyassignmentnocheck | spike | frontend/syntax | see `issues/done/1113-implement-capturedShorthandPropertyAssignmentNoCheck.md` |
| 1114 | Implement Capturedvarinloop | spike | frontend/syntax | see `issues/done/1114-implement-capturedVarInLoop.md` |
| 1115 | Implement Caseinsensitivefilesystemwithcapsimporttypedeclarations | spike | frontend/syntax | see `issues/done/1115-implement-caseInsensitiveFileSystemWithCapsImportTypeDeclarations.md` |
| 1116 | Implement Castexpressionparentheses | spike | frontend/syntax | see `issues/done/1116-implement-castExpressionParentheses.md` |
| 1117 | Implement Castfunctionexpressionshouldbeparenthesized | spike | frontend/syntax | see `issues/done/1117-implement-castFunctionExpressionShouldBeParenthesized.md` |
| 1118 | Implement Castnewobjectbug | spike | frontend/syntax | see `issues/done/1118-implement-castNewObjectBug.md` |
| 1119 | Implement Castofawait | spike | reference/triage | see `issues/done/1119-implement-castOfAwait.md` |
| 1120 | Implement Castparentheses | spike | frontend/syntax | see `issues/done/1120-implement-castParentheses.md` |
| 1121 | Implement Casttest | spike | frontend/syntax | see `issues/done/1121-implement-castTest.md` |
| 1122 | Implement Catch | spike | reference/triage | see `issues/done/1122-implement-catch.md` |
| 1123 | Implement Catchclausewithinitializer | spike | frontend/syntax | see `issues/done/1123-implement-catchClauseWithInitializer.md` |
| 1124 | Implement Cf | spike | frontend/resolver | see `issues/done/1124-implement-cf.md` |
| 1125 | Implement Chainedassignment | spike | frontend/syntax | see `issues/done/1125-implement-chainedAssignment.md` |
| 1126 | Implement Chainedcallswithtypeparameterconstrainedtoothertypeparameter | spike | frontend/syntax | see `issues/done/1126-implement-chainedCallsWithTypeParameterConstrainedToOtherTypeParameter.md` |
| 1127 | Implement Chainedimportalias | spike | frontend/syntax | see `issues/done/1127-implement-chainedImportAlias.md` |
| 1128 | Implement Chainedspecializationtoobjecttypeliteral | spike | frontend/syntax | see `issues/done/1128-implement-chainedSpecializationToObjectTypeLiteral.md` |
| 1129 | Implement Checkdestructuringshorthandassigment Destructuring | spike | frontend/syntax | see `issues/done/1129-implement-checkDestructuringShorthandAssigment-destructuring.md` |
| 1130 | Implement Checkdestructuringshorthandassigment Name Resolution | spike | frontend/resolver | see `issues/done/1130-implement-checkDestructuringShorthandAssigment-name-resolution.md` |
| 1131 | Implement Checkforobjecttoostrict | spike | frontend/syntax | see `issues/done/1131-implement-checkForObjectTooStrict.md` |
| 1132 | Implement Checkindexconstraintofjavascriptclassexpression | spike | frontend/resolver | see `issues/done/1132-implement-checkIndexConstraintOfJavascriptClassExpression.md` |
| 1133 | Implement Checkinfiniteexpansiontermination | spike | frontend/resolver | see `issues/done/1133-implement-checkInfiniteExpansionTermination.md` |
| 1134 | Implement Checkinheritedproperty | spike | frontend/syntax | see `issues/done/1134-implement-checkInheritedProperty.md` |
| 1135 | Implement Checkjsfiles | spike | frontend/syntax | see `issues/done/1135-implement-checkJsFiles.md` |
| 1136 | Implement Checkjsobjectliteralindexsignatures | spike | frontend/syntax | see `issues/done/1136-implement-checkJsObjectLiteralIndexSignatures.md` |
| 1137 | Implement Checkjstypedefnounusedlocalmarked | spike | frontend/syntax | see `issues/done/1137-implement-checkJsTypeDefNoUnusedLocalMarked.md` |
| 1138 | Implement Checkjsdoctypetagonexportassignment | spike | frontend/syntax | see `issues/done/1138-implement-checkJsdocTypeTagOnExportAssignment.md` |
| 1139 | Implement Checkjsxnotseterror | spike | reference/triage | see `issues/done/1139-implement-checkJsxNotSetError.md` |
| 1140 | Implement Checkmergedglobalumdsymbol | spike | frontend/syntax | see `issues/done/1140-implement-checkMergedGlobalUMDSymbol.md` |
| 1141 | Implement Checksupercallbeforethisaccess | spike | frontend/syntax | see `issues/done/1141-implement-checkSuperCallBeforeThisAccess.md` |
| 1142 | Implement Checksupercallbeforethisaccessing Class | spike | frontend/syntax | see `issues/done/1142-implement-checkSuperCallBeforeThisAccessing-class.md` |
| 1143 | Implement Checksupercallbeforethisaccessing Parser Syntax | spike | frontend/syntax | see `issues/done/1143-implement-checkSuperCallBeforeThisAccessing-parser-syntax.md` |
| 1144 | Implement Checkswitchstatementifcasetypeisstring | spike | frontend/syntax | see `issues/done/1144-implement-checkSwitchStatementIfCaseTypeIsString.md` |
| 1145 | Implement Checktypepredicateforredundantproperties | spike | frontend/semantics | see `issues/done/1145-implement-checkTypePredicateForRedundantProperties.md` |
| 1146 | Implement Checkerinitializationcrash | spike | frontend/syntax | see `issues/done/1146-implement-checkerInitializationCrash.md` |
| 1147 | Implement Checkingobjectdefinepropertyonfunctionnonexistentpropertynocrash | spike | frontend/syntax | see `issues/done/1147-implement-checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash.md` |
| 1148 | Implement Checkingobjectwiththisinnamepositionnocrash | spike | frontend/syntax | see `issues/done/1148-implement-checkingObjectWithThisInNamePositionNoCrash.md` |
| 1149 | Implement Circularaccessorannotations | spike | frontend/syntax | see `issues/done/1149-implement-circularAccessorAnnotations.md` |
| 1150 | Implement Circularbaseconstraint | spike | frontend/syntax | see `issues/done/1150-implement-circularBaseConstraint.md` |
| 1151 | Implement Circularconstraintyieldsappropriateerror | spike | frontend/syntax | see `issues/done/1151-implement-circularConstraintYieldsAppropriateError.md` |
| 1152 | Implement Circularconstructorwithreturn | spike | frontend/syntax | see `issues/done/1152-implement-circularConstructorWithReturn.md` |
| 1153 | Implement Circularcontextualmappedtype | spike | frontend/resolver | see `issues/done/1153-implement-circularContextualMappedType.md` |
| 1154 | Implement Circularcontextualreturntype | spike | frontend/resolver | see `issues/done/1154-implement-circularContextualReturnType.md` |
| 1155 | Implement Circularinferredtypeofvariable | spike | frontend/semantics | see `issues/done/1155-implement-circularInferredTypeOfVariable.md` |
| 1156 | Implement Circularinlinemappedgenerictupletypenocrash | spike | frontend/semantics | see `issues/done/1156-implement-circularInlineMappedGenericTupleTypeNoCrash.md` |
| 1157 | Implement Circularinstantiationexpression | spike | frontend/resolver | see `issues/done/1157-implement-circularInstantiationExpression.md` |
| 1158 | Implement Circularmappedtypeconstraint | spike | frontend/syntax | see `issues/done/1158-implement-circularMappedTypeConstraint.md` |
| 1159 | Implement Circularmoduleimports | spike | frontend/syntax | see `issues/done/1159-implement-circularModuleImports.md` |
| 1160 | Implement Circularobjectliteralaccessors | spike | frontend/syntax | see `issues/done/1160-implement-circularObjectLiteralAccessors.md` |
| 1161 | Implement Circularoptionalityremoval | spike | frontend/resolver | see `issues/done/1161-implement-circularOptionalityRemoval.md` |
| 1162 | Implement Circularreferenceinimport | spike | frontend/syntax | see `issues/done/1162-implement-circularReferenceInImport.md` |
| 1163 | Implement Circularreferenceinreturntype Name Resolution | spike | frontend/resolver | see `issues/done/1163-implement-circularReferenceInReturnType-name-resolution.md` |
| 1164 | Implement Circularreferenceinreturntype Parser Syntax | spike | frontend/syntax | see `issues/done/1164-implement-circularReferenceInReturnType-parser-syntax.md` |
| 1165 | Implement Circularresolvedsignature | spike | frontend/syntax | see `issues/done/1165-implement-circularResolvedSignature.md` |
| 1166 | Implement Circulartypeargumentslocalandouternocrash | spike | frontend/syntax | see `issues/done/1166-implement-circularTypeArgumentsLocalAndOuterNoCrash.md` |
| 1167 | Implement Circulartypeofwithfunctionmodule | spike | frontend/syntax | see `issues/done/1167-implement-circularTypeofWithFunctionModule.md` |
| 1168 | Implement Circularlyconstrainedmappedtypecontainingconditionalnoinfiniteinstantiationdepth | spike | frontend/semantics | see `issues/done/1168-implement-circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth.md` |
| 1169 | Implement Circularlysimplifyingconditionaltypesnocrash | spike | frontend/semantics | see `issues/done/1169-implement-circularlySimplifyingConditionalTypesNoCrash.md` |
| 1170 | Implement Class | spike | frontend/syntax | see `issues/done/1170-implement-class.md` |
| 1171 | Implement Classaccessorinitializationinferencewithelementaccess | spike | frontend/syntax | see `issues/done/1171-implement-classAccessorInitializationInferenceWithElementAccess.md` |
| 1172 | Implement Classattributeinferencetemplate | spike | frontend/semantics | see `issues/done/1172-implement-classAttributeInferenceTemplate.md` |
| 1173 | Implement Classattributeinferencetemplatejs | spike | frontend/semantics | see `issues/done/1173-implement-classAttributeInferenceTemplateJS.md` |
| 1174 | Implement Classblockscoping | spike | frontend/syntax | see `issues/done/1174-implement-classBlockScoping.md` |
| 1175 | Implement Classdeclarationblockscoping | spike | frontend/syntax | see `issues/done/1175-implement-classDeclarationBlockScoping.md` |
| 1176 | Implement Classdeclarationcheckusedbeforedefinitioninitself | spike | frontend/syntax | see `issues/done/1176-implement-classDeclarationCheckUsedBeforeDefinitionInItself.md` |
| 1177 | Implement Classdeclarationmergedinmodulewithcontinuation | spike | frontend/syntax | see `issues/done/1177-implement-classDeclarationMergedInModuleWithContinuation.md` |
| 1178 | Implement Classdeclarationshouldbeoutofscopeincomputednames | spike | frontend/resolver | see `issues/done/1178-implement-classDeclarationShouldBeOutOfScopeInComputedNames.md` |
| 1179 | Implement Classdeclaredbeforeclassfactory | spike | frontend/syntax | see `issues/done/1179-implement-classDeclaredBeforeClassFactory.md` |
| 1180 | Implement Classexpressionassignment | spike | frontend/syntax | see `issues/done/1180-implement-classExpressionAssignment.md` |
| 1181 | Implement Classexpressionextendingabstractclass | spike | frontend/syntax | see `issues/done/1181-implement-classExpressionExtendingAbstractClass.md` |
| 1182 | Implement Classexpressioninclassstaticdeclarations | spike | frontend/syntax | see `issues/done/1182-implement-classExpressionInClassStaticDeclarations.md` |
| 1183 | Implement Classexpressionnames | spike | frontend/syntax | see `issues/done/1183-implement-classExpressionNames.md` |
| 1184 | Implement Classexpressionpropertymodifiers | spike | frontend/syntax | see `issues/done/1184-implement-classExpressionPropertyModifiers.md` |
| 1185 | Implement Classexpressiontest | spike | frontend/syntax | see `issues/done/1185-implement-classExpressionTest.md` |
| 1186 | Implement Classexpressionwithdecorator | spike | frontend/syntax | see `issues/done/1186-implement-classExpressionWithDecorator.md` |
| 1187 | Implement Classexpressionwithresolutionofnamespaceofsamename | spike | frontend/syntax | see `issues/done/1187-implement-classExpressionWithResolutionOfNamespaceOfSameName.md` |
| 1188 | Implement Classexpressionwithstaticproperties Parser Syntax | spike | frontend/syntax | see `issues/done/1188-implement-classExpressionWithStaticProperties-parser-syntax.md` |
| 1189 | Implement Classexpressionwithstaticproperties Unknown Unsupported | spike | frontend/syntax | see `issues/done/1189-implement-classExpressionWithStaticProperties-unknown-unsupported.md` |
| 1190 | Implement Classexpressionwithstaticpropertieses Parser Syntax | spike | frontend/syntax | see `issues/done/1190-implement-classExpressionWithStaticPropertiesES-parser-syntax.md` |
| 1191 | Implement Classexpressionwithstaticpropertieses Unknown Unsupported | spike | frontend/syntax | see `issues/done/1191-implement-classExpressionWithStaticPropertiesES-unknown-unsupported.md` |
| 1192 | Implement Classexpressions | spike | frontend/syntax | see `issues/done/1192-implement-classExpressions.md` |
| 1193 | Implement Classextendingabstractclasswithmembercalledthesameasitsowntypeparam | spike | frontend/syntax | see `issues/done/1193-implement-classExtendingAbstractClassWithMemberCalledTheSameAsItsOwnTypeParam.md` |
| 1194 | Implement Classextendingany | spike | frontend/syntax | see `issues/done/1194-implement-classExtendingAny.md` |
| 1195 | Implement Classextendingqualifiedname | spike | frontend/syntax | see `issues/done/1195-implement-classExtendingQualifiedName.md` |
| 1196 | Implement Classextendsacrossfiles | spike | frontend/syntax | see `issues/done/1196-implement-classExtendsAcrossFiles.md` |
| 1197 | Implement Classextendsclauseclassmergedwithmodulenotreferingconstructor | spike | frontend/syntax | see `issues/done/1197-implement-classExtendsClauseClassMergedWithModuleNotReferingConstructor.md` |
| 1198 | Implement Classextendsclauseclassnotreferringconstructor | spike | frontend/syntax | see `issues/done/1198-implement-classExtendsClauseClassNotReferringConstructor.md` |
| 1199 | Implement Classextendsinterface Parser Syntax | spike | frontend/syntax | see `issues/done/1199-implement-classExtendsInterface-parser-syntax.md` |
| 1200 | Implement Classextendsinterface Unknown Unsupported | spike | frontend/syntax | see `issues/done/1200-implement-classExtendsInterface-unknown-unsupported.md` |
| 1201 | Implement Classextendsinterfaceinexpression | spike | frontend/syntax | see `issues/done/1201-implement-classExtendsInterfaceInExpression.md` |
| 1202 | Implement Classextendsinterfaceinmodule | spike | frontend/syntax | see `issues/done/1202-implement-classExtendsInterfaceInModule.md` |
| 1203 | Implement Classextendsinterfacethatextendsclasswithprivates | spike | frontend/syntax | see `issues/done/1203-implement-classExtendsInterfaceThatExtendsClassWithPrivates.md` |
| 1204 | Implement Classextendsmultiplebaseclasses | spike | frontend/syntax | see `issues/done/1204-implement-classExtendsMultipleBaseClasses.md` |
| 1205 | Implement Classextendsnull | spike | frontend/syntax | see `issues/done/1205-implement-classExtendsNull.md` |
| 1206 | Implement Classextensionnameoutput | spike | frontend/syntax | see `issues/done/1206-implement-classExtensionNameOutput.md` |
| 1207 | Implement Classfieldsuperaccessible | spike | frontend/syntax | see `issues/done/1207-implement-classFieldSuperAccessible.md` |
| 1208 | Implement Classfieldsuperaccessiblejs | spike | frontend/syntax | see `issues/done/1208-implement-classFieldSuperAccessibleJs.md` |
| 1209 | Implement Classfieldsupernotaccessible | spike | frontend/syntax | see `issues/done/1209-implement-classFieldSuperNotAccessible.md` |
| 1210 | Implement Classfieldsupernotaccessiblejs | spike | frontend/syntax | see `issues/done/1210-implement-classFieldSuperNotAccessibleJs.md` |
| 1211 | Implement Classfieldsbrokenconstructoremitnocrash | spike | frontend/syntax | see `issues/done/1211-implement-classFieldsBrokenConstructorEmitNoCrash.md` |
| 1212 | Implement Classfunctionmerging Import Export | spike | frontend/syntax | see `issues/done/1212-implement-classFunctionMerging-import-export.md` |
| 1213 | Implement Classfunctionmerging Parser Syntax | spike | frontend/syntax | see `issues/done/1213-implement-classFunctionMerging-parser-syntax.md` |
| 1214 | Implement Classheritagewithtrailingseparator | spike | frontend/syntax | see `issues/done/1214-implement-classHeritageWithTrailingSeparator.md` |
| 1215 | Implement Classimplementinginterfaceindexer | spike | frontend/syntax | see `issues/done/1215-implement-classImplementingInterfaceIndexer.md` |
| 1216 | Implement Classimplementsclass | spike | frontend/syntax | see `issues/done/1216-implement-classImplementsClass.md` |
| 1217 | Implement Classimplementsimportedinterface | spike | frontend/syntax | see `issues/done/1217-implement-classImplementsImportedInterface.md` |
| 1218 | Implement Classimplementsmethodwithtupleargs | spike | frontend/syntax | see `issues/done/1218-implement-classImplementsMethodWIthTupleArgs.md` |
| 1219 | Implement Classimplementsprimitive | spike | frontend/syntax | see `issues/done/1219-implement-classImplementsPrimitive.md` |
| 1220 | Implement Classinconvertedloopes | spike | frontend/syntax | see `issues/done/1220-implement-classInConvertedLoopES.md` |
| 1221 | Implement Classindexer | spike | frontend/syntax | see `issues/done/1221-implement-classIndexer.md` |
| 1222 | Implement Classmemberinitializerscoping | spike | frontend/syntax | see `issues/done/1222-implement-classMemberInitializerScoping.md` |
| 1223 | Implement Classmemberinitializerwithlamdascoping Import Export | spike | frontend/syntax | see `issues/done/1223-implement-classMemberInitializerWithLamdaScoping-import-export.md` |
| 1224 | Implement Classmemberinitializerwithlamdascoping Module System Amd | spike | frontend/syntax | see `issues/done/1224-implement-classMemberInitializerWithLamdaScoping-module-system-amd.md` |
| 1225 | Implement Classmemberwithmissingidentifier | spike | frontend/syntax | see `issues/done/1225-implement-classMemberWithMissingIdentifier.md` |
| 1226 | Implement Classmergedwithinterfacemultiplebasesnoerror | spike | frontend/syntax | see `issues/done/1226-implement-classMergedWithInterfaceMultipleBasesNoError.md` |
| 1227 | Implement Classmethodwithkeywordname | spike | frontend/syntax | see `issues/done/1227-implement-classMethodWithKeywordName.md` |
| 1228 | Implement Classnamereferencesinstaticelements | spike | frontend/syntax | see `issues/done/1228-implement-classNameReferencesInStaticElements.md` |
| 1229 | Implement Classnonuniquesymbolmethodhassymbolindexer | spike | frontend/syntax | see `issues/done/1229-implement-classNonUniqueSymbolMethodHasSymbolIndexer.md` |
| 1230 | Implement Classorder | spike | frontend/syntax | see `issues/done/1230-implement-classOrder.md` |
| 1231 | Implement Classorderbug | spike | frontend/syntax | see `issues/done/1231-implement-classOrderBug.md` |
| 1232 | Implement Classpropinitializationinferencewithelementaccess | spike | frontend/syntax | see `issues/done/1232-implement-classPropInitializationInferenceWithElementAccess.md` |
| 1233 | Implement Classpropertyerroronnameonly | spike | runtime/builtins | see `issues/done/1233-implement-classPropertyErrorOnNameOnly.md` |
| 1234 | Implement Classpropertyinferencefrombroadertypeconst | spike | frontend/semantics | see `issues/done/1234-implement-classPropertyInferenceFromBroaderTypeConst.md` |
| 1235 | Implement Classreferencedincontextualparameterwithinitsownbaseexpression | spike | frontend/syntax | see `issues/done/1235-implement-classReferencedInContextualParameterWithinItsOwnBaseExpression.md` |
| 1236 | Implement Classsideinheritance Name Resolution | spike | frontend/resolver | see `issues/done/1236-implement-classSideInheritance-name-resolution.md` |
| 1237 | Implement Classsideinheritance Parser Syntax | spike | frontend/syntax | see `issues/done/1237-implement-classSideInheritance-parser-syntax.md` |
| 1238 | Implement Classstaticinitializersusepropertiesbeforedeclaration | spike | frontend/syntax | see `issues/done/1238-implement-classStaticInitializersUsePropertiesBeforeDeclaration.md` |
| 1239 | Implement Classstaticpropertyaccess | spike | frontend/syntax | see `issues/done/1239-implement-classStaticPropertyAccess.md` |
| 1240 | Implement Classstaticpropertytypeguard | spike | frontend/syntax | see `issues/done/1240-implement-classStaticPropertyTypeGuard.md` |
| 1241 | Implement Classtypeparametersinstatics | spike | frontend/syntax | see `issues/done/1241-implement-classTypeParametersInStatics.md` |
| 1242 | Implement Classupdatetests | spike | runtime/builtins | see `issues/done/1242-implement-classUpdateTests.md` |
| 1243 | Implement Classusedbeforeinitializedvariables | spike | frontend/syntax | see `issues/done/1243-implement-classUsedBeforeInitializedVariables.md` |
| 1244 | Implement Classvariancecircularity | spike | frontend/syntax | see `issues/done/1244-implement-classVarianceCircularity.md` |
| 1245 | Implement Classvarianceresolvecircularity | spike | frontend/syntax | see `issues/done/1245-implement-classVarianceResolveCircularity.md` |
| 1246 | Implement Classwithemptytypeparameter | spike | frontend/syntax | see `issues/done/1246-implement-classWithEmptyTypeParameter.md` |
| 1247 | Implement Classwithmultiplebaseclasses | spike | frontend/syntax | see `issues/done/1247-implement-classWithMultipleBaseClasses.md` |
| 1248 | Implement Classwithoverloadimplementationofwrongname | spike | frontend/syntax | see `issues/done/1248-implement-classWithOverloadImplementationOfWrongName.md` |
| 1249 | Implement Classdecl | spike | frontend/syntax | see `issues/done/1249-implement-classdecl.md` |
| 1250 | Implement Clinterfaces | spike | frontend/syntax | see `issues/done/1250-implement-clinterfaces.md` |
| 1251 | Implement Cloduleacrossmoduledefinitions | spike | frontend/syntax | see `issues/done/1251-implement-cloduleAcrossModuleDefinitions.md` |
| 1252 | Implement Cloduleandtypeparameters | spike | frontend/syntax | see `issues/done/1252-implement-cloduleAndTypeParameters.md` |
| 1253 | Implement Clodulegenericonselfmember | spike | frontend/semantics | see `issues/done/1253-implement-cloduleGenericOnSelfMember.md` |
| 1254 | Implement Clodulesplitacrossfiles | spike | frontend/syntax | see `issues/done/1254-implement-cloduleSplitAcrossFiles.md` |
| 1255 | Implement Clodulestaticmembers | spike | frontend/syntax | see `issues/done/1255-implement-cloduleStaticMembers.md` |
| 1256 | Implement Cloduletest | spike | frontend/syntax | see `issues/done/1256-implement-cloduleTest.md` |
| 1257 | Implement Clodulewithduplicatemember | spike | frontend/syntax | see `issues/done/1257-implement-cloduleWithDuplicateMember.md` |
| 1258 | Implement Clodulewithpriorinstantiatedmodule | spike | frontend/syntax | see `issues/done/1258-implement-cloduleWithPriorInstantiatedModule.md` |
| 1259 | Implement Clodulewithprioruninstantiatedmodule | spike | frontend/syntax | see `issues/done/1259-implement-cloduleWithPriorUninstantiatedModule.md` |
| 1260 | Implement Clodulewithrecursivereference | spike | frontend/syntax | see `issues/done/1260-implement-cloduleWithRecursiveReference.md` |
| 1261 | Implement Clodulesderivedclasses | spike | frontend/syntax | see `issues/done/1261-implement-clodulesDerivedClasses.md` |
| 1262 | Implement Coandcontravariantinferences Name Resolution | spike | frontend/resolver | see `issues/done/1262-implement-coAndContraVariantInferences-name-resolution.md` |
| 1263 | Implement Coandcontravariantinferences Parser Syntax | spike | frontend/syntax | see `issues/done/1263-implement-coAndContraVariantInferences-parser-syntax.md` |
| 1264 | Implement Coandcontravariantinferences Type System | spike | frontend/semantics | see `issues/done/1264-implement-coAndContraVariantInferences-type-system.md` |
| 1265 | Implement Collectionpatternnoerror | spike | runtime/builtins | see `issues/done/1265-implement-collectionPatternNoError.md` |
| 1266 | Implement Collisionargumentsarrowfunctions | spike | frontend/syntax | see `issues/done/1266-implement-collisionArgumentsArrowFunctions.md` |
| 1267 | Implement Collisionargumentsclassconstructor | spike | frontend/syntax | see `issues/done/1267-implement-collisionArgumentsClassConstructor.md` |
| 1268 | Implement Collisionargumentsclassmethod | spike | frontend/syntax | see `issues/done/1268-implement-collisionArgumentsClassMethod.md` |
| 1269 | Implement Collisionargumentsfunction | spike | frontend/syntax | see `issues/done/1269-implement-collisionArgumentsFunction.md` |
| 1270 | Implement Collisionargumentsfunctionexpressions | spike | frontend/syntax | see `issues/done/1270-implement-collisionArgumentsFunctionExpressions.md` |
| 1271 | Implement Collisionargumentsintype | spike | frontend/syntax | see `issues/done/1271-implement-collisionArgumentsInType.md` |
| 1272 | Implement Collisioncodegenenumwithenummemberconflict | spike | frontend/syntax | see `issues/done/1272-implement-collisionCodeGenEnumWithEnumMemberConflict.md` |
| 1273 | Implement Collisioncodegenmodulewithaccessorchildren | spike | frontend/syntax | see `issues/done/1273-implement-collisionCodeGenModuleWithAccessorChildren.md` |
| 1274 | Implement Collisioncodegenmodulewithconstructorchildren | spike | frontend/syntax | see `issues/done/1274-implement-collisionCodeGenModuleWithConstructorChildren.md` |
| 1275 | Implement Collisioncodegenmodulewithenummemberconflict | spike | frontend/syntax | see `issues/done/1275-implement-collisionCodeGenModuleWithEnumMemberConflict.md` |
| 1276 | Implement Collisioncodegenmodulewithfunctionchildren | spike | frontend/syntax | see `issues/done/1276-implement-collisionCodeGenModuleWithFunctionChildren.md` |
| 1277 | Implement Collisioncodegenmodulewithmemberclassconflict | spike | frontend/syntax | see `issues/done/1277-implement-collisionCodeGenModuleWithMemberClassConflict.md` |
| 1278 | Implement Collisioncodegenmodulewithmemberinterfaceconflict | spike | frontend/syntax | see `issues/done/1278-implement-collisionCodeGenModuleWithMemberInterfaceConflict.md` |
| 1279 | Implement Collisioncodegenmodulewithmembervariable | spike | frontend/syntax | see `issues/done/1279-implement-collisionCodeGenModuleWithMemberVariable.md` |
| 1280 | Implement Collisioncodegenmodulewithmethodchildren | spike | frontend/syntax | see `issues/done/1280-implement-collisionCodeGenModuleWithMethodChildren.md` |
| 1281 | Implement Collisioncodegenmodulewithmodulechildren | spike | frontend/syntax | see `issues/done/1281-implement-collisionCodeGenModuleWithModuleChildren.md` |
| 1282 | Implement Collisioncodegenmodulewithmodulereopening | spike | frontend/syntax | see `issues/done/1282-implement-collisionCodeGenModuleWithModuleReopening.md` |
| 1283 | Implement Collisioncodegenmodulewithprivatemember | spike | frontend/syntax | see `issues/done/1283-implement-collisionCodeGenModuleWithPrivateMember.md` |
| 1284 | Implement Collisionexportsrequireandalias | spike | frontend/syntax | see `issues/done/1284-implement-collisionExportsRequireAndAlias.md` |
| 1285 | Implement Collisionexportsrequireandambientclass | spike | frontend/syntax | see `issues/done/1285-implement-collisionExportsRequireAndAmbientClass.md` |
| 1286 | Implement Collisionexportsrequireandambientenum | spike | frontend/syntax | see `issues/done/1286-implement-collisionExportsRequireAndAmbientEnum.md` |
| 1287 | Implement Collisionexportsrequireandambientfunction | spike | frontend/syntax | see `issues/done/1287-implement-collisionExportsRequireAndAmbientFunction.md` |
| 1288 | Implement Collisionexportsrequireandambientfunctioninglobalfile | spike | frontend/syntax | see `issues/done/1288-implement-collisionExportsRequireAndAmbientFunctionInGlobalFile.md` |
| 1289 | Implement Collisionexportsrequireandambientmodule | spike | frontend/syntax | see `issues/done/1289-implement-collisionExportsRequireAndAmbientModule.md` |
| 1290 | Implement Collisionexportsrequireandambientvar | spike | frontend/syntax | see `issues/done/1290-implement-collisionExportsRequireAndAmbientVar.md` |
| 1291 | Implement Collisionexportsrequireandclass | spike | frontend/syntax | see `issues/done/1291-implement-collisionExportsRequireAndClass.md` |
| 1292 | Implement Collisionexportsrequireandenum | spike | frontend/syntax | see `issues/done/1292-implement-collisionExportsRequireAndEnum.md` |
| 1293 | Implement Collisionexportsrequireandfunction | spike | frontend/syntax | see `issues/done/1293-implement-collisionExportsRequireAndFunction.md` |
| 1294 | Implement Collisionexportsrequireandfunctioninglobalfile | spike | frontend/syntax | see `issues/done/1294-implement-collisionExportsRequireAndFunctionInGlobalFile.md` |
| 1295 | Implement Collisionexportsrequireandinternalmodulealias | spike | frontend/syntax | see `issues/done/1295-implement-collisionExportsRequireAndInternalModuleAlias.md` |
| 1296 | Implement Collisionexportsrequireandinternalmodulealiasinglobalfile | spike | frontend/syntax | see `issues/done/1296-implement-collisionExportsRequireAndInternalModuleAliasInGlobalFile.md` |
| 1297 | Implement Collisionexportsrequireandmodule | spike | frontend/syntax | see `issues/done/1297-implement-collisionExportsRequireAndModule.md` |
| 1298 | Implement Collisionexportsrequireanduninstantiatedmodule | spike | frontend/syntax | see `issues/done/1298-implement-collisionExportsRequireAndUninstantiatedModule.md` |
| 1299 | Implement Collisionexportsrequireandvar | spike | frontend/syntax | see `issues/done/1299-implement-collisionExportsRequireAndVar.md` |
| 1300 | Implement Collisionrestparameterarrowfunctions | spike | frontend/syntax | see `issues/done/1300-implement-collisionRestParameterArrowFunctions.md` |
| 1301 | Implement Collisionrestparameterclassconstructor | spike | frontend/syntax | see `issues/done/1301-implement-collisionRestParameterClassConstructor.md` |
| 1302 | Implement Collisionrestparameterclassmethod | spike | frontend/syntax | see `issues/done/1302-implement-collisionRestParameterClassMethod.md` |
| 1303 | Implement Collisionrestparameterfunction | spike | frontend/syntax | see `issues/done/1303-implement-collisionRestParameterFunction.md` |
| 1304 | Implement Collisionrestparameterfunctionexpressions | spike | frontend/syntax | see `issues/done/1304-implement-collisionRestParameterFunctionExpressions.md` |
| 1305 | Implement Collisionrestparameterintype | spike | frontend/syntax | see `issues/done/1305-implement-collisionRestParameterInType.md` |
| 1306 | Implement Collisionrestparameterunderscoreiusage | spike | frontend/syntax | see `issues/done/1306-implement-collisionRestParameterUnderscoreIUsage.md` |
| 1307 | Implement Collisionsuperandlocalfunctioninaccessors | spike | frontend/syntax | see `issues/done/1307-implement-collisionSuperAndLocalFunctionInAccessors.md` |
| 1308 | Implement Collisionsuperandlocalfunctioninconstructor | spike | frontend/syntax | see `issues/done/1308-implement-collisionSuperAndLocalFunctionInConstructor.md` |
| 1309 | Implement Collisionsuperandlocalfunctioninmethod | spike | frontend/syntax | see `issues/done/1309-implement-collisionSuperAndLocalFunctionInMethod.md` |
| 1310 | Implement Collisionsuperandlocalfunctioninproperty | spike | frontend/syntax | see `issues/done/1310-implement-collisionSuperAndLocalFunctionInProperty.md` |
| 1311 | Implement Collisionsuperandlocalvarinaccessors | spike | frontend/syntax | see `issues/done/1311-implement-collisionSuperAndLocalVarInAccessors.md` |
| 1312 | Implement Collisionsuperandlocalvarinconstructor | spike | frontend/syntax | see `issues/done/1312-implement-collisionSuperAndLocalVarInConstructor.md` |
| 1313 | Implement Collisionsuperandlocalvarinmethod | spike | frontend/syntax | see `issues/done/1313-implement-collisionSuperAndLocalVarInMethod.md` |
| 1314 | Implement Collisionsuperandlocalvarinproperty | spike | frontend/syntax | see `issues/done/1314-implement-collisionSuperAndLocalVarInProperty.md` |
| 1315 | Implement Collisionsuperandnameresolution | spike | frontend/resolver | see `issues/done/1315-implement-collisionSuperAndNameResolution.md` |
| 1316 | Implement Collisionsuperandparameter | spike | frontend/syntax | see `issues/done/1316-implement-collisionSuperAndParameter.md` |
| 1317 | Implement Collisionsuperandpropertynameasconstuctorparameter | spike | frontend/syntax | see `issues/done/1317-implement-collisionSuperAndPropertyNameAsConstuctorParameter.md` |
| 1318 | Implement Collisionthisexpressionandaliasinglobal | spike | frontend/syntax | see `issues/done/1318-implement-collisionThisExpressionAndAliasInGlobal.md` |
| 1319 | Implement Collisionthisexpressionandambientclassinglobal | spike | frontend/resolver | see `issues/done/1319-implement-collisionThisExpressionAndAmbientClassInGlobal.md` |
| 1320 | Implement Collisionthisexpressionandambientvaringlobal | spike | frontend/resolver | see `issues/done/1320-implement-collisionThisExpressionAndAmbientVarInGlobal.md` |
| 1321 | Implement Collisionthisexpressionandclassinglobal | spike | frontend/syntax | see `issues/done/1321-implement-collisionThisExpressionAndClassInGlobal.md` |
| 1322 | Implement Collisionthisexpressionandenuminglobal | spike | frontend/syntax | see `issues/done/1322-implement-collisionThisExpressionAndEnumInGlobal.md` |
| 1323 | Implement Collisionthisexpressionandfunctioninglobal | spike | frontend/syntax | see `issues/done/1323-implement-collisionThisExpressionAndFunctionInGlobal.md` |
| 1324 | Implement Collisionthisexpressionandlocalvarinaccessors | spike | frontend/syntax | see `issues/done/1324-implement-collisionThisExpressionAndLocalVarInAccessors.md` |
| 1325 | Implement Collisionthisexpressionandlocalvarinconstructor | spike | frontend/syntax | see `issues/done/1325-implement-collisionThisExpressionAndLocalVarInConstructor.md` |
| 1326 | Implement Collisionthisexpressionandlocalvarinfunction | spike | frontend/syntax | see `issues/done/1326-implement-collisionThisExpressionAndLocalVarInFunction.md` |
| 1327 | Implement Collisionthisexpressionandlocalvarinlambda | spike | frontend/syntax | see `issues/done/1327-implement-collisionThisExpressionAndLocalVarInLambda.md` |
| 1328 | Implement Collisionthisexpressionandlocalvarinmethod | spike | frontend/syntax | see `issues/done/1328-implement-collisionThisExpressionAndLocalVarInMethod.md` |
| 1329 | Implement Collisionthisexpressionandlocalvarinproperty | spike | frontend/syntax | see `issues/done/1329-implement-collisionThisExpressionAndLocalVarInProperty.md` |
| 1330 | Implement Collisionthisexpressionandlocalvarwithsuperexperssion | spike | frontend/syntax | see `issues/done/1330-implement-collisionThisExpressionAndLocalVarWithSuperExperssion.md` |
| 1331 | Implement Collisionthisexpressionandmoduleinglobal | spike | frontend/syntax | see `issues/done/1331-implement-collisionThisExpressionAndModuleInGlobal.md` |
| 1332 | Implement Collisionthisexpressionandnameresolution | spike | frontend/syntax | see `issues/done/1332-implement-collisionThisExpressionAndNameResolution.md` |
| 1333 | Implement Collisionthisexpressionandparameter | spike | frontend/syntax | see `issues/done/1333-implement-collisionThisExpressionAndParameter.md` |
| 1334 | Implement Collisionthisexpressionandpropertynameasconstuctorparameter | spike | frontend/syntax | see `issues/done/1334-implement-collisionThisExpressionAndPropertyNameAsConstuctorParameter.md` |
| 1335 | Implement Collisionthisexpressionandvaringlobal | spike | frontend/syntax | see `issues/done/1335-implement-collisionThisExpressionAndVarInGlobal.md` |
| 1336 | Implement Commaoperator | spike | frontend/syntax | see `issues/done/1336-implement-commaOperator.md` |
| 1337 | Implement Commaoperatorinconditionalexpression | spike | frontend/semantics | see `issues/done/1337-implement-commaOperatorInConditionalExpression.md` |
| 1338 | Implement Commaoperatorleftsideunused | spike | frontend/syntax | see `issues/done/1338-implement-commaOperatorLeftSideUnused.md` |
| 1339 | Implement Commentbeforestaticmethod | spike | frontend/syntax | see `issues/done/1339-implement-commentBeforeStaticMethod.md` |
| 1340 | Implement Commentemitatendoffile | spike | frontend/syntax | see `issues/done/1340-implement-commentEmitAtEndOfFile.md` |
| 1341 | Implement Commentemitonparenthesizedassertioninreturnstatement | spike | frontend/syntax | see `issues/done/1341-implement-commentEmitOnParenthesizedAssertionInReturnStatement.md` |
| 1342 | Implement Commentinmethodcall | spike | frontend/syntax | see `issues/done/1342-implement-commentInMethodCall.md` |
| 1343 | Implement Commentinnamespacedeclarationwithidentifierpathname | spike | frontend/syntax | see `issues/done/1343-implement-commentInNamespaceDeclarationWithIdentifierPathName.md` |
| 1344 | Implement Commentleadingclosebrace | spike | frontend/resolver | see `issues/done/1344-implement-commentLeadingCloseBrace.md` |
| 1345 | Implement Commentonambientmodule | spike | frontend/syntax | see `issues/done/1345-implement-commentOnAmbientModule.md` |
| 1346 | Implement Commentonambientvariable | spike | frontend/resolver | see `issues/done/1346-implement-commentOnAmbientVariable.md` |
| 1347 | Implement Commentonclassaccessor | spike | reference/triage | see `issues/done/1347-implement-commentOnClassAccessor.md` |
| 1348 | Implement Commentondecoratedclassdeclaration | spike | frontend/syntax | see `issues/done/1348-implement-commentOnDecoratedClassDeclaration.md` |
| 1349 | Implement Commentonelidedmodule | spike | frontend/syntax | see `issues/done/1349-implement-commentOnElidedModule.md` |
| 1350 | Implement Commentonexportenumdeclaration | spike | frontend/syntax | see `issues/done/1350-implement-commentOnExportEnumDeclaration.md` |
| 1351 | Implement Commentonimportstatement | spike | frontend/syntax | see `issues/done/1351-implement-commentOnImportStatement.md` |
| 1352 | Implement Commentonparameter | spike | frontend/syntax | see `issues/done/1352-implement-commentOnParameter.md` |
| 1353 | Implement Commentonparenthesizedexpressionopenparen | spike | frontend/syntax | see `issues/done/1353-implement-commentOnParenthesizedExpressionOpenParen.md` |
| 1354 | Implement Commentonsignature | spike | frontend/syntax | see `issues/done/1354-implement-commentOnSignature.md` |
| 1355 | Implement Commentwithunreasonableindentationlevel | spike | frontend/syntax | see `issues/done/1355-implement-commentWithUnreasonableIndentationLevel.md` |
| 1356 | Implement Commentsafterfunctionexpression | spike | frontend/syntax | see `issues/done/1356-implement-commentsAfterFunctionExpression.md` |
| 1357 | Implement Commentsafterspread | spike | frontend/syntax | see `issues/done/1357-implement-commentsAfterSpread.md` |
| 1358 | Implement Commentsatendoffile | spike | frontend/syntax | see `issues/done/1358-implement-commentsAtEndOfFile.md` |
| 1359 | Implement Commentsbeforefunctionexpression | spike | frontend/syntax | see `issues/done/1359-implement-commentsBeforeFunctionExpression.md` |
| 1360 | Implement Commentsbeforevariablestatement | spike | frontend/syntax | see `issues/done/1360-implement-commentsBeforeVariableStatement.md` |
| 1361 | Implement Commentsclass | spike | frontend/resolver | see `issues/done/1361-implement-commentsClass.md` |
| 1362 | Implement Commentsclassmembers | spike | frontend/syntax | see `issues/done/1362-implement-commentsClassMembers.md` |
| 1363 | Implement Commentscommentparsing | spike | frontend/syntax | see `issues/done/1363-implement-commentsCommentParsing.md` |
| 1364 | Implement Commentsdottedmodulename | spike | frontend/syntax | see `issues/done/1364-implement-commentsDottedModuleName.md` |
| 1365 | Implement Commentsenums | spike | frontend/syntax | see `issues/done/1365-implement-commentsEnums.md` |
| 1366 | Implement Commentsexternalmodules | spike | frontend/syntax | see `issues/done/1366-implement-commentsExternalModules.md` |
| 1367 | Implement Commentsformatting | spike | frontend/syntax | see `issues/done/1367-implement-commentsFormatting.md` |
| 1368 | Implement Commentsfunction | spike | frontend/syntax | see `issues/done/1368-implement-commentsFunction.md` |
| 1369 | Implement Commentsinheritance | spike | frontend/syntax | see `issues/done/1369-implement-commentsInheritance.md` |
| 1370 | Implement Commentsinterface | spike | frontend/syntax | see `issues/done/1370-implement-commentsInterface.md` |
| 1371 | Implement Commentsmodules | spike | frontend/syntax | see `issues/done/1371-implement-commentsModules.md` |
| 1372 | Implement Commentsmultimodulemultifile | spike | frontend/syntax | see `issues/done/1372-implement-commentsMultiModuleMultiFile.md` |
| 1373 | Implement Commentsmultimodulesinglefile | spike | frontend/syntax | see `issues/done/1373-implement-commentsMultiModuleSingleFile.md` |
| 1374 | Implement Commentsonobjectliteral Name Resolution | spike | frontend/resolver | see `issues/done/1374-implement-commentsOnObjectLiteral-name-resolution.md` |
| 1375 | Implement Commentsonobjectliteral Object Literal | spike | frontend/syntax | see `issues/done/1375-implement-commentsOnObjectLiteral-object-literal.md` |
| 1376 | Implement Commentsonrequirestatement | spike | frontend/syntax | see `issues/done/1376-implement-commentsOnRequireStatement.md` |
| 1377 | Implement Commentsonreturnstatement | spike | frontend/syntax | see `issues/done/1377-implement-commentsOnReturnStatement.md` |
| 1378 | Implement Commentsonstaticmembers | spike | frontend/syntax | see `issues/done/1378-implement-commentsOnStaticMembers.md` |
| 1379 | Implement Commentsoverloads | spike | frontend/syntax | see `issues/done/1379-implement-commentsOverloads.md` |
| 1380 | Implement Commentstypeparameters | spike | frontend/syntax | see `issues/done/1380-implement-commentsTypeParameters.md` |
| 1381 | Implement Commentsdonotemitcomments | spike | frontend/syntax | see `issues/done/1381-implement-commentsdoNotEmitComments.md` |
| 1382 | Implement Commentsemitcomments | spike | frontend/syntax | see `issues/done/1382-implement-commentsemitComments.md` |
| 1383 | Implement Commonjsexporttypedeclarationerror | spike | frontend/syntax | see `issues/done/1383-implement-commonJsExportTypeDeclarationError.md` |
| 1384 | Implement Commonjsimportclassexpression | spike | frontend/syntax | see `issues/done/1384-implement-commonJsImportClassExpression.md` |
| 1385 | Implement Commonjsisolatedmodules | spike | frontend/syntax | see `issues/done/1385-implement-commonJsIsolatedModules.md` |
| 1386 | Implement Commonmissingsemicolons | spike | reference/triage | see `issues/done/1386-implement-commonMissingSemicolons.md` |
| 1387 | Implement Commonsourcedir | spike | frontend/syntax | see `issues/done/1387-implement-commonSourceDir.md` |
| 1388 | Implement Commonsourcedirectory | spike | frontend/syntax | see `issues/done/1388-implement-commonSourceDirectory.md` |
| 1389 | Implement Commonjsaccessexports | spike | frontend/syntax | see `issues/done/1389-implement-commonjsAccessExports.md` |
| 1390 | Implement Commonjssafeimport | spike | frontend/syntax | see `issues/done/1390-implement-commonjsSafeImport.md` |
| 1391 | Implement Comparabilitytypeparametersrelatedbyunion | spike | frontend/syntax | see `issues/done/1391-implement-comparabilityTypeParametersRelatedByUnion.md` |
| 1392 | Implement Comparablerelationbidirectional | spike | frontend/syntax | see `issues/done/1392-implement-comparableRelationBidirectional.md` |
| 1393 | Implement Comparisonofpartialdeepandindexedaccessterminateswithouterror | spike | frontend/syntax | see `issues/done/1393-implement-comparisonOfPartialDeepAndIndexedAccessTerminatesWithoutError.md` |
| 1394 | Implement Complexclassrelationships | spike | frontend/syntax | see `issues/done/1394-implement-complexClassRelationships.md` |
| 1395 | Implement Complexnarrowingwithany | spike | frontend/syntax | see `issues/done/1395-implement-complexNarrowingWithAny.md` |
| 1396 | Implement Complexrecursivecollections | spike | frontend/syntax | see `issues/done/1396-implement-complexRecursiveCollections.md` |
| 1397 | Implement Complicatedgenericrecursivebaseclassreference | spike | frontend/semantics | see `issues/done/1397-implement-complicatedGenericRecursiveBaseClassReference.md` |
| 1398 | Implement Complicatedindexedaccesskeyofreliesonkeyofneverupperbound | spike | frontend/syntax | see `issues/done/1398-implement-complicatedIndexedAccessKeyofReliesOnKeyofNeverUpperBound.md` |
| 1399 | Implement Complicatedindexesofintersectionsareinferencable | spike | frontend/semantics | see `issues/done/1399-implement-complicatedIndexesOfIntersectionsAreInferencable.md` |
| 1400 | Implement Complicatedprivacy | spike | frontend/syntax | see `issues/done/1400-implement-complicatedPrivacy.md` |
| 1401 | Implement Compositecontextualsignature | spike | frontend/syntax | see `issues/done/1401-implement-compositeContextualSignature.md` |
| 1402 | Implement Compositegenericfunction | spike | reference/triage | see `issues/done/1402-implement-compositeGenericFunction.md` |
| 1403 | Implement Compositewithnodemodulessourcefile | spike | frontend/syntax | see `issues/done/1403-implement-compositeWithNodeModulesSourceFile.md` |
| 1404 | Implement Compoundvardecl | spike | frontend/syntax | see `issues/done/1404-implement-compoundVarDecl.md` |
| 1405 | Implement Computedenummembersyntacticallystring Enum | spike | frontend/syntax | see `issues/done/1405-implement-computedEnumMemberSyntacticallyString-enum.md` |
| 1406 | Implement Computedenummembersyntacticallystring Parser Syntax | spike | frontend/syntax | see `issues/done/1406-implement-computedEnumMemberSyntacticallyString-parser-syntax.md` |
| 1407 | Implement Computedenumtypewidening | spike | frontend/syntax | see `issues/done/1407-implement-computedEnumTypeWidening.md` |
| 1408 | Implement Computedpropertiesindestructuring | spike | frontend/syntax | see `issues/done/1408-implement-computedPropertiesInDestructuring.md` |
| 1409 | Implement Computedpropertiesnarrowed | spike | frontend/syntax | see `issues/done/1409-implement-computedPropertiesNarrowed.md` |
| 1410 | Implement Computedpropertiestransformedinotherwisenontsclasses | spike | frontend/syntax | see `issues/done/1410-implement-computedPropertiesTransformedInOtherwiseNonTSClasses.md` |
| 1411 | Implement Computedpropertieswithsetterassignment | spike | frontend/syntax | see `issues/done/1411-implement-computedPropertiesWithSetterAssignment.md` |
| 1412 | Implement Computedpropertybindingelementdeclarationnocrash | spike | frontend/syntax | see `issues/done/1412-implement-computedPropertyBindingElementDeclarationNoCrash.md` |
| 1413 | Implement Computedpropertynameandtypeparameterconflict | spike | frontend/syntax | see `issues/done/1413-implement-computedPropertyNameAndTypeParameterConflict.md` |
| 1414 | Implement Computedpropertynamewithimportedkey | spike | frontend/syntax | see `issues/done/1414-implement-computedPropertyNameWithImportedKey.md` |
| 1415 | Implement Computerpropertiesines | spike | frontend/syntax | see `issues/done/1415-implement-computerPropertiesInES.md` |
| 1416 | Implement Concatclassandstring | spike | frontend/resolver | see `issues/done/1416-implement-concatClassAndString.md` |
| 1417 | Implement Conditionalequalityonliteralobjects | spike | frontend/semantics | see `issues/done/1417-implement-conditionalEqualityOnLiteralObjects.md` |
| 1418 | Implement Conditionalexpression | spike | frontend/semantics | see `issues/done/1418-implement-conditionalExpression.md` |
| 1419 | Implement Conditionalexpressionnewline | spike | frontend/resolver | see `issues/done/1419-implement-conditionalExpressionNewLine.md` |
| 1420 | Implement Conditionalexpressions | spike | frontend/semantics | see `issues/done/1420-implement-conditionalExpressions.md` |
| 1421 | Implement Conditionalreturnexpression | spike | frontend/resolver | see `issues/done/1421-implement-conditionalReturnExpression.md` |
| 1422 | Implement Conditionaltypeassignabilitywhendeferred | spike | frontend/syntax | see `issues/done/1422-implement-conditionalTypeAssignabilityWhenDeferred.md` |
| 1423 | Implement Conditionaltypebasedcontextualtypereturntypewidening | spike | frontend/semantics | see `issues/done/1423-implement-conditionalTypeBasedContextualTypeReturnTypeWidening.md` |
| 1424 | Implement Conditionaltypeclassmembers | spike | frontend/semantics | see `issues/done/1424-implement-conditionalTypeClassMembers.md` |
| 1425 | Implement Conditionaltypediscriminatinglargeunionregulartypefetchingspeedreasonable | spike | frontend/syntax | see `issues/done/1425-implement-conditionalTypeDiscriminatingLargeUnionRegularTypeFetchingSpeedReasonable.md` |
| 1426 | Implement Conditionaltypedoesntspinforever | spike | frontend/syntax | see `issues/done/1426-implement-conditionalTypeDoesntSpinForever.md` |
| 1427 | Implement Conditionaltyperelaxingconstraintassignability | spike | frontend/syntax | see `issues/done/1427-implement-conditionalTypeRelaxingConstraintAssignability.md` |
| 1428 | Implement Conditionaltypesubclassextendstypeparam | spike | frontend/semantics | see `issues/done/1428-implement-conditionalTypeSubclassExtendsTypeParam.md` |
| 1429 | Implement Conditionaltypessimplifywhentrivial | spike | frontend/semantics | see `issues/done/1429-implement-conditionalTypesSimplifyWhenTrivial.md` |
| 1430 | Implement Conditionallyduplicateoverloadscausedbyoverloadresolution | spike | frontend/syntax | see `issues/done/1430-implement-conditionallyDuplicateOverloadsCausedByOverloadResolution.md` |
| 1431 | Implement Conflictmarkerdiff Parser Syntax | spike | frontend/syntax | see `issues/done/1431-implement-conflictMarkerDiff-parser-syntax.md` |
| 1432 | Implement Conflictmarkerdiff Unknown Unsupported | spike | frontend/syntax | see `issues/done/1432-implement-conflictMarkerDiff-unknown-unsupported.md` |
| 1433 | Implement Conflictmarkertrivia Parser Syntax | spike | frontend/syntax | see `issues/done/1433-implement-conflictMarkerTrivia-parser-syntax.md` |
| 1434 | Implement Conflictmarkertrivia Unknown Unsupported | spike | frontend/syntax | see `issues/done/1434-implement-conflictMarkerTrivia-unknown-unsupported.md` |
| 1435 | Implement Conflictingdeclarationsimportfromnamespace | spike | frontend/syntax | see `issues/done/1435-implement-conflictingDeclarationsImportFromNamespace.md` |
| 1436 | Implement Conflictingtypeannotatedvar | spike | reference/triage | see `issues/done/1436-implement-conflictingTypeAnnotatedVar.md` |
| 1437 | Implement Conflictingtypeparametersymboltransfer | spike | frontend/syntax | see `issues/done/1437-implement-conflictingTypeParameterSymbolTransfer.md` |
| 1438 | Implement Consistentaliasvsnonaliasrecordbehavior | spike | frontend/syntax | see `issues/done/1438-implement-consistentAliasVsNonAliasRecordBehavior.md` |
| 1439 | Implement Constdeclarationshadowedbyvardeclaration | spike | frontend/syntax | see `issues/done/1439-implement-constDeclarationShadowedByVarDeclaration.md` |
| 1440 | Implement Constdeclarations Import Export | spike | frontend/syntax | see `issues/done/1440-implement-constDeclarations-import-export.md` |
| 1441 | Implement Constdeclarations Name Resolution | spike | frontend/resolver | see `issues/done/1441-implement-constDeclarations-name-resolution.md` |
| 1442 | Implement Constdeclarations Parser Syntax | spike | frontend/syntax | see `issues/done/1442-implement-constDeclarations-parser-syntax.md` |
| 1443 | Implement Constdeclarations Scope Analysis | spike | frontend/resolver | see `issues/done/1443-implement-constDeclarations-scope-analysis.md` |
| 1444 | Implement Constdeclarations Unknown Unsupported | spike | frontend/syntax | see `issues/done/1444-implement-constDeclarations-unknown-unsupported.md` |
| 1445 | Implement Constenumbadpropertynames | spike | frontend/syntax | see `issues/done/1445-implement-constEnumBadPropertyNames.md` |
| 1446 | Implement Constenumdeclarations | spike | frontend/syntax | see `issues/done/1446-implement-constEnumDeclarations.md` |
| 1447 | Implement Constenumerrors | spike | frontend/syntax | see `issues/done/1447-implement-constEnumErrors.md` |
| 1448 | Implement Constenumexternalmodule | spike | frontend/syntax | see `issues/done/1448-implement-constEnumExternalModule.md` |
| 1449 | Implement Constenummergingwithvalues Import Export | spike | frontend/syntax | see `issues/done/1449-implement-constEnumMergingWithValues-import-export.md` |
| 1450 | Implement Constenummergingwithvalues Parser Syntax | spike | frontend/syntax | see `issues/done/1450-implement-constEnumMergingWithValues-parser-syntax.md` |
| 1451 | Implement Constenumnamespacereferencecausesnoimport | spike | frontend/syntax | see `issues/done/1451-implement-constEnumNamespaceReferenceCausesNoImport.md` |
| 1452 | Implement Constenumnoemitreexport | spike | frontend/syntax | see `issues/done/1452-implement-constEnumNoEmitReexport.md` |
| 1453 | Implement Constenumnopreservedeclarationreexport | spike | frontend/syntax | see `issues/done/1453-implement-constEnumNoPreserveDeclarationReexport.md` |
| 1454 | Implement Constenumonlymodulemerging | spike | frontend/syntax | see `issues/done/1454-implement-constEnumOnlyModuleMerging.md` |
| 1455 | Implement Constenumpreserveemitnamedexport | spike | frontend/syntax | see `issues/done/1455-implement-constEnumPreserveEmitNamedExport.md` |
| 1456 | Implement Constenumpreserveemitreexport | spike | frontend/syntax | see `issues/done/1456-implement-constEnumPreserveEmitReexport.md` |
| 1457 | Implement Constenumsyntheticnodescomments | spike | frontend/syntax | see `issues/done/1457-implement-constEnumSyntheticNodesComments.md` |
| 1458 | Implement Constenumtostringnocomments | spike | frontend/syntax | see `issues/done/1458-implement-constEnumToStringNoComments.md` |
| 1459 | Implement Constenumtostringwithcomments | spike | frontend/syntax | see `issues/done/1459-implement-constEnumToStringWithComments.md` |
| 1460 | Implement Constenums | spike | frontend/syntax | see `issues/done/1460-implement-constEnums.md` |
| 1461 | Implement Constinclassexpression | spike | frontend/syntax | see `issues/done/1461-implement-constInClassExpression.md` |
| 1462 | Implement Constindexedaccess | spike | frontend/syntax | see `issues/done/1462-implement-constIndexedAccess.md` |
| 1463 | Implement Constwithnonnull | spike | frontend/syntax | see `issues/done/1463-implement-constWithNonNull.md` |
| 1464 | Implement Constantenumassert | spike | frontend/syntax | see `issues/done/1464-implement-constantEnumAssert.md` |
| 1465 | Implement Constraintcheckingenericbasetypereference | spike | frontend/semantics | see `issues/done/1465-implement-constraintCheckInGenericBaseTypeReference.md` |
| 1466 | Implement Constraints | spike | frontend/resolver | see `issues/done/1466-implement-constraints.md` |
| 1467 | Implement Constraintsthatreferenceothercontstraints | spike | frontend/syntax | see `issues/done/1467-implement-constraintsThatReferenceOtherContstraints.md` |
| 1468 | Implement Constraintsusedinprototypeproperty | spike | frontend/syntax | see `issues/done/1468-implement-constraintsUsedInPrototypeProperty.md` |
| 1469 | Implement Constructorargwithgenericcallsignature | spike | frontend/syntax | see `issues/done/1469-implement-constructorArgWithGenericCallSignature.md` |
| 1470 | Implement Constructorargserrors | spike | frontend/syntax | see `issues/done/1470-implement-constructorArgsErrors.md` |
| 1471 | Implement Constructorastype | spike | frontend/resolver | see `issues/done/1471-implement-constructorAsType.md` |
| 1472 | Implement Constructorinvocationwithtoofewtypeargs | spike | frontend/syntax | see `issues/done/1472-implement-constructorInvocationWithTooFewTypeArgs.md` |
| 1473 | Implement Constructoroverloads Import Export | spike | frontend/syntax | see `issues/done/1473-implement-constructorOverloads-import-export.md` |
| 1474 | Implement Constructoroverloads Name Resolution | spike | frontend/resolver | see `issues/done/1474-implement-constructorOverloads-name-resolution.md` |
| 1475 | Implement Constructoroverloads Parser Syntax | spike | frontend/syntax | see `issues/done/1475-implement-constructorOverloads-parser-syntax.md` |
| 1476 | Implement Constructorparametersinvariabledeclarations | spike | frontend/syntax | see `issues/done/1476-implement-constructorParametersInVariableDeclarations.md` |
| 1477 | Implement Constructorparametersthatshadowexternalnamesinvariabledeclarations | spike | frontend/syntax | see `issues/done/1477-implement-constructorParametersThatShadowExternalNamesInVariableDeclarations.md` |
| 1478 | Implement Constructorreturningaprimitive | spike | frontend/syntax | see `issues/done/1478-implement-constructorReturningAPrimitive.md` |
| 1479 | Implement Constructorstaticparamname | spike | frontend/syntax | see `issues/done/1479-implement-constructorStaticParamName.md` |
| 1480 | Implement Constructorstaticparamnameerrors | spike | frontend/syntax | see `issues/done/1480-implement-constructorStaticParamNameErrors.md` |
| 1481 | Implement Constructorwithcapturedsuper | spike | frontend/syntax | see `issues/done/1481-implement-constructorWithCapturedSuper.md` |
| 1482 | Implement Constructorwithincompletetypeannotation | spike | frontend/syntax | see `issues/done/1482-implement-constructorWithIncompleteTypeAnnotation.md` |
| 1483 | Implement Constructorwithparameterpropertiesandprivatefields | spike | frontend/syntax | see `issues/done/1483-implement-constructorWithParameterPropertiesAndPrivateFields.md` |
| 1484 | Implement Constructorwithsuperandprologue | spike | frontend/syntax | see `issues/done/1484-implement-constructorWithSuperAndPrologue.md` |
| 1485 | Implement Constructorswithspecializedsignatures | spike | frontend/syntax | see `issues/done/1485-implement-constructorsWithSpecializedSignatures.md` |
| 1486 | Implement Contextsensitivereturntypeinference | spike | frontend/resolver | see `issues/done/1486-implement-contextSensitiveReturnTypeInference.md` |
| 1487 | Implement Contextualcomputednonbindablepropertytype | spike | frontend/syntax | see `issues/done/1487-implement-contextualComputedNonBindablePropertyType.md` |
| 1488 | Implement Contextualexpressiontypecheckingdoesntblowstack | spike | frontend/syntax | see `issues/done/1488-implement-contextualExpressionTypecheckingDoesntBlowStack.md` |
| 1489 | Implement Contextualoutertypeparameters | spike | frontend/syntax | see `issues/done/1489-implement-contextualOuterTypeParameters.md` |
| 1490 | Implement Contextualoverloadlistfromarrayunion | spike | frontend/syntax | see `issues/done/1490-implement-contextualOverloadListFromArrayUnion.md` |
| 1491 | Implement Contextualparamtypevsnestedreturntypeinference | spike | frontend/semantics | see `issues/done/1491-implement-contextualParamTypeVsNestedReturnTypeInference.md` |
| 1492 | Implement Contextualparameterandselfreferentialconstraint | spike | frontend/syntax | see `issues/done/1492-implement-contextualParameterAndSelfReferentialConstraint.md` |
| 1493 | Implement Contextualpropertyofgenericfilteringmappedtype | spike | frontend/semantics | see `issues/done/1493-implement-contextualPropertyOfGenericFilteringMappedType.md` |
| 1494 | Implement Contextualpropertyofgenericmappedtype | spike | frontend/resolver | see `issues/done/1494-implement-contextualPropertyOfGenericMappedType.md` |
| 1495 | Implement Contextualreturntypeofiife Import Export | spike | frontend/syntax | see `issues/done/1495-implement-contextualReturnTypeOfIIFE-import-export.md` |
| 1496 | Implement Contextualreturntypeofiife Unknown Unsupported | spike | frontend/syntax | see `issues/done/1496-implement-contextualReturnTypeOfIIFE-unknown-unsupported.md` |
| 1497 | Implement Contextualsignatureconditionaltypeinstantiationusingdefault | spike | frontend/semantics | see `issues/done/1497-implement-contextualSignatureConditionalTypeInstantiationUsingDefault.md` |
| 1498 | Implement Contextualsignatureinarrayelementlibes | spike | frontend/syntax | see `issues/done/1498-implement-contextualSignatureInArrayElementLibEs.md` |
| 1499 | Implement Contextualsignatureinobjectfreeze | spike | frontend/resolver | see `issues/done/1499-implement-contextualSignatureInObjectFreeze.md` |
| 1500 | Implement Contextualsignatureinstantiation Duplicate Local | spike | reference/triage | see `issues/done/1500-implement-contextualSignatureInstantiation-duplicate-local.md` |
| 1501 | Implement Contextualsignatureinstantiation Parser Syntax | spike | frontend/syntax | see `issues/done/1501-implement-contextualSignatureInstantiation-parser-syntax.md` |
| 1502 | Implement Contextualsignatureinstantiation Unknown Unsupported | spike | frontend/syntax | see `issues/done/1502-implement-contextualSignatureInstantiation-unknown-unsupported.md` |
| 1503 | Implement Contextualsignatureinstantiationwithtypeparameterconstrainedtooutertypeparameter | spike | frontend/syntax | see `issues/done/1503-implement-contextualSignatureInstantiationWithTypeParameterConstrainedToOuterTypeParameter.md` |
| 1504 | Implement Contextualsignatureinstatiationcontravariance | spike | frontend/resolver | see `issues/done/1504-implement-contextualSignatureInstatiationContravariance.md` |
| 1505 | Implement Contextualtupletypeparameterreadonly | spike | frontend/syntax | see `issues/done/1505-implement-contextualTupleTypeParameterReadonly.md` |
| 1506 | Implement Contextualtypearrayreturntype | spike | frontend/syntax | see `issues/done/1506-implement-contextualTypeArrayReturnType.md` |
| 1507 | Implement Contextualtypebasedonintersectionwithanyinthemix Name Resolution | spike | frontend/resolver | see `issues/done/1507-implement-contextualTypeBasedOnIntersectionWithAnyInTheMix-name-resolution.md` |
| 1508 | Implement Contextualtypebasedonintersectionwithanyinthemix Unknown Unsupported | spike | frontend/syntax | see `issues/done/1508-implement-contextualTypeBasedOnIntersectionWithAnyInTheMix-unknown-unsupported.md` |
| 1509 | Implement Contextualtypecaching | spike | frontend/syntax | see `issues/done/1509-implement-contextualTypeCaching.md` |
| 1510 | Implement Contextualtypeforinitalizedvariablesfiltersundefined | spike | reference/triage | see `issues/done/1510-implement-contextualTypeForInitalizedVariablesFiltersUndefined.md` |
| 1511 | Implement Contextualtypefunctionobjectpropertyintersection | spike | frontend/syntax | see `issues/done/1511-implement-contextualTypeFunctionObjectPropertyIntersection.md` |
| 1512 | Implement Contextualtypeiterableunions | spike | frontend/syntax | see `issues/done/1512-implement-contextualTypeIterableUnions.md` |
| 1513 | Implement Contextualtypeofindexedaccessparameter | spike | frontend/resolver | see `issues/done/1513-implement-contextualTypeOfIndexedAccessParameter.md` |
| 1514 | Implement Contextualtypeonyield | spike | frontend/syntax | see `issues/done/1514-implement-contextualTypeOnYield.md` |
| 1515 | Implement Contextualtypeselfreferencing | spike | frontend/resolver | see `issues/done/1515-implement-contextualTypeSelfReferencing.md` |
| 1516 | Implement Contextualtypeshouldbeliteral | spike | reference/triage | see `issues/done/1516-implement-contextualTypeShouldBeLiteral.md` |
| 1517 | Implement Contextualtypesnegatedtypelikeconstraintingenericmappedtype | spike | frontend/syntax | see `issues/done/1517-implement-contextualTypesNegatedTypeLikeConstraintInGenericMappedType.md` |
| 1518 | Implement Contextualtyping Import Export | spike | frontend/syntax | see `issues/done/1518-implement-contextualTyping-import-export.md` |
| 1519 | Implement Contextualtyping Parser Syntax | spike | frontend/syntax | see `issues/done/1519-implement-contextualTyping-parser-syntax.md` |
| 1520 | Implement Contextualtyping Unknown Unsupported | spike | frontend/syntax | see `issues/done/1520-implement-contextualTyping-unknown-unsupported.md` |
| 1521 | Implement Contextualtypingarraydestructuringwithdefaults | spike | reference/triage | see `issues/done/1521-implement-contextualTypingArrayDestructuringWithDefaults.md` |
| 1522 | Implement Contextualtypingfunctionreturningfunction | spike | frontend/syntax | see `issues/done/1522-implement-contextualTypingFunctionReturningFunction.md` |
| 1523 | Implement Contextualtypingofaccessors | spike | frontend/syntax | see `issues/done/1523-implement-contextualTypingOfAccessors.md` |
| 1524 | Implement Contextualtypingofarrayliterals | spike | frontend/syntax | see `issues/done/1524-implement-contextualTypingOfArrayLiterals.md` |
| 1525 | Implement Contextualtypingofconditionalexpression | spike | frontend/semantics | see `issues/done/1525-implement-contextualTypingOfConditionalExpression.md` |
| 1526 | Implement Contextualtypingofgenericfunctiontypedarguments | spike | frontend/semantics | see `issues/done/1526-implement-contextualTypingOfGenericFunctionTypedArguments.md` |
| 1527 | Implement Contextualtypingoflambdareturnexpression | spike | frontend/syntax | see `issues/done/1527-implement-contextualTypingOfLambdaReturnExpression.md` |
| 1528 | Implement Contextualtypingoflambdawithmultiplesignatures | spike | frontend/syntax | see `issues/done/1528-implement-contextualTypingOfLambdaWithMultipleSignatures.md` |
| 1529 | Implement Contextualtypingoftooshortoverloads | spike | frontend/syntax | see `issues/done/1529-implement-contextualTypingOfTooShortOverloads.md` |
| 1530 | Implement Contextualtypingreturnstatementwithreturntypeannotation | spike | frontend/resolver | see `issues/done/1530-implement-contextualTypingReturnStatementWithReturnTypeAnnotation.md` |
| 1531 | Implement Contextualtypingtwoinstancesofsametypeparameter | spike | frontend/syntax | see `issues/done/1531-implement-contextualTypingTwoInstancesOfSameTypeParameter.md` |
| 1532 | Implement Contextualtypingwithfixedtypeparameters | spike | frontend/syntax | see `issues/done/1532-implement-contextualTypingWithFixedTypeParameters.md` |
| 1533 | Implement Contextualtypingwithgenericandnongenericsignature | spike | frontend/semantics | see `issues/done/1533-implement-contextualTypingWithGenericAndNonGenericSignature.md` |
| 1534 | Implement Contextualtypingwithgenericsignature | spike | frontend/semantics | see `issues/done/1534-implement-contextualTypingWithGenericSignature.md` |
| 1535 | Implement Contextuallytypeargumentskeyword | spike | frontend/syntax | see `issues/done/1535-implement-contextuallyTypeArgumentsKeyword.md` |
| 1536 | Implement Contextuallytypeasyncfunctionreturntypefromunion | spike | runtime/builtins | see `issues/done/1536-implement-contextuallyTypeAsyncFunctionReturnTypeFromUnion.md` |
| 1537 | Implement Contextuallytypegeneratorreturntypefromunion | spike | runtime/builtins | see `issues/done/1537-implement-contextuallyTypeGeneratorReturnTypeFromUnion.md` |
| 1538 | Implement Contextuallytypedbooleanliterals | spike | frontend/resolver | see `issues/done/1538-implement-contextuallyTypedBooleanLiterals.md` |
| 1539 | Implement Contextuallytypedbydiscriminableunion Parser Syntax | spike | frontend/syntax | see `issues/done/1539-implement-contextuallyTypedByDiscriminableUnion-parser-syntax.md` |
| 1540 | Implement Contextuallytypedbydiscriminableunion Unknown Unsupported | spike | frontend/syntax | see `issues/done/1540-implement-contextuallyTypedByDiscriminableUnion-unknown-unsupported.md` |
| 1543 | Implement Contextuallytypedparametersoptionalinjsdoc | spike | reference/triage | see `issues/done/1543-implement-contextuallyTypedParametersOptionalInJSDoc.md` |
| 1546 | Implement Contextuallytypedparameterswithinitializers Unknown Unsupported | spike | frontend/syntax | see `issues/done/1546-implement-contextuallyTypedParametersWithInitializers-unknown-unsupported.md` |
| 1547 | Implement Contextuallytypedparameterswithquestiontoken | spike | reference/triage | see `issues/done/1547-implement-contextuallyTypedParametersWithQuestionToken.md` |
| 1548 | Implement Contextuallytypedsymbolnamedproperties | spike | frontend/syntax | see `issues/done/1548-implement-contextuallyTypedSymbolNamedProperties.md` |
| 1549 | Implement Contextuallytypingrestparameters | spike | reference/triage | see `issues/done/1549-implement-contextuallyTypingRestParameters.md` |
| 2050 | Implement Duplicatelocalvariable Duplicate Local | spike | reference/triage | see `issues/done/2050-implement-duplicateLocalVariable-duplicate-local.md` |
| 3002 | Implement Isolateddeclarationerrorsdefault | spike | runtime/builtins | see `issues/done/3002-implement-isolatedDeclarationErrorsDefault.md` |
| 3029 | Implement Isolatedmodulesnonambientconstenum | spike | frontend/syntax | see `issues/done/3029-implement-isolatedModulesNonAmbientConstEnum.md` |
| 3131 | Implement Jsxfactorynotidentifierorqualifiedname | spike | reference/triage | see `issues/done/3131-implement-jsxFactoryNotIdentifierOrQualifiedName.md` |
| 3305 | Implement Moduleandinterfacewithsamename | spike | frontend/syntax | see `issues/done/3305-implement-moduleAndInterfaceWithSameName.md` |
| 3306 | Implement Moduleasbasetype | spike | frontend/syntax | see `issues/done/3306-implement-moduleAsBaseType.md` |
| 3307 | Implement Moduleassignmentcompat | spike | frontend/syntax | see `issues/done/3307-implement-moduleAssignmentCompat.md` |
| 3308 | Implement Moduleaugmentationcollidingnamesinaugmentation | spike | frontend/syntax | see `issues/done/3308-implement-moduleAugmentationCollidingNamesInAugmentation.md` |
| 3309 | Implement Moduleaugmentationdeclarationemit | spike | frontend/syntax | see `issues/done/3309-implement-moduleAugmentationDeclarationEmit.md` |
| 3310 | Implement Moduleaugmentationdisallowedextensions | spike | frontend/syntax | see `issues/done/3310-implement-moduleAugmentationDisallowedExtensions.md` |
| 3311 | Implement Moduleaugmentationdoesinterfacemergeofreexport | spike | frontend/syntax | see `issues/done/3311-implement-moduleAugmentationDoesInterfaceMergeOfReexport.md` |
| 3312 | Implement Moduleaugmentationdoesnamespaceenummergeofreexport | spike | frontend/syntax | see `issues/done/3312-implement-moduleAugmentationDoesNamespaceEnumMergeOfReexport.md` |
| 3313 | Implement Moduleaugmentationdoesnamespacemergeofreexport | spike | frontend/syntax | see `issues/done/3313-implement-moduleAugmentationDoesNamespaceMergeOfReexport.md` |
| 3314 | Implement Moduleaugmentationduringsyntheticdefaultcheck | spike | frontend/syntax | see `issues/done/3314-implement-moduleAugmentationDuringSyntheticDefaultCheck.md` |
| 3315 | Implement Moduleaugmentationenumclassmergeofreexportiserror | spike | frontend/syntax | see `issues/done/3315-implement-moduleAugmentationEnumClassMergeOfReexportIsError.md` |
| 3316 | Implement Moduleaugmentationextendambientmodule | spike | frontend/syntax | see `issues/done/3316-implement-moduleAugmentationExtendAmbientModule.md` |
| 3317 | Implement Moduleaugmentationextendfilemodule | spike | frontend/syntax | see `issues/done/3317-implement-moduleAugmentationExtendFileModule.md` |
| 3318 | Implement Moduleaugmentationglobal Import Export | spike | frontend/syntax | see `issues/done/3318-implement-moduleAugmentationGlobal-import-export.md` |
| 3319 | Implement Moduleaugmentationglobal Parser Syntax | spike | frontend/syntax | see `issues/done/3319-implement-moduleAugmentationGlobal-parser-syntax.md` |
| 3320 | Implement Moduleaugmentationimportsandexports | spike | frontend/syntax | see `issues/done/3320-implement-moduleAugmentationImportsAndExports.md` |
| 3321 | Implement Moduleaugmentationinambientmodule | spike | frontend/syntax | see `issues/done/3321-implement-moduleAugmentationInAmbientModule.md` |
| 3322 | Implement Moduleaugmentationindependency | spike | frontend/syntax | see `issues/done/3322-implement-moduleAugmentationInDependency.md` |
| 3323 | Implement Moduleaugmentationnonewnames | spike | frontend/syntax | see `issues/done/3323-implement-moduleAugmentationNoNewNames.md` |
| 3324 | Implement Moduleaugmentationofalias | spike | frontend/syntax | see `issues/done/3324-implement-moduleAugmentationOfAlias.md` |
| 3325 | Implement Moduleaugmentationwithnonexistentnamedimport | spike | frontend/syntax | see `issues/done/3325-implement-moduleAugmentationWithNonExistentNamedImport.md` |
| 3326 | Implement Moduleaugmentationsbundledoutput | spike | frontend/syntax | see `issues/done/3326-implement-moduleAugmentationsBundledOutput.md` |
| 3327 | Implement Moduleaugmentationsimports | spike | frontend/syntax | see `issues/done/3327-implement-moduleAugmentationsImports.md` |
| 3328 | Implement Moduleclassarraycodegentest | spike | frontend/syntax | see `issues/done/3328-implement-moduleClassArrayCodeGenTest.md` |
| 3329 | Implement Modulecodegentest | spike | frontend/syntax | see `issues/done/3329-implement-moduleCodeGenTest.md` |
| 3330 | Implement Modulecodegentest | spike | frontend/syntax | see `issues/done/3330-implement-moduleCodegenTest.md` |
| 3331 | Implement Modulecrashbug | spike | frontend/syntax | see `issues/done/3331-implement-moduleCrashBug.md` |
| 3332 | Implement Moduledeclarationexportstarshadowingglobalisnameable | spike | frontend/syntax | see `issues/done/3332-implement-moduleDeclarationExportStarShadowingGlobalIsNameable.md` |
| 3333 | Implement Moduledetectionisolatedmodulescjsfilescope | spike | reference/triage | see `issues/done/3333-implement-moduleDetectionIsolatedModulesCjsFileScope.md` |
| 3334 | Implement Moduleduplicateidentifiers | spike | frontend/syntax | see `issues/done/3334-implement-moduleDuplicateIdentifiers.md` |
| 3335 | Implement Moduleelementsinwrongcontext | spike | frontend/syntax | see `issues/done/3335-implement-moduleElementsInWrongContext.md` |
| 3336 | Implement Moduleexportnonstructured | spike | frontend/syntax | see `issues/done/3336-implement-moduleExportNonStructured.md` |
| 3337 | Implement Moduleexports | spike | reference/triage | see `issues/done/3337-implement-moduleExports.md` |
| 3338 | Implement Moduleexportstypenoexcesspropertycheckfromcontainedliteral | spike | reference/triage | see `issues/done/3338-implement-moduleExportsTypeNoExcessPropertyCheckFromContainedLiteral.md` |
| 3339 | Implement Moduleexportsunaryexpression | spike | reference/triage | see `issues/done/3339-implement-moduleExportsUnaryExpression.md` |
| 3340 | Implement Moduleidentifiers | spike | frontend/syntax | see `issues/done/3340-implement-moduleIdentifiers.md` |
| 3341 | Implement Moduleimport | spike | reference/triage | see `issues/done/3341-implement-moduleImport.md` |
| 3342 | Implement Moduleimportedfortypeargumentposition | spike | reference/triage | see `issues/done/3342-implement-moduleImportedForTypeArgumentPosition.md` |
| 3343 | Implement Moduleintypeposition | maintenance | frontend/syntax | see `issues/done/3343-implement-moduleInTypePosition.md` |
| 3344 | Implement Modulekeyworddeprecated | maintenance | frontend/syntax | see `issues/done/3344-implement-moduleKeywordDeprecated.md` |
| 3345 | Implement Modulekeywordrepeaterror | maintenance | frontend/syntax | see `issues/done/3345-implement-moduleKeywordRepeatError.md` |
| 3346 | Implement Modulelocalimportnotincorrectlyredirected | maintenance | frontend/syntax | see `issues/done/3346-implement-moduleLocalImportNotIncorrectlyRedirected.md` |
| 3347 | Implement Modulemembermissingerrorisrelative | maintenance | frontend/syntax | see `issues/done/3347-implement-moduleMemberMissingErrorIsRelative.md` |
| 3348 | Implement Modulememberwithouttypeannotation | maintenance | frontend/syntax | see `issues/done/3348-implement-moduleMemberWithoutTypeAnnotation.md` |
| 3349 | Implement Modulemerge | maintenance | frontend/syntax | see `issues/done/3349-implement-moduleMerge.md` |
| 3350 | Implement Modulemergeconstructor | maintenance | frontend/syntax | see `issues/done/3350-implement-moduleMergeConstructor.md` |
| 3351 | Implement Modulenewexportbug | maintenance | frontend/syntax | see `issues/done/3351-implement-moduleNewExportBug.md` |
| 3352 | Implement Modulenoemit | maintenance | frontend/syntax | see `issues/done/3352-implement-moduleNoEmit.md` |
| 3353 | Implement Modulenodedefaultimports | maintenance | frontend/syntax | see `issues/done/3353-implement-moduleNodeDefaultImports.md` |
| 3354 | Implement Modulenodeimportrequireemit | spike | frontend/syntax | see `issues/done/3354-implement-moduleNodeImportRequireEmit.md` |
| 3355 | Implement Modulenonedynamicimport | maintenance | frontend/syntax | see `issues/done/3355-implement-moduleNoneDynamicImport.md` |
| 3356 | Implement Modulenoneerrors | maintenance | frontend/syntax | see `issues/done/3356-implement-moduleNoneErrors.md` |
| 3357 | Implement Moduleouterqualification | maintenance | frontend/syntax | see `issues/done/3357-implement-moduleOuterQualification.md` |
| 3358 | Implement Modulepreserve | maintenance | frontend/syntax | see `issues/done/3358-implement-modulePreserve.md` |
| 3359 | Implement Modulepreserveimporthelpers | maintenance | frontend/syntax | see `issues/done/3359-implement-modulePreserveImportHelpers.md` |
| 3360 | Implement Modulepreservetoplevelawait | maintenance | reference/triage | see `issues/done/3360-implement-modulePreserveTopLevelAwait.md` |
| 3361 | Implement Moduleprologueamd | maintenance | frontend/syntax | see `issues/done/3361-implement-modulePrologueAMD.md` |
| 3362 | Implement Moduleprologuecommonjs | maintenance | frontend/syntax | see `issues/done/3362-implement-modulePrologueCommonjs.md` |
| 3363 | Implement Moduleprologuees | maintenance | frontend/syntax | see `issues/done/3363-implement-modulePrologueES.md` |
| 3364 | Implement Moduleprologuesystem | maintenance | frontend/syntax | see `issues/done/3364-implement-modulePrologueSystem.md` |
| 3365 | Implement Moduleprologueumd | maintenance | frontend/syntax | see `issues/done/3365-implement-modulePrologueUmd.md` |
| 3366 | Implement Moduleproperty | maintenance | frontend/syntax | see `issues/done/3366-implement-moduleProperty.md` |
| 3367 | Implement Moduleredifinitionerrors | maintenance | frontend/syntax | see `issues/done/3367-implement-moduleRedifinitionErrors.md` |
| 3368 | Implement Modulereopenedtypeotherblock | maintenance | frontend/syntax | see `issues/done/3368-implement-moduleReopenedTypeOtherBlock.md` |
| 3369 | Implement Modulereopenedtypesameblock | maintenance | frontend/syntax | see `issues/done/3369-implement-moduleReopenedTypeSameBlock.md` |
| 3370 | Implement Moduleresolution Import Export | maintenance | frontend/syntax | see `issues/done/3370-implement-moduleResolution-import-export.md` |
| 3371 | Implement Moduleresolution Module Resolution | maintenance | frontend/syntax | see `issues/done/3371-implement-moduleResolution-module-resolution.md` |
| 3372 | Implement Moduleresolution Name Resolution (audit reopened #3372) | maintenance | frontend/resolver | see `issues/done/3372-implement-moduleResolution-name-resolution.md` |
| 3373 | Implement Moduleresolutionastypereferencedirective | maintenance | frontend/syntax | see `issues/done/3373-implement-moduleResolutionAsTypeReferenceDirective.md` |
| 3374 | Implement Moduleresolutionastypereferencedirectiveambient (audit reopened #3374) | maintenance | frontend/syntax | see `issues/done/3374-implement-moduleResolutionAsTypeReferenceDirectiveAmbient.md` |
| 3375 | Implement Moduleresolutionastypereferencedirectivescoped | maintenance | frontend/syntax | see `issues/done/3375-implement-moduleResolutionAsTypeReferenceDirectiveScoped.md` |
| 3376 | Implement Moduleresolutionnoresolve (audit reopened #3376) | maintenance | frontend/syntax | see `issues/done/3376-implement-moduleResolutionNoResolve.md` |
| 3377 | Implement Moduleresolutionnotscjs (audit reopened #3377) | maintenance | frontend/syntax | see `issues/done/3377-implement-moduleResolutionNoTsCJS.md` |
| 3378 | Implement Moduleresolutionnotsesm (audit reopened #3378) | maintenance | frontend/syntax | see `issues/done/3378-implement-moduleResolutionNoTsESM.md` |
| 3379 | Implement Moduleresolutionpackageidwithrelativeandabsolutepath (audit reopened #3379) | maintenance | frontend/syntax | see `issues/done/3379-implement-moduleResolutionPackageIdWithRelativeAndAbsolutePath.md` |
| 3380 | Implement Moduleresolutionwithextensions Import Export (audit reopened #3380) | maintenance | frontend/syntax | see `issues/done/3380-implement-moduleResolutionWithExtensions-import-export.md` |
| 3381 | Implement Moduleresolutionwithextensions Module Resolution | maintenance | compiler/multi-section | see `issues/done/3381-implement-moduleResolutionWithExtensions-module-resolution.md` |
| 3382 | Implement Moduleresolutionwithmodule (audit reopened #3382) | maintenance | compiler/multi-section | see `issues/done/3382-implement-moduleResolutionWithModule.md` |
| 3383 | Implement Moduleresolutionwithrequire (audit reopened #3383) | maintenance | compiler/module-graph | see `issues/done/3383-implement-moduleResolutionWithRequire.md` |
| 3384 | Implement Moduleresolutionwithrequireandimport | maintenance | compiler/module-graph | see `issues/done/3384-implement-moduleResolutionWithRequireAndImport.md` |
| 3385 | Implement Moduleresolutionwithsuffixes Import Export (audit reopened #3385) | maintenance | compiler/multi-section | see `issues/done/3385-implement-moduleResolutionWithSuffixes-import-export.md` |
| 3386 | Implement Moduleresolutionwithsuffixes Module Resolution | maintenance | compiler/multi-section | see `issues/done/3386-implement-moduleResolutionWithSuffixes-module-resolution.md` |
| 3387 | Implement Moduleresolutionwithsymlinks Import Export | maintenance | compiler/module-graph | see `issues/done/3387-implement-moduleResolutionWithSymlinks-import-export.md` |
| 3388 | Split moduleResolutionWithSymlinks parser-syntax bucket | maintenance | compiler/module-resolution | see `issues/done/3388-implement-moduleResolutionWithSymlinks-parser-syntax.md` |
| 3389 | Close moduleSameValueDuplicateExportedBindings bucket to virtual re-export owner | maintenance | compiler/module-graph | see `issues/done/3389-implement-moduleSameValueDuplicateExportedBindings.md` |
| 3390 | Close moduleScopingBug generated bucket after build pass | maintenance | frontend/syntax | see `issues/done/3390-implement-moduleScopingBug.md` |
| 3391 | Close moduleSharesNameWithImportDeclarationInsideIt bucket after build pass | maintenance | frontend/syntax | see `issues/done/3391-implement-moduleSharesNameWithImportDeclarationInsideIt.md` |
| 3392 | Close moduleSymbolMerging bucket to namespace-only multi-section owner | maintenance | compiler | see `issues/done/3392-implement-moduleSymbolMerging.md` |
| 3393 | Close moduleUnassignedVariable bucket after build pass | maintenance | frontend/syntax | see `issues/done/3393-implement-moduleUnassignedVariable.md` |
| 3394 | Close moduleVariableArrayIndexer bucket after build pass | maintenance | frontend/syntax | see `issues/done/3394-implement-moduleVariableArrayIndexer.md` |
| 3395 | Close moduleVariables bucket after build pass | maintenance | frontend/syntax | see `issues/done/3395-implement-moduleVariables.md` |
| 3396 | Close moduleVisibilityTest bucket to namespace qualified access owner | maintenance | frontend/name-resolution | see `issues/done/3396-implement-moduleVisibilityTest.md` |
| 3397 | Close moduleWithNoValuesAsType bucket to TS2709 variable annotation owner | maintenance | frontend/semantics | see `issues/done/3397-implement-moduleWithNoValuesAsType.md` |
| 3398 | Close moduleWithTryStatement bucket to namespace value owner | maintenance | frontend/name-resolution | see `issues/done/3398-implement-moduleWithTryStatement.md` |
| 3399 | Close moduleWithValuesAsType bucket to TS2709 variable annotation owner | maintenance | frontend/semantics | see `issues/done/3399-implement-moduleWithValuesAsType.md` |
| 3400 | Close moduledecl bucket to ambient namespace value owner | maintenance | frontend/resolver | see `issues/done/3400-implement-moduledecl.md` |
| 3401 | Close multiCallOverloads bucket to nested default-parameter closure owner | maintenance | ir/lowering | see `issues/done/3401-implement-multiCallOverloads.md` |
| 3402 | Split multiExtendsSplitInterfaces bucket to DOM self global issue | maintenance | frontend/resolver | see `issues/done/3402-implement-multiExtendsSplitInterfaces.md` |
| 3403 | Split multiImportExport bucket to exported require import-equals issue | maintenance | frontend/parser | see `issues/done/3403-implement-multiImportExport.md` |
| 3404 | Split multiLineErrors bucket to object return type parser issue | maintenance | frontend/parser | see `issues/done/3404-implement-multiLineErrors.md` |
| 3405 | Close multiLinePropertyAccessAndArrowFunctionIndent to TS1108 owner | maintenance | compiler/diagnostics | see `issues/done/3405-implement-multiLinePropertyAccessAndArrowFunctionIndent.md` |
| 3406 | Split multiModuleClodule to class namespace merge owner | maintenance | frontend/semantics | see `issues/done/3406-implement-multiModuleClodule.md` |
| 3407 | Close multiModuleFundule to function namespace merge owner | maintenance | frontend/semantics | see `issues/done/3407-implement-multiModuleFundule.md` |
| 3408 | Close multipleBaseInterfaesWithIncompatibleProperties as stale build-pass | maintenance | frontend/syntax | see `issues/done/3408-implement-multipleBaseInterfaesWithIncompatibleProperties.md` |
| 3409 | Close multipleClassPropertyModifiers to modified static field parser owner | maintenance | frontend | see `issues/done/3409-implement-multipleClassPropertyModifiers.md` |
| 3410 | Split multipleClassPropertyModifiersErrors to duplicate static modifier issue | maintenance | frontend/parser | see `issues/done/3410-implement-multipleClassPropertyModifiersErrors.md` |
| 3411 | Close multipleExportAssignments to CommonJS export assignment parser owner | maintenance | frontend/parser | see `issues/done/3411-implement-multipleExportAssignments.md` |
| 3412 | Split multipleExportAssignmentsInAmbientDeclaration to ambient duplicate export issue | maintenance | frontend/semantics | see `issues/done/3412-implement-multipleExportAssignmentsInAmbientDeclaration.md` |
| 3413 | Split multipleExports to namespace export declaration diagnostic | maintenance | frontend/semantics | see `issues/done/3413-implement-multipleExports.md` |
| 3414 | Close multipleInferenceContexts as stale build-pass | maintenance | frontend/resolver | see `issues/done/3414-implement-multipleInferenceContexts.md` |
| 3415 | Close multipleInheritance to multiple class heritage owner | maintenance | frontend/parser | see `issues/done/3415-implement-multipleInheritance.md` |
| 3416 | Split multivar to namespace var merge diagnostic | maintenance | frontend/semantics | see `issues/done/3416-implement-multivar.md` |
| 3417 | Close mutuallyRecursiveCallbacks to ambient var assignment owner | maintenance | frontend/resolver | see `issues/done/3417-implement-mutuallyRecursiveCallbacks.md` |
| 3418 | Split mutuallyRecursiveGenericBaseTypes bucket | maintenance | frontend/semantics | see `issues/done/3418-implement-mutuallyRecursiveGenericBaseTypes.md` |
| 3419 | Implement Mutuallyrecursiveinference | spike | frontend/semantics | see `issues/done/3419-implement-mutuallyRecursiveInference.md` |
| 3420 | Implement Mutuallyrecursiveinterfacedeclaration | spike | frontend/syntax | see `issues/done/3420-implement-mutuallyRecursiveInterfaceDeclaration.md` |
| 3421 | Implement Namecollisionwithblockscopedvariable | spike | frontend/syntax | see `issues/done/3421-implement-nameCollisionWithBlockScopedVariable.md` |
| 3422 | Implement Namecollisions | spike | frontend/syntax | see `issues/done/3422-implement-nameCollisions.md` |
| 3423 | Implement Namecollisionsinpropertyassignments | spike | frontend/syntax | see `issues/done/3423-implement-nameCollisionsInPropertyAssignments.md` |
| 3424 | Implement Namedfunctionexpressionassignedtoclassproperty | spike | frontend/syntax | see `issues/done/3424-implement-namedFunctionExpressionAssignedToClassProperty.md` |
| 3425 | Implement Namedfunctionexpressioncall | spike | frontend/syntax | see `issues/done/3425-implement-namedFunctionExpressionCall.md` |
| 3426 | Implement Namedfunctionexpressioncallerrors | spike | frontend/resolver | see `issues/done/3426-implement-namedFunctionExpressionCallErrors.md` |
| 3427 | Implement Namedfunctionexpressioninmodule | spike | frontend/syntax | see `issues/done/3427-implement-namedFunctionExpressionInModule.md` |
| 3428 | Implement Namedimportnonexistentname | spike | frontend/syntax | see `issues/done/3428-implement-namedImportNonExistentName.md` |
| 3429 | Implement Namespacedisambiguationinunion | spike | frontend/syntax | see `issues/done/3429-implement-namespaceDisambiguationInUnion.md` |
| 3430 | Implement Namespacemergedwithfunctionwithoverloadsusage | spike | frontend/syntax | see `issues/done/3430-implement-namespaceMergedWithFunctionWithOverloadsUsage.md` |
| 3431 | Implement Namespacemergedwithimportaliasnocrash | spike | frontend/syntax | see `issues/done/3431-implement-namespaceMergedWithImportAliasNoCrash.md` |
| 3432 | Implement Namespacenotmergedwithfunctiondefaultexport | spike | frontend/syntax | see `issues/done/3432-implement-namespaceNotMergedWithFunctionDefaultExport.md` |
| 3433 | Implement Namespaces | spike | frontend/syntax | see `issues/done/3433-implement-namespaces.md` |
| 3434 | Implement Namespacesdeclaration | spike | frontend/syntax | see `issues/done/3434-implement-namespacesDeclaration.md` |
| 3435 | Implement Namespaceswithtypealiasonlyexportsmerge | spike | frontend/syntax | see `issues/done/3435-implement-namespacesWithTypeAliasOnlyExportsMerge.md` |
| 3436 | Implement Nanequality | spike | frontend/resolver | see `issues/done/3436-implement-nanEquality.md` |
| 3437 | Implement Narrowbybooleancomparison | spike | frontend/syntax | see `issues/done/3437-implement-narrowByBooleanComparison.md` |
| 3438 | Implement Narrowbyclauseexpressioninswitchtrue Name Resolution | spike | frontend/resolver | see `issues/done/3438-implement-narrowByClauseExpressionInSwitchTrue-name-resolution.md` |
| 3439 | Implement Narrowbyclauseexpressioninswitchtrue Parser Syntax | spike | frontend/syntax | see `issues/done/3439-implement-narrowByClauseExpressionInSwitchTrue-parser-syntax.md` |
| 3440 | Implement Narrowbyclauseexpressioninswitchtrue Unknown Unsupported | spike | frontend/syntax | see `issues/done/3440-implement-narrowByClauseExpressionInSwitchTrue-unknown-unsupported.md` |
| 3441 | Implement Narrowbyequality | spike | frontend/syntax | see `issues/done/3441-implement-narrowByEquality.md` |
| 3442 | Implement Narrowbyinstanceof | spike | frontend/syntax | see `issues/done/3442-implement-narrowByInstanceof.md` |
| 3443 | Implement Narrowbyparenthesizedswitchexpression | spike | frontend/syntax | see `issues/done/3443-implement-narrowByParenthesizedSwitchExpression.md` |
| 3444 | Implement Narrowbyswitchdiscriminantundefinedcase | spike | frontend/syntax | see `issues/done/3444-implement-narrowBySwitchDiscriminantUndefinedCase.md` |
| 3690 | Implement Optionaltupleelementsandundefined | spike | reference/triage | see `issues/done/3690-implement-optionalTupleElementsAndUndefined.md` |
| 3996 | Implement Compiler (dup) | spike | frontend/syntax | see `issues/done/3996-implement-reference-typescript-tests-cases-compiler.md` |
| 4210 | Implement Splicetuples | spike | frontend/resolver | see `issues/done/4210-implement-spliceTuples.md` |
| 4228 | Implement Standalonebreak | spike | frontend/syntax | see `issues/done/4228-implement-standaloneBreak.md` |
| 4252 | Split staticModifierAlreadySeen to duplicate static modifier issue | maintenance | frontend/parser | see `issues/done/4252-implement-staticModifierAlreadySeen.md` |
| 4262 | Implement Staticsinconstructorbodies | spike | frontend/syntax | see `issues/done/4262-implement-staticsInConstructorBodies.md` |
| 4284 | Implement Stringincludes (audit reopened #4284) | spike | runtime/builtins | see `issues/done/4284-implement-stringIncludes.md` |
| 4291 | Implement Stringmatchall | spike | reference/triage | see `issues/done/4291-implement-stringMatchAll.md` |
| 4294 | Implement Stringtrim | spike | runtime/builtins | see `issues/done/4294-implement-stringTrim.md` |
| 4593 | Implement Typedarrayssubarray | spike | frontend/resolver | see `issues/done/4593-implement-typedArraysSubarray.md` |
| 4806 | Implement class syntax (dup) | spike | frontend/syntax | see `issues/done/4806-implement-class.md` |
| 4808 | Implement import/export module syntax (dup) | spike | frontend/syntax | see `issues/done/4808-implement-import-export.md` |
| 4809 | Implement name resolution (dup) | spike | frontend/resolver | see `issues/done/4809-implement-name-resolution.md` |
| 4810 | Implement object literal enhancements (dup) | spike | frontend/syntax | see `issues/done/4810-implement-object-literal.md` |
| 4811 | Implement parser syntax extensions (dup) | spike | frontend/syntax | see `issues/done/4811-implement-parser-syntax.md` |
| 4813 | Implement type-system support (dup) | spike | frontend/syntax | see `issues/done/4813-implement-type-system.md` |
| 4814 | Investigate and classify unknown-unsupported cases (dup) | spike | frontend/syntax | see `issues/done/4814-implement-unknown-unsupported.md` |
| 5000 | Meta: TypeScript Compiler Parser Syntax Coverage | meta | frontend/syntax | see file |
| 5001 | Meta: TypeScript Compiler Semantic Analysis Coverage | meta | frontend/semantics | see `issues/done/5001-meta-tsc-semantic-analysis.md` |
| 5002 | Meta: TypeScript Compiler Type System Coverage | meta | frontend/semantics | see `issues/done/5002-meta-tsc-type-system.md` |
| 5003 | Meta: TypeScript Compiler Declaration Emit Coverage | meta | frontend/syntax | see `issues/done/5003-meta-tsc-declaration-emit.md` |
| 5004 | Meta: Runtime Builtins Coverage (test262) (audit reopened #5004) | meta | runtime/builtins | see `issues/done/5004-meta-runtime-builtins.md` |
| 5005 | Meta: TypeScript Compiler Name Resolution Coverage | meta | frontend/resolver | see `issues/done/5005-meta-tsc-name-resolution.md` |
| 5006 | Meta: TypeScript Compiler Scope Analysis Coverage | meta | frontend/resolver | see `issues/done/5006-meta-tsc-scope-analysis.md` |
| 5007 | Meta: TypeScript Compiler Module Resolution Coverage (audit reopened #5007) | meta | frontend/resolver | see `issues/done/5007-meta-tsc-module-resolution.md` |
| 5008 | Implement static ES module export forms (default, named, namespace, re-export) (audit reopened #5008) | feature | ir/compiler | see `issues/done/5008-static-es-module-export-default-namespace-reexport.md` |
| 5009 | Remaining static ES module export forms (named list, default import, namespace, re-export, side-effect) (audit reopened #5009) | feature | ir/compiler | see `issues/done/5009-remaining-es-module-export-forms.md` |
| 5010 | Implement local named export (export { value } and export { value as alias }) for entry module (audit reopened #5010) | feature | ir/compiler | see `issues/done/5010-remaining-es-module-export-forms.md` |
| 5011 | Represent or reject class runtime values in lowered IR (audit reopened #5011) | feature | ir/backend | see `issues/done/5011-class-runtime-value-semantics.md` |
| 5013 | Implement duplicate-local support | spike | reference/triage | see `issues/done/5013-implement-duplicate-local.md` |
| 5014 | Implement eval support (dup) | spike | frontend/syntax | see `issues/done/5014-implement-eval.md` |
| 5016 | Implement function resolution (dup) | spike | frontend/resolver | see `issues/done/5016-implement-function-resolution.md` |
| 5017 | Implement html-comment support | spike | frontend/syntax | see `issues/done/5017-implement-html-comment.md` |
| 5019 | Implement name resolution (dup) | spike | frontend/resolver | see `issues/done/5019-implement-name-resolution.md` |
| 5021 | Implement string-builtin support | feature | frontend/syntax | see `issues/done/5021-implement-string-builtin.md` |
| 5022 | Implement Array.prototype.every receiver semantics for 2dArrays (audit reopened #5022) | feature | runtime/builtins | see `issues/done/5022-implement-array-every-receiver.md` |
| 5023 | Implement API Sample watcher arrow function return | feature | runtime/builtins | see `issues/done/5023-implement-api-sample-watcher-arrow.md` |
| 5024 | Implement anonymous interface new expression identifier | feature | runtime/builtins | see file |
| 5025 | Implement any as return type instanceof constructor RHS | feature | runtime/builtins | see file |
| 5026 | [backend-wasm] Implement real class declaration emission (audit reopened #5026) | feature | backend | see `issues/done/5026-backend-wasm-real-class-declaration.md` |
| 5027 | [backend-wasm] Replace throw-as-return with catchable exception runtime (audit reopened #5027) | feature | backend | see `issues/done/5027-backend-wasm-catchable-exception.md` |
| 5028 | [backend-wasm] Implement array growth and reallocation for push/write paths (audit reopened #5028) | feature | backend | see `issues/done/5028-backend-wasm-array-growth.md` |
| 5029 | [backend-wasm] Expand direct wasm binary emission beyond console.log string literal MVP (audit reopened #5029) | feature | backend | see `issues/done/5029-backend-wasm-direct-binary-emission.md` |
| 5030 | [backend-wasm] Split large runtime/WAT emitters into testable components (audit reopened #5030) | refactor | backend | see `issues/done/5030-backend-wasm-split-runtime-emitters.md` |
| 5031 | [cli] Replace placeholder parser keyword/operator tests with real assertions | test | cli | see `issues/done/5031-cli-real-parser-assertions.md` |
| 5032 | [cli] Add deterministic external tool capability detection | feature | cli | see `issues/done/5032-cli-tool-capability-detection.md` |
| 5033 | [cli] Normalize node-diff fixture reporting into structured records (audit reopened #5033) | feature | cli | see `issues/done/5033-cli-structured-node-diff-report.md` |
| 5034 | [cli] Add command contract tests for build/check/dump/server (audit reopened #5034) | test | cli | see `issues/done/5034-cli-command-contract-tests.md` |
| 5035 | [cli] Add --explain-unsupported diagnostics mode (audit reopened #5035) | feature | cli | see `issues/done/5035-cli-explain-unsupported.md` |
| 5036 | [compiler] Introduce CompileReport<T> for non-fatal diagnostics | feature | cli | see `issues/done/5036-compiler-compile-report.md` |
| 5037 | [compiler] Complete entry module export lowering for local references | feature | cli | see `issues/done/5037-compiler-module-export-lowering.md` |
| 5038 | [compiler] Harden module graph resolution and diagnostics (audit reopened #5038) | feature | cli | see `issues/done/5038-compiler-module-graph-resolution.md` |
| 5039 | [compiler] Stabilize test262 preprocessor feature handling (audit reopened #5039) | feature | cli | see `issues/done/5039-compiler-test262-preprocessor.md` |
| 5040 | [compiler] Add resource limits and cancellation to server batch compilation (audit reopened #5040) | feature | cli | see `issues/done/5040-compiler-resource-limits.md` |
| 5041 | [frontend] Complete Expr AST fixture coverage (audit reopened #5041) | test | frontend | see `issues/done/5041-frontend-expr-fixture-coverage.md` |
| 5042 | [frontend] Complete Stmt AST fixture coverage (audit reopened #5042) | test | frontend | see `issues/done/5042-frontend-stmt-fixture-coverage.md` |
| 5043 | [frontend] Split large lexer/parser files by grammar responsibility (audit reopened #5043) | refactor | frontend | see `issues/done/5043-frontend-split-parser.md` |
| 5044 | [frontend] Define and test TypeScript ambient declaration erasure boundaries (audit reopened #5044) | feature | frontend | see `issues/done/5044-frontend-ambient-erasure.md` |
| 5045 | [frontend] Improve syntax error recovery and source spans (audit reopened #5045) | feature | frontend | see `issues/done/5045-frontend-error-recovery.md` |
| 5046 | [ir] Design full class runtime IR representation | feature | ir | see `issues/done/5046-ir-class-ir.md` |
| 5047 | [ir] Implement env-cell lowering for outer-scope mutation | feature | ir | see `issues/done/5047-ir-env-cell.md` |
| 5048 | [ir] Broaden BigInt lowering beyond signed-i64/first-limb slice | feature | ir | see `issues/done/5048-ir-bigint-lowering.md` |
| 5049 | [ir] Complete destructuring, rest, and default binding lowering | feature | ir | see `issues/done/5049-ir-destructuring.md` |
| 5050 | [ir] Implement iterator protocol lowering for spread and for-of | feature | ir | see `issues/done/5050-ir-iterator-protocol.md` |
| 5051 | [runtime-abi] Add ABI layout golden tests and versioning | test | abi | see file |
| 5052 | [runtime-abi] Validate runtime memory map for overlap and headroom | feature | abi | see `issues/done/5052-abi-memory-map.md` |
| 5053 | [runtime-abi] Add typed wrappers for tagged values and heap pointers (audit reopened #5053) | refactor | abi | see `issues/done/5053-abi-typed-wrappers.md` |
| 5054 | [runtime-abi] Document value tags and object layout as public ABI (audit reopened #5054) | docs | abi | see `issues/done/5054-abi-document-layout.md` |
| 5055 | [runtime-abi] Add backward-compatibility tests for ABI constants (audit reopened #5055) | test | abi | see `issues/done/5055-abi-backward-compat.md` |
| 5056 | [shared] Replace manual TestRecord JSON construction with serde serialization | refactor | coverage | see `issues/done/5056-shared-serde-serialization.md` |
| 5057 | [shared] Version capability manifest schema and migration policy | feature | coverage | see `issues/done/5057-shared-manifest-versioning.md` |
| 5058 | [shared] Deduplicate and canonicalize capability reasons/imports | refactor | coverage | see `issues/done/5058-shared-deduplicate-capabilities.md` |
| 5059 | [shared] Add typed tracking IDs for unsupported and blocked tests | feature | coverage | see `issues/done/5059-shared-tracking-ids.md` |
| 5060 | [shared] Provide shared fixture schemas for CLI/compiler/backend tests | feature | coverage | see `issues/done/5060-shared-fixture-schemas.md` |
| 5061 | Implement arguments-object support (dup) | spike | frontend/syntax | see `issues/done/5061-implement-arguments-object.md` |
| 5062 | Implement arity support (dup) | spike | reference/triage | see `issues/done/5062-implement-arity.md` |
| 5063 | Implement array-builtin support (dup) | spike | frontend/syntax | see `issues/done/5063-implement-array-builtin.md` |
| 5064 | Implement arrow functions (dup) | spike | frontend/syntax | see `issues/done/5064-implement-arrow-function.md` |
| 5065 | Implement async/await support (dup) | spike | frontend/syntax | see `issues/done/5065-implement-async.md` |
| 5066 | Implement async-iteration support (dup) | spike | frontend/syntax | see `issues/done/5066-implement-async-iteration.md` |
| 5067 | Implement built-in API support (dup) | spike | frontend/syntax | see `issues/done/5067-implement-builtin-api.md` |
| 5068 | Implement class syntax (dup) | spike | frontend/syntax | see `issues/done/5068-implement-class.md` |
| 5069 | Implement class-accessor support (dup) | spike | frontend/syntax | see `issues/done/5069-implement-class-accessor.md` |
| 5070 | Implement Date object support (dup) | spike | frontend/syntax | see `issues/done/5070-implement-date.md` |
| 5071 | Implement declaration-emit support (dup) | spike | frontend/syntax | see `issues/done/5071-implement-declaration-emit.md` |
| 5072 | Implement destructuring (dup) | spike | frontend/syntax | see `issues/done/5072-implement-destructuring.md` |
| 5073 | Implement duplicate-function support (dup) | spike | reference/triage | see `issues/done/5073-implement-duplicate-function.md` |
| 5074 | Implement duplicate-local support | spike | reference/triage | see `issues/done/5074-implement-duplicate-local.md` |
| 5075 | Implement enum support (dup) | spike | frontend/syntax | see `issues/done/5075-implement-enum.md` |
| 5076 | Implement eval support (dup) | spike | reference/triage | see `issues/done/5076-implement-eval.md` |
| 5077 | Implement function support (dup) | spike | frontend/syntax | see `issues/done/5077-implement-function.md` |
| 5078 | Implement function resolution (dup) | spike | frontend/resolver | see `issues/done/5078-implement-function-resolution.md` |
| 5079 | Implement import/export module syntax (dup) | spike | frontend/syntax | see `issues/done/5079-implement-import-export.md` |
| 5080 | Implement legacy-global-builtin support (dup) | spike | frontend/syntax | see `issues/done/5080-implement-legacy-global-builtin.md` |
| 5081 | Implement module-resolution support (dup) | spike | frontend/syntax | see `issues/done/5081-implement-module-resolution.md` |
| 5082 | Implement name resolution (dup) | spike | frontend/resolver | see `issues/done/5082-implement-name-resolution.md` |
| 5083 | Implement negative-parse-syntaxerror support (dup) | spike | reference/triage | see `issues/done/5083-implement-negative-parse-syntaxerror.md` |
| 5084 | Implement object-builtin support (dup) | spike | frontend/syntax | see `issues/done/5084-implement-object-builtin.md` |
| 5085 | Implement object literal enhancements (dup) | spike | frontend/syntax | see `issues/done/5085-implement-object-literal.md` |
| 5086 | Implement RegExp literal support (dup) | spike | frontend/syntax | see `issues/done/5086-implement-regexp-literal.md` |
| 5087 | Implement scope-analysis support (dup) | spike | frontend/syntax | see `issues/done/5087-implement-scope-analysis.md` |
| 5088 | Implement string-builtin support | spike | frontend/syntax | see `issues/done/5088-implement-string-builtin.md` |
| 5089 | Implement type-alias support (dup) | spike | frontend/syntax | see `issues/done/5089-implement-type-alias.md` |
| 5090 | Implement type-system support (dup) | spike | frontend/syntax | see `issues/done/5090-implement-type-system.md` |
| 5091 | Investigate and classify unknown-unsupported cases (dup) | spike | frontend/syntax | see `issues/done/5091-implement-unknown-unsupported.md` |
| 5092 | (filler) Auto-generated gap placeholder #5092 | task | coverage | see `issues/done/5092-placeholder.md` |
| 5093 | (filler) Auto-generated gap placeholder #5093 | task | coverage | see `issues/done/5093-placeholder.md` |
| 5094 | (filler) Auto-generated gap placeholder #5094 | task | coverage | see `issues/done/5094-placeholder.md` |
| 5095 | (filler) Auto-generated gap placeholder #5095 | task | coverage | see `issues/done/5095-placeholder.md` |
| 5096 | (filler) Auto-generated gap placeholder #5096 | task | coverage | see `issues/done/5096-placeholder.md` |
| 5097 | (filler) Auto-generated gap placeholder #5097 | task | coverage | see `issues/done/5097-placeholder.md` |
| 5098 | (filler) Auto-generated gap placeholder #5098 | task | coverage | see `issues/done/5098-placeholder.md` |
| 5099 | (filler) Auto-generated gap placeholder #5099 | task | coverage | see `issues/done/5099-placeholder.md` |
| 5100 | (filler) Auto-generated gap placeholder #5100 | task | coverage | see `issues/done/5100-placeholder.md` |
| 5101 | (filler) Auto-generated gap placeholder #5101 | task | coverage | see `issues/done/5101-placeholder.md` |
| 5102 | (filler) Auto-generated gap placeholder #5102 | task | coverage | see `issues/done/5102-placeholder.md` |
| 5103 | (filler) Auto-generated gap placeholder #5103 | task | coverage | see `issues/done/5103-placeholder.md` |
| 5104 | (filler) Auto-generated gap placeholder #5104 | task | coverage | see `issues/done/5104-placeholder.md` |
| 5105 | (filler) Auto-generated gap placeholder #5105 | task | coverage | see `issues/done/5105-placeholder.md` |
| 5106 | (filler) Auto-generated gap placeholder #5106 | task | coverage | see `issues/done/5106-placeholder.md` |
| 5107 | (filler) Auto-generated gap placeholder #5107 | task | coverage | see `issues/done/5107-placeholder.md` |
| 5108 | (filler) Auto-generated gap placeholder #5108 | task | coverage | see `issues/done/5108-placeholder.md` |
| 5109 | (filler) Auto-generated gap placeholder #5109 | task | coverage | see `issues/done/5109-placeholder.md` |
| 5110 | (filler) Auto-generated gap placeholder #5110 | task | coverage | see `issues/done/5110-placeholder.md` |
| 5111 | (filler) Auto-generated gap placeholder #5111 | task | coverage | see `issues/done/5111-placeholder.md` |
| 5112 | (filler) Auto-generated gap placeholder #5112 | task | coverage | see `issues/done/5112-placeholder.md` |
| 5113 | (filler) Auto-generated gap placeholder #5113 | task | coverage | see `issues/done/5113-placeholder.md` |
| 5114 | (filler) Auto-generated gap placeholder #5114 | task | coverage | see `issues/done/5114-placeholder.md` |
| 5115 | (filler) Auto-generated gap placeholder #5115 | task | coverage | see `issues/done/5115-placeholder.md` |
| 5116 | (filler) Auto-generated gap placeholder #5116 | task | coverage | see `issues/done/5116-placeholder.md` |
| 5117 | (filler) Auto-generated gap placeholder #5117 | task | coverage | see `issues/done/5117-placeholder.md` |
| 5118 | (filler) Auto-generated gap placeholder #5118 | task | coverage | see `issues/done/5118-placeholder.md` |
| 5119 | (filler) Auto-generated gap placeholder #5119 | task | coverage | see `issues/done/5119-placeholder.md` |
| 5120 | (filler) Auto-generated gap placeholder #5120 | task | coverage | see `issues/done/5120-placeholder.md` |
| 5121 | (filler) Auto-generated gap placeholder #5121 | task | coverage | see `issues/done/5121-placeholder.md` |
| 5122 | (filler) Auto-generated gap placeholder #5122 | task | coverage | see `issues/done/5122-placeholder.md` |
| 5123 | (filler) Auto-generated gap placeholder #5123 | task | coverage | see `issues/done/5123-placeholder.md` |
| 5124 | Fix Object.keys on arguments exotic object (audit reopened #5124) | bug | runtime | see `issues/done/5124-fix-object-keys-on-arguments.md` |
| 5125 | Implement as type assertion expression parsing | feature | frontend/parser | see `issues/done/5125-implement-as-type-assertion-expression.md` |
| 5126 | Implement name resolver var redeclaration tolerance | feature | ir/lowering | see `issues/done/5126-implement-name-resolver-var-redeclaration.md` |
| 5127 | Implement export default multi-file lowering deduplication | feature | ir/lowering | see `issues/done/5127-implement-export-default-multifile-lowering.md` |
| 5128 | Add semantic parity for static re-export module forms | test | ir/compiler | see `issues/done/5128-static-re-export-semantic-parity.md` |
| 5129 | Implement String.prototype.matchAll literal RegExp lowering | feature | runtime/builtins | see `issues/done/5129-implement-string-match-all-literal-regexp.md` |
| 5130 | Implement own method ToPrimitive for mixed BigInt comparisons | feature | runtime/semantics | see `issues/done/5130-implement-object-method-toprimitive-for-bigint-comparisons.md` |
| 5131 | Design ABC451 non-top array growth strategy | design | runtime/memory | see `issues/done/5131-design-abc451-non-top-array-growth-strategy.md` |
| 5133 | Implement single-statement loop bodies for break and continue | feature | frontend/syntax | see `issues/done/5133-implement-single-statement-loop-body-break-continue.md` |
| 5140 | Implement type alias object parsing before exported interface | feature | frontend/syntax | see file |
| 5141 | Implement prefix increment in for update clauses | feature | frontend/syntax | see `issues/done/5141-implement-for-update-prefix-increment.md` |
| 5142 | Support class method calls on new-expression receivers | feature | ir/lowered | see `issues/done/5142-support-class-method-call-on-new-expression-receiver.md` |
| 5148 | Parse generic async generator declarations | feature | frontend/syntax | see `issues/done/5148-parse-generic-async-generator-declarations.md` |
| 5150 | Report empty element access diagnostics | feature | frontend/syntax | see `issues/done/5150-report-empty-element-access-diagnostics.md` |
<!-- generated:done:end -->

## Index generation contract

Run `mise run update-issue-index` after adding, closing, or moving issues. CI and agents should run `mise run update-issue-index -- --check` and `mise run check-issue-health`. `mise run check-issue-index` remains a compatibility alias.

A future generator replaces only the regions between the `<!-- generated:*:start -->` / `<!-- generated:*:end -->` markers.

Do not put hand-written policy text inside generated regions.

## Manual update checklist

When adding, completing, or blocking an issue:

- [ ] issue file is in the correct directory
- [ ] frontmatter is updated
- [ ] dependencies are reflected by re-running `mise run update-issue-index`
- [ ] done issue has completion evidence
- [ ] follow-up work is represented as a separate open issue
- [ ] final-state docs do not contain future TODOs
- [ ] current implementation gaps are in `current-state.md` (repo root)
