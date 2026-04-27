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
| 021 | Implement full wasm backend | feature | backend | implementation-ready | P2 | 008, 020 | Full wasm backend is not implemented. Current implementation is WAT-centric. docs/04 specifies initial linear memory ... |
| 022 | Expand test262 differential coverage | feature | tests/coverage | implementation-ready | P1 | 005 | test262 full differential operation is incomplete. Current coverage uses sample/ramp approach. docs/11 Gate D require... |
| 045 | Implement class declaration and expression | feature | frontend/semantics | implementation-ready | P1 |  | Implement class declaration and expression |
| 049 | Implement Map and Set | feature | runtime/builtins | implementation-ready | P1 |  | Implement Map and Set |
| 050 | Implement Date | feature | runtime/builtins | implementation-ready | P1 |  | Implement Date |
| 051 | Implement RegExp | feature | runtime/builtins | implementation-ready | P1 |  | Implement RegExp |
| 052 | Implement JSON | feature | runtime/builtins | implementation-ready | P1 |  | Implement JSON |
| 054 | Implement Error types | feature | runtime/builtins | implementation-ready | P1 |  | Implement Error types |
| 055 | Implement import and export | feature | frontend/semantics | implementation-ready | P1 |  | Implement import and export |
| 059 | Implement parser syntax extensions for TypeScript and advanced JS | feature | frontend | design-ready | P1 |  | Implement parser syntax extensions for TypeScript and advanced JS |
| 060 | Investigate and classify unknown-unsupported diagnostic cases | spike | frontend | design-ready | P1 |  | Investigate and classify unknown-unsupported diagnostic cases |
| 061 | Implement Date object support | feature | frontend | design-ready | P1 |  | Implement Date object support |
| 062 | Implement function support | feature | frontend | design-ready | P1 |  | Implement function support |
| 063 | Implement function resolution | feature | frontend | design-ready | P1 |  | Implement function resolution |
| 064 | Implement name resolution | feature | frontend | design-ready | P1 |  | Implement name resolution |
| 065 | Implement parser syntax extensions | feature | frontend | design-ready | P1 |  | Implement parser syntax extensions |
| 066 | Implement RegExp literal support | feature | frontend | design-ready | P1 |  | Implement RegExp literal support |
| 067 | Investigate and classify unknown-unsupported cases | feature | frontend | design-ready | P1 |  | Investigate and classify unknown-unsupported cases |
| 068 | Implement unsupported expression types | feature | frontend | design-ready | P1 |  | Implement unsupported expression types |
| 069 | Implement Apilibcheck | feature | frontend | design-ready | P1 |  | Implement Apilibcheck |
| 070 | Implement Apisample | feature | frontend | design-ready | P1 |  | Implement Apisample |
| 071 | Implement Arrowfunctionexpression | feature | frontend | design-ready | P1 |  | Implement Arrowfunctionexpression |
| 072 | Implement Classdeclaration | feature | frontend | design-ready | P1 |  | Implement Classdeclaration |
| 073 | Implement Classdeclarationwithinvalidconstonpropertydeclaration | feature | frontend | design-ready | P1 |  | Implement Classdeclarationwithinvalidconstonpropertydeclaration |
| 074 | Implement Declarationerrorsnoemitonerror | feature | frontend | design-ready | P1 |  | Implement Declarationerrorsnoemitonerror |
| 075 | Implement Exportassignment | feature | frontend | design-ready | P1 |  | Implement Exportassignment |
| 076 | Implement Functiondeclaration | feature | frontend | design-ready | P1 |  | Implement Functiondeclaration |
| 077 | Implement Interfacedeclaration | feature | frontend | design-ready | P1 |  | Implement Interfacedeclaration |
| 078 | Implement Memberaccessordeclaration | feature | frontend | design-ready | P1 |  | Implement Memberaccessordeclaration |
| 079 | Implement Parameterlist | feature | frontend | design-ready | P1 |  | Implement Parameterlist |
| 080 | Implement Systemmoduleforstatementnoinitializer | feature | frontend | design-ready | P1 |  | Implement Systemmoduleforstatementnoinitializer |
| 081 | Implement Transportstream | feature | frontend | design-ready | P1 |  | Implement Transportstream |
| 082 | Implement Abstractclassinlocalscope | feature | frontend | design-ready | P1 |  | Implement Abstractclassinlocalscope |
| 083 | Implement Abstractclassinlocalscopeisabstract | feature | frontend | design-ready | P1 |  | Implement Abstractclassinlocalscopeisabstract |
| 084 | Implement Abstractclassunioninstantiation | feature | frontend | design-ready | P1 |  | Implement Abstractclassunioninstantiation |
| 085 | Implement Abstractinterfaceidentifiername | feature | frontend | design-ready | P1 |  | Implement Abstractinterfaceidentifiername |
| 086 | Implement Abstractpropertybasics | feature | frontend | design-ready | P1 |  | Implement Abstractpropertybasics |
| 087 | Implement Abstractpropertyinconstructor | feature | frontend | design-ready | P1 |  | Implement Abstractpropertyinconstructor |
| 088 | Implement Abstractpropertynegative | feature | frontend | design-ready | P1 |  | Implement Abstractpropertynegative |
| 089 | Implement Acceptsymbolasweaktype | feature | frontend | design-ready | P1 |  | Implement Acceptsymbolasweaktype |
| 090 | Implement Acceptablealias | feature | frontend | design-ready | P1 |  | Implement Acceptablealias |
| 091 | Implement Accessinstancememberfromstaticmethod | feature | frontend | design-ready | P1 |  | Implement Accessinstancememberfromstaticmethod |
| 092 | Implement Accessoverriddenbaseclassmember | feature | frontend | design-ready | P1 |  | Implement Accessoverriddenbaseclassmember |
| 093 | Implement Accessstaticmemberfrominstancemethod | feature | frontend | design-ready | P1 |  | Implement Accessstaticmemberfrominstancemethod |
| 094 | Implement Accessoraccidentalcalldiagnostic | feature | frontend | design-ready | P1 |  | Implement Accessoraccidentalcalldiagnostic |
| 095 | Implement Accessorbodyintypecontext | feature | frontend | design-ready | P1 |  | Implement Accessorbodyintypecontext |
| 096 | Implement Accessordeclarationemitjs | feature | frontend | design-ready | P1 |  | Implement Accessordeclarationemitjs |
| 097 | Implement Accessordeclarationemitvisibilityerrors | feature | frontend | design-ready | P1 |  | Implement Accessordeclarationemitvisibilityerrors |
| 098 | Implement Accessordeclarationorder | feature | frontend | design-ready | P1 |  | Implement Accessordeclarationorder |
| 099 | Implement Accessorinambientcontextes | feature | frontend | design-ready | P1 |  | Implement Accessorinambientcontextes |
| 100 | Implement Accessorinferredreturntypeerrorinreturnstatement | feature | frontend | design-ready | P1 |  | Implement Accessorinferredreturntypeerrorinreturnstatement |
| 101 | Implement Accessorparameteraccessibilitymodifier | feature | frontend | design-ready | P1 |  | Implement Accessorparameteraccessibilitymodifier |
| 102 | Implement Accessorwithinitializer | feature | frontend | design-ready | P1 |  | Implement Accessorwithinitializer |
| 103 | Implement Accessorwithlineterminator | feature | frontend | design-ready | P1 |  | Implement Accessorwithlineterminator |
| 104 | Implement Accessorwithrestparam | feature | frontend | design-ready | P1 |  | Implement Accessorwithrestparam |
| 105 | Implement Accessorwithoutbody | feature | frontend | design-ready | P1 |  | Implement Accessorwithoutbody |
| 106 | Implement Accessors | feature | frontend | design-ready | P1 |  | Implement Accessors |
| 107 | Implement Accessorsemit | feature | frontend | design-ready | P1 |  | Implement Accessorsemit |
| 108 | Implement Accessorsinambientcontext | feature | frontend | design-ready | P1 |  | Implement Accessorsinambientcontext |
| 109 | Implement Addmorecallsignaturestobasesignature | feature | frontend | design-ready | P1 |  | Implement Addmorecallsignaturestobasesignature |
| 110 | Implement Addmoreoverloadstobasesignature | feature | frontend | design-ready | P1 |  | Implement Addmoreoverloadstobasesignature |
| 111 | Implement Aliasassignments | feature | frontend | design-ready | P1 |  | Implement Aliasassignments |
| 112 | Implement Aliasbug | feature | frontend | design-ready | P1 |  | Implement Aliasbug |
| 113 | Implement Aliasdoesnotduplicatesignatures | feature | frontend | design-ready | P1 |  | Implement Aliasdoesnotduplicatesignatures |
| 114 | Implement Aliaserrors | feature | frontend | design-ready | P1 |  | Implement Aliaserrors |
| 115 | Implement Aliasinaccessiblemodule | feature | frontend | design-ready | P1 |  | Implement Aliasinaccessiblemodule |
| 116 | Implement Aliasinstantiationexpressiongenericintersectionnocrash | feature | frontend | design-ready | P1 |  | Implement Aliasinstantiationexpressiongenericintersectionnocrash |
| 117 | Implement Aliasofgenericfunctionwithrestbehavedsameasunaliased | feature | frontend | design-ready | P1 |  | Implement Aliasofgenericfunctionwithrestbehavedsameasunaliased |
| 118 | Implement Aliasonmergedmoduleinterface | feature | frontend | design-ready | P1 |  | Implement Aliasonmergedmoduleinterface |
| 119 | Implement Aliasusageinaccessorsofclass | feature | frontend | design-ready | P1 |  | Implement Aliasusageinaccessorsofclass |
| 120 | Implement Aliasusageinarray | feature | frontend | design-ready | P1 |  | Implement Aliasusageinarray |
| 121 | Implement Aliasusageinfunctionexpression | feature | frontend | design-ready | P1 |  | Implement Aliasusageinfunctionexpression |
| 122 | Implement Aliasusageingenericfunction | feature | frontend | design-ready | P1 |  | Implement Aliasusageingenericfunction |
| 123 | Implement Aliasusageinindexerofclass | feature | frontend | design-ready | P1 |  | Implement Aliasusageinindexerofclass |
| 124 | Implement Aliasusageinobjectliteral | feature | frontend | design-ready | P1 |  | Implement Aliasusageinobjectliteral |
| 125 | Implement Aliasusageinorexpression | feature | frontend | design-ready | P1 |  | Implement Aliasusageinorexpression |
| 126 | Implement Aliasusageintypeargumentofextendsclause | feature | frontend | design-ready | P1 |  | Implement Aliasusageintypeargumentofextendsclause |
| 127 | Implement Aliasusageinvarassignment | feature | frontend | design-ready | P1 |  | Implement Aliasusageinvarassignment |
| 128 | Implement Aliasusedasnamevalue | feature | frontend | design-ready | P1 |  | Implement Aliasusedasnamevalue |
| 129 | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer | feature | frontend | design-ready | P1 |  | Implement Aliaswithinterfaceexportassignmentusedinvarinitializer |
| 130 | Implement Aliasesinsystemmodule | feature | frontend | design-ready | P1 |  | Implement Aliasesinsystemmodule |
| 131 | Implement Allowimportclausestomergewithtypes | feature | frontend | design-ready | P1 |  | Implement Allowimportclausestomergewithtypes |
| 132 | Implement Allowjsclassthistypecrash | feature | frontend | design-ready | P1 |  | Implement Allowjsclassthistypecrash |
| 133 | Implement Allowjscrossmonorepopackage | feature | frontend | design-ready | P1 |  | Implement Allowjscrossmonorepopackage |
| 134 | Implement Allowjscheckjstypeparameternocrash | feature | frontend | design-ready | P1 |  | Implement Allowjscheckjstypeparameternocrash |
| 135 | Implement Allowsyntheticdefaultimports | feature | frontend | design-ready | P1 |  | Implement Allowsyntheticdefaultimports |
| 136 | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration | feature | frontend | design-ready | P1 |  | Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration |
| 137 | Implement Alwaysstrictalreadyusestrict | feature | frontend | design-ready | P1 |  | Implement Alwaysstrictalreadyusestrict |
| 138 | Implement Alwaysstrictmodule | feature | frontend | design-ready | P1 |  | Implement Alwaysstrictmodule |
| 139 | Implement Alwaysstrictnoimplicitusestrict | feature | frontend | design-ready | P1 |  | Implement Alwaysstrictnoimplicitusestrict |
| 140 | Implement Ambientclassdeclarationwithextends | feature | frontend | design-ready | P1 |  | Implement Ambientclassdeclarationwithextends |
| 141 | Implement Ambientclassdeclaredbeforebase | feature | frontend | design-ready | P1 |  | Implement Ambientclassdeclaredbeforebase |
| 142 | Implement Ambientclassmergesoverloadswithinterface | feature | frontend | design-ready | P1 |  | Implement Ambientclassmergesoverloadswithinterface |
| 143 | Implement Ambientclassoverloadforfunction | feature | frontend | design-ready | P1 |  | Implement Ambientclassoverloadforfunction |
| 144 | Implement Ambientconstliterals | feature | frontend | design-ready | P1 |  | Implement Ambientconstliterals |
| 145 | Implement Ambientenum | feature | frontend | design-ready | P1 |  | Implement Ambientenum |
| 146 | Implement Ambientenumelementinitializer | feature | frontend | design-ready | P1 |  | Implement Ambientenumelementinitializer |
| 147 | Implement Ambienterrors | feature | frontend | design-ready | P1 |  | Implement Ambienterrors |
| 148 | Implement Ambientexportdefaulterrors | feature | frontend | design-ready | P1 |  | Implement Ambientexportdefaulterrors |
| 149 | Implement Ambientexternalmoduleinanotherexternalmodule | feature | frontend | design-ready | P1 |  | Implement Ambientexternalmoduleinanotherexternalmodule |
| 150 | Implement Ambientexternalmodulereopen | feature | frontend | design-ready | P1 |  | Implement Ambientexternalmodulereopen |
| 151 | Implement Ambientexternalmodulewithinternalimportdeclaration | feature | frontend | design-ready | P1 |  | Implement Ambientexternalmodulewithinternalimportdeclaration |
| 152 | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration | feature | frontend | design-ready | P1 |  | Implement Ambientexternalmodulewithrelativeexternalimportdeclaration |
| 153 | Implement Ambientexternalmodulewithrelativemodulename | feature | frontend | design-ready | P1 |  | Implement Ambientexternalmodulewithrelativemodulename |
| 154 | Implement Ambientexternalmodulewithoutinternalimportdeclaration | feature | frontend | design-ready | P1 |  | Implement Ambientexternalmodulewithoutinternalimportdeclaration |
| 155 | Implement Ambientfundule | feature | frontend | design-ready | P1 |  | Implement Ambientfundule |
| 156 | Implement Ambientgetters | feature | frontend | design-ready | P1 |  | Implement Ambientgetters |
| 157 | Implement Ambientmoduleexports | feature | frontend | design-ready | P1 |  | Implement Ambientmoduleexports |
| 158 | Implement Ambientmodulewithclassdeclarationwithextends | feature | frontend | design-ready | P1 |  | Implement Ambientmodulewithclassdeclarationwithextends |
| 159 | Implement Ambientmodulewithtemplateliterals | feature | frontend | design-ready | P1 |  | Implement Ambientmodulewithtemplateliterals |
| 160 | Implement Ambientmodules | feature | frontend | design-ready | P1 |  | Implement Ambientmodules |
| 161 | Implement Ambientnamerestrictions | feature | frontend | design-ready | P1 |  | Implement Ambientnamerestrictions |
| 162 | Implement Ambientpropertydeclarationinjs | feature | frontend | design-ready | P1 |  | Implement Ambientpropertydeclarationinjs |
| 163 | Implement Ambientrequirefunction | feature | frontend | design-ready | P1 |  | Implement Ambientrequirefunction |
| 164 | Implement Ambientstatement | feature | frontend | design-ready | P1 |  | Implement Ambientstatement |
| 165 | Implement Ambientwithstatements | feature | frontend | design-ready | P1 |  | Implement Ambientwithstatements |
| 166 | Implement Ambiguouscallswherereturntypesagree | feature | frontend | design-ready | P1 |  | Implement Ambiguouscallswherereturntypesagree |
| 167 | Implement Ambiguousgenericassertion | feature | frontend | design-ready | P1 |  | Implement Ambiguousgenericassertion |
| 168 | Implement Ambiguousoverload | feature | frontend | design-ready | P1 |  | Implement Ambiguousoverload |
| 169 | Implement Ambiguousoverloadresolution | feature | frontend | design-ready | P1 |  | Implement Ambiguousoverloadresolution |
| 170 | Implement Amddeclarationemitnoextradeclare | feature | frontend | design-ready | P1 |  | Implement Amddeclarationemitnoextradeclare |
| 171 | Implement Amddependencycomment | feature | frontend | design-ready | P1 |  | Implement Amddependencycomment |
| 172 | Implement Amddependencycommentname | feature | frontend | design-ready | P1 |  | Implement Amddependencycommentname |
| 173 | Implement Amdlikeinputdeclarationemit | feature | frontend | design-ready | P1 |  | Implement Amdlikeinputdeclarationemit |
| 174 | Implement Amdmodulebundlenoduplicatedeclarationemitcomments | feature | frontend | design-ready | P1 |  | Implement Amdmodulebundlenoduplicatedeclarationemitcomments |
| 175 | Implement Amdmoduleconstenumusage | feature | frontend | design-ready | P1 |  | Implement Amdmoduleconstenumusage |
| 176 | Implement Amdmodulename | feature | frontend | design-ready | P1 |  | Implement Amdmodulename |
| 177 | Implement Anonclassdeclarationemitisanon | feature | frontend | design-ready | P1 |  | Implement Anonclassdeclarationemitisanon |
| 178 | Implement Anonterface | feature | frontend | design-ready | P1 |  | Implement Anonterface |
| 179 | Implement Anonymousclassdeclarationdoesntprintwithreadonly | feature | frontend | design-ready | P1 |  | Implement Anonymousclassdeclarationdoesntprintwithreadonly |
| 180 | Implement Anonymousclassexpression | feature | frontend | design-ready | P1 |  | Implement Anonymousclassexpression |
| 181 | Implement Anonymousmodules | feature | frontend | design-ready | P1 |  | Implement Anonymousmodules |
| 182 | Implement Anyandunknownhavefalsycomponents | feature | frontend | design-ready | P1 |  | Implement Anyandunknownhavefalsycomponents |
| 183 | Implement Anyasreturntypefornewoncall | feature | frontend | design-ready | P1 |  | Implement Anyasreturntypefornewoncall |
| 184 | Implement Anydeclare | feature | frontend | design-ready | P1 |  | Implement Anydeclare |
| 185 | Implement Anyidenticaltoitself | feature | frontend | design-ready | P1 |  | Implement Anyidenticaltoitself |
| 186 | Implement Anyindexedaccessarraynoexception | feature | frontend | design-ready | P1 |  | Implement Anyindexedaccessarraynoexception |
| 187 | Implement Anyinferenceanonymousfunctions | feature | frontend | design-ready | P1 |  | Implement Anyinferenceanonymousfunctions |
| 188 | Implement Anyisassignabletoobject | feature | frontend | design-ready | P1 |  | Implement Anyisassignabletoobject |
| 189 | Implement Anyisassignabletovoid | feature | frontend | design-ready | P1 |  | Implement Anyisassignabletovoid |
| 190 | Implement Anymappedtypeserror | feature | frontend | design-ready | P1 |  | Implement Anymappedtypeserror |
| 191 | Implement Anyplusany | feature | frontend | design-ready | P1 |  | Implement Anyplusany |
| 192 | Implement Argsinscope | feature | frontend | design-ready | P1 |  | Implement Argsinscope |
| 193 | Implement Arguments | feature | frontend | design-ready | P1 |  | Implement Arguments |
| 194 | Implement Argumentsaspropertyname | feature | frontend | design-ready | P1 |  | Implement Argumentsaspropertyname |
| 195 | Implement Argumentsbindstofunctionscopeargumentlist | feature | frontend | design-ready | P1 |  | Implement Argumentsbindstofunctionscopeargumentlist |
| 196 | Implement Argumentsobjectcreatesrestforjs | feature | frontend | design-ready | P1 |  | Implement Argumentsobjectcreatesrestforjs |
| 197 | Implement Argumentsobjectiterator | feature | frontend | design-ready | P1 |  | Implement Argumentsobjectiterator |
| 198 | Implement Argumentspropertynameinjsmode | feature | frontend | design-ready | P1 |  | Implement Argumentspropertynameinjsmode |
| 199 | Implement Compiler | feature | frontend | design-ready | P1 |  | Implement Compiler |
| 200 | Implement parser syntax extensions | feature | frontend | design-ready | P1 |  | Implement parser syntax extensions |
| 201 | Investigate and classify unknown-unsupported cases | feature | frontend | design-ready | P1 |  | Investigate and classify unknown-unsupported cases |
| 202 | Implement RegExp literal support | feature | frontend/semantics | implementation-ready | P1 |  | RegExp literals are currently reported as unsupported in the test262 coverage sweep (`unsupported_features.regexp-lit... |
| 204 | Add typed IR dump command | feature | cli | implementation-ready | P2 | 020 | Add typed IR dump command |
| 207 | Complete instanceof prototype-chain semantics | feature | runtime/semantics | implementation-ready | P1 | 048 | Complete instanceof prototype-chain semantics |
| 209 | Implement labeled break and continue | feature | frontend/semantics | implementation-ready | P2 | 035 | Implement labeled break and continue |
| 211 | Complete this receiver binding semantics | feature | runtime/semantics | implementation-ready | P1 |  | Complete this receiver binding semantics |
| 212 | Implement rest parameter argument collection | feature | frontend/semantics | implementation-ready | P1 |  | Implement rest parameter argument collection |
| 215 | Define Math.random capability policy | feature | runtime/builtins | design-ready | P1 |  | Define Math.random capability policy |
<!-- generated:ready:end -->

## Blocked queue

<!-- generated:blocked:start -->
| ID | Title | Type | Area | Blocker | Summary |
|---:|---|---|---|---|---|
| 017b | Implement GC strategy | feature | runtime/memory | class: blocked | Implement GC strategy |
| 046 | Implement extends inheritance | feature | runtime/semantics | 045 | Implement extends inheritance |
| 047 | Implement super keyword | feature | runtime/semantics | 045, 046 | Implement super keyword |
| 205 | Add optimizer dump command | feature | cli | 020, 204 | Add optimizer dump command |
| 210 | Implement arrow function closure and lexical this semantics | feature | frontend/semantics | 211 | Implement arrow function closure and lexical this semantics |
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
| 048 | Implement prototype chain | feature | runtime/semantics | see `issues/done/048-implement-prototype-chain.md` |
| 053 | Implement Math | feature | runtime/builtins | see `issues/done/053-implement-math.md` |
| 056 | Implement name resolution for variables and identifiers | feature | frontend | see `issues/done/056-implement-name-resolution.md` |
| 057 | Implement function resolution for function calls | feature | frontend | see `issues/done/057-implement-function-resolution.md` |
| 058 | Implement equality operators (==, !=, ===, !==) | feature | runtime/semantics | see `issues/done/058-implement-equality-operators.md` |
| 203 | Reconcile partial feature semantics and placeholder completions | cleanup | docs/issues | see `issues/done/203-reconcile-partial-feature-semantics.md` |
| 206 | Make CLI a thin toolchain wrapper | refactor | cli | see `issues/done/206-make-cli-a-thin-toolchain-wrapper.md` |
| 208 | Implement switch fall-through semantics | feature | frontend/semantics | see `issues/done/208-implement-switch-fall-through-semantics.md` |
| 213 | Implement template literal interpolation | feature | frontend/semantics | see `issues/done/213-implement-template-literal-interpolation.md` |
| 214 | Replace string method placeholders | feature | runtime/builtins | see `issues/done/214-replace-string-method-placeholders.md` |
| 216 | Implement abstract equality coercion | feature | runtime/semantics | see `issues/done/216-implement-abstract-equality-coercion.md` |
<!-- generated:done:end -->

## Index generation contract

Run `scripts/manager update-issue-index` after adding, closing, or moving issues. CI and agents should run `scripts/manager update-issue-index --check` and `scripts/manager check-issue-health`. `scripts/manager check-issue-index` remains a compatibility alias.

A future generator replaces only the regions between the `<!-- generated:*:start -->` / `<!-- generated:*:end -->` markers.

Do not put hand-written policy text inside generated regions.

## Manual update checklist

When adding, completing, or blocking an issue:

- [ ] issue file is in the correct directory
- [ ] frontmatter is updated
- [ ] dependencies are reflected by re-running `scripts/manager update-issue-index`
- [ ] done issue has completion evidence
- [ ] follow-up work is represented as a separate open issue
- [ ] final-state docs do not contain future TODOs
- [ ] current implementation gaps are in `current-state.md` (repo root)
