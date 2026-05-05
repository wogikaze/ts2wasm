console.log(({ valueOf() { return 1n; } }) == 1n);
console.log(1n == ({ valueOf() { return 1n; } }));
console.log(({ valueOf() { return 2n; } }) != 1n);
console.log(({ toString() { return "3"; } }) == 3n);
console.log(({ valueOf() { return { boxed: 1n }; }, toString() { return "4"; } }) == 4n);

let localBigInt = { valueOf() { return 5n; } };
console.log(localBigInt == 5n);

let localString = { toString() { return "6"; } };
console.log(6n == localString);
