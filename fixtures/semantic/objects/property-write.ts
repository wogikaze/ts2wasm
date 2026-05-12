// Object property write/assignment
// Exercises: obj.key = value, obj["key"] = value, obj[variable] = value

let obj = { x: 1, y: 2 };

// Dot notation assignment
obj.x = 10;
console.log(obj.x);

// Bracket notation assignment
obj["y"] = 20;
console.log(obj.y);

// Variable key assignment
let k = "x";
obj[k] = 100;
console.log(obj.x);

// Add new property
obj.z = 3;
console.log(obj.z);
console.log(obj.x);
console.log(obj.y);

// Overwrite with different type
obj.x = "hello";
console.log(obj.x);
