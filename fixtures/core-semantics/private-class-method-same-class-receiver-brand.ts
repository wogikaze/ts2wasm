class C {
  #m() {
    console.log("body");
  }

  callFrom(other) {
    try {
      other.#m();
      console.log("after-call");
    } catch (e) {
      console.log("caught");
    }
  }
}

let c = new C();
c.callFrom({});
c.callFrom(c);
