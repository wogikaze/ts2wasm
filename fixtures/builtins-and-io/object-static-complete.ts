// Object static methods comprehensive test

// Object.fromEntries
let source = [["a", 1], ["b", 2], ["a", 3]];
let obj = Object.fromEntries(source);
console.log(obj.a);        // 3 (last wins)
console.log(obj.b);        // 2

// Object.hasOwn
console.log(Object.hasOwn(obj, "a"));        // true
console.log(Object.hasOwn(obj, "missing"));  // false

// Object.keys / values / entries
console.log(Object.keys(obj).length);        // 2
console.log(Object.values(obj)[0]);          // 3
console.log(Object.entries(obj).length);     // 2

// Object.is
console.log(Object.is(obj, obj));            // true
console.log(Object.is(obj, Object.fromEntries([["a", 3], ["b", 2]]))); // false

// Object.groupBy
let grouped = Object.groupBy([1, 2, 3, 4], (value) => value % 2 === 0 ? "even" : "odd");
console.log(grouped.odd.length);             // 2
console.log(grouped.odd[0]);                 // 1
console.log(grouped.even.length);            // 2
console.log(grouped.even[1]);                // 4

// Object.assign
let target = { x: 1 };
let src = { y: 2 };
Object.assign(target, src);
console.log(target.y);                       // 2

// Object.create
let proto = { hello: "world" };
let created = Object.create(proto);
console.log(created.hello);                  // world

// Object.defineProperty
let defObj = {};
Object.defineProperty(defObj, "p", { value: 42, writable: true, enumerable: true, configurable: true });
console.log(defObj.p);                       // 42

// Object.freeze / isFrozen
let frozen = { a: 1 };
console.log(Object.isFrozen(frozen));        // false
Object.freeze(frozen);
console.log(Object.isFrozen(frozen));        // true

// Object.seal / isSealed
let sealed = { a: 1 };
console.log(Object.isSealed(sealed));        // false
Object.seal(sealed);
console.log(Object.isSealed(sealed));        // true

// Object.isExtensible / preventExtensions
let ext = { a: 1 };
console.log(Object.isExtensible(ext));       // true
Object.preventExtensions(ext);
console.log(Object.isExtensible(ext));       // false

// Object.getOwnPropertyDescriptor
let desc = Object.getOwnPropertyDescriptor(obj, "a");
console.log(desc !== undefined);             // true
console.log(desc.value);                     // 3

// Object.getOwnPropertyNames
let names = Object.getOwnPropertyNames(obj);
console.log(names.length);                   // 2
console.log(names.indexOf("a") >= 0);        // true

// Object.getOwnPropertySymbols
let sym = Symbol("test");
let symObj = { [sym]: 1 };
let symbols = Object.getOwnPropertySymbols(symObj);
console.log(symbols.length);                 // 1

// Object.getPrototypeOf / setPrototypeOf
let protoObj = {};
let child = {};
Object.setPrototypeOf(child, protoObj);
console.log(Object.getPrototypeOf(child) === Object.getPrototypeOf(protoObj)); // false (different objects)

// Object.isPrototypeOf (instance method)
console.log(protoObj.isPrototypeOf(child));  // true

// Object.hasOwnProperty (instance method)
console.log(obj.hasOwnProperty("a"));        // true
console.log(obj.hasOwnProperty("missing"));  // false
