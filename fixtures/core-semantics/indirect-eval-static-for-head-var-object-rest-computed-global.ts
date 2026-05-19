let key = "caller";
let removed = "caller";
let rest = "caller";

globalThis.key = "drop";

let result = (0, eval)('for (var { [key]: removed, ...rest } of [{ drop: 1, keep: "ok" }]) {} removed + ":" + rest.keep + ":" + rest.drop');

console.log(result);
console.log(key);
console.log(removed);
console.log(rest);
console.log(globalThis.removed);
console.log(globalThis.rest.keep);
console.log(globalThis.rest.drop);

delete globalThis.key;
delete globalThis.removed;
delete globalThis.rest;
