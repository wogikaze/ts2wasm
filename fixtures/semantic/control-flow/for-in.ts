// for-in over object properties
let obj = { a: 1, b: 2, c: 3 };
let keys = "";
for (let key in obj) {
  keys = keys + key;
}
console.log(keys);

// for-in counting properties
let count = 0;
for (let k in obj) {
  count = count + 1;
}
console.log("count " + count);

// for-in with array (iterates indices)
let arr = [10, 20, 30];
let indices = "";
for (let idx in arr) {
  indices = indices + idx;
}
console.log("array indices " + indices);
