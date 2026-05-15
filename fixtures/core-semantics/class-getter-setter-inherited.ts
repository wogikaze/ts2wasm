// Inherited getter/setter from parent class
class Base {
  _x: number = 0;
  get x() {
    return this._x;
  }
  set x(v: number) {
    this._x = v;
  }
}

class Derived extends Base {
  get x() {
    return this._x * 2;
  }
}

const d = new Derived();
d.x = 5;
console.log(d.x);
const b = new Base();
b.x = 5;
console.log(b.x);
