let source = "({ x: 11, cb: function hostCallback(a) { return this.x + a; } })";
let obj = eval(source);
console.log(obj["cb"](4));
