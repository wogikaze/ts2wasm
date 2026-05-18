let body = "return { x: 9, cb: function hostCallback() { return this.x; } }";
let make = Function(body);
let obj = make();
console.log(obj.cb());
