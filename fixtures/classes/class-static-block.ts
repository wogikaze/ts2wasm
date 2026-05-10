// Test that class static blocks compile successfully
class MyClass {
  static x: number;
  static y: number;

  static {
    MyClass.x = 10;
    MyClass.y = 20;
  }

  static getX() {
    return MyClass.x;
  }
}

console.log(MyClass.getX());
