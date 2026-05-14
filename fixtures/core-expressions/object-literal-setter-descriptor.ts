let object = {
  set value(next: number) {
    this._value = next;
  },
  plain: 1
};

let descriptor = Object.getOwnPropertyDescriptor(object, "value");
console.log(descriptor.set === undefined ? "no_setter" : "has_setter");
console.log(descriptor.enumerable ? "enumerable" : "not_enumerable");
console.log(descriptor.configurable ? "configurable" : "not_configurable");

console.log(object.plain);
