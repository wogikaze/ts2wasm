// Boolean.prototype.toString/valueOf and Symbol.prototype.toString/valueOf
// Test Boolean.prototype.valueOf
let b: boolean = true;
let bv: any = b.valueOf();

// Test Boolean.prototype.toString
let bs: any = b.toString();

// Test Symbol.prototype.valueOf
let s: symbol = Symbol("test");
let sv: any = s.valueOf();

// Test Symbol.prototype.toString
let ss: any = s.toString();
