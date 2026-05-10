// W5.2: Centralized property access through OrdinaryGet/OrdinarySet
// Tests that basic obj.x, obj["x"], obj.x = v patterns work end-to-end

const obj = { a: 10, b: 20 };

// Static property access
console.log(obj.a);

// Dynamic property access
const key = "b";
console.log(obj[key]);

// Property write
obj.a = 99;
console.log(obj.a);
console.log(obj.b);

// Property write via dynamic key
obj[key] = 55;
console.log(obj.b);
