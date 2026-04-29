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
| 052b | Implement JSON non-integer number representation | feature | runtime/builtins | implementation-ready | P1 |  | `JSON.parse` currently rejects JSON numbers that cannot be reduced exactly to the tagged small-int representation, ev... |
| 052c | Implement JSON UTF-16 and surrogate string handling | feature | runtime/builtins | implementation-ready | P1 |  | JSON string parsing currently supports ASCII and ASCII-valued `\uXXXX` escapes, but rejects non-ASCII code points and... |
| 052d | Implement broader JSON.stringify replacer semantics | feature | runtime/builtins | implementation-ready | P1 |  | `JSON.stringify` currently supports a narrow object-literal array replacer subset and diagnoses function replacers an... |
| 052e | Complete JSON.stringify boxed argument edge cases | feature | runtime/builtins | implementation-ready | P2 |  | `JSON.stringify` has validated narrow boxed `space` handling, but broader boxed and object-coercion edge cases are no... |
| 052f | Implement JSON.parse throw-compatible diagnostics | feature | runtime/builtins | implementation-ready | P1 |  | Invalid `JSON.parse` cases are now rejected in many paths, but iwasm usually traps with `Exception: unreachable` inst... |
| 062c | Implement ordinary function declarations and direct calls | feature | frontend/semantics | implementation-ready | P1 |  | Ordinary function declarations and direct calls are a separate callable |
| 062d | Implement function this and arguments semantics | feature | frontend/semantics | implementation-ready | P1 |  | Function receiver binding and the `arguments` object have observable |
| 062e | Implement function closures | feature | frontend/semantics | implementation-ready | P1 |  | Captured lexical environments require different resolver, lowering, |
| 062f | Implement function object metadata | feature | frontend/semantics | implementation-ready | P2 |  | Function object metadata such as `name`, `length`, and prototype |
| 225 | Implement eval and Annex B function declaration semantics | feature | frontend/semantics | implementation-ready | P3 |  | Direct `eval` and dynamic code evaluation are required JavaScript semantics; when wasm-only implementation is not suf... |
| 238 | Make strict warning gates pass | infra | tests | implementation-ready | P0 |  | Make strict warning gates pass |
| 241 | Implement Annex B Date legacy methods | feature | runtime/builtins | implementation-ready | P2 |  | issue 061 was closed as a duplicate of the Date epic, but its Annex B Date reference evidence needs a closeable child... |
| 242 | Implement Date live time with WASI realtime clock | feature | runtime/builtins | implementation-ready | P1 | 239 | Live Date entry points still emit unsupported diagnostics even after the policy decision; implementation now needs to... |
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
| 062 | Implement function support | feature | frontend | class: blocked | Implement function support |
| 064 | Implement name resolution | spike | reference | class: triage-needed | Implement name resolution |
| 066 | Implement RegExp literal support | spike | reference | class: triage-needed | Implement RegExp literal support |
| 067 | Investigate and classify unknown-unsupported cases | spike | reference | class: triage-needed | Investigate and classify unknown-unsupported cases |
| 068 | Implement unsupported expression types | spike | reference | class: triage-needed | Implement unsupported expression types |
| 069 | Implement Apilibcheck | spike | reference | class: triage-needed | Implement Apilibcheck |
| 070 | Implement Apisample | spike | reference | class: triage-needed | Implement Apisample |
| 071 | Implement Arrowfunctionexpression | spike | reference | class: triage-needed | Implement Arrowfunctionexpression |
| 072 | Implement Classdeclaration | spike | reference | class: triage-needed | Implement Classdeclaration |
| 073 | Implement Classdeclarationwithinvalidconstonpropertydeclaration | spike | reference | class: triage-needed | Implement Classdeclarationwithinvalidconstonpropertydeclaration |
| 074 | Implement Declarationerrorsnoemitonerror | spike | reference | class: triage-needed | Implement Declarationerrorsnoemitonerror |
| 075 | Implement Exportassignment | spike | reference | class: triage-needed | Implement Exportassignment |
| 076 | Implement Functiondeclaration | spike | reference | class: triage-needed | Implement Functiondeclaration |
| 078 | Implement Memberaccessordeclaration | spike | reference | class: triage-needed | Implement Memberaccessordeclaration |
| 079 | Implement Parameterlist | spike | reference | class: triage-needed | Implement Parameterlist |
| 080 | Implement Systemmoduleforstatementnoinitializer | spike | reference | class: triage-needed | Implement Systemmoduleforstatementnoinitializer |
| 081 | Implement Transportstream | spike | reference | class: triage-needed | Implement Transportstream |
| 082 | Implement Abstractclassinlocalscope | spike | reference | class: triage-needed | Implement Abstractclassinlocalscope |
| 083 | Implement Abstractclassinlocalscopeisabstract | spike | reference | class: triage-needed | Implement Abstractclassinlocalscopeisabstract |
| 084 | Implement Abstractclassunioninstantiation | spike | reference | class: triage-needed | Implement Abstractclassunioninstantiation |
| 086 | Implement Abstractpropertybasics | spike | reference | class: triage-needed | Implement Abstractpropertybasics |
| 087 | Implement Abstractpropertyinconstructor | spike | reference | class: triage-needed | Implement Abstractpropertyinconstructor |
| 088 | Implement Abstractpropertynegative | spike | reference | class: triage-needed | Implement Abstractpropertynegative |
| 089 | Implement Acceptsymbolasweaktype | spike | reference | class: triage-needed | Implement Acceptsymbolasweaktype |
| 090 | Implement Acceptablealias | spike | reference | class: triage-needed | Implement Acceptablealias |
| 091 | Implement Accessinstancememberfromstaticmethod | spike | reference | class: triage-needed | Implement Accessinstancememberfromstaticmethod |
| 092 | Implement Accessoverriddenbaseclassmember | spike | reference | class: triage-needed | Implement Accessoverriddenbaseclassmember |
| 093 | Implement Accessstaticmemberfrominstancemethod | spike | reference | class: triage-needed | Implement Accessstaticmemberfrominstancemethod |
| 094 | Implement Accessoraccidentalcalldiagnostic | spike | reference | class: triage-needed | Implement Accessoraccidentalcalldiagnostic |
| 096 | Implement Accessordeclarationemitjs | spike | reference | class: triage-needed | Implement Accessordeclarationemitjs |
| 097 | Implement Accessordeclarationemitvisibilityerrors | spike | reference | class: triage-needed | Implement Accessordeclarationemitvisibilityerrors |
| 098 | Implement Accessordeclarationorder | spike | reference | class: triage-needed | Implement Accessordeclarationorder |
| 099 | Implement Accessorinambientcontextes | spike | reference | class: triage-needed | Implement Accessorinambientcontextes |
| 100 | Implement Accessorinferredreturntypeerrorinreturnstatement | spike | reference | class: triage-needed | Implement Accessorinferredreturntypeerrorinreturnstatement |
| 101 | Implement Accessorparameteraccessibilitymodifier | spike | reference | class: triage-needed | Implement Accessorparameteraccessibilitymodifier |
| 102 | Implement Accessorwithinitializer | spike | reference | class: triage-needed | Implement Accessorwithinitializer |
| 103 | Implement Accessorwithlineterminator | spike | reference | class: triage-needed | Implement Accessorwithlineterminator |
| 104 | Implement Accessorwithrestparam | spike | reference | class: triage-needed | Implement Accessorwithrestparam |
| 105 | Implement Accessorwithoutbody | spike | reference | class: triage-needed | Implement Accessorwithoutbody |
| 106 | Implement Accessors | spike | reference | class: triage-needed | Implement Accessors |
| 107 | Implement Accessorsemit | spike | reference | class: triage-needed | Implement Accessorsemit |
| 108 | Implement Accessorsinambientcontext | spike | reference | class: triage-needed | Implement Accessorsinambientcontext |
| 109 | Implement Addmorecallsignaturestobasesignature | spike | reference | class: triage-needed | Implement Addmorecallsignaturestobasesignature |
| 111 | Implement Aliasassignments | spike | reference | class: triage-needed | Implement Aliasassignments |
| 112 | Implement Aliasbug | spike | reference | class: triage-needed | Implement Aliasbug |
| 113 | Implement Aliasdoesnotduplicatesignatures | spike | reference | class: triage-needed | Implement Aliasdoesnotduplicatesignatures |
| 114 | Implement Aliaserrors | spike | reference | class: triage-needed | Implement Aliaserrors |
| 115 | Implement Aliasinaccessiblemodule | spike | reference | class: triage-needed | Implement Aliasinaccessiblemodule |
| 116 | Implement Aliasinstantiationexpressiongenericintersectionnocrash | spike | reference | class: triage-needed | Implement Aliasinstantiationexpressiongenericintersectionnocrash |
| 117 | Implement Aliasofgenericfunctionwithrestbehavedsameasunaliased | spike | reference | class: triage-needed | Implement Aliasofgenericfunctionwithrestbehavedsameasunaliased |
| 118 | Implement Aliasonmergedmoduleinterface | spike | reference | class: triage-needed | Implement Aliasonmergedmoduleinterface |
| 119 | Implement Aliasusageinaccessorsofclass | spike | reference | class: triage-needed | Implement Aliasusageinaccessorsofclass |
| 120 | Implement Aliasusageinarray | spike | reference | class: triage-needed | Implement Aliasusageinarray |
| 121 | Implement Aliasusageinfunctionexpression | spike | reference | class: triage-needed | Implement Aliasusageinfunctionexpression |
| 122 | Implement Aliasusageingenericfunction | spike | reference | class: triage-needed | Implement Aliasusageingenericfunction |
| 123 | Implement Aliasusageinindexerofclass | spike | reference | class: triage-needed | Implement Aliasusageinindexerofclass |
| 124 | Implement Aliasusageinobjectliteral | spike | reference | class: triage-needed | Implement Aliasusageinobjectliteral |
| 125 | Implement Aliasusageinorexpression | spike | reference | class: triage-needed | Implement Aliasusageinorexpression |
| 126 | Implement Aliasusageintypeargumentofextendsclause | spike | reference | class: triage-needed | Implement Aliasusageintypeargumentofextendsclause |
| 127 | Implement Aliasusageinvarassignment | spike | reference | class: triage-needed | Implement Aliasusageinvarassignment |
| 128 | Implement Aliasusedasnamevalue | spike | reference | class: triage-needed | Implement Aliasusedasnamevalue |
| 129 | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer | spike | reference | class: triage-needed | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer |
| 130 | Implement Aliasesinsystemmodule | spike | reference | class: triage-needed | Implement Aliasesinsystemmodule |
| 131 | Implement Allowimportclausestomergewithtypes | spike | reference | class: triage-needed | Implement Allowimportclausestomergewithtypes |
| 132 | Implement Allowjsclassthistypecrash | spike | reference | class: triage-needed | Implement Allowjsclassthistypecrash |
| 133 | Implement Allowjscrossmonorepopackage | spike | reference | class: triage-needed | Implement Allowjscrossmonorepopackage |
| 134 | Implement Allowjscheckjstypeparameternocrash | spike | reference | class: triage-needed | Implement Allowjscheckjstypeparameternocrash |
| 135 | Implement Allowsyntheticdefaultimports | spike | reference | class: triage-needed | Implement Allowsyntheticdefaultimports |
| 136 | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration | spike | reference | class: triage-needed | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration |
| 137 | Implement Alwaysstrictalreadyusestrict | spike | reference | class: triage-needed | Implement Alwaysstrictalreadyusestrict |
| 138 | Implement Alwaysstrictmodule | spike | reference | class: triage-needed | Implement Alwaysstrictmodule |
| 139 | Implement Alwaysstrictnoimplicitusestrict | spike | reference | class: triage-needed | Implement Alwaysstrictnoimplicitusestrict |
| 140 | Implement Ambientclassdeclarationwithextends | spike | reference | class: triage-needed | Implement Ambientclassdeclarationwithextends |
| 141 | Implement Ambientclassdeclaredbeforebase | spike | reference | class: triage-needed | Implement Ambientclassdeclaredbeforebase |
| 142 | Implement Ambientclassmergesoverloadswithinterface | spike | reference | class: triage-needed | Implement Ambientclassmergesoverloadswithinterface |
| 143 | Implement Ambientclassoverloadforfunction | spike | reference | class: triage-needed | Implement Ambientclassoverloadforfunction |
| 144 | Implement Ambientconstliterals | spike | reference | class: triage-needed | Implement Ambientconstliterals |
| 145 | Implement Ambientenum | spike | reference | class: triage-needed | Implement Ambientenum |
| 146 | Implement Ambientenumelementinitializer | spike | reference | class: triage-needed | Implement Ambientenumelementinitializer |
| 147 | Implement Ambienterrors | spike | reference | class: triage-needed | Implement Ambienterrors |
| 148 | Implement Ambientexportdefaulterrors | spike | reference | class: triage-needed | Implement Ambientexportdefaulterrors |
| 149 | Implement Ambientexternalmoduleinanotherexternalmodule | spike | reference | class: triage-needed | Implement Ambientexternalmoduleinanotherexternalmodule |
| 150 | Implement Ambientexternalmodulereopen | spike | reference | class: triage-needed | Implement Ambientexternalmodulereopen |
| 151 | Implement Ambientexternalmodulewithinternalimportdeclaration | spike | reference | class: triage-needed | Implement Ambientexternalmodulewithinternalimportdeclaration |
| 152 | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration | spike | reference | class: triage-needed | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration |
| 153 | Implement Ambientexternalmodulewithrelativemodulename | spike | reference | class: triage-needed | Implement Ambientexternalmodulewithrelativemodulename |
| 154 | Implement Ambientexternalmodulewithoutinternalimportdeclaration | spike | reference | class: triage-needed | Implement Ambientexternalmodulewithoutinternalimportdeclaration |
| 155 | Implement Ambientfundule | spike | reference | class: triage-needed | Implement Ambientfundule |
| 156 | Implement Ambientgetters | spike | reference | class: triage-needed | Implement Ambientgetters |
| 157 | Implement Ambientmoduleexports | spike | reference | class: triage-needed | Implement Ambientmoduleexports |
| 158 | Implement Ambientmodulewithclassdeclarationwithextends | spike | reference | class: triage-needed | Implement Ambientmodulewithclassdeclarationwithextends |
| 159 | Implement Ambientmodulewithtemplateliterals | spike | reference | class: triage-needed | Implement Ambientmodulewithtemplateliterals |
| 160 | Implement Ambientmodules | spike | reference | class: triage-needed | Implement Ambientmodules |
| 161 | Implement Ambientnamerestrictions | spike | reference | class: triage-needed | Implement Ambientnamerestrictions |
| 162 | Implement Ambientpropertydeclarationinjs | spike | reference | class: triage-needed | Implement Ambientpropertydeclarationinjs |
| 163 | Implement Ambientrequirefunction | spike | reference | class: triage-needed | Implement Ambientrequirefunction |
| 164 | Implement Ambientstatement | spike | reference | class: triage-needed | Implement Ambientstatement |
| 165 | Implement Ambientwithstatements | spike | reference | class: triage-needed | Implement Ambientwithstatements |
| 166 | Implement Ambiguouscallswherereturntypesagree | spike | reference | class: triage-needed | Implement Ambiguouscallswherereturntypesagree |
| 167 | Implement Ambiguousgenericassertion | spike | reference | class: triage-needed | Implement Ambiguousgenericassertion |
| 168 | Implement Ambiguousoverload | spike | reference | class: triage-needed | Implement Ambiguousoverload |
| 169 | Implement Ambiguousoverloadresolution | spike | reference | class: triage-needed | Implement Ambiguousoverloadresolution |
| 170 | Implement Amddeclarationemitnoextradeclare | spike | reference | class: triage-needed | Implement Amddeclarationemitnoextradeclare |
| 171 | Implement Amddependencycomment | spike | reference | class: triage-needed | Implement Amddependencycomment |
| 172 | Implement Amddependencycommentname | spike | reference | class: triage-needed | Implement Amddependencycommentname |
| 173 | Implement Amdlikeinputdeclarationemit | spike | reference | class: triage-needed | Implement Amdlikeinputdeclarationemit |
| 174 | Implement Amdmodulebundlenoduplicatedeclarationemitcomments | spike | reference | class: triage-needed | Implement Amdmodulebundlenoduplicatedeclarationemitcomments |
| 175 | Implement Amdmoduleconstenumusage | spike | reference | class: triage-needed | Implement Amdmoduleconstenumusage |
| 176 | Implement Amdmodulename | spike | reference | class: triage-needed | Implement Amdmodulename |
| 177 | Implement Anonclassdeclarationemitisanon | spike | reference | class: triage-needed | Implement Anonclassdeclarationemitisanon |
| 178 | Implement Anonterface | spike | reference | class: triage-needed | Implement Anonterface |
| 179 | Implement Anonymousclassdeclarationdoesntprintwithreadonly | spike | reference | class: triage-needed | Implement Anonymousclassdeclarationdoesntprintwithreadonly |
| 180 | Implement Anonymousclassexpression | spike | reference | class: triage-needed | Implement Anonymousclassexpression |
| 181 | Implement Anonymousmodules | spike | reference | class: triage-needed | Implement Anonymousmodules |
| 182 | Implement Anyandunknownhavefalsycomponents | spike | reference | class: triage-needed | Implement Anyandunknownhavefalsycomponents |
| 183 | Implement Anyasreturntypefornewoncall | spike | reference | class: triage-needed | Implement Anyasreturntypefornewoncall |
| 184 | Implement Anydeclare | spike | reference | class: triage-needed | Implement Anydeclare |
| 185 | Implement Anyidenticaltoitself | spike | reference | class: triage-needed | Implement Anyidenticaltoitself |
| 187 | Implement Anyinferenceanonymousfunctions | spike | reference | class: triage-needed | Implement Anyinferenceanonymousfunctions |
| 192 | Implement Argsinscope | spike | reference | class: triage-needed | Implement Argsinscope |
| 193 | Implement Arguments | spike | reference | class: triage-needed | Implement Arguments |
| 194 | Implement Argumentsaspropertyname | spike | reference | class: triage-needed | Implement Argumentsaspropertyname |
| 195 | Implement Argumentsbindstofunctionscopeargumentlist | spike | reference | class: triage-needed | Implement Argumentsbindstofunctionscopeargumentlist |
| 196 | Implement Argumentsobjectcreatesrestforjs | spike | reference | class: triage-needed | Implement Argumentsobjectcreatesrestforjs |
| 197 | Implement Argumentsobjectiterator | spike | reference | class: triage-needed | Implement Argumentsobjectiterator |
| 198 | Implement Argumentspropertynameinjsmode | spike | reference | class: triage-needed | Implement Argumentspropertynameinjsmode |
| 199 | Implement Compiler | spike | reference | class: triage-needed | Implement Compiler |
| 200 | Implement parser syntax extensions | spike | reference | class: triage-needed | Implement parser syntax extensions |
| 201 | Investigate and classify unknown-unsupported cases | spike | reference | class: triage-needed | Investigate and classify unknown-unsupported cases |
| 240 | Implement Date timezone-aware toString policy | feature | runtime/builtins | class: blocked | Implement Date timezone-aware toString policy |
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
| 050a | Document Date deterministic subset and live-time policy gap | docs | runtime/builtins | see `issues/done/050a-document-date-deterministic-subset-and-live-time-policy-gap.md` |
| 051 | Implement RegExp | feature | runtime/builtins | see `issues/done/051-implement-regexp.md` |
| 052a | Close JSON supported subset contract | docs | runtime/builtins | see `issues/done/052a-close-json-supported-subset-contract.md` |
| 053 | Implement Math | feature | runtime/builtins | see `issues/done/053-implement-math.md` |
| 054 | Implement Error types | feature | runtime/builtins | see `issues/done/054-implement-error-types.md` |
| 055 | Umbrella: implement import and export | feature | frontend/semantics | see `issues/done/055-implement-import-export.md` |
| 056 | Implement name resolution for variables and identifiers | feature | frontend | see `issues/done/056-implement-name-resolution.md` |
| 057 | Implement function resolution for function calls | feature | frontend | see `issues/done/057-implement-function-resolution.md` |
| 058 | Implement equality operators (==, !=, ===, !==) | feature | runtime/semantics | see `issues/done/058-implement-equality-operators.md` |
| 059a | Implement TypeScript satisfies and const assertion erasure | feature | frontend | see `issues/done/059a-implement-typescript-satisfies-and-const-assertion-erasure.md` |
| 060 | Investigate and classify unknown-unsupported diagnostic cases | spike | frontend | see `issues/done/060-investigate-unknown-unsupported-cases.md` |
| 060a | Close unknown-unsupported fixed-window spike | spike | frontend | see `issues/done/060a-close-unknown-unsupported-fixed-window-spike.md` |
| 061 | Implement Date object support | feature | frontend | see `issues/done/061-implement-date.md` |
| 061a | Merge Date reference issue into Date epic | cleanup | issues | see `issues/done/061a-merge-date-reference-issue-into-date-epic.md` |
| 062a | Split function epic into callable child issues | cleanup | issues | see `issues/done/062a-split-function-epic-into-callable-child-issues.md` |
| 062b | Own dynamic Function constructor diagnostics | feature | frontend/semantics | see `issues/done/062b-dynamic-function-constructor-diagnostics.md` |
| 063 | Implement function resolution | feature | frontend | see `issues/done/063-implement-function-resolution.md` |
| 064a | Resolve Date global builtin namespace | feature | frontend | see `issues/done/064a-resolve-date-global-builtin-namespace.md` |
| 065 | Implement parser syntax extensions | feature | frontend | see `issues/done/065-implement-parser-syntax.md` |
| 065a | Merge duplicate parser syntax issue into 059 | cleanup | issues | see `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` |
| 077 | Implement Interfacedeclaration | spike | reference | see `issues/done/077-implement-InterfaceDeclaration.md` |
| 085 | Implement Abstractinterfaceidentifiername | spike | reference | see `issues/done/085-implement-abstractInterfaceIdentifierName.md` |
| 095 | Implement Accessorbodyintypecontext | spike | reference | see `issues/done/095-implement-accessorBodyInTypeContext.md` |
| 110 | Implement Addmoreoverloadstobasesignature | spike | reference | see `issues/done/110-implement-addMoreOverloadsToBaseSignature.md` |
| 186 | Implement Anyindexedaccessarraynoexception | spike | reference | see `issues/done/186-implement-anyIndexedAccessArrayNoException.md` |
| 188 | Implement Anyisassignabletoobject | spike | reference | see `issues/done/188-implement-anyIsAssignableToObject.md` |
| 189 | Implement Anyisassignabletovoid | spike | reference | see `issues/done/189-implement-anyIsAssignableToVoid.md` |
| 190 | Implement Anymappedtypeserror | spike | reference | see `issues/done/190-implement-anyMappedTypesError.md` |
| 191 | Implement Anyplusany | spike | reference | see `issues/done/191-implement-anyPlusAny.md` |
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
| 239 | Design Date live-time capability policy | docs | runtime/builtins | see `issues/done/239-design-date-live-time-capability-policy.md` |
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
