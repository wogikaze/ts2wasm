// Class constructor with return value
class MyClass {
  constructor() {
    this.x = 10;
    return;
  }
  getX() {
    return this.x;
  }
}

const obj = new MyClass();
console.log(obj.getX());
