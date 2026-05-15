// @ts-nocheck
// Dynamic instanceof with a function (not a class) as RHS
function MyClass() {}
const obj = new MyClass();
console.log(obj instanceof MyClass);
