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
| 021a | Implement wasm-encoder hello binary MVP | feature | backend | implementation-ready | P2 |  | The wasm backend needs a first direct binary emission slice for a WASI stdout fixture; WAT and wasm binary are equiva... |
| 050a | Document Date deterministic subset and live-time policy gap | docs | runtime/builtins | docs-ready | P1 |  | Date has validated deterministic slices, but the open epic still presents live time, timezone formatting, frontend re... |
| 052a | Close JSON supported subset contract | docs | runtime/builtins | docs-ready | P1 |  | JSON has many validated progress slices, but the parent issue still reads as a full-spec implementation target and is... |
| 059a | Implement TypeScript satisfies and const assertion erasure | feature | frontend | implementation-ready | P1 |  | Parser syntax work needs the next small erasable TypeScript syntax slice instead of another broad parser epic selection. |
| 060a | Close unknown-unsupported fixed-window spike | spike | frontend | verification-ready | P1 |  | Unknown-unsupported classification has reached zero in large test262 windows, but the parent spike has no fixed compl... |
| 062a | Split function epic into callable child issues | cleanup | issues | docs-ready | P1 |  | Issue 062 mixes unrelated function surfaces, so implementation starts with scope design instead of code. |
| 225 | Implement eval and Annex B function declaration semantics | feature | frontend/semantics | implementation-ready | P3 |  | Direct `eval` and dynamic code evaluation are required JavaScript semantics; when wasm-only implementation is not suf... |
| 238 | Make strict warning gates pass | infra | tests | implementation-ready | P0 |  | Make strict warning gates pass |
<!-- generated:ready:end -->

## Blocked queue

