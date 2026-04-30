let dynamicWide = 18446744073709551616n;
if (true) {
  dynamicWide = 18446744073709551617n;
}

console.log(~dynamicWide);
console.log(dynamicWide & 255n);
console.log(dynamicWide | 255n);
console.log(dynamicWide ^ 255n);
