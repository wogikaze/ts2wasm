let key = "caller";
let item = "caller";

globalThis.key = "value";

let result = (0, eval)('for (var { [key]: item } of [{ value: "ok" }]) {} item');

console.log(result);
console.log(key);
console.log(item);
console.log(globalThis.item);

delete globalThis.key;
delete globalThis.item;
