# Semantic Feature Matrix

Last updated: 2026-05-12

この文書は、ts2wasm が対応する言語機能のセマンティックフィーチャーマトリクスを定義する。
各 feature label は `fixtures/catalog.yaml` で参照され、テストカバレッジとの対応を機械的に検証できる。

## Feature Labels

### 値と型 (ValueTypes)

| Label | 説明 | 実装状況 | 主な fixture 群 |
|---|---|---|---|
| `value-types:undefined` | undefined 値 | 実装済み | core-expressions, core-semantics |
| `value-types:null` | null 値 | 実装済み | core-expressions, core-semantics |
| `value-types:boolean` | boolean 値 | 実装済み | core-expressions, primitives-control-flow |
| `value-types:number` | number (IEEE 754 double) | 実装済み | core-expressions, primitives-control-flow |
| `value-types:string` | string (UTF-8 heap) | 実装済み | core-expressions, primitives-control-flow |
| `value-types:object` | object | 実装済み | arrays-objects, core-expressions |
| `value-types:array` | array | 実装済み | arrays-objects, builtins-and-io |
| `value-types:function` | function | 実装済み | core-statements, core-semantics |
| `value-types:bigint` | bigint | 部分実装 | core-semantics |
| `value-types:symbol` | symbol | 未実装 | - |

### 式と演算子 (Expressions)

| Label | 説明 | 実装状況 | 主な fixture 群 |
|---|---|---|---|
| `expr:strict-equality` | `===` / `!==` | 実装済み | core-expressions, core-semantics |
| `expr:abstract-equality` | `==` / `!=` | 実装済み | core-semantics |
| `expr:relational` | `<` `>` `<=` `>=` | 実装済み | core-expressions, core-semantics |
| `expr:arithmetic` | `+` `-` `*` `/` `%` | 実装済み | core-expressions, core-semantics |
| `expr:increment-decrement` | `++` `--` | 実装済み | core-expressions, core-semantics |
| `expr:logical` | `&&` `||` `!` | 実装済み | core-statements, core-expressions |
| `expr:nullish-coalescing` | `??` | 実装済み | core-semantics |
| `expr:optional-chaining` | `?.` | 部分実装 | core-expressions |
| `expr:typeof` | `typeof` | 実装済み | basics-typeof, core-semantics |
| `expr:instanceof` | `instanceof` | 実装済み | core-expressions, core-semantics |
| `expr:in-operator` | `in` | 実装済み | core-semantics |
| `expr:delete` | `delete` | 実装済み | core-semantics |
| `expr:void` | `void` | 実装済み | core-semantics |
| `expr:ternary` | ternary `? :` | 実装済み | core-expressions, core-semantics |
| `expr:comma` | comma operator | 実装済み | basics-syntax, core-semantics |
| `expr:spread` | spread (`...`) | 実装済み | spread-args, core-semantics |
| `expr:template-literal` | template literal | 実装済み | core-semantics |
| `expr:logical-assignment` | `&&=` `||=` `??=` | 実装済み | core-expressions, core-semantics |
| `expr:null` | null literal | 実装済み | core-expressions |
| `expr:undefined` | undefined literal | 実装済み | core-expressions |
| `expr:boolean-literal` | `true` / `false` | 実装済み | core-expressions |
| `expr:numeric-literal` | numeric literals | 実装済み | core-expressions |
| `expr:string-literal` | string literals | 実装済み | core-expressions |
| `expr:array-literal` | array literals `[...]` | 実装済み | arrays-objects, core-expressions |
| `expr:object-literal` | object literals `{...}` | 実装済み | arrays-objects, core-expressions |
| `expr:function-expr` | function expressions | 実装済み | core-expressions |
| `expr:arrow-fn` | arrow functions | 実装済み | arrow-functions, core-expressions |
| `expr:class-expr` | class expressions | 部分実装 | core-expressions, classes |
| `expr:await` | `await` expression | 未実装 | core-expressions (unsupported) |
| `expr:call` | function call | 実装済み | core-expressions |
| `expr:new` | `new` expression | 実装済み | core-expressions, classes-and-inheritance |
| `expr:member-access` | `.` property access | 実装済み | core-expressions, arrays-objects |
| `expr:computed-access` | `[]` computed access | 実装済み | core-expressions, arrays-objects |
| `expr:index-assign` | index assignment | 実装済み | core-expressions |
| `expr:property-assign` | property assignment | 実装済み | core-expressions |

### 文 (Statements)

