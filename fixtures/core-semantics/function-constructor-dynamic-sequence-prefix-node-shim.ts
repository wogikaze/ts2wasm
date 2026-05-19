let side = 0;
let generated = Function((side = 1, "return 7"));

console.log(side);
console.log(generated());
