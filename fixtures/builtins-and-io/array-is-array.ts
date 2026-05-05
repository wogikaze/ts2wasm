// Array.isArray basic tests
function assert(condition: boolean, message: string): void {
    if (!condition) {
        throw new Error(message);
    }
}

// Basic cases
assert(Array.isArray([]) === true, "empty array should be array");
assert(Array.isArray([1, 2, 3]) === true, "number array should be array");
assert(Array.isArray(["a", "b"]) === true, "string array should be array");

// Non-array cases
assert(Array.isArray({}) === false, "object should not be array");
assert(Array.isArray("hello") === false, "string should not be array");
assert(Array.isArray(42) === false, "number should not be array");
assert(Array.isArray(null) === false, "null should not be array");
assert(Array.isArray(undefined) === false, "undefined should not be array");

// Nested arrays
assert(Array.isArray([[1, 2], [3, 4]]) === true, "nested array should be array");

// Variable references
let a = [1, 2, 3];
assert(Array.isArray(a) === true, "variable array should be array");
let b = "not an array";
assert(Array.isArray(b) === false, "string variable should not be array");

console.log("PASS");
