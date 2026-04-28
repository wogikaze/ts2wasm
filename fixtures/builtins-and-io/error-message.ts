let generic = new Error("generic message");
let type_error = new TypeError("type message");
let reference = new ReferenceError("reference message");
let syntax = new SyntaxError("syntax message");
let empty = new Error();

console.log(generic.message);
console.log(type_error.message);
console.log(reference.message);
console.log(syntax.message);
console.log(empty.message);
