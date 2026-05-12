// Method receiver preservation across different instances
// Each method call must preserve the correct `this` binding.

class Box {
  value: number;
  label: string;

  constructor(value: number, label: string) {
    this.value = value;
    this.label = label;
  }

  getValue(): number {
    return this.value;
  }

  getLabel(): string {
    return this.label;
  }

  add(other: Box): number {
    return this.value + other.value;
  }
}

let a = new Box(10, "A");
let b = new Box(20, "B");
let c = new Box(30, "C");

console.log(a.getValue());
console.log(b.getValue());
console.log(c.getValue());
console.log(a.getLabel());
console.log(b.getLabel());
console.log(a.add(b));
console.log(b.add(c));
console.log(a.add(c));
