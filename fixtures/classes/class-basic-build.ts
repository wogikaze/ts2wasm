// Test that class constructs produce clear unsupported diagnostics via binary MVP
class Greeter {
  greeting: string;
  constructor(message: string) {
    this.greeting = message;
  }
  greet(): string {
    return "Hello, " + this.greeting;
  }
}
const g = new Greeter("world");
console.log(g.greet());
