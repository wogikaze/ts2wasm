function gap() {
  return "ignored";
}

console.log(JSON.stringify({ a: 1, b: 2 }, null, { gap: 2 }));
console.log(JSON.stringify([1, 2], null, gap));
