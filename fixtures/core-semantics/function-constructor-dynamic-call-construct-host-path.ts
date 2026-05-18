let callBody = "return 7";
let f = Function(callBody);
f();

let ctorBody = "this.value = 7";
let C = Function(ctorBody);
new C();
