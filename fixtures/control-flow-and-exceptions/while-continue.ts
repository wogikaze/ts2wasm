let x = 0;
let sum = 0;
while (x < 5) {
  x = x + 1;
  if (x === 3) {
    continue;
  }
  sum = sum + x;
}
