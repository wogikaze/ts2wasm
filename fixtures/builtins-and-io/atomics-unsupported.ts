// Atomics basic usage — should produce unsupported diagnostic
const sab = new SharedArrayBuffer(16);
const i32 = new Int32Array(sab);
Atomics.store(i32, 0, 42);
console.log(Atomics.load(i32, 0));
