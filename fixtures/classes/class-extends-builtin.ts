// Test that extending builtin constructors compiles successfully
class MyArray extends Array {
  first() {
    return this[0];
  }
}

const arr = new MyArray(1, 2, 3);
console.log(arr.length);
