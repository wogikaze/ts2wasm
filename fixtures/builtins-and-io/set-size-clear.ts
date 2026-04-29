let s = new Set();
console.log(s.size);
s.add("a");
s.add("b");
s.add("a");
console.log(s.size);
s.clear();
console.log(s.size);
console.log(s.has("a"));
