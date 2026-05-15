let object = {
  999999999999999999n: "large",
  0xfn() {
    return "hex";
  },
  0b101n: "binary",
};

console.log(object["999999999999999999"]);
console.log(object["15"]());
console.log(object["5"]);

let { 1n: one } = { "1": "destructured" };
console.log(one);
