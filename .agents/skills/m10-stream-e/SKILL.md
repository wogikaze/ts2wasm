# Stream E: Module System Foundations

## Goal
Implement require() parsing, linking, and basic module resolution for Node.js and WASI modules.

## Scope (1-2 hour window)

Implement:
1. **require() expression** parsing and lowering
2. **Module resolution** (relative paths + node_modules simulation)
3. **Module caching** (load each module once)
4. **exports** object and module.exports assignment
5. **Built-in modules** (fs, path, process, crypto mapped to manifest)
6. **Conditional imports** (require detection for capability manifest)

Not yet: ES6 import/export, circular dependency handling, complex require patterns (require in conditionals, dynamic requires).

## Implementation strategy

### Phase 1: Require parsing (10 min)

Extend parser to recognize require() calls:
```rust
// In parse_primary_expr or parse_call_expr
// Detect: Expr::Call(Expr::Ident("require"), args)
// Extract module name from string literal argument
```

Require variants:
1. `require("fs")` → built-in module
2. `require("./lib")` → relative file
3. `require("../other")` → relative directory
4. `require("package-name")` → node_modules search

### Phase 2: Module linker (20 min)

Create pass_link_requires() in lowering pipeline:
1. Scan all require() calls in program
2. For each required module:
   a. Check if built-in (fs, path, process, crypto, buffer)
   b. If file, resolve relative path → absolute
   c. Add to module load list
3. Populate capability manifest with module list
4. Return rewrite map: require("fs") → builtin.fs_module_id

### Phase 3: Lowering (15 min)

Lower IR: extend Expr with:
```rust
enum Expr {
    Require {
        module_name: String,
        resolved_id: String,  // after linking
    },
    ...
}
```

Lowering for require:
- If built-in: emit synthetic module object (exports dict)
- If file: emit stub (module link happens at runtime via host import)

### Phase 4: WAT Emission (20 min)

For built-in module require:
```wasm
(func $require_fs (result i32)
  ;; Return fs module object { readFileSync: func, writeFileSync: func, ... }
  (local $fs_module i32)
  (local.set $fs_module (call $alloc_heap (i32.const 64)))
  
  ;; Set readFileSync property → host import wrapper
  (call $object_set_property 
    (local.get $fs_module)
    "readFileSync"
    (func.ref $host_fs_readFileSync))
  ;; ... other properties ...
  
  (local.get $fs_module)
)

;; Wrapper for host import
(func $host_fs_readFileSync (param $path i32) (param $encoding i32) (result i32)
  ;; Call actual host import
  (call $host.fs.readFileSync (local.get $path) (local.get $encoding))
)
```

For file module require:
```wasm
(func $require_lib (result i32)
  ;; Load cached module or compile + link ./lib
  (global.get $module_lib_exports)
)
```

### Phase 5: Module caching (10 min)

Maintain global module cache:
```wasm
(global $module_cache i32 (i32.const 0))  ;; Map from module ID to exports

(func $get_or_load_module (param $id i32) (result i32)
  ;; Check if already loaded
  (call $map_get (global.get $module_cache) (local.get $id))
  ;; If not, compile and load
  ;; Cache result
)
```

### Phase 6: Capability manifest extension (10 min)

Extend manifest schema:
```json
{
  "modules": [
    {
      "id": "fs",
      "type": "builtin",
      "exports": ["readFileSync", "writeFileSync", "appendFileSync"]
    },
    {
      "id": "lib",
      "type": "file",
      "path": "./lib.ts",
      "exports": ["MyClass", "myFunction"]
    }
  ],
  "requires": [
    {
      "source": "main",
      "module": "fs",
      "alias": "fs"
    }
  ]
}
```

### Phase 7: Tests (10 min)

Fixtures:
1. `require-builtin-fs.ts`: require("fs") and call methods
2. `require-relative.ts`: require("./utils") from same directory
3. `require-caching.ts`: multiple requires return same object
4. `module-exports.ts`: module.exports assignment
5. `require-circular.ts`: basic circular dependency (should not hang)

All fixtures compare with Node.js (Node's require behavior as reference).

## Output

**Commits**:
1. `parser: add require() expression parsing`
2. `linker: add require resolution and module mapping`
3. `ir: extend Expr with Require variant`
4. `backend: emit module load stubs and caching`
5. `backend: emit require() expressions`
6. `backend: wire host imports through module wrappers`
7. `manifest: extend schema for module list and requires`
8. `tests: add module system integration tests`

**Tests added**:
- `crates/cli/tests/m7_modules.rs`
- Fixture files: `fixtures/m7/require-*.ts`, `fixtures/m7/module-*.ts`

**DiagCode impact**:
- `require()` calls no longer cause UnresolvedFunction
- Built-in modules (fs, path) recognized and mapped
- File-relative requires tracked in manifest

**Coverage matrix delta**:
- test262 doesn't include module tests (CommonJS specific)
- TypeScript/typescript-go tests using require should show progress
- `unsupported` decreases for CommonJS patterns

## Validation before commit

```bash
cargo fmt --all --check
cargo test -q
cargo test -q --test m7_modules
# Verify manifest includes modules
./target/debug/ts2wasm-cli build fixtures/m7/require-builtin-fs.ts -o /tmp/t.wasm --emit-capabilities /tmp/cap.json
jq .modules /tmp/cap.json
```

## Gatekeeper checklist

✓ Module resolution deterministic (no randomness)
✓ Module cache prevents duplicate loads
✓ require() returns same object on multiple calls (same module)
✓ Built-in module stubs have correct exports
✓ File module linking doesn't break on circular requires
✓ Manifest lists all required modules with resolution paths
✓ No hardcoded module paths (use constants/linker)
✓ All test fixtures use same Node.js require semantics

## Design decisions

1. **Circular requires**: Return partially-initialized module (ES5 behavior); break cycles by not re-entering
2. **Built-in modules**: Stubs in WASM; actual implementations via host imports (Stream F)
3. **Module cache**: Simple map; no GC (module lifetime = program lifetime)
4. **require.resolve**: Not implemented (deferred)
5. **Dynamic requires**: Not supported (e.g., require(variableName))
6. **require() arguments**: String literals only; compile-time resolution

## References

- Current parser: `crates/cli/src/lib.rs` (line 900+ for expression parsing)
- Host imports: already defined in manifest (Stream F defines actual functions)
- Module resolution algorithm: Node.js convention (simplified subset)
