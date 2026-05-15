// Reflect.construct should produce a clear unsupported diagnostic
// (Reflect.get, .set, .has, etc. are now supported)
const target = { x: 42 };
console.log(Reflect.construct(target, []));
