// Reflect.* methods — basic smoke tests

const target = { x: 42, y: "hello" };

// Reflect.get
console.log(Reflect.get(target, "x"));    // 42
console.log(Reflect.get(target, "y"));    // "hello"
console.log(Reflect.get(target, "z"));    // undefined

// Reflect.has
console.log(Reflect.has(target, "x"));    // true
console.log(Reflect.has(target, "z"));    // false

// Reflect.deleteProperty
console.log(Reflect.deleteProperty(target, "y")); // true
console.log(Reflect.get(target, "y"));    // undefined
console.log(Reflect.has(target, "y"));    // false

// Reflect.isExtensible
console.log(Reflect.isExtensible(target)); // true

// Reflect.preventExtensions
console.log(Reflect.preventExtensions(target)); // true
console.log(Reflect.isExtensible(target)); // false

// Reflect.getPrototypeOf
const proto = Reflect.getPrototypeOf(target);
console.log(proto !== null); // true

// Reflect.getOwnPropertyDescriptor
const desc = Reflect.getOwnPropertyDescriptor(target, "x");
console.log(desc !== undefined); // true
console.log(desc.value); // 42
console.log(desc.writable); // true
console.log(desc.enumerable); // true
console.log(desc.configurable); // true

// Reflect.defineProperty
const obj2 = {};
console.log(Reflect.defineProperty(obj2, "a", { value: 100, writable: true, enumerable: true, configurable: true })); // true
console.log(Reflect.get(obj2, "a")); // 100

// Reflect.ownKeys
const sym = Symbol("test");
const obj3 = { b: 2, a: 1 };
obj3[sym] = 3;
const keys = Reflect.ownKeys(obj3);
console.log(keys.length); // 3
console.log(keys.indexOf("a") >= 0); // true
console.log(keys.indexOf("b") >= 0); // true
