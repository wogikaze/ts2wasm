// Object.fromEntries test
let arr: any = [["x", 42], ["y", "hello"]];
let obj: any = Object.fromEntries(arr);
console.log(Object.keys(obj).length);
