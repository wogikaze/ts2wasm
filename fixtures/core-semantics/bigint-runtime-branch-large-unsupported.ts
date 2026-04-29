let a = 1n;
let cond = true;
if (cond) {
  a = 18446744073709551616n;
}
console.log(a + 1n);
