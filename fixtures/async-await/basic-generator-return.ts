function* simpleGenerator(): Generator<number, string, void> {
  yield 1;
  yield 2;
  return "done";
}

const gen = simpleGenerator();
const r1 = gen.next();
console.log(r1.value);
console.log(r1.done);
const r2 = gen.next();
console.log(r2.value);
console.log(r2.done);
const r3 = gen.next();
console.log(r3.value);
console.log(r3.done);
