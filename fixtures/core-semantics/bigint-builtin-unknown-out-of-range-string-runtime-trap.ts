let value = "18446744073709551616";
if (Date.now() === -1) {
  value = "10";
}
console.log(BigInt(value));
