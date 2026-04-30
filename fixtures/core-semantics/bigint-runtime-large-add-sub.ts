let base = 18446744073709551616n;
let one = 1n;
console.log(base + one);
console.log(base - one);
console.log(one - base);

let branch = 0n;
if (true) {
  branch = base + one;
} else {
  branch = one;
}
console.log(branch);
branch = branch - 2n;
console.log(branch);
