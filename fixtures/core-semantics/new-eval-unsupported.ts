// Diagnostic fixture for new eval() tracked by issue 429.
new eval("1 + 1");
console.log("unreachable");
