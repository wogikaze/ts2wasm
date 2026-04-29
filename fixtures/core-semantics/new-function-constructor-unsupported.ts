// Diagnostic fixture for dynamic new Function(...) evaluation tracked by issue 062b.
let f = new Function("return 1");
console.log(f());
