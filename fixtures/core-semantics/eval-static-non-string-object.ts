let obj = eval({ value: 7 });
console.log(obj.value);

let arr = (0, eval)([1, 2]);
console.log(arr.length);

let fn = eval(function () { return 5; });
console.log(fn());

let arrow = eval(() => 6);
console.log(arrow());
