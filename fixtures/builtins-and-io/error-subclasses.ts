// @ts-nocheck
// Test all 7 error subtypes with name, message, stack, and cause

// 1. Generic Error with cause (string)
let e1 = new Error("generic", { cause: "root cause" });
console.log(e1.name === "Error");
console.log(e1.message === "generic");
console.log(e1.stack.indexOf("Error: generic") === 0);
console.log(e1.cause === "root cause");

// 2. TypeError
let e2 = new TypeError("type error");
console.log(e2.name === "TypeError");
console.log(e2.message === "type error");
console.log(e2.stack.indexOf("TypeError: type error") === 0);

// 3. RangeError
let e3 = new RangeError("range error");
console.log(e3.name === "RangeError");
console.log(e3.message === "range error");
console.log(e3.stack.indexOf("RangeError: range error") === 0);

// 4. ReferenceError
let e4 = new ReferenceError("reference error");
console.log(e4.name === "ReferenceError");
console.log(e4.message === "reference error");
console.log(e4.stack.indexOf("ReferenceError: reference error") === 0);

// 5. SyntaxError
let e5 = new SyntaxError("syntax error");
console.log(e5.name === "SyntaxError");
console.log(e5.message === "syntax error");
console.log(e5.stack.indexOf("SyntaxError: syntax error") === 0);

// 6. EvalError
let e6 = new EvalError("eval error");
console.log(e6.name === "EvalError");
console.log(e6.message === "eval error");
console.log(e6.stack.indexOf("EvalError: eval error") === 0);

// 7. URIError
let e7 = new URIError("uri error");
console.log(e7.name === "URIError");
console.log(e7.message === "uri error");
console.log(e7.stack.indexOf("URIError: uri error") === 0);

// Cause with numeric value
let e8 = new Error("with number cause", { cause: 42 });
console.log(e8.cause === 42);
