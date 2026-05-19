let body = "return { child: { x: 8, cb: function hostCallback(a) { return this.x + a; } } }";
let make = Function(body);
let obj = make();
let child = obj?.child;
console.log(child.cb(2));
