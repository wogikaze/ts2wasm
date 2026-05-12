// basic for loop
for (let i = 0; i < 3; i = i + 1) {
  console.log(i);
}

// for loop with break
let found = -1;
for (let i = 0; i < 10; i = i + 1) {
  if (i === 5) {
    found = i;
    break;
  }
}
console.log("found " + found);

// for loop with continue
let oddSum = 0;
for (let i = 0; i < 10; i = i + 1) {
  if (i % 2 === 0) {
    continue;
  }
  oddSum = oddSum + i;
}
console.log("oddSum " + oddSum);

// for loop with empty body (just increment)
let counter = 0;
for (let i = 0; i < 5; i = i + 1) {
  counter = counter + 1;
}
console.log("counter " + counter);

// for without body statements (will parse but should still loop)
let x = 0;
for (; x < 3; x = x + 1) {
  // empty body
}
console.log("x after empty-for " + x);
