let key = "drop";
let removed = "caller";
let rest = "caller";

let result = (0, eval)('for (var { [key]: removed, ...rest } = { drop: 1, keep: "ok" }; false;) {} removed + ":" + rest.keep + ":" + rest.drop');

console.log(result);
console.log(removed);
console.log(rest);
console.log(globalThis.removed);
console.log(globalThis.rest.keep);
console.log(globalThis.rest.drop);

delete globalThis.removed;
delete globalThis.rest;
