let body =
  "globalThis.__ts2wasmArrayGrowth = globalThis.__ts2wasmArrayGrowth || [1, 2, 3, 4];" +
  "if (globalThis.__ts2wasmArrayGrowthSeen) { globalThis.__ts2wasmArrayGrowth.push(5); }" +
  "globalThis.__ts2wasmArrayGrowthSeen = true;" +
  "return { items: globalThis.__ts2wasmArrayGrowth }";
let grow = Function(body);
let first = grow();
console.log(first.items.length);
let second = grow();
console.log(second.items.length);
console.log(second.items[4]);
console.log(second.items[5]);
