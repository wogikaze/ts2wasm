let objectBigInt = { valueOf: () => ({ x: 1n }) };
console.log(objectBigInt == 1n);
