let body = "return ({ value: 1 })";
let f = Function(body);
console.log(f());
