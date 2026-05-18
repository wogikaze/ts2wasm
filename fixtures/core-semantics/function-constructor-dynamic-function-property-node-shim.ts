let body = "return { cb: function hostCallback(a, b) { return a + b; } }";
let make = Function(body);
let obj = make();
console.log(obj.cb.length);
console.log(obj.cb.name);
console.log(obj.cb.prototype);
console.log(obj.cb.missing);
