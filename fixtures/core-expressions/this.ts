const o = { x: 42, f() { return this.x; } };
console.log(o.f());
