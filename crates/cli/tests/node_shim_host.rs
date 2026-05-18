use std::{fs, process::Command};

#[path = "common/capability.rs"]
mod capability;

use capability::node_command;
use ts2wasm_shared::test_helpers::{fixture_path, temp_wasm_path, unique_temp_dir};

#[test]
fn dynamic_function_handles_execute_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n");
}

#[test]
fn dynamic_function_handle_returns_string_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-string-node-shim.ts";
    assert_node_shim_stdout(fixture, "dynamic-string\n");
}

#[test]
fn dynamic_function_handle_returns_object_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-object-node-shim.ts";
    assert_node_shim_stdout(fixture, "[object Object]\n");
}

#[test]
fn dynamic_function_handle_preserves_object_properties_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-object-properties-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\nok\nundefined\n");
}

#[test]
fn dynamic_function_handle_bridges_function_properties_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-function-property-node-shim.ts";
    assert_node_shim_stdout(fixture, "2\nhostCallback\n[object Object]\nundefined\n");
}

#[test]
fn dynamic_function_handle_calls_function_properties_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-function-property-call-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n7\n");
}

#[test]
fn dynamic_function_handle_binds_this_for_function_properties_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-function-property-this-node-shim.ts";
    assert_node_shim_stdout(fixture, "9\n");
}

#[test]
fn dynamic_function_handle_preserves_object_identity_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-object-identity-node-shim.ts";
    assert_node_shim_stdout(fixture, "true\n7\n");
}

#[test]
fn dynamic_function_handle_refreshes_object_properties_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-object-mutation-node-shim.ts";
    assert_node_shim_stdout(fixture, "2\ntrue\n3\n3\n");
}

#[test]
fn dynamic_function_handle_tracks_object_shape_changes_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-object-shape-node-shim.ts";
    assert_node_shim_stdout(fixture, "1\ntrue\n2\nok\nundefined\n");
}

#[test]
fn dynamic_function_handle_grows_object_shape_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-object-growth-node-shim.ts";
    assert_node_shim_stdout(fixture, "1\n5\nundefined\n");
}

#[test]
fn dynamic_function_handle_bridges_nested_arrays_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-nested-array-node-shim.ts";
    assert_node_shim_stdout(fixture, "2\n7\n8\nundefined\n");
}

#[test]
fn dynamic_function_handle_grows_nested_arrays_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-array-growth-node-shim.ts";
    assert_node_shim_stdout(fixture, "4\n5\n5\nundefined\n");
}

#[test]
fn dynamic_function_handle_exposes_metadata_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-metadata-node-shim.ts";
    assert_node_shim_stdout(fixture, "2\nanonymous\n[object Object]\n7\n");
}

#[test]
fn dynamic_function_construct_returns_object_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-construct-object-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\nok\nundefined\n");
}

#[test]
fn dynamic_indirect_eval_executes_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/indirect-eval-dynamic-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n");
}

#[test]
fn dynamic_optional_eval_executes_as_indirect_eval_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/optional-eval-dynamic-node-shim.ts";
    assert_node_shim_stdout(fixture, "optional-eval\n");
}

#[test]
fn dynamic_indirect_eval_preserves_object_properties_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/indirect-eval-dynamic-object-properties-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\nok\nundefined\n");
}

#[test]
fn dynamic_direct_eval_executes_through_node_shim_host_import() {
    let fixture = "fixtures/builtins-and-io/dynamic-eval-host-path.ts";
    assert_node_shim_stdout(fixture, "3\n");
}

#[test]
fn dynamic_direct_eval_writes_back_local_env_cell_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-local-writeback-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n7\n");
}

#[test]
fn dynamic_direct_eval_writes_back_parameter_env_cell_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-param-writeback-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n7\n");
}

#[test]
fn dynamic_direct_eval_writes_back_shadowed_block_env_cell_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-block-shadow-writeback-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n7\n1\n");
}

#[test]
fn dynamic_direct_eval_writes_back_string_env_cell_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-string-writeback-node-shim.ts";
    assert_node_shim_stdout(fixture, "after\nafter\n");
}

#[test]
fn dynamic_direct_eval_returns_object_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-object-result-node-shim.ts";
    assert_node_shim_stdout(fixture, "[object Object]\n");
}

#[test]
fn dynamic_direct_eval_preserves_object_properties_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-object-properties-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\nok\nundefined\n");
}

