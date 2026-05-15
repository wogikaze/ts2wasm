// Array find/findIndex with complex predicates
const arr = [10, 20, 30, 40, 50];
console.log(arr.find(x => x > 25));
console.log(arr.findIndex(x => x > 25));
console.log(arr.find(x => x > 100)); // undefined
console.log(arr.findIndex(x => x > 100)); // -1

// With thisArg
const threshold = { limit: 30 };
console.log(arr.find(function(this: any, x: number) { return x > this.limit; }, threshold));