| Label | 説明 | 実装状況 | 主な fixture 群 |
|---|---|---|---|
| `stmt:if` | `if` / `else` | 実装済み | core-statements, primitives-control-flow |
| `stmt:switch` | `switch` / `case` | 実装済み | control-flow-and-exceptions, core-statements |
| `stmt:for` | `for` loop | 実装済み | core-statements, control-flow-and-exceptions |
| `stmt:while` | `while` loop | 実装済み | core-statements, primitives-control-flow |
| `stmt:do-while` | `do` / `while` loop | 実装済み | control-flow-and-exceptions, core-statements |
| `stmt:for-in` | `for...in` | 部分実装 | control-flow-and-exceptions, core-statements |
| `stmt:for-of` | `for...of` | 実装済み | control-flow-and-exceptions, core-statements |
| `stmt:break` | `break` | 実装済み | control-flow-and-exceptions, core-statements |
| `stmt:continue` | `continue` | 実装済み | control-flow-and-exceptions, core-statements |
| `stmt:return` | `return` | 実装済み | core-statements |
| `stmt:throw` | `throw` | 実装済み | core-statements, control-flow-and-exceptions |
| `stmt:try-catch` | `try` / `catch` | 実装済み | core-statements, control-flow-and-exceptions |
| `stmt:try-finally` | `try` / `finally` | 実装済み | control-flow-and-exceptions |
| `stmt:label` | labeled statement | 実装済み | control-flow-and-exceptions, core-statements |
| `stmt:let` | `let` declaration | 実装済み | core-statements |
| `stmt:const` | `const` declaration | 実装済み | core-statements |
| `stmt:var` | `var` declaration | 未実装 | - |
| `stmt:function-decl` | function declaration | 実装済み | core-statements |
| `stmt:class-decl` | class declaration | 実装済み | classes, classes-and-inheritance |
| `stmt:export` | `export` declaration | 部分実装 | stmt, module-system |
| `stmt:import` | `import` declaration | 部分実装 | stmt, module-system |
| `stmt:debugger` | `debugger` | 未実装 | core-semantics (unsupported) |

### 関数 (Functions)

| Label | 説明 | 実装状況 | 主な fixture 群 |
|---|---|---|---|
| `fn:declaration` | function declaration | 実装済み | core-statements, core-semantics |
| `fn:expression` | function expression | 実装済み | core-expressions, core-semantics |
| `fn:arrow` | arrow function | 実装済み | arrow-functions, core-semantics |
| `fn:this` | `this` binding | 実装済み | this-binding, core-semantics |
| `fn:arguments` | `arguments` object | 実装済み | core-semantics |
| `fn:rest-params` | rest parameters | 実装済み | rest-parameters, core-semantics |
| `fn:spread-args` | spread arguments | 実装済み | spread-args, core-semantics |
| `fn:default-params` | default parameters | 実装済み | core-semantics |
| `fn:destructuring-params` | destructuring parameters | 部分実装 | core-semantics |
| `fn:closure` | closure / capture | 実装済み | core-semantics |
| `fn:recursive` | recursive function | 実装済み | primitives-control-flow, core-semantics |
| `fn:generator` | generator function | 未実装 | builtins-and-io (unsupported) |

### オブジェクト (Objects)

| Label | 説明 | 実装状況 | 主な fixture 群 |
|---|---|---|---|
| `obj:literal` | object literals | 実装済み | arrays-objects, core-expressions |
| `obj:property-access` | `obj.key` | 実装済み | core-expressions, arrays-objects |
| `obj:computed-property` | `obj[key]` | 実装済み | arrays-objects, core-expressions |
| `obj:property-assign` | property assignment | 実装済み | core-expressions, arrays-objects |
| `obj:delete` | `delete` property | 実装済み | core-semantics |
| `obj:in-operator` | `in` operator | 実装済み | core-semantics |
| `obj:spread` | object spread | 実装済み | core-semantics |
| `obj:prototype` | prototype chain | 実装済み | core-semantics, object-semantics-kernel |
| `obj:getter-setter` | getter/setter | 部分実装 | object-semantics-kernel |
| `obj:method-shorthand` | method shorthand | 未実装 | - |
| `obj:computed-literal-key` | computed property keys `{[expr]: v}` | 未実装 | - |
| `obj:shorthand-property` | shorthand `{x}` | 未実装 | - |

### Object Semantics Kernel

