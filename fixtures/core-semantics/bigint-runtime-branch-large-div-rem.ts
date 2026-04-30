let cond = true;
let left = 0n;
let right = 1n;

if (cond) {
  left = 123456789012345678901234567890n;
  right = 12345678901234567890n;
} else {
  left = 987654321098765432109876543210n;
  right = 7777777777777777777n;
}

console.log(left / right);
console.log(left % right);

cond = false;

if (cond) {
  left = 184467440737095516160n;
  right = 11n;
} else {
  left = 112233445566778899001122334455n;
  right = 13n;
}

console.log(left / right);
console.log(left % right);
