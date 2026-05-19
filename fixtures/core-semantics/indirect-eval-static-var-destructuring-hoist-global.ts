let item = "caller";

let result = (0, eval)("if (false) { var { item } = { item: 6 }; } typeof item");

console.log(result);
console.log(item);
console.log(globalThis.item);

delete globalThis.item;
