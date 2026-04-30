let objectString = { toString: () => ({ value: "1" }) };
console.log(1n == objectString);
