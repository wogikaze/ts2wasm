class C {
  #x = 1;

  readExternal(other) {
    try {
      console.log("before");
      other.#x;
      console.log("after-read");
    } catch (e) {
      console.log("caught");
    }
    console.log("after");
  }
}

let c = new C();
c.readExternal({});
