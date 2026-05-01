# Issues Index

This file is the human entrypoint for the issue queue.

Issue files are the source of truth for work items. The generated section below may be replaced by a script or pasted manually from a generated report.

## Summary

<!-- generated:summary:start -->
| Area | Total | Open | Resolved |
|---|---:|---:|---:|
| abi | 2 | 0 | 2 |
| backend | 8 | 1 | 7 |
| cli | 5 | 1 | 4 |
| compiler | 1 | 0 | 1 |
| coverage | 5 | 0 | 5 |
| docs | 2 | 0 | 2 |
| frontend | 774 | 687 | 87 |
| ir | 9 | 0 | 9 |
| issues | 4 | 0 | 4 |
| parser | 1 | 0 | 1 |
| reference | 38 | 33 | 5 |
| runtime | 158 | 31 | 127 |
| scripts | 2 | 0 | 2 |
| security | 1 | 0 | 1 |
| tests | 6 | 0 | 6 |
| wasi | 1 | 0 | 1 |
| total | 1017 | 753 | 264 |
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
| 341 | Implement core builtin API coverage (3,190 test262 cases) | meta | runtime/builtins | ready | P1 |  | 3,190 test262 cases fail due to missing core builtin API implementations. |
| 409 | Implement tsgo declaration emit: package-json exports and subpath reexport cases | feature | frontend/syntax | implementation-ready | P2 | 399 | Implement tsgo declaration emit: package-json exports and subpath reexport cases |
| 410 | Implement tsgo declaration emit: subpath import declaration emit cases | feature | frontend/syntax | implementation-ready | P2 | 399 | Implement tsgo declaration emit: subpath import declaration emit cases |
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
| 309 | Reduce ABC451 depth-9 live allocation shape | feature | runtime/memory | class: blocked | Reduce ABC451 depth-9 live allocation shape |
| 311 | Fix test262 arguments object index assignment semantics | bug | runtime/semantics | class: blocked | Fix test262 arguments object index assignment semantics |
| 312 | Triage test262 blocked P0 window | spike | reference | class: triage-needed | Triage test262 blocked P0 window |
| 313 | Implement array-builtin support | spike | runtime/builtins | class: triage-needed | Implement array-builtin support |
| 314 | Implement string-builtin support | spike | runtime/builtins | class: triage-needed | Implement string-builtin support |
| 316 | Fix Object.keys backend-io error | feature | runtime/builtins | class: triage-needed | Fix Object.keys backend-io error |
| 335 | Implement full Math.pow number semantics | feature | runtime/builtins | class: blocked | Implement full Math.pow number semantics |
| 336 | Implement test262 includes directive processing | feature | cli/reference | class: blocked | Implement test262 includes directive processing |
| 342 | Implement Object builtin method coverage (1,721 test262 cases) | feature | runtime/builtins | class: triage-needed | Implement Object builtin method coverage (1,721 test262 cases) |
| 343 | Implement DuplicateLocal diagnostic detection (66 test262 cases) | feature | frontend/resolver | class: triage-needed | Implement DuplicateLocal diagnostic detection (66 test262 cases) |
| 344 | Implement legacy global builtin bindings (8 test262 cases) | feature | runtime/builtins | class: triage-needed | Implement legacy global builtin bindings (8 test262 cases) |
| 345 | Implement TypeScript type alias coverage for tsc suite (23 cases) | feature | frontend/syntax | class: triage-needed | Implement TypeScript type alias coverage for tsc suite (23 cases) |
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
| 412 | Implement arguments-object support | spike | frontend/syntax | class: triage-needed | Implement arguments-object support |
| 413 | Implement arity support | spike | reference/triage | class: triage-needed | Implement arity support |
| 414 | Implement array-builtin support | spike | frontend/syntax | class: triage-needed | Implement array-builtin support |
| 415 | Implement arrow functions | spike | frontend/syntax | class: triage-needed | Implement arrow functions |
| 416 | Implement async/await support | spike | frontend/syntax | class: triage-needed | Implement async/await support |
| 417 | Implement async-iteration support | spike | frontend/syntax | class: triage-needed | Implement async-iteration support |
| 418 | Implement break/continue | spike | frontend/syntax | class: triage-needed | Implement break/continue |
| 419 | Implement built-in API support | spike | runtime/builtins | class: triage-needed | Implement built-in API support |
| 420 | Implement call expression support | spike | frontend/syntax | class: triage-needed | Implement call expression support |
| 421 | Implement class syntax | spike | frontend/syntax | class: triage-needed | Implement class syntax |
| 422 | Implement class-accessor support | spike | frontend/syntax | class: triage-needed | Implement class-accessor support |
| 423 | Implement Date object support | spike | runtime/builtins | class: triage-needed | Implement Date object support |
| 424 | Implement declaration-emit support | spike | frontend/syntax | class: triage-needed | Implement declaration-emit support |
| 425 | Implement destructuring | spike | frontend/syntax | class: triage-needed | Implement destructuring |
| 426 | Implement duplicate-function support | spike | reference/triage | class: triage-needed | Implement duplicate-function support |
| 427 | Implement duplicate-local support | spike | reference/triage | class: triage-needed | Implement duplicate-local support |
| 428 | Implement enum support | spike | frontend/syntax | class: triage-needed | Implement enum support |
| 429 | Implement eval support | spike | reference/triage | class: triage-needed | Implement eval support |
| 430 | Implement function support | spike | frontend/syntax | class: triage-needed | Implement function support |
| 431 | Implement function resolution | spike | frontend/resolver | class: triage-needed | Implement function resolution |
| 432 | Implement import/export module syntax | spike | frontend/syntax | class: triage-needed | Implement import/export module syntax |
| 433 | Implement legacy-global-builtin support | spike | frontend/syntax | class: triage-needed | Implement legacy-global-builtin support |
| 434 | Implement loop constructs | spike | frontend/syntax | class: triage-needed | Implement loop constructs |
| 435 | Implement method call support | spike | frontend/syntax | class: triage-needed | Implement method call support |
| 436 | Implement module-resolution support | spike | frontend/syntax | class: triage-needed | Implement module-resolution support |
| 437 | Implement name resolution | spike | frontend/resolver | class: triage-needed | Implement name resolution |
| 438 | Implement negative-parse-syntaxerror support | spike | reference/triage | class: triage-needed | Implement negative-parse-syntaxerror support |
| 439 | Implement new expression | spike | frontend/syntax | class: triage-needed | Implement new expression |
| 440 | Implement object-builtin support | spike | frontend/syntax | class: triage-needed | Implement object-builtin support |
| 441 | Implement object literal enhancements | spike | frontend/syntax | class: triage-needed | Implement object literal enhancements |
| 442 | Implement parser syntax extensions | spike | frontend/syntax | class: triage-needed | Implement parser syntax extensions |
| 443 | Implement property access support | spike | frontend/syntax | class: triage-needed | Implement property access support |
| 444 | Implement RegExp literal support | spike | runtime/builtins | class: triage-needed | Implement RegExp literal support |
| 445 | Implement runtime-subset support | spike | reference/triage | class: triage-needed | Implement runtime-subset support |
| 446 | Implement scope-analysis support | spike | frontend/syntax | class: triage-needed | Implement scope-analysis support |
| 447 | Implement spread operator | spike | frontend/syntax | class: triage-needed | Implement spread operator |
| 448 | Implement string-builtin support | spike | frontend/syntax | class: triage-needed | Implement string-builtin support |
| 449 | Implement super keyword | spike | frontend/syntax | class: triage-needed | Implement super keyword |
| 450 | Implement template literals | spike | frontend/syntax | class: triage-needed | Implement template literals |
| 451 | Implement try-catch-finally | spike | frontend/syntax | class: triage-needed | Implement try-catch-finally |
| 452 | Implement type-alias support | spike | frontend/syntax | class: triage-needed | Implement type-alias support |
| 453 | Implement type-system support | spike | frontend/syntax | class: triage-needed | Implement type-system support |
| 454 | Investigate and classify unknown-unsupported cases | spike | frontend/syntax | class: triage-needed | Investigate and classify unknown-unsupported cases |
| 455 | Implement Apilibcheck | spike | frontend/syntax | class: triage-needed | Implement Apilibcheck |
| 456 | Implement Apisample Arrow Function | spike | frontend/syntax | class: triage-needed | Implement Apisample Arrow Function |
| 457 | Implement Apisample Import Export | spike | frontend/syntax | class: triage-needed | Implement Apisample Import Export |
| 458 | Implement Apisample Jsdoc | spike | frontend/syntax | class: triage-needed | Implement Apisample Jsdoc |
| 459 | Implement Arrowfunctionexpression | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctionexpression |
| 460 | Implement Classdeclaration | spike | frontend/syntax | class: triage-needed | Implement Classdeclaration |
| 461 | Implement Classdeclarationwithinvalidconstonpropertydeclaration | spike | frontend/syntax | class: triage-needed | Implement Classdeclarationwithinvalidconstonpropertydeclaration |
| 462 | Implement Exportassignment | spike | frontend/syntax | class: triage-needed | Implement Exportassignment |
| 463 | Implement Functiondeclaration Import Export | spike | frontend/syntax | class: triage-needed | Implement Functiondeclaration Import Export |
| 464 | Implement Functiondeclaration Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Functiondeclaration Parser Syntax |
| 465 | Implement Memberaccessordeclaration | spike | frontend/syntax | class: triage-needed | Implement Memberaccessordeclaration |
| 466 | Implement Parameterlist | spike | frontend/syntax | class: triage-needed | Implement Parameterlist |
| 467 | Implement Transportstream | spike | frontend/syntax | class: triage-needed | Implement Transportstream |
| 468 | Implement Abstractclassinlocalscope | spike | frontend/syntax | class: triage-needed | Implement Abstractclassinlocalscope |
| 469 | Implement Abstractclassinlocalscopeisabstract | spike | frontend/syntax | class: triage-needed | Implement Abstractclassinlocalscopeisabstract |
| 470 | Implement Abstractclassunioninstantiation | spike | frontend/resolver | class: triage-needed | Implement Abstractclassunioninstantiation |
| 471 | Implement Abstractpropertybasics | spike | frontend/syntax | class: triage-needed | Implement Abstractpropertybasics |
| 472 | Implement Abstractpropertyinconstructor | spike | frontend/syntax | class: triage-needed | Implement Abstractpropertyinconstructor |
| 473 | Implement Abstractpropertynegative | spike | frontend/syntax | class: triage-needed | Implement Abstractpropertynegative |
| 474 | Implement Acceptsymbolasweaktype | spike | frontend/resolver | class: triage-needed | Implement Acceptsymbolasweaktype |
| 475 | Implement Acceptablealias | spike | frontend/syntax | class: triage-needed | Implement Acceptablealias |
| 476 | Implement Accessinstancememberfromstaticmethod | spike | frontend/resolver | class: triage-needed | Implement Accessinstancememberfromstaticmethod |
| 477 | Implement Accessoverriddenbaseclassmember | spike | frontend/syntax | class: triage-needed | Implement Accessoverriddenbaseclassmember |
| 478 | Implement Accessstaticmemberfrominstancemethod | spike | frontend/resolver | class: triage-needed | Implement Accessstaticmemberfrominstancemethod |
| 479 | Implement Accessoraccidentalcalldiagnostic | spike | frontend/syntax | class: triage-needed | Implement Accessoraccidentalcalldiagnostic |
| 480 | Implement Accessordeclarationemitjs | spike | frontend/syntax | class: triage-needed | Implement Accessordeclarationemitjs |
| 481 | Implement Accessordeclarationemitvisibilityerrors | spike | frontend/syntax | class: triage-needed | Implement Accessordeclarationemitvisibilityerrors |
| 482 | Implement Accessordeclarationorder | spike | frontend/syntax | class: triage-needed | Implement Accessordeclarationorder |
| 483 | Implement Accessorinambientcontextes | spike | frontend/syntax | class: triage-needed | Implement Accessorinambientcontextes |
| 484 | Implement Accessorinferredreturntypeerrorinreturnstatement | spike | frontend/syntax | class: triage-needed | Implement Accessorinferredreturntypeerrorinreturnstatement |
| 485 | Implement Accessorparameteraccessibilitymodifier | spike | frontend/syntax | class: triage-needed | Implement Accessorparameteraccessibilitymodifier |
| 486 | Implement Accessorwithlineterminator | spike | reference/triage | class: triage-needed | Implement Accessorwithlineterminator |
| 487 | Implement Accessorwithoutbody | spike | frontend/syntax | class: triage-needed | Implement Accessorwithoutbody |
| 488 | Implement Accessors | spike | frontend/syntax | class: triage-needed | Implement Accessors |
| 489 | Implement Accessorsinambientcontext | spike | frontend/syntax | class: triage-needed | Implement Accessorsinambientcontext |
| 490 | Implement Addmorecallsignaturestobasesignature | spike | frontend/resolver | class: triage-needed | Implement Addmorecallsignaturestobasesignature |
| 491 | Implement Aliasassignments | spike | frontend/syntax | class: triage-needed | Implement Aliasassignments |
| 492 | Implement Aliasbug | spike | frontend/syntax | class: triage-needed | Implement Aliasbug |
| 493 | Implement Aliasdoesnotduplicatesignatures | spike | frontend/syntax | class: triage-needed | Implement Aliasdoesnotduplicatesignatures |
| 494 | Implement Aliaserrors | spike | frontend/syntax | class: triage-needed | Implement Aliaserrors |
| 495 | Implement Aliasinaccessiblemodule | spike | frontend/syntax | class: triage-needed | Implement Aliasinaccessiblemodule |
| 496 | Implement Aliasinstantiationexpressiongenericintersectionnocrash | spike | frontend/syntax | class: triage-needed | Implement Aliasinstantiationexpressiongenericintersectionnocrash |
| 497 | Implement Aliasonmergedmoduleinterface | spike | frontend/syntax | class: triage-needed | Implement Aliasonmergedmoduleinterface |
| 498 | Implement Aliasusageinaccessorsofclass | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinaccessorsofclass |
| 499 | Implement Aliasusageinarray | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinarray |
| 500 | Implement Aliasusageinfunctionexpression | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinfunctionexpression |
| 501 | Implement Aliasusageingenericfunction | spike | frontend/syntax | class: triage-needed | Implement Aliasusageingenericfunction |
| 502 | Implement Aliasusageinindexerofclass | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinindexerofclass |
| 503 | Implement Aliasusageinobjectliteral | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinobjectliteral |
| 504 | Implement Aliasusageinorexpression | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinorexpression |
| 505 | Implement Aliasusageintypeargumentofextendsclause | spike | frontend/syntax | class: triage-needed | Implement Aliasusageintypeargumentofextendsclause |
| 506 | Implement Aliasusageinvarassignment | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinvarassignment |
| 507 | Implement Aliasusedasnamevalue | spike | frontend/syntax | class: triage-needed | Implement Aliasusedasnamevalue |
| 508 | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer | spike | frontend/syntax | class: triage-needed | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer |
| 509 | Implement Aliasesinsystemmodule | spike | frontend/syntax | class: triage-needed | Implement Aliasesinsystemmodule |
| 510 | Implement Allowimportclausestomergewithtypes | spike | frontend/syntax | class: triage-needed | Implement Allowimportclausestomergewithtypes |
| 511 | Implement Allowjsclassthistypecrash | spike | reference/triage | class: triage-needed | Implement Allowjsclassthistypecrash |
| 512 | Implement Allowjscrossmonorepopackage | spike | frontend/syntax | class: triage-needed | Implement Allowjscrossmonorepopackage |
| 513 | Implement Allowjscheckjstypeparameternocrash | spike | frontend/syntax | class: triage-needed | Implement Allowjscheckjstypeparameternocrash |
| 514 | Implement Allowsyntheticdefaultimports | spike | frontend/syntax | class: triage-needed | Implement Allowsyntheticdefaultimports |
| 515 | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration | spike | frontend/syntax | class: triage-needed | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration |
| 516 | Implement Alwaysstrictmodule | spike | frontend/syntax | class: triage-needed | Implement Alwaysstrictmodule |
| 517 | Implement Alwaysstrictnoimplicitusestrict | spike | frontend/syntax | class: triage-needed | Implement Alwaysstrictnoimplicitusestrict |
| 518 | Implement Ambientclassdeclarationwithextends | spike | frontend/syntax | class: triage-needed | Implement Ambientclassdeclarationwithextends |
| 519 | Implement Ambientclassdeclaredbeforebase | spike | frontend/syntax | class: triage-needed | Implement Ambientclassdeclaredbeforebase |
| 520 | Implement Ambientconstliterals | spike | frontend/syntax | class: triage-needed | Implement Ambientconstliterals |
| 521 | Implement Ambientenumelementinitializer | spike | frontend/syntax | class: triage-needed | Implement Ambientenumelementinitializer |
| 522 | Implement Ambienterrors | spike | frontend/syntax | class: triage-needed | Implement Ambienterrors |
| 523 | Implement Ambientexportdefaulterrors | spike | frontend/syntax | class: triage-needed | Implement Ambientexportdefaulterrors |
| 524 | Implement Ambientexternalmoduleinanotherexternalmodule | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmoduleinanotherexternalmodule |
| 525 | Implement Ambientexternalmodulereopen | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulereopen |
| 526 | Implement Ambientexternalmodulewithinternalimportdeclaration | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulewithinternalimportdeclaration |
| 527 | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration |
| 528 | Implement Ambientexternalmodulewithrelativemodulename | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulewithrelativemodulename |
| 529 | Implement Ambientexternalmodulewithoutinternalimportdeclaration | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulewithoutinternalimportdeclaration |
| 530 | Implement Ambientfundule | spike | frontend/syntax | class: triage-needed | Implement Ambientfundule |
| 531 | Implement Ambientmoduleexports | spike | frontend/syntax | class: triage-needed | Implement Ambientmoduleexports |
| 532 | Implement Ambientmodulewithclassdeclarationwithextends | spike | frontend/syntax | class: triage-needed | Implement Ambientmodulewithclassdeclarationwithextends |
| 533 | Implement Ambientmodulewithtemplateliterals | spike | frontend/syntax | class: triage-needed | Implement Ambientmodulewithtemplateliterals |
| 534 | Implement Ambientmodules | spike | frontend/syntax | class: triage-needed | Implement Ambientmodules |
| 535 | Implement Ambientnamerestrictions | spike | frontend/syntax | class: triage-needed | Implement Ambientnamerestrictions |
| 536 | Implement Ambientrequirefunction | spike | frontend/syntax | class: triage-needed | Implement Ambientrequirefunction |
| 537 | Implement Ambientstatement | spike | frontend/syntax | class: triage-needed | Implement Ambientstatement |
| 538 | Implement Ambientwithstatements | spike | frontend/syntax | class: triage-needed | Implement Ambientwithstatements |
| 539 | Implement Ambiguouscallswherereturntypesagree | spike | frontend/syntax | class: triage-needed | Implement Ambiguouscallswherereturntypesagree |
| 540 | Implement Ambiguousgenericassertion | spike | frontend/syntax | class: triage-needed | Implement Ambiguousgenericassertion |
| 541 | Implement Apilibcheck | spike | frontend/syntax | class: triage-needed | Implement Apilibcheck |
| 542 | Implement Apisample Arrow Function | spike | frontend/syntax | class: triage-needed | Implement Apisample Arrow Function |
| 543 | Implement Apisample Import Export | spike | frontend/syntax | class: triage-needed | Implement Apisample Import Export |
| 544 | Implement Apisample Jsdoc | spike | frontend/syntax | class: triage-needed | Implement Apisample Jsdoc |
| 545 | Implement Arrowfunctionexpression | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctionexpression |
| 546 | Implement Classdeclaration | spike | frontend/syntax | class: triage-needed | Implement Classdeclaration |
| 547 | Implement Classdeclarationwithinvalidconstonpropertydeclaration | spike | frontend/syntax | class: triage-needed | Implement Classdeclarationwithinvalidconstonpropertydeclaration |
| 548 | Implement Exportassignment | spike | frontend/syntax | class: triage-needed | Implement Exportassignment |
| 549 | Implement Functiondeclaration Import Export | spike | frontend/syntax | class: triage-needed | Implement Functiondeclaration Import Export |
| 550 | Implement Functiondeclaration Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Functiondeclaration Parser Syntax |
| 551 | Implement Memberaccessordeclaration | spike | frontend/syntax | class: triage-needed | Implement Memberaccessordeclaration |
| 552 | Implement Parameterlist | spike | frontend/syntax | class: triage-needed | Implement Parameterlist |
| 553 | Implement Transportstream | spike | frontend/syntax | class: triage-needed | Implement Transportstream |
| 554 | Implement Abstractclassinlocalscope | spike | frontend/syntax | class: triage-needed | Implement Abstractclassinlocalscope |
| 555 | Implement Abstractclassinlocalscopeisabstract | spike | frontend/syntax | class: triage-needed | Implement Abstractclassinlocalscopeisabstract |
| 556 | Implement Abstractclassunioninstantiation | spike | frontend/resolver | class: triage-needed | Implement Abstractclassunioninstantiation |
| 557 | Implement Abstractpropertybasics | spike | frontend/syntax | class: triage-needed | Implement Abstractpropertybasics |
| 558 | Implement Abstractpropertyinconstructor | spike | frontend/syntax | class: triage-needed | Implement Abstractpropertyinconstructor |
| 559 | Implement Abstractpropertynegative | spike | frontend/syntax | class: triage-needed | Implement Abstractpropertynegative |
| 560 | Implement Acceptsymbolasweaktype | spike | frontend/resolver | class: triage-needed | Implement Acceptsymbolasweaktype |
| 561 | Implement Acceptablealias | spike | frontend/syntax | class: triage-needed | Implement Acceptablealias |
| 562 | Implement Accessinstancememberfromstaticmethod | spike | frontend/resolver | class: triage-needed | Implement Accessinstancememberfromstaticmethod |
| 563 | Implement Accessoverriddenbaseclassmember | spike | frontend/syntax | class: triage-needed | Implement Accessoverriddenbaseclassmember |
| 564 | Implement Accessstaticmemberfrominstancemethod | spike | frontend/resolver | class: triage-needed | Implement Accessstaticmemberfrominstancemethod |
| 565 | Implement Accessoraccidentalcalldiagnostic | spike | frontend/syntax | class: triage-needed | Implement Accessoraccidentalcalldiagnostic |
| 566 | Implement Accessordeclarationemitjs | spike | frontend/syntax | class: triage-needed | Implement Accessordeclarationemitjs |
| 567 | Implement Accessordeclarationemitvisibilityerrors | spike | frontend/syntax | class: triage-needed | Implement Accessordeclarationemitvisibilityerrors |
| 568 | Implement Accessordeclarationorder | spike | frontend/syntax | class: triage-needed | Implement Accessordeclarationorder |
| 569 | Implement Accessorinambientcontextes | spike | frontend/syntax | class: triage-needed | Implement Accessorinambientcontextes |
| 570 | Implement Accessorinferredreturntypeerrorinreturnstatement | spike | frontend/syntax | class: triage-needed | Implement Accessorinferredreturntypeerrorinreturnstatement |
| 571 | Implement Accessorparameteraccessibilitymodifier | spike | frontend/syntax | class: triage-needed | Implement Accessorparameteraccessibilitymodifier |
| 572 | Implement Accessorwithlineterminator | spike | reference/triage | class: triage-needed | Implement Accessorwithlineterminator |
| 573 | Implement Accessorwithoutbody | spike | frontend/syntax | class: triage-needed | Implement Accessorwithoutbody |
| 574 | Implement Accessors | spike | frontend/syntax | class: triage-needed | Implement Accessors |
| 575 | Implement Accessorsinambientcontext | spike | frontend/syntax | class: triage-needed | Implement Accessorsinambientcontext |
| 576 | Implement Addmorecallsignaturestobasesignature | spike | frontend/syntax | class: triage-needed | Implement Addmorecallsignaturestobasesignature |
| 577 | Implement Aliasassignments | spike | frontend/syntax | class: triage-needed | Implement Aliasassignments |
| 578 | Implement Aliasbug | spike | frontend/syntax | class: triage-needed | Implement Aliasbug |
| 579 | Implement Aliasdoesnotduplicatesignatures | spike | frontend/syntax | class: triage-needed | Implement Aliasdoesnotduplicatesignatures |
| 580 | Implement Aliaserrors | spike | frontend/syntax | class: triage-needed | Implement Aliaserrors |
| 581 | Implement Aliasinaccessiblemodule | spike | frontend/syntax | class: triage-needed | Implement Aliasinaccessiblemodule |
| 582 | Implement Aliasinstantiationexpressiongenericintersectionnocrash | spike | frontend/syntax | class: triage-needed | Implement Aliasinstantiationexpressiongenericintersectionnocrash |
| 583 | Implement Aliasonmergedmoduleinterface | spike | frontend/syntax | class: triage-needed | Implement Aliasonmergedmoduleinterface |
| 584 | Implement Aliasusageinaccessorsofclass | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinaccessorsofclass |
| 585 | Implement Aliasusageinarray | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinarray |
| 586 | Implement Aliasusageinfunctionexpression | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinfunctionexpression |
| 587 | Implement Aliasusageingenericfunction | spike | frontend/syntax | class: triage-needed | Implement Aliasusageingenericfunction |
| 588 | Implement Aliasusageinindexerofclass | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinindexerofclass |
| 589 | Implement Aliasusageinobjectliteral | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinobjectliteral |
| 590 | Implement Aliasusageinorexpression | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinorexpression |
| 591 | Implement Aliasusageintypeargumentofextendsclause | spike | frontend/syntax | class: triage-needed | Implement Aliasusageintypeargumentofextendsclause |
| 592 | Implement Aliasusageinvarassignment | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinvarassignment |
| 593 | Implement Aliasusedasnamevalue | spike | frontend/syntax | class: triage-needed | Implement Aliasusedasnamevalue |
| 594 | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer | spike | frontend/syntax | class: triage-needed | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer |
| 595 | Implement Aliasesinsystemmodule | spike | frontend/syntax | class: triage-needed | Implement Aliasesinsystemmodule |
| 596 | Implement Allowimportclausestomergewithtypes | spike | frontend/syntax | class: triage-needed | Implement Allowimportclausestomergewithtypes |
| 597 | Implement Allowjsclassthistypecrash | spike | reference/triage | class: triage-needed | Implement Allowjsclassthistypecrash |
| 598 | Implement Allowjscrossmonorepopackage | spike | frontend/syntax | class: triage-needed | Implement Allowjscrossmonorepopackage |
| 599 | Implement Allowjscheckjstypeparameternocrash | spike | frontend/syntax | class: triage-needed | Implement Allowjscheckjstypeparameternocrash |
| 600 | Implement Allowsyntheticdefaultimports | spike | frontend/syntax | class: triage-needed | Implement Allowsyntheticdefaultimports |
| 601 | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration | spike | frontend/syntax | class: triage-needed | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration |
| 602 | Implement Alwaysstrictmodule | spike | frontend/syntax | class: triage-needed | Implement Alwaysstrictmodule |
| 603 | Implement Alwaysstrictnoimplicitusestrict | spike | frontend/syntax | class: triage-needed | Implement Alwaysstrictnoimplicitusestrict |
| 604 | Implement Ambientclassdeclarationwithextends | spike | frontend/syntax | class: triage-needed | Implement Ambientclassdeclarationwithextends |
| 605 | Implement Ambientclassdeclaredbeforebase | spike | frontend/syntax | class: triage-needed | Implement Ambientclassdeclaredbeforebase |
| 606 | Implement Ambientconstliterals | spike | frontend/syntax | class: triage-needed | Implement Ambientconstliterals |
| 607 | Implement Ambientenumelementinitializer | spike | frontend/syntax | class: triage-needed | Implement Ambientenumelementinitializer |
| 608 | Implement Ambienterrors | spike | frontend/syntax | class: triage-needed | Implement Ambienterrors |
| 609 | Implement Ambientexportdefaulterrors | spike | frontend/syntax | class: triage-needed | Implement Ambientexportdefaulterrors |
| 610 | Implement Ambientexternalmoduleinanotherexternalmodule | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmoduleinanotherexternalmodule |
| 611 | Implement Ambientexternalmodulereopen | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulereopen |
| 612 | Implement Ambientexternalmodulewithinternalimportdeclaration | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulewithinternalimportdeclaration |
| 613 | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration |
| 614 | Implement Ambientexternalmodulewithrelativemodulename | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulewithrelativemodulename |
| 615 | Implement Ambientexternalmodulewithoutinternalimportdeclaration | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulewithoutinternalimportdeclaration |
| 616 | Implement Ambientfundule | spike | frontend/syntax | class: triage-needed | Implement Ambientfundule |
| 617 | Implement Ambientmoduleexports | spike | frontend/syntax | class: triage-needed | Implement Ambientmoduleexports |
| 618 | Implement Ambientmodulewithclassdeclarationwithextends | spike | frontend/syntax | class: triage-needed | Implement Ambientmodulewithclassdeclarationwithextends |
| 619 | Implement Ambientmodulewithtemplateliterals | spike | frontend/syntax | class: triage-needed | Implement Ambientmodulewithtemplateliterals |
| 620 | Implement Ambientmodules | spike | frontend/syntax | class: triage-needed | Implement Ambientmodules |
| 621 | Implement Ambientnamerestrictions | spike | frontend/syntax | class: triage-needed | Implement Ambientnamerestrictions |
| 622 | Implement Ambientrequirefunction | spike | frontend/syntax | class: triage-needed | Implement Ambientrequirefunction |
| 623 | Implement Ambientstatement | spike | frontend/syntax | class: triage-needed | Implement Ambientstatement |
| 624 | Implement Ambientwithstatements | spike | frontend/syntax | class: triage-needed | Implement Ambientwithstatements |
| 625 | Implement Ambiguouscallswherereturntypesagree | spike | frontend/syntax | class: triage-needed | Implement Ambiguouscallswherereturntypesagree |
| 626 | Implement Ambiguousgenericassertion | spike | frontend/syntax | class: triage-needed | Implement Ambiguousgenericassertion |
| 627 | Implement Ambiguousoverloadresolution | spike | frontend/resolver | class: triage-needed | Implement Ambiguousoverloadresolution |
| 628 | Implement Amddeclarationemitnoextradeclare | spike | frontend/syntax | class: triage-needed | Implement Amddeclarationemitnoextradeclare |
| 629 | Implement Amddependencycomment | spike | frontend/syntax | class: triage-needed | Implement Amddependencycomment |
| 630 | Implement Amddependencycommentname | spike | frontend/syntax | class: triage-needed | Implement Amddependencycommentname |
| 631 | Implement Amdlikeinputdeclarationemit | spike | frontend/syntax | class: triage-needed | Implement Amdlikeinputdeclarationemit |
| 632 | Implement Amdmodulebundlenoduplicatedeclarationemitcomments | spike | frontend/syntax | class: triage-needed | Implement Amdmodulebundlenoduplicatedeclarationemitcomments |
| 633 | Implement Amdmoduleconstenumusage | spike | frontend/syntax | class: triage-needed | Implement Amdmoduleconstenumusage |
| 634 | Implement Amdmodulename | spike | frontend/syntax | class: triage-needed | Implement Amdmodulename |
| 635 | Implement Anonclassdeclarationemitisanon | spike | frontend/syntax | class: triage-needed | Implement Anonclassdeclarationemitisanon |
| 636 | Implement Anonterface | spike | frontend/syntax | class: triage-needed | Implement Anonterface |
| 637 | Implement Anonymousclassdeclarationdoesntprintwithreadonly | spike | frontend/syntax | class: triage-needed | Implement Anonymousclassdeclarationdoesntprintwithreadonly |
| 638 | Implement Anonymousclassexpression | spike | frontend/syntax | class: triage-needed | Implement Anonymousclassexpression |
| 639 | Implement Anonymousmodules | spike | frontend/syntax | class: triage-needed | Implement Anonymousmodules |
| 640 | Implement Anyandunknownhavefalsycomponents | spike | frontend/resolver | class: triage-needed | Implement Anyandunknownhavefalsycomponents |
| 641 | Implement Anyasreturntypefornewoncall | spike | frontend/syntax | class: triage-needed | Implement Anyasreturntypefornewoncall |
| 642 | Implement Anydeclare | spike | frontend/syntax | class: triage-needed | Implement Anydeclare |
| 643 | Implement Anyidenticaltoitself | spike | frontend/syntax | class: triage-needed | Implement Anyidenticaltoitself |
| 644 | Implement Anyinferenceanonymousfunctions | spike | frontend/syntax | class: triage-needed | Implement Anyinferenceanonymousfunctions |
| 645 | Implement Argsinscope | spike | frontend/syntax | class: triage-needed | Implement Argsinscope |
| 646 | Implement Arguments | spike | frontend/syntax | class: triage-needed | Implement Arguments |
| 647 | Implement Argumentsaspropertyname Arguments Object | spike | frontend/syntax | class: triage-needed | Implement Argumentsaspropertyname Arguments Object |
| 648 | Implement Argumentsaspropertyname Name Resolution | spike | frontend/resolver | class: triage-needed | Implement Argumentsaspropertyname Name Resolution |
| 649 | Implement Argumentsbindstofunctionscopeargumentlist | spike | frontend/resolver | class: triage-needed | Implement Argumentsbindstofunctionscopeargumentlist |
| 650 | Implement Argumentsobjectcreatesrestforjs | spike | frontend/syntax | class: triage-needed | Implement Argumentsobjectcreatesrestforjs |
| 651 | Implement Argumentsobjectiterator | spike | frontend/syntax | class: triage-needed | Implement Argumentsobjectiterator |
| 652 | Implement Argumentspropertynameinjsmode | spike | frontend/syntax | class: triage-needed | Implement Argumentspropertynameinjsmode |
| 653 | Implement Argumentsreferenceinconstructor Arguments Object | spike | frontend/syntax | class: triage-needed | Implement Argumentsreferenceinconstructor Arguments Object |
| 654 | Implement Argumentsreferenceinconstructor Name Resolution | spike | frontend/resolver | class: triage-needed | Implement Argumentsreferenceinconstructor Name Resolution |
| 655 | Implement Argumentsreferenceinfunction | spike | frontend/syntax | class: triage-needed | Implement Argumentsreferenceinfunction |
| 656 | Implement Argumentsreferenceinmethod Arguments Object | spike | frontend/syntax | class: triage-needed | Implement Argumentsreferenceinmethod Arguments Object |
| 657 | Implement Argumentsreferenceinmethod Name Resolution | spike | frontend/resolver | class: triage-needed | Implement Argumentsreferenceinmethod Name Resolution |
| 658 | Implement Argumentsreferenceinobjectliteral | spike | frontend/syntax | class: triage-needed | Implement Argumentsreferenceinobjectliteral |
| 659 | Implement Argumentsusedinclassfieldinitializerorstaticinitializationblock | spike | frontend/syntax | class: triage-needed | Implement Argumentsusedinclassfieldinitializerorstaticinitializationblock |
| 660 | Implement Argumentsusedinobjectliteralproperty | spike | frontend/syntax | class: triage-needed | Implement Argumentsusedinobjectliteralproperty |
| 661 | Implement Arithassigntyping | spike | frontend/syntax | class: triage-needed | Implement Arithassigntyping |
| 662 | Implement Arrayassignmenttest Import Export | spike | frontend/syntax | class: triage-needed | Implement Arrayassignmenttest Import Export |
| 663 | Implement Arrayassignmenttest Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Arrayassignmenttest Parser Syntax |
| 664 | Implement Arrayaugment | spike | reference/triage | class: triage-needed | Implement Arrayaugment |
| 665 | Implement Arraybestcommontypes | spike | frontend/syntax | class: triage-needed | Implement Arraybestcommontypes |
| 666 | Implement Arraybindingpatternomittedexpressions | spike | frontend/syntax | class: triage-needed | Implement Arraybindingpatternomittedexpressions |
| 667 | Implement Arraybufferisviewnarrowstype | spike | frontend/resolver | class: triage-needed | Implement Arraybufferisviewnarrowstype |
| 668 | Implement Arraycast | spike | frontend/syntax | class: triage-needed | Implement Arraycast |
| 669 | Implement Arrayconcat | spike | frontend/syntax | class: triage-needed | Implement Arrayconcat |
| 670 | Implement Arrayconcatmap | spike | frontend/syntax | class: triage-needed | Implement Arrayconcatmap |
| 671 | Implement Arrayconstructors | spike | frontend/syntax | class: triage-needed | Implement Arrayconstructors |
| 672 | Implement Arraydestructuringinswitch | spike | frontend/syntax | class: triage-needed | Implement Arraydestructuringinswitch |
| 673 | Implement Arrayevery | spike | frontend/syntax | class: triage-needed | Implement Arrayevery |
| 674 | Implement Arrayfakeflatnocrashinferencedeclarations | spike | frontend/syntax | class: triage-needed | Implement Arrayfakeflatnocrashinferencedeclarations |
| 675 | Implement Arrayfilter | spike | frontend/syntax | class: triage-needed | Implement Arrayfilter |
| 676 | Implement Arrayfind | spike | frontend/syntax | class: triage-needed | Implement Arrayfind |
| 677 | Implement Arrayflatmap | spike | frontend/syntax | class: triage-needed | Implement Arrayflatmap |
| 678 | Implement Arrayflatnocrashinference | spike | frontend/syntax | class: triage-needed | Implement Arrayflatnocrashinference |
| 679 | Implement Arrayflatnocrashinferencedeclarations | spike | frontend/syntax | class: triage-needed | Implement Arrayflatnocrashinferencedeclarations |
| 680 | Implement Arrayfrom | spike | frontend/syntax | class: triage-needed | Implement Arrayfrom |
| 681 | Implement Arrayfromasync | spike | reference/triage | class: triage-needed | Implement Arrayfromasync |
| 682 | Implement Arrayindexwitharrayfails | spike | frontend/resolver | class: triage-needed | Implement Arrayindexwitharrayfails |
| 683 | Implement Arrayiterationlibes | spike | frontend/resolver | class: triage-needed | Implement Arrayiterationlibes |
| 684 | Implement Arrayliteralandarrayconstructorequivalence | spike | frontend/resolver | class: triage-needed | Implement Arrayliteralandarrayconstructorequivalence |
| 685 | Implement Arrayliteralcomments | spike | frontend/syntax | class: triage-needed | Implement Arrayliteralcomments |
| 686 | Implement Arrayliteralcontextualtype | spike | frontend/syntax | class: triage-needed | Implement Arrayliteralcontextualtype |
| 687 | Implement Arrayliteraltypeinference | spike | frontend/syntax | class: triage-needed | Implement Arrayliteraltypeinference |
| 688 | Implement Arrayofexportedclass | spike | frontend/syntax | class: triage-needed | Implement Arrayofexportedclass |
| 689 | Implement Arrayofsubtypeisassignabletoreadonlyarray | spike | frontend/syntax | class: triage-needed | Implement Arrayofsubtypeisassignabletoreadonlyarray |
| 690 | Implement Arrayreferencewithouttypeargs | spike | frontend/syntax | class: triage-needed | Implement Arrayreferencewithouttypeargs |
| 691 | Implement Arraysigchecking | spike | frontend/syntax | class: triage-needed | Implement Arraysigchecking |
| 692 | Implement Arrayslice | spike | frontend/syntax | class: triage-needed | Implement Arrayslice |
| 693 | Implement Arraytolocalestringes Name Resolution | spike | frontend/resolver | class: triage-needed | Implement Arraytolocalestringes Name Resolution |
| 694 | Implement Arraytolocalestringes Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Arraytolocalestringes Unknown Unsupported |
| 695 | Implement Arraytypeinsignatureofinterfaceandclass | spike | frontend/syntax | class: triage-needed | Implement Arraytypeinsignatureofinterfaceandclass |
| 696 | Implement Arrayconcat | spike | frontend/syntax | class: triage-needed | Implement Arrayconcat |
| 697 | Implement Arrowfunctioninconstructorargument | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctioninconstructorargument |
| 698 | Implement Arrowfunctioninexpressionstatement | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctioninexpressionstatement |
| 699 | Implement Arrowfunctionmissingcurlywithsemicolon | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctionmissingcurlywithsemicolon |
| 700 | Implement Arrowfunctionparsingdoesnotconfuseparenthesizedobjectforarrowhead | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctionparsingdoesnotconfuseparenthesizedobjectforarrowhead |
| 701 | Implement Arrowfunctionparsinggenericinobject | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctionparsinggenericinobject |
| 702 | Implement Arrowfunctionwithobjectliteralbody | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctionwithobjectliteralbody |
| 703 | Implement Arrowfunctionsmissingtokens | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctionsmissingtokens |
| 704 | Implement Asiabstract | spike | frontend/syntax | class: triage-needed | Implement Asiabstract |
| 705 | Implement Asiambientfunctiondeclaration | spike | frontend/syntax | class: triage-needed | Implement Asiambientfunctiondeclaration |
| 706 | Implement Asiarith | spike | frontend/syntax | class: triage-needed | Implement Asiarith |
| 707 | Implement Asibreak | spike | frontend/syntax | class: triage-needed | Implement Asibreak |
| 708 | Implement Asicontinue | spike | frontend/syntax | class: triage-needed | Implement Asicontinue |
| 709 | Implement Asiines | spike | frontend/syntax | class: triage-needed | Implement Asiines |
| 710 | Implement Asipublicprivateprotected | spike | frontend/syntax | class: triage-needed | Implement Asipublicprivateprotected |
| 711 | Implement Asireturn | spike | reference/triage | class: triage-needed | Implement Asireturn |
| 712 | Implement Assertinwrapsometypeparameter | spike | frontend/syntax | class: triage-needed | Implement Assertinwrapsometypeparameter |
| 713 | Implement Assertionfunctionwildcardimport | spike | frontend/syntax | class: triage-needed | Implement Assertionfunctionwildcardimport |
| 714 | Implement Assertionfunctionscannarrowbydiscriminant | spike | frontend/syntax | class: triage-needed | Implement Assertionfunctionscannarrowbydiscriminant |
| 715 | Implement Assign | spike | frontend/syntax | class: triage-needed | Implement Assign |
| 716 | Implement Assigntoenum | spike | frontend/syntax | class: triage-needed | Implement Assigntoenum |
| 717 | Implement Assigntoexistingclass | spike | frontend/syntax | class: triage-needed | Implement Assigntoexistingclass |
| 718 | Implement Assigntofn | spike | frontend/syntax | class: triage-needed | Implement Assigntofn |
| 719 | Implement Assigntoinvalidlhs | spike | frontend/syntax | class: triage-needed | Implement Assigntoinvalidlhs |
| 720 | Implement Assigntomodule | spike | frontend/syntax | class: triage-needed | Implement Assigntomodule |
| 721 | Implement Assigntoobjecttypewithprototypeproperty | spike | frontend/resolver | class: triage-needed | Implement Assigntoobjecttypewithprototypeproperty |
| 722 | Implement Assigntoprototype | spike | frontend/resolver | class: triage-needed | Implement Assigntoprototype |
| 723 | Implement Assigningfromobjecttoanythingelse | spike | frontend/resolver | class: triage-needed | Implement Assigningfromobjecttoanythingelse |
| 724 | Implement Assigningfunctiontotupleissueserror | spike | frontend/resolver | class: triage-needed | Implement Assigningfunctiontotupleissueserror |
| 725 | Implement Assignmentcompat | spike | frontend/resolver | class: triage-needed | Implement Assignmentcompat |
| 726 | Implement Assignmentcompatbug | spike | frontend/syntax | class: triage-needed | Implement Assignmentcompatbug |
| 727 | Implement Assignmentcompatforenums | spike | frontend/syntax | class: triage-needed | Implement Assignmentcompatforenums |
| 728 | Implement Assignmentcompatfunctionswithoptionalargs | spike | frontend/syntax | class: triage-needed | Implement Assignmentcompatfunctionswithoptionalargs |
| 729 | Implement Assignmentcompatinterfacewithstringindexsignature | spike | frontend/syntax | class: triage-needed | Implement Assignmentcompatinterfacewithstringindexsignature |
| 730 | Implement Assignmentcompatonnew | spike | frontend/resolver | class: triage-needed | Implement Assignmentcompatonnew |
| 731 | Implement Assignmentcompatwithoverloads | spike | frontend/syntax | class: triage-needed | Implement Assignmentcompatwithoverloads |
| 732 | Implement Assignmentcompatability Import Export | spike | frontend/syntax | class: triage-needed | Implement Assignmentcompatability Import Export |
| 733 | Implement Assignmentcompatability Name Resolution | spike | frontend/resolver | class: triage-needed | Implement Assignmentcompatability Name Resolution |
| 734 | Implement Assignmentcompatability Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Assignmentcompatability Parser Syntax |
| 735 | Implement Assignmentindexedtoprimitives | spike | frontend/syntax | class: triage-needed | Implement Assignmentindexedtoprimitives |
| 736 | Implement Assignmentnestedinliterals | spike | reference/triage | class: triage-needed | Implement Assignmentnestedinliterals |
| 737 | Implement Assignmentnonobjecttypeconstraints | spike | frontend/syntax | class: triage-needed | Implement Assignmentnonobjecttypeconstraints |
| 738 | Implement Assignmentrestelementwitherrorsourcetype | spike | frontend/resolver | class: triage-needed | Implement Assignmentrestelementwitherrorsourcetype |
| 739 | Implement Assignmentstricterconstraints | spike | frontend/syntax | class: triage-needed | Implement Assignmentstricterconstraints |
| 740 | Implement Assignmenttoanyarrayrestparameters | spike | frontend/syntax | class: triage-needed | Implement Assignmenttoanyarrayrestparameters |
| 741 | Implement Assignmenttoconditionalbrandedstringtemplateormapping | spike | frontend/syntax | class: triage-needed | Implement Assignmenttoconditionalbrandedstringtemplateormapping |
| 742 | Implement Assignmenttoexpandingarraytype | spike | frontend/syntax | class: triage-needed | Implement Assignmenttoexpandingarraytype |
| 743 | Implement Assignmenttofunction | spike | frontend/syntax | class: triage-needed | Implement Assignmenttofunction |
| 744 | Implement Assignmenttoinstantiationexpression | spike | frontend/syntax | class: triage-needed | Implement Assignmenttoinstantiationexpression |
| 745 | Implement Assignmenttoobjectandfunction | spike | frontend/syntax | class: triage-needed | Implement Assignmenttoobjectandfunction |
| 746 | Implement Assignmenttoparenthesizedexpression | spike | frontend/syntax | class: triage-needed | Implement Assignmenttoparenthesizedexpression |
| 747 | Implement Assignmenttoreferencetypes | spike | frontend/syntax | class: triage-needed | Implement Assignmenttoreferencetypes |
| 748 | Implement Asyncarrowinclasses | spike | frontend/syntax | class: triage-needed | Implement Asyncarrowinclasses |
| 749 | Implement Asyncawaitwithcapturedblockscopevar | spike | reference/triage | class: triage-needed | Implement Asyncawaitwithcapturedblockscopevar |
| 750 | Implement Asyncfunctioncontextuallytypedreturns | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctioncontextuallytypedreturns |
| 751 | Implement Asyncfunctionnoreturntype | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctionnoreturntype |
| 752 | Implement Asyncfunctionreturnexpressionerrorspans | spike | reference/triage | class: triage-needed | Implement Asyncfunctionreturnexpressionerrorspans |
| 753 | Implement Asyncfunctionreturntype Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctionreturntype Parser Syntax |
| 754 | Implement Asyncfunctionreturntype Runtime Subset | spike | reference/triage | class: triage-needed | Implement Asyncfunctionreturntype Runtime Subset |
| 755 | Implement Asyncfunctiontempvariablescoping | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctiontempvariablescoping |
| 756 | Implement Asyncfunctionwithforstatementnoinitializer | spike | reference/triage | class: triage-needed | Implement Asyncfunctionwithforstatementnoinitializer |
| 757 | Implement Asyncfunctionsacrossfiles | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctionsacrossfiles |
| 758 | Implement Asyncfunctionsandstrictnullchecks | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctionsandstrictnullchecks |
| 759 | Implement Asynciife | spike | frontend/syntax | class: triage-needed | Implement Asynciife |
| 760 | Implement Asyncimportnestedyield | spike | reference/triage | class: triage-needed | Implement Asyncimportnestedyield |
| 761 | Implement Asynciteratorextraparameters | spike | frontend/syntax | class: triage-needed | Implement Asynciteratorextraparameters |
| 762 | Implement Asyncyieldstarcontextualtype | spike | frontend/syntax | class: triage-needed | Implement Asyncyieldstarcontextualtype |
| 763 | Implement Augmentexportequals | spike | frontend/syntax | class: triage-needed | Implement Augmentexportequals |
| 764 | Implement Augmentedclasswithprototypepropertyonmodule | spike | frontend/syntax | class: triage-needed | Implement Augmentedclasswithprototypepropertyonmodule |
| 765 | Implement Augmentedtypesclass | spike | frontend/syntax | class: triage-needed | Implement Augmentedtypesclass |
| 766 | Implement Augmentedtypesenum Import Export | spike | frontend/syntax | class: triage-needed | Implement Augmentedtypesenum Import Export |
| 767 | Implement Augmentedtypesenum Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Augmentedtypesenum Parser Syntax |
| 768 | Implement Augmentedtypesexternalmodule | spike | frontend/syntax | class: triage-needed | Implement Augmentedtypesexternalmodule |
| 769 | Implement Augmentedtypesfunction | spike | frontend/syntax | class: triage-needed | Implement Augmentedtypesfunction |
| 770 | Implement Augmentedtypesinterface | spike | frontend/syntax | class: triage-needed | Implement Augmentedtypesinterface |
| 771 | Implement Augmentedtypesmodules | spike | frontend/syntax | class: triage-needed | Implement Augmentedtypesmodules |
| 772 | Implement Augmentedtypesvar | spike | frontend/syntax | class: triage-needed | Implement Augmentedtypesvar |
| 773 | Implement Autoasiforstaticsinclassdeclaration | spike | frontend/syntax | class: triage-needed | Implement Autoasiforstaticsinclassdeclaration |
| 774 | Implement Autolift | spike | frontend/syntax | class: triage-needed | Implement Autolift |
| 775 | Implement Autotypeassignedusingdestructuringfromnevernocrash | spike | frontend/resolver | class: triage-needed | Implement Autotypeassignedusingdestructuringfromnevernocrash |
| 776 | Implement Apilibcheck | spike | frontend/syntax | class: triage-needed | Implement Apilibcheck |
| 777 | Implement Apisample Arrow Function | spike | frontend/syntax | class: triage-needed | Implement Apisample Arrow Function |
| 778 | Implement Apisample Import Export | spike | frontend/syntax | class: triage-needed | Implement Apisample Import Export |
| 779 | Implement Apisample Jsdoc | spike | frontend/syntax | class: triage-needed | Implement Apisample Jsdoc |
| 780 | Implement Arrowfunctionexpression | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctionexpression |
| 781 | Implement Classdeclaration | spike | frontend/syntax | class: triage-needed | Implement Classdeclaration |
| 782 | Implement Classdeclarationwithinvalidconstonpropertydeclaration | spike | frontend/syntax | class: triage-needed | Implement Classdeclarationwithinvalidconstonpropertydeclaration |
| 783 | Implement Exportassignment | spike | frontend/syntax | class: triage-needed | Implement Exportassignment |
| 784 | Implement Functiondeclaration Import Export | spike | frontend/syntax | class: triage-needed | Implement Functiondeclaration Import Export |
| 785 | Implement Functiondeclaration Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Functiondeclaration Parser Syntax |
| 786 | Implement Memberaccessordeclaration | spike | frontend/syntax | class: triage-needed | Implement Memberaccessordeclaration |
| 787 | Implement Parameterlist | spike | frontend/syntax | class: triage-needed | Implement Parameterlist |
| 788 | Implement Transportstream | spike | frontend/syntax | class: triage-needed | Implement Transportstream |
| 789 | Implement Abstractclassinlocalscope | spike | frontend/syntax | class: triage-needed | Implement Abstractclassinlocalscope |
| 790 | Implement Abstractclassinlocalscopeisabstract | spike | frontend/syntax | class: triage-needed | Implement Abstractclassinlocalscopeisabstract |
| 791 | Implement Abstractclassunioninstantiation | spike | frontend/resolver | class: triage-needed | Implement Abstractclassunioninstantiation |
| 792 | Implement Abstractpropertybasics | spike | frontend/syntax | class: triage-needed | Implement Abstractpropertybasics |
| 793 | Implement Abstractpropertyinconstructor | spike | frontend/syntax | class: triage-needed | Implement Abstractpropertyinconstructor |
| 794 | Implement Abstractpropertynegative | spike | frontend/syntax | class: triage-needed | Implement Abstractpropertynegative |
| 795 | Implement Acceptsymbolasweaktype | spike | frontend/resolver | class: triage-needed | Implement Acceptsymbolasweaktype |
| 796 | Implement Acceptablealias | spike | frontend/syntax | class: triage-needed | Implement Acceptablealias |
| 797 | Implement Accessinstancememberfromstaticmethod | spike | frontend/resolver | class: triage-needed | Implement Accessinstancememberfromstaticmethod |
| 798 | Implement Accessoverriddenbaseclassmember | spike | frontend/syntax | class: triage-needed | Implement Accessoverriddenbaseclassmember |
| 799 | Implement Accessstaticmemberfrominstancemethod | spike | frontend/resolver | class: triage-needed | Implement Accessstaticmemberfrominstancemethod |
| 800 | Implement Accessoraccidentalcalldiagnostic | spike | frontend/syntax | class: triage-needed | Implement Accessoraccidentalcalldiagnostic |
| 801 | Implement Accessordeclarationemitjs | spike | frontend/syntax | class: triage-needed | Implement Accessordeclarationemitjs |
| 802 | Implement Accessordeclarationemitvisibilityerrors | spike | frontend/syntax | class: triage-needed | Implement Accessordeclarationemitvisibilityerrors |
| 803 | Implement Accessordeclarationorder | spike | frontend/syntax | class: triage-needed | Implement Accessordeclarationorder |
| 804 | Implement Accessorinambientcontextes | spike | frontend/syntax | class: triage-needed | Implement Accessorinambientcontextes |
| 805 | Implement Accessorinferredreturntypeerrorinreturnstatement | spike | frontend/syntax | class: triage-needed | Implement Accessorinferredreturntypeerrorinreturnstatement |
| 806 | Implement Accessorparameteraccessibilitymodifier | spike | frontend/syntax | class: triage-needed | Implement Accessorparameteraccessibilitymodifier |
| 807 | Implement Accessorwithlineterminator | spike | reference/triage | class: triage-needed | Implement Accessorwithlineterminator |
| 808 | Implement Accessorwithoutbody | spike | frontend/syntax | class: triage-needed | Implement Accessorwithoutbody |
| 809 | Implement Accessors | spike | frontend/syntax | class: triage-needed | Implement Accessors |
| 810 | Implement Accessorsinambientcontext | spike | frontend/syntax | class: triage-needed | Implement Accessorsinambientcontext |
| 811 | Implement Addmorecallsignaturestobasesignature | spike | frontend/syntax | class: triage-needed | Implement Addmorecallsignaturestobasesignature |
| 812 | Implement Aliasassignments | spike | frontend/syntax | class: triage-needed | Implement Aliasassignments |
| 813 | Implement Aliasbug | spike | frontend/syntax | class: triage-needed | Implement Aliasbug |
| 814 | Implement Aliasdoesnotduplicatesignatures | spike | frontend/syntax | class: triage-needed | Implement Aliasdoesnotduplicatesignatures |
| 815 | Implement Aliaserrors | spike | frontend/syntax | class: triage-needed | Implement Aliaserrors |
| 816 | Implement Aliasinaccessiblemodule | spike | frontend/syntax | class: triage-needed | Implement Aliasinaccessiblemodule |
| 817 | Implement Aliasinstantiationexpressiongenericintersectionnocrash | spike | frontend/syntax | class: triage-needed | Implement Aliasinstantiationexpressiongenericintersectionnocrash |
| 818 | Implement Aliasonmergedmoduleinterface | spike | frontend/syntax | class: triage-needed | Implement Aliasonmergedmoduleinterface |
| 819 | Implement Aliasusageinaccessorsofclass | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinaccessorsofclass |
| 820 | Implement Aliasusageinarray | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinarray |
| 821 | Implement Aliasusageinfunctionexpression | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinfunctionexpression |
| 822 | Implement Aliasusageingenericfunction | spike | frontend/syntax | class: triage-needed | Implement Aliasusageingenericfunction |
| 823 | Implement Aliasusageinindexerofclass | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinindexerofclass |
| 824 | Implement Aliasusageinobjectliteral | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinobjectliteral |
| 825 | Implement Aliasusageinorexpression | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinorexpression |
| 826 | Implement Aliasusageintypeargumentofextendsclause | spike | frontend/syntax | class: triage-needed | Implement Aliasusageintypeargumentofextendsclause |
| 827 | Implement Aliasusageinvarassignment | spike | frontend/syntax | class: triage-needed | Implement Aliasusageinvarassignment |
| 828 | Implement Aliasusedasnamevalue | spike | frontend/syntax | class: triage-needed | Implement Aliasusedasnamevalue |
| 829 | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer | spike | frontend/syntax | class: triage-needed | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer |
| 830 | Implement Aliasesinsystemmodule | spike | frontend/syntax | class: triage-needed | Implement Aliasesinsystemmodule |
| 831 | Implement Allowimportclausestomergewithtypes | spike | frontend/syntax | class: triage-needed | Implement Allowimportclausestomergewithtypes |
| 832 | Implement Allowjsclassthistypecrash | spike | reference/triage | class: triage-needed | Implement Allowjsclassthistypecrash |
| 833 | Implement Allowjscrossmonorepopackage | spike | frontend/syntax | class: triage-needed | Implement Allowjscrossmonorepopackage |
| 834 | Implement Allowjscheckjstypeparameternocrash | spike | frontend/syntax | class: triage-needed | Implement Allowjscheckjstypeparameternocrash |
| 835 | Implement Allowsyntheticdefaultimports | spike | frontend/syntax | class: triage-needed | Implement Allowsyntheticdefaultimports |
| 836 | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration | spike | frontend/syntax | class: triage-needed | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration |
| 837 | Implement Alwaysstrictmodule | spike | frontend/syntax | class: triage-needed | Implement Alwaysstrictmodule |
| 838 | Implement Alwaysstrictnoimplicitusestrict | spike | frontend/syntax | class: triage-needed | Implement Alwaysstrictnoimplicitusestrict |
| 839 | Implement Ambientclassdeclarationwithextends | spike | frontend/syntax | class: triage-needed | Implement Ambientclassdeclarationwithextends |
| 840 | Implement Ambientclassdeclaredbeforebase | spike | frontend/syntax | class: triage-needed | Implement Ambientclassdeclaredbeforebase |
| 841 | Implement Ambientconstliterals | spike | frontend/syntax | class: triage-needed | Implement Ambientconstliterals |
| 842 | Implement Ambientenumelementinitializer | spike | frontend/syntax | class: triage-needed | Implement Ambientenumelementinitializer |
| 843 | Implement Ambienterrors | spike | frontend/syntax | class: triage-needed | Implement Ambienterrors |
| 844 | Implement Ambientexportdefaulterrors | spike | frontend/syntax | class: triage-needed | Implement Ambientexportdefaulterrors |
| 845 | Implement Ambientexternalmoduleinanotherexternalmodule | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmoduleinanotherexternalmodule |
| 846 | Implement Ambientexternalmodulereopen | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulereopen |
| 847 | Implement Ambientexternalmodulewithinternalimportdeclaration | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulewithinternalimportdeclaration |
| 848 | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration |
| 849 | Implement Ambientexternalmodulewithrelativemodulename | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulewithrelativemodulename |
| 850 | Implement Ambientexternalmodulewithoutinternalimportdeclaration | spike | frontend/syntax | class: triage-needed | Implement Ambientexternalmodulewithoutinternalimportdeclaration |
| 851 | Implement Ambientfundule | spike | frontend/syntax | class: triage-needed | Implement Ambientfundule |
| 852 | Implement Ambientmoduleexports | spike | frontend/syntax | class: triage-needed | Implement Ambientmoduleexports |
| 853 | Implement Ambientmodulewithclassdeclarationwithextends | spike | frontend/syntax | class: triage-needed | Implement Ambientmodulewithclassdeclarationwithextends |
| 854 | Implement Ambientmodulewithtemplateliterals | spike | frontend/syntax | class: triage-needed | Implement Ambientmodulewithtemplateliterals |
| 855 | Implement Ambientmodules | spike | frontend/syntax | class: triage-needed | Implement Ambientmodules |
| 856 | Implement Ambientnamerestrictions | spike | frontend/syntax | class: triage-needed | Implement Ambientnamerestrictions |
| 857 | Implement Ambientrequirefunction | spike | frontend/syntax | class: triage-needed | Implement Ambientrequirefunction |
| 858 | Implement Ambientstatement | spike | frontend/syntax | class: triage-needed | Implement Ambientstatement |
| 859 | Implement Ambientwithstatements | spike | frontend/syntax | class: triage-needed | Implement Ambientwithstatements |
| 860 | Implement Ambiguouscallswherereturntypesagree | spike | frontend/syntax | class: triage-needed | Implement Ambiguouscallswherereturntypesagree |
| 861 | Implement Ambiguousgenericassertion | spike | frontend/syntax | class: triage-needed | Implement Ambiguousgenericassertion |
| 862 | Implement Ambiguousoverloadresolution | spike | frontend/resolver | class: triage-needed | Implement Ambiguousoverloadresolution |
| 863 | Implement Amddeclarationemitnoextradeclare | spike | frontend/syntax | class: triage-needed | Implement Amddeclarationemitnoextradeclare |
| 864 | Implement Amddependencycomment | spike | frontend/syntax | class: triage-needed | Implement Amddependencycomment |
| 865 | Implement Amddependencycommentname | spike | frontend/syntax | class: triage-needed | Implement Amddependencycommentname |
| 866 | Implement Amdlikeinputdeclarationemit | spike | frontend/syntax | class: triage-needed | Implement Amdlikeinputdeclarationemit |
| 867 | Implement Amdmodulebundlenoduplicatedeclarationemitcomments | spike | frontend/syntax | class: triage-needed | Implement Amdmodulebundlenoduplicatedeclarationemitcomments |
| 868 | Implement Amdmoduleconstenumusage | spike | frontend/syntax | class: triage-needed | Implement Amdmoduleconstenumusage |
| 869 | Implement Amdmodulename | spike | frontend/syntax | class: triage-needed | Implement Amdmodulename |
| 870 | Implement Anonclassdeclarationemitisanon | spike | frontend/syntax | class: triage-needed | Implement Anonclassdeclarationemitisanon |
| 871 | Implement Anonterface | spike | frontend/syntax | class: triage-needed | Implement Anonterface |
| 872 | Implement Anonymousclassdeclarationdoesntprintwithreadonly | spike | frontend/syntax | class: triage-needed | Implement Anonymousclassdeclarationdoesntprintwithreadonly |
| 873 | Implement Anonymousclassexpression | spike | frontend/syntax | class: triage-needed | Implement Anonymousclassexpression |
| 874 | Implement Anonymousmodules | spike | frontend/syntax | class: triage-needed | Implement Anonymousmodules |
| 875 | Implement Anyandunknownhavefalsycomponents | spike | frontend/resolver | class: triage-needed | Implement Anyandunknownhavefalsycomponents |
| 876 | Implement Anyasreturntypefornewoncall | spike | frontend/syntax | class: triage-needed | Implement Anyasreturntypefornewoncall |
| 877 | Implement Anydeclare | spike | frontend/syntax | class: triage-needed | Implement Anydeclare |
| 878 | Implement Anyidenticaltoitself | spike | frontend/syntax | class: triage-needed | Implement Anyidenticaltoitself |
| 879 | Implement Anyinferenceanonymousfunctions | spike | frontend/syntax | class: triage-needed | Implement Anyinferenceanonymousfunctions |
| 880 | Implement Argsinscope | spike | frontend/syntax | class: triage-needed | Implement Argsinscope |
| 881 | Implement Arguments | spike | frontend/syntax | class: triage-needed | Implement Arguments |
| 882 | Implement Argumentsaspropertyname Arguments Object | spike | frontend/syntax | class: triage-needed | Implement Argumentsaspropertyname Arguments Object |
| 883 | Implement Argumentsaspropertyname Name Resolution | spike | frontend/resolver | class: triage-needed | Implement Argumentsaspropertyname Name Resolution |
| 884 | Implement Argumentsbindstofunctionscopeargumentlist | spike | frontend/resolver | class: triage-needed | Implement Argumentsbindstofunctionscopeargumentlist |
| 885 | Implement Argumentsobjectcreatesrestforjs | spike | frontend/syntax | class: triage-needed | Implement Argumentsobjectcreatesrestforjs |
| 886 | Implement Argumentsobjectiterator | spike | frontend/syntax | class: triage-needed | Implement Argumentsobjectiterator |
| 887 | Implement Argumentspropertynameinjsmode | spike | frontend/syntax | class: triage-needed | Implement Argumentspropertynameinjsmode |
| 888 | Implement Argumentsreferenceinconstructor Arguments Object | spike | frontend/syntax | class: triage-needed | Implement Argumentsreferenceinconstructor Arguments Object |
| 889 | Implement Argumentsreferenceinconstructor Name Resolution | spike | frontend/resolver | class: triage-needed | Implement Argumentsreferenceinconstructor Name Resolution |
| 890 | Implement Argumentsreferenceinfunction | spike | frontend/syntax | class: triage-needed | Implement Argumentsreferenceinfunction |
| 891 | Implement Argumentsreferenceinmethod Arguments Object | spike | frontend/syntax | class: triage-needed | Implement Argumentsreferenceinmethod Arguments Object |
| 892 | Implement Argumentsreferenceinmethod Name Resolution | spike | frontend/resolver | class: triage-needed | Implement Argumentsreferenceinmethod Name Resolution |
| 893 | Implement Argumentsreferenceinobjectliteral | spike | frontend/syntax | class: triage-needed | Implement Argumentsreferenceinobjectliteral |
| 894 | Implement Argumentsusedinclassfieldinitializerorstaticinitializationblock | spike | frontend/syntax | class: triage-needed | Implement Argumentsusedinclassfieldinitializerorstaticinitializationblock |
| 895 | Implement Argumentsusedinobjectliteralproperty | spike | frontend/syntax | class: triage-needed | Implement Argumentsusedinobjectliteralproperty |
| 896 | Implement Arithassigntyping | spike | frontend/syntax | class: triage-needed | Implement Arithassigntyping |
| 897 | Implement Arrayassignmenttest Import Export | spike | frontend/syntax | class: triage-needed | Implement Arrayassignmenttest Import Export |
| 898 | Implement Arrayassignmenttest Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Arrayassignmenttest Parser Syntax |
| 899 | Implement Arrayaugment | spike | reference/triage | class: triage-needed | Implement Arrayaugment |
| 900 | Implement Arraybestcommontypes | spike | frontend/syntax | class: triage-needed | Implement Arraybestcommontypes |
| 901 | Implement Arraybindingpatternomittedexpressions | spike | frontend/syntax | class: triage-needed | Implement Arraybindingpatternomittedexpressions |
| 902 | Implement Arraybufferisviewnarrowstype | spike | frontend/resolver | class: triage-needed | Implement Arraybufferisviewnarrowstype |
| 903 | Implement Arraycast | spike | frontend/syntax | class: triage-needed | Implement Arraycast |
| 904 | Implement Arrayconcat | spike | frontend/syntax | class: triage-needed | Implement Arrayconcat |
| 905 | Implement Arrayconcatmap | spike | frontend/syntax | class: triage-needed | Implement Arrayconcatmap |
| 906 | Implement Arrayconstructors | spike | frontend/syntax | class: triage-needed | Implement Arrayconstructors |
| 907 | Implement Arraydestructuringinswitch | spike | frontend/syntax | class: triage-needed | Implement Arraydestructuringinswitch |
| 908 | Implement Arrayevery | spike | frontend/syntax | class: triage-needed | Implement Arrayevery |
| 909 | Implement Arrayfakeflatnocrashinferencedeclarations | spike | frontend/syntax | class: triage-needed | Implement Arrayfakeflatnocrashinferencedeclarations |
| 910 | Implement Arrayfilter | spike | frontend/syntax | class: triage-needed | Implement Arrayfilter |
| 911 | Implement Arrayfind | spike | frontend/syntax | class: triage-needed | Implement Arrayfind |
| 912 | Implement Arrayflatmap | spike | frontend/syntax | class: triage-needed | Implement Arrayflatmap |
| 913 | Implement Arrayflatnocrashinference | spike | frontend/syntax | class: triage-needed | Implement Arrayflatnocrashinference |
| 914 | Implement Arrayflatnocrashinferencedeclarations | spike | frontend/syntax | class: triage-needed | Implement Arrayflatnocrashinferencedeclarations |
| 915 | Implement Arrayfrom | spike | frontend/syntax | class: triage-needed | Implement Arrayfrom |
| 916 | Implement Arrayfromasync | spike | reference/triage | class: triage-needed | Implement Arrayfromasync |
| 917 | Implement Arrayindexwitharrayfails | spike | frontend/resolver | class: triage-needed | Implement Arrayindexwitharrayfails |
| 918 | Implement Arrayiterationlibes | spike | frontend/resolver | class: triage-needed | Implement Arrayiterationlibes |
| 919 | Implement Arrayliteralandarrayconstructorequivalence | spike | frontend/resolver | class: triage-needed | Implement Arrayliteralandarrayconstructorequivalence |
| 920 | Implement Arrayliteralcomments | spike | frontend/syntax | class: triage-needed | Implement Arrayliteralcomments |
| 921 | Implement Arrayliteralcontextualtype | spike | frontend/syntax | class: triage-needed | Implement Arrayliteralcontextualtype |
| 922 | Implement Arrayliteraltypeinference | spike | frontend/syntax | class: triage-needed | Implement Arrayliteraltypeinference |
| 923 | Implement Arrayofexportedclass | spike | frontend/syntax | class: triage-needed | Implement Arrayofexportedclass |
| 924 | Implement Arrayofsubtypeisassignabletoreadonlyarray | spike | frontend/syntax | class: triage-needed | Implement Arrayofsubtypeisassignabletoreadonlyarray |
| 925 | Implement Arrayreferencewithouttypeargs | spike | frontend/syntax | class: triage-needed | Implement Arrayreferencewithouttypeargs |
| 926 | Implement Arraysigchecking | spike | frontend/syntax | class: triage-needed | Implement Arraysigchecking |
| 927 | Implement Arrayslice | spike | frontend/syntax | class: triage-needed | Implement Arrayslice |
| 928 | Implement Arraytolocalestringes Name Resolution | spike | frontend/resolver | class: triage-needed | Implement Arraytolocalestringes Name Resolution |
| 929 | Implement Arraytolocalestringes Unknown Unsupported | spike | frontend/syntax | class: triage-needed | Implement Arraytolocalestringes Unknown Unsupported |
| 930 | Implement Arraytypeinsignatureofinterfaceandclass | spike | frontend/syntax | class: triage-needed | Implement Arraytypeinsignatureofinterfaceandclass |
| 931 | Implement Arrayconcat | spike | frontend/syntax | class: triage-needed | Implement Arrayconcat |
| 932 | Implement Arrowfunctioninconstructorargument | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctioninconstructorargument |
| 933 | Implement Arrowfunctioninexpressionstatement | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctioninexpressionstatement |
| 934 | Implement Arrowfunctionmissingcurlywithsemicolon | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctionmissingcurlywithsemicolon |
| 935 | Implement Arrowfunctionparsingdoesnotconfuseparenthesizedobjectforarrowhead | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctionparsingdoesnotconfuseparenthesizedobjectforarrowhead |
| 936 | Implement Arrowfunctionparsinggenericinobject | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctionparsinggenericinobject |
| 937 | Implement Arrowfunctionwithobjectliteralbody | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctionwithobjectliteralbody |
| 938 | Implement Arrowfunctionsmissingtokens | spike | frontend/syntax | class: triage-needed | Implement Arrowfunctionsmissingtokens |
| 939 | Implement Asiabstract | spike | frontend/syntax | class: triage-needed | Implement Asiabstract |
| 940 | Implement Asiambientfunctiondeclaration | spike | frontend/syntax | class: triage-needed | Implement Asiambientfunctiondeclaration |
| 941 | Implement Asiarith | spike | frontend/syntax | class: triage-needed | Implement Asiarith |
| 942 | Implement Asibreak | spike | frontend/syntax | class: triage-needed | Implement Asibreak |
| 943 | Implement Asicontinue | spike | frontend/syntax | class: triage-needed | Implement Asicontinue |
| 944 | Implement Asiines | spike | frontend/syntax | class: triage-needed | Implement Asiines |
| 945 | Implement Asipublicprivateprotected | spike | frontend/syntax | class: triage-needed | Implement Asipublicprivateprotected |
| 946 | Implement Asireturn | spike | reference/triage | class: triage-needed | Implement Asireturn |
| 947 | Implement Assertinwrapsometypeparameter | spike | frontend/syntax | class: triage-needed | Implement Assertinwrapsometypeparameter |
| 948 | Implement Assertionfunctionwildcardimport | spike | frontend/syntax | class: triage-needed | Implement Assertionfunctionwildcardimport |
| 949 | Implement Assertionfunctionscannarrowbydiscriminant | spike | frontend/syntax | class: triage-needed | Implement Assertionfunctionscannarrowbydiscriminant |
| 950 | Implement Assign | spike | frontend/syntax | class: triage-needed | Implement Assign |
| 951 | Implement Assigntoenum | spike | frontend/syntax | class: triage-needed | Implement Assigntoenum |
| 952 | Implement Assigntoexistingclass | spike | frontend/syntax | class: triage-needed | Implement Assigntoexistingclass |
| 953 | Implement Assigntofn | spike | frontend/syntax | class: triage-needed | Implement Assigntofn |
| 954 | Implement Assigntoinvalidlhs | spike | frontend/syntax | class: triage-needed | Implement Assigntoinvalidlhs |
| 955 | Implement Assigntomodule | spike | frontend/syntax | class: triage-needed | Implement Assigntomodule |
| 956 | Implement Assigntoobjecttypewithprototypeproperty | spike | frontend/resolver | class: triage-needed | Implement Assigntoobjecttypewithprototypeproperty |
| 957 | Implement Assigntoprototype | spike | frontend/resolver | class: triage-needed | Implement Assigntoprototype |
| 958 | Implement Assigningfromobjecttoanythingelse | spike | frontend/resolver | class: triage-needed | Implement Assigningfromobjecttoanythingelse |
| 959 | Implement Assigningfunctiontotupleissueserror | spike | frontend/resolver | class: triage-needed | Implement Assigningfunctiontotupleissueserror |
| 960 | Implement Assignmentcompat | spike | frontend/resolver | class: triage-needed | Implement Assignmentcompat |
| 961 | Implement Assignmentcompatbug | spike | frontend/syntax | class: triage-needed | Implement Assignmentcompatbug |
| 962 | Implement Assignmentcompatforenums | spike | frontend/syntax | class: triage-needed | Implement Assignmentcompatforenums |
| 963 | Implement Assignmentcompatfunctionswithoptionalargs | spike | frontend/syntax | class: triage-needed | Implement Assignmentcompatfunctionswithoptionalargs |
| 964 | Implement Assignmentcompatinterfacewithstringindexsignature | spike | frontend/syntax | class: triage-needed | Implement Assignmentcompatinterfacewithstringindexsignature |
| 965 | Implement Assignmentcompatonnew | spike | frontend/resolver | class: triage-needed | Implement Assignmentcompatonnew |
| 966 | Implement Assignmentcompatwithoverloads | spike | frontend/syntax | class: triage-needed | Implement Assignmentcompatwithoverloads |
| 967 | Implement Assignmentcompatability Import Export | spike | frontend/syntax | class: triage-needed | Implement Assignmentcompatability Import Export |
| 968 | Implement Assignmentcompatability Name Resolution | spike | frontend/resolver | class: triage-needed | Implement Assignmentcompatability Name Resolution |
| 969 | Implement Assignmentcompatability Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Assignmentcompatability Parser Syntax |
| 970 | Implement Assignmentindexedtoprimitives | spike | frontend/syntax | class: triage-needed | Implement Assignmentindexedtoprimitives |
| 971 | Implement Assignmentnestedinliterals | spike | reference/triage | class: triage-needed | Implement Assignmentnestedinliterals |
| 972 | Implement Assignmentnonobjecttypeconstraints | spike | frontend/syntax | class: triage-needed | Implement Assignmentnonobjecttypeconstraints |
| 973 | Implement Assignmentrestelementwitherrorsourcetype | spike | frontend/resolver | class: triage-needed | Implement Assignmentrestelementwitherrorsourcetype |
| 974 | Implement Assignmentstricterconstraints | spike | frontend/syntax | class: triage-needed | Implement Assignmentstricterconstraints |
| 975 | Implement Assignmenttoanyarrayrestparameters | spike | frontend/syntax | class: triage-needed | Implement Assignmenttoanyarrayrestparameters |
| 976 | Implement Assignmenttoconditionalbrandedstringtemplateormapping | spike | frontend/syntax | class: triage-needed | Implement Assignmenttoconditionalbrandedstringtemplateormapping |
| 977 | Implement Assignmenttoexpandingarraytype | spike | frontend/syntax | class: triage-needed | Implement Assignmenttoexpandingarraytype |
| 978 | Implement Assignmenttofunction | spike | frontend/syntax | class: triage-needed | Implement Assignmenttofunction |
| 979 | Implement Assignmenttoinstantiationexpression | spike | frontend/syntax | class: triage-needed | Implement Assignmenttoinstantiationexpression |
| 980 | Implement Assignmenttoobjectandfunction | spike | frontend/syntax | class: triage-needed | Implement Assignmenttoobjectandfunction |
| 981 | Implement Assignmenttoparenthesizedexpression | spike | frontend/syntax | class: triage-needed | Implement Assignmenttoparenthesizedexpression |
| 982 | Implement Assignmenttoreferencetypes | spike | frontend/syntax | class: triage-needed | Implement Assignmenttoreferencetypes |
| 983 | Implement Asyncarrowinclasses | spike | frontend/syntax | class: triage-needed | Implement Asyncarrowinclasses |
| 984 | Implement Asyncawaitwithcapturedblockscopevar | spike | reference/triage | class: triage-needed | Implement Asyncawaitwithcapturedblockscopevar |
| 985 | Implement Asyncfunctioncontextuallytypedreturns | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctioncontextuallytypedreturns |
| 986 | Implement Asyncfunctionnoreturntype | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctionnoreturntype |
| 987 | Implement Asyncfunctionreturnexpressionerrorspans | spike | reference/triage | class: triage-needed | Implement Asyncfunctionreturnexpressionerrorspans |
| 988 | Implement Asyncfunctionreturntype Parser Syntax | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctionreturntype Parser Syntax |
| 989 | Implement Asyncfunctionreturntype Runtime Subset | spike | reference/triage | class: triage-needed | Implement Asyncfunctionreturntype Runtime Subset |
| 990 | Implement Asyncfunctiontempvariablescoping | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctiontempvariablescoping |
| 991 | Implement Asyncfunctionwithforstatementnoinitializer | spike | reference/triage | class: triage-needed | Implement Asyncfunctionwithforstatementnoinitializer |
| 992 | Implement Asyncfunctionsacrossfiles | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctionsacrossfiles |
| 993 | Implement Asyncfunctionsandstrictnullchecks | spike | frontend/syntax | class: triage-needed | Implement Asyncfunctionsandstrictnullchecks |
| 994 | Implement Asynciife | spike | frontend/syntax | class: triage-needed | Implement Asynciife |
| 995 | Implement Asyncimportnestedyield | spike | reference/triage | class: triage-needed | Implement Asyncimportnestedyield |
| 996 | Implement Asynciteratorextraparameters | spike | frontend/syntax | class: triage-needed | Implement Asynciteratorextraparameters |
| 997 | Implement Asyncyieldstarcontextualtype | spike | frontend/syntax | class: triage-needed | Implement Asyncyieldstarcontextualtype |
| 998 | Implement Augmentexportequals | spike | frontend/syntax | class: triage-needed | Implement Augmentexportequals |
| 999 | Implement Augmentedclasswithprototypepropertyonmodule | spike | frontend/syntax | class: triage-needed | Implement Augmentedclasswithprototypepropertyonmodule |
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
| 334 | Array.prototype.map completion: sparse array, thisArg, and generic call | meta | runtime/builtins | see `issues/done/334-complete-array-map-sparse-thisarg-test262.md` |
| 337 | Implement test262 features directive and $262 object | feature | cli/reference | see `issues/done/337-implement-test262-features-directive.md` |
| 338 | Sparse array holes handling for Array.prototype.map | feature | runtime/builtins | see `issues/done/338-array-map-sparse-array-holes.md` |
| 339 | Callback thisArg for Array.prototype.map | feature | runtime/builtins | see `issues/done/339-array-map-thisarg.md` |
| 340 | Generic call for Array.prototype.map (static dense receiver slice) | feature | runtime/builtins | see `issues/done/340-array-map-generic-call.md` |
| 341a | Implement isNaN, parseInt, parseFloat, isFinite global functions | feature | runtime/builtins | see `issues/done/341a-global-number-functions.md` |
| 341b | Implement Number constructor and static methods | feature | runtime/builtins | see `issues/done/341b-number-constructor.md` |
| 341c | Implement Boolean global | feature | runtime/builtins | see `issues/done/341c-boolean-global.md` |
| 341d | Implement globalThis binding | feature | runtime/builtins | see `issues/done/341d-globalthis-binding.md` |
| 341e | Implement encodeURI, decodeURI, escape, unescape | feature | runtime/builtins | see file |
| 346 | Implement TypeScript declaration emit coverage for tsgo suite (16 cases) | feature | frontend/syntax | see `issues/done/346-implement-tsgo-declaration-emit.md` |
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
| 408 | Implement tsgo declaration emit: AsConstSatisfies/const generic method cases | feature | frontend/syntax | see file |
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