<!-- generated:blocked:start -->
| ID | Title | Type | Area | Blocker | Summary |
|---:|---|---|---|---|---|
| 017b | Implement GC strategy | feature | runtime/memory | class: blocked | Implement GC strategy |
| 021 | Implement full wasm backend | feature | backend | class: blocked | Implement full wasm backend |
| 050 | Implement Date | feature | runtime/builtins | class: blocked | Implement Date |
| 052 | Implement JSON | feature | runtime/builtins | class: blocked | Implement JSON |
| 059 | Implement parser syntax extensions for TypeScript and advanced JS | feature | frontend | class: blocked | Implement parser syntax extensions for TypeScript and advanced JS |
| 060 | Investigate and classify unknown-unsupported diagnostic cases | spike | frontend | class: blocked | Investigate and classify unknown-unsupported diagnostic cases |
| 062 | Implement function support | feature | frontend | class: blocked | Implement function support |
| 063 | Implement function resolution | feature | frontend | class: blocked | Implement function resolution |
| 064 | Implement name resolution | feature | frontend | class: blocked | Implement name resolution |
| 066 | Implement RegExp literal support | feature | frontend | class: blocked | Implement RegExp literal support |
| 067 | Investigate and classify unknown-unsupported cases | feature | frontend | class: blocked | Investigate and classify unknown-unsupported cases |
| 068 | Implement unsupported expression types | feature | frontend | class: blocked | Implement unsupported expression types |
| 069 | Implement Apilibcheck | feature | frontend | class: blocked | Implement Apilibcheck |
| 070 | Implement Apisample | feature | frontend | class: blocked | Implement Apisample |
| 071 | Implement Arrowfunctionexpression | feature | frontend | class: blocked | Implement Arrowfunctionexpression |
| 072 | Implement Classdeclaration | feature | frontend | class: blocked | Implement Classdeclaration |
| 073 | Implement Classdeclarationwithinvalidconstonpropertydeclaration | feature | frontend | class: blocked | Implement Classdeclarationwithinvalidconstonpropertydeclaration |
| 074 | Implement Declarationerrorsnoemitonerror | feature | frontend | class: blocked | Implement Declarationerrorsnoemitonerror |
| 075 | Implement Exportassignment | feature | frontend | class: blocked | Implement Exportassignment |
| 076 | Implement Functiondeclaration | feature | frontend | class: blocked | Implement Functiondeclaration |
| 077 | Implement Interfacedeclaration | feature | frontend | class: blocked | Implement Interfacedeclaration |
| 078 | Implement Memberaccessordeclaration | feature | frontend | class: blocked | Implement Memberaccessordeclaration |
| 079 | Implement Parameterlist | feature | frontend | class: blocked | Implement Parameterlist |
| 080 | Implement Systemmoduleforstatementnoinitializer | feature | frontend | class: blocked | Implement Systemmoduleforstatementnoinitializer |
| 081 | Implement Transportstream | feature | frontend | class: blocked | Implement Transportstream |
| 082 | Implement Abstractclassinlocalscope | feature | frontend | class: blocked | Implement Abstractclassinlocalscope |
| 083 | Implement Abstractclassinlocalscopeisabstract | feature | frontend | class: blocked | Implement Abstractclassinlocalscopeisabstract |
| 084 | Implement Abstractclassunioninstantiation | feature | frontend | class: blocked | Implement Abstractclassunioninstantiation |
| 085 | Implement Abstractinterfaceidentifiername | feature | frontend | class: blocked | Implement Abstractinterfaceidentifiername |
| 086 | Implement Abstractpropertybasics | feature | frontend | class: blocked | Implement Abstractpropertybasics |
| 087 | Implement Abstractpropertyinconstructor | feature | frontend | class: blocked | Implement Abstractpropertyinconstructor |
| 088 | Implement Abstractpropertynegative | feature | frontend | class: blocked | Implement Abstractpropertynegative |
| 089 | Implement Acceptsymbolasweaktype | feature | frontend | class: blocked | Implement Acceptsymbolasweaktype |
| 090 | Implement Acceptablealias | feature | frontend | class: blocked | Implement Acceptablealias |
| 091 | Implement Accessinstancememberfromstaticmethod | feature | frontend | class: blocked | Implement Accessinstancememberfromstaticmethod |
| 092 | Implement Accessoverriddenbaseclassmember | feature | frontend | class: blocked | Implement Accessoverriddenbaseclassmember |
| 093 | Implement Accessstaticmemberfrominstancemethod | feature | frontend | class: blocked | Implement Accessstaticmemberfrominstancemethod |
| 094 | Implement Accessoraccidentalcalldiagnostic | feature | frontend | class: blocked | Implement Accessoraccidentalcalldiagnostic |
| 095 | Implement Accessorbodyintypecontext | feature | frontend | class: blocked | Implement Accessorbodyintypecontext |
| 096 | Implement Accessordeclarationemitjs | feature | frontend | class: blocked | Implement Accessordeclarationemitjs |
| 097 | Implement Accessordeclarationemitvisibilityerrors | feature | frontend | class: blocked | Implement Accessordeclarationemitvisibilityerrors |
| 098 | Implement Accessordeclarationorder | feature | frontend | class: blocked | Implement Accessordeclarationorder |
| 099 | Implement Accessorinambientcontextes | feature | frontend | class: blocked | Implement Accessorinambientcontextes |
| 100 | Implement Accessorinferredreturntypeerrorinreturnstatement | feature | frontend | class: blocked | Implement Accessorinferredreturntypeerrorinreturnstatement |
| 101 | Implement Accessorparameteraccessibilitymodifier | feature | frontend | class: blocked | Implement Accessorparameteraccessibilitymodifier |
| 102 | Implement Accessorwithinitializer | feature | frontend | class: blocked | Implement Accessorwithinitializer |
| 103 | Implement Accessorwithlineterminator | feature | frontend | class: blocked | Implement Accessorwithlineterminator |
| 104 | Implement Accessorwithrestparam | feature | frontend | class: blocked | Implement Accessorwithrestparam |
| 105 | Implement Accessorwithoutbody | feature | frontend | class: blocked | Implement Accessorwithoutbody |
| 106 | Implement Accessors | feature | frontend | class: blocked | Implement Accessors |
| 107 | Implement Accessorsemit | feature | frontend | class: blocked | Implement Accessorsemit |
| 108 | Implement Accessorsinambientcontext | feature | frontend | class: blocked | Implement Accessorsinambientcontext |
| 109 | Implement Addmorecallsignaturestobasesignature | feature | frontend | class: blocked | Implement Addmorecallsignaturestobasesignature |
| 110 | Implement Addmoreoverloadstobasesignature | feature | frontend | class: blocked | Implement Addmoreoverloadstobasesignature |
| 111 | Implement Aliasassignments | feature | frontend | class: blocked | Implement Aliasassignments |
| 112 | Implement Aliasbug | feature | frontend | class: blocked | Implement Aliasbug |
| 113 | Implement Aliasdoesnotduplicatesignatures | feature | frontend | class: blocked | Implement Aliasdoesnotduplicatesignatures |
| 114 | Implement Aliaserrors | feature | frontend | class: blocked | Implement Aliaserrors |
| 115 | Implement Aliasinaccessiblemodule | feature | frontend | class: blocked | Implement Aliasinaccessiblemodule |
| 116 | Implement Aliasinstantiationexpressiongenericintersectionnocrash | feature | frontend | class: blocked | Implement Aliasinstantiationexpressiongenericintersectionnocrash |
| 117 | Implement Aliasofgenericfunctionwithrestbehavedsameasunaliased | feature | frontend | class: blocked | Implement Aliasofgenericfunctionwithrestbehavedsameasunaliased |
| 118 | Implement Aliasonmergedmoduleinterface | feature | frontend | class: blocked | Implement Aliasonmergedmoduleinterface |
| 119 | Implement Aliasusageinaccessorsofclass | feature | frontend | class: blocked | Implement Aliasusageinaccessorsofclass |
| 120 | Implement Aliasusageinarray | feature | frontend | class: blocked | Implement Aliasusageinarray |
| 121 | Implement Aliasusageinfunctionexpression | feature | frontend | class: blocked | Implement Aliasusageinfunctionexpression |
| 122 | Implement Aliasusageingenericfunction | feature | frontend | class: blocked | Implement Aliasusageingenericfunction |
| 123 | Implement Aliasusageinindexerofclass | feature | frontend | class: blocked | Implement Aliasusageinindexerofclass |
| 124 | Implement Aliasusageinobjectliteral | feature | frontend | class: blocked | Implement Aliasusageinobjectliteral |
| 125 | Implement Aliasusageinorexpression | feature | frontend | class: blocked | Implement Aliasusageinorexpression |
| 126 | Implement Aliasusageintypeargumentofextendsclause | feature | frontend | class: blocked | Implement Aliasusageintypeargumentofextendsclause |
| 127 | Implement Aliasusageinvarassignment | feature | frontend | class: blocked | Implement Aliasusageinvarassignment |
| 128 | Implement Aliasusedasnamevalue | feature | frontend | class: blocked | Implement Aliasusedasnamevalue |
| 129 | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer | feature | frontend | class: blocked | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer |
| 130 | Implement Aliasesinsystemmodule | feature | frontend | class: blocked | Implement Aliasesinsystemmodule |
| 131 | Implement Allowimportclausestomergewithtypes | feature | frontend | class: blocked | Implement Allowimportclausestomergewithtypes |
| 132 | Implement Allowjsclassthistypecrash | feature | frontend | class: blocked | Implement Allowjsclassthistypecrash |
| 133 | Implement Allowjscrossmonorepopackage | feature | frontend | class: blocked | Implement Allowjscrossmonorepopackage |
| 134 | Implement Allowjscheckjstypeparameternocrash | feature | frontend | class: blocked | Implement Allowjscheckjstypeparameternocrash |
| 135 | Implement Allowsyntheticdefaultimports | feature | frontend | class: blocked | Implement Allowsyntheticdefaultimports |
| 136 | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration | feature | frontend | class: blocked | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration |
| 137 | Implement Alwaysstrictalreadyusestrict | feature | frontend | class: blocked | Implement Alwaysstrictalreadyusestrict |
| 138 | Implement Alwaysstrictmodule | feature | frontend | class: blocked | Implement Alwaysstrictmodule |
| 139 | Implement Alwaysstrictnoimplicitusestrict | feature | frontend | class: blocked | Implement Alwaysstrictnoimplicitusestrict |
| 140 | Implement Ambientclassdeclarationwithextends | feature | frontend | class: blocked | Implement Ambientclassdeclarationwithextends |
| 141 | Implement Ambientclassdeclaredbeforebase | feature | frontend | class: blocked | Implement Ambientclassdeclaredbeforebase |
| 142 | Implement Ambientclassmergesoverloadswithinterface | feature | frontend | class: blocked | Implement Ambientclassmergesoverloadswithinterface |
| 143 | Implement Ambientclassoverloadforfunction | feature | frontend | class: blocked | Implement Ambientclassoverloadforfunction |
| 144 | Implement Ambientconstliterals | feature | frontend | class: blocked | Implement Ambientconstliterals |
| 145 | Implement Ambientenum | feature | frontend | class: blocked | Implement Ambientenum |
| 146 | Implement Ambientenumelementinitializer | feature | frontend | class: blocked | Implement Ambientenumelementinitializer |
| 147 | Implement Ambienterrors | feature | frontend | class: blocked | Implement Ambienterrors |
| 148 | Implement Ambientexportdefaulterrors | feature | frontend | class: blocked | Implement Ambientexportdefaulterrors |
| 149 | Implement Ambientexternalmoduleinanotherexternalmodule | feature | frontend | class: blocked | Implement Ambientexternalmoduleinanotherexternalmodule |
| 150 | Implement Ambientexternalmodulereopen | feature | frontend | class: blocked | Implement Ambientexternalmodulereopen |
| 151 | Implement Ambientexternalmodulewithinternalimportdeclaration | feature | frontend | class: blocked | Implement Ambientexternalmodulewithinternalimportdeclaration |
| 152 | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration | feature | frontend | class: blocked | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration |
| 153 | Implement Ambientexternalmodulewithrelativemodulename | feature | frontend | class: blocked | Implement Ambientexternalmodulewithrelativemodulename |
| 154 | Implement Ambientexternalmodulewithoutinternalimportdeclaration | feature | frontend | class: blocked | Implement Ambientexternalmodulewithoutinternalimportdeclaration |
| 155 | Implement Ambientfundule | feature | frontend | class: blocked | Implement Ambientfundule |
| 156 | Implement Ambientgetters | feature | frontend | class: blocked | Implement Ambientgetters |
| 157 | Implement Ambientmoduleexports | feature | frontend | class: blocked | Implement Ambientmoduleexports |
| 158 | Implement Ambientmodulewithclassdeclarationwithextends | feature | frontend | class: blocked | Implement Ambientmodulewithclassdeclarationwithextends |
| 159 | Implement Ambientmodulewithtemplateliterals | feature | frontend | class: blocked | Implement Ambientmodulewithtemplateliterals |
| 160 | Implement Ambientmodules | feature | frontend | class: blocked | Implement Ambientmodules |
| 161 | Implement Ambientnamerestrictions | feature | frontend | class: blocked | Implement Ambientnamerestrictions |
| 162 | Implement Ambientpropertydeclarationinjs | feature | frontend | class: blocked | Implement Ambientpropertydeclarationinjs |
| 163 | Implement Ambientrequirefunction | feature | frontend | class: blocked | Implement Ambientrequirefunction |
| 164 | Implement Ambientstatement | feature | frontend | class: blocked | Implement Ambientstatement |
| 165 | Implement Ambientwithstatements | feature | frontend | class: blocked | Implement Ambientwithstatements |
| 166 | Implement Ambiguouscallswherereturntypesagree | feature | frontend | class: blocked | Implement Ambiguouscallswherereturntypesagree |
| 167 | Implement Ambiguousgenericassertion | feature | frontend | class: blocked | Implement Ambiguousgenericassertion |
| 168 | Implement Ambiguousoverload | feature | frontend | class: blocked | Implement Ambiguousoverload |
| 169 | Implement Ambiguousoverloadresolution | feature | frontend | class: blocked | Implement Ambiguousoverloadresolution |
| 170 | Implement Amddeclarationemitnoextradeclare | feature | frontend | class: blocked | Implement Amddeclarationemitnoextradeclare |
| 171 | Implement Amddependencycomment | feature | frontend | class: blocked | Implement Amddependencycomment |
| 172 | Implement Amddependencycommentname | feature | frontend | class: blocked | Implement Amddependencycommentname |
| 173 | Implement Amdlikeinputdeclarationemit | feature | frontend | class: blocked | Implement Amdlikeinputdeclarationemit |
| 174 | Implement Amdmodulebundlenoduplicatedeclarationemitcomments | feature | frontend | class: blocked | Implement Amdmodulebundlenoduplicatedeclarationemitcomments |
| 175 | Implement Amdmoduleconstenumusage | feature | frontend | class: blocked | Implement Amdmoduleconstenumusage |
| 176 | Implement Amdmodulename | feature | frontend | class: blocked | Implement Amdmodulename |
| 177 | Implement Anonclassdeclarationemitisanon | feature | frontend | class: blocked | Implement Anonclassdeclarationemitisanon |
| 178 | Implement Anonterface | feature | frontend | class: blocked | Implement Anonterface |
| 179 | Implement Anonymousclassdeclarationdoesntprintwithreadonly | feature | frontend | class: blocked | Implement Anonymousclassdeclarationdoesntprintwithreadonly |
| 180 | Implement Anonymousclassexpression | feature | frontend | class: blocked | Implement Anonymousclassexpression |
| 181 | Implement Anonymousmodules | feature | frontend | class: blocked | Implement Anonymousmodules |
| 182 | Implement Anyandunknownhavefalsycomponents | feature | frontend | class: blocked | Implement Anyandunknownhavefalsycomponents |
| 183 | Implement Anyasreturntypefornewoncall | feature | frontend | class: blocked | Implement Anyasreturntypefornewoncall |
| 184 | Implement Anydeclare | feature | frontend | class: blocked | Implement Anydeclare |
| 185 | Implement Anyidenticaltoitself | feature | frontend | class: blocked | Implement Anyidenticaltoitself |
| 186 | Implement Anyindexedaccessarraynoexception | feature | frontend | class: blocked | Implement Anyindexedaccessarraynoexception |
| 187 | Implement Anyinferenceanonymousfunctions | feature | frontend | class: blocked | Implement Anyinferenceanonymousfunctions |
| 188 | Implement Anyisassignabletoobject | feature | frontend | class: blocked | Implement Anyisassignabletoobject |
| 189 | Implement Anyisassignabletovoid | feature | frontend | class: blocked | Implement Anyisassignabletovoid |
| 190 | Implement Anymappedtypeserror | feature | frontend | class: blocked | Implement Anymappedtypeserror |
| 191 | Implement Anyplusany | feature | frontend | class: blocked | Implement Anyplusany |
| 192 | Implement Argsinscope | feature | frontend | class: blocked | Implement Argsinscope |
| 193 | Implement Arguments | feature | frontend | class: blocked | Implement Arguments |
| 194 | Implement Argumentsaspropertyname | feature | frontend | class: blocked | Implement Argumentsaspropertyname |
| 195 | Implement Argumentsbindstofunctionscopeargumentlist | feature | frontend | class: blocked | Implement Argumentsbindstofunctionscopeargumentlist |
| 196 | Implement Argumentsobjectcreatesrestforjs | feature | frontend | class: blocked | Implement Argumentsobjectcreatesrestforjs |
| 197 | Implement Argumentsobjectiterator | feature | frontend | class: blocked | Implement Argumentsobjectiterator |
| 198 | Implement Argumentspropertynameinjsmode | feature | frontend | class: blocked | Implement Argumentspropertynameinjsmode |
| 199 | Implement Compiler | feature | frontend | class: blocked | Implement Compiler |
| 200 | Implement parser syntax extensions | feature | frontend | class: blocked | Implement parser syntax extensions |
| 201 | Investigate and classify unknown-unsupported cases | feature | frontend | class: blocked | Investigate and classify unknown-unsupported cases |
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
| 051 | Implement RegExp | feature | runtime/builtins | see `issues/done/051-implement-regexp.md` |
| 053 | Implement Math | feature | runtime/builtins | see `issues/done/053-implement-math.md` |
| 054 | Implement Error types | feature | runtime/builtins | see `issues/done/054-implement-error-types.md` |
| 055 | Umbrella: implement import and export | feature | frontend/semantics | see `issues/done/055-implement-import-export.md` |
| 056 | Implement name resolution for variables and identifiers | feature | frontend | see `issues/done/056-implement-name-resolution.md` |
| 057 | Implement function resolution for function calls | feature | frontend | see `issues/done/057-implement-function-resolution.md` |
| 058 | Implement equality operators (==, !=, ===, !==) | feature | runtime/semantics | see `issues/done/058-implement-equality-operators.md` |
| 061 | Implement Date object support | feature | frontend | see `issues/done/061-implement-date.md` |
| 061a | Merge Date reference issue into Date epic | cleanup | issues | see `issues/done/061a-merge-date-reference-issue-into-date-epic.md` |
| 064a | Resolve Date global builtin namespace | feature | frontend | see `issues/done/064a-resolve-date-global-builtin-namespace.md` |
| 065 | Implement parser syntax extensions | feature | frontend | see `issues/done/065-implement-parser-syntax.md` |
| 065a | Merge duplicate parser syntax issue into 059 | cleanup | issues | see `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` |
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
