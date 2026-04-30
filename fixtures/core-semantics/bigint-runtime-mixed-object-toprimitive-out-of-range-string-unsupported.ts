let objectString = { toString: () => "2147483648" };
console.log(objectString < 1n);
