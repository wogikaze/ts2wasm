let prefix = "a" + "b";
let letters = prefix + "c";
let values = [...letters];

function join(a, b, c) {
  return a + b + c;
}

console.log(values.length);
console.log(values[0]);
console.log(values[1]);
console.log(values[2]);
console.log(join(...letters));
