# Implement test262 test harness and host-defined functions

**Status**: done
**Created**: 2026-04-29
**ID**: 266
**Type**: feature
**Area**: tests/harness
**Priority**: P1
**Class**: implementation-ready

Problem: Current test262 runner (scripts/run/test262.py) doesn't load required test harness files or provide host-defined functions, causing tests to fail with "UnresolvedName: assert" errors. This prevents meaningful test262 execution since assert.sameValue and other test helpers are undefined.

Current state:
- Test262 tests fail immediately with `error: [UnresolvedName] unresolved name: 'assert'`
- No harness files (assert.js, sta.js) are loaded before test execution
- No host-defined functions (print, $262, etc.) are provided
- Tests cannot validate actual ECMAScript semantics

Test262 requirements (from INTERPRETING.md):
- Must load `harness/assert.js` and `harness/sta.js` in global scope before test execution
- Must provide host-defined functions:
  - `print` function for async test communication
  - `$262` object with: createRealm, detachArrayBuffer, evalScript, gc, global, IsHTMLDDA, agent
- Each test must run in isolated ECMAScript realm
- Must respect test metadata flags (module, raw, async, etc.)

Scope:

Implement test262 harness infrastructure:

1. Harness file loading:
   - Load `reference/test262/harness/assert.js` before test execution
   - Load `reference/test262/harness/sta.js` before test execution
   - Parse and include Test262Error constructor from assert.js
   - Handle conditional harness loading based on test flags (raw flag skips harness)

2. Host-defined function implementation:
   - Implement `print` function for test output
   - Implement `$262` object with required properties:
     - `createRealm` - create new ECMAScript realm
     - `detachArrayBuffer` - detach array buffer operation
     - `evalScript` - evaluate script as ECMAScript
     - `gc` - garbage collection trigger (may throw if unsupported)
     - `global` - reference to global object
     - `IsHTMLDDA` - HTML document.all-like object (conditional)
     - `agent` - concurrent agent testing (may be deferred)

3. Test metadata parsing:
   - Parse YAML frontmatter from test files
   - Handle flags: module, raw, async, onlyStrict, noStrict, IsHTMLDDA
   - Handle negative test phases: parse, early, resolution, runtime
   - Skip tests with unsupported features

4. Distinguish between:
   - JavaScript test helpers (assert.sameValue, assert.throws, etc.) - load from harness
   - Runtime-required functions (print, $262.createRealm, etc.) - implement in runtime
   - Built-in ECMAScript methods (Object.getOwnPropertyDescriptor, etc.) - should work via standard implementation

Out of scope:
- Full $262.agent concurrent testing (defer to follow-up issue)
- Complete IsHTMLDDA implementation (conditional support)
- All test262 harness files (start with core assert.js and sta.js)

Acceptance Criteria:

- test262.py loads harness files before test execution
- Host-defined functions are provided to test environment
- Tests with assert.sameValue, assert.throws, etc. execute without UnresolvedName errors
- Test metadata flags are respected (module, raw, async, etc.)
- Basic test262 tests can run and produce meaningful pass/fail results
- Test harness functions work correctly (assert.sameValue compares values properly)

Validation:

```sh
# Run sample of test262 tests without assert errors
mise run test262 -- --sample 10 --jobs 1

# Verify harness loading works
# Check that assert.sameValue tests actually compare values
# Verify Test262Error is thrown when assertions fail
cargo fmt --all --check
cargo nextest run
```

Reference:
- Test262 INTERPRETING.md: <https://github.com/tc39/test262/blob/main/INTERPRETING.md>
- Test262 harness files: reference/test262/harness/
- Current runner: scripts/run/test262.py

Completion evidence:
- `scripts/run/test262.py` parses test262 metadata, prepares harness-wrapped source, loads real `sta.js`/`assert.js` for the Node oracle, and provides a wasm-compatible core harness shim for the current compiler slice.
- Host-defined `print` and `$262` hooks are provided to prepared test sources; unsupported hooks fail with explicit `Test262Error` diagnostics or are classified by metadata.
- Negative metadata is handled as expected rejection, and unsupported `module`, `async`, and `IsHTMLDDA` tests are classified instead of run as ordinary failures.
- `assert.sameValue`/`assert.notSameValue` mismatches emit a runner sentinel so assertion failures are not silently counted as passes while exception exit behavior is still incomplete.

Validation run:
- `python -m py_compile scripts/run/test262.py`
- `python scripts/manager.py check scripts`
- `python scripts/manager.py check issues`
- `python scripts/manager.py test262 --sample 1 --jobs 1` -> `Pass: 9`, `Fail: 1`, `Unsupported: 18`, `Blocked: 0`
- `cargo fmt --all --check`
- `cargo nextest run` -> `501 passed`, `4 skipped`
