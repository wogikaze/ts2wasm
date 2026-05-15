// Set.prototype.values() — returns array of values
let s = new Set();
s.add("x");
s.add("y");
s.add("z");

let values = s.values();
console.log(values.length);
console.log(values[0]);
console.log(values[1]);
console.log(values[2]);
