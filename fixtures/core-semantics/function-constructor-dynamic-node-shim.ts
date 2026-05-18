let body = "return 7";
let f = Function(body);
console.log(f());

let ctorBody = "this.value = 1";
let C = Function(ctorBody);
new C();
