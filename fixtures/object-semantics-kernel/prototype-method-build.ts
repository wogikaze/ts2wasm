// W5.7: Class prototype method dispatch (build_smoke)
// Tests that methods defined on class prototype are callable

class Greeter {
  prefix: string;

  constructor(p: string) {
    this.prefix = p;
  }

  greet(name: string): string {
    return this.prefix + " " + name;
  }
}

const g = new Greeter("Hello");
console.log(g.greet("World"));

class ExtendedGreeter extends Greeter {
  greet(name: string): string {
    return this.prefix + " " + name + "!";
  }
}

const e = new ExtendedGreeter("Hi");
console.log(e.greet("there"));
