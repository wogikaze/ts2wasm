// Basic generator function - ID 212 (W5)
function* gen() {
  yield 1;
  yield 2;
}
const g = gen();
const a = g.next();
console.log(a.value);
console.log(a.done);
const b = g.next();
console.log(b.value);
console.log(b.done);
const c = g.next();
console.log(c.value);
console.log(c.done);
