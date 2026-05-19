let local = "caller";

console.log((0, eval)("var indirectVar = 11; function indirectFn(){ return indirectVar + 1; } indirectFn()"));
console.log(globalThis.indirectVar);
console.log(globalThis.indirectFn());
console.log(typeof indirectVar);
console.log(local);
