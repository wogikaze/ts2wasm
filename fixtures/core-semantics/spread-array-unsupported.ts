function makeValues() {
  return "abc";
}

let values = makeValues();
let copy = [...values];
console.log(copy.length);
