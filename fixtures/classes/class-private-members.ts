// Test that class private fields and methods compile successfully
class MyClass {
  #x = 42;
  #count = 0;

  #increment() {
    this.#count = this.#count + 1;
  }

  getValue() {
    return this.#x;
  }

  tick() {
    this.#increment();
    return this.#count;
  }

  static getSecret() {
    return "hidden";
  }
}

const obj = new MyClass();
console.log(obj.getValue());
console.log(obj.tick());