#[test]
fn dynamic_direct_eval_strict_lexical_shadow_does_not_write_back_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-strict-lexical-shadow-node-shim.ts";
    assert_node_shim_stdout(fixture, "2\n1\n");
}

#[test]
fn dynamic_direct_eval_rejects_tdz_env_descriptor_conflict() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-tdz-conflict-unsupported.ts";
    assert_build_fails_with(fixture, "UnsupportedEval", "TDZ-aware env descriptors");
}

fn assert_build_fails_with(fixture: &str, expected_code: &str, expected_message: &str) {
    let fixture_path = fixture_path(fixture);
    let output_wasm = temp_wasm_path(fixture);

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output_wasm)
        .output()
        .expect("failed to execute ts2wasm build");

    assert!(
        !build.status.success(),
        "{fixture} should fail to build but succeeded\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let stderr = String::from_utf8_lossy(&build.stderr);
    assert!(
        stderr.contains(expected_code),
        "expected {expected_code} diagnostic for {fixture}, got:\n{stderr}"
    );
    assert!(
        stderr.contains(expected_message),
        "expected diagnostic message containing {expected_message:?} for {fixture}, got:\n{stderr}"
    );
}

fn assert_node_shim_stdout(fixture: &str, expected_stdout: &str) {
    let fixture_path = fixture_path(fixture);
    let output_wasm = temp_wasm_path(fixture);

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output_wasm)
        .output()
        .expect("failed to execute ts2wasm build");

    assert!(
        build.status.success(),
        "{fixture} should build for node-shim execution\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let runner_dir = unique_temp_dir("node-shim-host");
    let runner = runner_dir.join("runner.mjs");
    fs::write(&runner, NODE_SHIM_RUNNER).expect("failed to write node shim runner");

    let node = node_command()
        .arg(&runner)
        .arg(&output_wasm)
        .output()
        .expect("failed to execute node shim runner");

    assert!(
        node.status.success(),
        "node shim runner should execute {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&node.stdout),
        String::from_utf8_lossy(&node.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&node.stdout), expected_stdout);
}

const NODE_SHIM_RUNNER: &str = r#"
import fs from 'node:fs';

const wasmPath = process.argv[process.argv.length - 1];
const wasmBytes = fs.readFileSync(wasmPath);

const TAG_UNDEFINED = 0;
const TAG_NULL = 1;
const TAG_FALSE = 2;
const TAG_TRUE = 3;
const TAG_NUMBER = 4;
const TAG_ARRAY = 5;
const TAG_STRING = 6;
const TAG_OBJECT = 7;
const TAG_MASK = 7;
const HEAP_MASK = -8;
const ARRAY_HEADER_SIZE = 20;
const ARRAY_PRESENCE_WORDS_OFFSET = 16;
const OBJECT_HEADER_SIZE = 12;
const OBJECT_ENTRIES_OFFSET = 12;
const OBJECT_ENTRY_SIZE = 8;
const GC_HEADER_SIZE = 16;
const GC_FLAGS_AND_TYPE_OFFSET = 0;
const GC_BODY_SIZE_OFFSET = 4;
const GC_KIND_ARRAY = 8;
const GC_KIND_OBJECT = 12;

let memory;
const hostFunctions = [];
const hostFunctionHandles = new Map();
const hostFunctionHandleValues = new WeakMap();
const hostArrayHandles = new WeakMap();
const hostObjectHandles = new WeakMap();
const hostObjectValues = new Map();
const decoder = new TextDecoder();
const encoder = new TextEncoder();
let stdout = '';
let hostHeapCursor = 0;

function view() {
  return new DataView(memory.buffer);
}

function bytes() {
  return new Uint8Array(memory.buffer);
}

function rawTag(raw) {
  return raw & TAG_MASK;
}

function rawPtr(raw) {
  return raw & HEAP_MASK;
}

function decodeString(raw) {
  if (rawTag(raw) !== TAG_STRING) {
    throw new TypeError(`expected string RawValue, got ${raw}`);
  }
  const base = rawPtr(raw);
  const len = view().getInt32(base, true);
  return decoder.decode(bytes().subarray(base + 4, base + 4 + len));
}

