// Class constructor returning this (standard constructor behavior)
class MyClass {
  value: number;
  constructor(v: number) {
    this.value = v;
  }
}

const obj = new MyClass(42);
console.log(obj.value);
