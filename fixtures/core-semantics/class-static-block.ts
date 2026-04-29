console.log("before");

class Counter {
  static value() {
    return "value";
  }

  static {
    console.log("first:" + Counter.value());
  }

  static {
    let label = "second";
    console.log(label + ":" + Counter.value());
  }
}

console.log("after");
