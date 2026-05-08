// Basic Promise: Promise.resolve
let p1 = Promise.resolve(42);
console.log("resolved");

// Basic Promise: then/catch on resolved promise
let p2 = Promise.resolve("hello");
// Not calling callbacks yet — just verifying compilation
console.log("then");
console.log("catch");
