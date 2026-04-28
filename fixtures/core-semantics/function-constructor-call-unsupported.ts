// Diagnostic fixture for dynamic Function(...) evaluation tracked by issue 062.
let f = Function("return 1");
console.log(f());
