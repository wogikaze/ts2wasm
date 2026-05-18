let body =
  "globalThis.__ts2wasmObjectGrowth = globalThis.__ts2wasmObjectGrowth || { a: 1, b: 2, c: 3, d: 4 };" +
  "if (globalThis.__ts2wasmObjectGrowthSeen) { globalThis.__ts2wasmObjectGrowth.e = 5; }" +
  "globalThis.__ts2wasmObjectGrowthSeen = true;" +
  "return globalThis.__ts2wasmObjectGrowth";
let grow = Function(body);
let first = grow();
console.log(first.a);
let second = grow();
console.log(second.e);
console.log(second.missing);
