let key = "b";
let methodKey = "c";
let obj = {
  ["a"]() {
    return "A";
  },
  [key]: 2,
  [methodKey]() {
    return "C";
  },
};

console.log(obj.a());
console.log(obj.b);
console.log(obj.c());
