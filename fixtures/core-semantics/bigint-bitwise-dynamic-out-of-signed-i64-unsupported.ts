let dynamicWide = 18446744073709551616n;
let branchSelector = BigInt("1");
if (branchSelector === 1n) {
  dynamicWide = 18446744073709551617n;
} else {
  dynamicWide = 18446744073709551617n;
}

console.log(~dynamicWide);
console.log(dynamicWide & 255n);
console.log(dynamicWide | 255n);
console.log(dynamicWide ^ 255n);
