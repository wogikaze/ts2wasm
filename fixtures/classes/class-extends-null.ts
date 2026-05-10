// Test extends null
class NullProto extends null {
  constructor() {
    super();
  }
  getValue() {
    return 42;
  }
}

const obj = new NullProto();
console.log(obj.getValue());
console.log(Object.getPrototypeOf(obj));
console.log(Object.getPrototypeOf(NullProto.prototype));
