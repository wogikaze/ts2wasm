class Parent {
  static value() {
    return 1;
  }
}

class Child extends Parent {
  static {
    console.log(super.value());
  }
}
