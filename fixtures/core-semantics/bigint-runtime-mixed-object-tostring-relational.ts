console.log(({ toString: () => "1" }) < 2n);
console.log(2n > ({ toString: () => "1" }));
console.log(({ toString: () => "2" }) <= 2n);
console.log(3n >= ({ toString: () => "2" }));

let objectString = { toString: () => "-2" };
console.log(objectString < 0n);
console.log(0n > objectString);
