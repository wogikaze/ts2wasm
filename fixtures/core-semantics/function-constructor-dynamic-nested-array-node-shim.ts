let body = "return { items: [7, 8] }";
let make = Function(body);
let obj = make();
console.log(obj.items.length);
console.log(obj.items[0]);
console.log(obj.items[1]);
console.log(obj.items[2]);
