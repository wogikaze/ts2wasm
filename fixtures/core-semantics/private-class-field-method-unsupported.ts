class Base {}

class C extends Base {
  static #m() {
    return 1;
  }
}

let c = new C();
console.log(c);
