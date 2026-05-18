let p1 = "a";
let p2 = "b";
let body = "return a + b";
let f = Function(p1, p2, body);
console.log(f["toString"]());
