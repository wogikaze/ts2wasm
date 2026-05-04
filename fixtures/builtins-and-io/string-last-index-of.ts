// String.prototype.lastIndexOf tests
let s = "hello world hello";

// Basic: find last occurrence
let idx1 = s.lastIndexOf("hello");
console.log(idx1);

// Single occurrence
let idx2 = "hello world".lastIndexOf("world");
console.log(idx2);

// Not found
let idx3 = "hello world".lastIndexOf("xyz");
console.log(idx3);

// Empty needle returns string length
let idx4 = "hello".lastIndexOf("");
console.log(idx4);

// Needle at end
let idx5 = "hello world".lastIndexOf("world");
console.log(idx5);

// Repeated character
let idx6 = "aaaa".lastIndexOf("aa");
console.log(idx6);
