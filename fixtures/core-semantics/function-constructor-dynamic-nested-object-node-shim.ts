let body = "return { child: { value: 7, label: 'ok' } }";
let make = Function(body);
let obj = make();
console.log(obj.child.value);
console.log(obj.child.label);
console.log(obj.child.missing);
