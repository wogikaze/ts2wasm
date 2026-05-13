const cells = new Int32Array([8, 1, 0, 4]);

console.log(Atomics.load(cells, 0));
console.log(Atomics.store(cells, 1, 7));
console.log(Atomics.add(cells, 0, 2));
console.log(Atomics.sub(cells, 0, 1));
console.log(Atomics.and(cells, 0, 7));
console.log(Atomics.or(cells, 2, 3));
console.log(Atomics.xor(cells, 2, 1));
console.log(Atomics.exchange(cells, 3, 9));
console.log(Atomics.compareExchange(cells, 3, 9, 11));
console.log(Atomics.isLockFree(4));
console.log(Atomics.wait(cells, 3, 11, 0));
console.log(Atomics.notify(cells, 3, 1));
