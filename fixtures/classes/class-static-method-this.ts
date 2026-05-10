// Test static method this binding
class MyClass {
  static value: number = 0;

  static setValue(v: number) {
    MyClass.value = v;
  }

  static getValue() {
    return MyClass.value;
  }
}

MyClass.setValue(99);
console.log(MyClass.getValue());
