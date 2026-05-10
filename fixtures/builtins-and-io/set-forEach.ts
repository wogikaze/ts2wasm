// Set.prototype.forEach with arrow callback
let s = new Set();
s.add("x");
s.add("y");
s.add("z");

// forEach iterates in insertion order
s.forEach((v) => {
  console.log(v);
});

// forEach on empty set
let empty = new Set();
empty.forEach(() => {
  console.log("should not print");
});
console.log("done");
