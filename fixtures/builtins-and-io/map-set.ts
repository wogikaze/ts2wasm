let m = new Map();
console.log(m.has("a"));
console.log(m.get("a"));
console.log(m.set("a", 1) === m);
console.log(m.get("a"));
console.log(m.has("a"));
console.log(m.set("a", 2) === m);
console.log(m.get("a"));

let s = new Set();
console.log(s.has("x"));
console.log(s.add("x") === s);
console.log(s.has("x"));
console.log(s.has("missing"));
