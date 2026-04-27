// Dynamic property access tests
// @ts-nocheck - TypeScript can't verify dynamic properties

const obj = { a: 1, b: 2, c: 3 };
const key = "a";
const value = obj[key];
console.log(value);

const key2 = "b";
obj[key2] = 10;
console.log(obj[key2]);

const key3 = "c";
const key4 = "d";
obj[key3] = 30;
obj[key4] = 40;
console.log(obj[key3]);
console.log(obj[key4]);

let key5 = "y";
let obj2 = { x: 1, y: 2 };
console.log(obj2[key5]);

let idx = 1;
let arr = [10, 20, 30];
console.log(arr[idx]);
