// Set.prototype.keys() — returns array of values (same as values())
let s = new Set();
s.add("x");
s.add("y");
s.add("z");

let keys = s.keys();
console.log(keys.length);
console.log(keys[0]);
console.log(keys[1]);
console.log(keys[2]);
