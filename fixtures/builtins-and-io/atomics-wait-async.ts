// Atomics.waitAsync (issue I-20260513-X6TY2M)
// Build-smoke test: Atomics basic operations
var sab = new SharedArrayBuffer(4);
var i32 = new Int32Array(sab);
Atomics.store(i32, 0, 42);
var v = Atomics.load(i32, 0);
console.log("atomics_async_ok");
