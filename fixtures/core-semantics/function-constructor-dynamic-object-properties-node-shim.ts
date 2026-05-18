let body = "return ({ value: 7, label: 'ok' })";
let f = Function(body);
let obj = f();
console.log(obj.value);
console.log(obj.label);
console.log(obj.missing);
