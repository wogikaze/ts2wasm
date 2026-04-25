---
name: m10-stream-f
description: Use when implementing M10 Stream F Node.js API integration through manifest-driven host imports for fs, process, path, util, crypto, and related capability documentation in ts2wasm.
---

# Stream F: Node.js API Integration

## Goal
Implement manifest-driven host imports for fs, process, path, util, and crypto.
This achieves M10: "Node host が必要な API を manifest 付きで実行できる"

## Scope (1-2 hour window)

Implement host import wrappers for:
1. **fs**: readFileSync, writeFileSync, appendFileSync
2. **process**: process.argv, process.env, process.exit
3. **path**: path.join, path.resolve, path.basename, path.dirname
4. **util**: util.format, util.inspect (optional)
5. **crypto**: crypto.randomBytes, crypto.createHash (optional)

Not yet: async versions (readFile, writeFile), streams, fs.watch, process.on/off.

## Implementation strategy

### Phase 1: Extend capability manifest (10 min)

Update CapabilityManifest to include node_host imports:
```rust
#[derive(Serialize)]
pub struct CapabilityManifest {
    pub standalone: bool,
    pub wasi: WasiCapability,
    pub node_host: NodeHostCapability,
}

#[derive(Serialize)]
pub struct NodeHostCapability {
    pub required: bool,
    pub imports: Vec<String>,  // ["host.fs.readFileSync", ...]
}
```

### Phase 2: Host import function signatures (10 min)

Define WAT function imports:
```wasm
(import "host" "fs.readFileSync" 
  (func $host_fs_readFileSync (param $path i32) (param $encoding i32) (result i32)))

(import "host" "process.argv"
  (func $host_process_argv (result i32)))  ;; returns array

(import "host" "process.exit"
  (func $host_process_exit (param $code i32)))
```

Map to manifest: each import → CapabilityManifest entry.

### Phase 3: Module wrappers for built-in modules (20 min)

When require("fs") is called, return object with methods:
```wasm
(func $require_fs (result i32)
  (local $fs i32)
  (local.set $fs (call $alloc_heap (i32.const 128)))
  
  ;; fs.readFileSync = wrapper function
  (call $object_set_property
    (local.get $fs)
    "readFileSync"
    (func.ref $fs_readFileSync_wrapper))
  
  ;; ... other properties ...
  
  (local.get $fs)
)
```

### Phase 4: Method wrappers (20 min)

Each method wraps host import with type conversions:
```wasm
;; fs.readFileSync(path, encoding) → string
(func $fs_readFileSync_wrapper (param $path i32) (param $encoding i32) (result i32)
  ;; $path is i32 (encoded value)
  ;; Convert to host string format (extract from heap)
  (local $path_str_ptr i32)
  (local.set $path_str_ptr (i32.and (local.get $path) (i32.const -8)))  ;; strip tag
  
  ;; Call host import
  (local $result_str (call $host_fs_readFileSync 
    (local.get $path_str_ptr)
    (call $value_to_string (local.get $encoding))))
  
  ;; Convert result back to JS string value (heap + tag)
  (call $string_value (local.get $result_str))
)

;; process.argv getter
(func $process_argv_wrapper (result i32)
  ;; Call host import
  (local $array (call $host_process_argv))
  ;; Return as JS array value
  (local.get $array)
)
```

### Phase 5: process object (10 min)

process is special (not returned by require, but global object):
```wasm
(global $process_argv i32 (i32.const 0))

;; On module init:
(call $init_process_globals)

(func $init_process_globals
  ;; argv = host_process_argv()
  (global.set $process_argv (call $host_process_argv))
  ;; env = host_process_env()
  ;; etc.
)
```

Access patterns:
- `process.argv` → read global
- `process.exit(1)` → call host import

### Phase 6: path helpers (15 min)

Implement helpers (some in WAT, some host imports):
```wasm
;; path.join("a", "b", "c") → "a/b/c"
(func $path_join (param $arg1 i32) (param $arg2 i32) (param $arg3 i32) (result i32)
  ;; Take all arguments, concatenate with "/"
  ;; Use existing string concat logic
)

;; path.resolve(path) → absolute path
(func $path_resolve (param $path i32) (result i32)
  ;; Call host import
  (call $host_path_resolve (local.get $path))
)
```

