// Array.prototype.concat - flat shallow copy
let a = [1, 2, 3];
let b = [4, 5, 6];
let c = a.concat(b);

// Expected: [1, 2, 3, 4, 5, 6]
console.log(c[0] === 1);
console.log(c[1] === 2);
console.log(c[2] === 3);
console.log(c[3] === 4);
console.log(c[4] === 5);
console.log(c[5] === 6);
