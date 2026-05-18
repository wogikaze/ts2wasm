let body = "globalThis.__ts2wasmShapeObject ??= { value: 1 }; globalThis.__ts2wasmShapeCount = (globalThis.__ts2wasmShapeCount ?? 0) + 1; if (globalThis.__ts2wasmShapeCount === 2) { globalThis.__ts2wasmShapeObject.value = 2; globalThis.__ts2wasmShapeObject.label = 'ok'; } return globalThis.__ts2wasmShapeObject";
let getObject = Function(body);
let first = getObject();
console.log(first.value);
let second = getObject();
console.log(first === second);
console.log(second.value);
console.log(second.label);
console.log(first.missing);
