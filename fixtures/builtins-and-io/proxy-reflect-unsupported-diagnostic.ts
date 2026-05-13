// Reflect should produce a clear unsupported diagnostic
const target = { x: 42 };
console.log(Reflect.get(target, "x"));
