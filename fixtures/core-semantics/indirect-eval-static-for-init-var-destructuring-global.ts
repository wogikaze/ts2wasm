let item = "caller";
let first = "caller";

let result = (0, eval)('for (var { item } = { item: 6 }; false;) {} for (var [first, ...rest] = [8, 9]; false;) {} item + ":" + first + ":" + rest.length');

console.log(result);
console.log(item);
console.log(first);
console.log(globalThis.item);
console.log(globalThis.first);
console.log(globalThis.rest.length);

delete globalThis.item;
delete globalThis.first;
delete globalThis.rest;
