// Strict mode function semantics
"use strict";
function f() {
    return typeof this;
}
console.log(f());
console.log(f.call(undefined));
console.log(f.call(42));
