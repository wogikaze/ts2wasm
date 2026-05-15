// Map.prototype.keys() — returns array of keys
let m = new Map();
m.set("a", 1);
m.set("b", 2);
m.set("c", 3);

let keys = m.keys();
console.log(keys.length);
console.log(keys[0]);
console.log(keys[1]);
console.log(keys[2]);
