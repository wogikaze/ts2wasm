let proto = { kind: "proto" };
let obj = Object.create(proto);
obj.visible = 1;
Object.defineProperty(obj, "hidden", { value: 2, enumerable: false });

console.log(obj.hasOwnProperty("visible"));
console.log(obj.hasOwnProperty("kind"));
console.log(obj.propertyIsEnumerable("visible"));
console.log(obj.propertyIsEnumerable("hidden"));
console.log(obj.propertyIsEnumerable("kind"));

console.log(proto.isPrototypeOf(obj));
console.log(obj.isPrototypeOf(proto));

console.log(obj.toString());
console.log(obj.toLocaleString());
console.log(obj.valueOf() === obj);

console.log(obj.__proto__ === proto);
let other = { kind: "other" };
obj.__proto__ = other;
console.log(obj.__proto__ === other);
console.log(other.isPrototypeOf(obj));
console.log(proto.isPrototypeOf(obj));

console.log(Object.prototype.toString.call(obj));
console.log(Object.prototype.propertyIsEnumerable.call(obj, "visible"));
