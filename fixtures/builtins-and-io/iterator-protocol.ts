// Iterator protocol and for-of iteration (W5)
// Test basic for-of iteration on arrays
let sum = 0;
for (const val of [10, 20, 30]) {
  sum = sum + val;
}
console.log("sum=" + sum);

// For-of with break
for (const val of [5, 10, 15, 20]) {
  if (val === 15) {
    console.log("found=" + val);
    break;
  }
}

// Empty array
let count = 0;
for (const x of []) {
  count = count + 1;
}
console.log("empty=" + count);

// Single element
for (const x of [42]) {
  console.log("single=" + x);
}
