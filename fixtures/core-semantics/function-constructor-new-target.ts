let call = Function("return typeof new.target");
console.log(call());

let F = Function("this.kind = typeof new.target");
let obj = new F();
console.log(obj.kind);
