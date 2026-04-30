console.log(({ toString: () => "1" }) == 1n);
console.log(1n == ({ toString: () => "1" }));

let objectString = { toString: () => "2" };
console.log(2n == objectString);
console.log(objectString != 3n);
console.log(({ toString: () => "3" }) != 4n);
