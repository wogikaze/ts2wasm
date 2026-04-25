class Base {
  value() {
    return 7;
  }
}

class Child extends Base {
}

let c = new Child();
console.log(c.value());
