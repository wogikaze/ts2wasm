// Map.prototype.entries() — returns array of [key, value] pairs
let m = new Map();
m.set("a", 1);
m.set("b", 2);
m.set("c", 3);

let entries = m.entries();
console.log(entries.length);
console.log(entries[0][0]);
console.log(entries[0][1]);
console.log(entries[1][0]);
console.log(entries[1][1]);
console.log(entries[2][0]);
console.log(entries[2][1]);