| Label | 説明 | 実装状況 | 主な fixture 群 |
|---|---|---|---|
| `obj-kernel:ordinary-get` | OrdinaryGet | 実装済み | object-semantics-kernel |
| `obj-kernel:ordinary-set` | OrdinarySet | 実装済み | object-semantics-kernel |
| `obj-kernel:ordinary-has-property` | OrdinaryHasProperty | 実装済み | object-semantics-kernel |
| `obj-kernel:ordinary-delete` | OrdinaryDelete | 実装済み | object-semantics-kernel |
| `obj-kernel:ordinary-own-property-keys` | OrdinaryOwnPropertyKeys | 実装済み | object-semantics-kernel |
| `obj-kernel:ordinary-define-own-property` | OrdinaryDefineOwnProperty | 実装済み | object-semantics-kernel |
| `obj-kernel:ordinary-get-own-property` | OrdinaryGetOwnProperty | 実装済み | object-semantics-kernel |
| `obj-kernel:ordinary-get-prototype-of` | OrdinaryGetPrototypeOf | 実装済み | object-semantics-kernel |
| `obj-kernel:ordinary-set-prototype-of` | OrdinarySetPrototypeOf | 実装済み | object-semantics-kernel |

### 配列 (Arrays)

| Label | 説明 | 実装状況 | 主な fixture 群 |
|---|---|---|---|
| `array:literal` | array literal | 実装済み | arrays-objects, core-expressions |
| `array:index` | numeric index | 実装済み | arrays-objects |
| `array:length` | `.length` | 実装済み | arrays-objects |
| `array:spread` | array spread | 実装済み | spread-args, core-semantics |
| `array:destructuring` | array destructuring | 部分実装 | core-semantics |
| `array:holes` | sparse / holes | 未実装 | - |
| `array:nonnumber-index` | non-number index | 実装済み | arrays-objects |

### クラス (Classes)

| Label | 説明 | 実装状況 | 主な fixture 群 |
|---|---|---|---|
| `class:declaration` | class declaration | 実装済み | classes, classes-and-inheritance |
| `class:expression` | class expression | 実装済み | classes, core-expressions |
| `class:extends` | `extends` inheritance | 部分実装 | classes, classes-and-inheritance |
| `class:constructor` | constructor | 実装済み | classes, classes-and-inheritance |
| `class:super` | `super` access | 部分実装 | classes, classes-and-inheritance |
| `class:static-method` | static methods | 実装済み | classes |
| `class:static-field` | static field initializers | 部分実装 | classes |
| `class:static-block` | static initialization blocks | 実装済み | classes |
| `class:private-field` | private `#` fields | 部分実装 | classes, core-semantics |
| `class:private-method` | private `#` methods | 部分実装 | classes, core-semantics |
| `class:getter-setter` | class getter/setter | 部分実装 | classes |
| `class:field-initializer` | field initializers | 実装済み | classes |
| `class:new-target` | `new.target` | 実装済み | classes |

### 文字列 (Strings)

| Label | 説明 | 実装状況 | 主な fixture 群 |
|---|---|---|---|
| `string:literal` | string literals | 実装済み | core-expressions |
| `string:template` | template literals | 実装済み | core-semantics |
| `string:indexing` | `str[n]` | 実装済み | builtins-and-io |
| `string:concat` | `+` concatenation | 実装済み | builtins-and-io |
| `string:trim` | trim methods | 実装済み | builtins-and-io |
| `string:split` | split | 実装済み | builtins-and-io |
| `string:replace` | replace / replaceAll | 実装済み | builtins-and-io |
| `string:slice` | slice / substring / substr | 実装済み | builtins-and-io |
| `string:search` | search / match / matchAll | 実装済み | builtins-and-io |
| `string:case-conversion` | toUpperCase / toLowerCase | 実装済み | builtins-and-io |
| `string:padding` | padStart / padEnd | 実装済み | builtins-and-io |
| `string:repeat` | repeat | 実装済み | builtins-and-io |
| `string:includes` | includes | 実装済み | builtins-and-io |
| `string:index-of` | indexOf / lastIndexOf | 実装済み | builtins-and-io |
| `string:char-at` | charAt / charCodeAt | 実装済み | builtins-and-io |
| `string:from-char-code` | fromCharCode | 実装済み | builtins-and-io |
| `string:well-formed` | isWellFormed / toWellFormed | 実装済み | builtins-and-io |
| `string:html-wrappers` | anchor/bold/etc (Annex B) | 実装済み | builtins-and-io |

### ビルトイン (Builtins)

