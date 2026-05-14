// Atomics.load/store basic usage
const ta = new Int32Array([10, 20, 30]);
console.log(Atomics.load(ta, 0));
console.log(Atomics.load(ta, 1));
console.log(Atomics.load(ta, 2));
const rv = Atomics.store(ta, 1, 42);
console.log(rv);
console.log(Atomics.load(ta, 1));
