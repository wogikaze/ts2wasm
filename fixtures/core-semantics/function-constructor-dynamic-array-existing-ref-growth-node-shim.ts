let body =
  "if (!globalThis.__ts2wasmHostArrayGrowth) { globalThis.__ts2wasmHostArrayGrowth = [1, 2, 3, 4]; return { items: globalThis.__ts2wasmHostArrayGrowth }; } let a = globalThis.__ts2wasmHostArrayGrowth; a.push(5); return { items: a };";
let make = Function(body);
let first = make();
make();
console.log(first.items.length);
console.log(first.items[4]);
console.log(first.items[5]);
