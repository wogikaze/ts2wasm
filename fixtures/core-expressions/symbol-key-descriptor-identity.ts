const first: any = Symbol("slot");
const second: any = Symbol("slot");
const obj: any = {};

Object.defineProperty(obj, first, {
  value: "first",
  enumerable: true,
  configurable: true,
  writable: true,
});
Object.defineProperty(obj, second, {
  value: "second",
  enumerable: false,
  configurable: true,
  writable: true,
});

console.log(obj[first]);
console.log(obj[second]);
console.log(Object.getOwnPropertyDescriptor(obj, first).value);
console.log(Object.getOwnPropertyDescriptor(obj, second).value);
console.log(first in obj);
console.log(second in obj);
console.log(Object.hasOwn(obj, first));
console.log(Object.hasOwn(obj, second));
console.log(obj.propertyIsEnumerable(first));
console.log(obj.propertyIsEnumerable(second));

const keys: any = Object.keys(obj);
const symbols: any = Object.getOwnPropertySymbols(obj);

console.log(keys.length);
console.log(symbols.length);
console.log(symbols[0] === first);
console.log(symbols[1] === second);

delete obj[first];

console.log(first in obj);
console.log(second in obj);
console.log(obj[first]);
console.log(obj[second]);
