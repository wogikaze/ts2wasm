// NaN type (basic JS operator coverage)
// Note: ts2wasm runtime does not support NaN as a special IEEE 754 value.
// Operations that produce NaN (0/0, etc.) return undefined instead.
// NaN strict equality semantics (NaN !== NaN) are not yet implemented.
// These tests document the basic known-working areas.

// typeof NaN is still "number" in JavaScript
console.log(typeof NaN);