function decodeArray(raw) {
  if (rawTag(raw) !== TAG_ARRAY) {
    throw new TypeError(`expected array RawValue, got ${raw}`);
  }
  const base = rawPtr(raw);
  const len = view().getInt32(base, true);
  const presenceWords = view().getInt32(base + 8, true);
  const elementsOffset = view().getInt32(base + 12, true);
  const values = [];
  for (let i = 0; i < len; i += 1) {
    const wordIndex = i >> 5;
    const bitIndex = i & 31;
    const present =
      wordIndex < presenceWords &&
      (view().getUint32(base + ARRAY_PRESENCE_WORDS_OFFSET + wordIndex * 4, true) &
        (1 << bitIndex)) !==
        0;
    values.push(present ? view().getInt32(base + elementsOffset + i * 4, true) : TAG_UNDEFINED);
  }
  return values;
}

function decodeValue(raw) {
  switch (rawTag(raw)) {
    case TAG_UNDEFINED:
      return undefined;
    case TAG_NULL:
      return null;
    case TAG_FALSE:
      return false;
    case TAG_TRUE:
      return true;
    case TAG_NUMBER:
      return raw >> 3;
    case TAG_STRING:
      return decodeString(raw);
    default:
      throw new TypeError(`unsupported RawValue for this host-shim test: ${raw}`);
  }
}

function readEnvCellRaw(cellRaw) {
  if (rawTag(cellRaw) !== TAG_ARRAY) {
    throw new TypeError(`expected env cell array RawValue, got ${cellRaw}`);
  }
  const base = rawPtr(cellRaw);
  return view().getInt32(base + ARRAY_HEADER_SIZE, true);
}

function writeEnvCellRaw(cellRaw, valueRaw) {
  if (rawTag(cellRaw) !== TAG_ARRAY) {
    throw new TypeError(`expected env cell array RawValue, got ${cellRaw}`);
  }
  const base = rawPtr(cellRaw);
  view().setInt32(base + ARRAY_HEADER_SIZE, valueRaw, true);
}

function alignHostPtr(ptr) {
  return Math.ceil(ptr / 8) * 8;
}

function hostAlloc(size) {
  if (hostHeapCursor === 0) {
    hostHeapCursor = bytes().byteLength;
  }
  const ptr = alignHostPtr(hostHeapCursor);
  const end = ptr + size;
  while (end > bytes().byteLength) {
    memory.grow(1);
  }
  hostHeapCursor = end;
  return ptr;
}

function encodeString(value) {
  const data = encoder.encode(value);
  const ptr = hostAlloc(4 + data.length);
  view().setInt32(ptr, data.length, true);
  bytes().set(data, ptr + 4);
  return ptr | TAG_STRING;
}

function refreshHostArrayEntries(value, record) {
  if (value.length > record.capacity) {
    throw new TypeError('internal host array capacity mismatch for this test');
  }
  const ptr = rawPtr(record.raw);
  view().setInt32(ptr, value.length, true);
  for (let word = 0; word < record.presenceWords; word += 1) {
    view().setUint32(ptr + ARRAY_PRESENCE_WORDS_OFFSET + word * 4, 0, true);
  }
  for (let i = 0; i < value.length; i += 1) {
    const entry = ptr + record.elementsOffset + i * 4;
    if (Object.prototype.hasOwnProperty.call(value, i)) {
      const word = i >> 5;
      const bit = i & 31;
      const maskOffset = ptr + ARRAY_PRESENCE_WORDS_OFFSET + word * 4;
      const mask = view().getUint32(maskOffset, true) | (1 << bit);
      view().setUint32(maskOffset, mask, true);
      view().setInt32(entry, encodeHostValue(value[i]), true);
    } else {
      view().setInt32(entry, TAG_UNDEFINED, true);
    }
  }
}

function allocateHostArrayRecord(value, requestedCapacity) {
  const capacity = Math.max(value.length, requestedCapacity, 4);
  const presenceWords = Math.max(1, Math.ceil(capacity / 32));
  const elementsOffset = ARRAY_PRESENCE_WORDS_OFFSET + presenceWords * 4;
  const size = elementsOffset + capacity * 4;
  const base = hostAlloc(GC_HEADER_SIZE + size);
  view().setInt32(base + GC_FLAGS_AND_TYPE_OFFSET, GC_KIND_ARRAY, true);
  view().setInt32(base + GC_BODY_SIZE_OFFSET, size, true);
  const ptr = base + GC_HEADER_SIZE;
  view().setInt32(ptr + 4, capacity, true);
  view().setInt32(ptr + 8, presenceWords, true);
  view().setInt32(ptr + 12, elementsOffset, true);
  const raw = ptr | TAG_ARRAY;
  const record = { raw, capacity, presenceWords, elementsOffset };
  hostArrayHandles.set(value, record);
  return record;
}

