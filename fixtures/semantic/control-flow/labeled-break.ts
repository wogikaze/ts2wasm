// labeled break: break out of outer loop from inner
let outer = 0;
let inner = 0;

outerLoop:
while (outer < 3) {
  outer = outer + 1;
  inner = 0;
  while (inner < 5) {
    inner = inner + 1;
    if (inner === 2) {
      break outerLoop;
    }
  }
}

console.log("outer " + outer);
console.log("inner " + inner);

// labeled break with for loop
let a = 0;
let b = 0;
label1:
for (a = 0; a < 3; a = a + 1) {
  for (b = 0; b < 3; b = b + 1) {
    if (a === 1 && b === 1) {
      break label1;
    }
  }
}

console.log("a " + a);
console.log("b " + b);
