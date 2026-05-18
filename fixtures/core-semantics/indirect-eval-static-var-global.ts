// Static indirect eval var declarations land on the global object.
let localValue = "local";
let result = (0, eval)("var indirectEvalGlobal = 42; indirectEvalGlobal");

console.log(result);
console.log(globalThis.indirectEvalGlobal);
console.log(localValue);
