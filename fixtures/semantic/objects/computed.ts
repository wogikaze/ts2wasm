// Object computed property access via bracket notation
// Exercises: obj[key] with variable key, dynamic key assignment

let obj = { x: 10, y: 20, z: 30 };

// Dynamic read
let key1 = "x";
console.log(obj[key1]);

let key2 = "z";
console.log(obj[key2]);

// Dynamic write
let wk = "y";
obj[wk] = 200;
console.log(obj.y);
console.log(obj[wk]);

// Non-existent computed
let missingKey = "w";
console.log(obj[missingKey]);

// Add via computed
let newKey = "newProp";
obj[newKey] = 42;
console.log(obj[newKey]);
console.log(obj.newProp);
