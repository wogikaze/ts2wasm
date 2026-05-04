// Object.prototype.valueOf returns the same value (identity)
// Test with numbers
let n: number = 42;
let nv: any = n.valueOf();
console.log(nv === 42);

// Test with strings
let s: string = "hello";
let sv: any = s.valueOf();
console.log(sv === "hello");

// Test with booleans
let b: boolean = true;
let bv: any = b.valueOf();
console.log(bv === true);

// Test with number variable
let x: number = 100;
let y: any = x.valueOf();
console.log(y === 100);
