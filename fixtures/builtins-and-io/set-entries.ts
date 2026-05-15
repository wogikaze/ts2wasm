// Set.prototype.entries() — returns array of [value, value] pairs
let s = new Set();
s.add("x");
s.add("y");
s.add("z");

let entries = s.entries();
console.log(entries.length);
console.log(entries[0][1]);
console.log(entries[1][1]);
console.log(entries[2][1]);