### Phase 7: Error handling (10 min)

Host imports may fail (file not found, etc.):
```wasm
;; Return JS Error object (special tagged value)
;; Or: throw exception (propagate as exception value)
;; Decision: return error string in exception tag for now
```

### Phase 8: Tests (10 min)

Fixtures:
1. `fs-read.ts`: fs.readFileSync("file.txt", "utf8")
2. `fs-write.ts`: fs.writeFileSync("out.txt", "content")
3. `process-argv.ts`: process.argv iteration
4. `process-env.ts`: process.env.PATH access
5. `path-join.ts`: path.join("dir", "file.ts")
6. `path-resolve.ts`: path.resolve("./relative")

All fixtures assume files/environment available at runtime.

## Output

**Commits**:
1. `manifest: extend capability schema with node_host imports`
2. `backend: add host import function signatures to WAT`
3. `backend: implement fs module wrapper (readFileSync, writeFileSync, appendFileSync)`
4. `backend: implement process global object`
5. `backend: implement path helper functions`
6. `backend: add host import detection to manifest emission`
7. `tests: add Node.js API integration tests (M10 gate)`

**Tests added**:
- `crates/cli/tests/m10_node_apis.rs`
- Fixture files: `fixtures/m10/fs-*.ts`, `fixtures/m10/process-*.ts`, `fixtures/m10/path-*.ts`

**DiagCode impact**:
- `require("fs")`, `require("process")` now resolve correctly
- Methods like `fs.readFileSync` no longer cause UnresolvedFunction
- Manifest includes host.fs.*, host.process.*, host.path.* imports

**Coverage matrix delta**:
- TypeScript/test262 tests using Node APIs should show progress
- `unsupported` decreases for CommonJS+fs patterns
- `pass` increases for M10 fixture set

## Validation before commit

```bash
cargo fmt --all --check
cargo test -q
cargo test -q --test m10_node_apis
# Verify manifest shows node_host imports
./target/debug/ts2wasm-cli build fixtures/m10/fs-read.ts -o /tmp/t.wasm --emit-capabilities /tmp/cap.json
jq .node_host /tmp/cap.json
# Should show: standalone=false, required=true, imports=[...]
```

## Gatekeeper checklist

✓ All host imports declared in WAT module import section
✓ Host import names match manifest entries
✓ Manifest set standalone=false only if node_host.required=true
✓ All host imports must be explicitly imported (no implicit dependencies)
✓ Type conversions between JS values and host strings correct
✓ Error conditions return JS exception or throw
✓ process.exit actually terminates (calls host import)
✓ path functions produce correct file paths
✓ All test fixtures produce same output with Node.js reference

## Design decisions

1. **Async APIs**: Deferred (Promise/async/await not yet implemented)
2. **Error handling**: Return exception values (no try-catch needed for host failures)
3. **File paths**: Relative to current working directory (no special handling)
4. **process.env**: Snapshot at module load (doesn't reflect runtime changes)
5. **path.resolve**: Use Node.js path.resolve semantics (via host import if complex)
6. **Streams**: Not supported (fs.createReadStream, process.stdin/stdout)

## M10 Gate

Completion of Stream F enables:
- ✓ Generated WASM requires Node.js host to execute
- ✓ Manifest lists all required APIs with reasons
- ✓ TypeScript files using fs, process, path compile and run
- ✓ Capability audit: know what privileges code requires

M10 success means:
```bash
$ ts2wasm build my-app.ts -o app.wasm --emit-capabilities app.cap.json
$ node app-runner.js  # custom host that imports required functions
# Output matches: node my-app.ts
```

## References

- Capability manifest schema: `docs/11-shared-definitions.md`
- Host import calling convention: `docs/04-compiler-architecture-and-runtime.md`
- Existing require() implementation: Stream E (SKILL.md)
- Process global setup: patterns from M6 (stdin/stdout)
