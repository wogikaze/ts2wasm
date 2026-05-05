class Greeter {
  greet(name) {
    return "Hello, " + name + "!";
  }
  double(x) {
    return x * 2;
  }
  noop() {
    return 42;
  }
}

// new C().method() with no args
let a = new Greeter().noop();
console.log(a);

// new C().method() with one arg
let b = new Greeter().greet("World");
console.log(b);

// new C().method() with one numeric arg
let c = new Greeter().double(21);
console.log(c);

// new C().method().method() — chained (new C() evaluated once per chain)
let d = new Greeter().greet("Alice");
console.log(d);
