// TypeScript parameter property — constructor shorthand
class Person {
  constructor(public name: string, private age: number) {}
  greet() {
    console.log(this.name);
  }
}
const p = new Person("Alice", 30);
p.greet();
