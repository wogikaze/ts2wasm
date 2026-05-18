let body = "return 1";
let F = Function(body);
console.log(F.prototype.constructor === F);
console.log(F.prototype.constructor.name);
