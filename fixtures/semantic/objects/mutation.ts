// Object mutation — add and change properties
// Exercises: adding new properties, changing values

let obj = { a: 1 };

// Add new properties
obj.b = 2;
console.log(obj.a);
console.log(obj.b);

obj.c = 3;
console.log(obj.c);

// Change existing
obj.a = 99;
console.log(obj.a);

// Mixed add and change
obj.b = "hello";
obj.d = true;
console.log(obj.b);
console.log(obj.d);

// Bracket notation add
obj["e"] = 5;
console.log(obj.e);
