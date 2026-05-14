function id(value: string): string {
  return value;
}

let obj = {
  ["a"]() {
    return "A";
  },
  [id("b")]() {
    return "B";
  },
};

let keys = Object.keys(obj);
console.log(keys[0]);
console.log(keys[1]);
