// @ts-nocheck
let generic = new Error("generic stack");
let type_error = new TypeError("type stack");
let reference = new ReferenceError("reference stack");
let syntax = new SyntaxError("syntax stack");

console.log(generic.stack.indexOf("Error: generic stack") === 0);
console.log(type_error.stack.indexOf("TypeError: type stack") === 0);
console.log(reference.stack.indexOf("ReferenceError: reference stack") === 0);
console.log(syntax.stack.indexOf("SyntaxError: syntax stack") === 0);
