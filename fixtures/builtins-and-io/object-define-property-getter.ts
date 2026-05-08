// Object.defineProperty with getter/setter (accessor descriptor)
const obj: any = {};

// Define a property with a getter
Object.defineProperty(obj, "x", { get: () => 42, configurable: true });
const dx = Object.getOwnPropertyDescriptor(obj, "x");
// Test descriptor shape: accessor descriptors have get/set/configurable/enumerable
console.log(dx.get !== undefined ? "has_getter" : "no_getter");
console.log(dx.set === undefined ? "no_setter" : "has_setter");
console.log(dx.configurable ? "configurable" : "not_configurable");
console.log(dx.enumerable ? "enumerable" : "not_enumerable");

// Accessor properties with both get and set
let stored = 0;
Object.defineProperty(obj, "y", {
  get: () => stored,
  set: (v: number) => { stored = v; },
  enumerable: true,
  configurable: true
});
const dy = Object.getOwnPropertyDescriptor(obj, "y");
console.log(dy.get !== undefined ? "has_getter" : "no_getter");
console.log(dy.set !== undefined ? "has_setter" : "no_setter");
console.log(dy.configurable ? "configurable" : "not_configurable");
console.log(dy.enumerable ? "enumerable" : "not_enumerable");
