// Diagnostic fixture for direct eval() tracked by issue 429.
// Non-literal argument ensures parser doesn't expand it.
let x = "1 + 1";
eval(x);
console.log("unreachable");
