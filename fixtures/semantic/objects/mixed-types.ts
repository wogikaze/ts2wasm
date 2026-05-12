// Object properties with mixed value types
// Exercises: properties holding number, string, boolean, null, array

let obj = {
  num: 42,
  str: "hello",
  bool: true,
  nul: null,
  arr: [1, 2, 3]
};

console.log(obj.num);
console.log(obj.str);
console.log(obj.bool);
console.log(obj.nul);
console.log(obj.arr.length);
console.log(obj.arr[0]);
console.log(obj.arr[1]);

// Nested mixed types
let nested = {
  name: "config",
  values: [10, 20, 30],
  active: true
};
console.log(nested.name);
console.log(nested.values[1]);
console.log(nested.active);
