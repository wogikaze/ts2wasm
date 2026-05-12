// Missing property access
// Exercises: reading undefined property, conditional check

let obj = { x: 1 };

// Read missing property
console.log(obj.y);
console.log(obj["z"]);

// Read from empty object
let empty = {};
console.log(empty.a);
console.log(empty["b"]);

// Check undefined
let val = obj.missing;
console.log(val);

// Property exists vs missing
console.log(obj.x);
console.log(obj.y);
