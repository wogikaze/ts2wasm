const object: any = {
  [1 + 1]: "add",
  [3 - 1]: "subtract",
  [1 | 3]: "bitwise or",
};

console.log(object[2]);
console.log(object[String(1 + 1)]);
console.log(object[String(3 - 1)]);
console.log(object[String(1 | 3)]);
