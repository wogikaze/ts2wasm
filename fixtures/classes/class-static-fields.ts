// Test static field initializers
class MyClass {
  static base = 10;
  static multiplier = 3;
  static value = MyClass.base + MyClass.multiplier;

  static getValue() {
    return MyClass.value;
  }
}

console.log(MyClass.getValue());
