// Static indirect eval var declarations are hoisted onto the global object.
let result = (0, eval)(
  'if (false) { var indirectEvalHoisted = 1; } "indirectEvalHoisted" in globalThis'
);

console.log(result);
console.log(globalThis.indirectEvalHoisted);
