// Inherited getter/setter through prototype chain (using parent methods)
class Base {
  _val: number = 0;
  get value() {
    return this._val;
  }
  set value(v: number) {
    this._val = v;
  }
}

class Derived extends Base {
  getValue() {
    // Use parent getter/setter via inheritance
    return this.value;
  }
  setValue(v: number) {
    this.value = v;
  }
}

const b = new Base();
b.value = 10;
console.log(b.value);

const d = new Derived();
d.setValue(10);
console.log(d.getValue());
