class C {
  static {
    this.x = 7;
  }

  static value() {
    return this.x;
  }
}

console.log(C.value());
