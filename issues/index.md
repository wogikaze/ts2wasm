# Issues Index

This file is the human entrypoint for the issue queue.

Issue files are the source of truth for work items. The generated section below may be replaced by a script or pasted manually from a generated report.

## Summary

<!-- generated:summary:start -->
| Area | Total | Open | Resolved |
|---|---:|---:|---:|
| abi | 2 | 0 | 2 |
| backend | 8 | 2 | 6 |
| cli | 5 | 1 | 4 |
| compiler | 1 | 0 | 1 |
| coverage | 5 | 0 | 5 |
| docs | 2 | 0 | 2 |
| frontend | 210 | 132 | 78 |
| ir | 8 | 1 | 7 |
| issues | 4 | 0 | 4 |
| parser | 1 | 0 | 1 |
| reference | 8 | 3 | 5 |
| runtime | 148 | 40 | 108 |
| scripts | 2 | 0 | 2 |
| security | 1 | 0 | 1 |
| tests | 6 | 0 | 6 |
| wasi | 1 | 0 | 1 |
| total | 412 | 179 | 233 |
<!-- generated:summary:end -->

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
| 225 | Implement eval and Annex B function declaration semantics | meta | frontend/semantics | ready | P3 |  | Direct `eval` and dynamic code evaluation are required JavaScript semantics; when wasm-only implementation is not suf... |
| 255 | Implement private class element runtime semantics | meta | runtime/semantics | ready | P2 |  | Issue 248 tokenizes `#name` and parses private fields, methods, getters, and setters. The runtime slices now support ... |
| 274 | Implement spread operator | meta | frontend/semantics | ready | P2 |  | Implement spread operator |
| 309 | Reduce ABC451 depth-9 live allocation shape | feature | runtime/memory | implementation-ready | P1 |  | The depth-9 search-only reducer now fails at the explicit |
| 334 | Array.prototype.map completion: sparse array, thisArg, and generic call | meta | runtime/builtins | ready | P2 |  | supported dense-array map calls work, but `Array.prototype.map` still |
| 373 | Handle BigInt object ToPrimitive invalid and out-of-range string returns | feature | runtime/semantics | implementation-ready | P2 | 259, 261 | Issue 368 implemented `toString: () => <supported decimal string>` for equality and relational comparisons, but inval... |
| 375 | Handle non-source-backed out-of-range BigInt/String comparisons | feature | runtime/semantics | implementation-ready | P2 | 259, 261, 282 | Source-backed local and object-property out-of-range strings are diagnosed, but unknown non-source-backed dynamic str... |
| 378 | Implement BigInt shift operators and unsigned-right-shift policy | feature | runtime/semantics | implementation-ready | P2 | 260 | BigInt shift operators currently report issue-378 diagnostics and must not lower through ordinary number shifts; BigI... |
| 385 | Instrument ABC451 depth-8 copy vs GC time | feature | runtime/memory | implementation-ready | P1 |  | Issue 357's previous attempts to reduce the timeout did not identify whether the bottleneck is array copying, GC swee... |
| 387 | Implement BigInt bitwise outside signed-i64 helper slice | feature | runtime/semantics | implementation-ready | P2 | 260, 377 | issue 377 added BigInt-specific helpers for known operands/results that fit the signed-i64-backed first-limb construc... |
| 396 | Runtime JS exception object throwing substrate | feature | runtime/semantics | implementation-ready | P2 |  | Runtime helper exceptional paths can only trap today, so issue 380 cannot produce a compatible catchable `RangeError`... |
| 399 | Define TypeScript parse, erase, and emit boundary contract | spike | frontend | design-ready | P1 |  | TypeScript parse/erase/emit failures do not have a single boundary contract, so generated issues can be misread as ma... |
<!-- generated:ready:end -->

## Blocked queue

