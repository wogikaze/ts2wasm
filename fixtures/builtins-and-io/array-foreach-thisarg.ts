// Array.prototype.forEach with thisArg semantics
// forEach with arrow callback — element, index, array args
let arr = [10, 20, 30];
arr.forEach((v, i) => {
  console.log(v);
  console.log(i);
});

// forEach on empty array
let empty: number[] = [];
let called = false;
empty.forEach(() => { called = true; });
console.log(called ? 0 : 1);
