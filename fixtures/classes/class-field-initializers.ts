// Test that class field initializers compile successfully
class MyClass {
  base = 1;
  multiplier = 2;
  value = this.base + this.multiplier;

  getValue() {
    return this.value;
  }
}

const obj = new MyClass();
console.log(obj.getValue());
