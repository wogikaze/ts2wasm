// Test class getter/setter syntax
class MyClass {
  _x: number = 0;

  get x() {
    return this._x;
  }

  set x(val: number) {
    this._x = val;
  }
}

const obj = new MyClass();
obj.x = 42;
console.log(obj.x);
