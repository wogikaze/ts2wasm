let F = Function("return 1");
let obj = new F();

console.log(typeof obj);
console.log(F.prototype.constructor === F);
console.log(obj instanceof F);
