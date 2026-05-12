// basic do-while loop
let i = 0;
do {
  console.log(i);
  i = i + 1;
} while (i < 3);

// do-while executes at least once even when condition is false
let j = 0;
do {
  console.log("runs once " + j);
  j = j + 1;
} while (false);

// do-while with break
let k = 0;
do {
  k = k + 1;
  if (k === 3) {
    break;
  }
} while (true);
console.log("k after break " + k);

// do-while with continue
let m = 0;
let sum = 0;
do {
  m = m + 1;
  if (m === 3) {
    continue;
  }
  sum = sum + m;
} while (m < 5);
console.log("do-while sum " + sum);

// nested do-while
let outer = 0;
do {
  let inner = 0;
  do {
    console.log("nested do " + outer + "," + inner);
    inner = inner + 1;
  } while (inner < 2);
  outer = outer + 1;
} while (outer < 2);
