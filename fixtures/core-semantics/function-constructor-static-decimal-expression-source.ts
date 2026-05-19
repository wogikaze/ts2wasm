let body = Function(1.5 + 2.5);
let returned = Function("return " + (1.25 + 2.75));

console.log(body());
console.log(returned());
console.log(body.toString());
console.log(returned.toString());
