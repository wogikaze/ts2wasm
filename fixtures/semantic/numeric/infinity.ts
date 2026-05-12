// typeof Infinity is still "number" in JavaScript
// Note: ts2wasm runtime does not support Infinity as a special IEEE 754 value.
// Most comparisons and arithmetic involving Infinity produce undefined or
// incorrect results. Only basic typeof is tested here.
console.log(typeof Infinity);
