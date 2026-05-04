// Test: String.prototype.substr (Annex B legacy method)
// Basic cases
console.log("hello".substr(1, 2));
console.log("hello".substr(1, 3));
console.log("hello".substr(0, 5));

// Negative start
console.log("hello".substr(-3, 2));
console.log("hello".substr(-1, 1));

// Length beyond string
console.log("hello".substr(1, 10));

// No length (returns rest of string - but we need explicit length arg)
console.log("hello".substr(2, 3));

// Start beyond length
console.log("hello".substr(10, 2));

// Zero length
console.log("hello".substr(0, 0));

// Start exactly 0
console.log("hello".substr(0, 3));

// Start negative, length negative (spec: returns "")
console.log("hello".substr(1, -1));

// Empty string
console.log("".substr(0, 0));
console.log("".substr(0, 1));
