let doubled = Array.prototype.map.call({ "0": 1, "1": 2, length: 2 }, value => value * 2);

console.log(doubled.length);
console.log(doubled[0]);
console.log(doubled[1]);
