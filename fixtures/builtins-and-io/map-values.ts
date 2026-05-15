// Map.prototype.values() — returns array of values
let m = new Map();
m.set("a", 1);
m.set("b", 2);
m.set("c", 3);

let values = m.values();
console.log(values.length);
console.log(values[0]);
console.log(values[1]);
console.log(values[2]);
