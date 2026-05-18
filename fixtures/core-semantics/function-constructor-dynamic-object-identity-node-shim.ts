let body = "globalThis.__ts2wasmIdentityObject ??= { value: 7 }; return globalThis.__ts2wasmIdentityObject";
let make = Function(body);
let first = make();
let second = make();
console.log(first === second);
console.log(second.value);
