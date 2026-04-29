function makeValues() {
  return "345";
}

let values = makeValues();
function sum(a, b, c) {
  return a + b + c;
}

console.log(sum(...values));
