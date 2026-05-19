let body = "return { child: { x: 8, cb: function hostCallback(a) { return this.x + a; } } }";
let make = Function(body);
let obj = make();
let key = "child";
let child = obj?.[key];
console.log(child.cb(3));
