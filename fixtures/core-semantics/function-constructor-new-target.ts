let call = Function("return typeof new.target");
console.log(call());

let F = Function("this.kind = typeof new.target; this.name = new.target.name; this.len = new.target.length; this.desc = Object.getOwnPropertyDescriptor(new.target, 'length').value");
let obj = new F();
console.log(obj.kind);
console.log(obj.name);
console.log(obj.len);
console.log(obj.desc);
