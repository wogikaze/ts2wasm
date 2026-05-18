let sloppy = Function("return typeof this");
console.log(sloppy());

let strict = Function('"use strict"; return this === undefined');
console.log(strict());
