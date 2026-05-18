let eval = source => "shadow:" + source;
let missing = null;

console.log(eval?.("not intrinsic"));
console.log(missing?.("skip"));
