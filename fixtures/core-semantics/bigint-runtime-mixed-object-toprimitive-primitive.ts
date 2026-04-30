console.log(({ valueOf: () => true }) == 1n);
console.log(1n == ({ valueOf: () => 1 }));
console.log(({ valueOf: () => null }) == 0n);
console.log(0n != ({ toString: () => undefined }));
console.log(({ toString: () => "3" }) == 3n);

let localNumber = { valueOf: () => 2 };
console.log(localNumber <= 2n);
console.log(3n > localNumber);

let localString = { toString: () => "4" };
console.log(localString < 5n);
console.log(5n >= localString);

console.log(({ toString: () => true }) < 2n);
