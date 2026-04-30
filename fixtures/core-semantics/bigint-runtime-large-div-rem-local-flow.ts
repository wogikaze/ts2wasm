let left = 987654321098765432109876543210n;
let right = 7777777777777777777n;
let quotient = left / right;
let remainder = left % right;
console.log(quotient);
console.log(remainder);

if (quotient > 0n) {
  console.log(left / 9n);
} else {
  console.log(right % 9n);
}
