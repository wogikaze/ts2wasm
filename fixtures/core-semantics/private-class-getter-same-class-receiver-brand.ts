class C {
  get #x() {
    console.log("getter");
    return 3;
  }

  readFrom(other) {
    try {
      other.#x;
      console.log("after-get");
    } catch (e) {
      console.log("caught");
    }
  }
}

let c = new C();
c.readFrom({});
c.readFrom(c);
