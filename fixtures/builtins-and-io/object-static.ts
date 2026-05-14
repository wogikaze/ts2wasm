// Object static methods test (fromEntries, hasOwn, is)

// Object.fromEntries
let arr: any = [["a", 1], ["b", 2], ["c", 3]];
let obj: any = Object.fromEntries(arr);
console.log(Object.keys(obj).length);     // 3

// Object.hasOwn
let h1: any = Object.hasOwn(obj, "a");
let h2: any = Object.hasOwn(obj, "z");
console.log(h1);                          // true
console.log(h2);                          // false

// Object.is
let i1: any = Object.is(1, 1);
let i2: any = Object.is(1, 2);
let i3: any = Object.is(null, null);
console.log(i1);                          // true
console.log(i2);                          // false
console.log(i3);                          // true
