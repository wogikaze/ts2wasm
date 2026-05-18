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
fn dynamic_indirect_eval_executes_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/indirect-eval-dynamic-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n");
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
const TAG_MASK = 7;
const HEAP_MASK = -8;
const ARRAY_HEADER_SIZE = 20;
const ARRAY_PRESENCE_WORDS_OFFSET = 16;

let memory;
const hostFunctions = [];
const decoder = new TextDecoder();
let stdout = '';

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

function encodePrimitive(value) {
  if (value === undefined) return TAG_UNDEFINED;
  if (value === null) return TAG_NULL;
  if (value === false) return TAG_FALSE;
  if (value === true) return TAG_TRUE;
  if (Number.isInteger(value)) return (value << 3) | TAG_NUMBER;
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
      writeEnvCellRaw(bindings[i].cellRaw, encodePrimitive(updatedValues[i]));
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
      return encodePrimitive(result);
    },
    'eval.indirect'(sourceRaw, _envRaw) {
      const result = globalThis.eval(decodeString(sourceRaw));
      return encodePrimitive(result);
    },
    'function.compile'(argsRaw) {
      const args = decodeArgs(argsRaw);
      const fn = Function(...args);
      hostFunctions.push(fn);
      return ((hostFunctions.length - 1) << 3) | TAG_NUMBER;
    },
    'function.call'(handleRaw, argsRaw) {
      const fn = hostFunctions[decodeValue(handleRaw)];
      if (typeof fn !== 'function') {
        throw new TypeError(`unknown host function handle: ${handleRaw}`);
      }
      return encodePrimitive(fn(...decodeArgs(argsRaw)));
    },
    'function.construct'(handleRaw, argsRaw) {
      const fn = hostFunctions[decodeValue(handleRaw)];
      if (typeof fn !== 'function') {
        throw new TypeError(`unknown host function handle: ${handleRaw}`);
      }
      Reflect.construct(fn, decodeArgs(argsRaw));
      return TAG_UNDEFINED;
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
