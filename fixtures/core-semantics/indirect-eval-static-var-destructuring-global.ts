let item = "caller";
let leaf = "caller";
let first = "caller";

let result = (0, eval)('var { value: item, nested: { leaf } } = { value: 6, nested: { leaf: 7 } }; var [first, ...rest] = [8, 9]; item + ":" + leaf + ":" + first + ":" + rest.length');

console.log(result);
console.log(item);
console.log(leaf);
console.log(first);
console.log(globalThis.item);
console.log(globalThis.leaf);
console.log(globalThis.first);
console.log(globalThis.rest.length);

delete globalThis.item;
delete globalThis.leaf;
delete globalThis.first;
delete globalThis.rest;
