// Test fixture for `as` type assertion expression parsing
let x: number = 42;
let y = x as number;
let z = y as Array<number>;
console.log(z);
