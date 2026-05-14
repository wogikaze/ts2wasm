// @ts-nocheck
let generic = new Error("generic");
let range = new RangeError("range");
let reference = new ReferenceError("reference");
let syntax = new SyntaxError("syntax");
let type_err = new TypeError("type");
let eval_err = new EvalError("eval");
let uri = new URIError("uri");

console.log(generic.name);
console.log(range.name);
console.log(reference.name);
console.log(syntax.name);
console.log(type_err.name);
console.log(eval_err.name);
console.log(uri.name);
