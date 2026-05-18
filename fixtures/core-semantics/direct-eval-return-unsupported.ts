// Expected diagnostic: eval source text is parsed as eval code, where a bare
// return statement is invalid. This is a syntax/CompletionRecord boundary, not
// evidence that static direct eval is unsupported as a whole.
function run() {
  return eval("return 1;");
}

console.log(run());
