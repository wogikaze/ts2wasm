// @ts-nocheck
// Error.prototype.toString behavior: "name: message" per spec

// Test 1: new Error().toString() returns "Error" (no message)
console.log(new Error().toString() === "Error");

// Test 2: new Error("msg").toString() returns "Error: msg"
console.log(new Error("test message").toString() === "Error: test message");

// Test 3: new TypeError("type").toString() returns "TypeError: type"
console.log(new TypeError("type error").toString() === "TypeError: type error");

// Test 4: new RangeError("range").toString() returns "RangeError: range"
console.log(new RangeError("range error").toString() === "RangeError: range error");

// Test 5: new SyntaxError("syntax").toString() returns "SyntaxError: syntax"
console.log(new SyntaxError("syntax error").toString() === "SyntaxError: syntax error");

// Test 6: new ReferenceError("ref").toString() returns "ReferenceError: ref"
console.log(new ReferenceError("ref error").toString() === "ReferenceError: ref error");

// Test 7: new EvalError("eval").toString() returns "EvalError: eval"
console.log(new EvalError("eval error").toString() === "EvalError: eval error");

// Test 8: new URIError("uri").toString() returns "URIError: uri"
console.log(new URIError("uri error").toString() === "URIError: uri error");

// Test 9: Error.prototype.toString.call on plain object should use "Error" as default name
console.log(Error.prototype.toString.call({ message: "custom" }) === "Error: custom");

// Test 10: toString with custom name
console.log(Error.prototype.toString.call({ name: "MyError", message: "woops" }) === "MyError: woops");

// Test 11: toString when message is empty — returns just the name
console.log(new Error().toString() === "Error");

// Test 12: toString when message is empty string returns just name
console.log(Error.prototype.toString.call({ name: "Custom" }) === "Custom");
