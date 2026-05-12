// labeled continue: skip to next iteration of outer loop
let outerCount = 0;
let visited = 0;

outerLoop:
for (let i = 0; i < 3; i = i + 1) {
  outerCount = outerCount + 1;
  for (let j = 0; j < 3; j = j + 1) {
    if (j === 1) {
      continue outerLoop;
    }
    visited = visited + 1;
  }
}

console.log("outerCount " + outerCount);
console.log("visited " + visited);

// labeled continue with while
let a = 0;
let sum = 0;

outerWhile:
while (a < 3) {
  a = a + 1;
  let b = 0;
  while (b < 3) {
    b = b + 1;
    if (b === 2) {
      continue outerWhile;
    }
    sum = sum + 1;
  }
}

console.log("a " + a);
console.log("sum " + sum);
