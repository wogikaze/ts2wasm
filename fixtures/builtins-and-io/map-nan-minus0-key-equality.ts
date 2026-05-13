let nanMap = new Map();
nanMap.set(NaN, "first");
console.log(nanMap.has(NaN));
console.log(nanMap.get(NaN));
nanMap.set(NaN, "second");
console.log(nanMap.size);
console.log(nanMap.get(NaN));

let zeroMap = new Map();
zeroMap.set(-0, "minus");
console.log(zeroMap.has(0));
console.log(zeroMap.get(0));
zeroMap.set(0, "plus");
console.log(zeroMap.size);
console.log(zeroMap.get(-0));

let firstKey = {};
let secondKey = {};
let objectMap = new Map();
objectMap.set(firstKey, "first");
objectMap.set(secondKey, "second");
console.log(objectMap.size);
console.log(objectMap.get(firstKey));
console.log(objectMap.get(secondKey));
console.log(objectMap.has({}));
