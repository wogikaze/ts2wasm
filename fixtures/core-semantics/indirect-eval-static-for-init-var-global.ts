let x = "caller";

let result = (0, eval)("for (var x = 1; false;) {} x");

console.log(result);
console.log(x);
console.log(globalThis.x);

delete globalThis.x;
