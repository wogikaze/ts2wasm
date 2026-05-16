// Reflect.apply and Reflect.construct — host shim wrappers
// These call through to $host_reflect_apply / $host_reflect_construct

function add(a: number, b: number): number {
  return a + b;
}

// Reflect.apply with a simple function
console.log(Reflect.apply(add, undefined, [1, 2]));     // 3

// Reflect.construct (uses host shim)
class Point {
  x: number;
  y: number;
  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}
const p = Reflect.construct(Point, [3, 4]);
console.log(p.x);                                         // 3
console.log(p.y);                                         // 4
