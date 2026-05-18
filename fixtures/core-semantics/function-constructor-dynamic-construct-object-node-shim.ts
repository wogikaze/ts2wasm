let param = "value";
let body = "this.value = value; this.label = 'ok'";
let F = Function(param, body);
let obj = new F(7);
console.log(obj.value);
console.log(obj.label);
console.log(obj.missing);