| Label | 説明 | 実装状況 | 主な fixture 群 |
|---|---|---|---|
| `builtin:console` | console.log | 実装済み | basics-hello, builtins-and-io |
| `builtin:math` | Math.* | 実装済み | builtins-and-io |
| `builtin:date` | Date | 実装済み | builtins-and-io |
| `builtin:json` | JSON.parse / JSON.stringify | 実装済み | builtins-and-io |
| `builtin:regexp` | RegExp | 実装済み | builtins-and-io |
| `builtin:array-methods` | Array.prototype.* | 実装済み | builtins-and-io, core-semantics |
| `builtin:string-methods` | String.prototype.* | 実装済み | builtins-and-io |
| `builtin:object-methods` | Object.* | 実装済み | builtins-and-io, object-semantics-kernel |
| `builtin:map` | Map | 実装済み | builtins-and-io |
| `builtin:set` | Set | 実装済み | builtins-and-io |
| `builtin:promise` | Promise | 部分実装 | builtins-and-io, core-semantics |
| `builtin:error` | Error / TypeError | 実装済み | builtins-and-io |
| `builtin:global-functions` | parseInt/parseFloat/isNaN/isFinite/encodeURI/decodeURI/escape/unescape | 実装済み | builtins-and-io |
| `builtin:typed-array` | TypedArray / ArrayBuffer | 実装済み | builtins-and-io |
| `builtin:weakmap-weakset` | WeakMap / WeakSet | 実装済み | builtins-and-io |
| `builtin:symbol` | Symbol | 実装済み | builtins-and-io |
| `builtin:iterator` | Iterator protocol | 実装済み | builtins-and-io, core-semantics |
| `builtin:atomics` | Atomics | 未実装 | builtins-and-io (unsupported) |
| `builtin:intl` | Intl | 未実装 | builtins-and-io (unsupported) |
| `builtin:proxy-reflect` | Proxy / Reflect | 未実装 | builtins-and-io (unsupported) |
| `builtin:eval` | eval / Function constructor | 部分実装 | builtins-and-io (unsupported for dynamic) |

### 制御フロー (ControlFlow)

| Label | 説明 | 実装状況 | 主な fixture 群 |
|---|---|---|---|
| `control:if-else` | if/else | 実装済み | core-statements, control-flow-and-exceptions |
| `control:switch` | switch/case | 実装済み | control-flow-and-exceptions |
| `control:for` | for loop | 実装済み | core-statements, control-flow-and-exceptions |
| `control:while` | while loop | 実装済み | control-flow-and-exceptions, primitives-control-flow |
| `control:do-while` | do/while | 実装済み | control-flow-and-exceptions |
| `control:for-in` | for...in | 部分実装 | control-flow-and-exceptions |
| `control:for-of` | for...of | 実装済み | control-flow-and-exceptions, core-semantics |
| `control:break` | break (labeled/unlabeled) | 実装済み | control-flow-and-exceptions |
| `control:continue` | continue (labeled/unlabeled) | 実装済み | control-flow-and-exceptions |
| `control:throw` | throw | 実装済み | control-flow-and-exceptions |
| `control:try-catch` | try/catch | 実装済み | control-flow-and-exceptions |
| `control:try-finally` | try/finally | 実装済み | control-flow-and-exceptions |
| `control:labeled` | labeled statements | 実装済み | control-flow-and-exceptions |
| `control:completion-record` | CompletionRecord semantics | 部分実装 | control-flow-and-exceptions |

### モジュール (Modules)

| Label | 説明 | 実装状況 | 主な fixture 群 |
|---|---|---|---|
| `module:static-import` | static `import` | 部分実装 | module-system, stmt |
| `module:static-export` | static `export` | 部分実装 | module-system, stmt |
| `module:dynamic-import` | dynamic `import()` | 未実装 | module-system (unsupported), builtins-and-io |
| `module:named-export` | named export | 部分実装 | module-system |
| `module:default-export` | default export | 部分実装 | module-system |
| `module:namespace-import` | namespace import | 部分実装 | module-system |
| `module:re-export` | re-export | 部分実装 | module-system |
| `module:side-effect-import` | side-effect import | 部分実装 | module-system |
| `module:live-binding` | live ES module bindings | 未実装 | module-system (unsupported) |

### 非同期 (Async)

| Label | 説明 | 実装状況 | 主な fixture 群 |
|---|---|---|---|
| `async:promise` | Promise | 部分実装 | builtins-and-io, core-semantics |
| `async:async-function` | async/await | 未実装 | async-await, core-semantics (unsupported) |
| `async:for-await` | for-await-of | 未実装 | core-semantics (unsupported) |
| `async:generator` | generator / async-generator | 未実装 | builtins-and-io, core-semantics (unsupported) |

### TypeScript

