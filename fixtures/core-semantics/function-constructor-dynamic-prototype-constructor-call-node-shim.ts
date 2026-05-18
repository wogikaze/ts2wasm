let body = "return this.constructor.name + ':' + arguments.length";
let F = Function(body);
console.log(F.prototype.constructor(1, 2));
