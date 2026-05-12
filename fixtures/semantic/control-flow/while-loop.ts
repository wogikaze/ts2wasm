// basic while loop
let i = 0;
while (i < 3) {
  console.log(i);
  i = i + 1;
}

// while with break
let j = 0;
while (true) {
  if (j >= 3) {
    break;
  }
  console.log("break-test " + j);
  j = j + 1;
}

// while with continue
let k = 0;
let sum = 0;
while (k < 5) {
  k = k + 1;
  if (k === 3) {
    continue;
  }
  sum = sum + k;
}
console.log("sum " + sum);

// while with falsy condition (should not execute)
console.log("before unreachable");
while (false) {
  console.log("unreachable");
}
console.log("after unreachable");
