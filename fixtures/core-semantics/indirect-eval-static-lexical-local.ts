// Static indirect eval lexical declarations are eval-local and do not capture caller locals.
let indirectEvalLexical = "caller";
let result = (0, eval)(
  'let indirectEvalLexical = "eval"; const other = indirectEvalLexical; other'
);

console.log(result);
console.log(indirectEvalLexical);
console.log(typeof globalThis.indirectEvalLexical);
