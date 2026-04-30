let receiver = {};
receiver[0] = 4;
receiver[1] = 7;
receiver.length = 1 + 1;

let mapped = Array.prototype.map.call(receiver, value => value);

console.log(mapped.length);
console.log(mapped[0]);
console.log(mapped[1]);
