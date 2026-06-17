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
const HOST_EXCEPTION_ARRAY_CAPACITY = -2;
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
const refreshingHostObjects = new WeakSet();
const directEvalExtraBindings = new Map();
const EVAL_DESCRIPTOR_VERSION = '__ts2wasm_eval_descriptor_v2';
const EVAL_DESCRIPTOR_CALLER_STRICT = '__ts2wasm_eval_caller_strict';
const EVAL_DESCRIPTOR_BINDINGS = '__ts2wasm_eval_bindings';
const DIRECT_EVAL_MUTATION_LEDGER_VERSION = 1;
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
  const base = rawPtr(resolveHostArrayRaw(raw));
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

function decodeObject(raw) {
  if (rawTag(raw) !== TAG_OBJECT) {
    throw new TypeError(`expected object RawValue, got ${raw}`);
  }
  const ptr = rawPtr(raw);
  if (hostObjectValues.has(ptr)) return hostObjectValues.get(ptr);
  const forwarded = view().getInt32(ptr + 8, true);
  if (forwarded !== 0 && view().getInt32(ptr, true) === 0 && view().getInt32(ptr + 4, true) === 0) {
    return decodeObject(forwarded | TAG_OBJECT);
  }

  const len = view().getInt32(ptr, true);
  const object = {};
  for (let i = 0; i < len; i += 1) {
    const entry = ptr + OBJECT_ENTRIES_OFFSET + i * OBJECT_ENTRY_SIZE;
    object[decodeString(view().getInt32(entry, true))] = decodeValue(
      view().getInt32(entry + 4, true),
    );
  }
  return object;
}