<!-- generated:blocked:start -->
| ID | Title | Type | Area | Blocker | Summary |
|---:|---|---|---|---|---|
| 017b | Implement GC strategy | feature | runtime/memory | class: blocked | Implement GC strategy |
| 021 | Implement full wasm backend | feature | backend | class: blocked | Implement full wasm backend |
| 050 | Implement Date | feature | runtime/builtins | class: blocked | Implement Date |
| 052 | Implement JSON | feature | runtime/builtins | class: blocked | Implement JSON |
| 052d | Implement broader JSON.stringify replacer semantics | feature | runtime/builtins | class: blocked | Implement broader JSON.stringify replacer semantics |
| 064 | Implement name resolution (triaged - superseded by test262 metadata issues) | spike | frontend/resolver | class: blocked | Implement name resolution (triaged - superseded by test262 metadata issues) |
| 066 | Implement RegExp literal support | spike | runtime/builtins | class: triage-needed | Implement RegExp literal support |
| 067 | Investigate and classify unknown-unsupported cases | spike | reference/triage | class: triage-needed | Investigate and classify unknown-unsupported cases |
| 068 | Implement unsupported expression types | spike | frontend/semantics | class: triage-needed | Implement unsupported expression types |
| 069 | Implement Apilibcheck | spike | frontend/syntax | class: triage-needed | Implement Apilibcheck |
| 070 | Implement Apisample | spike | frontend/syntax | class: triage-needed | Implement Apisample |
| 071 | Implement Arrowfunctionexpression | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctionexpression |
| 072 | Implement Classdeclaration | spike | frontend/syntax | class: triage-needed | Implement Classdeclaration |
| 073 | Implement Classdeclarationwithinvalidconstonpropertydeclaration | spike | frontend/syntax | class: triage-needed | Implement Classdeclarationwithinvalidconstonpropertydeclaration |
| 074 | Implement Declarationerrorsnoemitonerror | spike | frontend/syntax | class: triage-needed | Implement Declarationerrorsnoemitonerror |
| 075 | Implement Exportassignment | spike | frontend/syntax | class: triage-needed | Implement Exportassignment |
| 076 | Implement Functiondeclaration | spike | frontend/syntax | class: triage-needed | Implement Functiondeclaration |
| 078 | Implement Memberaccessordeclaration | spike | frontend/syntax | class: triage-needed | Implement Memberaccessordeclaration |
| 079 | Implement Parameterlist | spike | frontend/syntax | class: triage-needed | Implement Parameterlist |
| 080 | Implement Systemmoduleforstatementnoinitializer | spike | frontend/syntax | class: triage-needed | Implement Systemmoduleforstatementnoinitializer |
| 081 | Implement Transportstream | spike | frontend/syntax | class: triage-needed | Implement Transportstream |
| 082 | Implement Abstractclassinlocalscope | spike | frontend/syntax | class: triage-needed | Implement Abstractclassinlocalscope |
| 083 | Implement Abstractclassinlocalscopeisabstract | spike | frontend/syntax | class: triage-needed | Implement Abstractclassinlocalscopeisabstract |
| 084 | Implement Abstractclassunioninstantiation | spike | frontend/syntax | class: triage-needed | Implement Abstractclassunioninstantiation |
| 086 | Implement Abstractpropertybasics | spike | frontend/syntax | class: triage-needed | Implement Abstractpropertybasics |
| 087 | Implement Abstractpropertyinconstructor | spike | frontend/syntax | class: triage-needed | Implement Abstractpropertyinconstructor |
| 088 | Implement Abstractpropertynegative | spike | frontend/syntax | class: triage-needed | Implement Abstractpropertynegative |
| 089 | Implement Acceptsymbolasweaktype | spike | frontend/resolver | class: triage-needed | Implement Acceptsymbolasweaktype |
| 090 | Implement Acceptablealias | spike | frontend/syntax | class: triage-needed | Implement Acceptablealias |
| 091 | Implement Accessinstancememberfromstaticmethod | spike | frontend/syntax | class: triage-needed | Implement Accessinstancememberfromstaticmethod |
| 092 | Implement Accessoverriddenbaseclassmember | spike | frontend/syntax | class: triage-needed | Implement Accessoverriddenbaseclassmember |
| 093 | Implement Accessstaticmemberfrominstancemethod | spike | frontend/syntax | class: triage-needed | Implement Accessstaticmemberfrominstancemethod |
| 094 | Implement Accessoraccidentalcalldiagnostic | spike | frontend/syntax | class: triage-needed | Implement Accessoraccidentalcalldiagnostic |
| 096 | Implement Accessordeclarationemitjs | spike | frontend/syntax | class: triage-needed | Implement Accessordeclarationemitjs |
| 097 | Implement Accessordeclarationemitvisibilityerrors | spike | frontend/syntax | class: triage-needed | Implement Accessordeclarationemitvisibilityerrors |
| 098 | Implement Accessordeclarationorder | spike | frontend/syntax | class: triage-needed | Implement Accessordeclarationorder |
| 099 | Implement Accessorinambientcontextes | spike | frontend/syntax | class: triage-needed | Implement Accessorinambientcontextes |
| 100 | Implement Accessorinferredreturntypeerrorinreturnstatement | spike | frontend/syntax | class: triage-needed | Implement Accessorinferredreturntypeerrorinreturnstatement |
| 101 | Implement Accessorparameteraccessibilitymodifier | spike | frontend/syntax | class: triage-needed | Implement Accessorparameteraccessibilitymodifier |
| 102 | Implement Accessorwithinitializer | spike | frontend/syntax | class: triage-needed | Implement Accessorwithinitializer |
| 103 | Implement Accessorwithlineterminator | spike | frontend/syntax | class: triage-needed | Implement Accessorwithlineterminator |
| 104 | Implement Accessorwithrestparam | spike | frontend/syntax | class: triage-needed | Implement Accessorwithrestparam |
| 105 | Implement Accessorwithoutbody | spike | frontend/syntax | class: triage-needed | Implement Accessorwithoutbody |
| 106 | Implement Accessors | spike | frontend/syntax | class: triage-needed | Implement Accessors |
| 107 | Implement Accessorsemit | spike | frontend/syntax | class: triage-needed | Implement Accessorsemit |
| 108 | Implement Accessorsinambientcontext | spike | frontend/syntax | class: triage-needed | Implement Accessorsinambientcontext |
| 109 | Implement Addmorecallsignaturestobasesignature | spike | frontend/resolver | class: triage-needed | Implement Addmorecallsignaturestobasesignature |
| 111 | Implement Aliasassignments | spike | frontend/syntax | class: triage-needed | Implement Aliasassignments |
| 112 | Implement Aliasbug | spike | frontend/syntax | class: triage-needed | Implement Aliasbug |
| 113 | Implement Aliasdoesnotduplicatesignatures | spike | frontend/syntax | class: triage-needed | Implement Aliasdoesnotduplicatesignatures |
| 114 | Implement Aliaserrors | spike | frontend/syntax | class: triage-needed | Implement Aliaserrors |
| 115 | Implement Aliasinaccessiblemodule | spike | frontend/syntax | class: triage-needed | Implement Aliasinaccessiblemodule |
| 116 | Implement Aliasinstantiationexpressiongenericintersectionnocrash | spike | frontend/syntax | class: triage-needed | Implement Aliasinstantiationexpressiongenericintersectionnocrash |
| 117 | Implement Aliasofgenericfunctionwithrestbehavedsameasunaliased | spike | frontend/syntax | class: triage-needed | Implement Aliasofgenericfunctionwithrestbehavedsameasunaliased |
| 118 | Implement Aliasonmergedmoduleinterface | spike | frontend/syntax | class: triage-needed | Implement Aliasonmergedmoduleinterface |
| 119 | Implement Aliasusageinaccessorsofclass | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinaccessorsofclass |
| 120 | Implement Aliasusageinarray | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinarray |
| 121 | Implement Aliasusageinfunctionexpression | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinfunctionexpression |
| 122 | Implement Aliasusageingenericfunction | spike | frontend/syntax | class: triage-needed | Implement Aliasusageingenericfunction |
| 123 | Implement Aliasusageinindexerofclass | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinindexerofclass |
| 124 | Implement Aliasusageinobjectliteral | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinobjectliteral |
| 125 | Implement Aliasusageinorexpression | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinorexpression |
| 126 | Implement Aliasusageintypeargumentofextendsclause | spike | frontend/syntax | class: triage-needed | Implement Aliasusageintypeargumentofextendsclause |
| 127 | Implement Aliasusageinvarassignment | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinvarassignment |
| 128 | Implement Aliasusedasnamevalue | spike | frontend/syntax | class: triage-needed | Implement Aliasusedasnamevalue |
| 129 | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer | spike | frontend/syntax | class: triage-needed | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer |
| 130 | Implement Aliasesinsystemmodule | spike | frontend/syntax | class: triage-needed | Implement Aliasesinsystemmodule |
| 131 | Implement Allowimportclausestomergewithtypes | spike | frontend/syntax | class: triage-needed | Implement Allowimportclausestomergewithtypes |
| 132 | Implement Allowjsclassthistypecrash | spike | frontend/semantics | class: triage-needed | Implement Allowjsclassthistypecrash |
| 133 | Implement Allowjscrossmonorepopackage | spike | frontend/syntax | class: triage-needed | Implement Allowjscrossmonorepopackage |
| 134 | Implement Allowjscheckjstypeparameternocrash | spike | frontend/syntax | class: triage-needed | Implement Allowjscheckjstypeparameternocrash |
| 135 | Implement Allowsyntheticdefaultimports | spike | frontend/syntax | class: triage-needed | Implement Allowsyntheticdefaultimports |
| 136 | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration | spike | frontend/syntax | class: triage-needed | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration |
| 137 | Implement Alwaysstrictalreadyusestrict | spike | frontend/syntax | class: triage-needed | Implement Alwaysstrictalreadyusestrict |
| 138 | Implement Alwaysstrictmodule | spike | frontend/syntax | class: triage-needed | Implement Alwaysstrictmodule |
| 139 | Implement Alwaysstrictnoimplicitusestrict | spike | frontend/syntax | class: triage-needed | Implement Alwaysstrictnoimplicitusestrict |
| 140 | Implement Ambientclassdeclarationwithextends | spike | frontend/syntax | class: triage-needed | Implement Ambientclassdeclarationwithextends |
| 141 | Implement Ambientclassdeclaredbeforebase | spike | frontend/syntax | class: triage-needed | Implement Ambientclassdeclaredbeforebase |
| 142 | Implement Ambientclassmergesoverloadswithinterface | spike | frontend/syntax | class: triage-needed | Implement Ambientclassmergesoverloadswithinterface |
| 143 | Implement Ambientclassoverloadforfunction | spike | frontend/syntax | class: triage-needed | Implement Ambientclassoverloadforfunction |
| 144 | Implement Ambientconstliterals | spike | frontend/syntax | class: triage-needed | Implement Ambientconstliterals |
| 145 | Implement Ambientenum | spike | frontend/syntax | class: triage-needed | Implement Ambientenum |
| 146 | Implement Ambientenumelementinitializer | spike | frontend/syntax | class: triage-needed | Implement Ambientenumelementinitializer |
| 147 | Implement Ambienterrors | spike | frontend/syntax | class: triage-needed | Implement Ambienterrors |
| 148 | Implement Ambientexportdefaulterrors | spike | frontend/syntax | class: triage-needed | Implement Ambientexportdefaulterrors |
| 149 | Implement Ambientexternalmoduleinanotherexternalmodule | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmoduleinanotherexternalmodule |
| 150 | Implement Ambientexternalmodulereopen | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulereopen |
| 151 | Implement Ambientexternalmodulewithinternalimportdeclaration | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulewithinternalimportdeclaration |
| 152 | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration |
| 153 | Implement Ambientexternalmodulewithrelativemodulename | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulewithrelativemodulename |
| 154 | Implement Ambientexternalmodulewithoutinternalimportdeclaration | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulewithoutinternalimportdeclaration |
| 155 | Implement Ambientfundule | spike | frontend/syntax | class: triage-needed | Implement Ambientfundule |
| 156 | Implement Ambientgetters | spike | frontend/syntax | class: triage-needed | Implement Ambientgetters |
| 157 | Implement Ambientmoduleexports | spike | frontend/syntax | class: triage-needed | Implement Ambientmoduleexports |
| 158 | Implement Ambientmodulewithclassdeclarationwithextends | spike | frontend/syntax | class: triage-needed | Implement Ambientmodulewithclassdeclarationwithextends |
| 159 | Implement Ambientmodulewithtemplateliterals | spike | frontend/syntax | class: triage-needed | Implement Ambientmodulewithtemplateliterals |
| 160 | Implement Ambientmodules | spike | frontend/syntax | class: triage-needed | Implement Ambientmodules |
| 161 | Implement Ambientnamerestrictions | spike | frontend/syntax | class: triage-needed | Implement Ambientnamerestrictions |
| 162 | Implement Ambientpropertydeclarationinjs | spike | frontend/syntax | class: triage-needed | Implement Ambientpropertydeclarationinjs |
| 163 | Implement Ambientrequirefunction | spike | frontend/syntax | class: triage-needed | Implement Ambientrequirefunction |
| 164 | Implement Ambientstatement | spike | frontend/syntax | class: triage-needed | Implement Ambientstatement |
| 165 | Implement Ambientwithstatements | spike | frontend/syntax | class: triage-needed | Implement Ambientwithstatements |
| 166 | Implement Ambiguouscallswherereturntypesagree | spike | frontend/syntax | class: triage-needed | Implement Ambiguouscallswherereturntypesagree |
| 167 | Implement Ambiguousgenericassertion | spike | frontend/syntax | class: triage-needed | Implement Ambiguousgenericassertion |
| 168 | Implement Ambiguousoverload | spike | frontend/syntax | class: triage-needed | Implement Ambiguousoverload |
| 169 | Implement Ambiguousoverloadresolution | spike | frontend/syntax | class: triage-needed | Implement Ambiguousoverloadresolution |
| 170 | Implement Amddeclarationemitnoextradeclare | spike | frontend/syntax | class: triage-needed | Implement Amddeclarationemitnoextradeclare |
| 171 | Implement Amddependencycomment | spike | frontend/syntax | class: triage-needed | Implement Amddependencycomment |
| 172 | Implement Amddependencycommentname | spike | frontend/syntax | class: triage-needed | Implement Amddependencycommentname |
| 173 | Implement Amdlikeinputdeclarationemit | spike | frontend/syntax | class: triage-needed | Implement Amdlikeinputdeclarationemit |
| 174 | Implement Amdmodulebundlenoduplicatedeclarationemitcomments | spike | frontend/syntax | class: triage-needed | Implement Amdmodulebundlenoduplicatedeclarationemitcomments |
| 175 | Implement Amdmoduleconstenumusage | spike | frontend/syntax | class: triage-needed | Implement Amdmoduleconstenumusage |
| 176 | Implement Amdmodulename | spike | frontend/syntax | class: triage-needed | Implement Amdmodulename |
| 177 | Implement Anonclassdeclarationemitisanon | spike | frontend/syntax | class: triage-needed | Implement Anonclassdeclarationemitisanon |
| 178 | Implement Anonterface | spike | frontend/syntax | class: triage-needed | Implement Anonterface |
| 179 | Implement Anonymousclassdeclarationdoesntprintwithreadonly | spike | frontend/syntax | class: triage-needed | Implement Anonymousclassdeclarationdoesntprintwithreadonly |
| 180 | Implement Anonymousclassexpression | spike | frontend/syntax | class: triage-needed | Implement Anonymousclassexpression |
| 181 | Implement Anonymousmodules | spike | frontend/syntax | class: triage-needed | Implement Anonymousmodules |
| 182 | Implement Anyandunknownhavefalsycomponents | spike | frontend/syntax | class: triage-needed | Implement Anyandunknownhavefalsycomponents |
| 183 | Implement Anyasreturntypefornewoncall | spike | frontend/syntax | class: triage-needed | Implement Anyasreturntypefornewoncall |
| 184 | Implement Anydeclare | spike | frontend/syntax | class: triage-needed | Implement Anydeclare |
| 185 | Implement Anyidenticaltoitself | spike | frontend/syntax | class: triage-needed | Implement Anyidenticaltoitself |
| 187 | Implement Anyinferenceanonymousfunctions | spike | frontend/syntax | class: triage-needed | Implement Anyinferenceanonymousfunctions |
| 192 | Implement Argsinscope | spike | frontend/syntax | class: triage-needed | Implement Argsinscope |
| 193 | Implement Arguments | spike | frontend/resolver | class: triage-needed | Implement Arguments |
| 194 | Implement Argumentsaspropertyname | spike | frontend/semantics | class: triage-needed | Implement Argumentsaspropertyname |
| 195 | Implement Argumentsbindstofunctionscopeargumentlist | spike | frontend/resolver | class: triage-needed | Implement Argumentsbindstofunctionscopeargumentlist |
| 196 | Implement Argumentsobjectcreatesrestforjs | spike | frontend/resolver | class: triage-needed | Implement Argumentsobjectcreatesrestforjs |
| 197 | Implement Argumentsobjectiterator | spike | frontend/semantics | class: triage-needed | Implement Argumentsobjectiterator |
| 198 | Implement Argumentspropertynameinjsmode | spike | frontend/semantics | class: triage-needed | Implement Argumentspropertynameinjsmode |
| 199 | Implement Compiler | spike | frontend/syntax | class: triage-needed | Implement Compiler |
| 201 | Investigate and classify unknown-unsupported cases | spike | reference/triage | class: triage-needed | Investigate and classify unknown-unsupported cases |
| 240 | Implement Date timezone-aware toString policy | feature | runtime/builtins | class: blocked | Implement Date timezone-aware toString policy |
| 294 | Support ABC451 D original submission without source rewrite | feature | frontend/runtime | class: blocked | Support ABC451 D original submission without source rewrite |
| 300 | Support ABC451 large integer number boundary | feature | runtime | class: blocked | Support ABC451 large integer number boundary |
| 308 | Implement ABC451 depth-9 GC cadence policy | feature | runtime/memory | class: blocked | Implement ABC451 depth-9 GC cadence policy |
| 311 | Fix test262 arguments object index assignment semantics | bug | runtime/semantics | class: blocked | Fix test262 arguments object index assignment semantics |
| 312 | Triage test262 blocked P0 window | spike | reference | class: triage-needed | Triage test262 blocked P0 window |
| 313 | Implement array-builtin support | spike | runtime/builtins | class: triage-needed | Implement array-builtin support |
| 314 | Implement string-builtin support | spike | runtime/builtins | class: triage-needed | Implement string-builtin support |
| 316 | Fix Object.keys backend-io error | feature | runtime/builtins | class: triage-needed | Fix Object.keys backend-io error |
| 335 | Implement full Math.pow number semantics | feature | runtime/builtins | class: blocked | Implement full Math.pow number semantics |
| 336 | Implement test262 includes directive processing | feature | cli/reference | class: blocked | Implement test262 includes directive processing |
| 338 | Sparse array holes handling for Array.prototype.map | feature | runtime/builtins | 334 | Sparse array holes handling for Array.prototype.map |
| 341 | Implement core builtin API coverage (3,190 test262 cases) | feature | runtime/builtins | class: triage-needed | Implement core builtin API coverage (3,190 test262 cases) |
| 342 | Implement Object builtin method coverage (1,721 test262 cases) | feature | runtime/builtins | class: triage-needed | Implement Object builtin method coverage (1,721 test262 cases) |
| 343 | Implement DuplicateLocal diagnostic detection (66 test262 cases) | feature | frontend/resolver | class: triage-needed | Implement DuplicateLocal diagnostic detection (66 test262 cases) |
| 344 | Implement legacy global builtin bindings (8 test262 cases) | feature | runtime/builtins | class: triage-needed | Implement legacy global builtin bindings (8 test262 cases) |
| 345 | Implement TypeScript type alias coverage for tsc suite (23 cases) | feature | frontend/syntax | class: triage-needed | Implement TypeScript type alias coverage for tsc suite (23 cases) |
| 346 | Implement TypeScript declaration emit coverage for tsgo suite (16 cases) | feature | frontend/syntax | class: triage-needed | Implement TypeScript declaration emit coverage for tsgo suite (16 cases) |
| 347 | Parser and resolver support for direct eval and eval-code scope | feature | frontend/semantics | class: blocked | Parser and resolver support for direct eval and eval-code scope |
| 348 | Lowering block-level function declarations in direct eval code | feature | ir | 347 | Lowering block-level function declarations in direct eval code |
| 349 | Runtime helper or shim JavaScript emission for direct eval execution | feature | backend | 347, 348 | Runtime helper or shim JavaScript emission for direct eval execution |
| 351 | Implement full private brand storage and brand-checking semantics | feature | runtime/semantics | 255 | Implement full private brand storage and brand-checking semantics |
| 353 | Implement iterator protocol integration for spread operator | feature | runtime/semantics | 274 | Implement iterator protocol integration for spread operator |
| 354 | Implement sparse array spread support | feature | runtime/semantics | 274 | Implement sparse array spread support |
| 355 | Implement dynamic object property enumeration spread | feature | runtime/semantics | 274 | Implement dynamic object property enumeration spread |
| 357 | Fix ABC451 depth-8 iwasm timeout | bug | runtime/memory | class: blocked | Fix ABC451 depth-8 iwasm timeout |
| 363 | Reduce ABC451 allocation and sweep volume after bulk copy narrowing | bug | runtime/memory | class: blocked | Reduce ABC451 allocation and sweep volume after bulk copy narrowing |
| 365 | Reduce ABC451 array-growth allocation and copy pressure | bug | runtime/memory | class: blocked | Reduce ABC451 array-growth allocation and copy pressure |
| 369 | Implement full multi-limb BigInt arithmetic | feature | runtime/semantics | class: blocked | Implement full multi-limb BigInt arithmetic |
| 370 | Implement BigInt arithmetic RangeError and TypeError parity | feature | runtime/semantics | class: blocked | Implement BigInt arithmetic RangeError and TypeError parity |
| 374 | Design broader object ToPrimitive for mixed BigInt comparisons | design | runtime/semantics | class: blocked | Design broader object ToPrimitive for mixed BigInt comparisons |
| 380 | BigInt division/remainder by zero RangeError | feature | runtime/semantics | class: blocked | BigInt division/remainder by zero RangeError |
| 381 | Mixed Number/BigInt arithmetic TypeError | feature | runtime/semantics | class: blocked | Mixed Number/BigInt arithmetic TypeError |
| 382 | Multi-limb BigInt addition and subtraction | feature | runtime/semantics | class: blocked | Multi-limb BigInt addition and subtraction |
| 386 | Reduce ABC451 depth-8 array copy pressure | feature | runtime/memory | class: triage-needed | Reduce ABC451 depth-8 array copy pressure |
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
| 256 | Lower returned immutable closures to heap closure values | feature | ir | see `issues/done/256-lower-returned-immutable-closures-to-heap-values.md` |
| 257 | Emit heap closure allocation and dispatch | feature | backend | see `issues/done/257-emit-heap-closure-allocation-and-dispatch.md` |
| 258 | Mark heap closure captures and add allocation-pressure fixture | feature | runtime | see `issues/done/258-mark-heap-closure-captures-and-add-allocation-pressure-fixture.md` |
| 259 | Implement BigInt literal runtime values | feature | runtime/semantics | see `issues/done/259-implement-bigint-literal-runtime-values.md` |
| 260 | Implement BigInt arithmetic operators | feature | runtime/semantics | see `issues/done/260-implement-bigint-arithmetic-operators.md` |
| 261 | Implement BigInt equality comparison and coercion boundaries | feature | runtime/semantics | see `issues/done/261-implement-bigint-equality-comparison-coercion.md` |
| 262 | Implement BigInt builtins and string conversion | feature | runtime/builtins | see `issues/done/262-implement-bigint-builtins-and-string-conversion.md` |
| 263 | Implement BigInt dynamic mul/div/rem signed-i64 runtime slice | feature | runtime/semantics | see `issues/done/263-implement-bigint-dynamic-mul-div-rem-signed-i64-slice.md` |
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
| 315 | Fix Math.max/min backend-io errors | feature | runtime/builtins | see `issues/done/315-fix-math-max-min-backend-io.md` |
| 333 | Implement BigInt dynamic string exception parity | feature | runtime/builtins | see `issues/done/333-implement-bigint-dynamic-string-exception-parity.md` |
| 337 | Implement test262 features directive and $262 object | feature | cli/reference | see `issues/done/337-implement-test262-features-directive.md` |
| 339 | Callback thisArg for Array.prototype.map | feature | runtime/builtins | see `issues/done/339-array-map-thisarg.md` |
| 340 | Generic call for Array.prototype.map (static dense receiver slice) | feature | runtime/builtins | see `issues/done/340-array-map-generic-call.md` |
| 350 | Implement derived-class private element initialization | feature | runtime/semantics | see `issues/done/350-derived-class-private-element-init.md` |
| 352 | Implement static private field ordering with static blocks | feature | runtime/semantics | see `issues/done/352-static-private-field-static-blocks-order.md` |
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
| 376 | Implement dynamic BigInt exponentiation | feature | runtime/semantics | see `issues/done/376-implement-dynamic-bigint-exponentiation.md` |
| 377 | Implement BigInt bitwise NOT/AND/OR/XOR | feature | runtime/semantics | see `issues/done/377-implement-bigint-bitwise-not-and-or-xor.md` |
| 379 | Validate Array.prototype.map thisArg against Test262 | test | reference/tests | see `issues/done/379-validate-array-map-thisarg-test262.md` |
| 383 | Multi-limb BigInt multiplication | feature | runtime/semantics | see `issues/done/383-multilimb-bigint-multiplication.md` |
| 384 | Multi-limb BigInt division and remainder | feature | runtime/semantics | see `issues/done/384-multilimb-bigint-division-remainder.md` |
| 388 | Runtime array-like generic call for Array.prototype.map | feature | runtime/builtins | see `issues/done/388-array-map-generic-runtime-array-like.md` |
| 389 | Unblock Array.map generic Test262 representative with function-expression initializer | bug | frontend/parser | see `issues/done/389-array-map-generic-test262-function-expression-blocker.md` |
| 390 | Allow or rewrite Test262 harness undefined binding name | bug | frontend/parser | see `issues/done/390-test262-harness-undefined-binding-name.md` |
| 391 | Multi-limb BigInt division | feature | runtime/semantics | see `issues/done/391-multilimb-bigint-division.md` |
| 392 | Multi-limb BigInt remainder | feature | runtime/semantics | see `issues/done/392-multilimb-bigint-remainder.md` |
| 393 | Multi-limb BigInt addition | feature | runtime/semantics | see `issues/done/393-multilimb-bigint-addition.md` |
| 394 | Multi-limb BigInt subtraction | feature | runtime/semantics | see `issues/done/394-multilimb-bigint-subtraction.md` |
| 395 | Runtime array-like Array.prototype.map callback dispatch | feature | runtime/builtins | see `issues/done/395-array-map-runtime-array-like-callback-dispatch.md` |
| 397 | BigInt add/sub branch-assigned operands | feature | runtime/semantics | see `issues/done/397-bigint-add-sub-branch-assigned-operands.md` |
| 398 | Track control-flow-assigned BigInt div/rem locals | feature | runtime/semantics | see `issues/done/398-track-control-flow-assigned-bigint-div-rem.md` |
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
