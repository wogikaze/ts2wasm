# Issues Index

This file is the human entrypoint for the issue queue.

Issue files are the source of truth for work items. The generated section below may be replaced by a script or pasted manually from a generated report.

## Summary

<!-- generated:summary:start -->
| Area | Total | Open | Resolved |
|---|---:|---:|---:|
| abi | 2 | 0 | 2 |
| backend | 5 | 2 | 3 |
| cli | 3 | 0 | 3 |
| compiler | 1 | 0 | 1 |
| docs | 2 | 0 | 2 |
| frontend | 191 | 129 | 62 |
| ir | 7 | 0 | 7 |
| issues | 4 | 0 | 4 |
| parser | 1 | 0 | 1 |
| reference | 2 | 2 | 0 |
| runtime | 58 | 11 | 47 |
| scripts | 2 | 0 | 2 |
| security | 1 | 0 | 1 |
| tests | 5 | 0 | 5 |
| wasi | 1 | 0 | 1 |
| total | 285 | 144 | 141 |
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
| 062e | Implement function closures | feature | frontend/semantics | implementation-ready | P1 |  | Captured lexical environments require different resolver, lowering, |
| 225 | Implement eval and Annex B function declaration semantics | feature | frontend/semantics | implementation-ready | P3 |  | Direct `eval` and dynamic code evaluation are required JavaScript semantics; when wasm-only implementation is not suf... |
| 250 | Design BigInt runtime value support | feature | runtime/semantics | design-ready | P2 |  | BigInt literals now parse as explicit AST nodes, but runtime representation, operations, equality, and builtin behavi... |
| 251 | Implement destructuring binding runtime semantics | feature | frontend/semantics | implementation-ready | P2 |  | Parsed destructuring binding patterns are accepted for AST/dump coverage, but name resolution, lowering, and runtime ... |
| 253 | Implement optional chaining runtime semantics | feature | frontend/semantics | implementation-ready | P2 |  | Issue 246 classifies `obj?.x`, `obj?.[key]`, and `fn?.()` in the frontend parser, but name resolution currently repor... |
| 254 | Implement class static block runtime semantics | feature | runtime/semantics | implementation-ready | P2 |  | Issue 249 parses `static { ... }` as `ClassStaticBlock`, but builtin resolution currently rejects static blocks with ... |
| 255 | Implement private class element runtime semantics | feature | runtime/semantics | implementation-ready | P2 |  | Issue 248 tokenizes `#name` and parses private fields, methods, getters, and setters, but builtin resolution rejects ... |
| 257 | Emit heap closure allocation and dispatch | feature | backend | implementation-ready | P1 | 256 | The backend currently emits `LoweredExpr::ArrowFn` as an opaque numeric |
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
| 062 | Implement function support | feature | frontend/semantics | class: blocked | Implement function support |
| 062g | Define and implement heap closure object ABI and rooting | feature | runtime/abi | class: blocked | Define and implement heap closure object ABI and rooting |
| 064 | Implement name resolution | spike | frontend/resolver | class: triage-needed | Implement name resolution |
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
| 258 | Mark heap closure captures and add allocation-pressure fixture | feature | runtime | 257 | Mark heap closure captures and add allocation-pressure fixture |
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
| 062a | Split function epic into callable child issues | cleanup | issues | see `issues/done/062a-split-function-epic-into-callable-child-issues.md` |
| 062b | Own dynamic Function constructor diagnostics | feature | frontend/semantics | see `issues/done/062b-dynamic-function-constructor-diagnostics.md` |
| 062c | Implement ordinary function declarations and direct calls | feature | frontend/semantics | see `issues/done/062c-ordinary-function-declarations-and-calls.md` |
| 062d | Implement function this and arguments semantics | feature | frontend/semantics | see `issues/done/062d-function-this-and-arguments.md` |
| 062f | Implement function object metadata | feature | frontend/semantics | see `issues/done/062f-function-object-metadata.md` |
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
| 252 | Implement destructuring assignment pattern parser support | feature | frontend/syntax | see `issues/done/252-implement-destructuring-assignment-pattern-parser.md` |
| 256 | Lower returned immutable closures to heap closure values | feature | ir | see `issues/done/256-lower-returned-immutable-closures-to-heap-values.md` |
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
