// Static indirect eval function declarations land on the global object.
let localValue = "local";
let result = (0, eval)(
  'function indirectEvalGlobalFn() { return "global-fn"; } indirectEvalGlobalFn()'
);

console.log(result);
console.log(globalThis.indirectEvalGlobalFn());
console.log(localValue);
