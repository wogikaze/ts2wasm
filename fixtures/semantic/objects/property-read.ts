// Object property read — dot and bracket notation
// Exercises: obj.key, obj["key"], obj[variable]

let obj = { a: 1, b: 2, c: 3 };

// Dot notation
console.log(obj.a);
console.log(obj.b);
console.log(obj.c);

// Bracket notation with string literal
console.log(obj["a"]);
console.log(obj["c"]);

// Bracket notation with variable
let key = "b";
console.log(obj[key]);

// Read non-existent property (returns undefined)
console.log(obj.z);
console.log(obj["missing"]);
