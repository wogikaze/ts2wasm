let localKey = "caller";
let localValue = "caller";

let result = (0, eval)("for (var key in { alpha: 1 }) {} for (var value of [4]) {} key + ':' + value");

console.log(result);
console.log(localKey);
console.log(localValue);
console.log(globalThis.key);
console.log(globalThis.value);

delete globalThis.key;
delete globalThis.value;
