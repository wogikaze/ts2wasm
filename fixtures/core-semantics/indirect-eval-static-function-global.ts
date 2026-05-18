// Static indirect eval function declarations land on the global object.
let indirectEvalGlobalValue = "local";
globalThis.indirectEvalGlobalValue = "global";
let result = (0, eval)(
  "function indirectEvalGlobalFn() { return indirectEvalGlobalValue; } indirectEvalGlobalFn()"
);

console.log(result);
console.log(globalThis.indirectEvalGlobalFn());
console.log(indirectEvalGlobalValue);
