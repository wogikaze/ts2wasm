// Atomics operations: add, sub, and, or, xor, exchange, compareExchange
const ta = new Int32Array([10, 20, 30, 40, 50]);

// Atomics.add
console.log(Atomics.add(ta, 0, 5));   // old: 10
console.log(Atomics.load(ta, 0));     // new: 15

// Atomics.sub
console.log(Atomics.sub(ta, 1, 3));   // old: 20
console.log(Atomics.load(ta, 1));     // new: 17

// Atomics.and
console.log(Atomics.and(ta, 2, 0xFF));  // old: 30
console.log(Atomics.load(ta, 2));       // new: 30 (30 & 0xFF = 30)

// Atomics.or
console.log(Atomics.or(ta, 3, 8));    // old: 40
console.log(Atomics.load(ta, 3));     // new: 40 (40 | 8 = 40)

// Atomics.xor
console.log(Atomics.xor(ta, 4, 0xFF)); // old: 50
console.log(Atomics.load(ta, 4));      // new: 173 (50 ^ 255 = 173)

// Atomics.exchange
console.log(Atomics.exchange(ta, 0, 99)); // old: 15
console.log(Atomics.load(ta, 0));         // new: 99

// Atomics.compareExchange
console.log(Atomics.compareExchange(ta, 1, 17, 42)); // old: 17 (matches expected)
console.log(Atomics.load(ta, 1));                      // new: 42
