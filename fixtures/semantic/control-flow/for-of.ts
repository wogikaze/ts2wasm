// for-of over array
let arr = [10, 20, 30];
for (let val of arr) {
  console.log(val);
}

// for-of summing array
let sum = 0;
for (let v of [1, 2, 3, 4, 5]) {
  sum = sum + v;
}
console.log("sum " + sum);

// for-of with break
let found = -1;
for (let v of [1, 2, 3, 4, 5]) {
  if (v === 3) {
    found = v;
    break;
  }
}
console.log("found " + found);

// for-of with continue (skip even numbers)
let oddSum = 0;
for (let v of [1, 2, 3, 4, 5]) {
  if (v % 2 === 0) {
    continue;
  }
  oddSum = oddSum + v;
}
console.log("oddSum " + oddSum);
