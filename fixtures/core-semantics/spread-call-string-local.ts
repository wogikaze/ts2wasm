let letters = "abc";
let copy = letters;

function join(a, b, c) {
  return a + b + c;
}

console.log(join(...copy));
