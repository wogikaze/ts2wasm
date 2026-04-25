# Stream D: OOP Foundations (Classes, Constructors, Methods)

## Goal
Implement class declarations, constructors, instance/static methods, and basic inheritance (extends).

## Scope (1-2 hour window)

Leverage Stream A parser AST; implement:
1. **Class declaration** lowering to object factory functions
2. **Constructor** → factory function body
3. **Instance methods** → prototype property functions
4. **Static members** → class-level properties
5. **new** expression → factory call + `this` binding
6. **this** binding → first parameter of methods
7. **extends** → prototype chain setup
8. **super** calls → parent constructor invocation (basic)

Not yet: getters/setters, private fields, static initialization blocks, decorators.

## Implementation strategy

### Phase 1: Class lowering design (10 min)

Transform:
```typescript
class Animal {
  name: string;
  constructor(name: string) { this.name = name; }
  speak() { console.log(this.name); }
  static info() { console.log("Animal"); }
}
```

To WAT:
```javascript
// Factory function
function Animal(name) {
  this = { name: name };
  return this;
}
Animal.speak = function() { console.log("Animal"); };
Animal.prototype.speak = function() { console.log(this.name); };

// In compiled form:
// $animal_constructor: takes name param, returns object
// $animal_speak: takes this param
// Static: stored as property on factory object
```

### Phase 2: Lowered IR extension (10 min)

Add to IR:
```rust
enum Stmt {
    ClassDecl {
        name: String,
        extends: Option<String>,  // parent class name
        constructor_params: Vec<String>,
        constructor_body: Vec<Stmt>,
        methods: Vec<Method>,
        statics: Vec<(String, Expr)>,
    },
}

struct Method {
    name: String,
    params: Vec<String>,
    body: Vec<Stmt>,
    is_static: bool,
}
```

### Phase 3: Lower pass (20 min)

Implement pass_lower_class() in lowering pipeline:
1. For each ClassDecl:
   a. Create factory function: `$<ClassName>_constructor`
   b. Create method wrappers: `$<ClassName>_<method_name>`
   c. Create prototype object: `$<ClassName>_prototype`
   d. Store static methods as properties on factory
2. Emit initialization code that builds prototype chain
3. If `extends`, set up `__proto__` link

### Phase 4: WAT Emission (30 min)

#### Factory function + this binding

```wasm
(func $Animal_constructor (param $name i32) (result i32)
  (local $this i32)
  ;; Allocate object
  (local.set $this (call $alloc_heap (i32.const 32)))
  ;; Set name property
  (call $object_set_property (local.get $this) "name" (local.get $name))
  ;; Execute constructor body
  ;; ...
  (local.get $this)  ;; return this
)
```

#### Method wrapper + this binding

```wasm
(func $Animal_speak (param $this i32) (result i32)
  ;; Get name property
  (local.set $name (call $object_get_property (local.get $this) "name"))
  ;; console.log(name)
  (call $console_log (local.get $name))
  (i32.const 0)  ;; undefined
)
```

#### Static method

```wasm
;; During initialization:
(call $object_set_property 
  (global.get $Animal_constructor)
  "info"
  (func.ref $Animal_info))
```

#### new expression

```wasm
;; new Animal(name)
(call $Animal_constructor (local.get $name_value))
```

#### Prototype chain (for inherited methods)

```wasm
;; During class initialization:
(call $object_set_property 
  (global.get $Animal_prototype)
  "__proto__"
  (global.get $Mammal_prototype))  ;; if extends Mammal
```

Method lookup on `new` will check `__proto__` chain.

### Phase 5: Property access through this (15 min)

Update property read/write to detect `this` binding:
- In method context, `this.prop` → `call $object_get_property this "prop"`
- `this.prop = val` → `call $object_set_property this "prop" val`

### Phase 6: Tests (10 min)

Fixtures:
1. `class-basic.ts`: simple class with constructor + method
2. `class-static.ts`: static method and property
3. `class-extends.ts`: inheritance with super
4. `class-this.ts`: this binding in methods
5. `class-instanceof.ts`: instanceof check
6. `new-expression.ts`: new with multiple arguments

All fixtures compare with Node.js.

## Output

**Commits**:
1. `ir: add ClassDecl to LoweredIR with method/static structures`
2. `backend: add class lowering pass (factory + method wrappers)`
3. `backend: emit factory functions and method wrappers`
4. `backend: emit prototype chain setup and inheritance`
5. `backend: implement this binding in method calls`
6. `backend: implement property access through this`
7. `tests: add class and inheritance integration tests`

**Tests added**:
- `crates/cli/tests/m6_oop_classes.rs`
- Fixture files: `fixtures/m6/class-*.ts`, `fixtures/m6/inheritance-*.ts`

**DiagCode impact**:
- `class` keyword no longer causes UnsupportedSyntax
- Constructor/method calls work through factory/wrapper functions
- `instanceof` implemented for basic class type checks

**Coverage matrix delta**:
- test262 OOP tests should show progress (if present in reference corpus)
- `unsupported` decreases for files with class declarations
- `pass` increases for OOP patterns

## Validation before commit

```bash
cargo fmt --all --check
cargo test -q
cargo test -q --test m6_oop_classes
# Run OOP fixture
./target/debug/ts2wasm-cli build fixtures/m6/class-basic.ts -o /tmp/t.wasm
iwasm /tmp/t.wasm
# Compare with Node
node fixtures/m6/class-basic.ts
```

## Gatekeeper checklist

✓ this binding is automatic (not caller responsibility)
✓ Constructor returns new instance
✓ Methods receive this as first parameter (hidden)
✓ Prototype chain set up correctly for instanceof
✓ Static methods stored as factory properties
✓ super calls parent constructor correctly
✓ Inheritance works through __proto__ lookup
✓ No string literals for method names (constants)
✓ All test fixtures produce Node-compatible output

## Design decisions

1. **this binding**: Hidden first parameter in method calls (simpler than property semantics)
2. **Factory functions**: Class becomes function storing prototype + statics (ES5 compatible model)
3. **instanceof**: Checks __proto__ chain against class.prototype
4. **super**: Explicit parent constructor call in constructor (not implicit field initialization)
5. **Private fields**: Not supported (class fields are public; prefixed names as convention only)
6. **Getters/setters**: Deferred to later stream (use explicit getX/setX methods for now)

## References

- Current ClassDecl parsing: Stream A added this (SKILL.md)
- Runtime heap allocation: `crates/cli/src/lib.rs` (emitter, line ~600)
- Object property model: Already implemented for M5; use existing $object_get_property/$object_set_property
- Prototype chain: Add to object representation (currently flat)
