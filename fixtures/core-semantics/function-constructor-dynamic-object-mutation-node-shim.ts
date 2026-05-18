let body = "globalThis.__ts2wasmMutableObject ??= { value: 1 }; globalThis.__ts2wasmMutableObject.value = globalThis.__ts2wasmMutableObject.value + 1; return globalThis.__ts2wasmMutableObject";
let bump = Function(body);
let first = bump();
console.log(first.value);
let second = bump();
console.log(first === second);
console.log(second.value);
console.log(first.value);
