let source = "({ x: 9, add: (a, b) => a + b, getX() { return this.x; } })";
let add;
let obj;
obj = eval(source);
console.log(obj.add(2, 5));
add = obj.add;
console.log(add(3, 4));
console.log(obj.getX());
