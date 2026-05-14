let generic = new Error("generic");
let eval_err = new EvalError("eval message");
let range = new RangeError("range message");
let reference = new ReferenceError("reference message");
let syntax = new SyntaxError("syntax message");
let type_err = new TypeError("type message");
let uri = new URIError("uri message");

console.log(generic.message);
console.log(eval_err.message);
console.log(range.message);
console.log(reference.message);
console.log(syntax.message);
console.log(type_err.message);
console.log(uri.message);

console.log(eval_err instanceof EvalError);
console.log(eval_err instanceof Error);
console.log(uri instanceof URIError);
console.log(uri instanceof Error);
console.log(range instanceof RangeError);
