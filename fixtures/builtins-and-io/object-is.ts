// Object.is tests
console.log(Object.is(1, 1));
console.log(Object.is(1, 2));
console.log(Object.is("hello", "hello"));
console.log(Object.is("hello", "world"));
console.log(Object.is(true, true));
console.log(Object.is(true, false));
console.log(Object.is(null, null));
console.log(Object.is(undefined, undefined));
console.log(Object.is([], []));
let a = [1, 2, 3];
let b = a;
console.log(Object.is(a, b));
console.log(Object.is(a, [1, 2, 3]));
