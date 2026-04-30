let cond = true;
let left = 0n;
let right = 3n;

if (cond) {
  left = 123456789012345678901234567890n;
} else {
  left = 1;
}

console.log(left / right);
