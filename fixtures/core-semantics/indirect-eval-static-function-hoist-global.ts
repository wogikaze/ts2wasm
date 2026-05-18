// Static indirect eval function declarations are hoisted onto the global object.
let result = (0, eval)(
  'indirectEvalHoistedFn(); function indirectEvalHoistedFn() { return "hoisted"; }'
);

console.log(result);
console.log(globalThis.indirectEvalHoistedFn());
