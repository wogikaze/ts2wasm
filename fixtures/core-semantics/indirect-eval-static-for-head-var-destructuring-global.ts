let item = "caller";
let first = "caller";
let rest = "caller";

let result = (0, eval)("for (var { item } of [{ item: 6 }]) {} for (var [first, ...rest] of [[8, 9]]) {} item + ':' + first + ':' + rest.length");

console.log(result);
console.log(item);
console.log(first);
console.log(rest);
console.log(globalThis.item);
console.log(globalThis.first);
console.log(globalThis.rest.length);

delete globalThis.item;
delete globalThis.first;
delete globalThis.rest;
