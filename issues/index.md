# Issues Index

This file is the human entrypoint for the issue queue.

Issue files are the source of truth for work items. The generated section below may be replaced by a script or pasted manually from a generated report.

## Summary

<!-- generated:summary:start -->
| Area | Total | Open | Resolved |
|---|---:|---:|---:|
| abi | 7 | 1 | 6 |
| backend | 13 | 6 | 7 |
| cli | 15 | 7 | 8 |
| compiler | 1 | 0 | 1 |
| coverage | 10 | 0 | 10 |
| docs | 2 | 0 | 2 |
| frontend | 4350 | 4254 | 96 |
| harness | 1 | 1 | 0 |
| ir | 18 | 4 | 14 |
| issues | 4 | 0 | 4 |
| parser | 1 | 0 | 1 |
| reference | 206 | 201 | 5 |
| runtime | 258 | 120 | 138 |
| scripts | 2 | 0 | 2 |
| security | 1 | 0 | 1 |
| tests | 6 | 0 | 6 |
| wasi | 1 | 0 | 1 |
| total | 4896 | 4594 | 302 |
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

```
5000 (Meta: TypeScript Compiler Parser Syntax Coverage) [P1] ch:1172
│   ├── 5001 (Meta: TypeScript Compiler Semantic Analysis Coverage) [P1] ch:2112
│   ├── 5002 (Meta: TypeScript Compiler Type System Coverage) [P1] ch:243 (also ← 5005)
│   ├── 5005 (Meta: TypeScript Compiler Name Resolution Coverage) [P1] ch:428
│   │   ├── 5006 (Meta: TypeScript Compiler Scope Analysis Coverage) [P2] ch:32
│   │   └── 5007 (Meta: TypeScript Compiler Module Resolution Coverage) [P2] ch:30
│   └── 5003 (Meta: TypeScript Compiler Declaration Emit Coverage) [P2] ch:104 (also ← 5001)
5004 (Meta: Runtime Builtins Coverage (test262)) [P1] ch:45
```

### Multi-parent notes

- **5003** (Meta: TypeScript Compiler Declaration Emit Coverage) also depends on **5001** (Meta: TypeScript Compiler Semantic Analysis Coverage) — shown under primary parent in tree above
- **5002** (Meta: TypeScript Compiler Type System Coverage) also depends on **5005** (Meta: TypeScript Compiler Name Resolution Coverage) — shown under primary parent in tree above

### Meta issue overview

| Order | ID | Title | Area | Priority | Level | Depends on | Child count |
|-----:|---:|------|------|--------:|------:|-----------:|-----------:|
| 1 | 5000 | Meta: TypeScript Compiler Parser Syntax Coverage | frontend/syntax | P1 | 0 | - | 1172 |
| 2 | 5004 | Meta: Runtime Builtins Coverage (test262) | runtime/builtins | P1 | 0 | - | 45 |
| 3 | 5001 | Meta: TypeScript Compiler Semantic Analysis Coverage | frontend/semantics | P1 | 1 | 5000 | 2112 |
| 4 | 5005 | Meta: TypeScript Compiler Name Resolution Coverage | frontend/resolver | P1 | 1 | 5000 | 428 |
| 5 | 5003 | Meta: TypeScript Compiler Declaration Emit Coverage | frontend/syntax | P2 | 2 | 5000, 5001 | 104 |
| 6 | 5007 | Meta: TypeScript Compiler Module Resolution Coverage | frontend/resolver | P2 | 2 | 5005 | 30 |
| 7 | 5002 | Meta: TypeScript Compiler Type System Coverage | frontend/semantics | P1 | 2 | 5000, 5005 | 243 |
| 8 | 5006 | Meta: TypeScript Compiler Scope Analysis Coverage | frontend/resolver | P2 | 2 | 5005 | 32 |

### Topological order (design-ready + key runtime blocked)

| Order | ID | Title | Area | Class | Priority | Level | Depends on |
|-----:|---:|------|------|-------|--------:|------:|-----------:|
| 1 | 5000 | Meta: TypeScript Compiler Parser Syntax Coverage | frontend/syntax | design-ready | P1 | 0 | - |
| 2 | 5004 | Meta: Runtime Builtins Coverage (test262) | runtime/builtins | design-ready | P1 | 0 | - |
| 3 | 5001 | Meta: TypeScript Compiler Semantic Analysis Coverage | frontend/semantics | design-ready | P1 | 1 | 5000 |
| 4 | 5005 | Meta: TypeScript Compiler Name Resolution Coverage | frontend/resolver | design-ready | P1 | 1 | 5000 |
| 5 | 5003 | Meta: TypeScript Compiler Declaration Emit Coverage | frontend/syntax | design-ready | P2 | 2 | 5000, 5001 |
| 6 | 5007 | Meta: TypeScript Compiler Module Resolution Coverage | frontend/resolver | design-ready | P2 | 2 | 5005 |
| 7 | 5002 | Meta: TypeScript Compiler Type System Coverage | frontend/semantics | design-ready | P1 | 2 | 5000, 5005 |
| 8 | 5006 | Meta: TypeScript Compiler Scope Analysis Coverage | frontend/resolver | design-ready | P2 | 2 | 5005 |
| 9 | 5011 | Represent or reject class runtime values in lowered IR | ir/backend | design | P3 | 0 | - |
| 10 | 316 | Fix Object.keys backend-io error | runtime/builtins | blocked | P0 | 1 | 5004 |
| 11 | 4284 | Implement Stringincludes | runtime/builtins | blocked | P1 | 1 | 5004 |
| 12 | 4683 | Implement Unterminatedregexatendofsource | runtime/builtins | blocked | P1 | 1 | 5004 |
| 13 | 4000 | Implement Regexpwithslashincharclass | runtime/builtins | blocked | P1 | 1 | 5004 |
| 14 | 4479 | Implement Tsxfragmentchildrencheck | runtime/builtins | blocked | P1 | 1 | 5004 |
| 15 | 4291 | Implement Stringmatchall | runtime/builtins | blocked | P1 | 1 | 5004 |
| 16 | 313 | Implement array-builtin support | runtime/builtins | blocked | P1 | 1 | 5004 |
| 17 | 3135 | Implement Jsxpreservewithjsinput | runtime/builtins | blocked | P1 | 1 | 5004 |
| 18 | 2230 | Implement Excessivestackdepthflatarray | runtime/builtins | blocked | P1 | 1 | 5004 |
| 19 | 4812 | Implement RegExp literal support | runtime/builtins | blocked | P1 | 1 | 5004 |
| 20 | 2421 | Implement Foroftransformsexpression | runtime/builtins | blocked | P1 | 1 | 5004 |
| 21 | 444 | Implement RegExp literal support | runtime/builtins | blocked | P1 | 1 | 5004 |
| 22 | 4003 | Implement Regularexpressioncharacterclassrangeorder | runtime/builtins | blocked | P1 | 1 | 5004 |
| 23 | 3778 | Implement Parsejsxextends | runtime/builtins | blocked | P1 | 1 | 5004 |
| 24 | 3777 | Implement Parsejsxelementinunaryexpressionnocrash Regexp Literal | runtime/builtins | blocked | P1 | 1 | 5004 |
| 25 | 336 | Implement test262 includes directive processing | cli/reference | blocked | P1 | 0 | 050 |
| 26 | 3130 | Implement Jsxfactorymissingerrorinsideaclass | runtime/builtins | blocked | P1 | 1 | 5004 |
| 27 | 300 | Support ABC451 large integer number boundary | runtime | blocked | P1 | 0 | 308, 309 |
| 28 | 2872 | Implement Initializeddestructuringassignmenttypes | runtime/builtins | blocked | P1 | 1 | 5004 |
| 29 | 1139 | Implement Checkjsxnotseterror | runtime/builtins | blocked | P1 | 1 | 5004 |
| 30 | 052 | Implement JSON | runtime/builtins | blocked | P1 | 1 | 5004 |
| 31 | 342 | Implement Object builtin method coverage (1,721 test262 cases) | runtime/builtins | blocked | P1 | 1 | 5004 |
| 32 | 240 | Implement Date timezone-aware toString policy | runtime/builtins | blocked | P1 | 1 | 239, 5004 |
| 33 | 429 | Implement eval support | reference/triage | blocked | P1 | 2 | 5005 |
| 34 | 3134 | Implement Jsxfactoryqualifiednamewithes | runtime/builtins | blocked | P1 | 1 | 5004 |
| 35 | 4005 | Implement Regularexpressionscanning | runtime/builtins | blocked | P1 | 1 | 5004 |
| 36 | 052d | Implement broader JSON.stringify replacer semantics | runtime/builtins | blocked | P1 | 1 | 5004 |
| 37 | 423 | Implement Date object support | runtime/builtins | blocked | P1 | 1 | 5004 |
| 38 | 3137 | Implement Jsxspreadtag | runtime/builtins | blocked | P1 | 1 | 5004 |
| 39 | 066 | Implement RegExp literal support | runtime/builtins | blocked | P1 | 1 | 5004 |
| 40 | 4294 | Implement Stringtrim | runtime/builtins | blocked | P1 | 1 | 5004 |
| 41 | 3125 | Implement Jsxemitwithattributes | runtime/builtins | blocked | P1 | 1 | 5004 |
| 42 | 363 | Reduce ABC451 allocation and sweep volume after bulk copy narrowing | runtime/memory | blocked | P1 | 0 | 362, 364 |
| 43 | 3097 | Implement Jsfilecompilationtypeargumentsyntaxofcall | runtime/builtins | blocked | P1 | 1 | 5004 |
| 44 | 3131 | Implement Jsxfactorynotidentifierorqualifiedname | runtime/builtins | blocked | P1 | 1 | 5004 |
| 45 | 294 | Support ABC451 D original submission without source rewrite | frontend/runtime | blocked | P1 | 0 | 274 |
| 46 | 3126 | Implement Jsxfactoryandreactnamespace | runtime/builtins | blocked | P1 | 1 | 5004 |
| 47 | 309 | Reduce ABC451 depth-9 live allocation shape | runtime/memory | blocked | P1 | 0 | 308 |
| 48 | 017b | Implement GC strategy | runtime/memory | blocked | P1 | 0 | 217, 218, 219, 220, 221 |
| 49 | 4697 | Implement Unusedimports Regexp Literal | runtime/builtins | blocked | P1 | 1 | 5004 |
| 50 | 357 | Fix ABC451 depth-8 iwasm timeout | runtime/memory | blocked | P1 | 0 | 385, 386 |
| 51 | 3127 | Implement Jsxfactoryidentifier | runtime/builtins | blocked | P1 | 1 | 5004 |
| 52 | 419 | Implement built-in API support | runtime/builtins | blocked | P1 | 1 | 5000, 5004 |
| 53 | 050 | Implement Date | runtime/builtins | blocked | P1 | 1 | 5004 |
| 54 | 3136 | Implement Jsxruntimepragma | runtime/builtins | blocked | P1 | 1 | 5004 |
| 55 | 4776 | Implement Verbatimmodulesyntaxreactreference | runtime/builtins | blocked | P1 | 1 | 5004 |
| 56 | 365 | Reduce ABC451 array-growth allocation and copy pressure | runtime/memory | blocked | P1 | 0 | 364, 366, 367 |
| 57 | 314 | Implement string-builtin support | runtime/builtins | blocked | P1 | 1 | 5004 |
| 58 | 4480 | Implement Tsxresolveexternalmoduleexportstypes | runtime/builtins | blocked | P1 | 1 | 5004 |
| 59 | 3999 | Implement Regexpwithopenbracketincharclass | runtime/builtins | blocked | P1 | 1 | 5004 |
| 60 | 308 | Implement ABC451 depth-9 GC cadence policy | runtime/memory | blocked | P1 | 0 | 309 |
| 61 | 4004 | Implement Regularexpressionextendedunicodeescapes | runtime/builtins | blocked | P1 | 1 | 5004 |
| 62 | 3132 | Implement Jsxfactoryqualifiedname | runtime/builtins | blocked | P1 | 1 | 5004 |
| 63 | 335 | Implement full Math.pow number semantics | runtime/builtins | blocked | P2 | 1 | 5004 |
| 64 | 021 | Implement full wasm backend | backend | blocked | P2 | 0 | 008, 020 |
| 65 | 407 | Implement key-preserving Map entry storage for spread iteration | runtime/semantics | blocked | P2 | 0 | 353 |
| 66 | 374 | Design broader object ToPrimitive for mixed BigInt comparisons | runtime/semantics | blocked | P2 | 0 | 259, 261 |
| 67 | 382 | Multi-limb BigInt addition and subtraction | runtime/semantics | blocked | P2 | 0 | 259, 260, 393, 394 |
| 68 | 369 | Implement full multi-limb BigInt arithmetic | runtime/semantics | blocked | P2 | 0 | 259, 260, 393, 394, 383, 391, 392 |
| 69 | 370 | Implement BigInt arithmetic RangeError and TypeError parity | runtime/semantics | blocked | P2 | 0 | 260, 380, 381 |
| 70 | 353 | Implement iterator protocol integration for spread operator | runtime/semantics | blocked | P2 | 0 | 274 |
| 71 | 344 | Implement legacy global builtin bindings (8 test262 cases) | runtime/builtins | blocked | P3 | 1 | 5004 |

<!-- generated:dep-graph:end -->

## Ready queue

<!-- generated:ready:start -->
| ID | Title | Type | Area | Class | Priority | Depends on | Summary |
|---:|---|---|---|---|---|---|---|
| 408 | Implement tsgo declaration emit: AsConstSatisfies/const generic method cases | feature | frontend/syntax | implementation-ready | P2 | 399 | Implement tsgo declaration emit: AsConstSatisfies/const generic method cases |
| 5000 | Meta: TypeScript Compiler Parser Syntax Coverage | meta | frontend/syntax | design | P1 |  | Meta: TypeScript Compiler Parser Syntax Coverage |
| 5026 | [backend-wasm] Implement real class declaration emission | feature | backend | implementation-ready | P0 |  | [backend-wasm] Implement real class declaration emission |
| 5027 | [backend-wasm] Replace throw-as-return with catchable exception runtime | feature | backend | implementation-ready | P0 |  | [backend-wasm] Replace throw-as-return with catchable exception runtime |
| 5028 | [backend-wasm] Implement array growth and reallocation for push/write paths | feature | backend | implementation-ready | P0 |  | [backend-wasm] Implement array growth and reallocation for push/write paths |
| 5029 | [backend-wasm] Expand direct wasm binary emission beyond console.log string literal MVP | feature | backend | implementation-ready | P1 |  | [backend-wasm] Expand direct wasm binary emission beyond console.log string literal MVP |
| 5030 | [backend-wasm] Split large runtime/WAT emitters into testable components | refactor | backend | implementation-ready | P1 |  | [backend-wasm] Split large runtime/WAT emitters into testable components |
| 5033 | [cli] Normalize node-diff fixture reporting into structured records | feature | cli | implementation-ready | P1 |  | [cli] Normalize node-diff fixture reporting into structured records |
| 5034 | [cli] Add command contract tests for build/check/dump/server | test | cli | implementation-ready | P1 |  | [cli] Add command contract tests for build/check/dump/server |
| 5035 | [cli] Add --explain-unsupported diagnostics mode | feature | cli | implementation-ready | P2 |  | [cli] Add --explain-unsupported diagnostics mode |
| 5038 | [compiler] Harden module graph resolution and diagnostics | feature | cli | implementation-ready | P1 |  | [compiler] Harden module graph resolution and diagnostics |
| 5039 | [compiler] Stabilize test262 preprocessor feature handling | feature | cli | implementation-ready | P1 |  | [compiler] Stabilize test262 preprocessor feature handling |
| 5040 | [compiler] Add resource limits and cancellation to server batch compilation | feature | cli | implementation-ready | P2 |  | [compiler] Add resource limits and cancellation to server batch compilation |
| 5041 | [frontend] Complete Expr AST fixture coverage | test | frontend | implementation-ready | P0 |  | [frontend] Complete Expr AST fixture coverage |
| 5042 | [frontend] Complete Stmt AST fixture coverage | test | frontend | implementation-ready | P0 |  | [frontend] Complete Stmt AST fixture coverage |
| 5043 | [frontend] Split large lexer/parser files by grammar responsibility | refactor | frontend | implementation-ready | P1 |  | [frontend] Split large lexer/parser files by grammar responsibility |
| 5044 | [frontend] Define and test TypeScript ambient declaration erasure boundaries | feature | frontend | implementation-ready | P1 |  | [frontend] Define and test TypeScript ambient declaration erasure boundaries |
| 5045 | [frontend] Improve syntax error recovery and source spans | feature | frontend | implementation-ready | P2 |  | [frontend] Improve syntax error recovery and source spans |
| 5046 | [ir] Design full class runtime IR representation | feature | ir | implementation-ready | P0 |  | [ir] Design full class runtime IR representation |
| 5048 | [ir] Broaden BigInt lowering beyond signed-i64/first-limb slice | feature | ir | implementation-ready | P0 |  | [ir] Broaden BigInt lowering beyond signed-i64/first-limb slice |
| 5049 | [ir] Complete destructuring, rest, and default binding lowering | feature | ir | implementation-ready | P1 |  | [ir] Complete destructuring, rest, and default binding lowering |
| 5050 | [ir] Implement iterator protocol lowering for spread and for-of | feature | ir | implementation-ready | P1 |  | [ir] Implement iterator protocol lowering for spread and for-of |
| 5052 | [runtime-abi] Validate runtime memory map for overlap and headroom | feature | abi | implementation-ready | P1 |  | [runtime-abi] Validate runtime memory map for overlap and headroom |
<!-- generated:ready:end -->

## Blocked queue

<!-- generated:blocked:start -->
| ID | Title | Type | Area | Blocker | Summary |
|---:|---|---|---|---|---|
| 017b | Implement GC strategy | feature | runtime/memory | class: blocked | Implement GC strategy |
| 021 | Implement full wasm backend | feature | backend | class: blocked | Implement full wasm backend |
| 050 | Implement Date | feature | runtime/builtins | class: blocked | Implement Date |
| 052 | Implement JSON | feature | runtime/builtins | class: blocked | Implement JSON |
| 064 | Implement name resolution (triaged - superseded by test262 metadata issues) | spike | frontend/resolver | class: blocked | Implement name resolution (triaged - superseded by test262 metadata issues) |
| 066 | Implement RegExp literal support | spike | runtime/builtins | class: blocked | Implement RegExp literal support |
| 067 | Investigate and classify unknown-unsupported cases | spike | reference/triage | class: triage-needed | Investigate and classify unknown-unsupported cases |
| 068 | Implement unsupported expression types | spike | frontend/semantics | class: blocked | Implement unsupported expression types |
| 069 | Implement Apilibcheck | spike | runtime/builtins | class: blocked | Implement Apilibcheck |
| 070 | Implement Apisample | spike | runtime/builtins | class: blocked | Implement Apisample |
| 071 | Implement Arrowfunctionexpression | spike | frontend/syntax | class: blocked | Implement Arrowfunctionexpression |
| 072 | Implement Classdeclaration | spike | frontend/syntax | class: blocked | Implement Classdeclaration |
| 073 | Implement Classdeclarationwithinvalidconstonpropertydeclaration | spike | frontend/syntax | class: blocked | Implement Classdeclarationwithinvalidconstonpropertydeclaration |
| 074 | Implement Declarationerrorsnoemitonerror | spike | frontend/syntax | class: blocked | Implement Declarationerrorsnoemitonerror |
| 075 | Implement Exportassignment | spike | frontend/syntax | class: blocked | Implement Exportassignment |
| 076 | Implement Functiondeclaration | spike | frontend/syntax | class: blocked | Implement Functiondeclaration |
| 078 | Implement Memberaccessordeclaration | spike | frontend/syntax | class: blocked | Implement Memberaccessordeclaration |
| 079 | Implement Parameterlist | spike | frontend/syntax | class: blocked | Implement Parameterlist |
| 080 | Implement Systemmoduleforstatementnoinitializer | spike | frontend/syntax | class: blocked | Implement Systemmoduleforstatementnoinitializer |
| 081 | Implement Transportstream | spike | reference/triage | class: triage-needed | Implement Transportstream |
| 082 | Implement Abstractclassinlocalscope | spike | frontend/syntax | class: blocked | Implement Abstractclassinlocalscope |
| 083 | Implement Abstractclassinlocalscopeisabstract | spike | frontend/syntax | class: blocked | Implement Abstractclassinlocalscopeisabstract |
| 084 | Implement Abstractclassunioninstantiation | spike | frontend/syntax | class: blocked | Implement Abstractclassunioninstantiation |
| 086 | Implement Abstractpropertybasics | spike | frontend/syntax | class: blocked | Implement Abstractpropertybasics |
| 087 | Implement Abstractpropertyinconstructor | spike | frontend/syntax | class: blocked | Implement Abstractpropertyinconstructor |
| 088 | Implement Abstractpropertynegative | spike | frontend/syntax | class: blocked | Implement Abstractpropertynegative |
| 089 | Implement Acceptsymbolasweaktype | spike | frontend/resolver | class: blocked | Implement Acceptsymbolasweaktype |
| 090 | Implement Acceptablealias | spike | frontend/syntax | class: blocked | Implement Acceptablealias |
| 091 | Implement Accessinstancememberfromstaticmethod | spike | frontend/syntax | class: blocked | Implement Accessinstancememberfromstaticmethod |
| 092 | Implement Accessoverriddenbaseclassmember | spike | frontend/semantics | class: blocked | Implement Accessoverriddenbaseclassmember |
| 093 | Implement Accessstaticmemberfrominstancemethod | spike | frontend/syntax | class: blocked | Implement Accessstaticmemberfrominstancemethod |
| 094 | Implement Accessoraccidentalcalldiagnostic | spike | frontend/resolver | class: blocked | Implement Accessoraccidentalcalldiagnostic |
| 096 | Implement Accessordeclarationemitjs | spike | frontend/syntax | class: blocked | Implement Accessordeclarationemitjs |
| 097 | Implement Accessordeclarationemitvisibilityerrors | spike | frontend/syntax | class: blocked | Implement Accessordeclarationemitvisibilityerrors |
| 098 | Implement Accessordeclarationorder | spike | frontend/syntax | class: blocked | Implement Accessordeclarationorder |
| 099 | Implement Accessorinambientcontextes | spike | frontend/syntax | class: blocked | Implement Accessorinambientcontextes |
| 100 | Implement Accessorinferredreturntypeerrorinreturnstatement | spike | frontend/syntax | class: blocked | Implement Accessorinferredreturntypeerrorinreturnstatement |
| 101 | Implement Accessorparameteraccessibilitymodifier | spike | frontend/syntax | class: blocked | Implement Accessorparameteraccessibilitymodifier |
| 102 | Implement Accessorwithinitializer | spike | frontend/syntax | class: blocked | Implement Accessorwithinitializer |
| 103 | Implement Accessorwithlineterminator | spike | frontend/syntax | class: blocked | Implement Accessorwithlineterminator |
| 104 | Implement Accessorwithrestparam | spike | frontend/syntax | class: blocked | Implement Accessorwithrestparam |
| 105 | Implement Accessorwithoutbody | spike | frontend/syntax | class: blocked | Implement Accessorwithoutbody |
| 106 | Implement Accessors | spike | frontend/syntax | class: triage-needed | Implement Accessors |
| 107 | Implement Accessorsemit | spike | frontend/syntax | class: blocked | Implement Accessorsemit |
| 108 | Implement Accessorsinambientcontext | spike | frontend/syntax | class: blocked | Implement Accessorsinambientcontext |
| 109 | Implement Addmorecallsignaturestobasesignature | spike | frontend/resolver | class: blocked | Implement Addmorecallsignaturestobasesignature |
| 111 | Implement Aliasassignments | spike | frontend/syntax | class: blocked | Implement Aliasassignments |
| 112 | Implement Aliasbug | spike | frontend/syntax | class: blocked | Implement Aliasbug |
| 113 | Implement Aliasdoesnotduplicatesignatures | spike | frontend/syntax | class: blocked | Implement Aliasdoesnotduplicatesignatures |
| 114 | Implement Aliaserrors | spike | frontend/syntax | class: blocked | Implement Aliaserrors |
| 115 | Implement Aliasinaccessiblemodule | spike | frontend/syntax | class: blocked | Implement Aliasinaccessiblemodule |
| 116 | Implement Aliasinstantiationexpressiongenericintersectionnocrash | spike | frontend/syntax | class: blocked | Implement Aliasinstantiationexpressiongenericintersectionnocrash |
| 117 | Implement Aliasofgenericfunctionwithrestbehavedsameasunaliased | spike | frontend/syntax | class: blocked | Implement Aliasofgenericfunctionwithrestbehavedsameasunaliased |
| 118 | Implement Aliasonmergedmoduleinterface | spike | frontend/syntax | class: blocked | Implement Aliasonmergedmoduleinterface |
| 119 | Implement Aliasusageinaccessorsofclass | spike | frontend/syntax | class: blocked | Implement Aliasusageinaccessorsofclass |
| 120 | Implement Aliasusageinarray | spike | frontend/syntax | class: blocked | Implement Aliasusageinarray |
| 121 | Implement Aliasusageinfunctionexpression | spike | frontend/syntax | class: blocked | Implement Aliasusageinfunctionexpression |
| 122 | Implement Aliasusageingenericfunction | spike | frontend/syntax | class: blocked | Implement Aliasusageingenericfunction |
| 123 | Implement Aliasusageinindexerofclass | spike | frontend/syntax | class: blocked | Implement Aliasusageinindexerofclass |
| 124 | Implement Aliasusageinobjectliteral | spike | frontend/syntax | class: blocked | Implement Aliasusageinobjectliteral |
| 125 | Implement Aliasusageinorexpression | spike | frontend/syntax | class: blocked | Implement Aliasusageinorexpression |
| 126 | Implement Aliasusageintypeargumentofextendsclause | spike | frontend/syntax | class: blocked | Implement Aliasusageintypeargumentofextendsclause |
| 127 | Implement Aliasusageinvarassignment | spike | frontend/syntax | class: blocked | Implement Aliasusageinvarassignment |
| 128 | Implement Aliasusedasnamevalue | spike | frontend/syntax | class: blocked | Implement Aliasusedasnamevalue |
| 129 | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer | spike | frontend/syntax | class: blocked | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer |
| 130 | Implement Aliasesinsystemmodule | spike | frontend/syntax | class: blocked | Implement Aliasesinsystemmodule |
| 131 | Implement Allowimportclausestomergewithtypes | spike | frontend/syntax | class: blocked | Implement Allowimportclausestomergewithtypes |
| 132 | Implement Allowjsclassthistypecrash | spike | runtime/builtins | class: blocked | Implement Allowjsclassthistypecrash |
| 133 | Implement Allowjscrossmonorepopackage | spike | frontend/syntax | class: blocked | Implement Allowjscrossmonorepopackage |
| 134 | Implement Allowjscheckjstypeparameternocrash | spike | frontend/syntax | class: blocked | Implement Allowjscheckjstypeparameternocrash |
| 135 | Implement Allowsyntheticdefaultimports | spike | frontend/syntax | class: blocked | Implement Allowsyntheticdefaultimports |
| 136 | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration | spike | frontend/syntax | class: blocked | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration |
| 137 | Implement Alwaysstrictalreadyusestrict | spike | frontend/syntax | class: blocked | Implement Alwaysstrictalreadyusestrict |
| 138 | Implement Alwaysstrictmodule | spike | frontend/syntax | class: blocked | Implement Alwaysstrictmodule |
| 139 | Implement Alwaysstrictnoimplicitusestrict | spike | frontend/syntax | class: blocked | Implement Alwaysstrictnoimplicitusestrict |
| 140 | Implement Ambientclassdeclarationwithextends | spike | frontend/syntax | class: blocked | Implement Ambientclassdeclarationwithextends |
| 141 | Implement Ambientclassdeclaredbeforebase | spike | frontend/syntax | class: blocked | Implement Ambientclassdeclaredbeforebase |
| 142 | Implement Ambientclassmergesoverloadswithinterface | spike | frontend/syntax | class: blocked | Implement Ambientclassmergesoverloadswithinterface |
| 143 | Implement Ambientclassoverloadforfunction | spike | frontend/syntax | class: blocked | Implement Ambientclassoverloadforfunction |
| 144 | Implement Ambientconstliterals | spike | frontend/syntax | class: blocked | Implement Ambientconstliterals |
| 145 | Implement Ambientenum | spike | frontend/syntax | class: blocked | Implement Ambientenum |
| 146 | Implement Ambientenumelementinitializer | spike | frontend/syntax | class: blocked | Implement Ambientenumelementinitializer |
| 147 | Implement Ambienterrors | spike | frontend/syntax | class: blocked | Implement Ambienterrors |
| 148 | Implement Ambientexportdefaulterrors | spike | frontend/syntax | class: blocked | Implement Ambientexportdefaulterrors |
| 149 | Implement Ambientexternalmoduleinanotherexternalmodule | spike | frontend/syntax | class: blocked | Implement Ambientexternalmoduleinanotherexternalmodule |
| 150 | Implement Ambientexternalmodulereopen | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulereopen |
| 151 | Implement Ambientexternalmodulewithinternalimportdeclaration | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulewithinternalimportdeclaration |
| 152 | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration |
| 153 | Implement Ambientexternalmodulewithrelativemodulename | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulewithrelativemodulename |
| 154 | Implement Ambientexternalmodulewithoutinternalimportdeclaration | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulewithoutinternalimportdeclaration |
| 155 | Implement Ambientfundule | spike | frontend/syntax | class: blocked | Implement Ambientfundule |
| 156 | Implement Ambientgetters | spike | frontend/syntax | class: blocked | Implement Ambientgetters |
| 157 | Implement Ambientmoduleexports | spike | frontend/syntax | class: blocked | Implement Ambientmoduleexports |
| 158 | Implement Ambientmodulewithclassdeclarationwithextends | spike | frontend/syntax | class: blocked | Implement Ambientmodulewithclassdeclarationwithextends |
| 159 | Implement Ambientmodulewithtemplateliterals | spike | frontend/syntax | class: blocked | Implement Ambientmodulewithtemplateliterals |
| 160 | Implement Ambientmodules | spike | frontend/syntax | class: blocked | Implement Ambientmodules |
| 161 | Implement Ambientnamerestrictions | spike | frontend/syntax | class: blocked | Implement Ambientnamerestrictions |
| 162 | Implement Ambientpropertydeclarationinjs | spike | frontend/syntax | class: blocked | Implement Ambientpropertydeclarationinjs |
| 163 | Implement Ambientrequirefunction | spike | frontend/syntax | class: blocked | Implement Ambientrequirefunction |
| 164 | Implement Ambientstatement | spike | frontend/syntax | class: blocked | Implement Ambientstatement |
| 165 | Implement Ambientwithstatements | spike | frontend/syntax | class: blocked | Implement Ambientwithstatements |
| 166 | Implement Ambiguouscallswherereturntypesagree | spike | frontend/syntax | class: blocked | Implement Ambiguouscallswherereturntypesagree |
| 167 | Implement Ambiguousgenericassertion | spike | frontend/syntax | class: blocked | Implement Ambiguousgenericassertion |
| 168 | Implement Ambiguousoverload | spike | frontend/semantics | class: blocked | Implement Ambiguousoverload |
| 169 | Implement Ambiguousoverloadresolution | spike | frontend/syntax | class: blocked | Implement Ambiguousoverloadresolution |
| 170 | Implement Amddeclarationemitnoextradeclare | spike | frontend/syntax | class: blocked | Implement Amddeclarationemitnoextradeclare |
| 171 | Implement Amddependencycomment | spike | frontend/syntax | class: blocked | Implement Amddependencycomment |
| 172 | Implement Amddependencycommentname | spike | frontend/syntax | class: blocked | Implement Amddependencycommentname |
| 173 | Implement Amdlikeinputdeclarationemit | spike | frontend/syntax | class: blocked | Implement Amdlikeinputdeclarationemit |
| 174 | Implement Amdmodulebundlenoduplicatedeclarationemitcomments | spike | frontend/syntax | class: blocked | Implement Amdmodulebundlenoduplicatedeclarationemitcomments |
| 175 | Implement Amdmoduleconstenumusage | spike | frontend/syntax | class: blocked | Implement Amdmoduleconstenumusage |
| 176 | Implement Amdmodulename | spike | frontend/syntax | class: blocked | Implement Amdmodulename |
| 177 | Implement Anonclassdeclarationemitisanon | spike | frontend/syntax | class: blocked | Implement Anonclassdeclarationemitisanon |
| 178 | Implement Anonterface | spike | frontend/syntax | class: blocked | Implement Anonterface |
| 179 | Implement Anonymousclassdeclarationdoesntprintwithreadonly | spike | frontend/syntax | class: blocked | Implement Anonymousclassdeclarationdoesntprintwithreadonly |
| 180 | Implement Anonymousclassexpression | spike | frontend/syntax | class: blocked | Implement Anonymousclassexpression |
| 181 | Implement Anonymousmodules | spike | frontend/syntax | class: blocked | Implement Anonymousmodules |
| 182 | Implement Anyandunknownhavefalsycomponents | spike | frontend/syntax | class: blocked | Implement Anyandunknownhavefalsycomponents |
| 183 | Implement Anyasreturntypefornewoncall | spike | frontend/syntax | class: blocked | Implement Anyasreturntypefornewoncall |
| 184 | Implement Anydeclare | spike | frontend/syntax | class: blocked | Implement Anydeclare |
| 185 | Implement Anyidenticaltoitself | spike | frontend/syntax | class: blocked | Implement Anyidenticaltoitself |
| 187 | Implement Anyinferenceanonymousfunctions | spike | frontend/syntax | class: blocked | Implement Anyinferenceanonymousfunctions |
| 192 | Implement Argsinscope | spike | frontend/syntax | class: blocked | Implement Argsinscope |
| 193 | Implement Arguments | spike | frontend/resolver | class: blocked | Implement Arguments |
| 194 | Implement Argumentsaspropertyname | spike | frontend/semantics | class: blocked | Implement Argumentsaspropertyname |
| 195 | Implement Argumentsbindstofunctionscopeargumentlist | spike | frontend/resolver | class: blocked | Implement Argumentsbindstofunctionscopeargumentlist |
| 196 | Implement Argumentsobjectcreatesrestforjs | spike | frontend/resolver | class: blocked | Implement Argumentsobjectcreatesrestforjs |
| 197 | Implement Argumentsobjectiterator | spike | frontend/semantics | class: blocked | Implement Argumentsobjectiterator |
| 198 | Implement Argumentspropertynameinjsmode | spike | frontend/semantics | class: blocked | Implement Argumentspropertynameinjsmode |
| 199 | Implement Compiler | spike | frontend/syntax | class: blocked | Implement Compiler |
| 201 | Investigate and classify unknown-unsupported cases | spike | reference/triage | class: triage-needed | Investigate and classify unknown-unsupported cases |
| 294 | Support ABC451 D original submission without source rewrite | feature | frontend/runtime | class: blocked | Support ABC451 D original submission without source rewrite |
| 300 | Support ABC451 large integer number boundary | feature | runtime | class: blocked | Support ABC451 large integer number boundary |
| 308 | Implement ABC451 depth-9 GC cadence policy | feature | runtime/memory | class: blocked | Implement ABC451 depth-9 GC cadence policy |
| 309 | Reduce ABC451 depth-9 live allocation shape | feature | runtime/memory | class: blocked | Reduce ABC451 depth-9 live allocation shape |
| 312 | Triage test262 blocked P0 window | spike | reference | class: triage-needed | Triage test262 blocked P0 window |
| 313 | Implement array-builtin support | feature | runtime/builtins | class: blocked | Implement array-builtin support |
| 314 | Implement string-builtin support | feature | runtime/builtins | class: blocked | Implement string-builtin support |
| 316 | Fix Object.keys backend-io error | feature | harness | class: blocked | Fix Object.keys backend-io error |
| 335 | Implement full Math.pow number semantics | feature | runtime/builtins | class: blocked | Implement full Math.pow number semantics |
| 336 | Implement test262 includes directive processing | feature | cli/reference | class: blocked | Implement test262 includes directive processing |
| 342 | Implement Object builtin method coverage (1,721 test262 cases) | feature | runtime/builtins | class: blocked | Implement Object builtin method coverage (1,721 test262 cases) |
| 343 | Implement DuplicateLocal diagnostic detection (66 test262 cases) | feature | frontend/resolver | class: blocked | Implement DuplicateLocal diagnostic detection (66 test262 cases) |
| 345 | Implement TypeScript type alias coverage for tsc suite (23 cases) | feature | frontend/syntax | class: blocked | Implement TypeScript type alias coverage for tsc suite (23 cases) |
| 346 | Implement TypeScript declaration emit coverage for tsgo suite (16 cases) | feature | frontend/syntax | class: triage-needed | Implement TypeScript declaration emit coverage for tsgo suite (16 cases) |
| 353 | Implement iterator protocol integration for spread operator | feature | runtime/semantics | class: blocked | Implement iterator protocol integration for spread operator |
| 357 | Fix ABC451 depth-8 iwasm timeout | bug | runtime/memory | class: blocked | Fix ABC451 depth-8 iwasm timeout |
| 363 | Reduce ABC451 allocation and sweep volume after bulk copy narrowing | bug | runtime/memory | class: blocked | Reduce ABC451 allocation and sweep volume after bulk copy narrowing |
| 365 | Reduce ABC451 array-growth allocation and copy pressure | bug | runtime/memory | class: blocked | Reduce ABC451 array-growth allocation and copy pressure |
| 369 | Implement full multi-limb BigInt arithmetic | feature | runtime/semantics | class: blocked | Implement full multi-limb BigInt arithmetic |
| 370 | Implement BigInt arithmetic RangeError and TypeError parity | feature | runtime/semantics | class: blocked | Implement BigInt arithmetic RangeError and TypeError parity |
| 374 | Design broader object ToPrimitive for mixed BigInt comparisons | design | runtime/semantics | class: blocked | Design broader object ToPrimitive for mixed BigInt comparisons |
| 382 | Multi-limb BigInt addition and subtraction | feature | runtime/semantics | class: blocked | Multi-limb BigInt addition and subtraction |
| 386 | Reduce ABC451 depth-8 array copy pressure | feature | runtime/memory | class: triage-needed | Reduce ABC451 depth-8 array copy pressure |
| 407 | Implement key-preserving Map entry storage for spread iteration | feature | runtime/semantics | class: blocked | Implement key-preserving Map entry storage for spread iteration |
| 411 | Implement annexb-ishtmldda support | spike | frontend/syntax | class: triage-needed | Implement annexb-ishtmldda support |
| 412 | Implement arguments-object support | spike | runtime/builtins | class: blocked | Implement arguments-object support |
| 413 | Implement arity support | spike | reference/triage | class: triage-needed | Implement arity support |
| 414 | Implement array-builtin support | spike | frontend/syntax | class: triage-needed | Implement array-builtin support |
| 415 | Implement arrow functions | spike | frontend/syntax | class: blocked | Implement arrow functions |
| 416 | Implement async/await support | spike | frontend/syntax | class: triage-needed | Implement async/await support |
| 417 | Implement async-iteration support | spike | frontend/syntax | class: triage-needed | Implement async-iteration support |
| 418 | Implement break/continue | spike | frontend/syntax | class: blocked | Implement break/continue |
| 419 | Implement built-in API support | spike | runtime/builtins | class: blocked | Implement built-in API support |
| 420 | Implement call expression support | spike | frontend/syntax | class: blocked | Implement call expression support |
| 421 | Implement class syntax | spike | frontend/syntax | class: triage-needed | Implement class syntax |
| 422 | Implement class-accessor support | spike | frontend/syntax | class: triage-needed | Implement class-accessor support |
| 423 | Implement Date object support | spike | runtime/builtins | class: blocked | Implement Date object support |
| 424 | Implement declaration-emit support | spike | frontend/syntax | class: blocked | Implement declaration-emit support |
| 425 | Implement destructuring | spike | frontend/syntax | class: blocked | Implement destructuring |
| 426 | Implement duplicate-function support | spike | reference/triage | class: triage-needed | Implement duplicate-function support |
| 427 | Implement duplicate-local support | spike | reference/triage | class: triage-needed | Implement duplicate-local support |
| 428 | Implement enum support | spike | frontend/syntax | class: triage-needed | Implement enum support |
| 429 | Implement eval support | spike | reference/triage | class: blocked | Implement eval support |
| 430 | Implement function support | spike | frontend/syntax | class: triage-needed | Implement function support |
| 431 | Implement function resolution | spike | frontend/resolver | class: triage-needed | Implement function resolution |
| 432 | Implement import/export module syntax | spike | frontend/syntax | class: triage-needed | Implement import/export module syntax |
| 433 | Implement legacy-global-builtin support | spike | frontend/syntax | class: triage-needed | Implement legacy-global-builtin support |
| 434 | Implement loop constructs | spike | frontend/syntax | class: blocked | Implement loop constructs |
| 435 | Implement method call support | spike | frontend/syntax | class: blocked | Implement method call support |
| 436 | Implement module-resolution support | spike | frontend/syntax | class: triage-needed | Implement module-resolution support |
| 437 | Implement name resolution | spike | frontend/resolver | class: blocked | Implement name resolution |
| 438 | Implement negative-parse-syntaxerror support | spike | reference/triage | class: triage-needed | Implement negative-parse-syntaxerror support |
| 439 | Implement new expression | spike | frontend/syntax | class: blocked | Implement new expression |
| 440 | Implement object-builtin support | spike | frontend/syntax | class: blocked | Implement object-builtin support |
| 441 | Implement object literal enhancements | spike | frontend/syntax | class: blocked | Implement object literal enhancements |
| 442 | Implement parser syntax extensions | spike | frontend/syntax | class: blocked | Implement parser syntax extensions |
| 443 | Implement property access support | spike | frontend/syntax | class: blocked | Implement property access support |
| 445 | Implement runtime-subset support | spike | reference/triage | class: triage-needed | Implement runtime-subset support |
| 446 | Implement scope-analysis support | spike | frontend/syntax | class: blocked | Implement scope-analysis support |
| 447 | Implement spread operator | spike | frontend/syntax | class: blocked | Implement spread operator |
| 448 | Implement string-builtin support | spike | frontend/syntax | class: triage-needed | Implement string-builtin support |
| 449 | Implement super keyword | spike | frontend/syntax | class: triage-needed | Implement super keyword |
| 450 | Implement template literals | spike | frontend/syntax | class: triage-needed | Implement template literals |
| 451 | Implement try-catch-finally | spike | frontend/syntax | class: blocked | Implement try-catch-finally |
| 452 | Implement type-alias support | spike | frontend/syntax | class: blocked | Implement type-alias support |
| 453 | Implement type-system support | spike | frontend/syntax | class: blocked | Implement type-system support |
| 454 | Investigate and classify unknown-unsupported cases | spike | frontend/syntax | class: triage-needed | Investigate and classify unknown-unsupported cases |
| 455 | Implement Apilibcheck | spike | frontend/syntax | class: blocked | Implement Apilibcheck |
| 456 | Implement Apisample Arrow Function | spike | frontend/syntax | class: blocked | Implement Apisample Arrow Function |
| 457 | Implement Apisample Import Export | spike | frontend/syntax | class: blocked | Implement Apisample Import Export |
| 458 | Implement Apisample Jsdoc | spike | frontend/syntax | class: blocked | Implement Apisample Jsdoc |
| 459 | Implement Arrowfunctionexpression | spike | frontend/syntax | class: blocked | Implement Arrowfunctionexpression |
| 460 | Implement Classdeclaration | spike | frontend/syntax | class: blocked | Implement Classdeclaration |
| 461 | Implement Classdeclarationwithinvalidconstonpropertydeclaration | spike | frontend/syntax | class: blocked | Implement Classdeclarationwithinvalidconstonpropertydeclaration |
| 462 | Implement Exportassignment | spike | frontend/syntax | class: blocked | Implement Exportassignment |
| 463 | Implement Functiondeclaration Import Export | spike | frontend/syntax | class: blocked | Implement Functiondeclaration Import Export |
| 464 | Implement Functiondeclaration Parser Syntax | spike | frontend/syntax | class: blocked | Implement Functiondeclaration Parser Syntax |
| 465 | Implement Memberaccessordeclaration | spike | frontend/syntax | class: blocked | Implement Memberaccessordeclaration |
| 466 | Implement Parameterlist | spike | frontend/syntax | class: blocked | Implement Parameterlist |
| 467 | Implement Transportstream | spike | frontend/syntax | class: blocked | Implement Transportstream |
| 468 | Implement Abstractclassinlocalscope | spike | frontend/syntax | class: blocked | Implement Abstractclassinlocalscope |
| 469 | Implement Abstractclassinlocalscopeisabstract | spike | frontend/syntax | class: blocked | Implement Abstractclassinlocalscopeisabstract |
| 470 | Implement Abstractclassunioninstantiation | spike | frontend/resolver | class: blocked | Implement Abstractclassunioninstantiation |
| 471 | Implement Abstractpropertybasics | spike | frontend/syntax | class: blocked | Implement Abstractpropertybasics |
| 472 | Implement Abstractpropertyinconstructor | spike | frontend/syntax | class: blocked | Implement Abstractpropertyinconstructor |
| 473 | Implement Abstractpropertynegative | spike | frontend/syntax | class: blocked | Implement Abstractpropertynegative |
| 474 | Implement Acceptsymbolasweaktype | spike | frontend/resolver | class: blocked | Implement Acceptsymbolasweaktype |
| 475 | Implement Acceptablealias | spike | frontend/syntax | class: blocked | Implement Acceptablealias |
| 476 | Implement Accessinstancememberfromstaticmethod | spike | frontend/resolver | class: blocked | Implement Accessinstancememberfromstaticmethod |
| 477 | Implement Accessoverriddenbaseclassmember | spike | frontend/semantics | class: blocked | Implement Accessoverriddenbaseclassmember |
| 478 | Implement Accessstaticmemberfrominstancemethod | spike | frontend/resolver | class: blocked | Implement Accessstaticmemberfrominstancemethod |
| 479 | Implement Accessoraccidentalcalldiagnostic | spike | frontend/syntax | class: blocked | Implement Accessoraccidentalcalldiagnostic |
| 480 | Implement Accessordeclarationemitjs | spike | frontend/syntax | class: blocked | Implement Accessordeclarationemitjs |
| 481 | Implement Accessordeclarationemitvisibilityerrors | spike | frontend/syntax | class: blocked | Implement Accessordeclarationemitvisibilityerrors |
| 482 | Implement Accessordeclarationorder | spike | frontend/syntax | class: blocked | Implement Accessordeclarationorder |
| 483 | Implement Accessorinambientcontextes | spike | frontend/syntax | class: blocked | Implement Accessorinambientcontextes |
| 484 | Implement Accessorinferredreturntypeerrorinreturnstatement | spike | frontend/syntax | class: blocked | Implement Accessorinferredreturntypeerrorinreturnstatement |
| 485 | Implement Accessorparameteraccessibilitymodifier | spike | frontend/syntax | class: blocked | Implement Accessorparameteraccessibilitymodifier |
| 486 | Implement Accessorwithlineterminator | spike | reference/triage | class: triage-needed | Implement Accessorwithlineterminator |
| 487 | Implement Accessorwithoutbody | spike | frontend/syntax | class: blocked | Implement Accessorwithoutbody |
| 488 | Implement Accessors | spike | frontend/syntax | class: triage-needed | Implement Accessors |
| 489 | Implement Accessorsinambientcontext | spike | frontend/syntax | class: blocked | Implement Accessorsinambientcontext |
| 490 | Implement Addmorecallsignaturestobasesignature | spike | frontend/resolver | class: blocked | Implement Addmorecallsignaturestobasesignature |
| 491 | Implement Aliasassignments | spike | frontend/syntax | class: blocked | Implement Aliasassignments |
| 492 | Implement Aliasbug | spike | frontend/syntax | class: blocked | Implement Aliasbug |
| 493 | Implement Aliasdoesnotduplicatesignatures | spike | frontend/syntax | class: blocked | Implement Aliasdoesnotduplicatesignatures |
| 494 | Implement Aliaserrors | spike | frontend/syntax | class: blocked | Implement Aliaserrors |
| 495 | Implement Aliasinaccessiblemodule | spike | frontend/syntax | class: blocked | Implement Aliasinaccessiblemodule |
| 496 | Implement Aliasinstantiationexpressiongenericintersectionnocrash | spike | frontend/syntax | class: blocked | Implement Aliasinstantiationexpressiongenericintersectionnocrash |
| 497 | Implement Aliasonmergedmoduleinterface | spike | frontend/syntax | class: blocked | Implement Aliasonmergedmoduleinterface |
| 498 | Implement Aliasusageinaccessorsofclass | spike | frontend/syntax | class: blocked | Implement Aliasusageinaccessorsofclass |
| 499 | Implement Aliasusageinarray | spike | frontend/syntax | class: blocked | Implement Aliasusageinarray |
| 500 | Implement Aliasusageinfunctionexpression | spike | frontend/syntax | class: blocked | Implement Aliasusageinfunctionexpression |
| 501 | Implement Aliasusageingenericfunction | spike | frontend/syntax | class: blocked | Implement Aliasusageingenericfunction |
| 502 | Implement Aliasusageinindexerofclass | spike | frontend/syntax | class: blocked | Implement Aliasusageinindexerofclass |
| 503 | Implement Aliasusageinobjectliteral | spike | frontend/syntax | class: blocked | Implement Aliasusageinobjectliteral |
| 504 | Implement Aliasusageinorexpression | spike | frontend/syntax | class: blocked | Implement Aliasusageinorexpression |
| 505 | Implement Aliasusageintypeargumentofextendsclause | spike | frontend/syntax | class: blocked | Implement Aliasusageintypeargumentofextendsclause |
| 506 | Implement Aliasusageinvarassignment | spike | frontend/syntax | class: blocked | Implement Aliasusageinvarassignment |
| 507 | Implement Aliasusedasnamevalue | spike | frontend/syntax | class: blocked | Implement Aliasusedasnamevalue |
| 508 | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer | spike | frontend/syntax | class: blocked | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer |
| 509 | Implement Aliasesinsystemmodule | spike | frontend/syntax | class: blocked | Implement Aliasesinsystemmodule |
| 510 | Implement Allowimportclausestomergewithtypes | spike | frontend/syntax | class: blocked | Implement Allowimportclausestomergewithtypes |
| 511 | Implement Allowjsclassthistypecrash | spike | reference/triage | class: triage-needed | Implement Allowjsclassthistypecrash |
| 512 | Implement Allowjscrossmonorepopackage | spike | frontend/syntax | class: blocked | Implement Allowjscrossmonorepopackage |
| 513 | Implement Allowjscheckjstypeparameternocrash | spike | frontend/syntax | class: blocked | Implement Allowjscheckjstypeparameternocrash |
| 514 | Implement Allowsyntheticdefaultimports | spike | frontend/syntax | class: blocked | Implement Allowsyntheticdefaultimports |
| 515 | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration | spike | frontend/syntax | class: blocked | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration |
| 516 | Implement Alwaysstrictmodule | spike | frontend/syntax | class: blocked | Implement Alwaysstrictmodule |
| 517 | Implement Alwaysstrictnoimplicitusestrict | spike | frontend/syntax | class: blocked | Implement Alwaysstrictnoimplicitusestrict |
| 518 | Implement Ambientclassdeclarationwithextends | spike | frontend/syntax | class: blocked | Implement Ambientclassdeclarationwithextends |
| 519 | Implement Ambientclassdeclaredbeforebase | spike | frontend/syntax | class: blocked | Implement Ambientclassdeclaredbeforebase |
| 520 | Implement Ambientconstliterals | spike | frontend/syntax | class: blocked | Implement Ambientconstliterals |
| 521 | Implement Ambientenumelementinitializer | spike | frontend/syntax | class: blocked | Implement Ambientenumelementinitializer |
| 522 | Implement Ambienterrors | spike | runtime/builtins | class: blocked | Implement Ambienterrors |
| 523 | Implement Ambientexportdefaulterrors | spike | frontend/syntax | class: blocked | Implement Ambientexportdefaulterrors |
| 524 | Implement Ambientexternalmoduleinanotherexternalmodule | spike | frontend/syntax | class: blocked | Implement Ambientexternalmoduleinanotherexternalmodule |
| 525 | Implement Ambientexternalmodulereopen | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulereopen |
| 526 | Implement Ambientexternalmodulewithinternalimportdeclaration | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulewithinternalimportdeclaration |
| 527 | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration |
| 528 | Implement Ambientexternalmodulewithrelativemodulename | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulewithrelativemodulename |
| 529 | Implement Ambientexternalmodulewithoutinternalimportdeclaration | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulewithoutinternalimportdeclaration |
| 530 | Implement Ambientfundule | spike | frontend/syntax | class: blocked | Implement Ambientfundule |
| 531 | Implement Ambientmoduleexports | spike | frontend/syntax | class: blocked | Implement Ambientmoduleexports |
| 532 | Implement Ambientmodulewithclassdeclarationwithextends | spike | frontend/syntax | class: blocked | Implement Ambientmodulewithclassdeclarationwithextends |
| 533 | Implement Ambientmodulewithtemplateliterals | spike | frontend/syntax | class: blocked | Implement Ambientmodulewithtemplateliterals |
| 534 | Implement Ambientmodules | spike | frontend/syntax | class: blocked | Implement Ambientmodules |
| 535 | Implement Ambientnamerestrictions | spike | frontend/syntax | class: blocked | Implement Ambientnamerestrictions |
| 536 | Implement Ambientrequirefunction | spike | frontend/syntax | class: blocked | Implement Ambientrequirefunction |
| 537 | Implement Ambientstatement | spike | frontend/syntax | class: blocked | Implement Ambientstatement |
| 538 | Implement Ambientwithstatements | spike | frontend/syntax | class: blocked | Implement Ambientwithstatements |
| 539 | Implement Ambiguouscallswherereturntypesagree | spike | frontend/syntax | class: blocked | Implement Ambiguouscallswherereturntypesagree |
| 540 | Implement Ambiguousgenericassertion | spike | frontend/syntax | class: blocked | Implement Ambiguousgenericassertion |
| 541 | Implement Apilibcheck | spike | frontend/syntax | class: blocked | Implement Apilibcheck |
| 542 | Implement Apisample Arrow Function | spike | frontend/syntax | class: blocked | Implement Apisample Arrow Function |
| 543 | Implement Apisample Import Export | spike | frontend/syntax | class: blocked | Implement Apisample Import Export |
| 544 | Implement Apisample Jsdoc | spike | frontend/syntax | class: blocked | Implement Apisample Jsdoc |
| 545 | Implement Arrowfunctionexpression | spike | frontend/syntax | class: blocked | Implement Arrowfunctionexpression |
| 546 | Implement Classdeclaration | spike | frontend/syntax | class: blocked | Implement Classdeclaration |
| 547 | Implement Classdeclarationwithinvalidconstonpropertydeclaration | spike | frontend/syntax | class: blocked | Implement Classdeclarationwithinvalidconstonpropertydeclaration |
| 548 | Implement Exportassignment | spike | frontend/syntax | class: blocked | Implement Exportassignment |
| 549 | Implement Functiondeclaration Import Export | spike | frontend/syntax | class: blocked | Implement Functiondeclaration Import Export |
| 550 | Implement Functiondeclaration Parser Syntax | spike | frontend/syntax | class: blocked | Implement Functiondeclaration Parser Syntax |
| 551 | Implement Memberaccessordeclaration | spike | frontend/syntax | class: blocked | Implement Memberaccessordeclaration |
| 552 | Implement Parameterlist | spike | frontend/syntax | class: blocked | Implement Parameterlist |
| 553 | Implement Transportstream | spike | frontend/syntax | class: blocked | Implement Transportstream |
| 554 | Implement Abstractclassinlocalscope | spike | frontend/syntax | class: blocked | Implement Abstractclassinlocalscope |
| 555 | Implement Abstractclassinlocalscopeisabstract | spike | frontend/syntax | class: blocked | Implement Abstractclassinlocalscopeisabstract |
| 556 | Implement Abstractclassunioninstantiation | spike | frontend/resolver | class: blocked | Implement Abstractclassunioninstantiation |
| 557 | Implement Abstractpropertybasics | spike | frontend/syntax | class: blocked | Implement Abstractpropertybasics |
| 558 | Implement Abstractpropertyinconstructor | spike | frontend/syntax | class: blocked | Implement Abstractpropertyinconstructor |
| 559 | Implement Abstractpropertynegative | spike | frontend/syntax | class: blocked | Implement Abstractpropertynegative |
| 560 | Implement Acceptsymbolasweaktype | spike | frontend/resolver | class: blocked | Implement Acceptsymbolasweaktype |
| 561 | Implement Acceptablealias | spike | frontend/syntax | class: blocked | Implement Acceptablealias |
| 562 | Implement Accessinstancememberfromstaticmethod | spike | frontend/resolver | class: blocked | Implement Accessinstancememberfromstaticmethod |
| 563 | Implement Accessoverriddenbaseclassmember | spike | frontend/semantics | class: blocked | Implement Accessoverriddenbaseclassmember |
| 564 | Implement Accessstaticmemberfrominstancemethod | spike | frontend/resolver | class: blocked | Implement Accessstaticmemberfrominstancemethod |
| 565 | Implement Accessoraccidentalcalldiagnostic | spike | frontend/syntax | class: blocked | Implement Accessoraccidentalcalldiagnostic |
| 566 | Implement Accessordeclarationemitjs | spike | frontend/syntax | class: blocked | Implement Accessordeclarationemitjs |
| 567 | Implement Accessordeclarationemitvisibilityerrors | spike | frontend/syntax | class: blocked | Implement Accessordeclarationemitvisibilityerrors |
| 568 | Implement Accessordeclarationorder | spike | frontend/syntax | class: blocked | Implement Accessordeclarationorder |
| 569 | Implement Accessorinambientcontextes | spike | frontend/syntax | class: blocked | Implement Accessorinambientcontextes |
| 570 | Implement Accessorinferredreturntypeerrorinreturnstatement | spike | frontend/syntax | class: blocked | Implement Accessorinferredreturntypeerrorinreturnstatement |
| 571 | Implement Accessorparameteraccessibilitymodifier | spike | frontend/syntax | class: blocked | Implement Accessorparameteraccessibilitymodifier |
| 572 | Implement Accessorwithlineterminator | spike | reference/triage | class: triage-needed | Implement Accessorwithlineterminator |
| 573 | Implement Accessorwithoutbody | spike | frontend/syntax | class: blocked | Implement Accessorwithoutbody |
| 574 | Implement Accessors | spike | frontend/syntax | class: triage-needed | Implement Accessors |
| 575 | Implement Accessorsinambientcontext | spike | frontend/syntax | class: blocked | Implement Accessorsinambientcontext |
| 576 | Implement Addmorecallsignaturestobasesignature | spike | frontend/syntax | class: blocked | Implement Addmorecallsignaturestobasesignature |
| 577 | Implement Aliasassignments | spike | frontend/syntax | class: blocked | Implement Aliasassignments |
| 578 | Implement Aliasbug | spike | frontend/syntax | class: blocked | Implement Aliasbug |
| 579 | Implement Aliasdoesnotduplicatesignatures | spike | frontend/syntax | class: blocked | Implement Aliasdoesnotduplicatesignatures |
| 580 | Implement Aliaserrors | spike | frontend/syntax | class: blocked | Implement Aliaserrors |
| 581 | Implement Aliasinaccessiblemodule | spike | frontend/syntax | class: blocked | Implement Aliasinaccessiblemodule |
| 582 | Implement Aliasinstantiationexpressiongenericintersectionnocrash | spike | frontend/syntax | class: blocked | Implement Aliasinstantiationexpressiongenericintersectionnocrash |
| 583 | Implement Aliasonmergedmoduleinterface | spike | frontend/syntax | class: blocked | Implement Aliasonmergedmoduleinterface |
| 584 | Implement Aliasusageinaccessorsofclass | spike | frontend/syntax | class: blocked | Implement Aliasusageinaccessorsofclass |
| 585 | Implement Aliasusageinarray | spike | frontend/syntax | class: blocked | Implement Aliasusageinarray |
| 586 | Implement Aliasusageinfunctionexpression | spike | frontend/syntax | class: blocked | Implement Aliasusageinfunctionexpression |
| 587 | Implement Aliasusageingenericfunction | spike | frontend/syntax | class: blocked | Implement Aliasusageingenericfunction |
| 588 | Implement Aliasusageinindexerofclass | spike | frontend/syntax | class: blocked | Implement Aliasusageinindexerofclass |
| 589 | Implement Aliasusageinobjectliteral | spike | frontend/syntax | class: blocked | Implement Aliasusageinobjectliteral |
| 590 | Implement Aliasusageinorexpression | spike | frontend/syntax | class: blocked | Implement Aliasusageinorexpression |
| 591 | Implement Aliasusageintypeargumentofextendsclause | spike | frontend/syntax | class: blocked | Implement Aliasusageintypeargumentofextendsclause |
| 592 | Implement Aliasusageinvarassignment | spike | frontend/syntax | class: blocked | Implement Aliasusageinvarassignment |
| 593 | Implement Aliasusedasnamevalue | spike | frontend/syntax | class: blocked | Implement Aliasusedasnamevalue |
| 594 | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer | spike | frontend/syntax | class: blocked | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer |
| 595 | Implement Aliasesinsystemmodule | spike | frontend/syntax | class: blocked | Implement Aliasesinsystemmodule |
| 596 | Implement Allowimportclausestomergewithtypes | spike | frontend/syntax | class: blocked | Implement Allowimportclausestomergewithtypes |
| 597 | Implement Allowjsclassthistypecrash | spike | reference/triage | class: triage-needed | Implement Allowjsclassthistypecrash |
| 598 | Implement Allowjscrossmonorepopackage | spike | frontend/syntax | class: blocked | Implement Allowjscrossmonorepopackage |
| 599 | Implement Allowjscheckjstypeparameternocrash | spike | frontend/syntax | class: blocked | Implement Allowjscheckjstypeparameternocrash |
| 600 | Implement Allowsyntheticdefaultimports | spike | frontend/syntax | class: blocked | Implement Allowsyntheticdefaultimports |
| 601 | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration | spike | frontend/syntax | class: blocked | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration |
| 602 | Implement Alwaysstrictmodule | spike | frontend/syntax | class: blocked | Implement Alwaysstrictmodule |
| 603 | Implement Alwaysstrictnoimplicitusestrict | spike | frontend/syntax | class: blocked | Implement Alwaysstrictnoimplicitusestrict |
| 604 | Implement Ambientclassdeclarationwithextends | spike | frontend/syntax | class: blocked | Implement Ambientclassdeclarationwithextends |
| 605 | Implement Ambientclassdeclaredbeforebase | spike | frontend/syntax | class: blocked | Implement Ambientclassdeclaredbeforebase |
| 606 | Implement Ambientconstliterals | spike | frontend/syntax | class: blocked | Implement Ambientconstliterals |
| 607 | Implement Ambientenumelementinitializer | spike | frontend/syntax | class: blocked | Implement Ambientenumelementinitializer |
| 608 | Implement Ambienterrors | spike | runtime/builtins | class: blocked | Implement Ambienterrors |
| 609 | Implement Ambientexportdefaulterrors | spike | frontend/syntax | class: blocked | Implement Ambientexportdefaulterrors |
| 610 | Implement Ambientexternalmoduleinanotherexternalmodule | spike | frontend/syntax | class: blocked | Implement Ambientexternalmoduleinanotherexternalmodule |
| 611 | Implement Ambientexternalmodulereopen | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulereopen |
| 612 | Implement Ambientexternalmodulewithinternalimportdeclaration | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulewithinternalimportdeclaration |
| 613 | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration |
| 614 | Implement Ambientexternalmodulewithrelativemodulename | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulewithrelativemodulename |
| 615 | Implement Ambientexternalmodulewithoutinternalimportdeclaration | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulewithoutinternalimportdeclaration |
| 616 | Implement Ambientfundule | spike | frontend/syntax | class: blocked | Implement Ambientfundule |
| 617 | Implement Ambientmoduleexports | spike | frontend/syntax | class: blocked | Implement Ambientmoduleexports |
| 618 | Implement Ambientmodulewithclassdeclarationwithextends | spike | frontend/syntax | class: blocked | Implement Ambientmodulewithclassdeclarationwithextends |
| 619 | Implement Ambientmodulewithtemplateliterals | spike | frontend/syntax | class: blocked | Implement Ambientmodulewithtemplateliterals |
| 620 | Implement Ambientmodules | spike | frontend/syntax | class: blocked | Implement Ambientmodules |
| 621 | Implement Ambientnamerestrictions | spike | frontend/syntax | class: blocked | Implement Ambientnamerestrictions |
| 622 | Implement Ambientrequirefunction | spike | frontend/syntax | class: blocked | Implement Ambientrequirefunction |
| 623 | Implement Ambientstatement | spike | frontend/syntax | class: blocked | Implement Ambientstatement |
| 624 | Implement Ambientwithstatements | spike | frontend/syntax | class: blocked | Implement Ambientwithstatements |
| 625 | Implement Ambiguouscallswherereturntypesagree | spike | frontend/syntax | class: blocked | Implement Ambiguouscallswherereturntypesagree |
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
| 636 | Implement Anonterface | spike | frontend/syntax | class: blocked | Implement Anonterface |
| 637 | Implement Anonymousclassdeclarationdoesntprintwithreadonly | spike | frontend/syntax | class: blocked | Implement Anonymousclassdeclarationdoesntprintwithreadonly |
| 638 | Implement Anonymousclassexpression | spike | frontend/syntax | class: blocked | Implement Anonymousclassexpression |
| 639 | Implement Anonymousmodules | spike | frontend/syntax | class: blocked | Implement Anonymousmodules |
| 640 | Implement Anyandunknownhavefalsycomponents | spike | frontend/resolver | class: blocked | Implement Anyandunknownhavefalsycomponents |
| 641 | Implement Anyasreturntypefornewoncall | spike | frontend/syntax | class: blocked | Implement Anyasreturntypefornewoncall |
| 642 | Implement Anydeclare | spike | frontend/syntax | class: blocked | Implement Anydeclare |
| 643 | Implement Anyidenticaltoitself | spike | frontend/syntax | class: blocked | Implement Anyidenticaltoitself |
| 644 | Implement Anyinferenceanonymousfunctions | spike | frontend/syntax | class: blocked | Implement Anyinferenceanonymousfunctions |
| 645 | Implement Argsinscope | spike | frontend/syntax | class: blocked | Implement Argsinscope |
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
| 661 | Implement Arithassigntyping | spike | frontend/syntax | class: blocked | Implement Arithassigntyping |
| 662 | Implement Arrayassignmenttest Import Export | spike | frontend/syntax | class: blocked | Implement Arrayassignmenttest Import Export |
| 663 | Implement Arrayassignmenttest Parser Syntax | spike | frontend/syntax | class: blocked | Implement Arrayassignmenttest Parser Syntax |
| 664 | Implement Arrayaugment | spike | reference/triage | class: triage-needed | Implement Arrayaugment |
| 665 | Implement Arraybestcommontypes | spike | frontend/syntax | class: blocked | Implement Arraybestcommontypes |
| 666 | Implement Arraybindingpatternomittedexpressions | spike | frontend/syntax | class: blocked | Implement Arraybindingpatternomittedexpressions |
| 667 | Implement Arraybufferisviewnarrowstype | spike | frontend/resolver | class: blocked | Implement Arraybufferisviewnarrowstype |
| 668 | Implement Arraycast | spike | frontend/syntax | class: triage-needed | Implement Arraycast |
| 669 | Implement Arrayconcat | spike | frontend/syntax | class: blocked | Implement Arrayconcat |
| 670 | Implement Arrayconcatmap | spike | frontend/syntax | class: blocked | Implement Arrayconcatmap |
| 671 | Implement Arrayconstructors | spike | frontend/syntax | class: blocked | Implement Arrayconstructors |
| 672 | Implement Arraydestructuringinswitch | spike | frontend/syntax | class: blocked | Implement Arraydestructuringinswitch |
| 673 | Implement Arrayevery | spike | frontend/syntax | class: blocked | Implement Arrayevery |
| 674 | Implement Arrayfakeflatnocrashinferencedeclarations | spike | runtime/builtins | class: blocked | Implement Arrayfakeflatnocrashinferencedeclarations |
| 675 | Implement Arrayfilter | spike | runtime/builtins | class: blocked | Implement Arrayfilter |
| 676 | Implement Arrayfind | spike | frontend/syntax | class: triage-needed | Implement Arrayfind |
| 677 | Implement Arrayflatmap | spike | frontend/syntax | class: blocked | Implement Arrayflatmap |
| 678 | Implement Arrayflatnocrashinference | spike | frontend/syntax | class: blocked | Implement Arrayflatnocrashinference |
| 679 | Implement Arrayflatnocrashinferencedeclarations | spike | frontend/syntax | class: blocked | Implement Arrayflatnocrashinferencedeclarations |
| 680 | Implement Arrayfrom | spike | runtime/builtins | class: blocked | Implement Arrayfrom |
| 681 | Implement Arrayfromasync | spike | reference/triage | class: triage-needed | Implement Arrayfromasync |
| 682 | Implement Arrayindexwitharrayfails | spike | frontend/resolver | class: blocked | Implement Arrayindexwitharrayfails |
| 683 | Implement Arrayiterationlibes | spike | frontend/resolver | class: blocked | Implement Arrayiterationlibes |
| 684 | Implement Arrayliteralandarrayconstructorequivalence | spike | frontend/resolver | class: blocked | Implement Arrayliteralandarrayconstructorequivalence |
| 685 | Implement Arrayliteralcomments | spike | frontend/syntax | class: blocked | Implement Arrayliteralcomments |
| 686 | Implement Arrayliteralcontextualtype | spike | frontend/semantics | class: blocked | Implement Arrayliteralcontextualtype |
| 687 | Implement Arrayliteraltypeinference | spike | frontend/syntax | class: blocked | Implement Arrayliteraltypeinference |
| 688 | Implement Arrayofexportedclass | spike | frontend/syntax | class: blocked | Implement Arrayofexportedclass |
| 689 | Implement Arrayofsubtypeisassignabletoreadonlyarray | spike | frontend/semantics | class: blocked | Implement Arrayofsubtypeisassignabletoreadonlyarray |
| 690 | Implement Arrayreferencewithouttypeargs | spike | frontend/syntax | class: blocked | Implement Arrayreferencewithouttypeargs |
| 691 | Implement Arraysigchecking | spike | frontend/syntax | class: blocked | Implement Arraysigchecking |
| 692 | Implement Arrayslice | spike | frontend/syntax | class: blocked | Implement Arrayslice |
| 693 | Implement Arraytolocalestringes Name Resolution | spike | frontend/resolver | class: blocked | Implement Arraytolocalestringes Name Resolution |
| 694 | Implement Arraytolocalestringes Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Arraytolocalestringes Unknown Unsupported |
| 695 | Implement Arraytypeinsignatureofinterfaceandclass | spike | frontend/syntax | class: blocked | Implement Arraytypeinsignatureofinterfaceandclass |
| 696 | Implement Arrayconcat | spike | runtime/builtins | class: blocked | Implement Arrayconcat |
| 697 | Implement Arrowfunctioninconstructorargument | spike | frontend/syntax | class: blocked | Implement Arrowfunctioninconstructorargument |
| 698 | Implement Arrowfunctioninexpressionstatement | spike | frontend/syntax | class: blocked | Implement Arrowfunctioninexpressionstatement |
| 699 | Implement Arrowfunctionmissingcurlywithsemicolon | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctionmissingcurlywithsemicolon |
| 700 | Implement Arrowfunctionparsingdoesnotconfuseparenthesizedobjectforarrowhead | spike | frontend/syntax | class: blocked | Implement Arrowfunctionparsingdoesnotconfuseparenthesizedobjectforarrowhead |
| 701 | Implement Arrowfunctionparsinggenericinobject | spike | frontend/syntax | class: blocked | Implement Arrowfunctionparsinggenericinobject |
| 702 | Implement Arrowfunctionwithobjectliteralbody | spike | frontend/syntax | class: blocked | Implement Arrowfunctionwithobjectliteralbody |
| 703 | Implement Arrowfunctionsmissingtokens | spike | frontend/syntax | class: blocked | Implement Arrowfunctionsmissingtokens |
| 704 | Implement Asiabstract | spike | frontend/syntax | class: blocked | Implement Asiabstract |
| 705 | Implement Asiambientfunctiondeclaration | spike | frontend/syntax | class: blocked | Implement Asiambientfunctiondeclaration |
| 706 | Implement Asiarith | spike | frontend/syntax | class: blocked | Implement Asiarith |
| 707 | Implement Asibreak | spike | frontend/syntax | class: blocked | Implement Asibreak |
| 708 | Implement Asicontinue | spike | frontend/syntax | class: blocked | Implement Asicontinue |
| 709 | Implement Asiines | spike | frontend/syntax | class: blocked | Implement Asiines |
| 710 | Implement Asipublicprivateprotected | spike | frontend/semantics | class: blocked | Implement Asipublicprivateprotected |
| 711 | Implement Asireturn | spike | reference/triage | class: triage-needed | Implement Asireturn |
| 712 | Implement Assertinwrapsometypeparameter | spike | frontend/semantics | class: blocked | Implement Assertinwrapsometypeparameter |
| 713 | Implement Assertionfunctionwildcardimport | spike | frontend/syntax | class: blocked | Implement Assertionfunctionwildcardimport |
| 714 | Implement Assertionfunctionscannarrowbydiscriminant | spike | frontend/semantics | class: blocked | Implement Assertionfunctionscannarrowbydiscriminant |
| 715 | Implement Assign | spike | frontend/syntax | class: blocked | Implement Assign |
| 716 | Implement Assigntoenum | spike | frontend/syntax | class: blocked | Implement Assigntoenum |
| 717 | Implement Assigntoexistingclass | spike | frontend/syntax | class: blocked | Implement Assigntoexistingclass |
| 718 | Implement Assigntofn | spike | frontend/syntax | class: blocked | Implement Assigntofn |
| 719 | Implement Assigntoinvalidlhs | spike | frontend/syntax | class: blocked | Implement Assigntoinvalidlhs |
| 720 | Implement Assigntomodule | spike | frontend/syntax | class: blocked | Implement Assigntomodule |
| 721 | Implement Assigntoobjecttypewithprototypeproperty | spike | frontend/resolver | class: blocked | Implement Assigntoobjecttypewithprototypeproperty |
| 722 | Implement Assigntoprototype | spike | frontend/resolver | class: blocked | Implement Assigntoprototype |
| 723 | Implement Assigningfromobjecttoanythingelse | spike | frontend/resolver | class: blocked | Implement Assigningfromobjecttoanythingelse |
| 724 | Implement Assigningfunctiontotupleissueserror | spike | frontend/resolver | class: blocked | Implement Assigningfunctiontotupleissueserror |
| 725 | Implement Assignmentcompat | spike | frontend/resolver | class: blocked | Implement Assignmentcompat |
| 726 | Implement Assignmentcompatbug | spike | frontend/semantics | class: blocked | Implement Assignmentcompatbug |
| 727 | Implement Assignmentcompatforenums | spike | frontend/semantics | class: blocked | Implement Assignmentcompatforenums |
| 728 | Implement Assignmentcompatfunctionswithoptionalargs | spike | frontend/semantics | class: blocked | Implement Assignmentcompatfunctionswithoptionalargs |
| 729 | Implement Assignmentcompatinterfacewithstringindexsignature | spike | frontend/semantics | class: blocked | Implement Assignmentcompatinterfacewithstringindexsignature |
| 730 | Implement Assignmentcompatonnew | spike | frontend/resolver | class: blocked | Implement Assignmentcompatonnew |
| 731 | Implement Assignmentcompatwithoverloads | spike | frontend/semantics | class: blocked | Implement Assignmentcompatwithoverloads |
| 732 | Implement Assignmentcompatability Import Export | spike | frontend/syntax | class: blocked | Implement Assignmentcompatability Import Export |
| 733 | Implement Assignmentcompatability Name Resolution | spike | frontend/resolver | class: blocked | Implement Assignmentcompatability Name Resolution |
| 734 | Implement Assignmentcompatability Parser Syntax | spike | frontend/semantics | class: blocked | Implement Assignmentcompatability Parser Syntax |
| 735 | Implement Assignmentindexedtoprimitives | spike | frontend/syntax | class: blocked | Implement Assignmentindexedtoprimitives |
| 736 | Implement Assignmentnestedinliterals | spike | reference/triage | class: triage-needed | Implement Assignmentnestedinliterals |
| 737 | Implement Assignmentnonobjecttypeconstraints | spike | frontend/syntax | class: blocked | Implement Assignmentnonobjecttypeconstraints |
| 738 | Implement Assignmentrestelementwitherrorsourcetype | spike | frontend/resolver | class: blocked | Implement Assignmentrestelementwitherrorsourcetype |
| 739 | Implement Assignmentstricterconstraints | spike | frontend/semantics | class: blocked | Implement Assignmentstricterconstraints |
| 740 | Implement Assignmenttoanyarrayrestparameters | spike | frontend/semantics | class: blocked | Implement Assignmenttoanyarrayrestparameters |
| 741 | Implement Assignmenttoconditionalbrandedstringtemplateormapping | spike | frontend/syntax | class: blocked | Implement Assignmenttoconditionalbrandedstringtemplateormapping |
| 742 | Implement Assignmenttoexpandingarraytype | spike | frontend/syntax | class: blocked | Implement Assignmenttoexpandingarraytype |
| 743 | Implement Assignmenttofunction | spike | frontend/syntax | class: blocked | Implement Assignmenttofunction |
| 744 | Implement Assignmenttoinstantiationexpression | spike | frontend/syntax | class: triage-needed | Implement Assignmenttoinstantiationexpression |
| 745 | Implement Assignmenttoobjectandfunction | spike | frontend/syntax | class: blocked | Implement Assignmenttoobjectandfunction |
| 746 | Implement Assignmenttoparenthesizedexpression | spike | frontend/syntax | class: blocked | Implement Assignmenttoparenthesizedexpression |
| 747 | Implement Assignmenttoreferencetypes | spike | frontend/syntax | class: blocked | Implement Assignmenttoreferencetypes |
| 748 | Implement Asyncarrowinclasses | spike | runtime/builtins | class: blocked | Implement Asyncarrowinclasses |
| 749 | Implement Asyncawaitwithcapturedblockscopevar | spike | reference/triage | class: triage-needed | Implement Asyncawaitwithcapturedblockscopevar |
| 750 | Implement Asyncfunctioncontextuallytypedreturns | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctioncontextuallytypedreturns |
| 751 | Implement Asyncfunctionnoreturntype | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctionnoreturntype |
| 752 | Implement Asyncfunctionreturnexpressionerrorspans | spike | reference/triage | class: triage-needed | Implement Asyncfunctionreturnexpressionerrorspans |
| 753 | Implement Asyncfunctionreturntype Parser Syntax | spike | runtime/builtins | class: blocked | Implement Asyncfunctionreturntype Parser Syntax |
| 754 | Implement Asyncfunctionreturntype Runtime Subset | spike | reference/triage | class: triage-needed | Implement Asyncfunctionreturntype Runtime Subset |
| 755 | Implement Asyncfunctiontempvariablescoping | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctiontempvariablescoping |
| 756 | Implement Asyncfunctionwithforstatementnoinitializer | spike | reference/triage | class: triage-needed | Implement Asyncfunctionwithforstatementnoinitializer |
| 757 | Implement Asyncfunctionsacrossfiles | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctionsacrossfiles |
| 758 | Implement Asyncfunctionsandstrictnullchecks | spike | frontend/syntax | class: blocked | Implement Asyncfunctionsandstrictnullchecks |
| 759 | Implement Asynciife | spike | frontend/syntax | class: triage-needed | Implement Asynciife |
| 760 | Implement Asyncimportnestedyield | spike | reference/triage | class: triage-needed | Implement Asyncimportnestedyield |
| 761 | Implement Asynciteratorextraparameters | spike | runtime/builtins | class: blocked | Implement Asynciteratorextraparameters |
| 762 | Implement Asyncyieldstarcontextualtype | spike | frontend/semantics | class: blocked | Implement Asyncyieldstarcontextualtype |
| 763 | Implement Augmentexportequals | spike | frontend/syntax | class: blocked | Implement Augmentexportequals |
| 764 | Implement Augmentedclasswithprototypepropertyonmodule | spike | frontend/syntax | class: blocked | Implement Augmentedclasswithprototypepropertyonmodule |
| 765 | Implement Augmentedtypesclass | spike | frontend/resolver | class: blocked | Implement Augmentedtypesclass |
| 766 | Implement Augmentedtypesenum Import Export | spike | frontend/syntax | class: blocked | Implement Augmentedtypesenum Import Export |
| 767 | Implement Augmentedtypesenum Parser Syntax | spike | frontend/resolver | class: blocked | Implement Augmentedtypesenum Parser Syntax |
| 768 | Implement Augmentedtypesexternalmodule | spike | frontend/syntax | class: blocked | Implement Augmentedtypesexternalmodule |
| 769 | Implement Augmentedtypesfunction | spike | frontend/resolver | class: blocked | Implement Augmentedtypesfunction |
| 770 | Implement Augmentedtypesinterface | spike | frontend/resolver | class: blocked | Implement Augmentedtypesinterface |
| 771 | Implement Augmentedtypesmodules | spike | frontend/syntax | class: blocked | Implement Augmentedtypesmodules |
| 772 | Implement Augmentedtypesvar | spike | frontend/resolver | class: blocked | Implement Augmentedtypesvar |
| 773 | Implement Autoasiforstaticsinclassdeclaration | spike | frontend/syntax | class: blocked | Implement Autoasiforstaticsinclassdeclaration |
| 774 | Implement Autolift | spike | frontend/syntax | class: blocked | Implement Autolift |
| 775 | Implement Autotypeassignedusingdestructuringfromnevernocrash | spike | frontend/resolver | class: blocked | Implement Autotypeassignedusingdestructuringfromnevernocrash |
| 776 | Implement Apilibcheck | spike | frontend/syntax | class: blocked | Implement Apilibcheck |
| 777 | Implement Apisample Arrow Function | spike | frontend/syntax | class: blocked | Implement Apisample Arrow Function |
| 778 | Implement Apisample Import Export | spike | frontend/syntax | class: blocked | Implement Apisample Import Export |
| 779 | Implement Apisample Jsdoc | spike | frontend/syntax | class: blocked | Implement Apisample Jsdoc |
| 780 | Implement Arrowfunctionexpression | spike | frontend/syntax | class: blocked | Implement Arrowfunctionexpression |
| 781 | Implement Classdeclaration | spike | frontend/syntax | class: blocked | Implement Classdeclaration |
| 782 | Implement Classdeclarationwithinvalidconstonpropertydeclaration | spike | frontend/syntax | class: blocked | Implement Classdeclarationwithinvalidconstonpropertydeclaration |
| 783 | Implement Exportassignment | spike | frontend/syntax | class: blocked | Implement Exportassignment |
| 784 | Implement Functiondeclaration Import Export | spike | frontend/syntax | class: blocked | Implement Functiondeclaration Import Export |
| 785 | Implement Functiondeclaration Parser Syntax | spike | frontend/syntax | class: blocked | Implement Functiondeclaration Parser Syntax |
| 786 | Implement Memberaccessordeclaration | spike | frontend/syntax | class: blocked | Implement Memberaccessordeclaration |
| 787 | Implement Parameterlist | spike | frontend/syntax | class: blocked | Implement Parameterlist |
| 788 | Implement Transportstream | spike | frontend/syntax | class: blocked | Implement Transportstream |
| 789 | Implement Abstractclassinlocalscope | spike | frontend/syntax | class: blocked | Implement Abstractclassinlocalscope |
| 790 | Implement Abstractclassinlocalscopeisabstract | spike | frontend/syntax | class: blocked | Implement Abstractclassinlocalscopeisabstract |
| 791 | Implement Abstractclassunioninstantiation | spike | frontend/resolver | class: blocked | Implement Abstractclassunioninstantiation |
| 792 | Implement Abstractpropertybasics | spike | frontend/syntax | class: blocked | Implement Abstractpropertybasics |
| 793 | Implement Abstractpropertyinconstructor | spike | frontend/syntax | class: blocked | Implement Abstractpropertyinconstructor |
| 794 | Implement Abstractpropertynegative | spike | frontend/syntax | class: blocked | Implement Abstractpropertynegative |
| 795 | Implement Acceptsymbolasweaktype | spike | frontend/resolver | class: blocked | Implement Acceptsymbolasweaktype |
| 796 | Implement Acceptablealias | spike | frontend/syntax | class: blocked | Implement Acceptablealias |
| 797 | Implement Accessinstancememberfromstaticmethod | spike | frontend/resolver | class: blocked | Implement Accessinstancememberfromstaticmethod |
| 798 | Implement Accessoverriddenbaseclassmember | spike | frontend/semantics | class: blocked | Implement Accessoverriddenbaseclassmember |
| 799 | Implement Accessstaticmemberfrominstancemethod | spike | frontend/resolver | class: blocked | Implement Accessstaticmemberfrominstancemethod |
| 800 | Implement Accessoraccidentalcalldiagnostic | spike | frontend/syntax | class: blocked | Implement Accessoraccidentalcalldiagnostic |
| 801 | Implement Accessordeclarationemitjs | spike | frontend/syntax | class: blocked | Implement Accessordeclarationemitjs |
| 802 | Implement Accessordeclarationemitvisibilityerrors | spike | frontend/syntax | class: blocked | Implement Accessordeclarationemitvisibilityerrors |
| 803 | Implement Accessordeclarationorder | spike | frontend/syntax | class: blocked | Implement Accessordeclarationorder |
| 804 | Implement Accessorinambientcontextes | spike | frontend/syntax | class: blocked | Implement Accessorinambientcontextes |
| 805 | Implement Accessorinferredreturntypeerrorinreturnstatement | spike | frontend/syntax | class: blocked | Implement Accessorinferredreturntypeerrorinreturnstatement |
| 806 | Implement Accessorparameteraccessibilitymodifier | spike | frontend/syntax | class: blocked | Implement Accessorparameteraccessibilitymodifier |
| 807 | Implement Accessorwithlineterminator | spike | reference/triage | class: triage-needed | Implement Accessorwithlineterminator |
| 808 | Implement Accessorwithoutbody | spike | frontend/syntax | class: blocked | Implement Accessorwithoutbody |
| 809 | Implement Accessors | spike | frontend/syntax | class: blocked | Implement Accessors |
| 810 | Implement Accessorsinambientcontext | spike | frontend/syntax | class: blocked | Implement Accessorsinambientcontext |
| 811 | Implement Addmorecallsignaturestobasesignature | spike | frontend/syntax | class: blocked | Implement Addmorecallsignaturestobasesignature |
| 812 | Implement Aliasassignments | spike | frontend/syntax | class: blocked | Implement Aliasassignments |
| 813 | Implement Aliasbug | spike | frontend/syntax | class: blocked | Implement Aliasbug |
| 814 | Implement Aliasdoesnotduplicatesignatures | spike | frontend/syntax | class: blocked | Implement Aliasdoesnotduplicatesignatures |
| 815 | Implement Aliaserrors | spike | frontend/syntax | class: blocked | Implement Aliaserrors |
| 816 | Implement Aliasinaccessiblemodule | spike | frontend/syntax | class: blocked | Implement Aliasinaccessiblemodule |
| 817 | Implement Aliasinstantiationexpressiongenericintersectionnocrash | spike | frontend/syntax | class: blocked | Implement Aliasinstantiationexpressiongenericintersectionnocrash |
| 818 | Implement Aliasonmergedmoduleinterface | spike | frontend/syntax | class: blocked | Implement Aliasonmergedmoduleinterface |
| 819 | Implement Aliasusageinaccessorsofclass | spike | frontend/syntax | class: blocked | Implement Aliasusageinaccessorsofclass |
| 820 | Implement Aliasusageinarray | spike | frontend/syntax | class: blocked | Implement Aliasusageinarray |
| 821 | Implement Aliasusageinfunctionexpression | spike | frontend/syntax | class: blocked | Implement Aliasusageinfunctionexpression |
| 822 | Implement Aliasusageingenericfunction | spike | frontend/syntax | class: blocked | Implement Aliasusageingenericfunction |
| 823 | Implement Aliasusageinindexerofclass | spike | frontend/syntax | class: blocked | Implement Aliasusageinindexerofclass |
| 824 | Implement Aliasusageinobjectliteral | spike | frontend/syntax | class: blocked | Implement Aliasusageinobjectliteral |
| 825 | Implement Aliasusageinorexpression | spike | frontend/syntax | class: blocked | Implement Aliasusageinorexpression |
| 826 | Implement Aliasusageintypeargumentofextendsclause | spike | frontend/syntax | class: blocked | Implement Aliasusageintypeargumentofextendsclause |
| 827 | Implement Aliasusageinvarassignment | spike | frontend/syntax | class: blocked | Implement Aliasusageinvarassignment |
| 828 | Implement Aliasusedasnamevalue | spike | frontend/syntax | class: blocked | Implement Aliasusedasnamevalue |
| 829 | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer | spike | frontend/syntax | class: blocked | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer |
| 830 | Implement Aliasesinsystemmodule | spike | frontend/syntax | class: blocked | Implement Aliasesinsystemmodule |
| 831 | Implement Allowimportclausestomergewithtypes | spike | frontend/syntax | class: blocked | Implement Allowimportclausestomergewithtypes |
| 832 | Implement Allowjsclassthistypecrash | spike | reference/triage | class: triage-needed | Implement Allowjsclassthistypecrash |
| 833 | Implement Allowjscrossmonorepopackage | spike | frontend/syntax | class: blocked | Implement Allowjscrossmonorepopackage |
| 834 | Implement Allowjscheckjstypeparameternocrash | spike | frontend/syntax | class: blocked | Implement Allowjscheckjstypeparameternocrash |
| 835 | Implement Allowsyntheticdefaultimports | spike | frontend/syntax | class: blocked | Implement Allowsyntheticdefaultimports |
| 836 | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration | spike | frontend/syntax | class: blocked | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration |
| 837 | Implement Alwaysstrictmodule | spike | frontend/syntax | class: blocked | Implement Alwaysstrictmodule |
| 838 | Implement Alwaysstrictnoimplicitusestrict | spike | frontend/syntax | class: blocked | Implement Alwaysstrictnoimplicitusestrict |
| 839 | Implement Ambientclassdeclarationwithextends | spike | frontend/syntax | class: blocked | Implement Ambientclassdeclarationwithextends |
| 840 | Implement Ambientclassdeclaredbeforebase | spike | frontend/syntax | class: blocked | Implement Ambientclassdeclaredbeforebase |
| 841 | Implement Ambientconstliterals | spike | frontend/syntax | class: blocked | Implement Ambientconstliterals |
| 842 | Implement Ambientenumelementinitializer | spike | frontend/syntax | class: blocked | Implement Ambientenumelementinitializer |
| 843 | Implement Ambienterrors | spike | runtime/builtins | class: blocked | Implement Ambienterrors |
| 844 | Implement Ambientexportdefaulterrors | spike | frontend/syntax | class: blocked | Implement Ambientexportdefaulterrors |
| 845 | Implement Ambientexternalmoduleinanotherexternalmodule | spike | frontend/syntax | class: blocked | Implement Ambientexternalmoduleinanotherexternalmodule |
| 846 | Implement Ambientexternalmodulereopen | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulereopen |
| 847 | Implement Ambientexternalmodulewithinternalimportdeclaration | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulewithinternalimportdeclaration |
| 848 | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration |
| 849 | Implement Ambientexternalmodulewithrelativemodulename | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulewithrelativemodulename |
| 850 | Implement Ambientexternalmodulewithoutinternalimportdeclaration | spike | frontend/syntax | class: blocked | Implement Ambientexternalmodulewithoutinternalimportdeclaration |
| 851 | Implement Ambientfundule | spike | frontend/syntax | class: blocked | Implement Ambientfundule |
| 852 | Implement Ambientmoduleexports | spike | frontend/syntax | class: blocked | Implement Ambientmoduleexports |
| 853 | Implement Ambientmodulewithclassdeclarationwithextends | spike | frontend/syntax | class: blocked | Implement Ambientmodulewithclassdeclarationwithextends |
| 854 | Implement Ambientmodulewithtemplateliterals | spike | frontend/syntax | class: blocked | Implement Ambientmodulewithtemplateliterals |
| 855 | Implement Ambientmodules | spike | frontend/syntax | class: blocked | Implement Ambientmodules |
| 856 | Implement Ambientnamerestrictions | spike | frontend/syntax | class: blocked | Implement Ambientnamerestrictions |
| 857 | Implement Ambientrequirefunction | spike | frontend/syntax | class: blocked | Implement Ambientrequirefunction |
| 858 | Implement Ambientstatement | spike | frontend/syntax | class: blocked | Implement Ambientstatement |
| 859 | Implement Ambientwithstatements | spike | frontend/syntax | class: blocked | Implement Ambientwithstatements |
| 860 | Implement Ambiguouscallswherereturntypesagree | spike | frontend/syntax | class: blocked | Implement Ambiguouscallswherereturntypesagree |
| 861 | Implement Ambiguousgenericassertion | spike | frontend/syntax | class: blocked | Implement Ambiguousgenericassertion |
| 862 | Implement Ambiguousoverloadresolution | spike | frontend/resolver | class: blocked | Implement Ambiguousoverloadresolution |
| 863 | Implement Amddeclarationemitnoextradeclare | spike | frontend/syntax | class: blocked | Implement Amddeclarationemitnoextradeclare |
| 864 | Implement Amddependencycomment | spike | frontend/syntax | class: blocked | Implement Amddependencycomment |
| 865 | Implement Amddependencycommentname | spike | frontend/syntax | class: blocked | Implement Amddependencycommentname |
| 866 | Implement Amdlikeinputdeclarationemit | spike | frontend/syntax | class: blocked | Implement Amdlikeinputdeclarationemit |
| 867 | Implement Amdmodulebundlenoduplicatedeclarationemitcomments | spike | frontend/syntax | class: blocked | Implement Amdmodulebundlenoduplicatedeclarationemitcomments |
| 868 | Implement Amdmoduleconstenumusage | spike | frontend/syntax | class: blocked | Implement Amdmoduleconstenumusage |
| 869 | Implement Amdmodulename | spike | frontend/syntax | class: blocked | Implement Amdmodulename |
| 870 | Implement Anonclassdeclarationemitisanon | spike | frontend/syntax | class: blocked | Implement Anonclassdeclarationemitisanon |
| 871 | Implement Anonterface | spike | frontend/syntax | class: blocked | Implement Anonterface |
| 872 | Implement Anonymousclassdeclarationdoesntprintwithreadonly | spike | frontend/syntax | class: blocked | Implement Anonymousclassdeclarationdoesntprintwithreadonly |
| 873 | Implement Anonymousclassexpression | spike | frontend/syntax | class: blocked | Implement Anonymousclassexpression |
| 874 | Implement Anonymousmodules | spike | frontend/syntax | class: blocked | Implement Anonymousmodules |
| 875 | Implement Anyandunknownhavefalsycomponents | spike | frontend/resolver | class: blocked | Implement Anyandunknownhavefalsycomponents |
| 876 | Implement Anyasreturntypefornewoncall | spike | frontend/syntax | class: blocked | Implement Anyasreturntypefornewoncall |
| 877 | Implement Anydeclare | spike | frontend/syntax | class: blocked | Implement Anydeclare |
| 878 | Implement Anyidenticaltoitself | spike | frontend/syntax | class: blocked | Implement Anyidenticaltoitself |
| 879 | Implement Anyinferenceanonymousfunctions | spike | frontend/syntax | class: blocked | Implement Anyinferenceanonymousfunctions |
| 880 | Implement Argsinscope | spike | frontend/syntax | class: blocked | Implement Argsinscope |
| 881 | Implement Arguments | spike | frontend/syntax | class: blocked | Implement Arguments |
| 882 | Implement Argumentsaspropertyname Arguments Object | spike | frontend/syntax | class: blocked | Implement Argumentsaspropertyname Arguments Object |
| 883 | Implement Argumentsaspropertyname Name Resolution | spike | frontend/resolver | class: blocked | Implement Argumentsaspropertyname Name Resolution |
| 884 | Implement Argumentsbindstofunctionscopeargumentlist | spike | frontend/resolver | class: blocked | Implement Argumentsbindstofunctionscopeargumentlist |
| 885 | Implement Argumentsobjectcreatesrestforjs | spike | frontend/syntax | class: blocked | Implement Argumentsobjectcreatesrestforjs |
| 886 | Implement Argumentsobjectiterator | spike | frontend/syntax | class: blocked | Implement Argumentsobjectiterator |
| 887 | Implement Argumentspropertynameinjsmode | spike | frontend/syntax | class: blocked | Implement Argumentspropertynameinjsmode |
| 888 | Implement Argumentsreferenceinconstructor Arguments Object | spike | frontend/syntax | class: blocked | Implement Argumentsreferenceinconstructor Arguments Object |
| 889 | Implement Argumentsreferenceinconstructor Name Resolution | spike | frontend/resolver | class: blocked | Implement Argumentsreferenceinconstructor Name Resolution |
| 890 | Implement Argumentsreferenceinfunction | spike | frontend/syntax | class: blocked | Implement Argumentsreferenceinfunction |
| 891 | Implement Argumentsreferenceinmethod Arguments Object | spike | frontend/syntax | class: blocked | Implement Argumentsreferenceinmethod Arguments Object |
| 892 | Implement Argumentsreferenceinmethod Name Resolution | spike | frontend/resolver | class: blocked | Implement Argumentsreferenceinmethod Name Resolution |
| 893 | Implement Argumentsreferenceinobjectliteral | spike | frontend/syntax | class: blocked | Implement Argumentsreferenceinobjectliteral |
| 894 | Implement Argumentsusedinclassfieldinitializerorstaticinitializationblock | spike | frontend/syntax | class: blocked | Implement Argumentsusedinclassfieldinitializerorstaticinitializationblock |
| 895 | Implement Argumentsusedinobjectliteralproperty | spike | frontend/syntax | class: blocked | Implement Argumentsusedinobjectliteralproperty |
| 896 | Implement Arithassigntyping | spike | frontend/syntax | class: blocked | Implement Arithassigntyping |
| 897 | Implement Arrayassignmenttest Import Export | spike | frontend/syntax | class: blocked | Implement Arrayassignmenttest Import Export |
| 898 | Implement Arrayassignmenttest Parser Syntax | spike | frontend/syntax | class: blocked | Implement Arrayassignmenttest Parser Syntax |
| 899 | Implement Arrayaugment | spike | reference/triage | class: triage-needed | Implement Arrayaugment |
| 900 | Implement Arraybestcommontypes | spike | frontend/syntax | class: blocked | Implement Arraybestcommontypes |
| 901 | Implement Arraybindingpatternomittedexpressions | spike | frontend/syntax | class: blocked | Implement Arraybindingpatternomittedexpressions |
| 902 | Implement Arraybufferisviewnarrowstype | spike | frontend/resolver | class: blocked | Implement Arraybufferisviewnarrowstype |
| 903 | Implement Arraycast | spike | frontend/syntax | class: triage-needed | Implement Arraycast |
| 904 | Implement Arrayconcat | spike | frontend/syntax | class: blocked | Implement Arrayconcat |
| 905 | Implement Arrayconcatmap | spike | frontend/syntax | class: blocked | Implement Arrayconcatmap |
| 906 | Implement Arrayconstructors | spike | frontend/syntax | class: blocked | Implement Arrayconstructors |
| 907 | Implement Arraydestructuringinswitch | spike | frontend/syntax | class: blocked | Implement Arraydestructuringinswitch |
| 908 | Implement Arrayevery | spike | frontend/syntax | class: blocked | Implement Arrayevery |
| 909 | Implement Arrayfakeflatnocrashinferencedeclarations | spike | runtime/builtins | class: blocked | Implement Arrayfakeflatnocrashinferencedeclarations |
| 910 | Implement Arrayfilter | spike | runtime/builtins | class: blocked | Implement Arrayfilter |
| 911 | Implement Arrayfind | spike | frontend/syntax | class: triage-needed | Implement Arrayfind |
| 912 | Implement Arrayflatmap | spike | frontend/syntax | class: blocked | Implement Arrayflatmap |
| 913 | Implement Arrayflatnocrashinference | spike | frontend/syntax | class: blocked | Implement Arrayflatnocrashinference |
| 914 | Implement Arrayflatnocrashinferencedeclarations | spike | frontend/syntax | class: blocked | Implement Arrayflatnocrashinferencedeclarations |
| 915 | Implement Arrayfrom | spike | runtime/builtins | class: blocked | Implement Arrayfrom |
| 916 | Implement Arrayfromasync | spike | reference/triage | class: triage-needed | Implement Arrayfromasync |
| 917 | Implement Arrayindexwitharrayfails | spike | frontend/resolver | class: blocked | Implement Arrayindexwitharrayfails |
| 918 | Implement Arrayiterationlibes | spike | frontend/resolver | class: blocked | Implement Arrayiterationlibes |
| 919 | Implement Arrayliteralandarrayconstructorequivalence | spike | frontend/resolver | class: blocked | Implement Arrayliteralandarrayconstructorequivalence |
| 920 | Implement Arrayliteralcomments | spike | frontend/syntax | class: blocked | Implement Arrayliteralcomments |
| 921 | Implement Arrayliteralcontextualtype | spike | frontend/semantics | class: blocked | Implement Arrayliteralcontextualtype |
| 922 | Implement Arrayliteraltypeinference | spike | frontend/syntax | class: blocked | Implement Arrayliteraltypeinference |
| 923 | Implement Arrayofexportedclass | spike | frontend/syntax | class: blocked | Implement Arrayofexportedclass |
| 924 | Implement Arrayofsubtypeisassignabletoreadonlyarray | spike | frontend/semantics | class: blocked | Implement Arrayofsubtypeisassignabletoreadonlyarray |
| 925 | Implement Arrayreferencewithouttypeargs | spike | frontend/syntax | class: blocked | Implement Arrayreferencewithouttypeargs |
| 926 | Implement Arraysigchecking | spike | frontend/syntax | class: blocked | Implement Arraysigchecking |
| 927 | Implement Arrayslice | spike | frontend/syntax | class: blocked | Implement Arrayslice |
| 928 | Implement Arraytolocalestringes Name Resolution | spike | frontend/resolver | class: blocked | Implement Arraytolocalestringes Name Resolution |
| 929 | Implement Arraytolocalestringes Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Arraytolocalestringes Unknown Unsupported |
| 930 | Implement Arraytypeinsignatureofinterfaceandclass | spike | frontend/syntax | class: blocked | Implement Arraytypeinsignatureofinterfaceandclass |
| 931 | Implement Arrayconcat | spike | runtime/builtins | class: blocked | Implement Arrayconcat |
| 932 | Implement Arrowfunctioninconstructorargument | spike | frontend/syntax | class: blocked | Implement Arrowfunctioninconstructorargument |
| 933 | Implement Arrowfunctioninexpressionstatement | spike | frontend/syntax | class: blocked | Implement Arrowfunctioninexpressionstatement |
| 934 | Implement Arrowfunctionmissingcurlywithsemicolon | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctionmissingcurlywithsemicolon |
| 935 | Implement Arrowfunctionparsingdoesnotconfuseparenthesizedobjectforarrowhead | spike | frontend/syntax | class: blocked | Implement Arrowfunctionparsingdoesnotconfuseparenthesizedobjectforarrowhead |
| 936 | Implement Arrowfunctionparsinggenericinobject | spike | frontend/syntax | class: blocked | Implement Arrowfunctionparsinggenericinobject |
| 937 | Implement Arrowfunctionwithobjectliteralbody | spike | frontend/syntax | class: blocked | Implement Arrowfunctionwithobjectliteralbody |
| 938 | Implement Arrowfunctionsmissingtokens | spike | frontend/syntax | class: blocked | Implement Arrowfunctionsmissingtokens |
| 939 | Implement Asiabstract | spike | frontend/syntax | class: blocked | Implement Asiabstract |
| 940 | Implement Asiambientfunctiondeclaration | spike | frontend/syntax | class: blocked | Implement Asiambientfunctiondeclaration |
| 941 | Implement Asiarith | spike | frontend/syntax | class: blocked | Implement Asiarith |
| 942 | Implement Asibreak | spike | frontend/syntax | class: blocked | Implement Asibreak |
| 943 | Implement Asicontinue | spike | frontend/syntax | class: blocked | Implement Asicontinue |
| 944 | Implement Asiines | spike | frontend/syntax | class: blocked | Implement Asiines |
| 945 | Implement Asipublicprivateprotected | spike | frontend/semantics | class: blocked | Implement Asipublicprivateprotected |
| 946 | Implement Asireturn | spike | reference/triage | class: triage-needed | Implement Asireturn |
| 947 | Implement Assertinwrapsometypeparameter | spike | frontend/semantics | class: blocked | Implement Assertinwrapsometypeparameter |
| 948 | Implement Assertionfunctionwildcardimport | spike | frontend/syntax | class: blocked | Implement Assertionfunctionwildcardimport |
| 949 | Implement Assertionfunctionscannarrowbydiscriminant | spike | frontend/semantics | class: blocked | Implement Assertionfunctionscannarrowbydiscriminant |
| 950 | Implement Assign | spike | frontend/syntax | class: blocked | Implement Assign |
| 951 | Implement Assigntoenum | spike | frontend/syntax | class: blocked | Implement Assigntoenum |
| 952 | Implement Assigntoexistingclass | spike | frontend/syntax | class: blocked | Implement Assigntoexistingclass |
| 953 | Implement Assigntofn | spike | frontend/syntax | class: blocked | Implement Assigntofn |
| 954 | Implement Assigntoinvalidlhs | spike | frontend/syntax | class: blocked | Implement Assigntoinvalidlhs |
| 955 | Implement Assigntomodule | spike | frontend/syntax | class: blocked | Implement Assigntomodule |
| 956 | Implement Assigntoobjecttypewithprototypeproperty | spike | frontend/resolver | class: blocked | Implement Assigntoobjecttypewithprototypeproperty |
| 957 | Implement Assigntoprototype | spike | frontend/resolver | class: blocked | Implement Assigntoprototype |
| 958 | Implement Assigningfromobjecttoanythingelse | spike | frontend/resolver | class: blocked | Implement Assigningfromobjecttoanythingelse |
| 959 | Implement Assigningfunctiontotupleissueserror | spike | frontend/resolver | class: blocked | Implement Assigningfunctiontotupleissueserror |
| 960 | Implement Assignmentcompat | spike | frontend/resolver | class: blocked | Implement Assignmentcompat |
| 961 | Implement Assignmentcompatbug | spike | frontend/semantics | class: blocked | Implement Assignmentcompatbug |
| 962 | Implement Assignmentcompatforenums | spike | frontend/semantics | class: blocked | Implement Assignmentcompatforenums |
| 963 | Implement Assignmentcompatfunctionswithoptionalargs | spike | frontend/semantics | class: blocked | Implement Assignmentcompatfunctionswithoptionalargs |
| 964 | Implement Assignmentcompatinterfacewithstringindexsignature | spike | frontend/semantics | class: blocked | Implement Assignmentcompatinterfacewithstringindexsignature |
| 965 | Implement Assignmentcompatonnew | spike | frontend/resolver | class: blocked | Implement Assignmentcompatonnew |
| 966 | Implement Assignmentcompatwithoverloads | spike | frontend/semantics | class: blocked | Implement Assignmentcompatwithoverloads |
| 967 | Implement Assignmentcompatability Import Export | spike | frontend/syntax | class: blocked | Implement Assignmentcompatability Import Export |
| 968 | Implement Assignmentcompatability Name Resolution | spike | frontend/resolver | class: blocked | Implement Assignmentcompatability Name Resolution |
| 969 | Implement Assignmentcompatability Parser Syntax | spike | frontend/semantics | class: blocked | Implement Assignmentcompatability Parser Syntax |
| 970 | Implement Assignmentindexedtoprimitives | spike | frontend/syntax | class: blocked | Implement Assignmentindexedtoprimitives |
| 971 | Implement Assignmentnestedinliterals | spike | reference/triage | class: triage-needed | Implement Assignmentnestedinliterals |
| 972 | Implement Assignmentnonobjecttypeconstraints | spike | frontend/syntax | class: blocked | Implement Assignmentnonobjecttypeconstraints |
| 973 | Implement Assignmentrestelementwitherrorsourcetype | spike | frontend/resolver | class: blocked | Implement Assignmentrestelementwitherrorsourcetype |
| 974 | Implement Assignmentstricterconstraints | spike | frontend/semantics | class: blocked | Implement Assignmentstricterconstraints |
| 975 | Implement Assignmenttoanyarrayrestparameters | spike | frontend/semantics | class: blocked | Implement Assignmenttoanyarrayrestparameters |
| 976 | Implement Assignmenttoconditionalbrandedstringtemplateormapping | spike | frontend/syntax | class: blocked | Implement Assignmenttoconditionalbrandedstringtemplateormapping |
| 977 | Implement Assignmenttoexpandingarraytype | spike | frontend/syntax | class: blocked | Implement Assignmenttoexpandingarraytype |
| 978 | Implement Assignmenttofunction | spike | frontend/syntax | class: blocked | Implement Assignmenttofunction |
| 979 | Implement Assignmenttoinstantiationexpression | spike | frontend/syntax | class: triage-needed | Implement Assignmenttoinstantiationexpression |
| 980 | Implement Assignmenttoobjectandfunction | spike | frontend/syntax | class: blocked | Implement Assignmenttoobjectandfunction |
| 981 | Implement Assignmenttoparenthesizedexpression | spike | frontend/syntax | class: blocked | Implement Assignmenttoparenthesizedexpression |
| 982 | Implement Assignmenttoreferencetypes | spike | frontend/syntax | class: blocked | Implement Assignmenttoreferencetypes |
| 983 | Implement Asyncarrowinclasses | spike | runtime/builtins | class: blocked | Implement Asyncarrowinclasses |
| 984 | Implement Asyncawaitwithcapturedblockscopevar | spike | reference/triage | class: triage-needed | Implement Asyncawaitwithcapturedblockscopevar |
| 985 | Implement Asyncfunctioncontextuallytypedreturns | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctioncontextuallytypedreturns |
| 986 | Implement Asyncfunctionnoreturntype | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctionnoreturntype |
| 987 | Implement Asyncfunctionreturnexpressionerrorspans | spike | reference/triage | class: triage-needed | Implement Asyncfunctionreturnexpressionerrorspans |
| 988 | Implement Asyncfunctionreturntype Parser Syntax | spike | runtime/builtins | class: blocked | Implement Asyncfunctionreturntype Parser Syntax |
| 989 | Implement Asyncfunctionreturntype Runtime Subset | spike | reference/triage | class: triage-needed | Implement Asyncfunctionreturntype Runtime Subset |
| 990 | Implement Asyncfunctiontempvariablescoping | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctiontempvariablescoping |
| 991 | Implement Asyncfunctionwithforstatementnoinitializer | spike | reference/triage | class: triage-needed | Implement Asyncfunctionwithforstatementnoinitializer |
| 992 | Implement Asyncfunctionsacrossfiles | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctionsacrossfiles |
| 993 | Implement Asyncfunctionsandstrictnullchecks | spike | frontend/syntax | class: blocked | Implement Asyncfunctionsandstrictnullchecks |
| 994 | Implement Asynciife | spike | frontend/syntax | class: triage-needed | Implement Asynciife |
| 995 | Implement Asyncimportnestedyield | spike | reference/triage | class: triage-needed | Implement Asyncimportnestedyield |
| 996 | Implement Asynciteratorextraparameters | spike | runtime/builtins | class: blocked | Implement Asynciteratorextraparameters |
| 997 | Implement Asyncyieldstarcontextualtype | spike | frontend/semantics | class: blocked | Implement Asyncyieldstarcontextualtype |
| 998 | Implement Augmentexportequals | spike | frontend/syntax | class: blocked | Implement Augmentexportequals |
| 999 | Implement Augmentedclasswithprototypepropertyonmodule | spike | frontend/syntax | class: blocked | Implement Augmentedclasswithprototypepropertyonmodule |
| 1000 | Implement Augmentedtypesclass | spike | frontend/resolver | class: blocked | Implement Augmentedtypesclass |
| 1001 | Implement Augmentedtypesenum Import Export | spike | frontend/syntax | class: blocked | Implement Augmentedtypesenum Import Export |
| 1002 | Implement Augmentedtypesenum Parser Syntax | spike | frontend/resolver | class: blocked | Implement Augmentedtypesenum Parser Syntax |
| 1003 | Implement Augmentedtypesexternalmodule | spike | frontend/syntax | class: blocked | Implement Augmentedtypesexternalmodule |
| 1004 | Implement Augmentedtypesfunction | spike | frontend/resolver | class: blocked | Implement Augmentedtypesfunction |
| 1005 | Implement Augmentedtypesinterface | spike | frontend/resolver | class: blocked | Implement Augmentedtypesinterface |
| 1006 | Implement Augmentedtypesmodules | spike | frontend/syntax | class: blocked | Implement Augmentedtypesmodules |
| 1007 | Implement Augmentedtypesvar | spike | frontend/resolver | class: blocked | Implement Augmentedtypesvar |
| 1008 | Implement Autoasiforstaticsinclassdeclaration | spike | frontend/syntax | class: blocked | Implement Autoasiforstaticsinclassdeclaration |
| 1009 | Implement Autolift | spike | frontend/syntax | class: blocked | Implement Autolift |
| 1010 | Implement Autotypeassignedusingdestructuringfromnevernocrash | spike | frontend/resolver | class: blocked | Implement Autotypeassignedusingdestructuringfromnevernocrash |
| 1011 | Implement Autolift | spike | frontend/syntax | class: blocked | Implement Autolift |
| 1012 | Implement Autonumberinginenums | spike | frontend/syntax | class: blocked | Implement Autonumberinginenums |
| 1013 | Implement Avoid | spike | frontend/syntax | class: blocked | Implement Avoid |
| 1014 | Implement Avoidcyclewithvoidexpressionreturnedfromarrow | spike | frontend/syntax | class: blocked | Implement Avoidcyclewithvoidexpressionreturnedfromarrow |
| 1015 | Implement Avoidnarrowingusingconstvariablefrombindingelementwithliteralinitializer | spike | frontend/syntax | class: blocked | Implement Avoidnarrowingusingconstvariablefrombindingelementwithliteralinitializer |
| 1016 | Implement Awaitcallexpressioninsyncfunction | spike | reference/triage | class: triage-needed | Implement Awaitcallexpressioninsyncfunction |
| 1017 | Implement Awaitexpressioninnercommentemit | spike | reference/triage | class: triage-needed | Implement Awaitexpressioninnercommentemit |
| 1018 | Implement Awaitinclassinasyncfunction | spike | reference/triage | class: triage-needed | Implement Awaitinclassinasyncfunction |
| 1019 | Implement Awaitinnonasyncfunction | spike | reference/triage | class: triage-needed | Implement Awaitinnonasyncfunction |
| 1020 | Implement Awaitliteralvalues | spike | reference/triage | class: triage-needed | Implement Awaitliteralvalues |
| 1021 | Implement Awaitunionpromise | spike | reference/triage | class: triage-needed | Implement Awaitunionpromise |
| 1022 | Implement Awaitedtype | spike | reference/triage | class: triage-needed | Implement Awaitedtype |
| 1023 | Implement Awaitedtypecrash | spike | reference/triage | class: triage-needed | Implement Awaitedtypecrash |
| 1024 | Implement Awaitedtypenolib | spike | runtime/builtins | class: blocked | Implement Awaitedtypenolib |
| 1025 | Implement Awaitedtypestrictnull | spike | runtime/builtins | class: blocked | Implement Awaitedtypestrictnull |
| 1026 | Implement Badarrayindex | spike | frontend/syntax | class: triage-needed | Implement Badarrayindex |
| 1027 | Implement Badarraysyntax | spike | frontend/syntax | class: blocked | Implement Badarraysyntax |
| 1028 | Implement Badexternalmodulereference | spike | frontend/syntax | class: blocked | Implement Badexternalmodulereference |
| 1029 | Implement Badinferencelowerprioritythangoodinference | spike | frontend/syntax | class: blocked | Implement Badinferencelowerprioritythangoodinference |
| 1030 | Implement Badoverloaderror | spike | frontend/syntax | class: triage-needed | Implement Badoverloaderror |
| 1031 | Implement Badthisbinding | spike | frontend/syntax | class: blocked | Implement Badthisbinding |
| 1032 | Implement Banginmodulename | spike | frontend/syntax | class: blocked | Implement Banginmodulename |
| 1033 | Implement Basecheck | spike | frontend/resolver | class: blocked | Implement Basecheck |
| 1034 | Implement Baseclassimprovedmismatcherrors | spike | frontend/semantics | class: blocked | Implement Baseclassimprovedmismatcherrors |
| 1035 | Implement Baseconstraintofdecorator | spike | frontend/syntax | class: blocked | Implement Baseconstraintofdecorator |
| 1036 | Implement Baseexpressiontypeparameters | spike | frontend/semantics | class: blocked | Implement Baseexpressiontypeparameters |
| 1037 | Implement Baseindexsignatureresolution | spike | frontend/syntax | class: blocked | Implement Baseindexsignatureresolution |
| 1038 | Implement Basetypeafterderivedtype | spike | frontend/syntax | class: blocked | Implement Basetypeafterderivedtype |
| 1039 | Implement Basetypeorderchecking | spike | frontend/syntax | class: blocked | Implement Basetypeorderchecking |
| 1040 | Implement Basetypeprivatememberclash | spike | frontend/semantics | class: blocked | Implement Basetypeprivatememberclash |
| 1041 | Implement Basetypewrappinginstantiationchain | spike | frontend/syntax | class: blocked | Implement Basetypewrappinginstantiationchain |
| 1042 | Implement Bases | spike | frontend/syntax | class: blocked | Implement Bases |
| 1043 | Implement Bestchoicetype | spike | frontend/syntax | class: triage-needed | Implement Bestchoicetype |
| 1044 | Implement Bestcommontypewithcontextualtyping | spike | frontend/resolver | class: blocked | Implement Bestcommontypewithcontextualtyping |
| 1045 | Implement Bettererrorforaccidentalcall | spike | frontend/syntax | class: triage-needed | Implement Bettererrorforaccidentalcall |
| 1046 | Implement Bigintwithtargetes | spike | runtime/builtins | class: blocked | Implement Bigintwithtargetes |
| 1047 | Implement Bigintwithtargetlessthanes | spike | runtime/builtins | class: blocked | Implement Bigintwithtargetlessthanes |
| 1048 | Implement Bigint | spike | frontend/resolver | class: blocked | Implement Bigint |
| 1049 | Implement Bigintambientminimal | spike | runtime/builtins | class: blocked | Implement Bigintambientminimal |
| 1050 | Implement Bigintarbirtraryidentifier | spike | runtime/builtins | class: blocked | Implement Bigintarbirtraryidentifier |
| 1051 | Implement Bigintindex | spike | frontend/resolver | class: blocked | Implement Bigintindex |
| 1052 | Implement Bigintpropertyname | spike | runtime/builtins | class: blocked | Implement Bigintpropertyname |
| 1053 | Implement Bigintwithlib | spike | runtime/builtins | class: blocked | Implement Bigintwithlib |
| 1054 | Implement Bigintwithoutlib | spike | runtime/builtins | class: blocked | Implement Bigintwithoutlib |
| 1055 | Implement Binaryarithmatic | spike | frontend/syntax | class: blocked | Implement Binaryarithmatic |
| 1056 | Implement Binaryarithmeticcontrolflowgraphnottoolarge | spike | frontend/semantics | class: blocked | Implement Binaryarithmeticcontrolflowgraphnottoolarge |
| 1057 | Implement Bind | spike | frontend/syntax | class: blocked | Implement Bind |
| 1058 | Implement Binderbinaryexpressionstress | spike | reference/triage | class: triage-needed | Implement Binderbinaryexpressionstress |
| 1059 | Implement Binderbinaryexpressionstressjs | spike | reference/triage | class: triage-needed | Implement Binderbinaryexpressionstressjs |
| 1060 | Implement Bindingpatterncannotbeonlyinferencesource | spike | reference/triage | class: triage-needed | Implement Bindingpatterncannotbeonlyinferencesource |
| 1061 | Implement Bindingpatterncontextualtypedoesnotcausewidening | spike | frontend/resolver | class: blocked | Implement Bindingpatterncontextualtypedoesnotcausewidening |
| 1062 | Implement Bindingpatterninparameter | spike | frontend/syntax | class: blocked | Implement Bindingpatterninparameter |
| 1063 | Implement Bindingpatternomittedexpressionnesting | spike | frontend/syntax | class: blocked | Implement Bindingpatternomittedexpressionnesting |
| 1064 | Implement Binopassignmentshouldhavetype | spike | frontend/syntax | class: blocked | Implement Binopassignmentshouldhavetype |
| 1065 | Implement Bitwisecompoundassignmentoperators | spike | frontend/syntax | class: triage-needed | Implement Bitwisecompoundassignmentoperators |
| 1066 | Implement Blockscopedbindingcapturethisinfunction | spike | reference/triage | class: triage-needed | Implement Blockscopedbindingcapturethisinfunction |
| 1067 | Implement Blockscopedbindingusedbeforedef | spike | frontend/syntax | class: blocked | Implement Blockscopedbindingusedbeforedef |
| 1068 | Implement Blockscopedbindingsreassignedinloop Name Resolution | spike | frontend/resolver | class: blocked | Implement Blockscopedbindingsreassignedinloop Name Resolution |
| 1069 | Implement Blockscopedbindingsreassignedinloop Scope Analysis | spike | frontend/syntax | class: blocked | Implement Blockscopedbindingsreassignedinloop Scope Analysis |
| 1070 | Implement Blockscopedenumvariablesusebeforedef Enum | spike | frontend/syntax | class: blocked | Implement Blockscopedenumvariablesusebeforedef Enum |
| 1071 | Implement Blockscopedenumvariablesusebeforedef Import Export | spike | frontend/syntax | class: blocked | Implement Blockscopedenumvariablesusebeforedef Import Export |
| 1072 | Implement Blockscopedfunctiondeclarationes | spike | frontend/resolver | class: blocked | Implement Blockscopedfunctiondeclarationes |
| 1073 | Implement Blockscopedfunctiondeclarationinstrictclass | spike | frontend/resolver | class: blocked | Implement Blockscopedfunctiondeclarationinstrictclass |
| 1074 | Implement Blockscopedfunctiondeclarationinstrictmodule | spike | frontend/syntax | class: blocked | Implement Blockscopedfunctiondeclarationinstrictmodule |
| 1075 | Implement Blockscopedfunctiondeclarationstrictes | spike | frontend/resolver | class: blocked | Implement Blockscopedfunctiondeclarationstrictes |
| 1076 | Implement Blockscopednamespacedifferentfile | spike | frontend/syntax | class: blocked | Implement Blockscopednamespacedifferentfile |
| 1077 | Implement Blockscopedsamenamefunctiondeclarationes | spike | reference/triage | class: triage-needed | Implement Blockscopedsamenamefunctiondeclarationes |
| 1078 | Implement Blockscopedsamenamefunctiondeclarationstrictes | spike | reference/triage | class: triage-needed | Implement Blockscopedsamenamefunctiondeclarationstrictes |
| 1079 | Implement Blockscopedvariablesusebeforedef | spike | frontend/syntax | class: blocked | Implement Blockscopedvariablesusebeforedef |
| 1080 | Implement Bluebirdstaticthis | spike | frontend/syntax | class: blocked | Implement Bluebirdstaticthis |
| 1081 | Implement Booleanassignment | spike | frontend/resolver | class: blocked | Implement Booleanassignment |
| 1082 | Implement Booleanfilteranyarray | spike | frontend/syntax | class: blocked | Implement Booleanfilteranyarray |
| 1083 | Implement Breakiniterationorswitchstatement | spike | frontend/resolver | class: blocked | Implement Breakiniterationorswitchstatement |
| 1084 | Implement Breaknotiniterationorswitchstatement | spike | frontend/syntax | class: blocked | Implement Breaknotiniterationorswitchstatement |
| 1085 | Implement Breaktarget | spike | frontend/syntax | class: blocked | Implement Breaktarget |
| 1086 | Implement Builtiniterator | spike | frontend/syntax | class: triage-needed | Implement Builtiniterator |
| 1087 | Implement Bundleddtslateexportrenaming | spike | frontend/syntax | class: blocked | Implement Bundleddtslateexportrenaming |
| 1088 | Implement Cacheresolutions | spike | frontend/syntax | class: blocked | Implement Cacheresolutions |
| 1089 | Implement Cachedcontextualtypes | spike | frontend/semantics | class: blocked | Implement Cachedcontextualtypes |
| 1090 | Implement Cachedmoduleresolution | spike | frontend/syntax | class: blocked | Implement Cachedmoduleresolution |
| 1091 | Implement Callconstructassignment | spike | frontend/syntax | class: blocked | Implement Callconstructassignment |
| 1092 | Implement Callexpressionwithmissingtypeargument | spike | frontend/semantics | class: blocked | Implement Callexpressionwithmissingtypeargument |
| 1093 | Implement Callexpressionwithtypeparameterconstrainedtooutertypeparameter | spike | frontend/syntax | class: blocked | Implement Callexpressionwithtypeparameterconstrainedtooutertypeparameter |
| 1094 | Implement Callofconditionaltypewithconcretebranches | spike | frontend/syntax | class: blocked | Implement Callofconditionaltypewithconcretebranches |
| 1095 | Implement Callonclass | spike | frontend/resolver | class: blocked | Implement Callonclass |
| 1096 | Implement Calloninstance | spike | frontend/resolver | class: blocked | Implement Calloninstance |
| 1097 | Implement Calloverloadviaelementaccessexpression | spike | frontend/syntax | class: blocked | Implement Calloverloadviaelementaccessexpression |
| 1098 | Implement Calloverloads Class | spike | frontend/syntax | class: blocked | Implement Calloverloads Class |
| 1099 | Implement Calloverloads Parser Syntax | spike | frontend/semantics | class: blocked | Implement Calloverloads Parser Syntax |
| 1100 | Implement Callsignaturefunctionoverload | spike | frontend/semantics | class: blocked | Implement Callsignaturefunctionoverload |
| 1101 | Implement Callsignaturesshouldberesolvedbeforespecialization | spike | frontend/syntax | class: blocked | Implement Callsignaturesshouldberesolvedbeforespecialization |
| 1102 | Implement Callbackargsdifferbyoptionality | spike | frontend/syntax | class: blocked | Implement Callbackargsdifferbyoptionality |
| 1103 | Implement Callbacksdontsharetypes | spike | frontend/syntax | class: blocked | Implement Callbacksdontsharetypes |
| 1104 | Implement Cannotinvokenewonerrorexpression | spike | frontend/syntax | class: blocked | Implement Cannotinvokenewonerrorexpression |
| 1105 | Implement Cannotinvokenewonindexexpression | spike | frontend/resolver | class: blocked | Implement Cannotinvokenewonindexexpression |
| 1106 | Implement Capturesuperpropertyaccessinsupercall | spike | frontend/syntax | class: blocked | Implement Capturesuperpropertyaccessinsupercall |
| 1107 | Implement Capturedletconstinloop Arrow Function | spike | frontend/syntax | class: blocked | Implement Capturedletconstinloop Arrow Function |
| 1108 | Implement Capturedletconstinloop Duplicate Local | spike | reference/triage | class: triage-needed | Implement Capturedletconstinloop Duplicate Local |
| 1109 | Implement Capturedletconstinloop Import Export | spike | frontend/syntax | class: blocked | Implement Capturedletconstinloop Import Export |
| 1110 | Implement Capturedletconstinloop Name Resolution | spike | frontend/resolver | class: blocked | Implement Capturedletconstinloop Name Resolution |
| 1111 | Implement Capturedletconstinloop Parser Syntax | spike | frontend/syntax | class: blocked | Implement Capturedletconstinloop Parser Syntax |
| 1112 | Implement Capturedparametersininitializers | spike | frontend/syntax | class: triage-needed | Implement Capturedparametersininitializers |
| 1113 | Implement Capturedshorthandpropertyassignmentnocheck | spike | frontend/syntax | class: blocked | Implement Capturedshorthandpropertyassignmentnocheck |
| 1114 | Implement Capturedvarinloop | spike | frontend/syntax | class: blocked | Implement Capturedvarinloop |
| 1115 | Implement Caseinsensitivefilesystemwithcapsimporttypedeclarations | spike | frontend/syntax | class: blocked | Implement Caseinsensitivefilesystemwithcapsimporttypedeclarations |
| 1116 | Implement Castexpressionparentheses | spike | frontend/syntax | class: blocked | Implement Castexpressionparentheses |
| 1117 | Implement Castfunctionexpressionshouldbeparenthesized | spike | frontend/syntax | class: blocked | Implement Castfunctionexpressionshouldbeparenthesized |
| 1118 | Implement Castnewobjectbug | spike | frontend/syntax | class: blocked | Implement Castnewobjectbug |
| 1119 | Implement Castofawait | spike | reference/triage | class: triage-needed | Implement Castofawait |
| 1120 | Implement Castparentheses | spike | frontend/syntax | class: triage-needed | Implement Castparentheses |
| 1121 | Implement Casttest | spike | frontend/syntax | class: triage-needed | Implement Casttest |
| 1122 | Implement Catch | spike | reference/triage | class: triage-needed | Implement Catch |
| 1123 | Implement Catchclausewithinitializer | spike | frontend/syntax | class: blocked | Implement Catchclausewithinitializer |
| 1124 | Implement Cf | spike | frontend/resolver | class: blocked | Implement Cf |
| 1125 | Implement Chainedassignment | spike | frontend/syntax | class: blocked | Implement Chainedassignment |
| 1126 | Implement Chainedcallswithtypeparameterconstrainedtoothertypeparameter | spike | frontend/semantics | class: blocked | Implement Chainedcallswithtypeparameterconstrainedtoothertypeparameter |
| 1127 | Implement Chainedimportalias | spike | frontend/syntax | class: blocked | Implement Chainedimportalias |
| 1128 | Implement Chainedspecializationtoobjecttypeliteral | spike | frontend/syntax | class: blocked | Implement Chainedspecializationtoobjecttypeliteral |
| 1129 | Implement Checkdestructuringshorthandassigment Destructuring | spike | frontend/syntax | class: blocked | Implement Checkdestructuringshorthandassigment Destructuring |
| 1130 | Implement Checkdestructuringshorthandassigment Name Resolution | spike | frontend/resolver | class: blocked | Implement Checkdestructuringshorthandassigment Name Resolution |
| 1131 | Implement Checkforobjecttoostrict | spike | frontend/syntax | class: blocked | Implement Checkforobjecttoostrict |
| 1132 | Implement Checkindexconstraintofjavascriptclassexpression | spike | frontend/resolver | class: blocked | Implement Checkindexconstraintofjavascriptclassexpression |
| 1133 | Implement Checkinfiniteexpansiontermination | spike | frontend/resolver | class: blocked | Implement Checkinfiniteexpansiontermination |
| 1134 | Implement Checkinheritedproperty | spike | frontend/syntax | class: blocked | Implement Checkinheritedproperty |
| 1135 | Implement Checkjsfiles | spike | frontend/syntax | class: blocked | Implement Checkjsfiles |
| 1136 | Implement Checkjsobjectliteralindexsignatures | spike | frontend/syntax | class: blocked | Implement Checkjsobjectliteralindexsignatures |
| 1137 | Implement Checkjstypedefnounusedlocalmarked | spike | frontend/resolver | class: blocked | Implement Checkjstypedefnounusedlocalmarked |
| 1138 | Implement Checkjsdoctypetagonexportassignment | spike | frontend/syntax | class: blocked | Implement Checkjsdoctypetagonexportassignment |
| 1139 | Implement Checkjsxnotseterror | spike | reference/triage | class: blocked | Implement Checkjsxnotseterror |
| 1140 | Implement Checkmergedglobalumdsymbol | spike | frontend/syntax | class: blocked | Implement Checkmergedglobalumdsymbol |
| 1141 | Implement Checksupercallbeforethisaccess | spike | frontend/syntax | class: blocked | Implement Checksupercallbeforethisaccess |
| 1142 | Implement Checksupercallbeforethisaccessing Class | spike | frontend/syntax | class: blocked | Implement Checksupercallbeforethisaccessing Class |
| 1143 | Implement Checksupercallbeforethisaccessing Parser Syntax | spike | frontend/syntax | class: blocked | Implement Checksupercallbeforethisaccessing Parser Syntax |
| 1144 | Implement Checkswitchstatementifcasetypeisstring | spike | frontend/syntax | class: blocked | Implement Checkswitchstatementifcasetypeisstring |
| 1145 | Implement Checktypepredicateforredundantproperties | spike | frontend/syntax | class: blocked | Implement Checktypepredicateforredundantproperties |
| 1146 | Implement Checkerinitializationcrash | spike | frontend/syntax | class: blocked | Implement Checkerinitializationcrash |
| 1147 | Implement Checkingobjectdefinepropertyonfunctionnonexistentpropertynocrash | spike | frontend/syntax | class: blocked | Implement Checkingobjectdefinepropertyonfunctionnonexistentpropertynocrash |
| 1148 | Implement Checkingobjectwiththisinnamepositionnocrash | spike | frontend/syntax | class: blocked | Implement Checkingobjectwiththisinnamepositionnocrash |
| 1149 | Implement Circularaccessorannotations | spike | frontend/semantics | class: blocked | Implement Circularaccessorannotations |
| 1150 | Implement Circularbaseconstraint | spike | frontend/syntax | class: triage-needed | Implement Circularbaseconstraint |
| 1151 | Implement Circularconstraintyieldsappropriateerror | spike | frontend/semantics | class: blocked | Implement Circularconstraintyieldsappropriateerror |
| 1152 | Implement Circularconstructorwithreturn | spike | frontend/syntax | class: blocked | Implement Circularconstructorwithreturn |
| 1153 | Implement Circularcontextualmappedtype | spike | frontend/resolver | class: blocked | Implement Circularcontextualmappedtype |
| 1154 | Implement Circularcontextualreturntype | spike | frontend/resolver | class: blocked | Implement Circularcontextualreturntype |
| 1155 | Implement Circularinferredtypeofvariable | spike | frontend/syntax | class: blocked | Implement Circularinferredtypeofvariable |
| 1156 | Implement Circularinlinemappedgenerictupletypenocrash | spike | frontend/syntax | class: blocked | Implement Circularinlinemappedgenerictupletypenocrash |
| 1157 | Implement Circularinstantiationexpression | spike | frontend/resolver | class: blocked | Implement Circularinstantiationexpression |
| 1158 | Implement Circularmappedtypeconstraint | spike | frontend/syntax | class: blocked | Implement Circularmappedtypeconstraint |
| 1159 | Implement Circularmoduleimports | spike | frontend/syntax | class: blocked | Implement Circularmoduleimports |
| 1160 | Implement Circularobjectliteralaccessors | spike | frontend/syntax | class: blocked | Implement Circularobjectliteralaccessors |
| 1161 | Implement Circularoptionalityremoval | spike | frontend/resolver | class: blocked | Implement Circularoptionalityremoval |
| 1162 | Implement Circularreferenceinimport | spike | frontend/syntax | class: blocked | Implement Circularreferenceinimport |
| 1163 | Implement Circularreferenceinreturntype Name Resolution | spike | frontend/resolver | class: blocked | Implement Circularreferenceinreturntype Name Resolution |
| 1164 | Implement Circularreferenceinreturntype Parser Syntax | spike | frontend/syntax | class: blocked | Implement Circularreferenceinreturntype Parser Syntax |
| 1165 | Implement Circularresolvedsignature | spike | frontend/syntax | class: blocked | Implement Circularresolvedsignature |
| 1166 | Implement Circulartypeargumentslocalandouternocrash | spike | frontend/syntax | class: blocked | Implement Circulartypeargumentslocalandouternocrash |
| 1167 | Implement Circulartypeofwithfunctionmodule | spike | frontend/syntax | class: blocked | Implement Circulartypeofwithfunctionmodule |
| 1168 | Implement Circularlyconstrainedmappedtypecontainingconditionalnoinfiniteinstantiationdepth | spike | frontend/syntax | class: blocked | Implement Circularlyconstrainedmappedtypecontainingconditionalnoinfiniteinstantiationdepth |
| 1169 | Implement Circularlysimplifyingconditionaltypesnocrash | spike | frontend/syntax | class: blocked | Implement Circularlysimplifyingconditionaltypesnocrash |
| 1170 | Implement Class | spike | frontend/syntax | class: triage-needed | Implement Class |
| 1171 | Implement Classaccessorinitializationinferencewithelementaccess | spike | frontend/syntax | class: blocked | Implement Classaccessorinitializationinferencewithelementaccess |
| 1172 | Implement Classattributeinferencetemplate | spike | frontend/syntax | class: blocked | Implement Classattributeinferencetemplate |
| 1173 | Implement Classattributeinferencetemplatejs | spike | frontend/syntax | class: blocked | Implement Classattributeinferencetemplatejs |
| 1174 | Implement Classblockscoping | spike | frontend/syntax | class: triage-needed | Implement Classblockscoping |
| 1175 | Implement Classdeclarationblockscoping | spike | frontend/syntax | class: blocked | Implement Classdeclarationblockscoping |
| 1176 | Implement Classdeclarationcheckusedbeforedefinitioninitself | spike | frontend/syntax | class: blocked | Implement Classdeclarationcheckusedbeforedefinitioninitself |
| 1177 | Implement Classdeclarationmergedinmodulewithcontinuation | spike | frontend/syntax | class: blocked | Implement Classdeclarationmergedinmodulewithcontinuation |
| 1178 | Implement Classdeclarationshouldbeoutofscopeincomputednames | spike | frontend/syntax | class: blocked | Implement Classdeclarationshouldbeoutofscopeincomputednames |
| 1179 | Implement Classdeclaredbeforeclassfactory | spike | frontend/syntax | class: blocked | Implement Classdeclaredbeforeclassfactory |
| 1180 | Implement Classexpressionassignment | spike | frontend/syntax | class: blocked | Implement Classexpressionassignment |
| 1181 | Implement Classexpressionextendingabstractclass | spike | frontend/syntax | class: blocked | Implement Classexpressionextendingabstractclass |
| 1182 | Implement Classexpressioninclassstaticdeclarations | spike | frontend/syntax | class: blocked | Implement Classexpressioninclassstaticdeclarations |
| 1183 | Implement Classexpressionnames | spike | frontend/syntax | class: triage-needed | Implement Classexpressionnames |
| 1184 | Implement Classexpressionpropertymodifiers | spike | frontend/syntax | class: blocked | Implement Classexpressionpropertymodifiers |
| 1185 | Implement Classexpressiontest | spike | frontend/syntax | class: blocked | Implement Classexpressiontest |
| 1186 | Implement Classexpressionwithdecorator | spike | frontend/syntax | class: blocked | Implement Classexpressionwithdecorator |
| 1187 | Implement Classexpressionwithresolutionofnamespaceofsamename | spike | frontend/syntax | class: blocked | Implement Classexpressionwithresolutionofnamespaceofsamename |
| 1188 | Implement Classexpressionwithstaticproperties Parser Syntax | spike | frontend/syntax | class: blocked | Implement Classexpressionwithstaticproperties Parser Syntax |
| 1189 | Implement Classexpressionwithstaticproperties Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Classexpressionwithstaticproperties Unknown Unsupported |
| 1190 | Implement Classexpressionwithstaticpropertieses Parser Syntax | spike | frontend/syntax | class: blocked | Implement Classexpressionwithstaticpropertieses Parser Syntax |
| 1191 | Implement Classexpressionwithstaticpropertieses Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Classexpressionwithstaticpropertieses Unknown Unsupported |
| 1192 | Implement Classexpressions | spike | frontend/syntax | class: blocked | Implement Classexpressions |
| 1193 | Implement Classextendingabstractclasswithmembercalledthesameasitsowntypeparam | spike | frontend/syntax | class: blocked | Implement Classextendingabstractclasswithmembercalledthesameasitsowntypeparam |
| 1194 | Implement Classextendingany | spike | frontend/syntax | class: blocked | Implement Classextendingany |
| 1195 | Implement Classextendingqualifiedname | spike | frontend/syntax | class: blocked | Implement Classextendingqualifiedname |
| 1196 | Implement Classextendsacrossfiles | spike | frontend/syntax | class: blocked | Implement Classextendsacrossfiles |
| 1197 | Implement Classextendsclauseclassmergedwithmodulenotreferingconstructor | spike | frontend/syntax | class: blocked | Implement Classextendsclauseclassmergedwithmodulenotreferingconstructor |
| 1198 | Implement Classextendsclauseclassnotreferringconstructor | spike | frontend/syntax | class: blocked | Implement Classextendsclauseclassnotreferringconstructor |
| 1199 | Implement Classextendsinterface Parser Syntax | spike | frontend/semantics | class: blocked | Implement Classextendsinterface Parser Syntax |
| 1200 | Implement Classextendsinterface Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Classextendsinterface Unknown Unsupported |
| 1201 | Implement Classextendsinterfaceinexpression | spike | frontend/syntax | class: triage-needed | Implement Classextendsinterfaceinexpression |
| 1202 | Implement Classextendsinterfaceinmodule | spike | frontend/syntax | class: blocked | Implement Classextendsinterfaceinmodule |
| 1203 | Implement Classextendsinterfacethatextendsclasswithprivates | spike | frontend/semantics | class: blocked | Implement Classextendsinterfacethatextendsclasswithprivates |
| 1204 | Implement Classextendsmultiplebaseclasses | spike | frontend/semantics | class: blocked | Implement Classextendsmultiplebaseclasses |
| 1205 | Implement Classextendsnull | spike | frontend/syntax | class: triage-needed | Implement Classextendsnull |
| 1206 | Implement Classextensionnameoutput | spike | frontend/syntax | class: blocked | Implement Classextensionnameoutput |
| 1207 | Implement Classfieldsuperaccessible | spike | frontend/syntax | class: blocked | Implement Classfieldsuperaccessible |
| 1208 | Implement Classfieldsuperaccessiblejs | spike | frontend/syntax | class: blocked | Implement Classfieldsuperaccessiblejs |
| 1209 | Implement Classfieldsupernotaccessible | spike | frontend/syntax | class: blocked | Implement Classfieldsupernotaccessible |
| 1210 | Implement Classfieldsupernotaccessiblejs | spike | frontend/syntax | class: blocked | Implement Classfieldsupernotaccessiblejs |
| 1211 | Implement Classfieldsbrokenconstructoremitnocrash | spike | frontend/syntax | class: blocked | Implement Classfieldsbrokenconstructoremitnocrash |
| 1212 | Implement Classfunctionmerging Import Export | spike | frontend/syntax | class: blocked | Implement Classfunctionmerging Import Export |
| 1213 | Implement Classfunctionmerging Parser Syntax | spike | frontend/syntax | class: blocked | Implement Classfunctionmerging Parser Syntax |
| 1214 | Implement Classheritagewithtrailingseparator | spike | frontend/syntax | class: blocked | Implement Classheritagewithtrailingseparator |
| 1215 | Implement Classimplementinginterfaceindexer | spike | frontend/semantics | class: blocked | Implement Classimplementinginterfaceindexer |
| 1216 | Implement Classimplementsclass | spike | frontend/semantics | class: blocked | Implement Classimplementsclass |
| 1217 | Implement Classimplementsimportedinterface | spike | frontend/syntax | class: blocked | Implement Classimplementsimportedinterface |
| 1218 | Implement Classimplementsmethodwithtupleargs | spike | frontend/syntax | class: blocked | Implement Classimplementsmethodwithtupleargs |
| 1219 | Implement Classimplementsprimitive | spike | frontend/semantics | class: blocked | Implement Classimplementsprimitive |
| 1220 | Implement Classinconvertedloopes | spike | frontend/syntax | class: triage-needed | Implement Classinconvertedloopes |
| 1221 | Implement Classindexer | spike | frontend/syntax | class: blocked | Implement Classindexer |
| 1222 | Implement Classmemberinitializerscoping | spike | frontend/syntax | class: blocked | Implement Classmemberinitializerscoping |
| 1223 | Implement Classmemberinitializerwithlamdascoping Import Export | spike | frontend/syntax | class: blocked | Implement Classmemberinitializerwithlamdascoping Import Export |
| 1224 | Implement Classmemberinitializerwithlamdascoping Module System Amd | spike | frontend/syntax | class: blocked | Implement Classmemberinitializerwithlamdascoping Module System Amd |
| 1225 | Implement Classmemberwithmissingidentifier | spike | frontend/syntax | class: blocked | Implement Classmemberwithmissingidentifier |
| 1226 | Implement Classmergedwithinterfacemultiplebasesnoerror | spike | frontend/syntax | class: blocked | Implement Classmergedwithinterfacemultiplebasesnoerror |
| 1227 | Implement Classmethodwithkeywordname | spike | frontend/syntax | class: blocked | Implement Classmethodwithkeywordname |
| 1228 | Implement Classnamereferencesinstaticelements | spike | frontend/syntax | class: blocked | Implement Classnamereferencesinstaticelements |
| 1229 | Implement Classnonuniquesymbolmethodhassymbolindexer | spike | frontend/syntax | class: blocked | Implement Classnonuniquesymbolmethodhassymbolindexer |
| 1230 | Implement Classorder | spike | frontend/syntax | class: blocked | Implement Classorder |
| 1231 | Implement Classorderbug | spike | frontend/syntax | class: blocked | Implement Classorderbug |
| 1232 | Implement Classpropinitializationinferencewithelementaccess | spike | frontend/syntax | class: blocked | Implement Classpropinitializationinferencewithelementaccess |
| 1233 | Implement Classpropertyerroronnameonly | spike | runtime/builtins | class: blocked | Implement Classpropertyerroronnameonly |
| 1234 | Implement Classpropertyinferencefrombroadertypeconst | spike | frontend/syntax | class: blocked | Implement Classpropertyinferencefrombroadertypeconst |
| 1235 | Implement Classreferencedincontextualparameterwithinitsownbaseexpression | spike | frontend/syntax | class: blocked | Implement Classreferencedincontextualparameterwithinitsownbaseexpression |
| 1236 | Implement Classsideinheritance Name Resolution | spike | frontend/resolver | class: blocked | Implement Classsideinheritance Name Resolution |
| 1237 | Implement Classsideinheritance Parser Syntax | spike | frontend/semantics | class: blocked | Implement Classsideinheritance Parser Syntax |
| 1238 | Implement Classstaticinitializersusepropertiesbeforedeclaration | spike | frontend/syntax | class: blocked | Implement Classstaticinitializersusepropertiesbeforedeclaration |
| 1239 | Implement Classstaticpropertyaccess | spike | frontend/syntax | class: blocked | Implement Classstaticpropertyaccess |
| 1240 | Implement Classstaticpropertytypeguard | spike | frontend/semantics | class: blocked | Implement Classstaticpropertytypeguard |
| 1241 | Implement Classtypeparametersinstatics | spike | frontend/syntax | class: blocked | Implement Classtypeparametersinstatics |
| 1242 | Implement Classupdatetests | spike | runtime/builtins | class: blocked | Implement Classupdatetests |
| 1243 | Implement Classusedbeforeinitializedvariables | spike | frontend/syntax | class: blocked | Implement Classusedbeforeinitializedvariables |
| 1244 | Implement Classvariancecircularity | spike | frontend/syntax | class: blocked | Implement Classvariancecircularity |
| 1245 | Implement Classvarianceresolvecircularity | spike | frontend/syntax | class: blocked | Implement Classvarianceresolvecircularity |
| 1246 | Implement Classwithemptytypeparameter | spike | frontend/semantics | class: blocked | Implement Classwithemptytypeparameter |
| 1247 | Implement Classwithmultiplebaseclasses | spike | frontend/semantics | class: blocked | Implement Classwithmultiplebaseclasses |
| 1248 | Implement Classwithoverloadimplementationofwrongname | spike | frontend/semantics | class: blocked | Implement Classwithoverloadimplementationofwrongname |
| 1249 | Implement Classdecl | spike | frontend/syntax | class: blocked | Implement Classdecl |
| 1250 | Implement Clinterfaces | spike | frontend/syntax | class: blocked | Implement Clinterfaces |
| 1251 | Implement Cloduleacrossmoduledefinitions | spike | frontend/syntax | class: blocked | Implement Cloduleacrossmoduledefinitions |
| 1252 | Implement Cloduleandtypeparameters | spike | frontend/semantics | class: blocked | Implement Cloduleandtypeparameters |
| 1253 | Implement Clodulegenericonselfmember | spike | frontend/syntax | class: blocked | Implement Clodulegenericonselfmember |
| 1254 | Implement Clodulesplitacrossfiles | spike | frontend/syntax | class: blocked | Implement Clodulesplitacrossfiles |
| 1255 | Implement Clodulestaticmembers | spike | frontend/syntax | class: blocked | Implement Clodulestaticmembers |
| 1256 | Implement Cloduletest | spike | frontend/syntax | class: blocked | Implement Cloduletest |
| 1257 | Implement Clodulewithduplicatemember | spike | frontend/syntax | class: blocked | Implement Clodulewithduplicatemember |
| 1258 | Implement Clodulewithpriorinstantiatedmodule | spike | frontend/syntax | class: blocked | Implement Clodulewithpriorinstantiatedmodule |
| 1259 | Implement Clodulewithprioruninstantiatedmodule | spike | frontend/syntax | class: blocked | Implement Clodulewithprioruninstantiatedmodule |
| 1260 | Implement Clodulewithrecursivereference | spike | frontend/syntax | class: blocked | Implement Clodulewithrecursivereference |
| 1261 | Implement Clodulesderivedclasses | spike | frontend/syntax | class: blocked | Implement Clodulesderivedclasses |
| 1262 | Implement Coandcontravariantinferences Name Resolution | spike | frontend/resolver | class: blocked | Implement Coandcontravariantinferences Name Resolution |
| 1263 | Implement Coandcontravariantinferences Parser Syntax | spike | frontend/syntax | class: blocked | Implement Coandcontravariantinferences Parser Syntax |
| 1264 | Implement Coandcontravariantinferences Type System | spike | frontend/syntax | class: blocked | Implement Coandcontravariantinferences Type System |
| 1265 | Implement Collectionpatternnoerror | spike | runtime/builtins | class: blocked | Implement Collectionpatternnoerror |
| 1266 | Implement Collisionargumentsarrowfunctions | spike | frontend/syntax | class: blocked | Implement Collisionargumentsarrowfunctions |
| 1267 | Implement Collisionargumentsclassconstructor | spike | frontend/syntax | class: blocked | Implement Collisionargumentsclassconstructor |
| 1268 | Implement Collisionargumentsclassmethod | spike | frontend/syntax | class: blocked | Implement Collisionargumentsclassmethod |
| 1269 | Implement Collisionargumentsfunction | spike | frontend/syntax | class: blocked | Implement Collisionargumentsfunction |
| 1270 | Implement Collisionargumentsfunctionexpressions | spike | frontend/syntax | class: blocked | Implement Collisionargumentsfunctionexpressions |
| 1271 | Implement Collisionargumentsintype | spike | frontend/resolver | class: blocked | Implement Collisionargumentsintype |
| 1272 | Implement Collisioncodegenenumwithenummemberconflict | spike | frontend/resolver | class: blocked | Implement Collisioncodegenenumwithenummemberconflict |
| 1273 | Implement Collisioncodegenmodulewithaccessorchildren | spike | frontend/syntax | class: blocked | Implement Collisioncodegenmodulewithaccessorchildren |
| 1274 | Implement Collisioncodegenmodulewithconstructorchildren | spike | frontend/syntax | class: blocked | Implement Collisioncodegenmodulewithconstructorchildren |
| 1275 | Implement Collisioncodegenmodulewithenummemberconflict | spike | frontend/syntax | class: blocked | Implement Collisioncodegenmodulewithenummemberconflict |
| 1276 | Implement Collisioncodegenmodulewithfunctionchildren | spike | frontend/syntax | class: blocked | Implement Collisioncodegenmodulewithfunctionchildren |
| 1277 | Implement Collisioncodegenmodulewithmemberclassconflict | spike | frontend/syntax | class: blocked | Implement Collisioncodegenmodulewithmemberclassconflict |
| 1278 | Implement Collisioncodegenmodulewithmemberinterfaceconflict | spike | frontend/syntax | class: blocked | Implement Collisioncodegenmodulewithmemberinterfaceconflict |
| 1279 | Implement Collisioncodegenmodulewithmembervariable | spike | frontend/syntax | class: blocked | Implement Collisioncodegenmodulewithmembervariable |
| 1280 | Implement Collisioncodegenmodulewithmethodchildren | spike | frontend/syntax | class: blocked | Implement Collisioncodegenmodulewithmethodchildren |
| 1281 | Implement Collisioncodegenmodulewithmodulechildren | spike | frontend/syntax | class: blocked | Implement Collisioncodegenmodulewithmodulechildren |
| 1282 | Implement Collisioncodegenmodulewithmodulereopening | spike | frontend/syntax | class: blocked | Implement Collisioncodegenmodulewithmodulereopening |
| 1283 | Implement Collisioncodegenmodulewithprivatemember | spike | frontend/syntax | class: blocked | Implement Collisioncodegenmodulewithprivatemember |
| 1284 | Implement Collisionexportsrequireandalias | spike | frontend/syntax | class: blocked | Implement Collisionexportsrequireandalias |
| 1285 | Implement Collisionexportsrequireandambientclass | spike | frontend/syntax | class: blocked | Implement Collisionexportsrequireandambientclass |
| 1286 | Implement Collisionexportsrequireandambientenum | spike | frontend/syntax | class: blocked | Implement Collisionexportsrequireandambientenum |
| 1287 | Implement Collisionexportsrequireandambientfunction | spike | frontend/syntax | class: blocked | Implement Collisionexportsrequireandambientfunction |
| 1288 | Implement Collisionexportsrequireandambientfunctioninglobalfile | spike | frontend/syntax | class: blocked | Implement Collisionexportsrequireandambientfunctioninglobalfile |
| 1289 | Implement Collisionexportsrequireandambientmodule | spike | frontend/syntax | class: blocked | Implement Collisionexportsrequireandambientmodule |
| 1290 | Implement Collisionexportsrequireandambientvar | spike | frontend/syntax | class: blocked | Implement Collisionexportsrequireandambientvar |
| 1291 | Implement Collisionexportsrequireandclass | spike | frontend/syntax | class: blocked | Implement Collisionexportsrequireandclass |
| 1292 | Implement Collisionexportsrequireandenum | spike | frontend/syntax | class: blocked | Implement Collisionexportsrequireandenum |
| 1293 | Implement Collisionexportsrequireandfunction | spike | frontend/syntax | class: blocked | Implement Collisionexportsrequireandfunction |
| 1294 | Implement Collisionexportsrequireandfunctioninglobalfile | spike | frontend/syntax | class: blocked | Implement Collisionexportsrequireandfunctioninglobalfile |
| 1295 | Implement Collisionexportsrequireandinternalmodulealias | spike | frontend/syntax | class: blocked | Implement Collisionexportsrequireandinternalmodulealias |
| 1296 | Implement Collisionexportsrequireandinternalmodulealiasinglobalfile | spike | frontend/syntax | class: blocked | Implement Collisionexportsrequireandinternalmodulealiasinglobalfile |
| 1297 | Implement Collisionexportsrequireandmodule | spike | frontend/syntax | class: blocked | Implement Collisionexportsrequireandmodule |
| 1298 | Implement Collisionexportsrequireanduninstantiatedmodule | spike | frontend/syntax | class: blocked | Implement Collisionexportsrequireanduninstantiatedmodule |
| 1299 | Implement Collisionexportsrequireandvar | spike | frontend/syntax | class: blocked | Implement Collisionexportsrequireandvar |
| 1300 | Implement Collisionrestparameterarrowfunctions | spike | frontend/syntax | class: blocked | Implement Collisionrestparameterarrowfunctions |
| 1301 | Implement Collisionrestparameterclassconstructor | spike | frontend/semantics | class: blocked | Implement Collisionrestparameterclassconstructor |
| 1302 | Implement Collisionrestparameterclassmethod | spike | frontend/semantics | class: blocked | Implement Collisionrestparameterclassmethod |
| 1303 | Implement Collisionrestparameterfunction | spike | frontend/semantics | class: blocked | Implement Collisionrestparameterfunction |
| 1304 | Implement Collisionrestparameterfunctionexpressions | spike | frontend/semantics | class: blocked | Implement Collisionrestparameterfunctionexpressions |
| 1305 | Implement Collisionrestparameterintype | spike | frontend/semantics | class: blocked | Implement Collisionrestparameterintype |
| 1306 | Implement Collisionrestparameterunderscoreiusage | spike | frontend/syntax | class: blocked | Implement Collisionrestparameterunderscoreiusage |
| 1307 | Implement Collisionsuperandlocalfunctioninaccessors | spike | frontend/syntax | class: blocked | Implement Collisionsuperandlocalfunctioninaccessors |
| 1308 | Implement Collisionsuperandlocalfunctioninconstructor | spike | frontend/syntax | class: blocked | Implement Collisionsuperandlocalfunctioninconstructor |
| 1309 | Implement Collisionsuperandlocalfunctioninmethod | spike | frontend/resolver | class: blocked | Implement Collisionsuperandlocalfunctioninmethod |
| 1310 | Implement Collisionsuperandlocalfunctioninproperty | spike | frontend/resolver | class: blocked | Implement Collisionsuperandlocalfunctioninproperty |
| 1311 | Implement Collisionsuperandlocalvarinaccessors | spike | frontend/syntax | class: blocked | Implement Collisionsuperandlocalvarinaccessors |
| 1312 | Implement Collisionsuperandlocalvarinconstructor | spike | frontend/syntax | class: blocked | Implement Collisionsuperandlocalvarinconstructor |
| 1313 | Implement Collisionsuperandlocalvarinmethod | spike | frontend/resolver | class: blocked | Implement Collisionsuperandlocalvarinmethod |
| 1314 | Implement Collisionsuperandlocalvarinproperty | spike | frontend/resolver | class: blocked | Implement Collisionsuperandlocalvarinproperty |
| 1315 | Implement Collisionsuperandnameresolution | spike | frontend/resolver | class: blocked | Implement Collisionsuperandnameresolution |
| 1316 | Implement Collisionsuperandparameter | spike | frontend/resolver | class: blocked | Implement Collisionsuperandparameter |
| 1317 | Implement Collisionsuperandpropertynameasconstuctorparameter | spike | frontend/syntax | class: blocked | Implement Collisionsuperandpropertynameasconstuctorparameter |
| 1318 | Implement Collisionthisexpressionandaliasinglobal | spike | frontend/syntax | class: blocked | Implement Collisionthisexpressionandaliasinglobal |
| 1319 | Implement Collisionthisexpressionandambientclassinglobal | spike | frontend/resolver | class: blocked | Implement Collisionthisexpressionandambientclassinglobal |
| 1320 | Implement Collisionthisexpressionandambientvaringlobal | spike | frontend/resolver | class: blocked | Implement Collisionthisexpressionandambientvaringlobal |
| 1321 | Implement Collisionthisexpressionandclassinglobal | spike | frontend/syntax | class: blocked | Implement Collisionthisexpressionandclassinglobal |
| 1322 | Implement Collisionthisexpressionandenuminglobal | spike | frontend/resolver | class: blocked | Implement Collisionthisexpressionandenuminglobal |
| 1323 | Implement Collisionthisexpressionandfunctioninglobal | spike | frontend/syntax | class: blocked | Implement Collisionthisexpressionandfunctioninglobal |
| 1324 | Implement Collisionthisexpressionandlocalvarinaccessors | spike | frontend/syntax | class: blocked | Implement Collisionthisexpressionandlocalvarinaccessors |
| 1325 | Implement Collisionthisexpressionandlocalvarinconstructor | spike | frontend/syntax | class: triage-needed | Implement Collisionthisexpressionandlocalvarinconstructor |
| 1326 | Implement Collisionthisexpressionandlocalvarinfunction | spike | frontend/resolver | class: blocked | Implement Collisionthisexpressionandlocalvarinfunction |
| 1327 | Implement Collisionthisexpressionandlocalvarinlambda | spike | frontend/syntax | class: triage-needed | Implement Collisionthisexpressionandlocalvarinlambda |
| 1328 | Implement Collisionthisexpressionandlocalvarinmethod | spike | frontend/syntax | class: triage-needed | Implement Collisionthisexpressionandlocalvarinmethod |
| 1329 | Implement Collisionthisexpressionandlocalvarinproperty | spike | frontend/resolver | class: blocked | Implement Collisionthisexpressionandlocalvarinproperty |
| 1330 | Implement Collisionthisexpressionandlocalvarwithsuperexperssion | spike | frontend/resolver | class: blocked | Implement Collisionthisexpressionandlocalvarwithsuperexperssion |
| 1331 | Implement Collisionthisexpressionandmoduleinglobal | spike | frontend/syntax | class: blocked | Implement Collisionthisexpressionandmoduleinglobal |
| 1332 | Implement Collisionthisexpressionandnameresolution | spike | frontend/resolver | class: blocked | Implement Collisionthisexpressionandnameresolution |
| 1333 | Implement Collisionthisexpressionandparameter | spike | frontend/resolver | class: blocked | Implement Collisionthisexpressionandparameter |
| 1334 | Implement Collisionthisexpressionandpropertynameasconstuctorparameter | spike | frontend/syntax | class: blocked | Implement Collisionthisexpressionandpropertynameasconstuctorparameter |
| 1335 | Implement Collisionthisexpressionandvaringlobal | spike | frontend/syntax | class: blocked | Implement Collisionthisexpressionandvaringlobal |
| 1336 | Implement Commaoperator | spike | frontend/syntax | class: blocked | Implement Commaoperator |
| 1337 | Implement Commaoperatorinconditionalexpression | spike | frontend/syntax | class: blocked | Implement Commaoperatorinconditionalexpression |
| 1338 | Implement Commaoperatorleftsideunused | spike | frontend/resolver | class: blocked | Implement Commaoperatorleftsideunused |
| 1339 | Implement Commentbeforestaticmethod | spike | frontend/syntax | class: blocked | Implement Commentbeforestaticmethod |
| 1340 | Implement Commentemitatendoffile | spike | frontend/syntax | class: blocked | Implement Commentemitatendoffile |
| 1341 | Implement Commentemitonparenthesizedassertioninreturnstatement | spike | frontend/syntax | class: blocked | Implement Commentemitonparenthesizedassertioninreturnstatement |
| 1342 | Implement Commentinmethodcall | spike | frontend/syntax | class: blocked | Implement Commentinmethodcall |
| 1343 | Implement Commentinnamespacedeclarationwithidentifierpathname | spike | frontend/syntax | class: blocked | Implement Commentinnamespacedeclarationwithidentifierpathname |
| 1344 | Implement Commentleadingclosebrace | spike | frontend/resolver | class: blocked | Implement Commentleadingclosebrace |
| 1345 | Implement Commentonambientmodule | spike | frontend/syntax | class: blocked | Implement Commentonambientmodule |
| 1346 | Implement Commentonambientvariable | spike | frontend/resolver | class: blocked | Implement Commentonambientvariable |
| 1347 | Implement Commentonclassaccessor | spike | reference/triage | class: triage-needed | Implement Commentonclassaccessor |
| 1348 | Implement Commentondecoratedclassdeclaration | spike | frontend/syntax | class: blocked | Implement Commentondecoratedclassdeclaration |
| 1349 | Implement Commentonelidedmodule | spike | frontend/syntax | class: blocked | Implement Commentonelidedmodule |
| 1350 | Implement Commentonexportenumdeclaration | spike | frontend/syntax | class: blocked | Implement Commentonexportenumdeclaration |
| 1351 | Implement Commentonimportstatement | spike | frontend/syntax | class: blocked | Implement Commentonimportstatement |
| 1352 | Implement Commentonparameter | spike | frontend/syntax | class: blocked | Implement Commentonparameter |
| 1353 | Implement Commentonparenthesizedexpressionopenparen | spike | frontend/syntax | class: triage-needed | Implement Commentonparenthesizedexpressionopenparen |
| 1354 | Implement Commentonsignature | spike | frontend/syntax | class: blocked | Implement Commentonsignature |
| 1355 | Implement Commentwithunreasonableindentationlevel | spike | frontend/syntax | class: blocked | Implement Commentwithunreasonableindentationlevel |
| 1356 | Implement Commentsafterfunctionexpression | spike | frontend/syntax | class: blocked | Implement Commentsafterfunctionexpression |
| 1357 | Implement Commentsafterspread | spike | frontend/semantics | class: blocked | Implement Commentsafterspread |
| 1358 | Implement Commentsatendoffile | spike | frontend/syntax | class: triage-needed | Implement Commentsatendoffile |
| 1359 | Implement Commentsbeforefunctionexpression | spike | frontend/syntax | class: blocked | Implement Commentsbeforefunctionexpression |
| 1360 | Implement Commentsbeforevariablestatement | spike | frontend/syntax | class: blocked | Implement Commentsbeforevariablestatement |
| 1361 | Implement Commentsclass | spike | frontend/resolver | class: blocked | Implement Commentsclass |
| 1362 | Implement Commentsclassmembers | spike | frontend/syntax | class: blocked | Implement Commentsclassmembers |
| 1363 | Implement Commentscommentparsing | spike | frontend/syntax | class: blocked | Implement Commentscommentparsing |
| 1364 | Implement Commentsdottedmodulename | spike | frontend/syntax | class: blocked | Implement Commentsdottedmodulename |
| 1365 | Implement Commentsenums | spike | frontend/syntax | class: blocked | Implement Commentsenums |
| 1366 | Implement Commentsexternalmodules | spike | frontend/syntax | class: blocked | Implement Commentsexternalmodules |
| 1367 | Implement Commentsformatting | spike | frontend/syntax | class: blocked | Implement Commentsformatting |
| 1368 | Implement Commentsfunction | spike | frontend/syntax | class: blocked | Implement Commentsfunction |
| 1369 | Implement Commentsinheritance | spike | frontend/syntax | class: blocked | Implement Commentsinheritance |
| 1370 | Implement Commentsinterface | spike | frontend/syntax | class: blocked | Implement Commentsinterface |
| 1371 | Implement Commentsmodules | spike | frontend/syntax | class: blocked | Implement Commentsmodules |
| 1372 | Implement Commentsmultimodulemultifile | spike | frontend/syntax | class: blocked | Implement Commentsmultimodulemultifile |
| 1373 | Implement Commentsmultimodulesinglefile | spike | frontend/syntax | class: blocked | Implement Commentsmultimodulesinglefile |
| 1374 | Implement Commentsonobjectliteral Name Resolution | spike | frontend/resolver | class: blocked | Implement Commentsonobjectliteral Name Resolution |
| 1375 | Implement Commentsonobjectliteral Object Literal | spike | frontend/syntax | class: blocked | Implement Commentsonobjectliteral Object Literal |
| 1376 | Implement Commentsonrequirestatement | spike | frontend/syntax | class: blocked | Implement Commentsonrequirestatement |
| 1377 | Implement Commentsonreturnstatement | spike | frontend/syntax | class: blocked | Implement Commentsonreturnstatement |
| 1378 | Implement Commentsonstaticmembers | spike | frontend/syntax | class: blocked | Implement Commentsonstaticmembers |
| 1379 | Implement Commentsoverloads | spike | frontend/semantics | class: blocked | Implement Commentsoverloads |
| 1380 | Implement Commentstypeparameters | spike | frontend/semantics | class: blocked | Implement Commentstypeparameters |
| 1381 | Implement Commentsdonotemitcomments | spike | frontend/syntax | class: blocked | Implement Commentsdonotemitcomments |
| 1382 | Implement Commentsemitcomments | spike | frontend/syntax | class: blocked | Implement Commentsemitcomments |
| 1383 | Implement Commonjsexporttypedeclarationerror | spike | frontend/syntax | class: blocked | Implement Commonjsexporttypedeclarationerror |
| 1384 | Implement Commonjsimportclassexpression | spike | frontend/syntax | class: blocked | Implement Commonjsimportclassexpression |
| 1385 | Implement Commonjsisolatedmodules | spike | frontend/syntax | class: blocked | Implement Commonjsisolatedmodules |
| 1386 | Implement Commonmissingsemicolons | spike | reference/triage | class: triage-needed | Implement Commonmissingsemicolons |
| 1387 | Implement Commonsourcedir | spike | frontend/syntax | class: blocked | Implement Commonsourcedir |
| 1388 | Implement Commonsourcedirectory | spike | frontend/syntax | class: blocked | Implement Commonsourcedirectory |
| 1389 | Implement Commonjsaccessexports | spike | frontend/syntax | class: blocked | Implement Commonjsaccessexports |
| 1390 | Implement Commonjssafeimport | spike | frontend/syntax | class: blocked | Implement Commonjssafeimport |
| 1391 | Implement Comparabilitytypeparametersrelatedbyunion | spike | frontend/semantics | class: blocked | Implement Comparabilitytypeparametersrelatedbyunion |
| 1392 | Implement Comparablerelationbidirectional | spike | frontend/syntax | class: blocked | Implement Comparablerelationbidirectional |
| 1393 | Implement Comparisonofpartialdeepandindexedaccessterminateswithouterror | spike | frontend/syntax | class: triage-needed | Implement Comparisonofpartialdeepandindexedaccessterminateswithouterror |
| 1394 | Implement Complexclassrelationships | spike | frontend/syntax | class: blocked | Implement Complexclassrelationships |
| 1395 | Implement Complexnarrowingwithany | spike | frontend/syntax | class: blocked | Implement Complexnarrowingwithany |
| 1396 | Implement Complexrecursivecollections | spike | frontend/syntax | class: blocked | Implement Complexrecursivecollections |
| 1397 | Implement Complicatedgenericrecursivebaseclassreference | spike | frontend/syntax | class: blocked | Implement Complicatedgenericrecursivebaseclassreference |
| 1398 | Implement Complicatedindexedaccesskeyofreliesonkeyofneverupperbound | spike | frontend/syntax | class: blocked | Implement Complicatedindexedaccesskeyofreliesonkeyofneverupperbound |
| 1399 | Implement Complicatedindexesofintersectionsareinferencable | spike | frontend/syntax | class: blocked | Implement Complicatedindexesofintersectionsareinferencable |
| 1400 | Implement Complicatedprivacy | spike | frontend/syntax | class: blocked | Implement Complicatedprivacy |
| 1401 | Implement Compositecontextualsignature | spike | frontend/syntax | class: blocked | Implement Compositecontextualsignature |
| 1402 | Implement Compositegenericfunction | spike | reference/triage | class: triage-needed | Implement Compositegenericfunction |
| 1403 | Implement Compositewithnodemodulessourcefile | spike | frontend/syntax | class: blocked | Implement Compositewithnodemodulessourcefile |
| 1404 | Implement Compoundvardecl | spike | frontend/syntax | class: blocked | Implement Compoundvardecl |
| 1405 | Implement Computedenummembersyntacticallystring Enum | spike | frontend/syntax | class: blocked | Implement Computedenummembersyntacticallystring Enum |
| 1406 | Implement Computedenummembersyntacticallystring Parser Syntax | spike | frontend/syntax | class: blocked | Implement Computedenummembersyntacticallystring Parser Syntax |
| 1407 | Implement Computedenumtypewidening | spike | frontend/syntax | class: blocked | Implement Computedenumtypewidening |
| 1408 | Implement Computedpropertiesindestructuring | spike | frontend/syntax | class: blocked | Implement Computedpropertiesindestructuring |
| 1409 | Implement Computedpropertiesnarrowed | spike | frontend/syntax | class: blocked | Implement Computedpropertiesnarrowed |
| 1410 | Implement Computedpropertiestransformedinotherwisenontsclasses | spike | frontend/syntax | class: blocked | Implement Computedpropertiestransformedinotherwisenontsclasses |
| 1411 | Implement Computedpropertieswithsetterassignment | spike | frontend/syntax | class: triage-needed | Implement Computedpropertieswithsetterassignment |
| 1412 | Implement Computedpropertybindingelementdeclarationnocrash | spike | frontend/syntax | class: blocked | Implement Computedpropertybindingelementdeclarationnocrash |
| 1413 | Implement Computedpropertynameandtypeparameterconflict | spike | frontend/semantics | class: blocked | Implement Computedpropertynameandtypeparameterconflict |
| 1414 | Implement Computedpropertynamewithimportedkey | spike | frontend/syntax | class: blocked | Implement Computedpropertynamewithimportedkey |
| 1415 | Implement Computerpropertiesines | spike | frontend/syntax | class: blocked | Implement Computerpropertiesines |
| 1416 | Implement Concatclassandstring | spike | frontend/resolver | class: blocked | Implement Concatclassandstring |
| 1417 | Implement Conditionalequalityonliteralobjects | spike | frontend/syntax | class: blocked | Implement Conditionalequalityonliteralobjects |
| 1418 | Implement Conditionalexpression | spike | frontend/syntax | class: blocked | Implement Conditionalexpression |
| 1419 | Implement Conditionalexpressionnewline | spike | frontend/resolver | class: blocked | Implement Conditionalexpressionnewline |
| 1420 | Implement Conditionalexpressions | spike | frontend/syntax | class: blocked | Implement Conditionalexpressions |
| 1421 | Implement Conditionalreturnexpression | spike | frontend/resolver | class: blocked | Implement Conditionalreturnexpression |
| 1422 | Implement Conditionaltypeassignabilitywhendeferred | spike | frontend/syntax | class: blocked | Implement Conditionaltypeassignabilitywhendeferred |
| 1423 | Implement Conditionaltypebasedcontextualtypereturntypewidening | spike | frontend/syntax | class: blocked | Implement Conditionaltypebasedcontextualtypereturntypewidening |
| 1424 | Implement Conditionaltypeclassmembers | spike | frontend/syntax | class: blocked | Implement Conditionaltypeclassmembers |
| 1425 | Implement Conditionaltypediscriminatinglargeunionregulartypefetchingspeedreasonable | spike | frontend/syntax | class: blocked | Implement Conditionaltypediscriminatinglargeunionregulartypefetchingspeedreasonable |
| 1426 | Implement Conditionaltypedoesntspinforever | spike | frontend/syntax | class: blocked | Implement Conditionaltypedoesntspinforever |
| 1427 | Implement Conditionaltyperelaxingconstraintassignability | spike | frontend/syntax | class: blocked | Implement Conditionaltyperelaxingconstraintassignability |
| 1428 | Implement Conditionaltypesubclassextendstypeparam | spike | frontend/syntax | class: blocked | Implement Conditionaltypesubclassextendstypeparam |
| 1429 | Implement Conditionaltypessimplifywhentrivial | spike | frontend/syntax | class: blocked | Implement Conditionaltypessimplifywhentrivial |
| 1430 | Implement Conditionallyduplicateoverloadscausedbyoverloadresolution | spike | frontend/syntax | class: blocked | Implement Conditionallyduplicateoverloadscausedbyoverloadresolution |
| 1431 | Implement Conflictmarkerdiff Parser Syntax | spike | frontend/resolver | class: blocked | Implement Conflictmarkerdiff Parser Syntax |
| 1432 | Implement Conflictmarkerdiff Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Conflictmarkerdiff Unknown Unsupported |
| 1433 | Implement Conflictmarkertrivia Parser Syntax | spike | frontend/resolver | class: blocked | Implement Conflictmarkertrivia Parser Syntax |
| 1434 | Implement Conflictmarkertrivia Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Conflictmarkertrivia Unknown Unsupported |
| 1435 | Implement Conflictingdeclarationsimportfromnamespace | spike | frontend/syntax | class: blocked | Implement Conflictingdeclarationsimportfromnamespace |
| 1436 | Implement Conflictingtypeannotatedvar | spike | reference/triage | class: triage-needed | Implement Conflictingtypeannotatedvar |
| 1437 | Implement Conflictingtypeparametersymboltransfer | spike | frontend/semantics | class: blocked | Implement Conflictingtypeparametersymboltransfer |
| 1438 | Implement Consistentaliasvsnonaliasrecordbehavior | spike | frontend/syntax | class: blocked | Implement Consistentaliasvsnonaliasrecordbehavior |
| 1439 | Implement Constdeclarationshadowedbyvardeclaration | spike | frontend/resolver | class: blocked | Implement Constdeclarationshadowedbyvardeclaration |
| 1440 | Implement Constdeclarations Import Export | spike | frontend/syntax | class: blocked | Implement Constdeclarations Import Export |
| 1441 | Implement Constdeclarations Name Resolution | spike | frontend/resolver | class: blocked | Implement Constdeclarations Name Resolution |
| 1442 | Implement Constdeclarations Parser Syntax | spike | frontend/syntax | class: blocked | Implement Constdeclarations Parser Syntax |
| 1443 | Implement Constdeclarations Scope Analysis | spike | frontend/syntax | class: blocked | Implement Constdeclarations Scope Analysis |
| 1444 | Implement Constdeclarations Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Constdeclarations Unknown Unsupported |
| 1445 | Implement Constenumbadpropertynames | spike | frontend/syntax | class: blocked | Implement Constenumbadpropertynames |
| 1446 | Implement Constenumdeclarations | spike | frontend/syntax | class: blocked | Implement Constenumdeclarations |
| 1447 | Implement Constenumerrors | spike | frontend/syntax | class: blocked | Implement Constenumerrors |
| 1448 | Implement Constenumexternalmodule | spike | frontend/syntax | class: blocked | Implement Constenumexternalmodule |
| 1449 | Implement Constenummergingwithvalues Import Export | spike | frontend/syntax | class: blocked | Implement Constenummergingwithvalues Import Export |
| 1450 | Implement Constenummergingwithvalues Parser Syntax | spike | frontend/syntax | class: blocked | Implement Constenummergingwithvalues Parser Syntax |
| 1451 | Implement Constenumnamespacereferencecausesnoimport | spike | frontend/syntax | class: blocked | Implement Constenumnamespacereferencecausesnoimport |
| 1452 | Implement Constenumnoemitreexport | spike | frontend/syntax | class: blocked | Implement Constenumnoemitreexport |
| 1453 | Implement Constenumnopreservedeclarationreexport | spike | frontend/syntax | class: blocked | Implement Constenumnopreservedeclarationreexport |
| 1454 | Implement Constenumonlymodulemerging | spike | frontend/syntax | class: blocked | Implement Constenumonlymodulemerging |
| 1455 | Implement Constenumpreserveemitnamedexport | spike | frontend/syntax | class: blocked | Implement Constenumpreserveemitnamedexport |
| 1456 | Implement Constenumpreserveemitreexport | spike | frontend/syntax | class: blocked | Implement Constenumpreserveemitreexport |
| 1457 | Implement Constenumsyntheticnodescomments | spike | frontend/syntax | class: blocked | Implement Constenumsyntheticnodescomments |
| 1458 | Implement Constenumtostringnocomments | spike | frontend/syntax | class: blocked | Implement Constenumtostringnocomments |
| 1459 | Implement Constenumtostringwithcomments | spike | frontend/syntax | class: blocked | Implement Constenumtostringwithcomments |
| 1460 | Implement Constenums | spike | frontend/syntax | class: blocked | Implement Constenums |
| 1461 | Implement Constinclassexpression | spike | frontend/syntax | class: blocked | Implement Constinclassexpression |
| 1462 | Implement Constindexedaccess | spike | frontend/syntax | class: triage-needed | Implement Constindexedaccess |
| 1463 | Implement Constwithnonnull | spike | frontend/semantics | class: blocked | Implement Constwithnonnull |
| 1464 | Implement Constantenumassert | spike | frontend/syntax | class: blocked | Implement Constantenumassert |
| 1465 | Implement Constraintcheckingenericbasetypereference | spike | frontend/syntax | class: blocked | Implement Constraintcheckingenericbasetypereference |
| 1466 | Implement Constraints | spike | frontend/resolver | class: blocked | Implement Constraints |
| 1467 | Implement Constraintsthatreferenceothercontstraints | spike | frontend/semantics | class: blocked | Implement Constraintsthatreferenceothercontstraints |
| 1468 | Implement Constraintsusedinprototypeproperty | spike | frontend/semantics | class: blocked | Implement Constraintsusedinprototypeproperty |
| 1469 | Implement Constructorargwithgenericcallsignature | spike | frontend/syntax | class: blocked | Implement Constructorargwithgenericcallsignature |
| 1470 | Implement Constructorargserrors | spike | frontend/syntax | class: blocked | Implement Constructorargserrors |
| 1471 | Implement Constructorastype | spike | frontend/resolver | class: blocked | Implement Constructorastype |
| 1472 | Implement Constructorinvocationwithtoofewtypeargs | spike | frontend/syntax | class: blocked | Implement Constructorinvocationwithtoofewtypeargs |
| 1473 | Implement Constructoroverloads Import Export | spike | frontend/syntax | class: blocked | Implement Constructoroverloads Import Export |
| 1474 | Implement Constructoroverloads Name Resolution | spike | frontend/resolver | class: blocked | Implement Constructoroverloads Name Resolution |
| 1475 | Implement Constructoroverloads Parser Syntax | spike | frontend/semantics | class: blocked | Implement Constructoroverloads Parser Syntax |
| 1476 | Implement Constructorparametersinvariabledeclarations | spike | frontend/semantics | class: blocked | Implement Constructorparametersinvariabledeclarations |
| 1477 | Implement Constructorparametersthatshadowexternalnamesinvariabledeclarations | spike | frontend/semantics | class: blocked | Implement Constructorparametersthatshadowexternalnamesinvariabledeclarations |
| 1478 | Implement Constructorreturningaprimitive | spike | frontend/semantics | class: blocked | Implement Constructorreturningaprimitive |
| 1479 | Implement Constructorstaticparamname | spike | frontend/semantics | class: blocked | Implement Constructorstaticparamname |
| 1480 | Implement Constructorstaticparamnameerrors | spike | frontend/semantics | class: blocked | Implement Constructorstaticparamnameerrors |
| 1481 | Implement Constructorwithcapturedsuper | spike | frontend/syntax | class: blocked | Implement Constructorwithcapturedsuper |
| 1482 | Implement Constructorwithincompletetypeannotation | spike | frontend/syntax | class: blocked | Implement Constructorwithincompletetypeannotation |
| 1483 | Implement Constructorwithparameterpropertiesandprivatefields | spike | frontend/semantics | class: blocked | Implement Constructorwithparameterpropertiesandprivatefields |
| 1484 | Implement Constructorwithsuperandprologue | spike | frontend/syntax | class: blocked | Implement Constructorwithsuperandprologue |
| 1485 | Implement Constructorswithspecializedsignatures | spike | frontend/syntax | class: blocked | Implement Constructorswithspecializedsignatures |
| 1486 | Implement Contextsensitivereturntypeinference | spike | frontend/resolver | class: blocked | Implement Contextsensitivereturntypeinference |
| 1487 | Implement Contextualcomputednonbindablepropertytype | spike | frontend/syntax | class: blocked | Implement Contextualcomputednonbindablepropertytype |
| 1488 | Implement Contextualexpressiontypecheckingdoesntblowstack | spike | frontend/syntax | class: blocked | Implement Contextualexpressiontypecheckingdoesntblowstack |
| 1489 | Implement Contextualoutertypeparameters | spike | frontend/semantics | class: blocked | Implement Contextualoutertypeparameters |
| 1490 | Implement Contextualoverloadlistfromarrayunion | spike | frontend/syntax | class: blocked | Implement Contextualoverloadlistfromarrayunion |
| 1491 | Implement Contextualparamtypevsnestedreturntypeinference | spike | frontend/syntax | class: blocked | Implement Contextualparamtypevsnestedreturntypeinference |
| 1492 | Implement Contextualparameterandselfreferentialconstraint | spike | frontend/syntax | class: triage-needed | Implement Contextualparameterandselfreferentialconstraint |
| 1493 | Implement Contextualpropertyofgenericfilteringmappedtype | spike | frontend/syntax | class: blocked | Implement Contextualpropertyofgenericfilteringmappedtype |
| 1494 | Implement Contextualpropertyofgenericmappedtype | spike | frontend/resolver | class: blocked | Implement Contextualpropertyofgenericmappedtype |
| 1495 | Implement Contextualreturntypeofiife Import Export | spike | frontend/syntax | class: blocked | Implement Contextualreturntypeofiife Import Export |
| 1496 | Implement Contextualreturntypeofiife Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Contextualreturntypeofiife Unknown Unsupported |
| 1497 | Implement Contextualsignatureconditionaltypeinstantiationusingdefault | spike | frontend/syntax | class: blocked | Implement Contextualsignatureconditionaltypeinstantiationusingdefault |
| 1498 | Implement Contextualsignatureinarrayelementlibes | spike | frontend/syntax | class: blocked | Implement Contextualsignatureinarrayelementlibes |
| 1499 | Implement Contextualsignatureinobjectfreeze | spike | frontend/resolver | class: blocked | Implement Contextualsignatureinobjectfreeze |
| 1500 | Implement Contextualsignatureinstantiation Duplicate Local | spike | reference/triage | class: triage-needed | Implement Contextualsignatureinstantiation Duplicate Local |
| 1501 | Implement Contextualsignatureinstantiation Parser Syntax | spike | frontend/syntax | class: blocked | Implement Contextualsignatureinstantiation Parser Syntax |
| 1502 | Implement Contextualsignatureinstantiation Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Contextualsignatureinstantiation Unknown Unsupported |
| 1503 | Implement Contextualsignatureinstantiationwithtypeparameterconstrainedtooutertypeparameter | spike | frontend/semantics | class: blocked | Implement Contextualsignatureinstantiationwithtypeparameterconstrainedtooutertypeparameter |
| 1504 | Implement Contextualsignatureinstatiationcontravariance | spike | frontend/resolver | class: blocked | Implement Contextualsignatureinstatiationcontravariance |
| 1505 | Implement Contextualtupletypeparameterreadonly | spike | frontend/semantics | class: blocked | Implement Contextualtupletypeparameterreadonly |
| 1506 | Implement Contextualtypearrayreturntype | spike | frontend/semantics | class: blocked | Implement Contextualtypearrayreturntype |
| 1507 | Implement Contextualtypebasedonintersectionwithanyinthemix Name Resolution | spike | frontend/resolver | class: blocked | Implement Contextualtypebasedonintersectionwithanyinthemix Name Resolution |
| 1508 | Implement Contextualtypebasedonintersectionwithanyinthemix Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Contextualtypebasedonintersectionwithanyinthemix Unknown Unsupported |
| 1509 | Implement Contextualtypecaching | spike | frontend/semantics | class: blocked | Implement Contextualtypecaching |
| 1510 | Implement Contextualtypeforinitalizedvariablesfiltersundefined | spike | reference/triage | class: triage-needed | Implement Contextualtypeforinitalizedvariablesfiltersundefined |
| 1511 | Implement Contextualtypefunctionobjectpropertyintersection | spike | frontend/syntax | class: blocked | Implement Contextualtypefunctionobjectpropertyintersection |
| 1512 | Implement Contextualtypeiterableunions | spike | frontend/semantics | class: blocked | Implement Contextualtypeiterableunions |
| 1513 | Implement Contextualtypeofindexedaccessparameter | spike | frontend/resolver | class: blocked | Implement Contextualtypeofindexedaccessparameter |
| 1514 | Implement Contextualtypeonyield | spike | frontend/semantics | class: blocked | Implement Contextualtypeonyield |
| 1515 | Implement Contextualtypeselfreferencing | spike | frontend/resolver | class: blocked | Implement Contextualtypeselfreferencing |
| 1516 | Implement Contextualtypeshouldbeliteral | spike | reference/triage | class: triage-needed | Implement Contextualtypeshouldbeliteral |
| 1517 | Implement Contextualtypesnegatedtypelikeconstraintingenericmappedtype | spike | frontend/semantics | class: blocked | Implement Contextualtypesnegatedtypelikeconstraintingenericmappedtype |
| 1518 | Implement Contextualtyping Import Export | spike | frontend/syntax | class: blocked | Implement Contextualtyping Import Export |
| 1519 | Implement Contextualtyping Parser Syntax | spike | frontend/syntax | class: blocked | Implement Contextualtyping Parser Syntax |
| 1520 | Implement Contextualtyping Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Contextualtyping Unknown Unsupported |
| 1521 | Implement Contextualtypingarraydestructuringwithdefaults | spike | reference/triage | class: triage-needed | Implement Contextualtypingarraydestructuringwithdefaults |
| 1522 | Implement Contextualtypingfunctionreturningfunction | spike | frontend/syntax | class: blocked | Implement Contextualtypingfunctionreturningfunction |
| 1523 | Implement Contextualtypingofaccessors | spike | frontend/syntax | class: blocked | Implement Contextualtypingofaccessors |
| 1524 | Implement Contextualtypingofarrayliterals | spike | frontend/syntax | class: blocked | Implement Contextualtypingofarrayliterals |
| 1525 | Implement Contextualtypingofconditionalexpression | spike | frontend/syntax | class: blocked | Implement Contextualtypingofconditionalexpression |
| 1526 | Implement Contextualtypingofgenericfunctiontypedarguments | spike | frontend/syntax | class: blocked | Implement Contextualtypingofgenericfunctiontypedarguments |
| 1527 | Implement Contextualtypingoflambdareturnexpression | spike | frontend/syntax | class: blocked | Implement Contextualtypingoflambdareturnexpression |
| 1528 | Implement Contextualtypingoflambdawithmultiplesignatures | spike | frontend/syntax | class: blocked | Implement Contextualtypingoflambdawithmultiplesignatures |
| 1529 | Implement Contextualtypingoftooshortoverloads | spike | frontend/syntax | class: blocked | Implement Contextualtypingoftooshortoverloads |
| 1530 | Implement Contextualtypingreturnstatementwithreturntypeannotation | spike | frontend/resolver | class: blocked | Implement Contextualtypingreturnstatementwithreturntypeannotation |
| 1531 | Implement Contextualtypingtwoinstancesofsametypeparameter | spike | frontend/semantics | class: blocked | Implement Contextualtypingtwoinstancesofsametypeparameter |
| 1532 | Implement Contextualtypingwithfixedtypeparameters | spike | frontend/syntax | class: blocked | Implement Contextualtypingwithfixedtypeparameters |
| 1533 | Implement Contextualtypingwithgenericandnongenericsignature | spike | frontend/syntax | class: blocked | Implement Contextualtypingwithgenericandnongenericsignature |
| 1534 | Implement Contextualtypingwithgenericsignature | spike | frontend/syntax | class: blocked | Implement Contextualtypingwithgenericsignature |
| 1535 | Implement Contextuallytypeargumentskeyword | spike | frontend/syntax | class: blocked | Implement Contextuallytypeargumentskeyword |
| 1536 | Implement Contextuallytypeasyncfunctionreturntypefromunion | spike | runtime/builtins | class: blocked | Implement Contextuallytypeasyncfunctionreturntypefromunion |
| 1537 | Implement Contextuallytypegeneratorreturntypefromunion | spike | runtime/builtins | class: blocked | Implement Contextuallytypegeneratorreturntypefromunion |
| 1538 | Implement Contextuallytypedbooleanliterals | spike | frontend/resolver | class: blocked | Implement Contextuallytypedbooleanliterals |
| 1539 | Implement Contextuallytypedbydiscriminableunion Parser Syntax | spike | frontend/syntax | class: blocked | Implement Contextuallytypedbydiscriminableunion Parser Syntax |
| 1540 | Implement Contextuallytypedbydiscriminableunion Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Contextuallytypedbydiscriminableunion Unknown Unsupported |
| 1541 | Implement Contextuallytypedjsxattribute | spike | frontend/syntax | class: blocked | Implement Contextuallytypedjsxattribute |
| 1542 | Implement Contextuallytypedoptionalproperty | spike | frontend/resolver | class: blocked | Implement Contextuallytypedoptionalproperty |
| 1543 | Implement Contextuallytypedparametersoptionalinjsdoc | spike | reference/triage | class: triage-needed | Implement Contextuallytypedparametersoptionalinjsdoc |
| 1544 | Implement Contextuallytypedparameterswithinitializers Arrow Function | spike | frontend/syntax | class: blocked | Implement Contextuallytypedparameterswithinitializers Arrow Function |
| 1545 | Implement Contextuallytypedparameterswithinitializers Import Export | spike | frontend/syntax | class: blocked | Implement Contextuallytypedparameterswithinitializers Import Export |
| 1546 | Implement Contextuallytypedparameterswithinitializers Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Contextuallytypedparameterswithinitializers Unknown Unsupported |
| 1547 | Implement Contextuallytypedparameterswithquestiontoken | spike | reference/triage | class: triage-needed | Implement Contextuallytypedparameterswithquestiontoken |
| 1548 | Implement Contextuallytypedsymbolnamedproperties | spike | frontend/syntax | class: blocked | Implement Contextuallytypedsymbolnamedproperties |
| 1549 | Implement Contextuallytypingrestparameters | spike | reference/triage | class: triage-needed | Implement Contextuallytypingrestparameters |
| 1550 | Implement Continueiniterationstatement | spike | frontend/resolver | class: blocked | Implement Continueiniterationstatement |
| 1551 | Implement Continueinloopswithcapturedblockscopedbindings | spike | frontend/syntax | class: blocked | Implement Continueinloopswithcapturedblockscopedbindings |
| 1552 | Implement Continuenotiniterationstatement Arrow Function | spike | frontend/syntax | class: blocked | Implement Continuenotiniterationstatement Arrow Function |
| 1553 | Implement Continuenotiniterationstatement Break Continue | spike | frontend/syntax | class: blocked | Implement Continuenotiniterationstatement Break Continue |
| 1554 | Implement Continuetarget | spike | frontend/syntax | class: blocked | Implement Continuetarget |
| 1555 | Implement Contravariantinferenceandtypeguard | spike | frontend/syntax | class: blocked | Implement Contravariantinferenceandtypeguard |
| 1556 | Implement Contravariantonlyinferencewithannotatedoptionalparameter | spike | frontend/resolver | class: blocked | Implement Contravariantonlyinferencewithannotatedoptionalparameter |
| 1557 | Implement Contravarianttypealiasinference | spike | frontend/resolver | class: blocked | Implement Contravarianttypealiasinference |
| 1558 | Implement Controlflowaliaseddiscriminants | spike | frontend/syntax | class: blocked | Implement Controlflowaliaseddiscriminants |
| 1559 | Implement Controlflowanalysisonbarethiskeyword | spike | frontend/semantics | class: blocked | Implement Controlflowanalysisonbarethiskeyword |
| 1560 | Implement Controlflowarrayerrors | spike | frontend/resolver | class: blocked | Implement Controlflowarrayerrors |
| 1561 | Implement Controlflowarrays | spike | frontend/semantics | class: blocked | Implement Controlflowarrays |
| 1562 | Implement Controlflowautoaccessor | spike | frontend/syntax | class: blocked | Implement Controlflowautoaccessor |
| 1563 | Implement Controlflowbreakcontinuewithlabel | spike | frontend/semantics | class: blocked | Implement Controlflowbreakcontinuewithlabel |
| 1564 | Implement Controlflowcaching | spike | frontend/semantics | class: blocked | Implement Controlflowcaching |
| 1565 | Implement Controlflowcommaexpressionassertionmultiple | spike | frontend/syntax | class: blocked | Implement Controlflowcommaexpressionassertionmultiple |
| 1566 | Implement Controlflowcommaexpressionassertionwithinternary | spike | frontend/syntax | class: blocked | Implement Controlflowcommaexpressionassertionwithinternary |
| 1567 | Implement Controlflowcommaexpressionfunctioncall | spike | frontend/semantics | class: blocked | Implement Controlflowcommaexpressionfunctioncall |
| 1568 | Implement Controlflowdestructuringloop | spike | frontend/syntax | class: blocked | Implement Controlflowdestructuringloop |
| 1569 | Implement Controlflowdestructuringvariablesintrycatch | spike | frontend/resolver | class: blocked | Implement Controlflowdestructuringvariablesintrycatch |
| 1570 | Implement Controlflowfavorassertedtypethroughtypepredicate | spike | frontend/semantics | class: blocked | Implement Controlflowfavorassertedtypethroughtypepredicate |
| 1571 | Implement Controlflowforcatchandfinally | spike | reference/triage | class: triage-needed | Implement Controlflowforcatchandfinally |
| 1572 | Implement Controlflowforcompoundassignmenttothismember | spike | frontend/semantics | class: blocked | Implement Controlflowforcompoundassignmenttothismember |
| 1573 | Implement Controlflowforfunctionlike | spike | frontend/syntax | class: blocked | Implement Controlflowforfunctionlike |
| 1574 | Implement Controlflowforindexsignatures | spike | frontend/semantics | class: blocked | Implement Controlflowforindexsignatures |
| 1575 | Implement Controlflowforstatementcontinueintoincrementor | spike | frontend/semantics | class: blocked | Implement Controlflowforstatementcontinueintoincrementor |
| 1576 | Implement Controlflowfunctionlikecircular | spike | frontend/syntax | class: triage-needed | Implement Controlflowfunctionlikecircular |
| 1577 | Implement Controlflowinitializeddestructuringvariables | spike | reference/triage | class: triage-needed | Implement Controlflowinitializeddestructuringvariables |
| 1578 | Implement Controlflowinstanceof | spike | frontend/resolver | class: blocked | Implement Controlflowinstanceof |
| 1579 | Implement Controlflowinstanceofwithsymbolhasinstance | spike | frontend/semantics | class: blocked | Implement Controlflowinstanceofwithsymbolhasinstance |
| 1580 | Implement Controlflowjavascript | spike | frontend/syntax | class: blocked | Implement Controlflowjavascript |
| 1581 | Implement Controlflowloopanalysis | spike | frontend/resolver | class: blocked | Implement Controlflowloopanalysis |
| 1582 | Implement Controlflowmanyconsecutiveconditionsnotimeout | spike | frontend/syntax | class: blocked | Implement Controlflowmanyconsecutiveconditionsnotimeout |
| 1583 | Implement Controlflownoimplicitany | spike | frontend/syntax | class: blocked | Implement Controlflownoimplicitany |
| 1584 | Implement Controlflownulltypeandliteral | spike | frontend/semantics | class: blocked | Implement Controlflownulltypeandliteral |
| 1585 | Implement Controlflowoutervariable | spike | frontend/semantics | class: blocked | Implement Controlflowoutervariable |
| 1586 | Implement Controlflowpropertydeclarations | spike | frontend/semantics | class: blocked | Implement Controlflowpropertydeclarations |
| 1587 | Implement Controlflowpropertyinitializer | spike | frontend/semantics | class: blocked | Implement Controlflowpropertyinitializer |
| 1588 | Implement Controlflowselfreferentialloop | spike | frontend/semantics | class: blocked | Implement Controlflowselfreferentialloop |
| 1589 | Implement Controlflowunioncontainingtypeparameter | spike | frontend/semantics | class: blocked | Implement Controlflowunioncontainingtypeparameter |
| 1590 | Implement Controlflowwithincompletetypes | spike | frontend/resolver | class: blocked | Implement Controlflowwithincompletetypes |
| 1591 | Implement Convertclassexpressiontofunctionfromobjectproperty | spike | frontend/syntax | class: blocked | Implement Convertclassexpressiontofunctionfromobjectproperty |
| 1592 | Implement Convertkeywordsyes | spike | frontend/syntax | class: blocked | Implement Convertkeywordsyes |
| 1593 | Implement Copyrightwithnewline | spike | frontend/syntax | class: blocked | Implement Copyrightwithnewline |
| 1594 | Implement Copyrightwithoutnewline | spike | frontend/syntax | class: blocked | Implement Copyrightwithoutnewline |
| 1595 | Implement Correctorderofpromisemethod | spike | reference/triage | class: triage-needed | Implement Correctorderofpromisemethod |
| 1596 | Implement Correlatedunions | spike | frontend/syntax | class: blocked | Implement Correlatedunions |
| 1597 | Implement Corrupted | spike | frontend/syntax | class: blocked | Implement Corrupted |
| 1598 | Implement Covariance | spike | frontend/syntax | class: blocked | Implement Covariance |
| 1599 | Implement Crashdeclareglobaltypeofexport | spike | frontend/syntax | class: blocked | Implement Crashdeclareglobaltypeofexport |
| 1600 | Implement Crashinemittokenwithcomment | spike | frontend/syntax | class: triage-needed | Implement Crashinemittokenwithcomment |
| 1601 | Implement Crashingettextofcomputedpropertyname | spike | frontend/syntax | class: blocked | Implement Crashingettextofcomputedpropertyname |
| 1602 | Implement Crashinresolveinterface | spike | frontend/resolver | class: blocked | Implement Crashinresolveinterface |
| 1603 | Implement Crashinyieldstarinasyncfunction | spike | frontend/syntax | class: triage-needed | Implement Crashinyieldstarinasyncfunction |
| 1604 | Implement Crashinresolvereturnstatement | spike | frontend/syntax | class: blocked | Implement Crashinresolvereturnstatement |
| 1605 | Implement Crashinsourcepropertyisrelatabletotargetproperty | spike | frontend/syntax | class: blocked | Implement Crashinsourcepropertyisrelatabletotargetproperty |
| 1606 | Implement Crashintypecheckinvocationexpression | spike | frontend/syntax | class: blocked | Implement Crashintypecheckinvocationexpression |
| 1607 | Implement Crashintypecheckobjectcreationexpression | spike | frontend/syntax | class: blocked | Implement Crashintypecheckobjectcreationexpression |
| 1608 | Implement Crashonmethodsignatures | spike | frontend/syntax | class: blocked | Implement Crashonmethodsignatures |
| 1609 | Implement Crashregressiontest | spike | frontend/syntax | class: blocked | Implement Crashregressiontest |
| 1610 | Implement Createarray | spike | frontend/syntax | class: triage-needed | Implement Createarray |
| 1611 | Implement Crossfileoverloadmodifierconsistency | spike | frontend/syntax | class: blocked | Implement Crossfileoverloadmodifierconsistency |
| 1612 | Implement Ctsfileinesnexthelpers | spike | frontend/syntax | class: blocked | Implement Ctsfileinesnexthelpers |
| 1613 | Implement Customasynciterator | spike | runtime/builtins | class: blocked | Implement Customasynciterator |
| 1614 | Implement Customeventdetail | spike | frontend/syntax | class: blocked | Implement Customeventdetail |
| 1615 | Implement Cyclicmoduleimport | spike | frontend/syntax | class: blocked | Implement Cyclicmoduleimport |
| 1616 | Implement Dataviewconstructor | spike | frontend/resolver | class: blocked | Implement Dataviewconstructor |
| 1617 | Implement Debugger | spike | frontend/resolver | class: blocked | Implement Debugger |
| 1618 | Implement Debuggeremit | spike | frontend/syntax | class: blocked | Implement Debuggeremit |
| 1619 | Implement Declfileaccessors | spike | frontend/syntax | class: blocked | Implement Declfileaccessors |
| 1620 | Implement Declfilealiasusebeforedeclaration | spike | frontend/syntax | class: blocked | Implement Declfilealiasusebeforedeclaration |
| 1621 | Implement Declfileambientexternalmodulewithsingleexportedmodule | spike | frontend/syntax | class: blocked | Implement Declfileambientexternalmodulewithsingleexportedmodule |
| 1622 | Implement Declfileclassextendsnull | spike | frontend/syntax | class: triage-needed | Implement Declfileclassextendsnull |
| 1623 | Implement Declfileclasswithindexsignature | spike | frontend/syntax | class: blocked | Implement Declfileclasswithindexsignature |
| 1624 | Implement Declfileclasswithstaticmethodreturningconstructor | spike | frontend/syntax | class: blocked | Implement Declfileclasswithstaticmethodreturningconstructor |
| 1625 | Implement Declfileconstructors | spike | frontend/syntax | class: blocked | Implement Declfileconstructors |
| 1626 | Implement Declfileemitdeclarationonly | spike | frontend/syntax | class: blocked | Implement Declfileemitdeclarationonly |
| 1627 | Implement Declfileenumusedasvalue | spike | frontend/syntax | class: blocked | Implement Declfileenumusedasvalue |
| 1628 | Implement Declfileenums | spike | frontend/syntax | class: blocked | Implement Declfileenums |
| 1629 | Implement Declfileexportassignmentimportinternalmodule | spike | frontend/syntax | class: blocked | Implement Declfileexportassignmentimportinternalmodule |
| 1630 | Implement Declfileexportassignmentofgenericinterface | spike | frontend/syntax | class: blocked | Implement Declfileexportassignmentofgenericinterface |
| 1631 | Implement Declfileexportimportchain | spike | frontend/syntax | class: blocked | Implement Declfileexportimportchain |
| 1632 | Implement Declfileforclasswithmultiplebaseclasses | spike | frontend/semantics | class: blocked | Implement Declfileforclasswithmultiplebaseclasses |
| 1633 | Implement Declfileforclasswithprivateoverloadedfunction | spike | frontend/semantics | class: blocked | Implement Declfileforclasswithprivateoverloadedfunction |
| 1634 | Implement Declfileforexportedimport | spike | frontend/syntax | class: blocked | Implement Declfileforexportedimport |
| 1635 | Implement Declfileforfunctiontypeastypeparameter | spike | frontend/semantics | class: blocked | Implement Declfileforfunctiontypeastypeparameter |
| 1636 | Implement Declfilefortypeparameters | spike | frontend/semantics | class: blocked | Implement Declfilefortypeparameters |
| 1637 | Implement Declfilefunctions | spike | frontend/syntax | class: blocked | Implement Declfilefunctions |
| 1638 | Implement Declfilegenericclasswithgenericextendedclass | spike | frontend/syntax | class: blocked | Implement Declfilegenericclasswithgenericextendedclass |
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
| 1651 | Implement Declfileprivatemethodoverloads | spike | frontend/semantics | class: blocked | Implement Declfileprivatemethodoverloads |
| 1652 | Implement Declfileprivatestatic | spike | frontend/semantics | class: blocked | Implement Declfileprivatestatic |
| 1653 | Implement Declfilerestparametersoffunctionandfunctiontype | spike | frontend/syntax | class: triage-needed | Implement Declfilerestparametersoffunctionandfunctiontype |
| 1654 | Implement Declfiletypeannotationarraytype | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationarraytype |
| 1655 | Implement Declfiletypeannotationparentype | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationparentype |
| 1656 | Implement Declfiletypeannotationtupletype | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationtupletype |
| 1657 | Implement Declfiletypeannotationtypealias | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationtypealias |
| 1658 | Implement Declfiletypeannotationtypeliteral | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationtypeliteral |
| 1659 | Implement Declfiletypeannotationtypequery | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationtypequery |
| 1660 | Implement Declfiletypeannotationtypereference | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationtypereference |
| 1661 | Implement Declfiletypeannotationuniontype | spike | frontend/semantics | class: blocked | Implement Declfiletypeannotationuniontype |
| 1662 | Implement Declfiletypeannotationvisibilityerroraccessors | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationvisibilityerroraccessors |
| 1663 | Implement Declfiletypeannotationvisibilityerrorparameteroffunction | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationvisibilityerrorparameteroffunction |
| 1664 | Implement Declfiletypeannotationvisibilityerrorreturntypeoffunction | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationvisibilityerrorreturntypeoffunction |
| 1665 | Implement Declfiletypeannotationvisibilityerrortypealias | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationvisibilityerrortypealias |
| 1666 | Implement Declfiletypeannotationvisibilityerrortypeliteral | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationvisibilityerrortypeliteral |
| 1667 | Implement Declfiletypeannotationvisibilityerrorvariabledeclaration | spike | frontend/syntax | class: blocked | Implement Declfiletypeannotationvisibilityerrorvariabledeclaration |
| 1668 | Implement Declfiletypeofclass | spike | frontend/syntax | class: blocked | Implement Declfiletypeofclass |
| 1669 | Implement Declfiletypeofenum | spike | frontend/syntax | class: blocked | Implement Declfiletypeofenum |
| 1670 | Implement Declfiletypeoffunction | spike | frontend/syntax | class: blocked | Implement Declfiletypeoffunction |
| 1671 | Implement Declfiletypeofinanonymoustype | spike | frontend/syntax | class: blocked | Implement Declfiletypeofinanonymoustype |
| 1672 | Implement Declfiletypeofmodule | spike | frontend/syntax | class: blocked | Implement Declfiletypeofmodule |
| 1673 | Implement Declfilewithclassnameconflictingwithclassreferredbyextendsclause | spike | frontend/syntax | class: blocked | Implement Declfilewithclassnameconflictingwithclassreferredbyextendsclause |
| 1674 | Implement Declfilewitherrorsininputdeclarationfile | spike | frontend/syntax | class: blocked | Implement Declfilewitherrorsininputdeclarationfile |
| 1675 | Implement Declfilewitherrorsininputdeclarationfilewithout | spike | frontend/syntax | class: blocked | Implement Declfilewitherrorsininputdeclarationfilewithout |
| 1676 | Implement Declfilewithextendsclausethathasitscontainernameconflict | spike | frontend/syntax | class: blocked | Implement Declfilewithextendsclausethathasitscontainernameconflict |
| 1677 | Implement Declfilewithinternalmodulenameconflictsinextendsclause | spike | frontend/syntax | class: blocked | Implement Declfilewithinternalmodulenameconflictsinextendsclause |
| 1678 | Implement Declinput Import Export | spike | frontend/syntax | class: blocked | Implement Declinput Import Export |
| 1679 | Implement Declinput Parser Syntax | spike | frontend/syntax | class: blocked | Implement Declinput Parser Syntax |
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
| 1699 | Implement Declarationemitclassmembernameconflict Parser Syntax | spike | frontend/resolver | class: blocked | Implement Declarationemitclassmembernameconflict Parser Syntax |
| 1700 | Implement Declarationemitclassmemberwithcomputedpropertyname | spike | frontend/syntax | class: blocked | Implement Declarationemitclassmemberwithcomputedpropertyname |
| 1701 | Implement Declarationemitclassmixinlocalclassdeclaration | spike | frontend/syntax | class: blocked | Implement Declarationemitclassmixinlocalclassdeclaration |
| 1702 | Implement Declarationemitclassprivateconstructor | spike | frontend/syntax | class: blocked | Implement Declarationemitclassprivateconstructor |
| 1703 | Implement Declarationemitclasssetaccessorparamnameinjs | spike | frontend/syntax | class: blocked | Implement Declarationemitclasssetaccessorparamnameinjs |
| 1704 | Implement Declarationemitcommonjsmodulereferencedtype | spike | frontend/syntax | class: blocked | Implement Declarationemitcommonjsmodulereferencedtype |
| 1705 | Implement Declarationemitcommonsourcedirectorydoesnotcontainallfiles | spike | frontend/syntax | class: blocked | Implement Declarationemitcommonsourcedirectorydoesnotcontainallfiles |
| 1706 | Implement Declarationemitcomputednamecausesimporttobepainted | spike | frontend/syntax | class: blocked | Implement Declarationemitcomputednamecausesimporttobepainted |
| 1707 | Implement Declarationemitcomputednameconstenumalias | spike | frontend/syntax | class: blocked | Implement Declarationemitcomputednameconstenumalias |
| 1708 | Implement Declarationemitcomputednamewithquestiontoken | spike | frontend/syntax | class: blocked | Implement Declarationemitcomputednamewithquestiontoken |
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
| 1732 | Implement Declarationemitenumreadonlyproperty | spike | frontend/semantics | class: blocked | Implement Declarationemitenumreadonlyproperty |
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
| 1775 | Implement Declarationemitmappedtypepreservestypeparameterconstraint | spike | frontend/semantics | class: blocked | Implement Declarationemitmappedtypepreservestypeparameterconstraint |
| 1776 | Implement Declarationemitmappedtypepropertyfromnumericstringkey | spike | frontend/syntax | class: blocked | Implement Declarationemitmappedtypepropertyfromnumericstringkey |
| 1777 | Implement Declarationemitmappedtypetemplatetypeofsymbol | spike | reference/triage | class: triage-needed | Implement Declarationemitmappedtypetemplatetypeofsymbol |
| 1778 | Implement Declarationemitmergedaliaswithconst | spike | frontend/resolver | class: blocked | Implement Declarationemitmergedaliaswithconst |
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
| 1825 | Implement Declarationemitresolvetypesifnotreusable | spike | frontend/syntax | class: blocked | Implement Declarationemitresolvetypesifnotreusable |
| 1826 | Implement Declarationemitretainedannotationretainsimportinoutput | spike | frontend/syntax | class: blocked | Implement Declarationemitretainedannotationretainsimportinoutput |
| 1827 | Implement Declarationemitretainsjsdocycomments | spike | frontend/syntax | class: blocked | Implement Declarationemitretainsjsdocycomments |
| 1828 | Implement Declarationemitreuseslambdaparameternodes | spike | frontend/syntax | class: blocked | Implement Declarationemitreuseslambdaparameternodes |
| 1829 | Implement Declarationemitscopeconsistency Declaration Emit | spike | frontend/syntax | class: blocked | Implement Declarationemitscopeconsistency Declaration Emit |
| 1830 | Implement Declarationemitscopeconsistency Import Export | spike | frontend/syntax | class: blocked | Implement Declarationemitscopeconsistency Import Export |
| 1831 | Implement Declarationemitshadowing | spike | frontend/syntax | class: blocked | Implement Declarationemitshadowing |
| 1832 | Implement Declarationemitshadowinginfernotrenamed | spike | frontend/syntax | class: blocked | Implement Declarationemitshadowinginfernotrenamed |
| 1833 | Implement Declarationemitsimplecomputednames | spike | frontend/syntax | class: blocked | Implement Declarationemitsimplecomputednames |
| 1834 | Implement Declarationemitspreadstringlykeyedenum | spike | frontend/semantics | class: blocked | Implement Declarationemitspreadstringlykeyedenum |
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
| 1863 | Implement Declarationfilesgeneratingtypereferences | spike | frontend/syntax | class: blocked | Implement Declarationfilesgeneratingtypereferences |
| 1864 | Implement Declarationfileswithtypereferences | spike | frontend/syntax | class: blocked | Implement Declarationfileswithtypereferences |
| 1865 | Implement Declarationfunctiontypenonlocalshouldnotbeanerror | spike | frontend/syntax | class: blocked | Implement Declarationfunctiontypenonlocalshouldnotbeanerror |
| 1866 | Implement Declarationimporttypealiasinferredandemittable | spike | frontend/syntax | class: blocked | Implement Declarationimporttypealiasinferredandemittable |
| 1867 | Implement Declarationmaps | spike | frontend/syntax | class: blocked | Implement Declarationmaps |
| 1868 | Implement Declarationmapsmultifile | spike | frontend/syntax | class: blocked | Implement Declarationmapsmultifile |
| 1869 | Implement Declarationmapsoutfile | spike | frontend/syntax | class: blocked | Implement Declarationmapsoutfile |
| 1870 | Implement Declarationmapswithoutdeclaration | spike | frontend/syntax | class: blocked | Implement Declarationmapswithoutdeclaration |
| 1871 | Implement Declarationmerging Import Export | spike | frontend/syntax | class: blocked | Implement Declarationmerging Import Export |
| 1872 | Implement Declarationmerging Parser Syntax | spike | frontend/resolver | class: blocked | Implement Declarationmerging Parser Syntax |
| 1873 | Implement Declarationnodanglinggenerics | spike | frontend/syntax | class: blocked | Implement Declarationnodanglinggenerics |
| 1874 | Implement Declarationquotedmembers | spike | frontend/syntax | class: blocked | Implement Declarationquotedmembers |
| 1875 | Implement Declarationtypechecknousebeforereferencecheck | spike | frontend/syntax | class: blocked | Implement Declarationtypechecknousebeforereferencecheck |
| 1876 | Implement Declarationsforfileshadowingglobalnoerror | spike | frontend/resolver | class: blocked | Implement Declarationsforfileshadowingglobalnoerror |
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
| 1888 | Implement Declareidentifierasbeginningofstatementexpression | spike | frontend/syntax | class: blocked | Implement Declareidentifierasbeginningofstatementexpression |
| 1889 | Implement Declaremodifieronimport | spike | frontend/syntax | class: blocked | Implement Declaremodifieronimport |
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
| 1917 | Implement Deepcomparisons | spike | frontend/syntax | class: blocked | Implement Deepcomparisons |
| 1918 | Implement Deepelaborationsintoarrowexpressions | spike | frontend/syntax | class: blocked | Implement Deepelaborationsintoarrowexpressions |
| 1919 | Implement Deepexcesspropertycheckingwhentargetisintersection | spike | frontend/syntax | class: triage-needed | Implement Deepexcesspropertycheckingwhentargetisintersection |
| 1920 | Implement Deepkeysindexing | spike | frontend/syntax | class: blocked | Implement Deepkeysindexing |
| 1921 | Implement Deeplydependentlargearraymutation | spike | frontend/syntax | class: triage-needed | Implement Deeplydependentlargearraymutation |
| 1922 | Implement Deeplynestedassignabilityerrorscombined | spike | runtime/builtins | class: blocked | Implement Deeplynestedassignabilityerrorscombined |
| 1923 | Implement Deeplynestedassignabilityissue | spike | frontend/syntax | class: blocked | Implement Deeplynestedassignabilityissue |
| 1924 | Implement Deeplynestedcheck | spike | frontend/syntax | class: triage-needed | Implement Deeplynestedcheck |
| 1925 | Implement Deeplynestedconstraints | spike | frontend/semantics | class: blocked | Implement Deeplynestedconstraints |
| 1926 | Implement Deeplynestedmappedtypes | spike | frontend/syntax | class: blocked | Implement Deeplynestedmappedtypes |
| 1927 | Implement Deeplynestedtemplateliteralintersection | spike | frontend/syntax | class: blocked | Implement Deeplynestedtemplateliteralintersection |
| 1928 | Implement Defaultargsinfunctionexpressions | spike | frontend/syntax | class: blocked | Implement Defaultargsinfunctionexpressions |
| 1929 | Implement Defaultargsinoverloads | spike | frontend/syntax | class: blocked | Implement Defaultargsinoverloads |
| 1930 | Implement Defaultdeclarationemitdefaultimport | spike | frontend/syntax | class: blocked | Implement Defaultdeclarationemitdefaultimport |
| 1931 | Implement Defaultdeclarationemitnamedcorrectly | spike | frontend/syntax | class: blocked | Implement Defaultdeclarationemitnamedcorrectly |
| 1932 | Implement Defaultdeclarationemitshadowednamedcorrectly | spike | frontend/syntax | class: blocked | Implement Defaultdeclarationemitshadowednamedcorrectly |
| 1933 | Implement Defaultindexprops | spike | frontend/syntax | class: blocked | Implement Defaultindexprops |
| 1934 | Implement Defaultisnotvisibleinlocalscope | spike | frontend/syntax | class: blocked | Implement Defaultisnotvisibleinlocalscope |
| 1935 | Implement Defaultkeywordwithoutexport | spike | frontend/syntax | class: blocked | Implement Defaultkeywordwithoutexport |
| 1936 | Implement Defaultnamedexportwithtype | spike | frontend/syntax | class: blocked | Implement Defaultnamedexportwithtype |
| 1937 | Implement Defaultparameteraddsundefinedwithstrictnullchecks | spike | frontend/resolver | class: blocked | Implement Defaultparameteraddsundefinedwithstrictnullchecks |
| 1938 | Implement Defaultpropsemptycurlybecomesanyforjs | spike | frontend/syntax | class: blocked | Implement Defaultpropsemptycurlybecomesanyforjs |
| 1939 | Implement Defaultvalueinconstructoroverload | spike | frontend/semantics | class: blocked | Implement Defaultvalueinconstructoroverload |
| 1940 | Implement Defaultvalueinfunctionoverload | spike | frontend/semantics | class: blocked | Implement Defaultvalueinfunctionoverload |
| 1941 | Implement Defaultvalueinfunctiontypes | spike | frontend/syntax | class: triage-needed | Implement Defaultvalueinfunctiontypes |
| 1942 | Implement Deferredconditionaltypes | spike | frontend/syntax | class: blocked | Implement Deferredconditionaltypes |
| 1943 | Implement Deferredlookuptyperesolution | spike | frontend/resolver | class: blocked | Implement Deferredlookuptyperesolution |
| 1944 | Implement Definevariables | spike | frontend/syntax | class: blocked | Implement Definevariables |
| 1945 | Implement Definiteassignmentofdestructuredvariable | spike | frontend/syntax | class: blocked | Implement Definiteassignmentofdestructuredvariable |
| 1946 | Implement Definiteassignmentwitherrorstillstripped | spike | frontend/semantics | class: blocked | Implement Definiteassignmentwitherrorstillstripped |
| 1947 | Implement Deleteexpressionmustbeoptional | spike | frontend/syntax | class: blocked | Implement Deleteexpressionmustbeoptional |
| 1948 | Implement Deletereadonlyinstrictnullchecks | spike | frontend/resolver | class: blocked | Implement Deletereadonlyinstrictnullchecks |
| 1949 | Implement Dependencyviaimportalias | spike | frontend/syntax | class: blocked | Implement Dependencyviaimportalias |
| 1950 | Implement Derivedclassconstructorwithexplicitreturns | spike | frontend/semantics | class: blocked | Implement Derivedclassconstructorwithexplicitreturns |
| 1951 | Implement Derivedclassoverridesprivatefunction | spike | frontend/semantics | class: blocked | Implement Derivedclassoverridesprivatefunction |
| 1952 | Implement Derivedclasses | spike | frontend/semantics | class: blocked | Implement Derivedclasses |
| 1953 | Implement Derivedinterfacecallsignature | spike | frontend/resolver | class: blocked | Implement Derivedinterfacecallsignature |
| 1954 | Implement Derivedtypecallingbaseimplwithoptionalparams | spike | frontend/syntax | class: blocked | Implement Derivedtypecallingbaseimplwithoptionalparams |
| 1955 | Implement Destructionassignmenterror | spike | runtime/builtins | class: blocked | Implement Destructionassignmenterror |
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
| 1978 | Implement Detachedcommentatstartofconstructor | spike | frontend/syntax | class: blocked | Implement Detachedcommentatstartofconstructor |
| 1979 | Implement Didyoumeanelaborationsforexpressionswhichcouldbecalled | spike | frontend/syntax | class: blocked | Implement Didyoumeanelaborationsforexpressionswhichcouldbecalled |
| 1980 | Implement Didyoumeansuggestionerrors | spike | frontend/syntax | class: blocked | Implement Didyoumeansuggestionerrors |
| 1981 | Implement Differenttypeswithsamename | spike | frontend/syntax | class: blocked | Implement Differenttypeswithsamename |
| 1982 | Implement Disallowedblockscopedinpresenceofparseerrors | spike | frontend/syntax | class: blocked | Implement Disallowedblockscopedinpresenceofparseerrors |
| 1983 | Implement Discriminableunionwithintersectedmembers | spike | frontend/syntax | class: blocked | Implement Discriminableunionwithintersectedmembers |
| 1984 | Implement Discriminantnarrowingcouldbecircular | spike | frontend/syntax | class: triage-needed | Implement Discriminantnarrowingcouldbecircular |
| 1985 | Implement Discriminantorderindependence | spike | frontend/resolver | class: blocked | Implement Discriminantorderindependence |
| 1986 | Implement Discriminantpropertycheck | spike | frontend/syntax | class: blocked | Implement Discriminantpropertycheck |
| 1987 | Implement Discriminantpropertyinference | spike | frontend/resolver | class: blocked | Implement Discriminantpropertyinference |
| 1988 | Implement Discriminantusingevaluatabletemplateexpression | spike | frontend/syntax | class: blocked | Implement Discriminantusingevaluatabletemplateexpression |
| 1989 | Implement Discriminantsandnullorundefined | spike | frontend/resolver | class: blocked | Implement Discriminantsandnullorundefined |
| 1990 | Implement Discriminantsandprimitives | spike | frontend/syntax | class: triage-needed | Implement Discriminantsandprimitives |
| 1991 | Implement Discriminatewithdivergentaccessors | spike | frontend/resolver | class: blocked | Implement Discriminatewithdivergentaccessors |
| 1992 | Implement Discriminatewithmissingproperty | spike | frontend/resolver | class: blocked | Implement Discriminatewithmissingproperty |
| 1993 | Implement Discriminatewithoptionalproperty Import Export | spike | frontend/syntax | class: blocked | Implement Discriminatewithoptionalproperty Import Export |
| 1994 | Implement Discriminatewithoptionalproperty Name Resolution | spike | frontend/resolver | class: blocked | Implement Discriminatewithoptionalproperty Name Resolution |
| 1995 | Implement Discriminatewithoptionalproperty Parser Syntax | spike | frontend/semantics | class: blocked | Implement Discriminatewithoptionalproperty Parser Syntax |
| 1996 | Implement Discriminatedunionerrormessage | spike | frontend/semantics | class: blocked | Implement Discriminatedunionerrormessage |
| 1997 | Implement Discriminatedunionwithindexsignature | spike | frontend/semantics | class: blocked | Implement Discriminatedunionwithindexsignature |
| 1998 | Implement Discriminatingunionwithunionpropertyagainstundefinedwithoutstrictnullchecks | spike | frontend/syntax | class: blocked | Implement Discriminatingunionwithunionpropertyagainstundefinedwithoutstrictnullchecks |
| 1999 | Implement Dissallowsymbolasweaktype | spike | frontend/resolver | class: blocked | Implement Dissallowsymbolasweaktype |
| 2000 | Implement Divergentaccessors | spike | frontend/syntax | class: blocked | Implement Divergentaccessors |
| 2001 | Implement Divergentaccessorstypes Class Accessor | spike | frontend/syntax | class: blocked | Implement Divergentaccessorstypes Class Accessor |
| 2002 | Implement Divergentaccessorstypes Name Resolution | spike | frontend/resolver | class: blocked | Implement Divergentaccessorstypes Name Resolution |
| 2003 | Implement Divergentaccessorstypes Parser Syntax | spike | frontend/semantics | class: blocked | Implement Divergentaccessorstypes Parser Syntax |
| 2004 | Implement Divergentaccessorsvisibility | spike | frontend/syntax | class: blocked | Implement Divergentaccessorsvisibility |
| 2005 | Implement Divideandconquerintersections | spike | frontend/syntax | class: blocked | Implement Divideandconquerintersections |
| 2006 | Implement Donotelaborateassignabilitytotypeparameters | spike | reference/triage | class: triage-needed | Implement Donotelaborateassignabilitytotypeparameters |
| 2007 | Implement Donotemitdetachedcommentsatstartoflambdafunction | spike | frontend/syntax | class: blocked | Implement Donotemitdetachedcommentsatstartoflambdafunction |
| 2008 | Implement Donotemitpinnedcommentnotontopoffile | spike | reference/triage | class: triage-needed | Implement Donotemitpinnedcommentnotontopoffile |
| 2009 | Implement Donotemitpinnedcommentonnotemittednode | spike | frontend/syntax | class: blocked | Implement Donotemitpinnedcommentonnotemittednode |
| 2010 | Implement Donotemitpinnedcommentonnotemittednodets | spike | frontend/syntax | class: blocked | Implement Donotemitpinnedcommentonnotemittednodets |
| 2011 | Implement Donotinferunrelatedtypes | spike | frontend/resolver | class: blocked | Implement Donotinferunrelatedtypes |
| 2012 | Implement Doyouneedtochangeyourtargetlibraryes Import Export | spike | frontend/syntax | class: blocked | Implement Doyouneedtochangeyourtargetlibraryes Import Export |
| 2013 | Implement Doyouneedtochangeyourtargetlibraryes Parser Syntax | spike | frontend/syntax | class: blocked | Implement Doyouneedtochangeyourtargetlibraryes Parser Syntax |
| 2014 | Implement Doyouneedtochangeyourtargetlibraryes Try Catch | spike | frontend/syntax | class: blocked | Implement Doyouneedtochangeyourtargetlibraryes Try Catch |
| 2015 | Implement Doesnotnarrowunionofconstructorswithinstanceof | spike | frontend/syntax | class: triage-needed | Implement Doesnotnarrowunionofconstructorswithinstanceof |
| 2016 | Implement Dottedmodulename | spike | frontend/syntax | class: blocked | Implement Dottedmodulename |
| 2017 | Implement Dottednamesinsystem | spike | frontend/syntax | class: blocked | Implement Dottednamesinsystem |
| 2018 | Implement Doublemixinconditionaltypebaseclassworks | spike | frontend/syntax | class: blocked | Implement Doublemixinconditionaltypebaseclassworks |
| 2019 | Implement Doubleunderscoreenumemit | spike | frontend/syntax | class: blocked | Implement Doubleunderscoreenumemit |
| 2020 | Implement Doubleunderscoreexportstarconflict | spike | frontend/syntax | class: blocked | Implement Doubleunderscoreexportstarconflict |
| 2021 | Implement Doubleunderscorereactnamespace | spike | frontend/syntax | class: blocked | Implement Doubleunderscorereactnamespace |
| 2022 | Implement Downleveliterationdeprecated | spike | frontend/resolver | class: blocked | Implement Downleveliterationdeprecated |
| 2023 | Implement Downlevelletconst Arrow Function | spike | frontend/syntax | class: blocked | Implement Downlevelletconst Arrow Function |
| 2024 | Implement Downlevelletconst Import Export | spike | frontend/syntax | class: blocked | Implement Downlevelletconst Import Export |
| 2025 | Implement Downlevelletconst Name Resolution | spike | frontend/resolver | class: blocked | Implement Downlevelletconst Name Resolution |
| 2026 | Implement Downlevelletconst Parser Syntax | spike | frontend/syntax | class: blocked | Implement Downlevelletconst Parser Syntax |
| 2027 | Implement Downlevelletconst Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Downlevelletconst Unknown Unsupported |
| 2028 | Implement Dtsemittripleslashavoidunnecessaryresolutionmode | spike | frontend/syntax | class: blocked | Implement Dtsemittripleslashavoidunnecessaryresolutionmode |
| 2029 | Implement Duplicateanonymousinners | spike | frontend/syntax | class: blocked | Implement Duplicateanonymousinners |
| 2030 | Implement Duplicateanonymousmoduleclasses | spike | frontend/syntax | class: blocked | Implement Duplicateanonymousmoduleclasses |
| 2031 | Implement Duplicateclasselements | spike | frontend/resolver | class: blocked | Implement Duplicateclasselements |
| 2032 | Implement Duplicateconstructoroverloadsignature | spike | frontend/semantics | class: blocked | Implement Duplicateconstructoroverloadsignature |
| 2033 | Implement Duplicatedefaultexport | spike | frontend/syntax | class: blocked | Implement Duplicatedefaultexport |
| 2034 | Implement Duplicateerrorassignability | spike | frontend/resolver | class: blocked | Implement Duplicateerrorassignability |
| 2035 | Implement Duplicateerrorclassexpression | spike | frontend/resolver | class: blocked | Implement Duplicateerrorclassexpression |
| 2036 | Implement Duplicateerrornamenotfound | spike | frontend/syntax | class: blocked | Implement Duplicateerrornamenotfound |
| 2037 | Implement Duplicateidentifierbindingelementinparameterdeclaration | spike | reference/triage | class: triage-needed | Implement Duplicateidentifierbindingelementinparameterdeclaration |
| 2038 | Implement Duplicateidentifiercomputedname | spike | frontend/resolver | class: blocked | Implement Duplicateidentifiercomputedname |
| 2039 | Implement Duplicateidentifierdifferentmodifiers | spike | frontend/resolver | class: blocked | Implement Duplicateidentifierdifferentmodifiers |
| 2040 | Implement Duplicateidentifierdifferentspelling | spike | frontend/resolver | class: blocked | Implement Duplicateidentifierdifferentspelling |
| 2041 | Implement Duplicateidentifierenum | spike | frontend/resolver | class: blocked | Implement Duplicateidentifierenum |
| 2042 | Implement Duplicateidentifierincatchblock | spike | reference/triage | class: triage-needed | Implement Duplicateidentifierincatchblock |
| 2043 | Implement Duplicateidentifierrelatedspans Duplicate Function | spike | reference/triage | class: triage-needed | Implement Duplicateidentifierrelatedspans Duplicate Function |
| 2044 | Implement Duplicateidentifierrelatedspans Import Export | spike | frontend/syntax | class: blocked | Implement Duplicateidentifierrelatedspans Import Export |
| 2045 | Implement Duplicateidentifierrelatedspans Parser Syntax | spike | frontend/resolver | class: blocked | Implement Duplicateidentifierrelatedspans Parser Syntax |
| 2046 | Implement Duplicateidentifiershouldnotshortcircuitbasetypebinding | spike | frontend/syntax | class: blocked | Implement Duplicateidentifiershouldnotshortcircuitbasetypebinding |
| 2047 | Implement Duplicateidentifiersacrosscontainerboundaries | spike | frontend/syntax | class: blocked | Implement Duplicateidentifiersacrosscontainerboundaries |
| 2048 | Implement Duplicateidentifiersacrossfileboundaries | spike | frontend/syntax | class: blocked | Implement Duplicateidentifiersacrossfileboundaries |
| 2049 | Implement Duplicatelabel | spike | frontend/syntax | class: triage-needed | Implement Duplicatelabel |
| 2050 | Implement Duplicatelocalvariable Duplicate Local | spike | reference/triage | class: triage-needed | Implement Duplicatelocalvariable Duplicate Local |
| 2051 | Implement Duplicatelocalvariable Import Export | spike | frontend/syntax | class: blocked | Implement Duplicatelocalvariable Import Export |
| 2052 | Implement Duplicatelocalvariable Parser Syntax | spike | frontend/resolver | class: blocked | Implement Duplicatelocalvariable Parser Syntax |
| 2053 | Implement Duplicateobjectliteralproperty Import Export | spike | frontend/syntax | class: blocked | Implement Duplicateobjectliteralproperty Import Export |
| 2054 | Implement Duplicateobjectliteralproperty Object Literal | spike | frontend/syntax | class: blocked | Implement Duplicateobjectliteralproperty Object Literal |
| 2055 | Implement Duplicateobjectliteralproperty Parser Syntax | spike | frontend/resolver | class: blocked | Implement Duplicateobjectliteralproperty Parser Syntax |
| 2056 | Implement Duplicateoverloadintypeaugmentation | spike | frontend/syntax | class: blocked | Implement Duplicateoverloadintypeaugmentation |
| 2057 | Implement Duplicatepackage Import Export | spike | frontend/syntax | class: blocked | Implement Duplicatepackage Import Export |
| 2058 | Implement Duplicatepackage Module Resolution | spike | frontend/syntax | class: blocked | Implement Duplicatepackage Module Resolution |
| 2059 | Implement Duplicatepackage Parser Syntax | spike | frontend/resolver | class: blocked | Implement Duplicatepackage Parser Syntax |
| 2060 | Implement Duplicatepropertiesinstrictmode | spike | frontend/resolver | class: blocked | Implement Duplicatepropertiesinstrictmode |
| 2061 | Implement Duplicatesymbolsexportmatching | spike | frontend/syntax | class: blocked | Implement Duplicatesymbolsexportmatching |
| 2062 | Implement Duplicatetypeparameters | spike | frontend/semantics | class: blocked | Implement Duplicatetypeparameters |
| 2063 | Implement Duplicatevarandimport | spike | frontend/syntax | class: blocked | Implement Duplicatevarandimport |
| 2064 | Implement Duplicatevariabledeclaration | spike | frontend/resolver | class: blocked | Implement Duplicatevariabledeclaration |
| 2065 | Implement Duplicatevariablesbyscope | spike | frontend/syntax | class: blocked | Implement Duplicatevariablesbyscope |
| 2066 | Implement Duplicatevariableswithany | spike | frontend/syntax | class: blocked | Implement Duplicatevariableswithany |
| 2067 | Implement Duplicatevarsacrossfileboundaries | spike | frontend/syntax | class: blocked | Implement Duplicatevarsacrossfileboundaries |
| 2068 | Implement Dynamicimportevaluatespecifier | spike | frontend/syntax | class: blocked | Implement Dynamicimportevaluatespecifier |
| 2069 | Implement Dynamicimportindefaultexportexpression | spike | frontend/syntax | class: blocked | Implement Dynamicimportindefaultexportexpression |
| 2070 | Implement Dynamicimporttrailingcomma | spike | frontend/syntax | class: blocked | Implement Dynamicimporttrailingcomma |
| 2071 | Implement Dynamicimportwithnestedthis | spike | frontend/syntax | class: blocked | Implement Dynamicimportwithnestedthis |
| 2072 | Implement Dynamicimportsdeclaration | spike | frontend/syntax | class: blocked | Implement Dynamicimportsdeclaration |
| 2073 | Implement Dynamicmoduletypecheckerror | spike | frontend/syntax | class: blocked | Implement Dynamicmoduletypecheckerror |
| 2074 | Implement Dynamicnames | spike | frontend/syntax | class: blocked | Implement Dynamicnames |
| 2075 | Implement Dynamicnameserrors | spike | frontend/syntax | class: blocked | Implement Dynamicnameserrors |
| 2076 | Implement Dynamicrequire | spike | frontend/syntax | class: blocked | Implement Dynamicrequire |
| 2077 | Implement Elaboratederrors | spike | runtime/builtins | class: blocked | Implement Elaboratederrors |
| 2078 | Implement Elaboratederrorsonnullabletargets | spike | frontend/resolver | class: blocked | Implement Elaboratederrorsonnullabletargets |
| 2079 | Implement Elaborationforpossiblycallabletypestillreferencesargumentattoplevel | spike | frontend/resolver | class: blocked | Implement Elaborationforpossiblycallabletypestillreferencesargumentattoplevel |
| 2080 | Implement Elidedembeddedstatementsreplacedwithsemicolon | spike | frontend/syntax | class: triage-needed | Implement Elidedembeddedstatementsreplacedwithsemicolon |
| 2081 | Implement Elidedjsimport | spike | frontend/syntax | class: blocked | Implement Elidedjsimport |
| 2082 | Implement Elidingimportnames | spike | frontend/syntax | class: blocked | Implement Elidingimportnames |
| 2083 | Implement Emitaccessexpressionofcastedobjectliteralexpressioninarrowfunctiones | spike | frontend/semantics | class: blocked | Implement Emitaccessexpressionofcastedobjectliteralexpressioninarrowfunctiones |
| 2084 | Implement Emitbundlewithprologuedirectives | spike | frontend/syntax | class: blocked | Implement Emitbundlewithprologuedirectives |
| 2085 | Implement Emitbundlewithshebang | spike | frontend/syntax | class: triage-needed | Implement Emitbundlewithshebang |
| 2086 | Implement Emitbundlewithshebangandprologuedirectives | spike | frontend/syntax | class: triage-needed | Implement Emitbundlewithshebangandprologuedirectives |
| 2087 | Implement Emitcapturingthisintupledestructuring | spike | frontend/syntax | class: blocked | Implement Emitcapturingthisintupledestructuring |
| 2088 | Implement Emitclassexpressionindeclarationfile | spike | frontend/syntax | class: blocked | Implement Emitclassexpressionindeclarationfile |
| 2089 | Implement Emitclassmergedwithconstnamespacenotelided | spike | frontend/syntax | class: blocked | Implement Emitclassmergedwithconstnamespacenotelided |
| 2090 | Implement Emitdecoratormetadata Decorator | spike | frontend/syntax | class: blocked | Implement Emitdecoratormetadata Decorator |
| 2091 | Implement Emitdecoratormetadata Import Export | spike | frontend/syntax | class: blocked | Implement Emitdecoratormetadata Import Export |
| 2092 | Implement Emithelperswithlocalcollisions | spike | frontend/resolver | class: blocked | Implement Emithelperswithlocalcollisions |
| 2093 | Implement Emitmemberaccessexpression | spike | frontend/syntax | class: blocked | Implement Emitmemberaccessexpression |
| 2094 | Implement Emitmethodcallednew | spike | frontend/syntax | class: blocked | Implement Emitmethodcallednew |
| 2095 | Implement Emitonelinevariabledeclarationremovecommentsfalse | spike | frontend/syntax | class: blocked | Implement Emitonelinevariabledeclarationremovecommentsfalse |
| 2096 | Implement Emitskipsthiswithrestparameter | spike | frontend/semantics | class: blocked | Implement Emitskipsthiswithrestparameter |
| 2097 | Implement Emitsupercallbeforeemitparameterpropertydeclaration | spike | frontend/syntax | class: blocked | Implement Emitsupercallbeforeemitparameterpropertydeclaration |
| 2098 | Implement Emitsupercallbeforeemitpropertydeclaration | spike | frontend/syntax | class: blocked | Implement Emitsupercallbeforeemitpropertydeclaration |
| 2099 | Implement Emitsupercallbeforeemitpropertydeclarationandparameterpropertydeclaration | spike | frontend/syntax | class: blocked | Implement Emitsupercallbeforeemitpropertydeclarationandparameterpropertydeclaration |
| 2100 | Implement Emitthisinobjectliteralgetter | spike | frontend/syntax | class: blocked | Implement Emitthisinobjectliteralgetter |
| 2101 | Implement Emitthisinsupermethodcall | spike | frontend/syntax | class: blocked | Implement Emitthisinsupermethodcall |
| 2102 | Implement Emittopoffiletripleslashcommentonnotemittednodeifremovecommentsisfalse | spike | frontend/syntax | class: blocked | Implement Emittopoffiletripleslashcommentonnotemittednodeifremovecommentsisfalse |
| 2103 | Implement Emptyanonymousobjectnarrowing | spike | frontend/syntax | class: blocked | Implement Emptyanonymousobjectnarrowing |
| 2104 | Implement Emptyargumentslistcomment | spike | frontend/resolver | class: blocked | Implement Emptyargumentslistcomment |
| 2105 | Implement Emptyarraydestructuringexpressionvisitedbytransformer | spike | frontend/resolver | class: blocked | Implement Emptyarraydestructuringexpressionvisitedbytransformer |
| 2106 | Implement Emptydeclarationemitismodule | spike | frontend/syntax | class: blocked | Implement Emptydeclarationemitismodule |
| 2107 | Implement Emptyenum | spike | frontend/syntax | class: blocked | Implement Emptyenum |
| 2108 | Implement Emptygenericparamlist | spike | frontend/syntax | class: blocked | Implement Emptygenericparamlist |
| 2109 | Implement Emptyindexer | spike | frontend/syntax | class: blocked | Implement Emptyindexer |
| 2110 | Implement Emptymemberaccess | spike | frontend/syntax | class: triage-needed | Implement Emptymemberaccess |
| 2111 | Implement Emptymodulename | spike | frontend/syntax | class: blocked | Implement Emptymodulename |
| 2112 | Implement Emptyobjectnotsubtypeofindexsignaturecontainingobject | spike | frontend/syntax | class: blocked | Implement Emptyobjectnotsubtypeofindexsignaturecontainingobject |
| 2113 | Implement Emptyoptionalbindingpatternindeclarationsignature | spike | frontend/syntax | class: blocked | Implement Emptyoptionalbindingpatternindeclarationsignature |
| 2114 | Implement Emptythenwarning | spike | frontend/syntax | class: triage-needed | Implement Emptythenwarning |
| 2115 | Implement Emptytypeargumentlist | spike | frontend/syntax | class: triage-needed | Implement Emptytypeargumentlist |
| 2116 | Implement Emptytypeargumentlistwithnew | spike | frontend/semantics | class: blocked | Implement Emptytypeargumentlistwithnew |
| 2117 | Implement Ensurenocrashexportassignmentdefineproperrtypotentialmerge | spike | frontend/syntax | class: blocked | Implement Ensurenocrashexportassignmentdefineproperrtypotentialmerge |
| 2118 | Implement Enumassignmentcompat Import Export | spike | frontend/syntax | class: blocked | Implement Enumassignmentcompat Import Export |
| 2119 | Implement Enumassignmentcompat Parser Syntax | spike | frontend/semantics | class: blocked | Implement Enumassignmentcompat Parser Syntax |
| 2120 | Implement Enumbasics Import Export | spike | frontend/syntax | class: blocked | Implement Enumbasics Import Export |
| 2121 | Implement Enumbasics Parser Syntax | spike | frontend/syntax | class: blocked | Implement Enumbasics Parser Syntax |
| 2122 | Implement Enumcodegennewlines | spike | frontend/syntax | class: blocked | Implement Enumcodegennewlines |
| 2123 | Implement Enumconflictswithglobalidentifier | spike | frontend/resolver | class: blocked | Implement Enumconflictswithglobalidentifier |
| 2124 | Implement Enumdecl | spike | frontend/syntax | class: blocked | Implement Enumdecl |
| 2125 | Implement Enumdeclarationemitinitializerhasimport | spike | frontend/syntax | class: blocked | Implement Enumdeclarationemitinitializerhasimport |
| 2126 | Implement Enumfromexternalmodule | spike | frontend/syntax | class: blocked | Implement Enumfromexternalmodule |
| 2127 | Implement Enumgenerictypeclash | spike | frontend/syntax | class: blocked | Implement Enumgenerictypeclash |
| 2128 | Implement Enumidentifierliterals | spike | frontend/syntax | class: blocked | Implement Enumidentifierliterals |
| 2129 | Implement Enumindexer | spike | frontend/syntax | class: blocked | Implement Enumindexer |
| 2130 | Implement Enuminitializerswithexponents | spike | frontend/syntax | class: blocked | Implement Enuminitializerswithexponents |
| 2131 | Implement Enumkeysquotedasobjectpropertiesindeclarationemit | spike | frontend/syntax | class: blocked | Implement Enumkeysquotedasobjectpropertiesindeclarationemit |
| 2132 | Implement Enumliteralassignabletoenuminsideunion | spike | frontend/syntax | class: blocked | Implement Enumliteralassignabletoenuminsideunion |
| 2133 | Implement Enumliteralunionnotwidened | spike | frontend/syntax | class: blocked | Implement Enumliteralunionnotwidened |
| 2134 | Implement Enumliteralssubtypereduction | spike | frontend/syntax | class: blocked | Implement Enumliteralssubtypereduction |
| 2135 | Implement Enummapbackintoitself | spike | frontend/syntax | class: blocked | Implement Enummapbackintoitself |
| 2136 | Implement Enummembernamenonidentifier | spike | frontend/syntax | class: blocked | Implement Enummembernamenonidentifier |
| 2137 | Implement Enummemberreduction | spike | frontend/syntax | class: blocked | Implement Enummemberreduction |
| 2138 | Implement Enummemberresolution | spike | frontend/syntax | class: blocked | Implement Enummemberresolution |
| 2139 | Implement Enumnegativeliteral | spike | frontend/syntax | class: blocked | Implement Enumnegativeliteral |
| 2140 | Implement Enumnoinitializerfollowsnonliteralinitializer | spike | frontend/syntax | class: blocked | Implement Enumnoinitializerfollowsnonliteralinitializer |
| 2141 | Implement Enumnumbering | spike | frontend/syntax | class: blocked | Implement Enumnumbering |
| 2142 | Implement Enumoperations | spike | frontend/syntax | class: blocked | Implement Enumoperations |
| 2143 | Implement Enumpropertyaccess | spike | frontend/syntax | class: blocked | Implement Enumpropertyaccess |
| 2144 | Implement Enumpropertyaccessbeforeinitalisation | spike | frontend/syntax | class: blocked | Implement Enumpropertyaccessbeforeinitalisation |
| 2145 | Implement Enumusedbeforedeclaration | spike | frontend/syntax | class: blocked | Implement Enumusedbeforedeclaration |
| 2146 | Implement Enumwithbigint | spike | runtime/builtins | class: blocked | Implement Enumwithbigint |
| 2147 | Implement Enumwithcomputedmember | spike | frontend/syntax | class: blocked | Implement Enumwithcomputedmember |
| 2148 | Implement Enumwithexport | spike | frontend/syntax | class: blocked | Implement Enumwithexport |
| 2149 | Implement Enumwithinfinityproperty | spike | frontend/syntax | class: blocked | Implement Enumwithinfinityproperty |
| 2150 | Implement Enumwithnanproperty | spike | frontend/syntax | class: blocked | Implement Enumwithnanproperty |
| 2151 | Implement Enumwithnegativeinfinityproperty | spike | frontend/syntax | class: blocked | Implement Enumwithnegativeinfinityproperty |
| 2152 | Implement Enumwithnonliteralstringinitializer | spike | frontend/syntax | class: blocked | Implement Enumwithnonliteralstringinitializer |
| 2153 | Implement Enumwithparenthesizedinitializer | spike | frontend/syntax | class: blocked | Implement Enumwithparenthesizedinitializer |
| 2154 | Implement Enumwithprimitivename | spike | frontend/syntax | class: blocked | Implement Enumwithprimitivename |
| 2155 | Implement Enumwithquotedelementname | spike | frontend/syntax | class: blocked | Implement Enumwithquotedelementname |
| 2156 | Implement Enumwithunicodeescape | spike | frontend/syntax | class: blocked | Implement Enumwithunicodeescape |
| 2157 | Implement Enumwithoutinitializeraftercomputedmember | spike | frontend/syntax | class: blocked | Implement Enumwithoutinitializeraftercomputedmember |
| 2158 | Implement Enumswithmultipledeclarations Import Export | spike | frontend/syntax | class: blocked | Implement Enumswithmultipledeclarations Import Export |
| 2159 | Implement Enumswithmultipledeclarations Parser Syntax | spike | frontend/syntax | class: blocked | Implement Enumswithmultipledeclarations Parser Syntax |
| 2160 | Implement Erasablesyntaxonly Import Export | spike | frontend/syntax | class: blocked | Implement Erasablesyntaxonly Import Export |
| 2161 | Implement Erasablesyntaxonly Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Erasablesyntaxonly Unknown Unsupported |
| 2162 | Implement Erasablesyntaxonlydeclaration | spike | frontend/syntax | class: blocked | Implement Erasablesyntaxonlydeclaration |
| 2163 | Implement Errorcause | spike | frontend/resolver | class: blocked | Implement Errorcause |
| 2164 | Implement Errorconstructorsubtypes | spike | runtime/builtins | class: blocked | Implement Errorconstructorsubtypes |
| 2165 | Implement Errorelaboration | spike | runtime/builtins | class: blocked | Implement Errorelaboration |
| 2166 | Implement Errorforbarespecifierwithimplicitmoduleresolutionnone | spike | frontend/syntax | class: blocked | Implement Errorforbarespecifierwithimplicitmoduleresolutionnone |
| 2167 | Implement Errorforconflictingexportequalsvalue | spike | frontend/syntax | class: blocked | Implement Errorforconflictingexportequalsvalue |
| 2168 | Implement Errorforusingpropertyoftypeastype | spike | frontend/syntax | class: blocked | Implement Errorforusingpropertyoftypeastype |
| 2169 | Implement Errorforwardreferenceforwadingconstructor | spike | reference/triage | class: triage-needed | Implement Errorforwardreferenceforwadingconstructor |
| 2170 | Implement Errorhandlingininstanceof | spike | frontend/resolver | class: blocked | Implement Errorhandlingininstanceof |
| 2171 | Implement Errorinunnamedclassexpression | spike | frontend/syntax | class: blocked | Implement Errorinunnamedclassexpression |
| 2172 | Implement Errorinfoforrelatedindextypesnoconstraintelaboration | spike | frontend/semantics | class: blocked | Implement Errorinfoforrelatedindextypesnoconstraintelaboration |
| 2173 | Implement Errormessageonintersectionswithdiscriminants | spike | frontend/resolver | class: blocked | Implement Errormessageonintersectionswithdiscriminants |
| 2174 | Implement Errormessageonobjectliteraltype | spike | frontend/resolver | class: blocked | Implement Errormessageonobjectliteraltype |
| 2175 | Implement Errormessagesintersectiontypes | spike | frontend/resolver | class: blocked | Implement Errormessagesintersectiontypes |
| 2176 | Implement Erroronenumreferenceincondition | spike | runtime/builtins | class: blocked | Implement Erroronenumreferenceincondition |
| 2177 | Implement Errorrecoveryinclassdeclaration | spike | runtime/builtins | class: blocked | Implement Errorrecoveryinclassdeclaration |
| 2178 | Implement Errorrecoverywithdotfollowedbynamespacekeyword | spike | frontend/syntax | class: blocked | Implement Errorrecoverywithdotfollowedbynamespacekeyword |
| 2179 | Implement Errorsupression | spike | frontend/resolver | class: blocked | Implement Errorsupression |
| 2180 | Implement Errorwithsamenametype | spike | runtime/builtins | class: blocked | Implement Errorwithsamenametype |
| 2181 | Implement Errorwithtruncatedtype | spike | frontend/resolver | class: blocked | Implement Errorwithtruncatedtype |
| 2182 | Implement Errorsforcallandassignmentaresimilar | spike | runtime/builtins | class: blocked | Implement Errorsforcallandassignmentaresimilar |
| 2183 | Implement Errorsingenerictypereference | spike | frontend/syntax | class: blocked | Implement Errorsingenerictypereference |
| 2184 | Implement Errorsonimportedsymbol | spike | frontend/syntax | class: blocked | Implement Errorsonimportedsymbol |
| 2185 | Implement Errorsonunionsofoverlappingobjects | spike | frontend/syntax | class: blocked | Implement Errorsonunionsofoverlappingobjects |
| 2186 | Implement Errorswithinvokablesinunions | spike | frontend/syntax | class: blocked | Implement Errorswithinvokablesinunions |
| 2187 | Implement Es Destructuring | spike | frontend/syntax | class: blocked | Implement Es Destructuring |
| 2188 | Implement Es Duplicate Local | spike | reference/triage | class: triage-needed | Implement Es Duplicate Local |
| 2189 | Implement Es Import Export | spike | frontend/syntax | class: blocked | Implement Es Import Export |
| 2190 | Implement Es Module System Amd | spike | frontend/syntax | class: blocked | Implement Es Module System Amd |
| 2191 | Implement Es Object Literal | spike | frontend/syntax | class: blocked | Implement Es Object Literal |
| 2192 | Implement Es Parser Syntax | spike | frontend/syntax | class: blocked | Implement Es Parser Syntax |
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
| 2214 | Implement Evalafter | spike | runtime/builtins | class: blocked | Implement Evalafter |
| 2215 | Implement Evalorargumentsindeclarationfunctions | spike | runtime/builtins | class: blocked | Implement Evalorargumentsindeclarationfunctions |
| 2216 | Implement Evolvingarraytypeinassert | spike | frontend/syntax | class: blocked | Implement Evolvingarraytypeinassert |
| 2217 | Implement Exactoptionalpropertytypesidentical | spike | frontend/syntax | class: blocked | Implement Exactoptionalpropertytypesidentical |
| 2218 | Implement Exactspellingsuggestion | spike | frontend/syntax | class: blocked | Implement Exactspellingsuggestion |
| 2219 | Implement Excesspropertiesinoverloads | spike | frontend/resolver | class: blocked | Implement Excesspropertiesinoverloads |
| 2220 | Implement Excesspropertycheckintersectionwithindexsignature | spike | frontend/syntax | class: triage-needed | Implement Excesspropertycheckintersectionwithindexsignature |
| 2221 | Implement Excesspropertycheckintersectionwithrecursivetype | spike | frontend/semantics | class: blocked | Implement Excesspropertycheckintersectionwithrecursivetype |
| 2222 | Implement Excesspropertycheckwithemptyobject | spike | frontend/resolver | class: blocked | Implement Excesspropertycheckwithemptyobject |
| 2223 | Implement Excesspropertycheckwithmultiplediscriminants | spike | frontend/semantics | class: blocked | Implement Excesspropertycheckwithmultiplediscriminants |
| 2224 | Implement Excesspropertycheckwithnestedarrayintersection | spike | frontend/semantics | class: blocked | Implement Excesspropertycheckwithnestedarrayintersection |
| 2225 | Implement Excesspropertycheckwithspread | spike | frontend/resolver | class: blocked | Implement Excesspropertycheckwithspread |
| 2226 | Implement Excesspropertycheckwithunions | spike | frontend/semantics | class: blocked | Implement Excesspropertycheckwithunions |
| 2227 | Implement Excesspropertycheckingintersectionwithconditional | spike | frontend/syntax | class: blocked | Implement Excesspropertycheckingintersectionwithconditional |
| 2228 | Implement Excesspropertycheckswithnestedintersections | spike | frontend/syntax | class: blocked | Implement Excesspropertycheckswithnestedintersections |
| 2229 | Implement Excesspropertyerrorforfunctiontypes | spike | frontend/semantics | class: blocked | Implement Excesspropertyerrorforfunctiontypes |
| 2230 | Implement Excessivestackdepthflatarray | spike | reference/triage | class: blocked | Implement Excessivestackdepthflatarray |
| 2231 | Implement Excessivelylargetuplespread | spike | frontend/semantics | class: blocked | Implement Excessivelylargetuplespread |
| 2232 | Implement Exhaustiveswitchcheckcircularity | spike | frontend/semantics | class: blocked | Implement Exhaustiveswitchcheckcircularity |
| 2233 | Implement Exhaustiveswitchwithwideningliteraltypes | spike | frontend/semantics | class: blocked | Implement Exhaustiveswitchwithwideningliteraltypes |
| 2234 | Implement Expandofunctionblockshadowing | spike | frontend/syntax | class: blocked | Implement Expandofunctionblockshadowing |
| 2235 | Implement Expandofunctioncontextualtypesjsdocints | spike | frontend/syntax | class: blocked | Implement Expandofunctioncontextualtypesjsdocints |
| 2236 | Implement Expandofunctioncontextualtypesjs | spike | frontend/syntax | class: blocked | Implement Expandofunctioncontextualtypesjs |
| 2237 | Implement Expandofunctioncontextualtypesnovalue | spike | frontend/syntax | class: blocked | Implement Expandofunctioncontextualtypesnovalue |
| 2238 | Implement Expandofunctionexpressionswithdynamicnames | spike | frontend/syntax | class: blocked | Implement Expandofunctionexpressionswithdynamicnames |
| 2239 | Implement Expandofunctionnestedassigments | spike | frontend/syntax | class: blocked | Implement Expandofunctionnestedassigments |
| 2240 | Implement Expandofunctionnestedassigmentsdeclared | spike | frontend/syntax | class: blocked | Implement Expandofunctionnestedassigmentsdeclared |
| 2241 | Implement Expandofunctionnullishproperty | spike | frontend/syntax | class: blocked | Implement Expandofunctionnullishproperty |
| 2242 | Implement Expandofunctionsymbolproperty | spike | frontend/syntax | class: blocked | Implement Expandofunctionsymbolproperty |
| 2243 | Implement Expandofunctionsymbolpropertyjs | spike | frontend/syntax | class: blocked | Implement Expandofunctionsymbolpropertyjs |
| 2244 | Implement Experimentaldecoratormetadataunresolvedtypeobjectinemit | spike | frontend/syntax | class: blocked | Implement Experimentaldecoratormetadataunresolvedtypeobjectinemit |
| 2245 | Implement Exportalreadyseen | spike | frontend/syntax | class: blocked | Implement Exportalreadyseen |
| 2246 | Implement Exportarraybindingpattern | spike | frontend/syntax | class: blocked | Implement Exportarraybindingpattern |
| 2247 | Implement Exportasnamespace | spike | frontend/syntax | class: blocked | Implement Exportasnamespace |
| 2248 | Implement Exportasnamespaceconflict | spike | frontend/resolver | class: blocked | Implement Exportasnamespaceconflict |
| 2249 | Implement Exportassignclassandmodule | spike | frontend/syntax | class: blocked | Implement Exportassignclassandmodule |
| 2250 | Implement Exportassignvalueandtype | spike | frontend/syntax | class: blocked | Implement Exportassignvalueandtype |
| 2251 | Implement Exportassignednamespaceisvisibleindeclarationemit | spike | frontend/syntax | class: blocked | Implement Exportassignednamespaceisvisibleindeclarationemit |
| 2252 | Implement Exportassignedtypeastypeannotation | spike | frontend/syntax | class: blocked | Implement Exportassignedtypeastypeannotation |
| 2253 | Implement Exportassignmentclass | spike | frontend/syntax | class: blocked | Implement Exportassignmentclass |
| 2254 | Implement Exportassignmentenum | spike | frontend/resolver | class: blocked | Implement Exportassignmentenum |
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
| 2265 | Implement Exportassignmentwithdeclareandexportmodifiers | spike | frontend/resolver | class: blocked | Implement Exportassignmentwithdeclareandexportmodifiers |
| 2266 | Implement Exportassignmentwithdeclaremodifier | spike | frontend/resolver | class: blocked | Implement Exportassignmentwithdeclaremodifier |
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
| 2348 | Implement Expr | spike | frontend/syntax | class: blocked | Implement Expr |
| 2349 | Implement Expressiontypenodeshoulderror | spike | frontend/syntax | class: triage-needed | Implement Expressiontypenodeshoulderror |
| 2350 | Implement Expressionwithjsdoctypearguments | spike | frontend/syntax | class: blocked | Implement Expressionwithjsdoctypearguments |
| 2351 | Implement Expressionsforbiddeninparameterinitializers | spike | frontend/syntax | class: blocked | Implement Expressionsforbiddeninparameterinitializers |
| 2352 | Implement Extbaseclass | spike | frontend/syntax | class: blocked | Implement Extbaseclass |
| 2353 | Implement Extendandimplementthesamebasetype | spike | frontend/syntax | class: blocked | Implement Extendandimplementthesamebasetype |
| 2354 | Implement Extendarray | spike | frontend/syntax | class: blocked | Implement Extendarray |
| 2355 | Implement Extendconstructsignatureininterface | spike | frontend/syntax | class: blocked | Implement Extendconstructsignatureininterface |
| 2356 | Implement Extendfromany | spike | frontend/syntax | class: blocked | Implement Extendfromany |
| 2357 | Implement Extendgenericarray | spike | frontend/syntax | class: blocked | Implement Extendgenericarray |
| 2358 | Implement Extendglobalthis Import Export | spike | frontend/syntax | class: blocked | Implement Extendglobalthis Import Export |
| 2359 | Implement Extendglobalthis Parser Syntax | spike | frontend/syntax | class: blocked | Implement Extendglobalthis Parser Syntax |
| 2360 | Implement Extendnonclasssymbol Class | spike | frontend/syntax | class: blocked | Implement Extendnonclasssymbol Class |
| 2361 | Implement Extendnonclasssymbol Name Resolution | spike | frontend/resolver | class: blocked | Implement Extendnonclasssymbol Name Resolution |
| 2362 | Implement Extendprivateconstructorclass | spike | frontend/syntax | class: blocked | Implement Extendprivateconstructorclass |
| 2363 | Implement Extendedinterfacegenerictype | spike | frontend/syntax | class: blocked | Implement Extendedinterfacegenerictype |
| 2364 | Implement Extendedunicodeplaneidentifiers | spike | frontend/syntax | class: blocked | Implement Extendedunicodeplaneidentifiers |
| 2365 | Implement Extendedunicodeplaneidentifiersjsdoc | spike | frontend/syntax | class: blocked | Implement Extendedunicodeplaneidentifiersjsdoc |
| 2366 | Implement Extendingclassfromaliasandusageinindexer | spike | frontend/syntax | class: blocked | Implement Extendingclassfromaliasandusageinindexer |
| 2367 | Implement Extendingcollectionswithcheckjs | spike | frontend/syntax | class: blocked | Implement Extendingcollectionswithcheckjs |
| 2368 | Implement Extendsclausealreadyseen | spike | frontend/syntax | class: blocked | Implement Extendsclausealreadyseen |
| 2369 | Implement Extendsuntypedmodule | spike | frontend/syntax | class: blocked | Implement Extendsuntypedmodule |
| 2370 | Implement Extension | spike | frontend/syntax | class: blocked | Implement Extension |
| 2371 | Implement Externmodule | spike | frontend/syntax | class: blocked | Implement Externmodule |
| 2372 | Implement Externmoduleclobber | spike | frontend/syntax | class: blocked | Implement Externmoduleclobber |
| 2373 | Implement Externsemantics | spike | frontend/syntax | class: blocked | Implement Externsemantics |
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
| 2384 | Implement Extractinferenceimprovement | spike | frontend/syntax | class: blocked | Implement Extractinferenceimprovement |
| 2385 | Implement Fakeinfinity | spike | frontend/syntax | class: blocked | Implement Fakeinfinity |
| 2386 | Implement Fallfromlastcase | spike | frontend/resolver | class: blocked | Implement Fallfromlastcase |
| 2387 | Implement Fallbacktobindingpatternfortypeinference | spike | frontend/resolver | class: blocked | Implement Fallbacktobindingpatternfortypeinference |
| 2388 | Implement Fatarrowself | spike | frontend/syntax | class: blocked | Implement Fatarrowself |
| 2389 | Implement Fatarrowfunctionastype | spike | frontend/syntax | class: blocked | Implement Fatarrowfunctionastype |
| 2390 | Implement Fatarrowfunctions | spike | frontend/syntax | class: blocked | Implement Fatarrowfunctions |
| 2391 | Implement Fatarrowfunctionserrors | spike | runtime/builtins | class: blocked | Implement Fatarrowfunctionserrors |
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
| 2403 | Implement Firstmatchregexpmatcharray | spike | runtime/builtins | class: blocked | Implement Firstmatchregexpmatcharray |
| 2404 | Implement Fixcrashaliaslookupfordefauledimport | spike | frontend/syntax | class: blocked | Implement Fixcrashaliaslookupfordefauledimport |
| 2405 | Implement Fixingtypeparametersrepeatedly Duplicate Local | spike | reference/triage | class: triage-needed | Implement Fixingtypeparametersrepeatedly Duplicate Local |
| 2406 | Implement Fixingtypeparametersrepeatedly Name Resolution | spike | frontend/resolver | class: blocked | Implement Fixingtypeparametersrepeatedly Name Resolution |
| 2407 | Implement Flatarraynoexcessivestackdepth | spike | frontend/syntax | class: blocked | Implement Flatarraynoexcessivestackdepth |
| 2408 | Implement Flowafterfinally | spike | frontend/semantics | class: blocked | Implement Flowafterfinally |
| 2409 | Implement Flowcontroltypeguardthenswitch | spike | frontend/semantics | class: blocked | Implement Flowcontroltypeguardthenswitch |
| 2410 | Implement For | spike | frontend/syntax | class: triage-needed | Implement For |
| 2411 | Implement Forawaitforintersection | spike | reference/triage | class: triage-needed | Implement Forawaitforintersection |
| 2412 | Implement Forawaitforunion | spike | reference/triage | class: triage-needed | Implement Forawaitforunion |
| 2413 | Implement Forin | spike | frontend/syntax | class: blocked | Implement Forin |
| 2414 | Implement Forinmodule | spike | frontend/syntax | class: blocked | Implement Forinmodule |
| 2415 | Implement Forinstatement Duplicate Local | spike | reference/triage | class: triage-needed | Implement Forinstatement Duplicate Local |
| 2416 | Implement Forinstatement Name Resolution | spike | frontend/resolver | class: blocked | Implement Forinstatement Name Resolution |
| 2417 | Implement Forinstatement Parser Syntax | spike | frontend/syntax | class: blocked | Implement Forinstatement Parser Syntax |
| 2418 | Implement Forloopendingmultilinecomments | spike | frontend/syntax | class: blocked | Implement Forloopendingmultilinecomments |
| 2419 | Implement Forloopwithdestructuringdoesnotelidefollowingstatement | spike | frontend/syntax | class: blocked | Implement Forloopwithdestructuringdoesnotelidefollowingstatement |
| 2420 | Implement Forofstringconstituents | spike | frontend/syntax | class: blocked | Implement Forofstringconstituents |
| 2421 | Implement Foroftransformsexpression | spike | reference/triage | class: blocked | Implement Foroftransformsexpression |
| 2422 | Implement Forstatementinnercomments | spike | frontend/syntax | class: blocked | Implement Forstatementinnercomments |
| 2423 | Implement Formattopartsfractionalsecond | spike | frontend/resolver | class: blocked | Implement Formattopartsfractionalsecond |
| 2424 | Implement Forwarddeclaredcommontypes | spike | frontend/resolver | class: blocked | Implement Forwarddeclaredcommontypes |
| 2425 | Implement Forwardrefinclassproperties | spike | frontend/syntax | class: blocked | Implement Forwardrefinclassproperties |
| 2426 | Implement Forwardrefinenum | spike | frontend/syntax | class: blocked | Implement Forwardrefinenum |
| 2427 | Implement Forwardrefintypedeclaration | spike | frontend/syntax | class: blocked | Implement Forwardrefintypedeclaration |
| 2428 | Implement Freshliteralinference | spike | frontend/resolver | class: blocked | Implement Freshliteralinference |
| 2429 | Implement Freshliteraltypesinintersections | spike | frontend/resolver | class: blocked | Implement Freshliteraltypesinintersections |
| 2430 | Implement Funclodule | spike | frontend/syntax | class: blocked | Implement Funclodule |
| 2431 | Implement Funcdecl | spike | frontend/syntax | class: blocked | Implement Funcdecl |
| 2432 | Implement Functionandimportnameconflict | spike | frontend/syntax | class: blocked | Implement Functionandimportnameconflict |
| 2433 | Implement Functionandinterfacewithseparateerrors | spike | runtime/builtins | class: blocked | Implement Functionandinterfacewithseparateerrors |
| 2434 | Implement Functionandpropertynameconflict | spike | frontend/resolver | class: blocked | Implement Functionandpropertynameconflict |
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
| 2445 | Implement Functionexpressioninwithblock | spike | frontend/syntax | class: blocked | Implement Functionexpressioninwithblock |
| 2446 | Implement Functionexpressionnames | spike | frontend/syntax | class: blocked | Implement Functionexpressionnames |
| 2447 | Implement Functionexpressionshadowedbyparams | spike | reference/triage | class: triage-needed | Implement Functionexpressionshadowedbyparams |
| 2448 | Implement Functionexpressionwithresolutionoftypenamedarguments | spike | frontend/syntax | class: blocked | Implement Functionexpressionwithresolutionoftypenamedarguments |
| 2449 | Implement Functionexpressionwithresolutionoftypeofsamename | spike | frontend/syntax | class: blocked | Implement Functionexpressionwithresolutionoftypeofsamename |
| 2450 | Implement Functioninifstatementinmodule | spike | frontend/syntax | class: blocked | Implement Functioninifstatementinmodule |
| 2451 | Implement Functionlikeinparameterinitializer | spike | frontend/syntax | class: blocked | Implement Functionlikeinparameterinitializer |
| 2452 | Implement Functionmergedwithmodule | spike | frontend/syntax | class: blocked | Implement Functionmergedwithmodule |
| 2453 | Implement Functionoverloadambiguity | spike | frontend/semantics | class: blocked | Implement Functionoverloadambiguity |
| 2454 | Implement Functionoverloadimplementationofwrongname | spike | frontend/semantics | class: blocked | Implement Functionoverloadimplementationofwrongname |
| 2455 | Implement Functionoverloads Name Resolution | spike | frontend/resolver | class: blocked | Implement Functionoverloads Name Resolution |
| 2456 | Implement Functionoverloads Parser Syntax | spike | frontend/semantics | class: blocked | Implement Functionoverloads Parser Syntax |
| 2457 | Implement Functionoverloads Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Functionoverloads Unknown Unsupported |
| 2458 | Implement Functionoverloadsoutoforder | spike | frontend/semantics | class: blocked | Implement Functionoverloadsoutoforder |
| 2459 | Implement Functionoverloadsrecursivegenericreturntype | spike | frontend/syntax | class: blocked | Implement Functionoverloadsrecursivegenericreturntype |
| 2460 | Implement Functionparameteraritymismatch | spike | frontend/resolver | class: blocked | Implement Functionparameteraritymismatch |
| 2461 | Implement Functionreturningitself | spike | frontend/resolver | class: blocked | Implement Functionreturningitself |
| 2462 | Implement Functionsignatureassignmentcompat | spike | frontend/resolver | class: blocked | Implement Functionsignatureassignmentcompat |
| 2463 | Implement Functionsubtypingofvarargs | spike | frontend/syntax | class: blocked | Implement Functionsubtypingofvarargs |
| 2464 | Implement Functiontofunctionwithproperror | spike | runtime/builtins | class: blocked | Implement Functiontofunctionwithproperror |
| 2465 | Implement Functiontype | spike | reference/triage | class: triage-needed | Implement Functiontype |
| 2466 | Implement Functiontypeargumentarityerrors | spike | frontend/semantics | class: blocked | Implement Functiontypeargumentarityerrors |
| 2467 | Implement Functiontypeargumentarrayassignment | spike | frontend/syntax | class: blocked | Implement Functiontypeargumentarrayassignment |
| 2468 | Implement Functiontypeargumentassignmentcompat | spike | frontend/resolver | class: blocked | Implement Functiontypeargumentassignmentcompat |
| 2469 | Implement Functiontypeslackingreturntypes | spike | frontend/syntax | class: blocked | Implement Functiontypeslackingreturntypes |
| 2470 | Implement Functionwithdefaultparameterwithnostatements | spike | frontend/resolver | class: blocked | Implement Functionwithdefaultparameterwithnostatements |
| 2471 | Implement Functionwithsamenameasfield | spike | frontend/syntax | class: blocked | Implement Functionwithsamenameasfield |
| 2472 | Implement Functionsinclassexpressions | spike | frontend/syntax | class: blocked | Implement Functionsinclassexpressions |
| 2473 | Implement Functionsmissingreturnstatementsandexpressions | spike | frontend/syntax | class: blocked | Implement Functionsmissingreturnstatementsandexpressions |
| 2474 | Implement Functionsmissingreturnstatementsandexpressionsstrictnullchecks | spike | frontend/syntax | class: blocked | Implement Functionsmissingreturnstatementsandexpressionsstrictnullchecks |
| 2475 | Implement Functionswithimplicitreturntypeassignabletoundefined | spike | frontend/syntax | class: blocked | Implement Functionswithimplicitreturntypeassignabletoundefined |
| 2476 | Implement Functionswithmodifiersinblocks | spike | frontend/syntax | class: blocked | Implement Functionswithmodifiersinblocks |
| 2477 | Implement Funduleexportedclassisusedbeforedeclaration | spike | frontend/syntax | class: blocked | Implement Funduleexportedclassisusedbeforedeclaration |
| 2478 | Implement Funduleoffunctionwithoutreturntypeannotation | spike | frontend/syntax | class: blocked | Implement Funduleoffunctionwithoutreturntypeannotation |
| 2479 | Implement Fundulesplitacrossfiles | spike | frontend/syntax | class: blocked | Implement Fundulesplitacrossfiles |
| 2480 | Implement Funduleusedacrossfileboundary | spike | frontend/syntax | class: blocked | Implement Funduleusedacrossfileboundary |
| 2481 | Implement Fuzzy | spike | frontend/syntax | class: blocked | Implement Fuzzy |
| 2482 | Implement Generativerecursionwithtypeof | spike | frontend/syntax | class: blocked | Implement Generativerecursionwithtypeof |
| 2483 | Implement Generatores Import Export | spike | frontend/syntax | class: blocked | Implement Generatores Import Export |
| 2484 | Implement Generatores Parser Syntax | spike | runtime/builtins | class: blocked | Implement Generatores Parser Syntax |
| 2485 | Implement Generatortransformfinallabel | spike | reference/triage | class: triage-needed | Implement Generatortransformfinallabel |
| 2486 | Implement Genericandnongenericoverload | spike | frontend/resolver | class: blocked | Implement Genericandnongenericoverload |
| 2487 | Implement Genericargumentcallsigassignmentcompat | spike | frontend/syntax | class: blocked | Implement Genericargumentcallsigassignmentcompat |
| 2488 | Implement Genericarrayassignment | spike | frontend/resolver | class: blocked | Implement Genericarrayassignment |
| 2489 | Implement Genericarrayassignmentcompaterrors | spike | frontend/resolver | class: blocked | Implement Genericarrayassignmentcompaterrors |
| 2490 | Implement Genericarrayextenstions | spike | frontend/syntax | class: blocked | Implement Genericarrayextenstions |
| 2491 | Implement Genericarraywithouttypeannotation | spike | frontend/syntax | class: blocked | Implement Genericarraywithouttypeannotation |
| 2492 | Implement Genericassignmentcompatoffunctionsignatures | spike | frontend/syntax | class: blocked | Implement Genericassignmentcompatoffunctionsignatures |
| 2493 | Implement Genericassignmentcompatwithinterfaces | spike | frontend/syntax | class: blocked | Implement Genericassignmentcompatwithinterfaces |
| 2494 | Implement Genericbaseclassliteralproperty | spike | frontend/syntax | class: blocked | Implement Genericbaseclassliteralproperty |
| 2495 | Implement Genericcallatyieldexpressioningenericcall | spike | frontend/syntax | class: blocked | Implement Genericcallatyieldexpressioningenericcall |
| 2496 | Implement Genericcallinferenceconditionaltype Name Resolution | spike | frontend/resolver | class: blocked | Implement Genericcallinferenceconditionaltype Name Resolution |
| 2497 | Implement Genericcallinferenceconditionaltype Type System | spike | frontend/syntax | class: blocked | Implement Genericcallinferenceconditionaltype Type System |
| 2498 | Implement Genericcallinferenceinconditionaltypes | spike | frontend/syntax | class: blocked | Implement Genericcallinferenceinconditionaltypes |
| 2499 | Implement Genericcallinferenceusingthistypenoinvalidcachereuseaftermappedtypeapplication | spike | frontend/semantics | class: blocked | Implement Genericcallinferenceusingthistypenoinvalidcachereuseaftermappedtypeapplication |
| 2500 | Implement Genericcallinferencewithgenericlocalfunction | spike | frontend/syntax | class: blocked | Implement Genericcallinferencewithgenericlocalfunction |
| 2501 | Implement Genericcallonmemberreturningclosedoverobject | spike | frontend/syntax | class: blocked | Implement Genericcallonmemberreturningclosedoverobject |
| 2502 | Implement Genericcallspecializedtotypearg | spike | frontend/syntax | class: blocked | Implement Genericcallspecializedtotypearg |
| 2503 | Implement Genericcallwithinownbodycasttypeparameteridentity | spike | frontend/syntax | class: blocked | Implement Genericcallwithinownbodycasttypeparameteridentity |
| 2504 | Implement Genericcallwithoutargs | spike | frontend/syntax | class: blocked | Implement Genericcallwithoutargs |
| 2505 | Implement Genericcallbackinvokedinsideitscontainingfunction | spike | frontend/resolver | class: blocked | Implement Genericcallbackinvokedinsideitscontainingfunction |
| 2506 | Implement Genericcallbacksandclasshierarchy | spike | frontend/syntax | class: blocked | Implement Genericcallbacksandclasshierarchy |
| 2507 | Implement Genericcapturingfunctionnarrowing | spike | frontend/resolver | class: blocked | Implement Genericcapturingfunctionnarrowing |
| 2508 | Implement Genericchainedcalls | spike | frontend/syntax | class: blocked | Implement Genericchainedcalls |
| 2509 | Implement Genericclassimplementinggenericinterfacefromanothermodule | spike | frontend/syntax | class: blocked | Implement Genericclassimplementinggenericinterfacefromanothermodule |
| 2510 | Implement Genericclassinheritsconstructorfromnongenericclass | spike | frontend/syntax | class: blocked | Implement Genericclassinheritsconstructorfromnongenericclass |
| 2511 | Implement Genericclasspropertyinheritancespecialization | spike | frontend/syntax | class: blocked | Implement Genericclasspropertyinheritancespecialization |
| 2512 | Implement Genericclassstaticmethod | spike | frontend/syntax | class: blocked | Implement Genericclassstaticmethod |
| 2513 | Implement Genericclasswithstaticfactory | spike | frontend/syntax | class: blocked | Implement Genericclasswithstaticfactory |
| 2514 | Implement Genericclasswithstaticsusingtypearguments | spike | frontend/syntax | class: blocked | Implement Genericclasswithstaticsusingtypearguments |
| 2515 | Implement Genericclasses | spike | frontend/syntax | class: blocked | Implement Genericclasses |
| 2516 | Implement Genericclassesinmodule | spike | frontend/syntax | class: blocked | Implement Genericclassesinmodule |
| 2517 | Implement Genericclassesredeclaration | spike | frontend/syntax | class: blocked | Implement Genericclassesredeclaration |
| 2518 | Implement Genericcloduleinmodule | spike | frontend/syntax | class: blocked | Implement Genericcloduleinmodule |
| 2519 | Implement Genericclonereturntypes | spike | frontend/syntax | class: blocked | Implement Genericclonereturntypes |
| 2520 | Implement Genericcombinators | spike | frontend/semantics | class: blocked | Implement Genericcombinators |
| 2521 | Implement Genericconditionalconstrainedtounknownnotassignabletoconcreteobject | spike | frontend/resolver | class: blocked | Implement Genericconditionalconstrainedtounknownnotassignabletoconcreteobject |
| 2522 | Implement Genericconstraint | spike | frontend/syntax | class: blocked | Implement Genericconstraint |
| 2523 | Implement Genericconstraintdeclaration | spike | frontend/syntax | class: blocked | Implement Genericconstraintdeclaration |
| 2524 | Implement Genericconstraintonextendedbuiltintypes | spike | frontend/syntax | class: blocked | Implement Genericconstraintonextendedbuiltintypes |
| 2525 | Implement Genericconstraintsatisfaction | spike | frontend/semantics | class: blocked | Implement Genericconstraintsatisfaction |
| 2526 | Implement Genericconstructinvocationwithnotypearg | spike | frontend/resolver | class: blocked | Implement Genericconstructinvocationwithnotypearg |
| 2527 | Implement Genericconstructsignatureininterface | spike | frontend/syntax | class: blocked | Implement Genericconstructsignatureininterface |
| 2528 | Implement Genericconstructorfunction | spike | frontend/syntax | class: blocked | Implement Genericconstructorfunction |
| 2529 | Implement Genericcontextualtypingspecialization | spike | frontend/syntax | class: blocked | Implement Genericcontextualtypingspecialization |
| 2530 | Implement Genericdefaults | spike | frontend/syntax | class: blocked | Implement Genericdefaults |
| 2531 | Implement Genericdefaultserrors | spike | frontend/syntax | class: blocked | Implement Genericdefaultserrors |
| 2532 | Implement Genericdefaultsjs | spike | frontend/syntax | class: blocked | Implement Genericdefaultsjs |
| 2533 | Implement Genericderivedtypewithspecializedbase | spike | frontend/syntax | class: blocked | Implement Genericderivedtypewithspecializedbase |
| 2534 | Implement Genericfunctioncallsignaturereturntypemismatch | spike | frontend/resolver | class: blocked | Implement Genericfunctioncallsignaturereturntypemismatch |
| 2535 | Implement Genericfunctionhasfreshtypeargs | spike | frontend/syntax | class: blocked | Implement Genericfunctionhasfreshtypeargs |
| 2536 | Implement Genericfunctioninference | spike | frontend/syntax | class: blocked | Implement Genericfunctioninference |
| 2537 | Implement Genericfunctionspecializations | spike | frontend/syntax | class: blocked | Implement Genericfunctionspecializations |
| 2538 | Implement Genericfunctiontypedargumentsarefixed | spike | frontend/resolver | class: blocked | Implement Genericfunctiontypedargumentsarefixed |
| 2539 | Implement Genericfunctions | spike | frontend/resolver | class: blocked | Implement Genericfunctions |
| 2540 | Implement Genericfunctionsandconditionalinference | spike | frontend/syntax | class: blocked | Implement Genericfunctionsandconditionalinference |
| 2541 | Implement Genericfunctionsnotcontextsensitive | spike | frontend/syntax | class: blocked | Implement Genericfunctionsnotcontextsensitive |
| 2542 | Implement Genericfunctionswithoptionalparameters Name Resolution | spike | frontend/resolver | class: blocked | Implement Genericfunctionswithoptionalparameters Name Resolution |
| 2543 | Implement Genericfunctionswithoptionalparameters Type System | spike | frontend/syntax | class: blocked | Implement Genericfunctionswithoptionalparameters Type System |
| 2544 | Implement Genericfunduleinmodule | spike | frontend/syntax | class: blocked | Implement Genericfunduleinmodule |
| 2545 | Implement Genericgetter | spike | frontend/syntax | class: blocked | Implement Genericgetter |
| 2546 | Implement Genericimplements | spike | frontend/syntax | class: blocked | Implement Genericimplements |
| 2547 | Implement Genericindexedaccessmethodintersectioncanbeaccessed | spike | frontend/syntax | class: blocked | Implement Genericindexedaccessmethodintersectioncanbeaccessed |
| 2548 | Implement Genericindexedaccessvariancecomparisonresultcorrect | spike | frontend/resolver | class: blocked | Implement Genericindexedaccessvariancecomparisonresultcorrect |
| 2549 | Implement Genericinference | spike | frontend/syntax | class: blocked | Implement Genericinference |
| 2550 | Implement Genericinferencedefaulttypeparameter | spike | frontend/resolver | class: blocked | Implement Genericinferencedefaulttypeparameter |
| 2551 | Implement Genericinheriteddefaultconstructors | spike | frontend/syntax | class: blocked | Implement Genericinheriteddefaultconstructors |
| 2552 | Implement Genericinstanceof | spike | frontend/syntax | class: blocked | Implement Genericinstanceof |
| 2553 | Implement Genericinterfacefunctiontypeparameter | spike | frontend/syntax | class: blocked | Implement Genericinterfacefunctiontypeparameter |
| 2554 | Implement Genericinterfaceimplementation | spike | frontend/syntax | class: blocked | Implement Genericinterfaceimplementation |
| 2555 | Implement Genericinterfacetypecall | spike | frontend/resolver | class: blocked | Implement Genericinterfacetypecall |
| 2556 | Implement Genericinterfaceswithouttypearguments | spike | frontend/syntax | class: blocked | Implement Genericinterfaceswithouttypearguments |
| 2557 | Implement Genericisneveremptyobject | spike | reference/triage | class: triage-needed | Implement Genericisneveremptyobject |
| 2558 | Implement Genericmemberfunction | spike | frontend/syntax | class: blocked | Implement Genericmemberfunction |
| 2559 | Implement Genericmergeddeclarationusingtypeparameter Import Export | spike | frontend/syntax | class: blocked | Implement Genericmergeddeclarationusingtypeparameter Import Export |
| 2560 | Implement Genericmergeddeclarationusingtypeparameter Type System | spike | frontend/syntax | class: blocked | Implement Genericmergeddeclarationusingtypeparameter Type System |
| 2561 | Implement Genericmethodoverspecialization | spike | frontend/resolver | class: blocked | Implement Genericmethodoverspecialization |
| 2562 | Implement Genericnewinterface | spike | frontend/syntax | class: blocked | Implement Genericnewinterface |
| 2563 | Implement Genericobjectcreationwithouttypeargs | spike | frontend/syntax | class: blocked | Implement Genericobjectcreationwithouttypeargs |
| 2564 | Implement Genericobjectlitreturntype | spike | frontend/syntax | class: blocked | Implement Genericobjectlitreturntype |
| 2565 | Implement Genericobjectspreadresultinswitch | spike | frontend/syntax | class: blocked | Implement Genericobjectspreadresultinswitch |
| 2566 | Implement Genericofacloduletype | spike | frontend/syntax | class: blocked | Implement Genericofacloduletype |
| 2567 | Implement Genericoverloadsignatures | spike | frontend/syntax | class: blocked | Implement Genericoverloadsignatures |
| 2568 | Implement Genericparameterassignability | spike | frontend/syntax | class: blocked | Implement Genericparameterassignability |
| 2569 | Implement Genericprototypeproperty | spike | frontend/syntax | class: blocked | Implement Genericprototypeproperty |
| 2570 | Implement Genericrecursiveimplicitconstructorerrors | spike | frontend/syntax | class: blocked | Implement Genericrecursiveimplicitconstructorerrors |
| 2571 | Implement Genericreduce | spike | frontend/syntax | class: blocked | Implement Genericreduce |
| 2572 | Implement Genericreturntypefromgetter | spike | frontend/syntax | class: blocked | Implement Genericreturntypefromgetter |
| 2573 | Implement Genericreversingtypeparameters | spike | frontend/syntax | class: blocked | Implement Genericreversingtypeparameters |
| 2574 | Implement Genericsignatureidentity | spike | reference/triage | class: triage-needed | Implement Genericsignatureidentity |
| 2575 | Implement Genericspecializations | spike | frontend/syntax | class: blocked | Implement Genericspecializations |
| 2576 | Implement Genericstaticanytypefunction | spike | frontend/syntax | class: blocked | Implement Genericstaticanytypefunction |
| 2577 | Implement Generictemplateoverloadresolution | spike | frontend/resolver | class: blocked | Implement Generictemplateoverloadresolution |
| 2578 | Implement Generictuplewithsimplifiableelements | spike | frontend/syntax | class: blocked | Implement Generictuplewithsimplifiableelements |
| 2579 | Implement Generictypeargumentinference | spike | frontend/syntax | class: blocked | Implement Generictypeargumentinference |
| 2580 | Implement Generictypeassertions | spike | frontend/syntax | class: blocked | Implement Generictypeassertions |
| 2581 | Implement Generictypeconstraints | spike | frontend/syntax | class: blocked | Implement Generictypeconstraints |
| 2582 | Implement Generictypeparameterequivalence | spike | frontend/syntax | class: blocked | Implement Generictypeparameterequivalence |
| 2583 | Implement Generictypereferencesrequiretypeargs | spike | frontend/syntax | class: blocked | Implement Generictypereferencesrequiretypeargs |
| 2584 | Implement Generictypeusedwithouttypearguments | spike | frontend/syntax | class: blocked | Implement Generictypeusedwithouttypearguments |
| 2585 | Implement Generictypewithcallablemembers | spike | frontend/syntax | class: blocked | Implement Generictypewithcallablemembers |
| 2586 | Implement Generictypewithmultiplebases | spike | frontend/syntax | class: blocked | Implement Generictypewithmultiplebases |
| 2587 | Implement Generictypewithnongenericbasemismatch | spike | frontend/syntax | class: blocked | Implement Generictypewithnongenericbasemismatch |
| 2588 | Implement Genericunboundedtypeparamassignability | spike | frontend/syntax | class: blocked | Implement Genericunboundedtypeparamassignability |
| 2589 | Implement Genericwithcallsignaturereturningspecialization | spike | frontend/syntax | class: blocked | Implement Genericwithcallsignaturereturningspecialization |
| 2590 | Implement Genericwithcallsignatures | spike | frontend/syntax | class: blocked | Implement Genericwithcallsignatures |
| 2591 | Implement Genericwithindexeroftypeparametertype Import Export | spike | frontend/syntax | class: blocked | Implement Genericwithindexeroftypeparametertype Import Export |
| 2592 | Implement Genericwithindexeroftypeparametertype Type System | spike | frontend/syntax | class: blocked | Implement Genericwithindexeroftypeparametertype Type System |
| 2593 | Implement Genericwithopentypeparameters | spike | frontend/syntax | class: blocked | Implement Genericwithopentypeparameters |
| 2594 | Implement Generics | spike | frontend/syntax | class: blocked | Implement Generics |
| 2595 | Implement Genericsandhigherorderfunctions | spike | frontend/syntax | class: blocked | Implement Genericsandhigherorderfunctions |
| 2596 | Implement Genericswithduplicatetypeparameters | spike | frontend/syntax | class: blocked | Implement Genericswithduplicatetypeparameters |
| 2597 | Implement Genericswithouttypeparameters | spike | frontend/syntax | class: blocked | Implement Genericswithouttypeparameters |
| 2598 | Implement Getaccessorwithimpliedreturntypeandfunctionclassmerge | spike | frontend/syntax | class: blocked | Implement Getaccessorwithimpliedreturntypeandfunctionclassmerge |
| 2599 | Implement Getandsetasmembernames | spike | frontend/syntax | class: blocked | Implement Getandsetasmembernames |
| 2600 | Implement Getandsetnotidenticaltype Duplicate Function | spike | reference/triage | class: triage-needed | Implement Getandsetnotidenticaltype Duplicate Function |
| 2601 | Implement Getandsetnotidenticaltype Parser Syntax | spike | frontend/syntax | class: blocked | Implement Getandsetnotidenticaltype Parser Syntax |
| 2602 | Implement Getparameternameatposition | spike | frontend/resolver | class: blocked | Implement Getparameternameatposition |
| 2603 | Implement Getsetenumerable | spike | frontend/syntax | class: blocked | Implement Getsetenumerable |
| 2604 | Implement Getsetreturntypes | spike | frontend/syntax | class: blocked | Implement Getsetreturntypes |
| 2605 | Implement Gettercontrolflowstrictnull | spike | frontend/semantics | class: blocked | Implement Gettercontrolflowstrictnull |
| 2606 | Implement Gettermissingreturnerror | spike | frontend/semantics | class: blocked | Implement Gettermissingreturnerror |
| 2607 | Implement Gettersetternonaccessor | spike | frontend/syntax | class: blocked | Implement Gettersetternonaccessor |
| 2608 | Implement Gettersettersubtypeassignment | spike | frontend/semantics | class: blocked | Implement Gettersettersubtypeassignment |
| 2609 | Implement Getterthatthrowsshouldnotneedreturn | spike | frontend/semantics | class: blocked | Implement Getterthatthrowsshouldnotneedreturn |
| 2610 | Implement Gettersandsetters | spike | frontend/semantics | class: blocked | Implement Gettersandsetters |
| 2611 | Implement Gettersandsettersaccessibility | spike | frontend/semantics | class: blocked | Implement Gettersandsettersaccessibility |
| 2612 | Implement Gettersandsetterserrors | spike | frontend/semantics | class: blocked | Implement Gettersandsetterserrors |
| 2613 | Implement Gettersandsetterstypesagree | spike | frontend/semantics | class: blocked | Implement Gettersandsetterstypesagree |
| 2614 | Implement Giant | spike | frontend/syntax | class: blocked | Implement Giant |
| 2615 | Implement Global | spike | frontend/syntax | class: triage-needed | Implement Global |
| 2616 | Implement Globalfunctionaugmentationoverload | spike | frontend/syntax | class: blocked | Implement Globalfunctionaugmentationoverload |
| 2617 | Implement Globaliscontextualkeyword | spike | frontend/syntax | class: blocked | Implement Globaliscontextualkeyword |
| 2618 | Implement Globalthiscapture | spike | frontend/syntax | class: blocked | Implement Globalthiscapture |
| 2619 | Implement Globalthisdeclarationemit | spike | frontend/syntax | class: blocked | Implement Globalthisdeclarationemit |
| 2620 | Implement Grammarambiguities | spike | frontend/syntax | class: blocked | Implement Grammarambiguities |
| 2621 | Implement Heterogeneousarrayandoverloads | spike | frontend/semantics | class: blocked | Implement Heterogeneousarrayandoverloads |
| 2622 | Implement Hidingcallsignatures | spike | frontend/syntax | class: blocked | Implement Hidingcallsignatures |
| 2623 | Implement Hidingconstructsignatures | spike | frontend/syntax | class: blocked | Implement Hidingconstructsignatures |
| 2624 | Implement Higherordermappedindexlookupinference | spike | frontend/syntax | class: blocked | Implement Higherordermappedindexlookupinference |
| 2625 | Implement Homomorphicmappedtypewithnonhomomorphicinstantiationspreadable | spike | frontend/resolver | class: blocked | Implement Homomorphicmappedtypewithnonhomomorphicinstantiationspreadable |
| 2626 | Implement Hugedeclarationoutputgetstruncatedwitherror | spike | runtime/builtins | class: blocked | Implement Hugedeclarationoutputgetstruncatedwitherror |
| 2627 | Implement I | spike | frontend/resolver | class: blocked | Implement I |
| 2628 | Implement Icomparable | spike | frontend/resolver | class: blocked | Implement Icomparable |
| 2629 | Implement Identicalgenericconditionalswithinferrelated | spike | frontend/syntax | class: blocked | Implement Identicalgenericconditionalswithinferrelated |
| 2630 | Implement Identifierstartafternumericliteral | spike | frontend/syntax | class: blocked | Implement Identifierstartafternumericliteral |
| 2631 | Implement Identityanddivergentnormalizedtypes | spike | frontend/syntax | class: triage-needed | Implement Identityanddivergentnormalizedtypes |
| 2632 | Implement Identityforsignatureswithtypeparametersandany | spike | frontend/semantics | class: blocked | Implement Identityforsignatureswithtypeparametersandany |
| 2633 | Implement Identityforsignatureswithtypeparametersswitched | spike | frontend/semantics | class: blocked | Implement Identityforsignatureswithtypeparametersswitched |
| 2634 | Implement Identityrelationnevertypes | spike | frontend/syntax | class: blocked | Implement Identityrelationnevertypes |
| 2635 | Implement Ifelsewithstatements | spike | frontend/resolver | class: blocked | Implement Ifelsewithstatements |
| 2636 | Implement Illegalmodifiersonclasselements | spike | frontend/syntax | class: blocked | Implement Illegalmodifiersonclasselements |
| 2637 | Implement Illegalsupercallsinconstructor | spike | frontend/syntax | class: blocked | Implement Illegalsupercallsinconstructor |
| 2638 | Implement Implementarrayinterface | spike | frontend/syntax | class: blocked | Implement Implementarrayinterface |
| 2639 | Implement Implementclauseprecedingextends | spike | frontend/syntax | class: blocked | Implement Implementclauseprecedingextends |
| 2640 | Implement Implementgenericwithmismatchedtypes | spike | frontend/syntax | class: blocked | Implement Implementgenericwithmismatchedtypes |
| 2641 | Implement Implementinterfaceanymemberwithvoid | spike | frontend/syntax | class: blocked | Implement Implementinterfaceanymemberwithvoid |
| 2642 | Implement Implementpublicpropertyasprivate | spike | frontend/semantics | class: blocked | Implement Implementpublicpropertyasprivate |
| 2643 | Implement Implementsclausealreadyseen | spike | frontend/syntax | class: blocked | Implement Implementsclausealreadyseen |
| 2644 | Implement Implementsinclassexpression | spike | frontend/syntax | class: blocked | Implement Implementsinclassexpression |
| 2645 | Implement Implementsincorrectlynoassertion | spike | frontend/syntax | class: blocked | Implement Implementsincorrectlynoassertion |
| 2646 | Implement Implicitanyambients | spike | frontend/syntax | class: blocked | Implement Implicitanyambients |
| 2647 | Implement Implicitanyanyreturningfunction | spike | frontend/syntax | class: triage-needed | Implement Implicitanyanyreturningfunction |
| 2648 | Implement Implicitanycastedvalue | spike | frontend/syntax | class: triage-needed | Implement Implicitanycastedvalue |
| 2649 | Implement Implicitanydeclarefunctionexprwithoutformaltype | spike | frontend/syntax | class: blocked | Implement Implicitanydeclarefunctionexprwithoutformaltype |
| 2650 | Implement Implicitanydeclarememberwithouttype | spike | frontend/syntax | class: blocked | Implement Implicitanydeclarememberwithouttype |
| 2651 | Implement Implicitanydeclarevariableswithouttypeandinit | spike | frontend/resolver | class: blocked | Implement Implicitanydeclarevariableswithouttypeandinit |
| 2652 | Implement Implicitanyfromcircularinference | spike | frontend/syntax | class: blocked | Implement Implicitanyfromcircularinference |
| 2653 | Implement Implicitanyfunctioninvocationwithanyarguements | spike | frontend/syntax | class: triage-needed | Implement Implicitanyfunctioninvocationwithanyarguements |
| 2654 | Implement Implicitanygenerictypeinference | spike | frontend/syntax | class: blocked | Implement Implicitanygenerictypeinference |
| 2655 | Implement Implicitanygenerics | spike | frontend/syntax | class: blocked | Implement Implicitanygenerics |
| 2656 | Implement Implicitanygetandsetaccessorwithanyreturntype | spike | frontend/syntax | class: blocked | Implement Implicitanygetandsetaccessorwithanyreturntype |
| 2657 | Implement Implicitanyinambientdeclaration | spike | frontend/syntax | class: blocked | Implement Implicitanyinambientdeclaration |
| 2658 | Implement Implicitanyincatch | spike | frontend/syntax | class: blocked | Implement Implicitanyincatch |
| 2659 | Implement Implicitanynewexprlackconstructorsignature | spike | frontend/syntax | class: blocked | Implement Implicitanynewexprlackconstructorsignature |
| 2660 | Implement Implicitanywidentoany | spike | frontend/syntax | class: triage-needed | Implement Implicitanywidentoany |
| 2661 | Implement Implicitconstparameters | spike | frontend/syntax | class: blocked | Implement Implicitconstparameters |
| 2662 | Implement Implicitindexsignatures | spike | frontend/syntax | class: blocked | Implement Implicitindexsignatures |
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
| 2717 | Implement Importnonexportedmember Parser Syntax | spike | frontend/resolver | class: blocked | Implement Importnonexportedmember Parser Syntax |
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
| 2730 | Implement Importusedingenericimportresolves | spike | frontend/semantics | class: blocked | Implement Importusedingenericimportresolves |
| 2731 | Implement Importwithtrailingslash | spike | frontend/syntax | class: blocked | Implement Importwithtrailingslash |
| 2732 | Implement Importedaliasedconditionaltypeinstantiation | spike | frontend/syntax | class: blocked | Implement Importedaliasedconditionaltypeinstantiation |
| 2733 | Implement Importedaliasesintypepositions | spike | frontend/syntax | class: blocked | Implement Importedaliasesintypepositions |
| 2734 | Implement Importedenummembermergedwithexportedaliasiserror | spike | frontend/syntax | class: blocked | Implement Importedenummembermergedwithexportedaliasiserror |
| 2735 | Implement Importedmoduleaddtoglobal | spike | frontend/syntax | class: blocked | Implement Importedmoduleaddtoglobal |
| 2736 | Implement Importedmoduleclassnameclash | spike | frontend/syntax | class: blocked | Implement Importedmoduleclassnameclash |
| 2737 | Implement Importsinambientmodules | spike | frontend/syntax | class: blocked | Implement Importsinambientmodules |
| 2738 | Implement Indoesnotoperateonprimitivetypes | spike | frontend/syntax | class: triage-needed | Implement Indoesnotoperateonprimitivetypes |
| 2739 | Implement Inkeywordandintersection | spike | frontend/semantics | class: blocked | Implement Inkeywordandintersection |
| 2740 | Implement Inkeywordandunknown | spike | frontend/syntax | class: blocked | Implement Inkeywordandunknown |
| 2741 | Implement Inkeywordnarrowingwithnouncheckedindexedaccess | spike | frontend/semantics | class: blocked | Implement Inkeywordnarrowingwithnouncheckedindexedaccess |
| 2742 | Implement Inkeywordtypeguard | spike | frontend/semantics | class: blocked | Implement Inkeywordtypeguard |
| 2743 | Implement Inoperator | spike | frontend/resolver | class: blocked | Implement Inoperator |
| 2744 | Implement Inoperatorwithfunction | spike | frontend/syntax | class: blocked | Implement Inoperatorwithfunction |
| 2745 | Implement Inoperatorwithgeneric | spike | frontend/syntax | class: blocked | Implement Inoperatorwithgeneric |
| 2746 | Implement Incompatibleassignmentofidenticallynamedtypes | spike | frontend/semantics | class: blocked | Implement Incompatibleassignmentofidenticallynamedtypes |
| 2747 | Implement Incompatibleexports | spike | frontend/syntax | class: blocked | Implement Incompatibleexports |
| 2748 | Implement Incompatibletypes | spike | frontend/syntax | class: blocked | Implement Incompatibletypes |
| 2749 | Implement Incompletedottedexpressionateof | spike | frontend/syntax | class: blocked | Implement Incompletedottedexpressionateof |
| 2750 | Implement Incompleteobjectliteral | spike | frontend/syntax | class: blocked | Implement Incompleteobjectliteral |
| 2751 | Implement Incorrectclassoverloadchain | spike | frontend/semantics | class: blocked | Implement Incorrectclassoverloadchain |
| 2752 | Implement Incorrectnumberoftypeargumentsduringerrorreporting | spike | frontend/syntax | class: blocked | Implement Incorrectnumberoftypeargumentsduringerrorreporting |
| 2753 | Implement Incrementonnullassertion | spike | frontend/syntax | class: blocked | Implement Incrementonnullassertion |
| 2754 | Implement Incrementontypeparameter | spike | frontend/semantics | class: blocked | Implement Incrementontypeparameter |
| 2755 | Implement Indexat | spike | frontend/resolver | class: blocked | Implement Indexat |
| 2756 | Implement Indexintoarraysubclass | spike | frontend/resolver | class: blocked | Implement Indexintoarraysubclass |
| 2757 | Implement Indexintoenum | spike | frontend/syntax | class: blocked | Implement Indexintoenum |
| 2758 | Implement Indexsignatureandmappedtype | spike | frontend/semantics | class: blocked | Implement Indexsignatureandmappedtype |
| 2759 | Implement Indexsignatureinotherfile | spike | frontend/syntax | class: blocked | Implement Indexsignatureinotherfile |
| 2760 | Implement Indexsignaturemusthavetypeannotation | spike | frontend/syntax | class: blocked | Implement Indexsignaturemusthavetypeannotation |
| 2761 | Implement Indexsignatureoftypeunknownstillrequiresindexsignature | spike | frontend/resolver | class: blocked | Implement Indexsignatureoftypeunknownstillrequiresindexsignature |
| 2762 | Implement Indexsignaturetypecheck | spike | frontend/syntax | class: blocked | Implement Indexsignaturetypecheck |
| 2763 | Implement Indexsignaturewithaccessibilitymodifier | spike | frontend/syntax | class: blocked | Implement Indexsignaturewithaccessibilitymodifier |
| 2764 | Implement Indexsignaturewithinitializer | spike | frontend/syntax | class: blocked | Implement Indexsignaturewithinitializer |
| 2765 | Implement Indexsignaturewithtrailingcomma | spike | frontend/syntax | class: blocked | Implement Indexsignaturewithtrailingcomma |
| 2766 | Implement Indexsignaturewithouttypeannotation | spike | frontend/syntax | class: blocked | Implement Indexsignaturewithouttypeannotation |
| 2767 | Implement Indexsignaturesinferentialtyping | spike | frontend/syntax | class: blocked | Implement Indexsignaturesinferentialtyping |
| 2768 | Implement Indextypecheck | spike | frontend/syntax | class: triage-needed | Implement Indextypecheck |
| 2769 | Implement Indextypenosubstitutiontemplateliteral | spike | frontend/resolver | class: blocked | Implement Indextypenosubstitutiontemplateliteral |
| 2770 | Implement Indexwithoutparamtype | spike | frontend/syntax | class: blocked | Implement Indexwithoutparamtype |
| 2771 | Implement Indexedaccessandnullablenarrowing | spike | frontend/syntax | class: blocked | Implement Indexedaccessandnullablenarrowing |
| 2772 | Implement Indexedaccesscanbehighorder | spike | frontend/resolver | class: blocked | Implement Indexedaccesscanbehighorder |
| 2773 | Implement Indexedaccessconstraints | spike | frontend/syntax | class: blocked | Implement Indexedaccessconstraints |
| 2774 | Implement Indexedaccessimplicitlyany | spike | frontend/resolver | class: blocked | Implement Indexedaccessimplicitlyany |
| 2775 | Implement Indexedaccessnormalization | spike | frontend/resolver | class: blocked | Implement Indexedaccessnormalization |
| 2776 | Implement Indexedaccessprivatememberofgenericconstraint | spike | frontend/syntax | class: blocked | Implement Indexedaccessprivatememberofgenericconstraint |
| 2777 | Implement Indexedaccessrelation | spike | frontend/semantics | class: blocked | Implement Indexedaccessrelation |
| 2778 | Implement Indexedaccesstypeconstraints | spike | frontend/semantics | class: blocked | Implement Indexedaccesstypeconstraints |
| 2779 | Implement Indexedaccesswithfreshobjectliteral | spike | frontend/syntax | class: blocked | Implement Indexedaccesswithfreshobjectliteral |
| 2780 | Implement Indexedaccesswithvariableelement | spike | frontend/semantics | class: blocked | Implement Indexedaccesswithvariableelement |
| 2781 | Implement Indexer Parser Syntax | spike | frontend/syntax | class: blocked | Implement Indexer Parser Syntax |
| 2782 | Implement Indexer Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Indexer Unknown Unsupported |
| 2783 | Implement Indexera | spike | frontend/syntax | class: blocked | Implement Indexera |
| 2784 | Implement Indexerasoptional | spike | frontend/syntax | class: blocked | Implement Indexerasoptional |
| 2785 | Implement Indexerconstraints | spike | frontend/semantics | class: blocked | Implement Indexerconstraints |
| 2786 | Implement Indexerreturningtypeparameter | spike | frontend/semantics | class: blocked | Implement Indexerreturningtypeparameter |
| 2787 | Implement Indexersignaturewithrestparam | spike | frontend/semantics | class: blocked | Implement Indexersignaturewithrestparam |
| 2788 | Implement Indexingtypeswithnever | spike | frontend/syntax | class: blocked | Implement Indexingtypeswithnever |
| 2789 | Implement Indirectdiscriminantandexcessproperty | spike | frontend/resolver | class: blocked | Implement Indirectdiscriminantandexcessproperty |
| 2790 | Implement Indirectglobalsymbolpartofobjecttype | spike | frontend/syntax | class: blocked | Implement Indirectglobalsymbolpartofobjecttype |
| 2791 | Implement Indirectselfreferencegeneric | spike | frontend/syntax | class: blocked | Implement Indirectselfreferencegeneric |
| 2792 | Implement Indirecttypeparameterreferences | spike | frontend/syntax | class: triage-needed | Implement Indirecttypeparameterreferences |
| 2793 | Implement Indirectuniquesymboldeclarationemit | spike | frontend/syntax | class: blocked | Implement Indirectuniquesymboldeclarationemit |
| 2794 | Implement Inexistentpropertyinsidetostringtype | spike | frontend/syntax | class: blocked | Implement Inexistentpropertyinsidetostringtype |
| 2795 | Implement Inferfromannotatedreturn | spike | frontend/syntax | class: blocked | Implement Inferfromannotatedreturn |
| 2796 | Implement Inferfromgenericfunctionreturntypes | spike | frontend/syntax | class: blocked | Implement Inferfromgenericfunctionreturntypes |
| 2797 | Implement Inferfromnestedsameshapetuple | spike | frontend/syntax | class: blocked | Implement Inferfromnestedsameshapetuple |
| 2798 | Implement Inferobjecttypefromstringliteraltokeyof | spike | frontend/resolver | class: blocked | Implement Inferobjecttypefromstringliteraltokeyof |
| 2799 | Implement Inferparameterwithmethodcallinitializer | spike | frontend/syntax | class: blocked | Implement Inferparameterwithmethodcallinitializer |
| 2800 | Implement Inferpropertywithcontextsensitivereturnstatement | spike | frontend/resolver | class: blocked | Implement Inferpropertywithcontextsensitivereturnstatement |
| 2801 | Implement Inferrestargumentsmappedtuple | spike | frontend/resolver | class: blocked | Implement Inferrestargumentsmappedtuple |
| 2802 | Implement Infersecondaryparameter | spike | frontend/syntax | class: blocked | Implement Infersecondaryparameter |
| 2803 | Implement Infersetterparamtype | spike | reference/triage | class: triage-needed | Implement Infersetterparamtype |
| 2804 | Implement Inferstringliteralunionforbindingelement | spike | frontend/syntax | class: blocked | Implement Inferstringliteralunionforbindingelement |
| 2805 | Implement Infertuplefrombindingpattern | spike | frontend/resolver | class: blocked | Implement Infertuplefrombindingpattern |
| 2806 | Implement Infertypeconstraintinstantiationcircularity | spike | frontend/syntax | class: blocked | Implement Infertypeconstraintinstantiationcircularity |
| 2807 | Implement Infertypeparameterconstraints | spike | frontend/syntax | class: blocked | Implement Infertypeparameterconstraints |
| 2808 | Implement Infertypepredicates | spike | frontend/syntax | class: blocked | Implement Infertypepredicates |
| 2809 | Implement Inferenceandhkts | spike | frontend/syntax | class: blocked | Implement Inferenceandhkts |
| 2810 | Implement Inferencecontextualreturntypeunion Import Export | spike | frontend/syntax | class: blocked | Implement Inferencecontextualreturntypeunion Import Export |
| 2811 | Implement Inferencecontextualreturntypeunion Name Resolution | spike | frontend/resolver | class: blocked | Implement Inferencecontextualreturntypeunion Name Resolution |
| 2812 | Implement Inferencecontextualreturntypeunion Type System | spike | frontend/syntax | class: blocked | Implement Inferencecontextualreturntypeunion Type System |
| 2813 | Implement Inferencedoesnotaddundefinedornull | spike | frontend/syntax | class: blocked | Implement Inferencedoesnotaddundefinedornull |
| 2814 | Implement Inferencedoesntcompareagainstuninstantiatedtypeparameter | spike | frontend/syntax | class: blocked | Implement Inferencedoesntcompareagainstuninstantiatedtypeparameter |
| 2815 | Implement Inferenceerasedsignatures | spike | frontend/syntax | class: blocked | Implement Inferenceerasedsignatures |
| 2816 | Implement Inferenceexactoptionalproperties | spike | frontend/syntax | class: blocked | Implement Inferenceexactoptionalproperties |
| 2817 | Implement Inferencefromgenericclassnocrash | spike | frontend/syntax | class: blocked | Implement Inferencefromgenericclassnocrash |
| 2818 | Implement Inferencefromincompletesource | spike | frontend/syntax | class: blocked | Implement Inferencefromincompletesource |
| 2819 | Implement Inferencelimit | spike | frontend/syntax | class: blocked | Implement Inferencelimit |
| 2820 | Implement Inferenceofnullableobjecttypeswithcommonbase | spike | frontend/syntax | class: blocked | Implement Inferenceofnullableobjecttypeswithcommonbase |
| 2821 | Implement Inferenceoptionalproperties | spike | reference/triage | class: triage-needed | Implement Inferenceoptionalproperties |
| 2822 | Implement Inferenceoptionalpropertiesstrict | spike | reference/triage | class: triage-needed | Implement Inferenceoptionalpropertiesstrict |
| 2823 | Implement Inferenceoptionalpropertiestoindexsignatures | spike | frontend/syntax | class: blocked | Implement Inferenceoptionalpropertiestoindexsignatures |
| 2824 | Implement Inferenceouterresultnotincorrectlyinstantiatedwithinnerresult | spike | frontend/syntax | class: blocked | Implement Inferenceouterresultnotincorrectlyinstantiatedwithinnerresult |
| 2825 | Implement Inferenceunionofobjectsmappedcontextualtype | spike | frontend/syntax | class: blocked | Implement Inferenceunionofobjectsmappedcontextualtype |
| 2826 | Implement Inferentialtypingobjectliteralmethod | spike | frontend/resolver | class: blocked | Implement Inferentialtypingobjectliteralmethod |
| 2827 | Implement Inferentialtypingusingapparenttype | spike | frontend/syntax | class: blocked | Implement Inferentialtypingusingapparenttype |
| 2828 | Implement Inferentialtypingwithfunctiontype | spike | frontend/resolver | class: blocked | Implement Inferentialtypingwithfunctiontype |
| 2829 | Implement Inferentialtypingwithfunctiontypenested | spike | frontend/resolver | class: blocked | Implement Inferentialtypingwithfunctiontypenested |
| 2830 | Implement Inferentialtypingwithfunctiontypesyntacticscenarios | spike | frontend/syntax | class: blocked | Implement Inferentialtypingwithfunctiontypesyntacticscenarios |
| 2831 | Implement Inferentialtypingwithfunctiontypezip | spike | frontend/syntax | class: blocked | Implement Inferentialtypingwithfunctiontypezip |
| 2832 | Implement Inferentialtypingwithobjectliteralproperties | spike | frontend/syntax | class: blocked | Implement Inferentialtypingwithobjectliteralproperties |
| 2833 | Implement Inferentiallytypinganemptyarray | spike | frontend/resolver | class: blocked | Implement Inferentiallytypinganemptyarray |
| 2834 | Implement Inferredindexeronnamespaceimport | spike | frontend/syntax | class: blocked | Implement Inferredindexeronnamespaceimport |
| 2835 | Implement Inferrednonidentifiertypesgetquotes | spike | frontend/syntax | class: blocked | Implement Inferrednonidentifiertypesgetquotes |
| 2836 | Implement Inferredresttypefixedonce | spike | reference/triage | class: triage-needed | Implement Inferredresttypefixedonce |
| 2837 | Implement Inferredreturntypeincorrectreuse | spike | frontend/syntax | class: blocked | Implement Inferredreturntypeincorrectreuse |
| 2838 | Implement Inferrenceinfiniteloopwithsubtyping | spike | frontend/syntax | class: blocked | Implement Inferrenceinfiniteloopwithsubtyping |
| 2839 | Implement Inferringreturntypefromconstructsignaturegeneric | spike | frontend/syntax | class: blocked | Implement Inferringreturntypefromconstructsignaturegeneric |
| 2840 | Implement Infiniteconstraints | spike | frontend/resolver | class: blocked | Implement Infiniteconstraints |
| 2841 | Implement Infinitelyexpandingbasetypes | spike | frontend/syntax | class: blocked | Implement Infinitelyexpandingbasetypes |
| 2842 | Implement Infinitelyexpandingoverloads | spike | frontend/semantics | class: blocked | Implement Infinitelyexpandingoverloads |
| 2843 | Implement Infinitelyexpandingtypeassignability | spike | frontend/syntax | class: blocked | Implement Infinitelyexpandingtypeassignability |
| 2844 | Implement Infinitelyexpandingtypesnongenericbase | spike | frontend/syntax | class: blocked | Implement Infinitelyexpandingtypesnongenericbase |
| 2845 | Implement Inheritfromgenerictypeparameter | spike | frontend/syntax | class: blocked | Implement Inheritfromgenerictypeparameter |
| 2846 | Implement Inheritsamenameprivatepropertiesfromdifferentorigins | spike | frontend/semantics | class: blocked | Implement Inheritsamenameprivatepropertiesfromdifferentorigins |
| 2847 | Implement Inheritsamenameprivatepropertiesfromsameorigin | spike | frontend/semantics | class: blocked | Implement Inheritsamenameprivatepropertiesfromsameorigin |
| 2848 | Implement Inheritsamenamepropertieswithdifferentvisibility | spike | frontend/syntax | class: blocked | Implement Inheritsamenamepropertieswithdifferentvisibility |
| 2849 | Implement Inheritance | spike | frontend/syntax | class: blocked | Implement Inheritance |
| 2850 | Implement Inheritancegrandparentprivatemembercollision | spike | frontend/syntax | class: blocked | Implement Inheritancegrandparentprivatemembercollision |
| 2851 | Implement Inheritancegrandparentprivatemembercollisionwithpublicmember | spike | frontend/syntax | class: blocked | Implement Inheritancegrandparentprivatemembercollisionwithpublicmember |
| 2852 | Implement Inheritancegrandparentpublicmembercollisionwithprivatemember | spike | frontend/syntax | class: blocked | Implement Inheritancegrandparentpublicmembercollisionwithprivatemember |
| 2853 | Implement Inheritancememberaccessoroverridingaccessor | spike | reference/triage | class: triage-needed | Implement Inheritancememberaccessoroverridingaccessor |
| 2854 | Implement Inheritancememberaccessoroverridingmethod | spike | reference/triage | class: triage-needed | Implement Inheritancememberaccessoroverridingmethod |
| 2855 | Implement Inheritancememberaccessoroverridingproperty | spike | reference/triage | class: triage-needed | Implement Inheritancememberaccessoroverridingproperty |
| 2856 | Implement Inheritancememberfuncoverridingaccessor | spike | reference/triage | class: triage-needed | Implement Inheritancememberfuncoverridingaccessor |
| 2857 | Implement Inheritancememberpropertyoverridingaccessor | spike | frontend/syntax | class: blocked | Implement Inheritancememberpropertyoverridingaccessor |
| 2858 | Implement Inheritanceofgenericconstructormethod Import Export | spike | frontend/syntax | class: blocked | Implement Inheritanceofgenericconstructormethod Import Export |
| 2859 | Implement Inheritanceofgenericconstructormethod Type System | spike | frontend/syntax | class: blocked | Implement Inheritanceofgenericconstructormethod Type System |
| 2860 | Implement Inheritancestaticaccessoroverridingaccessor | spike | reference/triage | class: triage-needed | Implement Inheritancestaticaccessoroverridingaccessor |
| 2861 | Implement Inheritancestaticaccessoroverridingmethod | spike | reference/triage | class: triage-needed | Implement Inheritancestaticaccessoroverridingmethod |
| 2862 | Implement Inheritancestaticaccessoroverridingproperty | spike | reference/triage | class: triage-needed | Implement Inheritancestaticaccessoroverridingproperty |
| 2863 | Implement Inheritancestaticfuncoverridingaccessor | spike | reference/triage | class: triage-needed | Implement Inheritancestaticfuncoverridingaccessor |
| 2864 | Implement Inheritancestaticpropertyoverridingaccessor | spike | reference/triage | class: triage-needed | Implement Inheritancestaticpropertyoverridingaccessor |
| 2865 | Implement Inheritedconstructorpropertycontextualtype | spike | frontend/semantics | class: blocked | Implement Inheritedconstructorpropertycontextualtype |
| 2866 | Implement Inheritedconstructorwithrestparams Arity | spike | reference/triage | class: triage-needed | Implement Inheritedconstructorwithrestparams Arity |
| 2867 | Implement Inheritedconstructorwithrestparams Parser Syntax | spike | frontend/semantics | class: blocked | Implement Inheritedconstructorwithrestparams Parser Syntax |
| 2868 | Implement Inheritedgenericcallsignature | spike | frontend/syntax | class: blocked | Implement Inheritedgenericcallsignature |
| 2869 | Implement Inheritedmodulemembersforclodule | spike | frontend/syntax | class: blocked | Implement Inheritedmodulemembersforclodule |
| 2870 | Implement Inheritedoverloadedspecializedsignatures | spike | frontend/syntax | class: blocked | Implement Inheritedoverloadedspecializedsignatures |
| 2871 | Implement Initializepropertieswithrenamedlet | spike | frontend/syntax | class: blocked | Implement Initializepropertieswithrenamedlet |
| 2872 | Implement Initializeddestructuringassignmenttypes | spike | reference/triage | class: blocked | Implement Initializeddestructuringassignmenttypes |
| 2873 | Implement Initializerwiththispropertyaccess | spike | frontend/syntax | class: blocked | Implement Initializerwiththispropertyaccess |
| 2874 | Implement Inlineconditionalhassimilarassignability | spike | frontend/syntax | class: blocked | Implement Inlineconditionalhassimilarassignability |
| 2875 | Implement Inlinemappedtypemodifierdeclarationemit | spike | frontend/syntax | class: blocked | Implement Inlinemappedtypemodifierdeclarationemit |
| 2876 | Implement Inneraliases | spike | frontend/syntax | class: blocked | Implement Inneraliases |
| 2877 | Implement Innerboundlambdaemit | spike | frontend/syntax | class: blocked | Implement Innerboundlambdaemit |
| 2878 | Implement Innerextern | spike | frontend/syntax | class: blocked | Implement Innerextern |
| 2879 | Implement Innerfunc | spike | frontend/syntax | class: blocked | Implement Innerfunc |
| 2880 | Implement Innermodexport | spike | frontend/syntax | class: blocked | Implement Innermodexport |
| 2881 | Implement Inneroverloads | spike | frontend/semantics | class: blocked | Implement Inneroverloads |
| 2882 | Implement Instanceandstaticdeclarations | spike | frontend/syntax | class: blocked | Implement Instanceandstaticdeclarations |
| 2883 | Implement Instanceofassignability | spike | frontend/syntax | class: blocked | Implement Instanceofassignability |
| 2884 | Implement Instanceofinexternalmodules | spike | frontend/syntax | class: blocked | Implement Instanceofinexternalmodules |
| 2885 | Implement Instancesubtypecheck | spike | frontend/syntax | class: blocked | Implement Instancesubtypecheck |
| 2886 | Implement Instanceofnarrowreadonlyarray | spike | frontend/syntax | class: blocked | Implement Instanceofnarrowreadonlyarray |
| 2887 | Implement Instanceofoninstantiationexpression | spike | frontend/syntax | class: blocked | Implement Instanceofoninstantiationexpression |
| 2888 | Implement Instanceoftypealiastogenericclass | spike | frontend/syntax | class: blocked | Implement Instanceoftypealiastogenericclass |
| 2889 | Implement Instanceofwithprimitiveunion | spike | frontend/syntax | class: blocked | Implement Instanceofwithprimitiveunion |
| 2890 | Implement Instantiatecontextualtypes | spike | frontend/semantics | class: blocked | Implement Instantiatecontextualtypes |
| 2891 | Implement Instantiatecontextuallytypedgenericthis | spike | frontend/syntax | class: blocked | Implement Instantiatecontextuallytypedgenericthis |
| 2892 | Implement Instantiatecrossfilemerge | spike | frontend/syntax | class: blocked | Implement Instantiatecrossfilemerge |
| 2893 | Implement Instantiatedbasetypeconstraints | spike | frontend/semantics | class: blocked | Implement Instantiatedbasetypeconstraints |
| 2894 | Implement Instantiatedtypealiasdisplay | spike | frontend/syntax | class: blocked | Implement Instantiatedtypealiasdisplay |
| 2895 | Implement Instantiationexpressionerrornocrash | spike | frontend/syntax | class: triage-needed | Implement Instantiationexpressionerrornocrash |
| 2896 | Implement Inttypecheck | spike | frontend/syntax | class: triage-needed | Implement Inttypecheck |
| 2897 | Implement Intermixingmodulesinterfaces | spike | frontend/syntax | class: blocked | Implement Intermixingmodulesinterfaces |
| 2898 | Implement Interfaceassignmentcompat | spike | frontend/syntax | class: blocked | Implement Interfaceassignmentcompat |
| 2899 | Implement Interfaceclassmerging | spike | frontend/syntax | class: blocked | Implement Interfaceclassmerging |
| 2900 | Implement Interfacecontextualtype | spike | frontend/semantics | class: blocked | Implement Interfacecontextualtype |
| 2901 | Implement Interfacedeclaration Import Export | spike | frontend/syntax | class: blocked | Implement Interfacedeclaration Import Export |
| 2902 | Implement Interfacedeclaration Parser Syntax | spike | frontend/syntax | class: blocked | Implement Interfacedeclaration Parser Syntax |
| 2903 | Implement Interfaceextendsclass | spike | frontend/syntax | class: blocked | Implement Interfaceextendsclass |
| 2904 | Implement Interfaceextendsclasswithprivate | spike | frontend/semantics | class: blocked | Implement Interfaceextendsclasswithprivate |
| 2905 | Implement Interfaceimplementation | spike | frontend/syntax | class: blocked | Implement Interfaceimplementation |
| 2906 | Implement Interfaceinreopenedmodule | spike | frontend/syntax | class: blocked | Implement Interfaceinreopenedmodule |
| 2907 | Implement Interfaceinheritance Method Call | spike | frontend/syntax | class: blocked | Implement Interfaceinheritance Method Call |
| 2908 | Implement Interfaceinheritance Parser Syntax | spike | frontend/syntax | class: blocked | Implement Interfaceinheritance Parser Syntax |
| 2909 | Implement Interfacemergewithnongenerictypearguments | spike | frontend/syntax | class: blocked | Implement Interfacemergewithnongenerictypearguments |
| 2910 | Implement Interfacemergedunconstrainednoerrorirrespectiveoforder | spike | frontend/syntax | class: blocked | Implement Interfacemergedunconstrainednoerrorirrespectiveoforder |
| 2911 | Implement Interfacenameasidentifier | spike | frontend/syntax | class: blocked | Implement Interfacenameasidentifier |
| 2912 | Implement Interfacenaming | spike | frontend/syntax | class: blocked | Implement Interfacenaming |
| 2913 | Implement Interfacepropertieswithsamename | spike | frontend/syntax | class: blocked | Implement Interfacepropertieswithsamename |
| 2914 | Implement Interfacesubtyping | spike | frontend/syntax | class: blocked | Implement Interfacesubtyping |
| 2915 | Implement Interfacewithcommaseparators | spike | frontend/syntax | class: blocked | Implement Interfacewithcommaseparators |
| 2916 | Implement Interfacewithmultipledeclarations | spike | frontend/syntax | class: blocked | Implement Interfacewithmultipledeclarations |
| 2917 | Implement Interfacedecl | spike | frontend/syntax | class: blocked | Implement Interfacedecl |
| 2918 | Implement Interfacedeclwithindexererrors | spike | runtime/builtins | class: blocked | Implement Interfacedeclwithindexererrors |
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
| 2973 | Implement Intersectiontypeinference | spike | frontend/syntax | class: blocked | Implement Intersectiontypeinference |
| 2974 | Implement Intersectiontypenormalization | spike | frontend/syntax | class: blocked | Implement Intersectiontypenormalization |
| 2975 | Implement Intersectionwithconflictingprivates | spike | frontend/semantics | class: blocked | Implement Intersectionwithconflictingprivates |
| 2976 | Implement Intersectionsandoptionalproperties | spike | frontend/resolver | class: blocked | Implement Intersectionsandoptionalproperties |
| 2977 | Implement Intersectionsandreadonlyproperties | spike | frontend/resolver | class: blocked | Implement Intersectionsandreadonlyproperties |
| 2978 | Implement Intersectionsoflargeunions Import Export | spike | frontend/syntax | class: blocked | Implement Intersectionsoflargeunions Import Export |
| 2979 | Implement Intersectionsoflargeunions Parser Syntax | spike | frontend/semantics | class: blocked | Implement Intersectionsoflargeunions Parser Syntax |
| 2980 | Implement Intrabindingpatternreferences | spike | frontend/resolver | class: blocked | Implement Intrabindingpatternreferences |
| 2981 | Implement Intrinsics | spike | frontend/syntax | class: blocked | Implement Intrinsics |
| 2982 | Implement Invalidcontinueindownlevelasync | spike | reference/triage | class: triage-needed | Implement Invalidcontinueindownlevelasync |
| 2983 | Implement Invalidletinforofandforin | spike | frontend/syntax | class: blocked | Implement Invalidletinforofandforin |
| 2984 | Implement Invalidoptionalchainfromnewexpression | spike | frontend/syntax | class: blocked | Implement Invalidoptionalchainfromnewexpression |
| 2985 | Implement Invalidsplice | spike | frontend/syntax | class: blocked | Implement Invalidsplice |
| 2986 | Implement Invalidstaticfield | spike | frontend/syntax | class: blocked | Implement Invalidstaticfield |
| 2987 | Implement Invalidsymbolintypeparameter | spike | frontend/syntax | class: blocked | Implement Invalidsymbolintypeparameter |
| 2988 | Implement Invalidthisemitincontextualobjectliteral | spike | frontend/syntax | class: blocked | Implement Invalidthisemitincontextualobjectliteral |
| 2989 | Implement Invalidunicodeescapesequance Parser Syntax | spike | frontend/syntax | class: blocked | Implement Invalidunicodeescapesequance Parser Syntax |
| 2990 | Implement Invalidunicodeescapesequance Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Invalidunicodeescapesequance Unknown Unsupported |
| 2991 | Implement Invariantgenericerrorelaboration | spike | frontend/semantics | class: blocked | Implement Invariantgenericerrorelaboration |
| 2992 | Implement Invokingnongenericmethodwithtypearguments | spike | frontend/syntax | class: blocked | Implement Invokingnongenericmethodwithtypearguments |
| 2993 | Implement Ipromise Class | spike | frontend/syntax | class: blocked | Implement Ipromise Class |
| 2994 | Implement Ipromise Import Export | spike | frontend/syntax | class: blocked | Implement Ipromise Import Export |
| 2995 | Implement Isarray | spike | frontend/resolver | class: blocked | Implement Isarray |
| 2996 | Implement Isdeclarationvisiblenodekinds | spike | frontend/syntax | class: blocked | Implement Isdeclarationvisiblenodekinds |
| 2997 | Implement Isolateddeclarationerrortypes | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationerrortypes |
| 2998 | Implement Isolateddeclarationerrors | spike | runtime/builtins | class: blocked | Implement Isolateddeclarationerrors |
| 2999 | Implement Isolateddeclarationerrorsaugmentation | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationerrorsaugmentation |
| 3000 | Implement Isolateddeclarationerrorsclasses | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationerrorsclasses |
| 3001 | Implement Isolateddeclarationerrorsclassesexpressions | spike | runtime/builtins | class: blocked | Implement Isolateddeclarationerrorsclassesexpressions |
| 3002 | Implement Isolateddeclarationerrorsdefault | spike | runtime/builtins | class: blocked | Implement Isolateddeclarationerrorsdefault |
| 3003 | Implement Isolateddeclarationerrorsenums | spike | runtime/builtins | class: blocked | Implement Isolateddeclarationerrorsenums |
| 3004 | Implement Isolateddeclarationerrorsexpandofunctions | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationerrorsexpandofunctions |
| 3005 | Implement Isolateddeclarationerrorsexpressions | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationerrorsexpressions |
| 3006 | Implement Isolateddeclarationerrorsfunctiondeclarations | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationerrorsfunctiondeclarations |
| 3007 | Implement Isolateddeclarationerrorsobjects | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationerrorsobjects |
| 3008 | Implement Isolateddeclarationerrorsreturntypes | spike | runtime/builtins | class: blocked | Implement Isolateddeclarationerrorsreturntypes |
| 3009 | Implement Isolateddeclarationlazysymbols | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationlazysymbols |
| 3010 | Implement Isolateddeclarationoutfile | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationoutfile |
| 3011 | Implement Isolateddeclarationsaddundefined | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationsaddundefined |
| 3012 | Implement Isolateddeclarationsallowjs | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationsallowjs |
| 3013 | Implement Isolateddeclarationsliterals | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationsliterals |
| 3014 | Implement Isolateddeclarationsrequiresdeclaration | spike | frontend/syntax | class: blocked | Implement Isolateddeclarationsrequiresdeclaration |
| 3015 | Implement Isolateddeclarationsstrictbuiltiniteratorreturn | spike | runtime/builtins | class: blocked | Implement Isolateddeclarationsstrictbuiltiniteratorreturn |
| 3016 | Implement Isolatedmodules | spike | frontend/syntax | class: blocked | Implement Isolatedmodules |
| 3017 | Implement Isolatedmodulesambientconstenum | spike | frontend/syntax | class: blocked | Implement Isolatedmodulesambientconstenum |
| 3018 | Implement Isolatedmodulesconstenum | spike | frontend/syntax | class: blocked | Implement Isolatedmodulesconstenum |
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
| 3029 | Implement Isolatedmodulesnonambientconstenum | spike | frontend/syntax | class: blocked | Implement Isolatedmodulesnonambientconstenum |
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
| 3040 | Implement Iterabletreturntnext | spike | runtime/builtins | class: blocked | Implement Iterabletreturntnext |
| 3041 | Implement Iterablewithneverasunionmember | spike | frontend/resolver | class: blocked | Implement Iterablewithneverasunionmember |
| 3042 | Implement Iteratorextraparameters | spike | runtime/builtins | class: blocked | Implement Iteratorextraparameters |
| 3043 | Implement Iteratorsandstrictnullchecks | spike | frontend/syntax | class: blocked | Implement Iteratorsandstrictnullchecks |
| 3044 | Implement Javascriptcommonjsmodule | spike | frontend/syntax | class: blocked | Implement Javascriptcommonjsmodule |
| 3045 | Implement Javascriptdefinepropertyprototypenonconstructor | spike | frontend/resolver | class: blocked | Implement Javascriptdefinepropertyprototypenonconstructor |
| 3046 | Implement Javascriptimportdefaultbadexport | spike | frontend/syntax | class: blocked | Implement Javascriptimportdefaultbadexport |
| 3047 | Implement Javascriptthisassignmentinstaticblock | spike | frontend/syntax | class: blocked | Implement Javascriptthisassignmentinstaticblock |
| 3048 | Implement Jqueryinference | spike | frontend/syntax | class: blocked | Implement Jqueryinference |
| 3049 | Implement Jscheckobjectdefinethisnocrash | spike | frontend/resolver | class: blocked | Implement Jscheckobjectdefinethisnocrash |
| 3050 | Implement Jsdeclarationemitdoesnotrenameimport | spike | frontend/syntax | class: blocked | Implement Jsdeclarationemitdoesnotrenameimport |
| 3051 | Implement Jsdeclarationemitexportassignedarray | spike | frontend/syntax | class: blocked | Implement Jsdeclarationemitexportassignedarray |
| 3052 | Implement Jsdeclarationemitexportassignedfunctionwithextratypedefsmembers | spike | frontend/syntax | class: blocked | Implement Jsdeclarationemitexportassignedfunctionwithextratypedefsmembers |
| 3053 | Implement Jsdeclarationemitexportedclasswithextends | spike | frontend/syntax | class: blocked | Implement Jsdeclarationemitexportedclasswithextends |
| 3054 | Implement Jsdeclarationsglobalfileconstfunction | spike | reference/triage | class: triage-needed | Implement Jsdeclarationsglobalfileconstfunction |
| 3055 | Implement Jsdeclarationsglobalfileconstfunctionnamed | spike | frontend/syntax | class: blocked | Implement Jsdeclarationsglobalfileconstfunctionnamed |
| 3056 | Implement Jsdeclarationsinheritedtypes | spike | frontend/syntax | class: blocked | Implement Jsdeclarationsinheritedtypes |
| 3057 | Implement Jsdeclarationswithdefaultasnamespacelikemerge | spike | frontend/resolver | class: blocked | Implement Jsdeclarationswithdefaultasnamespacelikemerge |
| 3058 | Implement Jsdocdeclarationemitdoesnotusenodemodulespathwithouterror | spike | frontend/syntax | class: blocked | Implement Jsdocdeclarationemitdoesnotusenodemodulespathwithouterror |
| 3059 | Implement Jselementaccessnocontextualtypecrash | spike | frontend/resolver | class: blocked | Implement Jselementaccessnocontextualtypecrash |
| 3060 | Implement Jsemitintersectionproperty | spike | frontend/semantics | class: blocked | Implement Jsemitintersectionproperty |
| 3061 | Implement Jsenumcrossfileexport | spike | frontend/syntax | class: blocked | Implement Jsenumcrossfileexport |
| 3062 | Implement Jsenumtagonobjectfrozen | spike | frontend/syntax | class: blocked | Implement Jsenumtagonobjectfrozen |
| 3063 | Implement Jsexpandoobjectdefineproperty | spike | frontend/syntax | class: blocked | Implement Jsexpandoobjectdefineproperty |
| 3064 | Implement Jsexportassignmentnonmutablelocation | spike | frontend/syntax | class: blocked | Implement Jsexportassignmentnonmutablelocation |
| 3065 | Implement Jsexportmembermergedwithmoduleaugmentation | spike | frontend/syntax | class: blocked | Implement Jsexportmembermergedwithmoduleaugmentation |
| 3066 | Implement Jsextendsimplicitany | spike | frontend/syntax | class: blocked | Implement Jsextendsimplicitany |
| 3067 | Implement Jsfileclasspropertyinitalizationinobjectliteral | spike | frontend/syntax | class: blocked | Implement Jsfileclasspropertyinitalizationinobjectliteral |
| 3068 | Implement Jsfileclasspropertytype | spike | frontend/resolver | class: blocked | Implement Jsfileclasspropertytype |
| 3069 | Implement Jsfileclassselfreferencedproperty | spike | frontend/syntax | class: blocked | Implement Jsfileclassselfreferencedproperty |
| 3070 | Implement Jsfilecompilationabstractmodifier | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationabstractmodifier |
| 3071 | Implement Jsfilecompilationawaitmodifier | spike | runtime/builtins | class: blocked | Implement Jsfilecompilationawaitmodifier |
| 3072 | Implement Jsfilecompilationbinddeepexportsassignment | spike | frontend/resolver | class: blocked | Implement Jsfilecompilationbinddeepexportsassignment |
| 3073 | Implement Jsfilecompilationbinderrors | spike | reference/triage | class: triage-needed | Implement Jsfilecompilationbinderrors |
| 3074 | Implement Jsfilecompilationbindmultipledefaultexports | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationbindmultipledefaultexports |
| 3075 | Implement Jsfilecompilationbindreachabilityerrors | spike | frontend/resolver | class: blocked | Implement Jsfilecompilationbindreachabilityerrors |
| 3076 | Implement Jsfilecompilationbindstrictmodeerrors | spike | runtime/builtins | class: blocked | Implement Jsfilecompilationbindstrictmodeerrors |
| 3077 | Implement Jsfilecompilationconstructoroverloadsyntax | spike | frontend/semantics | class: blocked | Implement Jsfilecompilationconstructoroverloadsyntax |
| 3078 | Implement Jsfilecompilationdecoratorsyntax | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationdecoratorsyntax |
| 3079 | Implement Jsfilecompilationduplicatefunctionimplementation | spike | reference/triage | class: triage-needed | Implement Jsfilecompilationduplicatefunctionimplementation |
| 3080 | Implement Jsfilecompilationduplicatefunctionimplementationfileorderreversed | spike | reference/triage | class: triage-needed | Implement Jsfilecompilationduplicatefunctionimplementationfileorderreversed |
| 3081 | Implement Jsfilecompilationduplicatevariable | spike | reference/triage | class: triage-needed | Implement Jsfilecompilationduplicatevariable |
| 3082 | Implement Jsfilecompilationduplicatevariableerrorreported | spike | reference/triage | class: triage-needed | Implement Jsfilecompilationduplicatevariableerrorreported |
| 3083 | Implement Jsfilecompilationenumsyntax | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationenumsyntax |
| 3084 | Implement Jsfilecompilationexportassignmentsyntax | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationexportassignmentsyntax |
| 3085 | Implement Jsfilecompilationexternalpackageerror | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationexternalpackageerror |
| 3086 | Implement Jsfilecompilationfunctionoverloadsyntax | spike | frontend/semantics | class: blocked | Implement Jsfilecompilationfunctionoverloadsyntax |
| 3087 | Implement Jsfilecompilationheritageclausesyntaxofclass | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationheritageclausesyntaxofclass |
| 3088 | Implement Jsfilecompilationimportequalssyntax | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationimportequalssyntax |
| 3089 | Implement Jsfilecompilationletdeclarationorder | spike | frontend/resolver | class: blocked | Implement Jsfilecompilationletdeclarationorder |
| 3090 | Implement Jsfilecompilationmethodoverloadsyntax | spike | frontend/semantics | class: blocked | Implement Jsfilecompilationmethodoverloadsyntax |
| 3091 | Implement Jsfilecompilationmodulesyntax | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationmodulesyntax |
| 3092 | Implement Jsfilecompilationnonnullassertion | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationnonnullassertion |
| 3093 | Implement Jsfilecompilationoptionalclasselementsyntaxofclass | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationoptionalclasselementsyntaxofclass |
| 3094 | Implement Jsfilecompilationpublicmethodsyntaxofclass | spike | frontend/semantics | class: blocked | Implement Jsfilecompilationpublicmethodsyntaxofclass |
| 3095 | Implement Jsfilecompilationrestparamjsdocfunction | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationrestparamjsdocfunction |
| 3096 | Implement Jsfilecompilationshorthandproperty | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationshorthandproperty |
| 3097 | Implement Jsfilecompilationtypeargumentsyntaxofcall | spike | reference/triage | class: blocked | Implement Jsfilecompilationtypeargumentsyntaxofcall |
| 3098 | Implement Jsfilecompilationtypeassertions | spike | frontend/syntax | class: blocked | Implement Jsfilecompilationtypeassertions |
| 3099 | Implement Jsfilecompilationtypeparametersyntaxofclass | spike | frontend/semantics | class: blocked | Implement Jsfilecompilationtypeparametersyntaxofclass |
| 3100 | Implement Jsfilecompilationtypeparametersyntaxofclassexpression | spike | frontend/semantics | class: blocked | Implement Jsfilecompilationtypeparametersyntaxofclassexpression |
| 3101 | Implement Jsfileesmodulewithenumtag | spike | frontend/resolver | class: blocked | Implement Jsfileesmodulewithenumtag |
| 3102 | Implement Jsfilefunctionoverloads | spike | frontend/semantics | class: blocked | Implement Jsfilefunctionoverloads |
| 3103 | Implement Jsfileimportpreservedwhenused | spike | frontend/syntax | class: blocked | Implement Jsfileimportpreservedwhenused |
| 3104 | Implement Jsfilemethodoverloads Import Export | spike | frontend/syntax | class: blocked | Implement Jsfilemethodoverloads Import Export |
| 3105 | Implement Jsfilemethodoverloads Parser Syntax | spike | frontend/semantics | class: blocked | Implement Jsfilemethodoverloads Parser Syntax |
| 3106 | Implement Jsfilemethodoverloads Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Jsfilemethodoverloads Unknown Unsupported |
| 3107 | Implement Jsfunctionwithprototypenoerrortruncationnocrash | spike | frontend/resolver | class: blocked | Implement Jsfunctionwithprototypenoerrortruncationnocrash |
| 3108 | Implement Jsnoimplicitanynocascadingreferenceerrors | spike | frontend/resolver | class: blocked | Implement Jsnoimplicitanynocascadingreferenceerrors |
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
| 3125 | Implement Jsxemitwithattributes | spike | reference/triage | class: blocked | Implement Jsxemitwithattributes |
| 3126 | Implement Jsxfactoryandreactnamespace | spike | reference/triage | class: blocked | Implement Jsxfactoryandreactnamespace |
| 3127 | Implement Jsxfactoryidentifier | spike | reference/triage | class: blocked | Implement Jsxfactoryidentifier |
| 3128 | Implement Jsxfactoryidentifierasparameter | spike | frontend/syntax | class: blocked | Implement Jsxfactoryidentifierasparameter |
| 3129 | Implement Jsxfactoryidentifierwithabsentparameter | spike | frontend/syntax | class: blocked | Implement Jsxfactoryidentifierwithabsentparameter |
| 3130 | Implement Jsxfactorymissingerrorinsideaclass | spike | reference/triage | class: blocked | Implement Jsxfactorymissingerrorinsideaclass |
| 3131 | Implement Jsxfactorynotidentifierorqualifiedname | spike | reference/triage | class: blocked | Implement Jsxfactorynotidentifierorqualifiedname |
| 3132 | Implement Jsxfactoryqualifiedname | spike | reference/triage | class: blocked | Implement Jsxfactoryqualifiedname |
| 3133 | Implement Jsxfactoryqualifiednameresolutionerror | spike | frontend/syntax | class: blocked | Implement Jsxfactoryqualifiednameresolutionerror |
| 3134 | Implement Jsxfactoryqualifiednamewithes | spike | reference/triage | class: blocked | Implement Jsxfactoryqualifiednamewithes |
| 3135 | Implement Jsxpreservewithjsinput | spike | reference/triage | class: blocked | Implement Jsxpreservewithjsinput |
| 3136 | Implement Jsxruntimepragma | spike | reference/triage | class: blocked | Implement Jsxruntimepragma |
| 3137 | Implement Jsxspreadtag | spike | reference/triage | class: blocked | Implement Jsxspreadtag |
| 3138 | Implement Keepimportsindts | spike | frontend/syntax | class: blocked | Implement Keepimportsindts |
| 3139 | Implement Keyremappingkeyofresult | spike | frontend/syntax | class: blocked | Implement Keyremappingkeyofresult |
| 3140 | Implement Keyofdoesntcontainsymbols | spike | frontend/syntax | class: blocked | Implement Keyofdoesntcontainsymbols |
| 3141 | Implement Keyofgenericextendingclassdoublelayer | spike | frontend/syntax | class: blocked | Implement Keyofgenericextendingclassdoublelayer |
| 3142 | Implement Keyofisliteralcontexualtype | spike | frontend/resolver | class: blocked | Implement Keyofisliteralcontexualtype |
| 3143 | Implement Keyofmoduleobjecthascorrectkeys | spike | frontend/syntax | class: blocked | Implement Keyofmoduleobjecthascorrectkeys |
| 3144 | Implement Keyofobjectwithglobalsymbolincluded | spike | frontend/syntax | class: blocked | Implement Keyofobjectwithglobalsymbolincluded |
| 3145 | Implement Keywordexpressioninternalcomments | spike | frontend/syntax | class: blocked | Implement Keywordexpressioninternalcomments |
| 3146 | Implement Keywordfield | spike | frontend/syntax | class: blocked | Implement Keywordfield |
| 3147 | Implement Knockout | spike | frontend/syntax | class: blocked | Implement Knockout |
| 3148 | Implement Lambdaargcrash | spike | frontend/syntax | class: blocked | Implement Lambdaargcrash |
| 3149 | Implement Lambdaparamtypes | spike | frontend/resolver | class: blocked | Implement Lambdaparamtypes |
| 3150 | Implement Lambdaparameterwithtupleargshascorrectassignability | spike | frontend/syntax | class: blocked | Implement Lambdaparameterwithtupleargshascorrectassignability |
| 3151 | Implement Lambdapropself | spike | frontend/syntax | class: blocked | Implement Lambdapropself |
| 3152 | Implement Largetupletypes | spike | frontend/syntax | class: blocked | Implement Largetupletypes |
| 3153 | Implement Lastpropertyinliteralwins | spike | frontend/syntax | class: blocked | Implement Lastpropertyinliteralwins |
| 3154 | Implement Lateboundassignmentcandidatejs | spike | frontend/syntax | class: blocked | Implement Lateboundassignmentcandidatejs |
| 3155 | Implement Lateboundconstrainttypecheckscorrectly | spike | frontend/resolver | class: blocked | Implement Lateboundconstrainttypecheckscorrectly |
| 3156 | Implement Latebounddestructuringimplicitanyerror | spike | frontend/syntax | class: blocked | Implement Latebounddestructuringimplicitanyerror |
| 3157 | Implement Lateboundfunctionmemberassignmentdeclarations | spike | frontend/syntax | class: blocked | Implement Lateboundfunctionmemberassignmentdeclarations |
| 3158 | Implement Lateboundmethodnameassigmentjs | spike | frontend/syntax | class: blocked | Implement Lateboundmethodnameassigmentjs |
| 3159 | Implement Letandvarredeclaration | spike | frontend/syntax | class: blocked | Implement Letandvarredeclaration |
| 3160 | Implement Letasidentifier | spike | frontend/syntax | class: blocked | Implement Letasidentifier |
| 3161 | Implement Letasidentifierinstrictmode | spike | frontend/syntax | class: blocked | Implement Letasidentifierinstrictmode |
| 3162 | Implement Letconstincaseclauses | spike | frontend/syntax | class: blocked | Implement Letconstincaseclauses |
| 3163 | Implement Letconstmatchingparameternames | spike | frontend/resolver | class: blocked | Implement Letconstmatchingparameternames |
| 3164 | Implement Letdeclarations Duplicate Local | spike | reference/triage | class: triage-needed | Implement Letdeclarations Duplicate Local |
| 3165 | Implement Letdeclarations Import Export | spike | frontend/syntax | class: blocked | Implement Letdeclarations Import Export |
| 3166 | Implement Letdeclarations Name Resolution | spike | frontend/resolver | class: blocked | Implement Letdeclarations Name Resolution |
| 3167 | Implement Letdeclarations Parser Syntax | spike | frontend/syntax | class: blocked | Implement Letdeclarations Parser Syntax |
| 3168 | Implement Letdeclarations Scope Analysis | spike | frontend/syntax | class: blocked | Implement Letdeclarations Scope Analysis |
| 3169 | Implement Letinconstdeclarations | spike | frontend/syntax | class: blocked | Implement Letinconstdeclarations |
| 3170 | Implement Letinletconstdeclofforofandforin | spike | frontend/syntax | class: blocked | Implement Letinletconstdeclofforofandforin |
| 3171 | Implement Letinletdeclarations | spike | frontend/syntax | class: blocked | Implement Letinletdeclarations |
| 3172 | Implement Letinvardeclofforin | spike | frontend/syntax | class: blocked | Implement Letinvardeclofforin |
| 3173 | Implement Letinvardeclofforof | spike | frontend/syntax | class: blocked | Implement Letinvardeclofforof |
| 3174 | Implement Letkeepnamesoftoplevelitems | spike | frontend/syntax | class: blocked | Implement Letkeepnamesoftoplevelitems |
| 3175 | Implement Libmembers | spike | frontend/syntax | class: blocked | Implement Libmembers |
| 3176 | Implement Libtypescriptoverridesimple | spike | runtime/builtins | class: blocked | Implement Libtypescriptoverridesimple |
| 3177 | Implement Libtypescriptoverridesimpleconfig | spike | runtime/builtins | class: blocked | Implement Libtypescriptoverridesimpleconfig |
| 3178 | Implement Libtypescriptsubfileresolving | spike | runtime/builtins | class: blocked | Implement Libtypescriptsubfileresolving |
| 3179 | Implement Libtypescriptsubfileresolvingconfig | spike | runtime/builtins | class: blocked | Implement Libtypescriptsubfileresolvingconfig |
| 3180 | Implement Library Method Call | spike | frontend/syntax | class: blocked | Implement Library Method Call |
| 3181 | Implement Library Object Literal | spike | frontend/syntax | class: blocked | Implement Library Object Literal |
| 3182 | Implement Lift | spike | frontend/syntax | class: blocked | Implement Lift |
| 3183 | Implement Limitdeepinstantiations | spike | frontend/syntax | class: triage-needed | Implement Limitdeepinstantiations |
| 3184 | Implement Listfailure | spike | frontend/syntax | class: blocked | Implement Listfailure |
| 3185 | Implement Literalfreshnesspropagationonnarrowing | spike | frontend/semantics | class: blocked | Implement Literalfreshnesspropagationonnarrowing |
| 3186 | Implement Literaltypenameassertionnottriggered | spike | frontend/syntax | class: blocked | Implement Literaltypenameassertionnottriggered |
| 3187 | Implement Literalwideningwithcompoundlikeassignments | spike | frontend/syntax | class: blocked | Implement Literalwideningwithcompoundlikeassignments |
| 3188 | Implement Literals | spike | frontend/syntax | class: triage-needed | Implement Literals |
| 3189 | Implement Literalsincomputedproperties | spike | frontend/syntax | class: blocked | Implement Literalsincomputedproperties |
| 3190 | Implement Localaliasexportassignment | spike | frontend/syntax | class: blocked | Implement Localaliasexportassignment |
| 3191 | Implement Localclassesinloop | spike | frontend/resolver | class: blocked | Implement Localclassesinloop |
| 3192 | Implement Localimportnamevsglobalname | spike | frontend/syntax | class: blocked | Implement Localimportnamevsglobalname |
| 3193 | Implement Localrequirefunction | spike | frontend/syntax | class: blocked | Implement Localrequirefunction |
| 3194 | Implement Localtypeparameterinferencepriority | spike | frontend/syntax | class: blocked | Implement Localtypeparameterinferencepriority |
| 3195 | Implement Logicalnotexpression | spike | frontend/resolver | class: blocked | Implement Logicalnotexpression |
| 3196 | Implement Longobjectinstantiationchain Name Resolution | spike | frontend/resolver | class: blocked | Implement Longobjectinstantiationchain Name Resolution |
| 3197 | Implement Longobjectinstantiationchain Parser Syntax | spike | frontend/syntax | class: blocked | Implement Longobjectinstantiationchain Parser Syntax |
| 3198 | Implement M | spike | frontend/syntax | class: triage-needed | Implement M |
| 3199 | Implement Manycompilererrorsinthetwofiles | spike | runtime/builtins | class: blocked | Implement Manycompilererrorsinthetwofiles |
| 3200 | Implement Manyconstexports | spike | frontend/syntax | class: blocked | Implement Manyconstexports |
| 3201 | Implement Mapconstructor | spike | frontend/syntax | class: triage-needed | Implement Mapconstructor |
| 3202 | Implement Mapconstructoronreadonlytuple | spike | frontend/semantics | class: blocked | Implement Mapconstructoronreadonlytuple |
| 3203 | Implement Mapgroupby | spike | frontend/syntax | class: triage-needed | Implement Mapgroupby |
| 3204 | Implement Mapontupletypes | spike | frontend/syntax | class: blocked | Implement Mapontupletypes |
| 3205 | Implement Mapupsert | spike | frontend/syntax | class: blocked | Implement Mapupsert |
| 3206 | Implement Mappedtotoindexsignatureinference | spike | frontend/syntax | class: blocked | Implement Mappedtotoindexsignatureinference |
| 3207 | Implement Mappedtypecontextualtypesapplied | spike | frontend/resolver | class: blocked | Implement Mappedtypecontextualtypesapplied |
| 3208 | Implement Mappedtypegenericindexedaccess | spike | frontend/syntax | class: blocked | Implement Mappedtypegenericindexedaccess |
| 3209 | Implement Mappedtypegenericinstantiationpreserveshomomorphism | spike | frontend/syntax | class: blocked | Implement Mappedtypegenericinstantiationpreserveshomomorphism |
| 3210 | Implement Mappedtypegenericinstantiationpreservesinlineform | spike | frontend/syntax | class: blocked | Implement Mappedtypegenericinstantiationpreservesinlineform |
| 3211 | Implement Mappedtypegenericwithknownkeys | spike | frontend/syntax | class: blocked | Implement Mappedtypegenericwithknownkeys |
| 3212 | Implement Mappedtypeindexedaccessconstraint | spike | frontend/syntax | class: triage-needed | Implement Mappedtypeindexedaccessconstraint |
| 3213 | Implement Mappedtypeinferencealiassubstitution | spike | frontend/syntax | class: blocked | Implement Mappedtypeinferencealiassubstitution |
| 3214 | Implement Mappedtypeinferencecircularity | spike | frontend/resolver | class: blocked | Implement Mappedtypeinferencecircularity |
| 3215 | Implement Mappedtypeinferencefromapparenttype | spike | frontend/resolver | class: blocked | Implement Mappedtypeinferencefromapparenttype |
| 3216 | Implement Mappedtypeinferencetomappedtype | spike | frontend/syntax | class: blocked | Implement Mappedtypeinferencetomappedtype |
| 3217 | Implement Mappedtypemultiinference | spike | frontend/syntax | class: blocked | Implement Mappedtypemultiinference |
| 3218 | Implement Mappedtypenestedgenericinstantiation | spike | frontend/resolver | class: blocked | Implement Mappedtypenestedgenericinstantiation |
| 3219 | Implement Mappedtypenotmistakenlyhomomorphic | spike | frontend/semantics | class: blocked | Implement Mappedtypenotmistakenlyhomomorphic |
| 3220 | Implement Mappedtypepartialconstraints | spike | frontend/semantics | class: blocked | Implement Mappedtypepartialconstraints |
| 3221 | Implement Mappedtypepartialnonhomomorphicbaseconstraint | spike | frontend/semantics | class: blocked | Implement Mappedtypepartialnonhomomorphicbaseconstraint |
| 3222 | Implement Mappedtyperecursiveinference Parser Syntax | spike | frontend/semantics | class: blocked | Implement Mappedtyperecursiveinference Parser Syntax |
| 3223 | Implement Mappedtyperecursiveinference Type System | spike | frontend/syntax | class: blocked | Implement Mappedtyperecursiveinference Type System |
| 3224 | Implement Mappedtypetupleconstraintassignability | spike | frontend/semantics | class: blocked | Implement Mappedtypetupleconstraintassignability |
| 3225 | Implement Mappedtypeunionconstraintupletreatedasarraylike | spike | frontend/syntax | class: triage-needed | Implement Mappedtypeunionconstraintupletreatedasarraylike |
| 3226 | Implement Mappedtypeunionconstraintinferences | spike | frontend/syntax | class: blocked | Implement Mappedtypeunionconstraintinferences |
| 3227 | Implement Mappedtypewithasclauseandlateboundproperty Name Resolution | spike | frontend/resolver | class: blocked | Implement Mappedtypewithasclauseandlateboundproperty Name Resolution |
| 3228 | Implement Mappedtypewithasclauseandlateboundproperty Parser Syntax | spike | frontend/semantics | class: blocked | Implement Mappedtypewithasclauseandlateboundproperty Parser Syntax |
| 3229 | Implement Mappedtypewithcombinedtypemappers | spike | frontend/semantics | class: blocked | Implement Mappedtypewithcombinedtypemappers |
| 3230 | Implement Mappedtypewithnameclauseappliedtoarraytype | spike | frontend/resolver | class: blocked | Implement Mappedtypewithnameclauseappliedtoarraytype |
| 3231 | Implement Matchreturntypeinallbranches | spike | frontend/syntax | class: blocked | Implement Matchreturntypeinallbranches |
| 3232 | Implement Maxconstraints | spike | frontend/syntax | class: triage-needed | Implement Maxconstraints |
| 3233 | Implement Maxnodemodulejsdepthdefaultstozero | spike | frontend/syntax | class: blocked | Implement Maxnodemodulejsdepthdefaultstozero |
| 3234 | Implement Maximum | spike | frontend/resolver | class: blocked | Implement Maximum |
| 3235 | Implement Memberaccessmustusemoduleinstances | spike | frontend/syntax | class: blocked | Implement Memberaccessmustusemoduleinstances |
| 3236 | Implement Memberoverride | spike | frontend/syntax | class: blocked | Implement Memberoverride |
| 3237 | Implement Memberscope | spike | frontend/syntax | class: blocked | Implement Memberscope |
| 3238 | Implement Membervariabledeclarations | spike | frontend/syntax | class: blocked | Implement Membervariabledeclarations |
| 3239 | Implement Mergemultipleinterfacesreexported | spike | frontend/resolver | class: blocked | Implement Mergemultipleinterfacesreexported |
| 3240 | Implement Mergesymbolreexportinterface | spike | frontend/syntax | class: blocked | Implement Mergesymbolreexportinterface |
| 3241 | Implement Mergesymbolreexportedtypealiasinstantiation | spike | frontend/syntax | class: blocked | Implement Mergesymbolreexportedtypealiasinstantiation |
| 3242 | Implement Mergesymbolrexportfunction | spike | frontend/syntax | class: blocked | Implement Mergesymbolrexportfunction |
| 3243 | Implement Mergewithimportednamespace | spike | frontend/syntax | class: blocked | Implement Mergewithimportednamespace |
| 3244 | Implement Mergewithimportedtype | spike | frontend/syntax | class: blocked | Implement Mergewithimportedtype |
| 3245 | Implement Mergedclassnamespacerecordcast | spike | frontend/syntax | class: blocked | Implement Mergedclassnamespacerecordcast |
| 3246 | Implement Mergedclasswithnamespaceprototype | spike | frontend/syntax | class: blocked | Implement Mergedclasswithnamespaceprototype |
| 3247 | Implement Mergeddeclarationexports | spike | frontend/syntax | class: blocked | Implement Mergeddeclarationexports |
| 3248 | Implement Mergeddeclarations Import Export | spike | frontend/syntax | class: blocked | Implement Mergeddeclarations Import Export |
| 3249 | Implement Mergeddeclarations Parser Syntax | spike | frontend/resolver | class: blocked | Implement Mergeddeclarations Parser Syntax |
| 3250 | Implement Mergedenumdeclarationcodegen | spike | frontend/resolver | class: blocked | Implement Mergedenumdeclarationcodegen |
| 3251 | Implement Mergedinstantiationassignment | spike | frontend/syntax | class: blocked | Implement Mergedinstantiationassignment |
| 3252 | Implement Mergedinterfacefrommultiplefiles | spike | frontend/syntax | class: blocked | Implement Mergedinterfacefrommultiplefiles |
| 3253 | Implement Mergedmoduledeclarationcodegen | spike | frontend/syntax | class: blocked | Implement Mergedmoduledeclarationcodegen |
| 3254 | Implement Mergedmoduledeclarationwithsharedexportedvar | spike | frontend/syntax | class: blocked | Implement Mergedmoduledeclarationwithsharedexportedvar |
| 3255 | Implement Metadataimporttype | spike | frontend/syntax | class: blocked | Implement Metadataimporttype |
| 3256 | Implement Metadataofclassfromalias | spike | frontend/syntax | class: blocked | Implement Metadataofclassfromalias |
| 3257 | Implement Metadataofclassfrommodule | spike | frontend/syntax | class: blocked | Implement Metadataofclassfrommodule |
| 3258 | Implement Metadataofeventalias | spike | frontend/syntax | class: blocked | Implement Metadataofeventalias |
| 3259 | Implement Metadataofstringliteral | spike | frontend/syntax | class: blocked | Implement Metadataofstringliteral |
| 3260 | Implement Metadataofunion | spike | frontend/syntax | class: blocked | Implement Metadataofunion |
| 3261 | Implement Metadataofunionwithnull | spike | frontend/syntax | class: blocked | Implement Metadataofunionwithnull |
| 3262 | Implement Metadatareferencedwithinfilteredunion | spike | frontend/syntax | class: blocked | Implement Metadatareferencedwithinfilteredunion |
| 3263 | Implement Methodchainerror | spike | frontend/syntax | class: blocked | Implement Methodchainerror |
| 3264 | Implement Methodcontaininglocalfunction | spike | frontend/syntax | class: blocked | Implement Methodcontaininglocalfunction |
| 3265 | Implement Methodsignaturedeclarationemit | spike | frontend/syntax | class: blocked | Implement Methodsignaturedeclarationemit |
| 3266 | Implement Mismatchedexplicittypeparameterandargumenttype | spike | frontend/syntax | class: blocked | Implement Mismatchedexplicittypeparameterandargumenttype |
| 3267 | Implement Mismatchedgenericarguments | spike | frontend/syntax | class: blocked | Implement Mismatchedgenericarguments |
| 3268 | Implement Missingargument | spike | frontend/syntax | class: triage-needed | Implement Missingargument |
| 3269 | Implement Missingclosebrace | spike | frontend/syntax | class: blocked | Implement Missingclosebrace |
| 3270 | Implement Missingclosebraceinobjectliteral | spike | frontend/syntax | class: blocked | Implement Missingclosebraceinobjectliteral |
| 3271 | Implement Missingclosebracketinarray | spike | frontend/syntax | class: blocked | Implement Missingclosebracketinarray |
| 3272 | Implement Missingcloseparenstatements | spike | frontend/syntax | class: blocked | Implement Missingcloseparenstatements |
| 3273 | Implement Missingcommaintemplatestringsarray | spike | frontend/syntax | class: blocked | Implement Missingcommaintemplatestringsarray |
| 3274 | Implement Missingdiscriminants | spike | frontend/syntax | class: blocked | Implement Missingdiscriminants |
| 3275 | Implement Missingdomelements | spike | frontend/resolver | class: blocked | Implement Missingdomelements |
| 3276 | Implement Missingfunctionimplementation | spike | frontend/syntax | class: blocked | Implement Missingfunctionimplementation |
| 3277 | Implement Missingimportaftermoduleimport | spike | frontend/syntax | class: blocked | Implement Missingimportaftermoduleimport |
| 3278 | Implement Missingmembererrorhasshortpath | spike | frontend/syntax | class: blocked | Implement Missingmembererrorhasshortpath |
| 3279 | Implement Missingpropertiesofclassexpression | spike | frontend/syntax | class: triage-needed | Implement Missingpropertiesofclassexpression |
| 3280 | Implement Missingreturnstatement | spike | frontend/syntax | class: blocked | Implement Missingreturnstatement |
| 3281 | Implement Missingself | spike | frontend/syntax | class: blocked | Implement Missingself |
| 3282 | Implement Missingsemicoloninmodulespecifier | spike | frontend/syntax | class: blocked | Implement Missingsemicoloninmodulespecifier |
| 3283 | Implement Missingtypearguments Arguments Object | spike | frontend/syntax | class: blocked | Implement Missingtypearguments Arguments Object |
| 3284 | Implement Missingtypearguments Import Export | spike | frontend/syntax | class: blocked | Implement Missingtypearguments Import Export |
| 3285 | Implement Misspelledjsdoctypedeftags | spike | frontend/resolver | class: blocked | Implement Misspelledjsdoctypedeftags |
| 3286 | Implement Misspellednewmetaproperty | spike | frontend/syntax | class: triage-needed | Implement Misspellednewmetaproperty |
| 3287 | Implement Mixedexports | spike | frontend/syntax | class: blocked | Implement Mixedexports |
| 3288 | Implement Mixedtypeenumcomparison | spike | frontend/syntax | class: blocked | Implement Mixedtypeenumcomparison |
| 3289 | Implement Mixinintersectionisvalidbasetype | spike | frontend/syntax | class: triage-needed | Implement Mixinintersectionisvalidbasetype |
| 3290 | Implement Mixinprivateandprotected | spike | frontend/semantics | class: blocked | Implement Mixinprivateandprotected |
| 3291 | Implement Mixingapparenttypeoverrides | spike | frontend/syntax | class: triage-needed | Implement Mixingapparenttypeoverrides |
| 3292 | Implement Mixingfunctionandambientmodule | spike | frontend/syntax | class: blocked | Implement Mixingfunctionandambientmodule |
| 3293 | Implement Mixingstaticandinstanceoverloads | spike | frontend/semantics | class: blocked | Implement Mixingstaticandinstanceoverloads |
| 3294 | Implement Modfunctioncrash | spike | frontend/syntax | class: blocked | Implement Modfunctioncrash |
| 3295 | Implement Modkeyword | spike | frontend/syntax | class: blocked | Implement Modkeyword |
| 3296 | Implement Modifieronparameter | spike | frontend/syntax | class: blocked | Implement Modifieronparameter |
| 3297 | Implement Modifierparencast | spike | frontend/syntax | class: blocked | Implement Modifierparencast |
| 3298 | Implement Modifiersinobjectliterals | spike | frontend/syntax | class: blocked | Implement Modifiersinobjectliterals |
| 3299 | Implement Modularizelibrary Name Resolution | spike | frontend/resolver | class: blocked | Implement Modularizelibrary Name Resolution |
| 3300 | Implement Modularizelibrary Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Modularizelibrary Unknown Unsupported |
| 3301 | Implement Module | spike | frontend/syntax | class: blocked | Implement Module |
| 3302 | Implement Modulealiasasfunctionargument | spike | frontend/syntax | class: blocked | Implement Modulealiasasfunctionargument |
| 3303 | Implement Modulealiasinterface | spike | frontend/syntax | class: blocked | Implement Modulealiasinterface |
| 3304 | Implement Moduleandinterfacesharingname | spike | frontend/syntax | class: blocked | Implement Moduleandinterfacesharingname |
| 3305 | Implement Moduleandinterfacewithsamename | spike | frontend/syntax | class: blocked | Implement Moduleandinterfacewithsamename |
| 3306 | Implement Moduleasbasetype | spike | frontend/syntax | class: blocked | Implement Moduleasbasetype |
| 3307 | Implement Moduleassignmentcompat | spike | frontend/syntax | class: blocked | Implement Moduleassignmentcompat |
| 3308 | Implement Moduleaugmentationcollidingnamesinaugmentation | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationcollidingnamesinaugmentation |
| 3309 | Implement Moduleaugmentationdeclarationemit | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationdeclarationemit |
| 3310 | Implement Moduleaugmentationdisallowedextensions | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationdisallowedextensions |
| 3311 | Implement Moduleaugmentationdoesinterfacemergeofreexport | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationdoesinterfacemergeofreexport |
| 3312 | Implement Moduleaugmentationdoesnamespaceenummergeofreexport | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationdoesnamespaceenummergeofreexport |
| 3313 | Implement Moduleaugmentationdoesnamespacemergeofreexport | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationdoesnamespacemergeofreexport |
| 3314 | Implement Moduleaugmentationduringsyntheticdefaultcheck | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationduringsyntheticdefaultcheck |
| 3315 | Implement Moduleaugmentationenumclassmergeofreexportiserror | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationenumclassmergeofreexportiserror |
| 3316 | Implement Moduleaugmentationextendambientmodule | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationextendambientmodule |
| 3317 | Implement Moduleaugmentationextendfilemodule | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationextendfilemodule |
| 3318 | Implement Moduleaugmentationglobal Import Export | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationglobal Import Export |
| 3319 | Implement Moduleaugmentationglobal Parser Syntax | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationglobal Parser Syntax |
| 3320 | Implement Moduleaugmentationimportsandexports | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationimportsandexports |
| 3321 | Implement Moduleaugmentationinambientmodule | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationinambientmodule |
| 3322 | Implement Moduleaugmentationindependency | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationindependency |
| 3323 | Implement Moduleaugmentationnonewnames | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationnonewnames |
| 3324 | Implement Moduleaugmentationofalias | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationofalias |
| 3325 | Implement Moduleaugmentationwithnonexistentnamedimport | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationwithnonexistentnamedimport |
| 3326 | Implement Moduleaugmentationsbundledoutput | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationsbundledoutput |
| 3327 | Implement Moduleaugmentationsimports | spike | frontend/syntax | class: blocked | Implement Moduleaugmentationsimports |
| 3328 | Implement Moduleclassarraycodegentest | spike | frontend/syntax | class: blocked | Implement Moduleclassarraycodegentest |
| 3329 | Implement Modulecodegentest | spike | frontend/syntax | class: blocked | Implement Modulecodegentest |
| 3330 | Implement Modulecodegentest | spike | frontend/syntax | class: blocked | Implement Modulecodegentest |
| 3331 | Implement Modulecrashbug | spike | frontend/syntax | class: blocked | Implement Modulecrashbug |
| 3332 | Implement Moduledeclarationexportstarshadowingglobalisnameable | spike | frontend/resolver | class: blocked | Implement Moduledeclarationexportstarshadowingglobalisnameable |
| 3333 | Implement Moduledetectionisolatedmodulescjsfilescope | spike | reference/triage | class: triage-needed | Implement Moduledetectionisolatedmodulescjsfilescope |
| 3334 | Implement Moduleduplicateidentifiers | spike | frontend/syntax | class: blocked | Implement Moduleduplicateidentifiers |
| 3335 | Implement Moduleelementsinwrongcontext | spike | frontend/syntax | class: blocked | Implement Moduleelementsinwrongcontext |
| 3336 | Implement Moduleexportnonstructured | spike | frontend/syntax | class: blocked | Implement Moduleexportnonstructured |
| 3337 | Implement Moduleexports | spike | frontend/syntax | class: blocked | Implement Moduleexports |
| 3338 | Implement Moduleexportstypenoexcesspropertycheckfromcontainedliteral | spike | frontend/syntax | class: blocked | Implement Moduleexportstypenoexcesspropertycheckfromcontainedliteral |
| 3339 | Implement Moduleexportsunaryexpression | spike | frontend/syntax | class: blocked | Implement Moduleexportsunaryexpression |
| 3340 | Implement Moduleidentifiers | spike | frontend/syntax | class: blocked | Implement Moduleidentifiers |
| 3341 | Implement Moduleimport | spike | frontend/syntax | class: blocked | Implement Moduleimport |
| 3342 | Implement Moduleimportedfortypeargumentposition | spike | frontend/syntax | class: blocked | Implement Moduleimportedfortypeargumentposition |
| 3343 | Implement Moduleintypeposition | spike | frontend/syntax | class: blocked | Implement Moduleintypeposition |
| 3344 | Implement Modulekeyworddeprecated | spike | frontend/syntax | class: blocked | Implement Modulekeyworddeprecated |
| 3345 | Implement Modulekeywordrepeaterror | spike | frontend/syntax | class: blocked | Implement Modulekeywordrepeaterror |
| 3346 | Implement Modulelocalimportnotincorrectlyredirected | spike | frontend/syntax | class: blocked | Implement Modulelocalimportnotincorrectlyredirected |
| 3347 | Implement Modulemembermissingerrorisrelative | spike | frontend/syntax | class: blocked | Implement Modulemembermissingerrorisrelative |
| 3348 | Implement Modulememberwithouttypeannotation | spike | frontend/syntax | class: blocked | Implement Modulememberwithouttypeannotation |
| 3349 | Implement Modulemerge | spike | frontend/syntax | class: blocked | Implement Modulemerge |
| 3350 | Implement Modulemergeconstructor | spike | frontend/syntax | class: blocked | Implement Modulemergeconstructor |
| 3351 | Implement Modulenewexportbug | spike | frontend/syntax | class: blocked | Implement Modulenewexportbug |
| 3352 | Implement Modulenoemit | spike | frontend/syntax | class: blocked | Implement Modulenoemit |
| 3353 | Implement Modulenodedefaultimports | spike | frontend/syntax | class: blocked | Implement Modulenodedefaultimports |
| 3354 | Implement Modulenodeimportrequireemit | spike | frontend/syntax | class: blocked | Implement Modulenodeimportrequireemit |
| 3355 | Implement Modulenonedynamicimport | spike | frontend/syntax | class: blocked | Implement Modulenonedynamicimport |
| 3356 | Implement Modulenoneerrors | spike | frontend/syntax | class: blocked | Implement Modulenoneerrors |
| 3357 | Implement Moduleouterqualification | spike | frontend/syntax | class: blocked | Implement Moduleouterqualification |
| 3358 | Implement Modulepreserve | spike | frontend/syntax | class: blocked | Implement Modulepreserve |
| 3359 | Implement Modulepreserveimporthelpers | spike | frontend/syntax | class: blocked | Implement Modulepreserveimporthelpers |
| 3360 | Implement Modulepreservetoplevelawait | spike | reference/triage | class: triage-needed | Implement Modulepreservetoplevelawait |
| 3361 | Implement Moduleprologueamd | spike | frontend/syntax | class: blocked | Implement Moduleprologueamd |
| 3362 | Implement Moduleprologuecommonjs | spike | frontend/syntax | class: blocked | Implement Moduleprologuecommonjs |
| 3363 | Implement Moduleprologuees | spike | frontend/syntax | class: blocked | Implement Moduleprologuees |
| 3364 | Implement Moduleprologuesystem | spike | frontend/syntax | class: blocked | Implement Moduleprologuesystem |
| 3365 | Implement Moduleprologueumd | spike | frontend/syntax | class: blocked | Implement Moduleprologueumd |
| 3366 | Implement Moduleproperty | spike | frontend/syntax | class: blocked | Implement Moduleproperty |
| 3367 | Implement Moduleredifinitionerrors | spike | frontend/syntax | class: blocked | Implement Moduleredifinitionerrors |
| 3368 | Implement Modulereopenedtypeotherblock | spike | frontend/syntax | class: blocked | Implement Modulereopenedtypeotherblock |
| 3369 | Implement Modulereopenedtypesameblock | spike | frontend/syntax | class: blocked | Implement Modulereopenedtypesameblock |
| 3370 | Implement Moduleresolution Import Export | spike | frontend/syntax | class: blocked | Implement Moduleresolution Import Export |
| 3371 | Implement Moduleresolution Module Resolution | spike | frontend/syntax | class: blocked | Implement Moduleresolution Module Resolution |
| 3372 | Implement Moduleresolution Name Resolution | spike | frontend/resolver | class: blocked | Implement Moduleresolution Name Resolution |
| 3373 | Implement Moduleresolutionastypereferencedirective | spike | frontend/syntax | class: blocked | Implement Moduleresolutionastypereferencedirective |
| 3374 | Implement Moduleresolutionastypereferencedirectiveambient | spike | frontend/syntax | class: blocked | Implement Moduleresolutionastypereferencedirectiveambient |
| 3375 | Implement Moduleresolutionastypereferencedirectivescoped | spike | frontend/syntax | class: blocked | Implement Moduleresolutionastypereferencedirectivescoped |
| 3376 | Implement Moduleresolutionnoresolve | spike | frontend/syntax | class: blocked | Implement Moduleresolutionnoresolve |
| 3377 | Implement Moduleresolutionnotscjs | spike | frontend/syntax | class: blocked | Implement Moduleresolutionnotscjs |
| 3378 | Implement Moduleresolutionnotsesm | spike | frontend/syntax | class: blocked | Implement Moduleresolutionnotsesm |
| 3379 | Implement Moduleresolutionpackageidwithrelativeandabsolutepath | spike | frontend/syntax | class: blocked | Implement Moduleresolutionpackageidwithrelativeandabsolutepath |
| 3380 | Implement Moduleresolutionwithextensions Import Export | spike | frontend/syntax | class: blocked | Implement Moduleresolutionwithextensions Import Export |
| 3381 | Implement Moduleresolutionwithextensions Module Resolution | spike | frontend/syntax | class: blocked | Implement Moduleresolutionwithextensions Module Resolution |
| 3382 | Implement Moduleresolutionwithmodule | spike | frontend/syntax | class: blocked | Implement Moduleresolutionwithmodule |
| 3383 | Implement Moduleresolutionwithrequire | spike | frontend/syntax | class: blocked | Implement Moduleresolutionwithrequire |
| 3384 | Implement Moduleresolutionwithrequireandimport | spike | frontend/syntax | class: blocked | Implement Moduleresolutionwithrequireandimport |
| 3385 | Implement Moduleresolutionwithsuffixes Import Export | spike | frontend/syntax | class: blocked | Implement Moduleresolutionwithsuffixes Import Export |
| 3386 | Implement Moduleresolutionwithsuffixes Module Resolution | spike | frontend/syntax | class: blocked | Implement Moduleresolutionwithsuffixes Module Resolution |
| 3387 | Implement Moduleresolutionwithsymlinks Import Export | spike | frontend/syntax | class: blocked | Implement Moduleresolutionwithsymlinks Import Export |
| 3388 | Implement Moduleresolutionwithsymlinks Parser Syntax | spike | frontend/resolver | class: blocked | Implement Moduleresolutionwithsymlinks Parser Syntax |
| 3389 | Implement Modulesamevalueduplicateexportedbindings | spike | frontend/syntax | class: blocked | Implement Modulesamevalueduplicateexportedbindings |
| 3390 | Implement Modulescopingbug | spike | frontend/syntax | class: blocked | Implement Modulescopingbug |
| 3391 | Implement Modulesharesnamewithimportdeclarationinsideit | spike | frontend/syntax | class: blocked | Implement Modulesharesnamewithimportdeclarationinsideit |
| 3392 | Implement Modulesymbolmerging | spike | frontend/syntax | class: blocked | Implement Modulesymbolmerging |
| 3393 | Implement Moduleunassignedvariable | spike | frontend/syntax | class: blocked | Implement Moduleunassignedvariable |
| 3394 | Implement Modulevariablearrayindexer | spike | frontend/syntax | class: blocked | Implement Modulevariablearrayindexer |
| 3395 | Implement Modulevariables | spike | frontend/syntax | class: blocked | Implement Modulevariables |
| 3396 | Implement Modulevisibilitytest | spike | frontend/syntax | class: blocked | Implement Modulevisibilitytest |
| 3397 | Implement Modulewithnovaluesastype | spike | frontend/syntax | class: blocked | Implement Modulewithnovaluesastype |
| 3398 | Implement Modulewithtrystatement | spike | frontend/syntax | class: blocked | Implement Modulewithtrystatement |
| 3399 | Implement Modulewithvaluesastype | spike | frontend/syntax | class: blocked | Implement Modulewithvaluesastype |
| 3400 | Implement Moduledecl | spike | frontend/syntax | class: blocked | Implement Moduledecl |
| 3401 | Implement Multicalloverloads | spike | frontend/semantics | class: blocked | Implement Multicalloverloads |
| 3402 | Implement Multiextendssplitinterfaces | spike | frontend/resolver | class: blocked | Implement Multiextendssplitinterfaces |
| 3403 | Implement Multiimportexport | spike | frontend/syntax | class: blocked | Implement Multiimportexport |
| 3404 | Implement Multilineerrors | spike | runtime/builtins | class: blocked | Implement Multilineerrors |
| 3405 | Implement Multilinepropertyaccessandarrowfunctionindent | spike | reference/triage | class: triage-needed | Implement Multilinepropertyaccessandarrowfunctionindent |
| 3406 | Implement Multimoduleclodule | spike | frontend/syntax | class: blocked | Implement Multimoduleclodule |
| 3407 | Implement Multimodulefundule | spike | frontend/syntax | class: blocked | Implement Multimodulefundule |
| 3408 | Implement Multiplebaseinterfaeswithincompatibleproperties | spike | frontend/syntax | class: blocked | Implement Multiplebaseinterfaeswithincompatibleproperties |
| 3409 | Implement Multipleclasspropertymodifiers | spike | frontend/syntax | class: blocked | Implement Multipleclasspropertymodifiers |
| 3410 | Implement Multipleclasspropertymodifierserrors | spike | runtime/builtins | class: blocked | Implement Multipleclasspropertymodifierserrors |
| 3411 | Implement Multipleexportassignments | spike | frontend/syntax | class: blocked | Implement Multipleexportassignments |
| 3412 | Implement Multipleexportassignmentsinambientdeclaration | spike | frontend/syntax | class: blocked | Implement Multipleexportassignmentsinambientdeclaration |
| 3413 | Implement Multipleexports | spike | frontend/syntax | class: blocked | Implement Multipleexports |
| 3414 | Implement Multipleinferencecontexts | spike | frontend/resolver | class: blocked | Implement Multipleinferencecontexts |
| 3415 | Implement Multipleinheritance | spike | frontend/syntax | class: blocked | Implement Multipleinheritance |
| 3416 | Implement Multivar | spike | frontend/syntax | class: blocked | Implement Multivar |
| 3417 | Implement Mutuallyrecursivecallbacks | spike | frontend/resolver | class: blocked | Implement Mutuallyrecursivecallbacks |
| 3418 | Implement Mutuallyrecursivegenericbasetypes | spike | frontend/syntax | class: blocked | Implement Mutuallyrecursivegenericbasetypes |
| 3419 | Implement Mutuallyrecursiveinference | spike | frontend/syntax | class: blocked | Implement Mutuallyrecursiveinference |
| 3420 | Implement Mutuallyrecursiveinterfacedeclaration | spike | frontend/syntax | class: blocked | Implement Mutuallyrecursiveinterfacedeclaration |
| 3421 | Implement Namecollisionwithblockscopedvariable | spike | frontend/syntax | class: blocked | Implement Namecollisionwithblockscopedvariable |
| 3422 | Implement Namecollisions | spike | frontend/syntax | class: blocked | Implement Namecollisions |
| 3423 | Implement Namecollisionsinpropertyassignments | spike | frontend/resolver | class: blocked | Implement Namecollisionsinpropertyassignments |
| 3424 | Implement Namedfunctionexpressionassignedtoclassproperty | spike | frontend/syntax | class: blocked | Implement Namedfunctionexpressionassignedtoclassproperty |
| 3425 | Implement Namedfunctionexpressioncall | spike | frontend/syntax | class: blocked | Implement Namedfunctionexpressioncall |
| 3426 | Implement Namedfunctionexpressioncallerrors | spike | frontend/resolver | class: blocked | Implement Namedfunctionexpressioncallerrors |
| 3427 | Implement Namedfunctionexpressioninmodule | spike | frontend/syntax | class: blocked | Implement Namedfunctionexpressioninmodule |
| 3428 | Implement Namedimportnonexistentname | spike | frontend/syntax | class: blocked | Implement Namedimportnonexistentname |
| 3429 | Implement Namespacedisambiguationinunion | spike | frontend/syntax | class: blocked | Implement Namespacedisambiguationinunion |
| 3430 | Implement Namespacemergedwithfunctionwithoverloadsusage | spike | frontend/syntax | class: blocked | Implement Namespacemergedwithfunctionwithoverloadsusage |
| 3431 | Implement Namespacemergedwithimportaliasnocrash | spike | frontend/syntax | class: blocked | Implement Namespacemergedwithimportaliasnocrash |
| 3432 | Implement Namespacenotmergedwithfunctiondefaultexport | spike | frontend/syntax | class: blocked | Implement Namespacenotmergedwithfunctiondefaultexport |
| 3433 | Implement Namespaces | spike | frontend/syntax | class: blocked | Implement Namespaces |
| 3434 | Implement Namespacesdeclaration | spike | frontend/syntax | class: blocked | Implement Namespacesdeclaration |
| 3435 | Implement Namespaceswithtypealiasonlyexportsmerge | spike | frontend/syntax | class: blocked | Implement Namespaceswithtypealiasonlyexportsmerge |
| 3436 | Implement Nanequality | spike | frontend/resolver | class: blocked | Implement Nanequality |
| 3437 | Implement Narrowbybooleancomparison | spike | frontend/semantics | class: blocked | Implement Narrowbybooleancomparison |
| 3438 | Implement Narrowbyclauseexpressioninswitchtrue Name Resolution | spike | frontend/resolver | class: blocked | Implement Narrowbyclauseexpressioninswitchtrue Name Resolution |
| 3439 | Implement Narrowbyclauseexpressioninswitchtrue Parser Syntax | spike | frontend/semantics | class: blocked | Implement Narrowbyclauseexpressioninswitchtrue Parser Syntax |
| 3440 | Implement Narrowbyclauseexpressioninswitchtrue Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Narrowbyclauseexpressioninswitchtrue Unknown Unsupported |
| 3441 | Implement Narrowbyequality | spike | frontend/semantics | class: blocked | Implement Narrowbyequality |
| 3442 | Implement Narrowbyinstanceof | spike | frontend/semantics | class: blocked | Implement Narrowbyinstanceof |
| 3443 | Implement Narrowbyparenthesizedswitchexpression | spike | frontend/semantics | class: blocked | Implement Narrowbyparenthesizedswitchexpression |
| 3444 | Implement Narrowbyswitchdiscriminantundefinedcase | spike | frontend/semantics | class: blocked | Implement Narrowbyswitchdiscriminantundefinedcase |
| 3445 | Implement Narrowcommaoperatornestedwithinlhs | spike | frontend/semantics | class: blocked | Implement Narrowcommaoperatornestedwithinlhs |
| 3446 | Implement Narrowrefinedconstlikeparameterbindingelementnameininnerscope | spike | frontend/syntax | class: blocked | Implement Narrowrefinedconstlikeparameterbindingelementnameininnerscope |
| 3447 | Implement Narrowswitchoptionalchaincontainmentevolvingarraynocrash | spike | frontend/semantics | class: blocked | Implement Narrowswitchoptionalchaincontainmentevolvingarraynocrash |
| 3448 | Implement Narrowtypebyinstanceof | spike | frontend/semantics | class: blocked | Implement Narrowtypebyinstanceof |
| 3449 | Implement Narrowunknownbytypepredicate | spike | frontend/resolver | class: blocked | Implement Narrowunknownbytypepredicate |
| 3450 | Implement Narrowedconstinmethod | spike | frontend/syntax | class: triage-needed | Implement Narrowedconstinmethod |
| 3451 | Implement Narrowedimports | spike | frontend/syntax | class: blocked | Implement Narrowedimports |
| 3452 | Implement Narrowingassignmentreadonlyrespectsassertion | spike | frontend/syntax | class: blocked | Implement Narrowingassignmentreadonlyrespectsassertion |
| 3453 | Implement Narrowingbytypeofinswitch | spike | frontend/syntax | class: blocked | Implement Narrowingbytypeofinswitch |
| 3454 | Implement Narrowingconstrainedtypeparameter | spike | frontend/syntax | class: blocked | Implement Narrowingconstrainedtypeparameter |
| 3455 | Implement Narrowingdestructuring | spike | reference/triage | class: triage-needed | Implement Narrowingdestructuring |
| 3456 | Implement Narrowingincaseclauseaftercaseclausewithreturn | spike | frontend/semantics | class: blocked | Implement Narrowingincaseclauseaftercaseclausewithreturn |
| 3457 | Implement Narrowingmutualsubtypes | spike | frontend/semantics | class: blocked | Implement Narrowingmutualsubtypes |
| 3458 | Implement Narrowingnoinfer | spike | frontend/syntax | class: blocked | Implement Narrowingnoinfer |
| 3459 | Implement Narrowingofdottednames | spike | frontend/semantics | class: blocked | Implement Narrowingofdottednames |
| 3460 | Implement Narrowingofqualifiednames | spike | frontend/semantics | class: blocked | Implement Narrowingofqualifiednames |
| 3461 | Implement Narrowingpastlastassignment | spike | frontend/syntax | class: blocked | Implement Narrowingpastlastassignment |
| 3462 | Implement Narrowingpastlastassignmentinmodule | spike | frontend/syntax | class: blocked | Implement Narrowingpastlastassignmentinmodule |
| 3463 | Implement Narrowingplainjsnocrash | spike | frontend/semantics | class: blocked | Implement Narrowingplainjsnocrash |
| 3464 | Implement Narrowingrestgenericcall | spike | frontend/syntax | class: blocked | Implement Narrowingrestgenericcall |
| 3465 | Implement Narrowingtruthyobject | spike | frontend/syntax | class: blocked | Implement Narrowingtruthyobject |
| 3466 | Implement Narrowingtypeofparenthesized | spike | frontend/resolver | class: blocked | Implement Narrowingtypeofparenthesized |
| 3467 | Implement Narrowingtypeofundefined Name Resolution | spike | frontend/resolver | class: blocked | Implement Narrowingtypeofundefined Name Resolution |
| 3468 | Implement Narrowingtypeofundefined Parser Syntax | spike | frontend/semantics | class: blocked | Implement Narrowingtypeofundefined Parser Syntax |
| 3469 | Implement Narrowinguniontounion | spike | frontend/syntax | class: blocked | Implement Narrowinguniontounion |
| 3470 | Implement Narrowingunionwithbang | spike | frontend/semantics | class: blocked | Implement Narrowingunionwithbang |
| 3471 | Implement Narrowingwithnonnullexpression | spike | frontend/semantics | class: blocked | Implement Narrowingwithnonnullexpression |
| 3472 | Implement Nativetoboxedtypes | spike | frontend/syntax | class: blocked | Implement Nativetoboxedtypes |
| 3473 | Implement Nearbyidenticalgenericlambdasassignable | spike | frontend/syntax | class: blocked | Implement Nearbyidenticalgenericlambdasassignable |
| 3474 | Implement Negativezero | spike | frontend/syntax | class: blocked | Implement Negativezero |
| 3475 | Implement Nestedblockscopedbindings | spike | frontend/syntax | class: blocked | Implement Nestedblockscopedbindings |
| 3476 | Implement Nestedcallbackerrornotflattened | spike | frontend/resolver | class: blocked | Implement Nestedcallbackerrornotflattened |
| 3477 | Implement Nestedexcesspropertychecking | spike | frontend/semantics | class: blocked | Implement Nestedexcesspropertychecking |
| 3478 | Implement Nestedfreshliteral | spike | frontend/syntax | class: blocked | Implement Nestedfreshliteral |
| 3479 | Implement Nestedgenericspreadinference | spike | frontend/syntax | class: blocked | Implement Nestedgenericspreadinference |
| 3480 | Implement Nestedglobalnamespaceinclass | spike | frontend/syntax | class: blocked | Implement Nestedglobalnamespaceinclass |
| 3481 | Implement Nestedindexer | spike | frontend/syntax | class: blocked | Implement Nestedindexer |
| 3482 | Implement Nestedloopwithonlyinnerletcaptured | spike | frontend/syntax | class: blocked | Implement Nestedloopwithonlyinnerletcaptured |
| 3483 | Implement Nestedloops | spike | frontend/syntax | class: blocked | Implement Nestedloops |
| 3484 | Implement Nestedmoduleprivateaccess | spike | frontend/syntax | class: blocked | Implement Nestedmoduleprivateaccess |
| 3485 | Implement Nestedobjectrest | spike | frontend/syntax | class: blocked | Implement Nestedobjectrest |
| 3486 | Implement Nestedrecursivelambda | spike | frontend/syntax | class: blocked | Implement Nestedrecursivelambda |
| 3487 | Implement Nestedredeclarationines | spike | frontend/syntax | class: blocked | Implement Nestedredeclarationines |
| 3488 | Implement Nestedself | spike | frontend/syntax | class: blocked | Implement Nestedself |
| 3489 | Implement Nestedsupercallemit | spike | frontend/resolver | class: blocked | Implement Nestedsupercallemit |
| 3490 | Implement Nestedthiscontainer | spike | reference/triage | class: triage-needed | Implement Nestedthiscontainer |
| 3491 | Implement Nestedtypevariableinfersliteral | spike | frontend/syntax | class: blocked | Implement Nestedtypevariableinfersliteral |
| 3492 | Implement Nestedunaryexpressionhang | spike | frontend/syntax | class: triage-needed | Implement Nestedunaryexpressionhang |
| 3493 | Implement Neverasdiscriminanttype | spike | frontend/syntax | class: blocked | Implement Neverasdiscriminanttype |
| 3494 | Implement Newabstractinstance Name Resolution | spike | frontend/resolver | class: blocked | Implement Newabstractinstance Name Resolution |
| 3495 | Implement Newabstractinstance Parser Syntax | spike | frontend/syntax | class: blocked | Implement Newabstractinstance Parser Syntax |
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
| 3513 | Implement Nocircularityselfreferentialgetter | spike | frontend/semantics | class: blocked | Implement Nocircularityselfreferentialgetter |
| 3514 | Implement Nocollisionthisexpressionandclassinglobal | spike | frontend/resolver | class: blocked | Implement Nocollisionthisexpressionandclassinglobal |
| 3515 | Implement Nocollisionthisexpressionandlocalvarinaccessors | spike | frontend/syntax | class: blocked | Implement Nocollisionthisexpressionandlocalvarinaccessors |
| 3516 | Implement Nocollisionthisexpressionandlocalvarinconstructor | spike | frontend/syntax | class: triage-needed | Implement Nocollisionthisexpressionandlocalvarinconstructor |
| 3517 | Implement Nocollisionthisexpressionandlocalvarinfunction | spike | frontend/resolver | class: blocked | Implement Nocollisionthisexpressionandlocalvarinfunction |
| 3518 | Implement Nocollisionthisexpressionandlocalvarinlambda | spike | frontend/syntax | class: triage-needed | Implement Nocollisionthisexpressionandlocalvarinlambda |
| 3519 | Implement Nocollisionthisexpressionandlocalvarinmethod | spike | frontend/syntax | class: triage-needed | Implement Nocollisionthisexpressionandlocalvarinmethod |
| 3520 | Implement Nocollisionthisexpressionandlocalvarinproperty | spike | frontend/resolver | class: blocked | Implement Nocollisionthisexpressionandlocalvarinproperty |
| 3521 | Implement Nocollisionthisexpressioninfunctionandvaringlobal | spike | frontend/syntax | class: blocked | Implement Nocollisionthisexpressioninfunctionandvaringlobal |
| 3522 | Implement Noconstraintinreturntype | spike | frontend/semantics | class: blocked | Implement Noconstraintinreturntype |
| 3523 | Implement Nocrashonimportshadowing | spike | reference/triage | class: triage-needed | Implement Nocrashonimportshadowing |
| 3524 | Implement Nocrashonmixin | spike | frontend/syntax | class: blocked | Implement Nocrashonmixin |
| 3525 | Implement Nocrashonnolib | spike | frontend/syntax | class: blocked | Implement Nocrashonnolib |
| 3526 | Implement Nocrashonthistypeusage | spike | frontend/syntax | class: blocked | Implement Nocrashonthistypeusage |
| 3527 | Implement Nocrashumdmergedwithglobalvalue | spike | frontend/syntax | class: blocked | Implement Nocrashumdmergedwithglobalvalue |
| 3528 | Implement Nocrashwithverbatimmodulesyntaxandimportsnotusedasvalues | spike | frontend/syntax | class: blocked | Implement Nocrashwithverbatimmodulesyntaxandimportsnotusedasvalues |
| 3529 | Implement Noemithelpers | spike | frontend/syntax | class: blocked | Implement Noemithelpers |
| 3530 | Implement Noerrorusingimportexportmoduleaugmentationindeclarationfile | spike | frontend/syntax | class: blocked | Implement Noerrorusingimportexportmoduleaugmentationindeclarationfile |
| 3531 | Implement Noerrorsincallback | spike | frontend/syntax | class: blocked | Implement Noerrorsincallback |
| 3532 | Implement Noexcessivestackdeptherror | spike | reference/triage | class: triage-needed | Implement Noexcessivestackdeptherror |
| 3533 | Implement Noimplicitanyandprivatememberswithouttypeannotations | spike | frontend/resolver | class: blocked | Implement Noimplicitanyandprivatememberswithouttypeannotations |
| 3534 | Implement Noimplicitanydestructuringinprivatemethod | spike | frontend/syntax | class: blocked | Implement Noimplicitanydestructuringinprivatemethod |
| 3535 | Implement Noimplicitanydestructuringvardeclaration | spike | frontend/syntax | class: blocked | Implement Noimplicitanydestructuringvardeclaration |
| 3536 | Implement Noimplicitanyforin | spike | frontend/resolver | class: blocked | Implement Noimplicitanyforin |
| 3537 | Implement Noimplicitanyformethodparameters | spike | frontend/resolver | class: blocked | Implement Noimplicitanyformethodparameters |
| 3538 | Implement Noimplicitanyfunctionexpressionassignment | spike | frontend/resolver | class: blocked | Implement Noimplicitanyfunctionexpressionassignment |
| 3539 | Implement Noimplicitanyfunctions | spike | frontend/resolver | class: blocked | Implement Noimplicitanyfunctions |
| 3540 | Implement Noimplicitanyincastexpression | spike | frontend/syntax | class: triage-needed | Implement Noimplicitanyincastexpression |
| 3541 | Implement Noimplicitanyincontextuallytypesfunctionparamter | spike | frontend/syntax | class: blocked | Implement Noimplicitanyincontextuallytypesfunctionparamter |
| 3542 | Implement Noimplicitanyindexing | spike | frontend/resolver | class: blocked | Implement Noimplicitanyindexing |
| 3543 | Implement Noimplicitanyindexingsuppressed | spike | frontend/resolver | class: blocked | Implement Noimplicitanyindexingsuppressed |
| 3544 | Implement Noimplicitanyloopcrash | spike | frontend/syntax | class: blocked | Implement Noimplicitanyloopcrash |
| 3545 | Implement Noimplicitanymissinggetaccessor | spike | frontend/syntax | class: blocked | Implement Noimplicitanymissinggetaccessor |
| 3546 | Implement Noimplicitanymissingsetaccessor | spike | frontend/syntax | class: blocked | Implement Noimplicitanymissingsetaccessor |
| 3547 | Implement Noimplicitanymodule | spike | frontend/syntax | class: blocked | Implement Noimplicitanymodule |
| 3548 | Implement Noimplicitanynamelessparameter | spike | frontend/resolver | class: blocked | Implement Noimplicitanynamelessparameter |
| 3549 | Implement Noimplicitanyparametersinambientmodule | spike | frontend/syntax | class: blocked | Implement Noimplicitanyparametersinambientmodule |
| 3550 | Implement Noimplicitanyparametersinclass | spike | frontend/resolver | class: blocked | Implement Noimplicitanyparametersinclass |
| 3551 | Implement Noimplicitanyparametersinmodule | spike | frontend/syntax | class: blocked | Implement Noimplicitanyparametersinmodule |
| 3552 | Implement Noimplicitanystringindexeronobject | spike | frontend/syntax | class: blocked | Implement Noimplicitanystringindexeronobject |
| 3553 | Implement Noimplicitanywithoverloads | spike | frontend/semantics | class: blocked | Implement Noimplicitanywithoverloads |
| 3554 | Implement Noimplicitreturnsexclusions | spike | frontend/syntax | class: triage-needed | Implement Noimplicitreturnsexclusions |
| 3555 | Implement Noimplicitreturnsinasync | spike | reference/triage | class: triage-needed | Implement Noimplicitreturnsinasync |
| 3556 | Implement Noimplicitreturnswithprotectedblocks | spike | frontend/resolver | class: blocked | Implement Noimplicitreturnswithprotectedblocks |
| 3557 | Implement Noimplicitsymboltostring | spike | frontend/syntax | class: blocked | Implement Noimplicitsymboltostring |
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
| 3568 | Implement Notypeargumentonreturntype | spike | frontend/semantics | class: blocked | Implement Notypeargumentonreturntype |
| 3569 | Implement Nouncheckedindexaccess | spike | frontend/syntax | class: blocked | Implement Nouncheckedindexaccess |
| 3570 | Implement Nouncheckedindexedaccesscompoundassignments | spike | frontend/semantics | class: blocked | Implement Nouncheckedindexedaccesscompoundassignments |
| 3571 | Implement Nounusedlocals Destructuring | spike | frontend/syntax | class: blocked | Implement Nounusedlocals Destructuring |
| 3572 | Implement Nounusedlocals Import Export | spike | frontend/syntax | class: blocked | Implement Nounusedlocals Import Export |
| 3573 | Implement Nounusedlocals Name Resolution | spike | frontend/resolver | class: blocked | Implement Nounusedlocals Name Resolution |
| 3574 | Implement Nounusedlocals Parser Syntax | spike | frontend/resolver | class: blocked | Implement Nounusedlocals Parser Syntax |
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
| 3592 | Implement Nongenericclassextendinggenericclasswithany | spike | frontend/syntax | class: blocked | Implement Nongenericclassextendinggenericclasswithany |
| 3593 | Implement Nonidenticaltypeconstraints | spike | frontend/semantics | class: blocked | Implement Nonidenticaltypeconstraints |
| 3594 | Implement Noninferrabletypepropagation Parser Syntax | spike | frontend/syntax | class: blocked | Implement Noninferrabletypepropagation Parser Syntax |
| 3595 | Implement Noninferrabletypepropagation Type System | spike | frontend/syntax | class: blocked | Implement Noninferrabletypepropagation Type System |
| 3596 | Implement Nonmergedoverloads | spike | frontend/syntax | class: blocked | Implement Nonmergedoverloads |
| 3597 | Implement Nonnullfullinference | spike | frontend/syntax | class: blocked | Implement Nonnullfullinference |
| 3598 | Implement Nonnullmappedtype | spike | frontend/semantics | class: blocked | Implement Nonnullmappedtype |
| 3599 | Implement Nonnullparameterextendingstringassignabletostring | spike | frontend/semantics | class: blocked | Implement Nonnullparameterextendingstringassignabletostring |
| 3600 | Implement Nonnullreferencematching | spike | frontend/semantics | class: blocked | Implement Nonnullreferencematching |
| 3601 | Implement Nonnullablereduction | spike | frontend/semantics | class: blocked | Implement Nonnullablereduction |
| 3602 | Implement Nonnullablereductionnonstrict | spike | frontend/semantics | class: blocked | Implement Nonnullablereductionnonstrict |
| 3603 | Implement Nonnullabletypes | spike | frontend/semantics | class: blocked | Implement Nonnullabletypes |
| 3604 | Implement Nonnullablewithnullablegenericindexedaccessarg | spike | frontend/syntax | class: blocked | Implement Nonnullablewithnullablegenericindexedaccessarg |
| 3605 | Implement Nongenericpartialinstantiationsrelatedinbothdirections | spike | frontend/resolver | class: blocked | Implement Nongenericpartialinstantiationsrelatedinbothdirections |
| 3606 | Implement Nonnullassertionpropegatescontextualtype | spike | frontend/syntax | class: blocked | Implement Nonnullassertionpropegatescontextualtype |
| 3607 | Implement Normalizedintersectiontoocomplex | spike | frontend/resolver | class: blocked | Implement Normalizedintersectiontoocomplex |
| 3608 | Implement Nounusedtypeparameterconstraint | spike | frontend/syntax | class: blocked | Implement Nounusedtypeparameterconstraint |
| 3609 | Implement Nullablefunctionerror | spike | frontend/syntax | class: blocked | Implement Nullablefunctionerror |
| 3610 | Implement Numberassignabletoenuminsideunion | spike | runtime/builtins | class: blocked | Implement Numberassignabletoenuminsideunion |
| 3611 | Implement Numberliteralswithleadingzeros | spike | frontend/syntax | class: triage-needed | Implement Numberliteralswithleadingzeros |
| 3612 | Implement Numbervsbigintoperations | spike | runtime/builtins | class: blocked | Implement Numbervsbigintoperations |
| 3613 | Implement Numericclassmembers | spike | frontend/syntax | class: blocked | Implement Numericclassmembers |
| 3614 | Implement Numericenummappedtype | spike | frontend/semantics | class: blocked | Implement Numericenummappedtype |
| 3615 | Implement Numericindexexpressions | spike | frontend/resolver | class: blocked | Implement Numericindexexpressions |
| 3616 | Implement Numericindexerconstraint Name Resolution | spike | frontend/resolver | class: blocked | Implement Numericindexerconstraint Name Resolution |
| 3617 | Implement Numericindexerconstraint Parser Syntax | spike | frontend/semantics | class: blocked | Implement Numericindexerconstraint Parser Syntax |
| 3618 | Implement Numericindexertyping Name Resolution | spike | frontend/resolver | class: blocked | Implement Numericindexertyping Name Resolution |
| 3619 | Implement Numericindexertyping Parser Syntax | spike | frontend/syntax | class: blocked | Implement Numericindexertyping Parser Syntax |
| 3620 | Implement Numericliteralswithtrailingdecimalpoints | spike | frontend/syntax | class: blocked | Implement Numericliteralswithtrailingdecimalpoints |
| 3621 | Implement Numericmethodname | spike | frontend/syntax | class: blocked | Implement Numericmethodname |
| 3622 | Implement Numericunderscoredseparator | spike | frontend/syntax | class: blocked | Implement Numericunderscoredseparator |
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
| 3643 | Implement Objectliteralenumpropertynames | spike | frontend/syntax | class: blocked | Implement Objectliteralenumpropertynames |
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
| 3666 | Implement Observableinferencecanbemade | spike | frontend/syntax | class: blocked | Implement Observableinferencecanbemade |
| 3667 | Implement Octalliteralandescapesequence | spike | frontend/syntax | class: blocked | Implement Octalliteralandescapesequence |
| 3668 | Implement Omittypetesterrors | spike | frontend/syntax | class: blocked | Implement Omittypetesterrors |
| 3669 | Implement Omittypetests | spike | frontend/syntax | class: blocked | Implement Omittypetests |
| 3670 | Implement Omittedexpressionforofloop | spike | frontend/syntax | class: triage-needed | Implement Omittedexpressionforofloop |
| 3671 | Implement Operationsavailableonpromisedtype | spike | reference/triage | class: triage-needed | Implement Operationsavailableonpromisedtype |
| 3672 | Implement Operatoraddnullundefined | spike | frontend/syntax | class: blocked | Implement Operatoraddnullundefined |
| 3673 | Implement Optionalaccessorsininterface | spike | frontend/resolver | class: blocked | Implement Optionalaccessorsininterface |
| 3674 | Implement Optionalargswithdefaultvalues | spike | frontend/syntax | class: blocked | Implement Optionalargswithdefaultvalues |
| 3675 | Implement Optionalchainwithinstantiationexpression Import Export | spike | frontend/syntax | class: blocked | Implement Optionalchainwithinstantiationexpression Import Export |
| 3676 | Implement Optionalchainwithinstantiationexpression Parser Syntax | spike | frontend/syntax | class: blocked | Implement Optionalchainwithinstantiationexpression Parser Syntax |
| 3677 | Implement Optionalconstructorarginsuper | spike | frontend/syntax | class: blocked | Implement Optionalconstructorarginsuper |
| 3678 | Implement Optionalfunctionargassignability | spike | reference/triage | class: triage-needed | Implement Optionalfunctionargassignability |
| 3679 | Implement Optionalparamargstest | spike | frontend/syntax | class: blocked | Implement Optionalparamargstest |
| 3680 | Implement Optionalparamassignmentcompat | spike | frontend/resolver | class: blocked | Implement Optionalparamassignmentcompat |
| 3681 | Implement Optionalparaminoverride | spike | frontend/syntax | class: blocked | Implement Optionalparaminoverride |
| 3682 | Implement Optionalparamreferencingotherparams | spike | frontend/resolver | class: blocked | Implement Optionalparamreferencingotherparams |
| 3683 | Implement Optionalparamtypecomparison | spike | frontend/resolver | class: blocked | Implement Optionalparamtypecomparison |
| 3684 | Implement Optionalparameterindestructuringwithinitializer | spike | frontend/resolver | class: blocked | Implement Optionalparameterindestructuringwithinitializer |
| 3685 | Implement Optionalparameterretainsnull | spike | reference/triage | class: triage-needed | Implement Optionalparameterretainsnull |
| 3686 | Implement Optionalparamterandvariabledeclaration | spike | reference/triage | class: triage-needed | Implement Optionalparamterandvariabledeclaration |
| 3687 | Implement Optionalpropertiesinclasses | spike | frontend/syntax | class: blocked | Implement Optionalpropertiesinclasses |
| 3688 | Implement Optionalpropertiestest | spike | frontend/syntax | class: blocked | Implement Optionalpropertiestest |
| 3689 | Implement Optionalsetterparam | spike | frontend/semantics | class: blocked | Implement Optionalsetterparam |
| 3690 | Implement Optionaltupleelementsandundefined | spike | reference/triage | class: triage-needed | Implement Optionaltupleelementsandundefined |
| 3691 | Implement Optionsoutandnomodulegen | spike | frontend/syntax | class: blocked | Implement Optionsoutandnomodulegen |
| 3692 | Implement Ordermattersforsignaturegroupidentity | spike | frontend/resolver | class: blocked | Implement Ordermattersforsignaturegroupidentity |
| 3693 | Implement Out | spike | frontend/syntax | class: blocked | Implement Out |
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
| 3705 | Implement Overload Parser Syntax | spike | frontend/semantics | class: blocked | Implement Overload Parser Syntax |
| 3706 | Implement Overloadassignmentcompat | spike | frontend/resolver | class: blocked | Implement Overloadassignmentcompat |
| 3707 | Implement Overloadbindingacrossdeclarationboundaries | spike | reference/triage | class: triage-needed | Implement Overloadbindingacrossdeclarationboundaries |
| 3708 | Implement Overloadcalltest | spike | reference/triage | class: triage-needed | Implement Overloadcalltest |
| 3709 | Implement Overloadconsecutiveness | spike | frontend/semantics | class: blocked | Implement Overloadconsecutiveness |
| 3710 | Implement Overloadcrash | spike | frontend/resolver | class: blocked | Implement Overloadcrash |
| 3711 | Implement Overloadequivalencewithstatics | spike | frontend/semantics | class: blocked | Implement Overloadequivalencewithstatics |
| 3712 | Implement Overloaderrormatchesimplementationelaboaration | spike | frontend/resolver | class: blocked | Implement Overloaderrormatchesimplementationelaboaration |
| 3713 | Implement Overloadgenericfunctionwithrestargs | spike | frontend/syntax | class: blocked | Implement Overloadgenericfunctionwithrestargs |
| 3714 | Implement Overloadmodifiersmustagree | spike | frontend/semantics | class: blocked | Implement Overloadmodifiersmustagree |
| 3715 | Implement Overloadonconstconstraintchecks | spike | frontend/semantics | class: blocked | Implement Overloadonconstconstraintchecks |
| 3716 | Implement Overloadonconstduplicateoverloads | spike | frontend/semantics | class: blocked | Implement Overloadonconstduplicateoverloads |
| 3717 | Implement Overloadonconstinbasewithbadimplementationinderived | spike | frontend/semantics | class: blocked | Implement Overloadonconstinbasewithbadimplementationinderived |
| 3718 | Implement Overloadonconstincallback | spike | frontend/semantics | class: blocked | Implement Overloadonconstincallback |
| 3719 | Implement Overloadonconstinheritance | spike | frontend/semantics | class: blocked | Implement Overloadonconstinheritance |
| 3720 | Implement Overloadonconstnoanyimplementation | spike | frontend/semantics | class: blocked | Implement Overloadonconstnoanyimplementation |
| 3721 | Implement Overloadonconstnononspecializedsignature | spike | frontend/semantics | class: blocked | Implement Overloadonconstnononspecializedsignature |
| 3722 | Implement Overloadonconstnostringimplementation | spike | frontend/semantics | class: blocked | Implement Overloadonconstnostringimplementation |
| 3723 | Implement Overloadongenericclassandnongenericclass | spike | frontend/syntax | class: blocked | Implement Overloadongenericclassandnongenericclass |
| 3724 | Implement Overloadresolutionondefaultconstructor | spike | frontend/syntax | class: blocked | Implement Overloadresolutionondefaultconstructor |
| 3725 | Implement Overloadresolutionovernonctlambdas | spike | frontend/syntax | class: blocked | Implement Overloadresolutionovernonctlambdas |
| 3726 | Implement Overloadresolutionovernonctobjectlit | spike | frontend/syntax | class: blocked | Implement Overloadresolutionovernonctobjectlit |
| 3727 | Implement Overloadresolutionwithany | spike | frontend/syntax | class: blocked | Implement Overloadresolutionwithany |
| 3728 | Implement Overloadreturntypes | spike | frontend/resolver | class: blocked | Implement Overloadreturntypes |
| 3729 | Implement Overloadwithcallbackswithdifferingoptionalityonargs | spike | frontend/syntax | class: blocked | Implement Overloadwithcallbackswithdifferingoptionalityonargs |
| 3730 | Implement Overloadedconstructorfixesinferencesappropriately | spike | frontend/syntax | class: blocked | Implement Overloadedconstructorfixesinferencesappropriately |
| 3731 | Implement Overloadedstaticmethodspecialization | spike | frontend/semantics | class: blocked | Implement Overloadedstaticmethodspecialization |
| 3732 | Implement Overloadingonconstants | spike | frontend/semantics | class: blocked | Implement Overloadingonconstants |
| 3733 | Implement Overloadingonconstantsinimplementation | spike | frontend/semantics | class: blocked | Implement Overloadingonconstantsinimplementation |
| 3734 | Implement Overloadingstaticfunctionsinfunctions | spike | frontend/semantics | class: blocked | Implement Overloadingstaticfunctionsinfunctions |
| 3735 | Implement Overloadresolutionwithconstraintcheckingdeferred | spike | frontend/syntax | class: blocked | Implement Overloadresolutionwithconstraintcheckingdeferred |
| 3736 | Implement Overloadsandtypeargumentarity | spike | frontend/semantics | class: blocked | Implement Overloadsandtypeargumentarity |
| 3737 | Implement Overloadsandtypeargumentarityerrors | spike | frontend/semantics | class: blocked | Implement Overloadsandtypeargumentarityerrors |
| 3738 | Implement Overloadsindifferentcontainersdisagreeonambient | spike | frontend/syntax | class: blocked | Implement Overloadsindifferentcontainersdisagreeonambient |
| 3739 | Implement Overloadswithcomputednames | spike | frontend/semantics | class: blocked | Implement Overloadswithcomputednames |
| 3740 | Implement Overloadswithprovisionalerrors | spike | frontend/resolver | class: blocked | Implement Overloadswithprovisionalerrors |
| 3741 | Implement Overloadswithinclasses | spike | frontend/semantics | class: blocked | Implement Overloadswithinclasses |
| 3742 | Implement Overridebaseintersectionmethod | spike | frontend/syntax | class: triage-needed | Implement Overridebaseintersectionmethod |
| 3743 | Implement Overridingprivatestaticmembers | spike | frontend/semantics | class: blocked | Implement Overridingprivatestaticmembers |
| 3744 | Implement Overshifts | spike | frontend/syntax | class: blocked | Implement Overshifts |
| 3745 | Implement Parampropertiesinsignatures | spike | frontend/syntax | class: blocked | Implement Parampropertiesinsignatures |
| 3746 | Implement Parameterdecoratorsemitcrash | spike | frontend/syntax | class: blocked | Implement Parameterdecoratorsemitcrash |
| 3747 | Implement Parameterdestructuringobjectliteral | spike | reference/triage | class: triage-needed | Implement Parameterdestructuringobjectliteral |
| 3748 | Implement Parameterinitializerbeforedestructuringemit | spike | reference/triage | class: triage-needed | Implement Parameterinitializerbeforedestructuringemit |
| 3749 | Implement Parameterpropertyinconstructor | spike | frontend/syntax | class: blocked | Implement Parameterpropertyinconstructor |
| 3750 | Implement Parameterpropertyinconstructorwithprologues | spike | frontend/syntax | class: blocked | Implement Parameterpropertyinconstructorwithprologues |
| 3751 | Implement Parameterpropertyoutsideconstructor | spike | frontend/syntax | class: blocked | Implement Parameterpropertyoutsideconstructor |
| 3752 | Implement Parameterreferenceininitializer | spike | frontend/syntax | class: blocked | Implement Parameterreferenceininitializer |
| 3753 | Implement Parameterreferencesotherparameter | spike | frontend/syntax | class: blocked | Implement Parameterreferencesotherparameter |
| 3754 | Implement Parameterssyntaxerrornocrash Import Export | spike | frontend/syntax | class: blocked | Implement Parameterssyntaxerrornocrash Import Export |
| 3755 | Implement Parameterssyntaxerrornocrash Parser Syntax | spike | runtime/builtins | class: blocked | Implement Parameterssyntaxerrornocrash Parser Syntax |
| 3756 | Implement Paramsonlyhaveliteraltypeswhenappropriatelycontextualized | spike | frontend/syntax | class: blocked | Implement Paramsonlyhaveliteraltypeswhenappropriatelycontextualized |
| 3757 | Implement Parenthesisdoesnotblockaliassymbolcreation | spike | frontend/syntax | class: blocked | Implement Parenthesisdoesnotblockaliassymbolcreation |
| 3758 | Implement Parenthesizedasyncarrowfunction | spike | frontend/syntax | class: triage-needed | Implement Parenthesizedasyncarrowfunction |
| 3759 | Implement Parenthesizedjsdoccastatreturnstatement | spike | frontend/syntax | class: blocked | Implement Parenthesizedjsdoccastatreturnstatement |
| 3760 | Implement Parse Parser Syntax | spike | frontend/syntax | class: blocked | Implement Parse Parser Syntax |
| 3761 | Implement Parse Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Parse Unknown Unsupported |
| 3762 | Implement Parsearrowfunctionwithfunctionreturntype | spike | frontend/syntax | class: triage-needed | Implement Parsearrowfunctionwithfunctionreturntype |
| 3763 | Implement Parseassertentrieserror | spike | runtime/builtins | class: blocked | Implement Parseassertentrieserror |
| 3764 | Implement Parsebigint | spike | frontend/syntax | class: triage-needed | Implement Parsebigint |
| 3765 | Implement Parsecommaseparatednewlinenew | spike | frontend/syntax | class: blocked | Implement Parsecommaseparatednewlinenew |
| 3766 | Implement Parsecommaseparatednewlinenumber | spike | frontend/syntax | class: blocked | Implement Parsecommaseparatednewlinenumber |
| 3767 | Implement Parsecommaseparatednewlinestring | spike | frontend/syntax | class: blocked | Implement Parsecommaseparatednewlinestring |
| 3768 | Implement Parseentitynamewithreservedword | spike | frontend/syntax | class: blocked | Implement Parseentitynamewithreservedword |
| 3769 | Implement Parseerrordoublecommaincall | spike | runtime/builtins | class: blocked | Implement Parseerrordoublecommaincall |
| 3770 | Implement Parseerrorinheritageclause | spike | runtime/builtins | class: blocked | Implement Parseerrorinheritageclause |
| 3771 | Implement Parseerrorincorrectreturntoken | spike | frontend/syntax | class: blocked | Implement Parseerrorincorrectreturntoken |
| 3772 | Implement Parsegenericarrowratherthanleftshift | spike | frontend/syntax | class: blocked | Implement Parsegenericarrowratherthanleftshift |
| 3773 | Implement Parseimportattributeserror | spike | runtime/builtins | class: blocked | Implement Parseimportattributeserror |
| 3774 | Implement Parseinvalidnames | spike | frontend/syntax | class: blocked | Implement Parseinvalidnames |
| 3775 | Implement Parseinvalidnullabletypes | spike | frontend/syntax | class: triage-needed | Implement Parseinvalidnullabletypes |
| 3776 | Implement Parsejsxelementinunaryexpressionnocrash Jsx | spike | frontend/syntax | class: blocked | Implement Parsejsxelementinunaryexpressionnocrash Jsx |
| 3777 | Implement Parsejsxelementinunaryexpressionnocrash Regexp Literal | spike | reference/triage | class: blocked | Implement Parsejsxelementinunaryexpressionnocrash Regexp Literal |
| 3778 | Implement Parsejsxextends | spike | reference/triage | class: blocked | Implement Parsejsxextends |
| 3779 | Implement Parseobjectliteralswithouttypes | spike | frontend/syntax | class: blocked | Implement Parseobjectliteralswithouttypes |
| 3780 | Implement Parsetypes | spike | frontend/syntax | class: triage-needed | Implement Parsetypes |
| 3781 | Implement Parseunaryexpressionnotypeassertioninjsx | spike | frontend/syntax | class: blocked | Implement Parseunaryexpressionnotypeassertioninjsx |
| 3782 | Implement Parseunmatchedtypeassertion | spike | frontend/syntax | class: blocked | Implement Parseunmatchedtypeassertion |
| 3783 | Implement Parserconstructordeclaration | spike | reference/triage | class: triage-needed | Implement Parserconstructordeclaration |
| 3784 | Implement Parserisclassmemberstart | spike | frontend/syntax | class: blocked | Implement Parserisclassmemberstart |
| 3785 | Implement Parserprivateidentifierinarrayassignment | spike | frontend/semantics | class: blocked | Implement Parserprivateidentifierinarrayassignment |
| 3786 | Implement Parserunparsedtokencrash Import Export | spike | frontend/syntax | class: blocked | Implement Parserunparsedtokencrash Import Export |
| 3787 | Implement Parserunparsedtokencrash Parser Syntax | spike | frontend/syntax | class: blocked | Implement Parserunparsedtokencrash Parser Syntax |
| 3788 | Implement Parsingclassrecoverswhenhittingunexpectedsemicolon | spike | frontend/syntax | class: blocked | Implement Parsingclassrecoverswhenhittingunexpectedsemicolon |
| 3789 | Implement Parsingdeepparenthensizedexpression | spike | frontend/syntax | class: triage-needed | Implement Parsingdeepparenthensizedexpression |
| 3790 | Implement Partialoflargeapiisabletobeworkedwith | spike | frontend/resolver | class: blocked | Implement Partialoflargeapiisabletobeworkedwith |
| 3791 | Implement Partiallyambientclodule | spike | frontend/syntax | class: blocked | Implement Partiallyambientclodule |
| 3792 | Implement Partiallyambientfundule | spike | frontend/syntax | class: blocked | Implement Partiallyambientfundule |
| 3793 | Implement Partiallydiscriminantedunions | spike | frontend/syntax | class: triage-needed | Implement Partiallydiscriminantedunions |
| 3794 | Implement Pathmappingbasedmoduleresolution Import Export | spike | frontend/syntax | class: blocked | Implement Pathmappingbasedmoduleresolution Import Export |
| 3795 | Implement Pathmappingbasedmoduleresolution Module Resolution | spike | frontend/syntax | class: blocked | Implement Pathmappingbasedmoduleresolution Module Resolution |
| 3796 | Implement Pathmappingbasedmoduleresolution Parser Syntax | spike | frontend/resolver | class: blocked | Implement Pathmappingbasedmoduleresolution Parser Syntax |
| 3797 | Implement Pathmappinginheritedbaseurl | spike | frontend/syntax | class: blocked | Implement Pathmappinginheritedbaseurl |
| 3798 | Implement Pathmappingwithoutbaseurl | spike | frontend/syntax | class: blocked | Implement Pathmappingwithoutbaseurl |
| 3799 | Implement Pathsvalidation | spike | frontend/syntax | class: blocked | Implement Pathsvalidation |
| 3800 | Implement Performancecomparisonofstructurallyidenticalinterfaceswithgenericsignatures | spike | frontend/syntax | class: blocked | Implement Performancecomparisonofstructurallyidenticalinterfaceswithgenericsignatures |
| 3801 | Implement Pickoflargeobjectunionworks | spike | frontend/syntax | class: blocked | Implement Pickoflargeobjectunionworks |
| 3802 | Implement Potentiallyunassignedvariableincatch | spike | frontend/syntax | class: blocked | Implement Potentiallyunassignedvariableincatch |
| 3803 | Implement Potentiallyuncalleddecorators | spike | frontend/syntax | class: blocked | Implement Potentiallyuncalleddecorators |
| 3804 | Implement Predicatesemantics | spike | frontend/syntax | class: triage-needed | Implement Predicatesemantics |
| 3805 | Implement Prefixincrementasoperandofplusexpression | spike | frontend/syntax | class: triage-needed | Implement Prefixincrementasoperandofplusexpression |
| 3806 | Implement Prefixunaryoperatorsonexportedvariables | spike | frontend/syntax | class: blocked | Implement Prefixunaryoperatorsonexportedvariables |
| 3807 | Implement Preserveconstenums | spike | frontend/syntax | class: blocked | Implement Preserveconstenums |
| 3808 | Implement Preserveunusedimports | spike | frontend/syntax | class: blocked | Implement Preserveunusedimports |
| 3809 | Implement Prespecializedgenericmembers | spike | frontend/syntax | class: blocked | Implement Prespecializedgenericmembers |
| 3810 | Implement Prettycontextnotdebugassertion | spike | frontend/syntax | class: blocked | Implement Prettycontextnotdebugassertion |
| 3811 | Implement Prettyfilewitherrorsandtabs | spike | runtime/builtins | class: blocked | Implement Prettyfilewitherrorsandtabs |
| 3812 | Implement Primaryexpressionmods | spike | frontend/syntax | class: blocked | Implement Primaryexpressionmods |
| 3813 | Implement Primitiveconstraints | spike | frontend/semantics | class: blocked | Implement Primitiveconstraints |
| 3814 | Implement Primitivemembers | spike | frontend/syntax | class: blocked | Implement Primitivemembers |
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
| 3862 | Implement Privateaccessinsubclass | spike | frontend/semantics | class: blocked | Implement Privateaccessinsubclass |
| 3863 | Implement Privatefieldassignabilityfromunknown | spike | frontend/syntax | class: blocked | Implement Privatefieldassignabilityfromunknown |
| 3864 | Implement Privatefieldsinclassexpressiondeclaration | spike | frontend/semantics | class: blocked | Implement Privatefieldsinclassexpressiondeclaration |
| 3865 | Implement Privateinstancevisibility | spike | frontend/syntax | class: blocked | Implement Privateinstancevisibility |
| 3866 | Implement Privateinterfaceproperties | spike | frontend/semantics | class: blocked | Implement Privateinterfaceproperties |
| 3867 | Implement Privatepropertyinunion | spike | frontend/semantics | class: blocked | Implement Privatepropertyinunion |
| 3868 | Implement Privatepropertyusingobjecttype | spike | frontend/syntax | class: blocked | Implement Privatepropertyusingobjecttype |
| 3869 | Implement Privatevisibility | spike | frontend/semantics | class: blocked | Implement Privatevisibility |
| 3870 | Implement Privatevisibles | spike | frontend/semantics | class: blocked | Implement Privatevisibles |
| 3871 | Implement Promiseallonany | spike | reference/triage | class: triage-needed | Implement Promiseallonany |
| 3872 | Implement Promisechaining | spike | runtime/builtins | class: blocked | Implement Promisechaining |
| 3873 | Implement Promisedefinitiontest | spike | runtime/builtins | class: blocked | Implement Promisedefinitiontest |
| 3874 | Implement Promiseemptytuplenoexception | spike | frontend/syntax | class: blocked | Implement Promiseemptytuplenoexception |
| 3875 | Implement Promiseidentity | spike | runtime/builtins | class: blocked | Implement Promiseidentity |
| 3876 | Implement Promiseidentitywithany | spike | runtime/builtins | class: blocked | Implement Promiseidentitywithany |
| 3877 | Implement Promiseidentitywithconstraints | spike | frontend/semantics | class: blocked | Implement Promiseidentitywithconstraints |
| 3878 | Implement Promisepermutations | spike | frontend/resolver | class: blocked | Implement Promisepermutations |
| 3879 | Implement Promisetry | spike | runtime/builtins | class: blocked | Implement Promisetry |
| 3880 | Implement Promisetype | spike | reference/triage | class: triage-needed | Implement Promisetype |
| 3881 | Implement Promisetypeinference | spike | frontend/syntax | class: blocked | Implement Promisetypeinference |
| 3882 | Implement Promisetypeinferenceunion | spike | frontend/resolver | class: blocked | Implement Promisetypeinferenceunion |
| 3883 | Implement Promisetypestrictnull | spike | reference/triage | class: triage-needed | Implement Promisetypestrictnull |
| 3884 | Implement Promisevoiderrorcallback | spike | frontend/syntax | class: blocked | Implement Promisevoiderrorcallback |
| 3885 | Implement Promisewithresolvers | spike | frontend/syntax | class: triage-needed | Implement Promisewithresolvers |
| 3886 | Implement Promiseswithconstraints | spike | frontend/syntax | class: triage-needed | Implement Promiseswithconstraints |
| 3887 | Implement Proptypevalidatorinference | spike | frontend/syntax | class: blocked | Implement Proptypevalidatorinference |
| 3888 | Implement Propagatenoninferrabletype | spike | frontend/resolver | class: blocked | Implement Propagatenoninferrabletype |
| 3889 | Implement Propagationofpromiseinitialization | spike | frontend/syntax | class: blocked | Implement Propagationofpromiseinitialization |
| 3890 | Implement Properties | spike | frontend/syntax | class: blocked | Implement Properties |
| 3891 | Implement Propertiesandindexers | spike | frontend/syntax | class: blocked | Implement Propertiesandindexers |
| 3892 | Implement Propertiesandindexersfornumericnames | spike | frontend/syntax | class: blocked | Implement Propertiesandindexersfornumericnames |
| 3893 | Implement Propertyaccess Method Call | spike | frontend/syntax | class: blocked | Implement Propertyaccess Method Call |
| 3894 | Implement Propertyaccess Name Resolution | spike | frontend/resolver | class: blocked | Implement Propertyaccess Name Resolution |
| 3895 | Implement Propertyaccessexpressioninnercomments | spike | frontend/syntax | class: blocked | Implement Propertyaccessexpressioninnercomments |
| 3896 | Implement Propertyaccessofreadonlyindexsignature | spike | frontend/resolver | class: blocked | Implement Propertyaccessofreadonlyindexsignature |
| 3897 | Implement Propertyaccessonobjectliteral | spike | frontend/syntax | class: blocked | Implement Propertyaccessonobjectliteral |
| 3898 | Implement Propertyaccessibility | spike | frontend/syntax | class: blocked | Implement Propertyaccessibility |
| 3899 | Implement Propertyassignment | spike | frontend/syntax | class: blocked | Implement Propertyassignment |
| 3900 | Implement Propertyidentitywithprivacymismatch | spike | frontend/syntax | class: blocked | Implement Propertyidentitywithprivacymismatch |
| 3901 | Implement Propertynameswithstringliteral | spike | frontend/syntax | class: blocked | Implement Propertynameswithstringliteral |
| 3902 | Implement Propertyordering | spike | frontend/syntax | class: blocked | Implement Propertyordering |
| 3903 | Implement Propertyparameterwithquestionmark | spike | frontend/syntax | class: blocked | Implement Propertyparameterwithquestionmark |
| 3904 | Implement Propertysignatures | spike | reference/triage | class: triage-needed | Implement Propertysignatures |
| 3905 | Implement Propertywrappedintry | spike | frontend/syntax | class: blocked | Implement Propertywrappedintry |
| 3906 | Implement Protectedaccessthroughcontextualthis | spike | frontend/semantics | class: blocked | Implement Protectedaccessthroughcontextualthis |
| 3907 | Implement Protectedmembers | spike | frontend/semantics | class: blocked | Implement Protectedmembers |
| 3908 | Implement Protectedmembersthisparameter | spike | frontend/semantics | class: blocked | Implement Protectedmembersthisparameter |
| 3909 | Implement Protoasindexinindexexpression | spike | frontend/syntax | class: blocked | Implement Protoasindexinindexexpression |
| 3910 | Implement Protoassignment | spike | reference/triage | class: triage-needed | Implement Protoassignment |
| 3911 | Implement Prototypeinstantiatedwithbaseconstraint | spike | frontend/semantics | class: blocked | Implement Prototypeinstantiatedwithbaseconstraint |
| 3912 | Implement Prototypeonconstructorfunctions | spike | frontend/syntax | class: blocked | Implement Prototypeonconstructorfunctions |
| 3913 | Implement Prototypes | spike | frontend/syntax | class: blocked | Implement Prototypes |
| 3914 | Implement Publicgetterprotectedsetterfromthisparameter | spike | frontend/semantics | class: blocked | Implement Publicgetterprotectedsetterfromthisparameter |
| 3915 | Implement Publicmemberimplementedasprivateinderivedclass | spike | frontend/semantics | class: blocked | Implement Publicmemberimplementedasprivateinderivedclass |
| 3916 | Implement Pushtypegettypeofalias | spike | frontend/syntax | class: blocked | Implement Pushtypegettypeofalias |
| 3917 | Implement Qualifiedmodulelocals | spike | frontend/syntax | class: blocked | Implement Qualifiedmodulelocals |
| 3918 | Implement Qualifiedname | spike | frontend/syntax | class: blocked | Implement Qualifiedname |
| 3919 | Implement Qualify | spike | frontend/syntax | class: blocked | Implement Qualify |
| 3920 | Implement Quickintersectioncheckcorrectlycacheserrors | spike | frontend/syntax | class: blocked | Implement Quickintersectioncheckcorrectlycacheserrors |
| 3921 | Implement Quickinfotypeatreturnpositionsinaccurate | spike | frontend/syntax | class: blocked | Implement Quickinfotypeatreturnpositionsinaccurate |
| 3922 | Implement Quotedaccessorname | spike | frontend/syntax | class: blocked | Implement Quotedaccessorname |
| 3923 | Implement Quotedfunctionname | spike | frontend/syntax | class: blocked | Implement Quotedfunctionname |
| 3924 | Implement Quotedmodulenamemustbeambient | spike | frontend/syntax | class: blocked | Implement Quotedmodulenamemustbeambient |
| 3925 | Implement Quotedpropertyname | spike | frontend/syntax | class: blocked | Implement Quotedpropertyname |
| 3926 | Implement Ramdatoolsnoinfinite | spike | frontend/syntax | class: blocked | Implement Ramdatoolsnoinfinite |
| 3927 | Implement Reexportglobaldeclaration Import Export | spike | frontend/syntax | class: blocked | Implement Reexportglobaldeclaration Import Export |
| 3928 | Implement Reexportglobaldeclaration Parser Syntax | spike | frontend/resolver | class: blocked | Implement Reexportglobaldeclaration Parser Syntax |
| 3929 | Implement Reexportundefined | spike | frontend/syntax | class: blocked | Implement Reexportundefined |
| 3930 | Implement Reachabilitychecks Arrow Function | spike | frontend/syntax | class: blocked | Implement Reachabilitychecks Arrow Function |
| 3931 | Implement Reachabilitychecks Import Export | spike | frontend/syntax | class: blocked | Implement Reachabilitychecks Import Export |
| 3932 | Implement Reachabilitychecks Name Resolution | spike | frontend/resolver | class: blocked | Implement Reachabilitychecks Name Resolution |
| 3933 | Implement Reachabilitychecks Parser Syntax | spike | frontend/resolver | class: blocked | Implement Reachabilitychecks Parser Syntax |
| 3934 | Implement Reachabilitychecks Runtime Subset | spike | reference/triage | class: triage-needed | Implement Reachabilitychecks Runtime Subset |
| 3935 | Implement Reachabilitychecks Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Reachabilitychecks Unknown Unsupported |
| 3936 | Implement Reachabilitychecksnocrash | spike | frontend/syntax | class: blocked | Implement Reachabilitychecksnocrash |
| 3937 | Implement Reactimportdropped | spike | frontend/syntax | class: blocked | Implement Reactimportdropped |
| 3938 | Implement Reactreduxlikedeferredinferenceallowsassignment | spike | frontend/syntax | class: blocked | Implement Reactreduxlikedeferredinferenceallowsassignment |
| 3939 | Implement Reacttransitiveimporthasvaliddeclaration | spike | frontend/syntax | class: blocked | Implement Reacttransitiveimporthasvaliddeclaration |
| 3940 | Implement Readonlyassignmentinsubclassofclassexpression | spike | frontend/syntax | class: triage-needed | Implement Readonlyassignmentinsubclassofclassexpression |
| 3941 | Implement Readonlyfloat | spike | frontend/resolver | class: blocked | Implement Readonlyfloat |
| 3942 | Implement Readonlyindeclarationfile | spike | frontend/semantics | class: blocked | Implement Readonlyindeclarationfile |
| 3943 | Implement Readonlyinnonpropertyparameters | spike | frontend/semantics | class: blocked | Implement Readonlyinnonpropertyparameters |
| 3944 | Implement Readonlymembers | spike | frontend/semantics | class: blocked | Implement Readonlymembers |
| 3945 | Implement Readonlypropertysubtyperelationdirected | spike | reference/triage | class: triage-needed | Implement Readonlypropertysubtyperelationdirected |
| 3946 | Implement Readonlytupleandarrayelaboration | spike | frontend/semantics | class: blocked | Implement Readonlytupleandarrayelaboration |
| 3947 | Implement Reassignstaticprop | spike | frontend/syntax | class: blocked | Implement Reassignstaticprop |
| 3948 | Implement Reboundbaseclasssymbol | spike | frontend/syntax | class: blocked | Implement Reboundbaseclasssymbol |
| 3949 | Implement Reboundidentifieronimportalias | spike | frontend/syntax | class: blocked | Implement Reboundidentifieronimportalias |
| 3950 | Implement Rectype | spike | frontend/syntax | class: blocked | Implement Rectype |
| 3951 | Implement Recur | spike | frontend/syntax | class: blocked | Implement Recur |
| 3952 | Implement Recursivearraynotcircular | spike | frontend/syntax | class: blocked | Implement Recursivearraynotcircular |
| 3953 | Implement Recursivebasecheck Import Export | spike | frontend/syntax | class: blocked | Implement Recursivebasecheck Import Export |
| 3954 | Implement Recursivebasecheck Parser Syntax | spike | frontend/syntax | class: blocked | Implement Recursivebasecheck Parser Syntax |
| 3955 | Implement Recursivebaseconstructorcreation Name Resolution | spike | frontend/resolver | class: blocked | Implement Recursivebaseconstructorcreation Name Resolution |
| 3956 | Implement Recursivebaseconstructorcreation Parser Syntax | spike | frontend/syntax | class: blocked | Implement Recursivebaseconstructorcreation Parser Syntax |
| 3957 | Implement Recursiveclassbasetype | spike | frontend/syntax | class: triage-needed | Implement Recursiveclassbasetype |
| 3958 | Implement Recursiveclassinstantiationswithdefaultconstructors | spike | frontend/syntax | class: blocked | Implement Recursiveclassinstantiationswithdefaultconstructors |
| 3959 | Implement Recursiveclassreferencetest | spike | frontend/syntax | class: blocked | Implement Recursiveclassreferencetest |
| 3960 | Implement Recursiveclodulereference | spike | frontend/syntax | class: blocked | Implement Recursiveclodulereference |
| 3961 | Implement Recursivecomplicatedclasses | spike | frontend/syntax | class: blocked | Implement Recursivecomplicatedclasses |
| 3962 | Implement Recursiveconditionalcrash | spike | frontend/syntax | class: blocked | Implement Recursiveconditionalcrash |
| 3963 | Implement Recursiveconditionalevaluationnoninfinite | spike | frontend/resolver | class: blocked | Implement Recursiveconditionalevaluationnoninfinite |
| 3964 | Implement Recursiveconditionaltypes | spike | frontend/syntax | class: blocked | Implement Recursiveconditionaltypes |
| 3965 | Implement Recursiveexportassignmentandfindaliasedtype | spike | frontend/syntax | class: blocked | Implement Recursiveexportassignmentandfindaliasedtype |
| 3966 | Implement Recursivefieldsetting | spike | frontend/syntax | class: blocked | Implement Recursivefieldsetting |
| 3967 | Implement Recursivefunctiontypes | spike | frontend/resolver | class: blocked | Implement Recursivefunctiontypes |
| 3968 | Implement Recursivegenericuniontype | spike | frontend/syntax | class: blocked | Implement Recursivegenericuniontype |
| 3969 | Implement Recursiveidenticalassignment | spike | frontend/syntax | class: blocked | Implement Recursiveidenticalassignment |
| 3970 | Implement Recursiveidenticaloverloadresolution | spike | frontend/syntax | class: blocked | Implement Recursiveidenticaloverloadresolution |
| 3971 | Implement Recursiveinference | spike | frontend/syntax | class: blocked | Implement Recursiveinference |
| 3972 | Implement Recursiveinferencebug | spike | frontend/syntax | class: blocked | Implement Recursiveinferencebug |
| 3973 | Implement Recursiveinheritance Name Resolution | spike | frontend/resolver | class: blocked | Implement Recursiveinheritance Name Resolution |
| 3974 | Implement Recursiveinheritance Parser Syntax | spike | frontend/syntax | class: blocked | Implement Recursiveinheritance Parser Syntax |
| 3975 | Implement Recursiveletconst | spike | frontend/syntax | class: triage-needed | Implement Recursiveletconst |
| 3976 | Implement Recursivemods | spike | frontend/syntax | class: blocked | Implement Recursivemods |
| 3977 | Implement Recursivenamedlambdacall | spike | frontend/resolver | class: blocked | Implement Recursivenamedlambdacall |
| 3978 | Implement Recursivereturns | spike | frontend/syntax | class: triage-needed | Implement Recursivereturns |
| 3979 | Implement Recursivereversemappedtype | spike | frontend/syntax | class: blocked | Implement Recursivereversemappedtype |
| 3980 | Implement Recursivespecializationofsignatures | spike | frontend/syntax | class: blocked | Implement Recursivespecializationofsignatures |
| 3981 | Implement Recursivetypealiaswithspreadconditionalreturnnotcircular | spike | frontend/syntax | class: blocked | Implement Recursivetypealiaswithspreadconditionalreturnnotcircular |
| 3982 | Implement Recursivetypecomparison | spike | frontend/syntax | class: blocked | Implement Recursivetypecomparison |
| 3983 | Implement Recursivetypeparameterconstraintreferencelackstypeargs | spike | frontend/syntax | class: blocked | Implement Recursivetypeparameterconstraintreferencelackstypeargs |
| 3984 | Implement Recursivetypeparameterreferenceerror | spike | frontend/semantics | class: blocked | Implement Recursivetypeparameterreferenceerror |
| 3985 | Implement Recursivetyperelations | spike | frontend/semantics | class: blocked | Implement Recursivetyperelations |
| 3986 | Implement Recursivelyspecializedconstructordeclaration | spike | frontend/syntax | class: blocked | Implement Recursivelyspecializedconstructordeclaration |
| 3987 | Implement Redeclarationofvarwithgenerictype | spike | reference/triage | class: triage-needed | Implement Redeclarationofvarwithgenerictype |
| 3988 | Implement Redeclareparameterincatchblock | spike | frontend/syntax | class: blocked | Implement Redeclareparameterincatchblock |
| 3989 | Implement Redefinearray | spike | frontend/resolver | class: blocked | Implement Redefinearray |
| 3990 | Implement Reducibleindexedaccesstypes | spike | frontend/semantics | class: blocked | Implement Reducibleindexedaccesstypes |
| 3991 | Implement Reexportdefaultiscallable | spike | frontend/syntax | class: blocked | Implement Reexportdefaultiscallable |
| 3992 | Implement Reexportmissingdefault | spike | frontend/syntax | class: blocked | Implement Reexportmissingdefault |
| 3993 | Implement Reexportnamealiasedandhoisted | spike | frontend/syntax | class: blocked | Implement Reexportnamealiasedandhoisted |
| 3994 | Implement Reexportwrittencorrectlyindeclaration | spike | frontend/syntax | class: blocked | Implement Reexportwrittencorrectlyindeclaration |
| 3995 | Implement Reexportedmissingalias | spike | frontend/syntax | class: blocked | Implement Reexportedmissingalias |
| 3996 | Implement Compiler | spike | frontend/syntax | class: blocked | Implement Compiler |
| 3997 | Implement Referencesatisfiesexpression | spike | frontend/syntax | class: blocked | Implement Referencesatisfiesexpression |
| 3998 | Implement Referencetypespreferedtopathifpossible | spike | frontend/syntax | class: blocked | Implement Referencetypespreferedtopathifpossible |
| 3999 | Implement Regexpwithopenbracketincharclass | spike | reference/triage | class: blocked | Implement Regexpwithopenbracketincharclass |
| 4000 | Implement Regexpwithslashincharclass | spike | reference/triage | class: blocked | Implement Regexpwithslashincharclass |
| 4001 | Implement Regexmatchall | spike | frontend/syntax | class: blocked | Implement Regexmatchall |
| 4002 | Implement Regexpexecandmatchtypeusages | spike | frontend/syntax | class: blocked | Implement Regexpexecandmatchtypeusages |
| 4003 | Implement Regularexpressioncharacterclassrangeorder | spike | reference/triage | class: blocked | Implement Regularexpressioncharacterclassrangeorder |
| 4004 | Implement Regularexpressionextendedunicodeescapes | spike | reference/triage | class: blocked | Implement Regularexpressionextendedunicodeescapes |
| 4005 | Implement Regularexpressionscanning | spike | reference/triage | class: blocked | Implement Regularexpressionscanning |
| 4006 | Implement Regularexpressionwithnonbmpflags | spike | frontend/syntax | class: blocked | Implement Regularexpressionwithnonbmpflags |
| 4007 | Implement Relatedviadiscriminatedtypenoerror Name Resolution | spike | frontend/resolver | class: blocked | Implement Relatedviadiscriminatedtypenoerror Name Resolution |
| 4008 | Implement Relatedviadiscriminatedtypenoerror Parser Syntax | spike | frontend/semantics | class: blocked | Implement Relatedviadiscriminatedtypenoerror Parser Syntax |
| 4009 | Implement Relativenamesinclassicresolution | spike | frontend/syntax | class: blocked | Implement Relativenamesinclassicresolution |
| 4010 | Implement Renamingdestructuredpropertyinfunctiontype | spike | frontend/syntax | class: blocked | Implement Renamingdestructuredpropertyinfunctiontype |
| 4011 | Implement Reorderproperties | spike | frontend/syntax | class: blocked | Implement Reorderproperties |
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
| 4048 | Implement Requiredinitializedparameter | spike | frontend/syntax | class: blocked | Implement Requiredinitializedparameter |
| 4049 | Implement Requiredmappedtypemodifiertrumpsvariance | spike | frontend/syntax | class: blocked | Implement Requiredmappedtypemodifiertrumpsvariance |
| 4050 | Implement Reservednameoninterfaceimport | spike | frontend/syntax | class: blocked | Implement Reservednameoninterfaceimport |
| 4051 | Implement Reservednameonmoduleimport | spike | frontend/syntax | class: blocked | Implement Reservednameonmoduleimport |
| 4052 | Implement Reservednameonmoduleimportwithinterface | spike | frontend/syntax | class: blocked | Implement Reservednameonmoduleimportwithinterface |
| 4053 | Implement Reservedwords Import Export | spike | frontend/syntax | class: blocked | Implement Reservedwords Import Export |
| 4054 | Implement Reservedwords Parser Syntax | spike | frontend/syntax | class: blocked | Implement Reservedwords Parser Syntax |
| 4055 | Implement Resolutioncandidatefrompackagejsonfield Import Export | spike | frontend/syntax | class: blocked | Implement Resolutioncandidatefrompackagejsonfield Import Export |
| 4056 | Implement Resolutioncandidatefrompackagejsonfield Module Resolution | spike | frontend/syntax | class: blocked | Implement Resolutioncandidatefrompackagejsonfield Module Resolution |
| 4057 | Implement Resolvemodulenamewithsameletdeclarationname | spike | frontend/syntax | class: blocked | Implement Resolvemodulenamewithsameletdeclarationname |
| 4058 | Implement Resolvenamewithnamspace | spike | frontend/syntax | class: blocked | Implement Resolvenamewithnamspace |
| 4059 | Implement Resolvingclassdeclarationwheninbasetyperesolution | spike | frontend/syntax | class: blocked | Implement Resolvingclassdeclarationwheninbasetyperesolution |
| 4060 | Implement Restargassignmentcompat | spike | frontend/syntax | class: blocked | Implement Restargassignmentcompat |
| 4061 | Implement Restargmissingname | spike | frontend/syntax | class: blocked | Implement Restargmissingname |
| 4062 | Implement Restelementassignable | spike | frontend/syntax | class: blocked | Implement Restelementassignable |
| 4063 | Implement Restelementwithnumberpropertyname | spike | reference/triage | class: triage-needed | Implement Restelementwithnumberpropertyname |
| 4064 | Implement Restintersection | spike | reference/triage | class: triage-needed | Implement Restintersection |
| 4065 | Implement Restinvalidargumenttype | spike | frontend/syntax | class: blocked | Implement Restinvalidargumenttype |
| 4066 | Implement Restparammodifier | spike | frontend/semantics | class: blocked | Implement Restparammodifier |
| 4067 | Implement Restparamusingmappedtypeoverunionconstraint | spike | frontend/semantics | class: blocked | Implement Restparamusingmappedtypeoverunionconstraint |
| 4068 | Implement Restparameternotlast | spike | frontend/syntax | class: blocked | Implement Restparameternotlast |
| 4069 | Implement Restparametertypeinstantiation | spike | frontend/syntax | class: triage-needed | Implement Restparametertypeinstantiation |
| 4070 | Implement Restparameterwithbindingpattern | spike | frontend/syntax | class: blocked | Implement Restparameterwithbindingpattern |
| 4071 | Implement Restparamswithnonrestparams | spike | reference/triage | class: triage-needed | Implement Restparamswithnonrestparams |
| 4072 | Implement Resttyperetainsmappyness | spike | frontend/syntax | class: blocked | Implement Resttyperetainsmappyness |
| 4073 | Implement Restunion | spike | reference/triage | class: triage-needed | Implement Restunion |
| 4074 | Implement Returnconditionalexpressionjsdoccast | spike | frontend/syntax | class: blocked | Implement Returnconditionalexpressionjsdoccast |
| 4075 | Implement Returninconstructor | spike | frontend/syntax | class: blocked | Implement Returninconstructor |
| 4076 | Implement Returninfiniteintersection | spike | frontend/syntax | class: triage-needed | Implement Returninfiniteintersection |
| 4077 | Implement Returntypeinferencecontextualparametertypesingenerator | spike | frontend/semantics | class: blocked | Implement Returntypeinferencecontextualparametertypesingenerator |
| 4078 | Implement Returntypeinferencecontextualtypeignoreanyunknown | spike | frontend/resolver | class: blocked | Implement Returntypeinferencecontextualtypeignoreanyunknown |
| 4079 | Implement Returntypeinferencenottoobroad | spike | frontend/syntax | class: blocked | Implement Returntypeinferencenottoobroad |
| 4080 | Implement Returntypeparameter | spike | frontend/resolver | class: blocked | Implement Returntypeparameter |
| 4081 | Implement Returntypeparameterwithmodules | spike | frontend/syntax | class: blocked | Implement Returntypeparameterwithmodules |
| 4082 | Implement Returntypetypearguments | spike | frontend/syntax | class: blocked | Implement Returntypetypearguments |
| 4083 | Implement Reuseinnermodulemember | spike | frontend/syntax | class: blocked | Implement Reuseinnermodulemember |
| 4084 | Implement Reusetypeannotationimporttypeinglobalthistypeargument | spike | frontend/syntax | class: blocked | Implement Reusetypeannotationimporttypeinglobalthistypeargument |
| 4085 | Implement Reverseinferenceincontextualinstantiation | spike | frontend/syntax | class: blocked | Implement Reverseinferenceincontextualinstantiation |
| 4086 | Implement Reversemappedcontravariantinference | spike | frontend/resolver | class: blocked | Implement Reversemappedcontravariantinference |
| 4087 | Implement Reversemappedintersectioninference | spike | frontend/syntax | class: blocked | Implement Reversemappedintersectioninference |
| 4088 | Implement Reversemappedpartiallyinferabletypes | spike | frontend/syntax | class: blocked | Implement Reversemappedpartiallyinferabletypes |
| 4089 | Implement Reversemappedtuplecontext | spike | frontend/resolver | class: blocked | Implement Reversemappedtuplecontext |
| 4090 | Implement Reversemappedtypecontextualtypenotcircular | spike | frontend/resolver | class: blocked | Implement Reversemappedtypecontextualtypenotcircular |
| 4091 | Implement Reversemappedtypecontextualtypesperelementoftupleconstraint | spike | frontend/resolver | class: blocked | Implement Reversemappedtypecontextualtypesperelementoftupleconstraint |
| 4092 | Implement Reversemappedtypedeepdeclarationemit | spike | frontend/syntax | class: blocked | Implement Reversemappedtypedeepdeclarationemit |
| 4093 | Implement Reversemappedtypeinferencesamesource | spike | frontend/syntax | class: blocked | Implement Reversemappedtypeinferencesamesource |
| 4094 | Implement Reversemappedtypeinferencewidening Name Resolution | spike | frontend/resolver | class: blocked | Implement Reversemappedtypeinferencewidening Name Resolution |
| 4095 | Implement Reversemappedtypeinferencewidening Type System | spike | frontend/syntax | class: blocked | Implement Reversemappedtypeinferencewidening Type System |
| 4096 | Implement Reversemappedtypeintersectionconstraint | spike | frontend/syntax | class: triage-needed | Implement Reversemappedtypeintersectionconstraint |
| 4097 | Implement Reversemappedtypelimitedconstraint | spike | frontend/syntax | class: triage-needed | Implement Reversemappedtypelimitedconstraint |
| 4098 | Implement Reversemappedtypeprimitiveconstraintproperty | spike | frontend/resolver | class: blocked | Implement Reversemappedtypeprimitiveconstraintproperty |
| 4099 | Implement Reversemappedunioninference | spike | frontend/syntax | class: blocked | Implement Reversemappedunioninference |
| 4100 | Implement Reversedrecusivetypeinstantiation | spike | frontend/syntax | class: blocked | Implement Reversedrecusivetypeinstantiation |
| 4101 | Implement Satisfiesemit | spike | frontend/syntax | class: blocked | Implement Satisfiesemit |
| 4102 | Implement Scopecheckclassproperty | spike | frontend/syntax | class: blocked | Implement Scopecheckclassproperty |
| 4103 | Implement Scopecheckextendedclassinsidepublicmethod | spike | frontend/syntax | class: blocked | Implement Scopecheckextendedclassinsidepublicmethod |
| 4104 | Implement Scopecheckextendedclassinsidestaticmethod | spike | frontend/syntax | class: blocked | Implement Scopecheckextendedclassinsidestaticmethod |
| 4105 | Implement Scopecheckinsidepublicmethod | spike | frontend/syntax | class: blocked | Implement Scopecheckinsidepublicmethod |
| 4106 | Implement Scopecheckinsidestaticmethod | spike | frontend/syntax | class: blocked | Implement Scopecheckinsidestaticmethod |
| 4107 | Implement Scopecheckstaticinitializer | spike | frontend/syntax | class: blocked | Implement Scopecheckstaticinitializer |
| 4108 | Implement Scopetests | spike | frontend/syntax | class: blocked | Implement Scopetests |
| 4109 | Implement Scopingincatchblocks | spike | frontend/resolver | class: blocked | Implement Scopingincatchblocks |
| 4110 | Implement Selfincallback | spike | frontend/syntax | class: blocked | Implement Selfincallback |
| 4111 | Implement Selfinlambdas | spike | frontend/syntax | class: blocked | Implement Selfinlambdas |
| 4112 | Implement Selfnameandimportsemitinclusion | spike | frontend/syntax | class: blocked | Implement Selfnameandimportsemitinclusion |
| 4113 | Implement Selfref | spike | frontend/syntax | class: blocked | Implement Selfref |
| 4114 | Implement Selfreference | spike | frontend/resolver | class: blocked | Implement Selfreference |
| 4115 | Implement Selfreferencesinfunctionparameters | spike | frontend/syntax | class: blocked | Implement Selfreferencesinfunctionparameters |
| 4116 | Implement Selfreferentialdefaultnostackoverflow | spike | frontend/semantics | class: blocked | Implement Selfreferentialdefaultnostackoverflow |
| 4117 | Implement Semicolonsinmoduledeclarations | spike | frontend/syntax | class: blocked | Implement Semicolonsinmoduledeclarations |
| 4118 | Implement Separate Import Export | spike | frontend/syntax | class: blocked | Implement Separate Import Export |
| 4119 | Implement Separate Name Resolution | spike | frontend/resolver | class: blocked | Implement Separate Name Resolution |
| 4120 | Implement Setmethods | spike | frontend/syntax | class: blocked | Implement Setmethods |
| 4121 | Implement Setterbeforegetter | spike | frontend/semantics | class: blocked | Implement Setterbeforegetter |
| 4122 | Implement Setterwithreturn | spike | frontend/semantics | class: blocked | Implement Setterwithreturn |
| 4123 | Implement Shadowprivatemembers | spike | frontend/semantics | class: blocked | Implement Shadowprivatemembers |
| 4124 | Implement Shadowedfunctionscopedvariablesbyblockscopedones | spike | frontend/syntax | class: blocked | Implement Shadowedfunctionscopedvariablesbyblockscopedones |
| 4125 | Implement Shadowedreservedcompilerdeclarationswithnoemit | spike | frontend/syntax | class: blocked | Implement Shadowedreservedcompilerdeclarationswithnoemit |
| 4126 | Implement Shadowingvialocalvalue | spike | frontend/resolver | class: blocked | Implement Shadowingvialocalvalue |
| 4127 | Implement Shadowingvialocalvalueorbindingelement | spike | reference/triage | class: triage-needed | Implement Shadowingvialocalvalueorbindingelement |
| 4128 | Implement Shebang | spike | frontend/syntax | class: triage-needed | Implement Shebang |
| 4129 | Implement Shebangbeforereferences | spike | frontend/syntax | class: triage-needed | Implement Shebangbeforereferences |
| 4130 | Implement Shebangerror | spike | frontend/syntax | class: triage-needed | Implement Shebangerror |
| 4131 | Implement Shorthand Module System Amd | spike | frontend/syntax | class: blocked | Implement Shorthand Module System Amd |
| 4132 | Implement Shorthand Parser Syntax | spike | frontend/syntax | class: blocked | Implement Shorthand Parser Syntax |
| 4133 | Implement Shorthandofexportedentity | spike | frontend/syntax | class: blocked | Implement Shorthandofexportedentity |
| 4134 | Implement Shorthandpropertyassignmentines | spike | frontend/syntax | class: blocked | Implement Shorthandpropertyassignmentines |
| 4135 | Implement Shorthandpropertyassignmentsindestructuring | spike | frontend/syntax | class: blocked | Implement Shorthandpropertyassignmentsindestructuring |
| 4136 | Implement Shorthandpropertyundefined | spike | frontend/syntax | class: blocked | Implement Shorthandpropertyundefined |
| 4137 | Implement Shouldnotprintnullescapesintooctalliterals | spike | frontend/syntax | class: triage-needed | Implement Shouldnotprintnullescapesintooctalliterals |
| 4138 | Implement Sideeffectimports | spike | frontend/syntax | class: blocked | Implement Sideeffectimports |
| 4139 | Implement Sigantureissubtypeiftheyareidentical | spike | frontend/syntax | class: blocked | Implement Sigantureissubtypeiftheyareidentical |
| 4140 | Implement Signaturecombiningrestparameters Arrow Function | spike | frontend/syntax | class: blocked | Implement Signaturecombiningrestparameters Arrow Function |
| 4141 | Implement Signaturecombiningrestparameters Parser Syntax | spike | frontend/semantics | class: blocked | Implement Signaturecombiningrestparameters Parser Syntax |
| 4142 | Implement Signaturecombiningrestparameters Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Signaturecombiningrestparameters Unknown Unsupported |
| 4143 | Implement Signaturelengthmismatchwithoptionalparameters | spike | frontend/resolver | class: blocked | Implement Signaturelengthmismatchwithoptionalparameters |
| 4144 | Implement Signatureoverloadswithcomments | spike | frontend/syntax | class: triage-needed | Implement Signatureoverloadswithcomments |
| 4145 | Implement Signaturesusejsdocforoptionalparameters | spike | frontend/syntax | class: blocked | Implement Signaturesusejsdocforoptionalparameters |
| 4146 | Implement Silentneverpropagation | spike | frontend/syntax | class: blocked | Implement Silentneverpropagation |
| 4147 | Implement Simplerecursionwithbasecase Name Resolution | spike | frontend/resolver | class: blocked | Implement Simplerecursionwithbasecase Name Resolution |
| 4148 | Implement Simplerecursionwithbasecase Parser Syntax | spike | frontend/syntax | class: blocked | Implement Simplerecursionwithbasecase Parser Syntax |
| 4149 | Implement Simplerecursionwithbasecase Runtime Subset | spike | reference/triage | class: triage-needed | Implement Simplerecursionwithbasecase Runtime Subset |
| 4150 | Implement Simplifyingconditionalwithinteriorconditionalisrelated | spike | frontend/syntax | class: blocked | Implement Simplifyingconditionalwithinteriorconditionalisrelated |
| 4151 | Implement Slashbeforevariabledeclaration | spike | frontend/syntax | class: blocked | Implement Slashbeforevariabledeclaration |
| 4152 | Implement Sliceresultcast | spike | frontend/resolver | class: blocked | Implement Sliceresultcast |
| 4153 | Implement Slightlyindirecteddeepobjectliteralelaborations | spike | frontend/syntax | class: blocked | Implement Slightlyindirecteddeepobjectliteralelaborations |
| 4154 | Implement Sourcemap Import Export | spike | frontend/syntax | class: blocked | Implement Sourcemap Import Export |
| 4155 | Implement Sourcemap Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Sourcemap Unknown Unsupported |
| 4156 | Implement Sourcemapforfunctionininternalmodulewithcommentprecedingstatement | spike | frontend/syntax | class: blocked | Implement Sourcemapforfunctionininternalmodulewithcommentprecedingstatement |
| 4157 | Implement Sourcemapsample | spike | frontend/syntax | class: blocked | Implement Sourcemapsample |
| 4158 | Implement Sourcemapvalidationclass | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationclass |
| 4159 | Implement Sourcemapvalidationclasswithdefaultconstructor | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationclasswithdefaultconstructor |
| 4160 | Implement Sourcemapvalidationclasswithdefaultconstructorandcapturedthisstatement | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationclasswithdefaultconstructorandcapturedthisstatement |
| 4161 | Implement Sourcemapvalidationclasswithdefaultconstructorandextendsclause | spike | frontend/semantics | class: blocked | Implement Sourcemapvalidationclasswithdefaultconstructorandextendsclause |
| 4162 | Implement Sourcemapvalidationclasses | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationclasses |
| 4163 | Implement Sourcemapvalidationdebugger | spike | frontend/resolver | class: blocked | Implement Sourcemapvalidationdebugger |
| 4164 | Implement Sourcemapvalidationdecorators | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdecorators |
| 4165 | Implement Sourcemapvalidationdestructuringforarraybindingpattern | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringforarraybindingpattern |
| 4166 | Implement Sourcemapvalidationdestructuringforarraybindingpatterndefaultvalues | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringforarraybindingpatterndefaultvalues |
| 4167 | Implement Sourcemapvalidationdestructuringforobjectbindingpattern | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringforobjectbindingpattern |
| 4168 | Implement Sourcemapvalidationdestructuringforobjectbindingpatterndefaultvalues | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringforobjectbindingpatterndefaultvalues |
| 4169 | Implement Sourcemapvalidationdestructuringforofarraybindingpattern | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringforofarraybindingpattern |
| 4170 | Implement Sourcemapvalidationdestructuringforofarraybindingpatterndefaultvalues | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringforofarraybindingpatterndefaultvalues |
| 4171 | Implement Sourcemapvalidationdestructuringforofobjectbindingpattern | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringforofobjectbindingpattern |
| 4172 | Implement Sourcemapvalidationdestructuringforofobjectbindingpatterndefaultvalues | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringforofobjectbindingpatterndefaultvalues |
| 4173 | Implement Sourcemapvalidationdestructuringparameternestedobjectbindingpattern | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringparameternestedobjectbindingpattern |
| 4174 | Implement Sourcemapvalidationdestructuringparameternestedobjectbindingpatterndefaultvalues | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringparameternestedobjectbindingpatterndefaultvalues |
| 4175 | Implement Sourcemapvalidationdestructuringparameterobjectbindingpattern | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringparameterobjectbindingpattern |
| 4176 | Implement Sourcemapvalidationdestructuringparameterobjectbindingpatterndefaultvalues | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringparameterobjectbindingpatterndefaultvalues |
| 4177 | Implement Sourcemapvalidationdestructuringparametertarraybindingpattern | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringparametertarraybindingpattern |
| 4178 | Implement Sourcemapvalidationdestructuringparametertarraybindingpatterndefaultvalues | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringparametertarraybindingpatterndefaultvalues |
| 4179 | Implement Sourcemapvalidationdestructuringvariablestatement | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringvariablestatement |
| 4180 | Implement Sourcemapvalidationdestructuringvariablestatementarraybindingpattern | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringvariablestatementarraybindingpattern |
| 4181 | Implement Sourcemapvalidationdestructuringvariablestatementarraybindingpatterndefaultvalues | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringvariablestatementarraybindingpatterndefaultvalues |
| 4182 | Implement Sourcemapvalidationdestructuringvariablestatementdefaultvalues | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringvariablestatementdefaultvalues |
| 4183 | Implement Sourcemapvalidationdestructuringvariablestatementnestedobjectbindingpattern | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringvariablestatementnestedobjectbindingpattern |
| 4184 | Implement Sourcemapvalidationdestructuringvariablestatementnestedobjectbindingpatternwithdefaultvalues | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationdestructuringvariablestatementnestedobjectbindingpatternwithdefaultv... |
| 4185 | Implement Sourcemapvalidationenums | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationenums |
| 4186 | Implement Sourcemapvalidationexportassignment | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationexportassignment |
| 4187 | Implement Sourcemapvalidationexportassignmentcommonjs | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationexportassignmentcommonjs |
| 4188 | Implement Sourcemapvalidationfor | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationfor |
| 4189 | Implement Sourcemapvalidationforin | spike | frontend/resolver | class: blocked | Implement Sourcemapvalidationforin |
| 4190 | Implement Sourcemapvalidationfunctionexpressions | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationfunctionexpressions |
| 4191 | Implement Sourcemapvalidationfunctions | spike | frontend/resolver | class: blocked | Implement Sourcemapvalidationfunctions |
| 4192 | Implement Sourcemapvalidationimport | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationimport |
| 4193 | Implement Sourcemapvalidationmodule | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationmodule |
| 4194 | Implement Sourcemapvalidationstatements | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationstatements |
| 4195 | Implement Sourcemapvalidationswitch | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationswitch |
| 4196 | Implement Sourcemapvalidationtrycatchfinally | spike | reference/triage | class: triage-needed | Implement Sourcemapvalidationtrycatchfinally |
| 4197 | Implement Sourcemapvalidationwithcomments | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationwithcomments |
| 4198 | Implement Sourcemapwithmultiplefileswithfileendingwithinterface | spike | frontend/syntax | class: blocked | Implement Sourcemapwithmultiplefileswithfileendingwithinterface |
| 4199 | Implement Sourcemapvalidationduplicatenames | spike | frontend/syntax | class: blocked | Implement Sourcemapvalidationduplicatenames |
| 4200 | Implement Spacebeforequestionmarkinpropertyassignment | spike | frontend/syntax | class: blocked | Implement Spacebeforequestionmarkinpropertyassignment |
| 4201 | Implement Specednostackblown | spike | frontend/syntax | class: blocked | Implement Specednostackblown |
| 4202 | Implement Specializationofexportedclass | spike | frontend/syntax | class: blocked | Implement Specializationofexportedclass |
| 4203 | Implement Specializationsshouldnotaffecteachother | spike | frontend/syntax | class: blocked | Implement Specializationsshouldnotaffecteachother |
| 4204 | Implement Specializedinheritedconstructors | spike | frontend/syntax | class: blocked | Implement Specializedinheritedconstructors |
| 4205 | Implement Specializedlambdatypearguments | spike | frontend/syntax | class: blocked | Implement Specializedlambdatypearguments |
| 4206 | Implement Specializedsignatureascallbackparameter | spike | frontend/syntax | class: blocked | Implement Specializedsignatureascallbackparameter |
| 4207 | Implement Spellingsuggestionglobal | spike | frontend/syntax | class: blocked | Implement Spellingsuggestionglobal |
| 4208 | Implement Spellingsuggestionleadingunderscores | spike | frontend/syntax | class: blocked | Implement Spellingsuggestionleadingunderscores |
| 4209 | Implement Spellingsuggestionmodule | spike | frontend/syntax | class: blocked | Implement Spellingsuggestionmodule |
| 4210 | Implement Splicetuples | spike | frontend/resolver | class: blocked | Implement Splicetuples |
| 4211 | Implement Spreadbooleanrespectsfreshness | spike | frontend/resolver | class: blocked | Implement Spreadbooleanrespectsfreshness |
| 4212 | Implement Spreadexpressioncontainingobjectexpressioncontextualtype | spike | frontend/syntax | class: blocked | Implement Spreadexpressioncontainingobjectexpressioncontextualtype |
| 4213 | Implement Spreadexpressioncontextualtypewithnamespace | spike | frontend/syntax | class: blocked | Implement Spreadexpressioncontextualtypewithnamespace |
| 4214 | Implement Spreadintersection | spike | reference/triage | class: triage-needed | Implement Spreadintersection |
| 4215 | Implement Spreadinvalidargumenttype | spike | frontend/semantics | class: blocked | Implement Spreadinvalidargumenttype |
| 4216 | Implement Spreadobjectnocircular | spike | frontend/syntax | class: blocked | Implement Spreadobjectnocircular |
| 4217 | Implement Spreadobjectpermutations | spike | frontend/resolver | class: blocked | Implement Spreadobjectpermutations |
| 4218 | Implement Spreadobjectwithindexdoesnotaddundefinedtolocalindex | spike | frontend/syntax | class: blocked | Implement Spreadobjectwithindexdoesnotaddundefinedtolocalindex |
| 4219 | Implement Spreadofobjectliteralassignabletoindexsignature | spike | frontend/syntax | class: blocked | Implement Spreadofobjectliteralassignabletoindexsignature |
| 4220 | Implement Spreadofparamsfromgeneratormakesrequiredparams | spike | frontend/semantics | class: blocked | Implement Spreadofparamsfromgeneratormakesrequiredparams |
| 4221 | Implement Spreadparametertupletype | spike | frontend/semantics | class: blocked | Implement Spreadparametertupletype |
| 4222 | Implement Spreadtupleaccessedbytypeparameter | spike | frontend/syntax | class: blocked | Implement Spreadtupleaccessedbytypeparameter |
| 4223 | Implement Spreadunionpropoverride | spike | frontend/semantics | class: blocked | Implement Spreadunionpropoverride |
| 4224 | Implement Spreadsandcontextualtupletypes | spike | frontend/semantics | class: blocked | Implement Spreadsandcontextualtupletypes |
| 4225 | Implement Spycomparisonchecking | spike | frontend/syntax | class: blocked | Implement Spycomparisonchecking |
| 4226 | Implement Stabletypeordering | spike | frontend/syntax | class: blocked | Implement Stabletypeordering |
| 4227 | Implement Stackdepthlimitcastingtype | spike | frontend/syntax | class: blocked | Implement Stackdepthlimitcastingtype |
| 4228 | Implement Standalonebreak | spike | frontend/syntax | class: blocked | Implement Standalonebreak |
| 4229 | Implement Staticandmemberfunctions | spike | frontend/syntax | class: blocked | Implement Staticandmemberfunctions |
| 4230 | Implement Staticanonymoustypenotreferencingtypeparameter | spike | frontend/syntax | class: blocked | Implement Staticanonymoustypenotreferencingtypeparameter |
| 4231 | Implement Staticasidentifier | spike | frontend/syntax | class: blocked | Implement Staticasidentifier |
| 4232 | Implement Staticclassmembererror | spike | runtime/builtins | class: blocked | Implement Staticclassmembererror |
| 4233 | Implement Staticclassprops | spike | frontend/syntax | class: blocked | Implement Staticclassprops |
| 4234 | Implement Staticfieldwithinterfacecontext | spike | frontend/syntax | class: blocked | Implement Staticfieldwithinterfacecontext |
| 4235 | Implement Staticgetter | spike | frontend/syntax | class: blocked | Implement Staticgetter |
| 4236 | Implement Staticgetterandsetter | spike | reference/triage | class: triage-needed | Implement Staticgetterandsetter |
| 4237 | Implement Staticindexsignatureandnormalindexsignature | spike | frontend/syntax | class: blocked | Implement Staticindexsignatureandnormalindexsignature |
| 4238 | Implement Staticinheritance | spike | frontend/syntax | class: blocked | Implement Staticinheritance |
| 4239 | Implement Staticinitializersandlegacyclassdecorators | spike | frontend/syntax | class: blocked | Implement Staticinitializersandlegacyclassdecorators |
| 4240 | Implement Staticinstanceresolution Import Export | spike | frontend/syntax | class: blocked | Implement Staticinstanceresolution Import Export |
| 4241 | Implement Staticinstanceresolution Module Resolution | spike | frontend/syntax | class: blocked | Implement Staticinstanceresolution Module Resolution |
| 4242 | Implement Staticinstanceresolution Name Resolution | spike | frontend/resolver | class: blocked | Implement Staticinstanceresolution Name Resolution |
| 4243 | Implement Staticinterfaceassignmentcompat | spike | frontend/resolver | class: blocked | Implement Staticinterfaceassignmentcompat |
| 4244 | Implement Staticmemberaccessoffderivedtype | spike | frontend/syntax | class: blocked | Implement Staticmemberaccessoffderivedtype |
| 4245 | Implement Staticmemberexportaccess | spike | frontend/syntax | class: blocked | Implement Staticmemberexportaccess |
| 4246 | Implement Staticmemberofclassandpublicmemberofanotherclassassignment | spike | frontend/semantics | class: blocked | Implement Staticmemberofclassandpublicmemberofanotherclassassignment |
| 4247 | Implement Staticmemberwithstringandnumbernames | spike | frontend/syntax | class: blocked | Implement Staticmemberwithstringandnumbernames |
| 4248 | Implement Staticmethodreferencingtypeargument | spike | frontend/syntax | class: blocked | Implement Staticmethodreferencingtypeargument |
| 4249 | Implement Staticmethodwithtypeparameterextendsclausedeclfile | spike | frontend/syntax | class: blocked | Implement Staticmethodwithtypeparameterextendsclausedeclfile |
| 4250 | Implement Staticmethodsreferencingclasstypeparameters | spike | frontend/semantics | class: blocked | Implement Staticmethodsreferencingclasstypeparameters |
| 4251 | Implement Staticmismatchbecauseofprototype | spike | frontend/syntax | class: blocked | Implement Staticmismatchbecauseofprototype |
| 4252 | Implement Staticmodifieralreadyseen | spike | frontend/syntax | class: blocked | Implement Staticmodifieralreadyseen |
| 4253 | Implement Staticmustprecedepublic | spike | frontend/semantics | class: blocked | Implement Staticmustprecedepublic |
| 4254 | Implement Staticoffofinstance | spike | frontend/syntax | class: blocked | Implement Staticoffofinstance |
| 4255 | Implement Staticpropsuper | spike | frontend/syntax | class: blocked | Implement Staticpropsuper |
| 4256 | Implement Staticprototypeproperty | spike | frontend/syntax | class: blocked | Implement Staticprototypeproperty |
| 4257 | Implement Staticprototypepropertyonclass | spike | frontend/syntax | class: blocked | Implement Staticprototypepropertyonclass |
| 4258 | Implement Staticvisibility Duplicate Function | spike | reference/triage | class: triage-needed | Implement Staticvisibility Duplicate Function |
| 4259 | Implement Staticvisibility Parser Syntax | spike | frontend/syntax | class: blocked | Implement Staticvisibility Parser Syntax |
| 4260 | Implement Statics | spike | frontend/syntax | class: blocked | Implement Statics |
| 4261 | Implement Staticsinafunction | spike | frontend/syntax | class: blocked | Implement Staticsinafunction |
| 4262 | Implement Staticsinconstructorbodies | spike | frontend/syntax | class: triage-needed | Implement Staticsinconstructorbodies |
| 4263 | Implement Staticsnotinscopeinclodule | spike | frontend/syntax | class: blocked | Implement Staticsnotinscopeinclodule |
| 4264 | Implement Stradac | spike | frontend/resolver | class: blocked | Implement Stradac |
| 4265 | Implement Strictfunctiontypes | spike | frontend/resolver | class: blocked | Implement Strictfunctiontypes |
| 4266 | Implement Strictfunctiontypeserrors | spike | runtime/builtins | class: blocked | Implement Strictfunctiontypeserrors |
| 4267 | Implement Strictmodeenummembernamereserved | spike | frontend/syntax | class: blocked | Implement Strictmodeenummembernamereserved |
| 4268 | Implement Strictmodeinconstructor | spike | frontend/syntax | class: blocked | Implement Strictmodeinconstructor |
| 4269 | Implement Strictmodereservedword | spike | frontend/syntax | class: blocked | Implement Strictmodereservedword |
| 4270 | Implement Strictmodereservedwordinclassdeclaration | spike | frontend/syntax | class: blocked | Implement Strictmodereservedwordinclassdeclaration |
| 4271 | Implement Strictmodereservedwordindestructuring | spike | frontend/syntax | class: blocked | Implement Strictmodereservedwordindestructuring |
| 4272 | Implement Strictmodereservedwordinimportequaldeclaration | spike | frontend/syntax | class: blocked | Implement Strictmodereservedwordinimportequaldeclaration |
| 4273 | Implement Strictmodereservedwordinmoduledeclaration | spike | frontend/syntax | class: blocked | Implement Strictmodereservedwordinmoduledeclaration |
| 4274 | Implement Strictmodeusecontextualkeyword | spike | frontend/syntax | class: blocked | Implement Strictmodeusecontextualkeyword |
| 4275 | Implement Strictmodewordinexportdeclaration | spike | frontend/syntax | class: blocked | Implement Strictmodewordinexportdeclaration |
| 4276 | Implement Strictmodewordinimportdeclaration | spike | frontend/syntax | class: blocked | Implement Strictmodewordinimportdeclaration |
| 4277 | Implement Strictnullemptydestructuring | spike | reference/triage | class: triage-needed | Implement Strictnullemptydestructuring |
| 4278 | Implement Strictnulllogicalandor | spike | frontend/syntax | class: triage-needed | Implement Strictnulllogicalandor |
| 4279 | Implement Strictnullnotnullindextypenolib | spike | frontend/syntax | class: blocked | Implement Strictnullnotnullindextypenolib |
| 4280 | Implement Strictnullnotnullindextypeshouldwork | spike | frontend/syntax | class: blocked | Implement Strictnullnotnullindextypeshouldwork |
| 4281 | Implement Strictoptionalproperties | spike | frontend/syntax | class: blocked | Implement Strictoptionalproperties |
| 4282 | Implement Strictsubtypeandnarrowing | spike | frontend/semantics | class: blocked | Implement Strictsubtypeandnarrowing |
| 4283 | Implement Stricttypeofunionnarrowing | spike | frontend/syntax | class: triage-needed | Implement Stricttypeofunionnarrowing |
| 4284 | Implement Stringincludes | spike | runtime/builtins | class: blocked | Implement Stringincludes |
| 4285 | Implement Stringindexerandconstructor | spike | frontend/syntax | class: blocked | Implement Stringindexerandconstructor |
| 4286 | Implement Stringindexerassignments Name Resolution | spike | frontend/resolver | class: blocked | Implement Stringindexerassignments Name Resolution |
| 4287 | Implement Stringindexerassignments Parser Syntax | spike | frontend/syntax | class: blocked | Implement Stringindexerassignments Parser Syntax |
| 4288 | Implement Stringliteralobjectliteraldeclaration | spike | frontend/syntax | class: blocked | Implement Stringliteralobjectliteraldeclaration |
| 4289 | Implement Stringliteralpropertynamewithlinecontinuation | spike | frontend/syntax | class: blocked | Implement Stringliteralpropertynamewithlinecontinuation |
| 4290 | Implement Stringliteralserrors | spike | frontend/syntax | class: triage-needed | Implement Stringliteralserrors |
| 4291 | Implement Stringmatchall | spike | reference/triage | class: blocked | Implement Stringmatchall |
| 4292 | Implement Stringpropcodegen | spike | frontend/syntax | class: blocked | Implement Stringpropcodegen |
| 4293 | Implement Stringrawtype | spike | frontend/resolver | class: blocked | Implement Stringrawtype |
| 4294 | Implement Stringtrim | spike | runtime/builtins | class: blocked | Implement Stringtrim |
| 4295 | Implement Stripmembersoptionality | spike | frontend/resolver | class: blocked | Implement Stripmembersoptionality |
| 4296 | Implement Structural | spike | frontend/syntax | class: blocked | Implement Structural |
| 4297 | Implement Structuraltypeindeclarefileformodule | spike | frontend/syntax | class: blocked | Implement Structuraltypeindeclarefileformodule |
| 4298 | Implement Styledcomponentsinstantiaionlimitnotreached | spike | frontend/syntax | class: triage-needed | Implement Styledcomponentsinstantiaionlimitnotreached |
| 4299 | Implement Subsubclasscanaccessprotectedconstructor | spike | frontend/syntax | class: blocked | Implement Subsubclasscanaccessprotectedconstructor |
| 4300 | Implement Subclassthistypeassignable | spike | frontend/semantics | class: blocked | Implement Subclassthistypeassignable |
| 4301 | Implement Subclasswithpolymorphicthisisassignable | spike | frontend/syntax | class: blocked | Implement Subclasswithpolymorphicthisisassignable |
| 4302 | Implement Substitutiontypenomergeofassignabletype | spike | frontend/syntax | class: triage-needed | Implement Substitutiontypenomergeofassignabletype |
| 4303 | Implement Substitutiontypesinindexedaccesstypes | spike | frontend/resolver | class: blocked | Implement Substitutiontypesinindexedaccesstypes |
| 4304 | Implement Subtypereductionunionconstraints | spike | frontend/syntax | class: blocked | Implement Subtypereductionunionconstraints |
| 4305 | Implement Subtypereductionwithanyfunctiontype | spike | frontend/syntax | class: blocked | Implement Subtypereductionwithanyfunctiontype |
| 4306 | Implement Subtyperelationfornever | spike | frontend/syntax | class: triage-needed | Implement Subtyperelationfornever |
| 4307 | Implement Subtypingtransitivity | spike | frontend/syntax | class: blocked | Implement Subtypingtransitivity |
| 4308 | Implement Super Import Export | spike | frontend/syntax | class: blocked | Implement Super Import Export |
| 4309 | Implement Super Parser Syntax | spike | frontend/syntax | class: blocked | Implement Super Parser Syntax |
| 4310 | Implement Superaccess | spike | frontend/syntax | class: blocked | Implement Superaccess |
| 4311 | Implement Superaccessinfatarrow | spike | frontend/syntax | class: blocked | Implement Superaccessinfatarrow |
| 4312 | Implement Supercallargsmustmatch | spike | frontend/syntax | class: blocked | Implement Supercallargsmustmatch |
| 4313 | Implement Supercallfromclassthatderivesfromgenerictype | spike | frontend/syntax | class: blocked | Implement Supercallfromclassthatderivesfromgenerictype |
| 4314 | Implement Supercallfromclassthatderivesfromgenerictypebutwithincorrectnumberoftypearguments | spike | frontend/syntax | class: blocked | Implement Supercallfromclassthatderivesfromgenerictypebutwithincorrectnumberoftypearguments |
| 4315 | Implement Supercallfromclassthatderivesfromgenerictypebutwithnotypearguments | spike | frontend/syntax | class: blocked | Implement Supercallfromclassthatderivesfromgenerictypebutwithnotypearguments |
| 4316 | Implement Supercallfromclassthatderivesnongenerictypebutwithtypearguments | spike | frontend/syntax | class: blocked | Implement Supercallfromclassthatderivesnongenerictypebutwithtypearguments |
| 4317 | Implement Supercallfromclassthathasnobasetype | spike | frontend/syntax | class: blocked | Implement Supercallfromclassthathasnobasetype |
| 4318 | Implement Supercallfromfunction | spike | frontend/resolver | class: blocked | Implement Supercallfromfunction |
| 4319 | Implement Supercallinnonstaticmethod | spike | frontend/syntax | class: blocked | Implement Supercallinnonstaticmethod |
| 4320 | Implement Supercallinstaticmethod | spike | frontend/syntax | class: blocked | Implement Supercallinstaticmethod |
| 4321 | Implement Supercallinsideclassexpression | spike | frontend/syntax | class: blocked | Implement Supercallinsideclassexpression |
| 4322 | Implement Supercallinsideobjectliteralexpression | spike | frontend/syntax | class: blocked | Implement Supercallinsideobjectliteralexpression |
| 4323 | Implement Supercalloutsideconstructor | spike | frontend/syntax | class: blocked | Implement Supercalloutsideconstructor |
| 4324 | Implement Supercallwithmissingbaseclass | spike | frontend/syntax | class: blocked | Implement Supercallwithmissingbaseclass |
| 4325 | Implement Supercallsinconstructor | spike | frontend/syntax | class: blocked | Implement Supercallsinconstructor |
| 4326 | Implement Superelementaccess | spike | frontend/syntax | class: blocked | Implement Superelementaccess |
| 4327 | Implement Supererrors | spike | frontend/syntax | class: triage-needed | Implement Supererrors |
| 4328 | Implement Superhasmethodsfrommergedinterface | spike | frontend/syntax | class: blocked | Implement Superhasmethodsfrommergedinterface |
| 4329 | Implement Superinconstructorparam | spike | frontend/semantics | class: blocked | Implement Superinconstructorparam |
| 4330 | Implement Superinlambdas | spike | frontend/syntax | class: triage-needed | Implement Superinlambdas |
| 4331 | Implement Superinobjectliterals | spike | frontend/syntax | class: blocked | Implement Superinobjectliterals |
| 4332 | Implement Supernewcall | spike | frontend/syntax | class: blocked | Implement Supernewcall |
| 4333 | Implement Supernomodifierscrash | spike | frontend/syntax | class: blocked | Implement Supernomodifierscrash |
| 4334 | Implement Superpropertyaccess | spike | frontend/syntax | class: blocked | Implement Superpropertyaccess |
| 4335 | Implement Superpropertyaccessincomputedpropertiesofnestedtype | spike | frontend/syntax | class: triage-needed | Implement Superpropertyaccessincomputedpropertiesofnestedtype |
| 4336 | Implement Superpropertyaccessinsupercall | spike | frontend/syntax | class: blocked | Implement Superpropertyaccessinsupercall |
| 4337 | Implement Superpropertyelementnounusedlexicalthiscapture | spike | frontend/syntax | class: blocked | Implement Superpropertyelementnounusedlexicalthiscapture |
| 4338 | Implement Superwithgenericspecialization | spike | frontend/syntax | class: blocked | Implement Superwithgenericspecialization |
| 4339 | Implement Superwithgenerics | spike | frontend/syntax | class: blocked | Implement Superwithgenerics |
| 4340 | Implement Superwithtypeargument | spike | frontend/semantics | class: blocked | Implement Superwithtypeargument |
| 4341 | Implement Switchassignmentcompat | spike | frontend/resolver | class: blocked | Implement Switchassignmentcompat |
| 4342 | Implement Switchcasenarrowsmatchingclausesevenwhennonmatchingclausesexist | spike | frontend/semantics | class: blocked | Implement Switchcasenarrowsmatchingclausesevenwhennonmatchingclausesexist |
| 4343 | Implement Switchcasesexpressiontypemismatch | spike | frontend/syntax | class: blocked | Implement Switchcasesexpressiontypemismatch |
| 4344 | Implement Switchcomparablecompatforbrands | spike | frontend/syntax | class: blocked | Implement Switchcomparablecompatforbrands |
| 4345 | Implement Switchfallthroughs | spike | frontend/syntax | class: blocked | Implement Switchfallthroughs |
| 4346 | Implement Switchstatementswithmultipledefaults | spike | frontend/syntax | class: blocked | Implement Switchstatementswithmultipledefaults |
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
| 4360 | Implement Systemjsforinnoexception | spike | frontend/syntax | class: blocked | Implement Systemjsforinnoexception |
| 4361 | Implement Systemmodule Import Export | spike | frontend/syntax | class: blocked | Implement Systemmodule Import Export |
| 4362 | Implement Systemmodule Module System Amd | spike | frontend/syntax | class: blocked | Implement Systemmodule Module System Amd |
| 4363 | Implement Systemmodule Parser Syntax | spike | frontend/syntax | class: blocked | Implement Systemmodule Parser Syntax |
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
| 4375 | Implement Taggedtemplatestringwithsymbolexpression | spike | frontend/syntax | class: blocked | Implement Taggedtemplatestringwithsymbolexpression |
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
| 4392 | Implement Targettypecalls | spike | frontend/syntax | class: blocked | Implement Targettypecalls |
| 4393 | Implement Targettypecasttest | spike | frontend/syntax | class: blocked | Implement Targettypecasttest |
| 4394 | Implement Targettypeobjectliteral | spike | frontend/syntax | class: blocked | Implement Targettypeobjectliteral |
| 4395 | Implement Targettypeobjectliteraltoany | spike | frontend/syntax | class: blocked | Implement Targettypeobjectliteraltoany |
| 4396 | Implement Targettypetest | spike | frontend/syntax | class: blocked | Implement Targettypetest |
| 4397 | Implement Targettypevoidfunc | spike | frontend/syntax | class: triage-needed | Implement Targettypevoidfunc |
| 4398 | Implement Templateexpressionaspossiblydiscriminantvalue | spike | frontend/syntax | class: blocked | Implement Templateexpressionaspossiblydiscriminantvalue |
| 4399 | Implement Templateexpressionnoinlininingofconstantbindingwithinitializer | spike | frontend/syntax | class: blocked | Implement Templateexpressionnoinlininingofconstantbindingwithinitializer |
| 4400 | Implement Templateliteralconstantevaluation | spike | frontend/semantics | class: blocked | Implement Templateliteralconstantevaluation |
| 4401 | Implement Templateliteralescapesequence | spike | frontend/semantics | class: blocked | Implement Templateliteralescapesequence |
| 4402 | Implement Templateliteralintersection Name Resolution | spike | frontend/resolver | class: blocked | Implement Templateliteralintersection Name Resolution |
| 4403 | Implement Templateliteralintersection Parser Syntax | spike | frontend/semantics | class: blocked | Implement Templateliteralintersection Parser Syntax |
| 4404 | Implement Templateliteralsanddecoratormetadata | spike | frontend/syntax | class: blocked | Implement Templateliteralsanddecoratormetadata |
| 4405 | Implement Templateliteralsintypes | spike | reference/triage | class: triage-needed | Implement Templateliteralsintypes |
| 4406 | Implement Templatestringsarraytypedefinedines | spike | frontend/syntax | class: blocked | Implement Templatestringsarraytypedefinedines |
| 4407 | Implement Templatestringsarraytypenotdefinedes | spike | frontend/syntax | class: blocked | Implement Templatestringsarraytypenotdefinedes |
| 4408 | Implement Templatestringsarraytyperedefinedines | spike | frontend/syntax | class: blocked | Implement Templatestringsarraytyperedefinedines |
| 4409 | Implement Temporal | spike | frontend/syntax | class: blocked | Implement Temporal |
| 4410 | Implement Ternaryexpressionsourcemap | spike | frontend/syntax | class: triage-needed | Implement Ternaryexpressionsourcemap |
| 4411 | Implement Testcontainerlist | spike | frontend/syntax | class: blocked | Implement Testcontainerlist |
| 4412 | Implement This Import Export | spike | frontend/syntax | class: blocked | Implement This Import Export |
| 4413 | Implement This Parser Syntax | spike | frontend/syntax | class: blocked | Implement This Parser Syntax |
| 4414 | Implement Thisassignmentinnamespacedeclaration | spike | frontend/syntax | class: blocked | Implement Thisassignmentinnamespacedeclaration |
| 4415 | Implement Thisbinding Import Export | spike | frontend/syntax | class: blocked | Implement Thisbinding Import Export |
| 4416 | Implement Thisbinding Parser Syntax | spike | frontend/syntax | class: blocked | Implement Thisbinding Parser Syntax |
| 4417 | Implement Thiscapture | spike | frontend/syntax | class: blocked | Implement Thiscapture |
| 4418 | Implement Thisconditionalonmethodreturnofgenericinstance | spike | frontend/syntax | class: blocked | Implement Thisconditionalonmethodreturnofgenericinstance |
| 4419 | Implement Thisexpressionincallexpressionwithtypearguments | spike | frontend/syntax | class: blocked | Implement Thisexpressionincallexpressionwithtypearguments |
| 4420 | Implement Thisexpressionofgenericobject | spike | frontend/syntax | class: blocked | Implement Thisexpressionofgenericobject |
| 4421 | Implement Thisinaccessors | spike | reference/triage | class: triage-needed | Implement Thisinaccessors |
| 4422 | Implement Thisinarrowfunctioninstaticinitializer | spike | frontend/semantics | class: blocked | Implement Thisinarrowfunctioninstaticinitializer |
| 4423 | Implement Thisinclassbodystaticesnext | spike | frontend/syntax | class: blocked | Implement Thisinclassbodystaticesnext |
| 4424 | Implement Thisinconstructorparameter | spike | frontend/semantics | class: blocked | Implement Thisinconstructorparameter |
| 4425 | Implement Thisinfunctioncalljs | spike | frontend/syntax | class: blocked | Implement Thisinfunctioncalljs |
| 4426 | Implement Thisingenericstaticmembers | spike | frontend/syntax | class: blocked | Implement Thisingenericstaticmembers |
| 4427 | Implement Thisininnerfunctions | spike | frontend/syntax | class: blocked | Implement Thisininnerfunctions |
| 4428 | Implement Thisinlambda | spike | frontend/syntax | class: blocked | Implement Thisinlambda |
| 4429 | Implement Thisinmodule | spike | frontend/syntax | class: blocked | Implement Thisinmodule |
| 4430 | Implement Thisinmodulefunction | spike | frontend/syntax | class: blocked | Implement Thisinmodulefunction |
| 4431 | Implement Thisinobjectjs | spike | frontend/syntax | class: blocked | Implement Thisinobjectjs |
| 4432 | Implement Thisinouterclassbody | spike | frontend/syntax | class: blocked | Implement Thisinouterclassbody |
| 4433 | Implement Thisinpropertybounddeclarations | spike | frontend/syntax | class: blocked | Implement Thisinpropertybounddeclarations |
| 4434 | Implement Thisinstaticmethod | spike | frontend/syntax | class: blocked | Implement Thisinstaticmethod |
| 4435 | Implement Thisinstatics | spike | frontend/syntax | class: blocked | Implement Thisinstatics |
| 4436 | Implement Thisinsupercall | spike | frontend/syntax | class: blocked | Implement Thisinsupercall |
| 4437 | Implement Thisintupletypeparameterconstraints | spike | frontend/resolver | class: blocked | Implement Thisintupletypeparameterconstraints |
| 4438 | Implement Thisintypequery | spike | frontend/semantics | class: blocked | Implement Thisintypequery |
| 4439 | Implement Thisindexonexistingreadonlyfieldisnotnever | spike | frontend/semantics | class: blocked | Implement Thisindexonexistingreadonlyfieldisnotnever |
| 4440 | Implement Thiskeyword | spike | frontend/syntax | class: blocked | Implement Thiskeyword |
| 4441 | Implement Thispredicateinobjectliteral | spike | frontend/syntax | class: blocked | Implement Thispredicateinobjectliteral |
| 4442 | Implement Thisreferencedinfunctioninsidearrowfunction | spike | frontend/resolver | class: blocked | Implement Thisreferencedinfunctioninsidearrowfunction |
| 4443 | Implement Thisshadowingerrorspans | spike | reference/triage | class: triage-needed | Implement Thisshadowingerrorspans |
| 4444 | Implement Thistypeasconstraint | spike | frontend/syntax | class: blocked | Implement Thistypeasconstraint |
| 4445 | Implement Thiswhentypecheckfails | spike | frontend/semantics | class: blocked | Implement Thiswhentypecheckfails |
| 4446 | Implement Thislessfunctionsnotcontextsensitive Parser Syntax | spike | frontend/syntax | class: blocked | Implement Thislessfunctionsnotcontextsensitive Parser Syntax |
| 4447 | Implement Thislessfunctionsnotcontextsensitive Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Thislessfunctionsnotcontextsensitive Unknown Unsupported |
| 4448 | Implement Throwwithoutnewline Name Resolution | spike | frontend/resolver | class: blocked | Implement Throwwithoutnewline Name Resolution |
| 4449 | Implement Throwwithoutnewline Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Throwwithoutnewline Unknown Unsupported |
| 4450 | Implement Tostringonprimitives | spike | frontend/syntax | class: blocked | Implement Tostringonprimitives |
| 4451 | Implement Toofewargumentsingenericfunctiontypedargument | spike | frontend/syntax | class: blocked | Implement Toofewargumentsingenericfunctiontypedargument |
| 4452 | Implement Toomanytypeparameters | spike | frontend/syntax | class: triage-needed | Implement Toomanytypeparameters |
| 4453 | Implement Topfunctiontypenotcallable | spike | frontend/resolver | class: blocked | Implement Topfunctiontypenotcallable |
| 4454 | Implement Toplevel | spike | frontend/syntax | class: blocked | Implement Toplevel |
| 4455 | Implement Toplevelblockexpando | spike | frontend/syntax | class: blocked | Implement Toplevelblockexpando |
| 4456 | Implement Toplevelexports | spike | frontend/syntax | class: blocked | Implement Toplevelexports |
| 4457 | Implement Toplevellambda Arrow Function | spike | frontend/syntax | class: blocked | Implement Toplevellambda Arrow Function |
| 4458 | Implement Toplevellambda Class | spike | frontend/syntax | class: blocked | Implement Toplevellambda Class |
| 4459 | Implement Toplevellambda Import Export | spike | frontend/syntax | class: blocked | Implement Toplevellambda Import Export |
| 4460 | Implement Trackedsymbolsnocrash | spike | frontend/syntax | class: blocked | Implement Trackedsymbolsnocrash |
| 4461 | Implement Transformnestedgeneratorswithtry | spike | reference/triage | class: triage-needed | Implement Transformnestedgeneratorswithtry |
| 4462 | Implement Transformparenthesizesconditionalsubexpression | spike | frontend/syntax | class: blocked | Implement Transformparenthesizesconditionalsubexpression |
| 4463 | Implement Transformselidenullundefinedtype | spike | frontend/syntax | class: blocked | Implement Transformselidenullundefinedtype |
| 4464 | Implement Transitivetypeargumentinference | spike | frontend/syntax | class: blocked | Implement Transitivetypeargumentinference |
| 4465 | Implement Tripleslashincommentnotparsed | spike | frontend/syntax | class: blocked | Implement Tripleslashincommentnotparsed |
| 4466 | Implement Tripleslashtypesreferencewithmissingexports | spike | frontend/syntax | class: blocked | Implement Tripleslashtypesreferencewithmissingexports |
| 4467 | Implement Trivialsubtypereductionnostructuralcheck | spike | frontend/syntax | class: blocked | Implement Trivialsubtypereductionnostructuralcheck |
| 4468 | Implement Truthinesscallexpressioncoercion Name Resolution | spike | frontend/resolver | class: blocked | Implement Truthinesscallexpressioncoercion Name Resolution |
| 4469 | Implement Truthinesscallexpressioncoercion Parser Syntax | spike | frontend/syntax | class: blocked | Implement Truthinesscallexpressioncoercion Parser Syntax |
| 4470 | Implement Truthinesspromisecoercion | spike | runtime/builtins | class: blocked | Implement Truthinesspromisecoercion |
| 4471 | Implement Trycatchfinally | spike | reference/triage | class: triage-needed | Implement Trycatchfinally |
| 4472 | Implement Trycatchfinallycontrolflow | spike | frontend/syntax | class: blocked | Implement Trycatchfinallycontrolflow |
| 4473 | Implement Tsconfigmapoptionsarecaseinsensitive | spike | frontend/syntax | class: blocked | Implement Tsconfigmapoptionsarecaseinsensitive |
| 4474 | Implement Tslibmissinghelper | spike | reference/triage | class: triage-needed | Implement Tslibmissinghelper |
| 4475 | Implement Tslibmultiplemissinghelper | spike | reference/triage | class: triage-needed | Implement Tslibmultiplemissinghelper |
| 4476 | Implement Tslibnotfounddifferentmodules | spike | reference/triage | class: triage-needed | Implement Tslibnotfounddifferentmodules |
| 4477 | Implement Tslibreexporthelpers | spike | frontend/syntax | class: blocked | Implement Tslibreexporthelpers |
| 4478 | Implement Tsxdefaultimports | spike | frontend/syntax | class: blocked | Implement Tsxdefaultimports |
| 4479 | Implement Tsxfragmentchildrencheck | spike | reference/triage | class: blocked | Implement Tsxfragmentchildrencheck |
| 4480 | Implement Tsxresolveexternalmoduleexportstypes | spike | reference/triage | class: blocked | Implement Tsxresolveexternalmoduleexportstypes |
| 4481 | Implement Tsxtypeargumentpartialdefinitionstillerrors | spike | frontend/syntax | class: blocked | Implement Tsxtypeargumentpartialdefinitionstillerrors |
| 4482 | Implement Tupletypeinference Name Resolution | spike | frontend/resolver | class: blocked | Implement Tupletypeinference Name Resolution |
| 4483 | Implement Tupletypeinference Type System | spike | frontend/syntax | class: blocked | Implement Tupletypeinference Type System |
| 4484 | Implement Tupletypes | spike | reference/triage | class: triage-needed | Implement Tupletypes |
| 4485 | Implement Twicenestedkeyofindexinference | spike | frontend/resolver | class: blocked | Implement Twicenestedkeyofindexinference |
| 4486 | Implement Typealiasdeclarationemit | spike | frontend/syntax | class: blocked | Implement Typealiasdeclarationemit |
| 4487 | Implement Typealiasdoesntmakemoduleinstantiated | spike | frontend/syntax | class: blocked | Implement Typealiasdoesntmakemoduleinstantiated |
| 4488 | Implement Typealiasexport | spike | frontend/syntax | class: blocked | Implement Typealiasexport |
| 4489 | Implement Typealiasfunctiontypesharedsymbol | spike | frontend/syntax | class: blocked | Implement Typealiasfunctiontypesharedsymbol |
| 4490 | Implement Typearginference Name Resolution | spike | frontend/resolver | class: blocked | Implement Typearginference Name Resolution |
| 4491 | Implement Typearginference Type System | spike | frontend/syntax | class: blocked | Implement Typearginference Type System |
| 4492 | Implement Typeargumentconstraintresolution | spike | frontend/syntax | class: blocked | Implement Typeargumentconstraintresolution |
| 4493 | Implement Typeargumentdefaultusesconstraintoncirculardefault | spike | frontend/semantics | class: blocked | Implement Typeargumentdefaultusesconstraintoncirculardefault |
| 4494 | Implement Typeargumentinferencewithconstraintascommonroot | spike | frontend/resolver | class: blocked | Implement Typeargumentinferencewithconstraintascommonroot |
| 4495 | Implement Typeargumentinferencewithrecursivelyreferencedtypealiastotypeliteral | spike | frontend/syntax | class: blocked | Implement Typeargumentinferencewithrecursivelyreferencedtypealiastotypeliteral |
| 4496 | Implement Typeargumentsonfunctionswithnotypeparameters | spike | frontend/resolver | class: blocked | Implement Typeargumentsonfunctionswithnotypeparameters |
| 4497 | Implement Typeargumentsshoulddisallownongenericoverloads | spike | frontend/resolver | class: blocked | Implement Typeargumentsshoulddisallownongenericoverloads |
| 4498 | Implement Typeassertiontogenericfunctiontype | spike | frontend/syntax | class: blocked | Implement Typeassertiontogenericfunctiontype |
| 4499 | Implement Typeassignabilityerrormessage | spike | frontend/resolver | class: blocked | Implement Typeassignabilityerrormessage |
| 4500 | Implement Typecheckobjectcreationexpressionwithundefinedcallresolutiondata | spike | frontend/syntax | class: blocked | Implement Typecheckobjectcreationexpressionwithundefinedcallresolutiondata |
| 4501 | Implement Typechecktypeargument | spike | frontend/semantics | class: blocked | Implement Typechecktypeargument |
| 4502 | Implement Typecheckinginsidefunctionexpressioninarray | spike | frontend/resolver | class: blocked | Implement Typecheckinginsidefunctionexpressioninarray |
| 4503 | Implement Typecomparisoncaching | spike | frontend/resolver | class: blocked | Implement Typecomparisoncaching |
| 4504 | Implement Typeconstraintswithconstructsignatures | spike | frontend/semantics | class: blocked | Implement Typeconstraintswithconstructsignatures |
| 4505 | Implement Typeguardconstructorclassandnumber | spike | frontend/semantics | class: blocked | Implement Typeguardconstructorclassandnumber |
| 4506 | Implement Typeguardconstructorderivedclass | spike | frontend/semantics | class: blocked | Implement Typeguardconstructorderivedclass |
| 4507 | Implement Typeguardnarrowbymutableuntypedfield | spike | frontend/resolver | class: blocked | Implement Typeguardnarrowbymutableuntypedfield |
| 4508 | Implement Typeguardnarrowbyuntypedfield | spike | frontend/resolver | class: blocked | Implement Typeguardnarrowbyuntypedfield |
| 4509 | Implement Typeguardnarrowsindexedaccessofknownproperty Arrow Function | spike | frontend/syntax | class: blocked | Implement Typeguardnarrowsindexedaccessofknownproperty Arrow Function |
| 4510 | Implement Typeguardnarrowsindexedaccessofknownproperty Break Continue | spike | frontend/syntax | class: blocked | Implement Typeguardnarrowsindexedaccessofknownproperty Break Continue |
| 4511 | Implement Typeguardnarrowsindexedaccessofknownproperty Import Export | spike | frontend/syntax | class: blocked | Implement Typeguardnarrowsindexedaccessofknownproperty Import Export |
| 4512 | Implement Typeguardnarrowsindexedaccessofknownproperty Name Resolution | spike | frontend/resolver | class: blocked | Implement Typeguardnarrowsindexedaccessofknownproperty Name Resolution |
| 4513 | Implement Typeguardnarrowsindexedaccessofknownproperty Parser Syntax | spike | frontend/semantics | class: blocked | Implement Typeguardnarrowsindexedaccessofknownproperty Parser Syntax |
| 4514 | Implement Typeguardoncontainertypenohang | spike | frontend/syntax | class: blocked | Implement Typeguardoncontainertypenohang |
| 4515 | Implement Typeidentityconsidersbrands | spike | frontend/syntax | class: blocked | Implement Typeidentityconsidersbrands |
| 4516 | Implement Typeinfer | spike | frontend/syntax | class: blocked | Implement Typeinfer |
| 4517 | Implement Typeinferencecacheinvalidation | spike | frontend/resolver | class: blocked | Implement Typeinferencecacheinvalidation |
| 4518 | Implement Typeinferenceconflictingcandidates | spike | frontend/resolver | class: blocked | Implement Typeinferenceconflictingcandidates |
| 4519 | Implement Typeinferencefixearly | spike | frontend/resolver | class: blocked | Implement Typeinferencefixearly |
| 4520 | Implement Typeinferenceliteralunion | spike | frontend/syntax | class: blocked | Implement Typeinferenceliteralunion |
| 4521 | Implement Typeinferencereturntypecallback | spike | frontend/syntax | class: blocked | Implement Typeinferencereturntypecallback |
| 4522 | Implement Typeinferencetypepredicate Name Resolution | spike | frontend/resolver | class: blocked | Implement Typeinferencetypepredicate Name Resolution |
| 4523 | Implement Typeinferencetypepredicate Type System | spike | frontend/syntax | class: blocked | Implement Typeinferencetypepredicate Type System |
| 4524 | Implement Typeinferencewithtypeannotation | spike | frontend/resolver | class: blocked | Implement Typeinferencewithtypeannotation |
| 4525 | Implement Typeinterfacedeclarationsinblockstatements | spike | frontend/syntax | class: blocked | Implement Typeinterfacedeclarationsinblockstatements |
| 4526 | Implement Typeliteralcallback | spike | frontend/resolver | class: blocked | Implement Typeliteralcallback |
| 4527 | Implement Typematch | spike | frontend/syntax | class: blocked | Implement Typematch |
| 4528 | Implement Typename | spike | frontend/syntax | class: blocked | Implement Typename |
| 4529 | Implement Typenamedundefined | spike | frontend/syntax | class: blocked | Implement Typenamedundefined |
| 4530 | Implement Typeofenumandvarredeclarations | spike | frontend/syntax | class: blocked | Implement Typeofenumandvarredeclarations |
| 4531 | Implement Typeofprototype | spike | frontend/syntax | class: blocked | Implement Typeofprototype |
| 4532 | Implement Typeofthisinstatics | spike | frontend/syntax | class: blocked | Implement Typeofthisinstatics |
| 4533 | Implement Typeofyieldwithunionincontextualreturntype | spike | runtime/builtins | class: blocked | Implement Typeofyieldwithunionincontextualreturntype |
| 4534 | Implement Typeparamextendsothertypeparam | spike | frontend/syntax | class: blocked | Implement Typeparamextendsothertypeparam |
| 4535 | Implement Typeparameterandargumentofsamename | spike | frontend/syntax | class: blocked | Implement Typeparameterandargumentofsamename |
| 4536 | Implement Typeparameterargumentequivalence | spike | frontend/semantics | class: blocked | Implement Typeparameterargumentequivalence |
| 4537 | Implement Typeparameterasbaseclass | spike | frontend/semantics | class: blocked | Implement Typeparameterasbaseclass |
| 4538 | Implement Typeparameterassignmentcompat | spike | frontend/semantics | class: blocked | Implement Typeparameterassignmentcompat |
| 4539 | Implement Typeparametercompatibilityaccrossdeclarations | spike | frontend/semantics | class: blocked | Implement Typeparametercompatibilityaccrossdeclarations |
| 4540 | Implement Typeparameterconstrainedtooutertypeparameter | spike | frontend/resolver | class: blocked | Implement Typeparameterconstrainedtooutertypeparameter |
| 4541 | Implement Typeparameterconstraintinstantiation | spike | frontend/syntax | class: blocked | Implement Typeparameterconstraintinstantiation |
| 4542 | Implement Typeparameterdiamond | spike | frontend/semantics | class: blocked | Implement Typeparameterdiamond |
| 4543 | Implement Typeparameterdoesntblockparameterlookup | spike | frontend/semantics | class: blocked | Implement Typeparameterdoesntblockparameterlookup |
| 4544 | Implement Typeparameterequality | spike | reference/triage | class: triage-needed | Implement Typeparameterequality |
| 4545 | Implement Typeparameterexplicitlyextendsany | spike | frontend/semantics | class: blocked | Implement Typeparameterexplicitlyextendsany |
| 4546 | Implement Typeparameterextendingunion | spike | frontend/semantics | class: blocked | Implement Typeparameterextendingunion |
| 4547 | Implement Typeparameterextendsprimitive | spike | frontend/semantics | class: blocked | Implement Typeparameterextendsprimitive |
| 4548 | Implement Typeparameterfixingwithconstraints | spike | frontend/syntax | class: blocked | Implement Typeparameterfixingwithconstraints |
| 4549 | Implement Typeparameterfixingwithcontextsensitivearguments Arguments Object | spike | frontend/syntax | class: blocked | Implement Typeparameterfixingwithcontextsensitivearguments Arguments Object |
| 4550 | Implement Typeparameterfixingwithcontextsensitivearguments Name Resolution | spike | frontend/resolver | class: blocked | Implement Typeparameterfixingwithcontextsensitivearguments Name Resolution |
| 4551 | Implement Typeparameterinconstraint | spike | frontend/semantics | class: blocked | Implement Typeparameterinconstraint |
| 4552 | Implement Typeparameterleak | spike | frontend/resolver | class: blocked | Implement Typeparameterleak |
| 4553 | Implement Typeparameterlistwithtrailingcomma | spike | frontend/semantics | class: blocked | Implement Typeparameterlistwithtrailingcomma |
| 4554 | Implement Typeparameterwithinvalidconstrainttype | spike | frontend/semantics | class: blocked | Implement Typeparameterwithinvalidconstrainttype |
| 4555 | Implement Typeparametersandparametersincomputednames | spike | frontend/semantics | class: blocked | Implement Typeparametersandparametersincomputednames |
| 4556 | Implement Typeparametersinstaticaccessors | spike | frontend/syntax | class: blocked | Implement Typeparametersinstaticaccessors |
| 4557 | Implement Typeparametersinstaticmethods | spike | frontend/semantics | class: blocked | Implement Typeparametersinstaticmethods |
| 4558 | Implement Typeparametersinstaticproperties | spike | frontend/semantics | class: blocked | Implement Typeparametersinstaticproperties |
| 4559 | Implement Typeparametersshouldnotbeequal | spike | frontend/semantics | class: blocked | Implement Typeparametersshouldnotbeequal |
| 4560 | Implement Typepartameterconstraintinstantiatedwithdefaultwhencheckingdefault | spike | frontend/semantics | class: blocked | Implement Typepartameterconstraintinstantiatedwithdefaultwhencheckingdefault |
| 4561 | Implement Typepredicatefreshliteralwidening | spike | frontend/syntax | class: blocked | Implement Typepredicatefreshliteralwidening |
| 4562 | Implement Typepredicateinloop | spike | frontend/syntax | class: blocked | Implement Typepredicateinloop |
| 4563 | Implement Typepredicateinherit | spike | frontend/syntax | class: blocked | Implement Typepredicateinherit |
| 4564 | Implement Typepredicatestructuralmatch | spike | frontend/syntax | class: blocked | Implement Typepredicatestructuralmatch |
| 4565 | Implement Typepredicatetopleveltypeparameter | spike | frontend/syntax | class: blocked | Implement Typepredicatetopleveltypeparameter |
| 4566 | Implement Typepredicatewiththisparameter | spike | frontend/syntax | class: blocked | Implement Typepredicatewiththisparameter |
| 4567 | Implement Typepredicatescannarrowbydiscriminant | spike | frontend/semantics | class: blocked | Implement Typepredicatescannarrowbydiscriminant |
| 4568 | Implement Typepredicatesinunion Name Resolution | spike | frontend/resolver | class: blocked | Implement Typepredicatesinunion Name Resolution |
| 4569 | Implement Typepredicatesinunion Type System | spike | frontend/syntax | class: blocked | Implement Typepredicatesinunion Type System |
| 4570 | Implement Typepredicatesoptionalchaining Name Resolution | spike | frontend/resolver | class: blocked | Implement Typepredicatesoptionalchaining Name Resolution |
| 4571 | Implement Typepredicatesoptionalchaining Type System | spike | frontend/syntax | class: blocked | Implement Typepredicatesoptionalchaining Type System |
| 4572 | Implement Typereferencedirectivescopedpackagecustomtyperoot | spike | frontend/resolver | class: blocked | Implement Typereferencedirectivescopedpackagecustomtyperoot |
| 4573 | Implement Typereferencedirectivewithfailedfromtyperoot | spike | frontend/syntax | class: blocked | Implement Typereferencedirectivewithfailedfromtyperoot |
| 4574 | Implement Typereferencedirectivewithtypeasfile | spike | frontend/resolver | class: blocked | Implement Typereferencedirectivewithtypeasfile |
| 4575 | Implement Typereferencedirectives Import Export | spike | frontend/syntax | class: blocked | Implement Typereferencedirectives Import Export |
| 4576 | Implement Typereferencedirectives Parser Syntax | spike | frontend/syntax | class: blocked | Implement Typereferencedirectives Parser Syntax |
| 4577 | Implement Typeresolution | spike | frontend/syntax | class: blocked | Implement Typeresolution |
| 4578 | Implement Typerootsfrommultiplenodemodulesdirectories | spike | frontend/syntax | class: blocked | Implement Typerootsfrommultiplenodemodulesdirectories |
| 4579 | Implement Typerootsfromnodemodulesinparentdirectory | spike | frontend/syntax | class: blocked | Implement Typerootsfromnodemodulesinparentdirectory |
| 4580 | Implement Typeusedastypeliteralindex | spike | frontend/resolver | class: blocked | Implement Typeusedastypeliteralindex |
| 4581 | Implement Typeusedasvalueerror Import Export | spike | frontend/syntax | class: blocked | Implement Typeusedasvalueerror Import Export |
| 4582 | Implement Typeusedasvalueerror Name Resolution | spike | frontend/resolver | class: blocked | Implement Typeusedasvalueerror Name Resolution |
| 4583 | Implement Typevalueconflict | spike | frontend/syntax | class: blocked | Implement Typevalueconflict |
| 4584 | Implement Typevariableconstraintintersections | spike | frontend/resolver | class: blocked | Implement Typevariableconstraintintersections |
| 4585 | Implement Typevariableconstraintedtoaliasnotassignabletounion | spike | frontend/syntax | class: blocked | Implement Typevariableconstraintedtoaliasnotassignabletounion |
| 4586 | Implement Typevariabletypeguards | spike | frontend/semantics | class: blocked | Implement Typevariabletypeguards |
| 4587 | Implement Typecheckcommaexpression | spike | frontend/syntax | class: blocked | Implement Typecheckcommaexpression |
| 4588 | Implement Typecheckifcondition | spike | frontend/resolver | class: blocked | Implement Typecheckifcondition |
| 4589 | Implement Typedarrayconstructoroverloads | spike | frontend/syntax | class: blocked | Implement Typedarrayconstructoroverloads |
| 4590 | Implement Typedarrays Name Resolution | spike | frontend/resolver | class: blocked | Implement Typedarrays Name Resolution |
| 4591 | Implement Typedarrays Parser Syntax | spike | runtime/builtins | class: blocked | Implement Typedarrays Parser Syntax |
| 4592 | Implement Typedarrayscrossassignability | spike | frontend/resolver | class: blocked | Implement Typedarrayscrossassignability |
| 4593 | Implement Typedarrayssubarray | spike | frontend/resolver | class: blocked | Implement Typedarrayssubarray |
| 4594 | Implement Typedgenericprototypemember | spike | frontend/syntax | class: blocked | Implement Typedgenericprototypemember |
| 4595 | Implement Typeofambientexternalmodules | spike | frontend/syntax | class: blocked | Implement Typeofambientexternalmodules |
| 4596 | Implement Typeofclass | spike | frontend/resolver | class: blocked | Implement Typeofclass |
| 4597 | Implement Typeofenum | spike | frontend/syntax | class: blocked | Implement Typeofenum |
| 4598 | Implement Typeofexternalmodules | spike | frontend/syntax | class: blocked | Implement Typeofexternalmodules |
| 4599 | Implement Typeofimportinstantiationexpression | spike | frontend/syntax | class: blocked | Implement Typeofimportinstantiationexpression |
| 4600 | Implement Typeofinternalmodules | spike | frontend/syntax | class: blocked | Implement Typeofinternalmodules |
| 4601 | Implement Typeofobjectinference | spike | frontend/syntax | class: blocked | Implement Typeofobjectinference |
| 4602 | Implement Typeofproperty | spike | frontend/syntax | class: blocked | Implement Typeofproperty |
| 4603 | Implement Typeofsimple | spike | frontend/resolver | class: blocked | Implement Typeofsimple |
| 4604 | Implement Typeofstripsfreshness | spike | frontend/resolver | class: blocked | Implement Typeofstripsfreshness |
| 4605 | Implement Typeofthisinmethodsignature | spike | frontend/syntax | class: blocked | Implement Typeofthisinmethodsignature |
| 4606 | Implement Typeofundefined | spike | reference/triage | class: triage-needed | Implement Typeofundefined |
| 4607 | Implement Typeofunknownsymbol | spike | frontend/syntax | class: blocked | Implement Typeofunknownsymbol |
| 4608 | Implement Typeofusedbeforeblockscoped | spike | frontend/syntax | class: blocked | Implement Typeofusedbeforeblockscoped |
| 4609 | Implement Umddependencycomment | spike | frontend/syntax | class: blocked | Implement Umddependencycomment |
| 4610 | Implement Umddependencycommentname | spike | frontend/syntax | class: blocked | Implement Umddependencycommentname |
| 4611 | Implement Umdglobalaugmentationnocrash | spike | frontend/syntax | class: blocked | Implement Umdglobalaugmentationnocrash |
| 4612 | Implement Umdglobalconflict | spike | frontend/syntax | class: blocked | Implement Umdglobalconflict |
| 4613 | Implement Umdnamedamdmode | spike | frontend/syntax | class: blocked | Implement Umdnamedamdmode |
| 4614 | Implement Umdnamespacemergedwithglobalaugmentationisnotcircular | spike | frontend/syntax | class: blocked | Implement Umdnamespacemergedwithglobalaugmentationisnotcircular |
| 4615 | Implement Unaryoperators | spike | frontend/resolver | class: blocked | Implement Unaryoperators |
| 4616 | Implement Unaryoperatorsinstrictmode | spike | reference/triage | class: triage-needed | Implement Unaryoperatorsinstrictmode |
| 4617 | Implement Unaryplus | spike | frontend/syntax | class: triage-needed | Implement Unaryplus |
| 4618 | Implement Uncalledfunctionchecksinconditional Name Resolution | spike | frontend/resolver | class: blocked | Implement Uncalledfunctionchecksinconditional Name Resolution |
| 4619 | Implement Uncalledfunctionchecksinconditional Type System | spike | frontend/syntax | class: blocked | Implement Uncalledfunctionchecksinconditional Type System |
| 4620 | Implement Uncalledfunctionchecksinconditionalperf | spike | frontend/resolver | class: blocked | Implement Uncalledfunctionchecksinconditionalperf |
| 4621 | Implement Uncaughtcompilererror Name Resolution | spike | frontend/resolver | class: blocked | Implement Uncaughtcompilererror Name Resolution |
| 4622 | Implement Uncaughtcompilererror Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Uncaughtcompilererror Unknown Unsupported |
| 4623 | Implement Unclosedexportclause | spike | frontend/syntax | class: blocked | Implement Unclosedexportclause |
| 4624 | Implement Undeclaredbase | spike | frontend/syntax | class: blocked | Implement Undeclaredbase |
| 4625 | Implement Undeclaredmethod | spike | frontend/syntax | class: blocked | Implement Undeclaredmethod |
| 4626 | Implement Undeclaredmoduleerror | spike | frontend/syntax | class: blocked | Implement Undeclaredmoduleerror |
| 4627 | Implement Undeclaredvaremit | spike | frontend/resolver | class: blocked | Implement Undeclaredvaremit |
| 4628 | Implement Undefinedasdiscriminantwithunknown | spike | frontend/syntax | class: blocked | Implement Undefinedasdiscriminantwithunknown |
| 4629 | Implement Undefinedassignabletogenericmappedintersection | spike | frontend/syntax | class: blocked | Implement Undefinedassignabletogenericmappedintersection |
| 4630 | Implement Undefinedsymbolreferencedinarrayliteral | spike | frontend/resolver | class: blocked | Implement Undefinedsymbolreferencedinarrayliteral |
| 4631 | Implement Undefinedtypeargument | spike | frontend/syntax | class: blocked | Implement Undefinedtypeargument |
| 4632 | Implement Undefinedtypeassignment Operator | spike | frontend/syntax | class: blocked | Implement Undefinedtypeassignment Operator |
| 4633 | Implement Undefinedtypeassignment Parser Syntax | spike | frontend/syntax | class: blocked | Implement Undefinedtypeassignment Parser Syntax |
| 4634 | Implement Underscoreescapednameinenum | spike | frontend/syntax | class: blocked | Implement Underscoreescapednameinenum |
| 4635 | Implement Underscoremapfirst | spike | frontend/syntax | class: blocked | Implement Underscoremapfirst |
| 4636 | Implement Underscoretest | spike | frontend/syntax | class: blocked | Implement Underscoretest |
| 4637 | Implement Unexpectedstatementblockterminator | spike | frontend/syntax | class: triage-needed | Implement Unexpectedstatementblockterminator |
| 4638 | Implement Unexportedinstanceclassvariables | spike | frontend/syntax | class: blocked | Implement Unexportedinstanceclassvariables |
| 4639 | Implement Unicodeescapesinnames Parser Syntax | spike | frontend/syntax | class: blocked | Implement Unicodeescapesinnames Parser Syntax |
| 4640 | Implement Unicodeescapesinnames Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Unicodeescapesinnames Unknown Unsupported |
| 4641 | Implement Unicodeidentifiername | spike | frontend/syntax | class: blocked | Implement Unicodeidentifiername |
| 4642 | Implement Unicodestringliteral | spike | frontend/syntax | class: blocked | Implement Unicodestringliteral |
| 4643 | Implement Unioncallmixedtypeparameterpresence | spike | frontend/resolver | class: blocked | Implement Unioncallmixedtypeparameterpresence |
| 4644 | Implement Unionexcesspropertychecknoapparentproptypemismatcherrors | spike | frontend/resolver | class: blocked | Implement Unionexcesspropertychecknoapparentproptypemismatcherrors |
| 4645 | Implement Unionexcesspropswithpartialmember | spike | frontend/resolver | class: blocked | Implement Unionexcesspropswithpartialmember |
| 4646 | Implement Unionofarraysfiltercall | spike | frontend/syntax | class: triage-needed | Implement Unionofarraysfiltercall |
| 4647 | Implement Unionofclasscalls | spike | frontend/syntax | class: blocked | Implement Unionofclasscalls |
| 4648 | Implement Unionofenuminference | spike | frontend/syntax | class: blocked | Implement Unionofenuminference |
| 4649 | Implement Unionoffunctionandsignatureiscallable | spike | frontend/syntax | class: blocked | Implement Unionoffunctionandsignatureiscallable |
| 4650 | Implement Unionpropertyexistence | spike | frontend/resolver | class: blocked | Implement Unionpropertyexistence |
| 4651 | Implement Unionpropertyofprotectedandintersectionproperty | spike | frontend/semantics | class: blocked | Implement Unionpropertyofprotectedandintersectionproperty |
| 4652 | Implement Unionreductionmutualsubtypes | spike | frontend/syntax | class: blocked | Implement Unionreductionmutualsubtypes |
| 4653 | Implement Unionrelationshipcheckpasses | spike | frontend/syntax | class: blocked | Implement Unionrelationshipcheckpasses |
| 4654 | Implement Unionsignatureswiththisparameter | spike | frontend/syntax | class: blocked | Implement Unionsignatureswiththisparameter |
| 4655 | Implement Uniontypeerrormessagetyperefs | spike | frontend/resolver | class: blocked | Implement Uniontypeerrormessagetyperefs |
| 4656 | Implement Uniontypeparameterinference | spike | frontend/resolver | class: blocked | Implement Uniontypeparameterinference |
| 4657 | Implement Uniontypewithindexandmethodsignature | spike | frontend/resolver | class: blocked | Implement Uniontypewithindexandmethodsignature |
| 4658 | Implement Uniontypewithrecursivesubtypereduction Name Resolution | spike | frontend/resolver | class: blocked | Implement Uniontypewithrecursivesubtypereduction Name Resolution |
| 4659 | Implement Uniontypewithrecursivesubtypereduction Parser Syntax | spike | frontend/semantics | class: blocked | Implement Uniontypewithrecursivesubtypereduction Parser Syntax |
| 4660 | Implement Unionwithindexsignature | spike | frontend/syntax | class: blocked | Implement Unionwithindexsignature |
| 4661 | Implement Uniquesymbolallowsindexinobjectwithindexsignature | spike | frontend/syntax | class: blocked | Implement Uniquesymbolallowsindexinobjectwithindexsignature |
| 4662 | Implement Uniquesymbolassignmentonglobalaugmentationsuceeds | spike | frontend/syntax | class: blocked | Implement Uniquesymbolassignmentonglobalaugmentationsuceeds |
| 4663 | Implement Uniquesymboljs Function Resolution | spike | frontend/resolver | class: blocked | Implement Uniquesymboljs Function Resolution |
| 4664 | Implement Uniquesymboljs Parser Syntax | spike | frontend/syntax | class: blocked | Implement Uniquesymboljs Parser Syntax |
| 4665 | Implement Uniquesymbolpropertydeclarationemit | spike | frontend/syntax | class: blocked | Implement Uniquesymbolpropertydeclarationemit |
| 4666 | Implement Unknownlikeunionobjectflagsnotpropagated | spike | reference/triage | class: triage-needed | Implement Unknownlikeunionobjectflagsnotpropagated |
| 4667 | Implement Unknownsymbolingenericreturntype | spike | frontend/syntax | class: blocked | Implement Unknownsymbolingenericreturntype |
| 4668 | Implement Unknownsymboloffcontextualtype | spike | frontend/resolver | class: blocked | Implement Unknownsymboloffcontextualtype |
| 4669 | Implement Unknownsymbols Import Export | spike | frontend/syntax | class: blocked | Implement Unknownsymbols Import Export |
| 4670 | Implement Unknownsymbols Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Unknownsymbols Unknown Unsupported |
| 4671 | Implement Unknowntypeargoncall | spike | frontend/semantics | class: blocked | Implement Unknowntypeargoncall |
| 4672 | Implement Unmatchedparameterpositions | spike | frontend/resolver | class: blocked | Implement Unmatchedparameterpositions |
| 4673 | Implement Unmettypeconstraintinjsdocimportcall | spike | frontend/syntax | class: blocked | Implement Unmettypeconstraintinjsdocimportcall |
| 4674 | Implement Unqualifiedcalltoclassstatic | spike | frontend/syntax | class: blocked | Implement Unqualifiedcalltoclassstatic |
| 4675 | Implement Unreachabledeclarations | spike | frontend/resolver | class: blocked | Implement Unreachabledeclarations |
| 4676 | Implement Unreachableflowafterfinally | spike | frontend/semantics | class: blocked | Implement Unreachableflowafterfinally |
| 4677 | Implement Unreachablejavascriptchecked | spike | frontend/resolver | class: blocked | Implement Unreachablejavascriptchecked |
| 4678 | Implement Unreachableswitchtypeofany | spike | frontend/syntax | class: blocked | Implement Unreachableswitchtypeofany |
| 4679 | Implement Unreachableswitchtypeofunknown | spike | frontend/syntax | class: blocked | Implement Unreachableswitchtypeofunknown |
| 4680 | Implement Unresolvableselfreferencingawaitedunion | spike | frontend/syntax | class: blocked | Implement Unresolvableselfreferencingawaitedunion |
| 4681 | Implement Unresolvedtypeassertionsymbol | spike | frontend/syntax | class: blocked | Implement Unresolvedtypeassertionsymbol |
| 4682 | Implement Unspecializedconstraints | spike | frontend/syntax | class: blocked | Implement Unspecializedconstraints |
| 4683 | Implement Unterminatedregexatendofsource | spike | reference/triage | class: blocked | Implement Unterminatedregexatendofsource |
| 4684 | Implement Unterminatedstringliteralwithbackslash | spike | frontend/syntax | class: blocked | Implement Unterminatedstringliteralwithbackslash |
| 4685 | Implement Untypedargumentinlambdaexpression | spike | frontend/resolver | class: blocked | Implement Untypedargumentinlambdaexpression |
| 4686 | Implement Untypedfunctioncallswithtypeparameters | spike | frontend/syntax | class: triage-needed | Implement Untypedfunctioncallswithtypeparameters |
| 4687 | Implement Untypedmoduleimport | spike | frontend/syntax | class: blocked | Implement Untypedmoduleimport |
| 4688 | Implement Unusedclassesinmodule | spike | frontend/syntax | class: blocked | Implement Unusedclassesinmodule |
| 4689 | Implement Unusedclassesinnamespace | spike | frontend/syntax | class: blocked | Implement Unusedclassesinnamespace |
| 4690 | Implement Unuseddestructuring | spike | frontend/resolver | class: blocked | Implement Unuseddestructuring |
| 4691 | Implement Unusedfunctionsinnamespaces | spike | frontend/syntax | class: blocked | Implement Unusedfunctionsinnamespaces |
| 4692 | Implement Unusedgetterinclass | spike | frontend/semantics | class: blocked | Implement Unusedgetterinclass |
| 4693 | Implement Unusedidentifiersconsolidated | spike | frontend/resolver | class: blocked | Implement Unusedidentifiersconsolidated |
| 4694 | Implement Unusedimportdeclaration | spike | frontend/syntax | class: blocked | Implement Unusedimportdeclaration |
| 4695 | Implement Unusedimportwithspread | spike | frontend/syntax | class: blocked | Implement Unusedimportwithspread |
| 4696 | Implement Unusedimports Import Export | spike | frontend/syntax | class: blocked | Implement Unusedimports Import Export |
| 4697 | Implement Unusedimports Regexp Literal | spike | reference/triage | class: blocked | Implement Unusedimports Regexp Literal |
| 4698 | Implement Unusedinterfaceinnamespace | spike | frontend/syntax | class: blocked | Implement Unusedinterfaceinnamespace |
| 4699 | Implement Unusedinvalidtypearguments | spike | frontend/syntax | class: blocked | Implement Unusedinvalidtypearguments |
| 4700 | Implement Unusedlocalproperty | spike | frontend/resolver | class: blocked | Implement Unusedlocalproperty |
| 4701 | Implement Unusedlocalsandobjectspread Name Resolution | spike | frontend/resolver | class: blocked | Implement Unusedlocalsandobjectspread Name Resolution |
| 4702 | Implement Unusedlocalsandobjectspread Runtime Subset | spike | reference/triage | class: triage-needed | Implement Unusedlocalsandobjectspread Runtime Subset |
| 4703 | Implement Unusedlocalsandparameters | spike | frontend/resolver | class: blocked | Implement Unusedlocalsandparameters |
| 4704 | Implement Unusedlocalsandparametersdeferred | spike | frontend/syntax | class: blocked | Implement Unusedlocalsandparametersdeferred |
| 4705 | Implement Unusedlocalsandparametersoverloadsignatures | spike | frontend/syntax | class: blocked | Implement Unusedlocalsandparametersoverloadsignatures |
| 4706 | Implement Unusedlocalsandparameterstypealiases | spike | frontend/syntax | class: blocked | Implement Unusedlocalsandparameterstypealiases |
| 4707 | Implement Unusedlocalsinforinorof | spike | frontend/syntax | class: blocked | Implement Unusedlocalsinforinorof |
| 4708 | Implement Unusedlocalsinmethod | spike | frontend/resolver | class: blocked | Implement Unusedlocalsinmethod |
| 4709 | Implement Unusedlocalsonfunctiondeclarationwithinfunctionexpression | spike | frontend/resolver | class: blocked | Implement Unusedlocalsonfunctiondeclarationwithinfunctionexpression |
| 4710 | Implement Unusedlocalsonfunctionexpressionwithinfunctiondeclaration | spike | frontend/resolver | class: blocked | Implement Unusedlocalsonfunctionexpressionwithinfunctiondeclaration |
| 4711 | Implement Unusedlocalsonfunctionexpressionwithinfunctionexpression | spike | frontend/resolver | class: blocked | Implement Unusedlocalsonfunctionexpressionwithinfunctionexpression |
| 4712 | Implement Unusedlocalsstartingwithunderscore | spike | frontend/syntax | class: blocked | Implement Unusedlocalsstartingwithunderscore |
| 4713 | Implement Unusedmoduleinmodule | spike | frontend/syntax | class: blocked | Implement Unusedmoduleinmodule |
| 4714 | Implement Unusedmultipleparameter | spike | frontend/resolver | class: blocked | Implement Unusedmultipleparameter |
| 4715 | Implement Unusedmultipleparameters | spike | frontend/resolver | class: blocked | Implement Unusedmultipleparameters |
| 4716 | Implement Unusednamespaceinmodule | spike | frontend/syntax | class: blocked | Implement Unusednamespaceinmodule |
| 4717 | Implement Unusednamespaceinnamespace | spike | frontend/syntax | class: blocked | Implement Unusednamespaceinnamespace |
| 4718 | Implement Unusedparametersinlambda | spike | frontend/resolver | class: blocked | Implement Unusedparametersinlambda |
| 4719 | Implement Unusedparametersthis | spike | frontend/resolver | class: blocked | Implement Unusedparametersthis |
| 4720 | Implement Unusedprivatemembers | spike | frontend/semantics | class: blocked | Implement Unusedprivatemembers |
| 4721 | Implement Unusedprivatemethodinclass | spike | frontend/semantics | class: blocked | Implement Unusedprivatemethodinclass |
| 4722 | Implement Unusedprivatestaticmembers | spike | frontend/semantics | class: blocked | Implement Unusedprivatestaticmembers |
| 4723 | Implement Unusedprivatevariableinclass | spike | frontend/semantics | class: blocked | Implement Unusedprivatevariableinclass |
| 4724 | Implement Unusedsetterinclass | spike | frontend/semantics | class: blocked | Implement Unusedsetterinclass |
| 4725 | Implement Unusedsingleparameterinfunctionexpression | spike | frontend/resolver | class: blocked | Implement Unusedsingleparameterinfunctionexpression |
| 4726 | Implement Unusedsingleparameterinmethoddeclaration | spike | frontend/resolver | class: blocked | Implement Unusedsingleparameterinmethoddeclaration |
| 4727 | Implement Unusedswitchstatement | spike | frontend/resolver | class: blocked | Implement Unusedswitchstatement |
| 4728 | Implement Unusedtypeparameterinfunction | spike | frontend/semantics | class: blocked | Implement Unusedtypeparameterinfunction |
| 4729 | Implement Unusedtypeparameterinlambda | spike | frontend/semantics | class: blocked | Implement Unusedtypeparameterinlambda |
| 4730 | Implement Unusedtypeparameterinmethod | spike | frontend/semantics | class: blocked | Implement Unusedtypeparameterinmethod |
| 4731 | Implement Unusedtypeparameters | spike | frontend/semantics | class: blocked | Implement Unusedtypeparameters |
| 4732 | Implement Unusedtypeparameterscheckedbynounusedparameters | spike | frontend/semantics | class: blocked | Implement Unusedtypeparameterscheckedbynounusedparameters |
| 4733 | Implement Unusedtypeparametersnotcheckedbynounusedlocals | spike | frontend/semantics | class: blocked | Implement Unusedtypeparametersnotcheckedbynounusedlocals |
| 4734 | Implement Unusedtypeparameterswithunderscore | spike | frontend/semantics | class: blocked | Implement Unusedtypeparameterswithunderscore |
| 4735 | Implement Unusedvariableswithunderscoreinbindingelement | spike | reference/triage | class: triage-needed | Implement Unusedvariableswithunderscoreinbindingelement |
| 4736 | Implement Unusedvariableswithunderscoreinforofloop | spike | frontend/syntax | class: triage-needed | Implement Unusedvariableswithunderscoreinforofloop |
| 4737 | Implement Unusedvariablesinblocks | spike | frontend/resolver | class: blocked | Implement Unusedvariablesinblocks |
| 4738 | Implement Unusedvariablesinmodules | spike | frontend/syntax | class: blocked | Implement Unusedvariablesinmodules |
| 4739 | Implement Unusedvariablesinnamespaces | spike | frontend/syntax | class: blocked | Implement Unusedvariablesinnamespaces |
| 4740 | Implement Unwitnessedtypeparametervariance | spike | frontend/semantics | class: blocked | Implement Unwitnessedtypeparametervariance |
| 4741 | Implement Usebeforedeclaration Decorator | spike | frontend/syntax | class: blocked | Implement Usebeforedeclaration Decorator |
| 4742 | Implement Usebeforedeclaration Destructuring | spike | frontend/syntax | class: blocked | Implement Usebeforedeclaration Destructuring |
| 4743 | Implement Usebeforedeclaration Import Export | spike | frontend/syntax | class: blocked | Implement Usebeforedeclaration Import Export |
| 4744 | Implement Usebeforedeclaration Parser Syntax | spike | frontend/syntax | class: blocked | Implement Usebeforedeclaration Parser Syntax |
| 4745 | Implement Usebeforedefinitionindeclarationfiles | spike | frontend/syntax | class: blocked | Implement Usebeforedefinitionindeclarationfiles |
| 4746 | Implement Usedefineforclassfieldsflagdefault | spike | frontend/syntax | class: blocked | Implement Usedefineforclassfieldsflagdefault |
| 4747 | Implement Usestrictlikeprologuestring | spike | frontend/syntax | class: blocked | Implement Usestrictlikeprologuestring |
| 4748 | Implement Useunknownincatchvariables | spike | frontend/syntax | class: triage-needed | Implement Useunknownincatchvariables |
| 4749 | Implement Usedimportnotelidedinjs | spike | frontend/syntax | class: blocked | Implement Usedimportnotelidedinjs |
| 4750 | Implement Usingmodulewithexportimportinvalueposition | spike | frontend/syntax | class: blocked | Implement Usingmodulewithexportimportinvalueposition |
| 4751 | Implement Validregexp | spike | runtime/builtins | class: blocked | Implement Validregexp |
| 4752 | Implement Validuseofthisinsuper | spike | frontend/syntax | class: blocked | Implement Validuseofthisinsuper |
| 4753 | Implement Valueoftypedarray | spike | frontend/resolver | class: blocked | Implement Valueoftypedarray |
| 4754 | Implement Varandfunctionsharename | spike | reference/triage | class: triage-needed | Implement Varandfunctionsharename |
| 4755 | Implement Varargconstructormemberparameter | spike | frontend/semantics | class: blocked | Implement Varargconstructormemberparameter |
| 4756 | Implement Varargparamtypecheck | spike | frontend/syntax | class: blocked | Implement Varargparamtypecheck |
| 4757 | Implement Varargwithnoparamname | spike | frontend/syntax | class: blocked | Implement Varargwithnoparamname |
| 4758 | Implement Varargsonconstructortypes | spike | frontend/syntax | class: blocked | Implement Varargsonconstructortypes |
| 4759 | Implement Varasid | spike | frontend/syntax | class: blocked | Implement Varasid |
| 4760 | Implement Varblock | spike | frontend/syntax | class: blocked | Implement Varblock |
| 4761 | Implement Varnameconflictswithimportindifferentpartofmodule | spike | frontend/syntax | class: blocked | Implement Varnameconflictswithimportindifferentpartofmodule |
| 4762 | Implement Vararg | spike | frontend/syntax | class: blocked | Implement Vararg |
| 4763 | Implement Vardecl | spike | frontend/syntax | class: blocked | Implement Vardecl |
| 4764 | Implement Variabledeclarationdeclarationemituniquesymbolpartialstatement | spike | frontend/syntax | class: blocked | Implement Variabledeclarationdeclarationemituniquesymbolpartialstatement |
| 4765 | Implement Variabledeclarationinnercommentemit | spike | reference/triage | class: triage-needed | Implement Variabledeclarationinnercommentemit |
| 4766 | Implement Variabledeclaratorresolvedduringcontextualtyping | spike | frontend/syntax | class: blocked | Implement Variabledeclaratorresolvedduringcontextualtyping |
| 4767 | Implement Varianceannotationvalidation | spike | frontend/syntax | class: blocked | Implement Varianceannotationvalidation |
| 4768 | Implement Variancecantbestrictwhilestructureisnt | spike | frontend/resolver | class: blocked | Implement Variancecantbestrictwhilestructureisnt |
| 4769 | Implement Variancemeasurement | spike | frontend/syntax | class: blocked | Implement Variancemeasurement |
| 4770 | Implement Varianceproblingandzeroorderindexsignaturerelationsalign | spike | frontend/syntax | class: blocked | Implement Varianceproblingandzeroorderindexsignaturerelationsalign |
| 4771 | Implement Variancepropagation | spike | frontend/syntax | class: triage-needed | Implement Variancepropagation |
| 4772 | Implement Variancereferences | spike | frontend/resolver | class: blocked | Implement Variancereferences |
| 4773 | Implement Variancerepeatedlypropegateswithunreliableflag | spike | frontend/syntax | class: blocked | Implement Variancerepeatedlypropegateswithunreliableflag |
| 4774 | Implement Verbatim | spike | frontend/syntax | class: blocked | Implement Verbatim |
| 4775 | Implement Verbatimmodulesyntaxdefaultvalue | spike | frontend/syntax | class: blocked | Implement Verbatimmodulesyntaxdefaultvalue |
| 4776 | Implement Verbatimmodulesyntaxreactreference | spike | reference/triage | class: blocked | Implement Verbatimmodulesyntaxreactreference |
| 4777 | Implement Vissyntax | spike | frontend/syntax | class: blocked | Implement Vissyntax |
| 4778 | Implement Visibilityofcrossmoduletypeusage | spike | frontend/syntax | class: blocked | Implement Visibilityofcrossmoduletypeusage |
| 4779 | Implement Visibilityoftypeparameters | spike | frontend/syntax | class: blocked | Implement Visibilityoftypeparameters |
| 4780 | Implement Voidarraylit | spike | frontend/syntax | class: blocked | Implement Voidarraylit |
| 4781 | Implement Voidasnonambiguousreturntype | spike | frontend/syntax | class: blocked | Implement Voidasnonambiguousreturntype |
| 4782 | Implement Voidasoperator | spike | frontend/syntax | class: blocked | Implement Voidasoperator |
| 4783 | Implement Voidconstructor | spike | frontend/syntax | class: blocked | Implement Voidconstructor |
| 4784 | Implement Voidfunctionassignmentcompat | spike | frontend/semantics | class: blocked | Implement Voidfunctionassignmentcompat |
| 4785 | Implement Voidoperator | spike | frontend/syntax | class: blocked | Implement Voidoperator |
| 4786 | Implement Voidreturnindexunioninference | spike | frontend/syntax | class: blocked | Implement Voidreturnindexunioninference |
| 4787 | Implement Voidreturnlambdavalue | spike | frontend/syntax | class: blocked | Implement Voidreturnlambdavalue |
| 4788 | Implement Voidundefinedreduction | spike | frontend/resolver | class: blocked | Implement Voidundefinedreduction |
| 4789 | Implement Vuelikedataandpropsinference | spike | frontend/syntax | class: blocked | Implement Vuelikedataandpropsinference |
| 4790 | Implement Weaktype | spike | frontend/semantics | class: blocked | Implement Weaktype |
| 4791 | Implement Weaktypeandprimitivenarrowing | spike | frontend/syntax | class: blocked | Implement Weaktypeandprimitivenarrowing |
| 4792 | Implement Webworkeriterable | spike | runtime/builtins | class: blocked | Implement Webworkeriterable |
| 4793 | Implement Wellknownsymbolexpando | spike | frontend/resolver | class: blocked | Implement Wellknownsymbolexpando |
| 4794 | Implement Widenedtypes | spike | frontend/syntax | class: blocked | Implement Widenedtypes |
| 4795 | Implement Wideningwithtopleveltypeparameter | spike | frontend/semantics | class: blocked | Implement Wideningwithtopleveltypeparameter |
| 4796 | Implement Withexportdecl | spike | frontend/syntax | class: blocked | Implement Withexportdecl |
| 4797 | Implement Withimportdecl | spike | frontend/syntax | class: blocked | Implement Withimportdecl |
| 4798 | Implement Withstatement | spike | frontend/syntax | class: blocked | Implement Withstatement |
| 4799 | Implement Withstatementerrors | spike | runtime/builtins | class: blocked | Implement Withstatementerrors |
| 4800 | Implement Withstatementinternalcomments | spike | frontend/resolver | class: blocked | Implement Withstatementinternalcomments |
| 4801 | Implement Withstatementnestedscope | spike | frontend/syntax | class: blocked | Implement Withstatementnestedscope |
| 4802 | Implement Wrappedincovations | spike | frontend/syntax | class: blocked | Implement Wrappedincovations |
| 4803 | Implement Wrappedrecursivegenerictype | spike | frontend/resolver | class: blocked | Implement Wrappedrecursivegenerictype |
| 4804 | Implement Yieldstarcontextualtype | spike | frontend/semantics | class: blocked | Implement Yieldstarcontextualtype |
| 4805 | Implement Yieldstringliteral | spike | runtime/builtins | class: blocked | Implement Yieldstringliteral |
| 4806 | Implement class syntax | spike | frontend/syntax | class: triage-needed | Implement class syntax |
| 4807 | Implement decorator support | spike | frontend/syntax | class: blocked | Implement decorator support |
| 4808 | Implement import/export module syntax | spike | frontend/syntax | class: blocked | Implement import/export module syntax |
| 4809 | Implement name resolution | spike | frontend/resolver | class: blocked | Implement name resolution |
| 4810 | Implement object literal enhancements | spike | frontend/syntax | class: blocked | Implement object literal enhancements |
| 4811 | Implement parser syntax extensions | spike | frontend/syntax | class: blocked | Implement parser syntax extensions |
| 4812 | Implement RegExp literal support | spike | runtime/builtins | class: blocked | Implement RegExp literal support |
| 4813 | Implement type-system support | spike | frontend/syntax | class: blocked | Implement type-system support |
| 4814 | Investigate and classify unknown-unsupported cases | spike | frontend/syntax | class: triage-needed | Investigate and classify unknown-unsupported cases |
| 5001 | Meta: TypeScript Compiler Semantic Analysis Coverage | meta | frontend/semantics | 5000 | Meta: TypeScript Compiler Semantic Analysis Coverage |
| 5002 | Meta: TypeScript Compiler Type System Coverage | meta | frontend/semantics | 5000, 5005 | Meta: TypeScript Compiler Type System Coverage |
| 5003 | Meta: TypeScript Compiler Declaration Emit Coverage | meta | frontend/syntax | 5000, 5001 | Meta: TypeScript Compiler Declaration Emit Coverage |
| 5005 | Meta: TypeScript Compiler Name Resolution Coverage | meta | frontend/resolver | 5000 | Meta: TypeScript Compiler Name Resolution Coverage |
| 5006 | Meta: TypeScript Compiler Scope Analysis Coverage | meta | frontend/resolver | 5005 | Meta: TypeScript Compiler Scope Analysis Coverage |
| 5012 | Implement Date object support | spike | frontend/syntax | class: triage-needed | Implement Date object support |
| 5013 | Implement duplicate-local support | spike | reference/triage | class: triage-needed | Implement duplicate-local support |
| 5014 | Implement eval support | spike | frontend/syntax | class: triage-needed | Implement eval support |
| 5015 | Implement function support | spike | frontend/syntax | class: triage-needed | Implement function support |
| 5016 | Implement function resolution | spike | frontend/resolver | class: triage-needed | Implement function resolution |
| 5018 | Implement legacy-global-builtin support | spike | frontend/syntax | class: triage-needed | Implement legacy-global-builtin support |
| 5019 | Implement name resolution | spike | frontend/resolver | class: triage-needed | Implement name resolution |
| 5020 | Implement RegExp literal support | spike | frontend/syntax | class: triage-needed | Implement RegExp literal support |
<!-- generated:blocked:end -->

## Done queue

<!-- generated:done:start -->
| ID | Title | Type | Area | Completed evidence |
|---:|---|---|---|---|
| 000 | Short imperative title | feature | bug | refactor | docs | test | infra | cleanup | spike | frontend | ir | runtime | abi | wasi | cli | fixtures | scripts | docs | tests | coverage | reference | see `issues/done/000-sample-issue.md` |
| 001 | Fix issue infrastructure and current-state path references | infra | issues/docs | see `issues/done/001-fix-issue-infrastructure-and-current-state-path-references.md` |
| 002 | Emit canonical capability manifest schema | feature | abi/wasi | see `issues/done/002-emit-canonical-capability-manifest-schema.md` |
| 003 | Verify manifest against emitted WAT imports | test | wasi/tests | see file |
| 004 | Reclassify compile-only compatibility tests | test | tests/coverage | see `issues/done/004-reclassify-compile-only-compatibility-tests.md` |
| 005 | Add fine-grained unsupported feature breakdown | infra | scripts/coverage | see `issues/done/005-add-fine-grained-unsupported-feature-breakdown.md` |
| 006 | Remove stale milestone and transitional docs | cleanup | docs | see file |
| 007 | Harden reference coverage prerequisites | infra | scripts/reference | see file |
| 008 | Introduce typed WAT writer skeleton | refactor | backend | see `issues/done/008-introduce-typed-wat-writer-skeleton.md` |
| 009 | Select first coverage-improvement feature slice | spike | frontend/ir/runtime | see file |
| 010 | Extract frontend module from crates/cli | refactor | frontend | see file |
| 011 | Enable `RUSTFLAGS=-D warnings` for nextest / harness (warning-clean tree) | infra | tests | see `issues/done/011-enable-cargo-deny-warnings-in-ci-and-harnesses.md` |
| 012 | Fix computed property semantics bug | bug | runtime/semantics | see `issues/done/012-fix-computed-property-semantics-bug.md` |
| 013 | Implement heap OOM check | feature | runtime/memory | see file |
| 014 | Implement dynamic property key support | feature | runtime/semantics | see `issues/done/014-implement-dynamic-property-key-support.md` |
| 015 | Implement object literal string key support | feature | parser/semantics | see file |
| 016 | Implement prototype and method call support | feature | runtime/semantics | see `issues/done/016-implement-prototype-and-method-call-support.md` |
| 017 | Design and implement GC strategy | feature | runtime/memory | see file |
| 017a | Design GC strategy | feature | runtime/memory | see `issues/done/017a-design-gc-strategy.md` |
| 018 | Implement UTF-8 string support | feature | runtime/semantics | see file |
| 019 | Integrate TypeScript parser/checker | feature | frontend | see file |
| 019a | Integrate TypeScript compiler API for type checking | feature | frontend | see file |
| 019b | Extract type information for optimization hints | feature | frontend | see file |
| 020 | Implement generic JavaScript semantic IR | feature | ir/semantics | see file |
| 020a | Design JavaScript semantic IR | feature | ir/semantics | see file |
| 020b | Implement IR lowering from TypeScript AST | feature | ir/semantics | see file |
| 020c | Add IR validation passes and document contracts | feature | ir/semantics | see file |
| 021a | Implement wasm-encoder hello binary MVP | feature | backend | see `issues/done/021a-implement-wasm-encoder-hello-binary-mvp.md` |
| 022 | Expand test262 differential coverage | feature | tests/coverage | see file |
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
| 041 | Implement template literals | feature | frontend/semantics | see `issues/done/041-implement-template-literals.md` |
| 042 | Implement string methods | feature | runtime/builtins | see `issues/done/042-implement-string-methods.md` |
| 043 | Implement string indexing | feature | runtime/semantics | see `issues/done/043-implement-string-indexing.md` |
| 044 | Implement String.fromCharCode and charCodeAt | feature | runtime/builtins | see `issues/done/044-implement-string-from-char-code.md` |
| 045 | Implement class declaration and expression | feature | frontend/semantics | see `issues/done/045-implement-class-syntax.md` |
| 046 | Implement extends inheritance | feature | runtime/semantics | see `issues/done/046-implement-extends-inheritance.md` |
| 047 | Implement super keyword | feature | runtime/semantics | see `issues/done/047-implement-super-keyword.md` |
| 048 | Implement prototype chain | feature | runtime/semantics | see `issues/done/048-implement-prototype-chain.md` |
| 049 | Implement Map and Set | feature | runtime/builtins | see `issues/done/049-implement-map-set.md` |
| 050a | Document Date deterministic subset and live-time policy gap | docs | runtime/builtins | see `issues/done/050a-document-date-deterministic-subset-and-live-time-policy-gap.md` |
| 051 | Implement RegExp | feature | runtime/builtins | see `issues/done/051-implement-regexp.md` |
| 052a | Close JSON supported subset contract | docs | runtime/builtins | see `issues/done/052a-close-json-supported-subset-contract.md` |
| 052b | Implement JSON non-integer number representation | feature | runtime/builtins | see `issues/done/052b-implement-json-noninteger-number-representation.md` |
| 052c | Implement JSON UTF-16 and surrogate string handling | feature | runtime/builtins | see `issues/done/052c-implement-json-utf16-surrogate-strings.md` |
| 052d | Implement broader JSON.stringify replacer semantics | feature | runtime/builtins | see `issues/done/052d-implement-json-stringify-broader-replacer-semantics.md` |
| 052e | Complete JSON.stringify boxed argument edge cases | feature | runtime/builtins | see `issues/done/052e-complete-json-stringify-boxed-argument-edge-cases.md` |
| 052f | Implement JSON.parse throw-compatible diagnostics | feature | runtime/builtins | see `issues/done/052f-implement-json-parse-throw-compatible-diagnostics.md` |
| 052g | Implement JSON.stringify function replacer callbacks | feature | runtime/builtins | see `issues/done/052g-implement-json-stringify-function-replacer-callbacks.md` |
| 053 | Implement Math | feature | runtime/builtins | see `issues/done/053-implement-math.md` |
| 054 | Implement Error types | feature | runtime/builtins | see `issues/done/054-implement-error-types.md` |
| 055 | Umbrella: implement import and export | feature | frontend/semantics | see `issues/done/055-implement-import-export.md` |
| 056 | Implement name resolution for variables and identifiers | feature | frontend | see `issues/done/056-implement-name-resolution.md` |
| 057 | Implement function resolution for function calls | feature | frontend | see `issues/done/057-implement-function-resolution.md` |
| 058 | Implement equality operators (==, !=, ===, !==) | feature | runtime/semantics | see `issues/done/058-implement-equality-operators.md` |
| 059 | Implement parser syntax extensions for TypeScript and advanced JS | feature | frontend | see `issues/done/059-implement-parser-syntax-extensions.md` |
| 059a | Implement TypeScript satisfies and const assertion erasure | feature | frontend | see `issues/done/059a-implement-typescript-satisfies-and-const-assertion-erasure.md` |
| 060 | Investigate and classify unknown-unsupported diagnostic cases | spike | frontend | see `issues/done/060-investigate-unknown-unsupported-cases.md` |
| 060a | Close unknown-unsupported fixed-window spike | spike | frontend | see `issues/done/060a-close-unknown-unsupported-fixed-window-spike.md` |
| 061 | Implement Date object support | feature | runtime/builtins | see `issues/done/061-implement-date.md` |
| 061a | Merge Date reference issue into Date epic | cleanup | issues | see `issues/done/061a-merge-date-reference-issue-into-date-epic.md` |
| 062 | Implement function support | feature | frontend/semantics | see `issues/done/062-implement-function.md` |
| 062a | Split function epic into callable child issues | cleanup | issues | see `issues/done/062a-split-function-epic-into-callable-child-issues.md` |
| 062b | Own dynamic Function constructor diagnostics | feature | frontend/semantics | see `issues/done/062b-dynamic-function-constructor-diagnostics.md` |
| 062c | Implement ordinary function declarations and direct calls | feature | frontend/semantics | see `issues/done/062c-ordinary-function-declarations-and-calls.md` |
| 062d | Implement function this and arguments semantics | feature | frontend/semantics | see `issues/done/062d-function-this-and-arguments.md` |
| 062e | Implement function closures | feature | frontend/semantics | see `issues/done/062e-function-closures.md` |
| 062f | Implement function object metadata | feature | frontend/semantics | see `issues/done/062f-function-object-metadata.md` |
| 062g | Define and implement heap closure object ABI and rooting | feature | runtime/abi | see `issues/done/062g-heap-closure-object-abi-and-rooting.md` |
| 063 | Implement function resolution | feature | frontend/resolver | see `issues/done/063-implement-function-resolution.md` |
| 064a | Resolve Date global builtin namespace | feature | frontend | see `issues/done/064a-resolve-date-global-builtin-namespace.md` |
| 065 | Implement parser syntax extensions | feature | frontend/syntax | see `issues/done/065-implement-parser-syntax.md` |
| 065a | Merge duplicate parser syntax issue into 059 | cleanup | issues | see `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` |
| 077 | Implement Interfacedeclaration | spike | frontend/syntax | see `issues/done/077-implement-InterfaceDeclaration.md` |
| 085 | Implement Abstractinterfaceidentifiername | spike | frontend/syntax | see `issues/done/085-implement-abstractInterfaceIdentifierName.md` |
| 095 | Implement Accessorbodyintypecontext | spike | frontend/syntax | see `issues/done/095-implement-accessorBodyInTypeContext.md` |
| 110 | Implement Addmoreoverloadstobasesignature | spike | frontend/syntax | see `issues/done/110-implement-addMoreOverloadsToBaseSignature.md` |
| 186 | Implement Anyindexedaccessarraynoexception | spike | frontend/syntax | see `issues/done/186-implement-anyIndexedAccessArrayNoException.md` |
| 188 | Implement Anyisassignabletoobject | spike | frontend/syntax | see `issues/done/188-implement-anyIsAssignableToObject.md` |
| 189 | Implement Anyisassignabletovoid | spike | frontend/syntax | see `issues/done/189-implement-anyIsAssignableToVoid.md` |
| 190 | Implement Anymappedtypeserror | spike | frontend/syntax | see `issues/done/190-implement-anyMappedTypesError.md` |
| 191 | Implement Anyplusany | spike | frontend/syntax | see `issues/done/191-implement-anyPlusAny.md` |
| 200 | Implement parser syntax extensions | spike | frontend/syntax | see `issues/done/200-implement-parser-syntax.md` |
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
| 217 | Implement GC heap header and trigger accounting | feature | runtime/memory | see file |
| 218 | Implement GC mark root scanning | feature | runtime/memory | see file |
| 219 | Implement GC sweep reuse and fixtures | feature | runtime/memory | see file |
| 220 | Implement GC top-level local roots for object escape fixtures | feature | runtime/memory | see file |
| 221 | Implement GC call-frame roots for closure escape | feature | runtime/memory | see file |
| 222 | Investigate GC high-pressure OOB under repeated local-root allocation | bug | runtime/memory | see file |
| 223 | Add spans to receiver this diagnostics | bug | frontend/diagnostics | see file |
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
| 253 | Implement optional chaining runtime semantics | feature | frontend/semantics | see file |
| 254 | Implement class static block runtime semantics | feature | runtime/semantics | see `issues/done/254-implement-class-static-block-runtime-semantics.md` |
| 255 | Implement private class element runtime semantics | meta | runtime/semantics | see file |
| 256 | Lower returned immutable closures to heap closure values | feature | ir | see `issues/done/256-lower-returned-immutable-closures-to-heap-values.md` |
| 257 | Emit heap closure allocation and dispatch | feature | backend | see `issues/done/257-emit-heap-closure-allocation-and-dispatch.md` |
| 258 | Mark heap closure captures and add allocation-pressure fixture | feature | runtime | see `issues/done/258-mark-heap-closure-captures-and-add-allocation-pressure-fixture.md` |
| 259 | Implement BigInt literal runtime values | feature | runtime/semantics | see `issues/done/259-implement-bigint-literal-runtime-values.md` |
| 260 | Implement BigInt arithmetic operators | feature | runtime/semantics | see `issues/done/260-implement-bigint-arithmetic-operators.md` |
| 261 | Implement BigInt equality comparison and coercion boundaries | feature | runtime/semantics | see `issues/done/261-implement-bigint-equality-comparison-coercion.md` |
| 262 | Implement BigInt builtins and string conversion | feature | runtime/builtins | see `issues/done/262-implement-bigint-builtins-and-string-conversion.md` |
| 263 | Implement BigInt dynamic mul/div/rem signed-i64 runtime slice | feature | runtime/semantics | see `issues/done/263-implement-bigint-dynamic-mul-div-rem-signed-i64-slice.md` |
| 264 | Add broad expression fixture coverage | feature | frontend/syntax | see file |
| 265 | Add broad statement fixture coverage | feature | frontend/syntax | see file |
| 266 | Implement test262 test harness and host-defined functions | feature | tests/harness | see file |
| 267 | Implement interactive web UI for test results | feature | coverage | see `issues/done/267-implement-interactive-web-ui-for-test-results.md` |
| 267a | Implement web UI data generation and script integration | feature | coverage | see `issues/done/267a-web-ui-data-generation-and-script-integration.md` |
| 267b | Implement web UI interactive charts, regression detection, and performance trends | feature | coverage | see `issues/done/267b-web-ui-interactive-charts-regression-and-performance-trends.md` |
| 267c | Implement web UI real-time test run updates | feature | coverage | see `issues/done/267c-web-ui-real-time-test-run-updates.md` |
| 267d | Implement web UI export controls, theme toggle, and usage docs | feature | coverage | see `issues/done/267d-web-ui-export-theme-and-usage-docs.md` |
| 268 | Implement for loop increment operator | feature | frontend/semantics | see file |
| 269 | Implement integer-only Math.pow slice | feature | runtime/builtins | see `issues/done/269-implement-math-pow.md` |
| 270 | Implement Array.prototype.map named-callback slice | feature | runtime/builtins | see `issues/done/270-implement-array-prototype-map.md` |
| 271 | Implement Array.prototype.push | feature | runtime/builtins | see `issues/done/271-implement-array-prototype-push.md` |
| 272 | Implement Set | feature | runtime/builtins | see `issues/done/272-implement-set.md` |
| 273 | Implement recursive function calls | feature | runtime/semantics | see `issues/done/273-implement-recursive-function-calls.md` |
| 274 | Implement spread operator | meta | frontend/semantics | see file |
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
| 315 | Fix Math.max/min backend-io errors | feature | runtime/builtins | see `issues/done/315-fix-math-max-min-backend-io.md` |
| 333 | Implement BigInt dynamic string exception parity | feature | runtime/builtins | see `issues/done/333-implement-bigint-dynamic-string-exception-parity.md` |
| 334 | Array.prototype.map completion: sparse array, thisArg, and generic call | meta | runtime/builtins | see `issues/done/334-complete-array-map-sparse-thisarg-test262.md` |
| 337 | Implement test262 features directive and $262 object | feature | cli/reference | see `issues/done/337-implement-test262-features-directive.md` |
| 338 | Sparse array holes handling for Array.prototype.map | feature | runtime/builtins | see `issues/done/338-array-map-sparse-array-holes.md` |
| 339 | Callback thisArg for Array.prototype.map | feature | runtime/builtins | see `issues/done/339-array-map-thisarg.md` |
| 340 | Generic call for Array.prototype.map (static dense receiver slice) | feature | runtime/builtins | see `issues/done/340-array-map-generic-call.md` |
| 341 | Implement core builtin API coverage (3,190 test262 cases) | meta | runtime/builtins | see file |
| 341a | Implement isNaN, parseInt, parseFloat, isFinite global functions | feature | runtime/builtins | see `issues/done/341a-global-number-functions.md` |
| 341b | Implement Number constructor and static methods | feature | runtime/builtins | see `issues/done/341b-number-constructor.md` |
| 341c | Implement Boolean global | feature | runtime/builtins | see `issues/done/341c-boolean-global.md` |
| 341d | Implement globalThis binding | feature | runtime/builtins | see `issues/done/341d-globalthis-binding.md` |
| 341e | Implement encodeURI, decodeURI, escape, unescape | feature | runtime/builtins | see file |
| 344 | Implement legacy global builtin bindings (8 test262 cases) | feature | runtime/builtins | see file |
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
| 359 | Reduce ABC451 free-list scan cost | bug | runtime/memory | see file |
| 360 | Reduce ABC451 sweep and copy pressure after free-list fix | bug | runtime/memory | see `issues/done/360-reduce-abc451-sweep-and-copy-pressure-after-free-list-fix.md` |
| 361 | Reduce ABC451 array copy pressure after GC cadence fix | bug | runtime/memory | see `issues/done/361-reduce-abc451-array-copy-pressure-after-gc-cadence-fix.md` |
| 362 | Drive ABC451 depth-8 under iwasm timeout after copy reductions | bug | runtime/memory | see `issues/done/362-drive-abc451-depth8-under-iwasm-timeout-after-copy-reductions.md` |
| 364 | Add ABC451 allocation and copy attribution diagnostic | test | runtime/performance | see `issues/done/364-add-abc451-allocation-copy-attribution-diagnostic.md` |
| 366 | Add ABC451 ArrayPushGrow miss attribution diagnostic | test | runtime/performance | see `issues/done/366-add-arraypushgrow-miss-attribution-diagnostic.md` |
| 367 | Extract ArrayPushGrow into a runtime helper | refactor | backend/runtime | see `issues/done/367-extract-arraypushgrow-runtime-helper.md` |
| 368 | Implement remaining BigInt mixed runtime coercion edges | feature | runtime/semantics | see `issues/done/368-implement-remaining-bigint-mixed-runtime-coercion-edges.md` |
| 371 | Define BigInt bitwise and exponentiation policy | feature | runtime/semantics | see `issues/done/371-define-bigint-bitwise-and-exponentiation-policy.md` |
| 372 | Implement BigInt object ToPrimitive non-BigInt primitive returns | feature | runtime/semantics | see `issues/done/372-implement-bigint-object-toprimitive-non-bigint-primitive-returns.md` |
| 373 | Handle BigInt object ToPrimitive invalid and out-of-range string returns | feature | runtime/semantics | see `issues/done/373-handle-bigint-object-toprimitive-invalid-out-of-range-string-returns.md` |
| 375 | Handle non-source-backed out-of-range BigInt/String comparisons | feature | runtime/semantics | see `issues/done/375-handle-non-source-backed-out-of-range-bigint-string-comparisons.md` |
| 376 | Implement dynamic BigInt exponentiation | feature | runtime/semantics | see `issues/done/376-implement-dynamic-bigint-exponentiation.md` |
| 377 | Implement BigInt bitwise NOT/AND/OR/XOR | feature | runtime/semantics | see `issues/done/377-implement-bigint-bitwise-not-and-or-xor.md` |
| 378 | Implement BigInt shift operators and unsigned-right-shift policy | feature | runtime/semantics | see `issues/done/378-implement-bigint-shift-operators.md` |
| 379 | Validate Array.prototype.map thisArg against Test262 | test | reference/tests | see `issues/done/379-validate-array-map-thisarg-test262.md` |
| 380 | BigInt division/remainder by zero RangeError | feature | runtime/semantics | see `issues/done/380-bigint-division-remainder-zero-rangeerror.md` |
| 381 | Mixed Number/BigInt arithmetic TypeError | feature | runtime/semantics | see `issues/done/381-mixed-number-bigint-arithmetic-typeerror.md` |
| 383 | Multi-limb BigInt multiplication | feature | runtime/semantics | see `issues/done/383-multilimb-bigint-multiplication.md` |
| 384 | Multi-limb BigInt division and remainder | feature | runtime/semantics | see `issues/done/384-multilimb-bigint-division-remainder.md` |
| 385 | Instrument ABC451 depth-8 copy vs GC time | feature | runtime/memory | see `issues/done/385-instrument-abc451-depth8-copy-vs-gc-time.md` |
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
| 409 | Implement tsgo declaration emit: package-json exports and subpath reexport cases | feature | frontend/syntax | see `issues/done/409-implement-tsgo-declaration-emit-package-json-subpath.md` |
| 410 | Implement tsgo declaration emit: subpath import declaration emit cases | feature | frontend/syntax | see `issues/done/410-implement-tsgo-declaration-emit-subpath-import-links.md` |
| 444 | Implement RegExp literal support | spike | runtime/builtins | see `issues/done/444-implement-regexp-literal.md` |
| 1001e | Annex B eval-code function declaration residuals (existing-binding/no-skip/skip-early-err patterns) | feature | frontend/semantics | see file |
| 5004 | Meta: Runtime Builtins Coverage (test262) | meta | runtime/builtins | see file |
| 5007 | Meta: TypeScript Compiler Module Resolution Coverage | meta | frontend/resolver | see file |
| 5008 | Implement static ES module export forms (default, named, namespace, re-export) | feature | ir/compiler | see file |
| 5009 | Remaining static ES module export forms (named list, default import, namespace, re-export, side-effect) | feature | ir/compiler | see file |
| 5010 | Implement local named export (export { value } and export { value as alias }) for entry module | feature | ir/compiler | see file |
| 5011 | Represent or reject class runtime values in lowered IR | feature | ir/backend | see file |
| 5017 | Implement html-comment support | spike | frontend/syntax | see `issues/done/5017-implement-html-comment.md` |
| 5021 | Implement string-builtin support | feature | frontend/syntax | see `issues/done/5021-implement-string-builtin.md` |
| 5022 | Implement Array.prototype.every receiver semantics for 2dArrays | feature | runtime/builtins | see file |
| 5023 | Implement API Sample watcher arrow function return | feature | runtime/builtins | see file |
| 5024 | Implement anonymous interface new expression identifier | feature | runtime/builtins | see file |
| 5025 | Implement any as return type instanceof constructor RHS | feature | runtime/builtins | see file |
| 5031 | [cli] Replace placeholder parser keyword/operator tests with real assertions | test | cli | see `issues/done/5031-cli-real-parser-assertions.md` |
| 5032 | [cli] Add deterministic external tool capability detection | feature | cli | see `issues/done/5032-cli-tool-capability-detection.md` |
| 5036 | [compiler] Introduce CompileReport<T> for non-fatal diagnostics | feature | cli | see `issues/done/5036-compiler-compile-report.md` |
| 5037 | [compiler] Complete entry module export lowering for local references | feature | cli | see `issues/done/5037-compiler-module-export-lowering.md` |
| 5047 | [ir] Implement env-cell lowering for outer-scope mutation | feature | ir | see `issues/done/5047-ir-env-cell.md` |
| 5051 | [runtime-abi] Add ABI layout golden tests and versioning | test | abi | see file |
| 5053 | [runtime-abi] Add typed wrappers for tagged values and heap pointers | refactor | abi | see file |
| 5054 | [runtime-abi] Document value tags and object layout as public ABI | docs | abi | see file |
| 5055 | [runtime-abi] Add backward-compatibility tests for ABI constants | test | abi | see file |
| 5056 | [shared] Replace manual TestRecord JSON construction with serde serialization | refactor | coverage | see `issues/done/5056-shared-serde-serialization.md` |
| 5057 | [shared] Version capability manifest schema and migration policy | feature | coverage | see `issues/done/5057-shared-manifest-versioning.md` |
| 5058 | [shared] Deduplicate and canonicalize capability reasons/imports | refactor | coverage | see `issues/done/5058-shared-deduplicate-capabilities.md` |
| 5059 | [shared] Add typed tracking IDs for unsupported and blocked tests | feature | coverage | see `issues/done/5059-shared-tracking-ids.md` |
| 5060 | [shared] Provide shared fixture schemas for CLI/compiler/backend tests | feature | coverage | see `issues/done/5060-shared-fixture-schemas.md` |
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
