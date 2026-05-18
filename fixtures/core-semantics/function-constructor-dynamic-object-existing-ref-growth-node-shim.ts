let body =
  "if (!globalThis.__ts2wasmHostGrowth) { globalThis.__ts2wasmHostGrowth = { a: 1 }; return globalThis.__ts2wasmHostGrowth; } let o = globalThis.__ts2wasmHostGrowth; o.b = 2; o.c = 3; o.d = 4; o.e = 5; return o;";
let make = Function(body);
let first = make();
make();
console.log(first.a);
console.log(first.e);
console.log(first.missing);
