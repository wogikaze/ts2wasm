// Issue 5135: Builtin arity validation should accept 0-arg calls for coercion/math globals
// Boolean() returns false
const b = Boolean();
console.log(b);
// Number() returns 0
const n = Number();
console.log(n);
// isNaN(undefined) returns true
const nan = isNaN();
console.log(nan);
// isFinite(undefined) returns false
const fin = isFinite();
console.log(fin);
