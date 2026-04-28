// Diagnostic fixture for unsupported RegExp.prototype.compile tracked by issue 051.
let r = new RegExp("abc");
r.compile("def");
