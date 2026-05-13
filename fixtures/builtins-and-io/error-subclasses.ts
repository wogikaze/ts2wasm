let generic = new Error("generic message");
let eval_err = new EvalError("eval message", { cause: "eval cause" });
let range = new RangeError("range message", { cause: 42 });
let reference = new ReferenceError("reference message", { cause: null });
let syntax = new SyntaxError("syntax message");
let type_err = new TypeError("type message", { cause: true });
let uri = new URIError("uri message", { cause: undefined });

console.log(generic.message);
console.log(eval_err.message);
console.log(range.message);
console.log(reference.message);
console.log(syntax.message);
console.log(type_err.message);
console.log(uri.message);

console.log(generic instanceof Error);
console.log(eval_err instanceof EvalError);
console.log(eval_err instanceof Error);
console.log(range instanceof RangeError);
console.log(reference instanceof ReferenceError);
console.log(syntax instanceof SyntaxError);
console.log(type_err instanceof TypeError);
console.log(uri instanceof URIError);

console.log(eval_err.cause);
console.log(range.cause);
console.log(reference.cause);
console.log(type_err.cause);
console.log(uri.cause);
