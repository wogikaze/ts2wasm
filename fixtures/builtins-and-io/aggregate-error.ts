// @ts-nocheck
// AggregateError constructor behavior

// Test 1: Basic construction with errors and message
let e1 = new AggregateError([], "test message");
console.log(e1.name === "AggregateError");
console.log(e1.message === "test message");
console.log(Array.isArray(e1.errors));

// Test 2: AggregateError with error objects
let e2 = new AggregateError([new Error("e1"), new TypeError("e2")], "multiple errors");
console.log(e2.errors.length === 2);
console.log(e2.errors[0] instanceof Error);
console.log(e2.errors[1] instanceof TypeError);

// Test 3: instanceof and prototype chain
console.log(e1 instanceof AggregateError);
console.log(e1 instanceof Error);

// Test 4: No message argument
let e3 = new AggregateError([1, 2, 3]);
console.log(e3.message === "");
console.log(e3.errors.length === 3);

// Test 5: Empty errors array
let e4 = new AggregateError([]);
console.log(e4.errors.length === 0);

// Test 6: toString
console.log(e1.toString() === "AggregateError: test message");
console.log(e4.toString() === "AggregateError");

// Test 7: name property on different instances
console.log(e1.name === "AggregateError");
console.log(e2.name === "AggregateError");

// Test 8: errors is an array with correct values
console.log(e3.errors[0] === 1);
console.log(e3.errors[1] === 2);
console.log(e3.errors[2] === 3);
