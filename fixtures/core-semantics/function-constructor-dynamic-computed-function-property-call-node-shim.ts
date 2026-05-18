let body = "return { x: 8, cb: function hostCallback(a) { return this.x + a; } }";
let make = Function(body);
let obj = make();
console.log(obj["cb"](2));