function encodeHostArray(value) {
  let record = hostArrayHandles.get(value);
  if (record === undefined) {
    record = allocateHostArrayRecord(value, value.length);
  } else if (value.length > record.capacity) {
    record = allocateHostArrayRecord(value, record.capacity * 2);
  }
  refreshHostArrayEntries(value, record);
  return record.raw;
}

function refreshHostObjectEntries(value, record) {
  const keys = Object.keys(value);
  if (keys.length > record.capacity) {
    throw new TypeError('internal host object capacity mismatch for this test');
  }
  const ptr = rawPtr(record.raw);
  view().setInt32(ptr, keys.length, true);
  for (let i = 0; i < keys.length; i += 1) {
    const entry = ptr + OBJECT_ENTRIES_OFFSET + i * OBJECT_ENTRY_SIZE;
    view().setInt32(entry, encodeString(keys[i]), true);
    view().setInt32(entry + 4, encodeHostValue(value[keys[i]]), true);
  }
  record.keys = keys;
}

function allocateHostObjectRecord(value, requestedCapacity) {
  const capacity = Math.max(Object.keys(value).length, requestedCapacity, 4);
  const size = OBJECT_HEADER_SIZE + capacity * OBJECT_ENTRY_SIZE;
  const base = hostAlloc(GC_HEADER_SIZE + size);
  view().setInt32(base + GC_FLAGS_AND_TYPE_OFFSET, GC_KIND_OBJECT, true);
  view().setInt32(base + GC_BODY_SIZE_OFFSET, size, true);
  const ptr = base + GC_HEADER_SIZE;
  view().setInt32(ptr, 0, true);
  view().setInt32(ptr + 4, 0, true);
  view().setInt32(ptr + 8, 0, true);
  const raw = ptr | TAG_OBJECT;
  const record = { raw, keys: [], capacity };
  hostObjectHandles.set(value, record);
  hostObjectValues.set(ptr, value);
  return record;
}

function encodeHostObject(value) {
  const keys = Object.keys(value);
  let record = hostObjectHandles.get(value);
  if (record === undefined) {
    record = allocateHostObjectRecord(value, keys.length);
  } else if (keys.length > record.capacity) {
    record = allocateHostObjectRecord(value, record.capacity * 2);
  }
  refreshHostObjectEntries(value, record);
  return record.raw;
}

function encodeHostFunctionHandle(fn, index) {
  const raw = encodeHostObject({
    length: fn.length,
    name: fn.name,
    prototype: {},
  });
  hostFunctionHandles.set(rawPtr(raw), index);
  return raw;
}

function encodeHostFunctionValue(fn) {
  const existing = hostFunctionHandleValues.get(fn);
  if (existing !== undefined) return existing;
  hostFunctions.push(fn);
  const raw = encodeHostFunctionHandle(fn, hostFunctions.length - 1);
  hostFunctionHandleValues.set(fn, raw);
  return raw;
}

function decodeHostFunctionHandle(raw) {
  if (rawTag(raw) !== TAG_OBJECT) {
    throw new TypeError(`expected host function handle object RawValue, got ${raw}`);
  }
  const ptr = rawPtr(raw);
  if (!hostFunctionHandles.has(ptr)) {
    throw new TypeError(`unknown host function handle object: ${raw}`);
  }
  return hostFunctionHandles.get(ptr);
}

function decodeHostReceiver(raw) {
  if (rawTag(raw) === TAG_OBJECT) {
    const ptr = rawPtr(raw);
    if (hostObjectValues.has(ptr)) return hostObjectValues.get(ptr);
  }
  return decodeValue(raw);
}

function encodeHostValue(value) {
  if (value === undefined) return TAG_UNDEFINED;
  if (value === null) return TAG_NULL;
  if (value === false) return TAG_FALSE;
  if (value === true) return TAG_TRUE;
  if (Number.isInteger(value)) return (value << 3) | TAG_NUMBER;
  if (typeof value === 'string') return encodeString(value);
  if (Array.isArray(value)) return encodeHostArray(value);
  if (typeof value === 'object') return encodeHostObject(value);
  if (typeof value === 'function') return encodeHostFunctionValue(value);
  throw new TypeError(`unsupported host return value for this test: ${String(value)}`);
}

