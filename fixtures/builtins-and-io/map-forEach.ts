// Map.prototype.forEach with arrow callback
let m = new Map();
m.set("a", 1);
m.set("b", 2);
m.set("c", 3);

// forEach iterates in insertion order
m.forEach((v, k) => {
  console.log(k);
  console.log(v);
});

// forEach on empty map
let empty = new Map();
empty.forEach(() => {
  console.log("should not print");
});
console.log("done");