function decodeValue(raw) {
  switch (rawTag(raw)) {
    case TAG_UNDEFINED:
      if (raw !== TAG_UNDEFINED && raw >= 0 && raw + 4 <= bytes().byteLength) {
        const len = view().getInt32(raw, true);
        if (len >= 0 && raw + 4 + len <= bytes().byteLength) {
          return decoder.decode(bytes().subarray(raw + 4, raw + 4 + len));
        }
      }
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
    case TAG_ARRAY:
      return decodeArray(raw).map(decodeValue);
    case TAG_OBJECT: {
      const ptr = rawPtr(raw);
      if (hostFunctionHandles.has(ptr)) {
        return hostFunctions[hostFunctionHandles.get(ptr)];
      }
      return decodeObject(raw);
    }
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

function resolveHostArrayRaw(raw) {
  let current = raw;
  for (let steps = 0; steps < 64; steps += 1) {
    const ptr = rawPtr(current);
    if (view().getInt32(ptr + 4, true) !== -1) {
      return current;
    }
    current = view().getInt32(ptr + ARRAY_HEADER_SIZE, true);
  }
  throw new TypeError('host array forwarding cycle');
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

function forwardHostArrayRecord(from, to) {
  const fromPtr = rawPtr(from.raw);
  view().setInt32(fromPtr, 1, true);
  view().setInt32(fromPtr + 4, -1, true);
  view().setInt32(fromPtr + 8, 1, true);
  view().setInt32(fromPtr + 12, ARRAY_HEADER_SIZE, true);
  view().setUint32(fromPtr + ARRAY_PRESENCE_WORDS_OFFSET, 1, true);
  view().setInt32(fromPtr + ARRAY_HEADER_SIZE, to.raw, true);
}

function encodeHostArray(value) {
  let record = hostArrayHandles.get(value);
  if (record === undefined) {
    record = allocateHostArrayRecord(value, value.length);
  } else if (value.length > record.capacity) {
    const previous = record;
    record = allocateHostArrayRecord(value, record.capacity * 2);
    forwardHostArrayRecord(previous, record);
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

function forwardHostObjectRecord(from, to) {
  const fromPtr = rawPtr(from.raw);
  const toPtr = rawPtr(to.raw);
  view().setInt32(fromPtr, 0, true);
  view().setInt32(fromPtr + 4, 0, true);
  view().setInt32(fromPtr + 8, toPtr, true);
}

function encodeHostObject(value) {
  const keys = Object.keys(value);
  let record = hostObjectHandles.get(value);
  if (record === undefined) {
    record = allocateHostObjectRecord(value, keys.length);
  } else if (keys.length > record.capacity) {
    const previous = record;
    record = allocateHostObjectRecord(value, record.capacity * 2);
    forwardHostObjectRecord(previous, record);
  }
  if (refreshingHostObjects.has(value)) return record.raw;
  refreshingHostObjects.add(value);
  try {
    refreshHostObjectEntries(value, record);
  } finally {
    refreshingHostObjects.delete(value);
  }
  return record.raw;
}

function encodeHostFunctionHandle(fn, index) {
  const handleObject = { length: fn.length, name: fn.name };
  handleObject.prototype = { constructor: handleObject };
  if (fn.__ts2wasm_host_function_to_string !== true) {
    const toStringFn = function toString() {
      return fn.toString();
    };
    Object.defineProperty(toStringFn, '__ts2wasm_host_function_to_string', { value: true });
    handleObject.toString = toStringFn;
  }
  const raw = encodeHostObject(handleObject);
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

function encodeHostException(error) {
  const errorObject = error !== null && typeof error === 'object' ? { ...error } : {};
  if (typeof errorObject.name !== 'string') {
    errorObject.name = error && typeof error.name === 'string' ? error.name : 'Error';
  }
  if (typeof errorObject.message !== 'string') {
    errorObject.message =
      error && typeof error.message === 'string' ? error.message : String(error);
  }
  const errorRaw = encodeHostObject(errorObject);
  const base = hostAlloc(GC_HEADER_SIZE + ARRAY_HEADER_SIZE + 4);
  view().setInt32(base + GC_FLAGS_AND_TYPE_OFFSET, GC_KIND_ARRAY, true);
  view().setInt32(base + GC_BODY_SIZE_OFFSET, ARRAY_HEADER_SIZE + 4, true);
  const ptr = base + GC_HEADER_SIZE;
  view().setInt32(ptr, 1, true);
  view().setInt32(ptr + 4, HOST_EXCEPTION_ARRAY_CAPACITY, true);
  view().setInt32(ptr + 8, 1, true);
  view().setInt32(ptr + 12, ARRAY_HEADER_SIZE, true);
  view().setUint32(ptr + ARRAY_PRESENCE_WORDS_OFFSET, 1, true);
  view().setInt32(ptr + ARRAY_HEADER_SIZE, errorRaw, true);
  return ptr | TAG_ARRAY;
}

function uniqueInternalName(base, names) {
  let name = base;
  while (names.includes(name)) {
    name = `_${name}`;
  }
  return name;
}

function isIdentifierStart(ch) {
  return /[A-Za-z_$]/.test(ch);
}

function isIdentifierPart(ch) {
  return /[0-9A-Za-z_$]/.test(ch);
}

function isIdentifierBoundary(source, start, end) {
  return (
    (start === 0 || !isIdentifierPart(source[start - 1])) &&
    (end >= source.length || !isIdentifierPart(source[end]))
  );
}

function skipWhitespace(text, index) {
  let i = index;
  while (i < text.length && /\s/.test(text[i])) i += 1;
  return i;
}

function splitTopLevelComma(text) {
  const parts = [];
  let start = 0;
  let depth = 0;
  let quote = null;
  for (let i = 0; i < text.length; i += 1) {
    const ch = text[i];
    if (quote !== null) {
      if (ch === '\\') i += 1;
      else if (ch === quote) quote = null;
      continue;
    }
    if (ch === '"' || ch === "'" || ch === '`') {
      quote = ch;
    } else if (ch === '{' || ch === '[' || ch === '(') {
      depth += 1;
    } else if (ch === '}' || ch === ']' || ch === ')') {
      depth -= 1;
    } else if (ch === ',' && depth === 0) {
      parts.push(text.slice(start, i));
      start = i + 1;
    }
  }
  parts.push(text.slice(start));
  return parts;
}

function topLevelEqualsIndex(text) {
  let depth = 0;
  let quote = null;
  for (let i = 0; i < text.length; i += 1) {
    const ch = text[i];
    if (quote !== null) {
      if (ch === '\\') i += 1;
      else if (ch === quote) quote = null;
      continue;
    }
    if (ch === '"' || ch === "'" || ch === '`') {
      quote = ch;
    } else if (ch === '{' || ch === '[' || ch === '(') {
      depth += 1;
    } else if (ch === '}' || ch === ']' || ch === ')') {
      depth -= 1;
    } else if (ch === '=' && depth === 0) {
      return i;
    }
  }
  return -1;
}

function skipBindingInitializer(text, index) {
  let i = index;
  let depth = 0;
  let quote = null;
  while (i < text.length) {
    const ch = text[i];
    if (quote !== null) {
      if (ch === '\\') i += 1;
      else if (ch === quote) quote = null;
      i += 1;
      continue;
    }
    if (ch === '"' || ch === "'" || ch === '`') {
      quote = ch;
    } else if (ch === '{' || ch === '[' || ch === '(') {
      depth += 1;
    } else if (ch === '}' || ch === ']' || ch === ')') {
      if (depth === 0) return i;
      depth -= 1;
    } else if (ch === ',' && depth === 0) {
      return i;
    }
    i += 1;
  }
  return i;
}

function addBindingNamesFromPattern(pattern, addName) {
  for (let i = 0; i < pattern.length; ) {
    const ch = pattern[i];
    if (ch === '=') {
      i = skipBindingInitializer(pattern, i + 1);
      continue;
    }
    if (!isIdentifierStart(ch)) {
      i += 1;
      continue;
    }
    let end = i + 1;
    while (end < pattern.length && isIdentifierPart(pattern[end])) end += 1;
    const name = pattern.slice(i, end);
    const next = skipWhitespace(pattern, end);
    if (pattern[next] === ':') {
      i = next + 1;
      continue;
    }
    addName(name);
    i = end;
  }
}

function readVarDeclarationText(source, index) {
  let depth = 0;
  let quote = null;
  for (let i = index; i < source.length; i += 1) {
    const ch = source[i];
    if (quote !== null) {
      if (ch === '\\') i += 1;
      else if (ch === quote) quote = null;
      continue;
    }
    if (ch === '"' || ch === "'" || ch === '`') {
      quote = ch;
    } else if (ch === '{' || ch === '[' || ch === '(') {
      depth += 1;
    } else if (ch === '}' || ch === ']' || ch === ')') {
      depth -= 1;
    } else if (ch === ';' && depth === 0) {
      return source.slice(index, i);
    }
  }
  return source.slice(index);
}

function codeKeywordMatches(source, keyword) {
  const matches = [];
  for (let i = 0; i < source.length; ) {
    const ch = source[i];
    const next = source[i + 1];
    if (ch === '"' || ch === "'" || ch === '`') {
      const quote = ch;
      i += 1;
      while (i < source.length) {
        if (source[i] === '\\') {
          i += 2;
        } else if (source[i] === quote) {
          i += 1;
          break;
        } else {
          i += 1;
        }
      }
      continue;
    }
    if (ch === '/' && next === '/') {
      i += 2;
      while (i < source.length && source[i] !== '\n' && source[i] !== '\r') i += 1;
      continue;
    }
    if (ch === '/' && next === '*') {
      i += 2;
      while (i + 1 < source.length && !(source[i] === '*' && source[i + 1] === '/')) i += 1;
      i = Math.min(source.length, i + 2);
      continue;
    }
    if (ch === '/' && isRegexLiteralStart(source, i)) {
      i = skipRegexLiteralSource(source, i);
      continue;
    }
    if (
      source.startsWith(keyword, i) &&
      (i === 0 || !isIdentifierPart(source[i - 1])) &&
      (i + keyword.length >= source.length || !isIdentifierPart(source[i + keyword.length]))
    ) {
      matches.push(i);
      i += keyword.length;
      continue;
    }
    i += 1;
  }
  return matches;
}

function skipQuotedSource(source, index, quote) {
  let i = index + 1;
  while (i < source.length) {
    if (source[i] === '\\') {
      i += 2;
    } else if (source[i] === quote) {
      return i + 1;
    } else {
      i += 1;
    }
  }
  return source.length;
}

function isRegexLiteralStart(source, index) {
  const next = source[index + 1];
  if (next === '/' || next === '*') return false;
  let prior = index - 1;
  while (prior >= 0 && /\s/.test(source[prior])) prior -= 1;
  if (prior < 0) return true;
  if (isIdentifierPart(source[prior])) {
    let tokenStart = prior;
    while (tokenStart > 0 && isIdentifierPart(source[tokenStart - 1])) tokenStart -= 1;
    return [
      'await',
      'case',
      'delete',
      'return',
      'throw',
      'typeof',
      'void',
      'yield',
    ].includes(source.slice(tokenStart, prior + 1));
  }
  return '([{=,:;!?'.includes(source[prior]);
}

function skipRegexLiteralSource(source, index) {
  let i = index + 1;
  let inClass = false;
  while (i < source.length) {
    const ch = source[i];
    if (ch === '\\') {
      i += 2;
      continue;
    }
    if (ch === '[') {
      inClass = true;
      i += 1;
      continue;
    }
    if (ch === ']' && inClass) {
      inClass = false;
      i += 1;
      continue;
    }
    if (ch === '/' && !inClass) {
      i += 1;
      while (i < source.length && /[A-Za-z]/.test(source[i])) i += 1;
      return i;
    }
    if (ch === '\n' || ch === '\r') return i;
    i += 1;
  }
  return source.length;
}

function findTemplateExpressionEnd(source, index) {
  let depth = 1;
  let i = index;
  while (i < source.length) {
    const ch = source[i];
    const next = source[i + 1];
    if (ch === '"' || ch === "'") {
      i = skipQuotedSource(source, i, ch);
      continue;
    }
    if (ch === '`') {
      i = skipTemplateSource(source, i);
      continue;
    }
    if (ch === '/' && next === '/') {
      i += 2;
      while (i < source.length && source[i] !== '\n' && source[i] !== '\r') i += 1;
      continue;
    }
    if (ch === '/' && next === '*') {
      i += 2;
      while (i + 1 < source.length && !(source[i] === '*' && source[i + 1] === '/')) i += 1;
      i = Math.min(source.length, i + 2);
      continue;
    }
    if (ch === '/' && isRegexLiteralStart(source, i)) {
      i = skipRegexLiteralSource(source, i);
      continue;
    }
    if (ch === '{') depth += 1;
    if (ch === '}') {
      depth -= 1;
      if (depth === 0) return i;
    }
    i += 1;
  }
  return -1;
}

function skipTemplateSource(source, index) {
  let i = index + 1;
  while (i < source.length) {
    if (source[i] === '\\') {
      i += 2;
      continue;
    }
    if (source[i] === '`') return i + 1;
    if (source[i] === '$' && source[i + 1] === '{') {
      const end = findTemplateExpressionEnd(source, i + 2);
      if (end === -1) return source.length;
      i = end + 1;
      continue;
    }
    i += 1;
  }
  return source.length;
}

function templateMentionsIdentifier(source, index, name) {
  let i = index + 1;
  while (i < source.length) {
    if (source[i] === '\\') {
      i += 2;
      continue;
    }
    if (source[i] === '`') return { mentions: false, next: i + 1 };
    if (source[i] === '$' && source[i + 1] === '{') {
      const end = findTemplateExpressionEnd(source, i + 2);
      if (end === -1) return { mentions: false, next: source.length };
      if (sourceMentionsIdentifier(source.slice(i + 2, end), name)) {
        return { mentions: true, next: end + 1 };
      }
      i = end + 1;
      continue;
    }
    i += 1;
  }
  return { mentions: false, next: source.length };
}

function sourceMentionsIdentifier(source, name) {
  if (typeof source !== 'string' || name.length === 0) return false;
  for (let i = 0; i + name.length <= source.length; ) {
    const ch = source[i];
    const next = source[i + 1];
    if (ch === '"' || ch === "'") {
      i = skipQuotedSource(source, i, ch);
      continue;
    }
    if (ch === '`') {
      const template = templateMentionsIdentifier(source, i, name);
      if (template.mentions) return true;
      i = template.next;
      continue;
    }
    if (ch === '/' && next === '/') {
      i += 2;
      while (i < source.length && source[i] !== '\n' && source[i] !== '\r') i += 1;
      continue;
    }
    if (ch === '/' && next === '*') {
      i += 2;
      while (i + 1 < source.length && !(source[i] === '*' && source[i + 1] === '/')) i += 1;
      i = Math.min(source.length, i + 2);
      continue;
    }
    if (ch === '/' && isRegexLiteralStart(source, i)) {
      i = skipRegexLiteralSource(source, i);
      continue;
    }
    if (
      source.startsWith(name, i) &&
      (i === 0 || !isIdentifierPart(source[i - 1])) &&
      (i + name.length >= source.length || !isIdentifierPart(source[i + name.length]))
    ) {
      return true;
    }
    i += 1;
  }
  return false;
}

function focusedDirectEvalTdzCandidates(source) {
  if (typeof source !== 'string') return [];
  const trimmed = source.trim();
  if (isDirectEvalTdzCandidateName(trimmed)) return [trimmed];
  const parenthesizedMatch = /^\(\s*([A-Za-z_$][0-9A-Za-z_$]*)\s*\)$/.exec(trimmed);
  if (parenthesizedMatch !== null && isDirectEvalTdzCandidateName(parenthesizedMatch[1])) {
    return [parenthesizedMatch[1]];
  }
  const typeofMatch = /^typeof\s+([A-Za-z_$][0-9A-Za-z_$]*)$/.exec(trimmed);
  if (typeofMatch !== null && isDirectEvalTdzCandidateName(typeofMatch[1])) {
    return [typeofMatch[1]];
  }
  const memberMatch =
    /^([A-Za-z_$][0-9A-Za-z_$]*)\s*(?:\.|\?\.)\s*([A-Za-z_$][0-9A-Za-z_$]*)$/.exec(trimmed);
  if (memberMatch !== null && isDirectEvalTdzCandidateName(memberMatch[1])) {
    return [memberMatch[1]];
  }
  const computedMemberMatch =
    /^([A-Za-z_$][0-9A-Za-z_$]*)\s*(?:\?\.)?\s*\[.*\]$/.exec(trimmed);
  if (computedMemberMatch !== null && isDirectEvalTdzCandidateName(computedMemberMatch[1])) {
    return [computedMemberMatch[1]];
  }
  const templateMatch = /^`\$\{\s*([A-Za-z_$][0-9A-Za-z_$]*)\s*\}`$/.exec(trimmed);
  if (templateMatch !== null && isDirectEvalTdzCandidateName(templateMatch[1])) {
    return [templateMatch[1]];
  }
  return [];
}

function isDirectEvalTdzCandidateName(name) {
  return /^[A-Za-z_$][0-9A-Za-z_$]*$/.test(name) && !KNOWN_EVAL_GLOBAL_NAMES.has(name);
}

const KNOWN_EVAL_GLOBAL_NAMES = new Set([
  'globalThis',
  'console',
  'Array',
  'BigInt',
  'Boolean',
  'Date',
  'Error',
  'Infinity',
  'JSON',
  'Map',
  'Math',
  'NaN',
  'Number',
  'Object',
  'Promise',
  'RangeError',
  'ReferenceError',
  'RegExp',
  'Set',
  'String',
  'Symbol',
  'SyntaxError',
  'TypeError',
  'undefined',
]);

function directEvalEnvKey(bindings) {
  return bindings
    .map((binding) => `${binding.name}:${binding.cellRaw}`)
    .sort()
    .join('|');
}

function collectVariableDeclarationBindingNames(source, keyword) {
  const names = [];
  const addName = (name) => {
    if (!names.includes(name)) names.push(name);
  };
  for (const keywordIndex of codeKeywordMatches(source, keyword)) {
    const declarationText =
      readForHeadVariableDeclarationText(source, keywordIndex, keyword) ??
      readVarDeclarationText(source, keywordIndex + keyword.length);
    for (const declarator of splitTopLevelComma(declarationText)) {
      const equalsIndex = topLevelEqualsIndex(declarator);
      const pattern = (equalsIndex === -1 ? declarator : declarator.slice(0, equalsIndex)).trim();
      addBindingNamesFromPattern(pattern, addName);
    }
  }
  return names;
}

function readForHeadVariableDeclarationText(source, keywordIndex, keyword) {
  let prior = keywordIndex - 1;
  while (prior >= 0 && /\s/.test(source[prior])) prior -= 1;
  if (source[prior] !== '(') return null;
  let cursor = keywordIndex + keyword.length;
  let depth = 0;
  while (cursor < source.length) {
    const ch = source[cursor];
    if (ch === '"' || ch === "'" || ch === '`') {
      cursor = skipQuotedSource(source, cursor, ch);
      continue;
    }
    if (ch === '(' || ch === '[' || ch === '{') {
      depth += 1;
    } else if ((ch === ')' || ch === ']' || ch === '}') && depth > 0) {
      depth -= 1;
    } else if (depth === 0) {
      if (source.startsWith('in', cursor) && isIdentifierBoundary(source, cursor, cursor + 2)) {
        return source.slice(keywordIndex + keyword.length, cursor);
      }
      if (source.startsWith('of', cursor) && isIdentifierBoundary(source, cursor, cursor + 2)) {
        return source.slice(keywordIndex + keyword.length, cursor);
      }
    }
    cursor += 1;
  }
  return null;
}

function asyncFunctionDeclarationStart(source, functionIndex) {
  let asyncEnd = functionIndex;
  while (asyncEnd > 0 && /\s/.test(source[asyncEnd - 1])) asyncEnd -= 1;
  const asyncStart = asyncEnd - 'async'.length;
  if (
    asyncStart >= 0 &&
    source.slice(asyncStart, asyncEnd) === 'async' &&
    (asyncStart === 0 || !isIdentifierPart(source[asyncStart - 1]))
  ) {
    return asyncStart;
  }
  return functionIndex;
}

function collectFunctionDeclarationNames(source) {
  const names = [];
  const addName = (name) => {
    if (!names.includes(name)) names.push(name);
  };
  for (const functionIndex of codeKeywordMatches(source, 'function')) {
    const declarationStart = asyncFunctionDeclarationStart(source, functionIndex);
    let prior = declarationStart - 1;
    while (prior >= 0 && /\s/.test(source[prior])) prior -= 1;
    if (prior >= 0 && !';{}'.includes(source[prior])) {
      continue;
    }
    const rest = source.slice(functionIndex);
    const match = /^function\s*\*?\s+([A-Za-z_$][0-9A-Za-z_$]*)\s*\(/.exec(rest);
    if (match !== null) {
      addName(match[1]);
    }
  }
  return names;
}

function collectEvalDeclarationNames(source) {
  const names = collectVariableDeclarationBindingNames(source, 'var');
  const addName = (name) => {
    if (!names.includes(name)) names.push(name);
  };
  for (const name of collectFunctionDeclarationNames(source)) addName(name);
  return names;
}

function strictEvalHasDeleteIdentifier(source) {
  for (const keywordIndex of codeKeywordMatches(source, 'delete')) {
    let index = skipWhitespace(source, keywordIndex + 'delete'.length);
    if (!isIdentifierStart(source[index])) continue;
    index += 1;
    while (index < source.length && isIdentifierPart(source[index])) index += 1;
    const next = skipWhitespace(source, index);
    if (next >= source.length || !'.[('.includes(source[next])) return true;
  }
  return false;
}

function strictEvalHasRestrictedVariableBinding(source) {
  for (const keyword of ['var', 'let', 'const']) {
    const names = collectVariableDeclarationBindingNames(source, keyword);
    if (names.includes('arguments') || names.includes('eval')) {
      return true;
    }
  }
  return false;
}

function strictEvalHasRestrictedBinding(source) {
  return (
    strictEvalHasRestrictedVariableBinding(source) ||
    collectFunctionDeclarationNames(source).some((name) => name === 'arguments' || name === 'eval')
  );
}

function evalWithEnvDescriptor(source, envRaw) {
  if (envRaw === TAG_UNDEFINED) {
    return eval(source);
  }

  const pairs = decodeArray(envRaw);
  let callerIsStrict = false;
  let pairOffset = 0;
  if (
    pairs.length >= 2 &&
    rawTag(pairs[0]) === TAG_STRING &&
    decodeString(pairs[0]) === EVAL_DESCRIPTOR_VERSION
  ) {
    if (decodeValue(pairs[1]) !== true) {
      throw new TypeError('unsupported direct eval env descriptor version');
    }
    pairOffset = 2;
  }
  if (
    pairs.length >= pairOffset + 2 &&
    rawTag(pairs[pairOffset]) === TAG_STRING &&
    decodeString(pairs[pairOffset]) === EVAL_DESCRIPTOR_CALLER_STRICT
  ) {
    callerIsStrict = decodeValue(pairs[pairOffset + 1]) === true;
    pairOffset += 2;
  }
  if ((pairs.length - pairOffset) % 2 !== 0) {
    throw new TypeError('invalid direct eval env descriptor');
  }
  if (callerIsStrict && strictEvalHasDeleteIdentifier(source)) {
    throw new SyntaxError('Delete of an unqualified identifier in strict mode.');
  }
  if (callerIsStrict && strictEvalHasRestrictedBinding(source)) {
    throw new SyntaxError('Unexpected eval or arguments in strict mode.');
  }

  const bindings = [];
  const tdzBindings = [];
  if (
    pairs.length >= pairOffset + 2 &&
    rawTag(pairs[pairOffset]) === TAG_STRING &&
    decodeString(pairs[pairOffset]) === EVAL_DESCRIPTOR_BINDINGS
  ) {
    const bindingEntries = decodeArray(pairs[pairOffset + 1]);
    pairOffset += 2;
    if (pairOffset !== pairs.length) {
      throw new TypeError('invalid direct eval env descriptor');
    }
    for (const entryRaw of bindingEntries) {
      const entry = decodeArray(entryRaw);
      if (entry.length !== 2 && entry.length !== 3) {
        throw new TypeError('invalid direct eval env descriptor binding');
      }
      const kind = entry.length === 3 ? decodeString(entry[2]) : 'readwrite';
      if (kind !== 'readwrite') {
        if (kind !== 'tdz') {
          throw new TypeError(`unsupported direct eval env descriptor binding kind: ${kind}`);
        }
        tdzBindings.push(decodeString(entry[0]));
        continue;
      }
      const name = decodeString(entry[0]);
      const cellRaw = entry[1];
      const raw = readEnvCellRaw(cellRaw);
      bindings.push({ name, cellRaw, raw, value: decodeValue(raw) });
    }
  } else {
    if ((pairs.length - pairOffset) % 2 !== 0) {
      throw new TypeError('invalid direct eval env descriptor');
    }
    for (let i = pairOffset; i < pairs.length; i += 2) {
      const name = decodeString(pairs[i]);
      const cellRaw = pairs[i + 1];
      const raw = readEnvCellRaw(cellRaw);
      bindings.push({ name, cellRaw, raw, value: decodeValue(raw) });
    }
  }
  for (const name of tdzBindings) {
    if (sourceMentionsIdentifier(source, name)) {
      throw new ReferenceError(`Cannot access '${name}' before initialization`);
    }
  }

  const names = bindings.map((binding) => binding.name);
  const thisBinding = bindings.find((binding) => binding.name === 'this');
  const envKey = directEvalEnvKey(bindings);
  const extraMap = directEvalExtraBindings.get(envKey) ?? new Map();
  const extraBindings = [];
  for (const [name, value] of extraMap.entries()) {
    if (!names.includes(name)) {
      extraBindings.push({ name, value });
      names.push(name);
    }
  }
  if (!callerIsStrict) {
    for (const name of collectEvalDeclarationNames(source)) {
      if (!names.includes(name)) {
        extraBindings.push({ name, value: undefined });
        names.push(name);
      }
    }
  }
  for (const name of focusedDirectEvalTdzCandidates(source)) {
    if (!names.includes(name)) {
      if (/^typeof\s+/.test(source.trim())) continue;
      throw new ReferenceError(`Cannot access '${name}' before initialization`);
    }
    const binding = bindings.find((entry) => entry.name === name);
    if (name !== 'undefined' && binding !== undefined && binding.value === undefined) {
      throw new ReferenceError(`Cannot access '${name}' before initialization`);
    }
  }
  const formalBindings = bindings.filter((binding) => binding.name !== 'this');
  const allFormalBindings = formalBindings.concat(extraBindings);
  const sourceReferencesStrictReservedBinding = /\b(?:arguments|eval)\b/.test(source);
  const useStrictWrapper = callerIsStrict && !sourceReferencesStrictReservedBinding;
  const wrapperBindings = useStrictWrapper
    ? allFormalBindings.filter((binding) => binding.name !== 'arguments' && binding.name !== 'eval')
    : allFormalBindings;
  const formalNames = wrapperBindings.map((binding) => binding.name);
  const sourceName = uniqueInternalName('__ts2wasm_eval_source', names);
  const resultName = uniqueInternalName('__ts2wasm_eval_result', [...names, sourceName]);
  const abruptName = uniqueInternalName('__ts2wasm_eval_abrupt', [
    ...names,
    sourceName,
    resultName,
  ]);
  const strictPrefix = useStrictWrapper ? '"use strict"; ' : '';
  const wrapper = Function(
    sourceName,
    ...formalNames,
    `${strictPrefix}let ${resultName}; let ${abruptName} = null; try { ${resultName} = eval(${sourceName}); } catch (error) { ${abruptName} = { kind: "throw", value: error }; } return [${resultName}, ${abruptName}, ${formalNames.join(', ')}];`,
  );
  const values = wrapperBindings.map((binding) => binding.value);
  const thisValue = thisBinding === undefined ? undefined : thisBinding.value;
  const [result, abrupt, ...updatedValues] = wrapper.call(thisValue, source, ...values);
  const ledger = {
    version: DIRECT_EVAL_MUTATION_LEDGER_VERSION,
    result,
    writes: [],
    createdBindings: [],
    abrupt,
  };

  for (let i = 0; i < wrapperBindings.length; i += 1) {
    if (!Object.is(wrapperBindings[i].value, updatedValues[i])) {
      if (wrapperBindings[i].cellRaw !== undefined) {
        ledger.writes.push({
          cellRaw: wrapperBindings[i].cellRaw,
          value: updatedValues[i],
        });
      } else {
        ledger.createdBindings.push({
          name: wrapperBindings[i].name,
          value: updatedValues[i],
        });
      }
    }
  }
  applyDirectEvalMutationLedger(envKey, extraMap, ledger);
  if (extraMap.size > 0) directEvalExtraBindings.set(envKey, extraMap);

  return ledger.result;
}

function applyDirectEvalMutationLedger(envKey, extraMap, ledger) {
  if (ledger === null || typeof ledger !== 'object') {
    throw new TypeError(`invalid direct eval mutation ledger for ${envKey}`);
  }
  if (ledger.version !== DIRECT_EVAL_MUTATION_LEDGER_VERSION) {
    throw new TypeError(`unsupported direct eval mutation ledger version for ${envKey}`);
  }
  if (!Array.isArray(ledger.writes) || !Array.isArray(ledger.createdBindings)) {
    throw new TypeError(`invalid direct eval mutation ledger entries for ${envKey}`);
  }
  for (const write of ledger.writes) {
    if (write === null || typeof write !== 'object' || write.cellRaw === undefined) {
      throw new TypeError(`invalid direct eval mutation ledger write for ${envKey}`);
    }
    writeEnvCellRaw(write.cellRaw, encodeHostValue(write.value));
  }
  for (const binding of ledger.createdBindings) {
    if (binding === null || typeof binding !== 'object' || typeof binding.name !== 'string') {
      throw new TypeError(`invalid direct eval mutation ledger binding for ${envKey}`);
    }
    extraMap.set(binding.name, binding.value);
  }
  if (ledger.abrupt !== null) {
    if (
      ledger.abrupt === null ||
      typeof ledger.abrupt !== 'object' ||
      ledger.abrupt.kind !== 'throw'
    ) {
      throw new TypeError(`invalid direct eval abrupt completion ledger for ${envKey}`);
    }
    throw ledger.abrupt.value;
  }
}

function decodeArgs(raw) {
  return decodeArray(raw).map(decodeValue);
}

function getIterator(value) {
  if (value === null || value === undefined) {
    throw new TypeError('value is not iterable');
  }
  const method = value[Symbol.iterator];
  if (typeof method !== 'function') {
    throw new TypeError('value is not iterable');
  }
  const iterator = method.call(value);
  if (iterator === null || iterator === undefined || typeof iterator.next !== 'function') {
    throw new TypeError('iterator method did not return an iterator');
  }
  return iterator;
}

function getIteratorFromRaw(raw) {
  return getIterator(decodeValue(raw));
}

function requireCallable(raw) {
  const fn = decodeValue(raw);
  if (typeof fn !== 'function') {
    throw new TypeError('iterator helper callback must be callable');
  }
  return fn;
}

function decodeHostNumber(raw) {
  if (rawTag(raw) === TAG_NUMBER) return raw >> 3;
  const value = decodeValue(raw);
  return typeof value === 'number' ? value : raw;
}

function makeIteratorHelper(next) {
  return {
    [Symbol.iterator]() {
      return this;
    },
    next,
  };
}

function iteratorMap(iteratorRaw, callbackRaw) {
  const iterator = getIteratorFromRaw(iteratorRaw);
  const callback = requireCallable(callbackRaw);
  let index = 0;
  return makeIteratorHelper(() => {
    const result = iterator.next();
    if (result.done) return { value: undefined, done: true };
    return { value: callback(result.value, index++), done: false };
  });
}

function iteratorFilter(iteratorRaw, callbackRaw) {
  const iterator = getIteratorFromRaw(iteratorRaw);
  const callback = requireCallable(callbackRaw);
  let index = 0;
  return makeIteratorHelper(() => {
    while (true) {
      const result = iterator.next();
      if (result.done) return { value: undefined, done: true };
      const value = result.value;
      if (callback(value, index++)) return { value, done: false };
    }
  });
}

function iteratorTake(iteratorRaw, limitRaw) {
  const iterator = getIteratorFromRaw(iteratorRaw);
  const limit = Math.max(0, Math.trunc(Number(decodeHostNumber(limitRaw))));
  let remaining = limit;
  return makeIteratorHelper(() => {
    if (remaining <= 0) return { value: undefined, done: true };
    remaining -= 1;
    const result = iterator.next();
    return result.done ? { value: undefined, done: true } : result;
  });
}

function iteratorDrop(iteratorRaw, limitRaw) {
  const iterator = getIteratorFromRaw(iteratorRaw);
  let remaining = Math.max(0, Math.trunc(Number(decodeHostNumber(limitRaw))));
  let dropped = false;
  return makeIteratorHelper(() => {
    if (!dropped) {
      while (remaining > 0) {
        const result = iterator.next();
        if (result.done) return { value: undefined, done: true };
        remaining -= 1;
      }
      dropped = true;
    }
    const result = iterator.next();
    return result.done ? { value: undefined, done: true } : result;
  });
}

function iteratorToArray(iteratorRaw) {
  return Array.from(getIteratorFromRaw(iteratorRaw));
}

function iteratorReduce(iteratorRaw, callbackRaw, initialRaw, hasInitialRaw) {
  const iterator = getIteratorFromRaw(iteratorRaw);
  const callback = requireCallable(callbackRaw);
  let accumulator = decodeValue(initialRaw);
  let initialized = Boolean(decodeValue(hasInitialRaw));
  let index = 0;
  while (true) {
    const result = iterator.next();
    if (result.done) break;
    if (!initialized) {
      accumulator = result.value;
      initialized = true;
    } else {
      accumulator = callback(accumulator, result.value, index);
    }
    index += 1;
  }
  if (!initialized) {
    throw new TypeError('Reduce of empty iterator with no initial value');
  }
  return accumulator;
}

function iteratorForEach(iteratorRaw, callbackRaw) {
  const iterator = getIteratorFromRaw(iteratorRaw);
  const callback = requireCallable(callbackRaw);
  let index = 0;
  for (const value of iterator) {
    callback(value, index++);
  }
  return undefined;
}

function iteratorSome(iteratorRaw, callbackRaw) {
  const iterator = getIteratorFromRaw(iteratorRaw);
  const callback = requireCallable(callbackRaw);
  let index = 0;
  for (const value of iterator) {
    if (callback(value, index++)) return true;
  }
  return false;
}

function iteratorEvery(iteratorRaw, callbackRaw) {
  const iterator = getIteratorFromRaw(iteratorRaw);
  const callback = requireCallable(callbackRaw);
  let index = 0;
  for (const value of iterator) {
    if (!callback(value, index++)) return false;
  }
  return true;
}

function iteratorFind(iteratorRaw, callbackRaw) {
  const iterator = getIteratorFromRaw(iteratorRaw);
  const callback = requireCallable(callbackRaw);
  let index = 0;
  for (const value of iterator) {
    if (callback(value, index++)) return value;
  }
  return undefined;
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
      try {
        const result = evalWithEnvDescriptor(decodeString(sourceRaw), envRaw);
        return encodeHostValue(result);
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'eval.indirect'(sourceRaw, _envRaw) {
      try {
        const result = globalThis.eval(decodeString(sourceRaw));
        return encodeHostValue(result);
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'function.compile'(argsRaw) {
      try {
        const args = decodeArgs(argsRaw);
        const fn = Function(...args);
        return encodeHostFunctionValue(fn);
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'function.call'(handleRaw, argsRaw) {
      try {
        const fn = hostFunctions[decodeHostFunctionHandle(handleRaw)];
        if (typeof fn !== 'function') {
          throw new TypeError(`unknown host function handle: ${handleRaw}`);
        }
        return encodeHostValue(fn(...decodeArgs(argsRaw)));
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'function.callMethod'(handleRaw, receiverRaw, argsRaw) {
      try {
        const fn = hostFunctions[decodeHostFunctionHandle(handleRaw)];
        if (typeof fn !== 'function') {
          throw new TypeError(`unknown host function handle: ${handleRaw}`);
        }
        return encodeHostValue(fn.apply(decodeHostReceiver(receiverRaw), decodeArgs(argsRaw)));
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'function.construct'(handleRaw, argsRaw) {
      try {
        const fn = hostFunctions[decodeHostFunctionHandle(handleRaw)];
        if (typeof fn !== 'function') {
          throw new TypeError(`unknown host function handle: ${handleRaw}`);
        }
        return encodeHostValue(Reflect.construct(fn, decodeArgs(argsRaw)));
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'reflectConstruct'(targetRaw, argsRaw, newTargetRaw) {
      try {
        return encodeHostValue(
          Reflect.construct(
            decodeValue(targetRaw),
            decodeArgs(argsRaw),
            newTargetRaw === undefined
              ? decodeValue(targetRaw)
              : decodeValue(newTargetRaw),
          ),
        );
      } catch (error) {
        return encodeHostException(error);
      }
    },
    getIterator(valueRaw) {
      try {
        return encodeHostValue(getIteratorFromRaw(valueRaw));
      } catch (error) {
        return encodeHostException(error);
      }
    },
    iteratorNext(iteratorRaw) {
      try {
        return encodeHostValue(getIteratorFromRaw(iteratorRaw).next());
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'iterator.map'(iteratorRaw, callbackRaw) {
      try {
        return encodeHostValue(iteratorMap(iteratorRaw, callbackRaw));
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'iterator.filter'(iteratorRaw, callbackRaw) {
      try {
        return encodeHostValue(iteratorFilter(iteratorRaw, callbackRaw));
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'iterator.take'(iteratorRaw, limitRaw) {
      try {
        return encodeHostValue(iteratorTake(iteratorRaw, limitRaw));
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'iterator.drop'(iteratorRaw, limitRaw) {
      try {
        return encodeHostValue(iteratorDrop(iteratorRaw, limitRaw));
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'iterator.toArray'(iteratorRaw) {
      try {
        return encodeHostValue(iteratorToArray(iteratorRaw));
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'iterator.reduce'(iteratorRaw, callbackRaw, initialRaw, hasInitialRaw) {
      try {
        return encodeHostValue(iteratorReduce(iteratorRaw, callbackRaw, initialRaw, hasInitialRaw));
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'iterator.forEach'(iteratorRaw, callbackRaw) {
      try {
        return encodeHostValue(iteratorForEach(iteratorRaw, callbackRaw));
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'iterator.some'(iteratorRaw, callbackRaw) {
      try {
        return encodeHostValue(iteratorSome(iteratorRaw, callbackRaw));
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'iterator.every'(iteratorRaw, callbackRaw) {
      try {
        return encodeHostValue(iteratorEvery(iteratorRaw, callbackRaw));
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'iterator.find'(iteratorRaw, callbackRaw) {
      try {
        return encodeHostValue(iteratorFind(iteratorRaw, callbackRaw));
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'json.stringify'(valueRaw, replacerRaw, spaceRaw) {
      try {
        return encodeHostValue(
          JSON.stringify(decodeValue(valueRaw), decodeValue(replacerRaw), decodeValue(spaceRaw)),
        );
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'json.parse'(sourceRaw, reviverRaw) {
      try {
        return encodeHostValue(JSON.parse(String(decodeValue(sourceRaw)), decodeValue(reviverRaw)));
      } catch (error) {
        return encodeHostException(error);
      }
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
