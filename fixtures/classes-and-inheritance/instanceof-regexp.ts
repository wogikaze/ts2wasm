// instanceof RegExp prototype chain from extends
class MyRegExp extends RegExp {}

const r1 = new RegExp("hello");
const r2 = new MyRegExp("world");

// Basic RegExp instanceof
console.log(r1 instanceof RegExp);
console.log(r1 instanceof MyRegExp);

// Extended class instanceof
console.log(r2 instanceof MyRegExp);
console.log(r2 instanceof RegExp);
console.log(r2 instanceof Object);

// Non-instance checks
console.log("string" instanceof RegExp);
console.log(null instanceof MyRegExp);
