let source = [["a", 1], ["b", 2], ["a", 3]];
let obj = Object.fromEntries(source);

console.log(obj.a);
console.log(obj.b);
console.log(Object.hasOwn(obj, "a"));
console.log(Object.hasOwn(obj, "missing"));
console.log(Object.keys(obj).length);
console.log(Object.values(obj)[0]);
console.log(Object.entries(obj)[1][0]);
console.log(Object.entries(obj)[1][1]);
console.log(Object.is(obj, obj));
console.log(Object.is(obj, Object.fromEntries([["a", 3], ["b", 2]])));

let grouped = Object.groupBy([1, 2, 3, 4], (value) => value % 2 === 0 ? "even" : "odd");
console.log(grouped.odd.length);
console.log(grouped.odd[0]);
console.log(grouped.even.length);
console.log(grouped.even[1]);

let byLength = Object.groupBy(["ant", "bear", "cat"], (value) => value.length);
console.log(Object.keys(byLength).length);
console.log(byLength[3][1]);
