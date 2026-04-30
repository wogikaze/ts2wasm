let value = "bad";
if (Date.now() === -1) {
  value = "10";
}
console.log(BigInt(value));
