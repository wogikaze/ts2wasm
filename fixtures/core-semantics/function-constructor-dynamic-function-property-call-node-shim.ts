let body = "return { cb: function hostCallback(a, b) { return a + b; } }";
let make = Function(body);
let obj = make();
console.log(obj.cb(2, 5));
let cb = obj.cb;
console.log(cb(3, 4));
