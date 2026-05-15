// Inherited getter/setter
class Base {
  _val: number = 0;
  get value() {
    return this._val;
  }
  set value(v: number) {
    this._val = v;
  }
}

const b = new Base();
b.value = 42;
console.log(b.value);
