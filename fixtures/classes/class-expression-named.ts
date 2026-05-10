// Test named class expression
const Cls = class NamedClass {
  constructor(v: number) {
    (this as any).v = v;
  }

  getValue() {
    return (this as any).v;
  }
};

const obj = new Cls(42);
console.log(obj.getValue());
