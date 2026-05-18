let F = new Function("return 1");

console.log(F.prototype.constructor === F);
console.log(new F() instanceof F);