function uniqueInternalName(base, names) {
  let name = base;
  while (names.includes(name)) {
    name = `_${name}`;
  }
  return name;
}

function evalWithEnvDescriptor(source, envRaw) {
  if (envRaw === TAG_UNDEFINED) {
    return eval(source);
  }

  const pairs = decodeArray(envRaw);
  const bindings = [];
  for (let i = 0; i < pairs.length; i += 2) {
    const name = decodeString(pairs[i]);
    const cellRaw = pairs[i + 1];
    const raw = readEnvCellRaw(cellRaw);
    bindings.push({ name, cellRaw, raw, value: decodeValue(raw) });
  }

  const names = bindings.map((binding) => binding.name);
  const sourceName = uniqueInternalName('__ts2wasm_eval_source', names);
  const resultName = uniqueInternalName('__ts2wasm_eval_result', [...names, sourceName]);
  const wrapper = Function(
    sourceName,
    ...names,
    `let ${resultName} = eval(${sourceName}); return [${resultName}, ${names.join(', ')}];`,
  );
  const values = bindings.map((binding) => binding.value);
  const [result, ...updatedValues] = wrapper(source, ...values);

  for (let i = 0; i < bindings.length; i += 1) {
    if (!Object.is(bindings[i].value, updatedValues[i])) {
      writeEnvCellRaw(bindings[i].cellRaw, encodeHostValue(updatedValues[i]));
    }
  }

  return result;
}

function decodeArgs(raw) {
  return decodeArray(raw).map(decodeValue);
}

const imports = {
  wasi_snapshot_preview1: {
    fd_write(fd, iovs, iovsLen, nwritten) {
      if (fd !== 1) return 8;
      let written = 0;
      for (let i = 0; i < iovsLen; i += 1) {
        const iov = iovs + i * 8;
        const ptr = view().getInt32(iov, true);
        const len = view().getInt32(iov + 4, true);
        stdout += decoder.decode(bytes().subarray(ptr, ptr + len));
        written += len;
      }
      view().setInt32(nwritten, written, true);
      return 0;
    },
    proc_exit(code) {
      throw Object.assign(new Error(`proc_exit(${code})`), { code });
    },
  },
  host: {
    'eval.direct'(sourceRaw, envRaw) {
      const result = evalWithEnvDescriptor(decodeString(sourceRaw), envRaw);
      return encodeHostValue(result);
    },
    'eval.indirect'(sourceRaw, _envRaw) {
      const result = globalThis.eval(decodeString(sourceRaw));
      return encodeHostValue(result);
    },
    'function.compile'(argsRaw) {
      const args = decodeArgs(argsRaw);
      const fn = Function(...args);
      return encodeHostFunctionValue(fn);
    },
    'function.call'(handleRaw, argsRaw) {
      const fn = hostFunctions[decodeHostFunctionHandle(handleRaw)];
      if (typeof fn !== 'function') {
        throw new TypeError(`unknown host function handle: ${handleRaw}`);
      }
      return encodeHostValue(fn(...decodeArgs(argsRaw)));
    },
    'function.callMethod'(handleRaw, receiverRaw, argsRaw) {
      const fn = hostFunctions[decodeHostFunctionHandle(handleRaw)];
      if (typeof fn !== 'function') {
        throw new TypeError(`unknown host function handle: ${handleRaw}`);
      }
      return encodeHostValue(fn.apply(decodeHostReceiver(receiverRaw), decodeArgs(argsRaw)));
    },
    'function.construct'(handleRaw, argsRaw) {
      const fn = hostFunctions[decodeHostFunctionHandle(handleRaw)];
      if (typeof fn !== 'function') {
        throw new TypeError(`unknown host function handle: ${handleRaw}`);
      }
      return encodeHostValue(Reflect.construct(fn, decodeArgs(argsRaw)));
    },
  },
};

try {
  const { instance } = await WebAssembly.instantiate(wasmBytes, imports);
  memory = instance.exports.memory;
  instance.exports._start();
} catch (error) {
  if (error && error.code === 0) {
    process.stdout.write(stdout);
    process.exit(0);
  }
  throw error;
}

process.stdout.write(stdout);
"#;
