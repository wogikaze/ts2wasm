let local = "caller";

console.log(eval?.("var optVar = 7; function optFn(){ return optVar + 1; } optFn()"));
console.log(globalThis.optVar);
console.log(globalThis.optFn());
console.log(typeof optVar);
console.log(local);
