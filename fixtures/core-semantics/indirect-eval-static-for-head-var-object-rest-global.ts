let drop = "caller";
let rest = "caller";

let result = (0, eval)('for (var { drop, ...rest } of [{ drop: 1, keep: "ok", next: 2 }]) {} rest.keep + ":" + rest.next + ":" + drop');

console.log(result);
console.log(drop);
console.log(rest);
console.log(globalThis.drop);
console.log(globalThis.rest.keep);
console.log(globalThis.rest.next);

delete globalThis.drop;
delete globalThis.rest;