| Label | 説明 | 実装状況 | 主な fixture 群 |
|---|---|---|---|
| `ts:type-annotation` | type annotations | 実装済み | basics-types |
| `ts:interface` | interface | 実装済み | basics-types |
| `ts:type-alias` | type alias | 実装済み | basics-types |
| `ts:generic` | generics | 実装済み | basics-types |
| `ts:enum` | enum | 実装済み | basics-types |
| `ts:namespace` | namespace | 部分実装 | basics-types, core-semantics |
| `ts:ambient-declaration` | ambient declare | 実装済み | basics-types |
| `ts:as-assertion` | `as` assertion | 実装済み | basics-types |
| `ts:satisfies` | `satisfies` | 実装済み | basics-types |
| `ts:const-assertion` | `as const` | 実装済み | basics-types |
| `ts:type-only-import` | type-only import | 未実装 | core-semantics (unsupported) |
| `ts:parameter-property` | parameter properties | 未実装 | core-semantics (unsupported) |
| `ts:decorator` | decorators | 未実装 | core-semantics (unsupported) |

### その他 (Miscellaneous)

| Label | 説明 | 実装状況 | 主な fixture 群 |
|---|---|---|---|
| `misc:html-comment` | HTML comments `<!--` / `-->` | 実装済み | html-comments |
| `misc:comma-expression` | comma operator | 実装済み | basics-syntax |
| `misc:with-statement` | with statement | 未実装 | basics-syntax (unsupported) |
| `misc:direct-eval` | direct eval | 部分実装 | core-semantics |
| `misc:strict-mode` | strict mode | 未実装 | - |
| `misc:gc` | GC / memory management | 実装済み | core-semantics |

### オブジェクトセマンティクス (ObjectSemantics)

| Label | 説明 | 実装状況 | 主な fixture 群 |
|---|---|---|---|
| `obj-semantics:centralized-property-access` | centralized property access | 実装済み | object-semantics-kernel |
| `obj-semantics:computed-read-prototype` | computed prototype read | 実装済み | object-semantics-kernel |
| `obj-semantics:configurable-false` | configurable:false | 実装済み | object-semantics-kernel |
| `obj-semantics:define-property` | Object.defineProperty | 実装済み | object-semantics-kernel |
| `obj-semantics:descriptor-combinations` | descriptor combinations | 実装済み | object-semantics-kernel |
| `obj-semantics:enumerable-filtering` | enumerable filtering | 実装済み | object-semantics-kernel |
| `obj-semantics:prototype-descriptor-inheritance` | prototype descriptor inheritance | 実装済み | object-semantics-kernel |
| `obj-semantics:writable-false` | writable:false enforcement | 実装済み | object-semantics-kernel |
| `obj-semantics:seal-freeze` | Object.seal / Object.freeze | 実装済み | object-semantics-kernel, builtins-and-io |

### 入出力 (IO)

| Label | 説明 | 実装状況 | 主な fixture 群 |
|---|---|---|---|
| `io:stdout` | stdout / console.log | 実装済み | basics-hello, builtins-and-io |
| `io:stdin` | stdin | 実装済み | basics-hello, builtins-and-io |
| `io:process-env` | process.env | 実装済み | node-apis |
| `io:process-argv` | process.argv | 実装済み | node-apis |
| `io:process-exit` | process.exit | 実装済み | node-apis |
| `io:fs-read` | filesystem read | 実装済み | node-apis |
| `io:fs-write` | filesystem write | 実装済み | node-apis |
| `io:crypto` | crypto.randomBytes | 実装済み | node-apis |

### リンカ (Linker)

| Label | 説明 | 実装状況 | 主な fixture 群 |
|---|---|---|---|
| `linker:basic` | linker (snapshot-based) | 実装済み | linker |
| `linker:console-log` | linker with console.log | 実装済み | linker |
| `linker:for-loop` | linker with for loop | 実装済み | linker |
| `linker:number-add` | linker with arithmetic | 実装済み | linker |
| `linker:object-literal` | linker with objects | 実装済み | linker |

### テスト基盤 (TestInfrastructure)

| Label | 説明 | 実装状況 | 主な fixture 群 |
|---|---|---|---|
| `test-infra:pass` | passing test fixture | 実装済み | test-infrastructure |
| `test-infra:fail` | expected-fail fixture | 実装済み | test-infrastructure |
| `test-infra:unsupported` | unsupported diagnostic fixture | 実装済み | test-infrastructure |

## フィーチャーマトリクス運用ルール

1. **feature label は kebab-case** で定義する。カテゴリと機能名を `:` で区切る。
2. **fixtures/catalog.yaml** の各エントリは、少なくとも 1 つの feature label を参照する。
3. 新機能を追加する際は、本マトリクスに行を追加し、該当 fixture の catalog エントリに label を追記する。
4. `実装状況` 列は `実装済み` / `部分実装` / `未実装` のいずれかとする。
5. `部分実装` は、基本ケースは動くが edge case や完全な仕様準拠に達していないことを示す。
