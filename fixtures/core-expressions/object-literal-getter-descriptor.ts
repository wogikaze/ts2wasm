let object = {
  value: 7,
  get computed() {
    return this.value + 1;
  },
};

let descriptor = Object.getOwnPropertyDescriptor(object, "computed");
console.log(descriptor.get === undefined ? "no_getter" : "has_getter");
console.log(descriptor.set === undefined ? "no_setter" : "has_setter");
console.log(descriptor.enumerable ? "enumerable" : "not_enumerable");
console.log(descriptor.configurable ? "configurable" : "not_configurable");

let valueDescriptor = Object.getOwnPropertyDescriptor(object, "value");
console.log(valueDescriptor.value);
